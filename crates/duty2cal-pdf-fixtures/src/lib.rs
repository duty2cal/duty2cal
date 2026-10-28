// SPDX-License-Identifier: MPL-2.0
//! Synthetic duty-plan fixtures in the supported family's shape.
//!
//! The generator emits bytes at test time; no PDF is committed. It does not
//! claim to reproduce the producer's bytes — it reproduces the readable shape:
//! Type0/Identity-H fonts with per-font `ToUnicode` CMaps and `/W` advances,
//! Flate content, a classic xref table, an xref stream, or object streams. The
//! refusal variants place one named construct on a known page.

use std::collections::BTreeSet;

/// Column x positions of the supported detail table.
pub const DATE_X: f64 = 40.0;
pub const TRIP_X: f64 = 100.0;
pub const REPORT_X: f64 = 150.0;
pub const POS_X: f64 = 210.0;
pub const ACTIVITY_X: f64 = 250.0;
pub const FROM_X: f64 = 320.0;
pub const TO_X: f64 = 360.0;
pub const START_X: f64 = 400.0;
pub const END_X: f64 = 480.0;
pub const AIRCRAFT_X: f64 = 540.0;
pub const BLOCKTIME_X: f64 = 600.0;
pub const TZ_X: f64 = 660.0;
pub const LAYOVER_X: f64 = 760.0;
pub const TAGS_X: f64 = 820.0;

/// The generated page's media box.
pub const PAGE_WIDTH: f64 = 1200.0;
/// The generated page's media box height.
pub const PAGE_HEIGHT: f64 = 842.0;

/// Helvetica AFM advances for ASCII 32–126, in 1/1000 of an em.
const HELVETICA_WIDTHS: [u16; 95] = [
    278, 278, 355, 556, 556, 889, 667, 191, 333, 333, 389, 584, 278, 333, 278, 278, 556, 556, 556,
    556, 556, 556, 556, 556, 556, 556, 278, 278, 584, 584, 584, 556, 1015, 667, 667, 722, 722, 667,
    611, 778, 722, 278, 500, 667, 556, 833, 722, 778, 667, 778, 722, 667, 611, 722, 667, 944, 667,
    667, 611, 278, 278, 278, 469, 556, 333, 556, 556, 500, 556, 556, 278, 556, 556, 222, 222, 500,
    222, 833, 556, 556, 556, 556, 333, 500, 278, 556, 500, 722, 500, 500, 500, 334, 260, 334, 584,
];

/// A positioned piece of text in a generated fixture: x, y, and the text.
pub type TextItem = (f64, f64, String);

/// How the generated file encodes its objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// A classic cross-reference table.
    ClassicXref,
    /// A cross-reference stream (PDF 1.5).
    XrefStream,
    /// A cross-reference stream with every non-stream object in an object stream.
    ObjectStreams,
}

/// The construct a refusal fixture places on page 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Variant {
    Supported,
    Type3Font,
    TextRotation,
    PageRotation,
    HorizontalScaling,
    InlineImage,
    StreamFilter,
    NonIdentityEncoding,
    MissingToUnicode,
    UnmappedGlyph,
    PageBox,
    FormXObject,
    MissingDescent,
    TextOperatorTj,
    UnknownOperator,
    Encrypted,
}

/// The roster month and year a fixture's dates belong to, and its heading.
///
/// A fixture writes its dates as `YYYY-MM-DD`; the builder prints them the way
/// the producer does, so the reader always faces the producer's spelling.
fn producer_layout(items: &[TextItem]) -> (Vec<TextItem>, Option<Vec<TextItem>>) {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let mut months: Vec<(i32, u32)> = Vec::new();
    let mut printed = Vec::with_capacity(items.len());
    // A revision footer prints its own stamp, which is not a roster date.
    let footer_rows: Vec<f64> = items
        .iter()
        .filter(|(_, _, text)| text.contains("Created"))
        .map(|(_, y, _)| *y)
        .collect();
    for (x, y, text) in items {
        let Some((year, month, day)) = iso_date(text) else {
            printed.push((*x, *y, text.clone()));
            continue;
        };
        if footer_rows
            .iter()
            .any(|footer_y| (footer_y - y).abs() < 1.0)
        {
            printed.push((*x, *y, text.clone()));
            continue;
        }
        if !months.contains(&(year, month)) {
            months.push((year, month));
        }
        printed.push((
            *x,
            *y,
            format!("{day:02} {}", weekday_abbreviation(year, month, day)),
        ));
    }
    assert!(
        months.len() <= 1,
        "a fixture must print one roster month, found {months:?}"
    );
    let heading = months.first().map(|(year, month)| {
        at(
            830.0,
            &[
                (DATE_X, "Month:"),
                (DATE_X + 40.0, MONTHS[(month - 1) as usize]),
                // Clear of the longest month name, so the two never touch.
                (DATE_X + 100.0, &year.to_string()),
            ],
        )
    });
    (printed, heading)
}

