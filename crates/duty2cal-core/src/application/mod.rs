// SPDX-License-Identifier: MPL-2.0
//! Application: the driven ports and the `ConvertRoster` use case.
//!
//! Input crosses this boundary as bytes and output as text; no path or file
//! handle does, so the same use case is callable from the CLI, WASM and UniFFI.

/// Raw, position-based content of a supplied document — the anti-corruption
/// intermediate between the PDF adapter and the domain.
#[derive(Debug, Clone, Default)]
pub struct RawRosterTable {
    /// Rows of cell text, in source order.
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
    /// The plan is supported but this build cannot represent it yet.
    #[error("no supported activity representation is available for this plan")]
    UnsupportedPlan,
}

/// A completed conversion: the published calendar text.
#[derive(Debug, Clone)]
pub struct ConversionOutcome {
    /// Calendar text ready to be written beside the source.
    pub calendar: String,
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
    ///
    /// Interpretation and calendar composition arrive with the following
    /// scenarios; until then no plan can be represented, so the conversion
    /// refuses explicitly instead of publishing a partial calendar.
    pub fn execute(&self, input: &[u8]) -> Result<ConversionOutcome, ConvertError> {
        let _table = self.source.read(input)?;
        Err(ConvertError::UnsupportedPlan)
    }
}
