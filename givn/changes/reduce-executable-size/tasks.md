# Tasks: reduce-executable-size

Release note (commit subject): The Windows executable is smaller: a panic
aborts instead of unwinding.

The production change is one atomic slice (`Cargo.toml`, the lever command, the
step definitions and the report) at revision `6b54806`; each scenario records
its evidence revision, and the shared production slice is named beside them. The e2e transcript was regenerated after
the landed commit. The full suite and e2e are run by the archive gate and the
release workflow; the targeted runs below are the implementation evidence.

## Setup — runner, strict mode, toolchain, e2e

- [x] Confirm the Gherkin runner and strict mode.
  - `./run-tests.sh` (`verify.command`) and `./run-tests.sh --e2e`
    (`verify.e2e_command`, distinct) drive `features_runner`; the builder uses
    `.fail_on_skipped()`.
  - The measurement change's strict-mode probe (`probe-exit=101`,
    `Step doesn't match any function`) applies to the same runner.
  - The reduction step files are registered in `steps/mod.rs`,
    `steps/e2e/mod.rs`; `DutyWorld` carries the lever fields.
- [x] Confirm the toolchain and the e2e environment.
  - `rustc --print target-list` contains `x86_64-pc-windows-gnu`;
    `x86_64-w64-mingw32-gcc` and `cargo` are present. The local Nix run uses
    the empty-archive `-L` environment; CI's Ubuntu `mingw-w64` carries the
    library.
- [x] Remove `@wip` after the scenarios are green.
  - `givn lint --change reduce-executable-size` → `1 file(s) checked — clean`,
    exit 0.

## Scenario 1 — Maintainer measures the panic-abort lever and the landed executable is smaller (`@e2e`)

- [x] RED: before the landed commit, the command measured the pre-lever
  `HEAD` and the landed status was `not_landed`; the landed assertion failed.
  The earlier diagnostic step also failed on the source-name assertion before
  the fix (`bg_76`, `1 scenario (1 failed)`).
- [x] GREEN: `./run-tests.sh --name "Maintainer measures the panic-abort lever and the landed executable is smaller"` → `1 scenario (1 passed)`, `7 steps (7 passed)` after the landed commit `6b54806`.
  - Evidence: `- Revision: 6b54806...`, `- Status: applied`, baseline
    `2,098,176 B`, lever `2,014,208 B`, `saved bytes 83,968`, landed
    `2,014,208 B`, landed status `smaller`; the transcript has no host path,
    PID or duration; the diagnostic run exits 1 with a new numbered file.
- [x] REFACTOR: the transcript was hardened (report appended before
  substitution, canonical project path, both exit statuses, no build log);
  the e2e re-ran green.
- [x] COMMIT: revision `c2ca1ce` (scenario evidence; production slice `6b54806`).

## Scenario 2 — A lever that measures larger is rejected

- [x] RED: after the landed commit the old assertion compared the cumulative
  lever against the unchanged baseline; the step was corrected to compare the
  row's cumulative baseline.
- [x] GREEN: `./run-tests.sh --tags "@reduce-distributed-executable-size and not @e2e"` → scenario passes; `opt-level=s` row `2,084,864` against
  `2,014,208`, `saved bytes` negative, `rejected`; the `Cargo.toml` still
  carries `panic = "abort"` and no `opt-level = "s"`.
- [x] REFACTOR: re-ran the targeted scenario; green.
- [x] COMMIT: revision `8822b43` (scenario evidence).

## Scenario 3 — A second lever is measured against the configuration carrying the first

- [x] RED: the step expected the first row's applied status and the second
  row's cumulative baseline.
- [x] GREEN: the `panic=abort,opt-level=s` run has row 1 `applied` and row 2
  `rejected`, with row 2's baseline equal to row 1's lever bytes.
- [x] REFACTOR: re-ran; green.
- [x] COMMIT: revision `fa1221d` (scenario evidence).

## Scenario 4 — A nightly-only proposal is refused before a build

- [x] RED/GREEN: the `proposes the "nightly" lever` command exits non-zero,
  names `stable-toolchain`, writes no report and no build runs; the scenario
  passes.
- [x] REFACTOR: re-ran; green.
- [x] COMMIT: revision `bba277c` (scenario evidence).

## Scenario 5 — A proposal outside the supported approach is refused before a build (`upx`, `dynamic-linking`)

- [x] RED: the outline step text lacked the quotes around `<lever>`; the step
  did not match.
- [x] GREEN: `upx` names `single-file`, `dynamic-linking` names
  `static-linking`; no report and no build; both examples pass.
- [x] REFACTOR: re-ran the outline; green.
- [x] COMMIT: revision `faa2535` (scenario evidence).

## Scenario 6 — An unknown lever is refused before a build

- [x] RED/GREEN: `turbo` is refused with `not a supported size lever`; no
  baseline and no report; the scenario passes.
- [x] REFACTOR: re-ran; green.
- [x] COMMIT: revision `9794d5e` (scenario evidence).

## Scenario 7 — A build that fails records no delta

- [x] RED/GREEN: the fixture revision with `compile_error!` makes the baseline
  build fail; the report is `failed` with `## No measurement`, no cumulative
  rows and no saving; the command exits non-zero.
- [x] REFACTOR: re-ran; green.
- [x] COMMIT: revision `bc35bdf` (scenario evidence).

## Final checks

- [x] Targeted regular reduction scenarios: `7 scenarios (7 passed)`, `28
  steps (28 passed)`; targeted e2e: `1 scenario (1 passed)`, `7 steps (7
  passed)`.
- [x] The full suite and e2e counts are recorded by the archive gate's
  verification receipt; the release workflow re-runs the full suite on the
  tagged revision.
- [x] `givn lint --change reduce-executable-size` exits 0.
- [x] The e2e transcript is committed at
  `givn/changes/reduce-executable-size/evidence/visual/maintainer-measures-the-panic-abort-lever-and-the-landed-executable-is-smaller/transcript.txt`.
