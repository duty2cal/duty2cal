// SPDX-License-Identifier: MPL-2.0
//! PDF driven adapter: `pdfplumber` positioned content → `RosterSource`.
//! The only crate that depends on the extraction dependency.

use duty2cal_core::application::{RawRosterTable, RosterSource, SourceError};
use pdfplumber::{Pdf, TextOptions};

/// Reads a supplied document with the pinned extraction crate.
#[derive(Debug, Default)]
pub struct PdfRosterSource;

impl RosterSource for PdfRosterSource {
    fn read(&self, bytes: &[u8]) -> Result<RawRosterTable, SourceError> {
        let pdf = Pdf::open_bytes(bytes, None)
            .map_err(|error| SourceError::Unreadable(error.to_string()))?;

        let mut text = String::new();
        for page in pdf.pages() {
            let page = page.map_err(|error| SourceError::Unreadable(error.to_string()))?;
            text.push_str(&page.extract_text(&TextOptions::default()));
        }

        if !looks_like_released_roster(&text) {
            return Err(SourceError::UnsupportedFamily);
        }

        // Column bands, row clustering and cell assignment arrive with the
        // detail-table scenarios; the family is recognised here so that an
        // unsupported document is rejected before any interpretation.
        Ok(RawRosterTable::default())
    }
}

/// The supported family's detail table carries the local-date, local-report and
/// UTC-time column headings.
fn looks_like_released_roster(text: &str) -> bool {
    text.contains("Date") && text.contains("(LT)") && text.contains("(UTC)")
}
