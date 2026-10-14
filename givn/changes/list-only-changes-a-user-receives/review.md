# Review — list-only-changes-a-user-receives

Change: list-only-changes-a-user-receives. Reviewed against the approved proposal, the delta, `design.md`, `design-review.md`, `tasks.md`, the completed revisions and the running renderer. The delta holds three `@givn.added` scenarios and no `@e2e` scenario; the capability's two `@e2e` scenarios are permanent.

Receipts: `./run-tests.sh` → `3 features / 103 scenarios (103 passed) / 390 steps (390 passed)`; `./run-tests.sh --e2e` → `2 features / 3 scenarios (3 passed) / 16 steps (16 passed)`.

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | `git log -S"@e2e" -- givn/changes/list-only-changes-a-user-receives/specs/`; tag count in the delta | CLEAN — no commit ever carried `@e2e` into this delta; it holds three `@givn.added` scenarios and no `@e2e` tag, and the capability's two `@e2e` scenarios are untouched |
| 1 | Empty or no-op step bodies | grep for `unimplemented!`, `todo!()`, `{}`, bare `pass`/`return` across the step files | CLEAN — no match; each new `Given` builds a fixture repository and each new `Then` asserts on the section or the refusal transcript |
| 2 | Every `[x]` task has a commit touching production source | the four revisions recorded in `tasks.md` | CLEAN — `2712681` (cliff.toml, renderer, step file), `d569697` (README, CHANGELOG), `f303ada` (formatting, tasks). `295628a` touches no production source: its record states that the scenario it adds is a guard whose behaviour the code already produces, which is the intended disposition, not a stub |
| 3 | Every component `design.md` names exists | design.md's changed-files table against the tree | CLEAN — the marked list and the per-entry marker in `cliff.toml`, the filter and the list check in `scripts/render-changelog.sh`, the paths and scaffold revision in `build_history_with`, the re-rendered `CHANGELOG.md`, the runbook rule in `README.md`, the use-case rule |
| 4 | Strict-mode proof present with output | `tasks.md`, setup task | CLEAN — the RED runs exit non-zero (`Step doesn't match any function`, and two behavioural failures), and `.fail_on_skipped()` remains in `apps/cli/tests/features_runner.rs` |
| 5 | `@e2e` Then steps assert the real interface | the capability's e2e step file | CLEAN — the render `@e2e` scenario drives `scripts/render-changelog.sh` as a subprocess and reads the section it wrote; the preparation one drives `scripts/prepare-release.sh` and runs the archived executable |
| 6 | Driving mechanism is real, not in-process | the e2e step file | CLEAN — `std::process::Command` runs the scripts; no shell function is re-implemented in Rust |
| 7 | Exactly one `@e2e` implementation per capability | tree search including untracked files | CLEAN — one e2e step file per capability, bound by `verify.e2e_command` |
| 8 | E2E scope | the use case's inventory (two entries) against the feature | CLEAN — two entries, two `@e2e` scenarios; this change adds no consumer action and no `@e2e` scenario |
| 9 | `verify.command` vs `verify.e2e_command` | `givn/commands.yaml`: `./run-tests.sh` vs `./run-tests.sh --e2e`; observed counts | CLEAN — 103 scenarios vs 3 |
| 10 | Implementation matches what `design.md` names | marked line, marker, pipeline order, pinned `git` invocation, fixture builder | CLEAN — every one of them is realized as designed; the design's own correction (the pipeline order) is what the renderer implements |
| 11 | Local run command still works | `design.md` Local Runnability; `givn/commands.yaml` | CLEAN — `scripts/render-changelog.sh --version 0.2.0 --repo . --out CHANGELOG.md` runs against the real tree and writes the section the fixtures pin |
| 12 | Any finding here means unfinished implementation | — | CLEAN — no finding |
| 13 | Interaction coverage cross-reference | inventory ↔ matrix ↔ feature ↔ step file | see the table below |
| 14 | Coverage measurement validity | `measure_command`, then `merge_command`, fresh output | CLEAN — see the measurement note |

### Interaction coverage cross-reference

| Inventory entry (use case `## Interactions`) | Matrix row in `design.md` | `@e2e` scenario in the feature | Driving mechanism in the step file |
|---|---|---|---|
| `publish-public-release` — prepare the release for a version | "prepare the release for a version" → "Maintainer prepares a release and runs the archived executable", real interface CLI + git repository | `@e2e Scenario: Maintainer prepares a release and runs the archived executable` (permanent) | `std::process::Command` runs `scripts/prepare-release.sh` over a fixture binaries directory, then starts the executable taken out of the produced archive |
| `publish-public-release` — generate the change log for a version | "generate the change log for a version" → "Maintainer generates the change log for a version and reads its newest section", real interface CLI + git repository | `@e2e Scenario: Maintainer generates the change log for a version and reads its newest section` (permanent) | `std::process::Command` runs `scripts/render-changelog.sh` against a fixture repository and reads the section it wrote |

