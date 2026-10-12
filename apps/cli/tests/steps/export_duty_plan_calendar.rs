// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `export-duty-plan-calendar`.
//!
//! One file per capability. RED-phase steps use the `unimplemented!("<step>")`
//! stub; a step body is never empty.

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use cucumber::{given, then, when};

use crate::steps::support::{self, ACTIVITY_X, DATE_X, END_X, FROM_X, START_X, TO_X};
use crate::DutyWorld;

/// Scenario sequence, so concurrently executed scenarios never share a directory.
static SCENARIO_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

/// Fresh per-scenario working directory.
fn fresh_dir() -> PathBuf {
    let sequence = SCENARIO_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("duty2cal-tests-{}-{sequence}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the scenario directory");
    dir
}

/// Prepare a fixture PDF holding the given item rows.
fn write_fixture(w: &mut DutyWorld, items: &[(f64, f64, String)]) {
    let dir = fresh_dir();
    let source = dir.join("roster.pdf");
    std::fs::write(&source, support::build_pdf(items)).expect("write the source");
    w.dir = Some(dir);
    w.source = Some(source);
}

fn transcript(world: &DutyWorld) -> String {
    format!("{}{}", world.stdout, world.stderr)
}

/// The published calendar, or a panic carrying the converter transcript.
fn calendar(w: &DutyWorld) -> String {
    let path = w.working_dir().join("roster.ics");
    std::fs::read_to_string(&path).unwrap_or_else(|error| {
        let diagnostic = std::fs::read_to_string(w.working_dir().join("roster.parse-error.txt"))
            .unwrap_or_else(|_| "<no diagnostic>".to_string());
        panic!(
            "no calendar at {}: {error}\nconverter transcript: {}\ndiagnostic: {diagnostic}",
            path.display(),
            transcript(w)
        )
    })
}

/// Assert the calendar carries no activity at all.
fn assert_no_activity(calendar: &str, context: &str) {
    assert!(
        calendar.contains("BEGIN:VCALENDAR") && calendar.contains("END:VCALENDAR"),
        "the published calendar is not a complete VCALENDAR: {calendar}"
    );
    assert!(
        !calendar.contains("BEGIN:VEVENT"),
        "{context} produced an activity: {calendar}"
    );
}

#[given("a duty plan that is not a supported Released Roster detail-table export")]
async fn a_duty_plan_that_is_not_a_supported_export(w: &mut DutyWorld) {
    let dir = fresh_dir();
    let source = dir.join("roster.pdf");
    std::fs::write(&source, b"this file is not a duty-plan export\n").expect("write the source");
    w.dir = Some(dir);
    w.source = Some(source);
}

#[when("the crew member converts it")]
async fn the_crew_member_converts_it(w: &mut DutyWorld) {
    let output = Command::new(env!("CARGO_BIN_EXE_duty2cal"))
        .arg(w.source_path())
        .output()
        .expect("run the converter");
    w.exit_status = output.status.code();
    w.stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    w.stderr = String::from_utf8_lossy(&output.stderr).into_owned();
}

#[then("no calendar is published")]
async fn no_calendar_is_published(w: &mut DutyWorld) {
    let calendars: Vec<String> = std::fs::read_dir(w.working_dir())
        .expect("read the scenario directory")
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".ics"))
        .collect();
    assert!(
        calendars.is_empty(),
        "a calendar was published: {calendars:?}"
    );
}

#[then("the converter reports a conversion failure naming the source")]
async fn reports_a_failure_naming_the_source(w: &mut DutyWorld) {
    let text = transcript(w);
    assert!(
        text.contains("conversion failed"),
        "no conversion failure was reported: {text}"
    );
    assert!(
        text.contains("roster.pdf"),
        "the reported failure does not name the source: {text}"
    );
}

#[then("the invocation reports failure through a non-zero process exit status")]
async fn reports_a_non_zero_exit_status(w: &mut DutyWorld) {
    let code = w.exit_status.expect("the converter did not run");
    assert_ne!(code, 0, "expected a non-zero process exit status");
}

#[then("a new unused numbered diagnostic report identifies the source and the reason")]
async fn a_numbered_diagnostic_identifies_source_and_reason(w: &mut DutyWorld) {
    let path = w.working_dir().join("roster.parse-error.txt");
    let report = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no numbered diagnostic at {}: {error}", path.display()));
    assert!(
        report.contains("Source PDF:") && report.contains("roster.pdf"),
        "the diagnostic does not identify the source: {report}"
    );
    assert!(
        report.contains("Reason:"),
        "the diagnostic does not state the reason: {report}"
    );
}