/// A `YYYY-MM-DD` cell as its calendar date.
fn iso_date(text: &str) -> Option<(i32, u32, u32)> {
    if text.len() != 10 || &text[4..5] != "-" || &text[7..8] != "-" {
        return None;
    }
    let year = text[..4].parse::<i32>().ok()?;
    let month = text[5..7].parse::<u32>().ok()?;
    let day = text[8..10].parse::<u32>().ok()?;
    (1..=12).contains(&month).then_some((year, month, day))
}

/// The weekday of a calendar date, as the producer abbreviates it.
pub fn weekday_abbreviation(year: i32, month: u32, day: u32) -> &'static str {
    const SHIFTS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut shifted = year;
    if month < 3 {
        shifted -= 1;
    }
    let index = (shifted + shifted / 4 - shifted / 100
        + shifted / 400
        + SHIFTS[(month - 1) as usize]
        + day as i32)
        .rem_euclid(7) as usize;
    ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"][index]
}

/// Build a one-page PDF placing each `(x, y, text)` item with a 9 pt
/// Type0/Identity-H font.
pub fn build_pdf(items: &[(f64, f64, String)]) -> Vec<u8> {
    build(items, Encoding::ClassicXref, 0, true, Variant::Supported)
}

/// Build a PDF of exactly `size` bytes whose page content stays readable.
///
/// The padding is one comment inside an unfiltered content stream, so the
/// trailing `%%EOF` stays where the reader expects it. `/Length` grows with the
/// padding, so the builder corrects for the changed digit count.
pub fn build_pdf_at_size(items: &[(f64, f64, String)], size: usize) -> Vec<u8> {
    let base = build(items, Encoding::ClassicXref, 0, false, Variant::Supported);
    assert!(
        base.len() <= size,
        "a {size}-byte PDF cannot hold the {} bytes of fixture content",
        base.len()
    );
    let mut padding = size - base.len();
    for _ in 0..8 {
        let pdf = build(
            items,
            Encoding::ClassicXref,
            padding,
            false,
            Variant::Supported,
        );
        if pdf.len() == size {
            return pdf;
        }
        if pdf.len() < size {
            padding += size - pdf.len();
        } else {
            padding = padding.saturating_sub(pdf.len() - size);
        }
    }
    panic!("could not build a PDF of exactly {size} bytes");
}

/// Build a plan in the requested encoding, with the same page content.
pub fn build_plan(items: &[(f64, f64, String)], encoding: Encoding) -> Vec<u8> {
    build(items, encoding, 0, true, Variant::Supported)
}

/// Build the refusal fixture that places `construct` on page 1.
pub fn build_construct_plan(construct: &str) -> Vec<u8> {
    let variant = match construct {
        "Type3 font" => Variant::Type3Font,
        "text rotation" => Variant::TextRotation,
        "page rotation" => Variant::PageRotation,
        "horizontal text scaling" => Variant::HorizontalScaling,
        "inline image" => Variant::InlineImage,
        "unsupported stream filter" => Variant::StreamFilter,
        "font encoding other than Identity-H" => Variant::NonIdentityEncoding,
        "missing ToUnicode CMap" => Variant::MissingToUnicode,
        "unmapped glyph code" => Variant::UnmappedGlyph,
        "page box other than the media box" => Variant::PageBox,
        "form XObject" => Variant::FormXObject,
        "font descriptor without a descent" => Variant::MissingDescent,
        "text operator TJ" => Variant::TextOperatorTj,
        "unknown content-stream operator" => Variant::UnknownOperator,
        other => panic!("no generated fixture for the construct {other:?}"),
    };
    build(&equivalence_plan(), Encoding::ClassicXref, 0, true, variant)
}

