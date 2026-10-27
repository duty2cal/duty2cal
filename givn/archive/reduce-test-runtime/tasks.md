# Tasks: reduce-test-runtime

This is an operational maintenance change with no product capability delta:
`specs` is skipped in `.givn-skip` and no `.feature` scenario is added,
modified or removed. The scenario-by-scenario RED/GREEN/REFACTOR loop therefore
has no scenario to run. The task list records the ordered work, its evidence,
and the implementation revision.

The runner is already configured and strict: `verify.command` is
`./run-tests.sh`, `verify.e2e_command` is `./run-tests.sh --e2e`, and
`apps/cli/tests/features_runner.rs` calls `.fail_on_skipped()`. The baseline
below is the proof that the setup is inherited and still passes.

Design constraints for every task: the default run contains the core/PDF
unit and property tests, the architecture tests, the golden calendar test, and
the `export-duty-plan-calendar` cucumber scenarios; operational features are
`@documentation`; browser scenarios are `@browser`; no default run may spawn
`cargo`, `git worktree`, a cross-build, or a network call; the Windows budget is
2,097,152 B; the per-push attribution stays in `windows-gnu-build`; drift stays
local (ADR-0006).

## 1. Baseline and test scope

- [x] Confirm the inherited runner and the 84-scenario product baseline.
  - Design constraints: `./run-tests.sh`, no browser, no operational features, concurrency from the CPU count.
  - Evidence: `./run-tests.sh` → 1 feature, 84 scenarios, 303 steps, all passing; `phase=assemble`, `phase=rust-tests`, `phase=golden`, `phase=cucumber`; warm wall time ~2 s, cold ~33 s.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

- [x] Mark the eight operational feature files `@documentation` and skip them in the assembler.
  - Design constraints: the files stay in `givn/specs` as documentation; `scripts/assemble_features.py` skips a feature whose prefix carries the tag.
  - Evidence: the assembler prints one `is @documentation; not executed` line per file and emits only `f0001.feature` for the product capability.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

- [x] Delete the operational step definitions and trim the world and fixture support.
  - Design constraints: no step definition remains whose subject is not the CLI product or its WASM facade; `support.rs` keeps only the fixture builder; `DutyWorld` keeps only the product fields.
  - Evidence: `apps/cli/tests/steps` holds only `browser.rs`, `e2e/export_duty_plan_calendar.rs`, `export_duty_plan_calendar.rs`, `mod.rs`, `support.rs`; `git diff --stat` removes the eight operational files and their e2e counterparts.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

- [x] Tag the browser scenarios `@browser` and keep one smoke in CI.
  - Design constraints: the default run excludes `@browser`; the `browser` CI job runs the existing `@e2e` smoke by name; all variants remain runnable with `--browser`.
  - Evidence: 16 scenarios carry `@browser`; `./run-tests.sh --browser --name "Crew member converts a supported duty plan in the browser and saves the offered calendar"` → 1 scenario, 12 steps, passing.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

## 2. Runner and golden test

- [x] Add the direct golden test for the 2026-06 fixture.
  - Design constraints: `apps/cli/tests/golden.rs` builds the generated plan, runs the real binary, and compares `roster.ics` with `tests/fixtures/expected/2026-06.ics` after the agreed normalisation.
  - Evidence: `cargo test -p duty2cal --test golden` → 1 passed.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

- [x] Add phase timings, CPU-count concurrency, selection guards, and `--no-fail-fast`.
  - Design constraints: a run that selects no scenario fails; `--name` must select exactly one; an earlier failing class is reported as `failed >= 1`; all three classes run.
  - Evidence: `./run-tests.sh --tags "@no-such-tag"` → exit 1 and `run-tests: no scenario was selected`; `./run-tests.sh --name "No such scenario"` → exit 1 and `the name selected 0 scenarios; exactly one is required`; `./run-tests.sh --e2e` → 1 scenario, 5 steps, `{"scope":"e2e","total":1,"passed":1,"failed":0,"skipped":0}`.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

