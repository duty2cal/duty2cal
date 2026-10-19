// SPDX-License-Identifier: MPL-2.0
//! End-to-end steps for the capability `publish-public-release`.
//!
//! One file per capability. These steps drive the real preparation command and
//! inspect the Windows archive it stages. The Linux test host cannot execute a
//! Windows binary, so execution evidence stays with the `windows-latest` smoke
//! job; the local scenario proves the archive, its checksum and its member.

use std::path::PathBuf;
use std::process::Command;

use cucumber::{given, then, when};

use crate::steps::publish_public_release::{
    build_history, change_log_path, run_command, sha256_of, stage_stand_in_executable,
    staged_archive, staged_dir, supported_executable, supported_platform, FIRST_RELEASE_HISTORY,
};
use crate::DutyWorld;

/// Where the scenario's terminal transcript is committed as evidence. The
/// change owns it while it is active; the archive owns it afterwards.
fn transcript_path() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let active = root.join("givn/changes/windows-only-release");
    let home = if active.is_dir() {
        active
    } else {
        root.join("givn/archive/windows-only-release")
    };
    home.join(
        "evidence/visual/\
         maintainer-prepares-a-windows-release-and-inspects-the-archived-executable/transcript.txt",
    )
}

/// The change the render scenario belongs to, so its evidence follows the
/// change: `givn/changes/<change>/` while the change is active, and
/// `givn/archive/<change>/` once it is archived. Writing to the change path
/// after archive would resurrect a directory the archive owns, and `givn
/// archive` refuses a tree that changed outside the evidence it declares.
const RENDER_CHANGE: &str = "generate-release-notes";

/// Where the render scenario's terminal transcript is committed as evidence.
fn render_transcript_path() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let active = root.join("givn/changes").join(RENDER_CHANGE);
    let home = if active.is_dir() {
        active
    } else {
        root.join("givn/archive").join(RENDER_CHANGE)
    };
    home.join(
        "evidence/visual/\
         maintainer-generates-the-change-log-for-a-version-and-reads-its-newest-section/transcript.txt",
    )
}

/// The directory the archive is extracted into.
fn extracted_dir(w: &DutyWorld) -> PathBuf {
    w.working_dir().join("extracted")
}

/// Commit the scenario's terminal transcript as evidence, with the run-specific
/// directory replaced by a placeholder. The staged executable is the fixed
/// stand-in, so its digest is stable and needs no scrub.
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

#[given(regex = r#"^a stand-in executable built for "([^"]+)"$"#)]
async fn a_stand_in_executable(w: &mut DutyWorld, platform: String) {
    assert_eq!(
        platform,
        supported_platform(),
        "the scenario names the supported platform"
    );
    stage_stand_in_executable(w, &platform);
}

#[then(regex = r#"^the preparation reports the archive and its checksum for "([^"]+)"$"#)]
async fn the_preparation_reports_archive_and_checksum(w: &mut DutyWorld, platform: String) {
    assert_eq!(
        platform,
        supported_platform(),
        "the scenario names the supported platform"
    );
    let out = staged_dir(w);
    let archive = staged_archive(w, &platform);
    let checksum = std::fs::read_to_string(out.join(format!("{archive}.sha256")))
        .expect("read the checksum file beside the archive");
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

#[when(regex = r#"^the maintainer extracts the archive for "([^"]+)"$"#)]
async fn extracts_the_archive(w: &mut DutyWorld, platform: String) {
    assert_eq!(
        platform,
        supported_platform(),
        "the scenario names the supported platform"
    );
    let archive = staged_archive(w, &platform);
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

#[then(regex = r#"^the extracted archive holds exactly the executable "([^"]+)"$"#)]
async fn the_archive_holds_exactly(w: &mut DutyWorld, executable: String) {
    let archive = staged_archive(w, supported_platform());
    let mut command = Command::new("tar");
    command.arg("-tzf").arg(staged_dir(w).join(&archive));
    run_command(w, &mut command);
    assert_eq!(
        w.exit_status,
        Some(0),
        "tar could not list {archive}: {}",
        w.command_transcript
    );
    let members: Vec<&str> = w
        .command_transcript
        .lines()
        .filter(|line| !line.is_empty())
        .collect();
    assert_eq!(
        members,
        vec![executable.as_str()],
        "the archive {archive} holds {members:?}"
    );
}

#[then("the extracted executable is the executable the preparation staged")]
async fn the_extracted_executable_is_the_staged_one(w: &mut DutyWorld) {
    let executable = supported_executable();
    let extracted = extracted_dir(w).join(executable);
    let staged = w
        .binaries
        .as_ref()
        .expect("the scenario prepared no executables")
        .join(supported_platform())
        .join(executable);
    let extracted_bytes = std::fs::read(&extracted)
        .unwrap_or_else(|error| panic!("read {}: {error}", extracted.display()));
    let staged_bytes =
        std::fs::read(&staged).unwrap_or_else(|error| panic!("read {}: {error}", staged.display()));
    assert_eq!(
        extracted_bytes, staged_bytes,
        "the extracted executable is not the executable the preparation staged"
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
