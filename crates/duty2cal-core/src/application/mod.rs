// SPDX-License-Identifier: MPL-2.0
//! Application: the driven ports, the `ConvertRoster` use case and its errors.
//!
//! Input crosses this boundary as bytes and output as text; no path or file
//! handle does, so the same use case is callable from the CLI, WASM and UniFFI.

use crate::adapters::ics;
use crate::domain::Roster;

/// Raw, position-based content of a supplied document — the anti-corruption
/// intermediate between the PDF adapter and interpretation.
#[derive(Debug, Clone, Default)]
pub struct RawRosterTable {
    /// Column labels derived from the detail-table header, left to right.
    pub columns: Vec<String>,
    /// Rows of cell text below the header, in source order.
    pub rows: Vec<Vec<String>>,
}

/// Why a supplied source could not be read as a supported plan.
#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    /// The bytes are not a readable PDF.
    #[error("the source is not a readable PDF: {0}")]
    Unreadable(String),
    /// The document is readable but is not the supported source family.
    #[error("the source is not a supported Released Roster detail-table export")]
    UnsupportedFamily,
}

/// Driven port: yields raw rows from a supplied document.
pub trait RosterSource {
    /// Read one document into the raw table, or explain why it cannot be read.
    fn read(&self, bytes: &[u8]) -> Result<RawRosterTable, SourceError>;
}

/// Why a conversion could not be completed.
#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    /// The source could not be read as a supported plan.
    #[error(transparent)]
    Source(#[from] SourceError),
    /// A required source activity could not be resolved.
    #[error("a required activity could not be resolved: {0}")]
    UnresolvedRequiredActivity(String),
}

/// A completed conversion: the calendar text to publish.
#[derive(Debug, Clone)]
pub struct ConversionOutcome {
    /// Calendar text ready to be written beside the source.
    pub calendar: String,
}

/// The detail-table column that carries the activity code.
const ACTIVITY_COLUMN: &str = "Activity";

/// Turn the raw detail rows into the whole-plan roster.
///
/// Overview, statistics and footer material never reaches this function: the
/// adapter yields only rows below the detail-table header. A row that carries no
/// activity code (a date-only row) contributes nothing.
fn interpret(table: &RawRosterTable) -> Result<Roster, ConvertError> {
    let Some(activity_index) = table
        .columns
        .iter()
        .position(|column| column.eq_ignore_ascii_case(ACTIVITY_COLUMN))
    else {
        return Err(ConvertError::UnresolvedRequiredActivity(
            "the detail table carries no activity column".to_string(),
        ));
    };

    let activities = Vec::new();
    for row in &table.rows {
        let code = row
            .get(activity_index)
            .map(|cell| cell.trim())
            .filter(|cell| !cell.is_empty());
        if let Some(code) = code {
            return Err(ConvertError::UnresolvedRequiredActivity(code.to_string()));
        }
        // A date-only row carries no activity and contributes none.
    }

    let roster = Roster { activities };
    debug_assert!(roster.is_complete());
    Ok(roster)
}

/// Driving port: convert one supplied duty plan.
#[derive(Debug)]
pub struct ConvertRoster<S: RosterSource> {
    source: S,
}

impl<S: RosterSource> ConvertRoster<S> {
    /// Wire the use case to its driven port.
    pub fn new(source: S) -> Self {
        Self { source }
    }

    /// Convert one supplied plan, or explain why it could not be converted.
    pub fn execute(&self, input: &[u8]) -> Result<ConversionOutcome, ConvertError> {
        let table = self.source.read(input)?;
        let roster = interpret(&table)?;
        Ok(ConversionOutcome {
            calendar: ics::render(&roster),
        })
    }
}
