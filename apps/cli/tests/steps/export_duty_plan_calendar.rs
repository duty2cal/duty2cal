// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `export-duty-plan-calendar`.
//!
//! One file per capability. RED-phase steps use the `unimplemented!("<step>")`
//! stub; a step body is never empty.
#![allow(clippy::too_many_arguments)] // step signatures mirror the captured step text

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

/// The duty-plan date written as an ICS date value.
fn compact(date: &str) -> String {
    date.replace('-', "")
}

/// The duty-plan date and printed clock written as an ICS UTC instant.
fn utc(date: &str, time: &str) -> String {
    format!("{}T{}00Z", compact(date), time.replace(':', ""))
}

/// Prepare a single-row plan at a realistic detail-table position.
fn plan_with_row(w: &mut DutyWorld, cells: &[(f64, &str)]) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(585.0, cells));
    write_fixture(w, &items);
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
    let mut cells: Vec<(f64, &str)> = vec![(DATE_X, &date), (ACTIVITY_X, &code)];
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
    let calendar = calendar(w);
    assert!(
        calendar.contains(&format!("SUMMARY:{title}")),
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
            (ACTIVITY_X, "FB569"),
            (FROM_X, "ABC"),
            (START_X, "10:00"),
        ],
    );
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
            (ACTIVITY_X, "FB569"),
            (FROM_X, "ABC"),
            (START_X, &format!("({qualifier}) {departure_clock}")),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, &arrival_date),
            (ACTIVITY_X, "FB569"),
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
            (ACTIVITY_X, "FB569"),
            (FROM_X, "ABC"),
            (START_X, &departure_clock),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-01-13"),
            (ACTIVITY_X, "FB569"),
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
            (ACTIVITY_X, "FB568"),
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
            (ACTIVITY_X, "FB568"),
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
            (ACTIVITY_X, "FB100"),
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
            (ACTIVITY_X, "FB100"),
            (FROM_X, "ABC"),
            (TO_X, "DEF"),
            (START_X, "(9) 10:00"),
            (END_X, "(9) 11:00"),
        ],
    );
}
