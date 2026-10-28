// SPDX-License-Identifier: MPL-2.0
//! `ToUnicode` CMap parsing: `beginbfchar` and `beginbfrange` into a
//! code → Unicode string map.
//!
//! The supported subset needs only the mapping sections a per-font `ToUnicode`
//! stream carries. A malformed section contributes no mappings; a wholly
//! unreadable stream makes the font's map empty, which the font layer reports
//! as a missing CMap.

use std::borrow::Cow;
use std::collections::HashMap;

/// The largest source-code range a single `bfrange` entry may expand to.
const MAX_RANGE: u32 = 65_536;

/// A parsed `ToUnicode` map.
#[derive(Debug, Clone, Default)]
pub struct ToUnicode {
    mappings: HashMap<u32, String>,
    identity: bool,
}

impl ToUnicode {
    /// Parse a `ToUnicode` stream.
    pub fn parse(data: &[u8]) -> Self {
        let tokens = tokenize(data);
        let mut mappings = HashMap::new();
        let mut index = 0;
        while index < tokens.len() {
            match &tokens[index] {
                Token::Keyword(keyword) if keyword == "beginbfchar" => {
                    index += 1;
                    while index < tokens.len() {
                        if matches!(&tokens[index], Token::Keyword(k) if k == "endbfchar") {
                            index += 1;
                            break;
                        }
                        let (Some(source), Some(target)) =
                            (tokens.get(index), tokens.get(index + 1))
                        else {
                            break;
                        };
                        if let (Token::Hex(source), Token::Hex(target)) = (source, target) {
                            if let Some(code) = decode_code(source) {
                                if let Some(text) = decode_text(target) {
                                    mappings.insert(code, text);
                                }
                            }
                        }
                        index += 2;
                    }
                }
                Token::Keyword(keyword) if keyword == "beginbfrange" => {
                    index += 1;
                    while index < tokens.len() {
                        if matches!(&tokens[index], Token::Keyword(k) if k == "endbfrange") {
                            index += 1;
                            break;
                        }
                        let Some(Token::Hex(low)) = tokens.get(index) else {
                            index += 1;
                            continue;
                        };
                        let Some(Token::Hex(high)) = tokens.get(index + 1) else {
                            index += 1;
                            continue;
                        };
                        let (Some(low), Some(high)) = (decode_code(low), decode_code(high)) else {
                            index += 2;
                            continue;
                        };
                        index += 2;
                        match tokens.get(index) {
                            Some(Token::Hex(target)) => {
                                if let Some(base) = decode_text(target) {
                                    insert_range(&mut mappings, low, high, &base);
                                }
                                index += 1;
                            }
                            Some(Token::ArrayStart) => {
                                index += 1;
                                let mut code = low;
                                while let Some(Token::Hex(target)) = tokens.get(index) {
                                    if let Some(text) = decode_text(target) {
                                        mappings.insert(code, text);
                                    }
                                    code = code.saturating_add(1);
                                    index += 1;
                                    if code > high {
                                        break;
                                    }
                                }
                                if matches!(tokens.get(index), Some(Token::ArrayEnd)) {
                                    index += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => index += 1,
            }
        }
        let identity = mappings.is_empty()
            && tokens
                .iter()
                .any(|token| matches!(token, Token::Keyword(keyword) if keyword == "begincidrange" || keyword == "begincidchar"));
        Self { mappings, identity }
    }

    /// The mapped text for a character code.
    ///
    /// A `cidrange`-only CMap is the identity map the producer writes: the
    /// oracle reads the code as a Unicode scalar, so the owned extraction does
    /// the same instead of calling every such font unmapped.
    pub fn get(&self, code: u32) -> Option<Cow<'_, str>> {
        if let Some(text) = self.mappings.get(&code) {
            return Some(Cow::Borrowed(text));
        }
        if self.identity {
            return char::from_u32(code).map(|ch| Cow::Owned(ch.to_string()));
        }
        None
    }

    /// Whether the map carries no mappings at all.
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty() && !self.identity
    }
}

/// Insert `low..=high`, advancing the last code unit of `base` per entry.
fn insert_range(mappings: &mut HashMap<u32, String>, low: u32, high: u32, base: &str) {
    if high < low || high - low >= MAX_RANGE {
        return;
    }
    let mut units: Vec<u16> = base.encode_utf16().collect();
    for offset in 0..=(high - low) {
        if let Some(last) = units.last_mut() {
            *last = last.saturating_add(offset as u16);
        }
        mappings.insert(low + offset, String::from_utf16_lossy(&units));
    }
}

/// Decode a hex token as a character code (big-endian, as many bytes as it has).
fn decode_code(bytes: &[u8]) -> Option<u32> {
    if bytes.is_empty() || bytes.len() > 4 {
        return None;
    }
    Some(
        bytes
            .iter()
            .fold(0, |code, byte| (code << 8) | u32::from(*byte)),
    )
}

/// Decode a hex token as UTF-16BE text.
fn decode_text(bytes: &[u8]) -> Option<String> {
    if bytes.is_empty() {
        return None;
    }
    let mut units = Vec::with_capacity(bytes.len() / 2);
    let (chunks, rest) = bytes.as_chunks::<2>();
    for chunk in chunks {
        units.push(u16::from_be_bytes(*chunk));
    }
    if let [last] = rest {
        units.push(u16::from(*last));
    }
    Some(String::from_utf16_lossy(&units))
}

/// CMap tokens the parser has to distinguish.
#[derive(Debug)]
enum Token {
    Hex(Vec<u8>),
    Keyword(String),
    ArrayStart,
    ArrayEnd,
}

/// Tokenize the CMap enough to read its mapping sections.
fn tokenize(data: &[u8]) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < data.len() {
        match data[index] {
            b'%' => {
                while index < data.len() && data[index] != b'\n' {
                    index += 1;
                }
            }
            byte if byte.is_ascii_whitespace() => index += 1,
            b'<' => {
                if data.get(index + 1) == Some(&b'<') {
                    index += 2;
                    continue;
                }
                let start = index + 1;
                index = start;
                while index < data.len() && data[index] != b'>' {
                    index += 1;
                }
                tokens.push(Token::Hex(hex_bytes(&data[start..index])));
                index = (index + 1).min(data.len());
            }
            b'[' => {
                tokens.push(Token::ArrayStart);
                index += 1;
            }
            b']' => {
                tokens.push(Token::ArrayEnd);
                index += 1;
            }
            _ => {
                let start = index;
                while index < data.len()
                    && !data[index].is_ascii_whitespace()
                    && !matches!(data[index], b'<' | b'[' | b']' | b'%' | b'(' | b')' | b'/')
                {
                    index += 1;
                }
                if index == start {
                    index += 1;
                    continue;
                }
                tokens.push(Token::Keyword(
                    String::from_utf8_lossy(&data[start..index]).into_owned(),
                ));
            }
        }
    }
    tokens
}

/// Decode a run of hex digits, ignoring whitespace.
fn hex_bytes(digits: &[u8]) -> Vec<u8> {
    let mut cleaned = Vec::with_capacity(digits.len());
    for byte in digits {
        if byte.is_ascii_hexdigit() {
            cleaned.push(*byte);
        }
    }
    if cleaned.len() % 2 == 1 {
        cleaned.push(b'0');
    }
    let (pairs, _) = cleaned.as_chunks::<2>();
    pairs
        .iter()
        .map(|pair| {
            let hi = (pair[0] as char).to_digit(16).unwrap_or(0) as u8;
            let lo = (pair[1] as char).to_digit(16).unwrap_or(0) as u8;
            (hi << 4) | lo
        })
        .collect()
}
