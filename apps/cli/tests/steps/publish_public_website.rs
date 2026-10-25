// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `publish-public-website`.
//!
//! One file per capability. The steps drive `scripts/publish-site.sh` as a real
//! subprocess against a local git target, as design.md requires. A step body is
//! never empty: the RED phase writes a new step as a stub that panics with its
//! own name.
#![allow(clippy::too_many_arguments)] // step signatures mirror the captured step text

use cucumber::{given, then, when};

use crate::DutyWorld;

/// The fixture's default-branch revision label the scenarios name.
pub(crate) const BASE_REVISION: &str = "abc1234";

/// The fixture's second revision label the scenarios name.
pub(crate) const NEXT_REVISION: &str = "def5678";

#[given(regex = r#"^a hosting target serving the Site output built from "([^"]+)"$"#)]
async fn a_hosting_target_serving(w: &mut DutyWorld, revision: String) {
    let _ = (w, revision);
    unimplemented!("a hosting target serving the Site output built from a revision");
}

#[given(regex = r#"^the default branch's previous revision "([^"]+)"$"#)]
async fn the_previous_revision(w: &mut DutyWorld, revision: String) {
    let _ = (w, revision);
    unimplemented!("the default branch's previous revision");
}

#[given(regex = r#"^a default-branch revision "([^"]+)" that changes only the project's documents$"#)]
async fn a_documents_only_revision(w: &mut DutyWorld, revision: String) {
    let _ = (w, revision);
    unimplemented!("a default-branch revision that changes only the project's documents");
}

#[given(regex = r#"^the release tag "([^"]+)" pushed on a revision "([^"]+)" that changes none of the site's inputs$"#)]
async fn a_release_tag_on_a_revision(w: &mut DutyWorld, tag: String, revision: String) {
    let _ = (w, tag, revision);
    unimplemented!("the release tag pushed on a revision that changes none of the site's inputs");
}

#[given("the publication names no earlier revision to compare against")]
async fn no_comparison_base(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("the publication names no earlier revision to compare against");
}

#[given("a source revision whose history the repository does not hold")]
async fn a_revision_the_repository_lacks(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("a source revision whose history the repository does not hold");
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
    let _ = w;
    unimplemented!("the release maintainer publishes the site from that revision, compared with the previous revision");
}

#[when("the release maintainer publishes the site from the tagged revision")]
async fn publish_from_the_tagged_revision(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("the release maintainer publishes the site from the tagged revision");
}

#[when("the release maintainer publishes the site from that revision")]
async fn publish_from_that_revision(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("the release maintainer publishes the site from that revision");
}

#[then("the Site publication reports that the revision changes none of the site's inputs")]
async fn reports_no_site_input(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("the Site publication reports that the revision changes none of the site's inputs");
}

#[then(regex = r#"^the hosting target still serves the Site output built from "([^"]+)"$"#)]
async fn the_target_still_serves(w: &mut DutyWorld, revision: String) {
    let _ = (w, revision);
    unimplemented!("the hosting target still serves the Site output built from a revision");
}

#[then("the Site publication starts from the tagged revision even though it changes none of the site's inputs")]
async fn starts_from_the_tagged_revision(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("the Site publication starts from the tagged revision");
}

#[then(regex = r#"^the hosting target holds the Site output built from "([^"]+)"$"#)]
async fn the_target_holds_the_output(w: &mut DutyWorld, revision: String) {
    let _ = (w, revision);
    unimplemented!("the hosting target holds the Site output built from a revision");
}

#[then(regex = r#"^the footer names the Built revision "([^"]+)" and links the tagged commit$"#)]
async fn the_footer_names_the_built_revision(w: &mut DutyWorld, revision: String) {
    let _ = (w, revision);
    unimplemented!("the footer names the Built revision and links the tagged commit");
}

#[then("the footer names no tag-distance form")]
async fn the_footer_names_no_tag_distance(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("the footer names no tag-distance form");
}

#[then("the publication is refused and names the Built revision")]
async fn refused_naming_the_built_revision(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("the publication is refused and names the Built revision");
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
