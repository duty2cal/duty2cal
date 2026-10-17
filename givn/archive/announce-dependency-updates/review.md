# Review — announce-dependency-updates

Change: announce-dependency-updates. Reviewed against the approved proposal, the delta, `design.md`, `design-review.md`, `tasks.md`, the completed revisions and the running renderer. The delta holds nine `@givn` scenarios and no `@e2e` scenario; the capability's two `@e2e` scenarios are permanent.

Receipts: `./run-tests.sh` → `3 features / 112 scenarios (112 passed) / 427 steps (427 passed)`; `./run-tests.sh --e2e` → `2 features / 3 scenarios (3 passed) / 16 steps (16 passed)`.

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | `grep -n '@e2e'` over the delta feature; the permanent feature | CLEAN — the delta contains no `@e2e` tag and no commit added or removed one; the capability's two permanent `@e2e` scenarios are untouched |
| 1 | Empty or no-op step bodies | grep for `unimplemented!`, `todo!()`, `{}`, bare `return` across the step files | 0 empty step bodies found; each new `Given` builds a fixture repository and each new `Then` asserts a section, group, count or refusal |
| 2 | Every `[x]` task has a commit touching production source | revisions in `tasks.md`: `e2a6efd` (production plus tests), `3dc80c4` and `21fd4c2` (evidence and task boundary) | CLEAN — `e2a6efd` touches `cliff.toml`, `scripts/render-changelog.sh`, the step file and `README.md`; the change is one atomic production slice, so its scenarios share that revision, and the task list says so rather than pretending nine independent production commits |
| 3 | Every component `design.md` names exists | design's changed-files table against the tree | CLEAN — the parser row exists in `cliff.toml`, the empty-heading prune in `scripts/render-changelog.sh`, the fixtures and steps in `apps/cli/tests/steps/publish_public_release.rs`, the runbook paragraph in `README.md`, and the glossary terms |
| 4 | Strict-mode proof present with output | `tasks.md` setup task; `features_runner.rs` | CLEAN — baseline `112 scenarios (103 passed, 9 failed)` with `Step doesn't match any function`; `.fail_on_skipped()` remains |
| 5 | `@e2e` Then steps assert the real interface | the capability's e2e step file | CLEAN — the render scenario drives `scripts/render-changelog.sh` as a subprocess and reads the section it wrote; the preparation scenario drives `scripts/prepare-release.sh`, extracts the archive with `tar` and runs the executable |
| 6 | Driving mechanism is real, not in-process | the e2e step file | CLEAN — `std::process::Command`; no script is re-implemented in Rust |
| 7 | Exactly one `@e2e` implementation per capability | tree search | CLEAN — one e2e step file, `apps/cli/tests/steps/e2e/publish_public_release.rs`, bound by `verify.e2e_command` |
| 8 | E2E scope | use-case inventory (two entries) against the feature | CLEAN — two entries, two permanent `@e2e` scenarios; this change adds no consumer action and no `@e2e` scenario |
| 9 | `verify.command` vs `verify.e2e_command` | `givn/commands.yaml`: `./run-tests.sh` vs `./run-tests.sh --e2e`; observed counts | CLEAN — 112 scenarios vs 3; strictly scoped |
| 10 | Implementation matches what `design.md` names | parser row, prune, fixture paths, step file, runbook | CLEAN — every named mechanism is realized. The one correction (the repeated-subject fixture writes `Cargo.lock` then `Cargo.toml` so the second revision is a real change) was made in `design.md` before implementation, not silently in code |
| 11 | Coverage measurement validity | `./measure-coverage.sh`, then `./merge-coverages.sh` | CLEAN — both profiles produced; `merged 7 files, 886 lines (871 covered)` |
| 12 | Any finding here means unfinished implementation | — | CLEAN — no finding |

### Interaction coverage cross-reference

| Inventory entry (use case `## Interactions`) | Matrix row in `design.md` | `@e2e` scenario in the feature | Driving mechanism in the step file |
|---|---|---|---|
| `publish-public-release` — prepare the release for a version | "prepare the release for a version" → "Maintainer prepares a release and runs the archived executable", CLI | `@e2e Scenario: Maintainer prepares a release and runs the archived executable` (permanent) | `std::process::Command` runs `scripts/prepare-release.sh` over a fixture binaries directory, extracts the archive with `tar`, and starts the executable |
| `publish-public-release` — generate the change log for a version | "generate the change log for a version" → "Maintainer generates the change log for a version and reads its newest section", CLI | `@e2e Scenario: Maintainer generates the change log for a version and reads its newest section` (permanent) | `std::process::Command` runs `scripts/render-changelog.sh` against a fixture repository and reads the section it wrote |

Two entries, two rows, two scenarios, two real drivers — no unmapped interaction and no matrix row without a scenario.

