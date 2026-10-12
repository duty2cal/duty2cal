// SPDX-License-Identifier: MPL-2.0
//! Application: the driven ports, the `ConvertRoster` use case and its errors.
//!
//! Input crosses this boundary as bytes and output as text; no path or file
//! handle does, so the same use case is callable from the CLI, WASM and UniFFI.

use time::{Date, Duration, Month, OffsetDateTime};

use crate::domain::timing;
use crate::domain::{DutyEntry, FieldValue, Position, Roster, SnapshotId, UtcInterval};

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

/// One detail row with its cells resolved by column label.
struct Row {
    index: usize,
    date: Date,
    code: String,
    from: String,
    to: String,
    start: String,
    end: String,
    position: Position,
    /// The identifier printed on this row, when it prints one.
    trip: Option<String>,
    /// The identifier of the roster trip this row belongs to, carried from an
    /// earlier leg when this row prints none.
    carried_trip: Option<String>,
    report_lt: Option<String>,
    aircraft: Option<String>,
    blocktime: Option<String>,
    tz_diff: Option<String>,
    layover: Option<String>,
    tags: Option<String>,
}

/// What one detail row resolves to.
///
/// The variants are transient: a fragment is immediately paired into a travel
/// leg, so boxing the entry would only add an allocation.
#[allow(clippy::large_enum_variant)]
enum Classified {
    /// A complete activity.
    Entry(DutyEntry),
    /// A departure fragment of a travel leg, awaiting its arrival fragment.
    Departure(usize),
    /// An arrival fragment of a travel leg.
    Arrival(usize),
}

/// Turn the raw detail rows into the whole-plan roster.
///
/// Overview, statistics and footer material never reaches this function: the
/// adapter yields only rows inside the detail table. A row that carries neither
/// an activity code nor a route (a date-only row) contributes nothing.
fn interpret(table: &RawRosterTable, snapshot: SnapshotId) -> Result<Roster, ConvertError> {
    for label in ["Date", "From", "To"] {
        if column(table, label).is_none() {
            return Err(ConvertError::UnresolvedRequiredActivity(format!(
                "the detail table carries no {label} column"
            )));
        }
    }

    let rows = collect_rows(table)?;
    let mut entries: Vec<(usize, DutyEntry)> = Vec::new();
    let mut pending: Vec<usize> = Vec::new();

    for row in &rows {
        match classify(table, row)? {
            Classified::Entry(entry) => entries.push((row.index, entry)),
            Classified::Departure(index) => pending.push(index),
            Classified::Arrival(index) => {
                let arrival = &rows[index];
                let position = pending
                    .iter()
                    .rposition(|departure| rows[*departure].code == arrival.code);
                let Some(position) = position else {
                    return Err(unmatched_fragment(&arrival.code));
                };
                let departure = pending.remove(position);
                let trip = rows[departure]
                    .trip
                    .clone()
                    .or_else(|| rows[departure].carried_trip.clone());
                entries.push((
                    rows[departure].index,
                    merge_travel(&rows[departure], arrival, trip)?,
                ));
            }
        }
    }

    if let Some(unmatched) = pending.first() {
        return Err(unmatched_fragment(&rows[*unmatched].code));
    }

    // A split leg belongs where it departs, whatever order the fragments appear.
    entries.sort_by_key(|(index, _)| *index);
    Ok(Roster {
        snapshot,
        entries: entries.into_iter().map(|(_, entry)| entry).collect(),
    })
}

fn unmatched_fragment(designator: &str) -> ConvertError {
    ConvertError::UnresolvedRequiredActivity(format!(
        "no unique compatible partner for travel fragment {designator:?}"
    ))
}

