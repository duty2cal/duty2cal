// SPDX-License-Identifier: MPL-2.0
//! Test support: build a minimal, uncompressed PDF from positioned text items.
//!
//! Used to generate fictional duty-plan fixtures at test time, so no real plan
//! or committed binary fixture is needed (see the change's fixture decision).

/// Escape a string for a PDF literal string.
fn escape(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

/// Build a one-page PDF placing each `(x, y, text)` item with Helvetica 9pt.
///
/// Coordinates are in page space (origin bottom-left); the page is A4-sized.
pub fn build_pdf(items: &[(f64, f64, &str)]) -> Vec<u8> {
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

/// The detail-table header of the supported family, as positioned text items.
///
/// The heading row carries the local-date, local-report and UTC-time column
/// labels that family recognition looks for.
pub fn detail_header_row(y: f64) -> Vec<(f64, f64, &'static str)> {
    vec![
        (40.0, y, "Date"),
        (85.0, y, "(LT)"),
        (150.0, y, "Trip"),
        (210.0, y, "Report"),
        (265.0, y, "(LT)"),
        (320.0, y, "Activity"),
        (400.0, y, "From"),
        (450.0, y, "To"),
        (505.0, y, "Start"),
        (550.0, y, "(UTC)"),
        (610.0, y, "End"),
        (655.0, y, "(UTC)"),
    ]
}
