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
pub(crate) const SUPPORTED: [(&str, &str); 1] = [("windows-x86_64", "duty2cal.exe")];

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

/// The one supported platform: the archive the release offers.
pub(crate) fn supported_platform() -> &'static str {
    SUPPORTED[0].0
}

/// The executable name the supported platform uses.
pub(crate) fn supported_executable() -> &'static str {
    SUPPORTED[0].1
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

/// The bytes of the executable built for a platform: a fixed stand-in. The
/// local suite never runs the Windows binary, so it must not depend on the
/// host's build; the Windows smoke job is the execution evidence.
fn executable_bytes(platform: &str) -> Vec<u8> {
    format!("stand-in executable for {platform}\n").into_bytes()
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
        "the staged set is not the one Windows archive, its checksum, the release text and the change log"
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

#[given("no executable was built for the supported platform")]
async fn no_executable_built_for_the_supported_platform(w: &mut DutyWorld) {
    let dir = scenario_dir(w);
    let binaries = dir.join("binaries");
    let _ = std::fs::remove_dir_all(&binaries);
    std::fs::create_dir_all(&binaries).expect("create the empty binaries directory");
    let out = dir.join("out");
    w.binaries = Some(binaries);
    w.staged = Some(out);
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
    assert_eq!(
        archive,
        staged_archive(w, supported_platform()),
        "the verified archive is not the supported platform's archive"
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
#[given(
    "a history whose released revisions changed the converter, the release machinery, and both"
)]
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

/// A history whose only announced changes are two dependency updates. The
/// housekeeping revisions prove the group names no other change; the explicit
/// `Cargo.lock` override is what makes each dependency revision announceable.
const DEPENDENCY_ONLY_HISTORY: [(&str, &str, &str); 4] = [
    (
        "2026-01-01",
        "chore(deps): bump serde from 1.0.0 to 1.0.1",
        "",
    ),
    (
        "2026-01-02",
        "chore(deps): bump clap from 4.0.0 to 4.0.1",
        "",
    ),
    ("2026-01-03", "chore(ci): tune the release pipeline", ""),
    (
        "2026-01-04",
        "docs(readme): describe the download route",
        "v0.2.0",
    ),
];

/// Two revisions carrying the same dependency subject, beside a housekeeping
/// revision. The renderer's collapse must leave one entry.
const DEPENDENCY_COLLAPSE_HISTORY: [(&str, &str, &str); 3] = [
    (
        "2026-01-01",
        "chore(deps): bump serde from 1.0.0 to 1.0.1",
        "",
    ),
    (
        "2026-01-02",
        "chore(deps): bump serde from 1.0.0 to 1.0.1",
        "",
    ),
    (
        "2026-01-03",
        "chore(ci): tune the release pipeline",
        "v0.2.0",
    ),
];

/// A fix, a reversal, and two dependency updates, so the group order can be
/// read from the notes.
const DEPENDENCY_ORDER_HISTORY: [(&str, &str, &str); 4] = [
    (
        "2026-01-01",
        "fix(export-duty-plan-calendar): Name the platform in the refusal",
        "",
    ),
    (
        "2026-01-02",
        "revert(publish-public-release): Restore the previous download notice",
        "",
    ),
    (
        "2026-01-03",
        "chore(deps): bump serde from 1.0.0 to 1.0.1",
        "",
    ),
    (
        "2026-01-04",
        "chore(deps): bump clap from 4.0.0 to 4.0.1",
        "v0.2.0",
    ),
];

/// A converter fix beside a dependency update that changed only the CI
/// workflow. The update must be dropped, and its empty heading pruned.
const DEPENDENCY_CI_HISTORY: [(&str, &str, &str); 3] = [
    (
        "2026-01-01",
        "fix(export-duty-plan-calendar): Name the platform in the refusal",
        "",
    ),
    (
        "2026-01-02",
        "chore(deps): bump actions/checkout from 6 to 7",
        "",
    ),
    (
        "2026-01-03",
        "chore(ci): tune the release pipeline",
        "v0.2.0",
    ),
];

/// A dependency update that changed the release workflow, which decides what
/// an archive holds and is therefore announced.
const DEPENDENCY_RELEASE_WORKFLOW_HISTORY: [(&str, &str, &str); 2] = [
    (
        "2026-01-01",
        "chore(deps): bump actions/upload-artifact from 6 to 7",
        "",
    ),
    (
        "2026-01-02",
        "chore(ci): tune the release pipeline",
        "v0.2.0",
    ),
];

/// A capability-scoped revision that also changed the dependency graph, beside
/// an unscoped dependency update. The first stays in Fixes; the second is the
/// only member of Dependencies.
const DEPENDENCY_CAPABILITY_HISTORY: [(&str, &str, &str); 3] = [
    (
        "2026-01-01",
        "fix(export-duty-plan-calendar): Upgrade the PDF parser",
        "",
    ),
    (
        "2026-01-02",
        "chore(deps): bump serde from 1.0.0 to 1.0.1",
        "",
    ),
    (
        "2026-01-03",
        "chore(ci): tune the release pipeline",
        "v0.2.0",
    ),
];

/// A CI-only dependency update, a housekeeping revision and a document: no
/// entry survives the path filter, so the version is still refused.
const INVISIBLE_WITH_CI_DEPENDENCY_HISTORY: [(&str, &str, &str); 3] = [
    (
        "2026-01-01",
        "chore(deps): bump actions/checkout from 6 to 7",
        "",
    ),
    ("2026-01-02", "chore(ci): tune the release pipeline", ""),
    (
        "2026-01-03",
        "docs(readme): describe the download route",
        "v0.2.0",
    ),
];

/// The bullets under a `### <heading>` group, in order. A missing group has no
/// bullets.
fn group_of(text: &str, heading: &str) -> Vec<String> {
    let marker = format!("### {heading}");
    let mut inside = false;
    let mut entries = Vec::new();
    for line in text.lines() {
        if line == marker {
            inside = true;
            continue;
        }
        if inside && line.starts_with("### ") {
            break;
        }
        if inside && line.starts_with("## [") {
            break;
        }
        if inside {
            if let Some(entry) = line.strip_prefix("- ") {
                entries.push(entry.to_string());
            }
        }
    }
    entries
}

/// The index of the first note containing `needle`, case-insensitively.
fn position_of(notes: &[String], needle: &str) -> usize {
    let needle = needle.to_lowercase();
    notes
        .iter()
        .position(|note| note.to_lowercase().contains(&needle))
        .unwrap_or_else(|| panic!("the notes name no {needle:?}: {notes:?}"))
}

#[given("a history whose released revisions carry only dependency updates")]
async fn a_history_of_dependency_updates(w: &mut DutyWorld) {
    build_history_with(
        w,
        &DEPENDENCY_ONLY_HISTORY,
        &[(0, &["Cargo.lock"]), (1, &["Cargo.lock"])],
    );
}

#[then("the notes for version 0.2.0 name the dependency updates")]
async fn notes_name_the_dependency_updates(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the generation did not succeed: {}",
        w.command_transcript
    );
    let notes = change_log_notes(w).join("\n").to_lowercase();
    assert!(
        notes.contains("bump serde from 1.0.0 to 1.0.1"),
        "the notes name no serde update: {notes}"
    );
    assert!(
        notes.contains("bump clap from 4.0.0 to 4.0.1"),
        "the notes name no clap update: {notes}"
    );
}

