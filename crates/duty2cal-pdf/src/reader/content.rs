// SPDX-License-Identifier: MPL-2.0
//! Content-stream tokenizer and the frozen operator subset.
//!
//! The interpreter tracks the graphics-state stack (`q`, `Q`, `cm`) and the
//! text state the family prints with (`BT`, `ET`, `Tf`, `Tm`, `Tj`). Every
//! other text-positioning operator, an inline image, a form XObject and an
//! unknown operator is refused by name.

use std::collections::{BTreeMap, HashMap};

use duty2cal_core::application::{Construct, SourceError};
use lopdf::{Dictionary, Document, Object};

use super::font::{refusal, unreadable, Font};
use super::words::Char;

/// An affine matrix `[a b c d e f]` in PDF order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Matrix {
    /// Horizontal scale.
    pub a: f64,
    /// Vertical shear.
    pub b: f64,
    /// Horizontal shear.
    pub c: f64,
    /// Vertical scale.
    pub d: f64,
    /// Horizontal translation.
    pub e: f64,
    /// Vertical translation.
    pub f: f64,
}

impl Matrix {
    /// The identity matrix.
    pub(crate) fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    /// `self × other`.
    pub(crate) fn concat(&self, other: &Self) -> Self {
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
            e: self.e * other.a + self.f * other.c + other.e,
            f: self.e * other.b + self.f * other.d + other.f,
        }
    }

    /// Transform a point.
    fn transform(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.a * x + self.c * y + self.e,
            self.b * x + self.d * y + self.f,
        )
    }
}

/// The graphics and text state the interpreter carries.
#[derive(Debug, Clone)]
struct State {
    ctm: Matrix,
    font: Option<Vec<u8>>,
    font_size: f64,
}

/// Interpret one page's decoded content into positioned characters.
pub(crate) fn interpret(
    document: &Document,
    page: usize,
    page_height: f64,
    content: &[u8],
    resources: Option<&Dictionary>,
    page_fonts: &BTreeMap<Vec<u8>, &Dictionary>,
) -> Result<Vec<Char>, SourceError> {
    let mut chars = Vec::new();
    let mut fonts: HashMap<Vec<u8>, Font> = HashMap::new();
    let mut state = State {
        ctm: Matrix::identity(),
        font: None,
        font_size: 0.0,
    };
    let mut saved: Vec<State> = Vec::new();
    let mut tm = Matrix::identity();
    let mut position = 0.0;

    let mut parser = Parser::new(content);
    let mut operands: Vec<Operand> = Vec::new();
    while let Some(token) = parser.next_token() {
        let operator = match token {
            Token::Operand(operand) => {
                operands.push(operand);
                continue;
            }
            Token::Operator(operator) => operator,
        };
        match operator.as_str() {
            "q" => saved.push(state.clone()),
            "Q" => {
                if let Some(restored) = saved.pop() {
                    state = restored;
                }
            }
            "cm" => {
                let values = numbers(&operands, 6);
                if let [a, b, c, d, e, f] = values.as_slice() {
                    let matrix = Matrix {
                        a: *a,
                        b: *b,
                        c: *c,
                        d: *d,
                        e: *e,
                        f: *f,
                    };
                    state.ctm = state.ctm.concat(&matrix);
                }
            }
            "BT" => {
                tm = Matrix::identity();
                position = 0.0;
            }
            "ET" => {}
            "Tf" => {
                if let (Some(Operand::Name(name)), Some(size)) = (
                    operands.first(),
                    operands.get(1).and_then(Operand::as_number),
                ) {
                    let font = page_fonts
                        .get(name)
                        .ok_or_else(|| unreadable("the page references no such font"))?;
                    if !fonts.contains_key(name) {
                        let parsed = Font::parse(document, font, page)?;
                        fonts.insert(name.clone(), parsed);
                    }
                    state.font = Some(name.clone());
                    state.font_size = size;
                }
            }
            "Tm" => {
                let values = numbers(&operands, 6);
                if let [a, b, c, d, e, f] = values.as_slice() {
                    if b.abs() > 1e-9 || c.abs() > 1e-9 {
                        return Err(refusal(Construct::TextRotation, page));
                    }
                    tm = Matrix {
                        a: *a,
                        b: *b,
                        c: *c,
                        d: *d,
                        e: *e,
                        f: *f,
                    };
                    position = 0.0;
                }
            }
            "Tj" => {
                let Some(Operand::String(bytes)) = operands.first() else {
                    operands.clear();
                    continue;
                };
                let font_name = state
                    .font
                    .as_ref()
                    .ok_or_else(|| unreadable("a text string appears before a font is selected"))?;
                let font = fonts
                    .get(font_name)
                    .ok_or_else(|| unreadable("the selected font was not loaded"))?;
                let mut index = 0;
                while index < bytes.len() {
                    let code = if index + 1 < bytes.len() {
                        let code = (u32::from(bytes[index]) << 8) | u32::from(bytes[index + 1]);
                        index += 2;
                        code
                    } else {
                        let code = u32::from(bytes[index]);
                        index += 1;
                        code
                    };
                    let text = font.unicode(code, page)?;
                    let width = font.width(code);
                    let advance = width * 0.001 * state.font_size;
                    let text_matrix = Matrix {
                        a: tm.a,
                        b: tm.b,
                        c: tm.c,
                        d: tm.d,
                        e: tm.e + position * tm.a,
                        f: tm.f + position * tm.b,
                    };
                    let matrix = text_matrix.concat(&state.ctm);
                    let text_bottom = font.descent() * 0.001 * state.font_size;
                    let text_top = text_bottom + state.font_size;
                    let corners = [
                        matrix.transform(0.0, text_bottom),
                        matrix.transform(advance, text_bottom),
                        matrix.transform(advance, text_top),
                        matrix.transform(0.0, text_top),
                    ];
                    let min_x = corners
                        .iter()
                        .map(|(x, _)| *x)
                        .fold(f64::INFINITY, f64::min);
                    let max_x = corners
                        .iter()
                        .map(|(x, _)| *x)
                        .fold(f64::NEG_INFINITY, f64::max);
                    let max_y = corners
                        .iter()
                        .map(|(_, y)| *y)
                        .fold(f64::NEG_INFINITY, f64::max);
                    chars.push(Char {
                        text: text.into_owned(),
                        x0: min_x,
                        x1: max_x,
                        top: page_height - max_y,
                    });
                    position += advance;
                }
            }
            "TJ" | "Td" | "TD" | "T*" | "Tc" | "Tw" | "'" | "\"" => {
                return Err(SourceError::UnsupportedConstruct {
                    construct: Construct::TextOperator(text_operator_name(&operator)),
                    page: Some(page),
                });
            }
            "Tz" => return Err(refusal(Construct::HorizontalScaling, page)),
            "BI" => return Err(refusal(Construct::InlineImage, page)),
            "Do" => {
                if let Some(Operand::Name(name)) = operands.first() {
                    if !is_image_xobject(document, resources, name) {
                        return Err(refusal(Construct::FormXObject, page));
                    }
                } else {
                    return Err(refusal(Construct::FormXObject, page));
                }
            }
            // The graphics and path operators the family prints change no text
            // state the extraction reads.
            "rg" | "RG" | "w" | "J" | "d" | "j" | "S" | "f" | "re" | "m" | "l" | "h" | "W"
            | "n" | "f*" | "B" | "B*" | "b" | "b*" | "s" | "c" | "v" | "y" | "K" | "k" | "G"
            | "g" | "cs" | "CS" | "sc" | "SC" | "scn" | "SCN" | "gs" | "i" | "M" | "ri" => {}
            other => {
                return Err(SourceError::UnsupportedConstruct {
                    construct: Construct::UnknownOperator(other.to_string()),
                    page: Some(page),
                });
            }
        }
        operands.clear();
    }
    Ok(chars)
}

