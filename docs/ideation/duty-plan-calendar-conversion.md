# Duty-plan calendar conversion

Handoff published with the user's confirmation on **2026-10-12**. This records the approved domain contract and planning input, not an implemented or validated release.

## Planning records

- [Source topic and resume state](../../givn/ideation/duty-plan-calendar-conversion/session.md).
- [Approved use case: Export a duty-plan calendar](../../givn/ideation/duty-plan-calendar-conversion/use-cases/export-duty-plan-calendar.md).
- [Full decisions, alternatives, and parked questions](../../givn/ideation/duty-plan-calendar-conversion/questions.md).
- [Confirmed Handoff, mapping, and promotions](../../givn/ideation/duty-plan-calendar-conversion/handoff.md).
- [Ready seed: Export complete Released Roster calendars](../../givn/ideation/duty-plan-calendar-conversion/changes/export-released-roster-calendar.md).
- [Durable vocabulary](../arc42/12-glossary.md).

The normal proposal flow has **not** started. The seed is input to `/givn-propose`; Handoff creates no proposal, change scaffold, permanent spec leaf, tasks, code, fixtures, or golden calendar.

## Problem and vision

A crew member needs an accurate personal-calendar representation of their published duty plan without manual re-entry. One Jeppesen PDF produces an importable RFC 5545 calendar beside it. An unresolved required activity or failed publication produces explicit failure, not an apparently complete calendar with missing activities.

The first release targets the **Released Roster detail-table family represented by the supplied examples and explicitly validated variations**. It does not promise every Jeppesen export, scanned document, airline configuration, or unseen layout.

The product remains a local Rust CLI: one Windows executable, drag-and-drop or a path, no GUI or runtime installer. Rust stable/edition 2021, the exact pdfplumber pin, Windows GNU cross-build, no native C/C++ PDF extraction dependency, hand-written calendar output, permitted licenses, and public-repository privacy remain binding.

## Stakeholders and review lenses

- **Crew member:** correct dates, durations, distinctions, and useful descriptions with minimal manual work.
- **People identified in the source:** private information stays out of public repository material.
- **Maintainer/release owner:** evidence supports compatibility, calendar conformance, legal distribution, and executable portability.

The confirmed advisory Personas were promoted through Handoff:

- [Crew member exporting their own duty plan](../../givn/personas/crew-member--personal-calendar-owner.md) judges fidelity/usability and owner privacy, not release compliance.
- [Maintainer responsible for a public release](../../givn/personas/project-maintainer--release-owner.md) judges evidence/constraints, not unprovided source semantics.

Stable cores and dated records were retained; topic originals stay historical. Personas are not Gherkin Actors, autonomous approvers, or extra product goals.

## One actor goal and guarantees

**Export a duty-plan calendar (`export-duty-plan-calendar`)** is one changing user-goal-level (`!`) case, with **Crew member** as primary Actor. Its future leaf is `givn/specs/export-duty-plan-calendar/usecase.md`, written through normal proposal/spec/archive work, not Handoff.

Extraction, fragments, dates, formatting, publication, and diagnostics contribute to this complete outcome. They are not separately shipping parser/writer use cases.

### Successful flow

1. Receive one PDF/filename without changing source contents.
2. Identify the supported family, month/year, and detailed activities.
3. Resolve required activities and compatible fragments under the approved rules.
4. Retain distinctions/descriptions, including DH and within-trip identifiers.
5. Construct a complete calendar with snapshot identities and approved timestamp provenance.
6. Publish at an unused name beside the source, preserving occupied paths.
7. Report the actual new path.

### Minimal guarantee

Source and prior calendar/report contents stay unchanged. Unknown required activities, unresolved dates, unmatched/ambiguous fragments, and failed publication are not reported as successful complete conversion. Every failure attempts a new numbered diagnostic naming the source PDF. If it cannot be saved, the primary failure remains explicit and an older report is not claimed as current.

### Success guarantee

