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

/// Scenario sequence, so concurrently executed scenarios never share a
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
fn scenario_dir(w: &mut DutyWorld) -> PathBuf {
    if let Some(dir) = &w.dir {
        return dir.clone();
    }
    let dir = fresh_dir();
    w.dir = Some(dir.clone());
    dir
}

/// Copy the declaration into the scenario, excluding the machine-local cache,
/// the staging workspace and the applied state (the wrapper copies the state
/// itself, and a scenario owns the state it starts from).
fn copy_declaration(source: &Path, target: &Path) {
    fs::create_dir_all(target).expect("create the copied declaration");
    for entry in fs::read_dir(source).expect("read the declaration") {
        let entry = entry.expect("read a declaration entry");
        let name = entry.file_name();
        let text = name.to_string_lossy();
        if matches!(
            text.as_ref(),
            ".terraform"
                | ".adoption-staging"
                | "terraform.tfstate"
                | "terraform.tfstate.backup"
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
    let script = repository_root().join("scripts/fake-github/serve.py");
    let mut child = Command::new("python3")
        .arg(&script)
        .arg("--fixtures")
        .arg(fixtures)
        .arg("--port")
        .arg("0")
        .arg("--journal")
        .arg(journal)
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
fn terraformrc() -> PathBuf {
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

/// Prepare the temporary declaration, batch, state path and fake API.
fn prepare(w: &mut DutyWorld) -> PathBuf {
    let dir = scenario_dir(w);
    let declaration = dir.join("infra");
    copy_declaration(&repository_root().join("infra"), &declaration);

    let fixtures = dir.join("fixtures");
    fs::create_dir_all(&fixtures).expect("create the fixture directory");
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
    let batch = w
        .batch
        .clone()
        .expect("the scenario prepared no batch");
    let state = state_path(w);
    let port = w.fake_port.expect("the scenario started no fake API");

    let rc = terraformrc();
    assert!(
        rc.is_file(),
        "run ./scripts/setup-infra-tests.sh first: {} is absent",
        rc.display()
    );

    let script = repository_root().join("scripts/adopt-public-home.sh");
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
        .env("TF_VAR_state_passphrase", PASSPHRASE)
        .env("TF_VAR_billing_email", BILLING_EMAIL)
        .env("TF_CLI_CONFIG_FILE", &rc)
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
    assert_eq!(
        w.exit_status,
        Some(2),
        "a refusal exits 2; output was:\n{output}"
    );
    assert!(
        output.contains("refused "),
        "a refusal prints `refused <object>: <reason>`; output was:\n{output}"
    );
    assert!(
        output.contains(object),
        "the refusal names {object}; output was:\n{output}"
    );
}

/// The fake's journal as lines, asserting it exists.
fn journal_lines(w: &DutyWorld) -> Vec<String> {
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
    fs::write(
        declaration_dir(w).join("invalid.tf"),
        INVALID_DECLARATION,
    )
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

#[when("the organization owner adopts the batch")]
async fn the_organization_owner_adopts_the_batch(w: &mut DutyWorld) {
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
    let staging = declaration_dir(w).join(".adoption-staging");
    assert!(
        !staging.exists(),
        "the staging workspace is removed on refusal: {}",
        staging.display()
    );
}
