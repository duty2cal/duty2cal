// SPDX-License-Identifier: MPL-2.0
//! PDF driven adapter: the project-owned extraction's positioned words →
//! `RosterSource`.
//!
//! The reading boundary is the positioned word (`reader`); the detail-table
//! construction (`table`) is one implementation shared by the executable and
//! the browser. The supported plan's file structure is read by the project
//! itself; only the pure-Rust `miniz_oxide` decompressor remains third-party in
//! the reading path. No third-party PDF extractor is part of the shipped graph;
//! the extractor survives only as a test oracle. The frozen subset is ADR-0008's
//! contract: a construct outside it is refused by name rather than guessed at.

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
