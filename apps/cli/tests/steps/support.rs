// SPDX-License-Identifier: MPL-2.0
//! Test support: the generated fixture plans and their column positions.
//!
//! The fixture generator is `crates/duty2cal-pdf-fixtures`, a dev-only member.
//! It reproduces the supported family's shape (Type0/Identity-H fonts with
//! `ToUnicode` and `/W`, Flate content) so every conversion scenario exercises
//! the shipped reader, not a standard-font shortcut. Fixtures are generated at
//! test time, so no PDF is committed.

#[allow(unused_imports)] // the golden target includes this module but uses only two builders
pub use duty2cal_pdf_fixtures::{
    all_day_warned_plan, at, build_pdf, build_pdf_at_size, detail_header_row, equivalence_plan,
    error_and_warning_plan, two_warned_rows_plan, warned_plan, weekday_abbreviation, ACTIVITY_X,
    AIRCRAFT_X, BLOCKTIME_X, DATE_X, END_X, FROM_X, LAYOVER_X, POS_X, REPORT_X, START_X, TAGS_X,
    TO_X, TRIP_X, TZ_X,
};

/// The committed accepted calendar the e2e and regular steps compare against.
#[allow(dead_code)] // the golden target includes this module but uses only the fixtures
pub(crate) fn accepted_calendar() -> String {
    std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/expected/2026-06.ics"),
    )
    .expect("read the committed accepted calendar")
}

/// CRLF to LF, and folded lines joined back (the agreed normalisation).
#[allow(dead_code)] // the golden target includes this module but uses only the fixtures
pub(crate) fn normalise(calendar: &str) -> String {
    calendar.replace("\r\n ", "").replace("\r\n", "\n")
}

/// A calendar's lines with every `UID:` line dropped.
#[allow(dead_code)] // the golden target includes this module but uses only the fixtures
pub(crate) fn content_apart_from_identity(calendar: &str) -> Vec<String> {
    calendar
        .lines()
        .filter(|line| !line.starts_with("UID:"))
        .map(str::to_string)
        .collect()
}
