// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `reduce-distributed-executable-size`.
//!
//! One file per capability. The steps drive the real
//! `scripts/measure_size_lever.py` as a subprocess against the real project
//! revision. A step body is never empty.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cucumber::{given, then, when};

use crate::steps::measure_distributed_executable_size::{
    field, fixture_repo, init_repo, repository_root, scenario_dir,
};
use crate::steps::publish_public_release::run_command;
use crate::DutyWorld;

/// The `size-lever.md` the measurement wrote.
pub(crate) fn lever_report_text(w: &DutyWorld) -> String {
    let out = w
        .lever_out
        .as_ref()
        .expect("the scenario ran no size-lever measurement");
    let path = out.join("size-lever.md");
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// The `size-lever.json` the measurement wrote.
pub(crate) fn lever_json_text(w: &DutyWorld) -> String {
    let out = w
        .lever_out
        .as_ref()
        .expect("the scenario ran no size-lever measurement");
    let path = out.join("size-lever.json");
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

/// One row of the cumulative table for a lever.
pub(crate) fn lever_row<'a>(text: &'a str, lever: &str) -> Vec<&'a str> {
    text.lines()
        .find(|line| line.starts_with(&format!("| {lever} |")))
        .map(|line| line.split('|').map(str::trim).collect())
        .unwrap_or_else(|| panic!("the report carries no row for {lever:?}:\n{text}"))
}

/// A byte value from a labelled line.
pub(crate) fn lever_number(text: &str, prefix: &str) -> u64 {
    field(text, prefix)
        .split_whitespace()
        .next()
        .unwrap_or_else(|| panic!("no number after {prefix:?}"))
        .parse()
        .unwrap_or_else(|_| panic!("not a number after {prefix:?}"))
}

/// Build a host release executable with the landed profile and run it on an
/// unsupported source. The Windows executable cannot run on the Linux host;
/// the failure path is platform-independent, so the host build is the local
/// evidence and the Windows smoke job starts the target executable.
pub(crate) fn run_unsupported_source_on_host_lever(w: &mut DutyWorld) -> PathBuf {
    let target = scenario_dir(w).join("host-lever-target");
    let build = Command::new("cargo")
        .current_dir(repository_root())
        .env("CARGO_TARGET_DIR", &target)
        .args(["build", "--release", "--locked"])
        .output()
        .expect("build the host lever executable");
    assert!(
        build.status.success(),
        "the host lever build failed: {}",
        String::from_utf8_lossy(&build.stderr)
    );
    let executable = target.join("release").join("duty2cal");
    let source = scenario_dir(w).join("unsupported.pdf");
    fs::write(&source, "%PDF-1.4\nthis is not a supported roster\n%%EOF\n")
        .expect("write the unsupported source");
    let seeded = scenario_dir(w).join("unsupported.parse-error.txt");
    fs::write(&seeded, "previous diagnostic\n").expect("seed a previous diagnostic");
    let mut run = Command::new(&executable);
    run.arg(&source);
    run_command(w, &mut run);
    assert_ne!(
        w.exit_status,
        Some(0),
        "the lever executable accepted an unsupported source"
    );
    let diagnostics: Vec<PathBuf> = fs::read_dir(source.parent().expect("source parent"))
        .expect("read the scenario directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.contains(".parse-error"))
                && path != &seeded
        })
        .collect();
    assert_eq!(
        diagnostics.len(),
        1,
        "the lever executable did not write one new numbered diagnostic: {}",
        w.command_transcript
    );
    assert_eq!(
        fs::read_to_string(&seeded).expect("read the seeded diagnostic"),
        "previous diagnostic\n",
        "the seeded diagnostic was overwritten"
    );
    let diagnostic = fs::read_to_string(&diagnostics[0]).expect("read the diagnostic");
    let source_name = source
        .file_name()
        .expect("the source has a name")
        .to_string_lossy();
    assert!(
        diagnostic.contains(source_name.as_ref()),
        "the diagnostic does not name the source: {diagnostic}"
    );
    assert!(
        diagnostic.contains("Reason:"),
        "the diagnostic does not name the reason: {diagnostic}"
    );
    source
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
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn run_lever(w: &mut DutyWorld, levers: &str) {
    let repo = w.lever_repo.clone().unwrap_or_else(repository_root);
    let revision = w
        .lever_revision
        .clone()
        .expect("the scenario prepared no revision");
    let out = scenario_dir(w).join("lever");
    let mut command = Command::new("python3");
    command
        .current_dir(repository_root())
        .arg("scripts/measure_size_lever.py")
        .arg("--repo")
        .arg(&repo)
        .arg("--revision")
        .arg(&revision)
        .arg("--lever")
        .arg(levers)
        .arg("--out")
        .arg(&out);
    if let Some(published) = w.lever_published {
        command.arg("--published-size").arg(published.to_string());
    }
    for (key, value) in &w.env {
        command.env(key, value);
    }
    run_command(w, &mut command);
    w.lever_out = Some(out.clone());
    w.lever_report = Some(out.join("size-lever.md"));
    let status = w.exit_status.unwrap_or(-1);
    w.scenario_transcript
        .push_str(&format!("measurement exit status: {status}\n"));
}

