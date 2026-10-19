// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `publish-site-plumbing`.
//!
//! The non-e2e scenarios drive the real `tofu` binary against the local fake
//! GitHub API. The `@e2e` scenarios live in `steps/e2e/` because they commit
//! the scrubbed transcript.
#![allow(clippy::too_many_arguments)] // step signatures mirror the captured step text

use std::fs;
use std::path::PathBuf;

use cucumber::{given, then, when};

use crate::steps::adopt_existing_configuration::{
    adopt, adoption_output, copy_declaration, copy_tree, declaration_dir, fixtures_dir,
    journal_lines, repository_root, restart_fake_with, scenario_dir, start_fake_with, state_path,
};
use crate::steps::enable_repository_security::{
    combined, fake_request, jq, scenario_dir_for, stale_path, state_addresses, state_values, tofu,
    PREFLIGHT_FILTER,
};
use crate::steps::publish_organization_profile::{
    ensure_source_tree, write_profile_descriptors,
};
use crate::DutyWorld;

/// The deploy key's title on the pages repository.
const DEPLOY_KEY_TITLE: &str = "duty2cal.github.io publish";
/// The Actions secret's name on the monorepo.
const SECRET_NAME: &str = "PAGES_DEPLOY_KEY";
/// The pages repository's protection rule set.
const RULESET_NAME: &str = "pages protection";

/// The three live resources the preview targets, in the design's order.
const TARGETS: [&str; 3] = [
    "-target=github_repository_deploy_key.pages_publish",
    "-target=github_actions_secret.pages_publish",
    "-target=github_repository_ruleset.pages_publish",
];

/// The four addresses the revision declares, in the design's order.
const DECLARED_ADDRESSES: [&str; 4] = [
    "github_repository_deploy_key.pages_publish",
    "github_actions_secret.pages_publish",
    "github_repository_ruleset.pages_publish",
    "tls_private_key.pages_publish",
];

/// The declared addresses that have a live counterpart.
const LIVE_DECLARED: [&str; 3] = [
    "github_repository_deploy_key.pages_publish",
    "github_actions_secret.pages_publish",
    "github_repository_ruleset.pages_publish",
];

/// The inheritable addresses the earlier configuration changes bound.
const INHERITED_ADDRESSES: [&str; 9] = [
    "github_organization_settings.org",
    "github_repository.mono",
    "github_repository.pages",
    "github_repository_ruleset.main",
    "github_repository_pages.pages",
    "github_repository_vulnerability_alerts.mono",
    "github_repository_dependabot_security_updates.mono",
    "github_repository.dot_github",
    "github_repository_file.org_profile_readme",
];

/// The marker a pages scenario writes beside its primed declaration; the
/// shared preview/apply steps dispatch on it.
fn pages_marker(w: &DutyWorld) -> PathBuf {
    scenario_dir_for(w).join("pages-scenario")
}

/// Whether the current scenario primed the pages state.
pub(crate) fn is_pages_scenario(w: &DutyWorld) -> bool {
    w.dir
        .as_ref()
        .map(|dir| dir.join("pages-scenario").is_file())
        .unwrap_or(false)
}

/// A descriptive name for the guard's report, beside the address.
fn live_description(address: &str) -> &'static str {
    match address {
        "github_repository_deploy_key.pages_publish" => DEPLOY_KEY_TITLE,
        "github_actions_secret.pages_publish" => SECRET_NAME,
        "github_repository_ruleset.pages_publish" => RULESET_NAME,
        _ => "",
    }
}

/// Positive `jq` filter over a saved report.
fn jq_true(w: &DutyWorld, filter: &str, report: &std::path::Path) -> bool {
    jq(w, filter, report, false) == "true"
}

/// Write a response body to the scenario directory for `jq`.
fn report(w: &DutyWorld, name: &str, body: &str) -> PathBuf {
    let path = scenario_dir_for(w).join(name);
    fs::write(&path, body).expect("write a fake API response for jq");
    path
}

