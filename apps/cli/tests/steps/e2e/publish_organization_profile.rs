// SPDX-License-Identifier: MPL-2.0
//! End-to-end step definitions for the capability
//! `publish-organization-profile` (design.md "E2E smoke test infrastructure").
//!
//! Every step starts as a not-implemented stub: the `@e2e` scenarios remain
//! `@wip` until their definitions and the scrubbed committed transcript exist.

use cucumber::{given, then};

use crate::DutyWorld;

#[given(
    "a previewed revision that declares the organization profile repository, its profile \
     content and its default security policy"
)]
async fn a_previewed_revision_declares_the_profile_objects(_w: &mut DutyWorld) {
    unimplemented!(
        "Organization owner applies the previewed revision: \
         a previewed revision that declares the profile objects"
    );
}

#[then("the apply reports the organization profile repository and both files as created")]
async fn the_apply_reports_the_profile_objects_as_created(_w: &mut DutyWorld) {
    unimplemented!(
        "Organization owner applies the previewed revision: the apply reports the creations"
    );
}

#[then(
    "the organization profile repository is public and offers no issue tracker, projects or wiki"
)]
async fn the_profile_repository_is_public_and_offers_no_tracker(_w: &mut DutyWorld) {
    unimplemented!(
        "Organization owner applies the previewed revision: the repository's public shape"
    );
}

#[then(
    regex = r#"^the organization profile repository serves the reviewed profile content as "([^"]+)"$"#
)]
async fn the_profile_repository_serves_the_reviewed_profile_content(
    _w: &mut DutyWorld,
    _path: String,
) {
    unimplemented!(
        "Organization owner applies the previewed revision: the reviewed profile content"
    );
}

#[then(
    regex = r#"^the organization profile repository serves the default security policy as "([^"]+)"$"#
)]
async fn the_profile_repository_serves_the_default_security_policy(
    _w: &mut DutyWorld,
    _path: String,
) {
    unimplemented!(
        "Organization owner applies the previewed revision: the default security policy"
    );
}

#[then("the applied state records the organization profile repository and both files")]
async fn the_applied_state_records_the_profile_objects(_w: &mut DutyWorld) {
    unimplemented!(
        "Organization owner applies the previewed revision: the applied state records the objects"
    );
}

#[then("a later targeted preview of the organization profile reports no differences")]
async fn a_later_targeted_preview_reports_no_differences(_w: &mut DutyWorld) {
    unimplemented!("Organization owner applies the previewed revision: the follow-up preview");
}

#[given("the live organization has no organization profile repository")]
async fn the_live_organization_has_no_profile_repository(_w: &mut DutyWorld) {
    unimplemented!(
        "Organization owner previews the profile declaration: \
         the live organization has no profile repository"
    );
}

#[then("the preview names the organization profile repository as the only object to create")]
async fn the_preview_names_the_profile_repository_as_the_only_create(_w: &mut DutyWorld) {
    unimplemented!(
        "Organization owner previews the profile declaration: \
         the profile repository is the only object to create"
    );
}

#[then("the preview names the profile content and the default security policy as the only files to create")]
async fn the_preview_names_the_two_files_as_the_only_creates(_w: &mut DutyWorld) {
    unimplemented!(
        "Organization owner previews the profile declaration: \
         the two files are the only files to create"
    );
}

#[then("the preview names no change other than the three creations")]
async fn the_preview_names_no_change_other_than_the_three_creations(_w: &mut DutyWorld) {
    unimplemented!("Organization owner previews the profile declaration: no other change");
}
