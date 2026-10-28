// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `own-the-duty-plan-extraction`.
//!
//! One file per capability. A step body is never empty: every step starts as an
//! unimplemented stub that panics with its own text.

use std::path::PathBuf;
use std::process::Command;

use cucumber::{given, then, when};
use duty2cal_pdf_fixtures as fixtures;

use crate::steps::export_duty_plan_calendar::fresh_dir;
use crate::DutyWorld;

/// The second generated plan a scenario converts, beside the first.
const SECOND_PLAN: &str = "object-streams.pdf";

/// Write one generated plan into the scenario directory.
fn write_plan(w: &mut DutyWorld, name: &str, bytes: Vec<u8>) -> PathBuf {
    let dir = w.dir.clone().unwrap_or_else(fresh_dir);
    std::fs::create_dir_all(&dir).expect("create the scenario directory");
    let path = dir.join(name);
    std::fs::write(&path, bytes).expect("write the generated plan");
    w.dir = Some(dir);
    path
}

/// Run the executable on one source and record its output and exit status.
fn convert(w: &mut DutyWorld, source: &PathBuf) {
    let output = Command::new(env!("CARGO_BIN_EXE_duty2cal"))
        .arg(source)
        .output()
        .expect("run the converter");
    w.exit_status = output.status.code();
    if let Some(status) = w.exit_status {
        w.exit_statuses.push(status);
    }
    w.stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    w.stderr = String::from_utf8_lossy(&output.stderr).into_owned();
}

#[given("the supported duty plan encoded with a classic xref table")]
async fn the_supported_plan_with_a_classic_xref_table(w: &mut DutyWorld) {
    let bytes = fixtures::build_plan(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::ClassicXref,
    );
    w.source = Some(write_plan(w, "classic.pdf", bytes));
}

#[given("the same plan encoded with an xref stream and object streams")]
async fn the_same_plan_with_an_xref_stream_and_object_streams(w: &mut DutyWorld) {
    let bytes = fixtures::build_plan(
        &fixtures::equivalence_plan(),
        fixtures::Encoding::ObjectStreams,
    );
    write_plan(w, SECOND_PLAN, bytes);
}

#[when("the release maintainer converts both plans through the executable")]
async fn the_release_maintainer_converts_both_plans(w: &mut DutyWorld) {
    let classic = w.source_path();
    convert(w, &classic);
    let second = w.working_dir().join(SECOND_PLAN);
    convert(w, &second);
}

#[then("both published calendars are identical apart from their source-snapshot identities")]
async fn both_published_calendars_match_apart_from_uids(w: &mut DutyWorld) {
    // The two encodings are distinct source snapshots, and a calendar's UID
    // carries the snapshot identity (FNV-1a of the source bytes). Everything a
    // crew member reads besides that identity must be identical.
    let first = std::fs::read_to_string(w.working_dir().join("classic.ics"))
        .expect("the classic-xref conversion published no calendar");
    let second = std::fs::read_to_string(w.working_dir().join("object-streams.ics"))
        .expect("the object-stream conversion published no calendar");
    let content = |calendar: &str| -> Vec<String> {
        calendar
            .lines()
            .filter(|line| !line.starts_with("UID:"))
            .map(str::to_string)
            .collect()
    };
    assert_eq!(
        content(&first),
        content(&second),
        "the two encodings published different calendar content"
    );
}

#[then("the executable reports success for both conversions")]
async fn the_executable_reports_success_for_both_conversions(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_statuses,
        vec![0, 0],
        "a conversion reported failure: stdout={} stderr={}",
        w.stdout,
        w.stderr
    );
}

#[given(regex = r#"^a duty plan that exercises the construct "(.*)"$"#)]
async fn a_duty_plan_that_exercises_the_construct(_w: &mut DutyWorld, _construct: String) {
    unimplemented!("a duty plan that exercises the construct \"<construct>\"")
}

#[when("the release maintainer converts it through the executable")]
async fn the_release_maintainer_converts_it_through_the_executable(_w: &mut DutyWorld) {
    unimplemented!("the release maintainer converts it through the executable")
}