#[then("the notes name no housekeeping revision")]
async fn notes_name_no_housekeeping_revision(w: &mut DutyWorld) {
    let notes = change_log_notes(w).join("\n");
    for omitted in ["tune the release pipeline", "describe the download route"] {
        assert!(
            !notes.contains(omitted),
            "the notes carry the housekeeping subject {omitted:?}: {notes}"
        );
    }
}

#[given("a history whose dependency updates repeat one subject")]
async fn a_history_with_repeated_dependency_subjects(w: &mut DutyWorld) {
    build_history_with(
        w,
        &DEPENDENCY_COLLAPSE_HISTORY,
        &[(0, &["Cargo.lock"]), (1, &["Cargo.toml"])],
    );
}

#[then("the notes name that dependency update once")]
async fn dependency_update_named_once(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the generation did not succeed: {}",
        w.command_transcript
    );
    let count = change_log_notes(w)
        .iter()
        .filter(|note| note.to_lowercase().contains("bump serde"))
        .count();
    assert_eq!(
        count, 1,
        "the notes name the repeated dependency update {count} times"
    );
}

#[then("the prepared release holds an archive for every supported platform")]
async fn prepared_release_holds_every_platform(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the preparation did not succeed: {}",
        w.command_transcript
    );
    for (platform, _) in SUPPORTED {
        let archive = staged_archive(w, platform);
        assert!(
            staged_dir(w).join(&archive).is_file(),
            "the prepared release holds no archive named {archive}"
        );
    }
}

