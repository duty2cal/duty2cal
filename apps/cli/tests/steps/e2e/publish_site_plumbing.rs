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

use cucumber::{given, then};

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

/// The four declared addresses, in the design's order.
const DECLARED: [&str; 4] = [
    "github_repository_deploy_key.pages_publish",
    "github_actions_secret.pages_publish",
    "github_repository_ruleset.pages_publish",
    "tls_private_key.pages_publish",
];

/// The creates in the saved plan, sorted.
const CREATES: &str = r#"
[ .resource_changes[]?
  | select((.change.actions // []) == ["create"])
  | .address ]
| sort
| join(" ")
"#;

/// Every non-no-op change in the saved plan, as `address action`.
const CHANGES: &str = r#"
[ .resource_changes[]?
  | select((.change.actions // []) != ["no-op"])
  | "\(.address) \(.change.actions | join(","))" ]
| sort
| join(" ")
"#;

/// The saved plan's JSON report.
fn plan_report(w: &DutyWorld) -> PathBuf {
    let report = crate::steps::enable_repository_security::scenario_dir_for(w).join("preview.json");
    assert!(
        report.is_file(),
        "the preview wrote no plan report at {}",
        report.display()
    );
    report
}

#[given("the pages repository has no deploy key and no rule set")]
async fn the_pages_repository_has_no_deploy_key_and_no_rule_set(w: &mut DutyWorld) {
    mark_evidence(
        w,
        "organization-owner-previews-the-pages-plumbing-and-only-the-three-live-creations-are-named",
    );
    let (keys, _, rulesets) =
        crate::steps::publish_site_plumbing::live_lists(w).expect("the live lists are readable");
    let keys = crate::steps::publish_site_plumbing::report(w, "e2e-empty-keys.json", &keys);
    let rulesets =
        crate::steps::publish_site_plumbing::report(w, "e2e-empty-rulesets.json", &rulesets);
    assert_eq!(
        crate::steps::enable_repository_security::jq(w, ".[0] | length", &keys, false),
        "0",
        "the pages repository has no deploy key"
    );
    assert_eq!(
        crate::steps::enable_repository_security::jq(w, ".[0] | length", &rulesets, false),
        "0",
        "the pages repository has no rule set"
    );
}

#[given(regex = r#"^the monorepo has no "([^"]+)" secret$"#)]
async fn the_monorepo_has_no_secret(w: &mut DutyWorld, name: String) {
    let (_, secrets, _) =
        crate::steps::publish_site_plumbing::live_lists(w).expect("the live lists are readable");
    let secrets = crate::steps::publish_site_plumbing::report(w, "e2e-empty-secrets.json", &secrets);
    assert_eq!(
        crate::steps::enable_repository_security::jq(
            w,
            &format!("[.secrets[]? | select(.name == {name:?})] | length"),
            &secrets,
            false
        ),
        "0",
        "the monorepo has no {name} secret"
    );
}

/// Assert one address is in the saved plan's create set.
fn assert_create(w: &DutyWorld, address: &str) {
    let report = plan_report(w);
    let creates = crate::steps::enable_repository_security::jq(w, CREATES, &report, false);
    assert!(
        creates.split_whitespace().any(|created| created == address),
        "the preview names {address} as an object to create; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );
}

#[then("the preview names the pages deploy key as an object to create")]
async fn the_preview_names_the_deploy_key_as_a_create(w: &mut DutyWorld) {
    assert_create(w, "github_repository_deploy_key.pages_publish");
}

#[then(regex = r#"^the preview names the "([^"]+)" secret as an object to create$"#)]
async fn the_preview_names_the_secret_as_a_create(w: &mut DutyWorld, name: String) {
    assert!(
        plan_report(w).is_file(),
        "the preview wrote a plan report"
    );
    assert_eq!(name, "PAGES_DEPLOY_KEY", "the step names the declared secret");
    assert_create(w, "github_actions_secret.pages_publish");
}

#[then("the preview names the pages rule set as an object to create")]
async fn the_preview_names_the_rule_set_as_a_create(w: &mut DutyWorld) {
    assert_create(w, "github_repository_ruleset.pages_publish");
}

#[then("the preview names the generated pages publishing key material as an object to create")]
async fn the_preview_names_the_generated_key_as_a_create(w: &mut DutyWorld) {
    assert_create(w, "tls_private_key.pages_publish");
}

#[then("the preview names no other live object to create or change")]
async fn the_preview_names_no_other_change(w: &mut DutyWorld) {
    let report = plan_report(w);
    let mut expected: Vec<String> = DECLARED
        .iter()
        .map(|address| format!("{address} create"))
        .collect();
    expected.sort();
    assert_eq!(
        crate::steps::enable_repository_security::jq(w, CHANGES, &report, false),
        expected.join(" "),
        "the preview names exactly the four creations; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );
    let mut declared: Vec<String> = DECLARED.iter().map(|address| (*address).to_string()).collect();
    declared.sort();
    assert_eq!(
        crate::steps::enable_repository_security::jq(w, CREATES, &report, false),
        declared.join(" "),
        "the preview's create set is the four declared addresses"
    );
}