/// Build an encrypted document with a readable structure.
pub fn build_encrypted_plan() -> Vec<u8> {
    build(
        &equivalence_plan(),
        Encoding::ClassicXref,
        0,
        true,
        Variant::Encrypted,
    )
}

/// Build a readable PDF that prints no detail-table header.
pub fn build_no_header_plan() -> Vec<u8> {
    let items = at(
        700.0,
        &[(DATE_X, "Notes"), (FROM_X, "This page is not a duty plan.")],
    );
    build(&items, Encoding::ClassicXref, 0, true, Variant::Supported)
}

/// Build a truncated document: a valid fixture cut short.
pub fn build_truncated_plan() -> Vec<u8> {
    let mut bytes = build_plan(&equivalence_plan(), Encoding::ClassicXref);
    let keep = bytes.len() * 2 / 3;
    bytes.truncate(keep);
    bytes
}

/// Build a Type3 plan that prints an identifying name.
pub fn build_privacy_plan() -> Vec<u8> {
    let mut items = equivalence_plan();
    items.extend(at(760.0, &[(DATE_X, "Zephyr"), (DATE_X + 40.0, "Vale")]));
    build(&items, Encoding::ClassicXref, 0, true, Variant::Type3Font)
}

/// Build a supported plan whose detail table continues on a second page with a
/// repeated header and a TK row on each page.
pub fn build_multipage_plan() -> Vec<u8> {
    let row = |date: &str| {
        let mut items = detail_header_row(600.0);
        items.extend(at(
            585.0,
            &[
                (DATE_X, date),
                (ACTIVITY_X, "TK"),
                (FROM_X, "ABC"),
                (START_X, "08:00"),
                (END_X, "12:00"),
            ],
        ));
        items
    };
    build_pages(
        &[row("2026-06-12"), row("2026-06-13")],
        Encoding::ClassicXref,
        0,
        true,
        Variant::Supported,
    )
}

/// Build one fixture page set.
fn build(
    items: &[TextItem],
    encoding: Encoding,
    padding: usize,
    flate: bool,
    variant: Variant,
) -> Vec<u8> {
    build_pages(&[items.to_vec()], encoding, padding, flate, variant)
}

