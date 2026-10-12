// SPDX-License-Identifier: MPL-2.0
//! Application: the driven ports, the `ConvertRoster` use case and its errors.
//!
//! Input crosses this boundary as bytes and output as text; no path or file
//! handle does, so the same use case is callable from the CLI, WASM and UniFFI.

use time::{Date, Month, OffsetDateTime, Time};

use crate::domain::{DutyEntry, Position, Roster, SnapshotId, UtcInterval};

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

/// Driven port: serializes a roster into a calendar format.
pub trait CalendarSink {
    /// Write the calendar text for a roster at a given revision instant.
    fn write(&self, roster: &Roster, revision: OffsetDateTime) -> String;
}

/// Driven port: the current instant, used for the revision timestamp fallback.
pub trait Clock {
    /// The current UTC instant.
    fn now_utc(&self) -> OffsetDateTime;
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
    /// A required timing could not be resolved.
    #[error("a required timing could not be resolved: {0}")]
    UnresolvedTiming(String),
    /// A timed interval was not strictly positive.
    #[error("a timed interval must end after it starts")]
    NonPositiveInterval,
}

/// A completed conversion: the calendar text to publish.
#[derive(Debug, Clone)]
pub struct ConversionOutcome {
    /// Calendar text ready to be written beside the source.
    pub calendar: String,
}

/// The activity codes the supported family treats as all-day.
const ALL_DAY_CODES: [&str; 4] = ["FREE", "OFF", "U", "FZ"];
/// The activity codes that denote a timed availability window.
const AVAILABILITY_CODES: [&str; 3] = ["SB", "RE", "SBX"];
/// The activity codes that denote timed non-flight duty.
const NON_FLIGHT_CODES: [&str; 3] = ["TK", "DTUSA", "EM"];

/// Turn the raw detail rows into the whole-plan roster.
///
/// Overview, statistics and footer material never reaches this function: the
/// adapter yields only rows inside the detail table. A row that carries neither
/// an activity code nor a route (a date-only row) contributes nothing.
fn interpret(table: &RawRosterTable, snapshot: SnapshotId) -> Result<Roster, ConvertError> {
    for label in ["Date", "Activity", "From", "To"] {
        if column(table, label).is_none() && label != "Activity" {
            return Err(ConvertError::UnresolvedRequiredActivity(format!(
                "the detail table carries no {label} column"
            )));
        }
    }

    let mut entries = Vec::new();
    for cells in &table.rows {
        let code = value(table, cells, "Activity");
        let from = value(table, cells, "From");
        let to = value(table, cells, "To");
        let date_cell = value(table, cells, "Date");

        if code.is_empty() && from.is_empty() && to.is_empty() {
            // A date-only row carries no activity and contributes none.
            continue;
        }

        let date = parse_date(&date_cell).ok_or_else(|| {
            ConvertError::UnresolvedRequiredActivity(format!("unresolved date: {date_cell:?}"))
        })?;
        let entry = classify(table, cells, &code, &from, &to, date)?;
        entries.push(entry);
    }

    Ok(Roster { snapshot, entries })
}

/// Classify one detail row into the activity it represents.
fn classify(
    table: &RawRosterTable,
    cells: &[String],
    code: &str,
    from: &str,
    to: &str,
    date: Date,
) -> Result<DutyEntry, ConvertError> {
    let tags = optional(table, cells, "Tags");
    let station = optional(table, cells, "From");

    if ALL_DAY_CODES.contains(&code) {
        return Ok(DutyEntry::AllDay {
            date,
            code: code.to_string(),
            tags,
        });
    }

    if AVAILABILITY_CODES.contains(&code) {
        return Ok(DutyEntry::Availability {
            date,
            code: code.to_string(),
            station,
            interval: interval(table, cells)?,
            tags,
        });
    }

    if NON_FLIGHT_CODES.contains(&code) {
        return Ok(DutyEntry::NonFlightDuty {
            date,
            code: code.to_string(),
            station,
            trip: optional(table, cells, "Trip"),
            interval: interval(table, cells)?,
            tags,
        });
    }

    if code.eq_ignore_ascii_case("Layover") {
        let Some(station) = station else {
            return Err(ConvertError::UnresolvedRequiredActivity(
                "a layover row carries no station".to_string(),
            ));
        };
        return Ok(DutyEntry::Layover { date, station });
    }

    if !from.is_empty() && !to.is_empty() {
        let position = if value(table, cells, "Pos").eq_ignore_ascii_case("DH") {
            Position::Deadhead
        } else {
            Position::Operating
        };
        return Ok(DutyEntry::Travel {
            date,
            designator: code.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            position,
            interval: interval(table, cells)?,
            trip: optional(table, cells, "Trip"),
            report_lt: optional(table, cells, "Report"),
            aircraft: optional(table, cells, "A/C"),
            blocktime: optional(table, cells, "Blocktime"),
            tz_diff: optional(table, cells, "Timezone Difference"),
            layover: optional(table, cells, "Layover"),
            tags,
        });
    }

    Err(ConvertError::UnresolvedRequiredActivity(
        if code.is_empty() {
            "a detail row carries neither an activity code nor a route".to_string()
        } else {
            code.to_string()
        },
    ))
}

