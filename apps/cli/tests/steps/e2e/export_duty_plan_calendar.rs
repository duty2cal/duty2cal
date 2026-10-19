// SPDX-License-Identifier: MPL-2.0
//! End-to-end steps for the capability `export-duty-plan-calendar`.
//!
//! One file per capability. The scenario's `When` is shared with the in-process
//! steps; these steps only add what the real interface needs.

use std::path::PathBuf;

use cucumber::{given, then, when};

use crate::steps::export_duty_plan_calendar::{calendar, plan_with_row, transcript};
use crate::steps::support::{ACTIVITY_X, DATE_X, END_X, FROM_X, START_X, TO_X};
use crate::DutyWorld;

/// Where the scenario's terminal transcript is committed as evidence.
fn transcript_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../givn/archive/export-released-roster-calendar/evidence/visual/\
         crew-member-converts-a-supported-duty-plan/transcript.txt",
    )
}

#[given("a supported Released Roster duty plan supplied for conversion")]
async fn a_supported_plan(w: &mut DutyWorld) {
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
    // A fixed construction clock keeps the committed transcript reproducible.
    w.env = vec![(
        "DUTY2CAL_CONSTRUCTION_CLOCK".to_string(),
        "2099-06-10T00:00:00Z".to_string(),
    )];
}

#[then("a complete calendar file is published beside the source under an unused name")]
async fn a_complete_calendar_is_published(w: &mut DutyWorld) {
    let calendar = calendar(w);
    assert!(
        calendar.contains("BEGIN:VCALENDAR")
            && calendar.contains("BEGIN:VEVENT")
            && calendar.contains("SUMMARY:LU742 ABC-DEF")
            && calendar.contains("DTSTART:20990607T090000Z"),
        "the published calendar is not complete: {calendar}"
    );
}

#[then("the converter reports the path of the newly published calendar")]
async fn the_converter_reports_the_path(w: &mut DutyWorld) {
    let text = transcript(w);
    let expected = w.working_dir().join("roster.ics");
    assert!(
        text.contains("Written: ") && text.contains(&expected.display().to_string()),
        "the converter did not report the new calendar path: {text}"
    );
}

#[then("the invocation reports success through its process exit status")]
async fn success_exit_status(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the invocation did not report success: {}",
        transcript(w)
    );
    // Commit the terminal transcript as this scenario's evidence.
    let path = transcript_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create the evidence directory");
    }
    // The transcript is committed evidence, so the run-specific directory is
    // replaced by a placeholder.
    let mut evidence =
        transcript(w).replace(&w.working_dir().display().to_string(), "<scenario-dir>");
    evidence.push_str(&format!("exit status: {}\n", w.exit_status.unwrap_or(-1)));
    std::fs::write(&path, evidence).expect("write the transcript");
}

// ---------------------------------------------------------------------------
// Browser `@e2e` steps (design.md "Step definition locations"). The shared
// steps live in the regular capability file; these are the observations only
// the end-to-end run makes. Every body is the RED stub.
// ---------------------------------------------------------------------------

#[when("the crew member saves the offered calendar")]
async fn the_crew_member_saves_the_offered_calendar(_w: &mut DutyWorld) {
    unimplemented!("the crew member saves the offered calendar")
}

#[then("the browser has saved a complete RFC 5545 calendar")]
async fn the_browser_saved_a_complete_calendar(_w: &mut DutyWorld) {
    unimplemented!("the browser has saved a complete RFC 5545 calendar")
}

#[then(regex = r#"^the saved calendar matches the reference calendar for "([^"]+)"$"#)]
async fn the_saved_calendar_matches_the_reference(_w: &mut DutyWorld, _name: String) {
    unimplemented!("the saved calendar matches the reference calendar")
}

#[then("the page asks for no account, email, or upload")]
async fn the_page_asks_for_no_account(_w: &mut DutyWorld) {
    unimplemented!("the page asks for no account, email, or upload")
}

#[then("the page meets the visual baseline at every displayed state")]
async fn the_page_meets_the_visual_baseline(_w: &mut DutyWorld) {
    unimplemented!("the page meets the visual baseline at every displayed state")
}
