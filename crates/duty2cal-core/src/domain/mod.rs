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

/// The crew assignment on a travel leg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    /// Ordinary operating assignment.
    Operating,
    /// Positioning travel in uniform without operating work.
    Deadhead,
}

/// One represented calendar activity.
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
        /// Printed designator, leading zeroes preserved.
        designator: String,
        /// Departure station.
        from: String,
        /// Arrival station.
        to: String,
        /// Crew assignment.
        position: Position,
        /// Printed UTC interval.
        interval: UtcInterval,
        /// Roster trip context, when known.
        trip: Option<String>,
        /// Report time (local), descriptive only.
        report_lt: Option<String>,
        /// Aircraft, when supplied.
        aircraft: Option<String>,
        /// Accounting block time, descriptive only.
        blocktime: Option<String>,
        /// Timezone difference, descriptive only.
        tz_diff: Option<String>,
        /// Layover duration, descriptive only.
        layover: Option<String>,
        /// Tags, when supplied.
        tags: Option<String>,
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
}

impl DutyEntry {
    /// The duty-plan date this entry belongs to.
    pub fn date(&self) -> Date {
        match self {
            Self::AllDay { date, .. }
            | Self::Travel { date, .. }
            | Self::Availability { date, .. }
            | Self::NonFlightDuty { date, .. }
            | Self::Layover { date, .. } => *date,
        }
    }

    /// The printed UTC interval, absent for all-day activities.
    pub fn interval(&self) -> Option<UtcInterval> {
        match self {
            Self::AllDay { .. } | Self::Layover { .. } => None,
            Self::Travel { interval, .. }
            | Self::Availability { interval, .. }
            | Self::NonFlightDuty { interval, .. } => Some(*interval),
        }
    }

    /// The calendar title.
    pub fn summary(&self) -> String {
        match self {
            Self::AllDay { code, .. } => code.clone(),
            Self::Travel {
                designator,
                from,
                to,
                position,
                ..
            } => match position {
                Position::Deadhead => format!("DH {designator} {from}-{to}"),
                Position::Operating => format!("{designator} {from}-{to}"),
            },
            Self::Availability { code, .. } | Self::NonFlightDuty { code, .. } => code.clone(),
            Self::Layover { station, .. } => format!("Layover {station}"),
        }
    }

    /// Selected description fields, in the approved order, omitting any the
    /// source did not supply.
    pub fn description_fields(&self) -> Vec<(String, String)> {
        let mut fields = Vec::new();
        if let Self::Travel {
            trip: Some(trip), ..
        } = self
        {
            fields.push(("Trip".to_string(), trip.clone()));
        }
        fields
    }

    /// The calendar location, when the activity has one.
    pub fn location(&self) -> Option<String> {
        match self {
            Self::AllDay { .. } => None,
            Self::Travel { from, to, .. } => Some(format!("{from}-{to}")),
            Self::Availability { station, .. } | Self::NonFlightDuty { station, .. } => {
                station.clone()
            }
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
