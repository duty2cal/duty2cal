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

use crate::steps::adopt_existing_configuration::{
    journal_lines, repository_root, BILLING_EMAIL, PASSPHRASE, TOKEN,
};
use crate::steps::enable_repository_security::{
    fake_request, jq, scenario_dir_for, state_addresses, state_values,
};
use crate::steps::publish_site_plumbing::report;
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
    evidence = redact_private_material(&evidence);
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

// ---------------------------------------------------------------------------
// Scenario 10: apply the previewed revision
// ---------------------------------------------------------------------------

/// The journal lines the scenario's own preview/apply wrote.
fn change_journal(w: &DutyWorld) -> PathBuf {
    let lines = journal_lines(w);
    let path = scenario_dir_for(w).join("e2e-change-journal.jsonl");
    let text = lines
        .iter()
        .skip(w.journal_offset)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&path, format!("{text}\n")).expect("write the scenario's own journal");
    path
}

#[then("the apply reports the deploy key, the secret and the rule set as created")]
async fn the_apply_reports_the_pages_objects_created(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the pages revision applies; output was:\n{}",
        w.apply_transcript
    );
    for expected in [
        "github_repository_deploy_key.pages_publish",
        "github_actions_secret.pages_publish",
        "github_repository_ruleset.pages_publish",
        "Apply complete! Resources: 4 added, 0 changed, 0 destroyed.",
    ] {
        assert!(
            w.apply_transcript.contains(expected),
            "the apply reports {expected:?}; output was:\n{}",
            w.apply_transcript
        );
    }
    mark_evidence(
        w,
        "organization-owner-applies-the-previewed-revision-and-the-pages-publishing-credential-is-created",
    );
}

#[then(regex = r#"^the pages repository carries a write-enabled deploy key titled "([^"]+)"$"#)]
async fn the_pages_repository_carries_the_write_enabled_deploy_key(
    w: &mut DutyWorld,
    title: String,
) {
    let (status, body) = fake_request(w, "GET", "/repos/duty2cal/duty2cal.github.io/keys", &[], None);
    assert_eq!(status, 200, "the deploy keys are readable: {body}");
    let keys = report(w, "e2e-applied-keys.json", &format!("[{body}]"));
    assert_eq!(
        jq(
            w,
            &format!(
                "[.[][]? | select(.title == {title:?})] | length"
            ),
            &keys,
            false
        ),
        "1",
        "the pages repository carries the deploy key titled {title}"
    );
    assert_eq!(
        jq(
            w,
            &format!(
                "[.[][]? | select(.title == {title:?}) | .read_only] | .[0]"
            ),
            &keys,
            false
        ),
        "false",
        "the deploy key is write-enabled"
    );
}

#[then(
    "the secret write carries a sealed ciphertext for the fetched repository public key and no \
     plaintext key"
)]
async fn the_secret_write_is_sealed(w: &mut DutyWorld) {
    let journal = change_journal(w);
    let filter = |field: &str| {
        format!(
            "[ .[] | select(.applied_write == true and .method == \"PUT\" \
             and .path == \"/repos/duty2cal/duty2cal/actions/secrets/PAGES_DEPLOY_KEY\") ] \
             | .[0].{field}"
        )
    };
    assert_eq!(
        jq(w, &filter("secret_name"), &journal, true),
        "PAGES_DEPLOY_KEY",
        "the secret write names PAGES_DEPLOY_KEY"
    );
    assert_eq!(
        jq(w, &filter("key_id"), &journal, true),
        "568250167242205012",
        "the provider used the fetched repository public key"
    );
    let ciphertext_bytes: u64 = jq(w, &filter("ciphertext_bytes"), &journal, true)
        .parse()
        .expect("the journal records the ciphertext length");
    assert!(
        ciphertext_bytes > 48,
        "the sealed box carries more than its 48-byte overhead; length was {ciphertext_bytes}"
    );
    assert_eq!(
        jq(w, &filter("contains_pem_header"), &journal, true),
        "false",
        "the ciphertext carries no plaintext PEM header"
    );
    let text = fs::read_to_string(&journal).expect("read the scenario's journal");
    for forbidden in [PASSPHRASE, TOKEN, "-----BEGIN"] {
        assert!(
            !text.contains(forbidden),
            "the journal carries no plaintext credential: {forbidden}"
        );
    }
}

