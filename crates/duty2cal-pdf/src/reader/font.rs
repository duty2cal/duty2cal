// SPDX-License-Identifier: MPL-2.0
//! Type0/Identity-H font support: `/W` and `/DW` advances, the descriptor's
//! descent, and the per-font `ToUnicode` map.

use std::collections::HashMap;

use super::structure::{Dict, Document, Object};
use duty2cal_core::application::{Construct, SourceError};

use super::cmap::ToUnicode;
use super::document::decompress_stream;

/// The default glyph advance when `/DW` is absent, in glyph space units.
const DEFAULT_WIDTH: f64 = 1000.0;

/// One supported Type0/Identity-H font.
#[derive(Debug, Clone)]
pub struct Font {
    widths: HashMap<u32, f64>,
    default_width: f64,
    descent: f64,
    to_unicode: ToUnicode,
}

impl Font {
    /// Read a font dictionary, refusing anything outside the frozen subset.
    pub fn parse(document: &Document, font: &Dict, page: usize) -> Result<Self, SourceError> {
        let subtype = name(document, font, b"Subtype");
        match subtype.as_deref() {
            Some("Type0") => {}
            Some("Type3") | Some("Type1") | Some("TrueType") | Some("MMType1") => {
                return Err(refusal(Construct::Type3Font, page));
            }
            _ => return Err(refusal(Construct::Type3Font, page)),
        }
        match name(document, font, b"Encoding").as_deref() {
            Some("Identity-H") => {}
            _ => return Err(refusal(Construct::NonIdentityEncoding, page)),
        }

        let descendants = font
            .get(b"DescendantFonts")
            .ok()
            .and_then(|object| resolve(document, object))
            .and_then(|object| object.as_array().ok())
            .ok_or_else(|| unreadable("the Type0 font carries no descendant font"))?;
        let descendant = descendants
            .first()
            .and_then(|object| resolve(document, object))
            .and_then(|object| object.as_dict().ok())
            .ok_or_else(|| unreadable("the Type0 font carries no descendant font"))?;

        match name(document, descendant, b"Subtype").as_deref() {
            Some("CIDFontType2") => {}
            _ => return Err(refusal(Construct::NonIdentityEncoding, page)),
        }

        let widths = parse_widths(document, descendant);
        let default_width = number(document, descendant, b"DW").unwrap_or(DEFAULT_WIDTH);

        let descriptor = descendant
            .get(b"FontDescriptor")
            .ok()
            .and_then(|object| resolve(document, object))
            .and_then(|object| object.as_dict().ok())
            .ok_or_else(|| unreadable("the CID font carries no font descriptor"))?;
        let descent = number(document, descriptor, b"Descent")
            .ok_or_else(|| refusal(Construct::MissingDescent, page))?;

        let to_unicode_object = font
            .get(b"ToUnicode")
            .ok()
            .and_then(|object| resolve(document, object))
            .ok_or_else(|| refusal(Construct::MissingToUnicode, page))?;
        let stream = to_unicode_object
            .as_stream()
            .map_err(|_| refusal(Construct::MissingToUnicode, page))?;
        let bytes = decompress_stream(document, stream, Some(page))?;
        let to_unicode = ToUnicode::parse(&bytes);
        if to_unicode.is_empty() {
            return Err(refusal(Construct::MissingToUnicode, page));
        }

        Ok(Self {
            widths,
            default_width,
            descent,
            to_unicode,
        })
    }

    /// The glyph advance for a code, in glyph space units (1/1000 text space).
    pub fn width(&self, code: u32) -> f64 {
        self.widths
            .get(&code)
            .copied()
            .unwrap_or(self.default_width)
    }

    /// The descriptor descent, in glyph space units (negative below baseline).
    pub fn descent(&self) -> f64 {
        self.descent
    }

    /// The Unicode text for a glyph code, or a named refusal.
    pub fn unicode(
        &self,
        code: u32,
        page: usize,
    ) -> Result<std::borrow::Cow<'_, str>, SourceError> {
        self.to_unicode
            .get(code)
            .ok_or_else(|| refusal(Construct::UnmappedGlyph, page))
    }
}

/// Parse the `/W` array into per-CID advances.
fn parse_widths(document: &Document, descendant: &Dict) -> HashMap<u32, f64> {
    let mut widths = HashMap::new();
    let Ok(object) = descendant.get(b"W") else {
        return widths;
    };
    let Some(object) = resolve(document, object) else {
        return widths;
    };
    let Ok(entries) = object.as_array() else {
        return widths;
    };
    let mut index = 0;
    while index < entries.len() {
        let Some(start) = resolve(document, &entries[index]).and_then(as_u32) else {
            index += 1;
            continue;
        };
        index += 1;
        let Some(next) = entries.get(index).and_then(|o| resolve(document, o)) else {
            break;
        };
        if let Ok(list) = next.as_array() {
            for (offset, value) in list.iter().enumerate() {
                if let Some(width) = resolve(document, value).and_then(as_f64) {
                    widths.insert(start + offset as u32, width);
                }
            }
            index += 1;
        } else if let Some(end) = as_u32(next) {
            index += 1;
            if let Some(width) = entries
                .get(index)
                .and_then(|o| resolve(document, o))
                .and_then(as_f64)
            {
                if end >= start && end - start < 65_536 {
                    for code in start..=end {
                        widths.insert(code, width);
                    }
                }
            }
            index += 1;
        } else {
            index += 1;
        }
    }
    widths
}

/// Resolve one reference level.
pub(super) fn resolve<'a>(document: &'a Document, object: &'a Object) -> Option<&'a Object> {
    match object {
        Object::Reference(id) => document.get_object(*id).ok(),
        other => Some(other),
    }
}

/// A dictionary name value.
fn name(document: &Document, dict: &Dict, key: &[u8]) -> Option<String> {
    let object = dict.get(key).ok()?;
    let object = resolve(document, object)?;
    object.as_name().ok().map(|name| {
        String::from_utf8_lossy(name)
            .trim_start_matches('/')
            .to_string()
    })
}

/// A dictionary number value.
fn number(document: &Document, dict: &Dict, key: &[u8]) -> Option<f64> {
    dict.get(key)
        .ok()
        .and_then(|object| resolve(document, object))
        .and_then(as_f64)
}

/// A numeric object as `f64`.
pub(super) fn as_f64(object: &Object) -> Option<f64> {
    match object {
        Object::Integer(value) => Some(*value as f64),
        Object::Real(value) => Some(*value),
        _ => None,
    }
}

/// An integer object as `u32`.
pub(super) fn as_u32(object: &Object) -> Option<u32> {
    match object {
        Object::Integer(value) => u32::try_from(*value).ok(),
        _ => None,
    }
}

/// The named construct refusal for a page.
pub(super) fn refusal(construct: Construct, page: usize) -> SourceError {
    SourceError::UnsupportedConstruct {
        construct,
        page: Some(page),
    }
}

/// The unreadable refusal for a structural problem.
pub(super) fn unreadable(detail: impl Into<String>) -> SourceError {
    SourceError::Unreadable(detail.into())
}
