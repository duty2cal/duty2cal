# Review — include-report-time-in-travel-title

Change: include-report-time-in-travel-title. Reviewed against the proposal, the delta, `design.md`, `design-review.md`, `tasks.md`, the owning use case, the durable arc42 chapters, the completed revisions `fdbbe37`, `64a4ee0`, `667dabf`, `6198868`, `98b1896`, and the running system. The delta holds four `@givn.added` scenarios, none `@e2e`; the capability's one `@e2e` scenario is permanent and unchanged.

Receipts (as recorded in `tasks.md`; review did not re-run the suites):

- `./run-tests.sh` → `6 features / 149 scenarios (144 passed, 5 failed) / 603 steps (598 passed, 5 failed)`. Every permanent scenario and all four delta scenarios pass; the five failures are the concurrent, uncommitted `publish-organization-profile` change (`Step doesn't match any function` / `unimplemented!` bodies), not this capability.
- `./run-tests.sh --e2e` → `5 features / 8 scenarios (4 passed, 4 failed) / 27 steps (23 passed, 4 failed)`; this capability's `@e2e` scenario passes by name (`1 scenario (1 passed) / 5 steps (5 passed)`). The four e2e failures belong to the same concurrent change (missing `../org/SECURITY.md`).
- `cargo clippy --workspace --all-targets -- -D warnings` → clean; `rustfmt --check` on the two changed Rust files → clean; `cargo test -p duty2cal-core -p duty2cal-pdf` → 3 architecture tests pass; `givn lint --change include-report-time-in-travel-title` → exit 0 with 12 `[SHAPE]` + 9 `[SUBST]` advisories dispositioned below; `git diff --exit-code -- apps/cli/tests/fixtures/expected/2098-06.ics` → status 0.

