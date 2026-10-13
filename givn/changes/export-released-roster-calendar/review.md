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
- Fresh result: **885 production lines, 830 covered, 55 uncovered** across `crates/` and `apps/cli/src`
  (test-only paths excluded), produced by the merge command from the two fresh reports.

## Coverage classification (55 uncovered production lines)

Every gap is classified into exactly one bucket. There is **no dead code** to delete and **no
hard-to-test exception**: all 55 lines are **missing test coverage** (bucket 2), each named with the
scenario it needs.

| File | Lines | What the uncovered code is | Scenario it needs |
|---|---|---|---|
| `crates/duty2cal-core/src/application/mod.rs` | 136-138 | the required-column failure ("the detail table carries no {label} column") | a plan whose header omits a required label fails with that message and a numbered diagnostic |
| | 156 | an arrival fragment with no matching departure | an arrival-only row fails with "no unique compatible partner" |
| | 215-219 | a continuation row with no dated row above it | a plan whose first detail row prints no date fails, naming the reason |
| | 222-223 | an unreadable date cell | a row whose date cell cannot be read fails, naming the cell |
| | 269-274 | a `Layover:` marker whose station sits in the following column | a layover marker with the station in the next cell still yields the layover station |
| | 347-349 | a `Layover:` marker with no station at all | a stationless layover marker fails explicitly |
| | 381 | a row that prints in a body column but carries neither code nor route | such a row fails explicitly rather than being dropped |
| | 402 | a split leg where only the departure prints an optional field | a leg whose departure alone prints the timezone difference keeps that value |
| `crates/duty2cal-pdf/src/lib.rs` | 106, 132, 142 | the header-continuation exit, the gap branch, the unsupported-family rejection | a plan with no detail header fails as an unsupported family; material below the table is ignored |
| | 289-312 | the date-resolution guards (no heading month, no date band, unexpected token count, unparsable day, unknown weekday, weekday mismatch, invalid calendar date) | a date cell whose weekday contradicts the day is not accepted as timing; a plan with no heading month fails explicitly |
| | 334 | the joined month-and-year stamp (`January2099`) | a heading whose month touches its year still supplies the month |
| | 361, 379, 385, 394, 403-404 | the footer fallbacks (no `Created` stamp, ISO stamp, unparsable compact stamp) | a plan without a revision footer falls back to the construction clock; an ISO-stamped footer is read |
| `crates/duty2cal-core/src/adapters/ics/mod.rs` | 85-89, 125-126 | text escaping (`\\`, newline, carriage return) and the second fold boundary | a description carrying a backslash or newline escapes correctly; a long description folds twice |
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

## Pending before sign-off

1. `design.md` changed in this pass, so **design-review must be re-run** over it before `REVIEW: PASS`.
2. The 55 classified coverage gaps need their scenarios (RED, then GREEN, one commit each); they are named
   in the classification table above.

