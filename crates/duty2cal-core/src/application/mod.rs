// SPDX-License-Identifier: MPL-2.0
//! Application: the driven ports, the `ConvertRoster` use case and its errors.
//!
//! Input crosses this boundary as bytes and output as text; no path or file
//! handle does, so the same use case is callable from the CLI, WASM and UniFFI.

use time::{Date, Duration, OffsetDateTime};

use crate::domain::timing;
use crate::domain::{
    printed_identification, DutyEntry, FieldValue, Roster, SnapshotId, UtcInterval,
};

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

/// A supplied document: its raw detail rows and its own revision metadata.
#[derive(Debug, Clone, Default)]
pub struct SourceDocument {
    /// The detail-table content.
    pub table: RawRosterTable,
    /// The source's own revision instant, when it states one.
    pub revision: Option<OffsetDateTime>,
}

/// Driven port: yields raw rows from a supplied document.
pub trait RosterSource {
    /// Read one document into its raw content, or explain why it cannot be read.
    fn read(&self, bytes: &[u8]) -> Result<SourceDocument, SourceError>;
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

/// How a diagnostic affects publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// The row cannot be represented; any error prevents publication.
    Error,
    /// The row was represented by a documented fallback.
    Warning,
}

/// One reported finding about one detail row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Which severity a reader must apply.
    pub severity: Severity,
    /// 1-based position of the row among the rows the adapter yielded for the
    /// detail table, counting silent layout rows.
    pub row: usize,
    /// The row's own duty-plan date, or the carried date of its date block.
    pub date: Option<Date>,
    /// The complete sentence a reader sees.
    pub message: String,
}

impl Diagnostic {
    /// An error about one detail row.
    fn error(row: usize, date: Option<Date>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            row,
            date,
            message: message.into(),
        }
    }

    /// A warning about one detail row.
    fn warning(row: usize, date: Option<Date>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            row,
            date,
            message: message.into(),
        }
    }
}

/// The single spelling of one diagnostic in a report, a transcript, or a page.
pub fn diagnostic_line(diagnostic: &Diagnostic) -> String {
    match diagnostic.date {
        Some(date) => format!("row {} ({}): {}", diagnostic.row, date, diagnostic.message),
        None => format!("row {}: {}", diagnostic.row, diagnostic.message),
    }
}

/// The complete failure report for row-level errors.
pub fn failure_report_text(source_name: &str, diagnostics: &[Diagnostic]) -> String {
    let reason = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.severity == Severity::Error)
        .map(|diagnostic| diagnostic.message.as_str())
        .unwrap_or("the conversion failed");
    let mut report = format!("Source PDF: {source_name}\nReason: {reason}\n");
    append_section(&mut report, "Errors", diagnostics, Severity::Error);
    append_section(&mut report, "Warnings", diagnostics, Severity::Warning);
    report
}

/// The complete warning report for a conversion that published a calendar.
pub fn warning_report_text(source_name: &str, warnings: &[Diagnostic]) -> String {
    let mut report = format!("Source PDF: {source_name}\n");
    append_section(&mut report, "Warnings", warnings, Severity::Warning);
    report
}

/// Append one severity section with its entries, or nothing when it is empty.
fn append_section(report: &mut String, title: &str, diagnostics: &[Diagnostic], severity: Severity) {
    let matching: Vec<&Diagnostic> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == severity)
        .collect();
    if matching.is_empty() {
        return;
    }
    report.push_str(title);
    report.push_str(":\n");
    for diagnostic in matching {
        report.push_str("- ");
        report.push_str(&diagnostic_line(diagnostic));
        report.push('\n');
    }
}

