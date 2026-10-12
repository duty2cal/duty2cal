// SPDX-License-Identifier: MPL-2.0
//! RFC 5545 serializer — the `CalendarSink` implementation used by every facade.

use time::{Date, OffsetDateTime};

use crate::application::CalendarSink;
use crate::domain::{DutyEntry, Roster};

/// Writes rosters as RFC 5545 calendar text.
#[derive(Debug, Default)]
pub struct IcsSink;

impl CalendarSink for IcsSink {
    fn write(&self, roster: &Roster, revision: OffsetDateTime) -> String {
        render(roster, revision)
    }
}

/// Serialize a roster as a complete RFC 5545 calendar.
pub fn render(roster: &Roster, revision: OffsetDateTime) -> String {
    let mut calendar = String::new();
    calendar.push_str("BEGIN:VCALENDAR\r\n");
    calendar.push_str("VERSION:2.0\r\n");
    calendar.push_str("PRODID:-//kressnerd//duty2cal//EN\r\n");
    calendar.push_str("CALSCALE:GREGORIAN\r\n");
    for (index, entry) in roster.entries.iter().enumerate() {
        render_entry(&mut calendar, entry, roster.snapshot.0, index, revision);
    }
    calendar.push_str("END:VCALENDAR\r\n");
    calendar
}

fn render_entry(
    out: &mut String,
    entry: &DutyEntry,
    snapshot: u64,
    index: usize,
    revision: OffsetDateTime,
) {
    out.push_str("BEGIN:VEVENT\r\n");
    let uid = format!("{snapshot:016x}-{index}@duty2cal");
    line(out, "UID", &uid);
    line(out, "DTSTAMP", &format_utc(revision));
    line(out, "SUMMARY", &entry.summary());
    if let Some(location) = entry.location() {
        line(out, "LOCATION", &location);
    }
    let fields = entry.description_fields();
    if !fields.is_empty() {
        let description = fields
            .iter()
            .map(|(name, value)| format!("{name} {value}"))
            .collect::<Vec<_>>()
            .join(" | ");
        line(out, "DESCRIPTION", &description);
    }
    match entry.interval() {
        Some(interval) => {
            line(out, "DTSTART", &format_utc(interval.start()));
            line(out, "DTEND", &format_utc(interval.end()));
        }
        None => {
            let date = entry.date();
            line(out, "DTSTART;VALUE=DATE", &format_date(date));
            line(out, "DTEND;VALUE=DATE", &format_date(next_day(date)));
        }
    }
    out.push_str("END:VEVENT\r\n");
}

/// Fold at 75 octets; continuation lines begin with a single space (RFC 5545
/// §3.1). Folding never splits a character.
const FOLD_LIMIT: usize = 75;

fn line(out: &mut String, name: &str, value: &str) {
    let content = format!("{name}:{}", escape_text(value));
    fold_into(out, &content);
}

/// Escape a TEXT value as RFC 5545 §3.3.11 requires.
fn escape_text(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            ';' => escaped.push_str("\\;"),
            ',' => escaped.push_str("\\,"),
            '\n' => escaped.push_str("\\n"),
            '\r' => {}
            _ => escaped.push(character),
        }
    }
    escaped
}

/// Append one content line, folded within the octet limit.
fn fold_into(out: &mut String, content: &str) {
    let mut remaining = content;
    let mut continuation = false;
    while !remaining.is_empty() {
        // A continuation line spends one octet on its leading space.
        let limit = if continuation {
            FOLD_LIMIT - 1
        } else {
            FOLD_LIMIT
        };
        let take = floor_char_boundary(remaining, limit);
        if continuation {
            out.push(' ');
        }
        out.push_str(&remaining[..take]);
        out.push_str("\r\n");
        remaining = &remaining[take..];
        continuation = true;
    }
}

/// The largest character boundary not exceeding `limit`.
fn floor_char_boundary(text: &str, limit: usize) -> usize {
    if text.len() <= limit {
        return text.len();
    }
    let mut boundary = limit;
    while boundary > 0 && !text.is_char_boundary(boundary) {
        boundary -= 1;
    }
    boundary
}

/// `YYYYMMDD` for a date value.
pub fn format_date(date: Date) -> String {
    format!(
        "{:04}{:02}{:02}",
        date.year(),
        u8::from(date.month()),
        date.day()
    )
}

/// `YYYYMMDDTHHMMSSZ` for a UTC instant.
pub fn format_utc(instant: OffsetDateTime) -> String {
    format!(
        "{}T{:02}{:02}{:02}Z",
        format_date(instant.date()),
        instant.hour(),
        instant.minute(),
        instant.second()
    )
}

/// The following day, used as the non-inclusive all-day end.
fn next_day(date: Date) -> Date {
    date.next_day().unwrap_or(date)
}
