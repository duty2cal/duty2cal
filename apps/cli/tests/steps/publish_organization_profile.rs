// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `publish-organization-profile`.
//!
//! The non-e2e scenarios publish the reviewed `org/` copy through the real
//! `tofu` binary against the local fake GitHub API (design.md "Step
//! definitions"). The `@e2e` scenarios live in `steps/e2e/` because they
//! commit the scrubbed transcript.
#![allow(clippy::too_many_arguments)] // step signatures mirror the captured step text

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use cucumber::{given, then, when};

use crate::steps::adopt_existing_configuration::{
    copy_declaration, copy_tree, declaration_dir, fixtures_dir, journal_lines, repository_root,
    scenario_dir, start_fake_with, state_path,
};
use crate::steps::enable_repository_security::{
    combined, fake_request, jq, state_addresses, state_values, tofu, PREFLIGHT_FILTER,
};
use crate::DutyWorld;

/// The three resources the profile revision declares, in the design's order.
const TARGETS: [&str; 3] = [
    "-target=github_repository.dot_github",
    "-target=github_repository_file.org_profile_readme",
    "-target=github_repository_file.org_security",
];

/// The addresses the profile apply binds.
const PROFILE_ADDRESSES: [&str; 3] = [
    "github_repository.dot_github",
    "github_repository_file.org_profile_readme",
    "github_repository_file.org_security",
];

/// The private vulnerability reporting route the policy names.
const PRIVATE_REPORTING_ROUTE: &str =
    "https://github.com/duty2cal/duty2cal/security/advisories/new";

/// Whether the scenario primed the profile declaration (`org/` beside
/// `infra/`). The shared preview/apply steps dispatch on it.
pub(crate) fn is_profile_scenario(w: &DutyWorld) -> bool {
    w.source_tree.is_some()
}

/// Copy the reviewed `org/` tree beside the scenario's `infra/`, so the
/// declaration's default `../org` resolves without `TF_VAR_org_source_dir`.
pub(crate) fn ensure_source_tree(w: &mut DutyWorld) -> PathBuf {
    let dir = scenario_dir(w);
    let target = dir.join("org");
    if !target.is_dir() {
        copy_tree(&repository_root().join("org"), &target);
    }
    w.source_tree = Some(target.clone());
    target
}

/// The scenario directory without mutating the world.
fn scenario_dir_for(w: &DutyWorld) -> PathBuf {
    w.dir
        .clone()
        .expect("scenario prepared no working directory")
}

/// The profile source tree the scenario reads.
pub(crate) fn source_tree(w: &DutyWorld) -> PathBuf {
    w.source_tree
        .clone()
        .expect("the scenario prepared no org/ source tree")
}

/// The three pre-write descriptors the profile scenarios need plus the absent
/// profile repository itself, so the preview guard's live read is a clean 404.
pub(crate) fn write_profile_descriptors(fixtures: &Path) {
    for relative in [
        "repos/duty2cal/.github.json",
        "repos/duty2cal/.github/contents/profile/README.md.json",
        "repos/duty2cal/.github/contents/SECURITY.md.json",
        "repos/duty2cal/duty2cal.github.io/contents/SECURITY.md.json",
    ] {
        let path = fixtures.join(relative);
        fs::create_dir_all(path.parent().expect("a descriptor parent"))
            .expect("create the descriptor directory");
        fs::write(&path, "{\"__fake_response__\": {\"status\": 404}}\n")
            .expect("write the 404 descriptor");
    }
}

