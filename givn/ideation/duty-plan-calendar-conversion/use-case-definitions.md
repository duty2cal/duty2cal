# Duty-plan calendar conversion — use-case definition

Status: the one actor-goal cut and topic contract were approved on 2026-10-11; Handoff publication and named promotions were explicitly confirmed on 2026-10-12. The optional-conflict question stays parked. No normal change or permanent spec leaf is started by this publication.

## Actor-goal cut

| Name and stable use-case ID | Level | Primary Actor | Goal | Boundary | Mutation |
|---|---|---|---|---|---|
| Export a duty-plan calendar (`export-duty-plan-calendar`) | ! | Crew member | Obtain an importable calendar representation of one supplied duty plan. | Duty-plan calendar: the confirmed whole-plan consistency boundary. | Changing: create a calendar export at an unused name and expose its outcome, preserving existing outputs. |

The identifier `export-duty-plan-calendar` must survive the topic's elaborated use case, any change seed, the change-side contract, and the permanent leaf document. No permanent leaf is written during ideation.

## Bounded outcome

- **Trigger:** the crew member supplies one duty-plan source for calendar conversion.
- **Success guarantee:** the crew member obtains a complete importable calendar file representing the required resolved activities of one duty plan in the selected Released Roster detail-table family and its validated variations. The unobserved optional-description disagreement remains parked under its question (Q28), not assigned an implicit merge policy.
- **Minimal guarantee:** the converter leaves existing calendar files unchanged, exposes rejection or publication failure independently of diagnostic persistence, does not claim success for either, and does not silently omit a required activity from a successful export.
- **Calendar publication:** use an unused output name beside the PDF. If the normal basename is taken, add an enumerated suffix before `.ics`; draft examples use `roster.ics`, `roster-1.ics`, `roster-2.ics`. An occupied candidate must never be overwritten, including under concurrent conversion attempts; verification belongs in the later design/implementation flow.
- **Failure-output rule:** every conversion failure, including publication failure, attempts a new numbered diagnostic preserving prior reports and identifying the source PDF filename. Diagnostic inability remains an explicit failure outcome, not proof that a new report exists even if older ones do. Existing calendars and reports remain unchanged.
- **Closure test:** after a complete calendar file is published, the crew member can stop; importing it into a calendar application is another goal outside this converter's Frame.

## Inside this one goal

- Identify the required activity representations from the supplied duty plan.
- Resolve complete timed activities, including explicit UTC day qualifiers and fragments crossing page/date blocks.
- Represent non-flight activities using their eventual confirmed mappings; do not invent meanings for unfamiliar codes.
- Preserve the selected descriptive fields according to later confirmed inheritance/conflict rules.
- Publish the resulting calendar or expose failure without claiming success.

These are responsibilities within the goal, not independently valuable user outcomes. No component-shaped use cases for extraction, parsing, time assembly, or calendar writing are minted.

## Failure extensions rather than new goals

- Unsupported or unreadable source: conversion cannot reach the requested success outcome.
- Unknown activity or unresolved date: the completeness invariant prevents apparent success.
- Unmatched flight fragment: do not publish a calendar that silently excludes the leg.
- Publication failure: completion is not success; expose the failure even if diagnostic persistence is unavailable.

These protect the same goal's guarantees. Numbered main-flow branches will be written during elaboration, not invented here as a second error-handling user goal.

## Excluded actor goals

- Import the resulting events into a calendar application.
- Reconcile a calendar automatically with revised duty plans.
- Edit or authorize changes to the official duty plan.
- Compare two duty-plan revisions as a separate user-facing outcome.
- Publish or approve a converter release: an external project-delivery goal reviewed by the maintainer Persona, not a product command in the conversion tool.

No new summary-level use case is needed: the proposed conversion goal does not contain three independent actor goals or a conjunction of success outcomes. Subfunctions remain inside the leaf's flow.

## Stakeholders and Persona value reactions

