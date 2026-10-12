//! Step definitions for the capability `export-duty-plan-calendar`.
//!
//! One file per capability (see `design.md` "Step definition locations").
//! RED-phase steps use the `unimplemented!("<step>")` stub; a step body is
//! never empty.

use crate::DutyWorld;

// Steps are registered by the `#[given]`/`#[when]`/`#[then]` macros as the
// scenario tasks implement them. The world carries the last child invocation's
// transcript and exit status.
#[allow(dead_code)]
fn _use_world(_w: &mut DutyWorld) {}