/// Prime the inherited applied state: copy the declaration and the reviewed
/// `org/` copy, start the write sandbox, adopt both existing batches, then run
/// the base declaration's real apply with `org_profile.tf` removed (that apply
/// performs the security change's scanning write). The profile declaration is
/// restored afterwards and the journal offset is taken.
pub(crate) fn prime_base_state(w: &mut DutyWorld) {
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

    for batch in ["adoption-batch.txt", "adoption-batch-security.txt"] {
        w.batch = Some(declaration.join(batch));
        crate::steps::adopt_existing_configuration::adopt(w);
        assert_eq!(
            w.exit_status,
            Some(0),
            "the {batch} is adopted; output was:\n{}",
            w.adoption_transcript
        );
    }

    let profile_declaration = declaration.join("org_profile.tf");
    fs::remove_file(&profile_declaration)
        .expect("remove the profile declaration for the base apply");
    // The pages publishing declaration arrived after this change; the base
    // apply stays the declaration this change was written against.
    let pages_declaration = declaration.join("pages_publish.tf");
    if pages_declaration.is_file() {
        fs::remove_file(&pages_declaration)
            .expect("remove the pages declaration for the base apply");
    }

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
    let apply = tofu(w, &["apply", "-input=false", "-no-color", "base-plan.bin"]);
    assert!(
        apply.status.success(),
        "the base declaration's apply failed:\n{}",
        combined(&apply)
    );
    let _ = fs::remove_file(declaration.join("base-plan.bin"));

    fs::copy(
        repository_root().join("infra/org_profile.tf"),
        &profile_declaration,
    )
    .expect("restore the profile declaration after the base apply");
    fs::copy(
        repository_root().join("infra/pages_publish.tf"),
        &pages_declaration,
    )
    .expect("restore the pages declaration after the base apply");

    w.journal_offset = journal_lines(w).len();
    w.exit_status = None;
    w.stdout.clear();
    w.stderr.clear();
}