The crew member obtains one complete importable calendar at an unused path representing required resolved activities. The approved contract explicitly parks differing optional descriptions; Handoff does not claim handling for that unobserved unresolved case.

## Activity representation

| Activity/assignment | Meaning | Calendar rule |
|---|---|---|
| Operating travel | One complete station-to-station leg. | Printed UTC endpoints; one event per represented leg. |
| Pos DH | Positioning in uniform without usual operating work, still duty time. | Retain interval and title prefix, e.g. `DH 007 ABC-DEF`; no second event for the same leg. |
| FREE / OFF | Distinct printed day-off codes. | All-day on the duty-plan date; source clocks ignored. |
| U | Urlaub/vacation. | All-day; source clocks ignored. |
| FZ | Additional vacation-related day off. | All-day; no age/quota/entitlement validation. |
| TK | Office, development, seminar, or team-information duty. | Own printed UTC interval, not flight-route validation or all-day fallback. |
| DTUSA | Consulate visit for a USA visa; DT means Duty Time. | Own printed UTC interval; no other DT-suffix inference or visa integration. |
| SB | Standby/Stby availability. | Own UTC window; separate daily assignments. |
| SBX | Single-day standby or replacement for a cancelled flight. | Own UTC window; no all-day inference, old-flight time reuse, or extra cancelled-flight event. |
| RE | Reserve availability. | Own UTC window; activation/report lead time is not calculated. |
| EM | Printed non-flight code; no unprovided expansion asserted. | Approved inherited mapping: UTC interval/source station when supplied. |
| Explicit dated Layover | Station-bearing dated row. | Approved inherited mapping: all-day; travel-row duration is descriptive, not an invented interval. |

SB/SBX/RE are accepted expert information requiring validation against a later duty plan. FRA is current example context, **not** a universal default. Overview/statistical occurrences create no dated event.

## Dates and fragments

Local-date rows and UTC clocks can belong to different dates. Parenthesized UTC day numbers remain evidence attached to each endpoint:

- Resolve endpoints independently; unqualified clocks use their own source date context.
- Qualified day numbers match the unique previous/same/next Gregorian date around that local date, carrying month/year; no unique consistent match means failure.
- Never override explicit markers to make an interval positive.
- Narrow travel-only rule: both clocks unqualified in one local-date block, no separate arrival block, and End earlier than Start means the next UTC day.
- Equal clocks do not automatically become 24 hours. Every timed End must follow Start.
- Two local-date blocks can describe one UTC-day flight.
- Report (LT), offsets, accounting values, and revision metadata do not calculate activity instants.
- Page headers/footers do not terminate compatible date/trip/unfinished-leg context. Repeated flight number alone is not a sufficient merge key; orphan fragments are not silently dropped.

Generated examples:

| Facts | Interpretation |
|---|---|
| Local block 12: Start `(13) 00:20`; arrival block 13: End `08:10`. | UTC day 13, 7:50 duration, not an extra day. |
| Block 12: Start `19:15`; block 13: End `(12) 21:05`. | UTC day 12, 1:50 duration. |
| Same block 12: `23:10` to `01:05`, no qualifiers/arrival block. | Selected overnight rule: day 12 to 13, 1:55. |
| Local 1 January 2099: UTC `(31) 23:10`. | 31 December 2098. |
| Local 31 January 2099: UTC `(1) 00:20`. | 1 February 2099. |
| OFF with `23:00` and `01:00`. | Still all-day, not an overnight timed interval. |

These specify desired behavior, not proof that real boundary/standby rows were supplied or parsed by the Rust library.

## Descriptions, identity, and metadata

Approved DESCRIPTION fields/order: **Trip ID, Report (LT), A/C, Blocktime, Timezone Difference, Layover duration, Tags**, only where supplied/associated. Known Trip ID carries within an unambiguous travel trip, including blank cells; it does not leak to unrelated duties/new trips. Other fields do not indiscriminately inherit.

