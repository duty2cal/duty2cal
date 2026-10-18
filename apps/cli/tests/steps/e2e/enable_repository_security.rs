// SPDX-License-Identifier: MPL-2.0
//! End-to-end steps for the capability `enable-repository-security`.
//!
//! The two `@e2e` scenarios drive the real `tofu` binary as subprocesses
//! against the local fake GitHub API on loopback (design.md "E2E smoke test
//! infrastructure"). The applied state is primed by the base declaration's own
//! apply, the fake runs in `--writes` mode on an ephemeral port, and the
//! scenario's terminal transcript is committed under the change's evidence
//! with the temporary path, the port, the token and the passphrase replaced by
//! placeholders. The fake is killed with the scenario's `DutyWorld`, so the
//! sandbox starts and stops per scenario.

use std::fs;
use std::path::PathBuf;

use cucumber::{given, then};

use crate::steps::adopt_existing_configuration::{
    declaration_dir, BILLING_EMAIL, PASSPHRASE, TOKEN,
};
use crate::steps::enable_repository_security::{
    assert_scanning_status, combined, ensure_base_state, jq, scenario_dir_for, state_addresses,
    tofu,
};
use crate::DutyWorld;

/// The addresses the base apply and the second batch leave bound.
const BOUND: [&str; 7] = [
    "github_organization_settings.org",
    "github_repository.mono",
    "github_repository.pages",
    "github_repository_ruleset.main",
    "github_repository_pages.pages",
    "github_repository_vulnerability_alerts.mono",
    "github_repository_dependabot_security_updates.mono",
];