/// The refused text operator's spelling.
fn text_operator_name(operator: &str) -> &'static str {
    match operator {
        "TJ" => "TJ",
        "Td" => "Td",
        "TD" => "TD",
        "T*" => "T*",
        "Tc" => "Tc",
        "Tw" => "Tw",
        "'" => "'",
        "\"" => "\"",
        _ => "text",
    }
}

/// The numeric operands a matrix or simple operator carries.
fn numbers(operands: &[Operand], count: usize) -> Vec<f64> {
    operands
        .iter()
        .take(count)
        .filter_map(Operand::as_number)
        .collect()
}

/// Whether a `Do` name resolves to an image XObject that must be skipped.
fn is_image_xobject(document: &Document, resources: Option<&Dictionary>, name: &[u8]) -> bool {
    let Some(resources) = resources else {
        return false;
    };
    let Some(xobjects) = resources
        .get(b"XObject")
        .ok()
        .and_then(|object| resolve(document, object))
        .and_then(|object| object.as_dict().ok())
    else {
        return false;
    };
    let Some(stream) = xobjects
        .get(name)
        .ok()
        .and_then(|object| resolve(document, object))
        .and_then(|object| object.as_stream().ok())
    else {
        return false;
    };
    stream
        .dict
        .get(b"Subtype")
        .ok()
        .and_then(|object| resolve(document, object))
        .and_then(|object| object.as_name().ok())
        .is_some_and(|subtype| subtype == b"Image")
}

/// Resolve one reference level.
fn resolve<'a>(document: &'a Document, object: &'a Object) -> Option<&'a Object> {
    match object {
        Object::Reference(id) => document.get_object(*id).ok(),
        other => Some(other),
    }
}

/// The operands the interpreter reads.
#[derive(Debug)]
enum Operand {
    Number(f64),
    Name(Vec<u8>),
    String(Vec<u8>),
    Array,
    Null,
}

impl Operand {
    fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(value) => Some(*value),
            _ => None,
        }
    }
}

/// One parsed token.
enum Token {
    Operand(Operand),
    Operator(String),
}

