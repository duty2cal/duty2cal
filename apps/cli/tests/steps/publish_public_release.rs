// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `publish-public-release`.
//!
//! One file per capability. The steps drive the real preparation command
//! (`scripts/prepare-release.sh`) as a subprocess, as the conversion
//! capability's steps drive the built binary. The change-log scenarios drive
//! `scripts/render-changelog.sh` as a subprocess, as `design.md` requires. A
//! step body is never empty: the RED phase writes each new step as a stub that
//! panics with its own name.
#![allow(clippy::too_many_arguments)] // step signatures mirror the captured step text

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::SystemTime;

use cucumber::gherkin::Step;
use cucumber::{given, then, when};

use crate::DutyWorld;

/// The supported platforms and the executable name each one uses, in release
/// order. The script owns the canonical set; the steps speak it.
pub(crate) const SUPPORTED: [(&str, &str); 5] = [
    ("windows-x86_64", "duty2cal.exe"),
    ("linux-x86_64", "duty2cal"),
    ("linux-arm64", "duty2cal"),
    ("macos-arm64", "duty2cal"),
    ("macos-x86_64", "duty2cal"),
];

/// The release-text scenario's fixture history, oldest revision first: two
/// release-note subjects (one of them repeated by three revisions), three
/// housekeeping subjects, and one subject that carries no type at all. The
/// scopes are capabilities the repository holds and the whole history is the
/// version released, tagged `v0.2.0`, so the section rendered from it is the
/// release text.
const HISTORY: [(&str, &str, &str); 8] = [
    ("2026-01-01", "chore(ci): bootstrap the release channel", ""),
    (
        "2026-01-02",
        "feat(publish-public-release): Offer an archive for every supported platform",
        "",
    ),
    (
        "2026-01-03",
        "docs(readme): describe the download routes for readers",
        "",
    ),
    (
        "2026-01-04",
        "feat(publish-public-release): Offer an archive for every supported platform",
        "",
    ),
    (
        "2026-01-05",
        "fix(publish-public-release): Publish only an archive the evidence supports",
        "",
    ),
    ("2026-01-06", "Publish the notes by hand", ""),
    (
        "2026-01-07",
        "test(publish-public-release): cover the refusal paths",
        "",
    ),
    (
        "2026-01-08",
        "feat(publish-public-release): Offer an archive for every supported platform",
        "v0.2.0",
    ),
];

/// The subjects the release text must not carry: the housekeeping revisions'
/// notes and the subject that carries no release-note type.
const OMITTED: [&str; 4] = [
    "bootstrap the release channel",
    "describe the download routes for readers",
    "cover the refusal paths",
    "Publish the notes by hand",
];

/// Scenario sequence, so concurrently executed scenarios never share a directory.
static SCENARIO_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

/// Fresh per-scenario working directory.
fn fresh_dir() -> PathBuf {
    let sequence = SCENARIO_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let name = format!("duty2cal-release-{}-{sequence}", std::process::id());
    let dir = std::env::temp_dir().join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create the scenario directory");
    dir
}

/// Every supported platform name, in release order.
pub(crate) fn supported_platforms() -> Vec<&'static str> {
    SUPPORTED.iter().map(|(name, _)| *name).collect()
}

/// The executable name a platform uses.
pub(crate) fn executable_for(platform: &str) -> &'static str {
    SUPPORTED
        .iter()
        .find(|(name, _)| *name == platform)
        .map(|(_, executable)| *executable)
        .unwrap_or_else(|| panic!("unrecognised platform: {platform}"))
}

/// The platform this machine is: the one whose real executable the steps use.
pub(crate) fn machine_platform() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => "linux-x86_64",
        ("linux", "aarch64") => "linux-arm64",
        ("macos", "aarch64") => "macos-arm64",
        ("macos", "x86_64") => "macos-x86_64",
        ("windows", "x86_64") => "windows-x86_64",
        (os, arch) => panic!("this machine has no supported platform: {os}/{arch}"),
    }
}

/// The executable name this machine's platform uses.
pub(crate) fn machine_executable() -> &'static str {
    executable_for(machine_platform())
}

/// The repository root: the working directory the preparation is invoked from,
/// so a transcript shows `scripts/prepare-release.sh` and not a local path.
fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The scenario's working directory, created on first use.
fn scenario_dir(w: &mut DutyWorld) -> PathBuf {
    if w.dir.is_none() {
        w.dir = Some(fresh_dir());
    }
    w.dir.clone().expect("scenario directory")
}

/// The directory the preparation stages the release into.
pub(crate) fn staged_dir(w: &DutyWorld) -> PathBuf {
    w.working_dir().join("out")
}

/// The bytes of the executable built for a platform: the real built binary for
/// this machine's platform, a stand-in for the others.
fn executable_bytes(platform: &str) -> Vec<u8> {
    if platform == machine_platform() {
        std::fs::read(env!("CARGO_BIN_EXE_duty2cal")).expect("read the built executable")
    } else {
        format!("stand-in executable for {platform}\n").into_bytes()
    }
}

/// Write one executable for a platform.
fn write_executable(binaries: &Path, platform: &str, bytes: &[u8]) {
    let dir = binaries.join(platform);
    std::fs::create_dir_all(&dir).expect("create the platform directory");
    let path = dir.join(executable_for(platform));
    std::fs::write(&path, bytes).expect("write the executable");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mut permissions = std::fs::metadata(&path)
            .expect("stat the executable")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&path, permissions).expect("mark the executable");
    }
}

/// Build the scenario's executables directory from the platforms named.
pub(crate) fn build_binaries(w: &mut DutyWorld, platforms: &[&str]) {
    let dir = scenario_dir(w);
    let binaries = dir.join("binaries");
    let _ = std::fs::remove_dir_all(&binaries);
    std::fs::create_dir_all(&binaries).expect("create the binaries directory");
    for platform in platforms {
        write_executable(&binaries, platform, &executable_bytes(platform));
    }
    let out = dir.join("out");
    w.binaries = Some(binaries);
    w.staged = Some(out);
}

/// A command line, as a terminal would show it.
fn command_line(command: &Command) -> String {
    let mut parts = vec![command.get_program().to_string_lossy().into_owned()];
    parts.extend(
        command
            .get_args()
            .map(|argument| argument.to_string_lossy().into_owned()),
    );
    parts.join(" ")
}

/// Run a command, recording its transcript and exit status. The scenario's own
/// transcript reads like a terminal session, because an `@e2e` scenario commits
/// it as evidence.
pub(crate) fn run_command(w: &mut DutyWorld, command: &mut Command) {
    w.scenario_transcript.push_str("$ ");
    w.scenario_transcript.push_str(&command_line(command));
    w.scenario_transcript.push('\n');
    let output = command.output().expect("run the command");
    w.exit_status = output.status.code();
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    w.command_transcript = format!("{stdout}{stderr}");
    let text = w.command_transcript.clone();
    w.scenario_transcript.push_str(&text);
    w.scenario_transcript.push('\n');
}

