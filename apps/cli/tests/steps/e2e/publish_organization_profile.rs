// SPDX-License-Identifier: MPL-2.0
//! End-to-end step definitions for the capability
//! `publish-organization-profile` (design.md "E2E smoke test infrastructure").
//!
//! The `@e2e` scenarios drive the real `tofu` binary as subprocesses against
//! the local fake GitHub API on loopback, then read the fake's repository,
//! contents and journal. The scenario's terminal transcript is committed under
//! the change's evidence with the temporary path, the port, the token and the
//! passphrase replaced by placeholders. The fake is killed with the scenario's
//! `DutyWorld`.

use std::fs;
use std::path::PathBuf;

use cucumber::{given, then};

use crate::steps::adopt_existing_configuration::{
    fixtures_dir, journal_lines, repository_root, BILLING_EMAIL, PASSPHRASE, TOKEN,
};
use crate::steps::enable_repository_security::{fake_request, jq, state_addresses, state_values};
use crate::steps::publish_organization_profile::{
    ensure_source_tree, preview, published_file, source_tree, write_profile_descriptors,
};
use crate::DutyWorld;

/// The addresses the profile revision declares and the apply binds.
const PROFILE_ADDRESSES: [&str; 3] = [
    "github_repository.dot_github",
    "github_repository_file.org_profile_readme",
    "github_repository_file.org_security",
];

/// The create actions in the saved plan, sorted.
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

/// Where a scenario's terminal transcript is committed as evidence.
fn transcript_path(slug: &str) -> PathBuf {
    repository_root()
        .join("givn/changes/publish-organization-profile/evidence/visual")
        .join(slug)
        .join("transcript.txt")
}

/// Commit the transcript the consumer read, with the run-specific directory and
/// transport details replaced by placeholders so only the command's output
/// remains (design.md "Evidence").
fn commit_transcript(w: &DutyWorld, slug: &str) {
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

/// The scenario directory without mutating the world.
fn scenario_dir_for(w: &DutyWorld) -> PathBuf {
    w.dir
        .clone()
        .expect("scenario prepared no working directory")
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

/// The journal lines the scenario's own preview/apply wrote, after the priming
/// adoption and base apply. The priming writes are outside the change's proof.
fn change_journal(w: &DutyWorld) -> PathBuf {
    let lines = journal_lines(w);
    let path = scenario_dir_for(w).join("change-journal.jsonl");
    let text = lines
        .iter()
        .skip(w.journal_offset)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&path, format!("{text}\n")).expect("write the scenario's own journal");
    path
}

/// No write was refused and no path was unknown during the change.
fn assert_no_refused_or_unknown(w: &DutyWorld, journal: &PathBuf) {
    assert_eq!(
        jq(
            w,
            "[ .[] | select(.refused_write == true or .unknown_path == true) ] | length",
            journal,
            true
        ),
        "0",
        "the change refused no write and read no unknown path; journal was:\n{}",
        fs::read_to_string(journal).unwrap_or_default()
    );
}

#[given(
    "a previewed revision that declares the organization profile repository, its profile \
     content and its default security policy"
)]
async fn a_previewed_revision_declares_the_profile_objects(w: &mut DutyWorld) {
    ensure_source_tree(w);
    preview(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the profile revision previews; output was:\n{}",
        w.preview_transcript
    );
    let report = plan_report(w);
    assert_eq!(
        jq(w, CREATES, &report, false),
        PROFILE_ADDRESSES.join(" "),
        "the previewed revision declares exactly the three profile objects; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );
}

#[then("the apply reports the organization profile repository and both files as created")]
async fn the_apply_reports_the_profile_objects_as_created(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the profile revision applies; output was:\n{}",
        w.apply_transcript
    );
    for expected in [
        "github_repository.dot_github",
        "github_repository_file.org_profile_readme",
        "github_repository_file.org_security",
        "Apply complete! Resources: 3 added, 0 changed, 0 destroyed.",
    ] {
        assert!(
            w.apply_transcript.contains(expected),
            "the apply reports {expected:?}; output was:\n{}",
            w.apply_transcript
        );
    }
}

