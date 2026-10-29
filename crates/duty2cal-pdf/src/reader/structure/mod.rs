// SPDX-License-Identifier: MPL-2.0
//! The project's own reading of the supported PDF's file structure.
//!
//! The supplied bytes are parsed eagerly: the cross-reference sections first,
//! then every object they name, object streams decoded and their entries
//! inserted, and stream bodies sliced in a second pass so a forward-pointing
//! indirect `/Length` resolves. Nothing is repaired: a structural failure is the
//! unreadable refusal.

mod filter;
mod object;
mod parser;
mod xref;

pub(crate) use filter::decompress_stream;
pub(crate) use object::{Dict, ObjId, Object, Stream};

use std::collections::BTreeMap;

use duty2cal_core::application::SourceError;

use parser::{unreadable, Parser};

/// A loaded document: the supplied bytes and every object they define.
pub(crate) struct Document<'a> {
    bytes: &'a [u8],
    objects: BTreeMap<ObjId, Object>,
    trailer: Dict,
    encrypted: bool,
}

impl<'a> Document<'a> {
    /// The object `id` defines.
    pub(crate) fn get_object(&self, id: ObjId) -> Result<&Object, object::Error> {
        self.objects
            .get(&id)
            .ok_or_else(|| object::Error::new(format!("no object {} {} is defined", id.0, id.1)))
    }

    /// The dictionary `id` defines.
    pub(crate) fn get_dictionary(&self, id: ObjId) -> Result<&Dict, object::Error> {
        self.get_object(id).and_then(Object::as_dict)
    }

    /// The newest trailer.
    pub(crate) fn trailer(&self) -> &Dict {
        &self.trailer
    }

    /// Whether any trailer names an `/Encrypt` dictionary.
    pub(crate) fn is_encrypted(&self) -> bool {
        self.encrypted
    }

    /// A stream's body in the supplied bytes.
    pub(crate) fn stream_bytes(&self, stream: &Stream) -> &'a [u8] {
        &self.bytes[stream.data.clone()]
    }
}

/// How a stream's `/Length` is written.
enum LengthSpec {
    /// A direct non-negative integer.
    Direct(i64),
    /// A reference to an integer object.
    Indirect(ObjId),
    /// Absent or not a number.
    Missing,
}

/// Load the file structure of `bytes`.
pub(crate) fn load(bytes: &[u8]) -> Result<Document<'_>, SourceError> {
    let table = xref::read_xref(bytes)?;
    if table.encrypted {
        return Ok(Document {
            bytes,
            objects: BTreeMap::new(),
            trailer: table.trailer,
            encrypted: true,
        });
    }

    let mut objects = BTreeMap::new();
    let mut pending = Vec::new();
    for entry in table.entries.values() {
        if let xref::Entry::Uncompressed { offset } = entry {
            let parsed = parser::parse_indirect_object(bytes, *offset)?;
            if matches!(parsed.object, Object::Stream(_)) {
                pending.push(parsed.id);
            }
            objects.insert(parsed.id, parsed.object);
        }
    }

    let mut deferred = Vec::new();
    for id in pending {
        if !resolve_length(bytes, &mut objects, id)? {
            deferred.push(id);
        }
    }

    let mut containers: BTreeMap<u32, Vec<(u32, u16)>> = BTreeMap::new();
    for (id, entry) in &table.entries {
        if let xref::Entry::Compressed { container, index } = entry {
            containers
                .entry(*container)
                .or_default()
                .push((*id, *index));
        }
    }
    for (container, members) in containers {
        let parsed = {
            let object = objects
                .get(&(container, 0))
                .ok_or_else(|| unreadable("an object stream is not defined"))?;
            let stream = object
                .as_stream()
                .map_err(|error| unreadable(error.to_string()))?;
            let count = stream
                .dict
                .get(b"N")
                .and_then(Object::as_i64)
                .map_err(|error| unreadable(error.to_string()))?;
            let first = stream
                .dict
                .get(b"First")
                .and_then(Object::as_i64)
                .map_err(|error| unreadable(error.to_string()))?;
            let decoded = filter::decode_data(&stream.dict, &bytes[stream.data.clone()])
                .map_err(|error| unreadable(error.detail()))?;
            parse_object_stream(count, first, &decoded)?
        };
        let mut parsed: Vec<Option<Object>> = parsed.into_iter().map(Some).collect();
        for (number, index) in members {
            if let Some(Some(object)) = parsed.get_mut(index as usize).map(Option::take) {
                objects.entry((number, 0)).or_insert(object);
            }
        }
    }

    for id in deferred {
        if !resolve_length(bytes, &mut objects, id)? {
            return Err(unreadable("a stream carries no resolvable /Length"));
        }
    }

    Ok(Document {
        bytes,
        objects,
        trailer: table.trailer,
        encrypted: false,
    })
}

/// Slice one stream body from its `/Length`, or report that the length object is
/// not loaded yet.
fn resolve_length(
    bytes: &[u8],
    objects: &mut BTreeMap<ObjId, Object>,
    id: ObjId,
) -> Result<bool, SourceError> {
    let (start, spec) = {
        let Some(Object::Stream(stream)) = objects.get(&id) else {
            return Err(unreadable("a stream body has no stream"));
        };
        let spec = match stream.dict.get(b"Length") {
            Ok(Object::Integer(value)) => LengthSpec::Direct(*value),
            Ok(Object::Reference(target)) => LengthSpec::Indirect(*target),
            _ => LengthSpec::Missing,
        };
        (stream.data.start, spec)
    };
    let length = match spec {
        LengthSpec::Direct(value) => value,
        LengthSpec::Indirect(target) => match objects.get(&target) {
            None => return Ok(false),
            Some(object) => object
                .as_i64()
                .map_err(|_| unreadable("a stream /Length is not an integer"))?,
        },
        LengthSpec::Missing => return Err(unreadable("a stream carries no /Length")),
    };
    let length = usize::try_from(length).map_err(|_| unreadable("a stream /Length is negative"))?;
    let end = start
        .checked_add(length)
        .filter(|end| *end <= bytes.len())
        .ok_or_else(|| unreadable("a stream /Length runs past the end of the file"))?;
    if let Some(Object::Stream(stream)) = objects.get_mut(&id) {
        stream.data.end = end;
    }
    Ok(true)
}

/// Parse an object stream's `count` bodies after its `first` header bytes.
fn parse_object_stream(count: i64, first: i64, data: &[u8]) -> Result<Vec<Object>, SourceError> {
    let count =
        usize::try_from(count).map_err(|_| unreadable("an object stream /N is negative"))?;
    let first =
        usize::try_from(first).map_err(|_| unreadable("an object stream /First is negative"))?;
    if first > data.len() || count > data.len() {
        return Err(unreadable("an object stream header lies outside its body"));
    }
    let mut header = Parser::at(&data[..first], 0);
    let mut offsets = Vec::with_capacity(count);
    for _ in 0..count {
        let _number = header.parse_i64()?;
        let offset = header.parse_i64()?;
        offsets.push(
            usize::try_from(offset)
                .map_err(|_| unreadable("an object stream entry offset is negative"))?,
        );
    }
    let mut objects = Vec::with_capacity(count);
    for offset in offsets {
        let position = first
            .checked_add(offset)
            .filter(|position| *position <= data.len())
            .ok_or_else(|| unreadable("an object stream entry lies outside its body"))?;
        let mut parser = Parser::at(&data[first..], position - first);
        objects.push(parser.parse_object()?);
    }
    Ok(objects)
}
