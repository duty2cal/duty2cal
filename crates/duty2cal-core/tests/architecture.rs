// SPDX-License-Identifier: MPL-2.0
//! Architecture tests for `duty2cal-core` (design.md "Layer rules", level 3).
//!
//! Enforces the intra-crate module direction: `domain` depends on nothing,
//! `application` depends only on `domain`, `adapters` may depend on
//! `application` and `domain`, and the graph is cycle-free.
//!
//! Paths are workspace-relative, because `project_files()` discovers the
//! workspace. Limits (see ADR-0002): syntax-based, file granularity, no macro
//! expansion.

use archunit::{assert_passes, project_files};

#[test]
fn domain_does_not_depend_on_other_layers() {
    let rule = project_files()
        .in_path("crates/duty2cal-core/src/domain/**/*.rs")
        .should_not()
        .depend_on_files()
        .in_path("crates/duty2cal-core/src/application/**/*.rs")
        .in_path("crates/duty2cal-core/src/adapters/**/*.rs");
    assert_passes!(rule);
}

#[test]
fn application_does_not_depend_on_adapters() {
    let rule = project_files()
        .in_path("crates/duty2cal-core/src/application/**/*.rs")
        .should_not()
        .depend_on_files()
        .in_path("crates/duty2cal-core/src/adapters/**/*.rs");
    assert_passes!(rule);
}

#[test]
fn core_sources_are_cycle_free() {
    let rule = project_files()
        .in_path("crates/duty2cal-core/src/**/*.rs")
        .should()
        .have_no_cycles();
    assert_passes!(rule);
}