#[then(
    "the organization profile repository is public and offers no issue tracker, projects or wiki"
)]
async fn the_profile_repository_is_public_and_offers_no_tracker(w: &mut DutyWorld) {
    let (status, body) = fake_request(w, "GET", "/repos/duty2cal/.github", &[], None);
    assert_eq!(
        status, 200,
        "the fake serves the profile repository: {body}"
    );
    let report = scenario_dir_for(w).join("applied-repository.json");
    fs::write(&report, &body).expect("write the live profile repository body");
    for (field, expected) in [
        ("name", ".github"),
        ("full_name", "duty2cal/.github"),
        ("private", "false"),
        ("visibility", "public"),
        ("has_issues", "false"),
        ("has_projects", "false"),
        ("has_wiki", "false"),
        ("has_discussions", "false"),
    ] {
        assert_eq!(
            jq(w, &format!(".{field}"), &report, false),
            expected,
            "the profile repository's {field} is {expected}; body was:\n{body}"
        );
    }
}

#[then(
    regex = r#"^the organization profile repository serves the reviewed profile content as "([^"]+)"$"#
)]
async fn the_profile_repository_serves_the_reviewed_profile_content(
    w: &mut DutyWorld,
    path: String,
) {
    let published = published_file(w, ".github", &path, "profile-content");
    let reviewed = fs::read(source_tree(w).join(&path)).expect("read the reviewed profile content");
    assert_eq!(
        published, reviewed,
        "the fake serves the reviewed profile content byte for byte"
    );
}

#[then(
    regex = r#"^the organization profile repository serves the default security policy as "([^"]+)"$"#
)]
async fn the_profile_repository_serves_the_default_security_policy(
    w: &mut DutyWorld,
    path: String,
) {
    let published = published_file(w, ".github", &path, "security-policy");
    let reviewed = fs::read(source_tree(w).join(&path)).expect("read the reviewed security policy");
    assert_eq!(
        published, reviewed,
        "the fake serves the reviewed default security policy byte for byte"
    );
}

#[then("the applied state records the organization profile repository and both files")]
async fn the_applied_state_records_the_profile_objects(w: &mut DutyWorld) {
    let addresses = state_addresses(w);
    for address in PROFILE_ADDRESSES {
        assert!(
            addresses.iter().any(|recorded| recorded == address),
            "the applied state records {address}; it was: {addresses:?}"
        );
    }
}

#[then("a later targeted preview of the organization profile reports no differences")]
async fn a_later_targeted_preview_reports_no_differences(w: &mut DutyWorld) {
    preview(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the later preview succeeds; output was:\n{}",
        w.preview_transcript
    );
    assert!(
        w.preview_transcript.contains("No changes."),
        "the later preview reports no differences; output was:\n{}",
        w.preview_transcript
    );

    let journal = change_journal(w);
    assert_eq!(
        jq(
            w,
            "[ .[] | select(.applied_write == true and .method == \"POST\") ] | length",
            &journal,
            true
        ),
        "1",
        "the apply creates the profile repository once; journal was:\n{}",
        fs::read_to_string(&journal).unwrap_or_default()
    );
    assert_eq!(
        jq(
            w,
            "[ .[] | select(.applied_write == true and .method == \"POST\") ][0].path",
            &journal,
            true
        ),
        "/orgs/duty2cal/repos",
        "the single create targets the organization's repositories"
    );
    assert_eq!(
        jq(
            w,
            "[ .[] | select(.applied_write == true and .method == \"PUT\") \
             | .path ] | sort | join(\" \")",
            &journal,
            true
        ),
        "/repos/duty2cal/.github/contents/SECURITY.md \
         /repos/duty2cal/.github/contents/profile/README.md",
        "the apply writes exactly the two reviewed files; journal was:\n{}",
        fs::read_to_string(&journal).unwrap_or_default()
    );
    assert_eq!(
        jq(
            w,
            "[ .[] | select(.applied_write == true and .method == \"PATCH\") \
             | .path ] | unique | join(\" \")",
            &journal,
            true
        ),
        "/repos/duty2cal/.github",
        "every follow-up write targets the created profile repository"
    );
    assert_no_refused_or_unknown(w, &journal);
    commit_transcript(
        w,
        "organization-owner-applies-the-previewed-revision-and-the-organization-profile-is-published",
    );
}

