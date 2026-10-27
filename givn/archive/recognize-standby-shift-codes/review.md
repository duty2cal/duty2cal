# Review — recognize-standby-shift-codes

Change: recognize-standby-shift-codes. Reviewed against the proposal, the delta feature, `design.md`, `design-review.md`, `tasks.md`, the owning use case `givn/specs/export-duty-plan-calendar/usecase.md`, the durable arc42 chapters 11 and 12 (and the chapters claimed unaffected), `docs/evidence/supported-duty-plan.md`, `README.md`, `givn/commands.yaml`, `run-tests.sh`, `scripts/assemble_features.py`, `apps/cli/tests/features_runner.rs`, the step files, the working tree, and the four revisions `8328338`, `69fbea9`, `5181603`, `120bef3`.

The delta holds four `@givn.added` scenarios, none `@e2e`, at 5, 6, 6 and 7 steps. The capability's two `@e2e` scenarios are permanent and unchanged.

**Evidence base.** No archive verification receipt exists for this change (the change has not been archived; `givn/changes/recognize-standby-shift-codes/` holds no receipt and `givn/archive/` has no `recognize-standby-shift-codes` entry). The per-scope reference counts cited here are the recorded runs in `tasks.md`: regular `118 scenarios (118 passed) / 485 steps (485 passed)` and e2e `1 scenario (1 passed) / 5 steps (5 passed)`. The archive receipt will carry the merged measured result. Review did not run the suite, a build, or an instrumented pass. Static checks run in review: `givn status`, `givn lint --change recognize-standby-shift-codes` (exit 0, two `[SUBST]` advisories), `scripts/assemble-features.sh` (assembly only, no test execution), `git show`/`git log`/`git status`/`git diff`, `rustfmt --check` on a copy of the committed step file and on the working file, and greps. The pre-existing `coverage/*.xml` files carry mtime 22:44, before the change's revisions (23:45–23:48), so they are not evidence for this change. Line citations without a revision refer to the working-tree files (the reviewed candidate).

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | `git log -S'@e2e'` over the delta feature path; `git show <rev>:<delta feature>` for all four revisions; grep for `@e2e`/`@wip` under both spec trees | CLEAN — the delta feature never carried `@e2e`: the tag search over its history is empty, and the per-revision contents show only `@givn.added` / `@givn.added @wip` on the four scenarios. The feature header is `@givn.delta @export-duty-plan-calendar`; `@wip` is gone from every scenario (`grep` finds it only inside change prose). The permanent `@e2e` scenarios (`export-duty-plan-calendar.feature:5`, `:430`) are untouched by all four revisions; no delta `@e2e` was planned and removed |
| 1 | Empty or no-op step bodies | grep for `) {}`, whitespace-only bodies, `unimplemented!`, `todo!`, `pending!`, bare `pass` across all step files; read every body this change adds or binds | CLEAN — 0 empty or no-op bodies across all 8 step files under `apps/cli/tests/steps/` (3 define steps: 259 + 54 + 9 = 322 `#[given/when/then]` functions; `browser.rs`, `support.rs`, `e2e/visual_baseline.rs` and both `mod.rs` define helpers, not steps). The six new bodies (`git diff 27fc11f..HEAD` on the capability step file) build a generated PDF fixture or assert on the published calendar, the CLI transcript, the warning report or the failure report. The reused `When` runs the built binary (`export_duty_plan_calendar.rs:84-94,107-109`); warnings are read from the real transcript (`:2889-2895`) |
| 2 | Every `[x]` task has a commit; no spec-only stub | `givn status --change` (22/22 tasks); `git show --stat` for the four revisions; commit subjects and bodies | CLEAN — `8328338` touches production (`crates/duty2cal-core/src/application/mod.rs:220`, `crates/duty2cal-core/src/domain/mod.rs:142`), the first scenario's steps and the durable docs; `69fbea9`, `5181603` and `120bef3` touch `apps/cli/tests/steps/export_duty_plan_calendar.rs`, the delta feature and planning artifacts, and each carries the release note as subject plus its scenario title in the body. `design.md`'s "Exact edits" scopes the production edit to task 1, so tasks 2–4 are scenario/guard tasks with a committable test edit rather than production edits (same shape as the precedent review for `include-report-time-in-travel-title`, check 2). The setup and closeout tasks are verification tasks whose evidence is the recorded command output in `tasks.md`, not source commits |
| 3 | Every component `design.md` promises exists | design "Exact edits", "Step Definitions", "Golden oracle" vs the tree | CLEAN — the six-entry `AVAILABILITY_CODES` (`application/mod.rs:220`) read by `classify` (`:531`); the `DutyEntry::Availability` doc comment (`domain/mod.rs:142`) and its `code`-titled `summary()` (`:237`); the use-case rule (`usecase.md:62,68`) and six examples; glossary rows (`12-glossary.md:69,146,181`); chapter 11 rows (`11-risks-and-technical-debt.md:18,124`); the evidence-record split (`supported-duty-plan.md:32,35`); the new steps in `apps/cli/tests/steps/export_duty_plan_calendar.rs`; the e2e file exists and is unchanged |
| 4 | Strict-mode proof present with output | `features_runner.rs:73-74`; `tasks.md` setup task | CLEAN — `DutyWorld::cucumber().fail_on_skipped().run_and_exit(root)`. `tasks.md` records the pre-implementation single-scenario run selecting one scenario and failing with `Step doesn't match any function`, `1 scenario (1 failed)`, `EXIT=1`. No stub was used to manufacture GREEN |
| 5 | Every `@e2e` Then asserts the real interface | read the permanent `@e2e` Then steps and their definitions | CLEAN — the CLI scenario asserts published `.ics` content (`BEGIN:VCALENDAR`, `BEGIN:VEVENT`, `SUMMARY:LU742 ABC-DEF`, `DTSTART:20990607T090000Z`), the transcript naming the new path, and a zero process exit status; it commits the transcript. The browser scenario asserts rendered page state, the saved calendar's bytes against the reference, the no-request observation and the visual baseline. No Then is repository-only |
| 6 | Browser steps drive a real browser | `apps/cli/tests/steps/browser.rs`, `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`, `web/e2e/driver.mjs` | CLEAN — the steps speak the line protocol to `node web/e2e/driver.mjs`, which imports `playwright` and `chromium` and drives the page; failures are fail-closed (`BrowserDriver::send` accepts only the 13 protocol actions and panics on anything but `ok:true`). No `fetch`/`XMLHttpRequest`/HTTP client stands in for the browser |
| 7 | Exact file(s) `verify.e2e_command` invokes, and parallel-implementation search | `givn/commands.yaml:37`; `run-tests.sh:102,159`; `scripts/assemble_features.py`; `git status --porcelain -uall`; tree search | CLEAN — `e2e_command: "./run-tests.sh --e2e"`. `run-tests.sh` turns it into `--tags "@e2e and not @browser"` (`:102`), assembles the root with `scripts/assemble-features.sh`, and runs `cargo test -p duty2cal --test features_runner` (`:159`), i.e. `apps/cli/tests/features_runner.rs` with the e2e module `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` (plus the shared `browser.rs` driver and `e2e/visual_baseline.rs` helper). Search of the whole tree including the three working-tree-modified files and all untracked files found no second e2e implementation for this capability |
| 8 | E2E scope and inventory normalization | `usecase.md` `## Interactions`; design matrix; assembled feature tags | CLEAN — two normalized actions (supply a plan to the executable; supply a plan through the page and save the calendar), each with exactly one permanent `@e2e` scenario. The delta adds no consumer action and no `@e2e`; its four scenarios are variants of the CLI conversion action, and the three shift codes are an input variant. No excess e2e scenario |
| 9 | `verify.command` vs `verify.e2e_command`, scoped counts | both values read literally from `givn/commands.yaml`; assembled corpus; recorded counts | CLEAN — `verify.command: "./run-tests.sh"` (`commands.yaml:32`) and `verify.e2e_command: "./run-tests.sh --e2e"` (`:37`) are different strings, and the script supplies a real tag filter. Recorded per-scope counts: regular 118 scenarios / 485 steps, e2e 1 scenario / 5 steps — strictly smaller. The 1 is explained and independently reconciled from the assembled root: `@documentation` features are not executed (`assemble_features.py:104-112,194-196`), and `@e2e and not @browser` selects only `export-duty-plan-calendar.feature:6`; the non-browser expanded corpus is 127 scenarios in the export feature minus its 23 browser scenarios, plus 10 in the legal-notices feature, plus the 4 delta scenarios = 118. Counts come from the recorded runs, not an archive receipt |
| 10 | Implementation matches `design.md`'s named commands, layout and framework | design "Step Definitions", "Test runner", "E2E smoke test infrastructure" vs `commands.yaml`, `run-tests.sh`, the step tree, `features_runner.rs`, `web/e2e/driver.mjs` | CLEAN — the exact command value matches, both named step-file paths exist (the e2e one unchanged, no delta copy), cucumber-rs through `run-tests.sh` is unchanged, the CLI driver is `std::process::Command` on the built binary, the browser driver is Playwright, and no dependency moves. One wording-level note: design's parenthetical says the e2e filter is `--tags "@e2e"`, while the pre-existing script narrows it to `@e2e and not @browser`; the design's own E2E section and `commands.yaml` document the browser/e2e split, and this change neither introduces nor needs that filter |
| 11 | Interaction coverage cross-reference | use case inventory ↔ design matrix ↔ feature ↔ step files | CLEAN — see the table below: two entries, two matrix rows, two `@e2e` scenarios, two real drivers (CLI subprocess, Playwright) |
| 12 | Findings from 0–11 are unfinished work | — | CLEAN — no finding from checks 0–11 remains. The item-10 filter wording and the assertion-strength observation in the coverage section are recorded notes, not fabricated or missing behaviour |

