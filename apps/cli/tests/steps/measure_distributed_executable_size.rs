// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `measure-distributed-executable-size`.
//!
//! One file per capability. The steps drive the real
//! `scripts/measure_executable_size.py` as a subprocess against a real git
//! repository and a real cross-build. A step body is never empty: RED writes
//! each new step as a stub that panics with its own name.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cucumber::{given, then, when};

use crate::steps::publish_public_release::run_command;
use crate::DutyWorld;

/// The repository root: the working directory the measurement is invoked
/// from, so the command line shows `scripts/measure_executable_size.py`.
pub(crate) fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The committed fixture workspace.
pub(crate) fn fixture_source() -> PathBuf {
    repository_root().join("apps/cli/tests/fixtures/size-attribution")
}

/// Fresh per-scenario directory.
fn fresh_dir() -> PathBuf {
    let mut attempt = 0u32;
    loop {
        let path = std::env::temp_dir().join(format!(
            "duty2cal-measure-{}-{attempt}",
            std::process::id()
        ));
        if path.exists() {
            attempt += 1;
            continue;
        }
        fs::create_dir_all(&path).expect("create the scenario directory");
        return path;
    }
}

/// The scenario's working directory, created on first use.
pub(crate) fn scenario_dir(w: &mut DutyWorld) -> PathBuf {
    if w.dir.is_none() {
        w.dir = Some(fresh_dir());
    }
    w.working_dir()
}

