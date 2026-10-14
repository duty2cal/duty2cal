# Review — separate-position-marker-and-flight-number

Change: separate-position-marker-and-flight-number. Reviewed against the approved proposal, the delta, `design.md`, `design-review.md`, `tasks.md`, the completed revisions and the running system. The delta holds five `@givn.added` scenarios, none `@e2e`; the capability's one `@e2e` scenario is permanent and unchanged.

Receipts: `./run-tests.sh` → `3 features / 100 scenarios (100 passed) / 379 steps (379 passed)`; `./run-tests.sh --e2e` → `2 features / 3 scenarios (3 passed) / 16 steps (16 passed)`.

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | `grep -n "@e2e\|@givn"` over the delta; `git log -S"@e2e" -- givn/changes/separate-position-marker-and-flight-number/specs/` | CLEAN — no scenario of this delta ever carried `@e2e`; the only tags in its history are the feature's `@givn.delta @export-duty-plan-calendar` and the five `@givn.added` scenarios |
| 1 | Empty or no-op step bodies | grep for `unimplemented!`, `todo!()`, `{}`, bare `pass`/`return` across `apps/cli/tests/steps/**/*.rs` | CLEAN — no match; each of the five new `Given` bodies builds a PDF fixture and writes it, so none is an empty setup |
| 2 | Every `[x]` task has a commit touching production source | the six scenario/documentation revisions recorded in `tasks.md` | CLEAN — `f30e31d` (application), `035e944` (domain + application), `bd71e36` (application), `279e3d4` (docs), `96fa4a8` (tasks). `6ae3807` and `be3987a` touch no production source: their records state that these two scenarios are guards whose behaviour the code already produced, which is the intended disposition, not a stub |
| 3 | Every component `design.md` names exists | design.md's changed-files table vs the tree | CLEAN — `crates/duty2cal-core/src/domain/mod.rs` carries `printed_identification`, `application/mod.rs` carries the two-cell `Row` and the pairing, the delta and the steps exist under the names given |
| 4 | Strict-mode proof present with output | `tasks.md`, setup task | CLEAN — the RED state before any step existed exits 101 on `21 steps failed` / `Step doesn't match any function`, and the e2e filter is proven a strict subset (100 vs 3) |
| 5 | `@e2e` Then steps assert the real interface | the capability's e2e step file | CLEAN — the `@e2e` scenario asserts the CLI transcript (`Written: <path>`) and the published calendar text, and the exit status; unchanged by this change |
| 6 | Driving mechanism is real, not in-process | the e2e step file | CLEAN — `std::process::Command` runs `CARGO_BIN_EXE_duty2cal` with the plan path; no library call stands in for the CLI |
| 7 | Exactly one `@e2e` implementation for the capability | tree search including untracked files | CLEAN — one e2e step file, bound by `verify.e2e_command` |
| 8 | E2E scope | inventory + delta | CLEAN — the inventory has one action; this change adds none, so no `@e2e` scenario is owed and none was added; the three in-scope `@e2e` scenarios belong to two capabilities |
| 9 | `verify.command` vs `verify.e2e_command` | `givn/commands.yaml` read literally: `./run-tests.sh` vs `./run-tests.sh --e2e`; observed scenario counts | CLEAN — different strings, and the e2e run reports strictly fewer scenarios (3 of 100) |
| 10 | Implementation matches what `design.md` names | file layout, commands, framework | ONE INCONSISTENCY FIXED — design.md's changed-files table omitted `docs/arc42/08-crosscutting-concepts.md`, which `arc42.md` (the artifact that owns the chapter set) lists as affected row 8 and which the change edits; design.md's table now names it. No command, runner, framework or step-file deviation |
| 11 | Local run command starts the whole system | `design.md` Local Runnability; `givn/commands.yaml` product type | CLEAN — `cargo run -p duty2cal -- <plan>`; no server, database or digital twin exists to start |
| 12 | Any finding here means unfinished implementation | — | CLEAN — the one finding (check 10) is a stale list in a planning document, corrected before sign-off; no behaviour was silently overridden |
| 13 | Interaction coverage cross-reference | inventory ↔ matrix ↔ feature ↔ step file | see the table below |
| 14 | Coverage measurement validity | `measure_command`, then `merge_command`, fresh output | CLEAN — see the measurement note below |

### Interaction coverage cross-reference

