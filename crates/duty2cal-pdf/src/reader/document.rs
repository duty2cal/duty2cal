// SPDX-License-Identifier: MPL-2.0
//! Encryption refusal, page order, page boxes, decoded page content and
//! `/Resources` fonts, read through the project's own file-structure layer.

use std::collections::{BTreeMap, BTreeSet};

use duty2cal_core::application::{Construct, SourceError};

use super::content::interpret;
pub(super) use super::structure::decompress_stream;
use super::structure::{self, Dict, Document, ObjId, Object};
use super::words::extract_words;
use super::{ExtractedDocument, PageWords};

/// The deepest page-tree nesting the walk accepts.
const MAX_PAGE_TREE_DEPTH: usize = 256;

/// Read a supplied PDF into positioned words per page.
pub fn extract(bytes: &[u8]) -> Result<ExtractedDocument, SourceError> {
    let document = match structure::load(bytes) {
        Ok(document) => document,
        // An encrypted file the loader cannot open is still an encrypted
        // document, not an unreadable one: the trailer names the encryption.
        Err(_) if bytes.windows(8).any(|window| window == b"/Encrypt") => {
            return Err(SourceError::UnsupportedConstruct {
                construct: Construct::EncryptedDocument,
                page: None,
            });
        }
        Err(error) => return Err(error),
    };
    if document.is_encrypted() {
        return Err(SourceError::UnsupportedConstruct {
            construct: Construct::EncryptedDocument,
            page: None,
        });
    }

    let catalog = catalog(&document)?;
    let pages_id = catalog
        .get(b"Pages")
        .and_then(Object::as_reference)
        .map_err(|error| SourceError::Unreadable(error.to_string()))?;
    let mut page_ids = Vec::new();
    let mut seen = BTreeSet::new();
    collect_pages(&document, pages_id, 0, &mut seen, &mut page_ids)?;

    let mut extracted = Vec::with_capacity(page_ids.len());
    for (index, page_id) in page_ids.iter().enumerate() {
        let number = u32::try_from(index + 1)
            .map_err(|_| SourceError::Unreadable("the page tree is too long".to_string()))?;
        let page = document
            .get_dictionary(*page_id)
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
        let page_height = media[3] - media[1];
        if let Some(crop) = page_array(&document, page, b"CropBox") {
            if crop.iter().zip(&media).any(|(a, b)| a != b) {
                return Err(refusal(Construct::PageBox, number));
            }
        }

        let content = page_content(&document, page, number)?;
        let (direct, inherited) = page_resources(&document, page)?;
        let resources = direct.or_else(|| {
            inherited
                .first()
                .and_then(|id| document.get_dictionary(*id).ok())
        });
        let fonts = page_fonts(&document, direct, &inherited);

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

/// The document's catalog dictionary.
fn catalog<'a>(document: &'a Document<'_>) -> Result<&'a Dict, SourceError> {
    let root = document
        .trailer()
        .get(b"Root")
        .and_then(Object::as_reference)
        .map_err(|_| SourceError::Unreadable("the trailer carries no /Root".to_string()))?;
    document
        .get_dictionary(root)
        .map_err(|error| SourceError::Unreadable(error.to_string()))
}

/// Walk the page tree in kid order, refusing a cycle or a repeated node.
fn collect_pages(
    document: &Document,
    id: ObjId,
    depth: usize,
    seen: &mut BTreeSet<ObjId>,
    pages: &mut Vec<ObjId>,
) -> Result<(), SourceError> {
    if depth > MAX_PAGE_TREE_DEPTH {
        return Err(SourceError::Unreadable(
            "the page tree is nested too deeply".to_string(),
        ));
    }
    if !seen.insert(id) {
        return Err(SourceError::Unreadable(
            "the page tree repeats a page node".to_string(),
        ));
    }
    let node = document
        .get_dictionary(id)
        .map_err(|error| SourceError::Unreadable(error.to_string()))?;
    let is_branch = match node.get(b"Type").and_then(Object::as_name) {
        Ok(name) => name == b"Pages",
        Err(_) => node.get(b"Kids").is_ok(),
    };
    if !is_branch {
        pages.push(id);
        return Ok(());
    }
    let kids = node
        .get(b"Kids")
        .and_then(Object::as_array)
        .map_err(|error| SourceError::Unreadable(error.to_string()))?;
    for kid in kids {
        let kid = kid.as_reference().map_err(|_| {
            SourceError::Unreadable("a page tree kid is not a reference".to_string())
        })?;
        collect_pages(document, kid, depth + 1, seen, pages)?;
    }
    Ok(())
}

/// The page's direct resource dictionary and every `/Resources` reference up the
/// `/Parent` chain, nearest first.
fn page_resources<'a>(
    document: &'a Document,
    page: &'a Dict,
) -> Result<(Option<&'a Dict>, Vec<ObjId>), SourceError> {
    let direct = page.get(b"Resources").ok().and_then(|o| o.as_dict().ok());
    let mut ids = Vec::new();
    let mut current = page;
    let mut seen = BTreeSet::new();
    loop {
        if let Ok(id) = current.get(b"Resources").and_then(Object::as_reference) {
            ids.push(id);
        }
        let Ok(parent) = current.get(b"Parent").and_then(Object::as_reference) else {
            break;
        };
        if !seen.insert(parent) {
            return Err(SourceError::Unreadable(
                "the page tree carries a reference cycle".to_string(),
            ));
        }
        current = document
            .get_dictionary(parent)
            .map_err(|error| SourceError::Unreadable(error.to_string()))?;
    }
    Ok((direct, ids))
}

/// Every `/Font` entry a page reaches, the nearest definition winning a name.
fn page_fonts<'a>(
    document: &'a Document,
    direct: Option<&'a Dict>,
    inherited: &[ObjId],
) -> BTreeMap<Vec<u8>, &'a Dict> {
    let mut fonts = BTreeMap::new();
    if let Some(resources) = direct {
        collect_fonts(document, resources, &mut fonts);
    }
    for id in inherited {
        if let Ok(resources) = document.get_dictionary(*id) {
            collect_fonts(document, resources, &mut fonts);
        }
    }
    fonts
}

