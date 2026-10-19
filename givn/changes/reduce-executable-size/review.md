# Review: reduce-executable-size

- **Change:** `reduce-executable-size`
- **Date:** 2026-10-19
- **Implementation revision:** `6b54806` (the `Cargo.toml` lever, the command, the steps, the spec, the design and the docs).
- **Targeted evidence:** the reduction regular scenarios `7 passed / 28 steps`; the e2e scenario `1 passed / 7 steps`; `givn lint --change reduce-executable-size` exits 0.
- **Full suite and coverage:** the archive gate runs `verify.command`, `verify.e2e_command` and the coverage measurement; the verification receipt carries the per-scope counts and the merged coverage. No separate full-suite receipt existed during the review; the classification below is from inspection plus the targeted runs, as the archive receipt will carry the measured result.

## Step 0 — Fabrication audit

1. **`@e2e` tag integrity.** The delta has one `@e2e` scenario
   (`Maintainer measures the panic-abort lever and the landed executable is
   smaller`); the tag was never removed. The `@wip` tags were removed after
   the targeted GREEN runs; `givn lint` exits 0.
2. **Empty step bodies.** `apps/cli/tests/steps/reduce_distributed_executable_size.rs`
   and its e2e file contain no `{}`/`() {}`/bare `pass`/`return`/`todo!()`/
   `unimplemented!()` bodies; every step drives the command or asserts the
   report.
3. **Checked tasks have revisions.** The production/test/spec/docs slice is
   `6b54806`; `git show --stat 6b54806` includes `Cargo.toml`, the command,
   both step files, the feature, the use case, the design and the arc42/docs.
   The task entries record the shared revision (the one atomic production
   slice, as the reduction's scenarios are all evidence for it).
4. **Promised components exist.** `scripts/measure_size_lever.py`;
   `apps/cli/tests/steps/reduce_distributed_executable_size.rs`; the e2e file;
   the reduction world fields and module lines; the permanent use case; the
   feature; the `cliff.toml` allow-list/parser rows; the docs.
5. **Strict-mode proof.** The measurement change's probe (`probe-exit=101`,
   `Step doesn't match any function`) applies to the same `features_runner`;
   the reduction steps use the same `.fail_on_skipped()` builder.
6. **`@e2e` Then assertions.** The e2e asserts the landed revision in the
   report, the applied saving, the rebuilt baseline, the landed smaller build,
   the separately reported published size, the saved-baseline arithmetic and
   the new numbered diagnostic from the landed profile's host build. It is not
   repository-only.
7. **Browser driver.** Not applicable: the interface is a terminal command and
   a Markdown/JSON report; no HTML/CSS/JS is rendered.
8. **`verify.e2e_command`.** `./run-tests.sh --e2e` is distinct from
   `./run-tests.sh`; it passes `--tags @e2e` to the same `features_runner`.
   The e2e step file is the only reduction e2e implementation; no parallel
   implementation exists.
9. **E2E scope.** The use case has one inventory entry, "measure one size lever
   and read the landed saving and failure outcome"; the design has one matrix
   row; the delta has one matching `@e2e` scenario. The other seven scenarios
   are refusals, the rejected larger lever, the cumulative comparison and the
   build failure. The e2e count is the full count's strict subset.
10. **Design conformance.** The command path/flags, the landed build, the
    report labels and statuses, the step files, the transcript path and the
    release-note allow-list match `design.md`. The fixture's `lto = false` is
    the measurement fixture's reviewed fallback, not this change's product.

## Faults found and closed

| Finding | Resolution |
|---|---|
| The first e2e measured a revision without the landed lever (the report said `applied` on the pre-lever `HEAD`). | The command builds the landed committed configuration; the e2e Given asserts `git show HEAD:Cargo.toml`; the evidence was regenerated after `6b54806`. |
| The larger-lever assertion compared against the unchanged baseline and would fail after the landed commit. | The step compares the row's cumulative baseline (`2,014,208` vs `2,084,864`) and a negative `saved_bytes`. |
| The transcript leaked a project path, a PID path and a build duration, and the report was appended after substitution. | The build log is not recorded; the report is appended before the canonical `<project>`/`<scenario-dir>` substitution; the transcript carries the measurement and diagnostic exit statuses. |
| The published-size and JSON assertions were labels only. | The steps assert the supplied `2,097,152` value, `baseline_sha256`, `saved_bytes`, the landed section, the JSON keys and a newly numbered diagnostic with the seeded prior file unchanged. |
| The source-name diagnostic assertion used the full path while the product names the file. | The assertion uses the file name and the reason and seeds a prior report, so a new numbered report is proven. |
| The transcript committed with `6b54806` exposed the project path, the user name and a temp lever path (the visual privacy review). | The regenerated, sanitized transcript is committed with this review; no host path, user name, temp identifier or clock value remains. The build, owner and privacy reviews all pass on the committed copy. |

## Coverage

- **Dead code:** the unused `copy_dir_all` helper in the new test file was
  removed; the command's `config_target` helper was removed during hardening.
- **Missing coverage:** the eight delta scenarios pass; the command's refusal,
  build-failure, cumulative, landed and report paths are exercised. The
  diagnosis-loss refusal is a control (the profile does not remove the typed
  diagnostic; the host build proves it). The review-rejection path is a human
  control owned by the contributor reviewer (Q2). The zero-saving case shares
  the larger-lever comparison; no no-op lever exists.
- **Hard-to-test:** a Windows-side unsupported-source run is the release
  workflow's Windows smoke boundary; R-040 records it. The panic message itself
  is the toolchain's guarantee; the report's failure-outcome lines and the
  host diagnostic are asserted.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Release profile lever, measured saving, failure outcome | `02-architecture-constraints.md:26` | Yes | Measured evidence; D1/D4 | Scenarios 1–7 | `Cargo.toml`; command; report | yes |
