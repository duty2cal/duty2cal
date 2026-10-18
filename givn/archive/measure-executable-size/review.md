# Review: measure-executable-size

- **Change:** `measure-executable-size`
- **Date:** 2026-10-18
- **Evidence:** the full suite passed 160/160 scenarios in 699 steps; the e2e
  scope passed 9/9 scenarios in 70 steps. The task list records each
  scenario's RED/GREEN/REFACTOR output and revision. `givn check review`
  exits 0; `givn lint` is clean.

## Step 0 — Fabrication audit

1. **`@e2e` tag integrity.** The delta has one `@e2e` scenario
   (`Maintainer requests an attribution for a revision and reads the report`);
   the tag was never removed. The tag is not in tasks.md as a removable item.
2. **Empty step bodies.** `apps/cli/tests/steps/measure_distributed_executable_size.rs`
   and `apps/cli/tests/steps/e2e/measure_distributed_executable_size.rs` were
   grepped for `{}`, `() {}`, bare `pass`/`return`, `todo!()`, and
   `unimplemented!()`: 0 empty bodies. The `{}` hits are format placeholders
   in assertion messages and step names.
3. **Checked tasks have revisions touching production source.** Scenario
   revisions `7cbc9ba`, `441c0a6`, `666cbe8`, `0dbb11a`, `cf3f1bb`, `2e22568`,
   `5486a84`, `00d40b3` touch `scripts/measure_executable_size.py`; `2e22568`
   also touches the step file. The e2e revision `bf24188` touches the e2e step
   file and the committed transcript; the real command it drives is the script
   implemented in the earlier revisions. The wiring revision `1b06c6d` touches
   `ci.yml`, `cliff.toml` and `README.md`. No checked scenario is a spec-only
   change. `refactor` revision `d44185b` deletes two unused constants found in
   this review.
4. **Promised components exist.**
   - `scripts/measure_executable_size.py`
   - `apps/cli/tests/steps/measure_distributed_executable_size.rs`
   - `apps/cli/tests/steps/e2e/measure_distributed_executable_size.rs`
   - `apps/cli/tests/fixtures/size-attribution/**`
   - `apps/cli/tests/features_runner.rs` fields and module lines
   - `.github/workflows/ci.yml`, `cliff.toml`, `README.md`
   - `givn/specs/measure-distributed-executable-size/usecase.md`
5. **Strict-mode proof.** `tasks.md` records `probe-exit=101`,
   `✘ And a deliberately undefined step`, `Step doesn't match any function`,
   then the probe removal and a green re-run. The e2e runner is the same
   `features_runner` binary with `.fail_on_skipped()`.
6. **`@e2e` Then assertions.** The Then steps read the command's own
   `attribution.md` and `attribution.json` (via the Markdown labels), assert the
   requested and resolved revision, platform, build identity, concrete crate
   and function, the fixture executable's byte count, `total < file size`, and
   the attribution build's file size > the stripped file size. They are not
   repository-only assertions. The command line, report and exit status are the
   named real interface.
7. **Browser driver.** Not applicable: the interface is a terminal command and
   a Markdown/JSON report; no HTML/CSS/JS is rendered in a browser, so no
   browser driver applies.
8. **`verify.e2e_command` files.** `givn/commands.yaml` sets
   `verify.command: "./run-tests.sh"` and
   `verify.e2e_command: "./run-tests.sh --e2e"` (distinct strings).
   `./run-tests.sh --e2e` runs
   `cargo test -p duty2cal --test features_runner -- --tags @e2e` after
   `scripts/assemble-features.sh`. The e2e steps are
   `apps/cli/tests/steps/e2e/measure_distributed_executable_size.rs`; a tree
   search for `measure_executable_size.py` in `apps/cli/tests/steps` finds only
   the regular step file, and no parallel e2e implementation exists.
9. **E2E scope.** The use case has one inventory entry, "request the size
   attribution for a named revision"; the design has one matrix row; the delta
   has one matching `@e2e` scenario. The other eight scenarios are variants and
   invariants (earlier revision, build failures, refusals, incomplete, sections,
   unresolved ref). No extra e2e scenario. The e2e scope count (9) is strictly
   less than the full count (160).
10. **Design conformance.** The script path and flags, the two step files, the
    report labels and statuses, the evidence-only CI step (`set +e`, summary
    append, `exit 0`, `if: always()` upload), the `cliff.toml` allow-list/parser
    rows, the README section, and the fixture layout match `design.md`. The
    fixture's `lto = false` is the reviewed fallback recorded in `design.md`
    (LTO internalised `fixture_support::fixture_alpha`; `-C link-dead-code` did
    not restore it).
