// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `adopt-existing-configuration`.
//!
//! One file per capability. Every scenario drives the real adoption command
//! (`scripts/adopt-public-home.sh`) as a subprocess, with a real `tofu` binary
//! behind it and the local fake GitHub API on loopback (design.md "Test seam"
//! and "Step definitions"). The steps never call the command in process.
#![allow(clippy::too_many_arguments)] // step signatures mirror the captured step text

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use cucumber::{given, then, when};

use crate::DutyWorld;

/// The test-only passphrase: 16-plus characters, supplied through the
/// environment and never committed as a value the command can default to.
pub(crate) const PASSPHRASE: &str = "test-passphrase-0123456789";
/// The other passphrase a scenario uses to prime an applied state it cannot
/// adopt against.
pub(crate) const OTHER_PASSPHRASE: &str = "another-passphrase-9876543210";
/// The declaration's required billing email, satisfied only for the suite.
pub(crate) const BILLING_EMAIL: &str = "billing@example.test";
/// The token the provider sees; the fake never validates it.
pub(crate) const TOKEN: &str = "test-token";

/// The declaration file that makes `tofu validate` fail with a named error.
const INVALID_DECLARATION: &str = "\
resource \"github_repository\" \"invalid\" {
  name     = \"invalid\"
  nonsense = true
}
";

/// The addresses the applied state binds, read with the real binary.
pub(crate) fn applied_state_addresses(w: &DutyWorld) -> Vec<String> {
    let declaration = declaration_dir(w);
    let rc = terraformrc();
    let passphrase = w
        .passphrase
        .clone()
        .unwrap_or_else(|| PASSPHRASE.to_string());
    let init = Command::new("tofu")
        .args(["init", "-backend=false", "-input=false", "-no-color"])
        .current_dir(&declaration)
        .env("TF_CLI_CONFIG_FILE", &rc)
        .env("TF_VAR_state_passphrase", &passphrase)
        .env("TF_VAR_billing_email", BILLING_EMAIL)
        .output()
        .expect("run tofu init against the declaration");
    assert!(
        init.status.success(),
        "tofu init failed: {}{}",
        String::from_utf8_lossy(&init.stdout),
        String::from_utf8_lossy(&init.stderr)
    );
    let list = Command::new("tofu")
        .args(["state", "list"])
        .current_dir(&declaration)
        .env("TF_CLI_CONFIG_FILE", &rc)
        .env("TF_VAR_state_passphrase", &passphrase)
        .env("TF_VAR_billing_email", BILLING_EMAIL)
        .output()
        .expect("run tofu state list against the applied state");
    assert!(
        list.status.success(),
        "tofu state list failed: {}{}",
        String::from_utf8_lossy(&list.stdout),
        String::from_utf8_lossy(&list.stderr)
    );
    String::from_utf8_lossy(&list.stdout)
        .lines()
        .map(str::to_owned)
        .collect()
}

/// The applied state's rendered object, so a scenario can assert that the
/// binding records the live values rather than the declared ones. The state is
/// encrypted; the test passphrase decrypts it in the temporary declaration.
pub(crate) fn applied_state_show(w: &DutyWorld, address: &str) -> String {
    let declaration = declaration_dir(w);
    let rc = terraformrc();
    let passphrase = w
        .passphrase
        .clone()
        .unwrap_or_else(|| PASSPHRASE.to_string());
    let init = Command::new("tofu")
        .args(["init", "-backend=false", "-input=false", "-no-color"])
        .current_dir(&declaration)
        .env("TF_CLI_CONFIG_FILE", &rc)
        .env("TF_VAR_state_passphrase", &passphrase)
        .env("TF_VAR_billing_email", BILLING_EMAIL)
        .output()
        .expect("run tofu init against the declaration");
    assert!(
        init.status.success(),
        "tofu init failed: {}{}",
        String::from_utf8_lossy(&init.stdout),
        String::from_utf8_lossy(&init.stderr)
    );
    let show = Command::new("tofu")
        .args(["state", "show", address])
        .current_dir(&declaration)
        .env("TF_CLI_CONFIG_FILE", &rc)
        .env("TF_VAR_state_passphrase", &passphrase)
        .env("TF_VAR_billing_email", BILLING_EMAIL)
        .output()
        .expect("run tofu state show against the applied state");
    assert!(
        show.status.success(),
        "tofu state show {address} failed: {}{}",
        String::from_utf8_lossy(&show.stdout),
        String::from_utf8_lossy(&show.stderr)
    );
    String::from_utf8_lossy(&show.stdout).to_string()
}

