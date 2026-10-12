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

use crate::steps::support::{
    self, ACTIVITY_X, AIRCRAFT_X, BLOCKTIME_X, DATE_X, END_X, FROM_X, LAYOVER_X, START_X, TAGS_X,
    TO_X, TZ_X,
};
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

/// Invoke the converter once on the scenario's source.
fn run_converter(w: &mut DutyWorld) {
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
        (ACTIVITY_X, "FB568".to_string()),
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
            (ACTIVITY_X, "FB568"),
            (FROM_X, "ABC"),
            (START_X, "09:00"),
            (AIRCRAFT_X, "ABC"),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "FB568"),
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
            (ACTIVITY_X, "FB568"),
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

#[given("a compatible departure and arrival fragment of one travel leg supplying Report (LT) 07:30 and Report (LT) 08:00 respectively")]
async fn differing_report_times(w: &mut DutyWorld) {
    let mut items = support::detail_header_row(600.0);
    items.extend(support::at(
        585.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "FB568"),
            (FROM_X, "ABC"),
            (START_X, "09:00"),
            (support::REPORT_X, "07:30"),
        ],
    ));
    items.extend(support::at(
        570.0,
        &[
            (DATE_X, "2099-06-07"),
            (ACTIVITY_X, "FB568"),
            (TO_X, "DEF"),
            (END_X, "10:00"),
            (support::REPORT_X, "08:00"),
        ],
    ));
    write_fixture(w, &items);
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
            (ACTIVITY_X, "FB568"),
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
            (ACTIVITY_X, "FB569"),
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
            (ACTIVITY_X, "FB568"),
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
            (ACTIVITY_X, "FB568"),
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
            (ACTIVITY_X, "FB568"),
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
            (ACTIVITY_X, "FB568"),
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
            (ACTIVITY_X, "FB568"),
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
            && calendar.contains("SUMMARY:FB568 ABC-DEF"),
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
            (ACTIVITY_X, "FB568"),
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

/// The reference plan: the operator's 26-event example month (June 2026).
///
/// All values are generated; the layout reproduces the supported family's
/// detail table with the local-date, designator, route and printed UTC columns.
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
        let mut values: Vec<(f64, &str)> = vec![(DATE_X, date), (ACTIVITY_X, activity)];
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
            &format!("2026-06-0{day}"),
            "FREE",
            "",
            "",
            "",
            "",
        );
    }
    row(
        &mut items,
        "2026-06-07",
        "FB568",
        "FRA",
        "LOS",
        "09:25",
        "15:45",
    );
    row(
        &mut items,
        "2026-06-07",
        "FB568",
        "LOS",
        "SSG",
        "16:45",
        "18:10",
    );
    row(&mut items, "2026-06-08", "Layover", "SSG", "", "", "");
    row(
        &mut items,
        "2026-06-09",
        "FB569",
        "SSG",
        "LOS",
        "19:35",
        "21:00",
    );
    row(
        &mut items,
        "2026-06-09",
        "FB569",
        "LOS",
        "FRA",
        "22:10",
        "04:25",
    );
    row(
        &mut items,
        "2026-06-13",
        "FB1466",
        "FRA",
        "IBZ",
        "11:00",
        "13:25",
    );
    row(
        &mut items,
        "2026-06-13",
        "FB1467",
        "IBZ",
        "FRA",
        "14:25",
        "16:50",
    );
    row(
        &mut items,
        "2026-06-13",
        "FB892",
        "FRA",
        "RIX",
        "18:00",
        "20:10",
    );
    row(&mut items, "2026-06-14", "Layover", "RIX", "", "", "");
    row(
        &mut items,
        "2026-06-15",
        "FB893",
        "RIX",
        "FRA",
        "04:00",
        "06:20",
    );
    row(
        &mut items,
        "2026-06-15",
        "FB1186",
        "FRA",
        "ZRH",
        "07:30",
        "08:25",
    );
    row(
        &mut items,
        "2026-06-15",
        "FB1187",
        "ZRH",
        "FRA",
        "09:05",
        "10:10",
    );
    row(&mut items, "2026-06-18", "EM", "", "", "06:00", "10:00");
    row(
        &mut items,
        "2026-06-20",
        "FB732",
        "FRA",
        "PVG",
        "19:30",
        "07:05",
    );
    row(&mut items, "2026-06-22", "Layover", "PVG", "", "", "");
    row(
        &mut items,
        "2026-06-23",
        "FB733",
        "PVG",
        "FRA",
        "15:50",
        "05:30",
    );
    for day in 25..=29 {
        row(
            &mut items,
            &format!("2026-06-{day}"),
            "FREE",
            "",
            "",
            "",
            "",
        );
    }
    row(&mut items, "2026-06-30", "OFF", "", "", "", "");

    let dir = fresh_dir();
    let source = dir.join("roster.pdf");
    std::fs::write(&source, support::build_pdf(&items)).expect("write the reference plan");
    w.dir = Some(dir);
    w.source = Some(source);
    // The oracle states DTSTAMP 20260610T000000Z; the construction clock is the
    // fallback this plan has no revision footer for.
    w.env = vec![(
        "DUTY2CAL_CONSTRUCTION_CLOCK".to_string(),
        "2026-06-10T00:00:00Z".to_string(),
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
        .join("tests/fixtures/expected/2026-06.ics");
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
