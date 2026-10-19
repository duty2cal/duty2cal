// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `export-duty-plan-calendar`.
//!
//! One file per capability. A step body is never empty: the RED phase writes a
//! new step as a stub that panics with its own name.
#![allow(clippy::too_many_arguments)] // step signatures mirror the captured step text

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use cucumber::{given, then, when};
use serde_json::{Map, Value};

use crate::steps::browser::BrowserDriver;
use crate::steps::support::{
    self, ACTIVITY_X, AIRCRAFT_X, BLOCKTIME_X, DATE_X, END_X, FROM_X, LAYOVER_X, POS_X, START_X,
    TAGS_X, TO_X, TZ_X,
};
use crate::DutyWorld;

/// Scenario sequence, so concurrently executed scenarios never share a directory.
static SCENARIO_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

/// Fresh per-scenario working directory.
pub(crate) fn fresh_dir() -> PathBuf {
    let sequence = SCENARIO_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("duty2cal-tests-{}-{sequence}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the scenario directory");
    dir
}

/// Prepare a fixture PDF holding the given item rows.
pub(crate) fn write_fixture(w: &mut DutyWorld, items: &[(f64, f64, String)]) {
    let dir = fresh_dir();
    let source = dir.join("roster.pdf");
    std::fs::write(&source, support::build_pdf(items)).expect("write the source");
    w.dir = Some(dir);
    w.source = Some(source);
}

pub(crate) fn transcript(world: &DutyWorld) -> String {
    format!("{}{}", world.stdout, world.stderr)
}

/// The published calendar, or a panic carrying the converter transcript.
pub(crate) fn calendar(w: &DutyWorld) -> String {
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

/// Invoke the converter once on the scenario's source.
pub(crate) fn run_converter(w: &mut DutyWorld) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_duty2cal"));
    command.arg(w.source_path());
    for (name, value) in &w.env {
        command.env(name, value);
    }
    let output = command.output().expect("run the converter");
    w.exit_status = output.status.code();
    w.stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    w.stderr = String::from_utf8_lossy(&output.stderr).into_owned();
}

/// The component identifiers carried by a calendar (unfolded first).
fn component_ids(calendar: &str) -> Vec<String> {
    calendar
        .replace("\r\n ", "")
        .lines()
        .filter_map(|line| line.strip_prefix("UID:"))
        .map(str::to_string)
        .collect()
}

#[when("the crew member converts it")]
async fn the_crew_member_converts_it(w: &mut DutyWorld) {
    run_converter(w);
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
            (ACTIVITY_X, "LU742"),
            (FROM_X, "Tychon"),
            (TO_X, "Mareum"),
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
        calendar.contains("SUMMARY:LU742 Tychon-Mareum")
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

/// The duty-plan date written as an ICS date value.
fn compact(date: &str) -> String {
    date.replace('-', "")
}

/// The duty-plan date and printed clock written as an ICS UTC instant.
fn utc(date: &str, time: &str) -> String {
    format!("{}T{}00Z", compact(date), time.replace(':', ""))
}

/// Prepare a single-row plan at a realistic detail-table position.
pub(crate) fn plan_with_row(w: &mut DutyWorld, cells: &[(f64, &str)]) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(585.0, cells));
    write_fixture(w, &items);
}

/// The DESCRIPTION text of every event, empty when an event has none.
///
/// Content lines are unfolded first: a folded line continues after CRLF and a
/// single space (RFC 5545 §3.1), so a reader must join it back.
fn event_descriptions(calendar: &str) -> Vec<String> {
    let calendar = calendar.replace("\r\n ", "");
    calendar
        .split("BEGIN:VEVENT")
        .skip(1)
        .map(|event| {
            event
                .lines()
                .filter_map(|line| line.strip_prefix("DESCRIPTION:"))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}

/// Assert the calendar carries exactly `expected` activities.
fn assert_event_count(calendar: &str, expected: usize) {
    assert_eq!(
        calendar.matches("BEGIN:VEVENT").count(),
        expected,
        "expected {expected} activities: {calendar}"
    );
}

#[given(
    regex = r"^a supported duty plan dated (\d{4}-\d{2}-\d{2}) with activity code (\w+) and printed UTC times (.*) and (.*)$"
)]
async fn a_plan_with_an_all_day_code(
    w: &mut DutyWorld,
    date: String,
    code: String,
    start: String,
    end: String,
) {
    let mut cells: Vec<(f64, &str)> = vec![(DATE_X, &date), (ACTIVITY_X, &code), (TAGS_X, "==")];
    if !start.is_empty() {
        cells.push((START_X, &start));
    }
    if !end.is_empty() {
        cells.push((END_X, &end));
    }
    plan_with_row(w, &cells);
}

#[then(
    regex = r"^the calendar contains one all-day activity for (\d{4}-\d{2}-\d{2}) whose source code is (\w+)$"
)]
async fn one_all_day_activity(w: &mut DutyWorld, date: String, code: String) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 1);
    assert!(
        calendar.contains(&format!("SUMMARY:{code}"))
            && calendar.contains(&format!("DTSTART;VALUE=DATE:{}", compact(&date))),
        "the all-day {code} activity is missing or wrong: {calendar}"
    );
    assert!(
        !calendar.contains("=="),
        "the all-day marker reached the calendar: {calendar}"
    );
}

#[then("that activity carries no timed interval")]
async fn no_timed_interval(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert!(
        !calendar.contains("\nDTSTART:") && !calendar.starts_with("DTSTART:"),
        "the all-day activity carries a timed interval: {calendar}"
    );
}

#[given(
    regex = r"^a supported duty plan dated (\d{4}-\d{2}-\d{2}) with activity code (\w+) and resolved UTC start (\d{2}:\d{2}) and end (\d{2}:\d{2})$"
)]
async fn a_plan_with_a_timed_code(
    w: &mut DutyWorld,
    date: String,
    code: String,
    start: String,
    end: String,
) {
    plan_with_row(
        w,
        &[
            (DATE_X, &date),
            (ACTIVITY_X, &code),
            (START_X, &start),
            (END_X, &end),
        ],
    );
}

#[given(
    regex = r"^a supported duty plan dated (\d{4}-\d{2}-\d{2}) with activity code (\w+) and source station (\w+) and resolved UTC start (\d{2}:\d{2}) and end (\d{2}:\d{2})$"
)]
async fn a_plan_with_a_stationed_timed_code(
    w: &mut DutyWorld,
    date: String,
    code: String,
    station: String,
    start: String,
    end: String,
) {
    plan_with_row(
        w,
        &[
            (DATE_X, &date),
            (ACTIVITY_X, &code),
            (FROM_X, &station),
            (START_X, &start),
            (END_X, &end),
        ],
    );
}

#[given(
    regex = r"^a supported duty plan dated (\d{4}-\d{2}-\d{2}) with activity code (\w+) and its own resolved UTC start (\d{2}:\d{2}) and end (\d{2}:\d{2})$"
)]
async fn a_plan_with_an_own_window_code(
    w: &mut DutyWorld,
    date: String,
    code: String,
    start: String,
    end: String,
) {
    plan_with_row(
        w,
        &[
            (DATE_X, &date),
            (ACTIVITY_X, &code),
            (START_X, &start),
            (END_X, &end),
        ],
    );
}

#[then(
    regex = r"^the calendar contains one timed non-flight activity from (\d{2}:\d{2}) to (\d{2}:\d{2}) UTC on (\d{4}-\d{2}-\d{2}) lasting (\d+) hours$"
)]
async fn one_timed_non_flight(
    w: &mut DutyWorld,
    start: String,
    end: String,
    date: String,
    _hours: u32,
) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 1);
    assert!(
        calendar.contains(&format!("DTSTART:{}", utc(&date, &start)))
            && calendar.contains(&format!("DTEND:{}", utc(&date, &end))),
        "the timed non-flight activity is missing or wrong: {calendar}"
    );
}

#[then(
    regex = r"^the calendar contains one timed standby activity from (\d{2}:\d{2}) to (\d{2}:\d{2}) UTC lasting (\d+) hours$"
)]
async fn one_timed_standby(w: &mut DutyWorld, start: String, end: String, _hours: u32) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 1);
    assert!(
        calendar.contains("SUMMARY:SB")
            && calendar.contains(&format!("DTSTART:20990118T{}00Z", start.replace(':', "")))
            && calendar.contains(&format!("DTEND:20990118T{}00Z", end.replace(':', ""))),
        "the timed standby activity is missing or wrong: {calendar}"
    );
}

#[then(
    regex = r"^the calendar contains one timed reserve activity from (\d{2}:\d{2}) to (\d{2}:\d{2}) UTC lasting (\d+) hours$"
)]
async fn one_timed_reserve(w: &mut DutyWorld, start: String, end: String, _hours: u32) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 1);
    assert!(
        calendar.contains("SUMMARY:RE")
            && calendar.contains(&format!("DTSTART:20990118T{}00Z", start.replace(':', "")))
            && calendar.contains(&format!("DTEND:20990118T{}00Z", end.replace(':', ""))),
        "the timed reserve activity is missing or wrong: {calendar}"
    );
}

#[then(
    regex = r"^the calendar contains one timed SBX activity from (\d{2}:\d{2}) to (\d{2}:\d{2}) UTC$"
)]
async fn one_timed_sbx(w: &mut DutyWorld, start: String, end: String) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 1);
    assert!(
        calendar.contains("SUMMARY:SBX")
            && calendar.contains(&format!("DTSTART:20990122T{}00Z", start.replace(':', "")))
            && calendar.contains(&format!("DTEND:20990122T{}00Z", end.replace(':', ""))),
        "the timed SBX activity is missing or wrong: {calendar}"
    );
}

#[then("the calendar contains no activity for any cancelled flight")]
async fn no_cancelled_flight(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 1);
}

#[given(
    regex = r"^a supported duty plan with SB window (\d{2}:\d{2}) to (\d{2}:\d{2}) on (\d{4}-\d{2}-\d{2}), a FREE row on (\d{4}-\d{2}-\d{2}), and SB window (\d{2}:\d{2}) to (\d{2}:\d{2}) on (\d{4}-\d{2}-\d{2})$"
)]
async fn consecutive_standby_days(
    w: &mut DutyWorld,
    first_start: String,
    first_end: String,
    first_date: String,
    off_date: String,
    second_start: String,
    second_end: String,
    second_date: String,
) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, &first_date),
            (ACTIVITY_X, "SB"),
            (START_X, &first_start),
            (END_X, &first_end),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[(DATE_X, &off_date), (ACTIVITY_X, "FREE")],
    ));
    items.extend(support::at(
        555.0,
        &[
            (DATE_X, &second_date),
            (ACTIVITY_X, "SB"),
            (START_X, &second_start),
            (END_X, &second_end),
        ],
    ));
    write_fixture(w, &items);
}