| Inventory entry (use case `## Interactions`) | Matrix row in `design.md` | `@e2e` scenario in the feature | Driving mechanism in the step file |
|---|---|---|---|
| `export-duty-plan-calendar` — supply a duty plan for conversion | "supply a duty plan for conversion" → "Crew member converts a supported duty plan and receives a calendar file", real interface CLI | `@e2e Scenario: Crew member converts a supported duty plan and receives a calendar file` (permanent) | `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs: `Command::new(env!("CARGO_BIN_EXE_duty2cal"))` with the plan path, then the committed transcript |

One entry, one matrix row, one `@e2e` scenario, one driver — no unmapped interaction and no matrix row without a scenario.

## Coverage measurement (check 14)

| Question | Finding |
|---|---|
| Runner and production processes mapped | `./run-tests.sh` → `cargo test --test features_runner`; the `@e2e` run additionally spawns the built `duty2cal` binary |
| Each process instrumented, collision-safe, flushed, merged | `./measure-coverage.sh` runs two distinct `cargo llvm-cov --workspace --test features_runner` invocations (`--tags "not @e2e"` → `coverage/non-e2e-cobertura.xml`, `--tags "@e2e"` → `coverage/e2e-cobertura.xml`), and `./merge-coverages.sh` combines both into `coverage/cobertura-coverage.xml` |
| A known exercised production path is non-zero | `crates/duty2cal-core/src/domain/mod.rs`: 86 of 89 lines covered in the merged report, including `printed_identification` and `summary`, the two lines this change added |
| Not runner-only, test-only, absent or zero-production | VALID — the merged report carries production lines from all three crates |
| Merged report freshly produced | VALID — both source reports and the merged report were written after the last code revision (`merged 7 files, 886 lines (871 covered) -> coverage/cobertura-coverage.xml`) |

COVERAGE MEASUREMENT: VALID.

## Coverage classification — 15 uncovered production lines

<!-- bucket 1 = dead code, 2 = missing test coverage, 3 = legitimately hard to test -->

| File | Lines | What the uncovered code is | Bucket | Disposition |
|---|---|---|---|---|
| `crates/duty2cal-pdf/src/lib.rs` | 288, 365, 383, 389, 407, 408 | the reader's guards for input the supported family cannot print: a missing date band, an out-of-range footer date or zone, the compact-date conversions | 3 | Pre-existing, recorded by chapter 11's TD-007 (its "fifteen uncovered lines" is exactly this set). No fixture of the supported family can reach them; the previous change recorded the same arms |
| `crates/duty2cal-core/src/domain/mod.rs` | 184, 185, 186 | the exhaustive arms of the `date()` accessor for travel, availability and non-flight duty | 3 | Pre-existing, TD-007 |
| `apps/cli/src/fs.rs` | 47, 54 | the two hard-to-reach filesystem arms of the path helpers | 3 | Pre-existing, TD-007 |
| `crates/duty2cal-core/src/adapters/ics/mod.rs` | 88, 89 | the sink's raw newline and carriage-return escapes | 3 | Pre-existing, TD-007 |
| `apps/cli/src/main.rs` | 27 | the unparsable construction-clock fall-through | 3 | Pre-existing, TD-007 |
| `crates/duty2cal-core/src/application/mod.rs` | 398 | the equal-values arm of `combine` — "the same selected value repeated on both fragments is represented once" | 2 | Pre-existing, and outside this change's approved boundary: the proposal's Out of Scope names "the description fields, their labels, their order, and their fragment-role wording", so the scenario that closes it belongs to the fragment-description rule, not here. Recorded here and owned by TD-007 so the next conversion change closes it with a scenario |

This change introduced no uncovered line and deleted several: the retired `designator()` helper, its deadhead branch and the `Position` match are gone, so the uncovered set is exactly the fifteen lines chapter 11 already records.

## Split-or-keep decisions

The lint gate reports no long scenario in this delta, so no split-or-keep row is owed. Recorded anyway for the operator: every delta scenario is short (three steps for the three title scenarios, four for the failure-message and pairing scenarios), and each keeps one invariant, so none would be clearer split. The pair-level reasoning for the three title scenarios is in the disposition table above (they are **boundary** variants of one rule, not duplicates).

## Overlap dispositions

Every `give a fixture → convert → assert` shape match the lint gate reports involving this delta, one row each. `variant` = the pair shares the capability's single interaction shape but pins a different invariant; `boundary` = the pair differs only in the edge of the same rule (a printed position, no printed position, a non-flight row).

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A crew position and its flight number are printed as separate words | A travel leg that prints no crew position keeps its flight number alone | boundary |
| A crew position and its flight number are printed as separate words | A crew position printed beside a non-flight activity stays out of its title | boundary |
| A travel leg that prints no crew position keeps its flight number alone | A crew position printed beside a non-flight activity stays out of its title | boundary |
| An unmatched travel fragment is named with its position and flight number separate | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| A detail row printed before any dated row rejects the conversion | An unmatched travel fragment is named with its position and flight number separate | variant |
| A detail row printed before any dated row rejects the conversion | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| A date cell whose weekday contradicts the day is not accepted as activity timing | An unmatched travel fragment is named with its position and flight number separate | variant |
| A date cell whose weekday contradicts the day is not accepted as activity timing | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| A dated row in a plan with no roster month heading rejects the conversion | An unmatched travel fragment is named with its position and flight number separate | variant |
| A dated row in a plan with no roster month heading rejects the conversion | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| A detail table without a required column rejects the conversion | An unmatched travel fragment is named with its position and flight number separate | variant |
| A detail table without a required column rejects the conversion | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| An arrival row with no departure partner rejects the conversion | An unmatched travel fragment is named with its position and flight number separate | variant |
| An arrival row with no departure partner rejects the conversion | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| An unknown activity code with no route rejects the conversion | An unmatched travel fragment is named with its position and flight number separate | variant |
| An unknown activity code with no route rejects the conversion | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| A date cell with a single token is not accepted as activity timing | An unmatched travel fragment is named with its position and flight number separate | variant |
| A date cell with a single token is not accepted as activity timing | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| A date cell whose day cannot be read is not accepted as activity timing | An unmatched travel fragment is named with its position and flight number separate | variant |
| A date cell whose day cannot be read is not accepted as activity timing | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| A date cell whose weekday is unknown is not accepted as activity timing | An unmatched travel fragment is named with its position and flight number separate | variant |
| A date cell whose weekday is unknown is not accepted as activity timing | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| A date cell naming a day the month does not have is not accepted as activity timing | An unmatched travel fragment is named with its position and flight number separate | variant |
| A date cell naming a day the month does not have is not accepted as activity timing | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| A supplied plan that cannot be read fails naming the source | An unmatched travel fragment is named with its position and flight number separate | variant |
| A supplied plan that cannot be read fails naming the source | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| A row with a station but neither a code nor a route rejects the conversion | An unmatched travel fragment is named with its position and flight number separate | variant |
| A row with a station but neither a code nor a route rejects the conversion | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |
| A source name that yields no usable path fails without writing anything | An unmatched travel fragment is named with its position and flight number separate | variant |
| A source name that yields no usable path fails without writing anything | A positioning departure fragment has no partner when its arrival fragment prints no crew position | variant |

No scenario was removed or replaced by this change, so there is no removed+added pair to explain.

## README impact decision

README-IMPACT: none

This change alters no command, flag, configuration key or documented output surface. The README describes installing, running and verifying the tool; it names no calendar title. Checked: `grep -n "designator|title|SUMMARY|flight number" README.md` → no match.

## Arc42 implementation conformance

| Chapter | Verdict |
|---|---|
| 01 introduction-and-goals, 02 constraints, 03 context-and-scope, 04 solution-strategy, 06 runtime-view, 07 deployment, 09 decisions | MATCH — no claim of theirs changed; chapter 04's statement that component identity is not derived from a title still holds (`adapters/ics` builds the UID from the snapshot and the index). No ADR is minted, matching `arc42.md`'s NOT_QUALIFIED verdict |
| 05 building-block-view | MATCH — the `domain` row names "the one rule that reads a travel leg's printed crew position and activity together" (`printed_identification`, `domain/mod.rs`), and the `application` row names classification and pairing "by their two printed fields" (`interpret`'s pairing predicate) |
| 08 crosscutting-concepts | MATCH — "Printed identification of a travel leg" describes the two retained cells, the single join rule used by title and failure, the pairing rule, and the `DH` marker needing no second mechanism; all three exist in the code |
| 10 quality-requirements | MATCH — QS-013 lists exactly the three title outcomes the delta pins, plus the non-flight case |
| 11 risks-and-technical-debt | MATCH — TD-010 records the pairing refusal, its reason and the scenario that pins it, and the scenario exists under that title |
| 12 glossary | MATCH — **Crew position** → `DutyEntry::Travel.position`; **Flight number** → `DutyEntry::Travel.activity` and `printed_identification()`; **Travel leg** → `DutyEntry::Travel`, paired "by their two printed fields"; **Deadhead duty** → `DutyEntry::Travel.position` and `summary()`; `Position` no longer appears as a live symbol anywhere in the docs |

ARC42 CONFORMANCE: CLEAN.

## Ubiquitous language conformance

| Term / wording | Finding |
|---|---|
| **Crew position**, **Flight number** | Recorded in `docs/arc42/12-glossary.md` with code symbols that exist; the delta, the design and the use case use the same two terms |
| The retired word `designator` | Gone from the production code: `Designator`/`Position` and the `designator()` helper are deleted, and the failure message's parameter is named for the fragment. It survives in three recorded places only — the permanent deadhead scenario's `Given` text and its matched parameter (deliberately not reworded; `design-review.md` R1 and the reason), the glossary's contrast phrase "not a flight-designator prefix", and chapter 11's historical note. None contradicts the two confirmed terms |
| **Travel leg**, **Deadhead duty** | Both refined in the glossary and consistent with the code |

UBIQUITOUS LANGUAGE: CLEAN.

## Visual review — terminal interface

- Contract: `design.md` declares `Interface: terminal`, no viewports, evidence kind "captured transcript per `@e2e` scenario".
- Evidence completeness: the delta adds no `@e2e` scenario, so this change owes no new transcript; the capability's existing transcript (which names the published path and the exit status) already covers its unchanged `@e2e` scenario. The rendering change is text a crew member reads inside the imported `.ics`, which the golden-oracle scenario and the real-corpus diff pin.
- Fresh-context review: one subagent, which wrote none of these artifacts, judged the changed text surface with the confirmed `crew-member--personal-calendar-owner` lens. It read the expected calendar, the delta and the owning use case. Findings and dispositions:

| Finding | Rule / Persona goal | Disposition |
|---|---|---|
| F1 — the pinned calendar contains no positioning leg, so the golden surface cannot witness the `DH` shape | contract rule "a positioning leg stays distinguishable" | Accepted as an evidence boundary, not a defect: the shape is pinned by the permanent `Deadhead positioning travel keeps one event and marks the assignment in its title` scenario and by the delta's pairing scenario; TD-001 already records that the fixtures are generated and the observed corpus carries no such variation. Not changed here |
| F2 — the route appears in both `SUMMARY` and `LOCATION` of one event | contract rule "no visible content is duplicated" | Accepted, pre-existing and untouched: `proposal.md` lists "the route text in a title (`ABC-DEF`) and the calendar location value" as out of scope, and the surgical golden diff shows the two lines unchanged. The duplication is the RFC 5545 pair (a title a compact view shows and a location property a client shows), not one text repeated |
| F3 — the generated fixture's position token (`LU`) is itself shaped like an airline code, so the fixture does not demonstrate the position/number distinction | Persona goal "a crew position must not be indistinguishable from a flight number" | Accepted, no change: it is a property of generated stand-in data, and the rule is pinned by the delta's `FB 123 ABC-DEF` scenario and by the operator's own plans, where the printed position is `FB`, `DH` or `AK`. Recorded rather than reshaped, because changing the fixture's marker would alter twelve golden lines for no behavioural gain |

Every finding is dispositioned; no finding names a pixel or a coordinate. The baseline holds: every travel line reads `<position> <number> <route>`, a non-flight row prints its code alone, no internal identifier appears, and the failure text and the title are built by the same rule.

VISUAL-REVIEW: PASS

## Persona disposition

PERSONAS: crew-member--personal-calendar-owner
PERSONAS: project-maintainer--release-owner

| Persona | Surface | Finding |
|---|---|---|
| `crew-member--personal-calendar-owner` | the calendar text and the CLI failure text | Reviewed above (F1–F3, all dispositioned); the title and the failure name the leg in the same words |
| `project-maintainer--release-owner` | release artifacts and claims | No surface changed: no dependency, platform, archive, or release claim moves |

## Interface classification — opposite test

Tested the opposite before accepting `terminal`: is this a Web UI? The product writes an `.ics` text file and CLI text; nothing renders HTML, CSS or JavaScript, `givn/commands.yaml` declares a CLI product, and the `@e2e` scenario drives the binary as a subprocess. The fail-closed Web-UI rule cannot apply, so `terminal` holds and no browser driver is owed.

## Gate checks

- `./run-tests.sh` → 100 scenarios, 100 passed.
- `./run-tests.sh --e2e` → 3 scenarios, 3 passed.
- `cargo fmt --all --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → clean; `GIVN_FEATURES=$(scripts/assemble-features.sh) cargo test --workspace` → 100 scenarios passed plus 3 architecture tests.
- `scripts/validate-local-corpus.sh` → 5 passed, 0 failed; the title diff against the pre-change baseline changes 50 travel titles (each gaining one space) and nothing else.
- `scripts/pii-guard.sh` → clean; `givn lint --change separate-position-marker-and-flight-number` → clean.

## Sign-off

- [x] Fabrication audit complete and clean (one planning-document inconsistency found and fixed).
- [x] Every `[x]` task carries a revision; the two guard revisions state why they change no production source.
- [x] Interaction coverage cross-reference complete: one entry, one row, one `@e2e` scenario, one real driver.
- [x] Coverage measured, valid, and every gap classified; no gap introduced by this change.
- [x] Arc42 conformance clean; ubiquitous language clean.
- [x] Visual review complete for the changed text surface; every Persona finding dispositioned.
- [x] README impact decided.
- [x] Both runners green after the last revision.

REVIEW: PASS
