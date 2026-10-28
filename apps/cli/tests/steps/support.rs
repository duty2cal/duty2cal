// SPDX-License-Identifier: MPL-2.0
//! Test support: the generated fixture plans and their column positions.
//!
//! The fixture generator is `crates/duty2cal-pdf-fixtures`, a dev-only member.
//! It reproduces the supported family's shape (Type0/Identity-H fonts with
//! `ToUnicode` and `/W`, Flate content) so every conversion scenario exercises
//! the shipped reader, not a standard-font shortcut. Fixtures are generated at
//! test time, so no PDF is committed.

#[allow(unused_imports)] // the golden target includes this module but uses only two builders
pub use duty2cal_pdf_fixtures::{
    at, build_pdf, build_pdf_at_size, detail_header_row, equivalence_plan, error_and_warning_plan,
    two_warned_rows_plan, warned_plan, weekday_abbreviation, ACTIVITY_X, AIRCRAFT_X, BLOCKTIME_X,
    DATE_X, END_X, FROM_X, LAYOVER_X, POS_X, REPORT_X, START_X, TAGS_X, TO_X, TRIP_X, TZ_X,
};