/// Build one `Row` per detail row that carries reachable activity data.
fn collect_rows(table: &RawRosterTable) -> Result<Vec<Row>, ConvertError> {
    let mut rows = Vec::new();
    let mut carried_trip: Option<String> = None;
    for (index, cells) in table.rows.iter().enumerate() {
        let code = value(table, cells, "Activity");
        let from = value(table, cells, "From");
        let to = value(table, cells, "To");
        let date_cell = value(table, cells, "Date");
        if code.is_empty() && from.is_empty() && to.is_empty() {
            continue;
        }
        let date = parse_date(&date_cell).ok_or_else(|| {
            ConvertError::UnresolvedRequiredActivity(format!("unresolved date: {date_cell:?}"))
        })?;
        let own_trip = optional(table, cells, "Trip");
        if let Some(trip) = &own_trip {
            carried_trip = Some(trip.clone());
        }
        rows.push(Row {
            index,
            date,
            code,
            from,
            to,
            start: value(table, cells, "Start"),
            end: value(table, cells, "End"),
            position: if value(table, cells, "Pos").eq_ignore_ascii_case("DH") {
                Position::Deadhead
            } else {
                Position::Operating
            },
            trip: own_trip,
            carried_trip: carried_trip.clone(),
            report_lt: optional(table, cells, "Report"),
            aircraft: optional(table, cells, "A/C"),
            blocktime: optional(table, cells, "Blocktime"),
            tz_diff: optional(table, cells, "Timezone Difference"),
            layover: optional(table, cells, "Layover"),
            tags: optional(table, cells, "Tags"),
        });
    }
    Ok(rows)
}

/// Classify one detail row into the activity it represents.
fn classify(table: &RawRosterTable, row: &Row) -> Result<Classified, ConvertError> {
    let code = row.code.as_str();
    let _ = table;

    if ALL_DAY_CODES.contains(&code) {
        return Ok(Classified::Entry(DutyEntry::AllDay {
            date: row.date,
            code: row.code.clone(),
            tags: row.tags.clone(),
        }));
    }

    if AVAILABILITY_CODES.contains(&code) {
        return Ok(Classified::Entry(DutyEntry::Availability {
            date: row.date,
            code: row.code.clone(),
            station: station(row),
            interval: row_interval(row)?,
            tags: row.tags.clone(),
        }));
    }

    if NON_FLIGHT_CODES.contains(&code) {
        return Ok(Classified::Entry(DutyEntry::NonFlightDuty {
            date: row.date,
            code: row.code.clone(),
            station: station(row),
            trip: row.trip.clone(),
            interval: row_interval(row)?,
            tags: row.tags.clone(),
        }));
    }

    if code.eq_ignore_ascii_case("Layover") {
        return match station(row) {
            Some(station) => Ok(Classified::Entry(DutyEntry::Layover {
                date: row.date,
                station,
            })),
            None => Err(ConvertError::UnresolvedRequiredActivity(
                "a layover row carries no station".to_string(),
            )),
        };
    }

    let has_route = !row.from.is_empty() && !row.to.is_empty();
    let has_endpoints = !row.start.is_empty() && !row.end.is_empty();
    if has_route && has_endpoints {
        return Ok(Classified::Entry(DutyEntry::Travel {
            date: row.date,
            designator: row.code.clone(),
            from: row.from.clone(),
            to: row.to.clone(),
            position: row.position,
            interval: row_interval(row)?,
            trip: row.trip.clone().or_else(|| row.carried_trip.clone()),
            report_lt: single(row.report_lt.clone()),
            aircraft: single(row.aircraft.clone()),
            blocktime: single(row.blocktime.clone()),
            tz_diff: single(row.tz_diff.clone()),
            layover: single(row.layover.clone()),
            tags: single(row.tags.clone()),
        }));
    }
    if !row.from.is_empty() && !row.start.is_empty() && row.to.is_empty() && row.end.is_empty() {
        return Ok(Classified::Departure(row.index));
    }
    if !row.to.is_empty() && !row.end.is_empty() && row.from.is_empty() && row.start.is_empty() {
        return Ok(Classified::Arrival(row.index));
    }

    Err(ConvertError::UnresolvedRequiredActivity(
        if code.is_empty() {
            "a detail row carries neither an activity code nor a route".to_string()
        } else {
            row.code.clone()
        },
    ))
}

