// SPDX-License-Identifier: MPL-2.0
//! Driving adapter: argv → `ConvertRoster` → reported path/exit code.
//! Wires the filesystem adapter (`fs`) and the logger (`log`).

mod fs;

use std::path::Path;
use std::process::ExitCode;

use duty2cal_core::adapters::ics::IcsSink;
use duty2cal_core::application::{Clock, ConvertRoster};
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
        Err(error) => return reject(&source, &format!("cannot read the source: {error}")),
    };

    match ConvertRoster::new(PdfRosterSource, IcsSink, SystemClock).execute(&bytes) {
        Ok(outcome) => publish(&source, &outcome.calendar),
        Err(error) => reject(&source, &error.to_string()),
    }
}

/// Write the complete calendar beside the source under an unused name.
fn publish(source: &Path, calendar: &str) -> ExitCode {
    let Some(path) = fs::unused_calendar_path(source) else {
        return reject(source, "no unused calendar name is available");
    };
    match std::fs::write(&path, calendar) {
        Ok(()) => {
            println!("Written: {}", path.display());
            ExitCode::SUCCESS
        }
        Err(error) => reject(
            source,
            &format!("the calendar could not be written: {error}"),
        ),
    }
}

/// Report a failure, attempt a numbered diagnostic, and exit non-zero.
fn reject(source: &Path, reason: &str) -> ExitCode {
    report(source, reason);
    ExitCode::FAILURE
}

fn report(source: &Path, reason: &str) {
    let message = format!("conversion failed for {}: {reason}", source.display());
    match fs::unused_diagnostic_path(source) {
        Some(path) => {
            let body = format!(
                "Source PDF: {}\nReason: {reason}\n",
                source.file_name().unwrap_or_default().to_string_lossy()
            );
            match std::fs::write(&path, body) {
                Ok(()) => println!("{message}\nDiagnostic: {}", path.display()),
                Err(error) => {
                    println!("{message}\nDiagnostic: no new report was saved: {error}");
                }
            }
        }
        None => println!("{message}\nDiagnostic: no unused report name is available"),
    }
}