/// Collect one resource dictionary's fonts.
fn collect_fonts<'a>(
    document: &'a Document,
    resources: &'a Dict,
    fonts: &mut BTreeMap<Vec<u8>, &'a Dict>,
) {
    let font_dict = match resources.get(b"Font") {
        Ok(Object::Reference(id)) => document.get_object(*id).and_then(Object::as_dict).ok(),
        Ok(Object::Dictionary(dict)) => Some(dict),
        _ => None,
    };
    let Some(font_dict) = font_dict else {
        return;
    };
    for (name, value) in font_dict.iter() {
        if fonts.contains_key(name) {
            continue;
        }
        let font = match value {
            Object::Reference(id) => document.get_dictionary(*id).ok(),
            Object::Dictionary(dict) => Some(dict),
            _ => None,
        };
        if let Some(font) = font {
            fonts.insert(name.clone(), font);
        }
    }
}

/// The page's content stream ids, in `/Contents` order.
fn page_contents(document: &Document, page: &Dict) -> Vec<ObjId> {
    let mut streams = Vec::new();
    let Ok(mut contents) = page.get(b"Contents") else {
        return streams;
    };
    for _ in 0..10 {
        match contents {
            Object::Reference(id) => match document.get_object(*id) {
                Ok(Object::Stream(_)) | Err(_) => {
                    streams.push(*id);
                    return streams;
                }
                Ok(object) => contents = object,
            },
            Object::Array(items) => {
                for item in items {
                    if let Ok(id) = item.as_reference() {
                        streams.push(id);
                    }
                }
                return streams;
            }
            _ => return streams,
        }
    }
    streams
}

/// Decode every content stream of a page, refusing an unsupported filter.
fn page_content(
    document: &Document,
    page: &Dict,
    page_number: u32,
) -> Result<Vec<u8>, SourceError> {
    let mut content = Vec::new();
    for id in page_contents(document, page) {
        let stream = document
            .get_object(id)
            .and_then(Object::as_stream)
            .map_err(|error| SourceError::Unreadable(error.to_string()))?;
        content.extend(decompress_stream(
            document,
            stream,
            Some(page_number as usize),
        )?);
        content.push(b'\n');
    }
    Ok(content)
}

/// A page box as four numbers.
fn page_array(document: &Document, page: &Dict, key: &[u8]) -> Option<Vec<f64>> {
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
    resolve(document, object)?.as_f64().ok()
}

/// A named construct refusal for a page.
fn refusal(construct: Construct, page: u32) -> SourceError {
    SourceError::UnsupportedConstruct {
        construct,
        page: Some(page as usize),
    }
}