### Interaction coverage cross-reference

| Inventory entry (`usecase.md:205-206`) | `design.md` matrix row | `@e2e` scenario in the feature | Driving mechanism in the step file |
|---|---|---|---|
| `export-duty-plan-calendar` — supply a duty plan for conversion through the executable | "supply a duty plan for conversion through the executable" → real interface CLI | `export-duty-plan-calendar.feature:5-6` — "Crew member converts a supported duty plan and receives a calendar file" | `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs:26,46,58` over the shared `When`; the driver is `std::process::Command` on `CARGO_BIN_EXE_duty2cal` (`export_duty_plan_calendar.rs:84-94`), asserting the published `.ics`, the transcript path and exit status 0 |
| `export-duty-plan-calendar` — supply a duty plan through the converter page and save the offered calendar | "supply a duty plan through the converter page and save the offered calendar" → real interface Web UI | `export-duty-plan-calendar.feature:430-431` — "Crew member converts a supported duty plan in the browser and saves the offered calendar" | `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` browser steps through `apps/cli/tests/steps/browser.rs`; the driver is `web/e2e/driver.mjs`, which uses Playwright `chromium` to load the built page, supply the file, read the ready state, save the offer and read the rendered result |

Two entries, two matrix rows, two scenarios, no unmapped interaction and no matrix row without a scenario. This change adds no inventory entry.