/// The scenario sequence, so concurrently executed scenarios never share a
/// directory, a fake or a state.
static SCENARIO_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

/// The repository root: the command and the fake are invoked from here.
pub(crate) fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Fresh per-scenario working directory.
fn fresh_dir() -> PathBuf {
    let sequence = SCENARIO_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "duty2cal-adoption-{}-{sequence}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create the scenario directory");
    dir
}

/// The scenario's working directory, created on first use.
pub(crate) fn scenario_dir(w: &mut DutyWorld) -> PathBuf {
    if let Some(dir) = &w.dir {
        return dir.clone();
    }
    let dir = fresh_dir();
    w.dir = Some(dir.clone());
    dir
}

/// Copy a directory tree (the declaration or the fake's fixtures).
pub(crate) fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).expect("create a copied directory");
    for entry in fs::read_dir(source).expect("read a source directory") {
        let entry = entry.expect("read a source entry");
        let from = entry.path();
        let to = target.join(entry.file_name());
        if from.is_dir() {
            copy_tree(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy a source file");
        }
    }
}

/// Copy the declaration into the scenario, excluding the machine-local cache,
/// the staging workspace and the applied state (the wrapper copies the state
/// itself, and a scenario owns the state it starts from).
pub(crate) fn copy_declaration(source: &Path, target: &Path) {
    fs::create_dir_all(target).expect("create the copied declaration");
    for entry in fs::read_dir(source).expect("read the declaration") {
        let entry = entry.expect("read a declaration entry");
        let name = entry.file_name();
        let text = name.to_string_lossy();
        if matches!(
            text.as_ref(),
            ".terraform" | ".adoption-staging" | "terraform.tfstate" | "terraform.tfstate.backup"
        ) {
            continue;
        }
        let from = entry.path();
        let to = target.join(&name);
        if from.is_dir() {
            copy_declaration(&from, &to);
        } else {
            fs::copy(&from, &to).expect("copy a declaration file");
        }
    }
}

/// Start the fake GitHub API on an ephemeral loopback port and return it with
/// the port it printed.
pub(crate) fn start_fake(fixtures: &Path, journal: &Path) -> (Child, u16) {
    start_fake_with(fixtures, journal, false)
}

/// Start the fake with the test-only write sandbox enabled when `writes` is
/// set (design.md "Write sandbox").
pub(crate) fn start_fake_with(fixtures: &Path, journal: &Path, writes: bool) -> (Child, u16) {
    let script = repository_root().join("scripts/fake-github/serve.py");
    let mut command = Command::new("python3");
    command
        .arg(&script)
        .arg("--fixtures")
        .arg(fixtures)
        .arg("--port")
        .arg("0")
        .arg("--journal")
        .arg(journal);
    if writes {
        command.arg("--writes");
    }
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("start the fake GitHub API");
    let stdout = child.stdout.take().expect("the fake's stdout pipe");
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("read the fake's bound port");
    let port: u16 = line
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("the fake did not print a port: {line:?}"));
    (child, port)
}

/// The terraformrc `scripts/setup-infra-tests.sh` writes.
pub(crate) fn terraformrc() -> PathBuf {
    let cache = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))
        .expect("the scenario needs a cache directory");
    cache.join("duty2cal/terraformrc")
}

