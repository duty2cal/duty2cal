// SPDX-License-Identifier: MPL-2.0
//! Domain: the `Roster` aggregate and its value objects.
//!
//! Pure types and rules; no I/O, no clock, no format knowledge.

/// One represented calendar activity.
///
/// The variants the following scenarios add (all-day, timed availability,
/// timed non-flight duty, travel, layover) extend this value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Activity {
    /// Calendar title of the activity.
    pub summary: String,
}

/// The whole-plan composition of one supplied duty plan.
///
/// Only a roster whose required activities are all resolved may be published;
/// completeness is a property of this aggregate, not of the caller.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Roster {
    /// Represented activities, in source order.
    pub activities: Vec<Activity>,
}

impl Roster {
    /// An empty but complete roster: a plan with no dated activities.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Whether every required activity is resolved and the roster may be published.
    pub fn is_complete(&self) -> bool {
        true
    }
}
