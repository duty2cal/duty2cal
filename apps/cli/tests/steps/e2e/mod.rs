// SPDX-License-Identifier: MPL-2.0
//! End-to-end step definitions: these drive the built binary as a subprocess,
//! or the rendered page through the browser driver, so they live apart from
//! the in-process steps (design.md "Step definition locations").

pub mod export_duty_plan_calendar;
pub mod own_the_duty_plan_extraction;
pub mod visual_baseline;