/// The scenario's declaration directory.
pub(crate) fn declaration_dir(w: &DutyWorld) -> PathBuf {
    w.declaration
        .clone()
        .expect("the scenario prepared no declaration")
}

/// The scenario's applied state path.
pub(crate) fn state_path(w: &DutyWorld) -> PathBuf {
    w.state
        .clone()
        .expect("the scenario prepared no state path")
}

/// The scenario's fixture directory: the base fixture set with any scenario
/// overrides (a fault, a mutation, a removed object).
pub(crate) fn fixtures_dir(w: &DutyWorld) -> PathBuf {
    w.dir
        .clone()
        .expect("scenario prepared no directory")
        .join("fixtures")
}

/// Stop the running fake and start a fresh one over the same fixture tree and
/// a fresh journal. Scenario Givens that write `faults.json` or `mutate.json`
/// call this before the adoption, because the fake loads them at startup.
fn restart_fake(w: &mut DutyWorld) {
    restart_fake_with(w, false);
}

/// Restart the fake over the scenario's fixtures, optionally in write-sandbox
/// mode, with a fresh journal.
pub(crate) fn restart_fake_with(w: &mut DutyWorld, writes: bool) {
    if let Some(mut child) = w.fake.take() {
        let _ = child.kill();
        let _ = child.wait();
    }
    let fixtures = fixtures_dir(w);
    let journal = w.journal.clone().expect("the scenario kept no journal");
    let _ = fs::remove_file(&journal);
    let (child, port) = start_fake_with(&fixtures, &journal, writes);
    w.fake = Some(child);
    w.fake_port = Some(port);
    w.write_sandbox = writes;
}

/// Prepare the temporary declaration, batch, state path and fake API.
pub(crate) fn prepare(w: &mut DutyWorld) -> PathBuf {
    let dir = scenario_dir(w);
    let declaration = dir.join("infra");
    copy_declaration(&repository_root().join("infra"), &declaration);

    let fixtures = dir.join("fixtures");
    copy_tree(
        &repository_root().join("scripts/fake-github/fixtures/adopt-all"),
        &fixtures,
    );
    let journal = dir.join("journal.jsonl");
    let (child, port) = start_fake(&fixtures, &journal);

    w.declaration = Some(declaration.clone());
    w.batch = Some(declaration.join("adoption-batch.txt"));
    w.state = Some(declaration.join("terraform.tfstate"));
    w.journal = Some(journal);
    w.fake = Some(child);
    w.fake_port = Some(port);
    dir
}

/// The combined output of the last adoption invocation.
pub(crate) fn adoption_output(w: &DutyWorld) -> String {
    format!("{}{}", w.stdout, w.stderr)
}

/// The adoption command as the design's interaction matrix invokes it.
pub(crate) fn adopt(w: &mut DutyWorld) {
    let declaration = declaration_dir(w);
    let batch = w.batch.clone().expect("the scenario prepared no batch");
    let state = state_path(w);
    let port = w.fake_port.expect("the scenario started no fake API");

    let rc = terraformrc();
    assert!(
        rc.is_file(),
        "run ./scripts/setup-infra-tests.sh first: {} is absent",
        rc.display()
    );

    let script = repository_root().join("scripts/adopt-public-home.sh");
    let passphrase = w
        .passphrase
        .clone()
        .unwrap_or_else(|| PASSPHRASE.to_string());
    let output = Command::new(&script)
        .arg("--declaration")
        .arg(&declaration)
        .arg("--batch")
        .arg(&batch)
        .arg("--state")
        .arg(&state)
        .arg("--api-url")
        .arg(format!("http://127.0.0.1:{port}"))
        .arg("--token")
        .arg(TOKEN)
        .current_dir(repository_root())
        .env("TF_VAR_state_passphrase", passphrase)
        .env("TF_VAR_billing_email", BILLING_EMAIL)
        .env("TF_CLI_CONFIG_FILE", &rc)
        .envs(
            w.env
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str())),
        )
        .output()
        .expect("run the adoption command");

    let status = output.status.code().unwrap_or(-1);
    w.exit_status = Some(status);
    w.stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    w.stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    w.command_transcript = adoption_output(w);

    let mut transcript = format!("$ scripts/adopt-public-home.sh\n{}", w.stdout);
    if !w.stderr.is_empty() {
        transcript.push_str("[stderr]\n");
        transcript.push_str(&w.stderr);
    }
    transcript.push_str(&format!("exit status: {status}\n"));
    w.adoption_transcript = transcript.clone();
    w.scenario_transcript.push_str(&transcript);

    // The fake stays running; the Then steps read its journal.
}