- **Crew member exporting their own duty plan (crew-member--personal-calendar-owner):** the end-to-end calendar outcome is the valuable stopping point. Correct isolated parsing would not satisfy that goal if no calendar is published. Accuracy and visible failure are prerequisites, not optional later polish.
- **People identified in the PDF:** the confirmed owner Persona covers the owner's privacy interest; this does not authorize reuse of identifying source data in public fixtures or expected calendars.
- **Maintainer responsible for a public release (project-maintainer--release-owner):** the goal cut correctly keeps release verification as a quality/evidence obligation rather than adding a component or release-command goal to the converter. Compatibility must be explicit before the contract is claimed ready.

Value rank: the sole actor goal is core to the Frame. This is not an implementation order, a change count, a seed cut, or a release approval.

## Sizing checks

1. One primary Actor and one observable success outcome: an importable representation of one supplied duty plan.
2. Goal-shaped: no use-case name refers to a parser, writer, CLI, backend, or infrastructure component.
3. Level: `!`; no `+` summary owns direct capabilities, and no internal `-` subfunction becomes a permanent directory.
4. Mutation: successful export changes output state, so this is not labeled read-only.
5. Extension discipline: each listed failure prevents or reports the same requested outcome, rather than introducing a second goal.
6. Preconditions are not invented here; eligibility and recognition belong in later source-validation flow unless they genuinely hold before the trigger.
7. Technology independence: the Actor goal, trigger, and guarantee remain true if implementation language or extraction machinery changes; inherited technology constraints stay separate.
8. Relationships: no Includes or Extends relationship is needed for the one derived leaf; no relationship cycle exists.

## Quality review

The fresh-context reviewer found no blocking sizing, boundary, value, Actor, mutation, extension, or relationship defects on 2026-10-10. Applied its minor consistency corrections: the standalone minimal guarantee explicitly exposes failure independently of diagnostic persistence, and the Frame's diagnostic wording is qualified by writability. Both follow the already confirmed explicit-failure requirement and do not select recovery or overwrite behavior. The compatibility boundary was subsequently explicitly selected by the user on 2026-10-11; no complete use-case contract, behavior mapping, or Handoff has been silently approved.

## Elaboration progress — rules before the complete contract