#[given("a history whose released revisions include a fix, a reversal, and dependency updates")]
async fn a_history_with_a_reversal_and_dependency_updates(w: &mut DutyWorld) {
    build_history_with(
        w,
        &DEPENDENCY_ORDER_HISTORY,
        &[(2, &["Cargo.lock"]), (3, &["Cargo.lock"])],
    );
}

#[then("the notes name the fix before the dependency update")]
async fn fix_precedes_dependency_update(w: &mut DutyWorld) {
    let notes = change_log_notes(w);
    assert!(
        position_of(&notes, "Name the platform in the refusal") < position_of(&notes, "bump serde"),
        "the dependency update precedes the fix: {notes:?}"
    );
}

#[then("the notes name the reversal before the dependency update")]
async fn reversal_precedes_dependency_update(w: &mut DutyWorld) {
    let notes = change_log_notes(w);
    assert!(
        position_of(&notes, "Restore the previous download notice")
            < position_of(&notes, "bump serde"),
        "the dependency update precedes the reversal: {notes:?}"
    );
}

#[then("the notes name the dependency updates in history order")]
async fn dependency_updates_in_history_order(w: &mut DutyWorld) {
    let notes = change_log_notes(w);
    assert!(
        position_of(&notes, "bump serde") < position_of(&notes, "bump clap"),
        "the dependency updates are not in history order: {notes:?}"
    );
}

#[given("a history whose dependency updates changed only the project's own checks")]
async fn a_history_with_check_only_dependency_updates(w: &mut DutyWorld) {
    build_history_with(
        w,
        &DEPENDENCY_CI_HISTORY,
        &[(1, &[".github/workflows/ci.yml"])],
    );
}

#[then("the notes name the converter fix")]
async fn notes_name_the_converter_fix(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the generation did not succeed: {}",
        w.command_transcript
    );
    let notes = change_log_notes(w).join("\n");
    assert!(
        notes.contains("Name the platform in the refusal"),
        "the notes name no converter fix: {notes}"
    );
}

#[then("the notes name no dependency update")]
async fn notes_name_no_dependency_update(w: &mut DutyWorld) {
    let notes = change_log_notes(w).join("\n").to_lowercase();
    assert!(
        !notes.contains("bump actions/checkout"),
        "the notes carry a CI-only dependency update: {notes}"
    );
}

#[then("the notes name no Dependencies group")]
async fn notes_name_no_dependencies_group(w: &mut DutyWorld) {
    let rendered = change_log(w);
    assert!(
        !rendered.contains("### Dependencies"),
        "the notes carry an empty Dependencies heading: {rendered}"
    );
}

#[given("a history whose dependency update changed the release workflow")]
async fn a_history_with_a_release_workflow_dependency_update(w: &mut DutyWorld) {
    build_history_with(
        w,
        &DEPENDENCY_RELEASE_WORKFLOW_HISTORY,
        &[(0, &[".github/workflows/release.yml"])],
    );
}

#[then("the notes name the dependency update")]
async fn notes_name_the_dependency_update(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the generation did not succeed: {}",
        w.command_transcript
    );
    let notes = change_log_notes(w).join("\n").to_lowercase();
    assert!(
        notes.contains("bump actions/upload-artifact"),
        "the notes name no release-workflow dependency update: {notes}"
    );
}

