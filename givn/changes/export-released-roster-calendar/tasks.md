# Tasks: export-duty-plan-calendar

Release note (commit subject): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.

Owning use case: [Export a duty-plan calendar](specs/export-duty-plan-calendar/usecase.md) (`export-duty-plan-calendar`). Confirmed Personas: `crew-member--personal-calendar-owner` (accuracy/privacy lens), `project-maintainer--release-owner` (release-evidence lens). Actors in scenarios stay distinct from Personas.

Single-scenario run command (used by every RED/GREEN/REFACTOR below): `./run-tests.sh --name "<scenario title>"`.
Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`.

## Setup task — workspace, licensing, runner, strict mode, architecture test

- [x] Restructure to the adopted workspace (ADR-0002): `crates/duty2cal-core` (MPL-2.0; modules `domain/`, `application/`, `adapters/ics/`; deps `thiserror` + `time`; `clippy.toml` guardrails), `crates/duty2cal-pdf` (MPL-2.0; the only `pdfplumber` user), `apps/cli` (package/bin `duty2cal`, MPL-2.0). Root `Cargo.toml` workspace (`license = "MPL-2.0"`), `rust-toolchain.toml`, `deny.toml`, `REUSE.toml`, `LICENSES/MPL-2.0.txt`, per-file SPDX headers.
- [x] Configure the Gherkin runner and prove strict mode BEFORE any scenario.
  - `apps/cli/tests/features_runner.rs` is a `harness = false` cucumber-rs target. It reads the feature root from `GIVN_FEATURES`, sets `.fail_on_skipped()`, and uses `.run_and_exit(root)` so the exit code reflects failures.
  - cucumber-rs 0.23 takes a single feature root, so `./run-tests.sh` assembles every `.feature` from `givn/specs/**` and `givn/changes/*/specs/**` into a temporary directory and exports `GIVN_FEATURES`.
  - Step skeleton: `apps/cli/tests/steps/export_duty_plan_calendar.rs` (+ `steps/mod.rs`), one file per capability.
  - Proof-of-strictness (undefined step must fail): temporary `strict-mode-proof.feature` with an undefined step:
    ```
    Scenario: An undefined step fails the run
     ✘  Given a step that has no definition
        Step doesn't match any function
    1 feature / 1 scenario (1 failed) / 1 step (1 failed)
    RUNNER_EXIT=101
    ```
    The temporary feature was deleted after the proof.
- [x] Compile-proof the pinned extraction dependency (R-001) under the new pin: `cargo build --workspace` compiled `pdfplumber-core 0.4.1`, `pdfplumber-parse 0.4.1`, `pdfplumber 0.4.1`, `duty2cal-core`, `duty2cal-pdf`, `duty2cal` on rustc 1.98.1 / cargo 1.98.0; no native C/C++ dependency.
  ```
  Compiling pdfplumber-parse v0.4.1
  Compiling pdfplumber v0.4.1
  Compiling duty2cal-pdf v0.1.0
  Compiling duty2cal v0.1.0
  Finished `dev` profile ... in 36.43s
  ```
- [x] Architecture test, level 3 (ADR-0002): `cargo test -p duty2cal-core --test architecture` → 3 passed (domain isolation, application→adapters denial, cycle-free).
- [x] Prove the e2e filter is a strict subset: full suite reports 51 scenarios; `./run-tests.sh --e2e` reports 1 scenario.
  ```
  ./run-tests.sh          -> 1 feature, 51 scenarios (RED)
  ./run-tests.sh --e2e    -> 1 feature,  1 scenario  (RED)
  ```
  (The failures are the expected RED state: no step definitions exist yet and strict mode fails undefined steps.)
- [x] Remaining setup — CI, coverage, notices, gates.
  - `THIRD-PARTY-NOTICES.md` is generated with `cargo about generate about.hbs --output-file THIRD-PARTY-NOTICES.md` (110 licence sections) and is idempotent; CI diffs it for drift.
  - `.github/workflows/ci.yml`: fmt, clippy, test, deny, reuse, license-headers, pii-guard, notices-drift, coverage, `windows-gnu-build` (mingw-w64 cross-build plus a DLL-import assertion that only system DLLs are imported) and `windows-smoke`. `.github/workflows/release.yml` cross-builds the GNU target on a `v*` tag, writes a SHA256 and attaches both to the release. `.github/dependabot.yml` groups cargo updates weekly and ignores `pdfplumber`/`archunit` minor+major.
  - `./measure-coverage.sh` runs cargo-llvm-cov twice against the shared `scripts/assemble-features.sh` root — non-e2e (`--tags "not @e2e"`) and e2e (`--tags "@e2e"`) — writing `coverage/non-e2e-cobertura.xml` and `coverage/e2e-cobertura.xml`; `./merge-coverages.sh` combines them into `coverage/cobertura-coverage.xml` through `scripts/merge-cobertura.py`.
  - `.cargo/config.toml` sets the GNU linker plus static CRT/link flags; the release profile is `opt-level = "z"`, `lto`, `strip`, `codegen-units = 1`.
  - Verified locally: `cargo fmt --all --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean; `cargo deny check` → advisories/bans/licenses/sources ok (the BSD-2-Clause/ISC/Unicode-DFS-2016 allowances are intentionally unmatched; dev-only licences such as BlueOak-1.0.0 are outside cargo-deny's default graph); `reuse lint` → compliant, 81/81 files; `bash scripts/license-headers.sh` → clean; `bash scripts/pii-guard.sh` → clean; `./measure-coverage.sh && ./merge-coverages.sh` → two distinct reports (separate timestamps) plus the merged report; `./run-tests.sh` 51 scenarios, `./run-tests.sh --e2e` 1 scenario.
  - Locally, coverage needs the LLVM tools: `nix-shell -p rustc cargo cargo-llvm-cov llvm` with `LLVM_COV`/`LLVM_PROFDATA` exported (CI uses the `llvm-tools-preview` component).
  - Still release-evidence rather than setup: the Windows GNU cross-build and the absence of companion DLLs (R-002) are proven by the CI/release job, not locally here.

## Scenarios (non-@e2e first, in feature order)

### 1. A source outside the supported family is rejected

Use-case constraint: Extensions 2: an unreadable/unsupported source fails explicitly with a numbered diagnostic.

- [x] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  Scenario: A source outside the supported family is rejected
   ✔  Given a duty plan that is not a supported Released Roster detail-table export
   ✘  When the crew member converts it
      Step panicked. Captured output: not implemented: run the converter on the prepared source
  1 scenario (1 failed), 2 steps (1 passed, 1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — replace stubs with real assertions + minimum production code; production files: `crates/duty2cal-core/src/application/mod.rs` (RosterSource port, `SourceError`, `RawRosterTable`, `ConvertRoster`), `crates/duty2cal-pdf/src/lib.rs` (`PdfRosterSource` over pdfplumber 0.4.1), `apps/cli/src/fs.rs` (unused numbered paths), `apps/cli/src/main.rs` (conversion/rejection flow), `apps/cli/tests/features_runner.rs` (world), `apps/cli/tests/steps/export_duty_plan_calendar.rs` (steps); run the single-scenario command; paste the zero-exit output:
  ```
  Scenario: A source outside the supported family is rejected
   ✔  Given … ✔  When … ✔  Then no calendar is published
   ✔  And the converter reports a conversion failure naming the source
   ✔  And the invocation reports failure through a non-zero process exit status
   ✔  And a new unused numbered diagnostic report identifies the source and the reason
  1 scenario (1 passed), 6 steps (6 passed)
  GREEN_EXIT=0
  ```
  Interim state recorded honestly: `ConvertRoster::execute` refuses any readable plan (`ConvertError::UnsupportedPlan`) because interpretation and calendar composition arrive with the following scenarios; the publish branch in the CLI is written but first exercised by the calendar-naming scenario.
- [x] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  cargo fmt --all --check            -> FMT OK
  cargo clippy --workspace --all-targets -- -D warnings -> clean
  1 scenario (1 passed), 6 steps (6 passed)   REFACTOR_EXIT=0
  ```
- [x] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A source outside the supported family is rejected`; hash: `8166734`

### 2. Overview and statistics material supplies no dated activities

Use-case constraint: Rule: overview, statistics, footer, and date-only rows supply no dated activities.

- [x] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  Scenario: Overview and statistics material supplies no dated activities
   ✔  Given a supported duty plan whose overview calendar and statistics name activity codes …
   ✔  When the crew member converts it
   ✘  Then the published calendar contains no activity taken from the overview or statistics
      Step panicked. Captured output: not implemented: assert the published calendar carries no overview/statistics activity
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)
  RED_EXIT=101
  ```
  (The Given needed the fixture to exist, so `apps/cli/tests/steps/support.rs` — a positioned-text PDF builder — was added in RED; it also fixes a compile error: `support` had to be imported as `crate::steps::support`.)
- [x] GREEN — replace stubs with real assertions + minimum production code; production files: `crates/duty2cal-pdf/src/lib.rs` (rows clustered by vertical position, column bands derived from the detail-table header, only rows *below* the header are yielded; no coordinate is hard-coded and reading order is never used as structure), `crates/duty2cal-core/src/domain/mod.rs` (`Roster`, `Activity`), `crates/duty2cal-core/src/adapters/ics/mod.rs` (`render` producing a complete VCALENDAR), `crates/duty2cal-core/src/application/mod.rs` (`interpret` + `execute` wiring; the interim `UnsupportedPlan` refusal is replaced), `apps/cli/tests/steps/export_duty_plan_calendar.rs` (Given/Then); run the single-scenario command; paste the zero-exit output:
  ```
  1 scenario (1 passed), 3 steps (3 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  cargo fmt --all --check  -> FMT OK
  cargo clippy --workspace --all-targets -- -D warnings -> clean (elided a needless lifetime)
  full suite -> 51 scenarios (2 passed, 49 failed)
  ```
  Found and fixed a real defect: cucumber executes scenarios in parallel, so the shared per-process working directory made the suite interfere with itself (1 passed, 50 failed). Each scenario now gets its own directory.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: Overview and statistics material supplies no dated activities`; hash: `053163f`

### 3. Footer records supply no dated activities

Use-case constraint: Rule: overview, statistics, footer, and date-only rows supply no dated activities.

- [x] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  Scenario: Footer records supply no dated activities
   ✔  Given … ✔  When the crew member converts it
   ✘  Then the published calendar contains no activity taken from the footer
      Step panicked. Captured output: not implemented: assert the published calendar carries no footer activity
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   RED_EXIT=101
  ```
  Fixture note: the footer places its codes inside the activity column band, so the gap rule is genuinely required — without it the footer row is read as a coded detail row.
- [x] GREEN — replace stubs with real assertions + minimum production code; production files: `crates/duty2cal-pdf/src/lib.rs` (a vertical gap larger than the row pitch ends the detail table; a continuation page must repeat the header), `apps/cli/tests/steps/export_duty_plan_calendar.rs` (Given/Then); run the single-scenario command; paste the zero-exit output:
  ```
  1 scenario (1 passed), 3 steps (3 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  cargo fmt --all --check -> FMT OK
  cargo clippy --workspace --all-targets -- -D warnings -> clean
  full suite -> 51 scenarios (3 passed, 48 failed)
  ```
  Real defects found and fixed here: (1) the gap rule cleared the column areas, and the family-recognition check then read them — so a footer rejected the whole document; recognition is now tracked separately and the column areas are page-local; (2) a failing calendar assertion now surfaces the converter transcript and the diagnostic, which is how (1) was found.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: Footer records supply no dated activities`; hash: `d81f34a`

### 4. A date-only row is not turned into an activity

Use-case constraint: Rule: overview, statistics, footer, and date-only rows supply no dated activities.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A date-only row is not turned into an activity`; hash: `<hash>`

### 5. A mixed plan represents every required activity

Use-case constraint: Minimal guarantee: every required activity is accounted for before publication (no silent omission).

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A mixed plan represents every required activity`; hash: `<hash>`

### 6. All-day codes ignore printed clocks

Use-case constraint: Rule: FREE, OFF, U, FZ are all-day; source clocks do not decide kind or timing.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario Outline: All-day codes ignore printed clocks`; hash: `<hash>`

### 7. Office or seminar duty uses its printed UTC window

Use-case constraint: Rule: TK/DTUSA/EM are timed non-flight duty using their printed UTC interval.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Office or seminar duty uses its printed UTC window`; hash: `<hash>`

### 8. Consulate visa duty uses its printed UTC window

Use-case constraint: Rule: TK/DTUSA/EM are timed non-flight duty using their printed UTC interval.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Consulate visa duty uses its printed UTC window`; hash: `<hash>`

### 9. Extended non-flight duty uses its printed UTC window

Use-case constraint: Rule: TK/DTUSA/EM are timed non-flight duty using their printed UTC interval.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Extended non-flight duty uses its printed UTC window`; hash: `<hash>`

### 10. Standby duty uses its own UTC window

Use-case constraint: Rule: SB/RE/SBX use their own UTC availability window (SBX never the cancelled flight's).

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Standby duty uses its own UTC window`; hash: `<hash>`

### 11. Reserve duty uses its own UTC window

Use-case constraint: Rule: SB/RE/SBX use their own UTC availability window (SBX never the cancelled flight's).

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Reserve duty uses its own UTC window`; hash: `<hash>`

### 12. Consecutive standby days interrupted by a day off stay separate

Use-case constraint: Rule: consecutive SB windows and FREE interruptions stay separate; no continuous block.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Consecutive standby days interrupted by a day off stay separate`; hash: `<hash>`

### 13. Single-day standby uses its own window rather than a cancelled flight

Use-case constraint: Rule: SB/RE/SBX use their own UTC availability window (SBX never the cancelled flight's).

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Single-day standby uses its own window rather than a cancelled flight`; hash: `<hash>`

### 14. Deadhead positioning travel keeps one event and marks the assignment in its title

Use-case constraint: Rule: DH is one travel event with a DH title prefix; zero Blocktime/missing aircraft do not erase it.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Deadhead positioning travel keeps one event and marks the assignment in its title`; hash: `<hash>`

### 15. A dated station-bearing layover row becomes an all-day activity

Use-case constraint: Rule: an explicit dated station-bearing Layover row is an all-day activity.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A dated station-bearing layover row becomes an all-day activity`; hash: `<hash>`

### 16. An unknown required activity code rejects the conversion

Use-case constraint: Extensions 3/4: unknown required code or unmatched fragment rejects the conversion with a diagnostic.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: An unknown required activity code rejects the conversion`; hash: `<hash>`

### 17. A required travel leg without a compatible partner rejects the conversion

Use-case constraint: Extensions 3/4: unknown required code or unmatched fragment rejects the conversion with a diagnostic.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A required travel leg without a compatible partner rejects the conversion`; hash: `<hash>`

### 18. An explicit next-day qualifier moves the endpoint to the following UTC day

Use-case constraint: Timing rule: resolve each endpoint date independently to the unique adjacent Gregorian date; no match fails.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: An explicit next-day qualifier moves the endpoint to the following UTC day`; hash: `<hash>`

### 19. A split across local dates can remain on the same UTC day

Use-case constraint: Timing rule: resolve each endpoint date independently to the unique adjacent Gregorian date; no match fails.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A split across local dates can remain on the same UTC day`; hash: `<hash>`

### 20. Unqualified endpoints that appear to cross midnight roll to the next UTC day

Use-case constraint: Timing rule: the narrow unqualified overnight rollover; equal clocks are not repaired.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Unqualified endpoints that appear to cross midnight roll to the next UTC day`; hash: `<hash>`

### 21. A zero-length travel interval is not repaired

Use-case constraint: Timing rule: the narrow unqualified overnight rollover; equal clocks are not repaired.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A zero-length travel interval is not repaired`; hash: `<hash>`

### 22. A previous-year UTC qualifier resolves across the year boundary

Use-case constraint: Timing rule: resolve each endpoint date independently to the unique adjacent Gregorian date; no match fails.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A previous-year UTC qualifier resolves across the year boundary`; hash: `<hash>`

### 23. A next-month UTC qualifier resolves across the month boundary

Use-case constraint: Timing rule: resolve each endpoint date independently to the unique adjacent Gregorian date; no match fails.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A next-month UTC qualifier resolves across the month boundary`; hash: `<hash>`

### 24. An explicit qualifier with no unique adjacent date rejects the conversion

Use-case constraint: Timing rule: resolve each endpoint date independently to the unique adjacent Gregorian date; no match fails.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: An explicit qualifier with no unique adjacent date rejects the conversion`; hash: `<hash>`

### 25. A known trip identifier is carried across the legs of one roster trip

Use-case constraint: Rule: a known Trip ID carries within one unambiguous roster trip only, never leaking.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A known trip identifier is carried across the legs of one roster trip`; hash: `<hash>`

### 26. An unsupplied optional description field is omitted

Use-case constraint: Descriptive-field selection: approved field order; omit unsupplied fields; combine complementary/equal values.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario Outline: An unsupplied optional description field is omitted`; hash: `<hash>`

### 27. Complementary optional values from compatible fragments combine

Use-case constraint: Descriptive-field selection: approved field order; omit unsupplied fields; combine complementary/equal values.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Complementary optional values from compatible fragments combine`; hash: `<hash>`

### 28. Description fields follow the approved order

Use-case constraint: Descriptive-field selection: approved field order; omit unsupplied fields; combine complementary/equal values.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Description fields follow the approved order`; hash: `<hash>`

### 29. Differing optional values are both retained with their fragment roles

Use-case constraint: Decides D1 (seed Q28): differing optional values are both retained with their fragment roles.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Differing optional values are both retained with their fragment roles`; hash: `<hash>`

### 30. All-day activities use dates with a non-inclusive following-day end

Use-case constraint: Calendar representation: all-day events use a date with a non-inclusive following-day end.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: All-day activities use dates with a non-inclusive following-day end`; hash: `<hash>`

### 31. The published calendar is a valid RFC 5545 file

Use-case constraint: Calendar representation: RFC 5545 byte contract (CRLF, escaping, octet-aware folding, end after start).

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: The published calendar is a valid RFC 5545 file`; hash: `<hash>`

### 32. An identical source snapshot keeps its component identifiers

Use-case constraint: Calendar representation: UIDs are snapshot-scoped, globally distinct, deterministic for the identical snapshot.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: An identical source snapshot keeps its component identifiers`; hash: `<hash>`

### 33. A revised source snapshot has separate component identifiers

Use-case constraint: Calendar representation: UIDs are snapshot-scoped, globally distinct, deterministic for the identical snapshot.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A revised source snapshot has separate component identifiers`; hash: `<hash>`

### 34. The revision timestamp comes from the source's Created footer

Use-case constraint: Calendar representation: DTSTAMP prefers the source Created footer, construction-time fallback.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: The revision timestamp comes from the source's Created footer`; hash: `<hash>`

### 35. Without usable source metadata the construction time is the revision timestamp

Use-case constraint: Calendar representation: DTSTAMP prefers the source Created footer, construction-time fallback.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Without usable source metadata the construction time is the revision timestamp`; hash: `<hash>`

### 36. An occupied calendar name yields a new unused numbered name

Use-case constraint: Minimal guarantee: publish under an unused enumerated name; existing calendars unchanged.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: An occupied calendar name yields a new unused numbered name`; hash: `<hash>`

### 37. An interpretation failure preserves an older diagnostic

Use-case constraint: Minimal guarantee: every failure attempts a new numbered diagnostic; older reports are never claimed as current.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: An interpretation failure preserves an older diagnostic`; hash: `<hash>`

### 38. A publication failure is exposed and a diagnostic is attempted

Use-case constraint: Minimal guarantee: every failure attempts a new numbered diagnostic; older reports are never claimed as current.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A publication failure is exposed and a diagnostic is attempted`; hash: `<hash>`

### 39. An unsavable diagnostic keeps the original failure visible

Use-case constraint: Minimal guarantee: every failure attempts a new numbered diagnostic; older reports are never claimed as current.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: An unsavable diagnostic keeps the original failure visible`; hash: `<hash>`

### 40. An occupied diagnostic name yields a new unused numbered report

Use-case constraint: Minimal guarantee: every failure attempts a new numbered diagnostic; older reports are never claimed as current.

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — replace stubs with real assertions + minimum production code; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: An occupied diagnostic name yields a new unused numbered report`; hash: `<hash>`

### 41. The reference roster converts to the expected calendar

Use-case constraint: Success guarantee plus the operator's 26-event oracle — the whole-file comparison lives inside the Gherkin suite (no parallel golden test).

- [ ] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — generate `apps/cli/tests/fixtures/reference_roster.pdf` (fictional values) and `apps/cli/tests/fixtures/expected/2026-06.ics`; implement the conversion path; list every production file created/modified; run the single-scenario command; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up with no behaviour change; re-run the single-scenario command; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: The reference roster converts to the expected calendar`; hash: `<hash>`

## E2E setup task (before the first @e2e scenario)

- [ ] Configure and prove the e2e runner and its separation from `verify.command`.
  - Create e2e step skeleton at `tests/steps/e2e/export_duty_plan_calendar.rs` (separate from the non-e2e steps).
  - No local infrastructure is required (no server/DB/digital twin); the e2e driver is a subprocess of the built binary.
  - `verify.e2e_command` must be a strict subset of `verify.command`. Prove the filter is real: run both and paste the scenario counts (e2e count strictly less than the full count):
    ```
    <full-suite count output>
    <e2e-filtered count output>
    ```
  - Prove strict mode for the e2e runner as in the setup task.

## E2E scenario

### 42. Crew member converts a supported duty plan and receives a calendar file

Use-case constraint: Success guarantee — complete RFC 5545 calendar beside the source under an unused name, with the actual path reported.

- [ ] RED — remove `@wip` from this scenario only; e2e steps use `unimplemented!()`; run the e2e runner targeting this scenario; paste the non-zero output:
  ```
  <RED output>
  ```
- [ ] GREEN — set up the named infrastructure and assert through the real interface (`std::process::Command` running the built binary; primary assertion on the transcript + process exit status); capture the transcript to `evidence/visual/<scenario-slug>/transcript.txt`; list files touched; run the e2e runner targeting this scenario; paste the zero-exit output:
  ```
  <GREEN output>
  ```
- [ ] REFACTOR — clean up e2e code; runner still passes; paste the zero-exit output:
  ```
  <REFACTOR output>
  ```
- [ ] COMMIT — subject `test(e2e): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Crew member converts a supported duty plan and receives a calendar file`; hash: `<hash>`

## Completion

- [ ] Both `./run-tests.sh` and `./run-tests.sh --e2e` exit 0 on the full suite.
- [ ] Coverage: `./measure-coverage.sh` writes distinct `coverage/non-e2e-cobertura.xml` and `coverage/e2e-cobertura.xml`; `./merge-coverages.sh` writes `coverage/cobertura-coverage.xml` with a non-zero production probe.
- [ ] `./run-tests.sh --e2e` scenario count is strictly less than `./run-tests.sh` (recorded in the e2e setup task).
