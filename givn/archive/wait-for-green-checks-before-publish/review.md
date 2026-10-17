# Review — wait-for-green-checks-before-publish

Change: wait-for-green-checks-before-publish. Reviewed against the approved proposal, the delta, `design.md`, `design-review.md`, `tasks.md`, the completed revisions and the running gate. The delta holds eight `@givn.added` scenarios and no `@e2e` scenario; the capability's two `@e2e` scenarios are permanent.

Receipts: `./run-tests.sh` → `3 features / 120 scenarios (120 passed) / 455 steps (455 passed)`; `./run-tests.sh --e2e` → `2 features / 3 scenarios (3 passed) / 16 steps (16 passed)`.

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | grep over the delta feature | CLEAN — no `@e2e` tag was added or removed; the two permanent scenarios are untouched |
| 1 | Empty or no-op step bodies | grep for `unimplemented!`, `todo!()`, `{}`, bare `return` across the step file and the gate script | 0 empty step bodies; every new `Given` writes a stub and fixtures, every `Then` asserts on the gate's exit status or transcript |
| 2 | Every `[x]` task has a commit touching production source | revisions in `tasks.md`: `5565920` (script, workflow, steps, docs), `a1e7daa` (evidence), `021785a` (message fix) | CLEAN — `5565920` touches `.github/workflows/release.yml`, `scripts/await-green-checks.sh` and the step file; the change is one production slice, so the scenarios share that revision and the task list says so |
| 3 | Every component `design.md` names exists | changed-files table against the tree | CLEAN — the gate script, the workflow step, the stub helpers and fixtures, the arc42 updates, and the unchanged preparation/renderer |
| 4 | Strict-mode proof present with output | `tasks.md` setup task; `features_runner.rs` | CLEAN — baseline `120 scenarios (112 passed, 8 failed)` with `Step doesn't match any function`; `.fail_on_skipped()` remains |
| 5 | `@e2e` Then steps assert the real interface | the capability's e2e step file | CLEAN — the render scenario drives `scripts/render-changelog.sh`; the preparation scenario drives `scripts/prepare-release.sh`, extracts the archive and runs the executable |
| 6 | Driving mechanism is real | e2e step file | CLEAN — `std::process::Command`; no script re-implemented in Rust |
| 7 | Exactly one `@e2e` implementation per capability | tree search | CLEAN — one e2e step file bound by `verify.e2e_command` |
| 8 | E2E scope | inventory against the feature | CLEAN — two entries, two permanent `@e2e` scenarios; the gate is automation, not a consumer action |
| 9 | `verify.command` vs `verify.e2e_command` | `givn/commands.yaml`: `./run-tests.sh` vs `./run-tests.sh --e2e`; counts | CLEAN — 120 vs 3 |
| 10 | Implementation matches the design | script interface, suite exclusion, workflow step, fixtures | CLEAN — the repeatable `--exclude-suite`, the pre-loop release-runs query, the 15 s/1800 s bound, the 40-minute backstop and the fixture matrix are realized as designed. The only post-review change is the message composition in `021785a`, which restores the design's quoted skipped/neutral counts |
| 11 | Coverage measurement validity | `./measure-coverage.sh`, `./merge-coverages.sh` | CLEAN — both profiles produced; `merged 7 files, 886 lines (871 covered)` |
| 12 | Any finding here means unfinished implementation | — | CLEAN — no finding |

### Interaction coverage cross-reference

| Inventory entry (use case `## Interactions`) | Matrix row in `design.md` | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| `publish-public-release` — prepare the release for a version | "prepare the release for a version" | `Maintainer prepares a release and runs the archived executable` (permanent) | `std::process::Command` runs `scripts/prepare-release.sh`, extracts with `tar`, runs the executable |
| `publish-public-release` — generate the change log for a version | "generate the change log for a version" | `Maintainer generates the change log for a version and reads its newest section` (permanent) | `std::process::Command` runs `scripts/render-changelog.sh` and reads the section |

Two entries, two rows, two scenarios, two real drivers. The gate adds no inventory row because it is invoked by the release automation, not by a consumer.

## Coverage measurement

