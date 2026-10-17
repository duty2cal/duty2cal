# Tasks: wait-for-green-checks-before-publish

Release note (commit subject): A release is published only after every check on the tagged revision has concluded successfully, so the branch and tag can be pushed together without racing the gate.

Owning use case: [Publish a public release](specs/publish-public-release/usecase.md) (`publish-public-release`). Confirmed Personas: `project-maintainer--release-owner` and `crew-member--personal-calendar-owner`. Actors remain domain roles.

Single-scenario run command: `./run-tests.sh --name "<scenario title>"`. Full suite: `./run-tests.sh`. E2E-only: `./run-tests.sh --e2e`; no new `@e2e` scenario and none removed.

Constraints carried into every scenario task:

- The gate is the first step of the `publish` job; nothing later runs when it refuses.
- It excludes every check suite the release workflow created for the tagged revision, not only the current run's suite.
- A queued or running check is waited for; a completed non-success refuses immediately; skipped and neutral are allowed; at least one success is required; no evidence, an unreadable source, or the 30-minute bound refuses.
- The bounded wait is 15-second polls for at most 1800 seconds; the workflow step carries a 40-minute backstop.
- The real API and the job ordering are host-only; the command is proven locally through a stub `gh` on `PATH`.
- The atomic branch-and-tag push stays documented and becomes safe.

Baseline before implementation:

```
3 features
120 scenarios (112 passed, 8 failed)
436 steps (428 passed, 8 failed)
```

## Setup task — toolchain, strict mode, fixture facts

- [ ] Confirm the toolchain, strict mode and the gate fixture assumptions.
  - `nix-shell -p cargo git-cliff` provides the runner and the pinned generator; `jq` is available (local `jaq` on this workstation, GNU `jq` in CI), and `actionlint` is the CI job's existing static check.
  - Strict mode is `.fail_on_skipped()`; the baseline fails all eight new scenarios because their steps do not exist.
  - The stub `gh` is written mode 0755 in a per-scenario `gate/bin`; it records `calls`, appends to `args.log`, and prints `poll-N.json`; the gate runs with `--interval 0`.
  - Every fixture is the slurped page array `[{ "total_count": N, "check_runs": [...] }]`; own-suite runs use suite `99`, the atomic fixture uses suites `7` and `8`.

## Scenario tasks

### 1. A tagged revision whose checks are still running is not published until they conclude

**Design constraints:** Poll 1 is two pages: a successful check, a queued check, and an own-suite `publish` in progress; poll 2 is all success. Gate: `--timeout 5 --interval 0`; exit 0 after two calls; the own-suite row never appears under `still running`; `args.log` contains `filter=latest`.

- [ ] RED: remove `@wip`; the steps are stubs; run the single scenario; paste the non-zero output.
- [ ] GREEN: implement `scripts/await-green-checks.sh` and the stub helpers/steps. Production files: `scripts/await-green-checks.sh`, `apps/cli/tests/steps/publish_public_release.rs`. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: comment the classification order and the fail-closed default; re-run; paste.
- [ ] COMMIT: revision `<shared>` (the production slice is shared by the gate scenarios).

### 2. A completed check that is not successful refuses publication

**Design constraints:** Poll 1 has `failure`, `timed_out`, and an in-progress check; the gate refuses immediately, names both non-successes, and calls the API once.

- [ ] RED: remove `@wip`; run the single scenario with stubs; paste the non-zero output.
- [ ] GREEN: add the fixture/steps; no production change beyond task 1. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none; re-run; paste.
- [ ] COMMIT: revision `<shared>`.

### 3. A check that does not conclude within the bounded wait refuses publication

**Design constraints:** Poll 1 is a `queued` check only; `--timeout 1 --interval 0`; the gate names the queued check and `1 second`; no real sleep.

- [ ] RED: remove `@wip`; run the single scenario with stubs; paste the non-zero output.
- [ ] GREEN: add the fixture/steps; no production change beyond task 1. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none; re-run; paste.
- [ ] COMMIT: revision `<shared>`.

### 4. A skipped or neutral check does not refuse publication

**Design constraints:** Poll 1 has `success`, `skipped`, `neutral`; exit 0 on the first poll.

- [ ] RED: remove `@wip`; run the single scenario with stubs; paste the non-zero output.
- [ ] GREEN: add the fixture/steps; no production change beyond task 1. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none; re-run; paste.
- [ ] COMMIT: revision `<shared>`.

### 5. A tagged revision whose checks all concluded without success is refused

**Design constraints:** Poll 1 has `skipped`, `neutral`, and an own-suite success; no successful check outside the release suites; exit 1 with `no check on the tagged revision concluded successfully`; one call.

- [ ] RED: remove `@wip`; run the single scenario with stubs; paste the non-zero output.
- [ ] GREEN: add the fixture/steps; no production change beyond task 1. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none; re-run; paste.
- [ ] COMMIT: revision `<shared>`.

### 6. The check-run source cannot be read

**Design constraints:** The stub exits non-zero without printing a page; exit 1 with `could not read the check runs`; one call.

- [ ] RED: remove `@wip`; run the single scenario with stubs; paste the non-zero output.
- [ ] GREEN: add the failing-stub helper and step; no production change beyond task 1. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none; re-run; paste.
- [ ] COMMIT: revision `<shared>`.

### 7. A tagged revision with no check evidence is refused

**Design constraints:** Poll 1 is one success whose suite is `99`; after the exclusion no rows remain; exit 1 with `no check evidence`; one call.

- [ ] RED: remove `@wip`; run the single scenario with stubs; paste the non-zero output.
- [ ] GREEN: add the fixture/steps; no production change beyond task 1. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none; re-run; paste.
- [ ] COMMIT: revision `<shared>`.

### 8. A branch and its tag pushed together publish only after the checks in both suites conclude

**Design constraints:** Poll 1 has suite `7` `test` success, suite `8` `test` in progress, and own-suite `publish` in progress; poll 2 has both `test` checks successful; the gate waits for both suites, never lists the own-suite row, and exits 0 after two calls.

- [ ] RED: remove `@wip`; run the single scenario with stubs; paste the non-zero output.
- [ ] GREEN: add the two-suite fixture/steps; no production change beyond task 1. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none; re-run; paste.
- [ ] COMMIT: revision `<shared>`.

## Documentation and workflow task

- [ ] Replace the `publish` job's assertion step in `.github/workflows/release.yml` with the release-suite resolution and the gate invocation; keep the step first after checkout and every later step unchanged.
- [ ] Record the arc42 updates (chapters 2, 5, 6, 7, 8, 10, 11, 12 and `arc42.md`); `README.md` stays unchanged because the atomic push becomes safe.
- [ ] Confirm the script carries the SPDX header and mode 0755, and that `scripts/license-headers.sh` accepts it.

## Verification task

- [ ] `./run-tests.sh` → every scenario passes; paste the summary.
- [ ] `./run-tests.sh --e2e` → the existing `@e2e` scenarios pass; paste.
- [ ] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `GIVN_FEATURES=$(scripts/assemble-features.sh) cargo test --workspace` → clean; paste.
- [ ] `givn lint --change wait-for-green-checks-before-publish` clean; `scripts/pii-guard.sh`, `scripts/license-headers.sh` clean; paste.
- [ ] `./measure-coverage.sh` and `./merge-coverages.sh` → valid reports; paste the merged line.
- [ ] `rhysd/actionlint:1.7.12` (or the CI `actionlint` job) accepts the edited workflow; paste.