/// Assert the command refused with exit status 2 and a `refused` line.
pub(crate) fn assert_refused(w: &DutyWorld, object: &str) {
    let output = adoption_output(w);
    if w.exit_status != Some(2) {
        let trigger = w
            .env
            .iter()
            .find(|(key, _)| key == "ADOPT_TEST_TRIGGER")
            .map(|(_, value)| value.clone());
        let trigger_state = trigger
            .as_ref()
            .map(|path| (path.clone(), Path::new(path).is_file()));
        let mutation = w
            .dir
            .as_ref()
            .map(|dir| dir.join("fixtures/mutate.json"))
            .and_then(|path| fs::read_to_string(path).ok());
        let journal = journal_lines(w);
        let plan = w
            .env
            .iter()
            .find(|(key, _)| key == "ADOPT_TEST_PLAN_LOG")
            .and_then(|(_, path)| fs::read_to_string(path).ok());
        panic!(
            "a refusal exits 2; output was:\n{output}\ntrigger: {trigger_state:?}\nmutation: {mutation:?}\nplan: {plan:?}\njournal: {journal:#?}"
        );
    }
    assert!(
        output.contains("refused "),
        "a refusal prints `refused <object>: <reason>`; output was:\n{output}"
    );
    assert!(
        output.contains(object),
        "the refusal names {object}; output was:\n{output}"
    );
    // The fake's 599 marker is the second strictness net: a refusal must never
    // be the fake's own missing fixture leaking through as a provider error.
    let journal = journal_lines(w);
    assert!(
        journal
            .iter()
            .all(|line| !line.contains("\"unknown_path\": true")),
        "the refusal read a path the fake does not know; journal was:\n{journal:#?}"
    );
}

/// The fake's journal as lines, asserting it exists.
pub(crate) fn journal_lines(w: &DutyWorld) -> Vec<String> {
    let journal = w.journal.clone().expect("the scenario kept no journal");
    fs::read_to_string(&journal)
        .unwrap_or_else(|error| panic!("read the journal {}: {error}", journal.display()))
        .lines()
        .map(str::to_owned)
        .collect()
}

#[given("the declared configuration fails validation")]
async fn the_declared_configuration_fails_validation(w: &mut DutyWorld) {
    prepare(w);
    fs::write(declaration_dir(w).join("invalid.tf"), INVALID_DECLARATION)
        .expect("write the invalid declaration file");
}

#[given("the selected batch has a line without an import identifier")]
async fn the_selected_batch_has_a_line_without_an_import_identifier(w: &mut DutyWorld) {
    prepare(w);
    let declaration = declaration_dir(w);
    let batch = declaration.join("scenario-batch.txt");
    fs::write(
        &batch,
        "# The selected batch for this scenario.\n\
         github_organization_settings.org\t100000000\n\
         github_repository.mono\n\
         github_repository.pages\tduty2cal.github.io\n",
    )
    .expect("write the malformed batch");
    w.batch = Some(batch);
}