Two entries, two rows, two scenarios, two real drivers — no unmapped interaction and no matrix row without a scenario.

## Coverage measurement (check 14)

| Question | Finding |
|---|---|
| Runner and production processes mapped | `./run-tests.sh` → `cargo test --test features_runner`; the `@e2e` run additionally spawns the built `duty2cal` binary and the two scripts |
| Each process instrumented, collision-safe, flushed, merged | `./measure-coverage.sh` runs two distinct `cargo llvm-cov --workspace --test features_runner` invocations and `./merge-coverages.sh` combines them into `coverage/cobertura-coverage.xml` |
| A known exercised production path is non-zero | `crates/duty2cal-core/src/domain/mod.rs`: 86 of 89 lines covered, including `printed_identification` and `summary` |
| Not runner-only, test-only, absent or zero-production | VALID — production lines from all three crates; the change itself adds no instrumented line, because it changes a POSIX-shell script, a configuration file and documents, all outside the workspace (chapter 2 records the renderer as outside the workspace and off the coverage denominator) |
| Merged report freshly produced | VALID — `merged 7 files, 886 lines (871 covered) -> coverage/cobertura-coverage.xml`, written after the last code revision |

COVERAGE MEASUREMENT: VALID.

## Coverage classification — 15 uncovered production lines

| File | Lines | What the uncovered code is | Bucket | Disposition |
|---|---|---|---|---|
| `crates/duty2cal-pdf/src/lib.rs` | 288, 365, 383, 389, 407, 408 | the reader's guards for input the supported family cannot print | 3 | Pre-existing, owned by TD-007 |
| `crates/duty2cal-core/src/domain/mod.rs` | 184, 185, 186 | the exhaustive arms of the `date()` accessor | 3 | Pre-existing, owned by TD-007 |
| `apps/cli/src/fs.rs` | 47, 54 | the two hard-to-reach filesystem arms | 3 | Pre-existing, owned by TD-007 |
| `crates/duty2cal-core/src/adapters/ics/mod.rs` | 88, 89 | the sink's raw newline and carriage-return escapes | 3 | Pre-existing, owned by TD-007 |
| `apps/cli/src/main.rs` | 27 | the unparsable construction-clock fall-through | 3 | Pre-existing, owned by TD-007 |
| `crates/duty2cal-core/src/application/mod.rs` | 398 | the equal-values arm of `combine` | 2 | Pre-existing, held out by the previous change's approved out-of-scope boundary and owned by TD-007, whose plan is a scenario in the next conversion change |

This change introduces no uncovered line: the set is exactly the fifteen lines chapter 11 already records. One new line exists in this change and is deliberately not instrumented — the renderer's refusal for a revision whose changed paths cannot be read (`scripts/render-changelog.sh`) — because the renderer is a shell script outside the workspace; `design.md` records that arm as defensive, no fixture can reach it (a repository whose object is missing), and it is exercised by reading rather than by a scenario.

## Long-scenario split-or-keep

The lint gate reports no long scenario in this delta. Every delta scenario is short (five steps for the rule, three for the packaging guard, three for the path check) and keeps one invariant.

## Overlap dispositions

The lint gate reports no shape match involving this delta and no long scenario. The three scenarios assert different invariants — the filter's rule with a machinery-only revision and one that changed both, the packaging boundary that must stay announceable, and the marked list checked against the tree — and none was removed or replaced by this change, so there is no removed+added pair to explain.

## README impact decision

README-IMPACT: updated - Releasing a version (the authorship rule and the user-receiving rule)

The section was edited: it now states what a released subject must say and that engineering work takes a housekeeping type, and it names the filter. No clone address, platform, checksum, provenance or licence text changed, so `scripts/public-text-check.sh`'s promises are untouched.

## Arc42 implementation conformance

| Chapter | Verdict |
|---|---|
| 01 introduction-and-goals, 03 context-and-scope, 07 deployment, 09 decisions | MATCH — no claim of theirs changed; chapter 09 correctly holds no MADR for the not-qualifying decision, whose destination is `design.md` |
| 02 architecture-constraints | MATCH — the third marked list is described with its rationale and the line between the two kinds of tooling, and the config really carries it |
| 04 solution-strategy | MATCH — the release strategy's sentence states the filter and the path condition |
| 05 building-block-view | MATCH — the renderer's row names the path-list check, the resolution refusal and the drop; the `cliff.toml` row names the marked list and the per-entry marker |
| 06 runtime-view | MATCH — the pre-render check names the path list and step 4 states the drop |
| 08 crosscutting-concepts | MATCH — "What a reader sees" states the third condition and the diff-based resolution |
| 10 quality-requirements | MATCH — QS-011 names the machinery-only revision and the path rule |
| 11 risks-and-technical-debt | MATCH — TD-008 is recorded as partly closed with the authorship half and the list's judgement as the residual |
| 12 glossary | MATCH — **User-receiving path** is defined with its code symbol, and **Released change** and **Release text** carry the third condition |

