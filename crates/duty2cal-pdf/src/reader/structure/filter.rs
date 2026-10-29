// SPDX-License-Identifier: MPL-2.0
//! The stream filter subset: `FlateDecode`/`Fl` through `miniz_oxide`, with the
//! PNG predictors 10–15 applied and every other predictor value passed through
//! unchanged.
//!
//! The decoder mirrors the previous layer's behaviour where it is observable:
//! the predictor is read from a direct `/DecodeParms` dictionary, `/Columns`,
//! `/Colors` and `/BitsPerComponent` default as before, a zlib stream whose
//! checksum is corrupt keeps its decoded bytes, and a raw-deflate stream behind
//! a zlib header is recovered by retrying from byte 2 when zlib produced none.

use duty2cal_core::application::{Construct, SourceError};
use miniz_oxide::inflate::stream::{inflate, InflateState};
use miniz_oxide::{DataFormat, MZFlush, MZStatus};

use super::{Dict, Document, Object, Stream};

/// The most bytes one decoded stream may produce.
const MAX_DECODED: usize = 64 * 1024 * 1024;

/// Why a stream's bytes could not be decoded.
pub(crate) enum DecodeError {
    /// A filter outside the frozen subset.
    Unsupported(String),
    /// A malformed or oversized compressed stream.
    Malformed(String),
}

impl DecodeError {
    /// The detail clause the unreadable refusal carries.
    pub(crate) fn detail(self) -> String {
        match self {
            Self::Unsupported(name) => format!("unsupported stream filter {name}"),
            Self::Malformed(detail) => detail,
        }
    }
}

/// Decode one stream under the frozen filter subset, refusing an unsupported
/// filter by name.
pub(crate) fn decompress_stream(
    document: &Document,
    stream: &Stream,
    page: Option<usize>,
) -> Result<Vec<u8>, SourceError> {
    let data = document.stream_bytes(stream);
    match decode_data(&stream.dict, data) {
        Ok(decoded) => Ok(decoded),
        Err(DecodeError::Unsupported(name)) => Err(SourceError::UnsupportedConstruct {
            construct: Construct::StreamFilter(name),
            page,
        }),
        Err(error) => Err(SourceError::Unreadable(error.detail())),
    }
}

/// Decode stream bytes without naming a page, for a cross-reference or object
/// stream. A failure here is a structural one.
pub(crate) fn decode_data(dict: &Dict, data: &[u8]) -> Result<Vec<u8>, DecodeError> {
    let Some(names) = filter_names(dict) else {
        return Ok(data.to_vec());
    };
    if names.is_empty() {
        return Ok(Vec::new());
    }
    let parms = decode_parms(dict);
    let mut current: Option<Vec<u8>> = None;
    for name in names {
        let input = current.as_deref().unwrap_or(data);
        match name.as_slice() {
            b"FlateDecode" | b"Fl" => {
                current = Some(decode_flate(input, parms).map_err(DecodeError::Malformed)?)
            }
            other => {
                return Err(DecodeError::Unsupported(
                    String::from_utf8_lossy(other).into_owned(),
                ))
            }
        }
    }
    Ok(current.unwrap_or_default())
}

/// The stream's filters in decoding order, or `None` when the stream carries
/// none and its bytes are already the content.
fn filter_names(dict: &Dict) -> Option<Vec<Vec<u8>>> {
    match dict.get(b"Filter").ok()? {
        Object::Name(name) => Some(vec![name.clone()]),
        Object::Array(items) => items
            .iter()
            .map(|item| item.as_name().ok().map(<[u8]>::to_vec))
            .collect(),
        _ => None,
    }
}

/// The direct `/DecodeParms` dictionary, as the previous layer reads it.
fn decode_parms(dict: &Dict) -> Option<&Dict> {
    dict.get(b"DecodeParms").ok().and_then(|o| o.as_dict().ok())
}

/// A failed or oversized decode, with the bytes produced before the failure.
struct InflateFailure {
    detail: String,
    partial: Vec<u8>,
    too_large: bool,
}

/// Decode a zlib (`FlateDecode`) stream.
fn decode_flate(input: &[u8], parms: Option<&Dict>) -> Result<Vec<u8>, String> {
    let decoded = match inflate_data(input, DataFormat::Zlib) {
        Ok(output) => output,
        Err(failure) if failure.too_large => return Err(failure.detail),
        Err(failure) if failure.partial.is_empty() => {
            if input.len() <= 2 {
                return Err(failure.detail);
            }
            match inflate_data(&input[2..], DataFormat::Raw) {
                Ok(output) => output,
                Err(raw) if raw.too_large => return Err(raw.detail),
                Err(raw) if raw.partial.is_empty() => return Err(raw.detail),
                Err(raw) => raw.partial,
            }
        }
        Err(failure) => failure.partial,
    };
    match parms {
        Some(parms) => apply_predictor(decoded, parms),
        None => Ok(decoded),
    }
}

