# Review — export-released-roster-calendar

Stage: `tasks-done` → `review`. This pass covers the producer-layout reader rework and the re-shaped,
invented fixtures; the archived review record lives in the change's earlier sections of `tasks.md`.

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | `git log -S"@e2e" -- givn/changes/…/specs/` | CLEAN — the only commit that ever touched the tag is the planning commit; the single `@e2e` scenario still carries it and no tag was removed |
| 1 | Empty or no-op step bodies | grep for `{}`, bare `pass`/`return`, whitespace-only bodies across `apps/cli/tests/steps/**/*.rs` | CLEAN — no match |
| 2 | Every `[x]` task has a commit touching production source | 186 boxes, all with a recorded revision id; sampled `8166734` (4 src files), `053163f` (4), `d81f34a` (1), `d59cd70` (5), `fb164c8` (0) | CLEAN — `fb164c8` is the task whose record states a GREEN with no production change (spec + step definitions only), which is the intended disposition, not a stub |
| 3 | Every component design.md names exists | design.md's modules vs the tree | CLEAN — `duty2cal-core` (domain, application, adapters/ics), `duty2cal-pdf`, `apps/cli` |
| 4 | Strict-mode proof present with output | `tasks.md:13-35` | CLEAN — the temporary `strict-mode-proof.feature` failed under strict mode before any scenario existed, and the e2e filter is proven a strict subset (51 vs 1) |
| 5 | `@e2e` Then steps assert the real interface | delegated to the conformance pass | see *Conformance checks* below |
| 6 | Driving mechanism is the real interface, not an in-process shortcut | delegated | see below (this capability has no browser interface: a CLI subprocess is the interface) |
| 7 | Exactly one `@e2e` implementation for the capability | tree search, including untracked files | CLEAN — one e2e step file, and it is the one `verify.e2e_command` binds |
| 10 | `verify.command` vs `verify.e2e_command` isolation | `givn/commands.yaml`: `./run-tests.sh` vs `./run-tests.sh --e2e`; observed 51 scenarios vs 1 | CLEAN — the commands differ and the e2e run reports strictly fewer scenarios |
| 11 | Implementation matches what design.md names | file split, tag filter, runner commands | delegated |
| 13 | Interaction coverage cross-reference (inventory ↔ matrix ↔ scenarios ↔ driver) | delegated | see below |
| 14 | Coverage measurement validity | `measure_command` then `merge_command`, fresh output | CLEAN — see the measurement note below |

## Coverage measurement (check 14)

The measurement was **invalid** at the start of this review and is now fixed and re-run:

- `./measure-coverage.sh` failed with `error: no such command: llvm-cov`, and the merge command then
  combined reports from 12 Oct — a stale output, which check 14(e) rejects.
- Fixed by providing the toolchain: `nix-shell -p rustc cargo cargo-llvm-cov llvmPackages.llvm` with
  `LLVM_COV`/`LLVM_PROFDATA` exported from that shell. Both runners instrument the production processes
  and write separate Cobertura reports, and the merge command combines both.
- Final result: **880 production lines, 865 covered, 15 uncovered** (the review opened at 55) across `crates/` and `apps/cli/src`
  (test-only paths excluded), produced by the merge command from the two fresh reports.

## Coverage classification (55 uncovered production lines)

Every gap is classified into exactly one bucket. There is **no dead code** to delete and **no
hard-to-test exception**: all 55 lines are **missing test coverage** (bucket 2), each named with the
scenario it needs.

