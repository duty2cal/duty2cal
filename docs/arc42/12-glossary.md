# 12. Glossary

Confirmed vocabulary promoted by the user's Handoff on **2026-10-12**, plus
three terms added by the change to export released roster calendars
(export-released-roster-calendar).
Source: [Duty-plan calendar conversion](../ideation/duty-plan-calendar-conversion.md),
[introduction/collision records](../../givn/ideation/duty-plan-calendar-conversion/domain-terms.md),
and [approved contract](../../givn/ideation/duty-plan-calendar-conversion/use-cases/export-duty-plan-calendar.md).
This is vocabulary/required behavior, not an implemented enforcement claim.

## Ubiquitous language

| Term | Definition | Contract use | Code symbol | Contrast / anti-terms |
|---|---|---|---|---|
| Additional vacation day | FZ roster activity denoting an additional vacation-related day off. | All-day; source times ignored. | `DutyEntry::AllDay`, `ALL_DAY_CODES` (`crates/duty2cal-core/src/application/mod.rs`) | Distinct from U; no verified entitlement/quota or acronym expansion. |
| Consulate visa duty | Non-flight DTUSA duty for a consulate visit to obtain a USA visa. | Own printed UTC interval. | `DutyEntry::NonFlightDuty`, `NON_FLIGHT_CODES` (`application/mod.rs`) | Not a flight, converter-performed visa application, or generic DT-suffix rule. |
| Deadhead duty | Pos DH positioning to a departure destination in uniform without usual operating work, still duty time. | One travel interval with DH title prefix. | `Position::Deadhead`, `DutyEntry::summary` (`crates/duty2cal-core/src/domain/mod.rs`) | Not time off, another event for the same leg, or duration derived from Blocktime. |
| DT prefix | Activity-code prefix expanding to Duty Time in observed DTUSA. | Preserve known meaning without interpreting unknown suffixes. | `NON_FLIGHT_CODES` (`application/mod.rs`) | Code text, not a numeric duty-duration metric. |
| Duty-plan calendar | Complete activity composition from one supplied duty plan governing readiness to export. | Success only after required activities/dates/fields resolve. | `Roster` (`domain/mod.rs`) | Aggregate, not PDF, output file, request record, service, or snapshot. |
| EM duty | Non-flight duty represented from its printed UTC interval and source station; the code's expansion is not asserted. | Timed activity using its own printed UTC window. | `DutyEntry::NonFlightDuty`, `NON_FLIGHT_CODES` (`application/mod.rs`) | Not a flight, vacation, or an asserted code expansion. |
| Layover | Explicit dated station-bearing source row representing a rest stop. | All-day activity on its date; a travel row's printed duration stays descriptive. | `DutyEntry::Layover`, `layover_station` (`application/mod.rs`) | Not a travel leg, all-day absence, or an invented interval. |
| Office or seminar duty | TK non-flight duty requiring office/development/team-information attendance. | Printed UTC Start/End; no flight route required. | `DutyEntry::NonFlightDuty`, `NON_FLIGHT_CODES` (`application/mod.rs`) | Not vacation/flight; no unprovided acronym expansion. |
| Released Roster detail-table family | The supported source layout: the monthly crew duty-plan detail table, with repeated headers and page continuations. | The first release's supported source; an unrecognized family fails explicitly. | `PdfRosterSource`, `SourceError::UnsupportedFamily` (`crates/duty2cal-pdf/src/lib.rs`) | Not every PDF producer layout; not the overview, statistics, or footer tables. |
| Reserve duty | Bounded RE availability differing from SB in activation-to-report notice time. | Own UTC window; no notice calculator. | `DutyEntry::Availability`, `AVAILABILITY_CODES` (`application/mod.rs`) | Not an activation workflow or all-day default. |
| Roster trip | Source grouping of assigned travel legs associated with one Trip ID. | Carry known ID within one unambiguous travel trip. | the Trip ID column cell and `Row::carried_trip` (`application/mod.rs`) | Not one leg, leisure trip, or independent aggregate. |
| Single-day standby | Isolated/replacement SBX standby with its own UTC window. | Current SBX interval, not a former flight's times. | `DutyEntry::Availability`, `row_interval` (`application/mod.rs`) | Single-day is not all-day; no extra cancelled-flight/import-cancellation logic. |
| Source revision timestamp | UTC snapshot revision metadata using the user-accepted Created footer convention for this family. | Preferred DTSTAMP; actual construction-time fallback. | `SourceDocument::revision`, `created_footer` (`crates/duty2cal-pdf/src/lib.rs`) | Not activity start, month-first constant, or universal Created/Last-Modified equivalence. |
| Source snapshot | Fixed input PDF version scoping its component identities. | Identical snapshot is deterministic; revised snapshot has separate IDs. | `SnapshotId`, `fingerprint` (`application/mod.rs`) | Not numbered output copy, another aggregate, or calendar-data anonymization. |
| Standby duty | Bounded SB/Stby availability with start/end times for possible activation. | Separate dated UTC windows with FREE interruptions preserved. | `DutyEntry::Availability`, `AVAILABILITY_CODES` (`application/mod.rs`) | Not time off, continuous multi-day block, or annual-statistic event. |
| Vacation day | U/Urlaub activity denoting vacation on its duty-plan date. | All-day; source times ignored. | `DutyEntry::AllDay`, `ALL_DAY_CODES` (`application/mod.rs`) | Distinct from FREE/OFF/FZ; no entitlement inference. |

