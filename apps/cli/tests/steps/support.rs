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

/// The family's complete revision footer, as the producer prints it.
fn revision_footer(y: f64) -> Vec<(f64, f64, String)> {
    at(
        y,
        &[
            (DATE_X, "Created"),
            (TRIP_X, "2099-05-21"),
            (REPORT_X, "15:52"),
            (POS_X, "(UTC)"),
            (ACTIVITY_X, "by"),
            (FROM_X, "Jeppesen"),
        ],
    )
}

/// A cockpit-crew plan whose heading month starts after its first dated rows.
///
/// The first two rows are the previous month's tail, `30 Thu` and `31 Fri`
/// under an August heading; the third row is the heading month's first day.
pub(crate) fn cockpit_prior_month_plan() -> Vec<(f64, f64, String)> {
    let mut items = detail_header_row(600.0);
    items.extend(at(
        830.0,
        &[
            (DATE_X, "Month:"),
            (DATE_X + 40.0, "August"),
            (DATE_X + 100.0, "2099"),
        ],
    ));
    items.extend(at(
        585.0,
        &[
            (DATE_X, "30 Thu"),
            (POS_X, "CP"),
            (ACTIVITY_X, "1916"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    items.extend(at(
        570.0,
        &[
            (DATE_X, "31 Fri"),
            (POS_X, "CP"),
            (ACTIVITY_X, "1917"),
            (FROM_X, "DEF"),
            (TO_X, "ABC"),
            (START_X, "11:00"),
            (END_X, "12:00"),
        ],
    ));
    items.extend(at(555.0, &[(DATE_X, "01 Sat"), (ACTIVITY_X, "U1")]));
    items.extend(revision_footer(520.0));
    items
}

/// A cockpit-crew plan whose last dated row belongs to the next month.
pub(crate) fn cockpit_next_month_tail_plan() -> Vec<(f64, f64, String)> {
    let mut items = detail_header_row(600.0);
    items.extend(at(
        830.0,
        &[
            (DATE_X, "Month:"),
            (DATE_X + 40.0, "August"),
            (DATE_X + 100.0, "2099"),
        ],
    ));
    items.extend(at(
        585.0,
        &[
            (DATE_X, "31 Mon"),
            (POS_X, "CP"),
            (ACTIVITY_X, "1918"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    items.extend(at(
        570.0,
        &[
            (DATE_X, "01 Tue"),
            (POS_X, "CP"),
            (ACTIVITY_X, "1919"),
            (FROM_X, "DEF"),
            (TO_X, "ABC"),
            (START_X, "11:00"),
            (END_X, "12:00"),
        ],
    ));
    items.extend(revision_footer(520.0));
    items
}

/// A supported plan whose complete revision footer sits 10 pt below its last
/// dated row, inside the table's row pitch.
pub(crate) fn footer_beside_last_row_plan() -> Vec<(f64, f64, String)> {
    let mut items = detail_header_row(600.0);
    items.extend(at(585.0, &[(DATE_X, "2099-06-07"), (ACTIVITY_X, "U")]));
    items.extend(revision_footer(575.0));
    items
}

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
