//! Gherkin runner for the change `export-released-roster-calendar`.
//!
//! Strict by construction: `fail_on_skipped()` makes an undefined or skipped
//! step fail the run, so a task cannot tick while nothing is implemented.

use cucumber::World as _;

mod steps;

#[derive(Debug, Default, cucumber::World)]
pub struct DutyWorld {
    /// Exit status of the last child invocation, when one was run.
    pub exit_status: Option<i32>,
    /// Captured standard output of the last child invocation.
    pub stdout: String,
    /// Captured standard error of the last child invocation.
    pub stderr: String,
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