| Question | Finding |
|---|---|
| Runner and production processes mapped | `./run-tests.sh` → `cargo test --test features_runner`; the `@e2e` run additionally spawns the built CLI and both scripts |
| Each process instrumented, collision-safe, flushed, merged | `./measure-coverage.sh` runs two distinct `cargo llvm-cov` invocations; `./merge-coverages.sh` combines them |
| A known exercised production path is non-zero | The merged report keeps the same production paths and counts |
| Not runner-only, test-only, absent or zero-production | VALID — production lines from all three crates; the change adds no production Rust line; the gate is shell outside the denominator |
| Merged report freshly produced | VALID — `merged 7 files, 886 lines (871 covered)` after the implementation |

COVERAGE MEASUREMENT: VALID.

## Coverage classification — 15 uncovered production lines

The change introduces no instrumented production line: the gate is a POSIX-shell script and the new Rust is test code. The uncovered set is exactly the fifteen lines chapter 11 records.

| File | Lines | What the uncovered code is | Bucket | Disposition |
|---|---|---|---|---|
| `crates/duty2cal-pdf/src/lib.rs` | 288, 365, 383, 389, 407, 408 | reader guards for input the supported family cannot print | 3 | Pre-existing, owned by TD-007 |
| `crates/duty2cal-core/src/domain/mod.rs` | 184, 185, 186 | exhaustive arms of the `date()` accessor | 3 | Pre-existing, owned by TD-007 |
| `apps/cli/src/fs.rs` | 47, 54 | two hard-to-reach filesystem arms | 3 | Pre-existing, owned by TD-007 |
| `crates/duty2cal-core/src/adapters/ics/mod.rs` | 88, 89 | sink raw newline and carriage-return escapes | 3 | Pre-existing, owned by TD-007 |
| `apps/cli/src/main.rs` | 27 | unparsable construction-clock fall-through | 3 | Pre-existing, owned by TD-007 |
| `crates/duty2cal-core/src/application/mod.rs` | 398 | equal-values arm of `combine` | 2 | Pre-existing, held out by the previous change's approved boundary and owned by TD-007 |

No new gap is introduced. The gate's decisions are exercised by the eight scenarios through a real subprocess and a stub `gh`; the live API and the job ordering remain host-only under the recorded 2026-10-13 operator boundary.

## Long-scenario split-or-keep

`givn lint` reports no long scenario. Each gate scenario keeps one invariant: waiting, immediate failure, the bound, skipped/neutral allowance, no success, unreadable source, no evidence, or two-suite waiting.

## Overlap dispositions

The lint gate reports no shape match. The waiting scenario (one suite) and the atomic-push scenario (two suites with a same-named check) are distinct: the second fails against an implementation that excludes only one suite. The failed, timeout, all-skipped, unreadable and no-evidence scenarios assert different refusal arms.

## README impact decision

README-IMPACT: none

The runbook is unchanged by design (D4): the documented atomic branch-and-tag push becomes safe because the gate waits. `scripts/public-text-check.sh` was not rerun because the README did not change.

## Arc42 implementation conformance

| Chapter | Verdict |
|---|---|
| 01 introduction-and-goals | MATCH — no claim changed; the new requirement is in chapter 10 |
| 02 architecture-constraints | MATCH — the drift-check rationale now states the waiting gate and the independent preparation check |
| 03 context-and-scope | MATCH — GitHub remains the external system |
| 04 solution-strategy | MATCH — no strategy change |
| 05 building-block-view | MATCH — the release toolbox lists `scripts/await-green-checks.sh`, its edges and the `release.yml` wait |
| 06 runtime-view | MATCH — the release scenario includes the gate and the refusal edge; steps renumbered |
| 07 deployment-view | MATCH — the publish node and runner row name the wait; no permission change |
| 08 crosscutting-concepts | MATCH — the publication gate paragraph states the bounded wait, exclusion, skipped/neutral allowance and fail-closed default |
| 09 architecture-decisions | MATCH — no ADR; the `NOT_QUALIFIED` verdict is recorded |
| 10 quality-requirements | MATCH — tree node and QS-015 |
| 11 risks-and-technical-debt | MATCH — TD-009 closed with the host-only and 1000-suite residuals |
| 12 glossary | MATCH — **Check run**, **Check suite**, **Publication gate**; **Gate (on landing)** refined |

ARC42 CONFORMANCE: CLEAN.

## Ubiquitous language conformance