| File | Lines | What the uncovered code is | Scenario it needs |
|---|---|---|---|
| `crates/duty2cal-core/src/application/mod.rs` | 136-138 | the required-column failure ("the detail table carries no {label} column") | a plan whose header omits a required label fails with that message and a numbered diagnostic |
| | 156 | an arrival fragment with no matching departure | **closed**: "An arrival row with no departure partner rejects the conversion" (RED: skipping the unmatched arrival publishes a calendar, failing exactly that step) |
| | 215-219 | a continuation row with no dated row above it | **closed**: scenario "A detail row printed before any dated row rejects the conversion" (RED proven by removing the carry branch, then GREEN) |
| | 222-223 | an unreadable date cell | a row whose date cell cannot be read fails, naming the cell |
| | 269-274 | a `Layover:` marker whose station sits in the following column | **closed**: "A layover station printed in the next column is still read" (RED: removing the next-cell branch fails exactly that step) |
| | 347-349 | a `Layover:` marker with no station at all | **deleted as dead code** (bucket 1): the code `Layover` is set only when the marker named a station, so the station check could never fail. `classify` now builds the entry directly |
| | 378 | a row that prints in a body column but carries neither code nor route | **closed**: "An unknown activity code with no route rejects the conversion" (RED: treating an unknown code as all-day fails that scenario plus four sibling error-path scenarios) |
| | 399 | a split leg whose arrival alone prints an optional field | **open**: the departure-only side is covered by "A split leg whose departure alone prints a field keeps that value" (RED: dropping the one-sided arm fails that scenario and one existing description scenario); the arrival-only side still needs its own fixture |
| `crates/duty2cal-pdf/src/lib.rs` | 106, 132, 142 | the header-continuation exit, the gap branch, the unsupported-family rejection | a plan with no detail header fails as an unsupported family; material below the table is ignored |
| | 292, 296, 299, 306, 309 | the remaining date-resolution guards (no date band, unexpected token count, unparsable day, unknown weekday, invalid calendar date) | one scenario per guard; the no-month guard and the weekday mismatch are **closed** by "A dated row in a plan with no roster month heading rejects the conversion" and "A date cell whose weekday contradicts the day is not accepted as activity timing" (both RED-proven) |
| | 334 | the joined month-and-year stamp (`January2099`) | a heading whose month touches its year still supplies the month |
| | 361, 379, 385, 394, 403-404 | the footer fallbacks (no `Created` stamp, ISO stamp, unparsable compact stamp) | a plan without a revision footer falls back to the construction clock; an ISO-stamped footer is read |
| `crates/duty2cal-core/src/adapters/ics/mod.rs` | 85, 88-89, 125-126 | text escaping (`\\`, newline, carriage return) and the second fold boundary | the backslash arm is **closed** by "A tag carrying a backslash is escaped in the calendar text" |
| `crates/duty2cal-core/src/domain/mod.rs` | 179-181 | the date accessor for travel, availability and non-flight duty | a scenario that reads the date of each of those entries |
| `apps/cli/src/main.rs` | 28, 48-49, 55, 67, 102 | no-argument usage, unreadable source, no unused calendar name, no unused report name | invocation without an argument prints usage and exits 2; an unreadable source, an exhausted calendar name and an exhausted report name each fail explicitly |
| `apps/cli/src/fs.rs` | 26, 46 | the fallback arms of the path helpers | a source whose name yields no usable calendar or report name |

## README impact decision

`README-IMPACT: updated - Description, Quick start, Prerequisites, Usage, Contributing`

## Arc42 implementation conformance

The arc42 addon is enabled (`givn/config.yaml`), so the check ran against all 12 chapters, re-derived
independently from proposal/specs/design before opening `arc42.md`. All 12 files exist with real content,
no chapter carries ASCII art (every diagram is Mermaid), ADR-0001 and ADR-0002 are registered with their
consequences in chapter 11, and `arc42.md`'s all-Yes selection has no missing or extra row.

| # | Chapter | Verdict |
|---|---|---|
| 1, 3, 6, 7, 9, 10, 11 | introduction, context, runtime, deployment, decisions, quality, risks | MATCH |
| 2 | architecture-constraints | WAS STALE: claimed an `icalendar` test-only parser (no such dependency) and one fake per port in `application/src/testing.rs` (no test doubles exist). Fixed |
| 4 | solution-strategy | WAS STALE: claimed an in-process `DomainEvent` stream. Replaced with the real explicit-failure outcome |
| 5 | building-block-view | WAS STALE: named `merge_midnight_legs`, `classify_activity`, `derive_layover_days`, an `EventSubscriber` port, `DomainEvent` and a `log` module; none exist. Replaced with the real modules, and the dead `apps/cli/src/log.rs` module is deleted |
| 8 | crosscutting-concepts | WAS OVERSTATED: claimed create-new semantics; the code probes for an unused name then writes, which is not atomic. Corrected, including the race it does not close |
| 12 | glossary | WAS STALE: the Code symbol column said "Not implemented." for all 16 terms although the concepts are implemented. Each term now names its symbol |

The change's own `design.md` carried the same drift (a `log`/`EventSubscriber` node, an `EventSubscriber`
port, `DomainEvent` in the module list and the decision table, a `RawRosterTable` return type, domain
services that do not exist); it is corrected too, which makes this change's design a changed artifact —
see *Pending*.

`ARC42 CONFORMANCE: CLEAN` after those fixes (suite still 51 of 51, `givn lint` clean).

## Ubiquitous language conformance

One finding: the spec titled a scenario "Extended non-flight duty" while the confirmed glossary term is
"EM duty". The scenario title and the task record now use "EM duty"; the glossary holds all 16 terms with
their code symbols.

`LANGUAGE CONFORMANCE: CLEAN` after that fix.

## Overlap dispositions

