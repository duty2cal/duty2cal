# Persona: crew-member--personal-calendar-owner

Name: Crew member exporting their own duty plan

Status: confirmed by the user on 2026-10-10; promoted to the project collection by confirmed Handoff on 2026-10-12.

Origin topic: Duty-plan calendar conversion (duty-plan-calendar-conversion). Relative artifact references in the running record refer to givn/ideation/duty-plan-calendar-conversion/.

## Role

A crew member turning their published duty plan into their personal calendar.

## Authority boundary

Supplies their duty plan and decides whether its calendar representation is usable. Does not edit the official schedule or decide release compliance.

## Evaluation capability

Compares activity dates, flight legs, layovers, and report information with their own plan. Cannot certify PDF extraction APIs or RFC compliance.

## Own vocabulary

Duty plan, flight leg, report time, deadhead, layover, and activity codes as printed. Unexplained codes remain unexplained; this Persona does not establish their meanings by assumption.

## Interests

- Faithful activity representation without invented dates or omitted activities.
- Clear conversion failures rather than a misleading calendar.
- No exposure of private source information in public project material.

## Goals

Obtain a usable personal-calendar representation of the required activities in their published duty plan without manually re-entering them.

## Success criterion

The calendar faithfully represents the required activities and distinguishes operating flights, positioning travel, and non-flight activities without invented dates or omissions. The details of those distinctions remain for use-case elaboration; Persona confirmation alone does not approve calendar mappings.

## Topic stances

### 2026-10-10 — Confirmation for duty-plan calendar conversion

- Topic: duty-plan-calendar-conversion.
- User confirmed the displayed role, authority, evaluation capability, vocabulary, and success criterion through the crew-member review-lens decision (Q6), which authorizes this topic-local Persona and advisory reviews only.
- Generated motivating example: a departure shown as `(13) 00:20` under local day 12 must not appear one day early in the personal calendar.
- Coverage: the converter operator's accuracy interests and the identified plan owner's privacy interests. No separate biography was invented for the same person.
- Authority limit: no project-level promotion, Gherkin actor approval, mapping-rule approval, or implementation authorization.

### 2026-10-11 — Confirmed first-release topic stance

- The user selected the supplied Released Roster detail-table family and explicitly validated variations through the supported-export decision (Q4), which narrows the first-release promise.
- Accurate treatment of required activities in that family remains the goal; code meanings and calendar mappings are not settled by choosing the family.
- This dated stance does not change the Persona's stable authority or evaluation capability.

### 2026-10-11 — Confirmed vacation and all-day topic stance

- The user defined U as Urlaub/vacation and made FREE, OFF, and U all-day with source times ignored through the vacation/all-day decision (Q11).
- Generated example: an OFF row with 23:00 and 01:00 is still an all-day activity, not an invented overnight interval.
- Other activity meanings, metadata, and display-label choices remain unresolved; the stable Persona core is unchanged.

### 2026-10-11 — Confirmed FZ mapping stance

- The user accepted all-day FZ representation on its duty-plan date with source times ignored through the FZ representation decision (Q14).
- The FZ source code remains distinct from U/FREE/OFF; display wording and metadata await separate rules.
- This records the user's mapping, not entitlement validation or a stable-core change.

### 2026-10-11 — Confirmed TK meaning and timing stance

- The user defined TK as office/seminar attendance through the TK-meaning decision (Q15) and selected its printed UTC Start/End interval through the office/seminar timing decision (Q16).
- Generated example: with already-resolved UTC dates, 08:00–12:00 is a four-hour office/seminar event, not a whole-day marker or an incomplete flight.
- Display wording, metadata, and unresolved date-boundary handling remain separate; the stable Persona core is unchanged.

### 2026-10-11 — Confirmed DTUSA meaning and timing stance

- The user defined DTUSA as consulate visa duty (DT = Duty Time) through the DTUSA-meaning decision (Q17) and selected its printed UTC interval through the consulate-duty timing decision (Q18).
- Generated example: with already-resolved dates, 09:00–15:00 UTC is a six-hour non-flight duty, not a whole-day absence or a visa application performed by the converter.
- Labels, metadata, and unresolved date-boundary rules remain separate; the stable Persona core is unchanged.

### 2026-10-11 — Confirmed deadhead meaning and visibility stance

- The user defined DH positioning duty through the DH-assignment decision (Q19) and selected a DH title prefix through the deadhead-visibility decision (Q20).
- Generated example: `DH 007 ABC-DEF` distinguishes the assignment in a compact calendar view while retaining one travel event and its printed interval.
- This does not choose identifiers, reimport updates, or pay/accounting behavior; the stable Persona core is unchanged.

### 2026-10-11 — Confirmed unqualified overnight stance

- The user selected next-UTC-day End interpretation through the unqualified-overnight decision (Q21), only for travel with unqualified endpoints in one local-date block and no separate arrival-date block.
- Generated example: 23:10 to 01:05 becomes a 1:55 overnight event. Explicit day markers remain authoritative; a zero-length interval is not turned into a 24-hour event.
- This does not resolve month/year markers or prove a source-layout variation was observed; the stable Persona core is unchanged.

### 2026-10-11 — Confirmed no-overwrite numbered-calendar stance

- The user supplied the no-overwrite/enumerated-name variant through the calendar-preservation decision (Q10).
- Generated example: retain `roster.ics` and publish a complete new calendar under an unused numbered name such as `roster-1.ics`; report that new path.
- Old calendars are not removed; choosing which version to import remains the user's action. No duplicate-free reimport or error-report replacement is promised.

### 2026-10-11 — Confirmed travel-trip description stance

