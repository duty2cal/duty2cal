// SPDX-License-Identifier: MPL-2.0
//! End-to-end steps for the capability `measure-distributed-executable-size`.
//!
//! One file per capability, separate from the regular steps because it commits
//! the terminal transcript the visual contract requires. The shared Given/When
//! steps live in the regular file; these are the e2e-specific assertions.

use std::fs;
use std::path::PathBuf;

use cucumber::then;

use crate::steps::measure_distributed_executable_size::{
    build_fixture_for_target, field, fixture_repo, is_hex, number_after, report_text,
    scenario_dir, section,
};
use crate::DutyWorld;

/// The transcript the visual contract requires, committed with the change
/// while it is active and with the archive afterwards.
fn transcript_path() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let active = root.join("givn/changes/measure-executable-size");
    let home = if active.is_dir() {
        active
    } else {
        root.join("givn/archive/measure-executable-size")
    };
    home.join(
        "evidence/visual/\
         maintainer-requests-an-attribution-for-a-revision-and-reads-the-report/transcript.txt",
    )
}

fn commit_transcript(w: &DutyWorld) {
    let path = transcript_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create the evidence directory");
    }
    let mut evidence = w
        .scenario_transcript
        .replace(
            &w.working_dir().display().to_string(),
            "<scenario-dir>",
        );
    if let Some(out) = &w.measurement_out {
        let report = out.join("attribution.md");
        if let Ok(text) = fs::read_to_string(&report) {
            evidence.push_str("\n--- attribution.md ---\n");
            evidence.push_str(&text);
        }
    }
    evidence.push_str(&format!(
        "exit status: {}\n",
        w.exit_status.unwrap_or(-1)
    ));
    fs::write(&path, evidence).expect("write the transcript");
}

#[then(regex = r#"^the attribution names the platform "([^"]+)"$"#)]
async fn names_platform(w: &mut DutyWorld, platform: String) {
    let text = report_text(w);
    assert_eq!(field(&text, "- Platform: "), platform);
}

#[then(regex = r#"^the build identity records the rustc version, "([^"]+)", and a digest of "([^"]+)" and "([^"]+)"$"#)]
async fn records_build_identity(
    w: &mut DutyWorld,
    generator: String,
    manifest: String,
    configuration: String,
) {
    let text = report_text(w);
    assert!(
        text.contains("- Toolchain: rustc "),
        "the report carries no rustc line:\n{text}"
    );
    assert!(
        text.contains(&format!("- Generator: {generator}")),
        "the report does not name {generator:?}:\n{text}"
    );
    for name in [manifest, configuration] {
        let value = field(&text, &format!("- {name}: "));
        assert!(
            is_hex(value, 64),
            "the report does not carry a 64-hex digest for {name:?}: {value:?}"
        );
    }
}

#[then(regex = r#"^the attribution divides the size by crate, naming "([^"]+)"$"#)]
async fn divides_by_crate(w: &mut DutyWorld, crate_name: String) {
    let text = report_text(w);
    let table = section(&text, "## By crate");
    assert!(
        table.contains(&crate_name),
        "the crate table does not name {crate_name:?}:\n{text}"
    );
}

#[then(regex = r#"^the attribution divides the size by function, naming "([^"]+)"$"#)]
async fn divides_by_function(w: &mut DutyWorld, function: String) {
    let text = report_text(w);
    let table = section(&text, "## By function");
    assert!(
        table.contains(&function),
        "the function table does not name {function:?}:\n{text}"
    );
}

#[then("the stripped executable's file size equals the fixture executable's byte count")]
async fn file_size_equals_fixture(w: &mut DutyWorld) {
    let text = report_text(w);
    let reported = number_after(&text, "- Stripped executable (file): ");
    let repo = fixture_repo(w);
    let target = scenario_dir(w).join("verify-target");
    let executable = build_fixture_for_target(&repo, &target);
    let actual = fs::metadata(&executable)
        .unwrap_or_else(|error| panic!("read {}: {error}", executable.display()))
        .len();
    assert_eq!(
        reported, actual,
        "the report states {reported} B but the fixture executable is {actual} B"
    );
}

#[then("the attribution's total is less than the executable's file size")]
async fn total_is_less_than_file_size(w: &mut DutyWorld) {
    let text = report_text(w);
    let total = number_after(&text, "- Size attribution (attribution build .text): ");
    let file = number_after(&text, "- Stripped executable (file): ");
    assert!(
        total < file,
        "the attribution total {total} is not less than the file size {file}"
    );
}

#[then("the attribution build's file size is greater than the executable's file size")]
async fn build_is_larger_than_stripped(w: &mut DutyWorld) {
    let text = report_text(w);
    let build = number_after(&text, "- Attribution build (file): ");
    let file = number_after(&text, "- Stripped executable (file): ");
    assert!(
        build > file,
        "the attribution build {build} is not larger than the stripped executable {file}"
    );
    commit_transcript(w);
}