fn assert_report_surface(w: &DutyWorld) {
    let text = lever_report_text(w);
    assert!(text.contains("## Failure outcome"));
    assert!(text.contains("Panic: the standard Rust panic message"));
    assert!(text.contains("Conversion failure: a typed failure"));
    let json = lever_json_text(w);
    assert!(json.contains("\"kind\": \"size-lever\""));
    assert!(json.contains("\"saved_bytes\""));
    assert!(json.contains("\"landed\""));
    assert!(json.contains("\"lever_executable\""));
}

#[given("a repository at the revision carrying the landed panic-abort lever")]
async fn a_repository_with_the_landed_lever(w: &mut DutyWorld) {
    let manifest = run_git(&repository_root(), &["show", "HEAD:Cargo.toml"]);
    assert!(
        manifest.contains("[profile.release]") && manifest.contains("panic = \"abort\""),
        "the landed commit does not carry panic = \"abort\""
    );
    w.lever_revision = Some("HEAD".to_string());
    w.lever_repo = None;
    scenario_dir(w);
}

#[given("a fixture revision that does not build")]
async fn a_fixture_revision_that_does_not_build(w: &mut DutyWorld) {
    init_repo(w, false);
    let repo = fixture_repo(w);
    let source = repo.join("support/src/lib.rs");
    let text = fs::read_to_string(&source).expect("read the fixture support source");
    fs::write(
        &source,
        format!("compile_error!(\"fixture_build_failure\");\n{text}"),
    )
    .expect("break the fixture build");
    run_git(&repo, &["add", "-A"]);
    run_git(&repo, &["commit", "-qm", "broken"]);
    w.lever_repo = Some(repo.clone());
    w.lever_revision = Some(run_git(&repo, &["rev-parse", "HEAD"]));
    scenario_dir(w);
}

#[given(regex = r#"^a published binary size of (\d+) B$"#)]
async fn a_published_binary_size(w: &mut DutyWorld, bytes: u64) {
    w.lever_published = Some(bytes);
}

#[when(regex = r#"^the maintainer measures the "([^"]+)" levers?$"#)]
async fn measures_the_lever(w: &mut DutyWorld, levers: String) {
    run_lever(w, &levers);
}

#[when(regex = r#"^the maintainer proposes the "([^"]+)" lever$"#)]
async fn proposes_the_lever(w: &mut DutyWorld, levers: String) {
    run_lever(w, &levers);
}

#[then("the lever is applied and the landed executable is smaller")]
async fn lever_applied_and_smaller(w: &mut DutyWorld) {
    let text = lever_report_text(w);
    assert_eq!(field(&text, "- Status: "), "applied");
    let baseline = lever_number(&text, "- Baseline (rebuild, unchanged): ");
    let row = lever_row(&text, "panic=abort");
    let lever = row[3].parse::<u64>().expect("lever bytes");
    let saved = row[4].parse::<i64>().expect("saved bytes");
    let landed = lever_number(&text, "- Landed (committed configuration): ");
    assert!(lever < baseline, "the lever build is not smaller: {text}");
    assert_eq!(
        saved,
        baseline as i64 - lever as i64,
        "the saved bytes are wrong: {text}"
    );
    assert_eq!(
        landed, lever,
        "the landed build differs from the accepted lever build: {text}"
    );
    assert_eq!(field(&text, "- Landed status: "), "smaller");
    assert_report_surface(w);
}

#[then("the published binary's size is reported separately")]
async fn published_reported_separately(w: &mut DutyWorld) {
    let text = lever_report_text(w);
    let published = field(&text, "- Published binary (file): ");
    assert!(
        published.ends_with("B (reported separately)"),
        "the published size is not reported separately: {text}"
    );
    if let Some(expected) = w.lever_published {
        assert_eq!(
            lever_number(&text, "- Published binary (file): "),
            expected,
            "the published size is not the supplied size: {text}"
        );
    }
    assert!(
        text.contains("- Baseline (rebuild, unchanged): "),
        "the report carries no rebuilt baseline: {text}"
    );
}