## Aggregates / boundaries

| Root | Extent and invariant | Commands | Events |
|---|---|---|---|
| Duty-plan calendar | Required representations from one supplied plan, including fragments/date evidence; success only when every required activity/field resolves, never an apparently complete partial calendar. | Export duty plan; Complete duty-plan calendar; Publish calendar export; Record conversion failure. Last three are internal responsibilities for the Crew member. | Duty plan supplied; Duty-plan calendar completed; Duty-plan conversion rejected; Calendar export published; Calendar export publication failed; Conversion diagnostic written; Conversion diagnostic unavailable. |

PDFs, output files, diagnostics, source snapshots, and trip groupings are
records/projections or source context, **not additional aggregates**. The boundary
requires no database, persistent workflow, or particular code type.

## Invariants and required enforcement sites

| Invariant | Required future enforcement — not implemented yet |
|---|---|
| Resolve required activities/fields before successful publication; do not silently omit unknown required data. | Domain/application validation and scenario coverage. |
| Timed End follows Start; explicit day qualifiers use unique adjacent Gregorian dates and remain authoritative. | Activity/date construction and source interpretation tests. |
| FREE/OFF/U/FZ stay all-day regardless of clocks; SB/SBX/RE/TK/DTUSA use own UTC windows. | Classification/representation and expert-source validation. |
| Reporting, offsets, accounting values, and revision metadata do not calculate activity instants. | Date assembly and independent-field regressions. |
| Source/prior calendar/report contents remain unchanged; occupied candidates are not clobbered under races. | Publication/diagnostic ports and filesystem fault/concurrency evidence. |
| Every failure attempts a new report; failed saving preserves the primary error and never calls an older report current. | Application failure flow/read model and failure scenarios. |
| IDs are globally scoped/distinct and deterministic for the identical snapshot, independent of counter/title. | Snapshot identity/encoding and repeated/distinct-source evidence. |
| Private/identifying source values are not committed; public data has generated/anonymized provenance. | Repository/source-provenance gates and human review. |

Differing optional descriptions remain an unobserved parked policy; these
invariants do not silently choose their retention/merge handling.

## Lifecycle events

| Event | Observation |
|---|---|
| Duty plan supplied | A source is available for the Crew member's conversion attempt. |
| Duty-plan calendar completed | Required data resolved; publication has not yet succeeded. |
| Duty-plan conversion rejected | Interpretation cannot establish a complete representation. |
| Calendar export published | A complete new calendar was actually published at an unused path. |
| Calendar export publication failed | Completion did not produce a successful new output; failure is explicit. |
| Conversion diagnostic written | A new current-run report was actually persisted at an unused numbered path. |
| Conversion diagnostic unavailable | The current failure could not save a new report; old files prove no new write. |

## Promotion boundary and unverified terms

Sixteen confirmed introductions are promoted: the thirteen Handoff terms plus
**EM duty**, **Layover**, and **Released Roster detail-table family**, added by
the change to export released roster calendars (export-released-roster-calendar)
through its design-review glossary-scope decision. Raw source-column labels
(A/C, Blocktime, Report (LT), Timezone Difference, Tags) and the FREE/OFF day
codes are used verbatim in the specification and are not confirmed domain terms.
| Travel leg | One flight from a departure station to an arrival station, printed either as a whole row or as a departure row and an arrival row that the converter pairs. | One timed travel activity titled by designator and route. | `DutyEntry::Travel` (`crates/duty2cal-core/src/domain/mod.rs`) | Not a printed row, not two activities when a leg is split across rows, not an all-day absence. |
| Availability duty | Bounded standby or reserve window the crew member must be reachable in (SB, SBX, RE). | One timed availability activity with its own UTC window and station. | `DutyEntry::Availability`, `AVAILABILITY_CODES` (`crates/duty2cal-core/src/application/mod.rs`) | Not an all-day day off, not a flight, not a non-flight duty. |

Candidate **Duty plan** and **Calendar export** remain topic
candidates, not silently confirmed here. No rejected entry exists to remove.
Suggested FZ expansion and agreement/age/quota provenance remain background, not
leave/eligibility rules.