fn run_git(repo: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run git {args:?}: {error}"));
    assert!(
        output.status.success(),
        "git {args:?} failed in {}: {}",
        repo.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn copy_dir_all(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create the fixture copy");
    for entry in fs::read_dir(source).expect("read the fixture directory") {
        let entry = entry.expect("read a fixture entry");
        let target = destination.join(entry.file_name());
        if entry.file_type().expect("fixture entry type").is_dir() {
            copy_dir_all(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).expect("copy a fixture file");
        }
    }
}

fn replace_in_file(path: &Path, from: &str, to: &str) {
    let text = fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assert!(
        text.contains(from),
        "the fixture file {} does not contain {from:?}",
        path.display()
    );
    fs::write(path, text.replace(from, to))
        .unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
}

/// Build the fixture repository: one commit, tagged `v0.3.0`, optionally
/// followed by a `newest` commit that replaces `fixture_alpha` with
/// `fixture_beta`.
pub(crate) fn init_repo(w: &mut DutyWorld, with_newest: bool) {
    let repo = scenario_dir(w).join("repo");
    copy_dir_all(&fixture_source(), &repo);
    run_git(&repo, &["init", "-q"]);
    run_git(&repo, &["config", "user.email", "fixture@example.com"]);
    run_git(&repo, &["config", "user.name", "Fixture"]);
    run_git(&repo, &["add", "-A"]);
    run_git(&repo, &["commit", "-qm", "base"]);
    let base = run_git(&repo, &["rev-parse", "HEAD"]);
    if with_newest {
        replace_in_file(
            &repo.join("support/src/lib.rs"),
            "fixture_alpha",
            "fixture_beta",
        );
        replace_in_file(&repo.join("app/src/main.rs"), "fixture_alpha", "fixture_beta");
        run_git(&repo, &["add", "-A"]);
        run_git(&repo, &["commit", "-qm", "newest"]);
    }
    w.fixture_repo = Some(repo);
    w.measurement_revision = Some(base);
}

/// The fixture repository a scenario prepared.
pub(crate) fn fixture_repo(w: &DutyWorld) -> PathBuf {
    w.fixture_repo
        .clone()
        .expect("the scenario prepared no fixture repository")
}

/// Build the fixture for the supported platform and return the executable.
pub(crate) fn build_fixture_for_target(repo: &Path, target_dir: &Path) -> PathBuf {
    let output = Command::new("cargo")
        .current_dir(repo)
        .env("CARGO_TARGET_DIR", target_dir)
        .args([
            "build",
            "--release",
            "--locked",
            "--target",
            "x86_64-pc-windows-gnu",
        ])
        .output()
        .expect("build the fixture for the supported platform");
    assert!(
        output.status.success(),
        "the fixture did not build for x86_64-pc-windows-gnu: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    target_dir
        .join("x86_64-pc-windows-gnu")
        .join("release")
        .join("fixture-app.exe")
}

/// Build the fixture for the host and return the host executable.
fn build_fixture_for_host(repo: &Path, target_dir: &Path) -> PathBuf {
    let output = Command::new("cargo")
        .current_dir(repo)
        .env("CARGO_TARGET_DIR", target_dir)
        .args(["build", "--release", "--locked"])
        .output()
        .expect("build the fixture for the host");
    assert!(
        output.status.success(),
        "the fixture did not build for the host: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    target_dir.join("release").join("fixture-app")
}

fn preflight(need_generator: bool) {
    let rustc = Command::new("rustc")
        .args(["--print", "target-list"])
        .output()
        .expect("run rustc --print target-list");
    assert!(
        rustc.status.success()
            && String::from_utf8_lossy(&rustc.stdout).contains("x86_64-pc-windows-gnu"),
        "the x86_64-pc-windows-gnu target is not installed"
    );
    let linker = Command::new("sh")
        .args(["-c", "command -v x86_64-w64-mingw32-gcc"])
        .output()
        .expect("look for the mingw linker");
    assert!(
        linker.status.success(),
        "x86_64-w64-mingw32-gcc is not on PATH"
    );
    if need_generator {
        let generator = Command::new("sh")
            .args(["-c", "command -v cargo-bloat"])
            .output()
            .expect("look for cargo-bloat");
        assert!(generator.status.success(), "cargo-bloat is not on PATH");
    }
}

/// Run the measurement command and record its output in the world.
pub(crate) fn run_measurement(
    w: &mut DutyWorld,
    revision: &str,
    executable: Option<&Path>,
) {
    let repo = fixture_repo(w);
    let out = scenario_dir(w).join("out");
    let mut command = Command::new("python3");
    command
        .current_dir(repository_root())
        .arg("scripts/measure_executable_size.py")
        .arg("--repo")
        .arg(&repo)
        .arg("--revision")
        .arg(revision)
        .arg("--out")
        .arg(&out);
    if let Some(executable) = executable {
        command.arg("--executable").arg(executable);
        w.measurement_executable = Some(executable.to_path_buf());
    }
    for (key, value) in &w.env {
        command.env(key, value);
    }
    run_command(w, &mut command);
    w.measurement_out = Some(out.clone());
    w.attribution = Some(out.join("attribution.json"));
}

/// The `attribution.md` the measurement wrote.
pub(crate) fn report_text(w: &DutyWorld) -> String {
    let out = w
        .measurement_out
        .as_ref()
        .expect("the scenario ran no measurement");
    let path = out.join("attribution.md");
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// The value after a `- Label: ` line.
pub(crate) fn field<'a>(text: &'a str, prefix: &str) -> &'a str {
    text.lines()
        .find_map(|line| line.strip_prefix(prefix))
        .unwrap_or_else(|| panic!("the report carries no {prefix:?} line:\n{text}"))
        .trim()
}

/// A section's text, ending before the next `## ` heading.
pub(crate) fn section<'a>(text: &'a str, heading: &str) -> &'a str {
    let start = text
        .find(heading)
        .unwrap_or_else(|| panic!("the report carries no {heading:?} section:\n{text}"));
    let rest = &text[start + heading.len()..];
    let end = rest.find("\n## ").map(|index| index + 1).unwrap_or(rest.len());
    &rest[..end]
}

pub(crate) fn is_hex(value: &str, length: usize) -> bool {
    value.len() == length && value.chars().all(|character| character.is_ascii_hexdigit())
}

/// The first number in a labelled line, in bytes.
pub(crate) fn number_after(text: &str, prefix: &str) -> u64 {
    let value = field(text, prefix);
    let number = value
        .split_whitespace()
        .next()
        .unwrap_or_else(|| panic!("no number after {prefix:?}: {value:?}"));
    number
        .parse()
        .unwrap_or_else(|_| panic!("{number:?} is not a number after {prefix:?}"))
}

fn assert_no_attribution(text: &str) {
    assert!(
        text.contains("## No attribution"),
        "the report does not state that no attribution was produced:\n{text}"
    );
    assert!(
        !text.contains("## The two numbers"),
        "an unreadable report must not carry the two-number section:\n{text}"
    );
}

#[given(regex = r#"^a repository whose earlier revision "([^"]+)" carries the function "([^"]+)" and whose newest revision carries "([^"]+)" instead$"#)]
async fn a_repository_with_an_earlier_revision(w: &mut DutyWorld, tag: String, older: String, newer: String) {
    assert_eq!(tag, "v0.3.0", "the scenario names the earlier revision");
    assert_eq!(older, "fixture_alpha");
    assert_eq!(newer, "fixture_beta");
    init_repo(w, true);
    let repo = fixture_repo(w);
    let base = w
        .measurement_revision
        .clone()
        .expect("the scenario prepared no earlier revision");
    run_git(&repo, &["tag", tag.as_str(), base.as_str()]);
}

#[given("a repository whose revision builds for the supported platform")]
async fn a_repository_whose_revision_builds(w: &mut DutyWorld) {
    init_repo(w, false);
}

#[given(regex = r#"^that revision is tagged "([^"]+)"$"#)]
async fn that_revision_is_tagged(w: &mut DutyWorld, tag: String) {
    let repo = fixture_repo(w);
    let revision = w
        .measurement_revision
        .clone()
        .expect("the scenario prepared no revision");
    run_git(&repo, &["tag", &tag, &revision]);
}

#[given("a revision that does not build")]
async fn a_revision_that_does_not_build(w: &mut DutyWorld) {
    init_repo(w, false);
    let repo = fixture_repo(w);
    let path = repo.join("support/src/lib.rs");
    let text = fs::read_to_string(&path).expect("read the fixture support source");
    fs::write(
        &path,
        format!("compile_error!(\"fixture_compile_error\");\n{text}"),
    )
    .expect("break the fixture build");
    run_git(&repo, &["add", "-A"]);
    run_git(&repo, &["commit", "-qm", "broken"]);
    w.measurement_revision = Some(run_git(&repo, &["rev-parse", "HEAD"]));
}

#[given("a revision whose build keeps no symbol names")]
async fn a_revision_that_strips_names(w: &mut DutyWorld) {
    init_repo(w, false);
    let repo = fixture_repo(w);
    let path = repo.join(".cargo/config.toml");
    replace_in_file(
        &path,
        "\"-C\", \"target-feature=+crt-static\"",
        "\"-C\", \"target-feature=+crt-static\", \"-C\", \"link-arg=-s\"",
    );
    run_git(&repo, &["add", "-A"]);
    run_git(&repo, &["commit", "-qm", "strips names"]);
    w.measurement_revision = Some(run_git(&repo, &["rev-parse", "HEAD"]));
}

#[given("an executable built for the host machine")]
async fn an_executable_built_for_the_host(w: &mut DutyWorld) {
    init_repo(w, false);
    let repo = fixture_repo(w);
    let target = scenario_dir(w).join("host-target");
    let executable = build_fixture_for_host(&repo, &target);
    w.measurement_executable = Some(executable);
}

#[given("a release archive presented as the executable")]
async fn a_release_archive_presented_as_the_executable(w: &mut DutyWorld) {
    init_repo(w, false);
    let repo = fixture_repo(w);
    let target = scenario_dir(w).join("archive-target");
    let executable = build_fixture_for_target(&repo, &target);
    let archive = scenario_dir(w).join("fixture-app.tar.gz");
    let output = Command::new("tar")
        .arg("czf")
        .arg(&archive)
        .arg("-C")
        .arg(executable.parent().expect("the executable has a parent"))
        .arg(executable.file_name().expect("the executable has a name"))
        .output()
        .expect("archive the fixture executable");
    assert!(
        output.status.success(),
        "tar could not archive the fixture executable: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    w.measurement_executable = Some(archive);
}

#[given("the supported platform's executable at the named path cannot be read")]
async fn the_executable_cannot_be_read(w: &mut DutyWorld) {
    init_repo(w, false);
    w.measurement_executable = Some(scenario_dir(w).join("missing.exe"));
}

#[given(regex = r#"^a repository whose history does not contain the revision "([^"]+)"$"#)]
async fn a_repository_without_a_revision(w: &mut DutyWorld, revision: String) {
    assert_eq!(revision, "v9.9.9");
    init_repo(w, false);
}

#[given(regex = r#"^"([^"]+)" is not available on the host$"#)]
async fn a_tool_is_not_available(w: &mut DutyWorld, tool: String) {
    assert_eq!(tool, "objdump");
    let bin = scenario_dir(w).join("bin");
    fs::create_dir_all(&bin).expect("create the shim directory");
    let shim = bin.join("x86_64-w64-mingw32-objdump");
    fs::write(&shim, "#!/bin/sh\nexit 1\n").expect("write the objdump shim");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mut permissions = fs::metadata(&shim)
            .expect("stat the objdump shim")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&shim, permissions).expect("mark the objdump shim executable");
    }
    let existing = std::env::var("PATH").unwrap_or_default();
    w.env.push((
        "PATH".to_string(),
        format!("{}:{existing}", bin.display()),
    ));
}

