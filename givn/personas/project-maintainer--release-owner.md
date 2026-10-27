# Persona: project-maintainer--release-owner

Name: Maintainer responsible for a public release

Status: confirmed by the user on 2026-10-10; promoted to the project collection by confirmed Handoff on 2026-10-12.

Origin topic: Duty-plan calendar conversion (duty-plan-calendar-conversion). Relative artifact references in the running record refer to givn/ideation/duty-plan-calendar-conversion/.

## Role

Maintains the converter and decides whether it is ready for a public release.

## Authority boundary

Accepts or rejects a release against the agreed constraints. Does not change a crew member's official duty plan, relax confirmed constraints, or invent meanings for unexplained activity codes.

## Evaluation capability

Evaluates documented support boundaries, test and calendar-validation evidence, dependency licenses, Windows distribution checks, and the absence of private source material from public artifacts. Domain experts remain necessary for unexplained roster semantics.

## Own vocabulary

Supported export, generated fixture, golden calendar, validation evidence, dependency license, release artifact, and private-data exposure.

## Interests

- A maintainable, verifiable, legally distributable converter.
- Public support claims that match their evidence.
- Public artifacts free of private source material.

## Goals

Publish a release that satisfies the agreed distribution, calendar-compliance, licensing, and privacy requirements with evidence for its claimed duty-plan support.

## Success criterion

A release satisfies the agreed distribution, calendar-compliance, licensing, and privacy requirements, with claimed duty-plan support backed by evidence rather than one passing example.

## Topic stances

### 2026-10-10 — Confirmation for duty-plan calendar conversion

- Topic: duty-plan-calendar-conversion.
- User confirmed this Persona through the maintainer review-lens decision (Q8), which authorizes a topic-local release-conformance review lens only.
- Motivating example: a golden-calendar match alone does not justify the claim “supports every valid Jeppesen duty-plan PDF”; documented evidence must support the agreed scope.
- Authority limit: no project-level promotion, technology substitution, implementation, or change start.

### 2026-10-11 — Confirmed first-release topic stance

- The user selected the supplied Released Roster detail-table family and explicitly validated variations through the supported-export decision (Q4), which narrows the first-release promise.
- Release evidence must demonstrate this selected scope, not claim universal Jeppesen support or require additional export families without a new scope decision.
- This dated stance does not change the Persona's stable authority or evaluation capability.

### 2026-10-11 — Confirmed vacation and all-day topic stance

- The user explicitly settled U/vacation and the FREE/OFF/U all-day mapping through the vacation/all-day decision (Q11).
- Generated evidence must include missing, nonzero, and apparently overnight source times to show that all-day classification is code-driven rather than conditional on a zero time.
- This is a derived verification obligation on the confirmed rule, not authorization to write tests during ideation or change the stable Persona core.

### 2026-10-11 — Confirmed FZ mapping stance

- The user accepted all-day FZ representation with source times ignored through the FZ representation decision (Q14).
- The generated all-day evidence set must now include FZ; the unverified formal acronym and agreement source are not required to validate this output rule.
- No fixture, golden file, code, or eligibility check is authorized during ideation by this stance.

### 2026-10-11 — Confirmed TK meaning and timing stance

- TK's user-confirmed meaning and printed-UTC-interval mapping are recorded through the TK-meaning decision (Q15) and office/seminar timing decision (Q16).
- Later generated evidence must distinguish timed non-flight attendance from both all-day absence and a flight that requires a route. Invalid endpoints must not produce an invented all-day fallback.
- This is a verification consequence of the confirmed rule, not permission to write tests or implementation during ideation.

### 2026-10-11 — Confirmed DTUSA meaning and timing stance

- The user's DTUSA meaning and printed-UTC-interval rule are recorded through the DTUSA-meaning decision (Q17) and consulate-duty timing decision (Q18).
- Later generated evidence must distinguish a timed visa duty from flight-route validation and duty-accounting duration fields; no generic DT-suffix fallback is justified.
- This does not authorize implementation, visa integration, or a change to the Persona's stable core.

### 2026-10-11 — Confirmed deadhead meaning and visibility stance

