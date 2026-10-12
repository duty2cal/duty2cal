// SPDX-License-Identifier: MPL-2.0
//! Timing: resolving printed UTC cells to instants.
//!
//! A cell may carry an explicit day qualifier, written in parentheses, that
//! names the UTC day independently of the row's local date. The qualifier is
//! resolved to the unique previous, same or next Gregorian date around that
//! endpoint's own local date, carrying month and year.

use time::{Date, OffsetDateTime, Time};

/// Why a printed endpoint could not be resolved.
#[derive(Debug, thiserror::Error)]
pub enum TimingError {
    /// The clock text is not readable.
    #[error("the printed clock is not readable: {0:?}")]
    UnreadableClock(String),
    /// The day qualifier matches none, or more than one, adjacent date.
    #[error("the printed day qualifier matches no unique adjacent date: {0:?}")]
    NoUniqueAdjacentDate(String),
}

/// Whether a printed cell carries an explicit UTC day qualifier.
pub fn is_qualified(cell: &str) -> bool {
    cell.trim_start().starts_with('(')
}

/// Resolve a printed UTC cell against the row's own local date.
pub fn resolve_endpoint(local_date: Date, cell: &str) -> Result<OffsetDateTime, TimingError> {
    let (qualifier, clock) = parse_cell(cell)?;
    let date = match qualifier {
        None => local_date,
        Some(day) => adjacent_date(local_date, day)?,
    };
    Ok(date.with_time(clock).assume_utc())
}

/// Split a cell into its optional day qualifier and its clock.
fn parse_cell(cell: &str) -> Result<(Option<u8>, Time), TimingError> {
    let cell = cell.trim();
    let (qualifier, clock_text) = if let Some(rest) = cell.strip_prefix('(') {
        let (day, rest) = rest
            .split_once(')')
            .ok_or_else(|| TimingError::UnreadableClock(cell.to_string()))?;
        let day: u8 = day
            .trim()
            .parse()
            .map_err(|_| TimingError::UnreadableClock(cell.to_string()))?;
        (Some(day), rest.trim())
    } else {
        (None, cell)
    };

    let clock =
        parse_clock(clock_text).ok_or_else(|| TimingError::UnreadableClock(cell.to_string()))?;
    Ok((qualifier, clock))
}

/// Parse `HH:MM` or `HH:MM:SS`.
pub fn parse_clock(text: &str) -> Option<Time> {
    let mut parts = text.split(':');
    let hour: u8 = parts.next()?.trim().parse().ok()?;
    let minute: u8 = parts.next()?.trim().parse().ok()?;
    let second: u8 = parts
        .next()
        .and_then(|value| value.trim().parse().ok())
        .unwrap_or(0);
    Time::from_hms(hour, minute, second).ok()
}

/// Parse an ISO calendar date (`YYYY-MM-DD`).
pub fn parse_iso_date(text: &str) -> Option<Date> {
    let mut parts = text.split('-');
    let year: i32 = parts.next()?.trim().parse().ok()?;
    let month: u8 = parts.next()?.trim().parse().ok()?;
    let day: u8 = parts.next()?.trim().parse().ok()?;
    Date::from_calendar_date(year, time::Month::try_from(month).ok()?, day).ok()
}

/// The unique adjacent date whose day-of-month equals `day`.
fn adjacent_date(local_date: Date, day: u8) -> Result<Date, TimingError> {
    let candidates = [
        local_date.previous_day(),
        Some(local_date),
        local_date.next_day(),
    ];
    let matches: Vec<Date> = candidates
        .into_iter()
        .flatten()
        .filter(|date| date.day() == day)
        .collect();
    match matches.as_slice() {
        [only] => Ok(*only),
        _ => Err(TimingError::NoUniqueAdjacentDate(format!(
            "({day}) around {local_date}"
        ))),
    }
}
