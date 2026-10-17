// SPDX-License-Identifier: MPL-2.0
//! Gherkin runner (see design.md "Test runner, strict mode, and single-scenario
//! command"). Strict by construction: `fail_on_skipped()` makes an undefined or
//! skipped step fail the run.

use std::path::PathBuf;
use std::time::SystemTime;

use cucumber::World as _;

mod steps;

#[derive(Debug, Default, cucumber::World)]
pub struct DutyWorld {
    /// Per-scenario working directory.
    pub dir: Option<PathBuf>,
    /// The source file supplied for conversion.
    pub source: Option<PathBuf>,
    /// Exit status of the last child invocation, when one was run.
    pub exit_status: Option<i32>,
    /// Captured standard output of the last child invocation.
    pub stdout: String,
    /// Captured standard error of the last child invocation.
    pub stderr: String,
    /// Calendar lines the current scenario expects to find.
    pub expected: Vec<String>,
    /// Calendars produced by the current scenario, in order.
    pub calendars: Vec<String>,
    /// Environment variables passed to the converter subprocess.
    pub env: Vec<(String, String)>,
    /// Executables directory the release preparation reads: one subdirectory
    /// per supported platform.
    pub binaries: Option<PathBuf>,
    /// Directory the release preparation stages the release into.
    pub staged: Option<PathBuf>,
    /// Fixture repository whose history supplies the release text.
    pub fixture_repo: Option<PathBuf>,
    /// Combined output of the last command the scenario ran.
    pub command_transcript: String,
    /// Combined output of every command the scenario ran, in order, for the
    /// transcript an `@e2e` scenario commits as evidence.
    pub scenario_transcript: String,
    /// Modification times of the staged archives, captured before a repeated
    /// preparation.
    pub staged_snapshot: Vec<(String, SystemTime)>,
    /// The change log a change-log scenario rendered.
    pub changelog: Option<PathBuf>,
    /// The temporary declaration directory an adoption scenario prepared.
    pub declaration: Option<PathBuf>,
    /// The selected batch file the adoption command imports.
    pub batch: Option<PathBuf>,
    /// The applied state path the adoption command records.
    pub state: Option<PathBuf>,
    /// The local fake GitHub API child; killed when the scenario ends.
    pub fake: Option<std::process::Child>,
    /// The port the fake GitHub API bound on loopback.
    pub fake_port: Option<u16>,
    /// The fake's request journal, one JSON object per request.
    pub journal: Option<PathBuf>,
    /// What the adoption command printed, for the refusal assertions and the
    /// committed transcript.
    pub adoption_transcript: String,
}

impl Drop for DutyWorld {
    fn drop(&mut self) {
        if let Some(mut child) = self.fake.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

impl DutyWorld {
    /// Fresh working directory for the current scenario.
    pub fn working_dir(&self) -> PathBuf {
        self.dir
            .clone()
            .expect("scenario prepared no working directory")
    }

    /// The source file supplied for conversion.
    pub fn source_path(&self) -> PathBuf {
        self.source.clone().expect("scenario prepared no source")
    }
}

#[tokio::main]
async fn main() {
    // cucumber-rs takes a single feature root; `./run-tests.sh` assembles every
    // `.feature` from `givn/specs/**` and the change's `specs/**` into this root.
    let root = std::env::var("GIVN_FEATURES").unwrap_or_else(|_| "tests/features".to_string());
    DutyWorld::cucumber()
        .fail_on_skipped()
        .run_and_exit(root)
        .await;
}