/// The preview phase: `tofu init`, the targeted `plan -out`, `tofu show -json`
/// and the unbound-live-object guard. The saved plan is applied by the apply
/// phase.
pub(crate) fn preview(w: &mut DutyWorld) {
    let declaration = declaration_dir(w);
    let dir = scenario_dir_for(w);
    let report = dir.join("preview.json");
    let mut transcript =
        String::from("$ preview (tofu init; targeted plan -out; tofu show -json)\n");

    w.preview_transcript.clear();
    w.plan_path = None;

    let init = tofu(w, &["init", "-input=false"]);
    transcript.push_str("$ tofu init -input=false\n");
    transcript.push_str(&combined(&init));
    if !init.status.success() {
        w.exit_status = init.status.code();
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

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
    fs::write(&report, &show.stdout).expect("write the saved plan's JSON report");
    transcript.push_str("$ tofu show -json plan.bin\n");
    transcript.push_str(&combined(&show));
    if !show.status.success() {
        w.exit_status = show.status.code();
        w.preview_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    // The apply guard: a create for an object that already exists live and is
    // unbound is refused before the plan is saved for application
    // (design.md "The profile repository is created, not adopted").
    let creates = jq(
        w,
        "[.resource_changes[]? | select((.change.actions // []) == [\"create\"]) \
         | .address] | join(\" \")",
        &report,
        false,
    );
    if creates
        .split_whitespace()
        .any(|address| address == "github_repository.dot_github")
    {
        let (status, _) = fake_request(w, "GET", "/repos/duty2cal/.github", &[], None);
        match status {
            200 => {
                transcript.push_str(
                    "refused: the live organization profile repository already exists and is \
                     not bound (github_repository.dot_github)\n",
                );
                let _ = fs::remove_file(declaration.join("plan.bin"));
                w.exit_status = Some(2);
                w.preview_transcript = transcript.clone();
                w.scenario_transcript.push_str(&transcript);
                return;
            }
            404 => {}
            other => panic!(
                "the preview guard's live read answered {other}, not 200 or 404; transcript:\n{transcript}"
            ),
        }
    }

    w.plan_path = Some(declaration.join("plan.bin"));
    w.exit_status = Some(0);
    w.preview_transcript = transcript.clone();
    w.scenario_transcript.push_str(&transcript);
}

/// The apply phase: the read-only refresh-only preflight over the same targets,
/// then the saved plan's application.
pub(crate) fn apply_previewed(w: &mut DutyWorld) {
    let declaration = declaration_dir(w);
    let dir = scenario_dir_for(w);
    let report = dir.join("preflight.json");
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

    let mut preflight_args = vec![
        "plan",
        "-refresh-only",
        "-detailed-exitcode",
        "-json",
        "-input=false",
    ];
    preflight_args.extend(TARGETS.iter().copied());
    let preflight = tofu(w, &preflight_args);
    fs::write(&report, &preflight.stdout).expect("write the preflight's JSON report");
    transcript.push_str(&format!(
        "$ tofu plan -refresh-only -detailed-exitcode -json -input=false {}\n",
        TARGETS.join(" ")
    ));
    if !preflight.stderr.is_empty() {
        transcript.push_str(&String::from_utf8_lossy(&preflight.stderr));
    }
    let verdict = jq(w, PREFLIGHT_FILTER, &report, true);
    if verdict != "ok" {
        transcript.push_str(&format!(
            "refused: live changed after the preview: {verdict}\n"
        ));
        w.exit_status = Some(2);
        w.apply_transcript = transcript.clone();
        w.scenario_transcript.push_str(&transcript);
        return;
    }

    let apply = tofu(w, &["apply", "-input=false", "plan.bin"]);
    transcript.push_str("$ tofu apply -input=false plan.bin\n");
    transcript.push_str(&combined(&apply));
    w.stdout = String::from_utf8_lossy(&apply.stdout).into_owned();
    w.stderr = String::from_utf8_lossy(&apply.stderr).into_owned();
    w.exit_status = apply.status.code();
    if apply.status.success() {
        let _ = fs::remove_file(&plan);
        let _ = fs::remove_file(&report);
    }
    w.apply_transcript = transcript.clone();
    w.scenario_transcript.push_str(&transcript);
}

/// Publish the profile declaration once (the scenarios' Given state) and take
/// the journal offset after the priming writes.
fn publish_profile(w: &mut DutyWorld) {
    let addresses = state_addresses(w);
    if PROFILE_ADDRESSES
        .iter()
        .all(|address| addresses.iter().any(|recorded| recorded == address))
    {
        return;
    }
    preview(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the profile publication preview succeeds; output was:\n{}",
        w.preview_transcript
    );
    apply_previewed(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the profile publication apply succeeds; output was:\n{}",
        w.apply_transcript
    );
    w.journal_offset = journal_lines(w).len();
}

/// Fetch one published file's bytes through the fake's contents API.
pub(crate) fn published_file(w: &DutyWorld, repository: &str, path: &str, label: &str) -> Vec<u8> {
    let request = format!("/repos/duty2cal/{repository}/contents/{path}?ref=main");
    let (status, body) = fake_request(w, "GET", &request, &[], None);
    assert_eq!(
        status, 200,
        "the fake serves {repository}/{path}; response was: {body}"
    );
    let report = scenario_dir_for(w).join(format!("contents-{label}.json"));
    fs::write(&report, &body).expect("write the contents response");
    assert_eq!(
        jq(w, ".encoding", &report, false),
        "base64",
        "the contents response is base64"
    );
    assert_eq!(
        jq(w, ".path", &report, false),
        path,
        "the contents response names the requested path"
    );
    base64_decode(&jq(w, ".content", &report, false))
}

/// Fetch the profile repository's live body.
fn published_repository(w: &DutyWorld) -> String {
    let (status, body) = fake_request(w, "GET", "/repos/duty2cal/.github", &[], None);
    assert_eq!(
        status, 200,
        "the fake serves the profile repository: {body}"
    );
    body
}

/// The published profile content the reader scenario reads.
fn published_profile(w: &DutyWorld) -> String {
    let path = scenario_dir_for(w).join("published-profile.md");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read the published profile {}: {error}", path.display()))
}

/// The published default security policy.
fn published_security(w: &DutyWorld) -> String {
    let path = scenario_dir_for(w).join("published-security.md");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read the published policy {}: {error}", path.display()))
}

/// Every Markdown link target in the reviewed copy, in order.
fn markdown_links(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("](") {
        let after = &rest[start + 2..];
        let Some(end) = after.find(')') else {
            break;
        };
        links.push(after[..end].trim().to_string());
        rest = &after[end + 1..];
    }
    let mut rest = text;
    while let Some(start) = rest.find("<https://") {
        let after = &rest[start + 1..];
        let Some(end) = after.find('>') else {
            break;
        };
        links.push(after[..end].to_string());
        rest = &after[end + 1..];
    }
    links
}

fn assert_contains(text: &str, needle: &str) {
    assert!(
        text.contains(needle),
        "the published content contains {needle:?}; content was:\n{text}"
    );
}

/// A `jq` read that preserves the string's own bytes (no trailing-newline
/// trimming), for content comparisons.
fn jq_raw(w: &DutyWorld, filter: &str, report: &Path) -> String {
    let output = Command::new("jq")
        .args(["-j", "-r", filter])
        .arg(report)
        .current_dir(scenario_dir_for(w))
        .output()
        .expect("run jq");
    assert!(
        output.status.success(),
        "jq failed on {}: {}",
        report.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("the state content is UTF-8")
}

/// A small standard-alphabet base64 decoder; whitespace is ignored.
fn base64_decode(input: &str) -> Vec<u8> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut reverse = [255u8; 128];
    for (index, byte) in TABLE.iter().enumerate() {
        reverse[*byte as usize] = index as u8;
    }
    let mut output = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0u32;
    for byte in input.bytes() {
        if byte == b'=' {
            break;
        }
        if byte.is_ascii_whitespace() {
            continue;
        }
        assert!(byte < 128, "base64 content is ASCII");
        let value = reverse[byte as usize];
        assert_ne!(value, 255, "invalid base64 byte {}", byte as char);
        buffer = (buffer << 6) | u32::from(value);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            output.push((buffer >> bits) as u8);
        }
    }
    output
}

#[given("the applied state binds the organization profile repository and both files")]
async fn the_applied_state_binds_the_profile_repository_and_files(w: &mut DutyWorld) {
    prime_base_state(w);
    publish_profile(w);
    let addresses = state_addresses(w);
    for address in PROFILE_ADDRESSES {
        assert!(
            addresses.iter().any(|recorded| recorded == address),
            "the applied state binds {address}; it was: {addresses:?}"
        );
    }
}

#[given("a previewed revision whose profile content differs from the published profile content")]
async fn a_previewed_revision_whose_profile_content_differs(w: &mut DutyWorld) {
    let source = source_tree(w);
    let readme = source.join("profile/README.md");
    let mut revised = fs::read_to_string(&readme).expect("read the reviewed profile content");
    revised.push_str("\nThis revision changes only the organization profile content.\n");
    fs::write(&readme, &revised).expect("write the revised profile content");

    let dir = scenario_dir_for(w);
    fs::write(dir.join("repository-before.json"), published_repository(w))
        .expect("record the repository before the revision");
    fs::write(
        dir.join("security-before.md"),
        published_file(w, ".github", "SECURITY.md", "security-before"),
    )
    .expect("record the default security policy before the revision");

    preview(w);
    assert_eq!(
        w.exit_status,
        Some(0),
        "the revised revision previews; output was:\n{}",
        w.preview_transcript
    );
    let report = scenario_dir_for(w).join("preview.json");
    let changed = jq(
        w,
        "[.resource_changes[]? | select((.change.actions // []) != [\"no-op\"]) \
         | .address] | join(\" \")",
        &report,
        false,
    );
    assert_eq!(
        changed, "github_repository_file.org_profile_readme",
        "the revised preview names only the profile content"
    );
    let actions = jq(
        w,
        "[.resource_changes[]? | select(.address == \
         \"github_repository_file.org_profile_readme\") | .change.actions] \
         | .[0] | join(\" \")",
        &report,
        false,
    );
    assert_eq!(actions, "update", "the profile content is updated in place");
}

#[then("the apply reports the profile content as the only changed object")]
async fn the_apply_reports_the_profile_content_as_the_only_changed_object(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the revised revision applies; output was:\n{}",
        w.apply_transcript
    );
    assert!(
        w.stdout
            .contains("github_repository_file.org_profile_readme"),
        "the apply reports the profile content; output was:\n{}",
        w.stdout
    );
    for unchanged in [
        "github_repository.dot_github",
        "github_repository_file.org_security",
    ] {
        assert!(
            !w.stdout.contains(unchanged),
            "the apply changes nothing beside the profile content, but names {unchanged}; \
             output was:\n{}",
            w.stdout
        );
    }
}