11. **Interaction coverage cross-reference.**

| Use-case inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| request the size attribution for a named revision | yes | Maintainer requests an attribution for a revision and reads the report | `std::process::Command` runs `python3 scripts/measure_executable_size.py --repo <fixture repo> --revision v0.3.0 --out <dir>`; reads `attribution.md` labels/tables and exit status |

No missing or extra rows.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Per-revision attribution, two builds, two numbers, pinned generator | `02-architecture-constraints.md:24-25` | Yes | Technology decisions, two builds | Setup; all scenarios | `scripts/measure_executable_size.py`; `ci.yml` measurement step | yes |
| Measurement command, report files, step files, fixture | `05-building-block-view.md:37,64,66,70` | Yes | Architecture impact, changed files | Setup; scenarios 1-9 | script, fixtures, both step files | yes |
| Worktree, shipped build, name-retaining build, requested/resolved, evidence-only | `06-runtime-view.md:174-201` | Yes | The measurement command; CI wiring | Scenarios 1-9; wiring | script; `ci.yml` step exits 0 and uploads `if: always()` | yes |
| Attribution beside the run; suite jobs install the toolchain | `07-deployment-view.md:10-11,29,51-53` | Yes | CI wiring; Local runnability | Wiring; e2e setup | `ci.yml` `windows-gnu-build`, `test`, `coverage` | yes |
| Two-build/two-number/refusal/estimate/evidence-not-a-gate | `08-crosscutting-concepts.md:156-186` | Yes | Report schema; Markdown contract; risks | Scenarios 2-8 | script statuses and labels; R-034 | yes |
| QS-022 quality scenario | `10-quality-requirements.md:29,63` | Yes | Traceability | Scenarios 1-9 | full and e2e suite | yes |
| R-032 GNU-PE spike premise, R-033 supplied artifact, R-034 evidence-not-a-gate, TD-014 | `11-risks-and-technical-debt.md:38-40,58` | Yes | Risks and follow-ups | Risks; review | design risk; implementation; evidence-only CI | yes |
| Size attribution and report terms | `12-glossary.md:86-87` | Yes | Report schema; Markdown contract | Scenarios 1-9 | labels and JSON fields | yes |
| Stakeholder segment | `01-introduction-and-goals.md:38` | Yes | Persona impact | — | design/review | yes |
| Size attribution technology and release integrity | `04-solution-strategy.md:25,41` | Yes | Technology decisions | — | script; CI | yes |
| No new external system; build tooling not a product interface | `03-context-and-scope.md` | No | Architecture impact | — | GitHub remains the only external system | yes |
| No qualified ADR | `09-architecture-decisions.md:11-16` | No | Decisions | — | no new ADR file/register row | yes |

`ARC42 CONFORMANCE: CLEAN` — every chapter marked affected by either
assessment carries the current fact; chapters 3 and 9 stay No; no stale
contradiction or missing task mapping.

## Ubiquitous language conformance

- **Size attribution** — the glossary's contrast is "not a byte-exact file
  size" (the generator's numbers are estimates of `.text`); the `Where` column
  names both the `--crates` and functions runs. The design, spec, report labels
  and Markdown use the same term. The design's "attribution build" is defined
  once as the local shorthand for "a build that retains the names the
  measurement needs"; no unconfirmed term was added.
- **Size attribution report** — the two files and the three statuses are used
  consistently in the script, the spec and the design.
- The use case's `## Examples` and `## Personas` were normalized to the
  one-bullet parser contract during this review; the persona descriptions stay
  in the design's Persona impact. No persona is used as a Gherkin actor.

No ubiquitous-language finding remains.

## Coverage

No coverage receipt exists yet for this change; the archive receipt will carry
the measured result. Classification from inspection:

- **Dead code:** `EXECUTABLE_NAME` and `GENERATOR_VERSION` were unused
  constants in the measurement script; both were deleted in `d44185b`.
- **Missing test coverage:** none outstanding. The nine delta scenarios pass;
  the full suite passes 160/160. The step definitions exercise every status:
  complete, incomplete, unreadable (build failure, no names, foreign artifact),
  and the unresolved revision. The section table, the absent `objdump` (the
  shim returns non-zero), the concrete crate/function, and the two-number
  arithmetic are asserted.