#[then(
    regex = r"^the calendar contains two timed standby activities on (\d{4}-\d{2}-\d{2}) and (\d{4}-\d{2}-\d{2})$"
)]
async fn two_standby_activities(w: &mut DutyWorld, first: String, second: String) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 3);
    for date in [&first, &second] {
        assert!(
            calendar.contains(&format!("DTSTART:{}T080000Z", compact(date))),
            "the standby activity on {date} is missing: {calendar}"
        );
    }
}

#[then(regex = r"^the calendar contains one all-day day-off activity on (\d{4}-\d{2}-\d{2})$")]
async fn one_all_day_day_off(w: &mut DutyWorld, date: String) {
    let calendar = calendar(w);
    assert!(
        calendar.contains("SUMMARY:FREE")
            && calendar.contains(&format!("DTSTART;VALUE=DATE:{}", compact(&date))),
        "the all-day day off on {date} is missing: {calendar}"
    );
}

#[then(regex = r"^no standby activity spans (\d{4}-\d{2}-\d{2})$")]
async fn no_standby_spans(w: &mut DutyWorld, date: String) {
    let calendar = calendar(w);
    assert!(
        !calendar.contains(&format!("DTSTART:{}T", compact(&date))),
        "a standby activity starts on the day off {date}: {calendar}"
    );
}

#[given(
    regex = r"^a supported duty plan with designator (\w+) from (\w+) to (\w+), position (\w+), resolved UTC start (\d{2}:\d{2}) and end (\d{2}:\d{2}), no aircraft, and accounting Blocktime (\d{2}:\d{2})$"
)]
async fn a_deadhead_leg(
    w: &mut DutyWorld,
    designator: String,
    from: String,
    to: String,
    position: String,
    start: String,
    end: String,
    _blocktime: String,
) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (support::POS_X, &position),
            (ACTIVITY_X, &designator),
            (FROM_X, &from),
            (TO_X, &to),
            (START_X, &start),
            (END_X, &end),
        ],
    ));
    write_fixture(w, &items);
}

#[then(
    regex = r"^the calendar contains one travel activity from (\d{2}:\d{2}) to (\d{2}:\d{2}) UTC on the duty-plan date$"
)]
async fn one_travel_activity(w: &mut DutyWorld, start: String, end: String) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 1);
    assert!(
        calendar.contains(&format!("DTSTART:20990607T{}00Z", start.replace(':', "")))
            && calendar.contains(&format!("DTEND:20990607T{}00Z", end.replace(':', ""))),
        "the travel activity is missing or wrong: {calendar}"
    );
}

#[then(regex = r#"^that activity's title is "(.*)"$"#)]
async fn that_activity_title(w: &mut DutyWorld, title: String) {
    let calendar = calendar(w).replace("\r\n ", "");
    let expected = format!("SUMMARY:{title}");
    assert!(
        calendar.lines().any(|line| line == expected),
        "the activity title is not {title:?}: {calendar}"
    );
}

#[then("the calendar contains no second activity for that leg")]
async fn no_second_activity(w: &mut DutyWorld) {
    assert_event_count(&calendar(w), 1);
}

#[given(regex = r"^a supported duty plan with a dated Layover row for station (\w+)$")]
async fn a_layover_row(w: &mut DutyWorld, station: String) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-08"),
            (ACTIVITY_X, "Layover"),
            (FROM_X, &station),
        ],
    );
}

#[then("the calendar contains one all-day layover activity for that date")]
async fn one_all_day_layover(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 1);
    assert!(
        calendar.contains("SUMMARY:Layover ABC")
            && calendar.contains("DTSTART;VALUE=DATE:20990608")
            && calendar.contains("DTEND;VALUE=DATE:20990609"),
        "the all-day layover activity is missing or wrong: {calendar}"
    );
}

/// Clock digits of a printed `HH:MM` time.
fn digits(time: &str) -> String {
    time.replace(':', "")
}

/// The ICS instant for a UTC day and printed clock.
fn instant(day: &str, time: &str) -> String {
    format!("{}T{}00Z", compact(day), digits(time))
}

#[given(
    "a supported duty plan containing a required activity code the converter does not recognize"
)]
async fn an_unknown_activity_code(w: &mut DutyWorld) {
    plan_with_row(w, &[(DATE_X, "2099-06-07"), (ACTIVITY_X, "ZZ999")]);
}

#[given("a supported duty plan whose travel fragment has no unique compatible partner")]
async fn an_unmatched_travel_fragment(w: &mut DutyWorld) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU743"),
            (FROM_X, "ABC"),
            (START_X, "10:00"),
        ],
    );
}

#[given(
    regex = r"^a supported duty plan whose dated row prints crew position (\w+) beside activity code (\w+) with resolved UTC start (\d{2}:\d{2}) and end (\d{2}:\d{2})$"
)]
async fn a_non_flight_row_with_a_crew_position(
    w: &mut DutyWorld,
    position: String,
    code: String,
    start: String,
    end: String,
) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (POS_X, &position),
            (ACTIVITY_X, &code),
            (START_X, &start),
            (END_X, &end),
        ],
    );
}

#[given(
    regex = r"^a supported duty plan whose travel leg prints flight number (\w+) from (\w+) to (\w+) with resolved UTC start (\d{2}:\d{2}) and end (\d{2}:\d{2}) and no crew position$"
)]
async fn a_travel_leg_without_a_crew_position(
    w: &mut DutyWorld,
    flight_number: String,
    from: String,
    to: String,
    start: String,
    end: String,
) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, &flight_number),
            (FROM_X, &from),
            (TO_X, &to),
            (START_X, &start),
            (END_X, &end),
        ],
    );
}

#[given(
    "a supported duty plan whose unmatched travel fragment prints crew position FB and flight number 123"
)]
async fn an_unmatched_fragment_with_a_crew_position(w: &mut DutyWorld) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (POS_X, "FB"),
            (ACTIVITY_X, "123"),
            (FROM_X, "ABC"),
            (START_X, "10:00"),
        ],
    );
}

#[given(
    regex = r"^a supported duty plan whose travel leg prints crew position (\w+) and flight number (\w+) from (\w+) to (\w+) with resolved UTC start (\d{2}:\d{2}) and end (\d{2}:\d{2})$"
)]
async fn a_travel_leg_with_a_crew_position(
    w: &mut DutyWorld,
    position: String,
    flight_number: String,
    from: String,
    to: String,
    start: String,
    end: String,
) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (POS_X, &position),
            (ACTIVITY_X, &flight_number),
            (FROM_X, &from),
            (TO_X, &to),
            (START_X, &start),
            (END_X, &end),
        ],
    );
}

#[given(
    "a supported duty plan whose departure fragment prints crew position DH and flight number 124 and whose arrival fragment prints flight number 124 without a crew position"
)]
async fn one_fragment_prints_the_crew_position(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (POS_X, "DH"),
            (ACTIVITY_X, "124"),
            (FROM_X, "ABC"),
            (START_X, "10:00"),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "124"),
            (TO_X, "DEF"),
            (END_X, "12:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[given(
    regex = r#"^a supported duty plan whose local block for (\d{4}-\d{2}-\d{2}) carries a departure start "\((\d+)\) (\d{2}:\d{2})" and whose arrival block for (\d{4}-\d{2}-\d{2}) ends at (\d{2}:\d{2}) UTC$"#
)]
async fn split_leg_with_qualified_departure(
    w: &mut DutyWorld,
    departure_date: String,
    qualifier: String,
    departure_clock: String,
    arrival_date: String,
    arrival_clock: String,
) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, &departure_date),
            (ACTIVITY_X, "LU743"),
            (FROM_X, "ABC"),
            (START_X, &format!("({qualifier}) {departure_clock}")),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, &arrival_date),
            (ACTIVITY_X, "LU743"),
            (TO_X, "DEF"),
            (END_X, &arrival_clock),
        ],
    ));
    write_fixture(w, &items);
    // The printed qualifier names the UTC departure day; in this scenario it is
    // the arrival block's date.
    w.expected = vec![
        format!("DTSTART:{}", instant(&arrival_date, &departure_clock)),
        format!("DTEND:{}", instant(&arrival_date, &arrival_clock)),
    ];
}

#[given(
    regex = r#"^a supported duty plan whose local block for (\d{4}-\d{2}-\d{2}) starts at (\d{2}:\d{2}) UTC and whose next block ends "\((\d+)\) (\d{2}:\d{2})" UTC$"#
)]
async fn split_leg_with_qualified_arrival(
    w: &mut DutyWorld,
    departure_date: String,
    departure_clock: String,
    qualifier: String,
    arrival_clock: String,
) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, &departure_date),
            (ACTIVITY_X, "LU743"),
            (FROM_X, "ABC"),
            (START_X, &departure_clock),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-01-13"),
            (ACTIVITY_X, "LU743"),
            (TO_X, "DEF"),
            (END_X, &format!("({qualifier}) {arrival_clock}")),
        ],
    ));
    write_fixture(w, &items);
    // The printed qualifier names the UTC arrival day; in this scenario it is
    // the departure block's date.
    w.expected = vec![
        format!("DTSTART:{}", instant(&departure_date, &departure_clock)),
        format!("DTEND:{}", instant(&departure_date, &arrival_clock)),
    ];
}

#[then(
    regex = r"^the calendar contains one travel activity on UTC day (\d{4}-\d{2}-\d{2}) lasting (\d+) hours? (\d+) minutes?$"
)]
async fn one_travel_on_utc_day(w: &mut DutyWorld, day: String, _hours: u32, _minutes: u32) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 1);
    assert!(
        calendar.contains(&format!("DTSTART:{}T", compact(&day))),
        "the travel activity is not on UTC day {day}: {calendar}"
    );
    for expected in &w.expected {
        assert!(
            calendar.contains(expected),
            "expected {expected} in the calendar: {calendar}"
        );
    }
}

#[given(
    regex = r"^a supported duty plan with one local block for (\d{4}-\d{2}-\d{2}) whose travel start is (\d{2}:\d{2}) and end is (\d{2}:\d{2}), with no day qualifiers and no separate arrival block$"
)]
async fn unqualified_overnight(w: &mut DutyWorld, date: String, start: String, end: String) {
    plan_with_row(
        w,
        &[
            (DATE_X, &date),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, &start),
            (END_X, &end),
        ],
    );
    let next = "2099-01-13";
    w.expected = vec![
        format!("DTSTART:{}", instant(&date, &start)),
        format!("DTEND:{}", instant(next, &end)),
    ];
}

