// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `publish-public-release`.
//!
//! One file per capability. The steps drive the real preparation command
//! (`scripts/prepare-release.sh`) as a subprocess, as the conversion
//! capability's steps drive the built binary. RED-phase steps use the
//! `unimplemented!("<step>")` stub; a step body is never empty.
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
/// housekeeping subjects, and one subject that carries no type at all.
const HISTORY: [&str; 8] = [
    "chore(release): bootstrap the release channel",
    "feat(release): Offer an archive for every supported platform",
    "docs(release): describe the download routes for readers",
    "feat(release): Offer an archive for every supported platform",
    "fix(release): Publish only an archive the evidence supports",
    "Publish the notes by hand",
    "test(release): cover the refusal paths",
    "feat(release): Offer an archive for every supported platform",
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

/// The repository's preparation command.
fn prepare_script() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scripts/prepare-release.sh")
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

/// Run a command, recording its transcript and exit status.
pub(crate) fn run_command(w: &mut DutyWorld, command: &mut Command) {
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
    let mut command = Command::new(prepare_script());
    command
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
fn sha256_of(path: &Path) -> String {
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
    // The staged set is exactly the archives, their checksums and the release
    // text — no other archive in it.
    expected.push("release-notes.md".to_string());
    expected.sort();
    assert_eq!(
        entries(&out),
        expected,
        "the staged set is not the five archives, their checksums and the release text"
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
    let repo = scenario_dir(w).join("history");
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(&repo).expect("create the fixture repository");
    git(&repo, &["init", "--quiet"]);
    for (index, subject) in HISTORY.iter().enumerate() {
        let file = repo.join(format!("revision-{index}.txt"));
        std::fs::write(&file, format!("{subject}\n")).expect("write the revision's file");
        git(&repo, &["add", "--all"]);
        git(
            &repo,
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
    }
    w.fixture_repo = Some(repo);
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
    let text = release_text(w);
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
    let text = release_text(w);
    for omitted in OMITTED {
        assert!(
            !text.contains(omitted),
            "the release text carries the omitted subject {omitted:?}: {text}"
        );
    }
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