/// Inflate `input` in `format`, bounded by [`MAX_DECODED`].
fn inflate_data(input: &[u8], format: DataFormat) -> Result<Vec<u8>, InflateFailure> {
    let mut state = InflateState::new_boxed(format);
    let mut output = Vec::new();
    let mut buffer = vec![0u8; 16 * 1024];
    let mut consumed = 0usize;
    loop {
        let result = inflate(&mut state, &input[consumed..], &mut buffer, MZFlush::None);
        consumed += result.bytes_consumed;
        if output.len().saturating_add(result.bytes_written) > MAX_DECODED {
            return Err(InflateFailure {
                detail: "the decoded stream exceeds the 64 MiB bound".to_string(),
                partial: Vec::new(),
                too_large: true,
            });
        }
        output.extend_from_slice(&buffer[..result.bytes_written]);
        match result.status {
            Ok(MZStatus::StreamEnd) => return Ok(output),
            Ok(_) => {
                // No progress means the input ended before the stream did; a
                // stream that merely drained its input may still hold decoded
                // bytes internally, which the next call flushes.
                if result.bytes_consumed == 0 && result.bytes_written == 0 {
                    return Err(InflateFailure {
                        detail: "the compressed stream ends before its data".to_string(),
                        partial: output,
                        too_large: false,
                    });
                }
            }
            Err(_) => {
                return Err(InflateFailure {
                    detail: "the compressed stream is malformed".to_string(),
                    partial: output,
                    too_large: false,
                })
            }
        }
    }
}

/// Apply `/DecodeParms /Predictor` when it names a PNG predictor.
fn apply_predictor(data: Vec<u8>, parms: &Dict) -> Result<Vec<u8>, String> {
    let predictor = parms
        .get(b"Predictor")
        .ok()
        .and_then(|object| object.as_i64().ok())
        .unwrap_or(1);
    if !(10..=15).contains(&predictor) {
        return Ok(data);
    }
    let columns = parms
        .get(b"Columns")
        .ok()
        .and_then(|object| object.as_i64().ok())
        .unwrap_or(1)
        .max(1) as usize;
    let colors = parms
        .get(b"Colors")
        .ok()
        .and_then(|object| object.as_i64().ok())
        .unwrap_or(1)
        .max(1) as usize;
    let bits = parms
        .get(b"BitsPerComponent")
        .ok()
        .and_then(|object| object.as_i64().ok())
        .unwrap_or(8)
        .max(8) as usize;
    decode_png_frame(&data, colors * bits / 8, columns)
}

/// Reverse the PNG row filters over `data`, `columns` bytes per row.
fn decode_png_frame(
    data: &[u8],
    bytes_per_pixel: usize,
    columns: usize,
) -> Result<Vec<u8>, String> {
    let bytes_per_row = bytes_per_pixel * columns;
    if bytes_per_row > MAX_DECODED {
        return Err("the decoded stream exceeds the 64 MiB bound".to_string());
    }
    let mut previous = vec![0u8; bytes_per_row];
    let mut current = vec![0u8; bytes_per_row];
    let mut decoded = Vec::new();
    let mut position = 0usize;
    while position < data.len() {
        let filter = data[position];
        if filter > 4 {
            return Err(format!("invalid PNG filter type ({filter})"));
        }
        position += 1;
        let row = data
            .get(position..position + bytes_per_row)
            .ok_or_else(|| "a PNG predictor row is shorter than its columns".to_string())?;
        current.copy_from_slice(row);
        position += bytes_per_row;
        decode_row(filter, bytes_per_pixel, &previous, &mut current);
        if decoded.len().saturating_add(bytes_per_row) > MAX_DECODED {
            return Err("the decoded stream exceeds the 64 MiB bound".to_string());
        }
        decoded.extend_from_slice(&current);
        std::mem::swap(&mut previous, &mut current);
    }
    Ok(decoded)
}

/// Reverse one PNG row filter, in the previous layer's arithmetic.
fn decode_row(filter: u8, bytes_per_pixel: usize, previous: &[u8], current: &mut [u8]) {
    let length = current.len();
    let bpp = bytes_per_pixel.min(length);
    match filter {
        0 => {}
        1 => {
            for index in bpp..length {
                current[index] = current[index].wrapping_add(current[index - bpp]);
            }
        }
        2 => {
            for index in 0..length {
                current[index] = current[index].wrapping_add(previous[index]);
            }
        }
        3 => {
            for index in 0..bpp {
                current[index] = current[index].wrapping_add(previous[index] / 2);
            }
            for index in bpp..length {
                current[index] = current[index].wrapping_add(
                    (i16::from(current[index - bpp]) + i16::from(previous[index]) / 2) as u8,
                );
            }
        }
        4 => {
            for index in 0..bpp {
                current[index] = current[index].wrapping_add(paeth(0, previous[index], 0));
            }
            for index in bpp..length {
                current[index] = current[index].wrapping_add(paeth(
                    current[index - bpp],
                    previous[index],
                    previous[index - bpp],
                ));
            }
        }
        _ => {}
    }
}

/// The PNG Paeth predictor.
fn paeth(left: u8, above: u8, upper_left: u8) -> u8 {
    let estimate = i16::from(left) + i16::from(above) - i16::from(upper_left);
    let to_left = (estimate - i16::from(left)).abs();
    let to_above = (estimate - i16::from(above)).abs();
    let to_upper_left = (estimate - i16::from(upper_left)).abs();
    if to_left <= to_above && to_left <= to_upper_left {
        left
    } else if to_above <= to_upper_left {
        above
    } else {
        upper_left
    }
}