- The user selected within-trip Trip ID inheritance through the trip-description decision (Q23).
- Generated example: legs 007 and 008 of roster trip T42 both retain `Trip T42` in their descriptions despite a blank continuation cell.
- No report-time or station-offset propagation is inferred; trip context must not leak into unrelated duties or a later trip.

### 2026-10-11 — Confirmed numbered diagnostics stance

- The user selected preserving numbered reports and approved the source PDF filename as identifying content through the failure-report decision (Q22).
- Generated example: `roster.parse-error-1.txt` identifies `roster.pdf` while the prior report stays untouched; an unavailable new write does not make the older report current.
- This permits local diagnosis, not publication of actual private source names/data or deletion of older outputs.

### 2026-10-11 — Confirmed report attempts after every failure

- The user selected a report attempt after every conversion failure through the publication-failure decision (Q24), not only interpretation failures.
- Generated example: failed calendar publication still attempts an unused numbered report identifying roster.pdf, while its visible failure remains independent of saving that report.
- Existing outputs remain protected; old reports are not represented as current writes.

### 2026-10-11 — Confirmed source-snapshot identity stance

- The user selected separate source-snapshot component identities through the calendar-identity decision (Q25).
- Generated example: a revised PDF changing flight 007 from 09:00 to 10:00 has different component identities; another export of the identical PDF retains its identities despite a numbered output name.
- Importing different snapshots may keep multiple representations. No client update, duplicate suppression, or automatic cleanup is promised.

### 2026-10-11 — Confirmed revision-timestamp provenance stance

- The user prefers the source Created UTC footer and specifies construction-time fallback through the timestamp-provenance decision (Q27).
- Generated example: a source Created 10Jan2099 12:34 (UTC) value supplies metadata; without usable source metadata, the current construction timestamp does, while scheduled activity dates remain unchanged.
- This is a selected-family convention, not a generic Created/Last-Modified claim or a new calendar-client update guarantee.

### 2026-10-11 — Confirmed standby/reserve definition stance

- The user established SB/RE timed availability and SBX single-day/replacement meaning through the standby/reserve definition decision (Q26).
- Generated example: SB 08:00–16:00 is an eight-hour window; SB, SB, FREE, SB remains distinct daily duties and a day off, not one continuous absence.
- Call-to-report delay is not calculated; SBX endpoint availability is clarified separately rather than inventing all-day or old-flight times.

### 2026-10-11 — Confirmed SBX UTC window and validation stance

- The user confirms SBX's own UTC availability window and current FRA example context through the single-day-window decision (Q29).
- The expert-derived rule may be added now but must be validated against a later supplied duty plan; no all-day, cancelled-flight-time, or universal-station fallback is inferred.
- Current calendar snapshots remain separate; replacement duty is not an instruction to alter an older imported calendar.

### 2026-10-11 — Confirmed adjacent UTC boundary stance

- The user selected the unique adjacent Gregorian-date rule through the boundary-date decision (Q30), completing the source-date interpretation record (Q3).
- Generated example: local 1 January 2099 with UTC day marker 31 is 31 December 2098, not January 31; local January 31 with marker 1 is February 1.
- Activity dates remain independent from report/offset/accounting/revision metadata. Boundary-source validation remains pending; the rule is not a release sign-off.

### 2026-10-11 — Complete contract adoption

- The user adopted the complete one-goal contract through the complete-contract decision (Q31), including the displayed inherited mappings/field selection and the retained unknowns/evidence obligations.
- Goal closure: obtain a complete new calendar at an unused path; importing or reconciling it is outside this converter's goal.
- The topic-local Persona remains unpromoted until the separate Handoff decision; no automatic authority change follows contract adoption.

## Review record

### 2026-10-10 — Event Storming reaction

- Artifact: the conversion event flow in `eventstorming.md`, distinguishing complete calendar composition from successful publication.
- Advisory finding: the readable outcome must distinguish actual publication from completion or failure; a prior calendar must not be mistaken for a new successful result.
- Generated example: an unresolved arrival fragment prevents success even if other flights are understood.
- Open dependency: the existing-output/failure question (Q10), which governs replacement and unwritable storage, remains unresolved.
- Disposition: apply the confirmed no-silent-omission rule; retain unresolved recovery behavior as an open guarantee. No new direction is approved by this review.

### 2026-10-10 — Use Case Definition value reaction

- Artifact: the proposed end-to-end goal in `use-case-definitions.md`.
- Advisory value ranking: the one core goal is obtaining the complete calendar representation; isolated correct parsing without an exported calendar is not a valuable stopping point.
- Example: understanding a generated overnight leg but failing publication still leaves the personal-calendar goal unmet.
- Disposition: retain one goal and visible failure; activity/date mappings and the contract await elaboration and user confirmation.

### 2026-10-11 — Full contract elaboration reaction

- Artifact: the draft end-to-end contract in `use-cases/export-duty-plan-calendar.md`.
- Advisory findings: the draft preserves all-day versus timed distinctions, DH visibility, trip context, and prior files. It must report the actual new file path and distinguish a failed current diagnostic from older retained reports.
- Generated example: a revised departure for flight 007 may keep or change its identifier depending on the pending calendar-identity question (Q25); neither choice proves how a calendar application imports it.
- Disposition: retain identity/date/standby/timestamp prerequisites visibly, and require full human contract approval before Handoff. No new Actor, promotion, or import-reconciliation goal is approved.

## Promotion record

### 2026-10-12 — Confirmed ideation Handoff

- The user confirmed project-level promotion through the Handoff decision (Q32).
- The stable core and validated dated topic/review records were copied unchanged from the origin topic; the topic file remains historical.
- Promotion grants a reusable advisory lens only, not Actor identity, code authority, or automatic approval.