/// Why a conversion could not be completed.
#[derive(Debug)]
pub enum ConvertError {
    /// The source could not be read as a supported plan.
    Source(SourceError),
    /// The detail table itself is unusable (a required column is missing).
    Table(String),
    /// Row-level errors; carries every error and warning of the run.
    Rows(Vec<Diagnostic>),
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Source(error) => write!(formatter, "{error}"),
            Self::Table(message) => {
                write!(
                    formatter,
                    "a required activity could not be resolved: {message}"
                )
            }
            Self::Rows(diagnostics) => match diagnostics
                .iter()
                .find(|diagnostic| diagnostic.severity == Severity::Error)
            {
                Some(diagnostic) => write!(formatter, "{}", diagnostic.message),
                None => write!(formatter, "the conversion failed"),
            },
        }
    }
}

impl std::error::Error for ConvertError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Source(error) => Some(error),
            Self::Table(_) | Self::Rows(_) => None,
        }
    }
}

impl From<SourceError> for ConvertError {
    fn from(error: SourceError) -> Self {
        Self::Source(error)
    }
}

/// A completed conversion: the calendar text and every fallback it used.
#[derive(Debug, Clone)]
pub struct ConversionOutcome {
    /// Calendar text ready to be written beside the source.
    pub calendar: String,
    /// Every fallback the run used, in plan order.
    pub warnings: Vec<Diagnostic>,
}

/// The activity codes the supported family treats as all-day.
const ALL_DAY_CODES: [&str; 4] = ["FREE", "OFF", "U", "FZ"];
/// The activity codes that denote a timed availability window.
const AVAILABILITY_CODES: [&str; 3] = ["SB", "RE", "SBX"];
/// The activity codes that denote timed non-flight duty.
const NON_FLIGHT_CODES: [&str; 3] = ["TK", "DTUSA", "EM"];

/// One detail row with its cells resolved by column label.
struct Row {
    /// 1-based position in the adapter's raw table, counting silent rows.
    raw: usize,
    /// Position in the filtered row list used for pairing.
    index: usize,
    date: Date,
    /// The plan's `Activity` cell, as printed.
    activity: String,
    from: String,
    to: String,
    start: String,
    end: String,
    /// The crew position exactly as the plan prints it, when it prints one.
    ///
    /// The pairing and the title read this cell, not a derived assignment:
    /// two rows of one leg print the same position, so a row that prints a
    /// different one — or none — is a different printed fragment.
    position: Option<String>,
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
    /// A complete activity represented by a documented fallback.
    Warning(DutyEntry, String),
    /// A row that cannot be represented, with its complete sentence.
    Error(String),
}

/// The resolved rows and the row-level findings collected while reading them.
struct Interpretation {
    entries: Vec<DutyEntry>,
    diagnostics: Vec<Diagnostic>,
}

/// Turn the raw detail rows into the whole-plan roster.
///
/// Overview, statistics and footer material never reaches this function: the
/// adapter yields only rows inside the detail table. A row that carries neither
/// an activity code nor a route (a date-only row) contributes nothing.
fn interpret(table: &RawRosterTable) -> Result<Interpretation, ConvertError> {
    for label in ["Date", "From", "To"] {
        if column(table, label).is_none() {
            return Err(ConvertError::Table(format!(
                "the detail table carries no {label} column"
            )));
        }
    }

    let (rows, mut diagnostics) = collect_rows(table);
    let mut entries: Vec<(usize, DutyEntry)> = Vec::new();
    let mut pending: Vec<usize> = Vec::new();

    for row in &rows {
        match classify(table, row) {
            Classified::Entry(entry) => entries.push((row.index, entry)),
            Classified::Departure(index) => pending.push(index),
            Classified::Arrival(index) => {
                let arrival = &rows[index];
                let partner = pending.iter().rposition(|departure| {
                    let departure = &rows[*departure];
                    departure.activity == arrival.activity && departure.position == arrival.position
                });
                let Some(position) = partner else {
                    diagnostics.push(Diagnostic::error(
                        arrival.raw,
                        Some(arrival.date),
                        unmatched_fragment(arrival.position.as_deref(), &arrival.activity),
                    ));
                    continue;
                };
                let departure = pending.remove(position);
                let trip = rows[departure]
                    .trip
                    .clone()
                    .or_else(|| rows[departure].carried_trip.clone());
                match merge_travel(&rows[departure], arrival, trip) {
                    Ok(entry) => entries.push((rows[departure].index, entry)),
                    Err(message) => diagnostics.push(Diagnostic::error(
                        rows[departure].raw,
                        Some(rows[departure].date),
                        message,
                    )),
                }
            }
            Classified::Warning(entry, message) => {
                entries.push((row.index, entry));
                diagnostics.push(Diagnostic::warning(row.raw, Some(row.date), message));
            }
            Classified::Error(message) => diagnostics.push(Diagnostic::error(
                row.raw,
                Some(row.date),
                message,
            )),
        }
    }

    // Every departure that never found its arrival is a row that cannot be
    // represented; report all of them, not only the first.
    for departure in pending {
        let row = &rows[departure];
        diagnostics.push(Diagnostic::error(
            row.raw,
            Some(row.date),
            unmatched_fragment(row.position.as_deref(), &row.activity),
        ));
    }

    // A split leg belongs where it departs, whatever order the fragments appear.
    entries.sort_by_key(|(index, _)| *index);
    diagnostics.sort_by_key(|diagnostic| diagnostic.row);

    if diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == Severity::Error)
    {
        return Err(ConvertError::Rows(diagnostics));
    }

    Ok(Interpretation {
        entries: entries.into_iter().map(|(_, entry)| entry).collect(),
        diagnostics,
    })
}