// ---------------------------------------------------------------------------
// Scenario 6: preview only the three creations
// ---------------------------------------------------------------------------

#[given("the live organization has no organization profile repository")]
async fn the_live_organization_has_no_profile_repository(w: &mut DutyWorld) {
    // The primed base state is the security-applied tree; the profile
    // scenario adds the four 404 descriptors its own proof reads. The fixture
    // loader reads them from disk, so no fake restart is needed.
    write_profile_descriptors(&fixtures_dir(w));
    w.journal_offset = journal_lines(w).len();
    let (status, body) = fake_request(w, "GET", "/repos/duty2cal/.github", &[], None);
    assert_eq!(
        status, 404,
        "the live organization has no profile repository; response was: {body}"
    );
}

#[then("the preview names the organization profile repository as the only object to create")]
async fn the_preview_names_the_profile_repository_as_the_only_create(w: &mut DutyWorld) {
    let report = plan_report(w);
    let creates = jq(w, CREATES, &report, false);
    assert!(
        creates
            .split_whitespace()
            .any(|address| address == "github_repository.dot_github"),
        "the preview creates the profile repository; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );
    assert_eq!(
        creates.split_whitespace().count(),
        3,
        "the preview creates only the three profile objects; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );
}

#[then(
    "the preview names the profile content and the default security policy as the only files to \
     create"
)]
async fn the_preview_names_the_two_files_as_the_only_creates(w: &mut DutyWorld) {
    let report = plan_report(w);
    assert_eq!(
        jq(
            w,
            "[ .resource_changes[]? \
             | select((.change.actions // []) == [\"create\"]) \
             | select(.address | startswith(\"github_repository_file.\")) \
             | .address ] | sort | join(\" \")",
            &report,
            false
        ),
        "github_repository_file.org_profile_readme github_repository_file.org_security",
        "the preview creates exactly the two reviewed files; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );
}

#[then("the preview names no change other than the three creations")]
async fn the_preview_names_no_change_other_than_the_three_creations(w: &mut DutyWorld) {
    let report = plan_report(w);
    let mut expected: Vec<String> = PROFILE_ADDRESSES
        .iter()
        .map(|address| format!("{address} create"))
        .collect();
    expected.sort();
    assert_eq!(
        jq(w, CHANGES, &report, false),
        expected.join(" "),
        "the preview names exactly the three creations; report was:\n{}",
        fs::read_to_string(&report).unwrap_or_default()
    );

    let journal = change_journal(w);
    assert_eq!(
        jq(
            w,
            "[ .[] | select(.applied_write == true) ] | length",
            &journal,
            true
        ),
        "0",
        "the preview issued no write; journal was:\n{}",
        fs::read_to_string(&journal).unwrap_or_default()
    );
    assert_no_refused_or_unknown(w, &journal);
    let values = state_values(w);
    let before = w
        .applied_state_snapshot
        .as_ref()
        .expect("the scenario snapshotted no applied state");
    assert_eq!(
        &values,
        before,
        "the preview changed no applied state at {}",
        crate::steps::adopt_existing_configuration::declaration_dir(w).display()
    );
    commit_transcript(
        w,
        "organization-owner-previews-the-profile-declaration-and-only-the-three-creations-are-named",
    );
}