## Coverage measurement

| Question | Finding |
|---|---|
| Runner and production processes mapped | `./run-tests.sh` → `cargo test --test features_runner`; the `@e2e` run additionally spawns the built `duty2cal` binary and both scripts |
| Each process instrumented, collision-safe, flushed, merged | `./measure-coverage.sh` runs two distinct `cargo llvm-cov --workspace --test features_runner` invocations; `./merge-coverages.sh` combines them into `coverage/cobertura-coverage.xml` |
| A known exercised production path is non-zero | The merged report keeps the same production paths and counts; the change adds no production Rust line |
| Not runner-only, test-only, absent or zero-production | VALID — production lines from all three crates; the change alters a POSIX-shell script, a configuration file, tests and documents, all outside the workspace or outside the production denominator |
| Merged report freshly produced | VALID — `merged 7 files, 886 lines (871 covered)`, written after the implementation revision |

COVERAGE MEASUREMENT: VALID.

## Coverage classification — 15 uncovered production lines

The change introduces no instrumented production line: the parser row and the prune are configuration/shell outside the workspace, and the new Rust is test code. The uncovered set is exactly the fifteen lines chapter 11 already records.

| File | Lines | What the uncovered code is | Bucket | Disposition |
|---|---|---|---|---|
| `crates/duty2cal-pdf/src/lib.rs` | 288, 365, 383, 389, 407, 408 | reader guards for input the supported family cannot print | 3 | Pre-existing, owned by TD-007 |
| `crates/duty2cal-core/src/domain/mod.rs` | 184, 185, 186 | exhaustive arms of the `date()` accessor | 3 | Pre-existing, owned by TD-007 |
| `apps/cli/src/fs.rs` | 47, 54 | two hard-to-reach filesystem arms | 3 | Pre-existing, owned by TD-007 |
| `crates/duty2cal-core/src/adapters/ics/mod.rs` | 88, 89 | sink raw newline and carriage-return escapes | 3 | Pre-existing, owned by TD-007 |
| `apps/cli/src/main.rs` | 27 | unparsable construction-clock fall-through | 3 | Pre-existing, owned by TD-007 |
| `crates/duty2cal-core/src/application/mod.rs` | 398 | equal-values arm of `combine` | 2 | Pre-existing, held out by the previous change's approved boundary and owned by TD-007 |

No new gap is introduced by this change; the renderer's prune is exercised by the delta's checks-only scenario (the empty heading is absent) and by every scenario that renders a non-empty group.

## Long-scenario split-or-keep

`givn lint` reports no long scenario in this delta. Each scenario keeps one invariant: the dependency-only announcement, the collapse, the preparation, the order, the checks-only exclusion, the release-workflow inclusion, the refusal, the capability-scope single entry, or the release-text equality.

## Overlap dispositions

The lint gate reports no shape match involving this delta. The nine scenarios assert different invariants; the modified refusal scenario replaces its permanent title by design, and the new checks-only scenario is a distinct path-filter case, not a duplicate.

## README impact decision

README-IMPACT: updated - Releasing a version

The section was edited: the blanket “never released at all” now names the `chore(deps):` exception, the user-receiving path and machinery conditions, the dependency-only releasability, the **Dependencies** group and the checks-only exclusion. `scripts/public-text-check.sh` remains clean (12 promises present).

## Arc42 implementation conformance

