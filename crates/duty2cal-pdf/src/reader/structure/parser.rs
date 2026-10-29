// SPDX-License-Identifier: MPL-2.0
//! A tokenizer and object parser over the supplied bytes.
//!
//! Supported spellings: numbers, names with `#xx` escapes, literal and
//! hexadecimal strings, arrays, dictionaries, references, booleans, `null` and
//! comments. A dictionary followed by the `stream` keyword becomes a stream
//! whose body's range the loader resolves once every object is known, so a
//! forward-pointing indirect `/Length` is accepted.

use duty2cal_core::application::SourceError;

use super::object::{Dict, ObjId, Object, Stream};

/// The deepest container nesting the parser accepts.
const MAX_DEPTH: usize = 256;

/// A parser over a byte slice.
pub(crate) struct Parser<'a> {
    data: &'a [u8],
    index: usize,
    depth: usize,
}

/// An indirect object as written at a cross-reference offset.
pub(crate) struct IndirectObject {
    /// The object's identifier.
    pub id: ObjId,
    /// The parsed object; a stream's body range starts at its body.
    pub object: Object,
}

impl<'a> Parser<'a> {
    /// A parser starting at `index`.
    pub(crate) fn at(data: &'a [u8], index: usize) -> Self {
        Self {
            data,
            index,
            depth: 0,
        }
    }

    /// The current position.
    pub(crate) fn index(&self) -> usize {
        self.index
    }

    /// Move to `index`.
    pub(crate) fn seek(&mut self, index: usize) {
        self.index = index;
    }

    /// Skip whitespace and comments.
    pub(crate) fn skip_space(&mut self) {
        loop {
            while self
                .data
                .get(self.index)
                .is_some_and(|byte| byte.is_ascii_whitespace())
            {
                self.index += 1;
            }
            if self.data.get(self.index) == Some(&b'%') {
                while self.data.get(self.index).is_some_and(|byte| *byte != b'\n') {
                    self.index += 1;
                }
                continue;
            }
            break;
        }
    }

    /// Whether the next token is the `keyword`, without consuming it.
    pub(crate) fn peek_keyword(&mut self, keyword: &[u8]) -> bool {
        let saved = self.index;
        self.skip_space();
        let matched = self.matches_keyword(keyword);
        self.index = saved;
        matched
    }

    /// Consume the `keyword` when it is the next token.
    pub(crate) fn consume_keyword(&mut self, keyword: &[u8]) -> bool {
        self.skip_space();
        if self.matches_keyword(keyword) {
            self.index += keyword.len();
            true
        } else {
            false
        }
    }

    /// The next byte, without consuming it.
    pub(crate) fn peek_byte(&mut self) -> Result<u8, SourceError> {
        self.skip_space();
        self.data
            .get(self.index)
            .copied()
            .ok_or_else(|| unreadable("the file ends inside the file structure"))
    }

    /// Consume one byte.
    pub(crate) fn advance(&mut self) {
        self.index += 1;
    }

    /// Parse one non-negative or negative integer.
    pub(crate) fn parse_i64(&mut self) -> Result<i64, SourceError> {
        self.skip_space();
        let start = self.index;
        let mut length = 0;
        if matches!(self.data.get(self.index), Some(b'+') | Some(b'-')) {
            length += 1;
        }
        while self
            .data
            .get(start + length)
            .is_some_and(u8::is_ascii_digit)
        {
            length += 1;
        }
        if length == 0 || (length == 1 && !self.data[start].is_ascii_digit()) {
            return Err(unreadable(format!("expected an integer at offset {start}")));
        }
        let text = std::str::from_utf8(&self.data[start..start + length])
            .map_err(|_| unreadable("an integer is not ASCII"))?;
        let value = text
            .parse::<i64>()
            .map_err(|_| unreadable(format!("the integer at offset {start} is out of range")))?;
        self.index = start + length;
        Ok(value)
    }

    /// Parse one object.
    pub(crate) fn parse_object(&mut self) -> Result<Object, SourceError> {
        self.skip_space();
        let byte = *self
            .data
            .get(self.index)
            .ok_or_else(|| unreadable("the file ends inside an object"))?;
        match byte {
            b'/' => Ok(Object::Name(self.parse_name()?)),
            b'(' => Ok(Object::String(self.parse_literal_string()?)),
            b'<' => {
                if self.data.get(self.index + 1) == Some(&b'<') {
                    self.parse_dictionary()
                } else {
                    Ok(Object::String(self.parse_hex_string()?))
                }
            }
            b'[' => self.parse_array(),
            b'+' | b'-' | b'.' | b'0'..=b'9' => self.parse_number_or_reference(),
            b't' | b'f' | b'n' => self.parse_keyword_object(),
            other => Err(unreadable(format!(
                "unexpected byte {other:#04x} at offset {}",
                self.index
            ))),
        }
    }

