// SPDX-License-Identifier: MPL-2.0
//! Golden conversion: the generated `2026-06` plan must produce the committed
//! reference calendar. This is the fast, cucumber-free proof that the product
//! still converts the reviewed plan (see design.md "Test runner, strict mode,
//! and single-scenario command").

#[path = "steps/support.rs"]
#[allow(dead_code)]
mod support;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static SEQUENCE: AtomicUsize = AtomicUsize::new(0);

/// CRLF to LF and folded lines joined back, the agreed normalisation.
fn normalise(calendar: &str) -> String {
    calendar.replace("\r\n ", "").replace("\r\n", "\n")
}

fn scenario_dir() -> PathBuf {
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("duty2cal-golden-{}-{sequence}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the golden-test directory");
    dir
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/expected")
        .join(name)
}

#[test]
fn reference_roster_matches_the_expected_calendar() {
    let dir = scenario_dir();
    let source = dir.join("roster.pdf");
    std::fs::write(&source, support::build_pdf(&support::equivalence_plan()))
        .expect("write the reference plan");

    let output = Command::new(env!("CARGO_BIN_EXE_duty2cal"))
        .arg(&source)
        .env("DUTY2CAL_CONSTRUCTION_CLOCK", "2099-06-10T00:00:00Z")
        .output()
        .expect("run the converter");
    assert!(
        output.status.success(),
        "the converter failed: status {:?}\nstdout: {}\nstderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let published = dir.join("roster.ics");
    let published = std::fs::read_to_string(&published)
        .unwrap_or_else(|error| panic!("read {}: {error}", published.display()));
    let expected = std::fs::read_to_string(fixture("2026-06.ics"))
        .unwrap_or_else(|error| panic!("read the expected calendar: {error}"));

    assert_eq!(
        normalise(&published),
        normalise(&expected),
        "the published calendar differs from the committed reference"
    );
}
