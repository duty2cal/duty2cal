# Design review: report-row-diagnostics

- Date: 2026-10-27
- Step: `givn instructions design-review` (grilling, then hardening)
- Plan under review: `proposal.md`, `specs/export-duty-plan-calendar/export-duty-plan-calendar.feature`, `givn/specs/export-duty-plan-calendar/usecase.md`, `design.md`, `arc42.md`, `docs/arc42/09-architecture-decisions.md`, `docs/arc42/11-risks-and-technical-debt.md`, `docs/arc42/12-glossary.md`, `docs/design/design-system.md`, `docs/design/reference/converter-reference.html`.
- Grilling input: the fresh-context interrogation recorded at `agent://BlondeChicken` (Q1–Q12, F1–F16, branch ledger). Its evidence was re-verified here against the working tree before any edit.
- Result: **DESIGN-REVIEW: PASS** (every question resolved, every finding applied; see the validation log at the end).

## Operator decision

The operator chose **"Reference as drawn"** for the page states on 2026-10-27: implement the rendered reference literally, including the ready-state "Choose another duty plan" button and its alternate ready copy.

Recorded verbatim as the approval:

> The operator chose **"Reference as drawn"** for the page states: implement the rendered reference literally, including the ready-state "Choose another duty plan" button and its alternate ready copy. Date: 2026-10-27.

What that makes authoritative (`docs/design/reference/converter-reference.html`):

- **ready:** heading `Calendar ready`; `Your complete calendar is ready. The browser chooses where the file is saved.`; the calendar offer; status `Calendar ready for 2026-06.pdf.`; the `Choose another duty plan` button.
- **ready-with-warnings:** the same heading; the plural copy `Your complete calendar is ready. The browser chooses where the files are saved.`; the calendar offer and the warning-report offer; the `Warnings` list; status `Your calendar is ready to save. <n> rows were converted with warnings.`; the same `Choose another duty plan` button.
- **failure:** heading `Conversion failed`; a count-based reason sentence naming the source and the counts (`roster.pdf: the plan has one row that cannot be represented and one row that was converted with a warning.`); the `Errors` and `Warnings` lists; the report offer; the note; the `Choose another plan` button.

The first error sentence is **not** the page's failure reason; it appears once, as the first `Errors:` entry. The CLI keeps the first error as its `Reason:` line. A design rule for the count-based page sentence with singular/plural wording is recorded in `design.md` (§Page: DOM, JavaScript and CSS) and `docs/design/design-system.md`.

The approval is recorded in `design.md` (§Visual Design Contract, "Approved direction: reference as drawn") and in `docs/design/design-system.md` (the 2026-10-27 amendment).

## Question records

Each record states what was uncertain, the options with gain/cost/example, the resolution applied, and the alternatives that were rejected so a later reader does not re-open them.

### Q1 — Is the classifier exhaustive for a route row with an incomplete interval, and for a recognized timed code whose interval cannot be built?

**Context.** The plan's classification listed a route with both endpoints, three fragment shapes, three fallback shapes, and two error shapes — but no arm for a row that carries a route and only one endpoint (`TK` + `ABC→DEF` + Start, or `LU742` + `ABC→DEF` + Start), and none for a recognized timed code (`SB/RE/SBX/TK/DTUSA/EM`) that prints no endpoint at all. Today both are edge-case errors; the design as written would classify neither, so the row would be silently dropped, contradicting the use case's "every row is processed" and the proposal's "nothing is silently dropped".

**Options.**
1. **One exhaustive final catch-all error arm.** Any row that reaches no branch is an error; a code-bearing row with both stations uses `a route row carries an incomplete printed interval`, otherwise the existing "neither an activity code nor a route" sentence. Gain: exhaustive by construction, no silent path, one new sentence for every route-with-code case. Cost: one new message and two scenarios. Example: `TK`, `ABC→DEF`, Start 08:00, no End → `row N: … a route row carries an incomplete printed interval`.
2. **Keep today's per-case messages.** Gain: no new sentence. Cost: the message would name only the code for a row that does print a route, and it still needs the same catch-all. Example: `… could not be resolved: TK`.
3. **Represent from the single endpoint** (all-day or zero-length). Gain: never fails. Cost: invents an interval, explicitly out of scope. Example: an 08:00-only TK row becomes all-day.

**Resolution:** Option 1. `design.md` §Aggregation algorithm now qualifies the first branch ("with a resolvable positive interval"), adds the final catch-all, and states that no row leaves classification without an entry or a diagnostic. Scenarios added: "A timed activity code with a route and an incomplete printed interval rejects the conversion" and "An unrecognized activity code with a route and a single printed endpoint rejects the conversion".

**Rejected:** Option 2 (imprecise sentence for a row that carries a route), Option 3 (invents evidence).

### Q2 — Does "a code that carries a route remains travel, whatever its code" include the known codes?

**Context.** The use case (added by this change) said a route remains travel "whatever its code"; `design.md` places the known-code branches before the route branch, and D4's first clause says known codes keep today's semantics. The permanent contract therefore contradicted the realized classifier for a `TK`/`SB` row that also prints `From`/`To`.