/// Seed the base tree's three live list fixtures for the guard's reads.
fn write_list_fixture(fixtures: &std::path::Path, relative: &str, body: &str) {
    let path = fixtures.join(relative);
    fs::create_dir_all(path.parent().expect("a fixture parent"))
        .expect("create the fixture directory");
    fs::write(&path, body).expect("write the list fixture");
}

/// Prime the inherited state with both adoption batches and the inherited
/// declaration's real apply, remove `pages_publish.tf` for that apply and
/// restore it afterwards (design.md "Step definitions"). The guard's list
/// fixtures already exist as empty arrays in `security-applied`.
pub(crate) fn prime_pages_state(w: &mut DutyWorld) {
    if w.declaration.is_some() {
        return;
    }

    let dir = scenario_dir(w);
    let declaration = dir.join("infra");
    copy_declaration(&repository_root().join("infra"), &declaration);
    ensure_source_tree(w);

    let fixtures = dir.join("fixtures");
    copy_tree(
        &repository_root().join("scripts/fake-github/fixtures/security-applied"),
        &fixtures,
    );
    write_profile_descriptors(&fixtures);
    let journal = dir.join("journal.jsonl");
    let (child, port) = start_fake_with(&fixtures, &journal, true);

    w.declaration = Some(declaration.clone());
    w.state = Some(declaration.join("terraform.tfstate"));
    w.journal = Some(journal);
    w.fake = Some(child);
    w.fake_port = Some(port);
    w.write_sandbox = true;
    fs::write(pages_marker(w), "pages\n").expect("write the pages scenario marker");

    for batch in ["adoption-batch.txt", "adoption-batch-security.txt"] {
        w.batch = Some(declaration.join(batch));
        adopt(w);
        assert_eq!(
            w.exit_status,
            Some(0),
            "the {batch} is adopted; output was:\n{}",
            adoption_output(w)
        );
    }

    let pages_declaration = declaration.join("pages_publish.tf");
    fs::remove_file(&pages_declaration)
        .expect("remove the pages declaration for the inherited base apply");

    let init = tofu(w, &["init", "-input=false", "-no-color"]);
    assert!(
        init.status.success(),
        "the inherited declaration's tofu init failed:\n{}",
        combined(&init)
    );
    let plan = tofu(
        w,
        &["plan", "-input=false", "-no-color", "-out=base-plan.bin"],
    );
    assert!(
        plan.status.success(),
        "the inherited declaration's plan failed:\n{}",
        combined(&plan)
    );
    let apply = tofu(w, &["apply", "-input=false", "-no-color", "base-plan.bin"]);
    assert!(
        apply.status.success(),
        "the inherited declaration's apply failed:\n{}",
        combined(&apply)
    );
    let _ = fs::remove_file(declaration.join("base-plan.bin"));

    fs::copy(
        repository_root().join("infra/pages_publish.tf"),
        &pages_declaration,
    )
    .expect("restore the pages declaration after the inherited base apply");

    w.journal_offset = journal_lines(w).len();
    w.exit_status = None;
    w.stdout.clear();
    w.stderr.clear();
}

/// The fake's three live list bodies, wrapped the way `gh --slurp` returns
/// the paginated lists so the design's own verdict filters apply unchanged.
fn live_lists(w: &DutyWorld) -> Result<(String, String, String), String> {
    let (status, keys) =
        fake_request(w, "GET", "/repos/duty2cal/duty2cal.github.io/keys", &[], None);
    if status != 200 {
        return Err(format!(
            "GET /repos/duty2cal/duty2cal.github.io/keys answered {status}"
        ));
    }
    let (status, secrets) = fake_request(
        w,
        "GET",
        "/repos/duty2cal/duty2cal/actions/secrets",
        &[],
        None,
    );
    if status != 200 {
        return Err(format!(
            "GET /repos/duty2cal/duty2cal/actions/secrets answered {status}"
        ));
    }
    let (status, rulesets) = fake_request(
        w,
        "GET",
        "/repos/duty2cal/duty2cal.github.io/rulesets",
        &[],
        None,
    );
    if status != 200 {
        return Err(format!(
            "GET /repos/duty2cal/duty2cal.github.io/rulesets answered {status}"
        ));
    }
    Ok((format!("[{keys}]"), secrets, format!("[{rulesets}]")))
}

