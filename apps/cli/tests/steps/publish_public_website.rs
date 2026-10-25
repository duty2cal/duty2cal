// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `publish-public-website`.
//!
//! One file per capability. The steps drive `scripts/publish-site.sh` as a real
//! subprocess against a local git target, as design.md requires. A step body is
//! never empty: the RED phase writes a new step as a stub that panics with its
//! own name.
#![allow(clippy::too_many_arguments)] // step signatures mirror the captured step text

use cucumber::{given, then, when};

use crate::steps::support;
use crate::DutyWorld;

/// The fixture's default-branch revision label the scenarios name.
pub(crate) const BASE_REVISION: &str = "abc1234";

/// The fixture's second revision label the scenarios name.
pub(crate) const NEXT_REVISION: &str = "def5678";

#[given(regex = r#"^a hosting target serving the Site output built from "([^"]+)"$"#)]
async fn a_hosting_target_serving(w: &mut DutyWorld, revision: String) {
    let url = support::site_set_up_target(w, &revision);
    assert!(
        url.starts_with("http://127.0.0.1:"),
        "the fixture's serving address is not loopback: {url}"
    );
}

#[given(regex = r#"^the default branch's previous revision "([^"]+)"$"#)]
async fn the_previous_revision(w: &mut DutyWorld, revision: String) {
    let resolved = support::site_label(w, &revision);
    assert!(
        resolved.len() >= 40,
        "the fixture's revision label {revision} did not resolve: {resolved:?}"
    );
    support::site_set(w, "since", &resolved);
}

#[given(regex = r#"^a default-branch revision "([^"]+)" that changes only the project's documents$"#)]
async fn a_documents_only_revision(w: &mut DutyWorld, revision: String) {
    let committed = support::site_commit_file(
        w,
        "docs/fixture-documents.md",
        "Fixture documentation revision.\n",
        "docs: fixture documentation",
    );
    let recorded = support::site_record_label(w, &revision);
    assert_eq!(recorded, committed, "the fixture recorded another revision");
    support::site_set(w, "revision", &committed);
}

#[given(regex = r#"^the release tag "([^"]+)" pushed on a revision "([^"]+)" that changes none of the site's inputs$"#)]
async fn a_release_tag_on_a_revision(w: &mut DutyWorld, tag: String, revision: String) {
    let resolved = support::site_label(w, &revision);
    let changed = support::site_changed_paths(w, "v0.5.0", &resolved);
    assert!(
        changed.iter().all(|path| path.starts_with("docs/")),
        "the tagged revision changes a site input: {changed:?}"
    );
    support::site_tag(w, &tag, &resolved);
    assert_eq!(
        support::site_describe(w, &resolved),
        tag,
        "the fixture's own history does not yield the tag"
    );
    support::site_set(w, "tag", &tag);
}

#[given("the publication names no earlier revision to compare against")]
async fn no_comparison_base(w: &mut DutyWorld) {
    assert!(
        support::site_get(w, "since").is_none(),
        "the scenario named an earlier revision to compare against"
    );
}

#[given("a source revision whose history the repository does not hold")]
async fn a_revision_the_repository_lacks(w: &mut DutyWorld) {
    assert!(
        !support::site_resolves(w, "9999999"),
        "the fixture holds the revision the scenario expects it to lack"
    );
    support::site_set(w, "revision", "9999999");
}

#[given(regex = r#"^a working tree at the revision "([^"]+)" carrying edits to files the revision tracks$"#)]
async fn an_unclean_working_tree(w: &mut DutyWorld, revision: String) {
    let _ = (w, revision);
    unimplemented!("a working tree carrying edits to files the revision tracks");
}

#[given(regex = r#"^a source revision "([^"]+)" the repository cannot check out$"#)]
async fn a_revision_that_cannot_be_checked_out(w: &mut DutyWorld, revision: String) {
    let _ = (w, revision);
    unimplemented!("a source revision the repository cannot check out");
}

#[given("a source revision whose browser build fails")]
async fn a_revision_whose_browser_build_fails(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("a source revision whose browser build fails");
}

#[when("the release maintainer publishes the site from that revision, compared with the previous revision")]
async fn publish_compared_with_the_previous_revision(w: &mut DutyWorld) {
    assert!(
        support::site_get(w, "since").is_some(),
        "the scenario named no comparison base"
    );
    support::site_publish(w);
}

#[when("the release maintainer publishes the site from the tagged revision")]
async fn publish_from_the_tagged_revision(w: &mut DutyWorld) {
    let tag = support::site_get(w, "tag").expect("the scenario named no release tag");
    support::site_set(w, "revision", &tag);
    support::site_publish(w);
}

#[when("the release maintainer publishes the site from that revision")]
async fn publish_from_that_revision(w: &mut DutyWorld) {
    assert!(
        support::site_get(w, "revision").is_some(),
        "the scenario named no revision to publish"
    );
    support::site_publish(w);
}