### Disclosed working-tree state

`git status` shows three modified files: the delta's `tasks.md` and `design-review.md` (closeout evidence and a wording correction), and `apps/cli/tests/steps/export_duty_plan_calendar.rs`. The step-file difference is rustfmt output only: `rustfmt --check` on a copy of the file at `120bef3` reports two formatting diffs (the blank-times `Given` call at `:428` and the fallback-warning step at `:3017`), while the working file passes. Both spots are lines this change added, and the recorded closeout gate `cargo fmt --check EXIT=0` describes the working tree. The difference changes no assertion, fixture or behaviour; it must remain in the archived checkout so that the candidate the gates see is the candidate that was reviewed. No untracked files exist.

## Arc42 implementation conformance

Durable chapters read. The affected set derived independently from the implementation, `design.md` and `tasks.md` is 11 and 12 — the same set `arc42.md` claims. Chapters 1–10 are claimed unaffected and were checked for contradiction.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 11 — R-004 splits the operator-validated shift meaning from the expert semantics still awaiting their own dated rows | `docs/arc42/11-risks-and-technical-debt.md:18` | Row 11 Yes: R-004 split | Exact edit 6; decisions D1/D2 | Documentation task names chapter 11 | The row reads exactly that and names the generated-fixture limitation; the delta scenarios convert generated plans that print the codes | Yes |
| 11 — TD-029 narrowed to the remaining unrecognized codes; shift codes reuse `row_interval`/`station` and warn no more | `docs/arc42/11-risks-and-technical-debt.md:124` | Row 11 Yes: TD-029 narrowed | Exact edit 6; "Unchanged paths" interval/station | Documentation task names chapter 11 | TD-029 names the three codes as narrowed out, with `SB_X_CF` still on the fallback; the code path is unchanged | Yes |
| 12 — **Availability duty** code list gains the three shift codes | `docs/arc42/12-glossary.md:69` | Row 12 Yes: availability vocabulary extended | Exact edit 5 | Documentation task names chapter 12 | Glossary lists SB, SBX, RE, SB_F_CF, SB_M_CF, SB_S_CF; `AVAILABILITY_CODES` has the same six | Yes |
| 12 — new **Standby shift** term with genus, contrast pair, code symbols and anti-terms | `docs/arc42/12-glossary.md:146` | Row 12 Yes: term added | Exact edit 5; D2/D3 | Documentation task names chapter 12 | The row exists, names Standby Früh/Mittel/Spät, `CF` uninterpreted, the no-prefix contrast and the distinction from **Standby duty**; it matches `usecase.md:68` | Yes |
| 12 — timing invariant lists the shift codes among the own-window codes | `docs/arc42/12-glossary.md:181` | Row 12 Yes: invariant row revised | D4; "Unchanged paths" interval | Documentation task names chapter 12 | The row names the three codes with SB/SBX/RE | Yes |
| 12 — provenance paragraph records the new term | `docs/arc42/12-glossary.md:47-52` | Row 12 Yes: provenance paragraph | — | Documentation task names chapter 12 | The paragraph names recognize-standby-shift-codes and **Standby shift**, and the refinements it made | Yes |
| 3, 5, 6, 8, 10 claimed unaffected | chapters 03, 05, 06, 08, 10 | Rows No with reasons | No module/flow/boundary change | — | Chapter 5 does not enumerate activity codes; chapters 6 and 8 describe the unrecognized-code fallback, which remains true for every code outside the widened set; chapter 10 has no quality scenario about the fallback; chapter 3's supported-plan statement is unchanged | Yes |
| 1, 2, 4, 7, 9 claimed unaffected | chapters 01, 02, 04, 07, 09 | Rows No; ADR action NONE with a `NOT_QUALIFIED` qualification verdict routed to the use case | No new goal, constraint, deployment or ADR | — | No goal, constraint, deployment artifact or ADR file/register row is added; the decision is one array element set in one component, recorded in the use case | Yes |

