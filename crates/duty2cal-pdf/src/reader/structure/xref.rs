// SPDX-License-Identifier: MPL-2.0
//! Cross-reference reading: the header, the last `%%EOF`, `startxref`, classic
//! sections, cross-reference streams, hybrid `/XRefStm` and any `/Prev` chain.
//!
//! There is no repair. A pointer outside the file, an unparsable section, an
//! unsupported `/W` shape or a missing `/Root` is the unreadable refusal.

use std::collections::{BTreeMap, BTreeSet};

use duty2cal_core::application::SourceError;

use super::filter::decode_data;
use super::object::{Dict, Object};
use super::parser::{parse_indirect_object, unreadable, Parser};

/// How the newest section names one object.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Entry {
    /// A free entry.
    Free,
    /// A body written at `offset`.
    Uncompressed {
        /// The body's offset in the document bytes.
        offset: usize,
    },
    /// A body inside an object stream.
    Compressed {
        /// The object stream's object number.
        container: u32,
        /// The body's index in the object stream.
        index: u16,
    },
}

/// The merged cross-reference sections and the newest trailer.
pub(crate) struct XrefTable {
    /// The object number's newest entry.
    pub entries: BTreeMap<u32, Entry>,
    /// The newest trailer, with older sections filling absent keys.
    pub trailer: Dict,
    /// Whether any section's trailer carries `/Encrypt`.
    pub encrypted: bool,
}

/// One parsed section.
struct Section {
    entries: Vec<(u32, Entry)>,
    trailer: Dict,
}

/// Read the cross-reference sections of `bytes`.
pub(crate) fn read_xref(bytes: &[u8]) -> Result<XrefTable, SourceError> {
    check_header(bytes)?;
    let mut entries = BTreeMap::new();
    let mut trailer = Dict::default();
    let mut encrypted = false;
    let mut seen = BTreeSet::new();
    let mut offset = Some(start_xref(bytes)?);
    while let Some(position) = offset {
        if !seen.insert(position) {
            break;
        }
        let section = parse_section(bytes, position)?;
        for (id, entry) in section.entries {
            entries.entry(id).or_insert(entry);
        }
        if let Ok(stream_offset) = section.trailer.get(b"XRefStm").and_then(Object::as_i64) {
            let stream_offset = usize::try_from(stream_offset)
                .map_err(|_| unreadable("the hybrid cross-reference pointer is negative"))?;
            let hybrid = parse_section(bytes, stream_offset)?;
            for (id, entry) in hybrid.entries {
                entries.entry(id).or_insert(entry);
            }
        }
        encrypted |= section.trailer.get(b"Encrypt").is_ok();
        for (key, value) in section.trailer.iter() {
            trailer.insert_if_absent(key.clone(), value.clone());
        }
        offset = section
            .trailer
            .get(b"Prev")
            .and_then(Object::as_i64)
            .ok()
            .map(|previous| {
                usize::try_from(previous)
                    .map_err(|_| unreadable("the previous cross-reference pointer is negative"))
            })
            .transpose()?;
    }
    if trailer.get(b"Root").is_err() {
        return Err(unreadable("the trailer carries no /Root"));
    }
    Ok(XrefTable {
        entries,
        trailer,
        encrypted,
    })
}

/// Check for the `%PDF-` header at its first occurrence.
fn check_header(bytes: &[u8]) -> Result<(), SourceError> {
    let offset = bytes
        .windows(5)
        .position(|window| window == b"%PDF-")
        .ok_or_else(|| unreadable("the file carries no PDF header"))?;
    let after = &bytes[offset + 5..];
    if !after.first().is_some_and(u8::is_ascii_digit) {
        return Err(unreadable("the PDF header carries no version"));
    }
    if !after.iter().any(|byte| matches!(byte, b'\n' | b'\r')) {
        return Err(unreadable("the PDF header line does not end"));
    }
    Ok(())
}