| Lever technology choice | `04-solution-strategy.md:26` | Yes | Technology decisions | Scenarios | command | yes |
| Command, report, step blocks | `05-building-block-view.md` | Yes | Architecture impact | Setup; scenarios | command, report, steps | yes |
| Refusal, baseline, cumulative, landed flow | `06-runtime-view.md:203-224` | Yes | The size-lever command | Scenarios | command/report | yes |
| Release executable's profile | `07-deployment-view.md:45-47` | Yes | Scope; technology | Scenario 1 | `Cargo.toml`; release build | yes |
| Panic outcome and diagnostic | `08-crosscutting-concepts.md:230-240` | Yes | D4; failure outcome | e2e diagnostic | host lever run | yes |
| QS-025 measured reduction | `10-quality-requirements.md:67` | Yes | Measured evidence | Scenario 1 | e2e report | yes |
| R-039/R-040/TD-017 | `11-risks-and-technical-debt.md:45-46,68` | Yes | Risks | — | design/evidence | yes |
| Baseline/Published binary/Saved bytes; Size lever | `12-glossary.md:88-91` | Yes | Report/terms | Scenarios | labels/JSON | yes |
| No new external system | `03-context-and-scope.md` | No | Scope | — | tooling only | yes |
| No qualified ADR | `09-architecture-decisions.md` | No | Decisions | — | no ADR | yes |

All 12 chapter files exist, contain decision-specific content and use Mermaid
only. Chapter 09 has no missing qualified ADR; chapter 11 carries the
consequences.

`ARC42 CONFORMANCE: CLEAN`

## Ubiquitous language conformance

**Size lever**, **Baseline**, **Published binary** and **Saved bytes** are in
the glossary; "saved" is the positive convention; the proposal, use case,
design, report and tests use the same words. Refused proposals are not called
levers. No term is missing or reused inconsistently.

## E2E coverage

One normalized inventory action → one matrix row → one `@e2e` scenario → the
real `python3 scripts/measure_size_lever.py` subprocess, the report and the
exit status. The host lever build is the failure-outcome half of the same
action; the Windows executable is started by the release workflow's Windows
smoke job. The browser-UI classification is rejected: no page is rendered.

## Deferrals

The Windows-side unsupported-source run is not a local test; it is the
release workflow's Windows smoke boundary and is recorded as R-040 with the
operator decision (the requested release). No mandatory check is deferred
without that record.

## Use-case and Persona conformance

| Criterion | Evidence | Result |
|---|---|---|
| Goal conjunction | One goal: a smaller executable by a measured lever. | PASS |
| Missing primary actor | Release maintainer primary; named in Trigger. | PASS |
| Placement failure | The capability advances the reduction goal. | PASS |
| Mutation under an observing goal | The use case changes the build configuration and lands it. | PASS |
| Extension without a step reference | Extensions reference steps 3, 3, 4, 5, 7. | PASS |
| Inconsistent preconditions | The attribution exists; the constraints hold. | PASS |
| Mechanism vocabulary | Goal/Trigger/guarantees avoid mechanisms. | PASS |
| Component/channel-shaped name | Goal-shaped use case/capability. | PASS |

The three confirmed Personas are review lenses; the Actor is the Release
maintainer; no Persona biography is a Gherkin actor.

PERSONAS: project-maintainer--release-owner
PERSONAS: contributor--build-configuration-reviewer
PERSONAS: crew-member--personal-calendar-owner

## Visual review (terminal transcript)

The committed transcript is
`givn/changes/reduce-executable-size/evidence/visual/maintainer-measures-the-panic-abort-lever-and-the-landed-executable-is-smaller/transcript.txt`.
Three fresh-context reviews read it, one per confirmed Persona:
`project-maintainer--release-owner` — NO FINDING; the measured saving, the
landed smaller build, the published-size separation and the failure outcome are
shown. `contributor--build-configuration-reviewer` — NO FINDING; the
cumulative baseline, the rejected larger lever, the landed build and the panic
consequence are shown. `crew-member--personal-calendar-owner` — **FINDING,
dispositioned:** the pre-fix transcript committed with `6b54806` exposed the
project path, the user name and a temp lever path; the regenerated sanitized
transcript is committed with this review, and the committed copy has no host
path, user name, temp identifier or clock value.

`VISUAL-REVIEW: PASS`

## Deterministic gate dispositions

`givn check review --change reduce-executable-size` is run after this file and
the delta merge; the delta has no overlap shape match (the duplicate published
scenario was removed) and no long-scenario split is owed. `givn lint` exits 0.

## README impact

README-IMPACT: none

The executable's size is not stated in a README section; the release notes are
the user-visible statement of the saving, and the README's download instructions
are unchanged.

## Sign-off checklist

- Fabrication audit clean: yes.
- Checked tasks have verified revisions: `6b54806` (shared production/test/spec
  slice) and the tasks evidence.
- Every promised component exists: yes.
- Strict-mode proof present and passing: yes.
- Scenario execution evidence: targeted regular 7/7 and e2e 1/1; the archive
  receipt carries the full counts.
- Coverage classified: dead helper removed; no missing scenario; the
  hard-to-test boundaries are R-040/controls.
- Redundant tests removed: none added beyond the executable spec.
- No `@wip`; no implementation detail in the spec: yes.
- Canonical E2E policy: one `@e2e`; variants regular.
- Real-interface primary assertion: yes.
- `verify.e2e_command` is distinct and scoped: yes.
- Implementation matches design.md: yes (landed build, cumulative, transcript).
- Interaction coverage: one entry → one matrix row → one e2e → the real driver.
- No invented exception outside the three buckets: yes.

REVIEW: PASS