The chapter 3 reason ("no user-facing surface changes") is read here as the set of surfaces and external interfaces: no new interface is introduced. The terminal transcript's content shrinks for these rows, which `design.md` records under Persona impact and the Visual Design Contract; no durable chapter states otherwise.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

| Term / wording | Finding |
|---|---|
| **Standby shift** | Recorded at `12-glossary.md:146` with definition, code symbols (`SB_F_CF`, `SB_M_CF`, `SB_S_CF`), the Standby Früh/Mittel/Spät names, the `CF`-not-interpreted rule and the `SB_X_CF` contrast. The use case, delta feature, design, evidence record and code all use it in that sense; the spec never uses the shift names as titles |
| **Availability duty** | The glossary row (`:69`) now lists all six codes; `DutyEntry::Availability` and `AVAILABILITY_CODES` are its recorded symbols; the use case and design say "standby availability", the same concept |
| **Standby duty** | Kept distinct at `:145`; the new term's anti-term column says a Standby shift is one dated window of the umbrella, not a rename. The use case's SB rule and the new shift rule do not collide |
| **Conversion warning** | Glossary `:85` says only an unexpected code warns and a fallback represents the row; the delta's `no conversion warning` / `no warning report is published` steps and the `SB_X_CF` warning scenario use exactly that sense |
| **Conversion error** | Glossary `:84` defines the row-level failure that rejects the conversion; the delta's blank-times scenario asserts no calendar, the row under `Errors:` and a non-zero exit, matching the definition |
| **conversion failure** | The process-level phrase already used by the permanent scenarios and `README.md`; it is the observable outcome of a rejected conversion, not a competing row-level term. No new vocabulary is introduced |
| **Printed code / shift name** | The use case and design state the title is the printed code, never the shift name; the delta scenario asserts `SUMMARY:SB_F_CF/SB_M_CF/SB_S_CF`. No German or English name enters the spec or the calendar |

`UBIQUITOUS LANGUAGE: CLEAN` — every contract term this change uses is in the glossary in the same sense, and the new term is recorded with its code symbols and anti-terms.

## Coverage classification

No archive coverage receipt exists for this change yet; the archive will carry the measured result. The pre-existing `coverage/*.xml` files predate the change and are not used. Classification is from static inspection of the changed branch and the permanent scenarios; the `.feature` scenarios and the golden oracles are the regression cover, and no new unit test is expected or added.