/// Build a document with one content stream per page.
fn build_pages(
    pages: &[Vec<TextItem>],
    encoding: Encoding,
    padding: usize,
    flate: bool,
    variant: Variant,
) -> Vec<u8> {
    let mut used: BTreeSet<u32> = BTreeSet::new();
    let mut contents = Vec::with_capacity(pages.len());
    for items in pages {
        let (printed, heading) = producer_layout(items);
        let mut all = heading.unwrap_or_default();
        all.extend(printed);
        for (_, _, text) in &all {
            for unit in text.encode_utf16() {
                used.insert(u32::from(unit));
            }
        }
        contents.push(page_content(&all, padding, variant));
    }

    let mut pdf = Pdf::default();
    let content_ids: Vec<u32> = contents
        .iter()
        .map(|content| match variant {
            Variant::StreamFilter => {
                pdf.add_stream_with_filter("ASCIIHexDecode", content.as_bytes(), "")
            }
            _ => pdf.add_stream("", content.as_bytes(), flate),
        })
        .collect();
    let image_id = pdf.add_stream(
        "/Type /XObject /Subtype /Image /Width 1 /Height 1 /ColorSpace /DeviceGray \
         /BitsPerComponent 8 /Filter /DCTDecode",
        b"\xFF\xD8\xFF\xD9",
        false,
    );
    let form_id = (variant == Variant::FormXObject).then(|| {
        pdf.add_stream(
            "/Type /XObject /Subtype /Form /BBox [0 0 10 10] /Resources << >>",
            b"",
            false,
        )
    });

    let fonts = add_fonts(&mut pdf, &used, variant);
    let pages_id = pdf.reserve();
    let mut page_ids = Vec::new();
    for content_id in content_ids {
        let page_id = pdf.reserve();
        let rotation = if variant == Variant::PageRotation {
            " /Rotate 90"
        } else {
            ""
        };
        let crop = if variant == Variant::PageBox {
            " /CropBox [0 0 100 100]"
        } else {
            ""
        };
        let xobjects = match form_id {
            Some(form_id) => format!("/XObject << /Fm1 {form_id} 0 R >>"),
            None => format!("/XObject << /Im1 {image_id} 0 R >>"),
        };
        pdf.set_reserved(
            page_id,
            format!(
                "<< /Type /Page /Parent {pages_id} 0 R /MediaBox [0 0 {PAGE_WIDTH} {PAGE_HEIGHT}]\
                 {rotation}{crop} /Resources << /Font << /F1 {} 0 R /F2 {} 0 R /F3 {} 0 R >> \
                 {xobjects} >> /Contents {content_id} 0 R >>",
                fonts[0], fonts[1], fonts[2]
            ),
        );
        page_ids.push(page_id);
    }
    let kids = page_ids
        .iter()
        .map(|id| format!("{id} 0 R"))
        .collect::<Vec<_>>()
        .join(" ");
    pdf.set_reserved(
        pages_id,
        format!(
            "<< /Type /Pages /Kids [{kids}] /Count {} >>",
            page_ids.len()
        ),
    );
    let catalog_id = pdf.add_dict(format!("<< /Type /Catalog /Pages {pages_id} 0 R >>"));

    if variant == Variant::Encrypted {
        let encrypt_id =
            pdf.add_dict("<< /Filter /Standard /V 1 /R 2 /O <00> /U <00> /P -4 >>".to_string());
        pdf.finish(catalog_id, encoding, Some(encrypt_id))
    } else {
        pdf.finish(catalog_id, encoding, None)
    }
}

/// The page content stream: `Tm`-only text positioning and the variant hook.
fn page_content(items: &[TextItem], padding: usize, variant: Variant) -> String {
    let mut content = String::new();
    if variant == Variant::HorizontalScaling {
        content.push_str("90 Tz\n");
    }
    content.push_str("BT\n");
    let mut used_tj_operator = false;
    for (index, (x, y, text)) in items.iter().enumerate() {
        let font = index % 3 + 1;
        let matrix = if variant == Variant::TextRotation && index == 0 {
            format!("1 0.1 0 1 {x} {y} Tm")
        } else {
            format!("1 0 0 1 {x} {y} Tm")
        };
        content.push_str(&format!("/F{font} 9 Tf\n{matrix}\n<"));
        for unit in text.encode_utf16() {
            content.push_str(&format!("{unit:04X}"));
        }
        if variant == Variant::TextOperatorTj && !used_tj_operator {
            content.push_str("> TJ\n<0041");
            used_tj_operator = true;
        }
        content.push_str("> Tj\n");
    }
    content.push_str("ET\n");
    match variant {
        Variant::FormXObject => content.push_str("q\n/Form1 Do\nQ\n"),
        Variant::InlineImage => content.push_str("BI\n/W 1 /H 1 /BPC 8 /CS /G\nID \u{0} EI\n"),
        Variant::UnknownOperator => content.push_str("BZ\n"),
        _ => content.push_str("q\n105 0 0 30 701 529 cm\n/Im1 Do\nQ\n"),
    }
    if padding > 0 {
        content.push('%');
        content.push_str(&" ".repeat(padding.saturating_sub(2)));
        content.push('\n');
    }
    content
}