#[then(
    regex = r#"^the numbered failure report names the source PDF, the construct "(.*)" and the page it occurs on$"#
)]
async fn the_numbered_failure_report_names_the_construct_and_page(
    _w: &mut DutyWorld,
    _construct: String,
) {
    unimplemented!(
        "the numbered failure report names the source PDF, the construct \"<construct>\" and the page it occurs on"
    )
}

#[then("the executable reports failure through a non-zero process exit status")]
async fn the_executable_reports_failure_through_a_non_zero_exit(_w: &mut DutyWorld) {
    unimplemented!("the executable reports failure through a non-zero process exit status")
}

#[given("a duty plan that is encrypted")]
async fn a_duty_plan_that_is_encrypted(_w: &mut DutyWorld) {
    unimplemented!("a duty plan that is encrypted")
}

#[then("the numbered failure report names the source PDF and the encrypted document")]
async fn the_numbered_failure_report_names_the_encrypted_document(_w: &mut DutyWorld) {
    unimplemented!("the numbered failure report names the source PDF and the encrypted document")
}

#[given("a readable PDF that prints no detail-table header")]
async fn a_readable_pdf_without_a_detail_table_header(_w: &mut DutyWorld) {
    unimplemented!("a readable PDF that prints no detail-table header")
}

#[then("the numbered failure report names the source PDF and the unsupported family")]
async fn the_numbered_failure_report_names_the_unsupported_family(_w: &mut DutyWorld) {
    unimplemented!("the numbered failure report names the source PDF and the unsupported family")
}

#[given("a truncated PDF")]
async fn a_truncated_pdf(_w: &mut DutyWorld) {
    unimplemented!("a truncated PDF")
}

#[then(
    "the numbered failure report names the source PDF and that the source is not a readable PDF"
)]
async fn the_numbered_failure_report_names_the_unreadable_source(_w: &mut DutyWorld) {
    unimplemented!(
        "the numbered failure report names the source PDF and that the source is not a readable PDF"
    )
}

#[given("a duty plan that uses a Type3 font and prints an identifying name")]
async fn a_duty_plan_that_uses_a_type3_font(_w: &mut DutyWorld) {
    unimplemented!("a duty plan that uses a Type3 font and prints an identifying name")
}

#[then("the numbered failure report names the source PDF and the Type3 font")]
async fn the_numbered_failure_report_names_the_type3_font(_w: &mut DutyWorld) {
    unimplemented!("the numbered failure report names the source PDF and the Type3 font")
}

#[then("the failure report contains no printed text from the plan")]
async fn the_failure_report_contains_no_printed_text(_w: &mut DutyWorld) {
    unimplemented!("the failure report contains no printed text from the plan")
}

#[given("a supported duty plan whose detail table continues on a second page with a repeated header and a TK row on each page")]
async fn a_supported_plan_whose_table_continues_on_a_second_page(_w: &mut DutyWorld) {
    unimplemented!(
        "a supported duty plan whose detail table continues on a second page with a repeated header and a TK row on each page"
    )
}

#[then("the published calendar contains exactly two TK events, one dated 12 June 2026 and one dated 13 June 2026")]
async fn the_published_calendar_contains_exactly_two_tk_events(_w: &mut DutyWorld) {
    unimplemented!(
        "the published calendar contains exactly two TK events, one dated 12 June 2026 and one dated 13 June 2026"
    )
}

#[then("no detail row is represented twice")]
async fn no_detail_row_is_represented_twice(_w: &mut DutyWorld) {
    unimplemented!("no detail row is represented twice")
}

#[given("a supported duty plan whose one row uses an unrecognized activity code with both printed endpoints")]
async fn a_supported_plan_with_an_unrecognized_activity_code(_w: &mut DutyWorld) {
    unimplemented!(
        "a supported duty plan whose one row uses an unrecognized activity code with both printed endpoints"
    )
}

#[then("the warning report names that detail row and the fallback applied")]
async fn the_warning_report_names_the_row_and_the_fallback(_w: &mut DutyWorld) {
    unimplemented!("the warning report names that detail row and the fallback applied")
}