#[given(regex = r#"^the declared configuration names an additional repository "([^"]+)"$"#)]
async fn the_declared_configuration_names_an_additional_repository(
    w: &mut DutyWorld,
    slug: String,
) {
    prepare(w);
    let (_, repository) = slug
        .split_once('/')
        .unwrap_or_else(|| panic!("the scenario names owner/repository, got {slug}"));
    let declaration = declaration_dir(w);
    fs::write(
        declaration.join("example.tf"),
        format!("resource \"github_repository\" \"example\" {{\n  name = \"{repository}\"\n}}\n"),
    )
    .expect("write the additional repository declaration");
    w.missing_repository = Some(slug);
}

#[given("the selected batch names that repository")]
async fn the_selected_batch_names_that_repository(w: &mut DutyWorld) {
    let slug = w
        .missing_repository
        .clone()
        .expect("the scenario named no additional repository");
    let repository = slug
        .split_once('/')
        .expect("the scenario names owner/repository")
        .1;
    let batch = w.batch.clone().expect("the scenario prepared no batch");
    let mut entries = fs::read_to_string(&batch).expect("read the batch");
    if !entries.ends_with('\n') {
        entries.push('\n');
    }
    entries.push_str(&format!("github_repository.example\t{repository}\n"));
    fs::write(&batch, entries).expect("append the additional repository to the batch");
}

#[given("it does not exist live")]
async fn it_does_not_exist_live(w: &mut DutyWorld) {
    let slug = w
        .missing_repository
        .clone()
        .expect("the scenario named no additional repository");
    let fixtures = fixtures_dir(w);
    fs::write(
        fixtures.join("faults.json"),
        format!(
            "[{{\"path_regex\": \"^/repos/{slug}$\", \"status\": 404, \
             \"body\": {{\"message\": \"Not Found\"}}}}]\n"
        ),
    )
    .expect("write the missing-object fault");
    restart_fake(w);
}

#[given("the declared configuration names both repositories")]
async fn the_declared_configuration_names_both_repositories(w: &mut DutyWorld) {
    prepare(w);
    let repos = fs::read_to_string(declaration_dir(w).join("repos.tf"))
        .expect("read the repository declaration");
    assert!(
        repos.contains("resource \"github_repository\" \"mono\"")
            && repos.contains("resource \"github_repository\" \"pages\""),
        "the declaration names both repositories; it was:\n{repos}"
    );
}

#[given("the selected batch names only the organization and the monorepo")]
async fn the_selected_batch_names_only_the_organization_and_the_monorepo(w: &mut DutyWorld) {
    let declaration = declaration_dir(w);
    let batch = declaration.join("scenario-selected-batch.txt");
    fs::write(
        &batch,
        "github_organization_settings.org\t343978778\ngithub_repository.mono\tduty2cal\n",
    )
    .expect("write the selected batch");
    w.batch = Some(batch);
}

#[given("the applied state cannot be written")]
async fn the_applied_state_cannot_be_written(w: &mut DutyWorld) {
    use std::os::unix::fs::PermissionsExt as _;

    prepare(w);
    let state_dir = declaration_dir(w).join("state");
    fs::create_dir_all(&state_dir).expect("create the read-only state directory");
    let mut permissions = fs::metadata(&state_dir)
        .expect("read the state directory metadata")
        .permissions();
    permissions.set_mode(0o555);
    fs::set_permissions(&state_dir, permissions).expect("make the state directory read-only");
    w.state = Some(state_dir.join("terraform.tfstate"));
}

#[given("the applied state was encrypted with a different passphrase")]
async fn the_applied_state_was_encrypted_with_a_different_passphrase(w: &mut DutyWorld) {
    prepare(w);
    let declaration = declaration_dir(w);
    let priming = declaration.join("scenario-monorepo-batch.txt");
    fs::write(&priming, "github_repository.mono\tduty2cal\n").expect("write the priming batch");
    let full_batch = w.batch.clone().expect("the scenario prepared no batch");
    w.batch = Some(priming);
    w.passphrase = Some(OTHER_PASSPHRASE.to_string());
    adopt(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the priming adoption records a state under the other passphrase; output was:\n{}",
        adoption_output(w)
    );
    w.applied_state_snapshot =
        Some(fs::read(state_path(w)).expect("read the primed applied state"));
    w.batch = Some(full_batch);
    w.passphrase = Some(PASSPHRASE.to_string());
    w.exit_status = None;
    w.stdout.clear();
    w.stderr.clear();
}