/// Run the preparation command for a version.
pub(crate) fn run_prepare(w: &mut DutyWorld, version: &str) {
    if w.binaries.is_none() {
        build_binaries(w, &supported_platforms());
    }
    let out = match w.staged.clone() {
        Some(dir) => dir,
        None => w.working_dir().join("out"),
    };
    w.staged = Some(out.clone());
    std::fs::create_dir_all(&out).expect("create the staged output directory");
    let binaries = w
        .binaries
        .clone()
        .expect("the scenario prepared no executables");
    let mut command = Command::new("scripts/prepare-release.sh");
    command
        .current_dir(repository_root())
        .arg("--version")
        .arg(version)
        .arg("--binaries")
        .arg(binaries)
        .arg("--out")
        .arg(&out);
    if let Some(repo) = &w.fixture_repo {
        command.arg("--repo").arg(repo);
    }
    run_command(w, &mut command);
}

/// The names of the entries in a directory, sorted; a missing directory has none.
pub(crate) fn entries(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = match std::fs::read_dir(dir) {
        Ok(listed) => listed
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect(),
        Err(_) => Vec::new(),
    };
    names.sort();
    names
}

/// The staged archive for a platform, or a panic naming the staged set.
pub(crate) fn staged_archive(w: &DutyWorld, platform: &str) -> String {
    let suffix = format!("-{platform}.tar.gz");
    entries(&staged_dir(w))
        .into_iter()
        .find(|name| name.ends_with(&suffix))
        .unwrap_or_else(|| {
            panic!(
                "no staged archive for platform {platform}: {:?}",
                entries(&staged_dir(w))
            )
        })
}

/// The rows of a step's data table, without the header row.
fn table_rows(step: &Step) -> Vec<Vec<String>> {
    let table = step.table.as_ref().expect("the step carries a data table");
    table.rows.iter().skip(1).cloned().collect()
}

/// The delta change-log scenarios' fixture history, oldest revision first: a
/// new capability, a fix, and work no user can see (housekeeping and an
/// unrecognised type). The capability scope is one the repository holds, so the
/// allow-list accepts it.
const CHANGE_LOG_HISTORY: [(&str, &str, &str); 6] = [
    (
        "2026-01-01",
        "feat(export-duty-plan-calendar): Convert a supported duty plan into a calendar",
        "",
    ),
    ("2026-01-02", "chore(ci): tune the release pipeline", ""),
    (
        "2026-01-03",
        "docs(readme): describe the download route",
        "",
    ),
    (
        "2026-01-04",
        "fix(publish-public-release): Name the platform in the refusal",
        "",
    ),
    (
        "2026-01-05",
        "test(publish-public-release): cover the refusal paths",
        "v0.2.0",
    ),
    ("2026-01-06", "Publish the notes by hand", ""),
];

/// The notes the delta history must not carry: the housekeeping revisions and
/// the subject that carries no release-note type.
const CHANGE_LOG_OMITTED: [&str; 4] = [
    "tune the release pipeline",
    "describe the download route",
    "cover the refusal paths",
    "Publish the notes by hand",
];

/// Run one git command in the fixture repository with a fixed committer date,
/// asserting it succeeded.
fn git_dated(repo: &Path, date: &str, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .env("GIT_AUTHOR_DATE", format!("{date}T00:00:00+00:00"))
        .env("GIT_COMMITTER_DATE", format!("{date}T00:00:00+00:00"))
        .output()
        .expect("run git in the fixture repository");
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// A fixture path a user receives, and one that only decides how the project
/// works on what it ships. A revision's own subject picks one by default, so a
/// history reads as the changes it is meant to express; a scenario that needs
/// the opposite says so in `build_history_with`.
pub(crate) const RECEIVED: &str = "crates/duty2cal-core/src/lib.rs";
pub(crate) const MACHINERY: &str = "scripts/render-changelog.sh";

/// Every path the committed configuration lists as one a user receives, so the
/// renderer's list check passes against a fixture repository.
const RECEIVED_PATHS: [&str; 8] = [
    "crates/duty2cal-core/src/lib.rs",
    "apps/cli/src/main.rs",
    "Cargo.toml",
    "Cargo.lock",
    "rust-toolchain.toml",
    ".cargo/config.toml",
    "scripts/prepare-release.sh",
    ".github/workflows/release.yml",
];

/// The path a revision writes when the scenario does not override it: a
/// revision the project releases changed something a user receives, and every
/// other revision changed the machinery.
fn default_path(subject: &str) -> &'static str {
    let end = subject.find('(').unwrap_or(subject.len());
    match &subject[..end] {
        "feat" | "fix" | "perf" | "refactor" | "revert" => RECEIVED,
        _ => MACHINERY,
    }
}

/// Build a fixture repository whose history is the revisions named, oldest
/// first, with fixed committer dates and `v*` tags.
pub(crate) fn build_history(w: &mut DutyWorld, revisions: &[(&str, &str, &str)]) {
    build_history_with(w, revisions, &[]);
}

/// Build a fixture repository whose history is the revisions named, oldest
/// first, with fixed committer dates and `v*` tags, and whose revisions write
/// the paths given here instead of the ones their subjects imply.
///
/// The committed configuration and the capability directories the allow-list
/// check reads are copied in, and every path the configuration lists as one a
/// user receives is created, so the renderer runs exactly as it does against
/// the real tree.
pub(crate) fn build_history_with(
    w: &mut DutyWorld,
    revisions: &[(&str, &str, &str)],
    overrides: &[(usize, &[&str])],
) {
    let repo = scenario_dir(w).join("history");
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(&repo).expect("create the fixture repository");
    std::fs::copy(
        repository_root().join("cliff.toml"),
        repo.join("cliff.toml"),
    )
    .expect("copy the committed configuration");
    for capability in ["export-duty-plan-calendar", "publish-public-release"] {
        std::fs::create_dir_all(repo.join("givn/specs").join(capability))
            .expect("create a capability directory");
    }
    git(&repo, &["init", "--quiet"]);
    // The user-receiving paths the configuration lists are committed as
    // plumbing before the history starts, so a revision that changes one of
    // them is what names it, and this scaffold revision is never released: it
    // carries no conventional-commit scope at all.
    for path in RECEIVED_PATHS {
        let file = repo.join(path);
        std::fs::create_dir_all(file.parent().expect("a parent directory"))
            .expect("create a fixture directory");
        std::fs::write(&file, "fixture\n").expect("write a fixture path");
    }
    git(&repo, &["add", "--all"]);
    git_dated(
        &repo,
        "2025-12-31",
        &[
            "-c",
            "user.name=duty2cal tests",
            "-c",
            "user.email=tests@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "--quiet",
            "--no-verify",
            "-m",
            "Scaffold the fixture tree",
        ],
    );
    for (index, (date, subject, tag)) in revisions.iter().enumerate() {
        let paths: Vec<&str> = match overrides.iter().find(|(at, _)| *at == index) {
            Some((_, paths)) => paths.to_vec(),
            None => vec![default_path(subject)],
        };
        for path in paths {
            let file = repo.join(path);
            std::fs::create_dir_all(file.parent().expect("a parent directory"))
                .expect("create a revision directory");
            std::fs::write(&file, format!("{subject}\n")).expect("write the revision's file");
        }
        git(&repo, &["add", "--all"]);
        git_dated(
            &repo,
            date,
            &[
                "-c",
                "user.name=duty2cal tests",
                "-c",
                "user.email=tests@example.invalid",
                "-c",
                "commit.gpgsign=false",
                "commit",
                "--quiet",
                "--no-verify",
                "-m",
                subject,
            ],
        );
        if !tag.is_empty() {
            git(&repo, &["tag", tag]);
        }
    }
    w.fixture_repo = Some(repo);
}