/// The preview's changed resources, from the saved plan's JSON.
const CHANGED_RESOURCES: &str = r#"
[ .resource_changes[]?
  | select((.change.actions // []) != ["no-op"])
  | .address ]
| unique
| join(" ")
"#;

/// The changed resources' actions, from the saved plan's JSON.
const CHANGED_ACTIONS: &str = r#"
[ .resource_changes[]?
  | select((.change.actions // []) != ["no-op"])
  | (.change.actions // []) ]
| unique
| .[]
| join(" ")
"#;

/// Top-level monorepo attributes whose live value changes, excluding the
/// security block (expanded below) and the provider's state-only bookkeeping
/// field (design-review.md G6).
const CHANGED_ATTRIBUTES: &str = r#"
[ .resource_changes[]?
  | select(.address == "github_repository.mono")
  | .change as $change
  | (($change.after // {}) | keys[]) as $key
  | select($key != "security_and_analysis"
           and $key != "ignore_vulnerability_alerts_during_read")
  | select((($change.after // {})[$key]) != (($change.before // {})[$key]))
  | $key ]
| unique
| join(" ")
"#;

/// The security settings whose status changes in the monorepo's plan.
const CHANGED_SECURITY_SETTINGS: &str = r#"
[ .resource_changes[]?
  | select(.address == "github_repository.mono")
  | .change as $change
  | (((($change.after // {}).security_and_analysis) // []) | .[0] // {}) as $after
  | (((($change.before // {}).security_and_analysis) // []) | .[0] // {}) as $before
  | ($after | keys[]) as $setting
  | select((((($before[$setting] // []) | .[0] // {}).status) // null)
           != (((($after[$setting] // []) | .[0] // {}).status) // null))
  | $setting ]
| unique
| join(" ")
"#;

/// The number of create or delete actions anywhere in the saved plan.
const CREATED_OR_DESTROYED: &str = r#"
[ .resource_changes[]?
  | (.change.actions // [])[]
  | select(. == "create" or . == "delete") ]
| length
"#;

/// The security settings the plan enables, in sorted order.
const ENABLED_SETTINGS: &str = r#"
[ .resource_changes[]?
  | select(.address == "github_repository.mono")
  | .change as $change
  | (((($change.after // {}).security_and_analysis) // []) | .[0] // {}) as $after
  | ($after | to_entries[])
  | select((((.value // []) | .[0] // {}).status) == "enabled")
  | .key ]
| unique
| join(" ")
"#;

/// Where a scenario's terminal transcript is committed as evidence.
fn transcript_path(slug: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let active = root.join("givn/changes/enable-repository-security");
    let home = if active.is_dir() {
        active
    } else {
        root.join("givn/archive/enable-repository-security")
    };
    home.join("evidence/visual")
        .join(slug)
        .join("transcript.txt")
}

/// Commit the transcript the consumer read, with the run-specific directory and
/// transport details replaced by placeholders so only the command's output
/// remains (design.md "Visual Design Contract").
pub(crate) fn commit_transcript(w: &DutyWorld, slug: &str) {
    let path = transcript_path(slug);
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

/// The saved plan's JSON report.
fn plan_report(w: &DutyWorld) -> PathBuf {
    let report = scenario_dir_for(w).join("preview.json");
    assert!(
        report.is_file(),
        "the preview wrote no plan report at {}",
        report.display()
    );
    report
}

/// The changed leaf attributes the plan text renders, one per `~`/`+`/`-` line.
/// The provider's deprecated `ignore_vulnerability_alerts_during_read` field is
/// state-only bookkeeping and is classified, not a live change
/// (design-review.md G6).
fn changed_leaf_lines(transcript: &str) -> Vec<String> {
    transcript
        .lines()
        .map(str::trim)
        .filter(|line| {
            (line.starts_with('~') || line.starts_with('+') || line.starts_with('-'))
                && line.contains(" = ")
                && !line.contains("ignore_vulnerability_alerts_during_read")
        })
        .map(str::to_owned)
        .collect()
}

#[given(
    "the applied state binds the organization, both repositories, the default-branch rule, \
     the site source and the two Dependabot settings"
)]
async fn the_applied_state_binds_every_object_and_both_settings(w: &mut DutyWorld) {
    // The base batch's adoption and the base declaration's own apply prime the
    // state; the second batch then binds the two Dependabot settings. The fake
    // runs in `--writes` mode and is killed when the scenario's world drops.
    ensure_base_state(w);
    assert!(
        w.fake.is_some() && w.fake_port.is_some() && w.write_sandbox,
        "the scenario's fake GitHub API must run in write-sandbox mode"
    );
    let addresses = state_addresses(w);
    for address in BOUND {
        assert!(
            addresses.iter().any(|recorded| recorded == address),
            "the applied state binds {address}; it was: {addresses:?}"
        );
    }
}

#[given("that applied state was produced by the base declaration's own apply")]
async fn that_applied_state_was_produced_by_the_base_apply(w: &mut DutyWorld) {
    // The state was written once per imported object (five base objects and
    // two Dependabot settings) and once by the base declaration's own apply.
    // `tofu state pull` reads the applied state the real command left, so the
    // one extra serial is the base apply's write; the transcript carries its
    // `Apply complete!` and the no-change follow-up plan.
    let pull = tofu(w, &["state", "pull"]);
    assert!(
        pull.status.success(),
        "tofu state pull failed:\n{}",
        combined(&pull)
    );
    let report = scenario_dir_for(w).join("state-after-base-apply.json");
    fs::write(&report, &pull.stdout).expect("write the applied state's raw report");
    let serial = jq(w, ".serial", &report, false);
    let resources = jq(w, ".resources | length", &report, false);
    assert_eq!(
        resources,
        "7",
        "the applied state binds seven imported objects; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );
    let serial: u64 = serial
        .parse()
        .unwrap_or_else(|_| panic!("the applied state has no serial: {serial:?}"));
    assert_eq!(
        serial,
        resources.parse::<u64>().expect("a resource count") + 1,
        "the applied state was written once beyond the seven imports (the base apply)"
    );
    assert!(
        w.scenario_transcript
            .contains("$ base declaration's own apply (tofu init; tofu plan -out; tofu apply)"),
        "the scenario ran the base declaration's own apply"
    );
    assert!(
        w.scenario_transcript.contains("Apply complete!"),
        "the base declaration's apply reported success"
    );
}

#[given("the declaration enables secret scanning and push protection on the monorepo")]
async fn the_declaration_enables_the_scanning_settings(w: &mut DutyWorld) {
    let repos = fs::read_to_string(declaration_dir(w).join("repos.tf"))
        .expect("read the scenario's repository declaration");
    let normalized = repos.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        normalized.contains("security_and_analysis {"),
        "the declaration states the scanning block; it was:\n{repos}"
    );
    for setting in ["secret_scanning", "secret_scanning_push_protection"] {
        assert!(
            normalized.contains(&format!("{setting} {{ status = \"enabled\" }}")),
            "the declaration enables {setting}; it was:\n{repos}"
        );
    }
}

#[given("the monorepo reports both settings disabled")]
async fn the_monorepo_reports_both_settings_disabled(w: &mut DutyWorld) {
    assert_scanning_status(w, "disabled");
}

#[then("the preview names secret scanning and push protection as the only live changes")]
async fn the_preview_names_only_the_scanning_changes(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the preview succeeds; output was:\n{}",
        w.preview_transcript
    );
    for name in ["secret_scanning", "secret_scanning_push_protection"] {
        assert!(
            w.preview_transcript.contains(name),
            "the preview names {name}; output was:\n{}",
            w.preview_transcript
        );
    }
    for summary in [
        "0 to add, 1 to change, 0 to destroy",
        "~ status = \"disabled\" -> \"enabled\"",
    ] {
        assert!(
            w.preview_transcript.contains(summary),
            "the preview reports {summary:?}; output was:\n{}",
            w.preview_transcript
        );
    }
    let changed = changed_leaf_lines(&w.preview_transcript);
    assert_eq!(
        changed.len(),
        2,
        "the plan text changes exactly the two scanning statuses; changed lines were:\n{changed:#?}\n\
         output was:\n{}",
        w.preview_transcript
    );
    for line in changed {
        assert_eq!(
            line, "~ status = \"disabled\" -> \"enabled\"",
            "the only rendered leaf changes are the two scanning statuses"
        );
    }
}

#[then(regex = r#"^the plan's only changed resource is "([^"]+)"$"#)]
async fn the_plans_only_changed_resource_is(w: &mut DutyWorld, address: String) {
    let report = plan_report(w);
    assert_eq!(
        jq(w, CHANGED_RESOURCES, &report, false),
        address,
        "the saved plan changes only {address}; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );
    assert_eq!(
        jq(w, CHANGED_ACTIONS, &report, false),
        "update",
        "the saved plan updates the named resource; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );
}

#[then(regex = r#"^the plan's only changed live attributes are "([^"]+)" and "([^"]+)"$"#)]
async fn the_plans_only_changed_live_attributes_are(
    w: &mut DutyWorld,
    first: String,
    second: String,
) {
    let report = plan_report(w);
    assert_eq!(
        jq(w, CHANGED_ATTRIBUTES, &report, false),
        "",
        "no top-level live attribute other than the security block changes; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );

    let mut expected = vec![first, second];
    expected.sort();
    let mut changed: Vec<String> = jq(w, CHANGED_SECURITY_SETTINGS, &report, false)
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    changed.sort();
    assert_eq!(
        changed,
        expected,
        "the security block changes exactly the two named settings; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );

    let mut enabled: Vec<String> = jq(w, ENABLED_SETTINGS, &report, false)
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    enabled.sort();
    assert_eq!(
        enabled,
        expected,
        "the saved plan enables exactly the two named settings; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );
}

#[then("the preview reports no object to create or delete")]
async fn the_preview_reports_no_create_or_delete(w: &mut DutyWorld) {
    let report = plan_report(w);
    assert_eq!(
        jq(w, CREATED_OR_DESTROYED, &report, false),
        "0",
        "the saved plan creates and destroys nothing; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );
    assert!(
        w.preview_transcript
            .contains("0 to add, 1 to change, 0 to destroy"),
        "the preview reports no object to create or destroy; output was:\n{}",
        w.preview_transcript
    );
    commit_transcript(
        w,
        "organization-owner-previews-the-security-settings-and-only-the-declared-differences-are-named",
    );
}