/// The printed UTC interval of a timed row.
fn interval(table: &RawRosterTable, cells: &[String]) -> Result<UtcInterval, ConvertError> {
    let start = value(table, cells, "Start");
    let end = value(table, cells, "End");
    let start_date = value(table, cells, "Date");
    let date = parse_date(&start_date).ok_or_else(|| {
        ConvertError::UnresolvedTiming(format!("unresolved date: {start_date:?}"))
    })?;

    let start_time = parse_time(&start)
        .ok_or_else(|| ConvertError::UnresolvedTiming(format!("unresolved start: {start:?}")))?;
    let end_time = parse_time(&end)
        .ok_or_else(|| ConvertError::UnresolvedTiming(format!("unresolved end: {end:?}")))?;

    let start = combine(date, start_time);
    let end = combine(date, end_time);
    UtcInterval::new(start, end).map_err(|_| ConvertError::NonPositiveInterval)
}

fn combine(date: Date, time: Time) -> OffsetDateTime {
    date.with_time(time).assume_utc()
}

/// A column's index, matched by its derived header label.
///
/// Word extraction merges adjacent label words without a space, so labels are
/// compared with their punctuation and spacing removed.
fn column(table: &RawRosterTable, label: &str) -> Option<usize> {
    let wanted = normalise(label);
    table
        .columns
        .iter()
        .position(|column| normalise(column).starts_with(&wanted))
}

/// Lowercase letters and digits only.
fn normalise(text: &str) -> String {
    text.chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>()
        .to_ascii_lowercase()
}

fn value(table: &RawRosterTable, cells: &[String], label: &str) -> String {
    column(table, label)
        .and_then(|index| cells.get(index))
        .map(|cell| cell.trim().to_string())
        .unwrap_or_default()
}

fn optional(table: &RawRosterTable, cells: &[String], label: &str) -> Option<String> {
    let value = value(table, cells, label);
    (!value.is_empty()).then_some(value)
}

/// Parse an ISO calendar date (`YYYY-MM-DD`).
fn parse_date(text: &str) -> Option<Date> {
    let mut parts = text.split('-');
    let year: i32 = parts.next()?.trim().parse().ok()?;
    let month: u8 = parts.next()?.trim().parse().ok()?;
    let day: u8 = parts.next()?.trim().parse().ok()?;
    Date::from_calendar_date(year, Month::try_from(month).ok()?, day).ok()
}

/// Parse a printed UTC clock (`HH:MM` or `HH:MM:SS`), ignoring a day qualifier.
fn parse_time(text: &str) -> Option<Time> {
    let clock = text.rsplit(')').next().unwrap_or(text).trim();
    let mut parts = clock.split(':');
    let hour: u8 = parts.next()?.trim().parse().ok()?;
    let minute: u8 = parts.next()?.trim().parse().ok()?;
    let second: u8 = parts
        .next()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    Time::from_hms(hour, minute, second).ok()
}

/// A stable fingerprint of the supplied bytes, scoping the roster's identity.
fn fingerprint(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Driving port: convert one supplied duty plan.
#[derive(Debug)]
pub struct ConvertRoster<S: RosterSource, K: CalendarSink, C: Clock> {
    source: S,
    sink: K,
    clock: C,
}

impl<S: RosterSource, K: CalendarSink, C: Clock> ConvertRoster<S, K, C> {
    /// Wire the use case to its driven ports.
    pub fn new(source: S, sink: K, clock: C) -> Self {
        Self {
            source,
            sink,
            clock,
        }
    }

    /// Convert one supplied plan, or explain why it could not be converted.
    pub fn execute(&self, input: &[u8]) -> Result<ConversionOutcome, ConvertError> {
        let table = self.source.read(input)?;
        let roster = interpret(&table, SnapshotId(fingerprint(input)))?;
        let revision = self.clock.now_utc();
        Ok(ConversionOutcome {
            calendar: self.sink.write(&roster, revision),
        })
    }
}