/// The change log path for a scenario's working directory.
pub(crate) fn change_log_path(w: &DutyWorld) -> PathBuf {
    w.working_dir().join("CHANGELOG.md")
}

/// Run the renderer for a version against the fixture repository, recording the
/// transcript as an `@e2e` scenario commits it.
fn run_render(w: &mut DutyWorld, version: &str, repo: &Path, out: &Path) {
    let mut command = Command::new("scripts/render-changelog.sh");
    command
        .current_dir(repository_root())
        .arg("--version")
        .arg(version)
        .arg("--repo")
        .arg(repo)
        .arg("--out")
        .arg(out);
    run_command(w, &mut command);
}

/// Run one git command in the fixture repository, asserting it succeeded.
fn git(repo: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .expect("run git in the fixture repository");
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// The release text the preparation staged.
fn release_text(w: &DutyWorld) -> String {
    let path = staged_dir(w).join("release-notes.md");
    match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => panic!("no release text at {}: {error}", path.display()),
    }
}

/// The release text the scenario asserts against: the staged release text when
/// the preparation produced one, otherwise the change log the scenario
/// rendered.
fn release_text_or_change_log(w: &DutyWorld) -> String {
    let staged = staged_dir(w).join("release-notes.md");
    if staged.is_file() {
        return std::fs::read_to_string(&staged).expect("read the staged release text");
    }
    change_log(w)
}

/// A file's modification time.
fn modification_time(path: &Path) -> SystemTime {
    std::fs::metadata(path)
        .unwrap_or_else(|error| panic!("no file at {}: {error}", path.display()))
        .modified()
        .expect("the file records a modification time")
}

/// The archives in a directory with their modification times, sorted by name.
fn archives_with_times(dir: &Path) -> Vec<(String, SystemTime)> {
    entries(dir)
        .into_iter()
        .filter(|name| name.ends_with(".tar.gz"))
        .map(|name| {
            let time = modification_time(&dir.join(&name));
            (name, time)
        })
        .collect()
}

/// The sha256 digest of a file, as `sha256sum` prints it.
pub(crate) fn sha256_of(path: &Path) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("compute the digest with sha256sum");
    assert!(
        output.status.success(),
        "sha256sum could not read {}: {}",
        path.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .next()
        .expect("sha256sum printed a digest")
        .to_string()
}

/// The member names of an archive, as `tar` lists them.
fn archive_members(archive: &Path) -> Vec<String> {
    let output = Command::new("tar")
        .arg("-tzf")
        .arg(archive)
        .output()
        .expect("list the archive with tar");
    assert!(
        output.status.success(),
        "tar could not list {}: {}",
        archive.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

#[given("an executable built for every supported platform")]
async fn every_supported_platform(w: &mut DutyWorld) {
    build_binaries(w, &supported_platforms());
}

#[when(regex = r"^the maintainer prepares the release for version (\S+)$")]
async fn prepares_the_release(w: &mut DutyWorld, version: String) {
    run_prepare(w, &version);
}

#[then("the prepared release holds exactly these archives, each with its checksum file")]
async fn holds_exactly_these_archives(w: &mut DutyWorld, step: &Step) {
    let out = staged_dir(w);
    let rows = table_rows(step);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the preparation did not succeed: {}",
        w.command_transcript
    );
    assert_eq!(
        rows.len(),
        SUPPORTED.len(),
        "the step names one archive per supported platform"
    );
    let mut expected: Vec<String> = Vec::new();
    for row in &rows {
        let (archive, executable) = (&row[0], &row[1]);
        let platform = SUPPORTED
            .iter()
            .find(|(name, _)| archive.ends_with(&format!("-{name}.tar.gz")))
            .map(|(name, _)| *name)
            .unwrap_or_else(|| panic!("the step names no supported platform: {archive}"));
        assert_eq!(
            executable_for(platform),
            executable,
            "the executable named for {platform}"
        );
        assert!(
            out.join(archive).is_file(),
            "no archive at {}: {}",
            out.join(archive).display(),
            w.command_transcript
        );
        assert!(
            out.join(format!("{archive}.sha256")).is_file(),
            "no checksum file beside {archive}"
        );
        expected.push(archive.clone());
        expected.push(format!("{archive}.sha256"));
    }
    // The staged set is exactly the archives, their checksums, the release
    // text and the change log — no other archive in it.
    expected.push("release-notes.md".to_string());
    expected.push("CHANGELOG.md".to_string());
    expected.sort();
    assert_eq!(
        entries(&out),
        expected,
        "the staged set is not the five archives, their checksums, the release text and the change log"
    );
}

#[then("every archive holds only its own executable")]
async fn every_archive_holds_only_its_executable(w: &mut DutyWorld) {
    let out = staged_dir(w);
    for (platform, executable) in SUPPORTED {
        let archive = staged_archive(w, platform);
        assert_eq!(
            archive_members(&out.join(&archive)),
            vec![executable.to_string()],
            "the archive {archive} does not hold only {executable}"
        );
    }
}

#[given(regex = r#"^an executable built for every supported platform except "([^"]+)"$"#)]
async fn every_supported_platform_except(w: &mut DutyWorld, missing: String) {
    let platforms: Vec<&str> = supported_platforms()
        .into_iter()
        .filter(|platform| *platform != missing.as_str())
        .collect();
    assert_eq!(
        platforms.len(),
        SUPPORTED.len() - 1,
        "the step names a supported platform to leave out"
    );
    build_binaries(w, &platforms);
}

#[then(regex = r#"^the preparation fails and names "([^"]+)"$"#)]
async fn the_preparation_fails_and_names(w: &mut DutyWorld, named: String) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the preparation did not fail: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript.contains(&named),
        "the failure does not name {named}: {}",
        w.command_transcript
    );
}

#[then("the prepared release holds no archive")]
async fn holds_no_archive(w: &mut DutyWorld) {
    let staged = entries(&staged_dir(w));
    assert!(
        staged.is_empty(),
        "the refused preparation staged {staged:?}"
    );
}

#[given(regex = r"^a prepared release for version (\S+)$")]
async fn a_prepared_release(w: &mut DutyWorld, version: String) {
    run_prepare(w, &version);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the preparation did not succeed: {}",
        w.command_transcript
    );
}

#[when(regex = r#"^anyone verifies the checksum against the archive for "([^"]+)"$"#)]
async fn anyone_verifies_the_checksum(w: &mut DutyWorld, platform: String) {
    let out = staged_dir(w);
    let archive = staged_archive(w, &platform);
    let mut command = Command::new("sha256sum");
    command
        .arg("--check")
        .arg(format!("{archive}.sha256"))
        .current_dir(&out);
    run_command(w, &mut command);
}