/// The offset after the `startxref` keyword nearest the last `%%EOF`.
fn start_xref(bytes: &[u8]) -> Result<usize, SourceError> {
    let window = bytes.len().saturating_sub(512);
    let eof = bytes[window..]
        .windows(5)
        .rposition(|window| window == b"%%EOF")
        .map(|position| window + position)
        .filter(|position| *position > 25)
        .ok_or_else(|| unreadable("the file carries no %%%%EOF"))?;
    let from = eof.saturating_sub(25);
    let keyword = bytes[from..eof]
        .windows(9)
        .rposition(|window| window == b"startxref")
        .map(|position| from + position)
        .ok_or_else(|| unreadable("the file carries no startxref"))?;
    let mut index = keyword + 9;
    while bytes
        .get(index)
        .is_some_and(|byte| byte.is_ascii_whitespace())
    {
        index += 1;
    }
    let start = index;
    while bytes.get(index).is_some_and(u8::is_ascii_digit) {
        index += 1;
    }
    if index == start {
        return Err(unreadable("startxref carries no offset"));
    }
    let text = std::str::from_utf8(&bytes[start..index])
        .map_err(|_| unreadable("the startxref offset is not ASCII"))?;
    let offset = text
        .parse::<usize>()
        .map_err(|_| unreadable("the startxref offset is out of range"))?;
    if offset >= bytes.len() {
        return Err(unreadable("the startxref pointer lies outside the file"));
    }
    Ok(offset)
}

/// Parse the section starting at `offset`.
fn parse_section(bytes: &[u8], offset: usize) -> Result<Section, SourceError> {
    if offset >= bytes.len() {
        return Err(unreadable(
            "a cross-reference pointer lies outside the file",
        ));
    }
    let mut parser = Parser::at(bytes, offset);
    if parser.consume_keyword(b"xref") {
        parse_classic(&mut parser)
    } else {
        parse_stream_section(bytes, offset)
    }
}

/// Parse a classic `xref` section and its `trailer`.
fn parse_classic(parser: &mut Parser<'_>) -> Result<Section, SourceError> {
    let mut entries = Vec::new();
    loop {
        parser.skip_space();
        if parser.peek_keyword(b"trailer") {
            parser.consume_keyword(b"trailer");
            break;
        }
        let start = parser.parse_i64()?;
        let count = parser.parse_i64()?;
        if start < 0 || count < 0 {
            return Err(unreadable("a cross-reference subsection is negative"));
        }
        let start = u32::try_from(start)
            .map_err(|_| unreadable("a cross-reference subsection starts out of range"))?;
        for index in 0..count {
            let offset = parser.parse_i64()?;
            let _generation = parser.parse_i64()?;
            let kind = parser.peek_byte()?;
            parser.advance();
            let entry = match kind {
                b'n' => Entry::Uncompressed {
                    offset: usize::try_from(offset)
                        .map_err(|_| unreadable("a cross-reference offset is negative"))?,
                },
                b'f' => Entry::Free,
                _ => {
                    return Err(unreadable(
                        "a cross-reference entry is neither free nor in use",
                    ))
                }
            };
            entries.push((start.saturating_add(index as u32), entry));
        }
    }
    let trailer = parser.parse_object()?;
    let trailer = match trailer {
        Object::Dictionary(dict) => dict,
        _ => return Err(unreadable("the trailer is not a dictionary")),
    };
    Ok(Section { entries, trailer })
}

/// Parse a cross-reference stream section.
fn parse_stream_section(bytes: &[u8], offset: usize) -> Result<Section, SourceError> {
    let indirect = parse_indirect_object(bytes, offset)?;
    let Object::Stream(stream) = indirect.object else {
        return Err(unreadable(
            "the cross-reference section is neither a table nor a stream",
        ));
    };
    let start = stream.data.start;
    let length = stream
        .dict
        .get(b"Length")
        .and_then(Object::as_i64)
        .map_err(|_| unreadable("the cross-reference stream carries no direct /Length"))?;
    let length = usize::try_from(length)
        .map_err(|_| unreadable("the cross-reference stream length is negative"))?;
    let end = start
        .checked_add(length)
        .filter(|end| *end <= bytes.len())
        .ok_or_else(|| unreadable("the cross-reference stream runs past the end of the file"))?;
    let data = decode_data(&stream.dict, &bytes[start..end])
        .map_err(|error| unreadable(error.detail()))?;
    let entries = stream_entries(&stream.dict, &data)?;
    Ok(Section {
        entries,
        trailer: stream.dict,
    })
}