- The user's deadhead definition and title-prefix choice are recorded through the DH-assignment decision (Q19) and deadhead-visibility decision (Q20).
- Later generated evidence must retain zero-Blocktime travel, optional missing aircraft, leading-zero designators, and the visible DH distinction without emitting a second event for the same leg.
- Title placement does not settle event identity or imply that changing a title safely updates an imported calendar; those guarantees remain separate.

### 2026-10-11 — Confirmed unqualified overnight stance

- The user's narrow next-day End interpretation is recorded through the unqualified-overnight decision (Q21).
- Later generated evidence must distinguish unqualified same-block rollover from explicit day markers and separate arrival blocks. The desired rule alone does not prove a layout variation exists in the selected source family.
- The read-only Rust extraction documentation findings establish API availability for the pin, not a successful source extraction, GNU cross-build, static distribution, or resolved-license gate.

### 2026-10-11 — Confirmed no-overwrite numbered-calendar stance

- The user selected preservation plus automatic numbered output through the calendar-preservation decision (Q10), not replacement or a manual name-collision failure.
- Later publication evidence must prove an occupied candidate cannot be clobbered, including concurrent claims; only complete new calendars are reported as successful exports.
- This does not establish an implementation strategy, file atomicity/power-loss durability, diagnostic overwrite authority, or reimport compatibility.

### 2026-10-11 — Confirmed travel-trip description stance

- The within-trip Trip ID rule is user-confirmed through the trip-description decision (Q23).
- Later generated evidence must include blank cells, page/date continuations, and new/unrelated groups so a naive unbounded forward fill cannot pass as correct inheritance.
- This confirms descriptive context, not an additional aggregate, UTC calculation rule, or report-time/offset inheritance policy.

### 2026-10-11 — Confirmed numbered diagnostics stance

- Diagnostic preservation/numbering and source-filename identification are user-confirmed through the failure-report decision (Q22).
- Later output evidence must distinguish the newly saved diagnostic from retained older files and protect occupied report names under concurrent claims.
- Actual filenames can identify a person; local report permission does not authorize copying private names or reports into public repository artifacts. Publication-failure report scope remains separate.

### 2026-10-11 — Confirmed report attempts after every failure

- The user selected all-failure diagnostic attempts through the publication-failure decision (Q24).
- Later evidence must cover publication failure plus unavailable diagnostic storage without claiming success or losing the primary error.
- RFC UID/DTSTAMP and synthetic-oracle findings are evidence obligations for the full contract/design, not authorization to change code, fixtures, or dependencies during ideation.

### 2026-10-11 — Confirmed source-snapshot identity stance

- Snapshot identity is user-confirmed through the calendar-identity decision (Q25), superseding any assumption of stable corresponding-activity IDs across revised PDFs.
- Later evidence must show deterministic identical-snapshot IDs, distinct components within one snapshot, global source scoping, and independence from output counters/title rendering.
- The chosen identity model does not itself prove any calendar client's reimport/update behavior or anonymize real input/output data.

### 2026-10-11 — Confirmed revision-timestamp provenance stance

- The preferred Created UTC footer plus construction-time fallback is user-confirmed through the timestamp-provenance decision (Q27).
- Later evidence must cover source metadata parsing/association and unavailable-source fallback with a controlled clock, without treating metadata as event-date evidence or normalizing away calendar byte properties.
- No arbitrary roster-month-first timestamp is retained as a production shortcut; the exact extraction/fallback mechanics remain design work, not code written during ideation.

### 2026-10-11 — Confirmed standby/reserve definition stance

- User domain information establishes timed SB/RE and SBX's meaning through the standby/reserve definition decision (Q26), superseding the initial all-day/zero-time shortcut for SB.
- Later evidence must locate the actual UTC availability columns and distinguish them from statistics/accounting values; no real standby row was added to the private corpus by this answer.
- SBX timing remains explicit in its focused question; no activation engine, notice calculator, or new stable Persona is justified by the background difference.

### 2026-10-11 — Confirmed SBX UTC window and validation stance

