// SPDX-License-Identifier: MPL-2.0
//! Driving adapter: argv → `ConvertRoster` → reported path/exit code.
//! Wires the filesystem adapter (`fs`) and the logger (`log`).

mod fs;

use std::path::Path;
use std::process::ExitCode;

use duty2cal_core::adapters::ics::IcsSink;
use duty2cal_core::application::{
    diagnostic_line, diagnostic_text, failure_report_text, warning_report_text, Clock,
    ConversionOutcome, ConvertError, ConvertRoster, Severity,
};
use duty2cal_pdf::PdfRosterSource;
use time::OffsetDateTime;

/// The host clock, used for the revision timestamp fallback.
///
/// `DUTY2CAL_CONSTRUCTION_CLOCK` supplies a fixed construction instant, so a
/// run can be deterministic without a production-value shortcut (the change's
/// timestamp-provenance decision).
struct SystemClock;

impl Clock for SystemClock {
    fn now_utc(&self) -> OffsetDateTime {
        if let Ok(value) = std::env::var(CONSTRUCTION_CLOCK) {
            if let Some(instant) = parse_stamp(&value) {
                return instant;
            }
        }
        OffsetDateTime::now_utc()
    }
}

/// The environment variable that fixes the construction instant.
const CONSTRUCTION_CLOCK: &str = "DUTY2CAL_CONSTRUCTION_CLOCK";

/// Parse a `YYYY-MM-DDTHH:MM:SSZ` construction instant.
fn parse_stamp(value: &str) -> Option<OffsetDateTime> {
    let value = value.trim().trim_end_matches('Z');
    let (date, clock) = value.split_once('T')?;
    let date = duty2cal_core::domain::timing::parse_iso_date(date)?;
    let clock = duty2cal_core::domain::timing::parse_clock(clock)?;
    Some(date.with_time(clock).assume_utc())
}

fn main() -> ExitCode {
    let Some(argument) = std::env::args_os().nth(1) else {
        eprintln!("usage: duty2cal <duty-plan.pdf>");
        return ExitCode::from(2);
    };
    let source = std::path::PathBuf::from(argument);

    let bytes = match std::fs::read(&source) {
        Ok(bytes) => bytes,
        Err(error) => return reject_reason(&source, &format!("cannot read the source: {error}")),
    };

    match ConvertRoster::new(PdfRosterSource, IcsSink, SystemClock).execute(&bytes) {
        Ok(outcome) => publish_success(&source, &outcome),
        Err(error) => reject(&source, &error),
    }
}

/// Write the complete calendar and, when there are warnings, the warning
/// report beside the source under unused names.
///
/// The write creates the file, so a name taken since the check is skipped
/// rather than overwritten.
fn publish_success(source: &Path, outcome: &ConversionOutcome) -> ExitCode {
    // The warnings reach the screen before publication, so a later publication
    // failure cannot hide the fallbacks the run used.
    for warning in &outcome.warnings {
        println!("Warning: {}", diagnostic_line(warning));
    }

    match fs::publish(source, fs::Kind::Calendar, &outcome.calendar) {
        Ok(Some(path)) => {
            println!("Written: {}", path.display());
            if !outcome.warnings.is_empty() {
                publish_warnings(source, &outcome.warnings);
            }
            ExitCode::SUCCESS
        }
        Ok(None) => reject_reason(source, "no unused calendar name is available"),
        Err(error) => reject_reason(
            source,
            &format!("the calendar could not be written: {error}"),
        ),
    }
}

/// Write the warning report, or say why it was not saved.
///
/// A warning report that cannot be saved does not withdraw the published
/// calendar or hide the warnings already printed.
fn publish_warnings(source: &Path, warnings: &[duty2cal_core::application::Diagnostic]) {
    let body = warning_report_text(&source_name(source), warnings);
    match fs::publish(source, fs::Kind::Warning, &body) {
        Ok(Some(path)) => println!("Warning report: {}", path.display()),
        Ok(None) => println!("Warning report: no unused report name is available"),
        Err(error) => println!("Warning report: no new report was saved: {error}"),
    }
}

/// Report a converted failure, attempt the matching numbered report, and exit
/// non-zero.
fn reject(source: &Path, error: &ConvertError) -> ExitCode {
    let reason = error.to_string();
    let message = format!("conversion failed for {}: {reason}", source.display());
    let name = source_name(source);
    let body = match error {
        ConvertError::Rows(diagnostics) => {
            for diagnostic in diagnostics {
                let severity = match diagnostic.severity {
                    Severity::Error => "Error",
                    Severity::Warning => "Warning",
                };
                println!("{severity}: {}", diagnostic_line(diagnostic));
            }
            failure_report_text(&name, diagnostics)
        }
        ConvertError::Source(_) | ConvertError::Table(_) => diagnostic_text(&name, &reason),
    };
    report(source, &message, &body)
}

/// Report a failure that is not about a row, attempt a numbered report, and
/// exit non-zero.
fn reject_reason(source: &Path, reason: &str) -> ExitCode {
    let message = format!("conversion failed for {}: {reason}", source.display());
    let body = diagnostic_text(&source_name(source), reason);
    report(source, &message, &body)
}

fn report(source: &Path, message: &str, body: &str) -> ExitCode {
    match fs::publish(source, fs::Kind::Diagnostic, body) {
        Ok(Some(path)) => println!("{message}\nDiagnostic: {}", path.display()),
        Err(error) => println!("{message}\nDiagnostic: no new report was saved: {error}"),
        Ok(None) => println!("{message}\nDiagnostic: no unused report name is available"),
    }
    ExitCode::FAILURE
}

/// The source's filename, as the reports spell it.
fn source_name(source: &Path) -> String {
    source
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}