| Region | Classification | Disposition |
|---|---|---|
| `AVAILABILITY_CODES.contains` for the three exact strings (`application/mod.rs:531`) → `DutyEntry::Availability` from the row's own window and station | covered | Delta scenario 1 converts three rows (one per code) and asserts event count, each code's `SUMMARY`, `DTSTART`/`DTEND` and `LOCATION`; delta scenario 3 asserts two `SB_F_CF` days with their own windows and a FREE day between them |
| Availability title is the printed code, never the shift name (`domain/mod.rs:237`) | covered | Delta scenarios 1 and 3 assert `SUMMARY:SB_F_CF`, `SUMMARY:SB_M_CF`, `SUMMARY:SB_S_CF`; no title text is translated |
| No warning and no warning report for the three codes | covered | Delta scenarios 1 and 3 assert `no conversion warning` and `no warning report is published` from the real transcript and directory; the golden reference plan prints none of the codes, so the committed calendar is unchanged |
| Fallback for any other `SB_<letter>_CF` (`SB_X_CF`) with its sentence | covered | Delta scenario 2 asserts exactly one warning, row 1 and the date, the code, and the fallback phrase `converted as a timed activity using its printed UTC window 08:00-15:30` |
| `row_interval` error for a recognized shift row with no printed times (`classify`'s `Err → incomplete`) | covered | Delta scenario 4 asserts no calendar, no warning line, `a required timing could not be resolved`, the row under `Errors:` and a non-zero exit |
| One-endpoint and non-positive shift input variants | covered by the same `row_interval → Err → incomplete` branch | The production branch is the one delta scenario 4 exercises; the messages it chooses are pinned by the permanent single-endpoint and non-positive scenarios. No new production line is reachable only through these input variants |
| A recognized shift code that also prints a route | covered by the availability arm | Once the interval resolves, `has_route` is not read by the availability arm; delta scenario 1 exercises the same lines. The route-incomplete sentence is pinned by the permanent route scenarios, and known-code precedence by "A known timed activity code that also prints a route stays a timed non-flight activity" |
| Golden oracles `2098-06.ics` and `2026-06.ics` | covered | Confirmed byte-safe: neither committed calendar contains a shift code, and the reference and equivalence plan builders print none (`reference_plan` has no availability row; `support::equivalence_plan` prints `LU742` and `U`). No fixture file is touched by the four revisions |
| Dead code | none | The widened literal set is data read by `classify`; no helper, alias, branch or export is left unused, and the domain edit is a doc comment |
| Legitimately hard to test | none | Every changed branch is exercised through the real executable with generated fixtures; nothing needs a network, clock or external service beyond the existing construction-clock control |

Assertion-strength note (not a coverage gap): the new three-activity and two-activity Then steps assert `SUMMARY`, `LOCATION`, `DTSTART` and `DTEND` as whole-calendar substring checks plus an exact event count, so a regression that swapped two standby events' titles would not be caught by that step alone. The production code derives title and interval from the same entry and the windows per date are distinct, so no current path can produce that state; the design specified these assertions and every changed branch is covered. Recorded so the archive knows the scenario's discriminating limit.

## E2E coverage

Canonical policy applied from `givn instructions specs --change recognize-standby-shift-codes`: one normalized action per inventory entry, variants as regular scenarios. This change adds no consumer action — a shift row is an input variant of the existing conversion — so no `@e2e` scenario is owed and none was added; the capability keeps its two permanent `@e2e` scenarios.

The real-interface primary assertion holds: the CLI scenario drives the built `duty2cal` binary as a subprocess and asserts the published calendar, the transcript path and the exit status; the browser scenario drives the rendered page through Playwright and asserts page state, saved bytes, no outbound request and the visual baseline. The configured command is `./run-tests.sh --e2e` (a literal `@e2e and not @browser` filter, distinct from `verify.command`), and the per-scope counts (118 regular vs 1 e2e) prove isolation. The browser `@e2e` runs through its separate `--browser` mode, as `commands.yaml` documents.

Opposite classification tested: the converter page does render HTML/CSS/JS in a browser, so that surface is a Web UI and the browser-driver rule applies to it. It does not apply to this change's changed surface: no page source, copy, style, CSP or bundle is edited, the page's warning list is derived from the same conversion outcome and only shrinks for these inputs, and the browser action already has its permanent `@e2e` scenario. The changed observable is the CLI transcript and the calendar bytes, whose scenario variant is the delta. No downgraded e2e scenario exists.

## Deferrals

None. No mandatory check is deferred, and no operator deferral decision is recorded or needed. The missing archive receipt is disclosed in the evidence base; this review uses the recorded runs in `tasks.md` and the committed tree, and the archive will carry the measured result.

## Use-case and Persona conformance

Owning use case: `givn/specs/export-duty-plan-calendar/usecase.md`, the permanent leaf. The change adds the rule at `:68` (the three codes are SB standby windows, exact strings, own window/station, title = code, no warning; an unusable one is a conversion error) and refines the "Only an unexpected activity code warns" rule at `:62`, plus six generated examples. Goal, actors, trigger, preconditions, main flow, extensions, minimal guarantee, success guarantee, capability ownership, `## Interactions` and the typed relationships are unchanged. Gherkin keeps the domain Actor "Crew member"; Personas remain review lenses. The blank-times error outcome is carried by the existing extension "a recognized timed code cannot build a positive interval → conversion error", so no extension was rewritten for a new outcome.

Migration-ledger evidence: the repository has no migration ledger (`find` finds no `*ledger*` file and no `givn/migrations/`); there is no ledger entry to record. The durable vocabulary change is the glossary update above.

### Persona disposition

| Persona | Surface | Disposition |
|---|---|---|
| `crew-member--personal-calendar-owner` (confirmed; accuracy/privacy lens) | the calendar entries and the terminal transcript | Confirmed and satisfied: a plan printing the three codes now yields standby events titled by the printed code with their own windows and stations, no false findings, and an explicit conversion error instead of an invented all-day activity when the window is missing. Delta scenarios 1–4 exercise exactly these outcomes through the real executable; the recorded real-plan smoke converts the operator's November 2026 plan with exit 0, six warnings gone and a byte-identical calendar |
| `contributor--web-reviewer` (confirmed; browser lens) | page source, CSP, no-network proof, WASM bundle, page states | Not affected: this change edits no page code, copy, state or interaction. The page lists whatever warnings the conversion produces, so for these inputs it inherits the shorter list from the same outcome. Nothing in this lens's scope changed |
| `project-maintainer--release-owner` (confirmed; release-evidence lens) | release artifacts, public claims and the support statement's carriers | Not affected in outcome: no version, archive, platform, public statement, carrier or release text changes. The support **Evidence record** was edited truthfully (`supported-duty-plan.md:32,35` splits "printed by the fixture" from "accepted, exercised by scenarios") as the signed design-review Q2 decided; `design.md`'s and `tasks.md`'s parenthetical "no release-evidence change" is imprecise on that one file, but the release-owner's artifacts and claims are unchanged and `scripts/support-statement-check.sh` stays clean per the recorded gate. No Persona was invented or silently promoted |

### Use-case scope rejection criteria

| Criterion | Evidence | Verdict |
|---|---|---|
| Goal conjunction | `## Goal`: "Export one published duty plan as an importable personal calendar file." — one verb phrase | Pass |
| Missing primary actor | `## Actors`: "Crew member — primary Actor"; `## Trigger` names the Crew member | Pass |
| Placement failure | `## Capabilities`: only `export-duty-plan-calendar`, the conversion goal's own leaf; the shift codes belong to that capability | Pass |
| Mutation under an observing goal | `## Success guarantee` names a new calendar file; the use case states it is a changing use case | Pass |
| Extension without a step reference | Every `## Extensions` entry names its main-flow step (2, 3, 4, 6, 2–6) and adds an error or fallback, not a new success outcome | Pass |
| Inconsistent preconditions | `## Preconditions` states the browser capability requirement and says readability, format, the size bound, representability and output availability are checked during the flow | Pass |
| Mechanism vocabulary | Goal, trigger and guarantees name the plan and the calendar file, not a mechanism; PDF, code and report names appear only in the rules and examples | Pass |
| Component- or channel-shaped name | `export-duty-plan-calendar` and the scenario titles describe the actor goal; no module or channel name appears | Pass |
| Oversized leaf | The seven main-flow steps are stages of the single export goal, not three distinct actor goals; the warning report is stated as part of the same export outcome | Pass |
| Relationship misuse | `## Includes: none`, `## Extends: none`; no conditionless `Extends` and no mis-tiered goal | Pass |

## Visual review — terminal interface

- Contract: `design.md` declares `Interface: terminal`, viewports n/a, design system none, no new screen, and names the evidence for the changed lines: the delta's non-e2e assertions plus the existing `@e2e` transcript. It also states the interface classification tested the other way (the browser page is a Web UI, unchanged by this change).
- Evidence completeness: the committed transcript `givn/archive/export-released-roster-calendar/evidence/visual/crew-member-converts-a-supported-duty-plan/transcript.txt` carries `Written: <scenario-dir>/roster.ics` and `exit status: 0`. For the changed state, `tasks.md` records the real-plan terminal run: `Written: /tmp/d2c-after/MLP-2026-11.ics`, `EXIT=0`, `cmp` → `ICS_IDENTICAL`, `no parse-warnings.txt written`, with the six shift warnings gone. The delta's zero-warning, three-activity and error assertions pin the same terminal behaviour on a generated plan. For a terminal interface there are no screenshots or viewports to name; the covered states are the warning conversion, the warning-free conversion and the rejected conversion.
- Fresh-context review per confirmed Persona on the changed surface:
  - `crew-member--personal-calendar-owner`: the surface now names the actual calendar path and exit status without six fallback admissions that were not findings, while the calendar itself carries `SB_F_CF`/`SB_M_CF`/`SB_S_CF` with the plan's station and UTC window; a genuinely unrepresentable row still stops with an explicit failure listing the row. The surface stays English, uses no internal identifier, and the calendar remains the importable file the crew member uses.
  - `project-maintainer--release-owner`: no terminal surface of the release changed; the release text, platform list and archive workflow are untouched.
  - `contributor--web-reviewer`: no terminal surface of the browser lens changed.
- Findings and dispositions:

| Finding | Contract rule / Persona goal | Disposition |
|---|---|---|
| F1 — the real-plan smoke transcript lives in `tasks.md`, not as a separate committed evidence file, and the committed `@e2e` transcript belongs to a different (report-free) plan | contract evidence rule; Persona goal "no false findings on a confirmed plan" | Recorded evidence, no contract violation: `design.md` names the delta's non-e2e assertions as the terminal evidence for these rows and states that no new transcript is owed; the smoke transcript is recorded in `tasks.md`, which archives with the change, and the delta asserts the same zero-warning state through the real executable |
| F2 — the recorded smoke summarises the absence of the six lines rather than quoting the whole before/after transcript | contract rule "the transcript names the actual new calendar path and the exit status" | The named items are present (`Written: <actual path>`, `EXIT=0`, `ICS_IDENTICAL`, `no parse-warnings.txt written`); the absence of the lines is also asserted by the delta's zero-warning steps. No rule requires a line-by-line capture |

Baseline: the transcript still names the actual new calendar path and the process exit status, no internal identifier appears in visible text, the language is English and singular, and the refused-conversion state keeps an explicit reason. Every finding is dispositioned; none is pixel- or coordinate-level.

VISUAL-REVIEW: PASS

## Overlap dispositions

Two `[SUBST]` advisories involve this delta; `givn lint --change recognize-standby-shift-codes` reports exactly these two and no `[SHAPE]` or `[LONG]` finding. The first advisory is that the delta's `SB_X_CF` scenario re-asserts the permanent warning steps; the second is that the delta's shift-error scenario re-asserts the permanent error steps. Both pairs are permanent-vs-delta coverage of the same conversion arm with different determining invariants, so neither is behaviourally identical.

| Scenario A | Scenario B | Decision |
|---|---|---|
| A silent layout row before a warned row keeps its place in the row numbering (permanent) | A standby-like code outside the three shift codes still warns (delta) | variant |
| An unrecognized activity code with a single printed endpoint rejects the conversion (permanent) | A standby shift row without printed times rejects the conversion (delta) | variant |

Rationale: the first pair shares the unrecognized-code-with-window fallback arm but the permanent scenario pins row numbering after a silent row while the delta pins the exact-string recognition rule (`SB_X_CF` is not a prefix rule) and the fallback sentence. The second pair shares the conversion-error outcome but the permanent scenario exercises an unrecognized code with one endpoint while the delta exercises a recognized shift code with no endpoints — a different classification route into the same error branch. Each scenario therefore asserts an invariant the other does not, and removing either would drop a distinct rule.

No scenario was removed or replaced by this change (`@givn.removed` count is zero; the permanent feature is untouched), so there is no removed+added pair to explain.

## Split-or-keep

No delta scenario exceeds the long-scenario gate threshold: `givn lint --change recognize-standby-shift-codes` reports no `[LONG]` finding (exit 0, only the two `[SUBST]` advisories above). The longest delta scenario is "A standby shift row without printed times rejects the conversion" at 7 steps; the others are 5, 6 and 6 steps, and each keeps one invariant. No split-or-keep row is owed.

## README impact decision

README-IMPACT: none

The change touches no command, flag, configuration key, installation step or documented code list. `README.md`'s warning section (`:159-169`) describes the fallback generally and gives an `ZZ_M_CF` row as the example, which stays an unrecognized code and keeps its warning; the README names none of `SB_F_CF`, `SB_M_CF`, `SB_S_CF`, `SBX` or `RE` anywhere (`grep` finds only the `ZZ_M_CF` example line). The statement "when a row is represented by a fallback" remains true for these rows because they are no longer fallbacks. The release note is the changelog surface, not the README. No README section is stale, and this review edited no documentation file.

## Sign-off checklist

- [x] Fabrication audit: clean — 0 empty step bodies across all 8 step files; `@e2e` tag integrity clean (the delta never carried `@e2e`); interaction coverage cross-referenced; design/implementation conformant.
- [x] Every checked task has a verified revision; task 1 carries the production change and tasks 2–4 carry their committable test edits, as `design.md` scopes them. The setup/closeout tasks are verification tasks evidenced by recorded output.
- [x] Every promised component exists: the six-entry literal set, the domain doc comment, the steps, the feature, the use-case rules/examples, the glossary, chapter 11 and the evidence-record split.
- [x] Strict-mode proof present and passing: `.fail_on_skipped()` + `run_and_exit`, with the recorded undefined-step RED (`EXIT=1`) in `tasks.md`.
- [x] Scenario execution evidence recorded per scope: regular 118 / 485, e2e 1 / 5, both passing as recorded in `tasks.md`. No archive receipt exists for this change yet; the archive will carry the merged measured result.
- [x] Coverage classified under the three buckets from static inspection (no receipt available): every changed branch covered, no dead code, no hard-to-test region, no redundant unit test added.
- [x] No `@wip` tags remain; the delta scenario text carries no implementation-layer detail.
- [x] Canonical E2E policy read from the resolved specs instruction; every normalized inventory action has exactly one `@e2e` scenario and the delta's variants are regular scenarios.
- [x] Every `@e2e` scenario has a real-interface primary assertion and uses its reviewed driver; the browser scenario is never replaced by HTTP or `fetch`.
- [x] The local run command is a one-shot CLI (`run.product_type: cli`, no port, no server, no digital twin), so there is no stack to start; the change's recorded real-plan conversion of the operator's plan exited 0 with a byte-identical calendar. Not re-run in review.
- [x] The exact files `verify.e2e_command` invokes were identified and read (`run-tests.sh` → `features_runner.rs` → `steps/e2e/export_duty_plan_calendar.rs`); the tree including untracked files was searched and no parallel e2e implementation exists.
- [x] `verify.e2e_command` differs from `verify.command`; the e2e scope is strictly smaller (1 vs 118 scenarios), reconciled against the assembled corpus.
- [x] Implementation matches `design.md`'s commands, file layout and framework/driver; the only wording-level difference is the pre-existing `@e2e and not @browser` filter description, recorded above.
- [x] Interaction coverage verified: every inventory entry maps to a matrix row, an existing `@e2e` scenario, and a step definition using the promised driving mechanism.
- [x] No finding is excused by a classification outside the three buckets.
- [x] Working-tree state disclosed: the rustfmt-only normalization of two new step definitions is uncommitted and is part of the reviewed candidate; it changes no assertion or behaviour.

## Addendum — Persona-wording correction

After this review was written, the imprecision recorded in the Persona disposition table was corrected in the change's planning artifacts, with no behaviour, scope, step, task, command or code change:

- `design.md:143` (Persona impact) now reads that `project-maintainer--release-owner` is not materially affected because the support statement and release claim are unchanged, while the support-evidence record's accepted-code sentence gains the three codes so the evidence stays truthful.
- `proposal.md:12` (Use-Case Context) states the same.
- `tasks.md:5` (owning use case and Persona) states the same.

`git diff` shows exactly those three lines changed by this correction (plus the pre-existing uncommitted closeout evidence in `tasks.md`); this correction touched no production code, delta feature, step file, use case, durable arc42 chapter or glossary (the step file's pre-existing rustfmt-only working-tree edit is disclosed above and is unchanged by it), and `tasks.md` still records 22/22 tasks against the same four commits.

Disposition: the wording now states the signed design-review's Q2 decision (the fixture stays unchanged; the evidence record splits "printed by the fixture" from "accepted, exercised by scenarios") and removes the only planning-document contradiction this review recorded. No design decision, interface, step, command, task or durable chapter changes, so `design-review.md`'s `DESIGN-REVIEW: PASS` remains the review of that decision, and the arc42 comparison, ubiquitous-language check, coverage classification, e2e check, interaction mapping and visual review above are unaffected — none of them reads this paragraph. This review edited no other artifact.

REVIEW: PASS stands. This addendum supersedes only the "imprecise" clause in the Persona row above; that row's disposition is unchanged.

REVIEW: PASS
