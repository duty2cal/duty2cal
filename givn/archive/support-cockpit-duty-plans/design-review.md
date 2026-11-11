# Design review: support-cockpit-duty-plans

```text
Review kind: self-review
Author: omp
Reviewer context: self (same context)
```

## Grilling

### Scope

- **Question:** Does the design contain only the proposal's two reading corrections and the approved support wording?
- **Finding:** Yes. The code change is confined to `crates/duty2cal-pdf/src/table.rs`; the interpretation, calendar writer, UIDs, reports and both facades are untouched. The copy change is confined to the support statement, its five carriers, the organization description, the evidence record and the documents that quote them. Code meanings stay out of scope (D1).
- **Resolution:** No change.

### Tech choices

- **Question:** Is the three-month candidate window with row-order continuation the simplest mechanism that satisfies the cockpit case?
- **Finding:** Yes. A one-month window cannot date `30 Thu` under an August heading; an unbounded search could match a different year's month. The three-month window plus the printed weekday is bounded and deterministic. The sequence rule uses the plan's own order, which is already the table's reading order.
- **Resolution:** No change.

### Missing scenarios

- **Question:** Which observable rules in the proposal lacked a scenario?
- **Finding:** The proposal names the adjacent previous *and next* month, but the delta first covered only the previous-month tail. The next-month path was unexercised.
- **Hardening:** Added `A detail date printed in the next month is resolved by its weekday` with `cockpit_next_month_tail_plan()`, asserting 31 August and 1 September under an August heading. It passes.

### Testability

- **Question:** Can each scenario fail before production code, and do the assertions check concrete values?
- **Finding:** Yes. The prior-month fixture fails on the old reader with `unresolved date: "30 Thu"`; the close-footer fixture fails with `unresolved date: "Created"`; the next-month fixture would resolve the tail backwards or fail. Each asserts a date-bound `SUMMARY`/`DTSTART` pair or an exit status and revision timestamp, not merely that a process ran. The runner is `./run-tests.sh`, strict mode is `.fail_on_skipped()` in `apps/cli/tests/features_runner.rs`, and the single-scenario command is `./run-tests.sh --name "<title>"`.
- **Resolution:** No change.

### E2E fidelity

- **Question:** Does the change add or redefine a user action or a browser test?
- **Finding:** No. The cockpit cases are variants of the existing executable conversion action. The interaction inventory in `export-duty-plan-calendar/usecase.md` keeps two actions; `state-the-support-statement/usecase.md` keeps one. The design's matrix maps all three entries to their existing real drivers.
- **Resolution:** No change.

### Interaction coverage

- **Question:** Does every inventory entry have a matrix row?
- **Finding:** Yes: executable conversion (CLI subprocess), browser conversion and save (Playwright), support-statement check (shell check). No entry lacks a row, and no new row is invented.
- **Resolution:** No change. The lint also named one pre-existing unlisted profile scenario; its inventory row was added so the change's gated lint can pass.

### Risk

- **Question:** What is the most likely way this plan fails?
- **Finding:** The first-row heuristic could choose the wrong month when a day/weekday matches two adjacent months and no earlier row disambiguates. The observed plans start with the previous month's tail, where the nearest-month rule picks the tail; a plan that starts mid-month in the next month is not supported and would be refused or misdated. The design bounds this to three months and the sequence rule handles every later row. A typo that happens to match an adjacent month is accepted; the existing contradictory-weekday scenario remains the negative evidence.
- **Resolution:** Documented in the design's risks. The sequence rule was tightened: if a previous date exists and no candidate is at or after it, the cell is unresolved rather than silently jumping.

### Use-case and Persona context

- **Question:** Are the use-case identities, capability ownership and Personas correct?
- **Finding:** Yes. `export-duty-plan-calendar` and `state-the-converter-support-statement` own the two capabilities; neither is new. The confirmed Personas are the crew member (accuracy/privacy), the release owner (evidence) and the web reviewer (page copy). No Persona is used as a Gherkin actor.
- **Resolution:** No change.

