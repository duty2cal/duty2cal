// SPDX-License-Identifier: MPL-2.0
//! `@e2e` step definitions for the capability `own-the-duty-plan-extraction`.
//!
//! These steps drive the built executable through the terminal, compare the
//! published calendar with the committed accepted calendar, and write the
//! scenario's transcript.

use std::path::PathBuf;

use cucumber::{given, then};

use super::export_duty_plan_calendar::scenario_slug;
use crate::steps::export_duty_plan_calendar::fresh_dir;
use crate::steps::support;
use crate::DutyWorld;

#[given("a supported duty plan whose accepted calendar is recorded")]
async fn a_supported_plan_whose_accepted_calendar_is_recorded(w: &mut DutyWorld) {
    let dir = fresh_dir();
    let source = dir.join("roster.pdf");
    std::fs::write(&source, support::build_pdf(&support::equivalence_plan()))
        .expect("write the generated supported plan");
    w.dir = Some(dir);
    w.source = Some(source);
    w.accepted_calendar = Some(support::accepted_calendar());
}

#[then("the executable publishes the complete calendar at an unused path")]
async fn the_executable_publishes_the_complete_calendar(w: &mut DutyWorld) {
    let path = w.working_dir().join("roster.ics");
    let calendar = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "no calendar at {}: {error}; stdout={}",
            path.display(),
            w.stdout
        )
    });
    assert!(
        calendar.contains("BEGIN:VCALENDAR") && calendar.contains("END:VCALENDAR"),
        "the published calendar is not a complete VCALENDAR: {calendar}"
    );
    assert!(
        w.stdout.contains("Written:"),
        "the executable did not report the published path: {}",
        w.stdout
    );
}

#[then(
    "the published calendar is identical to the accepted calendar after the agreed normalisation"
)]
async fn the_published_calendar_matches_the_accepted_calendar(w: &mut DutyWorld) {
    let published = std::fs::read_to_string(w.working_dir().join("roster.ics"))
        .expect("read the published calendar");
    let accepted = w
        .accepted_calendar
        .as_deref()
        .expect("the accepted calendar was recorded");
    w.calendar_matched = support::normalise(&published) == support::normalise(accepted);
    assert!(
        w.calendar_matched,
        "the published calendar differs from the accepted one"
    );
}

#[then("the executable reports success through its process exit status")]
async fn the_executable_reports_success_through_its_exit_status(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the conversion did not succeed: stdout={} stderr={}",
        w.stdout,
        w.stderr
    );
    write_transcript(w);
}

/// Write the committed `@e2e` transcript with the scenario directory
/// substituted; no host path, user name or clock value survives.
fn write_transcript(w: &DutyWorld) {
    let scenario_dir = w.working_dir().display().to_string();
    let written = w
        .stdout
        .lines()
        .find(|line| line.starts_with("Written:"))
        .unwrap_or("Written: <none>")
        .replace(&scenario_dir, "<scenario-dir>");
    let exit = w
        .exit_status
        .map(|status| status.to_string())
        .unwrap_or_else(|| "none".to_string());
    let comparison = if w.calendar_matched {
        "identical"
    } else {
        "different"
    };
    let transcript = format!(
        "$ duty2cal <scenario-dir>/roster.pdf\n{written}\nexit status: {exit}\nnormalised comparison: {comparison}\n"
    );
    let override_dir = std::env::var_os("DUTY2CAL_EVIDENCE_DIR");
    let base = match &override_dir {
        Some(dir) => PathBuf::from(dir),
        None => std::env::temp_dir().join("duty2cal-evidence"),
    };
    let evidence = base.join(scenario_slug(&w.scenario)).join("transcript.txt");
    std::fs::create_dir_all(evidence.parent().expect("the evidence directory"))
        .expect("create the evidence directory");
    std::fs::write(&evidence, transcript).expect("write the e2e transcript");
    if override_dir.is_none() {
        eprintln!("e2e transcript: {}", evidence.display());
    }
}