/// Add the three Type0/Identity-H fonts, or the variant's font shape.
fn add_fonts(pdf: &mut Pdf, used: &BTreeSet<u32>, variant: Variant) -> Vec<u32> {
    let mut fonts = Vec::new();
    for index in 0..3 {
        if index == 0 && variant == Variant::Type3Font {
            fonts.push(
                pdf.add_dict(
                    "<< /Type /Font /Subtype /Type3 /FontBBox [0 0 1000 1000] \
                     /FontMatrix [0.001 0 0 0.001 0 0] /CharProcs << >> \
                     /Encoding << /Type /Encoding /Differences [65 /A] >> \
                     /FirstChar 65 /LastChar 65 /Widths [600] >>"
                        .to_string(),
                ),
            );
            continue;
        }
        let mapping = if variant == Variant::UnmappedGlyph {
            used.iter().copied().filter(|code| *code != 77).collect()
        } else {
            used.clone()
        };
        let to_unicode_id = if variant == Variant::MissingToUnicode {
            None
        } else {
            let stream = to_unicode_stream(&mapping);
            Some(pdf.add_stream("", stream.as_bytes(), true))
        };
        let width_entries: String = used
            .iter()
            .map(|code| format!("{code} [{}]", helvetica_width(*code)))
            .collect::<Vec<_>>()
            .join(" ");
        let descendant_id = pdf.reserve();
        let descriptor_id = pdf.reserve();
        let descent = if variant == Variant::MissingDescent && index == 0 {
            ""
        } else {
            "/Descent -120"
        };
        pdf.set_reserved(
            descriptor_id,
            format!(
                "<< /Type /FontDescriptor /FontName /HPDFA{index}+NotoSans /Ascent 880 \
                 {descent} /CapHeight 700 /Flags 32 /FontBBox [0 -200 1000 900] \
                 /ItalicAngle 0 /StemV 80 >>"
            ),
        );
        pdf.set_reserved(
            descendant_id,
            format!(
                "<< /Type /Font /Subtype /CIDFontType2 /BaseFont /HPDFA{index}+NotoSans \
                 /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> \
                 /FontDescriptor {descriptor_id} 0 R /DW 1000 /W [{width_entries}] >>"
            ),
        );
        let encoding = if variant == Variant::NonIdentityEncoding && index == 0 {
            "/WinAnsiEncoding"
        } else {
            "/Identity-H"
        };
        let to_unicode = to_unicode_id
            .map(|id| format!(" /ToUnicode {id} 0 R"))
            .unwrap_or_default();
        fonts.push(pdf.add_dict(format!(
            "<< /Type /Font /Subtype /Type0 /BaseFont /HPDFA{index}+NotoSans \
             /Encoding {encoding} /DescendantFonts [{descendant_id} 0 R]{to_unicode} >>"
        )));
    }
    fonts
}

/// The `ToUnicode` CMap stream.
fn to_unicode_stream(used: &BTreeSet<u32>) -> String {
    let mut cmap = String::from(
        "/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n\
         /CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n\
         /CMapName /Adobe-Identity-UCS def\n/CMapType 2 def\n\
         1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n",
    );
    let entries: Vec<String> = used
        .iter()
        .map(|code| format!("<{code:04X}> <{code:04X}>"))
        .collect();
    for chunk in entries.chunks(100) {
        cmap.push_str(&format!("{} beginbfchar\n", chunk.len()));
        for entry in chunk {
            cmap.push_str(entry);
            cmap.push('\n');
        }
        cmap.push_str("endbfchar\n");
    }
    cmap.push_str("endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n");
    cmap
}

/// The Helvetica advance for one code.
fn helvetica_width(code: u32) -> u16 {
    if (32..=126).contains(&code) {
        HELVETICA_WIDTHS[(code - 32) as usize]
    } else {
        600
    }
}

/// A tiny PDF object writer.
#[derive(Default)]
struct Pdf {
    objects: Vec<Option<ObjectBody>>,
}

/// One object body.
enum ObjectBody {
    Dict(String),
    Stream { dict: String, data: Vec<u8> },
}

impl Pdf {
    /// Reserve an object id, to be filled later.
    fn reserve(&mut self) -> u32 {
        self.objects.push(None);
        self.objects.len() as u32
    }

    /// Fill a reserved id with a dictionary body.
    fn set_reserved(&mut self, id: u32, body: String) -> u32 {
        self.objects[(id - 1) as usize] = Some(ObjectBody::Dict(body));
        id
    }

