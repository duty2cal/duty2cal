// SPDX-License-Identifier: MPL-2.0
//! `lopdf` load, encryption refusal, page order, `/MediaBox` height, decoded
//! page content and `/Resources` fonts.

use duty2cal_core::application::{Construct, SourceError};
use lopdf::{Dictionary, Document, Object, Stream};

use super::content::interpret;
use super::words::extract_words;
use super::{ExtractedDocument, PageWords};

/// Read a supplied PDF into positioned words per page.
pub fn extract(bytes: &[u8]) -> Result<ExtractedDocument, SourceError> {
    let document =
        Document::load_mem(bytes).map_err(|error| SourceError::Unreadable(error.to_string()))?;
    if document.is_encrypted() {
        return Err(SourceError::UnsupportedConstruct {
            construct: Construct::EncryptedDocument,
            page: None,
        });
    }

    let pages = document.get_pages();
    let mut extracted = Vec::with_capacity(pages.len());
    for (number, page_id) in pages {
        let page = document
            .get_dictionary(page_id)
            .map_err(|error| SourceError::Unreadable(error.to_string()))?;

        if let Some(rotation) = page
            .get(b"Rotate")
            .ok()
            .and_then(|object| numeric(&document, object))
        {
            if rotation != 0.0 {
                return Err(refusal(Construct::PageRotation, number));
            }
        }

        let media = page_array(&document, page, b"MediaBox")
            .ok_or_else(|| SourceError::Unreadable("the page carries no media box".to_string()))?;
        if media.len() < 4 {
            return Err(SourceError::Unreadable(
                "the page media box is not a rectangle".to_string(),
            ));
        }
        let page_height = media[3] - media[1];
        if let Some(crop) = page_array(&document, page, b"CropBox") {
            if crop.len() != media.len() || crop.iter().zip(&media).any(|(a, b)| a != b) {
                return Err(refusal(Construct::PageBox, number));
            }
        }

        let content = page_content(&document, page_id, number)?;
        let (direct, inherited) = document
            .get_page_resources(page_id)
            .map_err(|error| SourceError::Unreadable(error.to_string()))?;
        let resources = direct.or_else(|| {
            inherited
                .first()
                .and_then(|id| document.get_dictionary(*id).ok())
        });
        let fonts = document
            .get_page_fonts(page_id)
            .map_err(|error| SourceError::Unreadable(error.to_string()))?;

        let chars = interpret(
            &document,
            number as usize,
            page_height,
            &content,
            resources,
            &fonts,
        )?;
        let words = extract_words(chars);
        extracted.push(PageWords {
            page: number as usize,
            words,
        });
    }

    Ok(ExtractedDocument { pages: extracted })
}

/// Decode every content stream of a page, refusing an unsupported filter.
fn page_content(
    document: &Document,
    page_id: lopdf::ObjectId,
    page: u32,
) -> Result<Vec<u8>, SourceError> {
    let mut content = Vec::new();
    for id in document.get_page_contents(page_id) {
        let stream = document
            .get_object(id)
            .and_then(Object::as_stream)
            .map_err(|error| SourceError::Unreadable(error.to_string()))?;
        content.extend(decompress_stream(document, stream, Some(page as usize))?);
        content.push(b'\n');
    }
    Ok(content)
}

/// Decode a stream under the frozen filter subset.
pub(super) fn decompress_stream(
    _document: &Document,
    stream: &Stream,
    page: Option<usize>,
) -> Result<Vec<u8>, SourceError> {
    if let Some(filter) = unsupported_filter(&stream.dict) {
        return Err(SourceError::UnsupportedConstruct {
            construct: Construct::StreamFilter(filter),
            page,
        });
    }
    stream
        .decompressed_content()
        .map_err(|error| SourceError::Unreadable(error.to_string()))
}

/// The first filter outside the subset, if any.
fn unsupported_filter(dict: &Dictionary) -> Option<String> {
    let filter = dict.get(b"Filter").ok()?;
    let names: Vec<&[u8]> = match filter {
        Object::Name(name) => vec![name.as_slice()],
        Object::Array(items) => items
            .iter()
            .filter_map(|item| item.as_name().ok())
            .collect(),
        _ => return None,
    };
    for name in names {
        if name != b"FlateDecode" && name != b"Fl" {
            return Some(String::from_utf8_lossy(name).into_owned());
        }
    }
    None
}

/// A page box as four numbers.
fn page_array(document: &Document, page: &Dictionary, key: &[u8]) -> Option<Vec<f64>> {
    let object = page.get(key).ok()?;
    let object = resolve(document, object)?;
    let array = object.as_array().ok()?;
    if array.len() != 4 {
        return None;
    }
    array
        .iter()
        .map(|item| numeric(document, item))
        .collect::<Option<Vec<f64>>>()
}

/// Resolve one reference level.
fn resolve<'a>(document: &'a Document, object: &'a Object) -> Option<&'a Object> {
    match object {
        Object::Reference(id) => document.get_object(*id).ok(),
        other => Some(other),
    }
}

/// A numeric object.
fn numeric(document: &Document, object: &Object) -> Option<f64> {
    match resolve(document, object)? {
        Object::Integer(value) => Some(*value as f64),
        Object::Real(value) => Some(f64::from(*value)),
        _ => None,
    }
}

/// A named construct refusal for a page.
fn refusal(construct: Construct, page: u32) -> SourceError {
    SourceError::UnsupportedConstruct {
        construct,
        page: Some(page as usize),
    }
}