#[then(
    regex = r"^the calendar contains one travel activity from (\d{2}:\d{2}) on (\d{4}-\d{2}-\d{2}) to (\d{2}:\d{2}) on (\d{4}-\d{2}-\d{2}) UTC lasting (\d+) hours? (\d+) minutes?$"
)]
async fn one_travel_between(
    w: &mut DutyWorld,
    start: String,
    start_day: String,
    end: String,
    end_day: String,
    _hours: u32,
    _minutes: u32,
) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 1);
    for expected in [
        format!("DTSTART:{}", instant(&start_day, &start)),
        format!("DTEND:{}", instant(&end_day, &end)),
    ] {
        assert!(
            calendar.contains(&expected),
            "expected {expected} in the calendar: {calendar}"
        );
    }
}

#[given("a supported duty plan with one local block whose travel start and end clocks are equal and unqualified")]
async fn zero_length_interval(w: &mut DutyWorld) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-01-12"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "10:00"),
            (END_X, "10:00"),
        ],
    );
}

#[given(
    regex = r#"^a supported duty plan with local date (\d{4}-\d{2}-\d{2}) and a UTC cell "\((\d+)\) (\d{2}:\d{2})"$"#
)]
async fn boundary_qualified_cell(
    w: &mut DutyWorld,
    date: String,
    qualifier: String,
    clock: String,
) {
    plan_with_row(
        w,
        &[
            (DATE_X, &date),
            (ACTIVITY_X, "LU900"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, &format!("({qualifier}) {clock}")),
            (END_X, "(1) 01:00"),
        ],
    );
}

#[then(regex = r"^that endpoint is (\d{4})-(\d{2})-(\d{2}) (\d{2}:\d{2}) UTC$")]
async fn that_endpoint_is(
    w: &mut DutyWorld,
    year: String,
    month: String,
    day: String,
    clock: String,
) {
    let calendar = calendar(w);
    let expected = format!("DTSTART:{year}{month}{day}T{}00Z", digits(&clock));
    assert!(
        calendar.contains(&expected),
        "expected the endpoint {expected} in the calendar: {calendar}"
    );
}

#[given("a supported duty plan whose explicit UTC day number matches none of the previous, same, or next dates around its local-date block")]
async fn no_unique_adjacent_date(w: &mut DutyWorld) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-01-15"),
            (ACTIVITY_X, "LU900"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "(9) 10:00"),
            (END_X, "(9) 11:00"),
        ],
    );
}

#[given("a supported duty plan whose roster trip T42 has legs 007 and 008 where only leg 007 prints the Trip ID, followed by roster trip T43")]
async fn a_roster_trip_with_sparse_ids(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (support::TRIP_X, "T42"),
            (ACTIVITY_X, "007"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "008"),
            (FROM_X, "DEF"),
            (TO_X, "GHI"),
            (START_X, "11:00"),
            (END_X, "12:00"),
        ],
    ));
    items.extend(support::at(
        555.0,
        &[
            (DATE_X, "2099-06-08"),
            (support::TRIP_X, "T43"),
            (ACTIVITY_X, "009"),
            (FROM_X, "GHI"),
            (TO_X, "ABC"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[then(r#"the descriptions of leg 007 and leg 008 both contain "Trip T42""#)]
async fn two_legs_share_the_trip(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 3);
    let carrying = event_descriptions(&calendar)
        .iter()
        .filter(|description| description.contains("Trip T42"))
        .count();
    assert_eq!(
        carrying, 2,
        "expected exactly the two T42 legs to carry the trip: {calendar}"
    );
}

#[then(r#"the description of the leg in trip T43 does not contain "Trip T42""#)]
async fn a_later_trip_does_not_inherit(w: &mut DutyWorld) {
    let calendar = calendar(w);
    let descriptions = event_descriptions(&calendar);
    assert!(
        descriptions.iter().any(|d| d.contains("Trip T43"))
            && !descriptions
                .iter()
                .filter(|d| d.contains("Trip T43"))
                .any(|d| d.contains("Trip T42")),
        "the later trip inherited the earlier identifier: {calendar}"
    );
}

/// The rendered label each optional description field uses, and a fixture value.
fn optional_field(field: &str) -> (&'static str, f64, &'static str) {
    match field {
        "Report (LT)" => ("Report (LT)", support::REPORT_X, "08:10"),
        "A/C" => ("A/C", AIRCRAFT_X, "320"),
        "Blocktime" => ("Blocktime", BLOCKTIME_X, "03:00"),
        "Timezone Difference" => ("Timezone Difference", TZ_X, "+01:00"),
        "Layover duration" => ("Layover duration", LAYOVER_X, "51:30"),
        "Tags" => ("Tags", TAGS_X, "5_MFL"),
        _ => ("Trip", support::TRIP_X, "T42"),
    }
}

#[given(regex = r"^a supported duty plan travel leg whose (.*) is not supplied$")]
async fn a_travel_leg_missing_one_field(w: &mut DutyWorld, omitted: String) {
    let fields = [
        "Trip ID",
        "Report (LT)",
        "A/C",
        "Blocktime",
        "Timezone Difference",
        "Layover duration",
        "Tags",
    ];
    let mut values: Vec<(f64, String)> = vec![
        (DATE_X, "2099-06-07".to_string()),
        (ACTIVITY_X, "LU742".to_string()),
        (FROM_X, "ABC".to_string()),
        (TO_X, "DEF".to_string()),
        (START_X, "09:00".to_string()),
        (END_X, "10:00".to_string()),
    ];
    for field in fields {
        if field == omitted {
            continue;
        }
        let (_, x, value) = optional_field(field);
        values.push((x, value.to_string()));
    }
    let refs: Vec<(f64, &str)> = values.iter().map(|(x, v)| (*x, v.as_str())).collect();
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(585.0, &refs));
    write_fixture(w, &items);
}

#[then(regex = r"^the travel activity's description carries no (.*)$")]
async fn description_omits(w: &mut DutyWorld, omitted: String) {
    let calendar = calendar(w);
    let descriptions = event_descriptions(&calendar);
    let description = descriptions.first().cloned().unwrap_or_default();
    let all = [
        "Trip ID",
        "Report (LT)",
        "A/C",
        "Blocktime",
        "Timezone Difference",
        "Layover duration",
        "Tags",
    ];
    for field in all {
        let (label, _, value) = optional_field(field);
        let rendered = format!("{label} {value}");
        if field == omitted {
            assert!(
                !description.contains(label),
                "the unsupplied {field} appeared in the description: {description}"
            );
        } else {
            assert!(
                description.contains(&rendered),
                "the supplied {field} is missing from the description: {description}"
            );
        }
    }
}

#[given("a compatible departure and arrival fragment of one travel leg where the departure fragment supplies A/C ABC and the arrival fragment supplies Blocktime 02:00")]
async fn complementary_fragments(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (START_X, "09:00"),
            (AIRCRAFT_X, "ABC"),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (TO_X, "DEF"),
            (END_X, "10:00"),
            (BLOCKTIME_X, "02:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[then("the travel activity's description contains A/C ABC and Blocktime 02:00")]
async fn complementary_values_present(w: &mut DutyWorld) {
    let calendar = calendar(w);
    let descriptions = event_descriptions(&calendar);
    let description = descriptions.first().cloned().unwrap_or_default();
    assert!(
        description.contains("A/C ABC") && description.contains("Blocktime 02:00"),
        "the complementary values did not combine into one description: {description}"
    );
}

#[given("a supported duty plan travel leg supplying Trip ID T42, Report (LT) 07:30, A/C ABC, Blocktime 02:00, Timezone Difference +01, Layover duration 03:00, and Tags 5_MFL")]
async fn a_leg_supplying_every_selected_field(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (support::TRIP_X, "T42"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
            (support::REPORT_X, "07:30"),
            (AIRCRAFT_X, "ABC"),
            (BLOCKTIME_X, "02:00"),
            (TZ_X, "+01"),
            (LAYOVER_X, "03:00"),
            (TAGS_X, "5_MFL"),
        ],
    ));
    write_fixture(w, &items);
}

#[given("a supported duty plan travel leg without a report time")]
async fn a_travel_leg_without_a_report_time(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[then("the travel activity's description lists them in the order Trip ID, Report (LT), A/C, Blocktime, Timezone Difference, Layover duration, Tags")]
async fn description_lists_approved_order(w: &mut DutyWorld) {
    let calendar = calendar(w);
    let description = event_descriptions(&calendar)
        .first()
        .cloned()
        .unwrap_or_default();
    let order = [
        "Trip T42",
        "Report (LT) 07:30",
        "A/C ABC",
        "Blocktime 02:00",
        "Timezone Difference +01",
        "Layover duration 03:00",
        "Tags 5_MFL",
    ];
    let mut previous = 0;
    for field in order {
        let position = description
            .find(field)
            .unwrap_or_else(|| panic!("{field} is missing from the description: {description}"));
        assert!(
            position >= previous,
            "{field} appears out of the approved order: {description}"
        );
        previous = position;
    }
}

/// One travel leg split into fragments, each printing its own report time.
fn report_time_fragments(w: &mut DutyWorld, departure: &str, arrival: &str) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (START_X, "09:00"),
            (support::REPORT_X, departure),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (TO_X, "DEF"),
            (END_X, "10:00"),
            (support::REPORT_X, arrival),
        ],
    ));
    write_fixture(w, &items);
}

#[given("a compatible departure and arrival fragment of one travel leg supplying Report (LT) 07:30 and Report (LT) 08:00 respectively")]
async fn differing_report_times(w: &mut DutyWorld) {
    report_time_fragments(w, "07:30", "08:00");
}

#[given("a compatible departure and arrival fragment of one travel leg supplying different report times 07:30 and 08:00")]
async fn new_differing_report_times(w: &mut DutyWorld) {
    report_time_fragments(w, "07:30", "08:00");
}

#[given("a compatible departure and arrival fragment of one travel leg supplying the same report time 07:30")]
async fn equal_report_times(w: &mut DutyWorld) {
    report_time_fragments(w, "07:30", "07:30");
}

#[then(r#"the travel activity's description states "Report (LT) 07:30 (departure fragment)" and "Report (LT) 08:00 (arrival fragment)""#)]
async fn both_fragment_values_retained(w: &mut DutyWorld) {
    let calendar = calendar(w);
    let description = event_descriptions(&calendar)
        .first()
        .cloned()
        .unwrap_or_default();
    for expected in [
        "Report (LT) 07:30 (departure fragment)",
        "Report (LT) 08:00 (arrival fragment)",
    ] {
        assert!(
            description.contains(expected),
            "expected {expected:?} in the description: {description}"
        );
    }
}

#[then("neither printed value changes the activity's UTC interval")]
async fn printed_values_do_not_move_the_interval(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert!(
        calendar.contains("DTSTART:20990607T090000Z")
            && calendar.contains("DTEND:20990607T100000Z"),
        "the printed report times changed the interval: {calendar}"
    );
}

#[given(regex = r"^a supported duty plan dated (\d{4}-\d{2}-\d{2}) with activity code (\w+)$")]
async fn a_plan_with_one_all_day_code(w: &mut DutyWorld, date: String, code: String) {
    plan_with_row(w, &[(DATE_X, &date), (ACTIVITY_X, &code)]);
}

#[then(regex = r"^the all-day activity for (\d{4}-\d{2}-\d{2}) ends on (\d{4}-\d{2}-\d{2})$")]
async fn all_day_ends_the_following_day(w: &mut DutyWorld, starts: String, ends: String) {
    let calendar = calendar(w);
    assert_event_count(&calendar, 1);
    assert!(
        calendar.contains(&format!("DTSTART;VALUE=DATE:{}", compact(&starts))),
        "the all-day activity does not start on {starts}: {calendar}"
    );
    assert!(
        calendar.contains(&format!("DTEND;VALUE=DATE:{}", compact(&ends))),
        "the all-day activity does not end on {ends}: {calendar}"
    );
}

/// A tag value long enough to force line folding.
fn long_tag() -> String {
    "TAG".repeat(30)
}

#[given("a supported duty plan with timed and all-day activities")]
async fn a_plan_with_timed_and_all_day_activities(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    // Timed travel with a text value that needs escaping.
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
            (TAGS_X, "5_MFL;X,Y"),
        ],
    ));
    // Timed travel with a value long enough to force folding.
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-06-08"),
            (ACTIVITY_X, "LU743"),
            (FROM_X, "DEF"),
            (TO_X, "ABC"),
            (START_X, "11:00"),
            (END_X, "12:00"),
            (TAGS_X, &long_tag()),
        ],
    ));
    // An all-day activity.
    items.extend(support::at(
        555.0,
        &[(DATE_X, "2099-06-12"), (ACTIVITY_X, "U")],
    ));
    write_fixture(w, &items);
}