#[then("the organization profile repository serves the revised profile content")]
async fn the_profile_repository_serves_the_revised_profile_content(w: &mut DutyWorld) {
    let published = published_file(w, ".github", "profile/README.md", "revised-profile");
    let source = fs::read(source_tree(w).join("profile/README.md"))
        .expect("read the revised profile content");
    assert_eq!(
        published, source,
        "the fake serves the revised profile content byte for byte"
    );

    let report = scenario_dir_for(w).join("state-revised.json");
    let show = tofu(w, &["show", "-json"]);
    assert!(
        show.status.success(),
        "tofu show -json failed:\n{}",
        combined(&show)
    );
    fs::write(&report, &show.stdout).expect("write the applied state's JSON report");
    let recorded = jq_raw(
        w,
        "([.values.root_module.resources[]? \
          | select(.address == \"github_repository_file.org_profile_readme\") \
          | .values.content] | .[0] // \"\")",
        &report,
    );
    assert_eq!(
        recorded,
        String::from_utf8(source).expect("the reviewed profile content is UTF-8"),
        "the applied state records the revised content"
    );
}

#[then("the default security policy served by the organization profile repository is unchanged")]
async fn the_default_security_policy_served_is_unchanged(w: &mut DutyWorld) {
    let published = published_file(w, ".github", "SECURITY.md", "security-after");
    let before = fs::read(scenario_dir_for(w).join("security-before.md"))
        .expect("read the recorded security policy");
    assert_eq!(
        published, before,
        "the default security policy is unchanged by the profile revision"
    );
}