#[then("the pages rule set is active and grants write-enabled deploy keys an always bypass")]
async fn the_pages_rule_set_is_active_with_the_bypass(w: &mut DutyWorld) {
    let (status, body) =
        fake_request(w, "GET", "/repos/duty2cal/duty2cal.github.io/rulesets", &[], None);
    assert_eq!(status, 200, "the rule sets are readable: {body}");
    let rulesets = report(w, "e2e-applied-rulesets.json", &body);
    let filter = |field: &str| {
        format!("[.[] | select(.name == \"pages protection\")] | .[0].{field}")
    };
    assert_eq!(
        jq(w, &filter("enforcement"), &rulesets, false),
        "active",
        "the rule set is active"
    );
    assert_eq!(
        jq(w, &filter("target"), &rulesets, false),
        "branch",
        "the rule set targets branches"
    );
    assert_eq!(
        jq(
            w,
            "[.[] | select(.name == \"pages protection\")][0].bypass_actors | length",
            &rulesets,
            false
        ),
        "1",
        "the rule set carries one bypass actor"
    );
    assert_eq!(
        jq(
            w,
            "[.[] | select(.name == \"pages protection\")][0].bypass_actors[0] \
             | \"\\(.actor_type) \\(.bypass_mode) \\(.actor_id // \"none\")\"",
            &rulesets,
            false
        ),
        "DeployKey always none",
        "write-enabled deploy keys bypass the rules with no actor id"
    );
}

#[then("the pages rule set forbids deletion, force pushes and merge commits")]
async fn the_pages_rule_set_forbids_the_three_rules(w: &mut DutyWorld) {
    let (status, body) =
        fake_request(w, "GET", "/repos/duty2cal/duty2cal.github.io/rulesets", &[], None);
    assert_eq!(status, 200, "the rule sets are readable: {body}");
    let rulesets = report(w, "e2e-applied-rules.json", &body);
    let rules = jq(
        w,
        "[.[] | select(.name == \"pages protection\")][0].rules \
         | map(.type) | sort | join(\" \")",
        &rulesets,
        false,
    );
    assert_eq!(
        rules, "deletion non_fast_forward required_linear_history",
        "the rule set forbids deletion, force pushes and merge commits"
    );
}

#[then("the applied state records the generated pages publishing key material")]
async fn the_applied_state_records_the_generated_key(w: &mut DutyWorld) {
    let _ = state_values(w);
    let report = scenario_dir_for(w).join("state-values.json");
    let filter = |field: &str| {
        format!(
            "[ .values.root_module.resources[]? \
             | select(.address == \"tls_private_key.pages_publish\") \
             | .values.{field} ] | .[0] // \"\""
        )
    };
    let private = jq(w, &filter("private_key_openssh"), &report, false);
    assert!(
        private.starts_with("-----BEGIN OPENSSH PRIVATE KEY-----"),
        "the applied state records the generated private key material"
    );
    let public = jq(w, &filter("public_key_openssh"), &report, false);
    assert!(
        public.starts_with("ssh-ed25519 "),
        "the applied state records the generated public key material"
    );
    let addresses = state_addresses(w);
    assert!(
        addresses
            .iter()
            .any(|recorded| recorded == "tls_private_key.pages_publish"),
        "the applied state binds the generated key; it was: {addresses:?}"
    );
}

/// Redact the state's generated private key (the plan JSON prints it in the
/// saved state) and the sealed secret value from the committed transcript.
fn redact_private_material(text: &str) -> String {
    const BEGIN: [&str; 4] = [
        "-----BEGIN OPENSSH PRIVATE KEY-----",
        "-----BEGIN RSA PRIVATE KEY-----",
        "-----BEGIN EC PRIVATE KEY-----",
        "-----BEGIN PRIVATE KEY-----",
    ];
    let mut result = String::new();
    let mut rest = text;
    loop {
        let next = BEGIN
            .iter()
            .filter_map(|marker| rest.find(marker).map(|at| (at, *marker)))
            .min_by_key(|(at, _)| *at);
        let Some((begin, _)) = next else {
            result.push_str(rest);
            break;
        };
        result.push_str(&rest[..begin]);
        let after = &rest[begin..];
        let end = after
            .find("-----END ")
            .and_then(|end_at| {
                after[end_at + 9..]
                    .find("-----")
                    .map(|close| end_at + 9 + close + 5)
            });
        result.push_str("<private-key>");
        rest = match end {
            Some(end) => &after[end..],
            // A truncated key must not leak; drop the remainder.
            None => "",
        };
    }
    redact_sealed_value(&result)
}

/// Replace an `"encrypted_value":"..."` JSON string with a placeholder.
fn redact_sealed_value(text: &str) -> String {
    const KEY: &str = "\"encrypted_value\":\"";
    let mut result = String::new();
    let mut rest = text;
    while let Some(at) = rest.find(KEY) {
        result.push_str(&rest[..at + KEY.len()]);
        let after = &rest[at + KEY.len()..];
        let Some(end) = after.find('"') else {
            result.push_str("<sealed-value>");
            return result;
        };
        result.push_str("<sealed-value>");
        rest = &after[end..];
    }
    result.push_str(rest);
    result
}