    /// Add a dictionary object.
    fn add_dict(&mut self, body: String) -> u32 {
        self.objects.push(Some(ObjectBody::Dict(body)));
        self.objects.len() as u32
    }

    /// Add a stream object.
    fn add_stream(&mut self, extra: &str, data: &[u8], flate: bool) -> u32 {
        let data = if flate {
            miniz_oxide::deflate::compress_to_vec_zlib(data, 6)
        } else {
            data.to_vec()
        };
        let filter = if flate { " /Filter /FlateDecode" } else { "" };
        self.objects.push(Some(ObjectBody::Stream {
            dict: format!("<< /Length {}{filter} {extra} >>", data.len()),
            data,
        }));
        self.objects.len() as u32
    }

    /// Add a stream object with an explicit filter name.
    fn add_stream_with_filter(&mut self, filter: &str, data: &[u8], extra: &str) -> u32 {
        self.objects.push(Some(ObjectBody::Stream {
            dict: format!("<< /Length {} /Filter /{filter} {extra} >>", data.len()),
            data: data.to_vec(),
        }));
        self.objects.len() as u32
    }

    /// Serialize the document.
    fn finish(&self, root: u32, encoding: Encoding, encrypt: Option<u32>) -> Vec<u8> {
        match encoding {
            Encoding::ClassicXref => self.finish_classic(root, encrypt),
            Encoding::XrefStream | Encoding::ObjectStreams => {
                self.finish_xref_stream(root, encoding == Encoding::ObjectStreams, encrypt)
            }
        }
    }

