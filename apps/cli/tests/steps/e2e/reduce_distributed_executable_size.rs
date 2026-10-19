// SPDX-License-Identifier: MPL-2.0
//! End-to-end steps for the capability `reduce-distributed-executable-size`.
//!
//! The e2e steps drive the real size-lever command and commit the terminal
//! transcript the visual contract requires.

use std::fs;
use std::path::PathBuf;

use cucumber::then;

use crate::steps::measure_distributed_executable_size::repository_root;
use crate::steps::reduce_distributed_executable_size::{
    lever_report_text, run_unsupported_source_on_host_lever,
};
use crate::DutyWorld;

fn transcript_path() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let active = root.join("givn/changes/reduce-executable-size");
    let home = if active.is_dir() {
        active
    } else {
        root.join("givn/archive/reduce-executable-size")
    };
    home.join(
        "evidence/visual/\
         maintainer-measures-the-panic-abort-lever-and-the-landed-executable-is-smaller/transcript.txt",
    )
}

fn commit_transcript(w: &DutyWorld) {
    let path = transcript_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create the evidence directory");
    }
    let project = fs::canonicalize(repository_root()).unwrap_or_else(|_| repository_root());
    let mut evidence = w.scenario_transcript.clone();
    if let Some(out) = &w.lever_out {
        let report = out.join("size-lever.md");
        if let Ok(text) = fs::read_to_string(&report) {
            evidence.push_str("\n--- size-lever.md ---\n");
            evidence.push_str(&text);
        }
    }
    evidence.push_str(&format!(
        "diagnostic exit status: {}\n",
        w.exit_status.unwrap_or(-1)
    ));
    let evidence = evidence
        .replace(&w.working_dir().display().to_string(), "<scenario-dir>")
        .replace(&project.display().to_string(), "<project>");
    fs::write(&path, evidence).expect("write the transcript");
}

#[then("the lever executable still writes a numbered diagnostic for an unsupported source")]
async fn lever_executable_diagnoses_unsupported(w: &mut DutyWorld) {
    let _ = lever_report_text(w);
    run_unsupported_source_on_host_lever(w);
    commit_transcript(w);
}
