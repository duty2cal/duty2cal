# Proposal: export-released-roster-calendar

## Release note

> Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.

## Use-Case Context

- Use-case ID: export-duty-plan-calendar ("Export a duty-plan calendar", `!` level, primary Actor Crew member)
- Ideation topic: duty-plan-calendar-conversion ("Duty-plan calendar conversion")
- Seed path: givn/ideation/duty-plan-calendar-conversion/changes/export-released-roster-calendar.md
- Confirmed Personas: crew-member--personal-calendar-owner ("Crew member exporting their own duty plan"), project-maintainer--release-owner ("Maintainer responsible for a public release") — both confirmed and promoted to the project collection by the Handoff on 2026-10-12. The crew member is the accuracy/privacy review lens for the visible conversion outcome; the maintainer is the release-evidence review lens. Neither is a Gherkin Actor of this change.

The change originates from the ready ideation seed for "Export complete Released Roster calendars". Its approved contract is [the use case to export a duty-plan calendar](givn/ideation/duty-plan-calendar-conversion/use-cases/export-duty-plan-calendar.md); the confirmed vocabulary is in [the project glossary](docs/arc42/12-glossary.md).

## Problem / Opportunity

Crew members receive their published monthly duty plan as a PDF and re-enter it by hand into a personal calendar. The plan is a compact roster table: flight legs, absences, ground duties, standby windows, and descriptive annotations. Hand re-entry is slow and loses exactly the distinctions that matter — a leg whose printed days differ from the printed local date, a day off that must stay all-day despite printed clocks, a deadhead leg that carries no operating work, a layover that belongs to a travel leg, a standby window interrupted by a day off.

No converter exists yet. The only artifact so far is a first draft whose promises outran its evidence: it claimed any valid PDF, mis-associated printed UTC times with the local date block, classified all-day activities from zero-valued accounting columns, fixed its output and error paths so a later run could destroy an earlier result, and shipped a golden calendar with overlong lines and untraceable values. Two further constraints frame the problem: duty plans contain identifying personal data that must never reach the public repository, and any published release must be a single self-contained Windows executable whose claims match its verified evidence.

The opportunity is a local, path-or-drag-and-drop converter that turns one supported published plan into one calendar a crew member can import, and that refuses — visibly and without side effects — when it cannot represent the plan completely.

## Proposed Solution

### Supported source

The first release represents one source family: the **Released Roster detail-table family** supplied by the private examples, together with the variations it explicitly validates. Overview calendars, statistics, and footer records supply no dated activities; a row carrying only a date is not an absence or a layover. A source that cannot be read, identified, or interpreted as a supported plan is an explicit unsupported-source failure, never a guessed calendar.

### Activity representation

Every required activity in a supported plan is represented; none may be silently omitted.

- **Vacation day (U)**, **additional vacation day (FZ)**, and the FREE and OFF codes are all-day activities on their duty-plan date. Printed clocks never change their kind or their event timing and never trigger an overnight repair.
- **Office or seminar duty (TK)** and **consulate visa duty (DTUSA)** are timed non-flight activities using their own printed UTC start and end.
- **Standby duty (SB)**, **reserve duty (RE)**, and **single-day standby (SBX)** are timed availability activities, each with its own printed UTC window. Consecutive days stay separate windows, interrupted days off stay separate, and SBX uses its own window — never the cancelled flight's.
- **Deadhead duty (DH)** is a crew assignment on positioning travel that is still duty time. It produces one event for the travel leg, and the title carries the assignment so a compact calendar view distinguishes it, for example `DH 007 ABC-DEF`. Missing aircraft detail or a zero accounting Blocktime does not remove that travel or make it all-day.
- An explicit dated station-bearing **Layover** row is an all-day activity. The printed duration on a travel row stays descriptive and invents no interval.
- An unknown or unsupported activity code fails the conversion rather than defaulting to a flight, a layover, or an absence.

### Dates and timing of timed activities