- Confirmed compatibility rule: the first release targets the supplied Released Roster detail-table family and explicitly validated variations, not every Jeppesen export.
- Confirmed completeness rule: no required source activity is silently omitted from a successful export.
- Confirmed vacation rule: U means Urlaub/vacation. FREE, OFF, and U are all-day activities with source times ignored; no further confirmation of that mapping is needed.
- All-day classification for FREE, OFF, U, and FZ is driven by the activity code, not by `Start == 00:00`, source time presence, or a calculated time interval. Ignored source times do not cause midnight repair or timed-interval validation for these activities.
- Confirmed FZ meaning: additional days off associated with vacation. The formal acronym and entitlement source remain unverified and non-blocking; no age or annual-quota check is part of conversion.
- Confirmed FZ mapping: all-day on its duty-plan date, source times ignored, distinct source code retained. The user accepted this through the FZ representation decision (Q14); no duplicate confirmation is needed.
- Confirmed TK meaning: office attendance or professional-development/seminar duty, including team information sharing. It is a known non-flight activity rather than a guessed flight or layover.
- Confirmed TK timing: preserve the printed UTC Start/End interval as a timed non-flight activity. Required endpoints must resolve; there is no invented all-day fallback.
- Confirmed DTUSA meaning: DT expands to Duty Time; DTUSA is a non-flight consulate visit to obtain a USA visa. This does not approve other DT-prefixed codes or use duty-accounting durations to construct its event interval.
- Confirmed DTUSA timing: preserve the printed UTC Start/End interval as timed non-flight duty with resolvable endpoints and no invented all-day fallback. Its meaning and timing are settled through the DTUSA-meaning decision (Q17) and consulate-duty timing decision (Q18).
- Confirmed DH meaning: Dead Head positioning to a departure destination, still duty time in uniform without usual operating work. Pos DH is an assignment on the underlying travel, not a new absence activity or a second event for the same leg.
- The underlying travel interval is not erased by zero Blocktime or missing optional aircraft information. Preserve the printed UTC travel endpoints and keep accounting durations separate; no operating-work calculation is introduced.
- Confirmed deadhead visibility: DH prefixes the event title, retaining its printed designator and route, for example `DH 007 ABC-DEF`. One underlying travel leg produces one timed event, not a second separate deadhead event.
- Confirmed unqualified-overnight rule: for travel with unqualified UTC endpoints in one local-date block, no separate arrival-date block, and End clock earlier than Start, interpret End as the following UTC day. Explicit day qualifiers are not overridden, equal clocks are not repaired, and this does not invent a ground-duty fallback or derive dates from Blocktime.
- Confirmed output preservation: existing calendars are never replaced; collisions select unused numbered calendar names. The actual newly published path is reported. Choosing a filename does not choose event identifiers or reimport reconciliation.
- Confirmed report preservation: diagnostics use unused numbered names rather than replace old reports, include the source PDF filename, and report the actual new path. This is local diagnostic content; actual private filenames are not copied into public topic state or fixtures.
- Confirmed failure-report coverage: every conversion failure attempts a report, including failed calendar publication; if it cannot be saved, the original failure remains explicit.
- Confirmed component identity: use separate source-snapshot identities. Re-exporting an identical snapshot is deterministic and independent of output filename counters; a revised snapshot has separate identities, including unchanged activities. Distinct components must still have globally unique identifiers; no cross-revision matching or client import-update guarantee is added.
- Confirmed revision metadata: prefer the source Created UTC footer under the user's selected-family convention; when usable source revision metadata is unavailable, use the constructed representation's actual UTC timestamp. This is metadata, not an activity-date input or an arbitrary month-first test constant.
- Confirmed SB/RE rule: standby and reserve have start/end availability windows and produce timed events, preserving each dated interval and FREE interruptions. Their activation-to-reporting notice difference is outside conversion.
- Confirmed SBX rule: single-day/replacement standby supplies its own UTC availability window and is timed, like SB/RE, not all-day or a copy of the cancelled flight's times. FRA is the example context rather than a global default.
- The user authorizes adding expert-provided SB/SBX/RE semantics now while requiring later real-duty-plan validation. This closes the timing question without manufacturing extraction evidence.
- Confirmed date interpretation: qualified UTC cells map to the unique previous/same/next Gregorian date around their own local-date block, including month/year transitions; unqualified endpoints use their source date context before the selected narrow travel rollover. No unique/consistent resolution means failure, not an arbitrary repair.
- Unobserved optional description disagreements remain parked under their question (Q28), not a demonstrated corpus defect. Real-source standby/boundary validation and technical release evidence remain obligations rather than unresolved preferences.
- No timing or all-day rule has been invented for unresolved codes, and no complete use-case contract or scenario skeleton has been written ahead of rule elaboration.

## Generated examples for the confirmed all-day rule

These examples contain no private source values. They specify observable event kind/date, not unconfirmed display labels or metadata.

| Duty-plan date | Activity code | Example source time values | Expected calendar representation |
|---|---|---|---|
| 2099-01-12 | U | no values | All-day vacation on the duty-plan date; no event clock times. |
| 2099-01-13 | FREE | 09:00 / 17:00 | All-day FREE activity on the duty-plan date; source times ignored. |
| 2099-01-14 | OFF | 23:00 / 01:00 | All-day OFF activity on the duty-plan date; no overnight timed interval derived. |
| 2099-01-15 | FZ | 09:00 / 17:00 | All-day additional vacation-related day off on the duty-plan date; source times ignored. |

These are examples of the explicitly supplied user rule, not fixture files, a golden calendar, code, or a new behavior decision.

## Generated example for the confirmed TK timing rule

Given a generated TK office/seminar activity whose UTC endpoint dates have already been resolved to 16 January 2099, printed Start `08:00` and End `12:00` produce a four-hour timed event from 08:00 to 12:00 UTC. The example does not select a date-resolution algorithm or an output label. The absence of a flight destination does not turn this known office/seminar activity into a malformed flight.

## Generated example for the confirmed DTUSA timing rule