#[then("the organization profile repository itself is unchanged")]
async fn the_organization_profile_repository_itself_is_unchanged(w: &mut DutyWorld) {
    let before = fs::read_to_string(scenario_dir_for(w).join("repository-before.json"))
        .expect("read the recorded repository body");
    assert_eq!(
        published_repository(w),
        before,
        "the profile repository's live body is unchanged by the profile revision"
    );
}

#[given("the organization profile repository is published")]
async fn the_organization_profile_repository_is_published(w: &mut DutyWorld) {
    prime_base_state(w);
    publish_profile(w);
    let addresses = state_addresses(w);
    for address in PROFILE_ADDRESSES {
        assert!(
            addresses.iter().any(|recorded| recorded == address),
            "the applied state binds {address}; it was: {addresses:?}"
        );
    }
}

#[when("a reader reads the published profile content")]
async fn a_reader_reads_the_published_profile_content(w: &mut DutyWorld) {
    let published = published_file(w, ".github", "profile/README.md", "profile");
    let source = fs::read(source_tree(w).join("profile/README.md"))
        .expect("read the reviewed profile content");
    assert_eq!(
        published, source,
        "the published profile content is the reviewed copy byte for byte"
    );
    fs::write(scenario_dir_for(w).join("published-profile.md"), &published)
        .expect("record the published profile content");
}

#[then(
    "the profile content describes the converter as a local tool that reads a crew duty plan \
     and writes a calendar"
)]
async fn the_profile_content_describes_the_local_converter(w: &mut DutyWorld) {
    let text = published_profile(w);
    assert_contains(&text, "crew duty plan (PDF)");
    assert_contains(&text, "iCalendar");
    assert_contains(&text, "The conversion runs locally");
    assert_contains(&text, "no file leaves your device");
}

#[then("the profile content names Windows x86_64 as the only distributed platform")]
async fn the_profile_content_names_windows_only(w: &mut DutyWorld) {
    let text = published_profile(w);
    assert_contains(&text, "Windows x86_64");
    assert_contains(&text, "Other platforms are not");
    assert_contains(&text, "supported or distributed");
    for other in ["macOS", "MacOS", "Linux", "linux", "three platforms"] {
        assert!(
            !text.contains(other),
            "the profile content names no {other}; content was:\n{text}"
        );
    }
}

#[then("the profile content makes no claim that the converter runs in a browser")]
async fn the_profile_content_makes_no_browser_claim(w: &mut DutyWorld) {
    let text = published_profile(w).to_lowercase();
    for claim in ["browser", "wasm", "web app", "web ui", "web application"] {
        assert!(
            !text.contains(claim),
            "the profile content makes no {claim} claim"
        );
    }
}

#[then(regex = r#"^the profile content links the stable latest-release address "([^"]+)"$"#)]
async fn the_profile_content_links_the_stable_release(w: &mut DutyWorld, address: String) {
    assert_eq!(
        address, "https://github.com/duty2cal/duty2cal/releases/latest",
        "the step names the stable latest-release address"
    );
    let text = published_profile(w);
    assert_contains(&text, &address);
}

#[then(
    regex = r#"^the profile content links the source repository "([^"]+)" and the site repository "([^"]+)"$"#
)]
async fn the_profile_content_links_the_repositories(
    w: &mut DutyWorld,
    source: String,
    site: String,
) {
    let text = published_profile(w);
    assert_contains(&text, &source);
    assert_contains(&text, &site);
}

#[then(
    "the profile content states that the project is not affiliated with any duty-plan provider \
     or airline"
)]
async fn the_profile_content_states_non_affiliation(w: &mut DutyWorld) {
    let text = published_profile(w);
    assert_contains(
        &text,
        "Not affiliated with any duty-plan provider or airline.",
    );
}

