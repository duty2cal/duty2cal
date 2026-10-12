// SPDX-License-Identifier: MPL-2.0
//! `duty2cal` core — bounded context "Roster".
//!
//! Pure domain, application ports and use case, and the dependency-free ICS
//! adapter. No I/O, no clock, no environment (enforced by `clippy.toml`).

pub mod adapters;
pub mod application;
pub mod domain;
