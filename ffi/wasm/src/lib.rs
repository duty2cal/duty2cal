// SPDX-License-Identifier: MPL-2.0
//! Browser adapter for the `export-duty-plan-calendar` capability.
//!
//! The facade is a thin driving adapter: the page supplies the file's name and
//! bytes, and receives calendar text or a structured failure. It performs no
//! I/O of its own; the only wasm-only dependency is the browser clock the
//! existing `Clock` port needs as the revision fallback.
//!
//! The `wasm_bindgen` exports and `BrowserClock` are compiled only for
//! `wasm32`; the native build exists so `cargo fmt`, `cargo clippy` and the
//! Windows GNU workspace build lint the crate on the host.

use duty2cal_core::application::Clock;
use time::OffsetDateTime;

#[cfg(target_arch = "wasm32")]
use duty2cal_core::application::{
    diagnostic_line, diagnostic_text as core_diagnostic_text, failure_report_text,
    warning_report_text, ConvertError, Severity,
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// The host clock, used by the native build.
///
/// The browser never uses this clock; it exists so the crate compiles and
/// lints without a `wasm32` target.
pub struct NativeClock;

impl Clock for NativeClock {
    fn now_utc(&self) -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }
}

/// The browser clock: the existing `Clock` port over `Date.now()`.
///
/// Only the revision-timestamp fallback reads it; a source that states its own
/// revision keeps that stamp.
#[cfg(target_arch = "wasm32")]
pub struct BrowserClock;

#[cfg(target_arch = "wasm32")]
impl Clock for BrowserClock {
    fn now_utc(&self) -> OffsetDateTime {
        let millis = js_sys::Date::now();
        // `Date.now()` is integral milliseconds since the Unix epoch.
        let nanos = (millis as i128) * 1_000_000;
        OffsetDateTime::from_unix_timestamp_nanos(nanos)
            .expect("the browser clock is representable as a UTC instant")
    }
}

/// Convert one supplied plan and return the page's structured result.
///
/// Success is `{ status: "ok", calendar }`; failure is
/// `{ status: "failure", reason, diagnostic }`. The facade chooses neither a
/// path nor a save location, and performs no I/O of its own.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn convert(filename: &str, bytes: &[u8]) -> JsValue {
    use duty2cal_core::adapters::ics::IcsSink;
    use duty2cal_core::application::ConvertRoster;
    use duty2cal_pdf::PdfRosterSource;

    let result = js_sys::Object::new();
    match ConvertRoster::new(PdfRosterSource, IcsSink, BrowserClock).execute(bytes) {
        Ok(outcome) => {
            set_property(&result, "status", "ok");
            set_property(&result, "calendar", &outcome.calendar);
            set_lines_of(&result, "warnings", &outcome.warnings, Severity::Warning);
            set_property(
                &result,
                "warningReport",
                &warning_report_text(filename, &outcome.warnings),
            );
            set_count(&result, "errorCount", 0);
            set_count(&result, "warningCount", outcome.warnings.len());
        }
        Err(error) => {
            let reason = error.to_string();
            set_property(&result, "status", "failure");
            set_property(&result, "reason", &reason);
            match &error {
                ConvertError::Rows(diagnostics) => {
                    set_property(
                        &result,
                        "diagnostic",
                        &failure_report_text(filename, diagnostics),
                    );
                    set_lines_of(&result, "errors", diagnostics, Severity::Error);
                    set_lines_of(&result, "warnings", diagnostics, Severity::Warning);
                    set_count(
                        &result,
                        "errorCount",
                        diagnostics
                            .iter()
                            .filter(|diagnostic| diagnostic.severity == Severity::Error)
                            .count(),
                    );
                    set_count(
                        &result,
                        "warningCount",
                        diagnostics
                            .iter()
                            .filter(|diagnostic| diagnostic.severity == Severity::Warning)
                            .count(),
                    );
                }
                ConvertError::Source(_) | ConvertError::Table(_) => {
                    set_property(
                        &result,
                        "diagnostic",
                        &core_diagnostic_text(filename, &reason),
                    );
                    set_lines_of(&result, "errors", &[], Severity::Error);
                    set_lines_of(&result, "warnings", &[], Severity::Warning);
                    set_count(&result, "errorCount", 0);
                    set_count(&result, "warningCount", 0);
                }
            }
        }
    }
    result.into()
}

/// Set one string property on the plain result object.
#[cfg(target_arch = "wasm32")]
fn set_property(object: &js_sys::Object, key: &str, value: &str) {
    set_value(object, key, JsValue::from_str(value));
}

/// Set one numeric property on the plain result object.
#[cfg(target_arch = "wasm32")]
fn set_count(object: &js_sys::Object, key: &str, value: usize) {
    set_value(object, key, JsValue::from_f64(value as f64));
}

/// Set one plain value on the result object.
#[cfg(target_arch = "wasm32")]
fn set_value(object: &js_sys::Object, key: &str, value: JsValue) {
    js_sys::Reflect::set(object, &JsValue::from_str(key), &value)
        .expect("the result object accepts the property");
}

/// Set the diagnostic lines of one severity as a JavaScript array.
#[cfg(target_arch = "wasm32")]
fn set_lines_of(
    object: &js_sys::Object,
    key: &str,
    diagnostics: &[duty2cal_core::application::Diagnostic],
    severity: Severity,
) {
    let lines: Vec<String> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == severity)
        .map(diagnostic_line)
        .collect();
    set_lines(object, key, &lines);
}

/// Set a list of lines as a JavaScript string array.
#[cfg(target_arch = "wasm32")]
fn set_lines(object: &js_sys::Object, key: &str, lines: &[String]) {
    let array = js_sys::Array::new();
    for line in lines {
        array.push(&JsValue::from_str(line));
    }
    set_value(object, key, array.into());
}

/// The two-line diagnostic report for a failure the page detects itself.
///
/// The format lives once in `duty2cal_core::application` and is shared with the
/// executable's report path.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn diagnostic_text(filename: &str, reason: &str) -> String {
    core_diagnostic_text(filename, reason)
}