#[then("the checksum matches the archive it names")]
async fn the_checksum_matches_the_archive_it_names(w: &mut DutyWorld) {
    let out = staged_dir(w);
    let transcript = w.command_transcript.clone();
    let verified: Vec<&str> = transcript
        .lines()
        .filter_map(|line| line.strip_suffix(": OK"))
        .collect();
    assert_eq!(
        verified.len(),
        1,
        "the verification did not report exactly one verified file: {transcript}"
    );
    let archive = verified[0];
    assert_eq!(
        w.exit_status,
        Some(0),
        "the verification did not succeed: {transcript}"
    );
    assert!(
        out.join(archive).is_file(),
        "the verified archive is not in the staged set: {archive}"
    );
    let checksum = std::fs::read_to_string(out.join(format!("{archive}.sha256")))
        .expect("read the checksum file");
    let mut fields = checksum.split_whitespace();
    let digest = fields.next().expect("the checksum file carries a digest");
    let named = fields.next().expect("the checksum file names its archive");
    assert_eq!(
        named, archive,
        "the checksum does not name the archive it covers: {checksum}"
    );
    assert_eq!(
        sha256_of(&out.join(archive)),
        digest,
        "the checksum does not cover the archive: {checksum}"
    );
}

#[when("the maintainer prepares version 0.1.0 again from the same executables")]
async fn prepares_again_from_the_same_executables(w: &mut DutyWorld) {
    let again = w.working_dir().join("out-again");
    w.staged = Some(again);
    run_prepare(w, "0.1.0");
    assert_eq!(
        w.exit_status,
        Some(0),
        "the second preparation did not succeed: {}",
        w.command_transcript
    );
}

#[then("the archives are identical to the first preparation, byte for byte")]
async fn the_archives_are_identical(w: &mut DutyWorld) {
    let first = staged_dir(w);
    let again = w.working_dir().join("out-again");
    for (platform, _) in SUPPORTED {
        let archive = staged_archive(w, platform);
        let first_bytes = std::fs::read(first.join(&archive)).expect("read the first archive");
        let again_bytes = match std::fs::read(again.join(&archive)) {
            Ok(bytes) => bytes,
            Err(error) => panic!("the second preparation staged no {archive}: {error}"),
        };
        assert_eq!(
            first_bytes, again_bytes,
            "the archive {archive} differs between the two preparations"
        );
    }
}

#[when("the maintainer prepares version 0.1.0 in the same place again")]
async fn prepares_in_the_same_place_again(w: &mut DutyWorld) {
    let out = staged_dir(w);
    w.staged_snapshot = archives_with_times(&out);
    assert_eq!(
        w.staged_snapshot.len(),
        SUPPORTED.len(),
        "the first preparation staged no archives: {}",
        w.command_transcript
    );
    w.staged = Some(out);
    run_prepare(w, "0.1.0");
}

#[then(regex = r"^the preparation fails and names version (\S+)$")]
async fn the_preparation_fails_and_names_version(w: &mut DutyWorld, version: String) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the preparation did not fail: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript.contains(&version),
        "the failure does not name version {version}: {}",
        w.command_transcript
    );
}

#[then("the staged archives keep their earlier modification times")]
async fn keeps_their_earlier_modification_times(w: &mut DutyWorld) {
    assert!(
        !w.staged_snapshot.is_empty(),
        "the scenario captured no staged archive"
    );
    let out = staged_dir(w);
    for (archive, before) in &w.staged_snapshot {
        let after = modification_time(&out.join(archive));
        assert_eq!(
            &after, before,
            "the refused preparation rewrote the staged archive {archive}"
        );
    }
}

#[given(regex = r#"^an executable built for the platform "([^"]+)" that cannot be read$"#)]
async fn an_unreadable_executable(w: &mut DutyWorld, platform: String) {
    build_binaries(w, &supported_platforms());
    let path = w
        .binaries
        .clone()
        .expect("the scenario prepared no executables")
        .join(&platform)
        .join(executable_for(&platform));
    assert!(
        path.is_file(),
        "there is no executable to make unreadable: {}",
        path.display()
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mut permissions = std::fs::metadata(&path)
            .expect("stat the executable")
            .permissions();
        permissions.set_mode(0o000);
        std::fs::set_permissions(&path, permissions)
            .expect("remove every permission from the executable");
    }
    #[cfg(not(unix))]
    panic!("the release-preparation scenarios are Linux-only");
}

#[given("a history whose released revisions carry release-note subjects, including one subject repeated by several revisions and one subject that carries no release-note type, and whose housekeeping revisions do not")]
async fn a_history_with_release_note_subjects(w: &mut DutyWorld) {
    // The preparation renders the release text out of this history, so the
    // fixture carries the committed configuration, the capability directories
    // the allow-list check reads, fixed committer dates and the `v0.2.0` tag.
    build_history(w, &HISTORY);
}

#[then(regex = r"^the release text names version (\S+)$")]
async fn the_release_text_names_version(w: &mut DutyWorld, version: String) {
    let text = release_text(w);
    assert!(
        text.contains(&version),
        "the release text does not name version {version}: {text}"
    );
}

#[then("the release text lists these release notes in release order")]
async fn the_release_text_lists_these_release_notes(w: &mut DutyWorld, step: &Step) {
    let text = release_text_or_change_log(w);
    let expected: Vec<String> = table_rows(step)
        .into_iter()
        .map(|row| row[0].clone())
        .collect();
    let listed: Vec<String> = text
        .lines()
        .filter_map(|line| line.strip_prefix("- "))
        .map(str::to_string)
        .collect();
    assert_eq!(
        listed, expected,
        "the release text does not list the expected notes in order: {text}"
    );
}

#[then("the release text omits the housekeeping revisions and the subject that carries no release-note type")]
async fn the_release_text_omits_the_others(w: &mut DutyWorld) {
    let text = release_text_or_change_log(w);
    for omitted in OMITTED {
        assert!(
            !text.contains(omitted),
            "the release text carries the omitted subject {omitted:?}: {text}"
        );
    }
}

/// The change-log notes of the file the scenario rendered, in order: the
/// bullets of the section the scenario names. The set of subjects a release
/// lists is what the group and collapse assertions read.
fn change_log_notes(w: &DutyWorld) -> Vec<String> {
    let path = w.changelog.clone().unwrap_or_else(|| change_log_path(w));
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no change log at {}: {error}", path.display()));
    text.lines()
        .filter_map(|line| line.strip_prefix("- "))
        .map(str::to_string)
        .collect()
}

/// The newest section of the change log the scenario rendered.
fn change_log(w: &DutyWorld) -> String {
    let path = w.changelog.clone().unwrap_or_else(|| change_log_path(w));
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no change log at {}: {error}", path.display()))
}

#[given(
    "a history whose released revisions name a new capability, a fix, and work no user can see"
)]
async fn a_history_grouping(w: &mut DutyWorld) {
    build_history(w, &CHANGE_LOG_HISTORY);
}