#[when(regex = r#"^the maintainer requests the size attribution for "([^"]+)"$"#)]
async fn requests_for_a_named_revision(w: &mut DutyWorld, revision: String) {
    preflight(true);
    run_measurement(w, &revision, None);
}

#[when("the maintainer requests the size attribution for that revision")]
async fn requests_for_that_revision(w: &mut DutyWorld) {
    preflight(true);
    let revision = w
        .measurement_revision
        .clone()
        .expect("the scenario prepared no revision");
    run_measurement(w, &revision, None);
}

#[when("the maintainer requests the size attribution for the revision, describing that artifact")]
async fn requests_describing_an_artifact(w: &mut DutyWorld) {
    preflight(false);
    let revision = w
        .measurement_revision
        .clone()
        .expect("the scenario prepared no revision");
    let executable = w
        .measurement_executable
        .clone()
        .expect("the scenario prepared no artifact");
    run_measurement(w, &revision, Some(&executable));
}

#[when("the maintainer requests the size attribution for the revision, describing that executable")]
async fn requests_describing_an_executable(w: &mut DutyWorld) {
    preflight(true);
    let revision = w
        .measurement_revision
        .clone()
        .expect("the scenario prepared no revision");
    let executable = w
        .measurement_executable
        .clone()
        .expect("the scenario prepared no executable");
    run_measurement(w, &revision, Some(&executable));
}