#[given("a history whose released revisions name only work no user can see, including dependency updates that changed no path a user receives")]
async fn a_history_with_no_announceable_entries(w: &mut DutyWorld) {
    build_history_with(
        w,
        &INVISIBLE_WITH_CI_DEPENDENCY_HISTORY,
        &[(0, &[".github/workflows/ci.yml"])],
    );
}

#[given("a history whose dependency update carries a capability scope")]
async fn a_history_with_a_capability_scoped_dependency_update(w: &mut DutyWorld) {
    build_history_with(
        w,
        &DEPENDENCY_CAPABILITY_HISTORY,
        &[
            (0, &["crates/duty2cal-core/src/lib.rs", "Cargo.lock"]),
            (1, &["Cargo.lock"]),
        ],
    );
}

#[then("the notes name the capability-scoped update once")]
async fn capability_scoped_update_once(w: &mut DutyWorld) {
    let count = change_log_notes(w)
        .iter()
        .filter(|note| note.contains("Upgrade the PDF parser"))
        .count();
    assert_eq!(
        count, 1,
        "the capability-scoped update appears {count} times"
    );
}

#[then("the Dependencies group names only the update that carries no capability scope")]
async fn dependencies_group_holds_only_unscoped_updates(w: &mut DutyWorld) {
    let group = group_of(&change_log(w), "Dependencies").join("\n");
    assert!(
        group.to_lowercase().contains("bump serde"),
        "the Dependencies group names no unscoped update: {group}"
    );
    assert!(
        !group.contains("Upgrade the PDF parser"),
        "the Dependencies group repeats the capability-scoped update: {group}"
    );
}

#[then("the release text names the dependency updates")]
async fn release_text_names_the_dependency_updates(w: &mut DutyWorld) {
    let path = staged_dir(w).join("release-notes.md");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("no release text at {}: {error}", path.display()));
    let text = text.to_lowercase();
    assert!(
        text.contains("bump serde"),
        "the release text names no serde update: {text}"
    );
    assert!(
        text.contains("bump clap"),
        "the release text names no clap update: {text}"
    );
}

/// A capability-scoped revision that changed only a path no user receives,
/// beside a dependency update. The capability heading must be pruned even
/// though another group follows it.
const EMPTY_CAPABILITY_BEFORE_DEPENDENCY_HISTORY: [(&str, &str, &str); 3] = [
    (
        "2026-01-01",
        "feat(publish-public-release): Offer a dependency-only release",
        "",
    ),
    (
        "2026-01-02",
        "chore(deps): bump serde from 1.0.0 to 1.0.1",
        "",
    ),
    (
        "2026-01-03",
        "chore(ci): tune the release pipeline",
        "v0.2.0",
    ),
];

#[given("a history whose capability revision changed only a path no user receives, beside a dependency update that changed the dependency graph")]
async fn a_history_with_an_emptied_capability_group(w: &mut DutyWorld) {
    build_history_with(
        w,
        &EMPTY_CAPABILITY_BEFORE_DEPENDENCY_HISTORY,
        &[(0, &["docs/readme.md"]), (1, &["Cargo.lock"])],
    );
}

#[then("the notes name the dependency update in Dependencies")]
async fn notes_name_the_dependency_update_in_dependencies(w: &mut DutyWorld) {
    let group = group_of(&change_log(w), "Dependencies")
        .join("\n")
        .to_lowercase();
    assert!(
        group.contains("bump serde"),
        "the Dependencies group names no serde update: {group}"
    );
}

#[then("the notes name no New capabilities group")]
async fn notes_name_no_new_capabilities_group(w: &mut DutyWorld) {
    let rendered = change_log(w);
    assert!(
        !rendered.contains("### New capabilities"),
        "the notes carry an empty New capabilities heading: {rendered}"
    );
}

/// The scenario's gate fixture root: `gate/bin` holds the stub `gh`,
/// `gate/polls` holds the recorded pages and the call log.
fn gate_root(w: &mut DutyWorld) -> PathBuf {
    scenario_dir(w).join("gate")
}

