// SPDX-License-Identifier: MPL-2.0
//! `@e2e` step definitions for the capability `own-the-duty-plan-extraction`.
//!
//! These steps drive the built executable through the terminal. A step body is
//! never empty: every step starts as an unimplemented stub that panics with its
//! own text.

use cucumber::{given, then};

use crate::DutyWorld;

#[given("a supported duty plan whose accepted calendar is recorded")]
async fn a_supported_plan_whose_accepted_calendar_is_recorded(_w: &mut DutyWorld) {
    unimplemented!("a supported duty plan whose accepted calendar is recorded")
}

#[then("the executable publishes the complete calendar at an unused path")]
async fn the_executable_publishes_the_complete_calendar(_w: &mut DutyWorld) {
    unimplemented!("the executable publishes the complete calendar at an unused path")
}

#[then(
    "the published calendar is identical to the accepted calendar after the agreed normalisation"
)]
async fn the_published_calendar_matches_the_accepted_calendar(_w: &mut DutyWorld) {
    unimplemented!(
        "the published calendar is identical to the accepted calendar after the agreed normalisation"
    )
}

#[then("the executable reports success through its process exit status")]
async fn the_executable_reports_success_through_its_exit_status(_w: &mut DutyWorld) {
    unimplemented!("the executable reports success through its process exit status")
}
