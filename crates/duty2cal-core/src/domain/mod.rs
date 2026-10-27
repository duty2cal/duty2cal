// SPDX-License-Identifier: MPL-2.0
//! Domain: the `Roster` aggregate, its value objects and its pure rules.
//!
//! Pure types and rules; no I/O, no clock, no format knowledge.

pub mod timing;

use time::{Date, OffsetDateTime};

/// Why a domain value could not be constructed.
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    /// A timed interval must end strictly after it starts.
    #[error("the interval end must be after its start")]
    NonPositiveInterval,
}

/// A UTC interval with the invariant that its end is strictly after its start.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UtcInterval {
    start: OffsetDateTime,
    end: OffsetDateTime,
}

impl UtcInterval {
    /// Build an interval, rejecting a zero-length or reversed one.
    pub fn new(start: OffsetDateTime, end: OffsetDateTime) -> Result<Self, DomainError> {
        if end > start {
            Ok(Self { start, end })
        } else {
            Err(DomainError::NonPositiveInterval)
        }
    }

    /// Interval start.
    pub fn start(&self) -> OffsetDateTime {
        self.start
    }

    /// Interval end.
    pub fn end(&self) -> OffsetDateTime {
        self.end
    }
}

/// Append a selected description field when the source supplied it.
fn push_field(fields: &mut Vec<(String, String)>, label: &str, value: &Option<String>) {
    if let Some(value) = value {
        fields.push((label.to_string(), value.clone()));
    }
}

/// Append a selected optional value, retaining both fragments when they differ.
fn push_value(fields: &mut Vec<(String, String)>, label: &str, value: &Option<FieldValue>) {
    match value {
        None => {}
        Some(FieldValue::Single(value)) => fields.push((label.to_string(), value.clone())),
        Some(FieldValue::Differing { departure, arrival }) => {
            fields.push((
                label.to_string(),
                format!("{departure} (departure fragment)"),
            ));
            fields.push((label.to_string(), format!("{arrival} (arrival fragment)")));
        }
    }
}

/// A selected optional description value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldValue {
    /// Supplied by one fragment, or supplied equally by both.
    Single(String),
    /// Supplied differently by the two fragments; both are retained, each with
    /// the fragment that supplied it.
    Differing {
        /// The value printed on the departure fragment.
        departure: String,
        /// The value printed on the arrival fragment.
        arrival: String,
    },
}

/// How a travel leg's two printed fields read together.
///
/// The plan prints the crew position in its `Pos` column and the flight's
/// activity in its `Activity` column. They stay two fields; this is the one
/// rule that reads them together, so the title a crew member sees and the
/// fragment a failure message names cannot drift apart.
pub fn printed_identification(position: Option<&str>, activity: &str) -> String {
    match position {
        Some(position) if !position.is_empty() => format!("{position} {activity}"),
        _ => activity.to_string(),
    }
}

/// One represented calendar activity.
///
/// The travel variant is the largest by design: it carries the descriptive
/// fields of a leg. Boxing it would only add an indirection per entry.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DutyEntry {
    /// Code-driven all-day activity: FREE, OFF, U (vacation) or FZ.
    AllDay {
        /// Duty-plan date.
        date: Date,
        /// The source code, kept distinct.
        code: String,
        /// Tags, when supplied.
        tags: Option<String>,
    },
    /// Timed travel leg.
    Travel {
        /// Duty-plan date of the departure row.
        date: Date,
        /// The plan's `Activity` cell, as printed: the flight number of a
        /// flight leg, leading zeroes preserved.
        activity: String,
        /// The plan's `Pos` cell, as printed, when the plan prints one.
        position: Option<String>,
        /// Departure station.
        from: String,
        /// Arrival station.
        to: String,
        /// Printed UTC interval.
        interval: UtcInterval,
        /// Roster trip context, when known.
        trip: Option<String>,
        /// Report time (local), descriptive only.
        report_lt: Option<FieldValue>,
        /// Aircraft, when supplied.
        aircraft: Option<FieldValue>,
        /// Accounting block time, descriptive only.
        blocktime: Option<FieldValue>,
        /// Timezone difference, descriptive only.
        tz_diff: Option<FieldValue>,
        /// Layover duration, descriptive only.
        layover: Option<FieldValue>,
        /// Tags, when supplied.
        tags: Option<FieldValue>,
    },
    /// Timed availability window: SB, RE or SBX.
    Availability {
        /// Duty-plan date.
        date: Date,
        /// The source code, kept distinct.
        code: String,
        /// Station, when supplied.
        station: Option<String>,
        /// Printed UTC window.
        interval: UtcInterval,
        /// Tags, when supplied.
        tags: Option<String>,
    },
    /// Timed non-flight duty: TK, DTUSA or EM.
    NonFlightDuty {
        /// Duty-plan date.
        date: Date,
        /// The source code, kept distinct.
        code: String,
        /// Station, when supplied.
        station: Option<String>,
        /// Roster trip context, when known.
        trip: Option<String>,
        /// Printed UTC interval.
        interval: UtcInterval,
        /// Tags, when supplied.
        tags: Option<String>,
    },
    /// Explicit dated station-bearing layover row.
    Layover {
        /// Duty-plan date.
        date: Date,
        /// Layover station.
        station: String,
    },
    /// An activity represented from a row whose code the converter does not
    /// know. The kind comes from the printed evidence, never from the code's
    /// letters: both printed endpoints make it timed, none makes it all-day.
    UnrecognizedActivity {
        /// Duty-plan date.
        date: Date,
        /// The printed activity code, verbatim.
        code: String,
        /// The printed station, when the row prints one.
        station: Option<String>,
        /// The printed UTC window; `None` means all-day.
        interval: Option<UtcInterval>,
    },
}

