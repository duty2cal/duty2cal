# Tasks: export-duty-plan-calendar

Release note (commit subject): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.

Owning use case: [Export a duty-plan calendar](specs/export-duty-plan-calendar/usecase.md) (`export-duty-plan-calendar`). Confirmed Personas: `crew-member--personal-calendar-owner` (accuracy/privacy lens), `project-maintainer--release-owner` (release-evidence lens). Actors in scenarios stay distinct from Personas.

Single-scenario run command (used by every RED/GREEN/REFACTOR below): `./run-tests.sh --name "<scenario title>"`.
Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`.

## Setup task — runner, strict mode, step skeleton

- [x] Configure the Gherkin runner and prove strict mode BEFORE any scenario.
  - `tests/features_runner.rs` is a `harness = false` cucumber-rs target. It reads the feature root from `GIVN_FEATURES` (default `tests/features`), sets `.fail_on_skipped()`, and uses `.run_and_exit(root)` so the process exit code reflects failures.
  - cucumber-rs 0.23 takes a single feature root, so `./run-tests.sh` assembles every `.feature` from `givn/specs/**` and `givn/changes/*/specs/**` into a temporary directory and exports `GIVN_FEATURES`. (design.md amended; design-review re-run.)
  - Step skeleton: `tests/steps/export_duty_plan_calendar.rs` (`tests/steps/mod.rs`), one file per capability.
  - Proof-of-strictness (undefined step must fail): temporary `tests/features/strict-mode-proof.feature` with an undefined step; runner output:
    ```
    Feature: Strict-mode proof
      Scenario: An undefined step fails the run
       ✘  Given a step that has no definition
          Step doesn't match any function
    [Summary]
    1 feature
    1 scenario (1 failed)
    1 step (1 failed)
    thread 'main' panicked ...: 1 step failed
    RUNNER_EXIT=101
    ```
    The temporary feature was deleted after the proof.
- [x] Compile-proof the pinned extraction dependency (R-001): `cargo build` compiled `pdfplumber-core 0.1.0`, `pdfplumber-parse 0.1.0`, and `pdfplumber 0.1.0` on rustc 1.98.1 / cargo 1.98.0 (ephemeral `nix-shell -p rustc cargo`); the crate resolves on crates.io 0.1.0 (not yanked, MIT OR Apache-2.0) and builds with no native C/C++ dependency.
  ```
  Compiling pdfplumber-core v0.1.0
  Compiling pdfplumber-parse v0.1.0
  Compiling pdfplumber v0.1.0
  Compiling duty2ical v0.1.0
  Finished `dev` profile ... in 25.36s
  ```
- [x] Prove the e2e filter is a strict subset: full suite reports 50 scenarios (41 scenarios + 9 outline example rows); `./run-tests.sh --e2e` reports 1 scenario.
  ```
  ./run-tests.sh          -> 1 feature, 50 scenarios (50 failed)
  ./run-tests.sh --e2e    -> 1 feature,  1 scenario  ( 1 failed)
  ```
  (All failures are the expected RED state: no step definitions exist yet and strict mode fails undefined steps.)

## Scenarios (non-@e2e first, in feature order)

### 1. A source outside the supported family is rejected

Use-case constraint: Extensions 2: an unreadable/unsupported source fails explicitly with a numbered diagnostic.

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
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: A source outside the supported family is rejected`; hash: `<hash>`

### 2. Overview and statistics material supplies no dated activities

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
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Overview and statistics material supplies no dated activities`; hash: `<hash>`

### 3. Footer records supply no dated activities

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
- [ ] COMMIT — subject `feat(export-duty-plan-calendar): Convert a supported Released Roster duty-plan PDF into one complete RFC 5545 calendar file saved beside the source under an unused name, or fail explicitly with a numbered diagnostic, never overwriting an existing calendar or report.`; body `Scenario: Footer records supply no dated activities`; hash: `<hash>`

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

### 41. Crew member converts a supported duty plan and receives a calendar file

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