#[then("the published calendar uses CRLF line endings and UTF-8 text")]
async fn calendar_uses_crlf(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert!(
        !calendar.replace("\r\n", "").contains(['\r', '\n']),
        "the calendar carries a bare line ending: {calendar:?}"
    );
    assert!(
        calendar.is_ascii() || std::str::from_utf8(calendar.as_bytes()).is_ok(),
        "the calendar is not valid UTF-8"
    );
}

#[then("its text values are escaped as RFC 5545 requires")]
async fn text_values_are_escaped(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert!(
        calendar.contains(r"5_MFL\;X\,Y"),
        "the commas and semicolon in Tags are not escaped: {calendar}"
    );
}

#[then("its content lines are folded within 75 octets without splitting a character")]
async fn content_lines_are_folded(w: &mut DutyWorld) {
    let calendar = calendar(w);
    for line in calendar.split("\r\n") {
        assert!(
            line.len() <= 75,
            "a content line is longer than 75 octets ({}): {line:?}",
            line.len()
        );
    }
    let unfolded = calendar.replace("\r\n ", "");
    assert!(
        unfolded.contains(&long_tag()),
        "the long value cannot be unfolded back to its original text"
    );
}

#[then("every timed activity ends after it starts")]
async fn timed_activities_end_after_start(w: &mut DutyWorld) {
    let calendar = calendar(w);
    let mut checked = 0;
    for event in calendar.split("BEGIN:VEVENT").skip(1) {
        let start = event
            .lines()
            .find_map(|line| line.strip_prefix("DTSTART:"))
            .map(str::to_string);
        let end = event
            .lines()
            .find_map(|line| line.strip_prefix("DTEND:"))
            .map(str::to_string);
        if let (Some(start), Some(end)) = (start, end) {
            assert!(
                end > start,
                "a timed activity ends before it starts: {start} -> {end}"
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 2,
        "expected at least two timed activities, found {checked}"
    );
}

#[given(
    "a supported duty plan converted once and then converted again under a different output name"
)]
async fn the_same_plan_converted_twice(w: &mut DutyWorld) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    );
    run_converter(w);
    let first = std::fs::read_to_string(w.working_dir().join("roster.ics"))
        .expect("the first conversion published no calendar");
    run_converter(w);
    let second = std::fs::read_to_string(w.working_dir().join("roster-1.ics"))
        .expect("the second conversion published no differently named calendar");
    w.calendars = vec![first, second];
}

#[when("the crew member compares the two calendars")]
async fn compares_the_two_calendars(w: &mut DutyWorld) {
    assert_eq!(
        w.calendars.len(),
        2,
        "two calendars are needed for the comparison"
    );
    for calendar in &w.calendars {
        assert!(
            calendar.contains("BEGIN:VCALENDAR") && !component_ids(calendar).is_empty(),
            "a calendar carries no component identifiers: {calendar}"
        );
    }
}

#[then("the same activity carries the same component identifier in both calendars")]
async fn same_identifiers_in_both(w: &mut DutyWorld) {
    let first = component_ids(&w.calendars[0]);
    let second = component_ids(&w.calendars[1]);
    assert_eq!(
        first, second,
        "the identical source produced different component identifiers"
    );
}

/// Build one revision of the same plan, differing only in the leg's times.
fn revision_pdf(start: &str, end: &str) -> Vec<(f64, f64, String)> {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, start),
            (END_X, end),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[(DATE_X, "2099-06-12"), (ACTIVITY_X, "U")],
    ));
    items
}

#[given("a supported duty plan whose later revision changes one flight's UTC time while its other activities are unchanged")]
async fn a_revised_snapshot(w: &mut DutyWorld) {
    let dir = fresh_dir();
    let original = dir.join("roster.pdf");
    let revised = dir.join("roster-revised.pdf");
    std::fs::write(
        &original,
        support::build_pdf(&revision_pdf("09:00", "10:00")),
    )
    .expect("write the original");
    std::fs::write(
        &revised,
        support::build_pdf(&revision_pdf("10:00", "11:00")),
    )
    .expect("write the revision");
    w.dir = Some(dir);

    w.source = Some(original);
    run_converter(w);
    let first = std::fs::read_to_string(w.working_dir().join("roster.ics"))
        .expect("the original conversion published no calendar");

    w.source = Some(revised);
    run_converter(w);
    let second = std::fs::read_to_string(w.working_dir().join("roster-revised.ics"))
        .expect("the revised conversion published no calendar");

    w.calendars = vec![first, second];
}

#[when("the crew member converts both revisions")]
async fn converts_both_revisions(w: &mut DutyWorld) {
    assert_eq!(
        w.calendars.len(),
        2,
        "both revisions must have been converted"
    );
}

#[then("every activity of the revised snapshot carries a component identifier distinct from the earlier snapshot")]
async fn revised_identifiers_are_distinct(w: &mut DutyWorld) {
    let original = component_ids(&w.calendars[0]);
    let revised = component_ids(&w.calendars[1]);
    assert!(
        !original.is_empty(),
        "the original snapshot carried no identifiers"
    );
    assert_eq!(
        original.len(),
        revised.len(),
        "the revision changed the number of activities"
    );
    for identifier in &revised {
        assert!(
            !original.contains(identifier),
            "the revised snapshot reused the identifier {identifier}"
        );
    }
}

#[given(
    regex = r"^a supported duty plan whose footer states Created (\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}) UTC$"
)]
async fn a_plan_with_a_revision_footer(w: &mut DutyWorld, date: String, time: String) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    // Footer material below the table carrying the source revision stamp.
    items.extend(support::at(
        520.0,
        &[
            (DATE_X, "Created"),
            (support::TRIP_X, &date),
            (support::REPORT_X, &time),
            (support::POS_X, "UTC"),
        ],
    ));
    write_fixture(w, &items);
}

#[then(
    regex = r"^the published calendar's revision timestamp is (\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}) UTC$"
)]
async fn revision_timestamp_is(w: &mut DutyWorld, date: String, time: String) {
    let calendar = calendar(w);
    let expected = format!("DTSTAMP:{}T{}00Z", compact(&date), digits(&time));
    assert!(
        calendar.contains(&expected),
        "expected the revision timestamp {expected}: {calendar}"
    );
}

#[given(
    regex = r"^a supported duty plan with no usable source revision metadata and a controlled construction clock of (\d{4}-\d{2}-\d{2}) (\d{2}:\d{2}) UTC$"
)]
async fn a_plan_without_a_revision_footer(w: &mut DutyWorld, date: String, time: String) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    );
    w.env = vec![(
        "DUTY2CAL_CONSTRUCTION_CLOCK".to_string(),
        format!("{date}T{time}:00Z"),
    )];
}

#[then("its activity dates are unaffected")]
async fn activity_dates_are_unaffected(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert!(
        calendar.contains("DTSTART:20990607T090000Z")
            && calendar.contains("DTEND:20990607T100000Z"),
        "the revision metadata moved an activity date: {calendar}"
    );
}

/// Contents of a calendar that must survive an unrelated conversion.
const EXISTING_CALENDAR: &str = "BEGIN:VCALENDAR\r\nEND:VCALENDAR\r\n";

#[given("the source roster.pdf beside it existing calendars roster.ics and roster-1.ics")]
async fn a_source_beside_existing_calendars(w: &mut DutyWorld) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    );
    for name in ["roster.ics", "roster-1.ics"] {
        std::fs::write(w.working_dir().join(name), EXISTING_CALENDAR).expect("write the calendar");
    }
}

#[then("a complete calendar is published as roster-2.ics")]
async fn published_as_roster_two(w: &mut DutyWorld) {
    let path = w.working_dir().join("roster-2.ics");
    let calendar = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no calendar at {}: {error}", path.display()));
    assert!(
        calendar.contains("BEGIN:VCALENDAR")
            && calendar.contains("BEGIN:VEVENT")
            && calendar.contains("SUMMARY:LU742 ABC-DEF"),
        "the published calendar is not complete: {calendar}"
    );
}

#[then("roster.ics and roster-1.ics are unchanged")]
async fn existing_calendars_unchanged(w: &mut DutyWorld) {
    for name in ["roster.ics", "roster-1.ics"] {
        let content = std::fs::read_to_string(w.working_dir().join(name))
            .unwrap_or_else(|error| panic!("{name} disappeared: {error}"));
        assert_eq!(
            content, EXISTING_CALENDAR,
            "{name} was modified by the conversion"
        );
    }
}