impl DutyEntry {
    /// The duty-plan date this entry belongs to.
    pub fn date(&self) -> Date {
        match self {
            Self::AllDay { date, .. }
            | Self::Travel { date, .. }
            | Self::Availability { date, .. }
            | Self::NonFlightDuty { date, .. }
            | Self::Layover { date, .. }
            | Self::UnrecognizedActivity { date, .. } => *date,
        }
    }

    /// The printed UTC interval, absent for all-day activities.
    pub fn interval(&self) -> Option<UtcInterval> {
        match self {
            Self::AllDay { .. } | Self::Layover { .. } => None,
            Self::Travel { interval, .. }
            | Self::Availability { interval, .. }
            | Self::NonFlightDuty { interval, .. } => Some(*interval),
            Self::UnrecognizedActivity { interval, .. } => *interval,
        }
    }

    /// The calendar title.
    pub fn summary(&self) -> String {
        match self {
            Self::AllDay { code, .. } => code.clone(),
            Self::Travel {
                activity,
                position,
                from,
                to,
                report_lt,
                ..
            } => {
                let title = format!(
                    "{} {from}-{to}",
                    printed_identification(position.as_deref(), activity)
                );
                match report_lt {
                    Some(FieldValue::Single(value)) => format!("{title} Report {value} LT"),
                    None | Some(FieldValue::Differing { .. }) => title,
                }
            }
            Self::Availability { code, .. }
            | Self::NonFlightDuty { code, .. }
            | Self::UnrecognizedActivity { code, .. } => code.clone(),
            Self::Layover { station, .. } => format!("Layover {station}"),
        }
    }

    /// Selected description fields, in the approved order, omitting any the
    /// source did not supply.
    pub fn description_fields(&self) -> Vec<(String, String)> {
        let mut fields = Vec::new();
        if let Self::Travel {
            trip,
            report_lt,
            aircraft,
            blocktime,
            tz_diff,
            layover,
            tags,
            ..
        } = self
        {
            push_field(&mut fields, "Trip", trip);
            push_value(&mut fields, "Report (LT)", report_lt);
            push_value(&mut fields, "A/C", aircraft);
            push_value(&mut fields, "Blocktime", blocktime);
            push_value(&mut fields, "Timezone Difference", tz_diff);
            push_value(&mut fields, "Layover duration", layover);
            push_value(&mut fields, "Tags", tags);
        }
        fields
    }

    /// The calendar location, when the activity has one.
    pub fn location(&self) -> Option<String> {
        match self {
            Self::AllDay { .. } => None,
            Self::Travel { from, to, .. } => Some(format!("{from}-{to}")),
            Self::Availability { station, .. }
            | Self::NonFlightDuty { station, .. }
            | Self::UnrecognizedActivity { station, .. } => station.clone(),
            Self::Layover { station, .. } => Some(station.clone()),
        }
    }
}

/// The identity of the fixed source version a roster was read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapshotId(pub u64);

/// The whole-plan composition of one supplied duty plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Roster {
    /// The source version these activities were read from.
    pub snapshot: SnapshotId,
    /// Represented activities, in source order.
    pub entries: Vec<DutyEntry>,
}
