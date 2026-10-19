// SPDX-License-Identifier: MPL-2.0
//! Test support: build a minimal, uncompressed PDF from positioned text items,
//! and place them at the column positions of the supported detail table.
//!
//! Fixtures are generated at test time, so no real plan or committed binary
//! fixture is needed (see the change's fixture decision).

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

/// Escape a string for a PDF literal string.
fn escape(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

/// A positioned piece of text in a generated fixture: x, y, and the text.
pub type TextItem = (f64, f64, String);

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

/// Build a one-page PDF placing each `(x, y, text)` item with Helvetica 9pt.
///
/// Coordinates are in page space (origin bottom-left); the page is A4-sized.
/// The heading and the date spelling are the producer's, never the fixture's.
pub fn build_pdf(items: &[(f64, f64, String)]) -> Vec<u8> {
    build_pdf_with_padding(items, 0)
}

/// Build a PDF of exactly `size` bytes whose page content stays readable.
///
/// The padding is one comment line inside the content stream, so the trailing
/// `%%EOF` stays where the reader expects it. `/Length` grows with the
/// padding, so the builder corrects for the changed digit count.
pub fn build_pdf_at_size(items: &[(f64, f64, String)], size: usize) -> Vec<u8> {
    let base = build_pdf(items);
    assert!(
        base.len() <= size,
        "a {size}-byte PDF cannot hold the {} bytes of fixture content",
        base.len()
    );
    let mut padding = size - base.len();
    for _ in 0..8 {
        let pdf = build_pdf_with_padding(items, padding);
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

/// The PDF builder with `padding` bytes of content-stream comment.
fn build_pdf_with_padding(items: &[(f64, f64, String)], padding: usize) -> Vec<u8> {
    let (printed, heading) = producer_layout(items);
    let mut all = heading.unwrap_or_default();
    all.extend(printed);
    let items = &all;
    let mut content = String::from("BT\n/F1 9 Tf\n");
    for (x, y, text) in items {
        content.push_str(&format!("1 0 0 1 {x} {y} Tm ({}) Tj\n", escape(text)));
    }
    content.push_str("ET\n");
    match padding {
        0 => {}
        1 => content.push('\n'),
        _ => {
            content.push('%');
            content.push_str(&" ".repeat(padding - 2));
            content.push('\n');
        }
    }

    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 1200 842] \
         /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>"
            .to_string(),
        format!(
            "<< /Length {} >>\nstream\n{content}endstream",
            content.len()
        ),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
    ];

    let mut pdf = String::from("%PDF-1.4\n");
    let mut offsets = Vec::with_capacity(objects.len());
    for (index, body) in objects.iter().enumerate() {
        offsets.push(pdf.len());
        pdf.push_str(&format!("{} 0 obj\n{body}\nendobj\n", index + 1));
    }

    let xref_position = pdf.len();
    pdf.push_str(&format!("xref\n0 {}\n", objects.len() + 1));
    pdf.push_str("0000000000 65535 f \n");
    for offset in &offsets {
        pdf.push_str(&format!("{offset:010} 00000 n \n"));
    }
    pdf.push_str(&format!(
        "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref_position}\n%%EOF\n",
        objects.len() + 1
    ));

    pdf.into_bytes()
}

/// Place `(x, text)` values on one row at height `y`.
pub fn at(y: f64, values: &[(f64, &str)]) -> Vec<(f64, f64, String)> {
    values
        .iter()
        .map(|(x, text)| (*x, y, (*text).to_string()))
        .collect()
}

/// The detail-table header row: local-date, local-report and UTC-time labels.
///
/// Adjacent label words are placed within the merge tolerance so that wrapped
/// headings such as `Date (LT)` and `Start (UTC)` form one column each.
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
    // The header wraps over several print rows: this one carries the words that
    // continue a column, and the anchor labels of the family are spread across
    // both rows only when the producer's own header starts on a later row.
    items.extend(at(y - 9.0, &[(TZ_X, "Difference")]));
    items
}