| Term / wording | Finding |
|---|---|
| **Publication gate** | Defined, used in the rules, design, workflow, tests and chapter 08; the definition says “allows only when”, not “refuses until” |
| **Check run** / **Check suite** | Defined with status/conclusion and suite semantics; used consistently for the gate's input |
| **Gate (on landing)** | Refined to name the publication gate as its release-time counterpart |
| The five refusals | Match the use case: concluded non-success, no success, no evidence, unreadable source, bound; skipped/neutral allowed |
| Atomic push | The runbook wording remains valid and is now safe |

UBIQUITOUS LANGUAGE: CLEAN.

## Visual review — terminal interface

- Contract: `design.md` declares `Interface: terminal`, no viewports, evidence kind “none new”; no `@e2e` transcript is owed.
- Fresh-context review: a subagent that wrote none of these artifacts read the contract, the delta, the script's messages, the step assertions and the workflow position. It rendered the gate's real stdout/stderr through a temporary stub `gh` over nine states and returned **VISUAL-REVIEW: PASS** with four observations:

| Observation | Rule / Persona goal | Disposition |
|---|---|---|
| O1 — the progress and ready lines dropped the skipped/neutral count quoted in the design | maintainer can see the green set's composition | Fixed in `021785a`; the lines now print `<n> successful, <m> skipped or neutral`, and the skipped/neutral scenario asserts `1 successful, 2 skipped or neutral` |
| O2 — the all-skipped refusal names no individual checks | maintainer must know what blocked publication | Accepted: no check “failed”; the contract wording is exact, and the maintainer can open the checks page. No Persona goal is blocked |
| O3 — the ready-line composition was not pinned by an assertion | testability of the changed text | Fixed in `021785a` by the new assertion in `gate_reports_the_revision_ready` |
| O4 — `check run(s)` phrasing and unlabeled TSV rows before the refusal | baseline readability | Accepted as cosmetic; no rule or Persona goal is violated |

No observation is pixel-level. The baseline holds: check names/statuses are visible, the own-suite rows never appear, the bound is stated, and no token, host path or clock value enters the text.

VISUAL-REVIEW: PASS

## Persona disposition

PERSONAS: project-maintainer--release-owner
PERSONAS: crew-member--personal-calendar-owner

| Persona | Surface | Finding |
|---|---|---|
| `project-maintainer--release-owner` | the gate's progress and refusals | O1 fixed; failed and timeout arms name the checks; the bound is explicit; the atomic push needs no manual serialization |
| `crew-member--personal-calendar-owner` | the published release | A version becomes public only after every check outside the release suites concludes green; a refusal leaves no draft, asset or attestation |

## Interface classification — opposite test

Tested the opposite before accepting `terminal`: is this a Web UI? The gate writes progress to stdout and refusals to stderr in the Actions log; it renders no page, has no viewport, DOM, design system or screenshot, and the release page text is the unchanged staged notes file. The Web-UI classification does not hold.

## Deferrals

The live API, the runner's `gh` version and the job ordering are host-only. This is the 2026-10-13 operator boundary for the publication interface (no credential in the development environment; the host owns the interface), cited rather than a new deferral. The command's decisions are proven locally; the fail-closed default bounds the residual.

## Gate checks

- `./run-tests.sh` → 120 scenarios, 120 passed; `./run-tests.sh --e2e` → 3 scenarios, 3 passed.
- `cargo fmt --all --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean; `GIVN_FEATURES=$(scripts/assemble-features.sh) cargo test --workspace` exited 0.
- `actionlint 1.7.12` accepts `.github/workflows/release.yml`.
- `givn lint --change wait-for-green-checks-before-publish` clean; `scripts/pii-guard.sh` clean; `scripts/license-headers.sh` clean.
- `./measure-coverage.sh` and `./merge-coverages.sh` → `merged 7 files, 886 lines (871 covered)`.

## Sign-off

- [x] Fabrication audit complete and clean.
- [x] Every `[x]` task carries a revision; the shared production revision is recorded and explained.
- [x] Interaction coverage cross-reference complete: two entries, two rows, two `@e2e` scenarios, two real drivers.
- [x] Coverage measured, valid, and every gap classified; no gap introduced by this change.
- [x] Arc42 conformance clean; ubiquitous language clean.
- [x] Visual review complete for the terminal surface; every observation dispositioned.
- [x] README impact decided: none, by design.
- [x] Both runners green after the last revision.

REVIEW: PASS