/// One check-run fixture row.
fn check_run(suite: u64, name: &str, status: &str, conclusion: Option<&str>) -> String {
    let conclusion = conclusion
        .map(|value| format!(r#","conclusion":"{value}""#))
        .unwrap_or_default();
    format!(r#"{{"name":"{name}","status":"{status}"{conclusion},"check_suite":{{"id":{suite}}}}}"#)
}

/// One page object with its check runs.
fn check_page(checks: &[String]) -> String {
    format!(
        r#"{{"total_count":{},"check_runs":[{}]}}"#,
        checks.len(),
        checks.join(",")
    )
}

/// One slurped poll response, one or more pages.
fn check_poll(pages: &[String]) -> String {
    format!("[{}]", pages.join(","))
}

/// Write the stub `gh`, its poll files, the timeout and the optional failure
/// marker into the scenario's gate directory.
fn stub_check_runs(w: &mut DutyWorld, polls: &[String], timeout: u64, fail: bool) {
    let root = gate_root(w);
    let bin = root.join("bin");
    let poll_dir = root.join("polls");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&bin).expect("create the stub directory");
    std::fs::create_dir_all(&poll_dir).expect("create the poll directory");

    let stub = bin.join("gh");
    std::fs::write(
        &stub,
        r#"#!/usr/bin/env bash
set -euo pipefail
polls="$STUB_POLLS"
n=$(cat "$polls/calls" 2>/dev/null || echo 0)
n=$((n + 1))
printf '%s\n' "$n" > "$polls/calls"
printf '%s\n' "$*" >> "$polls/args.log"
if [ -e "$polls/fail" ]; then
  exit 1
fi
file="$polls/poll-$n.json"
[ -f "$file" ] || file="$polls/poll-last.json"
cat "$file"
"#,
    )
    .expect("write the stub gh");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mut permissions = std::fs::metadata(&stub)
            .expect("stat the stub")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&stub, permissions).expect("mark the stub executable");
    }

    for (index, poll) in polls.iter().enumerate() {
        std::fs::write(poll_dir.join(format!("poll-{}.json", index + 1)), poll)
            .expect("write a poll file");
    }
    if let Some(last) = polls.last() {
        std::fs::write(poll_dir.join("poll-last.json"), last).expect("write the repeating poll");
    }
    if fail {
        std::fs::write(poll_dir.join("fail"), "").expect("write the failure marker");
    }
    std::fs::write(root.join("timeout"), timeout.to_string()).expect("write the timeout");
}

/// Run the gate against the scenario's stub, recording stdout, stderr and the
/// exit status on the world.
fn run_gate(w: &mut DutyWorld) {
    let root = gate_root(w);
    let bin = root.join("bin");
    let poll_dir = root.join("polls");
    let timeout = std::fs::read_to_string(root.join("timeout"))
        .expect("read the gate timeout")
        .trim()
        .to_string();
    let mut command = Command::new("scripts/await-green-checks.sh");
    command
        .current_dir(repository_root())
        .env(
            "PATH",
            format!(
                "{}:{}",
                bin.display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        )
        .env("GH_TOKEN", "test")
        .env("STUB_POLLS", &poll_dir)
        .arg("--repo")
        .arg("kressnerd/duty2cal")
        .arg("--sha")
        .arg("0123456789abcdef0123456789abcdef01234567")
        .arg("--exclude-suite")
        .arg("99")
        .arg("--interval")
        .arg("0")
        .arg("--timeout")
        .arg(timeout);
    run_command(w, &mut command);
}

/// The stub's recorded call count.
fn gate_calls(w: &mut DutyWorld) -> usize {
    std::fs::read_to_string(gate_root(w).join("polls/calls"))
        .ok()
        .and_then(|text| text.trim().parse().ok())
        .unwrap_or(0)
}

/// The stub's recorded arguments, one invocation per line.
fn gate_args(w: &mut DutyWorld) -> String {
    std::fs::read_to_string(gate_root(w).join("polls/args.log")).unwrap_or_default()
}

#[given("a tagged revision whose checks are still running")]
async fn a_revision_with_running_checks(w: &mut DutyWorld) {
    let first = check_poll(&[
        check_page(&[
            check_run(7, "test", "completed", Some("success")),
            check_run(7, "coverage", "queued", None),
        ]),
        check_page(&[check_run(99, "publish", "in_progress", None)]),
    ]);
    let second = check_poll(&[check_page(&[
        check_run(7, "test", "completed", Some("success")),
        check_run(7, "coverage", "completed", Some("success")),
    ])]);
    stub_check_runs(w, &[first, second], 5, false);
}

#[when("the publication gate examines the tagged revision")]
async fn the_gate_examines_the_revision(w: &mut DutyWorld) {
    run_gate(w);
}

#[then("the gate reports the still-running checks")]
async fn gate_reports_the_running_checks(w: &mut DutyWorld) {
    assert!(
        w.command_transcript.contains("still running"),
        "the gate reported no running checks: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript.contains("coverage (queued)"),
        "the gate did not name the queued check: {}",
        w.command_transcript
    );
    assert!(
        !w.command_transcript.contains("publish (in_progress)"),
        "the gate treated its own suite's check as external: {}",
        w.command_transcript
    );
    assert!(gate_args(w).contains("filter=latest"));
}

#[then(
    "the gate reports the tagged revision ready only after every check has concluded successfully"
)]
async fn gate_reports_ready_after_conclusion(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the gate did not report the revision ready: {}",
        w.command_transcript
    );
    assert_eq!(gate_calls(w), 2, "the gate did not poll twice");
    assert!(
        w.command_transcript.contains("concluded"),
        "the gate did not report the concluded set: {}",
        w.command_transcript
    );
}