| Chapter | Verdict |
|---|---|
| 01 introduction-and-goals | MATCH — the release goal and quality goal name dependency updates |
| 02 architecture-constraints | MATCH — the allow-list row decides capability scopes; the dependency parser sits below it |
| 03 context-and-scope | MATCH — the release-page text names capability changes and dependency updates |
| 04 solution-strategy | MATCH — the release strategy names Dependencies and the path filter |
| 05 building-block-view | MATCH — the `cliff.toml` row names the parser/group and the renderer row names the empty-group prune |
| 06 runtime-view | MATCH — the prepare refusal, the group list, the housekeeping exception and the prune |
| 07 deployment-view | MATCH — no change; the same five-platform preparation and publication path |
| 08 crosscutting-concepts | MATCH — the released-change definition, group order and absent emptied group |
| 09 architecture-decisions | MATCH — no ADR qualifies; chapter 9 and both ADR indexes are unchanged |
| 10 quality-requirements | MATCH — the quality-tree node and QS-014 |
| 11 risks-and-technical-debt | MATCH — R-018, TD-011 and TD-012 (the prune's non-final limitation) |
| 12 glossary | MATCH — **Dependency update**, **Housekeeping revision**, **Released change**, **Release preparation** and **Release text** |

ARC42 CONFORMANCE: CLEAN.

## Ubiquitous language conformance

| Term / wording | Finding |
|---|---|
| **Dependency update** | Defined in the glossary and used with the same sense in the use case, design, runbook and tests |
| **Housekeeping revision** | The exception for `chore(deps)` is stated in the glossary and the use case |
| **Released change** | Covers the capability-scoped revision and the dependency update that changed a user-receiving path; matched by chapter 08 and the use case |
| **Release text** | Names the Dependencies group and the user-receiving filter |
| **Release preparation** | Refuses a version whose section names nothing at all — no user-receiving change and no dependency update |
| The retired wording | The renderer's legacy message remains “changes nothing a user can see”; the use case explains it as the empty-version refusal and no artifact now promises that a dependency-only version is unreleasable |

UBIQUITOUS LANGUAGE: CLEAN.

## Visual review — terminal interface

- Contract: `design.md` declares `Interface: terminal`, no viewports, evidence kind "none new" because no `@e2e` scenario is added.
- Evidence completeness: no new `@e2e` transcript is owed; the change's text is evidenced at assertion level by the nine scenarios (heading, bullet words, order, once-only, absent empty heading, release-text equality).
- Fresh-context review: one subagent, which wrote none of these artifacts, read the design contract, the delta, the use case, the runbook, the new step assertions and both archived terminal transcripts. It recorded **VISUAL-REVIEW: PASS** and three observations:

| Observation | Rule / Persona goal | Disposition |
|---|---|---|
| O1 — the empty-group prune matches a following `## ` heading or the section end, not a following `### ` group heading; an emptied non-final group would keep its heading | the crew member should not meet an empty group | Accepted for this change: Dependencies is the last parser group, so its emptied heading is always at the section end and the checks-only scenario asserts its absence. Recorded as TD-012: widen the prune to any following heading before adding a group below Dependencies |
| O2 — the runbook's condition omitted “and its words do not name the project's own tooling” | the maintainer should know which bump stays unannounced | Fixed: the README paragraph now carries the machinery condition |
| O3 — the archived transcripts predate this change and show version 0.1.0 with no Dependencies group | the evidence should be current | Accepted: the contract declares “no new evidence” because no `@e2e` scenario is added; the new text is asserted at the scenario level, and the permanent transcripts remain the terminal baseline |

No observation is pixel-level or coordinate-based. The baseline holds: the notes name the dependency updates in words a reader can act on, the emptied group is absent from the delivered surface, and the runbook states the exception and its limits.

VISUAL-REVIEW: PASS

## Persona disposition

PERSONAS: project-maintainer--release-owner
PERSONAS: crew-member--personal-calendar-owner

| Persona | Surface | Finding |
|---|---|---|
| `project-maintainer--release-owner` | the runbook, the preparation/render output, the task list | O2 fixed; the runbook now states the user-receiving path and machinery conditions, the dependency-only releasability and the checks-only exclusion |
| `crew-member--personal-calendar-owner` | the change log's newest section and the release text | O1 accepted for the delivered surface and recorded as TD-012; the Dependencies heading and bullets are exact, ordered, collapse to one entry and disappear when empty |

## Interface classification — opposite test

Tested the opposite before accepting `terminal`: is this a Web UI? The capability produces a change log file, a release text and terminal output from shell commands; nothing renders HTML, CSS or JavaScript, `givn/commands.yaml` declares a CLI product, and the `@e2e` scenarios drive subprocesses. The fail-closed Web-UI rule cannot apply, so `terminal` holds and no browser driver is owed.

## Gate checks

- `./run-tests.sh` → 112 scenarios, 112 passed; `./run-tests.sh --e2e` → 3 scenarios, 3 passed.
- `cargo fmt --all --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean; `GIVN_FEATURES=$(scripts/assemble-features.sh) cargo test --workspace` exited 0.
- `scripts/render-changelog.sh --version 0.2.0 --repo . --out CHANGELOG.md --check` → `change log is current for version 0.2.0`.
- `givn lint --change announce-dependency-updates` clean; `scripts/pii-guard.sh` clean; `scripts/public-text-check.sh` clean.
- `./measure-coverage.sh` and `./merge-coverages.sh` → `merged 7 files, 886 lines (871 covered)`.

## Sign-off

- [x] Fabrication audit complete and clean.
- [x] Every `[x]` task carries a revision; the shared production revision is recorded and explained.
- [x] Interaction coverage cross-reference complete: two entries, two rows, two `@e2e` scenarios, two real drivers.
- [x] Coverage measured, valid, and every gap classified; no gap introduced by this change.
- [x] Arc42 conformance clean; ubiquitous language clean.
- [x] Visual review complete for the terminal surface; every observation dispositioned.
- [x] README impact decided and the section edited.
- [x] Both runners green after the last revision.

REVIEW: PASS