#[when(regex = r"^the maintainer generates the change log for version (\S+)$")]
async fn generates_the_change_log(w: &mut DutyWorld, version: String) {
    render_version(w, &version);
}

#[then("the notes for version 0.2.0 name the new capability and the fix")]
async fn notes_name_capability_and_fix(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the generation did not succeed: {}",
        w.command_transcript
    );
    let notes = change_log_notes(w);
    let joined = notes.join("\n");
    assert!(
        joined.contains("Convert a supported duty plan into a calendar"),
        "the notes name no new capability: {joined}"
    );
    assert!(
        joined.contains("Name the platform in the refusal"),
        "the notes name no fix: {joined}"
    );
}

#[then("the notes name the new capability before the fix")]
async fn capability_before_fix(w: &mut DutyWorld) {
    let notes = change_log_notes(w);
    let capability = notes
        .iter()
        .position(|note| note.contains("Convert a supported duty plan into a calendar"))
        .expect("the notes name no new capability");
    let fix = notes
        .iter()
        .position(|note| note.contains("Name the platform in the refusal"))
        .expect("the notes name no fix");
    assert!(
        capability < fix,
        "the fix is named before the new capability: {notes:?}"
    );
}

#[then("the notes name no revision whose work no user can see")]
async fn notes_omit_invisible_work(w: &mut DutyWorld) {
    let notes = change_log_notes(w).join("\n");
    for omitted in CHANGE_LOG_OMITTED {
        assert!(
            !notes.contains(omitted),
            "the notes carry the invisible subject {omitted:?}: {notes}"
        );
    }
}

/// The delta history with one more released revision: a fix that carries the
/// capability's scope and names the project's own tooling. It passes the
/// allow-list, so only the machinery skip list can keep it out of the notes.
const CHANGE_LOG_TOOLING_HISTORY: [(&str, &str, &str); 7] = [
    (
        "2026-01-01",
        "feat(export-duty-plan-calendar): Convert a supported duty plan into a calendar",
        "",
    ),
    ("2026-01-02", "chore(ci): tune the release pipeline", ""),
    (
        "2026-01-03",
        "docs(readme): describe the download route",
        "",
    ),
    (
        "2026-01-04",
        "fix(publish-public-release): Name the platform in the refusal",
        "",
    ),
    (
        "2026-01-05",
        "fix(publish-public-release): build a clean merged coverage report",
        "",
    ),
    (
        "2026-01-06",
        "test(publish-public-release): cover the refusal paths",
        "v0.2.0",
    ),
    ("2026-01-07", "Publish the notes by hand", ""),
];

/// The release-note words the machinery skip list must keep out of a version's
/// notes even though the revision carries a capability scope.
const CHANGE_LOG_TOOLING: [&str; 1] = ["build a clean merged coverage report"];

#[given("a revision typed as a fix of that capability whose words name the project's own tooling")]
async fn a_revision_naming_the_projects_own_tooling(w: &mut DutyWorld) {
    build_history(w, &CHANGE_LOG_TOOLING_HISTORY);
}

#[then("the notes name no revision whose words name the project's own tooling")]
async fn notes_omit_the_projects_own_tooling(w: &mut DutyWorld) {
    let notes = change_log_notes(w).join("\n");
    for tooling in CHANGE_LOG_TOOLING {
        assert!(
            !notes.contains(tooling),
            "the notes carry the project's own tooling {tooling:?}: {notes}"
        );
    }
}

/// The delta history with one more released revision: a feature that borrows a
/// machinery skip-list word for a crew member's own work. The word names the
/// crew's work, not the project's tooling, so only a skip list narrowed to
/// distinctive tooling phrases leaves it in the notes.
const CHANGE_LOG_BORROWED_WORD_HISTORY: [(&str, &str, &str); 8] = [
    (
        "2026-01-01",
        "feat(export-duty-plan-calendar): Convert a supported duty plan into a calendar",
        "",
    ),
    ("2026-01-02", "chore(ci): tune the release pipeline", ""),
    (
        "2026-01-03",
        "docs(readme): describe the download route",
        "",
    ),
    (
        "2026-01-04",
        "fix(publish-public-release): Name the platform in the refusal",
        "",
    ),
    (
        "2026-01-05",
        "fix(publish-public-release): build a clean merged coverage report",
        "",
    ),
    (
        "2026-01-06",
        "feat(export-duty-plan-calendar): Support the crew's ramp workflow",
        "",
    ),
    (
        "2026-01-07",
        "test(publish-public-release): cover the refusal paths",
        "v0.2.0",
    ),
    ("2026-01-08", "Publish the notes by hand", ""),
];

/// The released subject that merely borrows a tooling word for a crew member's
/// own work; the narrowed skip list must still name it.
const BORROWED_TOOLING_WORD_SUBJECT: &str = "Support the crew's ramp workflow";

#[given("a revision whose subject borrows a tooling word for a crew member's own work")]
async fn a_revision_borrowing_a_tooling_word(w: &mut DutyWorld) {
    build_history(w, &CHANGE_LOG_BORROWED_WORD_HISTORY);
}

#[then(
    "the notes name the revision whose subject borrows a tooling word for a crew member's own work"
)]
async fn notes_name_the_borrowed_word_revision(w: &mut DutyWorld) {
    let notes = change_log_notes(w).join("\n");
    assert!(
        notes.contains(BORROWED_TOOLING_WORD_SUBJECT),
        "the notes dropped a released change whose subject merely borrows a tooling word: {notes}"
    );
}

/// A history whose released revisions name only work no user can see.
const INVISIBLE_HISTORY: [(&str, &str, &str); 3] = [
    ("2026-01-01", "chore(ci): tune the release pipeline", ""),
    (
        "2026-01-02",
        "docs(readme): describe the download route",
        "",
    ),
    (
        "2026-01-03",
        "build(packaging): pin the toolchain",
        "v0.2.0",
    ),
];

#[given("a history whose released revisions name only work no user can see")]
async fn a_history_with_invisible_work(w: &mut DutyWorld) {
    build_history(w, &INVISIBLE_HISTORY);
}

#[then("the generation fails and states that the version changes nothing a user can see")]
async fn generation_refuses_invisible_version(w: &mut DutyWorld) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the generation did not fail: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript
            .contains("version 0.2.0 changes nothing a user can see"),
        "the failure does not state that the version changes nothing a user can see: {}",
        w.command_transcript
    );
}

/// The section of a change log that carries a version, from its heading to the
/// heading of the next version (or the end of the file).
fn section_of(text: &str, version: &str) -> String {
    let heading = format!("## [{version}]");
    let mut section = String::new();
    let mut inside = false;
    for line in text.lines() {
        if line.starts_with(&heading) {
            inside = true;
        } else if inside && line.starts_with("## [") {
            break;
        }
        if inside {
            section.push_str(line);
            section.push('\n');
        }
    }
    assert!(
        !section.is_empty(),
        "the change log holds no section for version {version}: {text}"
    );
    section
}