/// Decode the `/W` and `/Index` rows of a cross-reference stream.
fn stream_entries(dict: &Dict, data: &[u8]) -> Result<Vec<(u32, Entry)>, SourceError> {
    let widths: Vec<i64> = dict
        .get(b"W")
        .and_then(Object::as_array)
        .map_err(|error| unreadable(error.to_string()))?
        .iter()
        .map(|object| object.as_i64())
        .collect::<Result<_, _>>()
        .map_err(|error| unreadable(error.to_string()))?;
    if widths.len() < 3 {
        return Err(unreadable(
            "the cross-reference stream /W has fewer than three fields",
        ));
    }
    let mut field = [0usize; 3];
    for (index, width) in widths[..3].iter().enumerate() {
        field[index] = usize::try_from(*width)
            .ok()
            .filter(|width| *width <= 8)
            .ok_or_else(|| unreadable("a cross-reference stream field width is outside 0-8"))?;
    }
    let size = dict
        .get(b"Size")
        .and_then(Object::as_i64)
        .map_err(|error| unreadable(error.to_string()))?;
    let index: Vec<i64> = match dict.get(b"Index") {
        Ok(object) => object
            .as_array()
            .map_err(|error| unreadable(error.to_string()))?
            .iter()
            .map(|object| object.as_i64())
            .collect::<Result<_, _>>()
            .map_err(|error| unreadable(error.to_string()))?,
        Err(_) => vec![0, size],
    };
    if !index.len().is_multiple_of(2) {
        return Err(unreadable(
            "the cross-reference stream /Index has an odd length",
        ));
    }
    let row = field[0] + field[1] + field[2];
    if row == 0 {
        return Err(unreadable("the cross-reference stream has no row bytes"));
    }
    let mut entries = Vec::new();
    let mut position = 0usize;
    for pair in index.chunks(2) {
        let start = u32::try_from(pair[0])
            .map_err(|_| unreadable("a cross-reference subsection starts out of range"))?;
        let count = u32::try_from(pair[1])
            .map_err(|_| unreadable("a cross-reference subsection is negative"))?;
        for number in 0..count {
            let end = position
                .checked_add(row)
                .filter(|end| *end <= data.len())
                .ok_or_else(|| unreadable("a cross-reference stream row is cut short"))?;
            let row_data = &data[position..end];
            position = end;
            let kind = if field[0] == 0 {
                1
            } else {
                read_field(&row_data[..field[0]])?
            };
            let second = read_field(&row_data[field[0]..field[0] + field[1]])?;
            let third = read_field(&row_data[field[0] + field[1]..])?;
            let entry = match kind {
                1 => Entry::Uncompressed {
                    offset: usize::try_from(second)
                        .map_err(|_| unreadable("a cross-reference offset is out of range"))?,
                },
                2 => Entry::Compressed {
                    container: u32::try_from(second)
                        .map_err(|_| unreadable("an object-stream number is out of range"))?,
                    index: u16::try_from(third)
                        .map_err(|_| unreadable("an object-stream index is out of range"))?,
                },
                0 => Entry::Free,
                _ => continue,
            };
            entries.push((start.saturating_add(number), entry));
        }
    }
    Ok(entries)
}

/// Read one big-endian field of at most eight bytes.
fn read_field(bytes: &[u8]) -> Result<u64, SourceError> {
    let mut value = 0u64;
    for byte in bytes {
        value = (value << 8) | u64::from(*byte);
    }
    Ok(value)
}