- SBX UTC window semantics and expert-evidence status are user-confirmed through the single-day-window decision (Q29).
- Later source validation must demonstrate the actual dated rows, UTC columns, and station association; FRA in the current examples is not a parser constant.
- Acceptance of the business rule is not a successful extraction/cross-build or permission to write code/fixtures during ideation.

### 2026-10-11 — Confirmed adjacent UTC boundary stance

- Adjacent Gregorian date resolution is user-confirmed through the boundary-date decision (Q30); source-date interpretation is no longer a pending preference.
- Later generated/source evidence must include month/year transitions and a marker with no unique adjacent match, and must preserve failure for contradictory required dates rather than extend the narrow unqualified rollover.
- No real-source boundary fixture, compile, or code is produced during ideation by recording this obligation.

### 2026-10-11 — Complete contract adoption

- The user adopted the complete contract through the complete-contract decision (Q31), including inherited EM/dated-Layover and DESCRIPTION selection, while preserving source/build/license/privacy/calendar evidence gates.
- The readable Handoff may propose a whole-deliverable seed; it must not treat contract approval as implemented release readiness or silently promote this review lens.
- Project-level promotion remains a separate Handoff choice; the stable Persona core is unchanged.

### 2026-10-25 — Generalized duty-plan input stance

- The support statement is release evidence, not marketing copy: a publication
  is named only when its fixture converts in the checks, and the statement
  shrinks when that evidence fails.
- The project names no formats, operators or families. What a reader can rely
  on is what a duty plan must state plus the publications whose evidence is
  exercised.
- Reused for `generalized-duty-plan-input` (Q3).

## Review record

### 2026-10-10 — Event Storming reaction

- Artifact: the conversion event flow in `eventstorming.md`, including kinds and separate publication outcomes.
- Advisory finding: a consistent domain flow is not evidence of universal PDF support, standards validation, portable distribution, or verified filesystem recovery.
- Example: a Calendar export published event says a file was produced; it does not certify every import client or every Jeppesen format.
- Disposition: retain these as evidence obligations for later elaboration and release review. No new scope, technology substitution, or release approval is granted.

### 2026-10-10 — Use Case Definition value reaction

- Artifact: the proposed end-to-end goal in `use-case-definitions.md`.
- Advisory finding: release verification is an evidence obligation on the converter, not a new product command or component-shaped use case.
- Example: one supported table family passing generated examples cannot justify a universal-format release promise.
- Disposition: keep the first-release supported-export question (Q4), which governs compatibility breadth, open for the user rather than selecting a scope through this review.

### 2026-10-11 — Full contract elaboration reaction

- Artifact: the draft end-to-end contract in `use-cases/export-duty-plan-calendar.md` and read-only RFC/oracle findings in `frame.md`.
- Advisory findings: a correct rule set still needs extraction, source-coverage, Windows distribution, license, privacy, and calendar-byte evidence. The existing synthetic golden has five overlong lines, and the PII regex mistakes a generated timestamp for a staff-shaped identifier.
- Generated example: a conforming folded description cannot match an unfurled verbatim oracle merely by removing DTSTAMP; normalization must not erase the CRLF property it is supposed to prove.
- Disposition: retain the findings as later test/design obligations, not permission to edit the user's draft or write code/fixtures during ideation. Open identity/timestamp/standby/date rules prevent treating the draft as signed off.

### 2026-10-25 — Review for generalized duty-plan input

- Artifacts: the Frame, the event chain and the three use cases of
  `generalized-duty-plan-input`.
- Advisory finding: the statement's two halves must be checked separately — the
  required facts against the confirmed conversion rules, the publication list
  against exercised fixtures. A statement that states neither is refused, and
  so is one that describes future support.
- Advisory finding: the four carriers (README, organization profile, chapter 3,
  issue form) must agree; the profile is published through the declared
  configuration, so a statement change is a configuration change there.
- Disposition: advisory only; the user arbitrates.

## Promotion record

### 2026-10-12 — Confirmed ideation Handoff

- The user confirmed project-level promotion through the Handoff decision (Q32).
- The stable core and validated dated topic/review records were copied unchanged from the origin topic; the topic file remains historical.
- Promotion grants a reusable advisory release-evidence lens only, not a product Actor, constraint override, or release approval.