/// The declared addresses that are live and not bound by the applied state.
fn live_addresses(w: &DutyWorld) -> Result<Vec<String>, String> {
    let (keys, secrets, rulesets) = live_lists(w)?;
    let keys = report(w, "live-keys.json", &keys);
    let secrets = report(w, "live-secrets.json", &secrets);
    let rulesets = report(w, "live-rulesets.json", &rulesets);
    let mut live = Vec::new();
    if jq_true(
        w,
        &format!("any(.[][]?; .title == {DEPLOY_KEY_TITLE:?})"),
        &keys,
    ) {
        live.push("github_repository_deploy_key.pages_publish".to_string());
    }
    if jq_true(
        w,
        &format!("any(.secrets[]?; .name == {SECRET_NAME:?})"),
        &secrets,
    ) {
        live.push("github_actions_secret.pages_publish".to_string());
    }
    if jq_true(
        w,
        &format!("any(.[][]?; .name == {RULESET_NAME:?})"),
        &rulesets,
    ) {
        live.push("github_repository_ruleset.pages_publish".to_string());
    }
    Ok(live)
}

/// Record the live list bodies before a preview so the scenario's
/// "live configuration is unchanged" assertion compares real responses.
fn snapshot_live(w: &DutyWorld) {
    let (keys, secrets, rulesets) = live_lists(w).expect("the guard's list fixtures are readable");
    let dir = scenario_dir_for(w);
    fs::write(dir.join("live-before-keys.json"), keys).expect("record the deploy-key list");
    fs::write(dir.join("live-before-secrets.json"), secrets).expect("record the secret list");
    fs::write(dir.join("live-before-rulesets.json"), rulesets).expect("record the rule-set list");
}

/// Assert the three live lists and the journal are unchanged since the
/// scenario's snapshot and journal offset.
pub(crate) fn assert_live_unchanged(w: &DutyWorld) {
    let (keys, secrets, rulesets) = live_lists(w).expect("the live lists remain readable");
    let dir = scenario_dir_for(w);
    let before_keys =
        fs::read_to_string(dir.join("live-before-keys.json")).expect("read the key-list snapshot");
    let before_secrets = fs::read_to_string(dir.join("live-before-secrets.json"))
        .expect("read the secret-list snapshot");
    let before_rulesets = fs::read_to_string(dir.join("live-before-rulesets.json"))
        .expect("read the rule-set snapshot");
    assert_eq!(keys, before_keys, "the deploy-key list changed");
    assert_eq!(secrets, before_secrets, "the secret list changed");
    assert_eq!(rulesets, before_rulesets, "the rule-set list changed");

    let writes = journal_lines(w)
        .iter()
        .skip(w.journal_offset)
        .filter(|line| {
            line.contains("\"applied_write\": true") || line.contains("\"refused_write\": true")
        })
        .count();
    assert_eq!(
        writes, 0,
        "the refused preview issued no write; journal was:\n{:#?}",
        journal_lines(w)
    );
}

/// Write the unbound live objects into the transcript, address and name.
fn refuse_unbound(transcript: &mut String, live: &[String]) {
    transcript.push_str("refused: the live object exists and is unbound:\n");
    for address in live {
        let description = live_description(address);
        if description.is_empty() {
            transcript.push_str(&format!("{address}\n"));
        } else {
            transcript.push_str(&format!("{address} ({description})\n"));
        }
    }
}

