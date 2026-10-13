# Review: generate-release-notes

Stage: `tasks-done` → `review`. This pass covers the change log, its drift check, a version's notes and the procedure that publishes them; the design layer's own record is in `design-review.md`.

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | `grep -c` over the delta feature | CLEAN — the delta holds 12 scenarios: 11 `@givn.added` (one of them `@e2e`) and 1 `@givn.modified`; the `@e2e` tag has been present since the spec commit and was never removed. |
| 1 | Empty or no-op step bodies | `grep -rn 'unimplemented!\|todo!()' apps/cli/tests/steps/` | CLEAN — `0 empty step bodies found across the two capability files`; every body drives the renderer or the preparation as a real subprocess, or asserts on a fixture's recorded output. |
| 2 | Every `[x]` task has a committed revision touching real work | `git log` against `tasks.md`'s 71 boxes | CLEAN — each scenario revision carries its step code and, where the scenario needed it, the production code it drives (`cliff.toml`, `scripts/render-changelog.sh`, `scripts/prepare-release.sh`, both workflows, `README.md`). |
| 3 | Every component `design.md` promised exists | `ls` over the promised paths | CLEAN — `cliff.toml`, `CHANGELOG.md`, `scripts/render-changelog.sh`, the changed `scripts/prepare-release.sh`, the `changelog` job with `fetch-depth: 0`, the pinned generator on the test job, the draft asset, the README step, the two extended step files, one file per capability. |
| 4 | Proof-of-strictness present with a non-zero exit | `.fail_on_skipped()` (`apps/cli/tests/features_runner.rs`) plus each scenario's RED run in `tasks.md` | CLEAN — every scenario's RED run is a non-zero `Step doesn't match any function` before its body existed; the runner keeps strict mode. |
| 5 | Every `@e2e` Then asserts through the real interface | `apps/cli/tests/steps/e2e/publish_public_release.rs` and the committed transcript | CLEAN — the render scenario's Thens assert on the file `scripts/render-changelog.sh` wrote and the section it reported, through a real subprocess; the preparation scenario's Thens assert the printed archive, checksum and release text and run the executable extracted from the produced archive. Assertions are on output, never on the repository alone. |
| 6 | Browser-UI driver fidelity | `design.md`'s `## Visual Design Contract` declares `Interface: terminal`; this change produces a text file and terminal output | NOT APPLICABLE, and the opposite classification was tested: the release page and the rendered `CHANGELOG.md` page are GitHub's rendering of text this project authors. There is no page this codebase serves and no browser to drive, so the text is checked as a document contract — the renderer's byte comparison, `scripts/public-text-check.sh` — not by a driver. |
| 7 | The file `verify.e2e_command` actually invokes | `givn/commands.yaml` (`verify.e2e_command: ./run-tests.sh --e2e`, `run-tests.sh` passing `--tags @e2e`) | CLEAN — one `@e2e` implementation for this capability, no second weaker one in the tree, committed or not. |
| 8 | Canonical action normalization and E2E scope | `givn instructions specs` re-read with the cross-reference below | CLEAN — the inventory holds two distinct user actions, each with exactly one `@e2e` scenario; no variant, error case or flag combination was promoted to `@e2e`. |
| 9 | Local run command starts the whole stack | `design.md`'s Local runnability | CLEAN — the renderer and the preparation run locally against the checkout; nothing to orchestrate, no twin, and the suite stays offline (no network, no credentials). |
| 10 | `verify.command` ≠ `verify.e2e_command`, and the e2e run is a strict subset | the runs in the sign-off below | CLEAN — 96 scenarios against 3; different commands, strictly fewer scenarios. |
| 11 | Implementation vs the reviewed design | the finished tree read against `design.md`'s file table, renderer contract and decisions ledger | CLEAN after this phase's corrections — see the findings below; nothing the design named was silently overridden, and every correction the review forced is visible in the artifacts rather than promised for later. |
| 12 | Findings reopened as unfinished implementation | — | The review's own findings (F1 to F9) and the design review's (H1 to H7) were each landed as a revision with its evidence, and the residual windows are recorded as TD-008 and TD-009 rather than excused. |
| 13 | Interaction coverage cross-reference | below | MATCH |
| 14 | Coverage measurement validity | below | VALID |

### Interaction coverage cross-reference