Concurrent user work exists in the tree (`givn/changes/publish-organization-profile/**`, `apps/cli/tests/steps/*publish_organization_profile*`, the e2e sibling, several `docs/arc42` files and `givn/specs/apply-configuration-change/usecase.md`). It is not part of this change, is not credited here, and was not modified.

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | `grep -n "@e2e\|@givn\|@wip"` over `givn/changes/include-report-time-in-travel-title/specs/`; `git log -S"@e2e"` and `git log -p` over the delta path | CLEAN — no delta scenario ever carried `@e2e`. The only tag history is `@givn.added @wip` → `@givn.added` (one per scenario). The feature header is `@givn.delta @export-duty-plan-calendar`; no `@wip` remains. The capability's permanent `@e2e` scenario is untouched |
| 1 | Empty or no-op step bodies | grep for `unimplemented!`, `todo!()`, `) {}` and whitespace-only bodies over both step files of this capability | CLEAN — 0 matches across 134 `#[given/when/then]` functions in `apps/cli/tests/steps/export_duty_plan_calendar.rs` and 4 in `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`. Every new `Given` builds and writes a PDF fixture; the strengthened `Then` reads the published calendar and asserts on it. The `unimplemented!` bodies in `apps/cli/tests/steps/publish_organization_profile.rs` and its e2e sibling belong to the concurrent change; they are not credited to this change and are the source of the suite's 5/4 failures |
| 2 | Every `[x]` task has a commit touching production source | `git show --stat` for the five recorded revisions | CLEAN — `64a4ee0` touches `crates/duty2cal-core/src/domain/mod.rs` (the `summary()` suffix) plus the strengthened step; `667dabf`, `6198868`, `98b1896` touch tests/spec/tasks only, and each declares "Production files: none" because its scenario is a guard for behaviour the code already produced (precedent: `separate-position-marker-and-flight-number` review, check 2); `fdbbe37` touches the durable arc42 chapters, the use case and the planning artifacts. The final verification evidence is an uncommitted edit to `tasks.md`, disclosed in the hand-off |
| 3 | Every component `design.md` promises exists | changed-files table vs tree | CLEAN — `DutyEntry::summary()` at `crates/duty2cal-core/src/domain/mod.rs:202`; the four delta scenarios; the changed step file; the use-case rule and example; `adapters/ics/mod.rs` unchanged and still writes `SUMMARY` from `entry.summary()` |
| 4 | Strict-mode proof present with output | `apps/cli/tests/features_runner.rs:121`, `tasks.md` setup | CLEAN — `.fail_on_skipped()` is present. `tasks.md` records the pre-implementation state: scenario 1 fails on the missing suffix and the three guards fail as undefined `Given` steps, exit 101. No stub was used to turn RED green |
| 5 | Every `@e2e` Then asserts the real interface | `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` | CLEAN — the scenario asserts published `.ics` content (`BEGIN:VCALENDAR`, the event, `SUMMARY:LU742 ABC-DEF`, `DTSTART`), the CLI transcript (`Written: <path>`), and the process exit status. No repository-only assertion |
| 6 | Browser/UI driving mechanism is real | e2e step file and the shared `When` | N/A — this is a CLI capability, not a browser UI. The shared `When` runs `Command::new(env!("CARGO_BIN_EXE_duty2cal"))` with the plan path (`export_duty_plan_calendar.rs:83`); no HTTP or `fetch` stands in for the binary |
| 7 | Exact files `verify.e2e_command` invokes, and parallel-implementation search | `givn/commands.yaml`; `run-tests.sh`; tree search for other `@e2e` implementations of this capability | CLEAN — `e2e_command: "./run-tests.sh --e2e"`; `run-tests.sh` converts it to `cargo test -p duty2cal --test features_runner -- --tags "@e2e"`; the target is `apps/cli/tests/features_runner.rs`, whose e2e module is `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`. The tree (including untracked files) has no second e2e implementation for `export-duty-plan-calendar`; the only other e2e step file belongs to `publish_organization_profile` |
| 8 | E2E scope | use-case `## Interactions`, delta, feature | CLEAN — one normalized action ("supply a duty plan for conversion") and exactly one `@e2e` scenario for it. This change adds no action, so no new `@e2e` scenario is owed; the four delta scenarios are regular scenarios of the same action (Single / None / Differing / equal values). No excess e2e scenario |
| 9 | `verify.command` vs `verify.e2e_command`, scoped counts | read literally from `givn/commands.yaml`; per-scope counts from `tasks.md` | CLEAN — `verify.command: "./run-tests.sh"` and `verify.e2e_command: "./run-tests.sh --e2e"` are different strings, and the script supplies a real `--tags "@e2e"` filter. Per-scope counts: regular 149 scenarios vs e2e 8 scenarios — strictly smaller. No archive receipt exists for this change yet (archive has not run); `tasks.md` carries the per-scope execution evidence and the archive receipt will carry the merged result |
| 10 | Implementation matches what `design.md` names (commands, file layout, framework/driver) | design.md vs `givn/commands.yaml`, the step files, `features_runner.rs`, the adapter | CLEAN — cucumber-rs `harness = false` through `./run-tests.sh`; the non-e2e step file is the one named; the e2e step file is unchanged as designed; the e2e driver is the built binary; no dependency, module, version or command deviates. One wording-level note is recorded below |
| 11 | Interaction coverage cross-reference | use-case `## Interactions` ↔ design matrix ↔ feature ↔ step file | CLEAN — see the table below; one entry, one matrix row, one `@e2e` scenario, one real driver |
| 12 | Any finding here is unfinished work | — | CLEAN — no finding from checks 0–11 remains unresolved. The `.any()` note in check 10 is a wording-level observation about assertion multiplicity, not a fabricated or missing behaviour; it is recorded for the archive rather than silently dropped |