    fn matches_keyword(&self, keyword: &[u8]) -> bool {
        let end = self.index + keyword.len();
        let Some(candidate) = self.data.get(self.index..end) else {
            return false;
        };
        if candidate != keyword {
            return false;
        }
        match self.data.get(end) {
            None => true,
            Some(byte) => is_delimiter(*byte) || byte.is_ascii_whitespace(),
        }
    }

    fn parse_name(&mut self) -> Result<Vec<u8>, SourceError> {
        self.index += 1;
        let mut name = Vec::new();
        while let Some(&byte) = self.data.get(self.index) {
            if byte.is_ascii_whitespace() || is_delimiter(byte) {
                break;
            }
            self.index += 1;
            if byte == b'#' {
                let pair = self
                    .data
                    .get(self.index..self.index + 2)
                    .ok_or_else(|| unreadable("a name ends inside a # escape"))?;
                let text = std::str::from_utf8(pair)
                    .map_err(|_| unreadable("a name escape is not ASCII"))?;
                let decoded = u8::from_str_radix(text, 16)
                    .map_err(|_| unreadable("a name escape is not hexadecimal"))?;
                name.push(decoded);
                self.index += 2;
            } else {
                name.push(byte);
            }
        }
        Ok(name)
    }

    fn parse_literal_string(&mut self) -> Result<Vec<u8>, SourceError> {
        self.index += 1;
        let mut bytes = Vec::new();
        let mut depth = 1usize;
        while let Some(&byte) = self.data.get(self.index) {
            self.index += 1;
            match byte {
                b'\\' => {
                    let next = *self
                        .data
                        .get(self.index)
                        .ok_or_else(|| unreadable("a string ends inside an escape"))?;
                    self.index += 1;
                    match next {
                        b'n' => bytes.push(b'\n'),
                        b'r' => bytes.push(b'\r'),
                        b't' => bytes.push(b'\t'),
                        b'b' => bytes.push(8),
                        b'f' => bytes.push(12),
                        b'(' | b')' | b'\\' => bytes.push(next),
                        b'\n' => {}
                        b'\r' => {
                            if self.data.get(self.index) == Some(&b'\n') {
                                self.index += 1;
                            }
                        }
                        b'0'..=b'7' => {
                            let mut value = u16::from(next - b'0');
                            for _ in 0..2 {
                                match self.data.get(self.index) {
                                    Some(digit @ b'0'..=b'7') => {
                                        value = value * 8 + u16::from(digit - b'0');
                                        self.index += 1;
                                    }
                                    _ => break,
                                }
                            }
                            bytes.push(value as u8);
                        }
                        other => bytes.push(other),
                    }
                }
                b'(' => {
                    depth += 1;
                    bytes.push(byte);
                }
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(bytes);
                    }
                    bytes.push(byte);
                }
                other => bytes.push(other),
            }
        }
        Err(unreadable("a literal string is not terminated"))
    }

    fn parse_hex_string(&mut self) -> Result<Vec<u8>, SourceError> {
        self.index += 1;
        let mut digits = Vec::new();
        loop {
            let byte = *self
                .data
                .get(self.index)
                .ok_or_else(|| unreadable("a hexadecimal string is not terminated"))?;
            self.index += 1;
            if byte == b'>' {
                break;
            }
            if byte.is_ascii_hexdigit() {
                digits.push(byte);
            } else if !byte.is_ascii_whitespace() {
                return Err(unreadable("a hexadecimal string carries a non-hex digit"));
            }
        }
        if digits.len() % 2 == 1 {
            digits.push(b'0');
        }
        let (pairs, _) = digits.as_chunks::<2>();
        Ok(pairs
            .iter()
            .map(|pair| {
                let hi = (pair[0] as char).to_digit(16).unwrap_or(0) as u8;
                let lo = (pair[1] as char).to_digit(16).unwrap_or(0) as u8;
                (hi << 4) | lo
            })
            .collect())
    }

    fn parse_array(&mut self) -> Result<Object, SourceError> {
        self.index += 1;
        self.enter()?;
        let mut items = Vec::new();
        loop {
            self.skip_space();
            match self.data.get(self.index) {
                None => return Err(unreadable("an array is not terminated")),
                Some(b']') => {
                    self.index += 1;
                    break;
                }
                Some(_) => items.push(self.parse_object()?),
            }
        }
        self.leave();
        Ok(Object::Array(items))
    }

    fn parse_dictionary(&mut self) -> Result<Object, SourceError> {
        self.index += 2;
        self.enter()?;
        let mut dict = Dict::default();
        loop {
            self.skip_space();
            match self.data.get(self.index) {
                None => return Err(unreadable("a dictionary is not terminated")),
                Some(b'>') if self.data.get(self.index + 1) == Some(&b'>') => {
                    self.index += 2;
                    break;
                }
                Some(b'/') => {
                    let key = self.parse_name()?;
                    let value = self.parse_object()?;
                    dict.insert(key, value);
                }
                Some(other) => {
                    return Err(unreadable(format!(
                        "a dictionary key is not a name ({other:#04x})"
                    )))
                }
            }
        }
        self.leave();
        Ok(Object::Dictionary(dict))
    }

    fn parse_keyword_object(&mut self) -> Result<Object, SourceError> {
        let start = self.index;
        while self
            .data
            .get(self.index)
            .is_some_and(u8::is_ascii_alphabetic)
        {
            self.index += 1;
        }
        let keyword = &self.data[start..self.index];
        match keyword {
            b"true" => Ok(Object::Boolean(true)),
            b"false" => Ok(Object::Boolean(false)),
            b"null" => Ok(Object::Null),
            _ => Err(unreadable(format!("unexpected keyword at offset {start}"))),
        }
    }

    fn parse_number_or_reference(&mut self) -> Result<Object, SourceError> {
        let number = self.parse_number()?;
        if let Object::Integer(first) = number {
            let saved = self.index;
            if let Some(reference) = self.try_reference(first) {
                return Ok(reference);
            }
            self.index = saved;
        }
        Ok(number)
    }

    fn try_reference(&mut self, first: i64) -> Option<Object> {
        let generation = match self.parse_i64().ok()? {
            generation @ 0..=65535 => generation,
            _ => return None,
        };
        let object = u32::try_from(first).ok()?;
        self.skip_space();
        if self.data.get(self.index) != Some(&b'R') {
            return None;
        }
        let after = self.data.get(self.index + 1);
        if after.is_some_and(|byte| !is_delimiter(*byte) && !byte.is_ascii_whitespace()) {
            return None;
        }
        self.index += 1;
        Some(Object::Reference((object, generation as u16)))
    }

    fn parse_number(&mut self) -> Result<Object, SourceError> {
        self.skip_space();
        let start = self.index;
        if matches!(self.data.get(self.index), Some(b'+') | Some(b'-')) {
            self.index += 1;
        }
        let mut seen_dot = false;
        let mut digits = 0usize;
        while let Some(&byte) = self.data.get(self.index) {
            match byte {
                b'0'..=b'9' => {
                    digits += 1;
                    self.index += 1;
                }
                b'.' if !seen_dot => {
                    seen_dot = true;
                    self.index += 1;
                }
                _ => break,
            }
        }
        if digits == 0 {
            self.index = start;
            return Err(unreadable(format!("expected a number at offset {start}")));
        }
        let text = std::str::from_utf8(&self.data[start..self.index])
            .map_err(|_| unreadable("a number is not ASCII"))?;
        if seen_dot {
            text.parse::<f64>()
                .map(Object::Real)
                .map_err(|_| unreadable(format!("the number at offset {start} is malformed")))
        } else {
            text.parse::<i64>()
                .map(Object::Integer)
                .map_err(|_| unreadable(format!("the integer at offset {start} is out of range")))
        }
    }

    fn enter(&mut self) -> Result<(), SourceError> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err(unreadable("the object nesting is too deep"));
        }
        Ok(())
    }

    fn leave(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }
}

