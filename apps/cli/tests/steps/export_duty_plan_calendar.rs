// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `export-duty-plan-calendar`.
//!
//! One file per capability. RED-phase steps use the `unimplemented!("<step>")`
//! stub; a step body is never empty.

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use cucumber::{given, then, when};

use crate::steps::support;
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

fn transcript(world: &DutyWorld) -> String {
    format!("{}{}", world.stdout, world.stderr)
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
    let dir = fresh_dir();
    let source = dir.join("roster.pdf");

    let mut items: Vec<(f64, f64, &str)> = vec![
        (40.0, 800.0, "Overview"),
        (40.0, 785.0, "FREE SB OFF"),
        (40.0, 770.0, "Statistics SB 12:00 FREE 00:00"),
    ];
    items.extend(support::detail_header_row(600.0));

    std::fs::write(&source, support::build_pdf(&items)).expect("write the source");
    w.dir = Some(dir);
    w.source = Some(source);
}

#[then("the published calendar contains no activity taken from the overview or statistics")]
async fn no_activity_from_overview_or_statistics(w: &mut DutyWorld) {
    let path = w.working_dir().join("roster.ics");
    let calendar = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        let diagnostic = std::fs::read_to_string(w.working_dir().join("roster.parse-error.txt"))
            .unwrap_or_else(|_| "<no diagnostic>".to_string());
        panic!(
            "no calendar at {}: {error}\nconverter transcript: {}\ndiagnostic: {diagnostic}",
            path.display(),
            transcript(w)
        )
    });

    assert!(
        calendar.contains("BEGIN:VCALENDAR") && calendar.contains("END:VCALENDAR"),
        "the published calendar is not a complete VCALENDAR: {calendar}"
    );
    assert!(
        !calendar.contains("BEGIN:VEVENT"),
        "an activity reached the calendar: {calendar}"
    );
    for code in ["FREE", "SB", "OFF"] {
        assert!(
            !calendar.contains(code),
            "the {code} code from the overview/statistics reached the calendar: {calendar}"
        );
    }
}

#[given("a supported duty plan whose footer names activity codes that never appear in its dated detail rows")]
async fn a_supported_plan_with_a_footer(w: &mut DutyWorld) {
    let dir = fresh_dir();
    let source = dir.join("roster.pdf");

    let mut items: Vec<(f64, f64, &str)> = vec![
        (40.0, 800.0, "Overview"),
        (40.0, 785.0, "FREE SB OFF"),
        (40.0, 770.0, "Statistics SB 12:00 FREE 00:00"),
    ];
    items.extend(support::detail_header_row(600.0));
    // One dated detail row that carries no activity code.
    items.push((40.0, 585.0, "12 Jan"));
    // Footer material below the table, naming codes inside the activity column:
    // these must never become activities.
    items.push((40.0, 520.0, "Footer"));
    items.push((330.0, 520.0, "SB"));
    items.push((400.0, 520.0, "OFF"));

    std::fs::write(&source, support::build_pdf(&items)).expect("write the source");
    w.dir = Some(dir);
    w.source = Some(source);
}

#[then("the published calendar contains no activity taken from the footer")]
async fn no_activity_from_footer(w: &mut DutyWorld) {
    let path = w.working_dir().join("roster.ics");
    let calendar = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        let diagnostic = std::fs::read_to_string(w.working_dir().join("roster.parse-error.txt"))
            .unwrap_or_else(|_| "<no diagnostic>".to_string());
        panic!(
            "no calendar at {}: {error}\nconverter transcript: {}\ndiagnostic: {diagnostic}",
            path.display(),
            transcript(w)
        )
    });

    assert!(
        calendar.contains("BEGIN:VCALENDAR") && calendar.contains("END:VCALENDAR"),
        "the published calendar is not a complete VCALENDAR: {calendar}"
    );
    assert!(
        !calendar.contains("BEGIN:VEVENT"),
        "an activity reached the calendar: {calendar}"
    );
    for code in ["SB", "OFF", "FREE"] {
        assert!(
            !calendar.contains(code),
            "the {code} code from the footer reached the calendar: {calendar}"
        );
    }
}

#[given("a supported duty plan with a detail row that carries only a date and no activity code")]
async fn a_plan_with_a_date_only_row(w: &mut DutyWorld) {
    let dir = fresh_dir();
    let source = dir.join("roster.pdf");

    let mut items: Vec<(f64, f64, &str)> = support::detail_header_row(600.0);
    // A dated detail row whose activity cell is empty.
    items.push((40.0, 585.0, "12 Jan"));

    std::fs::write(&source, support::build_pdf(&items)).expect("write the source");
    w.dir = Some(dir);
    w.source = Some(source);
}

#[then("the published calendar contains no absence or layover for that date")]
async fn no_absence_or_layover_for_that_date(w: &mut DutyWorld) {
    let path = w.working_dir().join("roster.ics");
    let calendar = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        let diagnostic = std::fs::read_to_string(w.working_dir().join("roster.parse-error.txt"))
            .unwrap_or_else(|_| "<no diagnostic>".to_string());
        panic!(
            "no calendar at {}: {error}\nconverter transcript: {}\ndiagnostic: {diagnostic}",
            path.display(),
            transcript(w)
        )
    });

    assert!(
        calendar.contains("BEGIN:VCALENDAR") && calendar.contains("END:VCALENDAR"),
        "the published calendar is not a complete VCALENDAR: {calendar}"
    );
    assert!(
        !calendar.contains("BEGIN:VEVENT"),
        "the date-only row produced an activity: {calendar}"
    );
    for invented in ["Layover", "FREE", "OFF", "FZ"] {
        assert!(
            !calendar.contains(invented),
            "the date-only row was turned into {invented}: {calendar}"
        );
    }
}
