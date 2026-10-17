// SPDX-License-Identifier: MPL-2.0
//! End-to-end steps for the capability `adopt-existing-configuration`.
//!
//! One file per capability. The `@e2e` scenario drives the same real adoption
//! command as the regular steps, adds the committed transcript and asserts the
//! journal's no-write condition (design.md "Step definitions"). The scenario's
//! steps are written in the bundle that takes it out of `@wip`; until then
//! this module is declared so the runner compiles it.
