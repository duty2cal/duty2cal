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

/// Escape a string for a PDF literal string.
fn escape(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

/// Build a one-page PDF placing each `(x, y, text)` item with Helvetica 9pt.
///
/// Coordinates are in page space (origin bottom-left); the page is A4-sized.
pub fn build_pdf(items: &[(f64, f64, String)]) -> Vec<u8> {
    let mut content = String::from("BT\n/F1 9 Tf\n");
    for (x, y, text) in items {
        content.push_str(&format!("1 0 0 1 {x} {y} Tm ({}) Tj\n", escape(text)));
    }
    content.push_str("ET\n");

    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] \
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
        ],
    ));
    items
}