/// Contents of a diagnostic that must survive a later failure.
const EXISTING_DIAGNOSTIC: &str = "Source PDF: earlier.pdf\nReason: an earlier failure\n";

#[given("a supported duty plan with an unresolvable required activity and an existing roster.parse-error.txt beside it")]
async fn a_plan_with_a_failure_and_an_older_diagnostic(w: &mut DutyWorld) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "ZZ999"),
            (FROM_X, "ABC"),
        ],
    );
    std::fs::write(
        w.working_dir().join("roster.parse-error.txt"),
        EXISTING_DIAGNOSTIC,
    )
    .expect("write the earlier diagnostic");
}

#[then("the converter reports the failure")]
async fn converter_reports_the_failure(w: &mut DutyWorld) {
    let text = transcript(w);
    assert!(
        text.contains("conversion failed"),
        "no conversion failure was reported: {text}"
    );
}

#[then("roster.parse-error-1.txt names the source PDF and the reason")]
async fn numbered_diagnostic_names_source_and_reason(w: &mut DutyWorld) {
    let path = w.working_dir().join("roster.parse-error-1.txt");
    let report = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no numbered diagnostic at {}: {error}", path.display()));
    assert!(
        report.contains("Source PDF:") && report.contains("roster.pdf"),
        "the diagnostic does not identify the source: {report}"
    );
    assert!(
        report.contains("Reason:") && report.contains("ZZ999"),
        "the diagnostic does not state the current reason: {report}"
    );
}

#[then("roster.parse-error.txt is unchanged")]
async fn older_diagnostic_unchanged(w: &mut DutyWorld) {
    let content = std::fs::read_to_string(w.working_dir().join("roster.parse-error.txt"))
        .expect("the earlier diagnostic disappeared");
    assert_eq!(
        content, EXISTING_DIAGNOSTIC,
        "the earlier diagnostic was overwritten"
    );
}

/// Contents of a calendar that must survive a failed publication.
const SURVIVING_CALENDAR: &str = "BEGIN:VCALENDAR\r\nEND:VCALENDAR\r\n";

#[given("a supported duty plan whose activities resolve but whose calendar cannot be published")]
async fn a_plan_that_cannot_be_published(w: &mut DutyWorld) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    );
    let dir = w.working_dir();
    // The ordinary calendar name is a dangling link into a directory that does
    // not exist: the name looks free, and writing it fails.
    std::os::unix::fs::symlink(
        dir.join("missing-dir").join("roster.ics"),
        dir.join("roster.ics"),
    )
    .expect("create the dangling candidate");
    // A previously published calendar must survive the failed attempt.
    std::fs::write(dir.join("roster-1.ics"), SURVIVING_CALENDAR)
        .expect("write the existing calendar");
}

#[then("a new unused numbered diagnostic report is attempted")]
async fn a_diagnostic_is_attempted(w: &mut DutyWorld) {
    let text = transcript(w);
    assert!(
        text.contains("Diagnostic:"),
        "no diagnostic was attempted: {text}"
    );
    let reports: Vec<String> = std::fs::read_dir(w.working_dir())
        .expect("read the scenario directory")
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with("roster.parse-error"))
        .collect();
    assert_eq!(
        reports.len(),
        1,
        "expected exactly one new report for this failure: {reports:?}"
    );
}

#[then("existing calendars are unchanged")]
async fn existing_calendars_are_unchanged(w: &mut DutyWorld) {
    let content = std::fs::read_to_string(w.working_dir().join("roster-1.ics"))
        .expect("the existing calendar disappeared");
    assert_eq!(
        content, SURVIVING_CALENDAR,
        "the failed publication modified the existing calendar"
    );
}

/// An older diagnostic that must not be presented as the current one.
const OLDER_REPORT: &str = "Source PDF: earlier.pdf\nReason: an earlier failure\n";

#[given("a conversion failure and no writable destination for a new numbered report")]
async fn a_failure_without_a_writable_report(w: &mut DutyWorld) {
    // A required activity the converter cannot resolve, so the conversion fails
    // before any calendar is attempted.
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "ZZ999"),
            (FROM_X, "ABC"),
        ],
    );
    let dir = w.working_dir();
    // The ordinary report name is a dangling link into a directory that does
    // not exist: the name looks free and writing it fails.
    std::os::unix::fs::symlink(
        dir.join("missing-dir").join("roster.parse-error.txt"),
        dir.join("roster.parse-error.txt"),
    )
    .expect("create the dangling report candidate");
    std::fs::write(dir.join("roster.parse-error-1.txt"), OLDER_REPORT)
        .expect("write the older report");
}

#[then("the original failure remains visible")]
async fn original_failure_remains_visible(w: &mut DutyWorld) {
    let text = transcript(w);
    assert!(
        text.contains("conversion failed"),
        "the original failure is not visible: {text}"
    );
    let candidate = w.working_dir().join("roster.parse-error.txt");
    let claimed = format!("Diagnostic: {}", candidate.display());
    assert!(
        !text.contains(&claimed),
        "a report that was never saved was presented as current: {text}"
    );
}

#[then("the converter does not present an older report as current")]
async fn older_report_not_current(w: &mut DutyWorld) {
    let text = transcript(w);
    assert!(
        text.contains("no new report was saved"),
        "the unsaved diagnostic was not stated: {text}"
    );
    assert!(
        !text.contains("roster.parse-error-1.txt"),
        "the older report was presented as the current one: {text}"
    );
    let older = std::fs::read_to_string(w.working_dir().join("roster.parse-error-1.txt"))
        .expect("the older report disappeared");
    assert_eq!(older, OLDER_REPORT, "the older report was modified");
}

#[given("a conversion failure beside existing reports roster.parse-error.txt and roster.parse-error-1.txt")]
async fn a_failure_beside_two_reports(w: &mut DutyWorld) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "ZZ999"),
            (FROM_X, "ABC"),
        ],
    );
    let dir = w.working_dir();
    std::fs::write(dir.join("roster.parse-error.txt"), OLDER_REPORT)
        .expect("write the first report");
    std::fs::write(dir.join("roster.parse-error-1.txt"), OLDER_REPORT)
        .expect("write the second report");
}

#[then("the new diagnostic is written as roster.parse-error-2.txt")]
async fn new_diagnostic_is_the_third_name(w: &mut DutyWorld) {
    let path = w.working_dir().join("roster.parse-error-2.txt");
    let report = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no diagnostic at {}: {error}", path.display()));
    assert!(
        report.contains("Source PDF:") && report.contains("roster.pdf"),
        "the diagnostic does not identify the source: {report}"
    );
    assert!(
        report.contains("Reason:") && report.contains("ZZ999"),
        "the diagnostic does not state the current reason: {report}"
    );
}

#[then("roster.parse-error.txt and roster.parse-error-1.txt are unchanged")]
async fn both_older_reports_unchanged(w: &mut DutyWorld) {
    for name in ["roster.parse-error.txt", "roster.parse-error-1.txt"] {
        let content = std::fs::read_to_string(w.working_dir().join(name))
            .unwrap_or_else(|error| panic!("{name} disappeared: {error}"));
        assert_eq!(content, OLDER_REPORT, "{name} was modified");
    }
}

/// Split a printed activity cell into its crew position and flight number.
fn split_position_and_flight_number(printed: &str) -> Option<(&str, &str)> {
    let split = printed.find(|c: char| c.is_ascii_digit())?;
    let (marker, number) = printed.split_at(split);
    (!marker.is_empty() && number.chars().all(|c| c.is_ascii_digit())).then_some((marker, number))
}

/// The reference plan: the operator's 26-event example month (June 2026).
///
/// All values are generated; the layout reproduces the supported family's
/// detail table with the local-date, flight number, route and printed UTC columns.
fn reference_plan(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(800.0);
    let mut y = 785.0;
    let mut row = |items: &mut Vec<(f64, f64, String)>,
                   date: &str,
                   activity: &str,
                   from: &str,
                   to: &str,
                   start: &str,
                   end: &str| {
        let mut values: Vec<(f64, &str)> = vec![(DATE_X, date)];
        match split_position_and_flight_number(activity) {
            Some((marker, number)) => {
                values.push((POS_X, marker));
                values.push((ACTIVITY_X, number));
            }
            None => values.push((ACTIVITY_X, activity)),
        }
        if !from.is_empty() {
            values.push((FROM_X, from));
        }
        if !to.is_empty() {
            values.push((TO_X, to));
        }
        if !start.is_empty() {
            values.push((START_X, start));
        }
        if !end.is_empty() {
            values.push((END_X, end));
        }
        items.extend(support::at(y, &values));
        y -= 15.0;
    };

    for day in 1..=4 {
        row(
            &mut items,
            &format!("2098-06-0{day}"),
            "FREE",
            "",
            "",
            "",
            "",
        );
    }
    row(
        &mut items,
        "2098-06-07",
        "LU742",
        "Tychon",
        "Mareum",
        "09:25",
        "15:45",
    );
    row(
        &mut items,
        "2098-06-07",
        "LU742",
        "Mareum",
        "Clavius",
        "16:45",
        "18:10",
    );
    row(&mut items, "2098-06-08", "Layover: Clavius", "", "", "", "");
    row(
        &mut items,
        "2098-06-09",
        "LU743",
        "Clavius",
        "Mareum",
        "19:35",
        "21:00",
    );
    row(
        &mut items,
        "2098-06-09",
        "LU743",
        "Mareum",
        "Tychon",
        "22:10",
        "04:25",
    );
    row(
        &mut items,
        "2098-06-13",
        "LU402",
        "Tychon",
        "Utopis",
        "11:00",
        "13:25",
    );
    row(
        &mut items,
        "2098-06-13",
        "LU401",
        "Utopis",
        "Tychon",
        "14:25",
        "16:50",
    );
    row(
        &mut items,
        "2098-06-13",
        "LU302",
        "Tychon",
        "Kopernik",
        "18:00",
        "20:10",
    );
    row(
        &mut items,
        "2098-06-14",
        "Layover: Kopernik",
        "",
        "",
        "",
        "",
    );
    row(
        &mut items,
        "2098-06-15",
        "LU301",
        "Kopernik",
        "Tychon",
        "04:00",
        "06:20",
    );
    row(
        &mut items,
        "2098-06-15",
        "LU502",
        "Tychon",
        "Argyra",
        "07:30",
        "08:25",
    );
    row(
        &mut items,
        "2098-06-15",
        "LU501",
        "Argyra",
        "Tychon",
        "09:05",
        "10:10",
    );
    row(&mut items, "2098-06-18", "EM", "", "", "06:00", "10:00");
    row(
        &mut items,
        "2098-06-20",
        "LU304",
        "Tychon",
        "Olympia",
        "19:30",
        "07:05",
    );
    row(&mut items, "2098-06-22", "Layover: Olympia", "", "", "", "");
    row(
        &mut items,
        "2098-06-23",
        "LU303",
        "Olympia",
        "Tychon",
        "15:50",
        "05:30",
    );
    for day in 25..=29 {
        row(
            &mut items,
            &format!("2098-06-{day}"),
            "FREE",
            "",
            "",
            "",
            "",
        );
    }
    row(&mut items, "2098-06-30", "OFF", "", "", "", "");

    // The producer's own revision footer, below the table.
    items.extend(support::at(
        380.0,
        &[
            (DATE_X, "Created"),
            (DATE_X + 40.0, "21May2098"),
            (DATE_X + 90.0, "15:52"),
            (DATE_X + 120.0, "(UTC)"),
            (DATE_X + 150.0, "by"),
            (DATE_X + 170.0, "Selene"),
        ],
    ));

    let dir = fresh_dir();
    let source = dir.join("roster.pdf");
    std::fs::write(&source, support::build_pdf(&items)).expect("write the reference plan");
    w.dir = Some(dir);
    w.source = Some(source);
    // The footer states the source revision, which outranks the construction
    // clock this plan sets for reproducibility.
    w.env = vec![(
        "DUTY2CAL_CONSTRUCTION_CLOCK".to_string(),
        "2098-06-10T00:00:00Z".to_string(),
    )];
}