#[then(regex = r#"^the report names the requested revision "([^"]+)" and a resolved commit$"#)]
async fn names_requested_and_resolved(w: &mut DutyWorld, revision: String) {
    let text = report_text(w);
    assert_eq!(field(&text, "- Requested: "), revision);
    assert!(
        is_hex(field(&text, "- Revision: "), 40),
        "the report does not name a resolved 40-hex commit:\n{text}"
    );
}

#[then(regex = r#"^the report names the requested revision "([^"]+)"$"#)]
async fn names_requested(w: &mut DutyWorld, revision: String) {
    let text = report_text(w);
    assert_eq!(field(&text, "- Requested: "), revision);
}

#[then("the attribution names the resolved commit as the revision it describes")]
async fn names_resolved_as_described(w: &mut DutyWorld) {
    let text = report_text(w);
    assert!(
        is_hex(field(&text, "- Revision: "), 40),
        "the report does not name a resolved 40-hex commit:\n{text}"
    );
    assert_eq!(field(&text, "- Requested: "), "v0.3.0");
}

#[then(regex = r#"^the attribution names the function "([^"]+)"$"#)]
async fn names_function(w: &mut DutyWorld, function: String) {
    let text = report_text(w);
    assert!(
        text.contains(&function),
        "the attribution does not name {function:?}:\n{text}"
    );
}

#[then(regex = r#"^the attribution names no function "([^"]+)"$"#)]
async fn names_no_function(w: &mut DutyWorld, function: String) {
    let text = report_text(w);
    assert!(
        !text.contains(&function),
        "the attribution names {function:?} but should not:\n{text}"
    );
}

