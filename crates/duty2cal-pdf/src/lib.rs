// SPDX-License-Identifier: MPL-2.0
//! PDF driven adapter: the project-owned extraction's positioned words →
//! `RosterSource`.
//!
//! The reading boundary is the positioned word (`reader`); the detail-table
//! construction (`table`) is one implementation shared by the executable and
//! the browser. No third-party PDF extractor is part of the shipped graph; the
//! extractor survives only as a test oracle.

use duty2cal_core::application::{RosterSource, SourceDocument, SourceError};

mod reader;
mod table;

#[cfg(test)]
mod differential_tests;

/// Reads a supplied document with the project-owned extraction.
#[derive(Debug, Default)]
pub struct PdfRosterSource;

impl RosterSource for PdfRosterSource {
    fn read(&self, bytes: &[u8]) -> Result<SourceDocument, SourceError> {
        table::build(reader::extract(bytes)?)
    }
}