#[given("the generated reference roster PDF and its expected calendar fixture")]
async fn the_reference_roster(w: &mut DutyWorld) {
    reference_plan(w);
}

#[when("the crew member converts the reference roster")]
async fn converts_the_reference_roster(w: &mut DutyWorld) {
    run_converter(w);
}

#[then("the published calendar equals the expected calendar after CRLF-to-LF and folding normalisation")]
async fn equals_the_expected_calendar(w: &mut DutyWorld) {
    let expected_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/expected/2098-06.ics");
    let published = calendar(w);
    let expected = std::fs::read_to_string(&expected_path).unwrap_or_else(|error| {
        panic!(
            "no expected calendar at {}: {error}",
            expected_path.display()
        )
    });
    assert_eq!(
        normalise(&published),
        normalise(&expected),
        "the published calendar differs from the expected one"
    );
}

/// CRLF to LF, and folded lines joined back (the agreed normalisation).
fn normalise(calendar: &str) -> String {
    calendar.replace("\r\n ", "").replace("\r\n", "\n")
}

#[given("a supported duty plan whose first detail row prints no date")]
async fn a_plan_whose_first_row_prints_no_date(w: &mut DutyWorld) {
    // The row carries a route and a clock, so it is an activity, but no row
    // above it prints a local date: nothing can date it.
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[then(regex = r#"^the conversion failure names "([^"]+)"$"#)]
async fn the_conversion_failure_names(w: &mut DutyWorld, reason: String) {
    let text = transcript(w);
    assert!(
        text.contains(&reason),
        "the failure does not name {reason:?}: {text}"
    );
}

#[given("a supported duty plan with a layover marker whose station sits in the following column")]
async fn a_plan_with_a_split_layover_marker(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    // The marker and its station land in different columns.
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-06-08"),
            (LAYOVER_X, "Layover:"),
            (TAGS_X, "ABC"),
        ],
    ));
    write_fixture(w, &items);
}

#[given("a supported duty plan whose dated row prints a weekday that contradicts the day")]
async fn a_plan_with_a_contradicting_weekday(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        830.0,
        &[
            (DATE_X, "Month:"),
            (DATE_X + 40.0, "June"),
            (DATE_X + 100.0, "2099"),
        ],
    ));
    // 2099-06-07 is a Sunday, so the printed weekday cannot date the row.
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "07 Mon"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[given("a supported duty plan whose dated rows print no roster month heading")]
async fn a_plan_without_a_roster_heading(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    // The family's own spelling, with no heading to supply the month.
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "07 Sun"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[given("a supported duty plan whose detail-table header omits the From label")]
async fn a_plan_without_the_from_column(w: &mut DutyWorld) {
    let mut items = support::at(
        600.0,
        &[
            (DATE_X, "Date"),
            (DATE_X + 20.0, "(LT)"),
            (ACTIVITY_X, "Activity"),
            (TO_X, "To"),
            (START_X, "Start"),
            (START_X + 24.0, "(UTC)"),
            (END_X, "End"),
            (END_X + 16.0, "(UTC)"),
        ],
    );
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[then(regex = r"^the calendar contains a layover activity on (\d{4}-\d{2}-\d{2}) at (\w+)$")]
async fn a_layover_activity_on(w: &mut DutyWorld, date: String, station: String) {
    let calendar = calendar(w);
    assert!(
        calendar.contains(&format!("SUMMARY:Layover {station}")),
        "the layover for {station} is missing: {calendar}"
    );
    assert!(
        calendar.contains(&format!("DTSTART;VALUE=DATE:{}", compact(&date))),
        "the layover is not dated {date}: {calendar}"
    );
}

#[given("a supported duty plan whose arrival row has no departure partner")]
async fn a_plan_with_a_lone_arrival(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU900"),
            (TO_X, "NEF"),
            (END_X, "10:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[given("a supported duty plan with an unknown activity code and no route")]
async fn a_plan_with_a_codeless_route(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[(DATE_X, "2099-06-07"), (ACTIVITY_X, "LU900")],
    ));
    write_fixture(w, &items);
}

#[given("a supported duty plan whose split leg prints its aircraft on the departure only")]
async fn a_plan_with_a_one_sided_field(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (START_X, "09:00"),
            (AIRCRAFT_X, "LU7"),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (TO_X, "NEF"),
            (END_X, "10:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[given(regex = r#"^a supported duty plan whose travel leg carries the tag "([^"]*)"$"#)]
async fn a_plan_with_a_tag(w: &mut DutyWorld, tag: String) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
            (TAGS_X, &tag),
        ],
    ));
    write_fixture(w, &items);
}

#[then(regex = r#"^the travel activity's description carries "([^"]*)"$"#)]
async fn the_description_carries(w: &mut DutyWorld, value: String) {
    let calendar = calendar(w);
    let unfolded = calendar.replace("\r\n ", "");
    assert!(
        unfolded.contains(&value),
        "the description does not carry {value:?}: {calendar}"
    );
}

#[then(regex = r#"^the calendar text carries "([^"]*)"$"#)]
async fn the_calendar_text_carries(w: &mut DutyWorld, value: String) {
    let calendar = calendar(w);
    assert!(
        calendar.contains(&value),
        "the calendar text does not carry {value:?}: {calendar}"
    );
}

/// One dated row whose date cell is printed exactly as given, under a heading.
fn plan_with_a_printed_date_cell(w: &mut DutyWorld, heading: Option<&[&str]>, cell: &str) {
    let mut items = support::detail_header_row(600.0);
    if let Some(parts) = heading {
        items.extend(support::at(
            830.0,
            &[
                (DATE_X, "Month:"),
                (DATE_X + 40.0, parts[0]),
                (DATE_X + 100.0, parts[1]),
            ],
        ));
    }
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, cell),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[given("a supported duty plan whose dated row prints only a day number")]
async fn a_plan_with_a_single_token_date(w: &mut DutyWorld) {
    plan_with_a_printed_date_cell(w, Some(&["June", "2099"]), "07");
}

#[given("a supported duty plan whose dated row prints an unreadable day")]
async fn a_plan_with_an_unreadable_day(w: &mut DutyWorld) {
    plan_with_a_printed_date_cell(w, Some(&["June", "2099"]), "aa Mon");
}

#[given("a supported duty plan whose dated row prints an unknown weekday")]
async fn a_plan_with_an_unknown_weekday(w: &mut DutyWorld) {
    plan_with_a_printed_date_cell(w, Some(&["June", "2099"]), "07 Xyz");
}

#[given("a supported duty plan whose dated row names a day the month does not have")]
async fn a_plan_with_an_impossible_day(w: &mut DutyWorld) {
    plan_with_a_printed_date_cell(w, Some(&["June", "2099"]), "31 Mon");
}

#[given("a supported duty plan whose roster heading prints the year touching the month")]
async fn a_plan_with_a_joined_heading(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    // The extraction joins a month and a year that touch, as it does for long
    // month names.
    items.extend(support::at(
        830.0,
        &[(DATE_X, "Month:"), (DATE_X + 40.0, "June2099")],
    ));
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "07 Sun"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    write_fixture(w, &items);
}

#[then(regex = r"^the calendar contains a travel activity on (\d{4}-\d{2}-\d{2})$")]
async fn a_travel_activity_on(w: &mut DutyWorld, date: String) {
    let calendar = calendar(w);
    assert!(
        calendar.contains("SUMMARY:LU742 ABC-NEF")
            && calendar.contains(&format!("DTSTART:{}T090000Z", compact(&date))),
        "no travel activity on {date}: {calendar}"
    );
}

#[given("a supported duty plan whose revision footer names no UTC zone")]
async fn a_plan_with_a_zoneless_footer(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    items.extend(support::at(
        520.0,
        &[
            (DATE_X, "Created"),
            (DATE_X + 40.0, "21May2098"),
            (DATE_X + 90.0, "15:52"),
            (DATE_X + 120.0, "CET"),
        ],
    ));
    write_fixture(w, &items);
    w.env = vec![(
        "DUTY2CAL_CONSTRUCTION_CLOCK".to_string(),
        "2099-06-10T00:00:00Z".to_string(),
    )];
}

#[then("the calendar's revision is the construction clock")]
async fn the_revision_is_the_clock(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert!(
        calendar.contains("DTSTAMP:20990610T000000Z"),
        "the zoneless footer supplied the revision: {calendar}"
    );
}

#[given("a supplied document that carries no detail-table labels")]
async fn a_document_without_table_labels(w: &mut DutyWorld) {
    let items = support::at(
        600.0,
        &[(DATE_X, "This page is prose, not a roster export.")],
    );
    write_fixture(w, &items);
}

#[when("the invocation supplies no duty plan")]
async fn the_invocation_supplies_no_plan(w: &mut DutyWorld) {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_duty2cal"))
        .output()
        .expect("run the converter");
    w.exit_status = output.status.code();
    w.stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    w.stderr = String::from_utf8_lossy(&output.stderr).into_owned();
}

#[then(regex = r#"^the transcript states "([^"]*)"$"#)]
async fn the_transcript_states(w: &mut DutyWorld, expected: String) {
    let text = transcript(w);
    assert!(
        text.contains(&expected),
        "the transcript does not state {expected:?}: {text}"
    );
}