### Decision ledger

- **Question:** Are D1 and D2 resolved, with rejected alternatives recorded?
- **Finding:** Yes. D1 keeps the unknown-code fallback; D2 adopts “Lufthansa crew duty plan” and “one crew member”. The proposal records the rejected options, and the design's ledger resolves both.
- **Resolution:** No change.

### Contracts and components

- **Question:** Does the change add or split a contract?
- **Finding:** No. `RosterSource` is unchanged; `PdfRosterSource` remains its one implementing component. The table construction is an internal helper, not a contract.
- **Resolution:** No change.

### Decision timing

- **Question:** Is anything carried or decided before its evidence exists?
- **Finding:** No. Both proposal questions are answered by the operator. No scenario depends on an undecided question.
- **Resolution:** No change.

### Visual design contract

- **Question:** Is the web classification and the copy-only nature of the change correct?
- **Finding:** The page is a Web UI and the design declares `Interface: web`, both default viewports and the existing design system. The change alters one rendered sentence and its lede, not layout, components, states or interaction. `docs/design/reference/converter-reference.html` is updated as the rendered reference. No new `@e2e` browser scenario is needed because no action or state changes.
- **Resolution:** No change.

### Deferrals

- **Question:** Is any mandatory check deferred?
- **Finding:** No. The regular suite, the E2E suite, the support-statement check and its bats evidence run locally; the change adds no infrastructure.
- **Resolution:** No change.

### Persona disposition

- **Question:** Which Personas review which surface?
- **Finding:** The crew member reviews the conversion accuracy and the in-scope answer; the release owner reviews the claim's evidence and the check; the web reviewer reviews the page's statement text. All three are confirmed project Personas.
- **Resolution:** No change.

### Interface classification

- **Question:** Could the change be classified as a non-browser interface?
- **Finding:** No. The browser page is a rendered Web UI; the design says so and keeps the Playwright-driven existing scenario. The executable scenarios remain CLI, which is a separate supported route.
- **Resolution:** No change.

### ADR qualification

- **Question:** Does any decision qualify as a new ADR?
- **Finding:** No. The reading rules are local to one adapter behind an unchanged port; the wording is observable product copy owned by its use case and check. ADR-0008 still owns the extraction boundary. The `arc42.md` assessment records the four qualification answers and the cheaper homes.
- **Resolution:** No change.

### Architecture documentation

- **Question:** Does the 12-row assessment match an independent reading, and do the affected chapters carry real content?
- **Finding:** Yes after hardening. Independently: 01, 02, 03, 05, 06, 08, 09, 11 and 12 are affected; 04, 07 and 10 are not. `arc42.md` marks the same rows and gives reasons. Every chapter file exists with real content, and no affected chapter contains an ASCII-art diagram. No ADR is minted, so chapter 09 needs only its register note updated.
- **Resolution:** No change.

### Ubiquitous language and structural model

- **Question:** Where is the project's vocabulary authority, and did the change keep it current?
- **Finding:** This project's durable glossary is `docs/arc42/12-glossary.md`; `docs/glossary.md` does not exist, and creating it now would create a second authority. The review instruction's default path is therefore satisfied by the project's established location. Chapter 12 was updated for **Crew position**, **Supported duty plan**, **Profile content** and **Organization profile**. `docs/domain-model.md` does not exist; the change moves no aggregate, boundary or relationship, so there is no structural model to update.
- **Resolution:** Recorded as the project's convention; no duplicate glossary or model was created.

## Hardening applied

1. Added the next-month scenario and fixture.
2. Tightened `choose_date` so a backwards candidate is unresolved rather than re-anchored to the heading month, and updated the design's rule.
3. Added the missing `publish-organization-profile` inventory row that `givn lint` reported as a pre-existing blocker.

## Verdict

Every branch is walked. No unresolved question and no failed contract check remains.

DESIGN-REVIEW: PASS