**Options.**
1. **Known codes win; narrow the sentence.** Gain: no behaviour change for known codes, D4/D7 and the byte-identical golden all hold; one sentence. Cost: the literal "whatever its code" is withdrawn. Example: `TK` + `ABC→DEF` + 08:00–12:00 stays timed non-flight.
2. **Route wins for every code.** Gain: matches the old sentence literally. Cost: changes known-code semantics, can change calendar bytes, needs golden review, contradicts D4. Example: `SB` + `ABC→DEF` becomes travel.
3. **Keep the order, delete the clause, add a scenario.** Gain: rule and behaviour agree. Cost: same withdrawal, one scenario. Example: "A task-duty row that also prints a route stays one non-flight activity."

**Resolution:** Option 1 plus the pinning scenario Option 3 would add. The use case now reads "A code the converter does not recognize that carries a route remains travel, including a flight number. Known codes win over the route…". `design.md` §Aggregation algorithm step 2.6 states the same. Scenario added: "A known timed activity code that also prints a route stays a timed non-flight activity". `proposal.md` is not edited: its "whatever its code" is clarified (it is about flight numbers, which are outside the known sets), not changed, as `design.md` D4 records.

**Rejected:** Option 2 (behaviour and byte changes outside the proposal's scope).

### Q3 — Where is the per-diagnostic entry line built for the browser's `errors`/`warnings` arrays?

**Context.** The design promised that the CLI and the page "cannot drift into different spellings", but only the two report builders and `diagnostic_text` were named. The page's arrays need the entry line *without* the `- ` prefix, and the page never parses the report — so the wasm facade would have had to re-spell `row <n> (<date>): <message>` in Rust, falsifying the anti-drift claim.

**Options.**
1. **Add `diagnostic_line(&Diagnostic) -> String`.** Gain: one spelling for report, transcript and page; testable through both interfaces. Cost: one small public function and a wasm import. Example: the report's `- row 4 (2099-06-07): …` is `<diagnostic_line>` prefixed.
2. **Return structured records and format in JS.** Gain: the page owns presentation. Cost: duplicates the row/date format in JavaScript and weakens the anti-drift claim. Example: the page assembles `row 4 (…): …`.
3. **Split the report text in the page.** Gain: no API. Cost: parsing is forbidden by the design and brittle.

**Resolution:** Option 1. `design.md` §Report text now specifies the function, its exact output, and that both builders and the wasm facade call it; §Browser result shape says the arrays carry its text. `ffi/wasm/src/lib.rs` keeps its changed-files purpose.

**Rejected:** Options 2 and 3 (both reintroduce a second spelling).

### Q4 — Which copy and controls are authoritative for the ready and failure states?

**Context.** The design declared the contract, but the amendment's operator approval was not recorded, and the reference's ready state has a `Choose another duty plan` button plus copy the current page and the design's JS rules did not have. The failure state additionally repeated the primary reason in `#failure-reason` and as the first error entry.

**Options.**
1. **Approve the reference as-is.** Gain: no design work. Cost: ready-state button and copy diverge from the page's older copy and the design's JS rules unless the design is updated; the duplicate reason stays. Example: ready shows an extra way-back control.
2. **Approve the direction, reconcile the reference to the page.** Gain: reference == implementation. Cost: edits the approved reference and drops a drawn control. Example: no ready-state way back; older ready copy.
3. **Render two alternatives first.** Gain: a real choice at 360 px. Cost: delay; D6/D10 already settled the surface.
4. **Drop the page lists.** Gain: smallest change. Cost: violates D6 and the use case's browser rules.

**Resolution:** the operator chose **"Reference as drawn"** (Option 1, with the design brought into line rather than the reference): the ready-state button and copy, the plural ready-with-warnings copy, the count-based failure sentence and the singular/plural wording rule are now specified in `design.md`, the use case, and the design system. F10's duplicate is removed by making the page's reason a count sentence instead of the primary error sentence. Options 2, 3 and 4 are rejected.

**Rejected:** Option 2 (contradicts the operator's recorded choice), Option 3 (delays a settled surface), Option 4 (contradicts D6).

### Q5 — How is the new page state verified and evidenced, and what is the terminal evidence kind?

**Context.** The design claimed `@axe-core/playwright` on the new states and change-local screenshots, but the delta's `@browser` scenarios contained no baseline/axe/capture steps; the helpers live in the e2e step file the design listed as unchanged. The design also claimed the existing `@e2e` covered the change, which is false: the `@e2e` browser scenario traverses `empty` → `ready` only.

**Options.**
1. **Add baseline/axe/capture steps to the new `@browser` scenarios**, list the e2e step file as changed, and state that the existing `@e2e` does not traverse the new states. Gain: the declared evidence exists. Cost: the "unchanged" list changes.
2. **Keep content-only scenarios and drop the claims.** Gain: honest. Cost: no visual/axe evidence for a declared state; the design system's axe claim becomes false.
3. **Option 1 plus a named per-push CI step.** Gain: per-push gate. Cost: edits `ci.yml` and adds a Chromium run to every push.
4. **Promote the state to `@e2e`.** Gain: e2e-level evidence. Cost: two `@e2e` scenarios for one action, contrary to D10.

**Resolution:** Option 1, without a `ci.yml` edit (Option 3 rejected for CI churn, Option 4 rejected for duplicate e2e coverage). The e2e step file is now listed as changed; the helper takes the state suffix; every new `@browser` scenario asserts `the page meets the visual baseline in the <state> state`; and `design.md` §Capture mechanism states explicitly that the existing `@e2e` does not prove the new states. The nightly-only posture is recorded in `design.md` §Verification posture and in R-070/R-071. The terminal surface is named as text evidence: the CLI scenarios assert transcript lines, not screenshots, because the Visual Contract's interface is web and the terminal's evidence is the transcript (specs instruction, terminal-evidence rule).

**Rejected:** Option 2 (removes declared evidence), Option 3 (CI edits outside this change), Option 4 (over-covers one action).

### Q6 — Is the negative half of the warning rule pinned?

**Context.** The proposal says a route row with a flight number stays travel and a date-only row stays silent; no scenario asserted the absence of a warning for either, so an implementation that warned on every route or every blank row would pass.

**Options.**
1. **One combined scenario.** Gain: fewer scenarios. Cost: compound fixture. Example: one plan with both shapes.
2. **Two scenarios.** Gain: one behaviour each, independently RED-testable. Cost: two fixtures.
3. **Strengthen the existing no-warning scenario.** Gain: no new scenario. Cost: strains the one-behaviour rule.

**Resolution:** Option 2. Scenarios added: "A travel leg with a flight number produces no warning" and "A silent layout row produces no warning" (the latter also asserts that no absence/layover is invented).

**Rejected:** Option 1 (compound), Option 3 (multi-behaviour).

### Q7 — Is the detail-row number that counts silent rows, and the diagnostics order, asserted anywhere?

**Context.** `Diagnostic.row` is the raw table position counting silent rows, and diagnostics are sorted by row so the `Reason:` line is the first error in plan order. No scenario had a silent row before a reported row, and none combined a late-discovered unmatched departure with a later warning.

**Options.**
1. **Two scenarios.** (a) silent row at position 1, unrecognized at position 2 → `row 2`; (b) unmatched departure plus later warning → plan order, departure error is the `Reason:`. Gain: both rules are RED-testable and the design's claims become true. Cost: two fixtures.
2. **Assert only the row number and drop the ordering claim.** Gain: less scope. Cost: report order unspecified; `Reason:` may not be the first error in plan order.
3. **Keep the claims, add nothing.** Cost: unverified rules; a false design statement.

**Resolution:** Option 1. Scenarios added: "A silent layout row before a warned row keeps its place in the row numbering" and "An unmatched departure and a later warning are reported in plan order". `design.md` now names both as the pinning scenarios (F11 closed).

**Rejected:** Options 2 and 3.

### Q8 — What is the fallback's interval rule for a non-positive or overnight unqualified window?

**Context.** The design said an unrecognized code with both endpoints becomes `interval: Some(..)` without a positive-interval qualification, while the use case requires every timed event to end strictly after its start and states the narrow overnight rollover for travel.

**Options.**
1. **Reuse `row_interval`.** Gain: one interval rule for all timed activities; no new semantics. Cost: the travel-worded rollover now applies to a non-travel row, which must be stated. Example: `ZZ_M_CF` 23:10→01:05 becomes next-day timed; 10:00→10:00 is an error.
2. **No rollover for the fallback.** Gain: no travel rule on non-travel. Cost: a readable window crossing midnight fails; inconsistent with SB/TK today. Example: 23:10→01:05 → no calendar.
3. **Always invent a next day.** Cost: invents an interval for equal clocks; rejected.

**Resolution:** Option 1. `design.md` §Aggregation algorithm says the fallback uses the same positive check and narrow unqualified rollover; the use case's timing rule now says "both endpoints of one timed activity — a travel leg, a known timed duty or availability window, or an unrecognized code's printed window". Scenarios added: "An unrecognized activity code with an unqualified window across midnight keeps it as the following UTC day" and "An unrecognized activity code with an equal printed window rejects the conversion".

**Rejected:** Option 2 (a printed, readable window would fail), Option 3 (invented instants).

### Q9 — When an arrival fragment finds no partner, is the result one error or two?

**Context.** The design's steps report an error at the arrival row and an error at every remaining pending departure; a mis-paired leg therefore produces two errors, but no scenario asserted the error count or order, and today's code returns one.

**Options.**
1. **Two errors** (arrival + orphan departure), in row order. Gain: every row accounted for; the permanent scenario's reason phrase still matches. Cost: one leg reported twice. Example: rows 5 and 6 both under `Errors:`, row 5 as the `Reason:`.
2. **One error at the arrival, consuming a departure.** Gain: current behaviour. Cost: the orphan is silently dropped, violating the all-rows invariant.
3. **One error at the departure, consuming the arrival.** Gain: one finding per leg. Cost: the arrival mismatch is not named.

**Resolution:** Option 1. `design.md` step 2.5 and step 3 now state the two errors and that a non-positive merged interval is not reported twice. Scenario added: "A mis-paired travel leg reports both the arrival error and the orphan departure" (two errors, departure error listed first).

**Rejected:** Option 2 (silent drop), Option 3 (hides the arrival mismatch).

### Q10 — Does an unrecognized-code activity carry the row's printed optional context in its description?

**Context.** The fallback variant carries only date, code, station and interval, and `description_fields()` is empty because "the source prints nothing else about the row" — an unverified assumption about other plans. The use case's descriptive-field rule otherwise requires every supplied selected field.

**Options.**
1. **Carry the same optional fields as other timed variants.** Gain: no information loss. Cost: more fields and scenarios.
2. **State the narrower rule.** Gain: smallest representation, matches the November plan. Cost: a documented exception; printed context on such a row is deliberately not represented.
3. **Keep the unverified assumption.** Cost: a future plan loses fields silently; not acceptable.

**Resolution:** Option 2. `design.md`'s rendering table and the use case's descriptive-field rule state the exception explicitly. Scenario added: "An unrecognized activity code's activity carries no description" (the fixture prints a Trip ID T42, and the event still has no DESCRIPTION).

**Rejected:** Option 1 (scope the proposal did not ask for), Option 3 (unverified assumption).

### Q11 — Does "and a warning report" make the success guarantee name two outcomes?

**Context.** The review's use-case conformance criterion rejects a success guarantee naming two outcomes. The guarantee named the calendar "and a warning report for every activity that was represented by a fallback".

**Options.**
1. **One export outcome with two artifacts.** Gain: no split; the criterion is satisfied by intent. Cost: a wording edit the reviewer must accept. Example: the report cannot be obtained without converting the plan.
2. **Treat it as a second outcome and split.** Gain: literal compliance. Cost: D9/D10 reject; a second capability for a non-action.
3. **Leave it.** Cost: the review gate may fail the use case.

**Resolution:** Option 1. The success guarantee now reads "…; when any activity was represented by a fallback, the same export also yields the warning report for it", followed by the recorded rationale: one export outcome with two artifacts, one Goal, one capability.

**Rejected:** Option 2 (contradicts D9/D10 and the interaction inventory).

### Q12 — Which page/report surface contracts were still unpinned?

**Context.** The use case and design specify the warning report's `Source PDF:`/`Warnings:` header, the page's warning-report content, the ready status count, the clearing of a previous attempt's lists, and the failed warning-report offer. The delta asserted none of these; `warningReport` was described as "the exact file text the page offers" with no comparison.

**Options.**
1. **Add assertions to existing scenarios** plus one new scenario for the failed offer. Gain: few new titles. Cost: longer steps; page content needs a captured offer.
2. **Add four new scenarios.** Gain: one behaviour each. Cost: more fixtures; overlap with Option 1.
3. **Narrow the design and use case** to drop these surface rules. Gain: less surface. Cost: weakens D5/D6 and the browser rules the operator's brief requires.

**Resolution:** the substance of Option 1 with one-behaviour scenarios (between Options 1 and 2). Added: "The warning report identifies the source and carries a warnings section" (CLI), "The page states how many rows were converted with warnings", "The page offers a warning report that names the source and carries the warnings", "A second attempt clears the previous attempt's warning list", and "The page keeps the calendar and the warnings when it cannot offer the warning report". The existing failure browser scenario now also asserts the count-based reason sentence and the failure-state baseline.

**Rejected:** Option 3 (drops rules the proposal and use case require).

## Recorded findings and dispositions

| # | Finding | Disposition applied |
|---|---|---|
| F1 | Duplicate `TD-021`: the new inference entry collided with the `cliff.toml` record, and `arc42.md` pointed at the wrong one. | Renumbered the new entry to **TD-029** in `docs/arc42/11-risks-and-technical-debt.md`; `arc42.md` row 11 now names TD-029. |
| F2 | The e2e coverage probe was false: `Kind::Warning` is only constructed by a regular scenario, not by the `@e2e` CLI conversion. | The non-e2e row now carries both probes (`interpret` and `Kind::Warning`'s enumeration); the e2e row now names `apps/cli/src/main.rs` (successful publication) and `Kind::Calendar`, which the `@e2e` run does exercise. |
| F3 | The not-implemented stub pattern was vague. | `design.md` §Test runner, strict mode, and single-scenario command now names it concretely: `panic!("not yet implemented: <step phrase>")`, with the RED run required to fail with that phrase. |
| F4 | `adapters/ics/mod.rs` needs no new match arm — it renders through accessors and has no `match` on the entry. | Removed from the changed-files list and marked **unchanged** in both the technology table and the file table; `arc42.md` row 5 corrected. |
| F5 | The dangling-symlink errno was written as `ENOENT`. | The mechanism sentence now states `EEXIST`: `create_new` fails with `AlreadyExists`, the guard's `exists()` re-check is false, so `publish` returns the error. The scenario and outcome are unchanged. |
| F6 | `ConvertError::Table`'s `Display` was unspecified and could reword today's failure text. | Specified as retaining the established prefix — `a required activity could not be resolved: the detail table carries no <label> column` — and added to the Sentences table. |
| F7 | The use-case example contradicted itself ("later rows are recognized" yet "any warning"). | Reworded to "whose later rows include one unrecognized row with a printed window". |
| F8 | "Date-only" was narrower than the actual silent predicate. | Glossary and design now say **silent layout row (no activity code and no route)**, and the glossary gains that term; the design's D3 wording and traceability row follow. |
| F9 | Reference ready copy/control drift (with the page and the design's JS rules). | Resolved by the operator's **reference as drawn** decision; `design.md`, `design-system.md` and the use case now specify the drawn copy and the `Choose another duty plan` control. |
| F10 | The failure state duplicated the primary reason. | Removed: `#failure-reason` shows the count-based sentence, the first error sentence appears once as the first `Errors:` entry, and the report keeps the first error as its `Reason:` line. |
| F11 | The design's "ordering is exercised" claim was false. | Now true: the ordering is pinned by "An unmatched departure and a later warning are reported in plan order", and the black-box paragraph names the ordering among the exercised branches. |
| F12 | `arc42.md` row 9 said "routed to the use case and design", against the exactly-one-artifact rule. | Reworded to "routed to the use case, with mechanics in `design.md`". |
| F13 | `Diagnostic.date` for a continuation row was ambiguous. | `design.md` now says the date is the row's own date or the carried date of its date block, and the glossary says a continuation row's carried date counts. No warned-continuation scenario is added: the aggregation never gives a continuation row a printed date of its own, and the carried date is asserted to exist before the row is used, so no new observable behaviour arises. |
| F14 | Coverage/script wording drift. | `design.md` §Coverage process boundaries now states `not @e2e and not @browser and not @nested-build` / `@e2e and not @browser and not @nested-build`, the instrumented packages, and that `./measure-coverage.sh` calls `scripts/assemble-features.sh` itself (verified at `measure-coverage.sh:26,33,38`). |
| F15 | The lint overlap advisory was undispositioned. | Dispositioned below (`## Overlap dispositions`): every reported pair is `variant` or `boundary`, and the two shared-shape route-error scenarios are deliberately separate. |
| F16 | Persona wording differed between proposal, use case and design. | `design.md` says "Confirmed Personas for this change" for the two change-confirmed lenses and "pre-existing use-case lens, unaffected by this change" for `project-maintainer--release-owner`. |

## Branch-coverage ledger

| Required branch | Result | Evidence and disposition |
|---|---|---|
| **Scope** | Finding, resolved | The proposal's ten solution bullets map to the delta and nothing extra was added; the classifier gap (Q1), the unasserted proposal rules (Q6), and the unpinned surface rules (Q12) were the gaps. All now have scenarios. `proposal.md` untouched. |
| **Tech choices** | No issue found | No new stack, dependency, crate or boundary. cucumber `0.23` (`apps/cli/Cargo.toml`), Playwright `1.64.1`, `@axe-core/playwright` `4.14.0` (`web/e2e/package.json`), Rust stable / edition 2021 — all unchanged. One factual error (F4) corrected. |
| **Missing scenarios** | Finding, resolved | Added 16 scenarios (12 CLI/contract, 4 page): route + incomplete interval (known and unknown code), known code + route, travel leg without warning, silent row without warning, silent-row numbering, plan-order `Reason:`, overnight fallback, equal-clock fallback, mis-pair two errors, no-description fallback, warning-report header/section, page status count, page warning-report content, stale-list clearing, failed warning-report offer. |
| **Testability** | Finding, resolved | Strict mode is concrete (`.fail_on_skipped()`, `apps/cli/tests/features_runner.rs`); single-scenario commands are concrete (`./run-tests.sh --name`, `--browser --name`); the stub pattern is now the named panic form (F3); row numbering and ordering are asserted (Q7); the false coverage probe is corrected (F2). Every new Then asserts a concrete calendar value, report section, transcript line, exit status, or rendered DOM value. |
| **E2E fidelity** | Finding, resolved | Normalized action scope unchanged (two inventory actions; the new states are variants, D10); real drivers only (Playwright over `web/e2e/driver.mjs`; real CLI subprocess); primary assertions on DOM/transcript/calendar/report; no HTTP substitution; no duplicate `@e2e`. The false "existing `@e2e` covers it" claim is corrected, the baseline/axe/capture steps exist for the new states, and no `@e2e` tag was added or removed. |
| **Interaction Coverage** | No issue found | The inventory lives in `usecase.md` §Interactions (two rows) and both appear in the design's Interaction Coverage Matrix with their `@e2e` titles; no missing or extra row. |
| **Risk** | Finding, resolved | Most likely failure: the collect-and-continue refactor missing a classification shape (Q1) — mitigated by the exhaustive catch-all and the known/unknown route scenarios. Secondary: facade spelling drift (Q3) — mitigated by `diagnostic_line`. |
| **Use-case and Persona context** | Finding, resolved | Identity, primary actor, trigger, one capability, typed relationships, goal-shaped name and changing-use-case declaration are intact; the Goal stays one verb phrase. The "whatever its code" rule (Q2) and the two-outcome reading of the success guarantee (Q11) are corrected; Persona wording aligned (F16). |
| **Decision ledger** | Finding, resolved | D1–D10 all resolve in `design.md` §Decisions with realization and pinning scenarios; no seed reference is required. D4's realization now states known-codes-win (Q2), and D2's one-endpoint sentence matches the Sentences table. |
| **Visual design contract** | Finding, resolved | The contract exists with `Interface:` and both viewports, tokens, primitives, breakpoints, accessibility, reference and capture. The 2026-10-27 approval is recorded (Q4), the reference's copy and controls are authoritative, the declared evidence has steps (Q5), the terminal evidence kind is named, and the duplicate reason is gone (F10). |
| **Deferrals** | No issue found | No mandatory check is deferred. The new states' nightly-only verification is a recorded posture (R-070/R-071), not a deferral, and the design does not edit either workflow. |
| **Persona disposition** | No issue found | Each listed persona has a relevance and evidence: `crew-member--personal-calendar-owner` (accuracy of the calendar and the visible fallback), `contributor--web-reviewer` (the new page surface), `project-maintainer--release-owner` (unaffected). The visual contract has two change-confirmed Personas. |
| **Interface classification** | No issue found | The new surface renders HTML/CSS/JS in a browser, so it is a Web UI and Playwright is the only valid driver; the executable is a terminal CLI driven by `std::process::Command`. The opposite (API-only) classification does not hold because the consumer action is selecting a file and saving a download. |
| **ADR qualification** | No issue found | The candidate (severity model + two report kinds) was assessed `NOT_QUALIFIED` against all five mandatory gates with a completed existing-ADR check in `docs/arc42/09-architecture-decisions.md`; routing is exactly one canonical artifact (the use case), with mechanics in `design.md`. No `NEW_ADR`, no cheaper-home omission. F12 fixed the summary wording. |
| **Architecture documentation** | Finding, resolved | Independently derived chapter set: **Yes** for 1, 3, 4, 5, 6, 8, 9, 10, 11, 12; **No** for 2 and 7. `arc42.md` matches exactly (no omission). Every Yes chapter was opened and carries decision-specific content; all 12 chapter files exist, none is a scaffold, no box-drawing or pseudo-diagram art, no placeholder text. Defect found and fixed: duplicate TD-021 (F1); row 5's ICS claim and row 9's routing wording corrected (F4, F12); row 11 now names R-070/R-071. |
| **Ubiquitous language** | Finding, resolved | The glossary already held **Detail row**, **Conversion error**, **Conversion warning**, **Warning report** and **Failure report**. Fixed: "date-only" → **silent layout row** with a new glossary term (F8), `Table`'s sentence (F6), the continuation-row date (F13), and the known-code-first rule (Q2). `ready-with-warnings` stays a design-system primitive, not a glossary term. |
| **Other** | F1–F16 | All disposed above. |

## Overlap dispositions

`givn lint --change report-row-diagnostics` exits 2 (warnings present, no errors) with `[SUBST]` and `[SHAPE]` advisories. They are dispositioned as follows; none is a duplicate scenario.

| Advisory pair | Class | Reason |
|---|---|---|
| `A silent layout row before a warned row keeps its place in the row numbering` ⊂ `An unrecognized activity code with a printed window…` | `variant` | The new scenario asserts a distinct invariant (raw table numbering across a silent row), not the fallback itself. |
| `An unrecognized activity code with an equal printed window rejects the conversion` ⊂ the two route-error scenarios | `boundary` | Same action, different boundary: a non-positive interval instead of a missing endpoint; it pins the positive-interval check Q8 requires. |
| The two route-error scenarios (`A timed activity code…` / `An unrecognized activity code with a route…`) sharing a shape | `variant` | Deliberately separate: one exercises a known timed code, the other an unknown code on the same route shape. They are the two arms Q1 requires, and merging them would hide which arm fired. |
| Each new negative scenario ⊂ a permanent negative scenario (e.g. `A row with a station but neither a code nor a route…`) | `boundary` | The shared steps are the assertion vocabulary; each new scenario supplies the fixture and the outcome that only its branch produces. |
| `A known timed activity code that also prints a route…` ⊂ the known-code window scenarios | `variant` | Adds the route-present condition; the older scenarios prove the window, this one proves the classification order (Q2). |
| `The page shows no warning list when the plan converts without warnings` ⊂ `A plan at the supported size bound is accepted` | `variant` | Different surface (page rendering vs. the CLI size bound); the shared step is the ready-state assertion. |

## Validation log

Run on 2026-10-27 after all edits, from the repository root:

- `givn check design --change report-row-diagnostics` → exit 0 (no findings).
- `givn check specs --change report-row-diagnostics` → exit 0 (no findings).
- `givn lint --change report-row-diagnostics` → exit 2 (the accepted status: warnings, no errors); 29 `[WIP]` scenario notices plus 53 `[SUBST]` and 1 `[SHAPE]` advisories, all dispositioned above.
- `scripts/assemble-features.sh` → exit 0; the delta assembled as `f0003.feature` beside the permanent `f0001.feature`, so the delta parses.

Not changed by this review: `proposal.md`, production code, step definitions, tests, `tasks.md`, `review.md`. No commit was made.

DESIGN-REVIEW: PASS

## Re-review (fresh context, 2026-10-27)

Scope: the F1/F2 corrections to `design.md`, the design's commands and file
layout against the implementation, and the two required `givn check` runs. No
production code, test, spec, proposal or `review.md` edit was made here.

### Findings and corrections

**F1 — Step Definitions column order: corrected, with one residual defect.**
The header is now
`| Capability | Step definition file | Module | Permanent merge target | New or changed phrase groups |`,
and every row carries the real path in column 2. All five named files exist:
`apps/cli/tests/steps/export_duty_plan_calendar.rs`,
`apps/cli/tests/steps/browser.rs`,
`apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`,
`apps/cli/tests/steps/support.rs`, `web/e2e/driver.mjs`.
`givn check review`'s integrity block now resolves them (`✓ integrity passed`).

**Residual defect R1 (blocking): the e2e row is malformed.** The row for
`export-duty-plan-calendar` (e2e) contains the phrase
`the page meets the visual baseline in the (ready-with-warnings|failure|ready) state`
with three unescaped `|` characters inside the code span. GFM parses table
pipes before inline content, so the row has **7 cells against the 5-cell
header**; the excess cells are ignored when rendered, truncating the phrase at
`(ready-with-warnings`. Every other row has 5 cells. This repository already
uses the escaped form in tables (`\|`, e.g.
`givn/archive/windows-only-release/design.md:134`). Fix: write
`(ready-with-warnings\|failure\|ready)`. The acceptance criterion "no row is
malformed" is therefore not met.

**F2 — Internal Primitives: corrected, with one residual contradiction.**
`## Internal Primitives` now exists and lists the nine primitives named by the
review, each matching the implementation:

| Primitive | Evidence |
|---|---|
| `Severity` | `crates/duty2cal-core/src/application/mod.rs:64` |
| `Diagnostic` | `application/mod.rs:73` |
| `ConversionOutcome.warnings` | `application/mod.rs:207-211` |
| `ConvertError::{Table, Rows}` | `application/mod.rs:160-168` |
| `DutyEntry::UnrecognizedActivity` | `crates/duty2cal-core/src/domain/mod.rs:180-190` |
| `diagnostic_line` | `application/mod.rs:108` |
| `failure_report_text` | `application/mod.rs:116` |
| `warning_report_text` | `application/mod.rs:129` |
| `Kind::Warning` | `apps/cli/src/fs.rs:65,74` |

The section does not contradict `## Data model` or the code.

**Residual defect R2 (blocking): it contradicts `## Report text`.** The new
section (`design.md:130`) and the implementation
(`crates/duty2cal-core/src/application/mod.rs:116`) declare
`failure_report_text(source_name, diagnostics)` — **two** parameters. The
`## Report text` block (`design.md:220`) declares
`failure_report_text(source_name: &str, primary: &str, diagnostics: &[Diagnostic]) -> String`
— **three** parameters. The stale `primary` predates this edit, but the new
section now exposes the contradiction, so the verification requirement "does
not contradict `## Report text`" fails. Fix: align line 220 to
`failure_report_text(source_name: &str, diagnostics: &[Diagnostic]) -> String;`.

### Documentation-only check

- `git status --porcelain` → only
  `M givn/changes/report-row-diagnostics/design.md`.
- `git diff --stat` → `design.md | 34 ++++++++++++++++++++-------`; 26
  insertions, 8 deletions, no other file.
- `git diff -- design.md` → exactly two hunks: the new `## Internal
  Primitives` section and the Step Definitions header/row reorder. No command,
  file layout, framework, driver, capability, scenario, report format or page
  contract changed. The runner commands (`./run-tests.sh`, `--e2e`,
  `--browser`, `--name`) and the paths in the changed-files table still match
  `run-tests.sh` and the implementation. No behavioral deviation.

### Checks run

- `givn check design --change report-row-diagnostics` → **exit 0**, no
  findings.
- `givn check review --change report-row-diagnostics` → **`✓ integrity passed`**
  (also `✓ run declaration passed`); the command exits **1** for reasons
  outside the integrity block: `verification/tasks-verification.json` is
  missing ("run `givn check tasks` before review"), stale
  overlap-disposition rows, and "`proposal.md` has no filled `## Capability
  Routing` table". The integrity block itself passes. No suite was re-run.

### Verdict

No design decision changed and no behavioral deviation was found, but the
corrected artifact is not clean: **R1** leaves a malformed Step Definitions row
and **R2** leaves `## Internal Primitives` contradicting `## Report text`.
The original PASS cannot stand; the two precise fixes above are required, then
a further design-review.

DESIGN-REVIEW: FAIL

## Final re-review (fresh context, 2026-10-27)

Scope: the R1/R2 corrections in `design.md`, the normalized Capability Routing
row in `proposal.md`, the two required gates, and confirmation that no design
decision changed. No production code, test, spec, task or `review.md` edit was
made here; `proposal.md` was read but not edited, and no suite was re-run.

### Verification

**1. `## Step Definitions` — corrected.** The header
`| Capability | Step definition file | Module | Permanent merge target | New or changed phrase groups |`
has 5 cells. All seven data rows (`design.md:409-415`) parse to exactly 5
cells when `\|` is treated as escaped text, so no row splits the table. The
e2e row writes the phrase as
`the page meets the visual baseline in the (ready-with-warnings\|failure\|ready) state`;
the three escaped pipes render as literal characters, so the phrase is no
longer truncated at `(ready-with-warnings`. Every column-2 path exists
(`test -f`, all present): `apps/cli/tests/steps/export_duty_plan_calendar.rs`,
`apps/cli/tests/steps/browser.rs`,
`apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`,
`apps/cli/tests/steps/support.rs`, `web/e2e/driver.mjs`.

**2. `## Internal Primitives` and `## Report text` — corrected and consistent
with the implementation.** Both `design.md:130` and `design.md:220` now
declare the two-parameter
`failure_report_text(source_name: &str, diagnostics: &[Diagnostic]) -> String`,
matching `crates/duty2cal-core/src/application/mod.rs:116`. The rest of the
inventory matches the code: `warning_report_text` (code `:129`) produces the
`Source PDF` / `Warnings` report; `diagnostic_line` (code `:108`) spells
`row <n> (<YYYY-MM-DD>): <message>` with the no-date form; `Severity`,
`Diagnostic`, `ConversionOutcome.warnings`, `ConvertError::{Table, Rows}` and
`DutyEntry::UnrecognizedActivity` are all present at the named locations; and
`Kind::Warning` at `apps/cli/src/fs.rs:65,74` spells
`(".parse-warnings", "txt")`. No contradiction between the two sections
remains.

**3. Capability Routing — the cells are correct, but the table is still not
parsed by the gate (finding R3).** `proposal.md:39-41` names
`export-duty-plan-calendar` in the capability cell and
`EXTEND export-duty-plan-calendar` in the decision cell, so the literal check
passes. The gate does not: `givn check review` still prints
`⚠ proposal.md has no filled '## Capability Routing' table — the delta
capabilities [export-duty-plan-calendar] are undeclared`. The table is
three-column with header `| Capability | Decision | Rationale |`; the parser
expects the four-column form `| Proposed capability | Delta file | Decision |
Rationale |` (recorded empirically in
`givn/archive/publish-public-website/review.md`, finding F4). I verified this
against an isolated copy of the change (created as
`givn/changes/zz-routing-probe`, run, then deleted): the current three-column
table reproduced the warning; changing only the table to the four-column form,
with the same cells plus the delta-file path, cleared it and left only the
expected missing-receipt error. This is therefore not the parser limitation
recorded for `build-browser-converter`; the normalization did not achieve its
stated purpose of letting the gate match the delta. Precise fix for
`proposal.md` (outside this reviewer's allowed edits):
`| Proposed capability | Delta file | Decision | Rationale |` with the row
`` | `export-duty-plan-calendar` | `specs/export-duty-plan-calendar/export-duty-plan-calendar.feature` | `EXTEND export-duty-plan-calendar` | … | ``.
Related staleness (does not change the decision): `proposal.md:37` says
`givn spec route --change report-row-diagnostics` "reports no signal: the top
candidates tie", while the current command reports signal
(`recommendation: EXTEND export-duty-plan-calendar`, score 17.00). The recorded
decision agrees with the tool; the sentence should be refreshed in the same
edit.

**4. Documentation-only — confirmed.** `git status --porcelain` shows exactly
three modified files in the change, all documentation: `design.md` (+27/−9,
three hunks: the `## Internal Primitives` block, the two-parameter signature,
and the Step Definitions header plus row normalization), `proposal.md` (+2/−2,
two hunks: the routing header and capability cell), `design-review.md` (this
appended review). No production code, spec, fixture, command, file layout,
framework, driver, scenario, report format, page contract or `tasks.md`
changed. Severity, classification order, report shapes, page states and runner
commands are identical to the signed design, so no behavioral design decision
changed.

### Checks run

- `givn check design --change report-row-diagnostics` → **exit 0**, no output.
- `givn check review --change report-row-diagnostics` → **exit 1**:
  `✓ integrity passed`, `✓ run declaration passed`,
  `✓ overlap dispositions passed`, net delta 29 added / 0 modified / 2 removed,
  53 stale-disposition advisories (unchanged by this review) and one
  stale split-or-keep advisory, the routing warning quoted in R3, and one
  error: `change verification receipt missing at …/verification/tasks-verification.json; run 'givn check tasks'`. That receipt error is the expected
  pre-`givn check tasks` state and is not a design finding.

### Verdict

R1 and R2 are fixed, the Step Definitions table renders intact, the primitives
and report text match the code, and the review is documentation-only with no
behavioral decision changed. The routing normalization is literal but the gate
still rejects the table, so requirement 3 is not clean and the expected
receipt error is not the only remaining gate reason. With the four-column
table (and the refreshed route sentence) applied, the remaining gate reason
would be the expected missing receipt alone; re-run `givn check review` then.

DESIGN-REVIEW: FAIL

## Routing correction confirmation (2026-10-27)

R3 is resolved. `proposal.md:37-41` now uses
`| Proposed capability | Delta file | Decision | Rationale |` with
`specs/export-duty-plan-calendar/export-duty-plan-calendar.feature` in cell 2
and `EXTEND export-duty-plan-calendar` in cell 3, and states the current route
signal (`EXTEND export-duty-plan-calendar`, 17.00).

Re-ran both gates:

- `givn check design --change report-row-diagnostics` → **exit 0**, no output.
- `givn check review --change report-row-diagnostics` → **exit 1**:
  `✓ integrity passed`, `✓ run declaration passed`,
  `✓ overlap dispositions passed`, net delta 29 added / 0 modified / 2 removed;
  the Capability Routing warning is gone; the 53 stale-disposition advisories
  and one stale split-or-keep advisory are unchanged; the only error is the
  expected `change verification receipt missing at …/verification/tasks-verification.json — run 'givn check tasks'`.

The tree remains documentation-only: `design-review.md`, `design.md` and
`proposal.md`, no production code, spec, fixture, task or `review.md` change,
and no behavioral design decision changed. R1, R2 and R3 are all clean; this
supersedes the earlier FAIL.

DESIGN-REVIEW: PASS
