// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `enable-repository-security`.
//!
//! Each scenario drives the real `tofu` binary as a subprocess against the
//! local fake GitHub API on loopback (design.md "Test seam" and "Step
//! definitions"). The declaration copy is primed by the base batch's real
//! adoption and the base declaration's real apply, then the second batch binds
//! the two Dependabot settings. The preview and the apply are the operator's
//! routine: `tofu init`, `tofu plan -out`, `tofu show -json`, the saved plan's
//! guard, the read-only refresh-only preflight, and `tofu apply plan.bin`.
#![allow(clippy::too_many_arguments)] // step signatures mirror the captured step text

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use cucumber::{given, then, when};

use crate::steps::adopt_existing_configuration::{
    adopt, adoption_output, copy_declaration, copy_tree, declaration_dir, fixtures_dir,
    journal_lines, repository_root, restart_fake_with, scenario_dir, start_fake_with, state_path,
    terraformrc, BILLING_EMAIL, PASSPHRASE, TOKEN,
};
use crate::DutyWorld;

/// The preview guard: refuse a create for a setting that is already live and
/// unbound (design.md "The apply procedure").
///
/// The refreshed prior state exposes the live Dependabot alerts flag
/// (`vulnerability_alerts`, measured true after the base apply) but not the
/// dependency-updates status: provider 6.13.0's repository schema has no
/// `security_and_analysis.dependabot_security_updates` member at all, so the
/// dependency-updates create is judged by the alerts flag. GitHub serves the
/// automated-security-fixes endpoint (200 `{enabled}`) only while alerts are
/// enabled and 404 otherwise, so a live-enabled alerts setting means the
/// dependency-updates object exists live and must be adopted before it is
/// updated.
const GUARD_FILTER: &str = r#"
(.prior_state.values.root_module.resources // []) as $res
| ($res | map(select(.address == "github_repository.mono")) | .[0].values) as $repo
| [ .resource_changes[]?
    | select((.change.actions // []) | index("create"))
    | .address
    | select(
        (. == "github_repository_vulnerability_alerts.mono"
         or . == "github_repository_dependabot_security_updates.mono")
        and $repo.vulnerability_alerts == true)]
| if length == 0 then "ok" else join(" ") end
"#;

/// The live-change preflight: only a completed report with no non-no-op change
/// continues; the JSON is the verdict, not the exit status
/// (opentofu/opentofu#3015).
const PREFLIGHT_FILTER: &str = r#"
if (map(select(.type == "change_summary")) | length) == 0 then "incomplete"
else
  [ .[] | select(.type == "planned_change" or .type == "resource_drift")
    | .change as $c
    | (($c.actions // []) + (if $c.action then [$c.action] else [] end))
    | select(any(.[]; . != "no-op"))
    | ($c.resource.addr // empty) ]
  | if length == 0 then "ok" else join(" ") end
end
"#;

/// The sidecar that marks a partial apply's state stale.
fn stale_path(w: &DutyWorld) -> PathBuf {
    PathBuf::from(format!("{}.stale", state_path(w).display()))
}

/// The combined output of a child invocation.
fn combined(output: &Output) -> String {
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    if !output.stderr.is_empty() {
        text.push_str("[stderr]\n");
        text.push_str(&String::from_utf8_lossy(&output.stderr));
    }
    text
}

/// Run the real `tofu` binary in the scenario's declaration with the fake's
/// environment; neither the real endpoint nor `gh auth token` can be reached.
fn tofu(w: &DutyWorld, args: &[&str]) -> Output {
    let declaration = declaration_dir(w);
    let port = w.fake_port.expect("the scenario started no fake API");
    let rc = terraformrc();
    assert!(
        rc.is_file(),
        "run ./scripts/setup-infra-tests.sh first: {} is absent",
        rc.display()
    );
    Command::new("tofu")
        .args(args)
        .current_dir(&declaration)
        .env("GITHUB_TOKEN", TOKEN)
        .env("GITHUB_BASE_URL", format!("http://127.0.0.1:{port}"))
        .env("TF_VAR_state_passphrase", PASSPHRASE)
        .env("TF_VAR_billing_email", BILLING_EMAIL)
        .env("TF_CLI_CONFIG_FILE", &rc)
        .env("TF_IN_AUTOMATION", "1")
        .output()
        .expect("run the real tofu binary")
}

/// Send one HTTP/1.1 request to the fake on loopback and return its status and
/// body. The test-only manual edit is a direct loopback request, not a `tofu`
/// invocation (design.md "Write sandbox").
fn fake_request(
    w: &DutyWorld,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: Option<&str>,
) -> (u16, String) {
    use std::io::{Read, Write};

    let port = w.fake_port.expect("the scenario started no fake API");
    let payload = body.unwrap_or("");
    let mut request = format!(
        "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\
         Content-Length: {}\r\n",
        payload.len()
    );
    for (name, value) in headers {
        request.push_str(&format!("{name}: {value}\r\n"));
    }
    request.push_str("\r\n");
    request.push_str(payload);

    let mut stream =
        std::net::TcpStream::connect(("127.0.0.1", port)).expect("connect to the fake GitHub API");
    stream
        .write_all(request.as_bytes())
        .expect("send the request to the fake GitHub API");
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("read the fake GitHub API's response");
    let (head, response_body) = response
        .split_once("\r\n\r\n")
        .expect("the fake GitHub API's response carries a body");
    let status = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .expect("the fake GitHub API's status line");
    (status, response_body.to_string())
}

/// The fake's live monorepo body, written to the scenario directory for `jq`.
fn live_monorepo_report(w: &DutyWorld) -> PathBuf {
    let (status, body) = fake_request(w, "GET", "/repos/duty2cal/duty2cal", &[], None);
    assert_eq!(
        status, 200,
        "the fake serves the monorepo's live body; response was: {body}"
    );
    let report = scenario_dir_for(w).join("live-monorepo.json");
    fs::write(&report, body).expect("write the live monorepo body");
    report
}

/// Assert the live scanning statuses the fake reports.
fn assert_scanning_status(w: &DutyWorld, expected: &str) {
    let report = live_monorepo_report(w);
    for field in ["secret_scanning", "secret_scanning_push_protection"] {
        let status = jq(
            w,
            &format!(".security_and_analysis.{field}.status // empty"),
            &report,
            false,
        );
        assert_eq!(
            status,
            expected,
            "the monorepo must report {field} {expected}; live body was:\n{}",
            fs::read_to_string(&report).unwrap_or_default()
        );
    }
}

/// The addresses the applied state records.
fn state_addresses(w: &DutyWorld) -> Vec<String> {
    let list = tofu(w, &["state", "list"]);
    assert!(
        list.status.success(),
        "tofu state list failed:\n{}",
        combined(&list)
    );
    String::from_utf8_lossy(&list.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect()
}

/// The live identifier of a drifted address, when the state records one.
fn object_live_id(w: &DutyWorld, address: &str) -> String {
    let report = scenario_dir_for(w).join("state-values.json");
    if !report.is_file() {
        return String::new();
    }
    let output = Command::new("jq")
        .args([
            "-r",
            "--arg",
            "address",
            address,
            "([.values.root_module.resources[]? | select(.address == $address) \
             | (.values.full_name // .values.repository // empty)] | .[0] // empty)",
        ])
        .arg(&report)
        .output()
        .expect("run jq over the state's values");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// Run one of the routine's jq filters and return its verdict.
fn jq(w: &DutyWorld, filter: &str, report: &Path, slurp: bool) -> String {
    let mut command = Command::new("jq");
    if slurp {
        command.arg("-s");
    }
    command
        .arg("-r")
        .arg(filter)
        .arg(report)
        .current_dir(scenario_dir_for(w));
    let output = command.output().expect("run jq");
    assert!(
        output.status.success(),
        "jq failed on {}: {}",
        report.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// The scenario directory without mutating the world.
fn scenario_dir_for(w: &DutyWorld) -> PathBuf {
    w.dir
        .clone()
        .expect("scenario prepared no working directory")
}

/// The applied state's decrypted values, canonically ordered: the scenario's
/// "state unchanged" invariant. A failed apply rewrites the envelope and may
/// persist planned values, so bytes and serials are not evidence; the routine's
/// write signal is live drift (design.md "The apply procedure").
fn state_values(w: &DutyWorld) -> Vec<u8> {
    let dir = scenario_dir_for(w);
    let show = tofu(w, &["show", "-json"]);
    assert!(
        show.status.success(),
        "tofu show -json failed:\n{}",
        combined(&show)
    );
    let report = dir.join("state-values.json");
    fs::write(&report, &show.stdout).expect("write the state's JSON report");
    let output = Command::new("jq")
        .args(["-S", "-c", ".values"])
        .arg(&report)
        .output()
        .expect("run jq over the state's values");
    assert!(
        output.status.success(),
        "jq failed over the state's values: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

/// Remove the `security_and_analysis` block from the base declaration.
fn remove_security_block(text: &str) -> String {
    let mut result = String::new();
    let mut depth = 0usize;
    for line in text.lines() {
        if depth == 0 && line.trim_start().starts_with("security_and_analysis {") {
            depth = 1;
            continue;
        }
        if depth > 0 {
            for character in line.chars() {
                match character {
                    '{' => depth += 1,
                    '}' => depth = depth.saturating_sub(1),
                    _ => {}
                }
            }
            continue;
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

/// Prepare the inherited applied state: adopt the base batch against
/// `security-applied`, apply the base declaration for real (which records the
/// provider's state-only read flag and the live values), and restore this
/// change's declaration. The second adoption batch is a separate step so a
/// scenario can prove the unadopted-setting guard.
fn ensure_base_adoption(w: &mut DutyWorld) {
    if w.declaration.is_some() {
        return;
    }

    let dir = scenario_dir(w);
    let declaration = dir.join("infra");
    copy_declaration(&repository_root().join("infra"), &declaration);

    let fixtures = dir.join("fixtures");
    copy_tree(
        &repository_root().join("scripts/fake-github/fixtures/security-applied"),
        &fixtures,
    );
    let journal = dir.join("journal.jsonl");
    let (child, port) = start_fake_with(&fixtures, &journal, true);

    w.declaration = Some(declaration.clone());
    w.batch = Some(declaration.join("adoption-batch.txt"));
    w.state = Some(declaration.join("terraform.tfstate"));
    w.journal = Some(journal);
    w.fake = Some(child);
    w.fake_port = Some(port);
    w.write_sandbox = true;

    adopt(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the base batch is adopted; output was:\n{}",
        adoption_output(w)
    );

    let repos = fs::read_to_string(declaration.join("repos.tf"))
        .expect("read the scenario's repository declaration");
    let base_repos = remove_security_block(&repos);
    assert_ne!(
        base_repos, repos,
        "the declaration carries the scanning block the base declaration inherits"
    );
    fs::write(declaration.join("repos.tf"), &base_repos)
        .expect("write the base declaration's repository file");
    fs::remove_file(declaration.join("security.tf"))
        .expect("remove the security declaration for the base apply");

    let init = tofu(w, &["init", "-input=false", "-no-color"]);
    assert!(
        init.status.success(),
        "the base declaration's tofu init failed:\n{}",
        combined(&init)
    );
    let plan = tofu(
        w,
        &["plan", "-input=false", "-no-color", "-out=base-plan.bin"],
    );
    assert!(
        plan.status.success(),
        "the base declaration's plan failed:\n{}",
        combined(&plan)
    );
    let plan_text = combined(&plan);
    for line in plan_text.lines() {
        let trimmed = line.trim();
        if (trimmed.starts_with('+') || trimmed.starts_with('~') || trimmed.starts_with('-'))
            && trimmed.contains(" = ")
            && !trimmed.contains("ignore_vulnerability_alerts_during_read")
        {
            panic!("the base declaration's plan changes a live attribute:\n{plan_text}");
        }
    }
    let apply = tofu(w, &["apply", "-input=false", "-no-color", "base-plan.bin"]);
    assert!(
        apply.status.success(),
        "the base declaration's apply failed:\n{}",
        combined(&apply)
    );
    let after = tofu(w, &["plan", "-input=false", "-no-color"]);
    assert!(
        after.status.success() && combined(&after).contains("No changes."),
        "the base apply left a live change:\n{}",
        combined(&after)
    );
    let _ = fs::remove_file(declaration.join("base-plan.bin"));

    fs::write(declaration.join("repos.tf"), &repos)
        .expect("restore this change's repository declaration");
    fs::copy(
        repository_root().join("infra/security.tf"),
        declaration.join("security.tf"),
    )
    .expect("restore this change's security declaration");
}

/// Adopt the second batch on top of the inherited applied state, record the
/// state snapshot and restart the fake so the scenario's journal is its own.
fn ensure_base_state(w: &mut DutyWorld) {
    ensure_base_adoption(w);
    if state_addresses(w)
        .iter()
        .any(|address| address == "github_repository_dependabot_security_updates.mono")
    {
        return;
    }

    let declaration = declaration_dir(w);
    w.batch = Some(declaration.join("adoption-batch-security.txt"));
    adopt(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the second batch is adopted; output was:\n{}",
        adoption_output(w)
    );
    assert!(
        w.stdout
            .contains("adopted github_repository_vulnerability_alerts.mono duty2cal")
            && w.stdout
                .contains("adopted github_repository_dependabot_security_updates.mono duty2cal"),
        "the second batch binds both settings; output was:\n{}",
        adoption_output(w)
    );

    w.applied_state_snapshot = Some(state_values(w));

    restart_fake_with(w, true);
    w.exit_status = None;
    w.stdout.clear();
    w.stderr.clear();
}

/// The preview phase of the routine: stale check, `tofu init`,
/// `tofu plan -out=plan.bin`, `tofu show -json`, and the unadopted-setting
/// guard over the saved plan's JSON.
fn preview(w: &mut DutyWorld) {
    let declaration = declaration_dir(w);
    let dir = scenario_dir_for(w);
    let report = dir.join("preview.json");
    let stale = stale_path(w);
    let mut transcript = String::from("$ preview (tofu init; tofu plan -out; tofu show -json)\n");

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

    let init = tofu(w, &["init", "-input=false"]);
    transcript.push_str("$ tofu init -input=false\n");
    transcript.push_str(&combined(&init));
    if !init.status.success() {
        w.exit_status = init.status.code();
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    let plan = tofu(w, &["plan", "-input=false", "-no-color", "-out=plan.bin"]);
    transcript.push_str("$ tofu plan -input=false -no-color -out=plan.bin\n");
    transcript.push_str(&combined(&plan));
    if !plan.status.success() {
        w.exit_status = plan.status.code();
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    let show = tofu(w, &["show", "-json", "plan.bin"]);
    fs::write(&report, &show.stdout).expect("write the saved plan's JSON report");
    transcript.push_str("$ tofu show -json plan.bin\n");
    transcript.push_str(&combined(&show));
    if !show.status.success() {
        w.exit_status = show.status.code();
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    let verdict = jq(w, GUARD_FILTER, &report, false);
    if verdict != "ok" {
        transcript.push_str(&format!("refused: adopt the setting first: {verdict}\n"));
        w.exit_status = Some(2);
    } else {
        w.plan_path = Some(declaration.join("plan.bin"));
        w.exit_status = Some(0);
    }
    w.preview_transcript = transcript.clone();
    w.scenario_transcript.push_str(&transcript);
}

/// The apply phase: stale check, the read-only refresh-only preflight with the
/// report as the verdict, the saved plan's application, and the stale marker
/// on a failure that changed the state.
fn apply_previewed(w: &mut DutyWorld) {
    let declaration = declaration_dir(w);
    let dir = scenario_dir_for(w);
    let before = dir.join("state-before-values.json");
    let state_before = dir.join("state-before.tfstate");
    let failed_state = dir.join("state-failed.tfstate");
    let failure_report = dir.join("post-failure.json");
    let report = dir.join("preflight.json");
    let stale = stale_path(w);
    let plan = w
        .plan_path
        .clone()
        .unwrap_or_else(|| declaration.join("plan.bin"));
    assert!(
        plan.is_file(),
        "the scenario holds no saved plan at {}",
        plan.display()
    );
    let mut transcript = String::from("$ apply (preflight; tofu apply plan.bin)\n");

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

    let values_before = state_values(w);
    fs::write(&before, &values_before).expect("record the state's values before the apply");
    fs::copy(state_path(w), &state_before).expect("record the applied state before the apply");

    let preflight = tofu(
        w,
        &[
            "plan",
            "-refresh-only",
            "-detailed-exitcode",
            "-json",
            "-input=false",
        ],
    );
    fs::write(&report, &preflight.stdout).expect("write the preflight's JSON report");
    transcript.push_str("$ tofu plan -refresh-only -detailed-exitcode -json -input=false\n");
    if !preflight.stderr.is_empty() {
        transcript.push_str(&String::from_utf8_lossy(&preflight.stderr));
    }
    let verdict = jq(w, PREFLIGHT_FILTER, &report, true);
    if verdict != "ok" {
        let named = verdict
            .split_whitespace()
            .map(|address| {
                let id = object_live_id(w, address);
                if id.is_empty() {
                    address.to_string()
                } else {
                    format!("{address} ({id})")
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        transcript.push_str(&format!(
            "refused: live changed after the preview: {named}\n"
        ));
        w.apply_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        w.exit_status = Some(2);
        return;
    }

    let apply = tofu(w, &["apply", "-input=false", "plan.bin"]);
    transcript.push_str("$ tofu apply -input=false plan.bin\n");
    transcript.push_str(&combined(&apply));
    if !apply.status.success() {
        // A failed provider update still makes OpenTofu persist the planned
        // values. Restore the pre-apply state, then ask live: a refresh-only
        // report with no drift proves the refusal happened before any write,
        // so the pre-apply state stays and no marker is written. Drift proves
        // a write landed, so the failed apply's state is kept and marked
        // stale (design.md "The apply procedure").
        fs::copy(state_path(w), &failed_state).expect("record the failed apply's state");
        fs::copy(&state_before, state_path(w)).expect("restore the pre-apply state");
        let drift = tofu(
            w,
            &[
                "plan",
                "-refresh-only",
                "-detailed-exitcode",
                "-json",
                "-input=false",
            ],
        );
        fs::write(&failure_report, &drift.stdout).expect("write the post-failure report");
        let drift_verdict = jq(w, PREFLIGHT_FILTER, &failure_report, true);
        if drift_verdict == "ok" {
            let _ = fs::remove_file(&failed_state);
            transcript
                .push_str("refused before the first write; the pre-apply state is restored\n");
        } else if drift_verdict == "incomplete" {
            let _ = fs::remove_file(&failed_state);
            transcript.push_str(
                "refused: live could not be verified after the failed apply; \
                 the applied state is unverified\n",
            );
        } else {
            fs::copy(&failed_state, state_path(w)).expect("keep the failed apply's state");
            let _ = fs::remove_file(&failed_state);
            let stamp = Command::new("date")
                .args(["-u", "+%FT%TZ"])
                .output()
                .expect("read the current UTC time");
            let stamp = String::from_utf8_lossy(&stamp.stdout).trim().to_string();
            fs::write(&stale, format!("{stamp}\n")).expect("write the stale marker");
            transcript.push_str(&format!(
                "stale applied state marked at {}; live drift: {drift_verdict}\n",
                stale.display()
            ));
        }
        w.stdout = String::from_utf8_lossy(&apply.stdout).into_owned();
        w.stderr = String::from_utf8_lossy(&apply.stderr).into_owned();
        w.exit_status = apply.status.code();
    } else {
        let _ = fs::remove_file(&plan);
        let _ = fs::remove_file(dir.join("preview.json"));
        let _ = fs::remove_file(&report);
        let _ = fs::remove_file(&before);
        let _ = fs::remove_file(&stale);
        w.stdout = String::from_utf8_lossy(&apply.stdout).into_owned();
        w.stderr = String::from_utf8_lossy(&apply.stderr).into_owned();
        w.exit_status = Some(0);
    }
    w.apply_transcript = transcript.clone();
    w.scenario_transcript.push_str(&transcript);
}

/// Assert the fake saw no write request at all.
fn assert_no_write_request(w: &DutyWorld) {
    let journal = journal_lines(w);
    for line in &journal {
        let writes = line.contains("\"applied_write\": true")
            || line.contains("\"manual_edit\": true")
            || line.contains("\"refused_write\": true")
            || line.contains("\"method\": \"PATCH\"")
            || line.contains("\"method\": \"PUT\"")
            || line.contains("\"method\": \"DELETE\"");
        assert!(
            !writes,
            "no write request may reach GitHub; journal was:\n{journal:#?}"
        );
    }
    assert!(
        !journal.is_empty(),
        "the preview read the live configuration; journal was empty"
    );
}

#[given("the applied state binds every declared object")]
async fn the_applied_state_binds_every_declared_object(w: &mut DutyWorld) {
    ensure_base_state(w);
}

#[given("a previewed revision that enables secret scanning and push protection")]
async fn a_previewed_revision_that_enables_the_scanning_settings(w: &mut DutyWorld) {
    ensure_base_state(w);
    preview(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the preview succeeds; output was:\n{}",
        w.preview_transcript
    );
}

#[given("the owner's authority cannot read the monorepo's security settings")]
async fn the_owners_authority_cannot_read_the_monorepos_security_settings(w: &mut DutyWorld) {
    ensure_base_state(w);
    let fixtures = fixtures_dir(w);
    fs::write(
        fixtures.join("faults.json"),
        "[{\"path_regex\": \"^/repos/duty2cal/duty2cal$\", \"method\": \"GET\", \
         \"status\": 403, \"body\": {\"message\": \"Resource not accessible by integration\"}}]\n",
    )
    .expect("write the unreadable-repository fault");
    restart_fake_with(w, true);
}

#[given("the owner's authority cannot change the monorepo")]
async fn the_owners_authority_cannot_change_the_monorepo(w: &mut DutyWorld) {
    ensure_base_state(w);
    let fixtures = fixtures_dir(w);
    fs::write(
        fixtures.join("faults.json"),
        "[{\"path_regex\": \"^/repos/duty2cal/duty2cal$\", \"method\": \"PATCH\", \
         \"status\": 403, \"body\": {\"message\": \"Resource not accessible by integration\"}}]\n",
    )
    .expect("write the refused-change fault");
    restart_fake_with(w, true);
}

#[when("the organization owner previews the declaration")]
async fn the_organization_owner_previews_the_declaration(w: &mut DutyWorld) {
    preview(w);
}

#[when("the organization owner applies the previewed revision")]
async fn the_organization_owner_applies_the_previewed_revision(w: &mut DutyWorld) {
    apply_previewed(w);
}

#[then(regex = r#"^the preview fails and names "([^"]+)"$"#)]
async fn the_preview_fails_and_names(w: &mut DutyWorld, object: String) {
    assert!(
        w.exit_status != Some(0),
        "the preview fails; output was:\n{}",
        w.preview_transcript
    );
    assert!(
        w.preview_transcript.contains(&object),
        "the preview names {object}; output was:\n{}",
        w.preview_transcript
    );
}

#[then("no write request reaches GitHub")]
async fn no_write_request_reaches_github(w: &mut DutyWorld) {
    assert_no_write_request(w);
}

#[then("the applied state is unchanged")]
async fn the_applied_state_is_unchanged(w: &mut DutyWorld) {
    let values = state_values(w);
    let before = w
        .applied_state_snapshot
        .as_ref()
        .expect("the scenario snapshotted no applied state");
    assert_eq!(
        &values,
        before,
        "the applied state's values at {} changed",
        state_path(w).display()
    );
    // A failed apply's routine keeps a byte copy of the pre-apply state; a
    // refusal before the first write restores it exactly.
    let routine_copy = scenario_dir_for(w).join("state-before.tfstate");
    if routine_copy.is_file() {
        let restored = fs::read(state_path(w)).expect("read the restored applied state");
        let pre_apply = fs::read(&routine_copy).expect("read the routine's state copy");
        assert_eq!(
            restored,
            pre_apply,
            "the applied state at {} was not restored byte for byte",
            state_path(w).display()
        );
    }
}

#[then(regex = r#"^the apply fails and names "([^"]+)"$"#)]
async fn the_apply_fails_and_names(w: &mut DutyWorld, object: String) {
    assert!(
        w.exit_status != Some(0),
        "the apply fails; output was:\n{}",
        w.apply_transcript
    );
    assert!(
        w.apply_transcript.contains(&object),
        "the apply names {object}; output was:\n{}",
        w.apply_transcript
    );
}

#[then("no setting changes live")]
async fn no_setting_changes_live(w: &mut DutyWorld) {
    let journal = journal_lines(w);
    assert!(
        journal
            .iter()
            .all(|line| !line.contains("\"applied_write\": true")),
        "no setting may change live; journal was:\n{journal:#?}"
    );
    assert!(
        journal
            .iter()
            .all(|line| !line.contains("\"manual_edit\": true")),
        "no manual edit was made; journal was:\n{journal:#?}"
    );
}

#[then("the applied state is not marked stale")]
async fn the_applied_state_is_not_marked_stale(w: &mut DutyWorld) {
    let stale = stale_path(w);
    assert!(
        !stale.exists(),
        "a refusal before the first write writes no stale marker at {}",
        stale.display()
    );
}

#[then("a later preview names the same change again")]
async fn a_later_preview_names_the_same_change_again(w: &mut DutyWorld) {
    preview(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the later preview succeeds; output was:\n{}",
        w.preview_transcript
    );
    for expected in [
        "secret_scanning",
        "secret_scanning_push_protection",
        "1 to change",
    ] {
        assert!(
            w.preview_transcript.contains(expected),
            "the later preview names {expected}; output was:\n{}",
            w.preview_transcript
        );
    }
}

#[given("the monorepo's projects are switched on by hand after the preview")]
async fn the_monorepos_projects_are_switched_on_by_hand(w: &mut DutyWorld) {
    assert!(
        w.plan_path.is_some(),
        "the scenario holds no previewed revision"
    );
    let report = live_monorepo_report(w);
    assert_eq!(
        jq(w, ".has_projects", &report, false),
        "false",
        "the preview saw the projects off"
    );
    let (status, body) = fake_request(
        w,
        "PATCH",
        "/repos/duty2cal/duty2cal",
        &[
            ("Content-Type", "application/json"),
            ("X-Fake-Manual-Edit", "true"),
        ],
        Some("{\"has_projects\": true}"),
    );
    assert_eq!(status, 200, "the manual edit applies; response was: {body}");
}

#[then(regex = r#"^the apply is refused before any write and names "([^"]+)"$"#)]
async fn the_apply_is_refused_before_any_write_and_names(w: &mut DutyWorld, object: String) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the apply is refused; output was:\n{}",
        w.apply_transcript
    );
    assert!(
        w.apply_transcript.contains(&object),
        "the apply names {object}; output was:\n{}",
        w.apply_transcript
    );
    let journal = journal_lines(w);
    assert!(
        journal
            .iter()
            .all(|line| !line.contains("\"applied_write\": true")),
        "the refused apply wrote nothing; journal was:\n{journal:#?}"
    );
    assert!(
        journal
            .iter()
            .all(|line| !line.contains("\"refused_write\": true")),
        "the refused apply attempted no write; journal was:\n{journal:#?}"
    );
}

#[then("the only live change is the manual edit")]
async fn the_only_live_change_is_the_manual_edit(w: &mut DutyWorld) {
    let journal = journal_lines(w);
    let manual = journal
        .iter()
        .filter(|line| line.contains("\"manual_edit\": true"))
        .count();
    assert_eq!(
        manual, 1,
        "exactly one manual edit was made; journal was:\n{journal:#?}"
    );
    assert!(
        journal
            .iter()
            .all(|line| !line.contains("\"applied_write\": true")),
        "no applied write landed; journal was:\n{journal:#?}"
    );
    let report = live_monorepo_report(w);
    assert_eq!(
        jq(w, ".has_projects", &report, false),
        "true",
        "the manual edit is live"
    );
    for field in ["secret_scanning", "secret_scanning_push_protection"] {
        assert_eq!(
            jq(
                w,
                &format!(".security_and_analysis.{field}.status // empty"),
                &report,
                false
            ),
            "disabled",
            "the saved plan's {field} change did not land"
        );
    }
}

#[then("the saved plan is not applied")]
async fn the_saved_plan_is_not_applied(w: &mut DutyWorld) {
    let plan = w.plan_path.clone().expect("the preview saved a plan");
    assert!(
        plan.is_file(),
        "the routine leaves the saved plan in place at {}",
        plan.display()
    );
    assert_scanning_status(w, "disabled");
    let journal = journal_lines(w);
    assert!(
        journal
            .iter()
            .all(|line| !line.contains("\"applied_write\": true")),
        "the saved plan was not applied; journal was:\n{journal:#?}"
    );
}

#[then("a new preview names the projects change")]
async fn a_new_preview_names_the_projects_change(w: &mut DutyWorld) {
    preview(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the new preview succeeds; output was:\n{}",
        w.preview_transcript
    );
    assert!(
        w.preview_transcript.contains("has_projects"),
        "the new preview names the projects change; output was:\n{}",
        w.preview_transcript
    );
}

/// Assert the applied state binds neither Dependabot setting.
fn assert_dependabot_unbound(w: &DutyWorld) {
    let addresses = state_addresses(w);
    for address in [
        "github_repository_vulnerability_alerts.mono",
        "github_repository_dependabot_security_updates.mono",
    ] {
        assert!(
            !addresses.iter().any(|recorded| recorded == address),
            "the applied state binds no {address}; it was: {addresses:?}"
        );
    }
}

#[given(
    "the applied state binds the organization, both repositories, the default-branch rule \
     and the site source"
)]
async fn the_applied_state_binds_the_base_objects(w: &mut DutyWorld) {
    ensure_base_adoption(w);
    let addresses = state_addresses(w);
    for address in [
        "github_organization_settings.org",
        "github_repository.mono",
        "github_repository.pages",
        "github_repository_ruleset.main",
        "github_repository_pages.pages",
    ] {
        assert!(
            addresses.iter().any(|recorded| recorded == address),
            "the applied state binds {address}; it was: {addresses:?}"
        );
    }
}

#[given("the applied state does not bind either Dependabot setting")]
async fn the_applied_state_does_not_bind_either_dependabot_setting(w: &mut DutyWorld) {
    assert_dependabot_unbound(w);
    w.applied_state_snapshot = Some(state_values(w));
}

#[given("the monorepo reports Dependabot alerts and Dependabot security updates enabled")]
async fn the_monorepo_reports_dependabot_enabled(w: &mut DutyWorld) {
    let report = live_monorepo_report(w);
    assert_eq!(
        jq(
            w,
            ".security_and_analysis.dependabot_security_updates.status // empty",
            &report,
            false
        ),
        "enabled",
        "the monorepo reports Dependabot security updates enabled"
    );
    let (status, _) = fake_request(
        w,
        "GET",
        "/repos/duty2cal/duty2cal/vulnerability-alerts",
        &[],
        None,
    );
    assert_eq!(
        status, 204,
        "the fake reports the monorepo's Dependabot alerts enabled"
    );
}

#[given("the declaration enables Dependabot alerts and Dependabot security updates")]
async fn the_declaration_enables_the_dependabot_settings(w: &mut DutyWorld) {
    let text = fs::read_to_string(declaration_dir(w).join("security.tf"))
        .expect("read the scenario's security declaration");
    for resource in [
        "github_repository_vulnerability_alerts",
        "github_repository_dependabot_security_updates",
    ] {
        assert!(
            text.contains(&format!("resource \"{resource}\" \"mono\"")),
            "the declaration binds {resource}.mono"
        );
    }
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    assert_eq!(
        normalized.matches("enabled = true").count(),
        2,
        "the declaration enables both Dependabot settings"
    );
}

#[then(regex = r#"^the preview is refused and names "([^"]+)"$"#)]
async fn the_preview_is_refused_and_names(w: &mut DutyWorld, object: String) {
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
        w.preview_transcript.contains(&object),
        "the preview names {object}; output was:\n{}",
        w.preview_transcript
    );
}

#[then(regex = r#"^the preview names "([^"]+)"$"#)]
async fn the_preview_names(w: &mut DutyWorld, object: String) {
    assert!(
        w.preview_transcript.contains(&object),
        "the preview names {object}; output was:\n{}",
        w.preview_transcript
    );
}