/// A content-stream tokenizer.
struct Parser<'a> {
    data: &'a [u8],
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, index: 0 }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_space()?;
        let byte = self.data[self.index];
        match byte {
            b'[' => {
                self.index += 1;
                loop {
                    self.skip_space();
                    if self.index >= self.data.len() || self.data[self.index] == b']' {
                        self.index = (self.index + 1).min(self.data.len());
                        break;
                    }
                    if self.operand().is_none() {
                        break;
                    }
                }
                Some(Token::Operand(Operand::Array))
            }
            b'<' => {
                if self.data.get(self.index + 1) == Some(&b'<') {
                    self.skip_dict();
                    Some(Token::Operand(Operand::Null))
                } else {
                    self.hex_string()
                        .map(|bytes| Token::Operand(Operand::String(bytes)))
                }
            }
            b'(' => self
                .literal_string()
                .map(|bytes| Token::Operand(Operand::String(bytes))),
            b'/' => self.name().map(|name| Token::Operand(Operand::Name(name))),
            _ => {
                let start = self.index;
                while self.index < self.data.len()
                    && !is_delimiter(self.data[self.index])
                    && !self.data[self.index].is_ascii_whitespace()
                {
                    self.index += 1;
                }
                if self.index == start {
                    self.index += 1;
                    return self.next_token();
                }
                let token = &self.data[start..self.index];
                if let Ok(number) = std::str::from_utf8(token).unwrap_or("").parse::<f64>() {
                    Some(Token::Operand(Operand::Number(number)))
                } else if matches!(token, b"true" | b"false" | b"null") {
                    Some(Token::Operand(Operand::Null))
                } else {
                    Some(Token::Operator(String::from_utf8_lossy(token).into_owned()))
                }
            }
        }
    }

    fn operand(&mut self) -> Option<Operand> {
        match self.next_token()? {
            Token::Operand(operand) => Some(operand),
            Token::Operator(_) => {
                self.index = self.index.saturating_sub(1);
                None
            }
        }
    }

    /// Skip whitespace and comments; `None` at end of input.
    fn skip_space(&mut self) -> Option<()> {
        loop {
            while self.index < self.data.len() && self.data[self.index].is_ascii_whitespace() {
                self.index += 1;
            }
            if self.data.get(self.index) == Some(&b'%') {
                while self.index < self.data.len() && self.data[self.index] != b'\n' {
                    self.index += 1;
                }
                continue;
            }
            break;
        }
        (self.index < self.data.len()).then_some(())
    }

    fn literal_string(&mut self) -> Option<Vec<u8>> {
        self.index += 1;
        let mut bytes = Vec::new();
        let mut depth = 1;
        while self.index < self.data.len() {
            let byte = self.data[self.index];
            self.index += 1;
            match byte {
                b'\\' => {
                    let next = *self.data.get(self.index)?;
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
                        return Some(bytes);
                    }
                    bytes.push(byte);
                }
                other => bytes.push(other),
            }
        }
        Some(bytes)
    }

    fn hex_string(&mut self) -> Option<Vec<u8>> {
        self.index += 1;
        let mut digits = Vec::new();
        while self.index < self.data.len() {
            let byte = self.data[self.index];
            self.index += 1;
            if byte == b'>' {
                break;
            }
            if byte.is_ascii_hexdigit() {
                digits.push(byte);
            }
        }
        if digits.len() % 2 == 1 {
            digits.push(b'0');
        }
        let (pairs, _) = digits.as_chunks::<2>();
        Some(
            pairs
                .iter()
                .map(|pair| {
                    let hi = (pair[0] as char).to_digit(16).unwrap_or(0) as u8;
                    let lo = (pair[1] as char).to_digit(16).unwrap_or(0) as u8;
                    (hi << 4) | lo
                })
                .collect(),
        )
    }

    fn name(&mut self) -> Option<Vec<u8>> {
        self.index += 1;
        let mut name = Vec::new();
        while self.index < self.data.len() {
            let byte = self.data[self.index];
            if byte.is_ascii_whitespace() || is_delimiter(byte) {
                break;
            }
            self.index += 1;
            if byte == b'#' {
                let pair = self.data.get(self.index..self.index + 2)?;
                let text = std::str::from_utf8(pair).ok()?;
                name.push(u8::from_str_radix(text, 16).ok()?);
                self.index += 2;
            } else {
                name.push(byte);
            }
        }
        Some(name)
    }

    fn skip_dict(&mut self) {
        self.index += 2;
        let mut depth = 1;
        while self.index < self.data.len() {
            match self.data[self.index] {
                b'<' if self.data.get(self.index + 1) == Some(&b'<') => {
                    depth += 1;
                    self.index += 2;
                }
                b'>' if self.data.get(self.index + 1) == Some(&b'>') => {
                    depth -= 1;
                    self.index += 2;
                    if depth == 0 {
                        return;
                    }
                }
                _ => self.index += 1,
            }
        }
    }
}

/// PDF delimiters that end a regular token.
fn is_delimiter(byte: u8) -> bool {
    matches!(
        byte,
        b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'/' | b'%'
    )
}
