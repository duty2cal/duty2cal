// SPDX-License-Identifier: MPL-2.0
//! RFC 5545 serializer — the `CalendarSink` implementation used by every facade.

use crate::domain::Roster;

/// Render a roster as a complete RFC 5545 calendar.
///
/// Byte-level contract: CRLF line endings, escaping, and octet-aware folding are
/// added with the timed/all-day scenarios; the empty roster already produces a
/// valid VCALENDAR.
pub fn render(roster: &Roster) -> String {
    let mut calendar = String::new();
    calendar.push_str("BEGIN:VCALENDAR\r\n");
    calendar.push_str("VERSION:2.0\r\n");
    calendar.push_str("PRODID:-//kressnerd//duty2cal//EN\r\n");
    calendar.push_str("CALSCALE:GREGORIAN\r\n");
    for _activity in &roster.activities {
        // Event serialization arrives with the timed/all-day scenarios.
    }
    calendar.push_str("END:VCALENDAR\r\n");
    calendar
}
