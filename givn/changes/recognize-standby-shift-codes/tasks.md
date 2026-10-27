# Tasks: recognize-standby-shift-codes

Release note (commit subject): Standby shift assignments printed as SB_F_CF, SB_M_CF or SB_S_CF are recognized as standby availability windows, so a supported duty plan that contains them converts without warnings for those rows.

Owning use case: [Export a duty-plan calendar](specs/export-duty-plan-calendar/usecase.md) (`export-duty-plan-calendar`). Confirmed Persona: `crew-member--personal-calendar-owner` (accuracy/privacy lens). `contributor--web-reviewer` is not affected (no page code, copy, state or interaction changes); `project-maintainer--release-owner` is unaffected (no release-evidence change).

Single-scenario command (used by every RED/GREEN/REFACTOR below): `./run-tests.sh --name "<scenario title>"`. Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`. The delta adds no `@e2e` scenario and no browser scenario.

Use-case constraints carried into every scenario task (from the owning use case's Rules and Extensions):

- `SB_F_CF`, `SB_M_CF` and `SB_S_CF` are SB standby windows for Standby Früh (early), Standby Mittel (middle) and Standby Spät (late), recognized by their exact printed code only — never by a prefix, a family rule, or the `CF` letters, whose meaning is not asserted.
- Each recognized shift supplies its own printed UTC start/end and station, is represented exactly like SB, and is titled by the printed code, never by the shift name.
- A representable shift row produces no conversion warning and no warning entry, and a plan with no other warning produces no warning report.
- A shift row that cannot build a positive printed interval — no printed times, one printed endpoint, or a non-positive window — is a conversion error exactly like SB/RE/SBX: no all-day fallback, no calendar, and the failure report lists the row under `Errors:`.
- Any other code, including another `SB_<letter>_CF` string, keeps the unexpected-code fallback and its warning.
- Known codes keep today's semantics and win over the route branch; recognition is data in the existing literal set.
- Every detail row is processed in one run; every error and warning is collected and reported.
- The calendar text of every already-recognized activity stays byte-identical; component identity stays snapshot plus ordinal.
- Private plans are never committed; every fixture is generated.

Shared design constraints (from `design.md`; every scenario task inherits these):

- The only production edit is the literal availability-code set (`AVAILABILITY_CODES`, `crates/duty2cal-core/src/application/mod.rs`) widening from three to six entries plus the `DutyEntry::Availability` doc comment (`crates/duty2cal-core/src/domain/mod.rs`), made in task 1.
- Title, interval, station, diagnostic and report paths are untouched; the ICS serializer renders through the entry's accessors.
- The golden oracles `apps/cli/tests/fixtures/expected/2098-06.ics` and `2026-06.ics` stay byte-identical: the reference and equivalence plans print none of the three shift codes.
- New step definitions belong in `apps/cli/tests/steps/export_duty_plan_calendar.rs`; no delta step copy and no archive rebinding.
- The new domain and durable-doc edits are already recorded: `givn/specs/export-duty-plan-calendar/usecase.md`, `docs/arc42/12-glossary.md`, `docs/arc42/11-risks-and-technical-debt.md`, `docs/evidence/supported-duty-plan.md`.

Interaction coverage (checked before writing tasks): the permanent inventory holds two entries — supply a duty plan to the executable, and supply a duty plan through the page and save the offered calendar — matched by the permanent `@e2e` scenarios named in `design.md`'s Interaction Coverage Matrix. This change adds no consumer action; a shift row is an input variant of the existing conversion action.

## Setup task — environment, runner, strict mode

- [ ] Confirm the toolchain, runner, strict mode, and the pre-implementation baseline.
  - Runner: `apps/cli/tests/features_runner.rs` calls `DutyWorld::cucumber().fail_on_skipped().run_and_exit(root)`. A missing or skipped step fails the run.
  - `verify.command: "./run-tests.sh"`; `verify.e2e_command: "./run-tests.sh --e2e"` (strict subset). No new environment, digital twin or framework is introduced.
  - Strictness proof against the new delta, before any scenario work: a single-scenario run selected one scenario and failed on the undefined step, exit non-zero.

  ```
  Feature: Export a duty-plan calendar
    Scenario: A standby shift row without printed times rejects the conversion
     ✘  Given a supported duty plan dated 2099-11-19 with activity code SB_F_CF at station FRA and no printed UTC times
        Step failed:
        Defined: tmp/tmp.zVLkNcw0tP/f0003.feature:39:5
        Step doesn't match any function
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)

  thread 'main' panicked at ... cucumber-0.23.0/src/cucumber.rs:1243:13:
  1 step failed
  error: test failed, to rerun pass `-p duty2cal --test features_runner`
  EXIT=1
  ```
  - Baseline: the four delta scenarios are `@wip`; the full suite is expected to fail only on them until their tasks are done.

## Scenario tasks

### 1. Standby shift codes keep their own printed windows

Design constraints (this scenario):
- `SB_F_CF` 03:30-13:00 on 2099-11-15, `SB_M_CF` 08:00-15:30 on 2099-11-16 and `SB_S_CF` 14:00-22:00 on 2099-11-17, all at FRA, become three availability activities titled by their printed codes with their own windows.
- Exactly zero warnings, so no `parse-warnings.txt` is written; exit zero.
- Before the production edit, each row reaches `UnrecognizedActivity` with a warning: the window/station/date steps pass, the zero-warning step fails. This is the observable proof that the fallback is gone; the constant widening plus the fixture and assertion steps are this task's edits.

- [x] RED: run the single-scenario command. The fixture and assertion steps were bound first, so the failure is behavioural (three fallback warnings), not undefined-step noise.
  ```
  Feature: Export a duty-plan calendar
    Scenario: Standby shift codes keep their own printed windows
     ✔  Given a supported duty plan with SB_F_CF 03:30 to 13:00 on 2099-11-15, SB_M_CF 08:00 to 15:30 on 2099-11-16, and SB_S_CF 14:00 to 22:00 on 2099-11-17 at station FRA
     ✔  When the crew member converts it
     ✔  Then the calendar contains three timed activities titled SB_F_CF, SB_M_CF and SB_S_CF on 2099-11-15, 2099-11-16 and 2099-11-17 from 03:30 to 13:00, 08:00 to 15:30 and 14:00 to 22:00 UTC at FRA
     ✘  And the converter reports no conversion warning
        Step panicked. Captured output: warnings were reported: ["row 1 (2099-11-15): unrecognized activity code \"SB_F_CF\"; converted as a timed activity using its printed UTC window 03:30-13:00", "row 2 (2099-11-16): unrecognized activity code \"SB_M_CF\"; ...", "row 3 (2099-11-17): unrecognized activity code \"SB_S_CF\"; ..."]
  [Summary]
  1 feature
  1 scenario (1 failed)
  4 steps (3 passed, 1 failed)
  EXIT=1
  ```
- [x] GREEN: widened `AVAILABILITY_CODES` to `["SB", "RE", "SBX", "SB_F_CF", "SB_M_CF", "SB_S_CF"]`, updated the `DutyEntry::Availability` doc comment, and bound the fixture and assertion steps. Compiled, then ran the single-scenario command.
  ```
  Feature: Export a duty-plan calendar
    Scenario: Standby shift codes keep their own printed windows
     ✔  Given a supported duty plan with SB_F_CF 03:30 to 13:00 on 2099-11-15, SB_M_CF 08:00 to 15:30 on 2099-11-16, and SB_S_CF 14:00 to 22:00 on 2099-11-17 at station FRA
     ✔  When the crew member converts it
     ✔  Then the calendar contains three timed activities titled SB_F_CF, SB_M_CF and SB_S_CF on 2099-11-15, 2099-11-16 and 2099-11-17 from 03:30 to 13:00, 08:00 to 15:30 and 14:00 to 22:00 UTC at FRA
     ✔  And the converter reports no conversion warning
     ✔  And no warning report is published
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change; `@wip` removed from this scenario and the run repeated.
  ```
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] COMMIT: one revision, release note as subject, scenario title in the body; record the hash.
  ```
  COMMIT: <pending>
  ```

### 2. A standby-like code outside the three shift codes still warns

Design constraints (this scenario):
- `SB_X_CF` with both endpoints and station FRA stays on the unexpected-code fallback: one timed activity titled `SB_X_CF`, one warning naming row 1 and 2099-11-18, warning text names the code and states the row was converted as a timed activity using its printed UTC window 08:00-15:30.
- Proves recognition is the exact three strings, not a prefix or family rule; no production change beyond task 1, and the golden oracles are untouched. The new warning-sentence assertion is this task's test edit.

- [ ] RED: run the single-scenario command.
  ```
  <paste output>
  ```
- [ ] GREEN: implement the step definition for the warning sentence; no production change. Compile, then run the single-scenario command.
  ```
  <paste output>
  ```
- [ ] REFACTOR: no behaviour change; re-run and paste.
  ```
  <paste output>
  ```
- [ ] COMMIT: one revision, release note as subject, scenario title in the body; record the hash.
  ```
  COMMIT: <hash>
  ```

### 3. Consecutive standby shift days interrupted by a day off stay separate

Design constraints (this scenario):
- `SB_F_CF` 03:30-13:00 on 2099-11-15 and 2099-11-17 with a `FREE` row on 2099-11-16 becomes two separate timed `SB_F_CF` activities at FRA plus one all-day FREE activity on 2099-11-16; no warning, no warning report.
- Each row keeps its own window; no continuous block, no all-day standby. The new Given step is this task's test edit; no production change beyond task 1.

- [ ] RED: run the single-scenario command.
  ```
  <paste output>
  ```
- [ ] GREEN: implement the Given step for the two shift days and the FREE row; no production change. Compile, then run the single-scenario command.
  ```
  <paste output>
  ```
- [ ] REFACTOR: no behaviour change; re-run and paste.
  ```
  <paste output>
  ```
- [ ] COMMIT: one revision, release note as subject, scenario title in the body; record the hash.
  ```
  COMMIT: <hash>
  ```

### 4. A standby shift row without printed times rejects the conversion

Design constraints (this scenario):
- An `SB_F_CF` row at station FRA on 2099-11-19 with both UTC cells blank is a conversion error: no calendar is published, no warning line, the failure names `a required timing could not be resolved`, the failure report lists the row under `Errors:`, exit non-zero.
- Before the production edit this row publishes an all-day activity plus a warning, so the scenario is RED; after recognition it takes the same error path as an SB row without a window. No all-day fallback is invented.
- No new diagnostic wording and no new report section; the new Given step is this task's test edit. No production change beyond task 1.

- [ ] RED: run the single-scenario command.
  ```
  <paste output>
  ```
- [ ] GREEN: implement the Given step (detail row with both UTC cells blank and a printed station); no production change. Compile, then run the single-scenario command.
  ```
  <paste output>
  ```
- [ ] REFACTOR: no behaviour change; re-run and paste.
  ```
  <paste output>
  ```
- [ ] COMMIT: one revision, release note as subject, scenario title in the body; record the hash.
  ```
  COMMIT: <hash>
  ```

## Closeout

- [ ] Remove `@wip` from every delta scenario and run the full suite `./run-tests.sh`; paste the summary.
- [ ] Run `./run-tests.sh --e2e`; paste the summary (no delta `@e2e` scenario, so the permanent set is unchanged).
- [ ] Run `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `givn lint --change recognize-standby-shift-codes`, and `scripts/support-statement-check.sh`; paste results.
- [ ] Confirm every step phrase bound by the delta is also bound in the permanent test tree (`givn archive` re-checks).
- [ ] Real-plan smoke: convert `private/MLP-2026-11.pdf` with a fixed construction clock; confirm the six shift warnings are gone, the `.ics` events are byte-identical to the pre-change capture, no warning report is written, and exit status is zero. Record the comparison.