Compatible fragments combine complementary fields and coalesce equal repeats. Differing optional values are a parked unobserved case, not permission to pick a preferred fragment.

UIDs are scoped to the **source snapshot and distinct activity**. Identical-snapshot conversion is deterministic independent of output counters; revised snapshots have separate identities even for unchanged activities. Global uniqueness still needs an implementation; titles/filenames are not identity. Different imported snapshots may coexist, without client update/deduplication promises.

Prefer **Created UTC footer** revision evidence for this family. If unusable/unavailable, use actual constructed-representation UTC revision time. Do not use a month-first constant or this metadata to date activities.

## Preservation and failures

For generated `roster.pdf`, use a free basename or numbered names such as `roster.ics`, `roster-1.ics`, `roster-2.ics`. Occupied candidates, including concurrent claims, are never overwritten. Report the actual path; older versions are not deleted.

Reports similarly preserve `roster.parse-error.txt` and choose names such as `roster.parse-error-1.txt`, identifying the PDF/current reason. Interpretation and publication failures both attempt a report. Its failure does not replace the primary error or create success.

Writable storage, power-loss recovery, automatic cleanup/backup, and imported-calendar updates are not invented guarantees.

## Research and evidence limits

The project initially had no permanent specs, changes, or Personas; durable documents were templates. Five private PDFs were inspected with permission. They showed repeated detailed headers, sparse fields, page continuations, UTC qualifiers, spanning layover text, and accounting columns distinct from clocks. No private filenames/itineraries or identifying values enter this public record.

Read-only documentation verified pdfplumber 0.1.0 exists, declares MIT OR Apache-2.0, and exposes positioned extraction/table APIs. The draft's method snippets differ. API/declared-license evidence is not source coverage, resolved-license compliance, Windows GNU compilation, or DLL/runtime proof. The exact pin was unchanged.

The original synthetic evidence needs correction before implementation relies on it:

- Five unfurled lines exceed the required 75 octets.
- 1 January 2099 is Thursday, not the provided Saturday.
- Claimed standby coverage has no standby row.
- Overnight Blocktime equals elapsed time, so the claimed independence regression is ineffective; zero-Blocktime DH provides another case.
- Some oracle timezone/tag values lack matching source fields.
- Normalization erases line-ending distinctions and cannot prove CRLF.
- The unanchored privacy regex flags generated calendar timestamps; binary-skipping grep does not prove a PDF is PII-free.

The user draft was unedited. No code, test, fixture, golden, private-source Rust extraction, compile, cross-build, or release gate was run during ideation.

## Parked questions and later validation

The [ready seed](../../givn/ideation/duty-plan-calendar-conversion/changes/export-released-roster-calendar.md#open-questions) retains standalone questions/alternatives:

- **Optional descriptions disagree:** hypothetical Report 07:30 versus 08:00 has not been observed; reject or preserve labelled values is unselected. Ordinary complementary/equal fragments are supported.
- **Source-derived publication rights:** private access/anonymization is not a license; independently generated tests remain available.
- **Formal FZ provenance:** acronym/agreement/entitlement origin stays unverified and is not needed to represent an assigned FZ day.

Before release, validate actual source coverage, standby/boundary rules, calendar bytes/metadata, no-clobber behavior, licenses/privacy, and Windows distribution. Real-source tests remain local/uncommitted; public validators and fixtures receive generated/anonymized material only.

## Handoff and next workflow

One seed was published: **Export complete Released Roster calendars (`export-released-roster-calendar`)**, `Status: ready`, first/only suggested start, no prerequisite change. It covers the complete actor guarantee rather than independently shipping parser/writer fragments.

Both approved Personas and thirteen confirmed terms were promoted. Candidate introductions Duty plan, Flight leg, Calendar export remain unpromoted; no rejected record was deleted.

The next separate action is normal proposal planning using that seed. Handoff starts no change and writes no permanent use-case/spec delta or implementation.