/// Parse the indirect object written at `offset`.
pub(crate) fn parse_indirect_object(
    data: &[u8],
    offset: usize,
) -> Result<IndirectObject, SourceError> {
    let mut parser = Parser::at(data, offset);
    let number = parser.parse_i64()?;
    let generation = parser.parse_i64()?;
    let id = (
        u32::try_from(number)
            .map_err(|_| unreadable("an indirect object number is out of range"))?,
        u16::try_from(generation)
            .map_err(|_| unreadable("an indirect object generation is out of range"))?,
    );
    if !parser.consume_keyword(b"obj") {
        return Err(unreadable(format!(
            "no indirect object starts at offset {offset}"
        )));
    }
    let object = match parser.parse_object()? {
        Object::Dictionary(dict) => {
            let after = parser.index();
            if parser.consume_keyword(b"stream") {
                let start = stream_body_start(data, parser.index())?;
                Object::Stream(Stream {
                    dict,
                    data: start..start,
                })
            } else {
                parser.seek(after);
                Object::Dictionary(dict)
            }
        }
        other => other,
    };
    Ok(IndirectObject { id, object })
}

/// The first body byte after the `stream` keyword.
fn stream_body_start(data: &[u8], index: usize) -> Result<usize, SourceError> {
    match data.get(index) {
        Some(b'\r') if data.get(index + 1) == Some(&b'\n') => Ok(index + 2),
        Some(b'\r') | Some(b'\n') => Ok(index + 1),
        _ => Err(unreadable(
            "the stream keyword is not followed by an end of line",
        )),
    }
}

/// PDF delimiters that end a regular token.
pub(crate) fn is_delimiter(byte: u8) -> bool {
    matches!(
        byte,
        b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'/' | b'%'
    )
}

/// The unreadable refusal for a structural problem.
pub(crate) fn unreadable(detail: impl Into<String>) -> SourceError {
    SourceError::Unreadable(detail.into())
}