**Note on the strengthened title step (check 10).** `design.md` says the step must "require exactly one line equal to `SUMMARY:<title>`". The implementation unfolds continuations (`calendar.replace("\r\n ", "")`) and requires full-line equality with `calendar.lines().any(|line| line == expected)`; `str::lines()` strips the trailing `\r`. This satisfies the design-review F1 requirement — a wrong `Differing` suffix now produces `SUMMARY:... Report 07:30 LT`, which is not equal to the expected line and fails, instead of passing a substring check. It does not enforce a multiplicity count. Every fixture in scope renders exactly one event, and the title-composition change cannot itself create a second event, so no reviewed behaviour false-passes on this difference. Recorded so the archive can decide whether the design's "exactly one" was meant as a count.

### Interaction coverage cross-reference

| Inventory entry (use case `## Interactions`) | Matrix row in `design.md` | `@e2e` scenario in the feature | Driving mechanism in the step file |
|---|---|---|---|
| `export-duty-plan-calendar` — supply a duty plan for conversion | "supply a duty plan for conversion" → "Crew member converts a supported duty plan and receives a calendar file", real interface CLI | `@e2e Scenario: Crew member converts a supported duty plan and receives a calendar file` | `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` via the shared `When`: `Command::new(env!("CARGO_BIN_EXE_duty2cal"))` with the plan path, then the committed transcript and exit status |

One entry, one matrix row, one `@e2e` scenario, one driver — no unmapped interaction and no matrix row without a scenario.

## Arc42 implementation conformance

Durable chapters read; the affected set derived independently from the implementation, `design.md` and `tasks.md` is 5, 8, 10 and 12 — the same set `arc42.md` claims. Chapters 1–4, 6, 7, 9 and 11 are claimed unaffected and were checked for contradiction.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 5 — `domain` responsibility composes the travel title including the conditional report suffix | `docs/arc42/05-building-block-view.md:118` | Row 5 Yes: same responsibility, no block added/moved | Architecture impact: one branch of the travel title; changed file `domain/mod.rs` | Documentation task names chapter 5 | `DutyEntry::summary()` (`domain/mod.rs:202-225`) appends ` Report {value} LT` only for `Some(FieldValue::Single(value))` | Yes |
| 8 — "Printed identification of a travel leg" describes the suffix, its exact spacing, `LT` as literal local time, and omission for no value/differing values | `docs/arc42/08-crosscutting-concepts.md:31` | Row 8 Yes: extended section | "The title composition rule", "Why differing values omit the suffix" | Documentation task names chapter 8 | `summary()` implements exactly that branch; `description_fields()` still renders `Report (LT)` via `push_value()` | Yes |
| 10 — QS-021 measurable quality case for the report suffix | `docs/arc42/10-quality-requirements.md:62` | Row 10 Yes: QS-021 added | Decisions: suffix shape, single-value scope, equal-fragment value, differing omission | Documentation task names QS-021 | Delta scenarios 1–4 observe the same outcomes through the published calendar | Yes |
| 12 — **Report time** term with code symbols and non-meanings; **Travel leg** contract use refined; promotion boundary names this change | `docs/arc42/12-glossary.md:75,87,177` | Row 12 Yes: term added, **Travel leg** refined | Glossary impacts section names both | Documentation task names chapter 12 | `DutyEntry::Travel.report_lt`, `FieldValue`, `DutyEntry::summary()`, `description_fields()` all exist; the title and description behaviour matches the entry | Yes |
| 1–4, 6, 7, 9, 11 marked No | chapters 01, 02, 03, 04, 06, 07, 09, 11 | Rows No with reasons | No goal/constraint/context/solution/deployment/flow change; ADR candidate routed to `design.md` | Documentation task scoped to 5/8/10/12 | No external system, no new flow participant, no deployment artifact, no ADR file or register row, no new risk. Chapter 11's TD-007 remains accurate: this change adds no uncovered defensive line in the conversion crates, and its new `summary()` lines are exercised by scenarios 1–4 | Yes |

