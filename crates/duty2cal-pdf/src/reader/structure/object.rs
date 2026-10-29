// SPDX-License-Identifier: MPL-2.0
//! The object vocabulary the interpretation modules read.
//!
//! The accessors keep the shape the previous layer exposed: a typed value or a
//! displayable error. A stream body is a byte range into the supplied document,
//! never a copy.

use std::collections::BTreeMap;
use std::fmt;
use std::ops::Range;

/// An indirect object identifier: object number and generation.
pub(crate) type ObjId = (u32, u16);

/// A typed-object access failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Error(String);

impl Error {
    /// A named type mismatch.
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self(detail.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

/// A parsed PDF object.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Object {
    /// The `null` object.
    Null,
    /// A boolean.
    Boolean(bool),
    /// An integer.
    Integer(i64),
    /// A real number.
    Real(f64),
    /// A name, without its leading slash.
    Name(Vec<u8>),
    /// A string's bytes.
    String(Vec<u8>),
    /// An array.
    Array(Vec<Object>),
    /// A dictionary.
    Dictionary(Dict),
    /// A stream: its dictionary and its body's range in the document.
    Stream(Stream),
    /// A reference to an indirect object.
    Reference(ObjId),
}

impl Object {
    /// The dictionary value.
    pub(crate) fn as_dict(&self) -> Result<&Dict, Error> {
        match self {
            Self::Dictionary(dict) => Ok(dict),
            _ => Err(Error::new("the object is not a dictionary")),
        }
    }

    /// The array value.
    pub(crate) fn as_array(&self) -> Result<&Vec<Object>, Error> {
        match self {
            Self::Array(array) => Ok(array),
            _ => Err(Error::new("the object is not an array")),
        }
    }

    /// The name value, without its leading slash.
    pub(crate) fn as_name(&self) -> Result<&[u8], Error> {
        match self {
            Self::Name(name) => Ok(name),
            _ => Err(Error::new("the object is not a name")),
        }
    }

    /// The stream value.
    pub(crate) fn as_stream(&self) -> Result<&Stream, Error> {
        match self {
            Self::Stream(stream) => Ok(stream),
            _ => Err(Error::new("the object is not a stream")),
        }
    }

    /// The reference's target.
    pub(crate) fn as_reference(&self) -> Result<ObjId, Error> {
        match self {
            Self::Reference(id) => Ok(*id),
            _ => Err(Error::new("the object is not a reference")),
        }
    }

    /// The integer value.
    pub(crate) fn as_i64(&self) -> Result<i64, Error> {
        match self {
            Self::Integer(value) => Ok(*value),
            _ => Err(Error::new("the object is not an integer")),
        }
    }

    /// The numeric value, an integer widened.
    pub(crate) fn as_f64(&self) -> Result<f64, Error> {
        match self {
            Self::Integer(value) => Ok(*value as f64),
            Self::Real(value) => Ok(*value),
            _ => Err(Error::new("the object is not a number")),
        }
    }
}

/// A dictionary: names to objects.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct Dict(BTreeMap<Vec<u8>, Object>);

impl Dict {
    /// The value for `key`.
    pub(crate) fn get(&self, key: &[u8]) -> Result<&Object, Error> {
        self.0.get(key).ok_or_else(|| {
            Error::new(format!(
                "the dictionary carries no /{}",
                String::from_utf8_lossy(key)
            ))
        })
    }

    /// Every entry, in name order.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&Vec<u8>, &Object)> {
        self.0.iter()
    }

    /// Insert one entry, replacing any previous value.
    pub(crate) fn insert(&mut self, key: Vec<u8>, value: Object) {
        self.0.insert(key, value);
    }

    /// Insert one entry only when the key is absent.
    pub(crate) fn insert_if_absent(&mut self, key: Vec<u8>, value: Object) {
        self.0.entry(key).or_insert(value);
    }
}

/// A stream: the dictionary that describes it and its body's range.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Stream {
    /// The stream dictionary.
    pub dict: Dict,
    /// The body's range in the supplied document bytes.
    pub data: Range<usize>,
}
