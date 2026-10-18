// SPDX-License-Identifier: MPL-2.0
//! End-to-end steps for the capability `adopt-existing-configuration`.
//!
//! One file per capability. The `@e2e` scenario drives the real adoption
//! command as a subprocess against the local fake GitHub API, asserts what the
//! command printed and reads the fake's request journal, then commits the
//! terminal transcript (design.md "Step definitions" and "Visual Design
//! Contract"). It never calls an adoption routine in process and never touches
//! `api.github.com`.

use std::fs;
use std::path::PathBuf;

use cucumber::{given, then};

use crate::steps::adopt_existing_configuration::{
    applied_state_addresses, applied_state_show, declaration_dir, journal_lines, prepare,
    repository_root,
};
use crate::DutyWorld;

/// The five objects the selected batch binds, in batch order: the organization,
/// both repositories, the default-branch rule and the site source.
const ADOPTED: [(&str, &str); 5] = [
    ("github_organization_settings.org", "343978778"),
    ("github_repository.mono", "duty2cal"),
    ("github_repository.pages", "duty2cal.github.io"),
    ("github_repository_ruleset.main", "duty2cal:24981082"),
    ("github_repository_pages.pages", "duty2cal.github.io"),
];

/// Where the scenario's terminal transcript is committed as evidence. The
/// change owns it while it is active; the archive owns it afterwards.
fn transcript_path() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let active = root.join("givn/changes/declare-public-home");
    let home = if active.is_dir() {
        active
    } else {
        root.join("givn/archive/declare-public-home")
    };
    home.join(
        "evidence/visual/\
         maintainer-adopts-the-existing-public-home-and-every-object-is-bound-without-changing-it/\
         transcript.txt",
    )
}

/// Commit the transcript the consumer read, with the run-specific directory and
/// transport details replaced by placeholders so only the command's output
/// remains (design.md "Visual Design Contract").
fn commit_transcript(w: &DutyWorld) {
    let path = transcript_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create the evidence directory");
    }
    let mut evidence = w
        .scenario_transcript
        .replace(&w.working_dir().display().to_string(), "<scenario-dir>");
    if let Some(port) = w.fake_port {
        evidence = evidence.replace(&format!("127.0.0.1:{port}"), "127.0.0.1:<port>");
    }
    evidence = evidence
        .replace("test-token", "<token>")
        .replace("test-passphrase-0123456789", "<passphrase>");
    // The adoption helper records the invocation's exit status; add one only
    // when the transcript does not already end with it.
    if !evidence
        .trim_end()
        .ends_with(&format!("exit status: {}", w.exit_status.unwrap_or(-1)))
    {
        evidence.push_str(&format!("exit status: {}\n", w.exit_status.unwrap_or(-1)));
    }
    fs::write(&path, evidence).expect("write the transcript");
}

/// The declaration's resource declarations, concatenated, so the Given can
/// name the objects without depending on the file layout.
fn declaration_text(w: &DutyWorld) -> String {
    let mut text = String::new();
    for entry in fs::read_dir(declaration_dir(w)).expect("read the declaration") {
        let path = entry.expect("read a declaration entry").path();
        if path.extension().is_some_and(|extension| extension == "tf") {
            text.push_str(&fs::read_to_string(&path).expect("read a declaration file"));
            text.push('\n');
        }
    }
    text
}

/// The selected batch's entries, comments and blank lines removed.
fn batch_entries(w: &DutyWorld) -> Vec<(String, String)> {
    let batch = w.batch.clone().expect("the scenario prepared no batch");
    fs::read_to_string(&batch)
        .expect("read the selected batch")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (address, import_id) = line
                .split_once('\t')
                .unwrap_or_else(|| panic!("the batch line has no import identifier: {line}"));
            (address.to_owned(), import_id.to_owned())
        })
        .collect()
}

#[given("the public home already exists with its hand-made configuration")]
async fn the_public_home_already_exists(w: &mut DutyWorld) {
    prepare(w);
    assert!(
        w.fake_port.is_some() && w.fake.is_some(),
        "the fake GitHub API must serve the existing hand-made home"
    );
    let fixtures = repository_root().join("scripts/fake-github/fixtures/adopt-all");
    assert!(
        fixtures.join("orgs/duty2cal.json").is_file()
            && fixtures.join("repos/duty2cal/duty2cal.json").is_file()
            && fixtures
                .join("repos/duty2cal/duty2cal.github.io.json")
                .is_file()
            && fixtures
                .join("repos/duty2cal/duty2cal/rulesets/24981082.json")
                .is_file()
            && fixtures
                .join("repos/duty2cal/duty2cal.github.io/pages.json")
                .is_file(),
        "the fake serves the organization, both repositories, the rule set and the Pages source"
    );
}