ARC42 CONFORMANCE: CLEAN

Every row matches; no stale or missing chapter, no unmapped fact.

## Ubiquitous language conformance

| Term / wording | Finding |
|---|---|
| **Report time** | Recorded in `docs/arc42/12-glossary.md:75` with definition, contrast and code symbols (`DutyEntry::Travel.report_lt`, `FieldValue`, `DutyEntry::summary()`, `description_fields()`), all of which exist. The use case, proposal, design and scenario titles use "report time" in the same sense. "report suffix" / "title suffix" in scenario titles name the contract use, not a competing term |
| **Travel leg** | Refined at line 87 to name the conditional ` Report <value> LT` suffix and the no-suffix cases; matches `summary()` exactly. Not "a printed row" and not two activities — unchanged distinctions |
| **Travel activity / title** | The use case rule, the spec and the code all say a travel activity's title; `summary()` is the one composition point. No component or internal name leaks into the spec text |
| **LT** | Used as literal local-time text in the proposal, use case, glossary, design and code. Never treated as an instant, offset or activity date; no timing code reads it |
| **FieldValue / Single / Differing** | Design vocabulary for the existing type; the spec keeps to the observable terms (one value / differing values) rather than implementation names. No implementation-layer detail in the `.feature` text |

`UBIQUITOUS LANGUAGE: CLEAN` — no term is used outside the glossary or in a contradictory sense, and no new contract term is left unrecorded.

## Coverage classification

No archive coverage receipt exists for this change yet (archive has not run), so the classification is from static inspection of the changed branch and the scenarios; the archive receipt will carry the measured result. The change adds one match arm in `DutyEntry::summary()` and no other production line.

| Region | Classification | Disposition |
|---|---|---|
| `FieldValue::Single` → suffix (whole-row value, and equal values from both fragments collapsing in `combine()`) | covered | Delta scenarios 1 and 4 publish the suffixed title; no separate unit test is added (it would duplicate the black-box observation) |
| `FieldValue::Differing` → no suffix, description keeps both labelled values | covered | Delta scenario 3 asserts the route-only title and both fragment-labelled description values |
| `None` → no suffix | covered | Delta scenario 2 plus the byte-identical golden calendar (`2098-06.ics`), which carries no report cells |
| Non-travel variants (`AllDay`, `Availability`, `NonFlightDuty`, `Layover`) | unchanged and covered | Their arms are untouched; the permanent title, layover, all-day and availability scenarios pass |
| One-fragment split leg (report printed on one row only) | covered by composition, no new scenario | The `combine(Some, None)` arm is exercised by the permanent "Complementary optional values from compatible fragments combine" scenario; both fragments' report cells are read and combined by delta scenario 3; `summary()`'s `Single` arm by scenarios 1 and 4. Adding a report-specific variant would re-pin already-observed branches (design-review F2 disposition) |
| Dead code | none | No helper, alias or path is left unused; the change is one match arm |
| Legitimately hard to test | none | No arm requires a real PDF, network, clock or external service beyond the generated fixture |

No file is classified outside the three buckets; the pre-existing `TD-007` uncovered defensive arms are outside this change's boundary and unchanged.

## E2E coverage

Canonical policy applied: the capability has one normalized consumer action, "supply a duty plan for conversion", mapped in the use-case inventory and in the design's Interaction Coverage Matrix to the permanent `@e2e` scenario. This change adds no consumer action and no output-affecting flag, so no new `@e2e` scenario is owed and none was added; the four delta scenarios are regular scenarios of that action.

The real-interface primary assertion holds: the e2e scenario drives the built `duty2cal` binary as a subprocess (`CARGO_BIN_EXE_duty2cal`), then asserts the published `.ics` content, the CLI transcript naming the new path, and exit status 0. The configured command is `./run-tests.sh --e2e` (a literal `--tags "@e2e"` filter, distinct from `verify.command`), and the per-scope counts (149 regular vs 8 e2e) prove isolation.