/// The failure sentence for a travel fragment whose partner is missing.
///
/// The fragment is named through the same rule the title uses, so a crew
/// member reads the leg the same way in the calendar and in the failure.
fn unmatched_fragment(position: Option<&str>, activity: &str) -> String {
    format!(
        "a required activity could not be resolved: no unique compatible partner for travel fragment {:?}",
        printed_identification(position, activity)
    )
}

/// Build one `Row` per detail row that carries reachable activity data.
///
/// A row whose date cannot be resolved contributes an error and no row; the
/// carried date is not advanced by it.
fn collect_rows(table: &RawRosterTable) -> (Vec<Row>, Vec<Diagnostic>) {
    let mut rows = Vec::new();
    let mut diagnostics = Vec::new();
    let mut carried_trip: Option<String> = None;
    let mut carried_date: Option<Date> = None;
    for (position, cells) in table.rows.iter().enumerate() {
        let raw = position + 1;
        let layover = layover_station(cells);
        let mut activity = value(table, cells, "Activity");
        let position_in_table = optional(table, cells, "Pos");
        let mut from = value(table, cells, "From");
        let to = value(table, cells, "To");
        if let Some(station) = &layover {
            activity = "Layover".to_string();
            from = station.clone();
        }
        if activity.is_empty() && from.is_empty() && to.is_empty() {
            continue;
        }
        let date_cell = value(table, cells, "Date");
        let date = if date_cell.is_empty() {
            // A continuation row prints no date of its own: the local date it
            // belongs to is the one the row above carries.
            match carried_date {
                Some(date) => date,
                None => {
                    diagnostics.push(Diagnostic::error(
                        raw,
                        None,
                        "a required activity could not be resolved: a detail row prints no date and no earlier row supplies one",
                    ));
                    continue;
                }
            }
        } else {
            match timing::parse_iso_date(&date_cell) {
                Some(date) => date,
                None => {
                    diagnostics.push(Diagnostic::error(
                        raw,
                        None,
                        format!(
                            "a required activity could not be resolved: unresolved date: {date_cell:?}"
                        ),
                    ));
                    continue;
                }
            }
        };
        carried_date = Some(date);
        let own_trip = optional(table, cells, "Trip");
        if let Some(trip) = &own_trip {
            carried_trip = Some(trip.clone());
        }
        // The pairing works on this list, so a row carries its position in it,
        // not its position among every row the table prints.
        let index = rows.len();
        rows.push(Row {
            raw,
            index,
            date,
            activity,
            from,
            to,
            start: value(table, cells, "Start"),
            end: value(table, cells, "End"),
            position: position_in_table,
            trip: own_trip,
            carried_trip: carried_trip.clone(),
            report_lt: optional(table, cells, "Report"),
            aircraft: optional(table, cells, "A/C"),
            blocktime: optional(table, cells, "Blocktime"),
            tz_diff: optional(table, cells, "Timezone Difference"),
            layover: optional(table, cells, "Layover"),
            tags: tags(table, cells),
        });
    }
    (rows, diagnostics)
}