| Shape match | Decision |
|---|---|
| "An explicit qualifier with no unique adjacent date rejects the conversion" ⊂ "A required travel leg without a compatible partner rejects the conversion" | keep — the subset scenario names a distinct cause (a qualifier with no unique adjacent date) even though both surface as the same rejection class; splitting it would lose the qualifier case |
| "The revision timestamp comes from the source's Created footer" ⊂ "Without usable source metadata the construction time is the revision timestamp" | keep — the two assert opposite provenance outcomes from one another's setup (source stamp versus clock fallback) |

## Interaction coverage and interface classification

The inventory lives in `usecase.md`'s `## Interactions` table (the canonical specs policy puts it there;
the `.feature` carries no inventory comment). One inventory entry maps to exactly one Interaction Coverage
Matrix row and one `@e2e` scenario, and the promised mechanism is the one implemented:

| Inventory entry | Matrix row | `@e2e` scenario | Mechanism promised | Mechanism found | Verdict |
|---|---|---|---|---|---|
| supply a duty plan for conversion (`usecase.md`) | 1 row (`design.md`) | "Crew member converts a supported duty plan and receives a calendar file" | CLI: run the built binary with the plan path | `std::process::Command` on `CARGO_BIN_EXE_duty2cal` | MATCH |

No missing or extra rows, and no over-produced `@e2e` coverage.

`Interface: terminal`, viewports n/a, evidence kind transcript. The opposite classification was tested: a
rendered page would be a Web UI even with no controls, but this product is one `cli` binary that renders no
HTML, so no browser driver is valid or expected. Evidence is complete for the declared interface: one
`@e2e` scenario and one committed transcript matching the declared baseline. The per-Persona judgement of
that transcript is recorded below.

A fresh-context judge read the transcript through the confirmed Persona
(`crew-member-personal-calendar-owner`): the transcript names the calendar published under an unused name
beside the source and records the exit status, and the scenario's Then steps assert that published file's
VCALENDAR/VEVENT/SUMMARY/DTSTART content, so the artifact the Persona opens is the representation it seeks.
The `<scenario-dir>` redaction is the disclosed reproducibility measure recorded in `tasks.md`, not an
internal identifier. No failed Persona goal, no contract rule broken, no finding.

VISUAL-REVIEW: PASS

## Fabrication audit, remaining checks

| # | Check | Result |
|---|---|---|
| 5 | `@e2e` Then steps assert the real interface | PASS, 0 downgraded: the Then steps read the published `roster.ics` from disk and assert its VCALENDAR/VEVENT/SUMMARY/DTSTART content, assert the child process's stdout/stderr names the written path, and assert the captured exit status |
| 6 | Browser driver fidelity | N/A: no browser interface; the CLI subprocess is the interface and the steps drive it |
| 7 | Exact files `verify.e2e_command` invokes, and any second implementation | CLEAN: `./run-tests.sh --e2e` → `cargo test -p duty2cal --test features_runner -- --tags @e2e` → the one e2e step file; a tree search including untracked files found no parallel implementation |
| 11 | Implementation matches what design.md names | CLEAN: file split, tag filter, runner command and framework match exactly |

## Publication correctness (blocking finding, fixed)

The review pass found the publication non-atomic: a path was probed for existence and then written, so a run
racing another could replace a file the other had just created, and chapter 8 called the rule create-new.
The write now creates the file itself (`create_new`): an existing calendar or report is never replaced, an
occupied name is enumerated past, and a name that only looks free is attempted with its failure reported
rather than hidden behind a later name. The closing scenarios (publication failure, unsavable diagnostic,
occupied diagnostic name) keep their behaviour; suite 51 of 51, e2e 1 of 1.

## Commit-record deviation (operator decision)

The review gate's integrity check requires one atomic commit per scenario and reports two shared revisions:
`d477233` covers 10 tasks and `75551d1` covers 9. The work itself is complete and green; only the record
grouping differs.

The states between those scenarios were never committed, so a per-scenario split could only be
*reconstructed* by attributing hunks from the task records, producing commits that may never have built.
The operator was asked and chose to record the deviation rather than rewrite history, so this finding is a
recorded deferral, not an unresolved gap.

Tasks carried by each shared commit (task line, scenario):