#[given("a tagged revision whose checks include a completed failure and a check still running")]
async fn a_revision_with_a_failure_and_a_running_check(w: &mut DutyWorld) {
    let poll = check_poll(&[check_page(&[
        check_run(7, "test", "completed", Some("failure")),
        check_run(7, "lint", "completed", Some("timed_out")),
        check_run(7, "coverage", "in_progress", None),
    ])]);
    stub_check_runs(w, &[poll], 5, false);
}

#[then("the gate refuses publication, naming the failed check")]
async fn gate_refuses_naming_the_failed_check(w: &mut DutyWorld) {
    assert_ne!(w.exit_status, Some(0), "the gate did not refuse");
    assert!(
        w.command_transcript.contains("test (failure)"),
        "the gate did not name the failed check: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript.contains("lint (timed_out)"),
        "the gate did not name the timed-out check: {}",
        w.command_transcript
    );
}

#[then("the gate does not wait for the check still running")]
async fn gate_does_not_wait_for_the_running_check(w: &mut DutyWorld) {
    assert_eq!(
        gate_calls(w),
        1,
        "the gate polled again after a completed failure"
    );
}

#[given("a tagged revision whose checks never conclude")]
async fn a_revision_whose_checks_never_conclude(w: &mut DutyWorld) {
    let poll = check_poll(&[check_page(&[check_run(7, "coverage", "queued", None)])]);
    stub_check_runs(w, &[poll], 1, false);
}

#[when("the publication gate examines the tagged revision until the elapsed limit")]
async fn the_gate_examines_until_the_limit(w: &mut DutyWorld) {
    run_gate(w);
}

#[then("the gate refuses publication, naming the still-running checks")]
async fn gate_refuses_naming_the_still_running_checks(w: &mut DutyWorld) {
    assert_ne!(w.exit_status, Some(0), "the gate did not refuse");
    assert!(
        w.command_transcript
            .contains("did not conclude within 1 second"),
        "the gate did not state the bound: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript.contains("coverage (queued)"),
        "the gate did not name the queued check: {}",
        w.command_transcript
    );
}

#[given("a tagged revision whose completed checks are one success, one skipped, and one neutral")]
async fn a_revision_with_success_skipped_and_neutral(w: &mut DutyWorld) {
    let poll = check_poll(&[check_page(&[
        check_run(7, "test", "completed", Some("success")),
        check_run(7, "dco", "completed", Some("skipped")),
        check_run(7, "actionlint", "completed", Some("neutral")),
    ])]);
    stub_check_runs(w, &[poll], 5, false);
}

