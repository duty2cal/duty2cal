// SPDX-License-Identifier: MPL-2.0
//! End-to-end step definitions for the capability `publish-public-website`.
//!
//! The step bodies drive the real publication command and then the rendered
//! page through the shared Playwright driver; a step body is never empty.

use cucumber::given;

use crate::DutyWorld;

#[given("a hosting target whose default branch serves an earlier page")]
async fn a_hosting_target_with_an_earlier_page(w: &mut DutyWorld) {
    let _ = w;
    unimplemented!("a hosting target whose default branch serves an earlier page");
}