/// The preview routine (design.md "The apply procedure", steps 0-4): the
/// state-aware creation guard, the targeted saved plan and its report.
pub(crate) fn preview(w: &mut DutyWorld) {
    let declaration = declaration_dir(w);
    let dir = scenario_dir_for(w);
    let report_path = dir.join("preview.json");
    let stale = stale_path(w);
    let mut transcript = String::from(
        "$ preview (stale check; guard; tofu init; tofu state list; targeted plan -out; tofu show -json)\n",
    );

    w.preview_transcript.clear();
    w.plan_path = None;

    if stale.exists() {
        transcript.push_str(&format!(
            "refused: the applied state is stale ({})\n",
            stale.display()
        ));
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        w.exit_status = Some(2);
        return;
    }

    snapshot_live(w);

    let init = tofu(w, &["init", "-input=false"]);
    transcript.push_str("$ tofu init -input=false\n");
    transcript.push_str(&combined(&init));
    if !init.status.success() {
        w.exit_status = init.status.code();
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    let list = tofu(w, &["state", "list"]);
    transcript.push_str("$ tofu state list\n");
    transcript.push_str(&combined(&list));
    if !list.status.success() {
        transcript.push_str("refused: the applied state could not be read; nothing was written\n");
        w.exit_status = Some(2);
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }
    let bound: Vec<String> = String::from_utf8_lossy(&list.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect();

    let live = match live_addresses(w) {
        Ok(live) => live,
        Err(reason) => {
            transcript.push_str(&format!(
                "refused: the live objects could not be read: {reason}\n"
            ));
            w.exit_status = Some(2);
            w.preview_transcript = transcript.clone();
            w.scenario_transcript.push_str(&transcript);
            return;
        }
    };
    let unbound: Vec<String> = live
        .iter()
        .filter(|address| !bound.iter().any(|recorded| recorded == *address))
        .cloned()
        .collect();
    if !unbound.is_empty() {
        refuse_unbound(&mut transcript, &unbound);
        w.exit_status = Some(2);
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    let mut expected: Vec<String> = DECLARED_ADDRESSES
        .iter()
        .filter(|address| !bound.iter().any(|recorded| recorded == *address))
        .map(|address| (*address).to_string())
        .collect();
    expected.sort();
    transcript.push_str(&format!("expected: {}\n", expected.join(" ")));

    let mut plan_args = vec!["plan", "-input=false", "-no-color"];
    plan_args.extend(TARGETS.iter().copied());
    plan_args.push("-out=plan.bin");
    let plan = tofu(w, &plan_args);
    transcript.push_str(&format!(
        "$ tofu plan -input=false -no-color {} -out=plan.bin\n",
        TARGETS.join(" ")
    ));
    transcript.push_str(&combined(&plan));
    if !plan.status.success() {
        w.exit_status = plan.status.code();
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    let show = tofu(w, &["show", "-json", "plan.bin"]);
    fs::write(&report_path, &show.stdout).expect("write the saved plan's JSON report");
    transcript.push_str("$ tofu show -json plan.bin\n");
    transcript.push_str(&combined(&show));
    if !show.status.success() {
        w.exit_status = show.status.code();
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    let bad_actions = jq(
        w,
        "[.resource_changes[]? \
         | select(((.change.actions // []) - [\"create\", \"no-op\", \"read\"]) | length > 0) \
         | .address] | join(\" \")",
        &report_path,
        false,
    );
    if !bad_actions.is_empty() {
        transcript.push_str(&format!(
            "refused: the preview would update, delete or replace an object: {bad_actions}\n"
        ));
        w.exit_status = Some(2);
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    let creates = jq(
        w,
        "[.resource_changes[]? | select((.change.actions // []) == [\"create\"]) | .address] \
         | sort | join(\" \")",
        &report_path,
        false,
    );
    if creates != expected.join(" ") {
        transcript.push_str(&format!(
            "refused: the preview is not the missing creations\nexpected: {}\nactual:   {creates}\n",
            expected.join(" ")
        ));
        w.exit_status = Some(2);
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    if expected.is_empty() {
        transcript.push_str("the revision is already applied; nothing to create\n");
        let _ = fs::remove_file(declaration.join("plan.bin"));
        let _ = fs::remove_file(&report_path);
        w.exit_status = Some(0);
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    let mut preflight_args = vec![
        "plan",
        "-refresh-only",
        "-detailed-exitcode",
        "-json",
        "-input=false",
    ];
    preflight_args.extend(TARGETS.iter().copied());
    let preflight = tofu(w, &preflight_args);
    let preflight_report = dir.join("preflight.json");
    fs::write(&preflight_report, &preflight.stdout).expect("write the preflight's JSON report");
    transcript.push_str(&format!(
        "$ tofu plan -refresh-only -detailed-exitcode -json -input=false {}\n",
        TARGETS.join(" ")
    ));
    if !preflight.stderr.is_empty() {
        transcript.push_str(&String::from_utf8_lossy(&preflight.stderr));
    }
    let verdict = jq(w, PREFLIGHT_FILTER, &preflight_report, true);
    if verdict != "ok" {
        transcript.push_str(&format!(
            "refused: the applied state could not be verified: {verdict} (plan exit {})\n",
            preflight.status.code().unwrap_or(-1)
        ));
        w.exit_status = Some(2);
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    w.plan_path = Some(declaration.join("plan.bin"));
    w.exit_status = Some(0);
    w.preview_transcript = transcript.clone();
    w.scenario_transcript.push_str(&transcript);
}

/// The apply routine (design.md "The apply procedure"): stale check, the
/// refresh-only verdict, the create-time guard immediately before the write,
/// then the saved plan's application.
pub(crate) fn apply_previewed(w: &mut DutyWorld) {
    let declaration = declaration_dir(w);
    let dir = scenario_dir_for(w);
    let stale = stale_path(w);
    let preflight_report = dir.join("preflight.json");
    let state_before = dir.join("state-before.tfstate");
    let mut transcript = String::from("$ apply (preflight; guard; tofu apply plan.bin)\n");

    w.apply_transcript.clear();

    if stale.exists() {
        transcript.push_str(&format!(
            "refused: the applied state is stale ({})\n",
            stale.display()
        ));
        w.apply_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        w.exit_status = Some(2);
        return;
    }

    let plan = w
        .plan_path
        .clone()
        .unwrap_or_else(|| declaration.join("plan.bin"));
    if !plan.is_file() {
        preview(w);
        transcript.push_str(&w.preview_transcript);
        if w.exit_status != Some(0) {
            w.apply_transcript = transcript.clone();
            w.scenario_transcript.push_str(&transcript);
            return;
        }
    }

    let mut preflight_args = vec![
        "plan",
        "-refresh-only",
        "-detailed-exitcode",
        "-json",
        "-input=false",
    ];
    preflight_args.extend(TARGETS.iter().copied());
    let preflight = tofu(w, &preflight_args);
    fs::write(&preflight_report, &preflight.stdout).expect("write the preflight's JSON report");
    transcript.push_str(&format!(
        "$ tofu plan -refresh-only -detailed-exitcode -json -input=false {}\n",
        TARGETS.join(" ")
    ));
    if !preflight.stderr.is_empty() {
        transcript.push_str(&String::from_utf8_lossy(&preflight.stderr));
    }
    let verdict = jq(w, PREFLIGHT_FILTER, &preflight_report, true);
    if verdict != "ok" {
        transcript.push_str(&format!(
            "refused: live changed after the preview: {verdict} (plan exit {})\n",
            preflight.status.code().unwrap_or(-1)
        ));
        w.apply_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        w.exit_status = Some(2);
        return;
    }

    let list = tofu(w, &["state", "list"]);
    assert!(
        list.status.success(),
        "the applied state must be readable before the apply:\n{}",
        combined(&list)
    );
    let bound: Vec<String> = String::from_utf8_lossy(&list.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect();
    let live = match live_addresses(w) {
        Ok(live) => live,
        Err(reason) => {
            transcript.push_str(&format!(
                "refused: the live objects could not be read: {reason}\n"
            ));
            w.apply_transcript = transcript.clone();
            w.scenario_transcript.push_str(&transcript);
            w.exit_status = Some(2);
            return;
        }
    };
    let unbound: Vec<String> = live
        .iter()
        .filter(|address| !bound.iter().any(|recorded| recorded == *address))
        .cloned()
        .collect();
    if !unbound.is_empty() {
        refuse_unbound(&mut transcript, &unbound);
        w.apply_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        w.exit_status = Some(2);
        return;
    }

    fs::copy(state_path(w), &state_before).expect("record the applied state before the apply");

    let apply = tofu(w, &["apply", "-input=false", "plan.bin"]);
    transcript.push_str("$ tofu apply -input=false plan.bin\n");
    transcript.push_str(&combined(&apply));
    w.stdout = String::from_utf8_lossy(&apply.stdout).into_owned();
    w.stderr = String::from_utf8_lossy(&apply.stderr).into_owned();
    w.exit_status = apply.status.code();
    if apply.status.success() {
        let _ = fs::remove_file(&plan);
        let _ = fs::remove_file(dir.join("preview.json"));
        let _ = fs::remove_file(&preflight_report);
        let _ = fs::remove_file(&state_before);
    }
    w.apply_transcript = transcript.clone();
    w.scenario_transcript.push_str(&transcript);
}

#[given("the applied state binds every object the earlier configuration changes declared")]
async fn the_applied_state_binds_every_earlier_object(w: &mut DutyWorld) {
    prime_pages_state(w);
    let addresses = state_addresses(w);
    for address in INHERITED_ADDRESSES {
        assert!(
            addresses.iter().any(|recorded| recorded == address),
            "the applied state binds {address}; it was: {addresses:?}"
        );
    }
    w.applied_state_snapshot = Some(state_values(w));
}

#[given(regex = r#"^the pages repository already has a deploy key titled "([^"]+)"$"#)]
async fn the_pages_repository_already_has_a_deploy_key(w: &mut DutyWorld, title: String) {
    assert_eq!(title, DEPLOY_KEY_TITLE, "the step names the declared key title");
    write_list_fixture(
        &fixtures_dir(w),
        "repos/duty2cal/duty2cal.github.io/keys.json",
        "[{\"id\": 100001, \
         \"key\": \"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILiveSeedKeyDuty2cal live@example.test\", \
         \"title\": \"duty2cal.github.io publish\", \"read_only\": false, \
         \"verified\": true, \"created_at\": \"2026-10-19T00:00:00Z\"}]\n",
    );
}

#[given(regex = r#"^the monorepo already stores "([^"]+)"$"#)]
async fn the_monorepo_already_stores(w: &mut DutyWorld, name: String) {
    assert_eq!(name, SECRET_NAME, "the step names the declared secret");
    write_list_fixture(
        &fixtures_dir(w),
        "repos/duty2cal/duty2cal/actions/secrets.json",
        "{\"total_count\": 1, \"secrets\": [{\"name\": \"PAGES_DEPLOY_KEY\", \
         \"created_at\": \"2026-10-19T00:00:00Z\", \"updated_at\": \"2026-10-19T00:00:00Z\"}]}\n",
    );
}

#[given(regex = r#"^the pages repository already has a rule set named "([^"]+)"$"#)]
async fn the_pages_repository_already_has_a_rule_set(w: &mut DutyWorld, name: String) {
    assert_eq!(name, RULESET_NAME, "the step names the declared rule set");
    write_list_fixture(
        &fixtures_dir(w),
        "repos/duty2cal/duty2cal.github.io/rulesets.json",
        "[{\"id\": 200001, \"name\": \"pages protection\", \"target\": \"branch\", \
         \"source_type\": \"Repository\", \"source\": \"duty2cal/duty2cal.github.io\", \
         \"enforcement\": \"active\"}]\n",
    );
}

#[given("the owner's authority cannot read the pages repository")]
async fn the_owners_authority_cannot_read_the_pages_repository(w: &mut DutyWorld) {
    assert!(
        w.declaration.is_some(),
        "the scenario primed no applied state"
    );
    let fixtures = fixtures_dir(w);
    fs::write(
        fixtures.join("faults.json"),
        "[{\"path_regex\": \"^/repos/duty2cal/duty2cal\\\\.github\\\\.io$\", \
         \"method\": \"GET\", \"status\": 403, \
         \"body\": {\"message\": \"Resource not accessible by integration\"}}]\n",
    )
    .expect("write the unreadable-pages-repository fault");
    restart_fake_with(w, true);
    // The restart starts a fresh journal; the guard's reads are not writes.
    w.journal_offset = 0;
}

#[given("the declaration adds the pages publishing key, its secret and the pages rule set")]
async fn the_declaration_adds_the_pages_objects(w: &mut DutyWorld) {
    let declaration = declaration_dir(w).join("pages_publish.tf");
    let text = fs::read_to_string(&declaration)
        .unwrap_or_else(|error| panic!("read {}: {error}", declaration.display()));
    for resource in [
        "resource \"tls_private_key\" \"pages_publish\"",
        "resource \"github_repository_deploy_key\" \"pages_publish\"",
        "resource \"github_actions_secret\" \"pages_publish\"",
        "resource \"github_repository_ruleset\" \"pages_publish\"",
    ] {
        assert!(
            text.contains(resource),
            "the declaration carries {resource}; it was:\n{text}"
        );
    }
    for value in [
        "algorithm = \"ED25519\"",
        "title      = \"duty2cal.github.io publish\"",
        "read_only  = false",
        "secret_name = \"PAGES_DEPLOY_KEY\"",
        "name        = \"pages protection\"",
        "actor_type  = \"DeployKey\"",
        "bypass_mode = \"always\"",
    ] {
        assert!(
            text.contains(value),
            "the declaration states {value}; it was:\n{text}"
        );
    }
}

#[given("a previewed revision that adds the pages publishing key, its secret and the pages rule set")]
async fn a_previewed_pages_revision(w: &mut DutyWorld) {
    preview(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the pages revision previews; output was:\n{}",
        w.preview_transcript
    );
}

#[given(regex = r#"^the pages repository gains a deploy key titled "([^"]+)" after the preview$"#)]
async fn the_pages_repository_gains_a_deploy_key_after_the_preview(
    w: &mut DutyWorld,
    title: String,
) {
    assert_eq!(title, DEPLOY_KEY_TITLE, "the step names the declared key title");
    assert!(
        w.plan_path.is_some(),
        "the scenario holds no previewed revision"
    );
    let body = format!(
        "{{\"title\": \"{}\", \
         \"key\": \"ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAfterPreviewKeyDuty2cal later@example.test\", \
         \"read_only\": false}}",
        DEPLOY_KEY_TITLE
    );
    let (status, response) = fake_request(
        w,
        "POST",
        "/repos/duty2cal/duty2cal.github.io/keys",
        &[("Content-Type", "application/json")],
        Some(&body),
    );
    assert_eq!(
        status, 201,
        "the deploy key appears live after the preview; response was: {response}"
    );
    fs::write(
        scenario_dir_for(w).join("seed-journal-offset"),
        journal_lines(w).len().to_string(),
    )
    .expect("record the journal offset after the live seed");
}

#[then("the apply is refused and names the pages deploy key")]
async fn the_apply_is_refused_and_names_the_pages_deploy_key(w: &mut DutyWorld) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the apply is refused; output was:\n{}",
        w.apply_transcript
    );
    assert!(
        w.apply_transcript.contains("refused"),
        "the apply reports the refusal; output was:\n{}",
        w.apply_transcript
    );
    assert!(
        w.apply_transcript
            .contains("github_repository_deploy_key.pages_publish"),
        "the apply names the pages deploy key; output was:\n{}",
        w.apply_transcript
    );
    assert!(
        w.apply_transcript.contains(DEPLOY_KEY_TITLE),
        "the apply names the key title; output was:\n{}",
        w.apply_transcript
    );
}

#[then("no live object the revision declares is created")]
async fn no_live_object_the_revision_declares_is_created(w: &mut DutyWorld) {
    let offset: usize = fs::read_to_string(scenario_dir_for(w).join("seed-journal-offset"))
        .expect("read the post-seed journal offset")
        .trim()
        .parse()
        .expect("parse the post-seed journal offset");
    let journal = journal_lines(w);
    let writes = journal
        .iter()
        .skip(offset)
        .filter(|line| {
            line.contains("\"applied_write\": true") || line.contains("\"refused_write\": true")
        })
        .collect::<Vec<_>>();
    assert!(
        writes.is_empty(),
        "the refused apply issued no write; journal was:\n{journal:#?}"
    );

    let (keys, secrets, rulesets) = live_lists(w).expect("the live lists remain readable");
    let keys = report(w, "after-keys.json", &keys);
    let secrets = report(w, "after-secrets.json", &secrets);
    let rulesets = report(w, "after-rulesets.json", &rulesets);
    assert_eq!(
        jq(w, ".[0] | length", &keys, false),
        "1",
        "only the deploy key that appeared after the preview is live"
    );
    assert_eq!(
        jq(w, ".total_count", &secrets, false),
        "0",
        "no secret was created"
    );
    assert_eq!(
        jq(w, ".[0] | length", &rulesets, false),
        "0",
        "no rule set was created"
    );
}

#[then("the preview is refused and names the pages deploy key")]
async fn the_preview_is_refused_and_names_the_pages_deploy_key(w: &mut DutyWorld) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the preview is refused; output was:\n{}",
        w.preview_transcript
    );
    assert!(
        w.preview_transcript.contains("refused"),
        "the preview reports the refusal; output was:\n{}",
        w.preview_transcript
    );
    assert!(
        w.preview_transcript
            .contains("github_repository_deploy_key.pages_publish"),
        "the preview names the pages deploy key; output was:\n{}",
        w.preview_transcript
    );
    assert!(
        w.preview_transcript.contains(DEPLOY_KEY_TITLE),
        "the preview names the key title; output was:\n{}",
        w.preview_transcript
    );
}

#[then(regex = r#"^the preview names the "([^"]+)" secret$"#)]
async fn the_preview_names_the_secret(w: &mut DutyWorld, name: String) {
    assert_eq!(name, SECRET_NAME, "the step names the declared secret");
    assert!(
        w.preview_transcript
            .contains("github_actions_secret.pages_publish"),
        "the preview names the pages secret; output was:\n{}",
        w.preview_transcript
    );
    assert!(
        w.preview_transcript.contains(SECRET_NAME),
        "the preview names {SECRET_NAME}; output was:\n{}",
        w.preview_transcript
    );
}

#[then("the preview names the pages rule set")]
async fn the_preview_names_the_pages_rule_set(w: &mut DutyWorld) {
    assert!(
        w.preview_transcript
            .contains("github_repository_ruleset.pages_publish"),
        "the preview names the pages rule set; output was:\n{}",
        w.preview_transcript
    );
    assert!(
        w.preview_transcript.contains(RULESET_NAME),
        "the preview names {RULESET_NAME}; output was:\n{}",
        w.preview_transcript
    );
}
