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
    /// Exit statuses of every child invocation in this scenario, in order.
    pub exit_statuses: Vec<i32>,
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
    /// The Playwright driver child; killed when the scenario ends.
    pub browser: Option<steps::browser::BrowserDriver>,
    /// The heading a notice scenario read before it activated a footer link.
    pub heading_before: String,
    /// The heading the same scenario read on the notice it reached.
    pub heading_after: String,
    /// The scenario's title, recorded by the before hook. A browser scenario
    /// names its committed visual evidence directory after it.
    pub scenario: String,
    /// The generated duty plan a scenario converts, when it is not the
    /// reference fixture.
    pub plan: Option<Vec<u8>>,
    /// The accepted calendar text an `@e2e` scenario compares against.
    pub accepted_calendar: Option<String>,
    /// Whether the published calendar equalled the accepted calendar after the
    /// agreed normalisation; the transcript records it.
    pub calendar_matched: bool,
}

impl Drop for DutyWorld {
    fn drop(&mut self) {
        // Dropping the driver kills and reaps its child.
        let _ = self.browser.take();
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
    // cucumber-rs takes a single feature root; `./run-tests.sh` assembles the
    // runnable corpus from `givn/specs/**` and the change's `specs/**`.
    let root = std::env::var("GIVN_FEATURES").unwrap_or_else(|_| "tests/features".to_string());
    DutyWorld::cucumber()
        .before(|_feature, _rule, scenario, world| {
            Box::pin(async move {
                world.scenario = scenario.name.clone();
            })
        })
        .fail_on_skipped()
        .run_and_exit(root)
        .await;
}
