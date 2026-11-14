# Tasks: recognize-cockpit-duty-codes

Release note (commit subject): Recognize the cockpit-crew codes U1, OFF_2, MED, WBT_*, EMCRM and EBTF3* by their confirmed meanings, so those rows publish without the unknown-duty warning.

Owning capability: [Export a duty-plan calendar](../../specs/export-duty-plan-calendar/usecase.md).

Single-scenario run command: `./run-tests.sh --name "<scenario title>"` for a regular scenario. An outline's named run selects every Example, so the outline batch is verified by the full suite.
Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`.

## Setup task — runner, strict mode, baseline

- [x] Confirm the existing Gherkin runner and strict mode; this change adds no runner configuration.
  - `apps/cli/tests/features_runner.rs` sets `.fail_on_skipped()`.
  - `./run-tests.sh` assembles the permanent corpus and the active delta; `./run-tests.sh --e2e` selects `@e2e`.
  - Baseline before the batch: 148 scenarios; after the delta: 160 scenarios, 679 steps. The E2E scope remains 2 scenarios.
  - `givn lint --change recognize-cockpit-duty-codes` is clean.

## Behavior batch: cockpit-code-recognition

Design constraints:
- `crates/duty2cal-core/src/application/mod.rs`:
  - `ALL_DAY_CODES` is `FREE`, `OFF`, `U`, `FZ`, `U1`, `OFF_2`.
  - `NON_FLIGHT_CODES` is `TK`, `DTUSA`, `EM`, `MED`, `EMCRM`.
  - `NON_FLIGHT_PREFIXES` is `WBT_`, `EBTF3`; the suffix is preserved and not interpreted.
  - `classify` uses `is_non_flight_code(code)`.
- The existing `DutyEntry::AllDay` and `DutyEntry::NonFlightDuty` representations are reused. No new event shape, description, UID, DTSTAMP or report changes.
- A code outside the exact members and prefixes keeps `DutyEntry::UnrecognizedActivity`, the `unknown duty: <code>` title and one conversion warning.
- `U1`/`OFF_2` ignore printed clocks. `MED`/`WBT_*`/`EMCRM`/`EBTF3*` use their own printed UTC window and printed station; the printed code is the title. The printed station is not translated into a home, clinic or company address.
- Step definitions: `apps/cli/tests/steps/export_duty_plan_calendar.rs` gains `the calendar contains one timed activity titled "<title>" from HH:MM to HH:MM UTC on YYYY-MM-DD`; the existing all-day and warning steps are reused.
- Evidence: `docs/evidence/supported-duty-plan.md` names the confirmed codes and scenarios; `scripts/support-statement-check.sh` requires the cockpit codes; `scripts/tests/support-statement.bats` refuses a record that drops `MED`.

### 1. All-day codes ignore printed clocks

Use-case constraint: `U1` is a vacation variant and `OFF_2` a day-off variant; both are all-day and keep the printed code as the title.

- [x] GREEN — the modified outline includes `U1` and `OFF_2` with printed clocks; all six examples pass.
- [x] COMMIT: b01b3be -- Batch: cockpit-code-recognition

### 2. Medical and training duties use their printed UTC window

Use-case constraint: `MED`, `WBT_*`, `EMCRM` and `EBTF3*` are timed non-flight duties using their own printed UTC window and station, titled by the printed code, with no warning.

- [x] GREEN — the new outline covers `MED`, `WBT_EM`, `WBT_GR`, `WBT_XY`, `EMCRM`, `EBTF3A`, `EBTF3B`, `EBTF3C`; each asserts the exact title, the printed window and no warning.
- [x] COMMIT: b01b3be -- Batch: cockpit-code-recognition

### 3. A code outside the confirmed training families still warns

Use-case constraint: an unconfirmed code never acquires a meaning from a resembling spelling.

- [x] GREEN — `WBTX` and `EBTF4A` publish `unknown duty: <code>` with one warning each.
- [x] COMMIT: b01b3be -- Batch: cockpit-code-recognition

## Existing E2E coverage

- [x] No new `@e2e` scenario is added. The new rows are variants of the executable conversion action. `./run-tests.sh --e2e` runs 2 scenarios and passes.

## Closeout

- [x] `./run-tests.sh` and `./run-tests.sh --e2e` exit 0.
- [x] `scripts/support-statement-check.sh` exits 0 and `bats scripts/tests/support-statement.bats` passes (9 tests).
- [x] `givn lint --change recognize-cockpit-duty-codes` exits clean.
- [x] The six cabin-crew calendars in `private/mk/` are byte-identical; all five cockpit-crew plans exit 0; August and September no longer write a warning report, July keeps `AMV`/`REP`, October keeps `S5`, November keeps `SBAO`.
- [x] `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` are clean.