Given a generated DTUSA consulate duty whose UTC endpoint dates have already been resolved to 17 January 2099, printed Start `09:00` and End `15:00` produce a six-hour timed event from 09:00 to 15:00 UTC. Duty-accounting durations do not replace those printed endpoints. The example does not choose date-boundary handling, a SUMMARY label, or a visa-processing capability.

## Generated example for the confirmed deadhead meaning

A generated travel row with Pos DH, designator `007`, route ABC to DEF, already-resolved UTC endpoints 06:00 and 07:00, blank aircraft, and Blocktime `00:00` remains one hour of positioning duty, titled `DH 007 ABC-DEF`. It must not disappear or become an all-day activity because the accounting Blocktime is zero. The title does not select a UID or reimport-update policy.

## Generated example for the confirmed unqualified overnight rule

A generated travel row in local-date block 12 has unqualified Start `23:10` and End `01:05`, with no separate arrival-date block. Under the user's selected interpretation it becomes UTC day 12 at 23:10 to UTC day 13 at 01:05, lasting 1:55. A row with explicit day markers must instead retain those markers; this example does not authorize replacing contradictory explicit dates or calculating duration from Blocktime.

## Generated example for confirmed no-overwrite naming

For a generated source `roster.pdf`, retain an existing `roster.ics` unchanged and publish the next complete calendar under an unused numbered name, illustrated as `roster-1.ics`. If that candidate is occupied too, select another unused candidate, illustrated as `roster-2.ics`; report the actual selected path. No old calendar is deleted or replaced, and a simultaneous claimant must not cause an occupied path to be overwritten. Hyphen punctuation is the draft notation, not a separately requested naming preference.

## Metadata elaboration in progress

The user selected Trip ID inheritance through the trip-description decision (Q23): each travel leg in the same unambiguous roster trip retains the known identifier, even when its source cell is blank or it continues on another page/date. This does not change event instants or authorize report-time/offset inheritance, unrelated-duty propagation, or new layover metadata.

Generated example: the first travel leg prints Trip T42 and the next leg in the same trip has a blank Trip ID cell; both descriptions include `Trip T42`. A later trip explicitly identified as T43 uses T43, not stale T42. Source examples must include blank continuation cells so this rule cannot pass merely because every synthetic row repeats the field.

Failure-report naming is settled through the failure-report decision (Q22): preserve old reports under numbered names and include the attempted source PDF filename. The user also selected a report attempt for every conversion failure through the publication-failure decision (Q24).

## Generated example for confirmed diagnostic preservation

For generated source `roster.pdf`, keep `roster.parse-error.txt` untouched and write a new report under an unused name, illustrated as `roster.parse-error-1.txt`. The new report contains `Source PDF: roster.pdf` and the current failure reason; the converter reports that actual new path. If the new diagnostic cannot be persisted, the outcome must not claim the older report was updated or a new one was written. No private source filename is used in this example.

## Full contract draft

The stable-schema elaboration now exists at `use-cases/export-duty-plan-calendar.md`, with numbered Actor/System flow, branch-specific extensions, guarantees, generated examples, Persona links, and explicit open prerequisites. Its fresh-context review and correction check found no remaining blocking draft defects; it remains draft pending the listed readiness prerequisites and human approval. It starts no change and writes no executable specifications or code.

The first fresh-context elaboration review identified unqualified non-trip description association/merge language. Corrected it by listing the inherited fields explicitly, separating required contradictions from optional disagreements, recording the descriptive-fragment question (Q28), and adding a generated differing-Report example. This is a clarification of unresolved work, not a new metadata policy selected through review.

The same reviewer verified those corrections. The corrected current failure canvas and actor-goal diagram rendered successfully; report attempts after all failures are unconditional, while report-write success is not. Draft quality is not human sign-off, release readiness, or permission to publish/promote/start a change.

The user then adopted the complete contract through the complete-contract decision (Q31), including inherited EM/dated-Layover mappings and non-trip DESCRIPTION selection. Handoff preparation now proposes one whole-deliverable seed and the stable permanent-leaf mapping; publication, promotions, code, and change start still require their own stated authority.