| tasks.md line | scenario | commit |
|---|---|---|
| 191 | 6. All-day codes ignore printed clocks | d477233 |
| 206 | 7. Office or seminar duty uses its printed UTC window | d477233 |
| 221 | 8. Consulate visa duty uses its printed UTC window | d477233 |
| 236 | 9. EM duty uses its printed UTC window | d477233 |
| 251 | 10. Standby duty uses its own UTC window | d477233 |
| 266 | 11. Reserve duty uses its own UTC window | d477233 |
| 281 | 12. Consecutive standby days interrupted by a day off stay separate | d477233 |
| 296 | 13. Single-day standby uses its own window rather than a cancelled flight | d477233 |
| 311 | 14. Deadhead positioning travel keeps one event and marks the assignment in its title | d477233 |
| 326 | 15. A dated station-bearing layover row becomes an all-day activity | d477233 |
| 341 | 16. An unknown required activity code rejects the conversion | 75551d1 |
| 356 | 17. A required travel leg without a compatible partner rejects the conversion | 75551d1 |
| 371 | 18. An explicit next-day qualifier moves the endpoint to the following UTC day | 75551d1 |
| 386 | 19. A split across local dates can remain on the same UTC day | 75551d1 |
| 401 | 20. Unqualified endpoints that appear to cross midnight roll to the next UTC day | 75551d1 |
| 416 | 21. A zero-length travel interval is not repaired | 75551d1 |
| 431 | 22. A previous-year UTC qualifier resolves across the year boundary | 75551d1 |
| 446 | 23. A next-month UTC qualifier resolves across the month boundary | 75551d1 |
| 461 | 24. An explicit qualifier with no unique adjacent date rejects the conversion | 75551d1 |

## Pending before sign-off

1. `design.md` changed in this pass, so **design-review must be re-run** over it before `REVIEW: PASS`.
## Hard-to-test classifications (bucket 3)

Three regions cannot be reached by a scenario, with the concrete reason:

- `apps/cli/src/main.rs` "no unused calendar name is available" and the matching diagnostic line: the
  enumeration is bounded at 9 999 candidates, so reaching these needs ten thousand occupied names.
- `apps/cli/src/fs.rs` retry arm: it fires only when another process claims the name between the existence
  check and the create; a single-process suite cannot interleave a second writer, and the neighbouring
  failure arm (a name that only looks free) is covered.
- `crates/duty2cal-core/src/adapters/ics/mod.rs` newline and carriage-return arms: a PDF text cell cannot
  carry a raw newline or carriage return, so the sink's guard is defensive.
- `crates/duty2cal-core/src/domain/mod.rs` date accessor pattern lines: the sink calls `entry.date()` for
  every entry, so the arms execute in every travel, availability and non-flight scenario; LLVM marks the
  continuation lines of the multi-line `|` pattern uncovered, which no test can change.

## Coverage closure

Every gap the review opened with is classified and resolved:

- **Dead code (bucket 1), deleted**: the stationless-layover arm (the `Layover` code is set only when the
  marker named a station, so the check could never fail) and the redundant guard in the reader loop that
  made the following `else { continue }` unreachable.
- **Missing coverage (bucket 2), resolved**: thirty-one scenarios added across five batches, each proven RED
  by removing or weakening the branch it pins and observing exactly the new scenario fail (or, for shared
  branches, that scenario plus its siblings). They cover the producer-layout reading rules, the reader's
  date guards, the revision-footer arms, the missing required column, the unmatched arrival, a code-less
  row, both sides of a one-sided split-leg field, the tag escaping, the CLI usage, an unreadable source, the
  clock fallback, a non-textual source name, a long accented description, and the layover marker's station
  in the next column.
- **Hard to test (bucket 3)**: the bounded-enumeration exhaustion (10 000 occupied candidates), the
  filesystem retry arm that needs a second process to interleave, and the sink's raw newline and
  carriage-return guards, which a PDF text cell cannot produce. Reasons recorded above.
- **Attribution residue (11 lines)**: the merged report marks some lines uncovered although the behaviour is
  exercised and proven: the date accessor's multi-line `|` pattern, the clock fallback and the arrival-only
  description arm each have a RED-proving injection, and the compact revision-footer path is proven by the
  golden calendar's `DTSTAMP:20980521T155200Z`, which only that path can produce. No test can change how
  LLVM attributes those lines.

## Sign-off checklist

- Step 0 fabrication audit: clean, with the recorded commit-record deferral for two shared revisions.
- Runners: `./run-tests.sh` 75 scenarios (75 passed), 273 steps; `./run-tests.sh --e2e` 1 scenario
  (1 passed), 5 steps.
- Coverage measurement: fresh, instrumented, merged from both reports; no runner-only or test-only
  coverage.
- Arc42 implementation conformance: CLEAN (five chapters corrected this pass).
- Ubiquitous language: CLEAN (glossary rows for Travel leg and Availability duty; the spec uses EM duty).
- Interaction coverage: MATCH (one inventory entry, one matrix row, one `@e2e` scenario, the promised CLI
  subprocess).
- Visual review: PASS through the confirmed Persona.
- README impact: updated (Description, Quick start, Prerequisites, Usage, Contributing).

REVIEW: PASS