Timed events carry UTC endpoints derived from the printed UTC time evidence, with each endpoint's date resolved independently. Printed explicit UTC day markers stay authoritative and are resolved to the uniquely matching previous, same, or next Gregorian date around that endpoint's own local-date block, carrying month and year correctly; no unique match fails. Report time, timezone difference, Blocktime, and accounting durations never supply an activity date or instant. Every timed event ends strictly after it starts. A narrow, expressly bounded case — both endpoints unqualified, in one local-date block, with no separate arrival-date block and an end clock earlier than the start — is read as an end on the following UTC day; equal clocks and explicit-marker contradictions are not repaired.

### Description content

An event's description carries the selected fields in this order: Trip ID, Report (LT), aircraft, Blocktime, timezone difference, Layover duration, and Tags. A field the source does not supply is omitted, never manufactured. A known Trip ID is carried across the legs of one unambiguous **roster trip**, including blank continuation cells and page or date breaks, and never leaks into an unrelated duty or a later trip. Descriptive values are not used to compute instants.

Where otherwise compatible departure and arrival fragments supply complementary or equal optional values, they combine to one value. Where they supply genuinely *different* selected optional values, the event is published once and the description retains both, each labelled with the fragment that supplied it (`Report (LT) 07:30 (departure fragment); Report (LT) 08:00 (arrival fragment)`); neither value computes an instant, and this rule never merges conflicting required values (classification, endpoints, route, or explicit crew assignment), which still fail.

### Calendar file and identity

The published calendar is a complete, valid RFC 5545 file: timed events with UTC endpoints, all-day events as dates with a non-inclusive following-day end, escaped text, CRLF line endings, UTF-8, and octet-aware folding that never splits a character. Component identifiers are globally unique and scoped to the **source snapshot** and the represented activity: converting the identical source again keeps the same identifiers even when the output name differs, while a revised source snapshot yields separate identifiers even for activities unchanged from its predecessor. Output numbering and titles are not identity. The revision timestamp is the snapshot's own UTC revision time from the accepted footer for this family; when no usable source revision metadata exists, the actual construction time is used.

### Writing beside the source without destroying anything

A successful calendar is written beside the supplied plan under an unused name, using the ordinary base name when it is free and an enumerated suffix otherwise (`roster.ics`, then `roster-1.ics`). An occupied candidate — including one claimed by a concurrent conversion — is never replaced, and the actual new path is reported. Existing calendar and report contents are unchanged by a conversion.

### Failure and diagnostics

Any conversion failure — unsupported source, unresolved required activity, unresolved date, unmatched travel fragment, or failed publication — is exposed, and a new numbered diagnostic is attempted that names the source file and the failure reason. The new report path is only exposed after that report is actually saved; older reports are retained and never presented as the current one. If no new report can be saved, the original failure stays visible and no older report is claimed as current. No writable-storage, power-loss, backup/cleanup, or re-import guarantee is offered.

### Release evidence

The change is ready for the maintainer to release only when its claims are backed by evidence: generated or anonymized fixtures and golden calendars that exercise the rules above (including the standby codes, month/year boundaries, and the unobserved unqualified-overnight variation); an actual Windows build that runs as one self-contained executable without a companion runtime installer; the adopted licence model enforced (`crates/**` MPL-2.0, `apps/**` GPL-3.0-or-later, no copyleft third-party crate, per-file SPDX headers) — see ADR-0001; and public artifacts free of private source data, checked against identifying combinations and not only names. These are pre-release gates, not new product behaviour.

## Capability Routing

`givn spec route` reports no signal — the permanent corpus (`givn/specs/`) is empty, so there is no command, token, or lexical overlap to rank. The decision is recorded directly:

| Proposed capability | Decision | Rationale |
|---|---|---|
| Export a duty-plan calendar (whole-plan conversion of one supported supplied plan) | `NEW in export-duty-plan-calendar` | No permanent specification exists yet. The confirmed Handoff maps this single actor goal to its own leaf at `givn/specs/export-duty-plan-calendar/usecase.md`; extraction, date assembly, calendar writing, output naming, diagnostics, and release packaging are contributions to that one complete outcome, not separately shipping capabilities. |

## Out of Scope