Opposite classification tested before accepting non-browser: it is not a Web UI. The product emits an `.ics` text file and a terminal transcript; `givn/commands.yaml` declares `product_type: cli`; no HTML, CSS, JavaScript or rendered page exists, and the e2e driver is a subprocess. The Web-UI rule cannot apply, so no browser driver is owed.

## Deferrals

None. No mandatory check is deferred, and no operator deferral decision is recorded or needed. The suite-level failures belong to a concurrent change and are neither deferred work of this change nor evidence of it.

## Use-case and Persona conformance

Owning use case: `givn/specs/export-duty-plan-calendar/usecase.md` ("Export a duty-plan calendar"), permanent leaf. The change inserts one rule ("A travel activity's title ends with the plan's report time in local time…") and one generated example. The goal, trigger, actors, preconditions, main flow, extensions, minimal guarantee, success guarantee, capability ownership and typed relationships are unchanged. Gherkin still uses the domain Actor "Crew member"; Personae remain review lenses. No aggregate, capability or interaction was invented or promoted.

### Use-case scope rejection criteria

| Criterion | Evidence | Verdict |
|---|---|---|
| Goal conjunction | `## Goal`: "Export one published duty plan as an importable personal calendar file." — one verb phrase | Pass |
| Missing primary actor | `## Actors`: "Crew member — primary Actor"; `## Trigger` names the Crew member | Pass |
| Placement failure | `## Capabilities`: only `export-duty-plan-calendar`, the conversion goal's own leaf; the title detail belongs to it | Pass |
| Mutation under an observing goal | `## Success guarantee` names a new calendar file; the use case is explicitly a changing use case | Pass |
| Extension without a step reference | Every `## Extensions` entry names its main-flow step (2, 3, 4, 6, 2–6) and adds failure, not a new success outcome | Pass |
| Inconsistent preconditions | `## Preconditions`: "None beyond the Actor making the request"; readability, format and output availability are checked in the flow | Pass |
| Mechanism vocabulary | Goal, trigger and guarantees name the duty plan and the calendar file, not a mechanism; `LT` and the suffix appear only in the rules | Pass |
| Component- or channel-shaped name | `export-duty-plan-calendar` and the scenario names describe the actor goal, not a module or channel | Pass |
| Oversized leaf | The seven main-flow steps are stages of the single export goal, not three distinct actor goals; the leaf remains one goal | Pass |
| Relationship misuse | `## Includes: none`, `## Extends: none`; no conditionless `Extends` and no mis-tiered goal | Pass |

### Persona disposition

PERSONAS: crew-member--personal-calendar-owner
PERSONAS: project-maintainer--release-owner

| Persona | Surface | Disposition |
|---|---|---|
| `crew-member--personal-calendar-owner` (confirmed; `givn/personas/crew-member--personal-calendar-owner.md`) | the calendar title and the CLI/error text | Confirmed accuracy lens. The title shows the briefing time at a glance, `LT` distinguishes it from the UTC event clock, and a leg with two different report values shows neither rather than guessing; the description keeps both labelled values. Satisfied by scenarios 1–4 |
| `project-maintainer--release-owner` (confirmed) | release artifacts and claims | Unaffected: no dependency, platform, archive, public claim or release artifact changes; the golden calendar is byte-identical |

No Persona was invented or silently promoted; a user-visible surface (the title) has a confirmed Persona.

## Visual review — terminal interface