#[given("a supplied duty plan path that does not exist")]
async fn a_missing_duty_plan(w: &mut DutyWorld) {
    let dir = fresh_dir();
    w.source = Some(dir.join("missing.pdf"));
    w.dir = Some(dir);
}

#[given("a supported duty plan with no construction clock supplied")]
async fn a_plan_without_a_clock(w: &mut DutyWorld) {
    plan_with_row(
        w,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    );
}

#[then("the calendar's revision is a current UTC stamp")]
async fn the_revision_is_current(w: &mut DutyWorld) {
    let stamp = calendar(w)
        .lines()
        .find_map(|line| line.strip_prefix("DTSTAMP:").map(str::to_string))
        .expect("the calendar carries a revision stamp");
    assert!(
        stamp.len() == 16 && stamp.ends_with('Z') && stamp.as_str() >= "20260101T000000Z",
        "the revision is not a current UTC stamp: {stamp}"
    );
}

#[given("a supported duty plan whose row prints only a station")]
async fn a_plan_with_a_station_only_row(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[(DATE_X, "2099-06-07"), (FROM_X, "ABC")],
    ));
    write_fixture(w, &items);
}

#[given("a supported duty plan whose split leg prints its aircraft on the arrival only")]
async fn a_plan_with_an_arrival_only_field(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (START_X, "09:00"),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (TO_X, "NEF"),
            (END_X, "10:00"),
            (AIRCRAFT_X, "LU7"),
        ],
    ));
    write_fixture(w, &items);
}

#[given("a supplied duty plan whose name is not valid text")]
async fn a_plan_with_a_binary_name(w: &mut DutyWorld) {
    use std::os::unix::ffi::OsStrExt;
    let dir = fresh_dir();
    let name = std::ffi::OsStr::from_bytes(b"ro\xff.pdf");
    let source = dir.join(name);
    // A readable roster, carried under a name that has no textual form.
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    std::fs::write(&source, support::build_pdf(&items)).expect("write the source");
    w.source = Some(source);
    w.dir = Some(dir);
}

#[given("a supported duty plan whose travel leg carries a long tag with accented letters")]
async fn a_plan_with_a_long_accented_tag(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
            (TAGS_X, &"\u{e4}".repeat(60)),
        ],
    ));
    write_fixture(w, &items);
}

#[given("a supported duty plan whose revision footer prints an unusable stamp")]
async fn a_plan_with_an_unusable_footer_stamp(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "ABC"),
            (TO_X, "NEF"),
            (START_X, "09:00"),
            (END_X, "10:00"),
        ],
    ));
    items.extend(support::at(
        520.0,
        &[
            (DATE_X, "Created"),
            (DATE_X + 40.0, "123May2098"),
            (DATE_X + 110.0, "15:52"),
            (DATE_X + 140.0, "(UTC)"),
        ],
    ));
    write_fixture(w, &items);
    w.env = vec![(
        "DUTY2CAL_CONSTRUCTION_CLOCK".to_string(),
        "2099-06-10T00:00:00Z".to_string(),
    )];
}

// ---------------------------------------------------------------------------
// Browser variants of the capability (design.md "Step definition locations").
//
// The bodies are the RED stubs: each panics with its own step name, so no
// browser scenario can pass before its step is implemented.
// ---------------------------------------------------------------------------

/// The driver for the current scenario, started on first use.
fn driver(w: &mut DutyWorld) -> &mut BrowserDriver {
    w.browser.get_or_insert_with(BrowserDriver::start)
}

/// The page's current observation object.
fn page_state(w: &mut DutyWorld) -> Value {
    driver(w).send("read_state", Map::new())
}

/// Write a named source fixture into a fresh scenario directory.
fn named_source(w: &mut DutyWorld, name: &str, bytes: Vec<u8>) {
    let dir = fresh_dir();
    let source = dir.join(name);
    std::fs::write(&source, bytes).expect("write the source fixture");
    w.dir = Some(dir);
    w.source = Some(source);
}

/// Supply the scenario's source through the real file picker.
fn supply_through_picker(w: &mut DutyWorld) {
    let path = w.source_path();
    let mut params = Map::new();
    params.insert(
        "path".to_string(),
        Value::String(path.display().to_string()),
    );
    driver(w).send("set_file", params);
}

/// Save one offered download and return the file it was written to.
fn capture_offer(w: &mut DutyWorld, kind: &str, file_name: &str) -> PathBuf {
    let path = w.working_dir().join(file_name);
    let mut params = Map::new();
    params.insert("kind".to_string(), Value::String(kind.to_string()));
    params.insert(
        "path".to_string(),
        Value::String(path.display().to_string()),
    );
    driver(w).send("save_offer", params);
    path
}

/// The diagnostic the page offers, read back from the captured download.
fn captured_diagnostic(w: &mut DutyWorld) -> String {
    let path = capture_offer(w, "diagnostic", "offered.parse-error.txt");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read the offered diagnostic {}: {error}", path.display()))
}

/// Assert the page shows a failure heading and a reason containing `needle`.
fn assert_failure_containing(w: &mut DutyWorld, needle: &str) {
    let state = page_state(w);
    assert_eq!(
        state["state"].as_str(),
        Some("failure"),
        "expected the visible failure state: {state}"
    );
    let text = state["visibleText"]
        .as_str()
        .expect("read_state returns visible text");
    assert!(
        text.contains("Conversion failed"),
        "the failure heading is not visible: {text}"
    );
    let reason = state["failureReason"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    assert!(
        reason.contains(needle),
        "the visible reason {reason:?} does not contain {needle:?}"
    );
}

/// Assert the page offers no calendar download.
fn assert_no_calendar_offer(w: &mut DutyWorld) {
    let state = page_state(w);
    let calendar = state["offers"]
        .as_array()
        .expect("read_state returns offers")
        .iter()
        .any(|offer| offer["kind"].as_str() == Some("calendar"));
    assert!(!calendar, "the page offered a calendar: {state}");
}

/// A generated June 2026 plan whose Created footer supplies the revision.
fn equivalence_plan() -> Vec<(f64, f64, String)> {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2026-06-07"),
            (ACTIVITY_X, "LU742"),
            (FROM_X, "Tychon"),
            (TO_X, "Mareum"),
            (START_X, "09:25"),
            (END_X, "15:45"),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[(DATE_X, "2026-06-08"), (ACTIVITY_X, "U")],
    ));
    items.extend(support::at(
        520.0,
        &[
            (DATE_X, "Created"),
            (support::TRIP_X, "2026-05-21"),
            (support::REPORT_X, "15:52"),
            (support::POS_X, "UTC"),
        ],
    ));
    items
}

#[given("the converter page served from the built Site output")]
async fn the_converter_page(w: &mut DutyWorld) {
    driver(w).send("start", Map::new());
}

#[given(
    "the converter page served from the built Site output in a browser without ES-module support"
)]
async fn the_converter_page_without_modules(_w: &mut DutyWorld) {
    unimplemented!(
        "the converter page served from the built Site output in a browser without ES-module support"
    )
}

#[given("the converter page served from the built Site output in a browser without WebAssembly")]
async fn the_converter_page_without_webassembly(_w: &mut DutyWorld) {
    unimplemented!(
        "the converter page served from the built Site output in a browser without WebAssembly"
    )
}

#[given("the converter page served from the built Site output that cannot create a download offer for the calendar")]
async fn the_converter_page_without_calendar_offer(_w: &mut DutyWorld) {
    unimplemented!(
        "the converter page served from the built Site output that cannot create a download offer for the calendar"
    )
}

#[given("the converter page served from the built Site output that cannot create a download offer for the diagnostic")]
async fn the_converter_page_without_diagnostic_offer(_w: &mut DutyWorld) {
    unimplemented!(
        "the converter page served from the built Site output that cannot create a download offer for the diagnostic"
    )
}

#[given("the converter page served from the built Site output whose converter module cannot be instantiated")]
async fn the_converter_page_with_a_broken_module(_w: &mut DutyWorld) {
    unimplemented!(
        "the converter page served from the built Site output whose converter module cannot be instantiated"
    )
}

#[given(
    regex = r#"^the supported duty plan "([^"]+)" whose Created footer supplies its revision timestamp$"#
)]
async fn the_plan_with_a_created_footer(w: &mut DutyWorld, name: String) {
    named_source(w, &name, support::build_pdf(&equivalence_plan()));
}

#[given("the executable has published the calendar for that plan")]
async fn the_executable_has_published_the_calendar(w: &mut DutyWorld) {
    run_converter(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the executable did not publish the calendar: {}",
        transcript(w)
    );
    let source = w.source_path();
    let name = source.file_name().unwrap_or_default().to_string_lossy();
    let stem = name.strip_suffix(".pdf").unwrap_or(&name);
    let published = w.working_dir().join(format!("{stem}.ics"));
    assert!(
        published.is_file(),
        "the executable published no calendar at {}: {}",
        published.display(),
        transcript(w)
    );
}

#[when(
    regex = r#"^the crew member supplies the supported duty plan "([^"]+)" through the file picker$"#
)]
async fn supplies_the_supported_plan(_w: &mut DutyWorld, _name: String) {
    unimplemented!("the crew member supplies the supported duty plan through the file picker")
}

#[when("the crew member supplies that plan through the file picker")]
async fn supplies_that_plan(w: &mut DutyWorld) {
    supply_through_picker(w);
}

#[when(
    regex = r#"^the crew member supplies the file "([^"]+)" that is not a supported duty plan$"#
)]
async fn supplies_an_unsupported_file(w: &mut DutyWorld, name: String) {
    // A readable PDF without the supported detail-table header: the family
    // check rejects it, not the page's size guard and not a read error.
    let items = support::at(
        700.0,
        &[(DATE_X, "Notes"), (FROM_X, "This page is not a duty plan.")],
    );
    named_source(w, &name, support::build_pdf(&items));
    supply_through_picker(w);
}

#[when(regex = r#"^the crew member supplies the plan "([^"]+)" of (\d+) MiB$"#)]
async fn supplies_an_oversized_plan(w: &mut DutyWorld, name: String, size: String) {
    let bytes = support::build_pdf_at_size(&equivalence_plan(), mebibytes(&size));
    assert_eq!(bytes.len(), mebibytes(&size));
    named_source(w, &name, bytes);
    supply_through_picker(w);
}

#[when(
    regex = r#"^the crew member supplies the supported duty plan "([^"]+)" containing a required activity the converter cannot resolve$"#
)]
async fn supplies_an_unresolvable_plan(_w: &mut DutyWorld, _name: String) {
    unimplemented!(
        "the crew member supplies a plan containing a required activity the converter cannot resolve"
    )
}