#[then("the profile content attributes the third-party Unicode data to Unicode-3.0")]
async fn the_profile_content_attributes_unicode(w: &mut DutyWorld) {
    let text = published_profile(w);
    assert_contains(&text, "Unicode-3.0");
    assert_contains(
        &text,
        "https://github.com/duty2cal/duty2cal/blob/main/LICENSES/Unicode-3.0.txt",
    );
}

#[then("every link in the profile content is absolute")]
async fn every_link_in_the_profile_content_is_absolute(w: &mut DutyWorld) {
    let text = published_profile(w);
    let links = markdown_links(&text);
    assert!(!links.is_empty(), "the profile content carries links");
    for link in links {
        assert!(
            link.starts_with("https://"),
            "every profile link is absolute; found {link:?}"
        );
    }
}

#[then("the profile content embeds no image, badge or other third-party resource")]
async fn the_profile_content_embeds_no_third_party_resource(w: &mut DutyWorld) {
    let text = published_profile(w);
    for forbidden in ["![", "<img", "src=", "shields.io", "http://", "badge"] {
        assert!(
            !text.contains(forbidden),
            "the profile content embeds no {forbidden}; content was:\n{text}"
        );
    }
    for link in markdown_links(&text) {
        assert!(
            link.starts_with("https://github.com/"),
            "the profile content links only github.com; found {link:?}"
        );
    }
}

#[given("the pages repository has no security policy of its own")]
async fn the_pages_repository_has_no_security_policy(w: &mut DutyWorld) {
    let (status, body) = fake_request(
        w,
        "GET",
        "/repos/duty2cal/duty2cal.github.io/contents/SECURITY.md?ref=main",
        &[],
        None,
    );
    assert_eq!(
        status, 404,
        "the pages repository has no policy of its own; response was: {body}"
    );
}

#[when("a contributor reads the organization's default security policy")]
async fn a_contributor_reads_the_default_security_policy(w: &mut DutyWorld) {
    let published = published_file(w, ".github", "SECURITY.md", "security");
    let source = fs::read(source_tree(w).join("SECURITY.md"))
        .expect("read the reviewed default security policy");
    assert_eq!(
        published, source,
        "the published default security policy is the reviewed copy byte for byte"
    );
    fs::write(
        scenario_dir_for(w).join("published-security.md"),
        &published,
    )
    .expect("record the published default security policy");
    fs::write(
        scenario_dir_for(w).join("published-profile-repository.json"),
        published_repository(w),
    )
    .expect("record the profile repository body");
}

#[then(
    "the organization profile repository carries the default security policy as \"SECURITY.md\""
)]
async fn the_profile_repository_carries_the_security_policy(w: &mut DutyWorld) {
    let published = published_security(w);
    let source = fs::read_to_string(source_tree(w).join("SECURITY.md"))
        .expect("read the reviewed default security policy");
    assert_eq!(published, source);
    assert!(
        published.contains("# Security policy"),
        "the SECURITY.md file is the default security policy"
    );
}

#[then("the default security policy names the private vulnerability reporting route")]
async fn the_security_policy_names_the_private_route(w: &mut DutyWorld) {
    assert_contains(&published_security(w), PRIVATE_REPORTING_ROUTE);
}

#[then("the default security policy forbids attaching a real duty plan or a converted calendar")]
async fn the_security_policy_forbids_attachments(w: &mut DutyWorld) {
    assert_contains(
        &published_security(w),
        "Never attach a real duty plan or a converted calendar",
    );
}

#[then("the default security policy states that only the latest published release is supported")]
async fn the_security_policy_states_the_supported_release(w: &mut DutyWorld) {
    assert_contains(
        &published_security(w),
        "Only the latest published release is supported",
    );
}

#[then("the organization profile repository offers no public issue tracker for security reports")]
async fn the_profile_repository_offers_no_public_issue_tracker(w: &mut DutyWorld) {
    let report = scenario_dir_for(w).join("published-profile-repository.json");
    assert_eq!(
        jq(w, ".has_issues", &report, false),
        "false",
        "the profile repository offers no public issue tracker"
    );
    assert_contains(&published_security(w), "Do not open a public issue");
}

