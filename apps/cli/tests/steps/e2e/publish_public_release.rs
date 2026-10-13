// SPDX-License-Identifier: MPL-2.0
//! End-to-end steps for the capability `publish-public-release`.
//!
//! One file per capability. The scenario's `Given`/`When`/`Then` that the
//! in-process steps already define stay there; these steps add what only the
//! real interface needs: the executable built for this machine, the archive
//! extracted with `tar`, and the executable taken out of it.

use std::path::PathBuf;
use std::process::Command;

use cucumber::{given, then, when};

use crate::steps::publish_public_release::{
    build_binaries, build_history, change_log_path, machine_executable, machine_platform,
    run_command, sha256_of, staged_archive, staged_dir, supported_platforms, FIRST_RELEASE_HISTORY,
};
use crate::DutyWorld;

/// Where the scenario's terminal transcript is committed as evidence.
fn transcript_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../givn/archive/publish-public-release/evidence/visual/\
         maintainer-prepares-a-release-and-runs-the-archived-executable/transcript.txt",
    )
}

/// Where the render scenario's terminal transcript is committed as evidence.
fn render_transcript_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../givn/changes/generate-release-notes/evidence/visual/\
         maintainer-generates-the-change-log-for-a-version-and-reads-its-newest-section/transcript.txt",
    )
}

/// The directory the archive is extracted into.
fn extracted_dir(w: &DutyWorld) -> PathBuf {
    w.working_dir().join("extracted")
}

/// Commit the scenario's terminal transcript as evidence, with the run-specific
/// directory replaced by a placeholder.
fn commit_transcript(w: &DutyWorld) {
    let path = transcript_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create the evidence directory");
    }
    let mut evidence = w
        .scenario_transcript
        .replace(&w.working_dir().display().to_string(), "<scenario-dir>");
    evidence.push_str(&format!("exit status: {}\n", w.exit_status.unwrap_or(-1)));
    std::fs::write(&path, evidence).expect("write the transcript");
}

/// Commit the render scenario's terminal transcript as evidence, with the
/// run-specific directory replaced by a placeholder, as the design's visual
/// contract requires.
fn commit_render_transcript(w: &DutyWorld) {
    let path = render_transcript_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create the evidence directory");
    }
    let mut evidence = w
        .scenario_transcript
        .replace(&w.working_dir().display().to_string(), "<scenario-dir>");
    evidence.push_str(&format!("exit status: {}\n", w.exit_status.unwrap_or(-1)));
    std::fs::write(&path, evidence).expect("write the transcript");
}

/// The change log the render scenario wrote.
fn rendered_change_log(w: &DutyWorld) -> PathBuf {
    w.changelog.clone().unwrap_or_else(|| change_log_path(w))
}

#[given("the executable built for this machine")]
async fn the_executable_built_for_this_machine(w: &mut DutyWorld) {
    // The real built binary for this machine's platform, stand-ins for the four
    // the preparation requires and this machine cannot run.
    build_binaries(w, &supported_platforms());
}

#[then("the preparation reports the archive and its checksum for this machine's platform")]
async fn the_preparation_reports_archive_and_checksum(w: &mut DutyWorld) {
    let out = staged_dir(w);
    let archive = staged_archive(w, machine_platform());
    let checksum = std::fs::read_to_string(out.join(format!("{archive}.sha256")))
        .expect("read the checksum file beside this machine's archive");
    let digest = checksum
        .split_whitespace()
        .next()
        .expect("the checksum file carries a digest");
    assert_eq!(
        sha256_of(&out.join(&archive)),
        digest,
        "the checksum beside {archive} does not cover it"
    );
    assert!(
        w.command_transcript.contains(&archive),
        "the preparation did not report {archive}: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript.contains(digest),
        "the preparation did not report the checksum of {archive}: {}",
        w.command_transcript
    );
}

#[when("the maintainer extracts the archive for this machine's platform")]
async fn extracts_the_archive(w: &mut DutyWorld) {
    let archive = staged_archive(w, machine_platform());
    let extracted = extracted_dir(w);
    std::fs::create_dir_all(&extracted).expect("create the extraction directory");
    let mut command = Command::new("tar");
    command
        .arg("-xzf")
        .arg(staged_dir(w).join(&archive))
        .arg("-C")
        .arg(&extracted);
    run_command(w, &mut command);
    assert_eq!(
        w.exit_status,
        Some(0),
        "tar could not extract {archive}: {}",
        w.command_transcript
    );
}

#[when("runs the executable taken from that archive")]
async fn runs_the_extracted_executable(w: &mut DutyWorld) {
    let executable = extracted_dir(w).join(machine_executable());
    assert!(
        executable.is_file(),
        "the archive held no executable at {}",
        executable.display()
    );
    let mut command = Command::new(&executable);
    run_command(w, &mut command);
}

#[then(
    regex = r#"^the executable prints "([^"]+)" and reports failure through its process exit status$"#
)]
async fn prints_usage_and_reports_failure(w: &mut DutyWorld, printed: String) {
    assert!(
        w.command_transcript.contains(&printed),
        "the extracted executable did not print {printed:?}: {}",
        w.command_transcript
    );
    assert_ne!(
        w.exit_status,
        Some(0),
        "the extracted executable reported success: {}",
        w.command_transcript
    );
    commit_transcript(w);
}

#[given("a repository whose history releases version 0.1.0")]
async fn a_repository_releasing_version_0_1_0(w: &mut DutyWorld) {
    build_history(w, &FIRST_RELEASE_HISTORY);
}

#[then("the change log file holds a section for version 0.1.0")]
async fn the_change_log_holds_a_section_for_version_0_1_0(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the renderer did not succeed: {}",
        w.command_transcript
    );
    let path = rendered_change_log(w);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no change log at {}: {error}", path.display()));
    assert!(
        text.contains("## [0.1.0]"),
        "the change log holds no section for version 0.1.0: {text}"
    );
}

#[then("that section names the released change a user can see")]
async fn that_section_names_the_released_change(w: &mut DutyWorld) {
    let path = rendered_change_log(w);
    // Read the section out of the file the real command wrote, as a terminal
    // session would, so the committed transcript shows the newest section.
    let mut command = Command::new("cat");
    command.arg(&path);
    run_command(w, &mut command);
    assert_eq!(
        w.exit_status,
        Some(0),
        "reading the change log failed: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript
            .contains("Convert a supported duty plan into a calendar"),
        "the section names no released change a user can see: {}",
        w.command_transcript
    );
    commit_render_transcript(w);
}
