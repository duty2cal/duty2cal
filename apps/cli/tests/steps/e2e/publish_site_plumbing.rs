// SPDX-License-Identifier: MPL-2.0
//! End-to-end step definitions for the capability `publish-site-plumbing`.
//!
//! These steps drive the real `tofu` binary and commit one scrubbed transcript
//! per scenario under the change's visual evidence path (design.md "E2E smoke
//! test infrastructure" and "Visual Design Contract"). The non-e2e steps live
//! in `steps/publish_site_plumbing.rs`; this module adds only the e2e-specific
//! assertions and the transcript capture.

use std::fs;
use std::path::PathBuf;

use crate::steps::adopt_existing_configuration::{repository_root, BILLING_EMAIL, PASSPHRASE, TOKEN};
use crate::DutyWorld;

/// Where a scenario's terminal transcript is committed as evidence.
///
/// Only the active change writes evidence. Once the change is archived, a run
/// from a later change leaves the archived transcript untouched.
fn transcript_path(slug: &str) -> Option<PathBuf> {
    let active = repository_root().join("givn/changes/publish-site-plumbing");
    active.is_dir().then(|| {
        active
            .join("evidence/visual")
            .join(slug)
            .join("transcript.txt")
    })
}

/// Record which scenario's transcript the terminal shared step commits.
pub(crate) fn mark_evidence(w: &DutyWorld, slug: &str) {
    let dir = w
        .dir
        .clone()
        .expect("scenario prepared no working directory");
    fs::write(dir.join("evidence-slug"), format!("{slug}\n"))
        .expect("record the scenario's evidence slug");
}

/// Commit the transcript if this scenario marked one. The marker is written by
/// the e2e-only steps, so the shared terminal steps stay inert for the non-e2e
/// scenarios that reuse them.
pub(crate) fn commit_if_marked(w: &DutyWorld) {
    let Some(dir) = &w.dir else {
        return;
    };
    let marker = dir.join("evidence-slug");
    let Ok(slug) = fs::read_to_string(&marker) else {
        return;
    };
    let slug = slug.trim();
    if !slug.is_empty() {
        commit_transcript(w, slug);
    }
}

/// Commit the transcript with the run-specific directory and transport details
/// replaced by placeholders so only the command's output remains (design.md
/// "Visual Design Contract").
fn commit_transcript(w: &DutyWorld, slug: &str) {
    let Some(path) = transcript_path(slug) else {
        return;
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create the evidence directory");
    }
    let mut evidence = w.scenario_transcript.clone();
    if let Some(dir) = &w.dir {
        evidence = evidence.replace(&dir.display().to_string(), "<scenario-dir>");
    }
    if let Some(port) = w.fake_port {
        evidence = evidence.replace(&format!("127.0.0.1:{port}"), "127.0.0.1:<port>");
    }
    evidence = evidence
        .replace(TOKEN, "<token>")
        .replace(PASSPHRASE, "<passphrase>")
        .replace(BILLING_EMAIL, "<billing-email>");
    let status = w.exit_status.unwrap_or(-1);
    if !evidence
        .trim_end()
        .ends_with(&format!("exit status: {status}"))
    {
        evidence.push_str(&format!("exit status: {status}\n"));
    }
    fs::write(&path, evidence).expect("write the transcript");
}