- [x] Coverage filters the product-only suite and keeps the nested-build guard.
  - Design constraints: non-e2e `not @e2e and not @browser and not @nested-build`; e2e `@e2e and not @browser and not @nested-build`; no web build.
  - Evidence: `measure-coverage.sh` contains both expressions and no `scripts/build-web.sh` call.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

## 3. CI

- [x] Split and harden the CI jobs.
  - Design constraints: `test` has a Rust cache, a 20-minute timeout, and no `mingw-w64`, `cargo-bloat`, `git-cliff`, OpenTofu, or browser toolchain; `browser` runs the smoke; `scripts` runs bats; `coverage` runs on `main` only; `deny` uses the install action; `windows-gnu-build` keeps the attribution and adds the 2,097,152-B budget.
  - Evidence: `.github/workflows/ci.yml` contains the split jobs, `Swatinem/rust-cache@v2`, `taiki-e/install-action@v2`, `timeout-minutes: 20`, `if: github.ref == 'refs/heads/main'`, `budget=2097152`, and the attribution before the budget.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

- [x] Run the golden test before the site build and keep the release check wait.
  - Design constraints: `site.yml` runs `cargo test -p duty2cal --test golden` before `publish-site.sh`; a failure deploys nothing; `release.yml` still waits for the tagged revision's checks.
  - Evidence: `.github/workflows/site.yml` has the golden step before the publish step; `release.yml` is unchanged and `scripts/await-green-checks.sh` remains the publication gate.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

## 4. Operational guarantees

- [x] Add the local drift check and keep the passphrase out of CI.
  - Design constraints: ADR-0006; `TF_VAR_state_passphrase`, `TF_VAR_billing_email`, and `GITHUB_TOKEN`/`GH_TOKEN` come from the maintainer's environment; no CI job holds them.
  - Evidence: `scripts/check-drift.sh` refuses without the passphrase (`check-drift: TF_VAR_state_passphrase is not set`) and runs `tofu plan -refresh-only -detailed-exitcode` with no `-out` and no apply.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

- [x] Add bats coverage for the publication's early refusals and the `--since` decision.
  - Design constraints: no cargo build, no network; bad arguments, missing paths, unclean tree, unknown base, docs-only revision, unresolvable revision.
  - Evidence: `bats scripts/tests` → 10/10 pass; the docs-only case reports `no site input changed` and writes no output or target.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

## 5. Documentation and verification

- [x] Update the arc42 chapters and the change assessment.
  - Design constraints: chapters 2, 5, 6, 7, 8, 10, 11 carry the test-scope amendment and name the new owner; chapter 6 is marked Yes; `STATUS: DONE` is honest.
  - Evidence: `givn/changes/reduce-test-runtime/arc42.md` has the 12-row table with row 6 Yes; the design review ended `DESIGN-REVIEW: PASS`.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

- [x] Run the final checks.
  - Design constraints: the product suite, the e2e run, the browser smoke, bats, formatting, clippy, the workspace compile, and lint.
  - Evidence: `./run-tests.sh` → 84 scenarios / 303 steps; `./run-tests.sh --e2e` → 1 / 5; browser smoke → 1 / 12; `bats scripts/tests` → 10/10; `cargo fmt --all --check` → 0; `cargo clippy --workspace --all-targets -- -D warnings` → 0; `cargo test --workspace --no-run` → success; `givn lint` → exit 0.
  - Commit: `e2218486aeb34f1df7a533b966d43b705273d7bb`.

## Commit log

| Revision | Purpose |
|---|---|
| `e2218486aeb34f1df7a533b966d43b705273d7bb` | Product-only test scope, runner/CI split, bats and drift checks, golden test, and the arc42 update. |
| this revision | The completed task record and its evidence; archived with the change. |