#[then("the report names the build as unreadable")]
async fn names_build_unreadable(w: &mut DutyWorld) {
    let text = report_text(w);
    assert_eq!(field(&text, "- Status: "), "unreadable");
}

#[then(regex = r#"^the report's reason names the build's "([^"]+)" failure$"#)]
async fn reason_names_failure(w: &mut DutyWorld, marker: String) {
    let text = report_text(w);
    let reason = section(&text, "## No attribution");
    assert!(
        reason.contains(&marker),
        "the reason does not name {marker:?}:\n{text}"
    );
}

#[then("the report states no attribution")]
async fn states_no_attribution(w: &mut DutyWorld) {
    assert_no_attribution(&report_text(w));
}

#[then("the report names the missing capability")]
async fn names_missing_capability(w: &mut DutyWorld) {
    let text = report_text(w);
    assert!(
        text.contains("cannot retain the names"),
        "the report does not name the missing capability:\n{text}"
    );
}

#[then(regex = r#"^the measurement is refused, naming "([^"]+)"$"#)]
async fn measurement_refused(w: &mut DutyWorld, named: String) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the measurement was not refused: {}",
        w.command_transcript
    );
    let out = w
        .measurement_out
        .as_ref()
        .expect("the scenario ran no measurement");
    let text = if out.join("attribution.md").is_file() {
        report_text(w)
    } else {
        w.command_transcript.clone()
    };
    assert!(
        text.contains(&named),
        "the refusal does not name {named:?}:\n{text}"
    );
}

#[then("the report exists and states that no attribution was produced")]
async fn report_exists_without_attribution(w: &mut DutyWorld) {
    let out = w
        .measurement_out
        .as_ref()
        .expect("the scenario ran no measurement");
    assert!(
        out.join("attribution.md").is_file(),
        "the refusal wrote no attribution.md"
    );
    assert_no_attribution(&report_text(w));
}

#[then("the attribution states that the executable's file size is unavailable")]
async fn file_size_unavailable(w: &mut DutyWorld) {
    let text = report_text(w);
    let line = field(&text, "- Stripped executable (file): ");
    assert!(
        line.starts_with("unavailable"),
        "the file-size line is {line:?}, not unavailable:\n{text}"
    );
}

#[then("the attribution's total is not presented as the executable's file size")]
async fn total_not_presented_as_file_size(w: &mut DutyWorld) {
    let text = report_text(w);
    let line = field(&text, "- Stripped executable (file): ");
    assert!(
        line.starts_with("unavailable"),
        "the file-size line is {line:?}:\n{text}"
    );
    assert!(
        text.contains("- Size attribution (attribution build .text): "),
        "the report does not carry the attribution total:\n{text}"
    );
}

#[then("no report is written")]
async fn no_report_written(w: &mut DutyWorld) {
    let out = w
        .measurement_out
        .as_ref()
        .expect("the scenario ran no measurement");
    assert!(
        !out.join("attribution.md").exists(),
        "a report was written for an unresolved revision"
    );
}

#[then("the report lists the stripped executable's sections")]
async fn lists_sections(w: &mut DutyWorld) {
    let text = report_text(w);
    assert!(
        text.contains("## Sections of the stripped executable"),
        "the report carries no section table:\n{text}"
    );
}

#[then(regex = r#"^the section table names the "([^"]+)" section$"#)]
async fn section_table_names(w: &mut DutyWorld, section_name: String) {
    let text = report_text(w);
    assert!(
        text.contains(&format!("| {section_name} |")),
        "the section table does not name {section_name:?}:\n{text}"
    );
}

#[then("the report states that the section sizes could not be read")]
async fn section_sizes_unread(w: &mut DutyWorld) {
    let text = report_text(w);
    assert!(
        text.contains("The section sizes could not be read."),
        "the report does not state that the section sizes could not be read:\n{text}"
    );
}

#[then("the attribution still states the stripped executable's file size")]
async fn still_states_file_size(w: &mut DutyWorld) {
    let text = report_text(w);
    let line = field(&text, "- Stripped executable (file): ");
    assert!(
        !line.starts_with("unavailable") && line.contains(" B"),
        "the file size is not stated: {line:?}\n{text}"
    );
}