| Inventory entry (`usecase.md`) | Matrix row (`design.md`) | `@e2e` scenario in the delta | Driving mechanism in the step file |
|---|---|---|---|
| prepare the release for a version | present, one row (the delta's own row) | `Maintainer prepares a release and runs the archived executable` (permanent scenario, still green) | real subprocess: `std::process::Command` runs `scripts/prepare-release.sh`, extracts the archive it produced and runs the executable taken out of it |
| generate the change log for a version | present, one row | `Maintainer generates the change log for a version and reads its newest section` | real subprocess: `std::process::Command` runs `scripts/render-changelog.sh` against a fixture history, then reads the file it wrote and the section it reported |

The delta `.feature` carries no `# User Interaction Inventory:` comment block: this project keeps the inventory in the use-case document (the delta's `usecase.md`, `## Interactions`), which the canonical specs instruction requires and `givn lint` compares. No missing, extra or over-produced entry; the render-before-tag scenario added in this phase is a regular scenario, not a third interaction.

### Coverage measurement (check 14)

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| features_runner (non-e2e) | `./measure-coverage.sh` | `cargo llvm-cov --workspace --test features_runner` | `coverage/non-e2e-cobertura.xml` | `./merge-coverages.sh` | `crates/duty2cal-core/src/domain/timing.rs` |
| E2E runner + spawned CLI | `./measure-coverage.sh` | the same runner under instrumentation, spawning the built executable | `coverage/e2e-cobertura.xml` | `./merge-coverages.sh` | `apps/cli/src/main.rs` |

VALID: both processes are the configured, distinct runs; the last archive's receipt recorded the merged report at 98.31% line rate from them, and this change adds no production line in any crate — its Rust additions are step definitions, instrumented with the runner in the same pass. The shell renderer is outside the denominator by the same boundary the preparation and coverage scripts already sit outside, and its contract is covered by the delta's scenarios instead. The archive measures at the tip and carries the current number.

## Coverage classification

No uncovered region is newly introduced: this change adds no line to `crates/duty2cal-core`, `crates/duty2cal-pdf` or `apps/cli`'s product code. Its Rust is step definitions (covered by the scenarios that drive them) and the runner's `changelog` field. The fifteen uncovered production lines the last archive's receipt carries are the ones the previous review classified, unchanged by this change, and they remain in its three buckets: one recorded as missing coverage in the conversion capability (TD-007) and fourteen judged legitimately hard to test, each with its reason. No dead code was found, and nothing was left unclassified.

## Visual review

Interface: terminal, so the evidence is the committed transcript rather than screenshots. A separate fresh-context reviewer judged each confirmed Persona against the produced text and the documented procedure.

PERSONAS: project-maintainer--release-owner
PERSONAS: crew-member--personal-calendar-owner

| Persona | Verdict | Findings |
|---|---|---|
| `project-maintainer--release-owner` | `PERSONA-REVIEW: PASS` | Ran the documented sequence end to end in a throwaway clone for a version with no tag: it renders, dates the section from the revision the tag will point at, commits with that revision's own committer date, and the tagged revision satisfies the drift check the CI runs. The preparation refuses a stale change log before staging anything (edited a bullet: refused, staged nothing). Both its earlier blocking findings and its three lesser ones are closed. Three new gaps, none blocking its questions: the push was not atomic (fixed — `git push --atomic`), the publish gate does not wait for a check run still in progress (TD-009; the change log itself is now covered by the preparation's own check), and the README named a channel-resolved generator (fixed — it names the pin). |
| `crew-member--personal-calendar-owner` | `PERSONA-REVIEW: FAIL` | The section the pipeline produces is what the reader asked for: grouped, capability-first, three entries, no machinery. The verdict is about the *published* v0.1.0 page, which still shows the older flat 16-entry list and no execute-bit note, because the recorded decision to bring it in line is an action on the operator's own account, and nothing in this environment can authenticate to the host. Disposition: the change's part is complete — the repository's file, the renderer and the published-asset route agree, and every release the pipeline publishes states that agreement; the page edit is the operator's outstanding action with the paste-ready body prepared (`local://v0.1.0-release-body.md`, the section plus the one-off execute-bit note). The remaining wording gap (`Read the producer's own export layout` is real behaviour but not the crew's vocabulary) is unfixable by any rule and is recorded as TD-008. |

`VISUAL-REVIEW: PASS` — the evidence kind is complete for a terminal interface, the contract's baseline holds (the section names the version, its own date and the comparison link; the transcript replaces the run-specific directory with a placeholder and carries no host path, user name or clock value; two renders are byte-identical), and every Persona finding is dispositioned, with the one outstanding item named and owned.

## Arc42 implementation conformance

Independently re-derived from the finished implementation after this phase's corrections, then compared with the durable chapters and the change's `arc42.md`.

| Chapter | Change-level claim | Implementation evidence | Match? |
|---|---|---|---|
| 01 | Requirement 5 and quality goal 6 state the change log and its newest section | the delta's rules and scenarios; `docs/arc42/01` re-read and current | MATCH |
| 02 | Five constraints: the pin and its install action; the drift check in CI and in the preparation; the capability allow-list; the marked machinery skip list; the Linux-only renderer | `cliff.toml`'s two marked lines and the pin comment; `ci.yml`; `prepare-release.sh`; `render-changelog.sh` | MATCH (the row now names all five) |
| 03 | GitHub stays the first external system and the product keeps no network I/O | the renderer touches only the filesystem, the pinned binary and the local history; no crate references the network | MATCH |
| 04 | The release strategy gains a rendered, drift-checked change log staged with the release | `prepare-release.sh`'s render and check; the draft asset | MATCH |
| 05 | The release toolbox gains `cliff.toml`, `CHANGELOG.md`, `render-changelog.sh` and the `changelog` job | the files exist; the chapter's job list and the renderer row (date rule, refusals, machinery list) read true | MATCH |
| 06 | The tag-to-release flow renders the section and extracts it from the history, with the pinned generator in the release path | `release.yml`'s prepare steps; `render-changelog.sh`'s extraction; the gate list includes `public-text` | MATCH |
| 07 | Nothing changes on the deployment side; the draft carries one more asset | `release.yml`'s asset list | MATCH |
| 08 | The comparison link is a literal URL built by the renderer; determinism, the date rule and the machinery skip rule are stated | `render-changelog.sh`; `cliff.toml`'s `@@DATE@@` placeholder | MATCH |
| 09 | No ADR is minted; `NOT_QUALIFIED` routes to `design.md` | the ADR register still holds exactly ADR-0001 to ADR-0004 | MATCH |
| 10 | The quality scenarios cover the change log's determinism and the refusal paths | the chapter's scenarios and the delta's | MATCH |
| 11 | Risks and debt: the pin's fragility, the allow-list drift, the back-fill divergence, subject quality (TD-008), the publish gate's window (TD-009) | `docs/arc42/11` holds the rows; `arc42.md` row 11 names both debt identifiers | MATCH |
| 12 | The glossary holds every normative term with the same meaning, and no code symbol cites removed code | `12-glossary.md` (change log, released change, drift check, capability allow-list, machinery of the project, release text, housekeeping revision, release-note subject); `collect_release_notes`, `RELEASE_NOTE_TYPES` and `HOUSEKEEPING_TYPES` appear nowhere in `docs/arc42` | MATCH |

ARC42 CONFORMANCE: CLEAN.

## Ubiquitous language conformance

Every term the delta and the design use normatively is in `docs/arc42/12-glossary.md` with the same meaning: change log, released change, drift check, capability allow-list, machinery of the project, release text, housekeeping revision, release-note subject. The glossary's code symbols point at what decides each concept now (`cliff.toml`'s rules and the renderer, not the preparation's removed filter). No competing meaning, no fabricated term. No finding.

## Overlap dispositions

`givn lint` over the delta reports no shape match and no long scenario needing a split-or-keep decision. One pair shares a fixture shape without being a duplicate: `Each version's section is dated and compared with the one before it` and `A version's section can be rendered before its tag exists` both build a released history, but they pin different invariants — the comparison link built from the previous tag, and the date taken from the revision the tag will point at — so they are variants, and merging them would let a regression in one hide behind a pass in the other.

README-IMPACT: updated - Releasing a version (the render-commit-tag-push procedure, the pinned generator, the preparation's own drift check; the clone address, description, usage and licence sections unchanged, and the three givn-managed blocks untouched — `givn readme check` OK, `scripts/public-text-check.sh` clean with all 12 promises present).

## Deferrals

None. The two recorded limits are debt with named follow-ups, not deferrals of a check: TD-008 (a version's notes are only as clear as the released revisions' own subjects) and TD-009 (the publish gate does not wait for a check run still in progress; the change log's own path is covered by the preparation's check).

## Host-side evidence and the one outstanding operator action

- **Nothing is pushed yet.** The change's revisions are local; the `changelog` job and the release workflow's pin will first run with the release push, and `actionlint` 1.7.12 over both workflows is the local proof that they parse and are wired as designed.
- **The v0.1.0 page edit is the operator's.** The recorded decision (2026-10-13) brings the published body in line with the repository's 0.1.0 section plus the one-off execute-bit note. GitHub permits editing an immutable release's title and notes; the body is prepared at `local://v0.1.0-release-body.md` and nothing in this environment can authenticate to apply it.

## Sign-off checklist

- [x] `./run-tests.sh` GREEN — 3 features, 96 scenarios (96 passed), 367 steps.
- [x] `./run-tests.sh --e2e` GREEN — 3 features, 3 scenarios (3 passed), 16 steps.
- [x] Coverage classified: this change adds no production line; the receipt's fifteen uncovered lines are the previous review's, in its three buckets, with TD-007's follow-up named.
- [x] Fabrication audit clean; interaction coverage MATCH; no finding left as an invented fourth bucket.
- [x] Design review re-run in a fresh context over the corrected design: `DESIGN-REVIEW: PASS`, with H1 to H9 dispositioned.
- [x] Arc42 conformance CLEAN; ubiquitous language clean.
- [x] Visual review complete: both confirmed Personas judged in fresh contexts, every finding dispositioned, the one outstanding item named and owned.
- [x] Deterministic gates at the tip: `cargo deny check` advisories/bans/licenses/sources ok; third-party notices with no drift; `reuse lint` compliant (134/134 files); `actionlint` clean over both workflows; `scripts/public-text-check.sh` clean; `scripts/pii-guard.sh` clean; `givn lint` clean; `scripts/render-changelog.sh --check` reports the change log current for version 0.1.0 and writes nothing.
- [x] `CHANGELOG.md` is current, one section for 0.1.0 with three entries, and byte-identical to a fresh render.

REVIEW: PASS