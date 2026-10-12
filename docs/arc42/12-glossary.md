# 12. Glossary

Confirmed vocabulary promoted by the user's Handoff on **2026-10-12**.
Source: [Duty-plan calendar conversion](../ideation/duty-plan-calendar-conversion.md),
[introduction/collision records](../../givn/ideation/duty-plan-calendar-conversion/domain-terms.md),
and [approved contract](../../givn/ideation/duty-plan-calendar-conversion/use-cases/export-duty-plan-calendar.md).
This is vocabulary/required behavior, not an implemented enforcement claim.

## Ubiquitous language

| Term | Definition | Contract use | Code symbol | Contrast / anti-terms |
|---|---|---|---|---|
| Additional vacation day | FZ roster activity denoting an additional vacation-related day off. | All-day; source times ignored. | Not implemented. | Distinct from U; no verified entitlement/quota or acronym expansion. |
| Consulate visa duty | Non-flight DTUSA duty for a consulate visit to obtain a USA visa. | Own printed UTC interval. | Not implemented. | Not a flight, converter-performed visa application, or generic DT-suffix rule. |
| Deadhead duty | Pos DH positioning to a departure destination in uniform without usual operating work, still duty time. | One travel interval with DH title prefix. | Not implemented. | Not time off, another event for the same leg, or duration derived from Blocktime. |
| DT prefix | Activity-code prefix expanding to Duty Time in observed DTUSA. | Preserve known meaning without interpreting unknown suffixes. | Not implemented. | Code text, not a numeric duty-duration metric. |
| Duty-plan calendar | Complete activity composition from one supplied duty plan governing readiness to export. | Success only after required activities/dates/fields resolve. | Not implemented. | Aggregate, not PDF, output file, request record, service, or snapshot. |
| Office or seminar duty | TK non-flight duty requiring office/development/team-information attendance. | Printed UTC Start/End; no flight route required. | Not implemented. | Not vacation/flight; no unprovided acronym expansion. |
| Reserve duty | Bounded RE availability differing from SB in activation-to-report notice time. | Own UTC window; no notice calculator. | Not implemented. | Not an activation workflow or all-day default. |
| Roster trip | Source grouping of assigned travel legs associated with one Trip ID. | Carry known ID within one unambiguous travel trip. | Not implemented. | Not one leg, leisure trip, or independent aggregate. |
| Single-day standby | Isolated/replacement SBX standby with its own UTC window. | Current SBX interval, not a former flight's times. | Not implemented. | Single-day is not all-day; no extra cancelled-flight/import-cancellation logic. |
| Source revision timestamp | UTC snapshot revision metadata using the user-accepted Created footer convention for this family. | Preferred DTSTAMP; actual construction-time fallback. | Not implemented. | Not activity start, month-first constant, or universal Created/Last-Modified equivalence. |
| Source snapshot | Fixed input PDF version scoping its component identities. | Identical snapshot is deterministic; revised snapshot has separate IDs. | Not implemented. | Not numbered output copy, another aggregate, or calendar-data anonymization. |
| Standby duty | Bounded SB/Stby availability with start/end times for possible activation. | Separate dated UTC windows with FREE interruptions preserved. | Not implemented. | Not time off, continuous multi-day block, or annual-statistic event. |
| Vacation day | U/Urlaub activity denoting vacation on its duty-plan date. | All-day; source times ignored. | Not implemented. | Distinct from FREE/OFF/FZ; no entitlement inference. |

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

Only thirteen confirmed introductions were promoted. Candidate **Duty plan**,
**Flight leg**, and **Calendar export** remain topic candidates, not silently
confirmed here. No rejected entry exists to remove. Suggested FZ expansion and
agreement/age/quota provenance remain background, not leave/eligibility rules.