#[then("the gate reports the tagged revision ready to publish")]
async fn gate_reports_the_revision_ready(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the gate refused an allowed conclusion set: {}",
        w.command_transcript
    );
    assert!(
        w.command_transcript
            .contains("1 successful, 2 skipped or neutral"),
        "the gate did not report the green set's composition: {}",
        w.command_transcript
    );
    assert_eq!(gate_calls(w), 1, "the gate polled more than once");
}

#[given("a tagged revision whose completed checks are one skipped and one neutral")]
async fn a_revision_with_only_allowed_non_successes(w: &mut DutyWorld) {
    let poll = check_poll(&[check_page(&[
        check_run(7, "dco", "completed", Some("skipped")),
        check_run(7, "actionlint", "completed", Some("neutral")),
        check_run(99, "publish", "completed", Some("success")),
    ])]);
    stub_check_runs(w, &[poll], 5, false);
}

#[then("the gate refuses publication, stating that no check concluded successfully")]
async fn gate_refuses_for_no_success(w: &mut DutyWorld) {
    assert_ne!(w.exit_status, Some(0), "the gate did not refuse");
    assert!(
        w.command_transcript
            .contains("no check on the tagged revision concluded successfully"),
        "the gate did not state the missing success: {}",
        w.command_transcript
    );
}

#[given("a tagged revision whose check runs cannot be read")]
async fn a_revision_whose_checks_cannot_be_read(w: &mut DutyWorld) {
    stub_check_runs(w, &[], 5, true);
}

#[then("the gate refuses publication, stating that the check runs could not be read")]
async fn gate_refuses_for_unreadable_checks(w: &mut DutyWorld) {
    assert_ne!(w.exit_status, Some(0), "the gate did not refuse");
    assert!(
        w.command_transcript
            .contains("could not read the check runs"),
        "the gate did not state the read failure: {}",
        w.command_transcript
    );
}

#[given("a tagged revision whose only check runs belong to the release workflow")]
async fn a_revision_with_only_the_release_workflow(w: &mut DutyWorld) {
    let poll = check_poll(&[check_page(&[check_run(
        99,
        "publish",
        "completed",
        Some("success"),
    )])]);
    stub_check_runs(w, &[poll], 5, false);
}

#[then("the gate refuses publication, stating that the revision carries no check evidence")]
async fn gate_refuses_for_no_check_evidence(w: &mut DutyWorld) {
    assert_ne!(w.exit_status, Some(0), "the gate did not refuse");
    assert!(
        w.command_transcript.contains("no check evidence"),
        "the gate did not state the missing evidence: {}",
        w.command_transcript
    );
}

#[given("the default branch and a version tag pushed together, each starting a check suite on the same revision")]
async fn branch_and_tag_pushed_together_with_two_suites(w: &mut DutyWorld) {
    let first = check_poll(&[check_page(&[
        check_run(7, "test", "completed", Some("success")),
        check_run(8, "test", "in_progress", None),
        check_run(99, "publish", "in_progress", None),
    ])]);
    let second = check_poll(&[check_page(&[
        check_run(7, "test", "completed", Some("success")),
        check_run(8, "test", "completed", Some("success")),
    ])]);
    stub_check_runs(w, &[first, second], 5, false);
}

#[then("the gate waits for both suites' checks instead of refusing them as not yet concluded")]
async fn gate_waits_for_both_suites(w: &mut DutyWorld) {
    assert!(
        w.command_transcript.contains("test (in_progress)"),
        "the gate did not wait for the tag suite's check: {}",
        w.command_transcript
    );
    assert!(
        !w.command_transcript.contains("publish (in_progress)"),
        "the gate listed its own suite's check: {}",
        w.command_transcript
    );
    assert_eq!(gate_calls(w), 2, "the gate did not poll both suites");
}

#[then("the gate reports the revision ready to publish only once every check in both suites has concluded successfully")]
async fn gate_ready_only_once_both_suites_concluded(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the gate refused the two-suite revision: {}",
        w.command_transcript
    );
    assert!(
        !w.command_transcript.contains("failure"),
        "the gate reported a failure after both suites concluded: {}",
        w.command_transcript
    );
}
