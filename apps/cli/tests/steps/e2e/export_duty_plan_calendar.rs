// SPDX-License-Identifier: MPL-2.0
//! End-to-end steps for the capability `export-duty-plan-calendar`.
//!
//! One file per capability. The scenario's `When` is shared with the in-process
//! steps; these steps only add what the real interface needs.

use std::path::PathBuf;

use cucumber::{given, then, when};
use serde_json::{Map, Value};

use crate::steps::e2e::visual_baseline::assert_visual_baseline;
use crate::steps::export_duty_plan_calendar::{calendar, driver, plan_with_row, transcript};
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
// Given/When/Then steps live in the regular capability file; these are the
// observations only the end-to-end run makes.
// ---------------------------------------------------------------------------

/// The committed per-viewport visual evidence directory for this scenario.
fn evidence_dir() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    // The override names a change id. The active/archived resolution below still
    // decides which tree the gate is testing, so an archived change's evidence
    // never lands outside the candidate.
    let change = std::env::var("DUTY2CAL_VISUAL_EVIDENCE_DIR")
        .unwrap_or_else(|_| "build-browser-converter".to_string());
    let active = root.join("givn/changes").join(&change);
    let archived = root.join("givn/archive").join(&change);
    // The archive gate moves the change before it runs the suite. Write the
    // evidence to the archived copy in that phase so the tested tree stays
    // inside the candidate; the active change uses its own directory.
    let base = if active.join("proposal.md").is_file() {
        active
    } else {
        archived
    };
    base.join(
        "evidence/visual/\
         crew-member-converts-a-supported-duty-plan-in-the-browser-and-saves-the-offered-calendar",
    )
}

/// Read the page with the visual-baseline observations included.
fn baseline_state(w: &mut DutyWorld) -> Value {
    let mut params = Map::new();
    params.insert("baseline".to_string(), Value::Bool(true));
    driver(w).send("read_state", params)
}

/// Run axe on the current rendered state and fail on any violation.
fn assert_axe_clean(w: &mut DutyWorld, displayed: &str) {
    let result = driver(w).send("axe", Map::new());
    let violations = result["violations"]
        .as_array()
        .expect("axe returns a violations array");
    assert!(
        violations.is_empty(),
        "the {displayed} state has accessibility violations: {violations:?}"
    );
}

/// Capture the committed screenshots for one displayed state at both viewports.
fn capture_screenshots(w: &mut DutyWorld, empty: bool) {
    // The archived screenshots are the frozen record of the reviewed run.
    // A normal run captures into the scenario directory so rendering
    // differences between machines cannot rewrite that record; updating it
    // is an explicit act.
    let dir = if std::env::var_os("DUTY2CAL_UPDATE_VISUAL_EVIDENCE").is_some() {
        evidence_dir()
    } else {
        w.working_dir().join("visual-capture")
    };
    std::fs::create_dir_all(&dir).expect("create the visual evidence directory");
    for (width, height, viewport) in [(360_u32, 780_u32, "mobile"), (1920, 1080, "desktop")] {
        let path = dir.join(if empty {
            format!("{viewport}--empty.png")
        } else {
            format!("{viewport}.png")
        });
        let mut params = Map::new();
        params.insert(
            "path".to_string(),
            Value::String(path.display().to_string()),
        );
        params.insert("width".to_string(), Value::from(width));
        params.insert("height".to_string(), Value::from(height));
        let result = driver(w).send("screenshot", params);
        let saved_to = path.display().to_string();
        assert_eq!(
            result["savedTo"].as_str(),
            Some(saved_to.as_str()),
            "the driver did not capture {saved_to}: {result}"
        );
        assert!(path.is_file(), "the screenshot {saved_to} was not written");
    }
}

#[when("the crew member saves the offered calendar")]
async fn the_crew_member_saves_the_offered_calendar(w: &mut DutyWorld) {
    let path = w.working_dir().join("saved-2026-06.ics");
    let mut params = Map::new();
    params.insert("kind".to_string(), Value::String("calendar".to_string()));
    params.insert(
        "path".to_string(),
        Value::String(path.display().to_string()),
    );
    let result = driver(w).send("save_offer", params);
    assert_eq!(
        result["suggestedFilename"].as_str(),
        Some("2026-06.ics"),
        "the browser's download does not carry the offered name: {result}"
    );
    let saved = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read the browser's download {}: {error}", path.display()));
    w.calendars = vec![saved];
}