ARC42 CONFORMANCE: CLEAN.

## Ubiquitous language conformance

| Term / wording | Finding |
|---|---|
| **User-receiving path** | Defined in the glossary, marked in `cliff.toml`, read by the renderer, and used with the same sense in the use case, the delta, the design and the runbook |
| **Released change** | Refined to three conditions in the glossary and matched by chapter 08 and the use-case rule |
| **Release text** | Refined to "filtered to the changes a user receives" |
| The retired phrasing | No artifact still describes visibility as a matter of words only: the use-case rule gained the third condition, and the delta, design, chapters 02/04/05/06/08/10 and the runbook agree |

UBIQUITOUS LANGUAGE: CLEAN.

## Visual review — terminal interface

- Contract: `design.md` declares `Interface: terminal`, no viewports, evidence kind "captured transcript per `@e2e` scenario".
- Evidence completeness: the delta adds no `@e2e` scenario, so no new transcript is owed; the change's text surface is the change log's newest section, which the capability's existing `@e2e` render scenario writes and the golden fixtures pin.
- Fresh-context review: one subagent, which wrote none of these artifacts, read the change log, the runbook, the configuration, the renderer's refusals, the delta and the use case through both confirmed Personas. It recorded **no baseline finding**, and three observations:

| Observation | Rule / Persona goal | Disposition |
|---|---|---|
| O1 — the version comparison link sits on the line directly after the fix entry, so a Markdown reader meets it as a lazy continuation of the bullet | the reader should meet what changed without ambiguity | Accepted, pre-existing and unchanged by this change: the section assembly glues the comparison with `printf` and `normalize` cannot insert a blank between non-blank lines; the same shape is in the committed file this change re-rendered. Not a baseline rule; recorded for a future change that touches the assembly |
| O2 — the refusal names the missing path and the configuration but no remedy | the refusal should state what happened without an internal identifier | Accepted: the rule is met, the maintainer can act on it, and every neighbouring refusal in the renderer reads the same way. Recorded, not changed here |
| O3 — the use case's grouping sentence omits the breaking-changes group that the configuration places first | the notes are grouped by kind | Accepted, pre-existing and untouched by this change; not exercised by the delta. Recorded for a future use-case edit |

Every observation is dispositioned; none names a pixel or a coordinate. The baseline holds: the newest section names one change a crew member can act on and no engineering work, earlier sections are byte-identical, the runbook rule is actionable, and no internal identifier appears in the refusals.

VISUAL-REVIEW: PASS

## Persona disposition

PERSONAS: project-maintainer--release-owner
PERSONAS: crew-member--personal-calendar-owner

| Persona | Surface | Finding |
|---|---|---|
| `project-maintainer--release-owner` | the runbook, the renderer's refusals, the task list | Reviewed above (O2, O3); the runbook states what to write and what to avoid, and every new refusal names what is wrong |
| `crew-member--personal-calendar-owner` | the change log's newest section | Reviewed above (O1); one entry, naming a change the crew member can act on |

## Interface classification — opposite test

Tested the opposite before accepting `terminal`: is this a Web UI? The capability renders a change log file, a release text and CLI output; nothing renders HTML, CSS or JavaScript, `givn/commands.yaml` declares a CLI product, and the `@e2e` scenarios drive shell commands as subprocesses. The fail-closed Web-UI rule cannot apply, so `terminal` holds and no browser driver is owed.

## Gate checks

- `./run-tests.sh` → 103 scenarios, 103 passed; `./run-tests.sh --e2e` → 3 scenarios, 3 passed.
- `cargo fmt --all --check` → clean; `cargo clippy --workspace --all-targets -- -D warnings` → clean; `GIVN_FEATURES=$(scripts/assemble-features.sh) cargo test --workspace` → 103 scenarios plus 3 architecture tests.
- `scripts/render-changelog.sh --version 0.2.0 --repo . --out CHANGELOG.md --check` → `change log is current for version 0.2.0`.
- `givn lint --change list-only-changes-a-user-receives` → clean; `scripts/pii-guard.sh` → clean.

## Sign-off

- [x] Fabrication audit complete and clean.
- [x] Every `[x]` task carries a revision; the guard revision states why it changes no production source.
- [x] Interaction coverage cross-reference complete: two entries, two rows, two `@e2e` scenarios, two real drivers.
- [x] Coverage measured, valid, and every gap classified; no gap introduced by this change.
- [x] Arc42 conformance clean; ubiquitous language clean.
- [x] Visual review complete for the changed text surface; every observation dispositioned.
- [x] README impact decided and the section edited.
- [x] Both runners green after the last revision.

REVIEW: PASS