/// Render a version into the scenario's change log, recording the transcript.
fn render_version(w: &mut DutyWorld, version: &str) {
    let repo = w
        .fixture_repo
        .clone()
        .expect("the scenario built no fixture repository");
    let out = w.changelog.clone().unwrap_or_else(|| change_log_path(w));
    w.changelog = Some(out.clone());

    run_render(w, version, &repo, &out);
}

/// The fixture history of two releases: a first version and a second whose
/// visible changes are a new capability and a fix.
fn two_release_history(w: &mut DutyWorld) {
    let longer: Vec<(&str, &str, &str)> = vec![
        ("2026-01-01", "chore(ci): bootstrap the release channel", ""),
        (
            "2026-01-02",
            "feat(export-duty-plan-calendar): Convert a supported plan into a calendar",
            "v0.1.0",
        ),
        (
            "2026-02-01",
            "feat(publish-public-release): Offer an archive per supported platform",
            "",
        ),
        (
            "2026-02-02",
            "fix(publish-public-release): Name the platform in the refusal",
            "v0.2.0",
        ),
    ];
    build_history(w, &longer);
}

#[given("a change log whose newest section is version 0.1.0")]
#[allow(dead_code)]
async fn a_change_log_with_version_0_1_0(w: &mut DutyWorld) {
    two_release_history(w);
    render_version(w, "0.1.0");
    assert_eq!(
        w.exit_status,
        Some(0),
        "rendering version 0.1.0 did not succeed: {}",
        w.command_transcript
    );
}

#[then("version 0.2.0 is the newest section of the change log")]

async fn version_0_2_0_is_newest(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "rendering version 0.2.0 did not succeed: {}",
        w.command_transcript
    );
    let text = change_log(w);
    let newest = text
        .lines()
        .find(|line| line.starts_with("## ["))
        .expect("the change log holds no section");
    assert!(
        newest.starts_with("## [0.2.0]"),
        "the newest section is not version 0.2.0: {newest}"
    );
}

#[then("the section for version 0.1.0 is unchanged")]

async fn earlier_section_is_unchanged(w: &mut DutyWorld) {
    let text = change_log(w);
    let section = section_of(&text, "0.1.0");
    assert!(
        section.contains("Convert a supported plan into a calendar"),
        "the earlier section lost its release note: {section}"
    );
    assert!(
        !section.contains("Offer an archive per supported platform"),
        "the earlier section absorbed a later version's note: {section}"
    );
    assert_eq!(
        text.matches("## [0.1.0]").count(),
        1,
        "version 0.1.0 has more than one section: {text}"
    );
    assert_eq!(
        text.matches("## [0.2.0]").count(),
        1,
        "version 0.2.0 has more than one section: {text}"
    );
}

/// Run the renderer in `--check` mode for a version, recording the transcript.
fn run_check(w: &mut DutyWorld, version: &str) {
    let repo = w
        .fixture_repo
        .clone()
        .expect("the scenario built no fixture repository");
    let out = w.changelog.clone().unwrap_or_else(|| change_log_path(w));
    let mut command = Command::new("scripts/render-changelog.sh");
    command
        .current_dir(repository_root())
        .arg("--version")
        .arg(version)
        .arg("--repo")
        .arg(&repo)
        .arg("--out")
        .arg(&out)
        .arg("--check");
    run_command(w, &mut command);
}

#[given("a committed change log whose newest section differs from what the history states")]
#[allow(dead_code)]
async fn a_committed_change_log_that_differs(w: &mut DutyWorld) {
    two_release_history(w);
    let path = w.changelog.clone().unwrap_or_else(|| change_log_path(w));
    w.changelog = Some(path.clone());
    let committed = "\
## [0.2.0] - 2026-02-02\n\
\n\
### New capabilities\n\
- Offer an archive for every supported platform\n\
\n\
[0.1.0...0.2.0](https://github.com/kressnerd/duty2cal/compare/v0.1.0...v0.2.0)\n\
\n\
## [0.1.0] - 2026-01-02\n\
\n\
### New capabilities\n\
- Convert a supported plan into a calendar\n";
    std::fs::write(&path, committed).expect("write the committed change log");
}

#[when("the change log is checked against the released revision")]
async fn the_change_log_is_checked(w: &mut DutyWorld) {
    run_check(w, "0.2.0");
}

#[then(regex = r"^the check fails and names the version whose section differs$")]
#[allow(dead_code)]
async fn the_check_fails_and_names_the_version(w: &mut DutyWorld) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the check did not fail: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript.contains("version 0.2.0"),
        "the failure does not name the version whose section differs: {}",
        w.command_transcript
    );
}

#[given(regex = r#"^an executable presented as the unrecognised platform "([^"]+)"$"#)]
async fn an_unrecognised_platform(w: &mut DutyWorld, platform: String) {
    assert!(
        !supported_platforms().contains(&platform.as_str()),
        "the step names a platform inside the supported set: {platform}"
    );
    let binaries = w
        .binaries
        .clone()
        .expect("the scenario prepared no executables");
    let dir = binaries.join(&platform);
    std::fs::create_dir_all(&dir).expect("create the unrecognised platform directory");
    std::fs::write(
        dir.join("duty2cal"),
        b"stand-in executable for an unrecognised platform\n",
    )
    .expect("write the unrecognised executable");
}

#[given("a change log holding version 0.1.0 and a history releasing version 0.2.0")]
async fn a_change_log_holding_version_0_1_0(w: &mut DutyWorld) {
    two_release_history(w);
    render_version(w, "0.1.0");
    assert_eq!(
        w.exit_status,
        Some(0),
        "rendering version 0.1.0 did not succeed: {}",
        w.command_transcript
    );
}

#[then("the section for version 0.2.0 states its own date")]
async fn section_states_its_own_date(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "rendering version 0.2.0 did not succeed: {}",
        w.command_transcript
    );
    let section = section_of(&change_log(w), "0.2.0");
    // The released revision is the one the `v0.2.0` tag names, committed with
    // the fixture's fixed committer date; the heading states that date, never
    // the clock.
    assert!(
        section.starts_with("## [0.2.0] - 2026-02-02"),
        "the section does not state the released revision's own date: {section}"
    );
}

#[then("the section for version 0.2.0 states the comparison against version 0.1.0")]
async fn section_states_the_comparison(w: &mut DutyWorld) {
    let section = section_of(&change_log(w), "0.2.0");
    assert!(
        section.contains("https://github.com/kressnerd/duty2cal/compare/v0.1.0...v0.2.0"),
        "the section does not state the comparison against version 0.1.0: {section}"
    );
}

/// The first release's fixture history, oldest revision first: a new capability
/// and work no user can see, the whole history tagged `v0.1.0`. Nothing earlier
/// exists, so the first version's section covers everything it contains.
pub(crate) const FIRST_RELEASE_HISTORY: [(&str, &str, &str); 3] = [
    ("2026-01-01", "chore(ci): bootstrap the release channel", ""),
    (
        "2026-01-02",
        "docs(readme): describe the download route",
        "",
    ),
    (
        "2026-01-03",
        "feat(export-duty-plan-calendar): Convert a supported duty plan into a calendar",
        "v0.1.0",
    ),
];