/// Parse the MiB count in a step.
fn mebibytes(size: &str) -> usize {
    size.parse::<usize>()
        .expect("the step carries an integer MiB count")
        * 1024
        * 1024
}

#[when(
    regex = r#"^the crew member supplies the supported duty plan "([^"]+)" of exactly (\d+) MiB$"#
)]
async fn supplies_a_plan_at_the_size_bound(w: &mut DutyWorld, name: String, size: String) {
    let bytes = support::build_pdf_at_size(&equivalence_plan(), mebibytes(&size));
    assert_eq!(bytes.len(), mebibytes(&size));

    // Builder-to-reader check: the reader must still recognize the padded
    // fixture, so this scenario proves the bound and not a rejected file.
    let dir = fresh_dir();
    let source = dir.join(&name);
    std::fs::write(&source, &bytes).expect("write the size-bound fixture");
    let output = Command::new(env!("CARGO_BIN_EXE_duty2cal"))
        .arg(&source)
        .output()
        .expect("run the converter on the size-bound fixture");
    assert!(
        output.status.success(),
        "the reader rejected the exactly-{size}-MiB fixture: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        dir.join("2026-06.ics").is_file(),
        "the reader accepted the fixture but published no calendar"
    );

    w.dir = Some(dir);
    w.source = Some(source);
    supply_through_picker(w);
}

#[when(regex = r#"^the crew member drops the supported duty plan "([^"]+)" onto the drop zone$"#)]
async fn drops_the_supported_plan(_w: &mut DutyWorld, _name: String) {
    unimplemented!("the crew member drops the supported duty plan onto the drop zone")
}

#[when("the crew member dismisses the save of the offered calendar")]
async fn dismisses_the_save(_w: &mut DutyWorld) {
    unimplemented!("the crew member dismisses the save of the offered calendar")
}

#[when("the crew member reloads the page")]
async fn reloads_the_page(_w: &mut DutyWorld) {
    unimplemented!("the crew member reloads the page")
}

#[then("the page shows that the calendar is ready")]
async fn the_calendar_is_ready(w: &mut DutyWorld) {
    let state = page_state(w);
    assert_eq!(
        state["state"].as_str(),
        Some("ready"),
        "the page did not reach the ready state: {state}"
    );
    let text = state["visibleText"]
        .as_str()
        .expect("read_state returns visible text");
    assert!(
        text.contains("Calendar ready"),
        "the ready heading is not visible: {text}"
    );
    assert_eq!(
        state["controls"]["picker"].as_bool(),
        Some(true),
        "the picker is not enabled in the ready state: {state}"
    );
}

#[then("the page shows that the drop was accepted and the calendar is ready")]
async fn the_drop_was_accepted(_w: &mut DutyWorld) {
    unimplemented!("the page shows that the drop was accepted and the calendar is ready")
}

#[then("the page shows a failure heading and a failure naming the 20 MiB bound")]
async fn the_failure_names_the_size_bound(w: &mut DutyWorld) {
    assert_failure_containing(w, "20 MiB");
}

#[then(regex = r#"^the page shows a failure heading and a failure naming "([^"]+)"$"#)]
async fn the_failure_names_the_file(w: &mut DutyWorld, name: String) {
    assert_failure_containing(w, &name);
}

#[then("the page shows a failure heading and the reason")]
async fn the_failure_shows_the_reason(_w: &mut DutyWorld) {
    unimplemented!("the page shows a failure heading and the reason")
}

#[then("the page shows a failure heading and states that the calendar could not be offered")]
async fn the_failure_states_the_calendar_could_not_be_offered(_w: &mut DutyWorld) {
    unimplemented!(
        "the page shows a failure heading and states that the calendar could not be offered"
    )
}

#[then(regex = r#"^the page shows a failure heading and the primary failure naming "([^"]+)"$"#)]
async fn the_failure_keeps_the_primary_reason(_w: &mut DutyWorld, _name: String) {
    unimplemented!("the page shows a failure heading and the primary failure naming the file")
}

#[then("the page shows a cannot-run heading and states that WebAssembly is required")]
async fn the_cannot_run_message_names_webassembly(_w: &mut DutyWorld) {
    unimplemented!("the page shows a cannot-run heading and states that WebAssembly is required")
}

#[then("the page shows a cannot-run heading and states that ES modules are required")]
async fn the_cannot_run_message_names_es_modules(_w: &mut DutyWorld) {
    unimplemented!("the page shows a cannot-run heading and states that ES modules are required")
}

#[then("the page shows a cannot-run heading and states that the converter could not start")]
async fn the_cannot_run_message_names_startup(_w: &mut DutyWorld) {
    unimplemented!(
        "the page shows a cannot-run heading and states that the converter could not start"
    )
}

#[then(regex = r#"^the page offers the file "([^"]+)"$"#)]
async fn the_page_offers_the_file(w: &mut DutyWorld, name: String) {
    let state = page_state(w);
    let offered = state["offers"]
        .as_array()
        .expect("read_state returns offers")
        .iter()
        .any(|offer| {
            offer["kind"].as_str() == Some("calendar")
                && offer["name"].as_str() == Some(name.as_str())
        });
    assert!(
        offered,
        "the page does not offer the calendar {name:?}: {state}"
    );
}

#[then("the page offers no calendar")]
async fn the_page_offers_no_calendar(w: &mut DutyWorld) {
    assert_no_calendar_offer(w);
}

#[then(regex = r#"^the page offers no calendar and no longer offers "([^"]+)"$"#)]
async fn the_page_withdraws_the_calendar(_w: &mut DutyWorld, _name: String) {
    unimplemented!("the page offers no calendar and no longer offers the earlier one")
}

#[then("the page offers no file picker")]
async fn the_page_offers_no_file_picker(_w: &mut DutyWorld) {
    unimplemented!("the page offers no file picker")
}

#[then(regex = r#"^the page offers the diagnostic text "([^"]+)"$"#)]
async fn the_page_offers_the_diagnostic(w: &mut DutyWorld, name: String) {
    let state = page_state(w);
    let offered = state["offers"]
        .as_array()
        .expect("read_state returns offers")
        .iter()
        .any(|offer| {
            offer["kind"].as_str() == Some("diagnostic")
                && offer["name"].as_str() == Some(name.as_str())
        });
    assert!(
        offered,
        "the page does not offer the diagnostic {name:?}: {state}"
    );
}

#[then(r#"the diagnostic text identifies "2026-06.pdf" and the size failure"#)]
async fn the_diagnostic_names_the_size_failure(w: &mut DutyWorld) {
    let text = captured_diagnostic(w);
    assert!(
        text.starts_with("Source PDF: 2026-06.pdf\nReason: "),
        "the diagnostic does not identify the plan: {text:?}"
    );
    assert!(
        text.contains("20 MiB") && text.ends_with('\n'),
        "the diagnostic does not state the size failure: {text:?}"
    );
}

#[then(regex = r#"^the diagnostic text identifies "([^"]+)" and the unsupported-family reason$"#)]
async fn the_diagnostic_names_the_unsupported_family(w: &mut DutyWorld, name: String) {
    let text = captured_diagnostic(w);
    assert!(
        text.starts_with(&format!("Source PDF: {name}\nReason: ")),
        "the diagnostic does not identify the file: {text:?}"
    );
    assert!(
        text.contains("not a supported Released Roster detail-table export")
            && text.ends_with('\n'),
        "the diagnostic does not state the unsupported-family reason: {text:?}"
    );
}

#[then(regex = r#"^the diagnostic text identifies "([^"]+)" and the reason$"#)]
async fn the_diagnostic_names_the_reason(_w: &mut DutyWorld, _name: String) {
    unimplemented!("the diagnostic text identifies the file and the reason")
}

#[then("the page offers a way to choose another plan")]
async fn the_page_offers_a_way_back(w: &mut DutyWorld) {
    let state = page_state(w);
    assert_eq!(
        state["controls"]["chooseAnother"].as_bool(),
        Some(true),
        "the failure state offers no way back: {state}"
    );
    let text = state["visibleText"]
        .as_str()
        .expect("read_state returns visible text");
    assert!(
        text.contains("Choose another plan"),
        "the visible way back is missing: {text}"
    );
}

#[then("the page states that no diagnostic was offered")]
async fn the_page_states_no_diagnostic_was_offered(_w: &mut DutyWorld) {
    unimplemented!("the page states that no diagnostic was offered")
}

#[then("the page has sent no request to another origin since it loaded")]
async fn the_page_sent_no_request_to_another_origin(_w: &mut DutyWorld) {
    unimplemented!("the page has sent no request to another origin since it loaded")
}

#[then("the page has stored nothing in persistent browser storage")]
async fn the_page_stored_nothing(_w: &mut DutyWorld) {
    unimplemented!("the page has stored nothing in persistent browser storage")
}

#[then("the page's address is unchanged and the browser history has no new entry")]
async fn the_page_address_is_unchanged(_w: &mut DutyWorld) {
    unimplemented!("the page's address is unchanged and the browser history has no new entry")
}

#[then("the page still offers the calendar")]
async fn the_page_still_offers_the_calendar(_w: &mut DutyWorld) {
    unimplemented!("the page still offers the calendar")
}

#[then("the offered calendar is byte-identical to the executable's calendar")]
async fn the_offered_calendar_is_byte_identical(w: &mut DutyWorld) {
    let state = page_state(w);
    assert_eq!(
        state["state"].as_str(),
        Some("ready"),
        "the page did not reach the ready state: {state}"
    );

    let offered_path = capture_offer(w, "calendar", "offered.ics");
    let offered = std::fs::read(&offered_path)
        .unwrap_or_else(|error| panic!("read the offered calendar: {error}"));

    let source = w.source_path();
    let name = source.file_name().unwrap_or_default().to_string_lossy();
    let stem = name.strip_suffix(".pdf").unwrap_or(&name);
    let executable_path = w.working_dir().join(format!("{stem}.ics"));
    let executable = std::fs::read(&executable_path)
        .unwrap_or_else(|error| panic!("read the executable's calendar: {error}"));

    assert_eq!(
        offered, executable,
        "the browser's calendar differs from the executable's calendar"
    );

    let fixture_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/expected/2026-06.ics");
    let fixture = std::fs::read(&fixture_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", fixture_path.display()));
    assert_eq!(
        executable, fixture,
        "the executable's calendar differs from the committed fixture"
    );

    let text = String::from_utf8_lossy(&offered);
    assert!(
        text.starts_with("BEGIN:VCALENDAR") && text.trim_end().ends_with("END:VCALENDAR"),
        "the offered download is not a complete VCALENDAR: {text}"
    );
}
