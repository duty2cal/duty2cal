// SPDX-License-Identifier: MPL-2.0
//! Gherkin runner (see design.md "Test runner, strict mode, and single-scenario
//! command"). Strict by construction: `fail_on_skipped()` makes an undefined or
//! skipped step fail the run.

use std::path::PathBuf;

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