    /// Serialize with a classic cross-reference table.
    fn finish_classic(&self, root: u32, encrypt: Option<u32>) -> Vec<u8> {
        let mut pdf = Vec::new();
        pdf.extend_from_slice(b"%PDF-1.7\n%\xE2\xE3\xCF\xD3\n");
        let mut offsets = Vec::with_capacity(self.objects.len());
        for (index, body) in self.objects.iter().enumerate() {
            offsets.push(pdf.len());
            let body = body.as_ref().expect("every object is filled");
            pdf.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
            write_body(&mut pdf, body);
            pdf.extend_from_slice(b"\nendobj\n");
        }
        let xref_position = pdf.len();
        pdf.extend_from_slice(format!("xref\n0 {}\n", self.objects.len() + 1).as_bytes());
        pdf.extend_from_slice(b"0000000000 65535 f \n");
        for offset in &offsets {
            pdf.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        let encrypt = encrypt
            .map(|id| format!(" /Encrypt {id} 0 R"))
            .unwrap_or_default();
        pdf.extend_from_slice(
            format!(
                "trailer\n<< /Size {} /Root {root} 0 R{encrypt} >>\nstartxref\n{xref_position}\n%%EOF\n",
                self.objects.len() + 1
            )
            .as_bytes(),
        );
        pdf
    }

    /// Serialize with an xref stream, optionally packing the dictionaries.
    fn finish_xref_stream(&self, root: u32, pack: bool, encrypt: Option<u32>) -> Vec<u8> {
        let mut packed: Vec<(u32, String)> = Vec::new();
        if pack {
            for (index, body) in self.objects.iter().enumerate() {
                if let Some(ObjectBody::Dict(dict)) = body {
                    packed.push(((index + 1) as u32, dict.clone()));
                }
            }
        }

        let mut header = Vec::new();
        let mut body = Vec::new();
        for (id, dict) in &packed {
            header.extend_from_slice(format!("{id} {} ", body.len()).as_bytes());
            body.extend_from_slice(dict.as_bytes());
            body.push(b'\n');
        }
        let mut object_stream = header;
        let first = object_stream.len();
        object_stream.extend_from_slice(&body);
        let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&object_stream, 6);

        // The object stream takes the next id when dictionaries are packed; the
        // xref stream follows both.
        let object_stream_id = self.objects.len() + 1;
        let xref_id = self.objects.len() + if pack { 2 } else { 1 };
        let size = xref_id + 1;

        let mut pdf = Vec::new();
        pdf.extend_from_slice(b"%PDF-1.7\n%\xE2\xE3\xCF\xD3\n");
        let mut offsets: Vec<Option<usize>> = vec![None; size];
        for (index, body) in self.objects.iter().enumerate() {
            if packed.iter().any(|(id, _)| *id == index as u32 + 1) {
                continue;
            }
            offsets[index + 1] = Some(pdf.len());
            let body = body.as_ref().expect("every object is filled");
            pdf.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
            write_body(&mut pdf, body);
            pdf.extend_from_slice(b"\nendobj\n");
        }
        if pack {
            offsets[object_stream_id] = Some(pdf.len());
            pdf.extend_from_slice(format!("{object_stream_id} 0 obj\n").as_bytes());
            pdf.extend_from_slice(
                format!(
                    "<< /Type /ObjStm /N {} /First {} /Length {} /Filter /FlateDecode >>\nstream\n",
                    packed.len(),
                    first,
                    compressed.len()
                )
                .as_bytes(),
            );
            pdf.extend_from_slice(&compressed);
            pdf.extend_from_slice(b"\nendstream\nendobj\n");
        }
        let xref_offset = pdf.len();
        offsets[xref_id] = Some(xref_offset);

        let mut xref = Vec::with_capacity(size * 7);
        xref.extend_from_slice(&[0, 0, 0, 0, 0, 255, 255]);
        for id in 1..=self.objects.len() as u32 {
            if let Some(position) = packed.iter().position(|(packed_id, _)| *packed_id == id) {
                xref.push(2);
                xref.extend_from_slice(&(object_stream_id as u32).to_be_bytes());
                xref.extend_from_slice(&(position as u16).to_be_bytes());
            } else {
                let offset = offsets[id as usize].expect("direct object offset");
                xref.push(1);
                xref.extend_from_slice(&(offset as u32).to_be_bytes());
                xref.extend_from_slice(&0u16.to_be_bytes());
            }
        }
        if pack {
            let offset = offsets[object_stream_id].expect("object stream offset");
            xref.push(1);
            xref.extend_from_slice(&(offset as u32).to_be_bytes());
            xref.extend_from_slice(&0u16.to_be_bytes());
        }
        xref.push(1);
        xref.extend_from_slice(&(xref_offset as u32).to_be_bytes());
        xref.extend_from_slice(&0u16.to_be_bytes());

        let encrypt = encrypt
            .map(|id| format!(" /Encrypt {id} 0 R"))
            .unwrap_or_default();
        pdf.extend_from_slice(format!("{xref_id} 0 obj\n").as_bytes());
        pdf.extend_from_slice(
            format!(
                "<< /Type /XRef /Size {size} /W [1 4 2] /Root {root} 0 R{encrypt} /Length {} >>\nstream\n",
                xref.len()
            )
            .as_bytes(),
        );
        pdf.extend_from_slice(&xref);
        pdf.extend_from_slice(b"\nendstream\nendobj\n");
        pdf.extend_from_slice(format!("startxref\n{xref_offset}\n%%EOF\n").as_bytes());
        pdf
    }
}

/// Write one object body.
fn write_body(pdf: &mut Vec<u8>, body: &ObjectBody) {
    match body {
        ObjectBody::Dict(dict) => pdf.extend_from_slice(dict.as_bytes()),
        ObjectBody::Stream { dict, data } => {
            pdf.extend_from_slice(dict.as_bytes());
            pdf.extend_from_slice(b"\nstream\n");
            pdf.extend_from_slice(data);
            pdf.extend_from_slice(b"\nendstream");
        }
    }
}

/// Place `(x, text)` values on one row at height `y`.
pub fn at(y: f64, values: &[(f64, &str)]) -> Vec<(f64, f64, String)> {
    values
        .iter()
        .map(|(x, text)| (*x, y, (*text).to_string()))
        .collect()
}