#[then("the baseline is a rebuild of the same revision with the unchanged configuration")]
async fn baseline_is_a_rebuild(w: &mut DutyWorld) {
    let text = lever_report_text(w);
    assert!(
        text.contains("- Baseline (rebuild, unchanged): "),
        "the report carries no rebuilt baseline: {text}"
    );
    let json = lever_json_text(w);
    assert!(json.contains("\"baseline_sha256\""));
}

#[then("the saved bytes are measured against the baseline, not the published binary's size")]
async fn saved_bytes_use_baseline(w: &mut DutyWorld) {
    let text = lever_report_text(w);
    let baseline = lever_number(&text, "- Baseline (rebuild, unchanged): ");
    let row = lever_row(&text, "panic=abort");
    let row_baseline = row[2].parse::<u64>().expect("row baseline");
    assert_eq!(
        row_baseline, baseline,
        "the saved bytes do not use the rebuilt baseline: {text}"
    );
    if let Some(published) = w.lever_published {
        assert_ne!(
            row_baseline, published,
            "the saved bytes use the published size as their baseline: {text}"
        );
    }
}

#[then("the lever is rejected with its measured comparison")]
async fn lever_rejected(w: &mut DutyWorld) {
    let text = lever_report_text(w);
    assert_eq!(field(&text, "- Status: "), "rejected");
    let row = lever_row(&text, "opt-level=s");
    let row_baseline = row[2].parse::<u64>().expect("row baseline");
    let lever = row[3].parse::<u64>().expect("lever bytes");
    let saved = row[4].parse::<i64>().expect("saved bytes");
    assert!(
        lever > row_baseline,
        "the rejected lever is not larger than its cumulative baseline: {text}"
    );
    assert!(
        saved < 0,
        "the rejected lever's saved bytes are not negative: {text}"
    );
    assert_report_surface(w);
}

#[then("the build configuration is unchanged")]
async fn build_configuration_unchanged(w: &mut DutyWorld) {
    let manifest = fs::read_to_string(repository_root().join("Cargo.toml"))
        .expect("read the project manifest");
    assert!(manifest.contains("panic = \"abort\""));
    assert!(!manifest.contains("opt-level = \"s\""));
    let text = lever_report_text(w);
    assert_eq!(field(&text, "- Status: "), "rejected");
}

#[then("the first lever is applied")]
async fn first_lever_applied(w: &mut DutyWorld) {
    let text = lever_report_text(w);
    let row = lever_row(&text, "panic=abort");
    assert_eq!(row[6], "applied");
}

#[then("the second lever is measured against the first lever's build")]
async fn second_lever_cumulative(w: &mut DutyWorld) {
    let text = lever_report_text(w);
    let first = lever_row(&text, "panic=abort");
    let second = lever_row(&text, "opt-level=s");
    assert_eq!(
        second[2], first[3],
        "the second lever's baseline is not the first lever's build: {text}"
    );
}

#[then("the second lever is rejected")]
async fn second_lever_rejected(w: &mut DutyWorld) {
    let text = lever_report_text(w);
    let row = lever_row(&text, "opt-level=s");
    assert_eq!(row[6], "rejected");
}

#[then(regex = r#"^the proposal is refused, naming the (.+) constraint$"#)]
async fn proposal_refused(w: &mut DutyWorld, constraint: String) {
    assert_ne!(w.exit_status, Some(0), "the proposal was not refused");
    assert!(
        w.command_transcript.contains(&constraint),
        "the refusal does not name {constraint:?}: {}",
        w.command_transcript
    );
}

#[then("the proposal is refused, naming the unsupported lever")]
async fn proposal_refused_unknown(w: &mut DutyWorld) {
    assert_ne!(w.exit_status, Some(0), "the proposal was not refused");
    assert!(
        w.command_transcript.contains("not a supported size lever"),
        "the refusal does not name the unsupported lever: {}",
        w.command_transcript
    );
}

#[then("no baseline is built")]
async fn no_baseline_built(w: &mut DutyWorld) {
    if let Some(report) = &w.lever_report {
        assert!(!report.exists(), "a report was written after a refusal");
    }
    assert!(
        !w.command_transcript.contains("Compiling") && !w.command_transcript.contains("Finished"),
        "a build ran after a refusal: {}",
        w.command_transcript
    );
}

#[then("the build failure is recorded with no delta")]
async fn build_failure_recorded(w: &mut DutyWorld) {
    assert_ne!(w.exit_status, Some(0), "the failed build was not reported");
    let text = lever_report_text(w);
    assert_eq!(field(&text, "- Status: "), "failed");
    assert!(text.contains("## No measurement"));
    assert!(!text.contains("## Cumulative"));
    let json = lever_json_text(w);
    assert!(json.contains("\"status\": \"failed\""));
    assert!(json.contains("\"builds\": []"));
}