/// A value supplied by one fragment.
fn single(value: Option<String>) -> Option<FieldValue> {
    value.map(FieldValue::Single)
}

/// Combine the two fragments' values for one selected optional field.
///
/// A value supplied by one fragment, or supplied equally by both, stays a
/// single value; two different values are both retained.
fn combine(departure: Option<String>, arrival: Option<String>) -> Option<FieldValue> {
    match (departure, arrival) {
        (None, None) => None,
        (Some(value), None) | (None, Some(value)) => Some(FieldValue::Single(value)),
        (Some(departure), Some(arrival)) if departure == arrival => {
            Some(FieldValue::Single(departure))
        }
        (Some(departure), Some(arrival)) => Some(FieldValue::Differing { departure, arrival }),
    }
}

fn station(row: &Row) -> Option<String> {
    (!row.from.is_empty()).then(|| row.from.clone())
}

/// Merge a departure and an arrival fragment into one travel leg.
fn merge_travel(
    departure: &Row,
    arrival: &Row,
    trip: Option<String>,
) -> Result<DutyEntry, ConvertError> {
    let start = resolve(departure.date, &departure.start)?;
    let end = resolve(arrival.date, &arrival.end)?;
    Ok(DutyEntry::Travel {
        date: departure.date,
        designator: departure.code.clone(),
        from: departure.from.clone(),
        to: arrival.to.clone(),
        position: departure.position,
        interval: UtcInterval::new(start, end).map_err(|_| ConvertError::NonPositiveInterval)?,
        trip,
        report_lt: combine(departure.report_lt.clone(), arrival.report_lt.clone()),
        aircraft: combine(departure.aircraft.clone(), arrival.aircraft.clone()),
        blocktime: combine(departure.blocktime.clone(), arrival.blocktime.clone()),
        tz_diff: combine(departure.tz_diff.clone(), arrival.tz_diff.clone()),
        layover: combine(departure.layover.clone(), arrival.layover.clone()),
        tags: combine(departure.tags.clone(), arrival.tags.clone()),
    })
}

/// The interval of a row that carries both of its own endpoints.
fn row_interval(row: &Row) -> Result<UtcInterval, ConvertError> {
    let start = resolve(row.date, &row.start)?;
    let mut end = resolve(row.date, &row.end)?;

    // The narrow unqualified rollover: both endpoints unqualified in one local
    // block with no separate arrival block, and the end clock earlier than the
    // start. Equal clocks and qualified endpoints are never repaired.
    if end < start && !timing::is_qualified(&row.start) && !timing::is_qualified(&row.end) {
        end += Duration::days(1);
    }

    UtcInterval::new(start, end).map_err(|_| ConvertError::NonPositiveInterval)
}

fn resolve(local_date: Date, cell: &str) -> Result<OffsetDateTime, ConvertError> {
    timing::resolve_endpoint(local_date, cell)
        .map_err(|error| ConvertError::UnresolvedTiming(error.to_string()))
}

/// A column's index, matched by its derived header label.
///
/// Word extraction merges adjacent label words without a space, so a label is
/// matched when it equals the wanted name, continues it at a real boundary, or
/// normalises to it. A bare normalised prefix would collide (`A/C` vs
/// `Activity`).
fn column(table: &RawRosterTable, label: &str) -> Option<usize> {
    let wanted = label.to_ascii_lowercase();
    let wanted_normalised = normalise(label);
    table.columns.iter().position(|column| {
        let lower = column.to_ascii_lowercase();
        lower == wanted
            || lower.starts_with(&format!("{wanted} "))
            || lower.starts_with(&format!("{wanted}("))
            || normalise(column) == wanted_normalised
    })
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