#[then("the Site publication reports that the revision changes none of the site's inputs")]
async fn reports_no_site_input(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the publication failed:\n{}",
        w.command_transcript
    );
    assert!(
        w.stdout
            .lines()
            .any(|line| line.trim() == "no site input changed"),
        "the publication did not report the unchanged site inputs:\n{}",
        w.command_transcript
    );
    let out = support::site_output_directory(w);
    assert!(
        !out.exists(),
        "the skipped publication wrote an output directory: {}",
        out.display()
    );
    assert_eq!(
        support::site_get(w, "target-tip"),
        Some(support::site_target_tip(w)),
        "the skipped publication changed the hosting target"
    );
}

#[then(regex = r#"^the hosting target still serves the Site output built from "([^"]+)"$"#)]
async fn the_target_still_serves(w: &mut DutyWorld, revision: String) {
    let label = support::site_label(w, &revision);
    assert_eq!(
        support::site_get(w, "base-revision"),
        Some(label),
        "the scenario's target was not built from {revision}"
    );
    let page = support::site_served_page(w);
    let served = support::served_revision(&page)
        .unwrap_or_else(|| panic!("the served page carries no Built revision: {page}"));
    let expected = support::site_get(w, "base-describe")
        .expect("the fixture recorded no base revision text");
    assert_eq!(
        served, expected,
        "the address serves another Built revision than {revision}"
    );
    assert_eq!(
        support::site_get(w, "target-tip"),
        Some(support::site_target_tip(w)),
        "the hosting target's history changed"
    );
}

#[then("the Site publication starts from the tagged revision even though it changes none of the site's inputs")]
async fn starts_from_the_tagged_revision(w: &mut DutyWorld) {
    assert_eq!(
        w.exit_status,
        Some(0),
        "the publication failed:\n{}",
        w.command_transcript
    );
    assert!(
        !w.stdout.contains("no site input changed"),
        "the tag publication was skipped as an unchanged revision:\n{}",
        w.command_transcript
    );
    let tag = support::site_get(w, "tag").expect("the scenario named no release tag");
    assert!(
        w.stdout.contains(&tag),
        "the publication did not report the tagged revision {tag}:\n{}",
        w.command_transcript
    );
}

#[then(regex = r#"^the hosting target holds the Site output built from "([^"]+)"$"#)]
async fn the_target_holds_the_output(w: &mut DutyWorld, revision: String) {
    let label = support::site_label(w, &revision);
    assert_eq!(
        support::site_get(w, "base-revision"),
        Some(label.clone()),
        "the scenario's target was not built from {revision}"
    );
    assert_eq!(
        support::site_target_members(w),
        support::SITE_MEMBERS.to_vec(),
        "the hosting target holds another member list"
    );
    let expected = support::site_describe(w, &label);
    let page = support::site_target_file(w, "index.html");
    let built = support::served_revision(&page)
        .unwrap_or_else(|| panic!("the published page carries no Built revision: {page}"));
    assert_eq!(
        built, expected,
        "the published page does not name the revision's own history"
    );
}

#[then(regex = r#"^the footer names the Built revision "([^"]+)" and links the tagged commit$"#)]
async fn the_footer_names_the_built_revision(w: &mut DutyWorld, revision: String) {
    let tagged = support::site_resolve(w, &revision);
    let page = support::site_target_file(w, "index.html");
    assert!(
        page.contains(&format!(
            "data-site-statement=\"built-revision\">{revision}<"
        )),
        "the footer does not name the Built revision {revision}: {page}"
    );
    assert!(
        page.contains(&format!("/commit/{tagged}")),
        "the footer does not link the tagged commit {tagged}: {page}"
    );
}

#[then("the footer names no tag-distance form")]
async fn the_footer_names_no_tag_distance(w: &mut DutyWorld) {
    let page = support::site_target_file(w, "index.html");
    let built = support::served_revision(&page)
        .unwrap_or_else(|| panic!("the published page carries no Built revision: {page}"));
    let parts: Vec<&str> = built.split('-').collect();
    let distance = parts.len() >= 3
        && parts[parts.len() - 2].parse::<u32>().is_ok()
        && parts[parts.len() - 1].starts_with('g');
    assert!(
        !distance,
        "the footer names the tag-distance form {built:?}"
    );
}

#[then("the publication is refused and names the Built revision")]
async fn refused_naming_the_built_revision(w: &mut DutyWorld) {
    assert_ne!(
        w.exit_status,
        Some(0),
        "the publication was not refused:\n{}",
        w.command_transcript
    );
    let revision = support::site_get(w, "revision").expect("the scenario named no revision");
    let text = support::site_publication_text(w);
    assert!(
        text.contains("Built revision"),
        "the refusal does not name the Built revision:\n{text}"
    );
    assert!(
        text.contains(&revision),
        "the refusal does not name the revision {revision}:\n{text}"
    );
}

#[then("the publication is refused and names the unclean revision")]
async fn refused_naming_the_unclean_revision(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("the publication is refused and names the unclean revision");
}

#[then("the publication is refused and names the worktree")]
async fn refused_naming_the_worktree(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("the publication is refused and names the worktree");
}

#[then("the publication is refused and names the revision's browser build step")]
async fn refused_naming_the_browser_build_step(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("the publication is refused and names the revision's browser build step");
}