- Contract: `design.md` declares `Interface: terminal`, viewports n/a, motion n/a, one evidence kind — the transcript committed by the `@e2e` scenario — and names `givn/archive/export-released-roster-calendar/evidence/visual/crew-member-converts-a-supported-duty-plan/transcript.txt`. The delta adds no `@e2e` scenario, so no new transcript is owed; the named transcript exists, is committed, and is unmodified by this change.
- Evidence completeness: the transcript (`Written: <scenario-dir>/roster.ics`, `exit status: 0`) plus the fresh run output in `tasks.md` (the `@e2e` scenario passes by name; the delta scenarios pass) constitute the complete terminal evidence for this change. For a terminal interface there are no screenshots or viewports to name; the one committed state is the conversion transcript above. The changed text itself is not in the transcript — it is the `SUMMARY` line inside the published `.ics`, pinned by delta scenarios 1, 3 and 4 and by the exact golden comparison.
- Fresh-context review per confirmed Persona, against the changed text surface:
  - `crew-member--personal-calendar-owner`: the title reads `LU742 ABC-DEF Report 07:30 LT` — the route remains intact, the suffix is one glance away, `LT` is unambiguous against the UTC event clock, and a conflicting pair leaves the title clean rather than picking a value. The description still carries the labelled `Report (LT)` values. No finding names a pixel or a coordinate.
  - `project-maintainer--release-owner`: no rendered or terminal surface of the release changed.
- Findings and dispositions:

| Finding | Contract rule / Persona goal | Disposition |
|---|---|---|
| F1 — the archived transcript shows the path and exit status, not the title text | contract evidence rule; Persona goal "briefing time visible at a glance" | Accepted as an evidence boundary, not a defect: for a CLI, the transcript is the terminal surface, while the changed title is published inside the `.ics`; it is pinned by the delta's exact `SUMMARY` assertions and by the byte-identical golden. `design.md` explicitly names this transcript and states that no new one is owed |
| F2 — the archived transcript's fixture supplies no report time, so it cannot witness the suffix | Persona goal "no guessing on conflicting values" | Accepted: the e2e fixture is deliberately report-free so the permanent title `LU742 ABC-DEF` stays correct and the golden stays byte-identical; the suffix and omission cases are pinned by scenarios 1–4 through the published calendar. No contract rule is violated |
| F3 — `Report 07:30 LT` at the end of a title is longer than the prior title | contract baseline "English, singular, free of duplicated content" | Accepted: `Report` is the plan's own column label and `LT` a domain marker, not an internal name; no content is duplicated and the baseline holds |

Baseline: no internal identifier or state name appears in the title; the title is English and singular; `LT` is the only new label and is domain vocabulary; the event clock remains UTC and untouched. Every finding is dispositioned, none is pixel-level.

VISUAL-REVIEW: PASS

## Overlap dispositions

Every `[SHAPE]` and `[SUBST]` advisory involving this delta, one row per pair (12 + 9 = 21). `variant` = distinct behavior branches of the same conversion shape; `boundary` = permanent-vs-delta coverage of the same title rule or a different edge of it. No pair is behaviorally identical, so none is `duplicate`.