- Importing the result into a calendar application, and reconciling an already-imported calendar after a plan is revised.
- Comparing plan revisions as a separate user-facing outcome, and automatic duplicate suppression or cleanup.
- Universal support for every PDF producer layout; the first release promises the selected family and its explicitly validated variations.
- Editing or authorising the official duty plan; granting leave; checking entitlement quotas; calculating pay or duty/rest legality.
- Obtaining a visa, booking positioning travel, or contacting external scheduling institutions.
- Activating standby or reserve, calculating call-to-report notice, or performing check-in or reporting.
- Publishing or approving a converter release as a product command, and any hosted upload, GUI, installer, or calendar-account connection.
- Replacing or deleting existing calendars or reports; no automatic backup, version cleanup, or re-import reconciliation.
- Formal claims about the FZ acronym expansion or its entitlement provenance, and any eligibility validator.

## Open Questions

### D1 — Differing optional descriptive values on compatible travel fragments

- Question: when a departure fragment and an arrival fragment of the same travel leg are otherwise compatible but supply different selected optional values, should conversion reject the calendar, or publish one event keeping both values labelled by their source fragment?
- Options:
  1. **Reject the conflicting optional values** — a difference in any selected optional description value fails the conversion like a required-value contradiction; gain: never emits one apparently authoritative value chosen from two; cost: a legitimate multi-valued annotation has no supported representation until another evidenced form is chosen; in practice: Report (LT) 07:30 on the departure fragment and 08:00 on the arrival fragment yields a numbered diagnostic instead of a calendar.
  2. **Publish both values with source-fragment roles** — one event retains both supplied values, each labelled with its fragment, and neither computes an instant; gain: no supplied descriptive value is lost while the leg is represented once; cost: longer, provenance-labelled descriptions, which must never merge conflicting required values; in practice: the same pair yields one event described `Report (LT) 07:30 (departure fragment); Report (LT) 08:00 (arrival fragment)`.
- Disposition: **answered** — the user selected option 2 (publish both with source-fragment roles) on 2026-10-12, in this proposal step. Rejected: option 1, because the leg's real interval is known and only the annotation is ambiguous, so losing supplied evidence is not warranted. This also resolves the `(open: Q28)` qualification carried through the approved contract and its success guarantee; the descriptive-fragment question (Q28) is no longer pending.

### D2 — Rights to publish source-derived test material

- Question: what source-use terms or authorisation, if any, permit publishing an anonymised derivative or adapted layout of a private duty-plan source?
- Options:
  1. **Establish permission before any derivative** — do not publish adapted or anonymised source material until its rights are evidenced; gain: legal provenance supports the public-material claim; cost: extra evidence, and a permissive project licence does not manufacture permission; in practice: an adapted source document is withheld until permission is shown.
  2. **Use independently generated material** — rely on generated examples only and leave source-derived publication unverified; gain: avoids needing a derivative-publication assumption; cost: generated layouts still need real extraction-fidelity and licence review and are not identical to producer evidence; in practice: a fictional future roster is generated for the evidence set while real plans stay local.
- Disposition: **out of scope for this change** — the evidence route already fixed by the approved contract is independently generated or anonymised material, and this change commits no source-derived public artifact. The publication-rights question (Q5) stays parked in the ideation record; it becomes live only if a source-derived fixture is later proposed.

### D3 — Formal FZ expansion and entitlement provenance

- Question: what authoritative source establishes the formal expansion of FZ and its entitlement provenance, if a verified statement is ever needed?
- Options:
  1. **Establish evidence before formal claims** — obtain authoritative wording/provenance before naming an agreement; gain: accurate background if needed; cost: research unrelated to the calendar outcome; in practice: the glossary names an agreement only after evidence.
  2. **Retain the practical meaning without expansion or provenance** — keep the optional background unknown; gain: bounded, accurate converter behaviour; cost: no formal entitlement explanation is provided; in practice: FZ appears as the assigned all-day activity without asserting an agreement or enforcing age or annual conditions.
- Disposition: **out of scope for this change** — the converter represents an already assigned FZ day as an all-day activity and performs no eligibility, age, or annual-quota validation. The FZ-provenance question (Q13) stays parked; it never blocks conversion.