- **Hard-to-test defensive arms:** `create_worktree` failure, a cargo exit 0
  with no executable found, and a repository without `.cargo/config.toml` are
  defensive refusals whose preconditions the use case excludes (the named
  revision exists and the automation can build; the fixture mirrors the
  project's committed configuration). They are recorded rather than claimed as
  scenario-covered. The `objdump` non-zero path is exercised by the absent
  objdump scenario.

## E2e coverage

The canonical specs policy was read. One normalized inventory action has one
`@e2e` scenario; the other scenarios are variants/invariants. The real
interface is the CLI command; the primary assertion is on the report it writes
and the exit status, not repository state. The driver is
`std::process::Command` invoking the real Python command against a real git
repository and the real generator. The browser-UI classification is rejected:
the command writes text/files and renders no HTML/CSS/JS in a browser.

## Deferrals

None. The cadence ("produced for every pushed revision") cannot be driven by
GitHub Actions from a local Gherkin run; its non-Gherkin evidence is the
workflow definition (`actionlint` exit 0) and the run's `size-attribution`
artifact and step summary. That is a recorded evidence limitation, not a
deferred mandatory check.

## Use-case and Persona conformance

| Rejection criterion | Evidence | Result |
|---|---|---|
| Goal conjunction | `usecase.md` Goal has one verb phrase ("Obtain a size attribution …"). | PASS |
| Missing primary actor | Release maintainer is primary; the event trigger names them. | PASS |
| Placement failure | The capability advances the measurement goal. | PASS |
| Mutation under an observing goal | The use case declares observing; scenarios assert report content, no state. | PASS |
| Extension without a step reference | Extensions reference steps 2 and 4. | PASS |
| Inconsistent preconditions | Preconditions are pre-existing facts. | PASS |
| Mechanism vocabulary | Goal/Trigger/guarantees avoid mechanisms; `strip` is under Rules. | PASS |
| Component/channel-shaped name | The use case and capability are goal-shaped. | PASS |

Confirmed Personas: `project-maintainer--release-owner` reads the attribution
as evidence; `contributor--build-configuration-reviewer` judges reproducibility
and the artifact; `crew-member--personal-calendar-owner` is the privacy lens.
No new Persona; no biography as an actor. Personas are review lenses.

## Visual review (terminal transcript)

The evidence is the committed transcript at
`givn/changes/measure-executable-size/evidence/visual/maintainer-requests-an-attribution-for-a-revision-and-reads-the-report/transcript.txt`.
Three fresh-context reviews read it, one per confirmed Persona:

- `project-maintainer--release-owner` — **NO FINDING**: requested/resolved
  revision, platform, build identity, split, the two separately labelled
  numbers and the no-substitution/estimate sentence are shown.
- `contributor--build-configuration-reviewer` — **NO FINDING**: toolchain,
  generator version, configuration digests, the split and the stripped-vs-
  attribution-build distinction make reproducibility and the artifact
  checkable.
- `crew-member--personal-calendar-owner` — **NO FINDING**: no plan-derived or
  identifying content; the fixture identities are generated; no host path,
  user name or clock value appears.

The transcript holds the command line, `--- attribution.md ---` and the exit
status, with `<scenario-dir>` substitution. No finding remains.

`VISUAL-REVIEW: PASS`

## Deterministic gate dispositions

`givn check review --change measure-executable-size` exits 0. It reports nine
added scenarios, zero modified, zero removed, and no overlap shape match; no
`## Overlap dispositions` or `## Split-or-keep` rows are owed. `givn lint` is
clean.

## README impact

README-IMPACT: updated - Usage

The README gains the measurement command, the published `size-attribution`
artifact, the local cross-toolchain setup and the two-number distinction.

## Sign-off checklist

- Fabrication audit clean (or findings remediated): yes.
- Every checked task has a verified commit touching production or test code:
  yes; the script revisions and the e2e step/transcript revision are listed
  above.
- Every promised component exists: yes.
- Strict-mode proof present and passing: yes.
- Scenario execution evidence recorded per scope: full 160/160, e2e 9/9.
- Coverage classified and resolved: dead constants deleted; no missing
  scenario; defensive arms recorded.
- Redundant unit tests removed: none added; the feature is the test surface.
- No `@wip` tags remain; no implementation-layer detail in the spec: yes.
- Canonical E2E policy applied; one `@e2e` scenario; variants are regular: yes.
- Every e2e scenario has a real-interface primary assertion and the reviewed
  driver: yes.
- Local run command starts the full stack cleanly: the command and fixture run;
  the local Nix mingw needed an empty `libpthread.a` shim for the missing
  library, while CI's Ubuntu mingw carries it. The e2e and full suite runs
  passed with that environment.
- `verify.e2e_command` is not `verify.command`; counts prove the scope: yes
  (9 < 160).
- Implementation matches design.md's commands, files and approach: yes; the
  LTO fallback is the reviewed one.
- Interaction coverage verified: one inventory entry → one matrix row → one
  `@e2e` scenario → the real driver.
- No finding excused with an invented classification outside the three
  buckets: yes.

REVIEW: PASS