#[given("the monorepo's live settings change after the adoption reads them")]
async fn the_monorepos_live_settings_change_after_the_adoption_reads_them(w: &mut DutyWorld) {
    prepare(w);
    let fixtures = fixtures_dir(w);
    let repository = fs::read_to_string(fixtures.join("repos/duty2cal/duty2cal.json"))
        .expect("read the monorepo fixture");
    let mutated = repository
        .replace(
            "\"description\": \"Convert a duty plan to a calendar\"",
            "\"description\": \"Changed after the adoption read it\"",
        )
        .replace("\"has_projects\": true", "\"has_projects\": false");
    assert_ne!(
        mutated, repository,
        "the monorepo fixture carries the description the scenario changes"
    );
    fs::write(
        fixtures.join("mutate.json"),
        format!(
            "{{\"/repos/duty2cal/duty2cal\": {{\"trigger_file\": \"mutate.trigger\", \"body\": {mutated}}}}}\n"
        ),
    )
    .expect("write the deterministic monorepo mutation");
    w.env.push((
        "ADOPT_TEST_TRIGGER".to_owned(),
        fixtures.join("mutate.trigger").display().to_string(),
    ));
    let plan_path = scenario_dir(w).join("plan.json");
    w.env.push((
        "ADOPT_TEST_PLAN_LOG".to_owned(),
        plan_path.display().to_string(),
    ));
    restart_fake(w);
}

#[given(
    "the selected batch names the archived repository, which the declared configuration does not name"
)]
async fn the_selected_batch_names_the_archived_repository(w: &mut DutyWorld) {
    prepare(w);
    let declaration = declaration_dir(w);
    let batch = declaration.join("scenario-archived-batch.txt");
    fs::write(&batch, "github_repository.archived\tarchived\n")
        .expect("write the batch naming an undeclared address");
    w.batch = Some(batch);
}

#[given("the organization settings cannot be read with the owner's authority")]
async fn the_organization_settings_cannot_be_read_with_the_owners_authority(w: &mut DutyWorld) {
    prepare(w);
    let fixtures = fixtures_dir(w);
    fs::write(
        fixtures.join("faults.json"),
        "[{\"path_regex\": \"^/orgs/duty2cal$\", \"status\": 401, \
         \"body\": {\"message\": \"Bad credentials\"}}]\n",
    )
    .expect("write the unreadable-organization fault");
    restart_fake(w);
}

#[given("the monorepo is already bound in the applied state")]
async fn the_monorepo_is_already_bound_in_the_applied_state(w: &mut DutyWorld) {
    prepare(w);
    let declaration = declaration_dir(w);
    let priming = declaration.join("scenario-monorepo-batch.txt");
    fs::write(&priming, "github_repository.mono\tduty2cal\n").expect("write the priming batch");
    w.batch = Some(priming);
    adopt(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the priming adoption binds the monorepo; output was:\n{}",
        adoption_output(w)
    );
    assert!(
        adoption_output(w).contains("adopted github_repository.mono duty2cal"),
        "the priming adoption reports the monorepo; output was:\n{}",
        adoption_output(w)
    );
    assert!(
        state_path(w).exists(),
        "the priming adoption records the applied state"
    );
    w.batch = Some(declaration.join("adoption-batch.txt"));
    w.exit_status = None;
    w.stdout.clear();
    w.stderr.clear();
}

#[when("the organization owner adopts the batch")]
async fn the_organization_owner_adopts_the_batch(w: &mut DutyWorld) {
    adopt(w);
}