| Scenario A | Scenario B | Decision |
|---|---|---|
| A travel activity's title ends with its report time in local time | A travel activity without a report time keeps its title unchanged | variant |
| A travel activity's title ends with its report time in local time | Equal report times on both fragments produce one title suffix | variant |
| A travel activity without a report time keeps its title unchanged | Equal report times on both fragments produce one title suffix | variant |
| A crew position and its flight number are printed as separate words | A travel activity's title ends with its report time in local time | boundary |
| A crew position and its flight number are printed as separate words | A travel activity without a report time keeps its title unchanged | boundary |
| A crew position and its flight number are printed as separate words | Equal report times on both fragments produce one title suffix | boundary |
| A travel leg that prints no crew position keeps its flight number alone | A travel activity's title ends with its report time in local time | boundary |
| A travel leg that prints no crew position keeps its flight number alone | A travel activity without a report time keeps its title unchanged | boundary |
| A travel leg that prints no crew position keeps its flight number alone | Equal report times on both fragments produce one title suffix | boundary |
| A crew position printed beside a non-flight activity stays out of its title | A travel activity's title ends with its report time in local time | boundary |
| A crew position printed beside a non-flight activity stays out of its title | A travel activity without a report time keeps its title unchanged | boundary |
| A crew position printed beside a non-flight activity stays out of its title | Equal report times on both fragments produce one title suffix | boundary |
| A travel activity's title ends with its report time in local time | Deadhead positioning travel keeps one event and marks the assignment in its title | boundary |
| A travel activity without a report time keeps its title unchanged | Deadhead positioning travel keeps one event and marks the assignment in its title | boundary |
| Equal report times on both fragments produce one title suffix | Deadhead positioning travel keeps one event and marks the assignment in its title | boundary |
| A travel activity's title ends with its report time in local time | Differing report times leave the title without a report suffix | variant |
| A travel activity without a report time keeps its title unchanged | Differing report times leave the title without a report suffix | variant |
| Equal report times on both fragments produce one title suffix | Differing report times leave the title without a report suffix | variant |
| A crew position and its flight number are printed as separate words | Differing report times leave the title without a report suffix | boundary |
| A travel leg that prints no crew position keeps its flight number alone | Differing report times leave the title without a report suffix | boundary |
| A crew position printed beside a non-flight activity stays out of its title | Differing report times leave the title without a report suffix | boundary |

No scenario was removed or replaced by this change, so there is no removed+added pair to explain. The delta's differing-report `Given` and the permanent differing-values `Given` register different step text but share the `report_time_fragments()` fixture helper; each is needed by its own scenario wording, and neither is dead.

## Split-or-keep

None. No scenario in this delta exceeds the gate's long-scenario threshold; the longest is four steps (`Differing report times leave the title without a report suffix`), and each pins one invariant. No split-or-keep row is owed.

## README impact decision

README-IMPACT: none

The change alters no command, flag, configuration key, installation step or documented CLI surface. The README does not describe calendar event titles: `grep -n "title\|SUMMARY\|Report (LT)\|calendar entry" README.md` returns no match. The release note in the proposal is the changelog surface, not the README.

## Sign-off checklist

- [x] Fabrication audit complete: 0 empty step bodies in this capability's two step files; `@e2e` tag integrity clean; all checked tasks carry their recorded revisions; the promised component exists; strict-mode proof present.
- [x] Every checked task has a verified revision; the three guard revisions declare why they change no production source.
- [x] Every promised component exists (one match arm in `DutyEntry::summary()`; no other production file promised or needed).
- [x] Strict-mode proof present and passing (`.fail_on_skipped()`; RED evidence in `tasks.md`, exit 101).
- [x] Scenario execution evidence recorded per scope (regular 149 / 144 passed, e2e 8 / 4 passed; the failures are the concurrent `publish-organization-profile` change). No archive receipt exists for this change yet; the archive will carry the merged result.
- [x] Coverage classified under the three buckets; no gap introduced; no dead code; no redundant unit test added.
- [x] No `@wip` tags; the delta scenarios contain no implementation-layer detail.
- [x] E2E policy applied: one normalized action, one `@e2e` scenario, variants are regular scenarios; real-interface assertion; no browser substitute.
- [x] The exact file `verify.e2e_command` invokes was identified and read; the tree was searched for a parallel e2e implementation and none exists for this capability.
- [x] `verify.e2e_command` differs from `verify.command`; the e2e scope is strictly smaller (8 vs 149 scenarios).
- [x] Implementation matches design.md's commands, file layout and framework/driver; the only wording-level note (`.any()` vs a multiplicity count) is recorded above and affects no reviewed behaviour.
- [x] Interaction coverage verified: every inventory entry maps to a matrix row, an existing `@e2e` scenario, and a step definition using the promised driver.
- [x] No finding is excused by an invented classification outside the three buckets; no deferral is unrecorded.
- [x] Arc42 conformance clean; ubiquitous language clean; visual review passed; README impact decided.

REVIEW: PASS