/// The non-`@e2e` change-log scenarios' fixture history, oldest revision first:
/// a first release (tagged `v0.1.0`) whose released revisions name a new
/// capability and work no user can see, then a second release (tagged `v0.2.0`)
/// whose released revisions name another new capability and more work no user
/// can see. Both sections are renderable from it, which is what the two
/// scenarios that share this step need.
const CAPABILITY_AND_INVISIBLE_HISTORY: [(&str, &str, &str); 6] = [
    ("2026-01-01", "chore(ci): bootstrap the release channel", ""),
    (
        "2026-01-02",
        "docs(readme): describe the download route",
        "",
    ),
    (
        "2026-01-03",
        "feat(export-duty-plan-calendar): Convert a supported duty plan into a calendar",
        "v0.1.0",
    ),
    ("2026-02-01", "chore(ci): tune the release pipeline", ""),
    (
        "2026-02-02",
        "feat(publish-public-release): Offer an archive per supported platform",
        "",
    ),
    (
        "2026-02-03",
        "test(publish-public-release): cover the refusal paths",
        "v0.2.0",
    ),
];

#[given("a history whose released revisions name a new capability and work no user can see")]
async fn a_history_with_a_capability_and_invisible_work(w: &mut DutyWorld) {
    build_history(w, &CAPABILITY_AND_INVISIBLE_HISTORY);
}

#[given("no change log exists yet")]
async fn no_change_log_exists_yet(w: &mut DutyWorld) {
    let path = change_log_path(w);
    assert!(
        !path.exists(),
        "a change log already exists at {}",
        path.display()
    );
    w.changelog = Some(path);
}

#[then("the change log holds one section, for version 0.1.0")]
async fn one_section_for_version_0_1_0(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the generation did not succeed: {}",
        w.command_transcript
    );
    let text = change_log(w);
    let headings: Vec<&str> = text
        .lines()
        .filter(|line| line.starts_with("## ["))
        .collect();
    assert_eq!(
        headings.len(),
        1,
        "the change log holds more than one section: {text}"
    );
    assert!(
        headings[0].starts_with("## [0.1.0]"),
        "the section is not version 0.1.0: {}",
        headings[0]
    );
}

#[then("that section names the new capability")]
async fn that_section_names_the_new_capability(w: &mut DutyWorld) {
    let section = section_of(&change_log(w), "0.1.0");
    assert!(
        section.contains("Convert a supported duty plan into a calendar"),
        "the section names no new capability: {section}"
    );
}

#[then("that section states no comparison against an earlier version")]
async fn that_section_states_no_comparison(w: &mut DutyWorld) {
    let section = section_of(&change_log(w), "0.1.0");
    assert!(
        !section.contains("compare/"),
        "the first version's section states a comparison against an earlier version: {section}"
    );
}

/// The untagged fixture: a history whose newest revision is the one the tag
/// will point at, and which carries no `v*` tag, so the section has to be
/// renderable before the tag exists.
const UNTAGGED_HISTORY: [(&str, &str, &str); 3] = [
    ("2026-03-01", "chore(ci): bootstrap the release channel", ""),
    (
        "2026-03-02",
        "docs(readme): describe the download route",
        "",
    ),
    (
        "2026-03-03",
        "feat(export-duty-plan-calendar): Convert a supported duty plan into a calendar",
        "",
    ),
];

#[given("a history whose newest revision names a new capability and carries its own date")]
async fn a_history_with_an_untagged_newest_revision(w: &mut DutyWorld) {
    build_history(w, &UNTAGGED_HISTORY);
}

#[given("the version is not tagged yet")]
async fn the_version_is_not_tagged_yet(w: &mut DutyWorld) {
    let repo = w
        .fixture_repo
        .clone()
        .expect("the scenario built no fixture repository");
    let output = Command::new("git")
        .arg("-C")
        .arg(&repo)
        .args(["tag", "--list", "v*"])
        .output()
        .expect("list the fixture repository's tags");
    assert!(
        output.status.success(),
        "git tag --list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let listed = String::from_utf8_lossy(&output.stdout);
    assert!(
        listed.trim().is_empty(),
        "the fixture repository already carries a v* tag: {listed}"
    );
}

#[then("that section states the date of the revision it will be tagged at")]
async fn section_states_the_untagged_revision_date(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the generation did not succeed: {}",
        w.command_transcript
    );
    let section = section_of(&change_log(w), "0.1.0");
    // The newest revision carries the fixture's fixed committer date, and the
    // tag will point at it, so the heading states that date rather than the
    // clock.
    assert!(
        section.starts_with("## [0.1.0] - 2026-03-03"),
        "the section does not state the newest revision's own date: {section}"
    );
}

/// The release-text scenario's fixture: the delta history with one released
/// change named by several revisions, which the release text must state once.
/// The capability scopes are ones the repository holds and the whole history is
/// the version released, so every revision here is released.
const RELEASE_TEXT_HISTORY: [(&str, &str, &str); 7] = [
    (
        "2026-01-01",
        "feat(export-duty-plan-calendar): Convert a supported duty plan into a calendar",
        "",
    ),
    ("2026-01-02", "chore(ci): tune the release pipeline", ""),
    (
        "2026-01-03",
        "docs(readme): describe the download route",
        "",
    ),
    (
        "2026-01-04",
        "fix(publish-public-release): Name the platform in the refusal",
        "",
    ),
    (
        "2026-01-05",
        "feat(export-duty-plan-calendar): Convert a supported duty plan into a calendar",
        "",
    ),
    (
        "2026-01-06",
        "test(publish-public-release): cover the refusal paths",
        "v0.2.0",
    ),
    ("2026-01-07", "Publish the notes by hand", ""),
];

#[given("one released change is named by several revisions")]
async fn one_released_change_is_named_by_several_revisions(w: &mut DutyWorld) {
    build_history(w, &RELEASE_TEXT_HISTORY);
}

#[then("the release text names the new capability before the fix")]
async fn release_text_names_capability_before_fix(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the preparation did not succeed: {}",
        w.command_transcript
    );
    let text = release_text(w);
    let capability = text
        .find("Convert a supported duty plan into a calendar")
        .unwrap_or_else(|| panic!("the release text names no new capability: {text}"));
    let fix = text
        .find("Name the platform in the refusal")
        .unwrap_or_else(|| panic!("the release text names no fix: {text}"));
    assert!(
        capability < fix,
        "the release text names the fix before the new capability: {text}"
    );
}

#[then("the release text names the released change once")]
async fn release_text_names_the_change_once(w: &mut DutyWorld) {
    let text = release_text(w);
    let occurrences = text
        .matches("- Convert a supported duty plan into a calendar")
        .count();
    assert_eq!(
        occurrences, 1,
        "the release text does not name the released change exactly once: {text}"
    );
}

#[then("the release text omits the work no user can see")]
async fn release_text_omits_the_invisible_work(w: &mut DutyWorld) {
    let text = release_text(w);
    for omitted in CHANGE_LOG_OMITTED {
        assert!(
            !text.contains(omitted),
            "the release text carries the invisible subject {omitted:?}: {text}"
        );
    }
}