/// The station of a row that prints the family's `Layover:` marker.
///
/// The marker row prints no activity code and no route; its station follows the
/// marker in the same cell or in the next one the row prints.
fn layover_station(cells: &[String]) -> Option<String> {
    const MARKER: &str = "layover:";
    for (index, cell) in cells.iter().enumerate() {
        let lower = cell.to_ascii_lowercase();
        let Some(at) = lower.find(MARKER) else {
            continue;
        };
        let tail = cell[at + MARKER.len()..]
            .trim()
            .trim_end_matches(',')
            .trim();
        if !tail.is_empty() {
            return Some(tail.to_string());
        }
        return cells[index + 1..]
            .iter()
            .map(|next| next.trim())
            .find(|next| !next.is_empty())
            .map(str::to_string);
    }
    None
}

/// The row's tags, without the all-day marker the family prints beside them.
fn tags(table: &RawRosterTable, cells: &[String]) -> Option<String> {
    let field = value(table, cells, "Tags");
    let kept: Vec<&str> = field
        .split_whitespace()
        .filter(|tag| *tag != "==")
        .collect();
    (!kept.is_empty()).then(|| kept.join(" "))
}

/// Classify one detail row into the activity it represents.
///
/// Known codes win over the route branch, so a known timed code that also
/// prints a route keeps its known kind. A route wins over the fallback, so a
/// flight number never warns. The final arm is exhaustive: no row leaves
/// classification without an entry or a diagnostic.
fn classify(table: &RawRosterTable, row: &Row) -> Classified {
    let code = row.activity.as_str();
    let _ = table;

    if ALL_DAY_CODES.contains(&code) {
        return Classified::Entry(DutyEntry::AllDay {
            date: row.date,
            code: row.activity.clone(),
            tags: row.tags.clone(),
        });
    }

    if AVAILABILITY_CODES.contains(&code) {
        return match row_interval(row) {
            Ok(interval) => Classified::Entry(DutyEntry::Availability {
                date: row.date,
                code: row.activity.clone(),
                station: station(row),
                interval,
                tags: row.tags.clone(),
            }),
            Err(message) => Classified::Error(message),
        };
    }

    if NON_FLIGHT_CODES.contains(&code) {
        return match row_interval(row) {
            Ok(interval) => Classified::Entry(DutyEntry::NonFlightDuty {
                date: row.date,
                code: row.activity.clone(),
                station: station(row),
                trip: row.trip.clone(),
                interval,
                tags: row.tags.clone(),
            }),
            Err(message) => Classified::Error(message),
        };
    }

    if code.eq_ignore_ascii_case("Layover") {
        // A row carries this code only when its marker named a station, which is
        // why the station needs no second check here.
        return Classified::Entry(DutyEntry::Layover {
            date: row.date,
            station: row.from.clone(),
        });
    }

    let has_route = !row.from.is_empty() && !row.to.is_empty();
    let has_endpoints = !row.start.is_empty() && !row.end.is_empty();
    if has_route && has_endpoints {
        return match row_interval(row) {
            Ok(interval) => Classified::Entry(DutyEntry::Travel {
                date: row.date,
                activity: row.activity.clone(),
                position: row.position.clone(),
                from: row.from.clone(),
                to: row.to.clone(),
                interval,
                trip: row.trip.clone().or_else(|| row.carried_trip.clone()),
                report_lt: single(row.report_lt.clone()),
                aircraft: single(row.aircraft.clone()),
                blocktime: single(row.blocktime.clone()),
                tz_diff: single(row.tz_diff.clone()),
                layover: single(row.layover.clone()),
                tags: single(row.tags.clone()),
            }),
            Err(message) => Classified::Error(message),
        };
    }
    if !row.from.is_empty() && !row.start.is_empty() && row.to.is_empty() && row.end.is_empty() {
        return Classified::Departure(row.index);
    }
    if !row.to.is_empty() && !row.end.is_empty() && row.from.is_empty() && row.start.is_empty() {
        return Classified::Arrival(row.index);
    }

    if !has_route {
        if has_endpoints {
            return match row_interval(row) {
                Ok(interval) => Classified::Warning(
                    DutyEntry::UnrecognizedActivity {
                        date: row.date,
                        code: row.activity.clone(),
                        station: station(row),
                        interval: Some(interval),
                    },
                    format!(
                        "unrecognized activity code {:?}; converted as a timed activity using its printed UTC window {}-{}",
                        row.activity, row.start, row.end
                    ),
                ),
                Err(message) => Classified::Error(message),
            };
        }
        let has_one_endpoint = !row.start.is_empty() || !row.end.is_empty();
        if has_one_endpoint {
            return Classified::Error(format!(
                "a required activity could not be resolved: unrecognized activity code {:?} with only one printed UTC endpoint",
                row.activity
            ));
        }
        if !row.activity.is_empty() {
            return Classified::Warning(
                DutyEntry::UnrecognizedActivity {
                    date: row.date,
                    code: row.activity.clone(),
                    station: station(row),
                    interval: None,
                },
                format!(
                    "unrecognized activity code {:?}; converted as an all-day activity",
                    row.activity
                ),
            );
        }
        return Classified::Error(
            "a required activity could not be resolved: a detail row carries neither an activity code nor a route"
                .to_string(),
        );
    }

    Classified::Error(
        "a required activity could not be resolved: a route row carries an incomplete printed interval"
            .to_string(),
    )
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
) -> Result<DutyEntry, String> {
    let start = resolve(departure.date, &departure.start)?;
    let end = resolve(arrival.date, &arrival.end)?;
    Ok(DutyEntry::Travel {
        date: departure.date,
        activity: departure.activity.clone(),
        position: departure.position.clone(),
        from: departure.from.clone(),
        to: arrival.to.clone(),
        interval: UtcInterval::new(start, end)
            .map_err(|_| "a timed interval must end after it starts".to_string())?,
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
fn row_interval(row: &Row) -> Result<UtcInterval, String> {
    let start = resolve(row.date, &row.start)?;
    let mut end = resolve(row.date, &row.end)?;

    // The narrow unqualified rollover: both endpoints unqualified in one local
    // block with no separate arrival block, and the end clock earlier than the
    // start. Equal clocks and qualified endpoints are never repaired.
    if end < start && !timing::is_qualified(&row.start) && !timing::is_qualified(&row.end) {
        end += Duration::days(1);
    }

    UtcInterval::new(start, end).map_err(|_| "a timed interval must end after it starts".to_string())
}

fn resolve(local_date: Date, cell: &str) -> Result<OffsetDateTime, String> {
    timing::resolve_endpoint(local_date, cell)
        .map_err(|error| format!("a required timing could not be resolved: {error}"))
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

/// A stable fingerprint of the supplied bytes, scoping the roster's identity.
fn fingerprint(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// The report a failed conversion offers beside the source.
///
/// The executable and the browser facade both call this function, so the two
/// interfaces cannot drift into different spellings. The page owns the offered
/// name; this function owns the two lines.
pub fn diagnostic_text(source_name: &str, reason: &str) -> String {
    format!("Source PDF: {source_name}\nReason: {reason}\n")
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
        let document = self.source.read(input)?;
        let interpretation = interpret(&document.table)?;
        let roster = Roster {
            snapshot: SnapshotId(fingerprint(input)),
            entries: interpretation.entries,
        };
        // The source's own revision metadata is preferred; the constructed
        // representation's clock is the fallback.
        let revision = document.revision.unwrap_or_else(|| self.clock.now_utc());
        Ok(ConversionOutcome {
            calendar: self.sink.write(&roster, revision),
            warnings: interpretation.diagnostics,
        })
    }
}