#[given("a supported duty plan whose overview calendar and statistics name activity codes that never appear in its dated detail rows")]
async fn a_supported_plan_with_only_summary_codes(w: &mut DutyWorld) {
    let mut items = support::at(800.0, &[(DATE_X, "Overview"), (FROM_X, "FREE SB OFF")]);
    items.extend(support::at(
        770.0,
        &[(DATE_X, "Statistics"), (FROM_X, "SB 12:00")],
    ));
    items.extend(support::detail_header_row(600.0));
    write_fixture(w, &items);
}

#[then("the published calendar contains no activity taken from the overview or statistics")]
async fn no_activity_from_overview_or_statistics(w: &mut DutyWorld) {
    assert_no_activity(&calendar(w), "the overview/statistics material");
}

#[given("a supported duty plan whose footer names activity codes that never appear in its dated detail rows")]
async fn a_supported_plan_with_a_footer(w: &mut DutyWorld) {
    let mut items = support::at(800.0, &[(DATE_X, "Overview"), (FROM_X, "FREE SB OFF")]);
    items.extend(support::detail_header_row(600.0));
    // One dated detail row that carries no activity code.
    items.extend(support::at(585.0, &[(DATE_X, "2099-06-12")]));
    // Footer material below the table, naming codes inside the activity column.
    items.extend(support::at(
        520.0,
        &[(DATE_X, "Footer"), (ACTIVITY_X, "SB"), (FROM_X, "OFF")],
    ));
    write_fixture(w, &items);
}

#[then("the published calendar contains no activity taken from the footer")]
async fn no_activity_from_footer(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert_no_activity(&calendar, "the footer material");
    for code in ["SB", "OFF", "FREE"] {
        assert!(
            !calendar.contains(code),
            "the {code} code from the footer reached the calendar: {calendar}"
        );
    }
}

#[given("a supported duty plan with a detail row that carries only a date and no activity code")]
async fn a_plan_with_a_date_only_row(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(585.0, &[(DATE_X, "2099-06-12")]));
    write_fixture(w, &items);
}

#[then("the published calendar contains no absence or layover for that date")]
async fn no_absence_or_layover_for_that_date(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert_no_activity(&calendar, "the date-only row");
    for invented in ["Layover", "FREE", "OFF", "FZ"] {
        assert!(
            !calendar.contains(invented),
            "the date-only row was turned into {invented}: {calendar}"
        );
    }
}

#[given("a supported duty plan with one timed travel leg, one all-day U row, and one timed SB row")]
async fn a_mixed_plan(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    // Timed travel leg.
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "FB568"),
            (FROM_X, "FRA"),
            (TO_X, "LOS"),
            (START_X, "09:25"),
            (END_X, "15:45"),
        ],
    ));
    // All-day vacation day.
    items.extend(support::at(
        570.0,
        &[(DATE_X, "2099-06-12"), (ACTIVITY_X, "U")],
    ));
    // Timed availability window.
    items.extend(support::at(
        555.0,
        &[
            (DATE_X, "2099-06-13"),
            (ACTIVITY_X, "SB"),
            (START_X, "08:00"),
            (END_X, "16:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[then("the calendar contains exactly three activities: one timed travel activity, one all-day U activity, and one timed SB activity")]
async fn three_activities_of_the_right_kinds(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert_eq!(
        calendar.matches("BEGIN:VEVENT").count(),
        3,
        "expected three activities: {calendar}"
    );
    assert!(
        calendar.contains("SUMMARY:FB568 FRA-LOS")
            && calendar.contains("DTSTART:20990607T092500Z")
            && calendar.contains("DTEND:20990607T154500Z"),
        "the timed travel activity is missing or wrong: {calendar}"
    );
    assert!(
        calendar.contains("SUMMARY:U")
            && calendar.contains("DTSTART;VALUE=DATE:20990612")
            && calendar.contains("DTEND;VALUE=DATE:20990613"),
        "the all-day U activity is missing or wrong: {calendar}"
    );
    assert!(
        calendar.contains("SUMMARY:SB")
            && calendar.contains("DTSTART:20990613T080000Z")
            && calendar.contains("DTEND:20990613T160000Z"),
        "the timed SB activity is missing or wrong: {calendar}"
    );
}