#[when("the organization owner adopts the batch again")]
async fn the_organization_owner_adopts_the_batch_again(w: &mut DutyWorld) {
    adopt(w);
}

#[then("the adoption refuses the batch and names the declaration error")]
async fn the_adoption_refuses_the_batch_and_names_the_declaration_error(w: &mut DutyWorld) {
    assert_refused(w, "refused declaration:");
    let output = adoption_output(w);
    assert!(
        output.contains("Unsupported argument"),
        "the refusal names the declaration's error; output was:\n{output}"
    );
}

#[then("no object is read from the live organization")]
async fn no_object_is_read_from_the_live_organization(w: &mut DutyWorld) {
    let journal = journal_lines(w);
    assert!(
        journal.is_empty(),
        "no request may reach the live organization; journal was:\n{journal:#?}"
    );
}

#[then("the adoption refuses the batch and names the malformed batch line")]
async fn the_adoption_refuses_the_batch_and_names_the_malformed_batch_line(w: &mut DutyWorld) {
    assert_refused(w, "refused batch:");
    let output = adoption_output(w);
    assert!(
        output.contains("line 3"),
        "the refusal names the malformed line; output was:\n{output}"
    );
    assert!(
        output.contains("github_repository.mono"),
        "the refusal quotes the malformed line; output was:\n{output}"
    );
}

#[then("no object is imported")]
async fn no_object_is_imported(w: &mut DutyWorld) {
    let journal = journal_lines(w);
    assert!(
        journal.is_empty(),
        "no import may run; journal was:\n{journal:#?}"
    );
    assert!(
        !state_path(w).exists(),
        "a refused batch records no applied state"
    );
}

#[then("no object in the batch is bound")]
async fn no_object_in_the_batch_is_bound(w: &mut DutyWorld) {
    let state = state_path(w);
    if let Some(before) = &w.applied_state_snapshot {
        let after = fs::read(&state)
            .unwrap_or_else(|error| panic!("read the applied state {}: {error}", state.display()));
        assert!(
            &after == before,
            "the refused adoption changed the applied state at {}",
            state.display()
        );
    } else {
        assert!(
            !state.exists(),
            "no applied state may be recorded at {}",
            state.display()
        );
        let backup = PathBuf::from(format!("{}.backup", state.display()));
        assert!(
            !backup.exists(),
            "no applied state backup may be recorded at {}",
            backup.display()
        );
    }
    let staging = declaration_dir(w).join(".adoption-staging");
    assert!(
        !staging.exists(),
        "the staging workspace is removed on refusal: {}",
        staging.display()
    );
}

#[then(regex = r#"^the adoption refuses the batch and names "([^"]+)"$"#)]
async fn the_adoption_refuses_the_batch_and_names(w: &mut DutyWorld, object: String) {
    assert_refused(w, &object);
}

#[then("the refusal states that the repository does not exist live")]
async fn the_refusal_states_that_the_repository_does_not_exist_live(w: &mut DutyWorld) {
    let output = adoption_output(w);
    assert!(
        output.contains("does not exist live"),
        "the refusal states the repository does not exist live; output was:\n{output}"
    );
}

/// Assert the journal shows no write attempt: no refused method and no GraphQL
/// mutation. A GraphQL query POST is a read.
pub(crate) fn assert_no_live_write(w: &DutyWorld) {
    let journal = journal_lines(w);
    assert!(
        journal
            .iter()
            .all(|line| !line.contains("\"refused_write\": true")),
        "the adoption attempted a write; journal was:\n{journal:#?}"
    );
    assert!(
        journal
            .iter()
            .all(|line| !line.contains("\"graphql_operation\": \"mutation")),
        "the adoption attempted a GraphQL mutation; journal was:\n{journal:#?}"
    );
}