#[given("the declared configuration names the organization, both repositories, the default-branch rule and the site source")]
async fn the_declared_configuration_names_every_adopted_object(w: &mut DutyWorld) {
    let declaration = declaration_text(w);
    for (address, _) in ADOPTED {
        let (resource_type, resource_name) = address
            .split_once('.')
            .expect("an adopted address is `<type>.<name>`");
        let block = format!("resource \"{resource_type}\" \"{resource_name}\"");
        assert!(
            declaration.contains(&block),
            "the declaration names {address} ({block}); it was:\n{declaration}"
        );
    }
}

#[given("the selected batch names those objects")]
async fn the_selected_batch_names_those_objects(w: &mut DutyWorld) {
    let expected: Vec<(String, String)> = ADOPTED
        .iter()
        .map(|(address, import_id)| ((*address).to_owned(), (*import_id).to_owned()))
        .collect();
    assert_eq!(
        batch_entries(w),
        expected,
        "the selected batch names exactly the five objects"
    );
}

#[then("the adoption reports the organization, both repositories, the default-branch rule and the site source as bound")]
async fn the_adoption_reports_every_object_as_bound(w: &mut DutyWorld) {
    let output = format!("{}{}", w.stdout, w.stderr);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the selected batch is adopted; output was:\n{output}"
    );

    // The primary assertion: what the command printed, one `adopted` line per
    // object, in batch order.
    let expected: Vec<String> = ADOPTED
        .iter()
        .map(|(address, import_id)| format!("adopted {address} {import_id}"))
        .collect();
    let reported: Vec<String> = w
        .stdout
        .lines()
        .filter(|line| line.starts_with("adopted "))
        .map(str::to_owned)
        .collect();
    assert_eq!(
        reported, expected,
        "the command reports the five objects as bound; stdout was:\n{}",
        w.stdout
    );

    // Secondary evidence: every read was served, so no refusal or success
    // passed through the fake's unknown-path marker.
    let journal = journal_lines(w);
    assert!(
        journal
            .iter()
            .all(|line| !line.contains("\"unknown_path\": true")),
        "every live read was served from the fixture tree; journal was:\n{journal:#?}"
    );

    // The binding is recorded in the encrypted applied state, which the real
    // binary can list with the test passphrase.
    let addresses = applied_state_addresses(w);
    for (address, _) in ADOPTED {
        assert!(
            addresses.iter().any(|bound| bound == address),
            "the applied state binds {address}; state was:\n{addresses:#?}"
        );
    }

    commit_transcript(w);
}

#[then("the applied state records the live values rather than the declared ones")]
async fn the_applied_state_records_the_live_values(w: &mut DutyWorld) {
    // The declaration states the decided values (projects off, merge and rebase
    // off, delete-branch on, pages issues and wiki off); the live fixtures hold
    // today's values. Adoption binds identity, so the state must carry the live
    // values, not the decided ones.
    let declaration = declaration_text(w);
    for expected in [
        "has_projects=false",
        "allow_merge_commit=false",
        "allow_rebase_merge=true",
        "allow_squash_merge=false",
        "delete_branch_on_merge=true",
        "ignore_vulnerability_alerts_during_read=false",
        "has_issues=false",
        "has_wiki=false",
    ] {
        assert!(
            declaration.replace(' ', "").contains(expected),
            "the declaration states {expected}; it was:\n{declaration}"
        );
    }

    let mono = applied_state_show(w, "github_repository.mono");
    for expected in [
        "has_projects=true",
        "allow_merge_commit=true",
        "allow_rebase_merge=true",
        "allow_squash_merge=true",
        "delete_branch_on_merge=false",
    ] {
        assert!(
            mono.lines()
                .any(|line| line.trim().replace(' ', "") == expected),
            "the applied state records the monorepo's live {expected}; state was:\n{mono}"
        );
    }

    let pages = applied_state_show(w, "github_repository.pages");
    for expected in ["has_issues=true", "has_wiki=true", "has_projects=true"] {
        assert!(
            pages
                .lines()
                .any(|line| line.trim().replace(' ', "") == expected),
            "the applied state records the pages repository's live {expected}; state was:\n{pages}"
        );
    }
}
