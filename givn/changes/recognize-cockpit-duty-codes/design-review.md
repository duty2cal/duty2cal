# Design review: recognize-cockpit-duty-codes

```text
Review kind: self-review
Author: omp
Reviewer context: self (same context)
```

## Grilling

### Scope

- **Question:** Does the design add only the six confirmed code families and leave the unconfirmed codes alone?
- **Finding:** Yes. `U1` and `OFF_2` join the all-day set; `MED` and `EMCRM` join the exact non-flight set; `WBT_` and `EBTF3` join a prefix set. `AMV`, `REP`, `SBAO`, `S5` and every other code keep the fallback. The support statement, calendar shape, descriptions and reports are untouched.
- **Resolution:** No change.

### Tech choices

- **Question:** Is a prefix predicate the right mechanism for `WBT_*` and `EBTF3*`, or should the design list only the observed codes?
- **Finding:** The operator wrote `WBT_*` and `EBTF3*`, so the prefix is the confirmed rule. The predicate is one helper, preserves the suffix in the title, and is bounded by the two prefixes. Exact matching would ignore the operator's instruction and warn on the next training row.
- **Resolution:** No change.

### Missing scenarios

- **Question:** Which observable rules lacked a scenario?
- **Finding:** The modified all-day outline covers `U1`/`OFF_2`, the new outline covers every exact code and both prefixes, and the boundary outline covers `WBTX`/`EBTF4A`. The existing unknown-code scenarios still cover the general fallback. A recognized code with one printed endpoint is already covered by the existing error scenarios.
- **Resolution:** No change.

### Testability

- **Question:** Can each new scenario fail before the code change, and do the assertions check concrete values?
- **Finding:** Yes. Under the old classifier `U1`/`OFF_2` would be `unknown duty:` events with warnings; the new `Then` asserts the exact title and the `DTSTART`/`DTEND`, and `the converter reports no conversion warning` asserts the transcript. The near-miss outline asserts the fallback title and one warning. The runner is `./run-tests.sh`; strict mode is `.fail_on_skipped()`; a regular scenario runs with `./run-tests.sh --name "<title>"`. A named outline selects all its Examples, so the outline batches are verified by the full suite.
- **Resolution:** The design's single-scenario sentence is clarified for outlines in the tasks.

### E2E fidelity

- **Question:** Does the change add or redefine a user action?
- **Finding:** No. The new rows are variants of the executable conversion action. The interaction inventory stays two conversion actions plus the support check; no new `@e2e` scenario is introduced.
- **Resolution:** No change.

### Interaction coverage

- **Question:** Does every inventory entry have a matrix row?
- **Finding:** Yes; the design's matrix maps the executable conversion, the browser conversion and the support check. No entry is missing or extra.
- **Resolution:** No change.

### Risk

- **Question:** What is the most likely failure?
- **Finding:** A future code that starts `WBT_` or `EBTF3` but means something else would be recognized. The operator supplied the prefixes; the near-miss scenarios bound the rule, and a new meaning needs its own change. The printed station remains the location; the user's “at home”/“at the company” is not turned into an invented venue. A crew member who finds the station misleading can report it, and a later change can omit or relabel it.
- **Resolution:** Recorded as a residual risk and a Persona observation; no behavior change.

### Use-case and Persona context

- **Question:** Are the use-case identity, capability ownership and Personas correct?
- **Finding:** Yes. `export-duty-plan-calendar` owns the capability; no new capability. The crew member is the accuracy lens; the release owner is the evidence lens. No Persona is used as a Gherkin actor.
- **Resolution:** No change.

### Decision ledger

- **Question:** Are D1 and D2 resolved with their alternatives?
- **Finding:** Yes. D1 chooses the existing non-flight representation with the printed code as title; D2 chooses the confirmed prefixes. The proposal records the rejected options and the design's ledger resolves both.
- **Resolution:** No change.

### Contracts and components

- **Question:** Does the change add or split a contract?
- **Finding:** No. `RosterSource` is unchanged; the classification is an internal application rule. No consumer imports an implementation.
- **Resolution:** No change.

### Decision timing

- **Question:** Is anything carried or decided before its evidence exists?
- **Finding:** No. The meanings come from the operator; the two open questions are answered. No scenario depends on an undecided question.
- **Resolution:** No change.

### Visual design contract

- **Question:** Is the interface classification correct?
- **Finding:** The converter page is a Web UI; the design declares `Interface: web` and the existing viewports. The change can shorten the `Unknown duties` list but adds no screen, state, control, layout or interaction. No new `@e2e` browser scenario or screenshot state is owed.
- **Resolution:** No change.

### Deferrals

- **Question:** Is a mandatory check deferred?
- **Finding:** No. The full suite, E2E suite, support check and bats suite run locally; the change adds no infrastructure.
- **Resolution:** No change.

### Persona disposition

- **Question:** Which Personas review which surface?
- **Finding:** The crew member reviews the calendar titles and locations; the release owner reviews the evidence record. The web reviewer sees no new page state. The location observation is recorded above.
- **Resolution:** No change.

### ADR qualification

- **Question:** Does any decision qualify as an ADR?
- **Finding:** No. The change edits constants and a predicate in an existing classifier. The `arc42.md` assessment records the four answers and the cheaper homes; ADR-0008 and the label-unknown-duties decision remain accurate.
- **Resolution:** No change.

### Architecture documentation

- **Question:** Does the 12-row assessment match an independent reading?
- **Finding:** Yes. Independently, chapters 11 and 12 are affected (R-074/TD-029 and the glossary); the other ten are not. `arc42.md` marks the same rows. R-074 no longer claims the converter never reads code letters, and TD-029 records the 2026-11-14 confirmed meanings.
- **Resolution:** No change.

### Ubiquitous language and structure

- **Question:** Is the vocabulary current?
- **Finding:** `docs/arc42/12-glossary.md` is the project's durable glossary (no `docs/glossary.md` exists). It now defines **Day off**, **Medical appointment duty** and **Training duty**, and updates **Vacation day** and the code invariant. No aggregate or relationship moves, so no `docs/domain-model.md` update is owed.
- **Resolution:** No change.

## Hardening applied

- Added the modified all-day outline with `U1` and `OFF_2`.
- Added the non-flight outline and the near-miss prefix outline.
- Added the generic titled timed-activity step.
- Extended the evidence record, the support-statement check and its bats suite with the confirmed codes.
- Updated the use case, glossary, R-074 and TD-029.

## Verdict

Every branch is walked; no unresolved question and no failed contract check remains.

DESIGN-REVIEW: PASS
