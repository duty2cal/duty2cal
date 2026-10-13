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

- [x] RED — remove `@wip` from this scenario only; unimplemented steps use `unimplemented!()`; run the single-scenario command; paste the non-zero output:
  ```
  Scenario: A date-only row is not turned into an activity
   ✔  Given … ✔  When the crew member converts it
   ✘  Then the published calendar contains no absence or layover for that date
      Step panicked. Captured output: not implemented: assert no absence or layover activity was invented for the date-only row
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   RED_EXIT=101
  ```
- [x] GREEN — **no production change was needed**: interpretation already skips a detail row whose activity cell is empty, so a date-only row contributes nothing instead of being invented as FREE/OFF/layover. Files: `apps/cli/tests/steps/export_duty_plan_calendar.rs` (Given/Then). Run the single-scenario command; paste the zero-exit output:
  ```
  1 scenario (1 passed), 3 steps (3 passed)   GREEN_EXIT=0
  ```
  (This is a legitimately reuse-covered scenario; the evidence records that no production file changed, rather than manufacturing a change to fit the template.)
- [x] REFACTOR — no cleanup required; re-run the single-scenario command; paste the zero-exit output:
  ```
  cargo fmt --all --check -> FMT OK
  cargo clippy --workspace --all-targets -- -D warnings -> clean
  full suite -> 51 scenarios (4 passed, 47 failed)
  ```
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: A date-only row is not turned into an activity`; hash: `fb164c8`

### 5. A mixed plan represents every required activity

Use-case constraint: Minimal guarantee: every required activity is accounted for before publication (no silent omission).

- [x] RED — ran before the behaviour existed: the scenario(s) failed.
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Production: classification of the confirmed codes, the domain model, the CalendarSink/Clock ports, VEVENT serialization, the stable source fingerprint, and CLI wiring.
  ```
  1 scenario (1 passed), 3 steps (3 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 18 passed, 33 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `d59cd70`

### 6. All-day codes ignore printed clocks

Use-case constraint: Rule: FREE, OFF, U, FZ are all-day; source clocks do not decide kind or timing.

- [x] RED — ran before the behaviour existed: the scenario(s) failed.
  ```
  4 scenarios (4 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; the classification landed with scenario 5.
  ```
  4 scenarios (4 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 18 passed, 33 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `d477233`

### 7. Office or seminar duty uses its printed UTC window

Use-case constraint: Rule: TK/DTUSA/EM are timed non-flight duty using their printed UTC interval.

- [x] RED — ran before the behaviour existed: the scenario(s) failed.
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; the classification landed with scenario 5.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 18 passed, 33 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `d477233`

### 8. Consulate visa duty uses its printed UTC window

Use-case constraint: Rule: TK/DTUSA/EM are timed non-flight duty using their printed UTC interval.

- [x] RED — ran before the behaviour existed: the scenario(s) failed.
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; the classification landed with scenario 5.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 18 passed, 33 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `d477233`

### 9. EM duty uses its printed UTC window

Use-case constraint: Rule: TK/DTUSA/EM are timed non-flight duty using their printed UTC interval.

- [x] RED — ran before the behaviour existed: the scenario(s) failed.
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; the classification landed with scenario 5.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 18 passed, 33 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `d477233`

### 10. Standby duty uses its own UTC window

Use-case constraint: Rule: SB/RE/SBX use their own UTC availability window (SBX never the cancelled flight's).

- [x] RED — ran before the behaviour existed: the scenario(s) failed.
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; the classification landed with scenario 5.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 18 passed, 33 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `d477233`

### 11. Reserve duty uses its own UTC window

Use-case constraint: Rule: SB/RE/SBX use their own UTC availability window (SBX never the cancelled flight's).

- [x] RED — ran before the behaviour existed: the scenario(s) failed.
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; the classification landed with scenario 5.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 18 passed, 33 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `d477233`

### 12. Consecutive standby days interrupted by a day off stay separate

Use-case constraint: Rule: consecutive SB windows and FREE interruptions stay separate; no continuous block.

- [x] RED — ran before the behaviour existed: the scenario(s) failed.
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; the classification landed with scenario 5.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 18 passed, 33 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `d477233`

### 13. Single-day standby uses its own window rather than a cancelled flight

Use-case constraint: Rule: SB/RE/SBX use their own UTC availability window (SBX never the cancelled flight's).

- [x] RED — ran before the behaviour existed: the scenario(s) failed.
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; the classification landed with scenario 5.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 18 passed, 33 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `d477233`

### 14. Deadhead positioning travel keeps one event and marks the assignment in its title

Use-case constraint: Rule: DH is one travel event with a DH title prefix; zero Blocktime/missing aircraft do not erase it.

- [x] RED — ran before the behaviour existed: the scenario(s) failed.
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; the classification landed with scenario 5.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 18 passed, 33 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `d477233`

### 15. A dated station-bearing layover row becomes an all-day activity

Use-case constraint: Rule: an explicit dated station-bearing Layover row is an all-day activity.

- [x] RED — ran before the behaviour existed: the scenario(s) failed.
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; the classification landed with scenario 5.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 18 passed, 33 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `d477233`

### 16. An unknown required activity code rejects the conversion

Use-case constraint: Extensions 3/4: unknown required code or unmatched fragment rejects the conversion with a diagnostic.

- [x] RED — ran before the steps existed: strict mode failed the scenario as undefined (`Step doesn't match any function`).
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; production: `domain/timing.rs` (qualifier -> unique adjacent Gregorian date carrying month/year), split-leg fragment pairing, the narrow unqualified rollover, and rejection of unmatched fragments, zero-length intervals and unmatched qualifiers.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 27 passed, 24 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `75551d1` (shared batch commit).

### 17. A required travel leg without a compatible partner rejects the conversion

Use-case constraint: Extensions 3/4: unknown required code or unmatched fragment rejects the conversion with a diagnostic.

- [x] RED — ran before the steps existed: strict mode failed the scenario as undefined (`Step doesn't match any function`).
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; production: `domain/timing.rs` (qualifier -> unique adjacent Gregorian date carrying month/year), split-leg fragment pairing, the narrow unqualified rollover, and rejection of unmatched fragments, zero-length intervals and unmatched qualifiers.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 27 passed, 24 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `75551d1` (shared batch commit).

### 18. An explicit next-day qualifier moves the endpoint to the following UTC day

Use-case constraint: Timing rule: resolve each endpoint date independently to the unique adjacent Gregorian date; no match fails.

- [x] RED — ran before the steps existed: strict mode failed the scenario as undefined (`Step doesn't match any function`).
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; production: `domain/timing.rs` (qualifier -> unique adjacent Gregorian date carrying month/year), split-leg fragment pairing, the narrow unqualified rollover, and rejection of unmatched fragments, zero-length intervals and unmatched qualifiers.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 27 passed, 24 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `75551d1` (shared batch commit).

### 19. A split across local dates can remain on the same UTC day

Use-case constraint: Timing rule: resolve each endpoint date independently to the unique adjacent Gregorian date; no match fails.

- [x] RED — ran before the steps existed: strict mode failed the scenario as undefined (`Step doesn't match any function`).
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; production: `domain/timing.rs` (qualifier -> unique adjacent Gregorian date carrying month/year), split-leg fragment pairing, the narrow unqualified rollover, and rejection of unmatched fragments, zero-length intervals and unmatched qualifiers.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 27 passed, 24 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `75551d1` (shared batch commit).

### 20. Unqualified endpoints that appear to cross midnight roll to the next UTC day

Use-case constraint: Timing rule: the narrow unqualified overnight rollover; equal clocks are not repaired.

- [x] RED — ran before the steps existed: strict mode failed the scenario as undefined (`Step doesn't match any function`).
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; production: `domain/timing.rs` (qualifier -> unique adjacent Gregorian date carrying month/year), split-leg fragment pairing, the narrow unqualified rollover, and rejection of unmatched fragments, zero-length intervals and unmatched qualifiers.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 27 passed, 24 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `75551d1` (shared batch commit).

### 21. A zero-length travel interval is not repaired

Use-case constraint: Timing rule: the narrow unqualified overnight rollover; equal clocks are not repaired.

- [x] RED — ran before the steps existed: strict mode failed the scenario as undefined (`Step doesn't match any function`).
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; production: `domain/timing.rs` (qualifier -> unique adjacent Gregorian date carrying month/year), split-leg fragment pairing, the narrow unqualified rollover, and rejection of unmatched fragments, zero-length intervals and unmatched qualifiers.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 27 passed, 24 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `75551d1` (shared batch commit).

### 22. A previous-year UTC qualifier resolves across the year boundary

Use-case constraint: Timing rule: resolve each endpoint date independently to the unique adjacent Gregorian date; no match fails.

- [x] RED — ran before the steps existed: strict mode failed the scenario as undefined (`Step doesn't match any function`).
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; production: `domain/timing.rs` (qualifier -> unique adjacent Gregorian date carrying month/year), split-leg fragment pairing, the narrow unqualified rollover, and rejection of unmatched fragments, zero-length intervals and unmatched qualifiers.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 27 passed, 24 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `75551d1` (shared batch commit).

### 23. A next-month UTC qualifier resolves across the month boundary

Use-case constraint: Timing rule: resolve each endpoint date independently to the unique adjacent Gregorian date; no match fails.

- [x] RED — ran before the steps existed: strict mode failed the scenario as undefined (`Step doesn't match any function`).
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; production: `domain/timing.rs` (qualifier -> unique adjacent Gregorian date carrying month/year), split-leg fragment pairing, the narrow unqualified rollover, and rejection of unmatched fragments, zero-length intervals and unmatched qualifiers.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 27 passed, 24 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `75551d1` (shared batch commit).

### 24. An explicit qualifier with no unique adjacent date rejects the conversion

Use-case constraint: Timing rule: resolve each endpoint date independently to the unique adjacent Gregorian date; no match fails.

- [x] RED — ran before the steps existed: strict mode failed the scenario as undefined (`Step doesn't match any function`).
  ```
  1 scenario (1 failed)   RED_EXIT non-zero
  ```
- [x] GREEN — Step fixtures and assertions; production: `domain/timing.rs` (qualifier -> unique adjacent Gregorian date carrying month/year), split-leg fragment pairing, the narrow unqualified rollover, and rejection of unmatched fragments, zero-length intervals and unmatched qualifiers.
  ```
  1 scenario (1 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 27 passed, 24 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: <title>`; hash: `75551d1` (shared batch commit).

### 25. A known trip identifier is carried across the legs of one roster trip

Use-case constraint: Rule: a known Trip ID carries within one unambiguous roster trip only, never leaking.

- [x] RED — the assertion was in place while no DESCRIPTION existed; the calendar held 3 events and 0 carried the trip.
  ```
  1 scenario (1 failed), 4 steps (3 passed, 1 failed)   RED_EXIT=101
  ```
- [x] GREEN — production: `Row::carried_trip` tracks the last printed Trip ID and hands it to later travel legs only (so it cannot leak into unrelated duties); `DutyEntry::description_fields` selects the approved field order (Trip so far); the ICS writer emits `DESCRIPTION`.
  ```
  1 scenario (1 passed), 4 steps (4 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; scenario still 4/4; full suite 28 passed, 23 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: A known trip identifier is carried across the legs of one roster trip`; hash: `2b63075`

### 26. An unsupplied optional description field is omitted

Use-case constraint: Descriptive-field selection: approved field order; omit unsupplied fields; combine complementary/equal values.

- [x] RED — all seven example rows failed: the description rendered only the trip, so every supplied field was missing.
  ```
  7 scenarios (7 failed), 21 steps (14 passed, 7 failed)   RED_EXIT=101
  ```
- [x] GREEN — production: `description_fields` selects the approved seven fields in order, omitting any the source did not supply. Fixture support gained the aircraft, blocktime, timezone, layover and tags columns.
  ```
  7 scenarios (7 passed), 21 steps (21 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — real defect fixed: relaxed label-prefix matching made `A/C` resolve to `Activity`; matching now requires an exact name, a real boundary, or an exact normalised name. fmt/clippy clean; suite 35 passed, 16 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: An unsupplied optional description field is omitted`; hash: `8054e9e`

### 27. Complementary optional values from compatible fragments combine

Use-case constraint: Descriptive-field selection: approved field order; omit unsupplied fields; combine complementary/equal values.

- [x] RED — the merged leg carried only the departure fragment's aircraft (`A/C ABC`); the arrival fragment's Blocktime was lost.
  ```
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   RED_EXIT=101
  ```
- [x] GREEN — production: `merge_travel` takes each optional field from the departure fragment and falls back to the arrival fragment, so complementary values combine and a repeated value stays represented once.
  ```
  1 scenario (1 passed), 3 steps (3 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; suite 36 passed, 15 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: Complementary optional values from compatible fragments combine`; hash: `b89e222`

### 28. Description fields follow the approved order

Use-case constraint: Descriptive-field selection: approved field order; omit unsupplied fields; combine complementary/equal values.

- [x] RED (fault injection) — the approved ordering was already delivered by scenario 26, so the scenario passed on first run. The field order was then deliberately reversed to prove the assertion discriminates:
  ```
  Step panicked: Report (LT) 07:30 appears out of the approved order: Tags 5_MFL | … | Trip T42
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   FAULT_EXIT=101
  ```
  The approved order was restored; the injection was never committed.
- [x] GREEN — no production change needed; the scenario passes against the approved order.
  ```
  1 scenario (1 passed), 3 steps (3 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — nothing to clean; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; suite 37 passed, 14 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: Description fields follow the approved order`; hash: `fcb95e0`

### 29. Differing optional values are both retained with their fragment roles

Use-case constraint: Decides D1 (seed Q28): differing optional values are both retained with their fragment roles.

- [x] RED — the description carried only the departure fragment's report time (`Report (LT) 07:30`); the arrival fragment's `08:00` was dropped with no fragment role.
  ```
  1 scenario (1 failed), 4 steps (3 passed, 1 failed)   RED_EXIT=101
  ```
- [x] GREEN — production: `FieldValue` distinguishes a value supplied once (or equally by both fragments) from two differing values; the description renders both, each labelled with its supplying fragment, and neither computes the interval. This is the D1 decision (seed Q28).
  ```
  1 scenario (1 passed), 4 steps (4 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — allowed the large travel variant (boxing would add an indirection per entry). fmt/clippy clean; scenario 27 still green; suite 38 passed, 13 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: Differing optional values are both retained with their fragment roles`; hash: `1a5d0ae`

### 30. All-day activities use dates with a non-inclusive following-day end

Use-case constraint: Calendar representation: all-day events use a date with a non-inclusive following-day end.

- [x] RED (fault injection) — the writer already emitted the non-inclusive following-day end, so the scenario passed on first run. Making the all-day end the same day proved the assertion discriminates:
  ```
  Step panicked: the all-day activity does not end on 2099-01-13
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   FAULT_EXIT=101
  ```
  The next-day end was restored; the injection was never committed.
- [x] GREEN — no production change needed; the scenario passes.
  ```
  1 scenario (1 passed), 3 steps (3 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — nothing to clean; fmt/clippy clean; suite 39 passed, 12 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: All-day activities use dates with a non-inclusive following-day end`; hash: `3d6ad9b`

### 31. The published calendar is a valid RFC 5545 file

Use-case constraint: Calendar representation: RFC 5545 byte contract (CRLF, escaping, octet-aware folding, end after start).

- [x] RED — text values were unescaped: the Tags value `5_MFL;X,Y` reached the calendar raw.
  ```
  1 scenario (1 failed), 4 steps (3 passed, 1 failed)   RED_EXIT=101
  ```
- [x] GREEN — production: content lines escape TEXT values (`\;`, `\,`, `\\`, newlines) and fold at 75 octets with a single-space continuation, cutting on character boundaries so a UTF-8 character is never split.
  ```
  1 scenario (1 passed), 6 steps (6 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — folding exposed that a reader must unfold first; the test helper now joins continuation lines. fmt/clippy clean; suite 40 passed, 11 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: The published calendar is a valid RFC 5545 file`; hash: `d9c1d9d`

### 32. An identical source snapshot keeps its component identifiers

Use-case constraint: Calendar representation: UIDs are snapshot-scoped, globally distinct, deterministic for the identical snapshot.

- [x] RED (fault injection) — identity was already scoped to the source fingerprint, so the scenario passed on first run. Tying the identifier to the output name (the behaviour the design forbids) proved the assertion discriminates:
  ```
  Step panicked: the identical source produced different component identifiers
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   FAULT_EXIT=101
  ```
  The injection was never committed; the output-name independence was restored.
- [x] GREEN — no production change needed; both calendars carry identical identifiers.
  ```
  1 scenario (1 passed), 3 steps (3 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — nothing to clean; the converter invocation was factored into `run_converter` for reuse. fmt/clippy clean; suite 41 passed, 10 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: An identical source snapshot keeps its component identifiers`; hash: `29f8b74`

### 33. A revised source snapshot has separate component identifiers

Use-case constraint: Calendar representation: UIDs are snapshot-scoped, globally distinct, deterministic for the identical snapshot.

- [x] RED (fault injection) — identity was already snapshot-scoped, so the scenario passed on first run. Replacing it with a content-derived identifier (the rejected corresponding-activity approach) proved the assertion discriminates:
  ```
  Step panicked: the revised snapshot reused the identifier FB568ABC-DEF-20990607@duty2cal
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   FAULT_EXIT=101
  ```
  The snapshot-scoped identifier was restored; the injection was never committed.
- [x] GREEN — no production change needed; every activity of the revision carries an identifier distinct from the original.
  ```
  1 scenario (1 passed), 3 steps (3 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — nothing to clean; fmt/clippy clean; suite 42 passed, 9 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: A revised source snapshot has separate component identifiers`; hash: `a42e728`

### 34. The revision timestamp comes from the source's Created footer

Use-case constraint: Calendar representation: DTSTAMP prefers the source Created footer, construction-time fallback.

- [x] RED — the published `DTSTAMP` was the construction time, not the footer's `20990110T123400Z`.
  ```
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   RED_EXIT=101
  ```
- [x] GREEN — production: the driven port returns a `SourceDocument` carrying the raw detail content and the source's own revision instant parsed from the `Created … UTC` footer; the use case prefers that metadata and falls back to the construction clock. `domain::timing` now owns the shared ISO date and clock parsers.
  ```
  1 scenario (1 passed), 3 steps (3 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; suite 43 passed, 8 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: The revision timestamp comes from the source's Created footer`; hash: `4bbe9a0`

### 35. Without usable source metadata the construction time is the revision timestamp

Use-case constraint: Calendar representation: DTSTAMP prefers the source Created footer, construction-time fallback.

- [x] RED — with no usable footer the `DTSTAMP` was the wall clock, not the controlled `20990111T080000Z`.
  ```
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   RED_EXIT=101
  ```
- [x] GREEN — production: the CLI clock honours `DUTY2CAL_CONSTRUCTION_CLOCK`, so the construction instant behind the revision-timestamp fallback is controllable; activity dates are unaffected.
  ```
  1 scenario (1 passed), 4 steps (4 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — fmt/clippy clean; scenario 34 still green; suite 44 passed, 7 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: Without usable source metadata the construction time is the revision timestamp`; hash: `77b6864`

### 36. An occupied calendar name yields a new unused numbered name

Use-case constraint: Minimal guarantee: publish under an unused enumerated name; existing calendars unchanged.

- [x] RED (fault injection) — enumeration already chose unused names, so the scenario passed on first run. Writing the ordinary name unconditionally (overwriting) proved the assertion discriminates:
  ```
  Step panicked: no calendar at …/roster-2.ics: No such file or directory
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   FAULT_EXIT=101
  ```
  The enumeration was restored; the injection was never committed.
- [x] GREEN — no production change needed; the complete calendar is published as `roster-2.ics` and both existing calendars are byte-identical afterwards.
  ```
  1 scenario (1 passed), 4 steps (4 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — nothing to clean; fmt/clippy clean; suite 45 passed, 6 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: An occupied calendar name yields a new unused numbered name`; hash: `89bee7c`

### 37. An interpretation failure preserves an older diagnostic

Use-case constraint: Minimal guarantee: every failure attempts a new numbered diagnostic; older reports are never claimed as current.

- [x] RED (fault injection) — the numbered-diagnostic allocation already existed, so the scenario passed on first run. Writing the fixed diagnostic path unconditionally proved the assertion discriminates:
  ```
  Step panicked: no numbered diagnostic at …/roster.parse-error-1.txt
  1 scenario (1 failed), 5 steps (4 passed, 1 failed)   FAULT_EXIT=101
  ```
  The enumeration was restored; the injection was never committed.
- [x] GREEN — no production change needed; the new report is `roster.parse-error-1.txt` naming `roster.pdf` and the current reason, and the earlier report is byte-identical.
  ```
  1 scenario (1 passed), 6 steps (6 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — nothing to clean; fmt/clippy clean; suite 46 passed, 5 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: An interpretation failure preserves an older diagnostic`; hash: `53ddb9d`

### 38. A publication failure is exposed and a diagnostic is attempted

Use-case constraint: Minimal guarantee: every failure attempts a new numbered diagnostic; older reports are never claimed as current.

- [x] RED (fault injection) — the failure path already behaved correctly, so the scenario passed on first run. Swallowing the write error and reporting success proved the assertion discriminates:
  ```
  Step panicked: no conversion failure was reported: Written: …/roster.ics
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   FAULT_EXIT=101
  ```
  The failure reporting was restored; the injection was never committed.
- [x] GREEN — no production change needed; the failure is exposed, a new numbered report is written, and the existing calendar survives unchanged.
  ```
  1 scenario (1 passed), 5 steps (5 passed)   GREEN_EXIT=0
  ```
  Fixture technique: the candidate calendar name is a dangling symbolic link — it reports as non-existent so it is chosen, and writing it fails while the directory stays writable, which keeps the diagnostic saveable and distinguishes this scenario from the unsavable-diagnostic one.
- [x] REFACTOR — nothing to clean; fmt/clippy clean; suite 47 passed, 4 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: A publication failure is exposed and a diagnostic is attempted`; hash: `970f46d`

### 39. An unsavable diagnostic keeps the original failure visible

Use-case constraint: Minimal guarantee: every failure attempts a new numbered diagnostic; older reports are never claimed as current.

- [x] RED (fault injection) — the unsavable path already behaved correctly, so the scenario passed on first run. Printing the older report's path as if current proved the assertion discriminates:
  ```
  Step panicked: the unsaved diagnostic was not stated
  1 scenario (1 failed), 4 steps (3 passed, 1 failed)   FAULT_EXIT=101
  ```
  The honest failure message was restored; the injection was never committed.
- [x] GREEN — no production change needed; the original failure stays visible, the unsaved diagnostic is stated, and the older report is byte-identical and never named as current.
  ```
  1 scenario (1 passed), 4 steps (4 passed)   GREEN_EXIT=0
  ```
  Fixture technique: a dangling symbolic link as the diagnostic candidate makes the report unsavable while an older report stays in place.
- [x] REFACTOR — nothing to clean; fmt/clippy clean; suite 48 passed, 3 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: An unsavable diagnostic keeps the original failure visible`; hash: `4ce0999`

### 40. An occupied diagnostic name yields a new unused numbered report

Use-case constraint: Minimal guarantee: every failure attempts a new numbered diagnostic; older reports are never claimed as current.

- [x] RED (fault injection) — the report enumeration already existed, so the scenario passed on first run. Limiting it to a single candidate proved the assertion discriminates:
  ```
  Step panicked: no diagnostic at …/roster.parse-error-2.txt
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   FAULT_EXIT=101
  ```
  The enumeration was restored; the injection was never committed.
- [x] GREEN — no production change needed; the new report is `roster.parse-error-2.txt` naming `roster.pdf` and the current reason, and both older reports are byte-identical.
  ```
  1 scenario (1 passed), 4 steps (4 passed)   GREEN_EXIT=0
  ```
- [x] REFACTOR — nothing to clean; fmt/clippy clean; suite 49 passed, 2 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: An occupied diagnostic name yields a new unused numbered report`; hash: `e0c075d`

### 41. The reference roster converts to the expected calendar

Use-case constraint: Success guarantee plus the operator's 26-event oracle — the whole-file comparison lives inside the Gherkin suite (no parallel golden test).

- [x] RED — no expected calendar fixture existed, so the comparison step failed.
  ```
  1 scenario (1 failed), 3 steps (2 passed, 1 failed)   RED_EXIT=101
  ```
- [x] GREEN — the reference plan is built from the operator's 26-event example month (June 2026) and the committed expected calendar is compared after CRLF-to-LF and folding normalisation; the plan sets the construction clock to the oracle's stated DTSTAMP `20260610T000000Z`.
  ```
  1 scenario (1 passed), 3 steps (3 passed)   GREEN_EXIT=0
  ```
  The committed fixture was verified line by line against the operator's oracle before being committed: 26 events, matching summaries, all-day activities with following-day non-inclusive ends, and the three overnight legs resolving to the next UTC day.
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; suite 50 passed, 1 failed.
- [x] COMMIT — subject `feat(export-duty-plan-calendar): …` (release note); body `Scenario: The reference roster converts to the expected calendar`; hash: `af93202`

## E2E setup task (before the first @e2e scenario)

- [x] Configure and prove the e2e runner and its separation from `verify.command`.
  - e2e step module: `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` (separate from the in-process steps; the shared `When` is reused rather than registered twice).
  - No local infrastructure is required (no server/DB/digital twin); the e2e driver is a subprocess of the built binary.
  - `verify.e2e_command` is a strict subset of `verify.command`; both counts were run:
    ```
    ./run-tests.sh         -> 51 scenarios (51 passed)
    ./run-tests.sh --e2e   ->  1 scenario  ( 1 passed)
    ```
  - Strict mode for the e2e runner is the same `fail_on_skipped()` configuration proven in the setup task (an undefined e2e step fails the run).

## E2E scenario

### 42. Crew member converts a supported duty plan and receives a calendar file

Use-case constraint: Success guarantee — complete RFC 5545 calendar beside the source under an unused name, with the actual path reported.

- [x] RED — the e2e steps existed as `unimplemented!()` stubs; the e2e runner failed the scenario.
  ```
  Step panicked: not implemented: supply a supported Released Roster duty plan
  1 scenario (1 failed)   RED_EXIT=101
  ```
- [x] GREEN — implemented through the real interface: the built binary is run as a subprocess and the primary assertions are the reported path and the process exit status, plus the complete calendar beside the source under an unused name. The scenario commits its terminal transcript to `evidence/visual/crew-member-converts-a-supported-duty-plan/transcript.txt` (run-specific directory replaced by a placeholder; the exit status is recorded alongside).
  ```
  1 scenario (1 passed), 5 steps (5 passed)   GREEN_EXIT=0
  ```
  Transcript: `Written: <scenario-dir>/roster.ics` / `exit status: 0`.
- [x] REFACTOR — `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean; full suite 51 scenarios, 51 passed.
- [x] COMMIT — subject `test(e2e): Convert a supported Released Roster duty-plan PDF …` (release note); body `Scenario: Crew member converts a supported duty plan and receives a calendar file`; hash: `26c9002`

## Completion

- [x] Both `./run-tests.sh` and `./run-tests.sh --e2e` exit 0 on the full suite.
  ```
  ./run-tests.sh         -> 51 scenarios (51 passed)
  ./run-tests.sh --e2e   ->  1 scenario  ( 1 passed)
  ```
- [x] Coverage: `./measure-coverage.sh` writes distinct `coverage/non-e2e-cobertura.xml` and `coverage/e2e-cobertura.xml`; `./merge-coverages.sh` writes `coverage/cobertura-coverage.xml` with non-zero production probes.
  ```
  merged 694 lines (662 covered) -> coverage/cobertura-coverage.xml
  files: apps/cli/src/{fs,main}.rs, crates/duty2cal-core/src/{adapters/ics,application,domain,domain/timing}.rs, crates/duty2cal-pdf/src/lib.rs
  probes non-zero: crates/duty2cal-core/src/domain/timing.rs (non-e2e), apps/cli/src/fs.rs (e2e)
  ```
- [x] `./run-tests.sh --e2e` scenario count (1) is strictly less than `./run-tests.sh` (51), recorded in the e2e setup task.

## Re-target: the real export layout (found after implementation)

Running the finished converter against the operator's five real duty plans (local, uncommitted) fails for
every one of them: `a required activity could not be resolved: the detail table carries no From column`,
and no calendar is produced. The committed suite is green only against fixtures shaped to this parser.

Measured divergences from the real export (structural facts only):

- dates are `DD Ddd` (for example `07 Sun`) and the month/year comes from the document heading `Month: June 2026`;
- the date cell is empty on continuation rows, so the current local date must carry across rows;
- the header is several wrapped rows, with `Timezone` and `Difference` on separate rows at the same x;
- the designator is split across `Pos` (`LU`) and `Activity` (`742`) and is printed as `LU742`;
- a layover row is `Layover: <station>` in the row text, not an activity code with a station column;
- all-day rows carry `00:00 00:00 ==`;
- the revision footer is `Created DDMonYYYY HH:MM (UTC) by <producer>`;
- the overview grid and the statistics tables interleave with the detail table on the same page.

- [x] Add the local validation script `scripts/validate-local-corpus.sh`: reads `private/*.pdf`, converts copies in a temporary directory, prints a per-file verdict and a summary, writes nothing into `private/` and nothing into the repository, and is not part of CI. Result after the reader was adapted: **5 passed, 0 failed**.
- [x] Adapt the reader to the real layout: heading-derived month/year, `DD Ddd` date cells, local-date carry across continuation rows, multi-row header bands, `Pos`+`Activity` designator, `Layover: <station>` rows, `Created DDMonYYYY HH:MM (UTC)` footer, and region-aware row selection so the overview/statistics tables are never activity sources. The bands now run from label to label across the wrapped header (`Date (LT)`, `Trip ID`, `Start (UTC)`, `Timezone Difference`) instead of merging by whitespace gap, and a row carries its position in the reachable-row list, which the fragment pairing needs.
- [x] The five real duty plans convert: `scripts/validate-local-corpus.sh` reports **5 passed, 0 failed**, and the plan for the operator's example month yields its 26 events.
- [x] Re-shape the reference fixture to the producer's layout (heading `Month: June 2098`, `DD Ddd` date cells, `Pos`/`Activity` printed apart): the golden end-to-end scenario now exercises the producer's spelling, its content is unchanged, and only the source-identity fingerprint in the 26 UIDs moved because the fixture bytes changed.
- [x] Re-shape every scenario fixture: the fixture builder prints the heading and the `DD Ddd` date spelling, so all 51 scenarios face the production layout, and all fixture data is invented (2090s, Moon and Mars destinations named after their large areas and craters, a producer-neutral footer). The expected calendar moved to `apps/cli/tests/fixtures/expected/2098-06.ics`.
- [x] Record the producer-layout reading rules in design.md (heading month, wrapped header bands, continuation rows, designator, layover marker, all-day rows, footer, row identity) with the local validation script as their standing check.
- [x] Re-run design-review over the updated design.md and the reader changes: three blocking findings (committed coverage for the producer-layout rules, header recognition across wrapped rows, the stale arc42 `R-003`) and three advisory ones are fixed, the operator answered the one open question (one producer template now, a second expected soon), and `design-review.md` carries `DESIGN-REVIEW: PASS`.
- [x] Re-drive the scenarios through the re-shaped fixtures. Fault injection carries the RED evidence the local corpus could not. Run it as: `git show 09a4297~1:crates/duty2cal-pdf/src/lib.rs > crates/duty2cal-pdf/src/lib.rs`, `git show 09a4297~1:crates/duty2cal-core/src/application/mod.rs > crates/duty2cal-core/src/application/mod.rs`, `./run-tests.sh` (output: `41 steps failed`), then `git checkout -- crates/`. The pre-change reader is the commit before the reader rework, so the state is reproducible even though both landed together; with the fixed reader, 51 of 51 scenarios pass, the e2e smoke passes, and `scripts/validate-local-corpus.sh` reports 5 passed, 0 failed. The committed suite therefore catches the reader-shaped-to-fixtures regression that the earlier green suite could not.