#[then("the pages repository serves no security policy of its own")]
async fn the_pages_repository_serves_no_security_policy(w: &mut DutyWorld) {
    let (status, body) = fake_request(
        w,
        "GET",
        "/repos/duty2cal/duty2cal.github.io/contents/SECURITY.md?ref=main",
        &[],
        None,
    );
    assert_eq!(
        status, 404,
        "the pages repository serves no policy of its own; response was: {body}"
    );
}

#[given(
    "the applied state binds every declared object except the organization profile repository \
     and its two files"
)]
async fn the_applied_state_binds_everything_except_the_profile(w: &mut DutyWorld) {
    prime_base_state(w);
    let addresses = state_addresses(w);
    for address in PROFILE_ADDRESSES {
        assert!(
            !addresses.iter().any(|recorded| recorded == address),
            "the applied state binds no {address} before the profile apply; it was: {addresses:?}"
        );
    }
    w.applied_state_snapshot = Some(state_values(w));
}

#[given("the live organization already has the organization profile repository")]
async fn the_live_organization_already_has_the_profile_repository(w: &mut DutyWorld) {
    let fixtures = fixtures_dir(w);
    fs::create_dir_all(fixtures.join("repos/duty2cal"))
        .expect("create the repository fixture directory");
    fs::write(
        fixtures.join("repos/duty2cal/.github.json"),
        "{\"id\": 900001, \"node_id\": \"R_fake_dot_github\", \
         \"name\": \".github\", \"full_name\": \"duty2cal/.github\", \
         \"private\": false, \"visibility\": \"public\", \"default_branch\": \"main\", \
         \"has_issues\": true, \"has_projects\": true, \"has_wiki\": true, \
         \"has_discussions\": false, \"has_pages\": false}\n",
    )
    .expect("write the live profile repository body");
}

#[given(
    "the declaration adds the organization profile repository, its profile content and its \
     default security policy"
)]
async fn the_declaration_adds_the_profile_objects(w: &mut DutyWorld) {
    ensure_source_tree(w);
    let declaration = declaration_dir(w).join("org_profile.tf");
    let text = fs::read_to_string(&declaration)
        .unwrap_or_else(|error| panic!("read {}: {error}", declaration.display()));
    for resource in [
        "resource \"github_repository\" \"dot_github\"",
        "resource \"github_repository_file\" \"org_profile_readme\"",
        "resource \"github_repository_file\" \"org_security\"",
    ] {
        assert!(
            text.contains(resource),
            "the declaration carries {resource}; it was:\n{text}"
        );
    }
    assert!(
        text.contains("file(\"${local.org_source_dir}/profile/README.md\")")
            && text.contains("file(\"${local.org_source_dir}/SECURITY.md\")"),
        "both files read the reviewed org/ copy; declaration was:\n{text}"
    );
    assert!(
        source_tree(w).join("profile/README.md").is_file()
            && source_tree(w).join("SECURITY.md").is_file(),
        "the reviewed org/ copy sits beside the declaration"
    );
}

#[then("the preview is refused and names the organization profile repository")]
async fn the_preview_is_refused_and_names_the_profile_repository(w: &mut DutyWorld) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the preview is refused; output was:\n{}",
        w.preview_transcript
    );
    for expected in [
        "refused",
        "organization profile repository",
        "github_repository.dot_github",
    ] {
        assert!(
            w.preview_transcript.contains(expected),
            "the refusal names {expected}; output was:\n{}",
            w.preview_transcript
        );
    }
    let journal = journal_lines(w);
    let writes = journal
        .iter()
        .skip(w.journal_offset)
        .filter(|line| {
            line.contains("\"applied_write\": true") || line.contains("\"refused_write\": true")
        })
        .count();
    assert_eq!(
        writes, 0,
        "the refused preview issued no write; journal was:\n{journal:#?}"
    );
}

#[then("no object in the preview is bound")]
async fn no_object_in_the_preview_is_bound(w: &mut DutyWorld) {
    let addresses = state_addresses(w);
    for address in PROFILE_ADDRESSES {
        assert!(
            !addresses.iter().any(|recorded| recorded == address),
            "the refusal binds no {address}; it was: {addresses:?}"
        );
    }
    let values = state_values(w);
    let before = w
        .applied_state_snapshot
        .as_ref()
        .expect("the scenario snapshotted no applied state");
    assert_eq!(
        &values,
        before,
        "the refused preview changed the applied state at {}",
        state_path(w).display()
    );
}
