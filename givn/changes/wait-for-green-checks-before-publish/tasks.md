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

The production slice (script plus workflow step) is shared by the eight gate scenarios, so they record revision `5565920` rather than eight artificial production commits.

## Setup task — toolchain, strict mode, fixture facts

- [x] Confirm the toolchain, strict mode and the gate fixture assumptions.
  - `nix-shell -p cargo git-cliff` provides the runner and generator; `actionlint` is available at 1.7.12 via `nix-shell -p actionlint`; `jq` is jaq locally and GNU jq in CI.
  - Strict mode is `.fail_on_skipped()`; the baseline failed all eight new scenarios because their steps did not exist.
  - The stub `gh` is written mode 0755 in `gate/bin`; it records `calls`, appends `args.log`, and prints `poll-N.json`; the gate runs with `--interval 0`.
  - Fixtures are the slurped page array; own-suite runs use suite `99`, the atomic fixture uses suites `7` and `8`.

Baseline before implementation:

```
3 features
120 scenarios (112 passed, 8 failed)
436 steps (428 passed, 8 failed)
```

## Scenario tasks

### 1. A tagged revision whose checks are still running is not published until they conclude

**Design constraints:** Poll 1 is two pages: a success, a queued check, and an own-suite `publish` in progress; poll 2 is all success. `--timeout 5 --interval 0`; exit 0 after two calls; the own-suite row never appears under `still running`; `args.log` contains `filter=latest`.

- [x] RED: baseline failed the scenario with `Step doesn't match any function` (part of `120 scenarios (112 passed, 8 failed)`).
- [x] GREEN: script, stub helper and steps added. Targeted run: `1 scenario (1 passed) / 4 steps (4 passed)`.
- [x] REFACTOR: classification order and fail-closed default commented; targeted run stays green.
- [x] COMMIT: revision `5565920`.

### 2. A completed check that is not successful refuses publication

**Design constraints:** Poll 1 has `failure`, `timed_out`, and an in-progress check; immediate refusal naming both; one call.

- [x] RED: baseline undefined-step failure.
- [x] GREEN: fixture/steps added. Targeted run: `1 scenario (1 passed) / 4 steps (4 passed)`.
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `5565920`.

### 3. A check that does not conclude within the bounded wait refuses publication

**Design constraints:** Poll 1 is `queued` only; `--timeout 1 --interval 0`; names the queued check and `1 second`; no real sleep.

- [x] RED: baseline undefined-step failure.
- [x] GREEN: fixture/steps added. Targeted run: `1 scenario (1 passed) / 3 steps (3 passed)`.
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `5565920`.

### 4. A skipped or neutral check does not refuse publication

**Design constraints:** Poll 1 has `success`, `skipped`, `neutral`; exit 0 on the first poll.

- [x] RED: baseline undefined-step failure.
- [x] GREEN: fixture/steps added. Targeted run: `1 scenario (1 passed) / 3 steps (3 passed)`.
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `5565920`.

### 5. A tagged revision whose checks all concluded without success is refused

**Design constraints:** Poll 1 has `skipped`, `neutral`, and an own-suite success; exit 1 with `no check on the tagged revision concluded successfully`; one call.

- [x] RED: baseline undefined-step failure.
- [x] GREEN: fixture/steps added. Targeted run: `1 scenario (1 passed) / 3 steps (3 passed)`.
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `5565920`.

### 6. The check-run source cannot be read

**Design constraints:** The stub exits non-zero without a page; exit 1 with `could not read the check runs`; one call.

- [x] RED: baseline undefined-step failure.
- [x] GREEN: failing-stub helper and steps added. Targeted run: `1 scenario (1 passed) / 3 steps (3 passed)`.
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `5565920`.

### 7. A tagged revision with no check evidence is refused

**Design constraints:** Poll 1 is one success in suite `99`; after exclusion no rows remain; exit 1 with `no check evidence`; one call.

- [x] RED: baseline undefined-step failure.
- [x] GREEN: fixture/steps added. Targeted run: `1 scenario (1 passed) / 3 steps (3 passed)`.
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `5565920`.

### 8. A branch and its tag pushed together publish only after the checks in both suites conclude

**Design constraints:** Poll 1 has suite `7` `test` success, suite `8` `test` in progress, and own-suite `publish` in progress; poll 2 has both `test` checks successful; both suites waited for, own-suite row never listed, exit 0 after two calls.

- [x] RED: baseline undefined-step failure.
- [x] GREEN: two-suite fixture/steps added. Targeted run: `1 scenario (1 passed) / 4 steps (4 passed)`.
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `5565920`.

## Documentation and workflow task

- [x] Replaced the `publish` job's assertion step in `.github/workflows/release.yml` with the release-suite resolution and the gate invocation; the step stays first after checkout and every later step is unchanged. Revision `5565920`.
- [x] Recorded the arc42 updates: chapters 2, 5, 6, 7, 8, 10, 11, 12 and `arc42.md`. `README.md` stays unchanged because the atomic push becomes safe. Revision `5565920`.
- [x] The script carries `SPDX-License-Identifier: MPL-2.0`, mode 0755; `scripts/license-headers.sh` is clean.

## Verification task

- [x] `./run-tests.sh`:

  ```
  3 features
  120 scenarios (120 passed)
  455 steps (455 passed)
  ```
- [x] `./run-tests.sh --e2e`:

  ```
  2 features
  3 scenarios (3 passed)
  16 steps (16 passed)
  ```
- [x] `cargo fmt --all --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean; `GIVN_FEATURES=$(scripts/assemble-features.sh) cargo test --workspace` exited 0.
- [x] `givn lint --change wait-for-green-checks-before-publish` clean; `scripts/pii-guard.sh` clean; `scripts/license-headers.sh` clean.
- [x] `./measure-coverage.sh` and `./merge-coverages.sh` → `merged 7 files, 886 lines (871 covered) -> coverage/cobertura-coverage.xml`.
- [x] `actionlint 1.7.12` accepts `.github/workflows/release.yml`.