#[then("the prepared release stages the change log for version 0.2.0")]
async fn stages_the_change_log_for_0_2_0(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the preparation did not succeed: {}",
        w.command_transcript
    );
    let path = staged_dir(w).join("CHANGELOG.md");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no staged change log at {}: {error}", path.display()));
    assert!(
        text.contains("## [0.2.0]"),
        "the staged change log holds no section for version 0.2.0: {text}"
    );
}

#[then("the staged change log holds the section the release text states")]
async fn staged_change_log_holds_the_section(w: &mut DutyWorld) {
    let path = staged_dir(w).join("CHANGELOG.md");
    let staged = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no staged change log at {}: {error}", path.display()));
    let section = section_of(&staged, "0.2.0");
    let text = release_text(w);
    assert_eq!(
        section.trim(),
        text.trim(),
        "the staged change log's section is not the release text: {staged}"
    );
}

/// The capability the fixture repository holds and the committed configuration
/// does not name: the subject of the allow-list refusal.
const UNLISTED_CAPABILITY: &str = "check-duty-plan";

#[given("a capability the repository holds that the configuration does not name")]
async fn a_capability_the_configuration_does_not_name(w: &mut DutyWorld) {
    build_history(w, &CAPABILITY_AND_INVISIBLE_HISTORY);
    let repo = w
        .fixture_repo
        .clone()
        .expect("the scenario built no fixture repository");
    let config =
        std::fs::read_to_string(repo.join("cliff.toml")).expect("read the committed configuration");
    assert!(
        !config.contains(UNLISTED_CAPABILITY),
        "the committed configuration already names {UNLISTED_CAPABILITY}"
    );
    std::fs::create_dir_all(repo.join("givn/specs").join(UNLISTED_CAPABILITY))
        .expect("create the unlisted capability directory");
}

#[then("the generation fails and names the capability missing from the configuration")]
async fn generation_names_the_missing_capability(w: &mut DutyWorld) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the generation did not fail: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript.contains(UNLISTED_CAPABILITY),
        "the failure does not name the capability missing from the configuration: {}",
        w.command_transcript
    );
}

#[given(regex = r"^a committed change log that holds no section for version (\S+)$")]
async fn a_committed_change_log_without_the_section(w: &mut DutyWorld, version: String) {
    two_release_history(w);
    let path = change_log_path(w);
    w.changelog = Some(path.clone());
    // The change log is committed before the version's tag, so a committed file
    // that stops at the previous release is the precondition the check refuses.
    let committed = "\
## [0.1.0] - 2026-01-02\n\
\n\
### New capabilities\n\
- Convert a supported plan into a calendar\n";
    std::fs::write(&path, committed).expect("write the committed change log");
    assert!(
        !committed.contains(&format!("## [{version}]")),
        "the committed change log already holds a section for version {version}"
    );
}

#[then(regex = r"^the check fails and names version (\S+)$")]
async fn the_check_fails_and_names_version(w: &mut DutyWorld, version: String) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the check did not fail: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript.contains(&format!("version {version}")),
        "the failure does not name version {version}: {}",
        w.command_transcript
    );
}

/// The subjects the history that changes each kind of path carries, so the
/// Then steps can read them back out of the section.
const CONVERTER_CHANGE: &str = "Print the route a reader can check";
const MACHINERY_CHANGE: &str = "Keep the pipeline's own notes out of a release";
const BOTH_CHANGE: &str = "Name a packaged change beside the pipeline";
const PACKAGING_CHANGE: &str = "Force the executable mode into the archive";

/// A revision the project releases, writing a path a user receives.
#[given("a history whose released revisions changed the converter, the release machinery, and both")]
async fn a_history_changing_each_kind_of_path(w: &mut DutyWorld) {
    build_history_with(
        w,
        &[
            (
                "2026-01-01",
                "fix(export-duty-plan-calendar): Print the route a reader can check",
                "",
            ),
            (
                "2026-01-02",
                "fix(publish-public-release): Keep the pipeline's own notes out of a release",
                "",
            ),
            (
                "2026-01-03",
                "fix(publish-public-release): Name a packaged change beside the pipeline",
                "v0.2.0",
            ),
        ],
        &[(1, &[MACHINERY]), (2, &[RECEIVED, MACHINERY])],
    );
}

/// A revision that changed only the packaging the configuration lists.
#[given("a history whose released revisions changed the packaging")]
async fn a_history_changing_the_packaging(w: &mut DutyWorld) {
    build_history_with(
        w,
        &[(
            "2026-01-01",
            "fix(publish-public-release): Force the executable mode into the archive",
            "v0.2.0",
        )],
        &[(0, &["scripts/prepare-release.sh"])],
    );
}

/// A fixture repository whose tree is missing a path the configuration lists.
#[given("a configured user-receiving path the repository does not hold")]
async fn a_missing_receiving_path(w: &mut DutyWorld) {
    build_history(
        w,
        &[(
            "2026-01-01",
            "fix(export-duty-plan-calendar): Print the route a reader can check",
            "v0.2.0",
        )],
    );
    let repo = w.fixture_repo.clone().expect("a fixture repository");
    std::fs::remove_file(repo.join("Cargo.lock")).expect("remove a listed path");
}

/// The section of version 0.2.0, asserting the render succeeded.
fn section_0_2_0(w: &DutyWorld) -> String {
    assert_eq!(
        w.exit_status,
        Some(0),
        "rendering version 0.2.0 did not succeed: {}",
        w.command_transcript
    );
    section_of(&change_log(w), "0.2.0")
}

#[then("the section for version 0.2.0 names the change to the converter")]
async fn section_names_the_converter_change(w: &mut DutyWorld) {
    let section = section_0_2_0(w);
    assert!(
        section.contains(CONVERTER_CHANGE),
        "the section does not name the converter change: {section}"
    );
}

#[then("the section for version 0.2.0 names the change that touched both")]
async fn section_names_the_both_paths_change(w: &mut DutyWorld) {
    let section = section_0_2_0(w);
    assert!(
        section.contains(BOTH_CHANGE),
        "the section does not name the revision that changed both: {section}"
    );
}

#[then("the section for version 0.2.0 names no change to the release machinery")]
async fn section_names_no_machinery_change(w: &mut DutyWorld) {
    let section = section_0_2_0(w);
    assert!(
        !section.contains(MACHINERY_CHANGE),
        "the section names a revision that changed only the machinery: {section}"
    );
}

#[then("the section for version 0.2.0 names the change to the packaging")]
async fn section_names_the_packaging_change(w: &mut DutyWorld) {
    let section = section_0_2_0(w);
    assert!(
        section.contains(PACKAGING_CHANGE),
        "the section does not name the packaging change: {section}"
    );
}

#[then("the generation fails and names the path missing from the repository")]
async fn generation_fails_naming_the_missing_path(w: &mut DutyWorld) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the generation did not fail: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript
            .contains("a configured user-receiving path is not in the repository"),
        "the failure does not state the reason: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript.contains("Cargo.lock"),
        "the failure does not name the missing path: {}",
        w.command_transcript
    );
}