/// The detail-table header row: local-date, local-report and UTC-time labels.
pub fn detail_header_row(y: f64) -> Vec<(f64, f64, String)> {
    let mut items = Vec::new();
    items.extend(at(
        y,
        &[
            (DATE_X, "Date"),
            (DATE_X + 20.0, "(LT)"),
            (TRIP_X, "Trip"),
            (REPORT_X, "Report"),
            (REPORT_X + 30.0, "(LT)"),
            (POS_X, "Pos"),
            (ACTIVITY_X, "Activity"),
            (FROM_X, "From"),
            (TO_X, "To"),
            (START_X, "Start"),
            (START_X + 24.0, "(UTC)"),
            (END_X, "End"),
            (END_X + 16.0, "(UTC)"),
            (AIRCRAFT_X, "A/C"),
            (BLOCKTIME_X, "Blocktime"),
            (TZ_X, "Timezone"),
            (LAYOVER_X, "Layover"),
            (TAGS_X, "Tags"),
        ],
    ));
    items.extend(at(y - 9.0, &[(TZ_X, "Difference")]));
    items
}

/// The fictional June plan both the golden test and the browser equivalence
/// scenario convert.
pub fn equivalence_plan() -> Vec<(f64, f64, String)> {
    let mut items = detail_header_row(600.0);
    items.extend(at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "Tychon"),
            (TO_X, "Mareum"),
            (START_X, "09:25"),
            (END_X, "15:45"),
        ],
    ));
    items.extend(at(570.0, &[(DATE_X, "2099-06-08"), (ACTIVITY_X, "U")]));
    items.extend(at(
        520.0,
        &[
            (DATE_X, "Created"),
            (TRIP_X, "2099-05-21"),
            (REPORT_X, "15:52"),
            (POS_X, "UTC"),
        ],
    ));
    items
}

/// A supported plan with the detail header, the given dated rows and the
/// revision footer the producer prints.
fn dated_plan(rows: &[(f64, &[(f64, &str)])]) -> Vec<(f64, f64, String)> {
    let mut items = detail_header_row(600.0);
    for (y, values) in rows {
        items.extend(at(*y, values));
    }
    items.extend(at(
        520.0,
        &[
            (DATE_X, "Created"),
            (TRIP_X, "2099-05-21"),
            (REPORT_X, "15:52"),
            (POS_X, "UTC"),
        ],
    ));
    items
}

/// A supported plan whose one detail row prints an unrecognized activity code
/// with a station and both UTC endpoints, so the conversion warns.
pub fn warned_plan() -> Vec<(f64, f64, String)> {
    dated_plan(&[(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "ZZ_M_CF"),
            (FROM_X, "ABC"),
            (START_X, "08:00"),
            (END_X, "15:30"),
        ],
    )])
}

/// A supported plan whose one detail row prints an unrecognized activity code
/// with no printed UTC times and no route, so the all-day fallback warns.
pub fn all_day_warned_plan() -> Vec<(f64, f64, String)> {
    dated_plan(&[(585.0, &[(DATE_X, "2099-06-07"), (ACTIVITY_X, "ZZ_M_CF")])])
}

/// A supported plan with two detail rows carrying unrecognized activity codes.
pub fn two_warned_rows_plan() -> Vec<(f64, f64, String)> {
    dated_plan(&[
        (
            585.0,
            &[
                (DATE_X, "2099-06-07"),
                (ACTIVITY_X, "ZZ_M_CF"),
                (FROM_X, "ABC"),
                (START_X, "08:00"),
                (END_X, "15:30"),
            ],
        ),
        (
            570.0,
            &[
                (DATE_X, "2099-06-08"),
                (ACTIVITY_X, "ZZ_TST"),
                (FROM_X, "DEF"),
                (START_X, "09:00"),
                (END_X, "12:00"),
            ],
        ),
    ])
}

/// A supported plan with one unrepresentable row and one warned row.
pub fn error_and_warning_plan() -> Vec<(f64, f64, String)> {
    dated_plan(&[
        (
            585.0,
            &[
                (DATE_X, "2099-06-07"),
                (ACTIVITY_X, "ZZ999"),
                (START_X, "08:00"),
            ],
        ),
        (
            570.0,
            &[
                (DATE_X, "2099-06-08"),
                (ACTIVITY_X, "ZZ_M_CF"),
                (FROM_X, "ABC"),
                (START_X, "08:00"),
                (END_X, "15:30"),
            ],
        ),
    ])
}