#[then("the refusal states that the organization settings cannot be read")]
async fn the_refusal_states_that_the_organization_settings_cannot_be_read(w: &mut DutyWorld) {
    assert_refused(w, "github_organization_settings.org");
    let output = adoption_output(w);
    assert!(
        output.contains("cannot be read"),
        "the refusal states the organization settings cannot be read; output was:\n{output}"
    );
}

#[then("the adoption refuses the batch and names the organization settings")]
async fn the_adoption_refuses_the_batch_and_names_the_organization_settings(w: &mut DutyWorld) {
    assert_refused(w, "github_organization_settings.org");
}

#[then("the live configuration is unchanged")]
async fn the_live_configuration_is_unchanged(w: &mut DutyWorld) {
    if crate::steps::publish_site_plumbing::is_pages_scenario(w) {
        crate::steps::publish_site_plumbing::assert_live_unchanged(w);
        return;
    }
    assert_no_live_write(w);
}

#[then("nothing is created in the live organization")]
async fn nothing_is_created_in_the_live_organization(w: &mut DutyWorld) {
    let journal = journal_lines(w);
    assert!(
        !journal.is_empty(),
        "the scenario read the live objects; journal was empty"
    );
    assert_no_live_write(w);
}

#[then("the adoption refuses the batch and names the archived repository")]
async fn the_adoption_refuses_the_batch_and_names_the_archived_repository(w: &mut DutyWorld) {
    assert_refused(w, "github_repository.archived");
    let output = adoption_output(w);
    assert!(
        output.contains("archived"),
        "the refusal names the archived repository; output was:\n{output}"
    );
}

#[then("the adoption refuses the batch and names the applied state")]
async fn the_adoption_refuses_the_batch_and_names_the_applied_state(w: &mut DutyWorld) {
    let state = state_path(w).display().to_string();
    assert_refused(w, &state);
}

#[then("the adoption reports the organization and the monorepo as bound")]
async fn the_adoption_reports_the_organization_and_the_monorepo_as_bound(w: &mut DutyWorld) {
    let output = adoption_output(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the selected batch is adopted; output was:\n{output}"
    );
    assert!(
        output.contains("adopted github_organization_settings.org 343978778"),
        "the adoption reports the organization as bound; output was:\n{output}"
    );
    assert!(
        output.contains("adopted github_repository.mono duty2cal"),
        "the adoption reports the monorepo as bound; output was:\n{output}"
    );
    let adopted: Vec<&str> = output
        .lines()
        .filter(|line| line.starts_with("adopted "))
        .collect();
    assert_eq!(
        adopted.len(),
        2,
        "only the selected batch is reported; output was:\n{output}"
    );
}

#[then("the site repository is not bound")]
async fn the_site_repository_is_not_bound(w: &mut DutyWorld) {
    let addresses = applied_state_addresses(w);
    assert!(
        addresses
            .iter()
            .any(|address| address == "github_organization_settings.org")
            && addresses
                .iter()
                .any(|address| address == "github_repository.mono"),
        "the applied state binds the selected batch; state was:\n{addresses:#?}"
    );
    assert!(
        !addresses
            .iter()
            .any(|address| address == "github_repository.pages"),
        "the site repository is not bound; state was:\n{addresses:#?}"
    );
    assert!(
        !addresses
            .iter()
            .any(|address| address == "github_repository_pages.pages"),
        "the site source is not bound; state was:\n{addresses:#?}"
    );
    assert!(
        !addresses
            .iter()
            .any(|address| address == "github_repository_ruleset.main"),
        "the default-branch rule is not bound; state was:\n{addresses:#?}"
    );
}

#[then("the applied state binds the monorepo once")]
async fn the_applied_state_binds_the_monorepo_once(w: &mut DutyWorld) {
    let addresses = applied_state_addresses(w);
    assert_eq!(
        addresses
            .iter()
            .filter(|address| address.as_str() == "github_repository.mono")
            .count(),
        1,
        "the applied state binds the monorepo exactly once; state was:\n{addresses:#?}"
    );
}
