// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `own-the-duty-plan-extraction`.
//!
//! One file per capability. A step body is never empty: every step starts as an
//! unimplemented stub that panics with its own text.

use cucumber::{given, then, when};

use crate::DutyWorld;

#[given("the supported duty plan encoded with a classic xref table")]
async fn the_supported_plan_with_a_classic_xref_table(_w: &mut DutyWorld) {
    unimplemented!("the supported duty plan encoded with a classic xref table")
}

#[given("the same plan encoded with an xref stream and object streams")]
async fn the_same_plan_with_an_xref_stream_and_object_streams(_w: &mut DutyWorld) {
    unimplemented!("the same plan encoded with an xref stream and object streams")
}

#[when("the release maintainer converts both plans through the executable")]
async fn the_release_maintainer_converts_both_plans(_w: &mut DutyWorld) {
    unimplemented!("the release maintainer converts both plans through the executable")
}

#[then("both published calendars are byte-identical")]
async fn both_published_calendars_are_byte_identical(_w: &mut DutyWorld) {
    unimplemented!("both published calendars are byte-identical")
}

#[then("the executable reports success for both conversions")]
async fn the_executable_reports_success_for_both_conversions(_w: &mut DutyWorld) {
    unimplemented!("the executable reports success for both conversions")
}

#[given(regex = r#"^a duty plan that exercises the construct "(.*)"$"#)]
async fn a_duty_plan_that_exercises_the_construct(_w: &mut DutyWorld, _construct: String) {
    unimplemented!("a duty plan that exercises the construct \"<construct>\"")
}

#[when("the release maintainer converts it through the executable")]
async fn the_release_maintainer_converts_it_through_the_executable(_w: &mut DutyWorld) {
    unimplemented!("the release maintainer converts it through the executable")
}

#[then(regex = r#"^the numbered failure report names the source PDF, the construct "(.*)" and the page it occurs on$"#)]
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

#[then("the numbered failure report names the source PDF and that the source is not a readable PDF")]
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