#[then("the browser has saved a complete RFC 5545 calendar")]
async fn the_browser_saved_a_complete_calendar(w: &mut DutyWorld) {
    let saved = w
        .calendars
        .last()
        .expect("the save step captured no calendar");
    assert!(
        saved.starts_with("BEGIN:VCALENDAR\r\n"),
        "the saved download does not open a VCALENDAR: {saved:?}"
    );
    assert!(
        saved.trim_end_matches("\r\n").ends_with("END:VCALENDAR"),
        "the saved download does not close its VCALENDAR: {saved:?}"
    );
    for required in ["VERSION:2.0", "PRODID:"] {
        assert!(
            saved.contains(required),
            "the saved download has no {required}: {saved:?}"
        );
    }
    let starts = saved.matches("BEGIN:VEVENT").count();
    let ends = saved.matches("END:VEVENT").count();
    assert!(
        starts > 0 && starts == ends,
        "the saved download has {starts} VEVENT starts and {ends} ends: {saved:?}"
    );
}

/// The `VEVENT` blocks of a calendar, in order.
fn event_blocks(calendar: &str) -> Vec<&str> {
    let mut blocks = Vec::new();
    let mut rest = calendar;
    while let Some(start) = rest.find("BEGIN:VEVENT") {
        let tail = &rest[start..];
        let end = tail
            .find("END:VEVENT")
            .unwrap_or_else(|| panic!("unterminated VEVENT in {calendar:?}"));
        blocks.push(&tail[..end + "END:VEVENT".len()]);
        rest = &tail[end + "END:VEVENT".len()..];
    }
    blocks
}

#[then(regex = r#"^the saved calendar matches the reference calendar for "([^"]+)"$"#)]
async fn the_saved_calendar_matches_the_reference(w: &mut DutyWorld, name: String) {
    let saved = w
        .calendars
        .last()
        .expect("the save step captured no calendar")
        .clone();
    let bytes = saved.as_bytes();
    for (index, byte) in bytes.iter().enumerate() {
        match byte {
            b'\n' => assert!(
                index > 0 && bytes[index - 1] == b'\r',
                "the saved calendar has a bare LF at byte {index}: {saved:?}"
            ),
            b'\r' => assert!(
                index + 1 < bytes.len() && bytes[index + 1] == b'\n',
                "the saved calendar has a bare CR at byte {index}: {saved:?}"
            ),
            _ => {}
        }
    }

    let stem = name.strip_suffix(".pdf").unwrap_or(&name);
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/expected")
        .join(format!("{stem}.ics"));
    let expected = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|error| panic!("read {}: {error}", fixture_path.display()));
    let expected_events = event_blocks(&expected);
    let saved_events = event_blocks(&saved);
    assert!(
        !expected_events.is_empty(),
        "the reference calendar has no VEVENT: {expected:?}"
    );
    assert_eq!(
        saved_events.len(),
        expected_events.len(),
        "the saved calendar has {} events; the reference has {}: {saved:?}",
        saved_events.len(),
        expected_events.len()
    );
    for (index, (saved_event, expected_event)) in
        saved_events.iter().zip(expected_events.iter()).enumerate()
    {
        assert_eq!(
            saved_event, expected_event,
            "VEVENT {index} differs from the reference; saved {saved_event:?}, reference {expected_event:?}"
        );
    }
}

#[then("the page asks for no account, email, or upload")]
async fn the_page_asks_for_no_account(w: &mut DutyWorld) {
    let state = baseline_state(w);
    let baseline = state["baseline"]
        .as_object()
        .unwrap_or_else(|| panic!("the page reports no baseline: {state}"));
    for field in [
        "forms",
        "emailInputs",
        "submitControls",
        "accountHints",
        "fileInputsInForms",
    ] {
        assert_eq!(
            baseline[field].as_u64(),
            Some(0),
            "the page offers {field}: {state}"
        );
    }
    let text = state["visibleText"]
        .as_str()
        .expect("read_state returns visible text");
    assert!(
        text.contains("No account, email or upload is needed."),
        "the page does not state that it asks for no account, email, or upload: {text}"
    );
    let lower = text.to_lowercase();
    for request in [
        "sign in",
        "log in",
        "log on",
        "create an account",
        "enter your email",
    ] {
        assert!(
            !lower.contains(request),
            "the page asks the crew member to {request}: {text}"
        );
    }
}

#[then("the page meets the visual baseline at every displayed state")]
async fn the_page_meets_the_visual_baseline(w: &mut DutyWorld) {
    // The ready state is the one the scenario just rendered.
    let ready = baseline_state(w);
    assert_eq!(
        ready["state"].as_str(),
        Some("ready"),
        "the visual baseline ran outside the ready state: {ready}"
    );
    assert_visual_baseline(&ready, "ready", "en");
    assert_axe_clean(w, "ready");
    capture_screenshots(w, false);

    // Return to the empty state the scenario displayed first and assert it too.
    driver(w).send("reload", Map::new());
    let empty = baseline_state(w);
    assert_eq!(
        empty["state"].as_str(),
        Some("empty"),
        "the page did not return to the empty state for its capture: {empty}"
    );
    assert_visual_baseline(&empty, "empty", "en");
    assert_axe_clean(w, "empty");
    capture_screenshots(w, true);
}
