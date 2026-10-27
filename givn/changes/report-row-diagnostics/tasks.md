# Tasks: report-row-diagnostics

Release note (commit subject): Every row of a supplied duty plan is reported in one run: an activity code the converter does not recognize is converted from its printed times and station with a warning, a row that cannot be represented at all still fails the conversion, and both lists appear in the report file and on the conversion page.

Owning use case: [Export a duty-plan calendar](specs/export-duty-plan-calendar/usecase.md) (`export-duty-plan-calendar`). Confirmed Personas for this change: `crew-member--personal-calendar-owner` (accuracy lens), `contributor--web-reviewer` (page lens). `project-maintainer--release-owner` is the pre-existing use-case lens and is unaffected.

Single-scenario command (used by every RED/GREEN/REFACTOR below): `./run-tests.sh --name "<scenario title>"`. Browser scenario: `./run-tests.sh --browser --name "<scenario title>"`. Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`.

Use-case constraints carried into every scenario task (from the owning use case's Rules and Extensions):

- Every detail row is processed in one run; every error and every warning is collected and reported. No row is silently dropped.
- A conversion error means the row cannot be represented: the conversion fails, no calendar is published, and a numbered `parse-error` report carries every error and warning.
- A conversion warning means the row was represented by a documented fallback: the calendar is published with a numbered `parse-warnings` report. Exit status is zero.
- An unrecognized code with both printed UTC endpoints and no route becomes a timed activity titled by the printed code, located at the printed station, using the printed window; with neither endpoint it becomes an all-day activity titled by the code; with exactly one endpoint it is an error.
- Known codes keep today's semantics and win over the route branch. A code the converter does not recognize that carries a route remains travel, including a flight number.
- A silent layout row (no activity code and no route) produces no event and no warning. A row that prints a station but neither a code nor a route is an error.
- Every timed interval ends strictly after it starts; the narrow unqualified overnight rollover applies to every timed row, including the fallback.
- The calendar text of every already-recognized activity stays byte-identical; component identity stays snapshot plus ordinal.
- Private plans are never committed: every fixture is generated.
- The page shows the warnings in the ready state and the errors and warnings in the failure state, offers the matching report, writes nothing, sends no request after load, and stores nothing.

Shared design constraints (from `design.md`; every scenario task inherits these):

- `Diagnostic { severity, row, date, message }`; `row` is the raw detail-table position (1-based, counting silent layout rows); `date` is the row's own or carried date.
- `diagnostic_line()` renders `row <n> (<date>): <message>` or `row <n>: <message>`; report entries are `- ` + that line.
- Failure report: `Source PDF:`, `Reason:` (first error in plan order), then `Errors:` and `Warnings:` sections that have entries, one per line, LF, trailing newline.
- Warning report: `Source PDF:`, then `Warnings:`, one per line, LF, trailing newline.
- Files: `<stem>.parse-warnings.txt` and `<stem>.parse-error.txt`, enumerated `-1`, `-2`, … on collision; create-new, never replace.
- Page: reference as drawn (operator approval 2026-10-27). Ready heading `Calendar ready`; ready copy singular/plural; ready status `Calendar ready for <name>.` / `Your calendar is ready to save. <n> rows were converted with warnings.`; ready-state `Choose another duty plan`; count-based failure reason; `Errors`/`Warnings` lists; failure `Choose another plan`.

Interaction coverage (checked before writing tasks): the permanent inventory holds two entries — supply a duty plan to the executable, and supply a duty plan through the page and save the offered calendar — matched by the permanent `@e2e` scenarios named in `design.md`'s Interaction Coverage Matrix. This change adds no consumer action, so the delta has no `@e2e` scenario. The browser scenarios below are regular coverage of the second action.

## Setup task — environment, runner, strict mode

- [x] Confirm the toolchain, runner, strict mode, and the pre-implementation baseline.
  - Runner: `apps/cli/tests/features_runner.rs` calls `DutyWorld::cucumber().fail_on_skipped().run_and_exit(root)`. A missing or skipped step fails the run.
  - `verify.command: "./run-tests.sh"`; `verify.e2e_command: "./run-tests.sh --e2e"` (strict subset: `@e2e and not @browser`). Browser: `./run-tests.sh --browser` (builds `web/pkg` and `dist/site`, then `@browser`).
  - Toolchain path note: the rustup shim in `~/.cargo/bin` is broken in this environment; the runs below use `PATH=/etc/profiles/per-user/dan/bin:$PATH`.
  - Strictness proof, before any scenario work: `./run-tests.sh --name "An unrecognized activity code with a printed window becomes a timed activity with a warning"` selected one scenario and failed on the undefined step, exit non-zero.

  ```
  Feature: Export a duty-plan calendar
    Scenario: An unrecognized activity code with a printed window becomes a timed activity with a warning
     ✘  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF at station ABC and printed UTC start 08:00 and end 15:30
        Step doesn't match any function
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)

  thread 'main' panicked ... 1 step failed
  error: test failed, to rerun pass `-p duty2cal --test features_runner`
  ```
  - E2E setup: the e2e runner is the existing `./run-tests.sh --e2e`; the browser driver is the existing Playwright driver over `web/e2e/driver.mjs`. No new environment or digital twin is introduced.
  - Baseline: the remaining delta scenarios are `@wip`; the full suite is expected to fail only on them until their tasks are done.

## Scenario tasks

### 1. An unrecognized activity code with a printed window becomes a timed activity with a warning

Design constraints (this scenario):
- Code-only row with station ABC and printed UTC 08:00–15:30 becomes `DutyEntry::UnrecognizedActivity { interval: Some(..) }`; summary `ZZ_M_CF`, location `ABC`, `DTSTART/DTEND` 08:00–15:30 UTC on 2099-06-07.
- Exactly one warning at raw row 1 with the date; `parse-warnings.txt` written; no error; exit zero.
- Warning message: `unrecognized activity code "ZZ_M_CF"; converted as a timed activity using its printed UTC window 08:00-15:30`.

- [ ] RED: run the single-scenario command. The `Given` is undefined, so strict mode fails.
  ```
  PENDING
  ```
- [ ] GREEN: implement the diagnostics model, the fallback, the warning report, and the CLI output. Production files: `crates/duty2cal-core/src/application/mod.rs`, `crates/duty2cal-core/src/domain/mod.rs`, `crates/duty2cal-core/src/adapters/ics/mod.rs` (accessor arms only), `apps/cli/src/fs.rs`, `apps/cli/src/main.rs`. Test file: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change; targeted run still zero.
  ```
  PENDING
  ```
- [ ] COMMIT: release-note subject; body names this scenario. Revision `PENDING`.

### 2. An unrecognized activity code without printed times becomes an all-day activity with a warning

Design constraints:
- Code-only row with no endpoints becomes an all-day `UnrecognizedActivity`; summary `ZZ_M_CF`, `DTSTART;VALUE=DATE` and non-inclusive next-day end.
- One warning at the row; no interval is invented.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: add the all-day fallback arm and the `Given` fixture. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 3. An unrecognized activity code with a single printed endpoint rejects the conversion

Design constraints:
- One printed endpoint and no route is an error, message names the code and the single endpoint; no calendar; failure report lists the row under `Errors:`; exit non-zero.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: add the one-endpoint error arm, the failure-report builder, and the CLI row-failure path. Production: `crates/duty2cal-core/src/application/mod.rs`, `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 4. Several unrecognized rows are all reported in one conversion

Design constraints:
- Three code-only rows with windows become three activities and three warnings in one run; no early stop; report has one entry per warned row.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: make the interpretation collect every row and warning. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 5. Several failing rows are all reported with the warnings in one failure report

Design constraints:
- Two unrepresentable rows plus one representable unknown row: no calendar, two errors, one warning, all three in the failure report; exit non-zero.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: collect every error and warning; discard entries when any error exists. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 6. Error and warning entries are distinguishable in the failure report

Design constraints:
- Failure report carries `Errors:` with one entry and `Warnings:` with one entry; `Source PDF:` and `Reason:` present; the two sections are ordered errors first.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: implement `failure_report_text` section ordering and the step assertions. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 7. A second conversion with warnings uses the next unused warning-report name

Design constraints:
- Existing `roster.parse-warnings.txt` and `roster.parse-warnings-1.txt` stay unchanged; the new report is `roster.parse-warnings-2.txt`; `Kind::Warning` enumeration uses create-new.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: add `Kind::Warning` and its spelling. Production: `apps/cli/src/fs.rs`, `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 8. The executable prints each warning beside the new calendar path

Design constraints:
- Transcript names the calendar path, carries one warning line per warned row, and names the warning report; exit zero.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: print `Warning: <diagnostic_line>` for every warning and `Warning report: <path>`. Production: `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 9. A conversion without warnings publishes no warning report

Design constraints:
- Recognized-only plan publishes the calendar and no `parse-warnings` file; transcript reports no warning; exit zero.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: write the warning report only when warnings exist. Production: `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 10. A warning report that cannot be published keeps the calendar and the warnings visible

Design constraints:
- Calendar publication succeeds; warning-report publication fails (dangling symlink yields `EEXIST`); exit zero; transcript still carries the warning line and says no new warning report was saved.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: make warning-report failure non-fatal while keeping the warning visible. Production: `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 11. The page shows the warnings and offers both files when a plan converts with warnings

Design constraints:
- Reference as drawn: `Calendar ready`; plural copy; status `Your calendar is ready to save. 1 row was converted with a warning.`; warning list; calendar and `2026-06.parse-warnings.txt` offers; ready-state `Choose another duty plan`; no save claim; no cross-origin request.
- Visual baseline, axe, and the `ready-with-warnings` screenshot per viewport.

- [ ] RED: `./run-tests.sh --browser --name "<title>"` fails on the undefined `Given` or the missing state.
  ```
  PENDING
  ```
- [ ] GREEN: extend the WASM result, the page DOM/JS/CSS, and the driver. Production: `ffi/wasm/src/lib.rs`, `web/index.html`, `web/app.js`, `web/styles.css`, `web/e2e/driver.mjs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`, `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 12. The page lists the errors and the warnings when a plan cannot be converted

Design constraints:
- Failure heading; count-based reason; `Errors` and `Warnings` lists; no calendar; `2026-06.parse-error.txt` offer; `Choose another plan`; visual baseline, axe, failure screenshot.

- [ ] RED: browser scenario fails on the undefined `Given` or the missing lists.
  ```
  PENDING
  ```
- [ ] GREEN: failure lists, count-based reason, diagnostic offer. Production: `web/index.html`, `web/app.js`, `web/styles.css`, `ffi/wasm/src/lib.rs`, `web/e2e/driver.mjs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`, `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 13. The page shows no warning list when the plan converts without warnings

Design constraints:
- Ready state, no warning list, no warning-report offer; calendar offer only.

- [ ] RED: browser scenario fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: hide the warning block and skip the warning offer when there are no warnings. Production: `web/app.js`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 14. A timed activity code with a route and an incomplete printed interval rejects the conversion

Design constraints:
- Known timed code `TK` + route + only Start reaches the final catch-all error `a route row carries an incomplete printed interval`; no calendar; report lists it.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: add the exhaustive catch-all error arm. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 15. An unrecognized activity code with a route and a single printed endpoint rejects the conversion

Design constraints:
- Unrecognized code + route + one endpoint reaches the same route-incomplete error; no calendar.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: the catch-all covers this shape; add the fixture. Production: none beyond task 14. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 16. A known timed activity code that also prints a route stays a timed non-flight activity

Design constraints:
- Known codes win over the route branch: `TK` + ABC→DEF + 08:00–12:00 is a timed non-flight duty, not travel; no warning.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: keep the known-code branch before the route branch; add the fixture and assertion. Production: none beyond the classifier order. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 17. A travel leg with a flight number produces no warning

Design constraints:
- Route-bearing flight leg converts silently; no `parse-warnings.txt`; transcript carries no warning.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: route-before-fallback order; assert no warning. Production: none beyond the classifier. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 18. A silent layout row produces no warning

Design constraints:
- A row with no activity code and no route is skipped before date reading; no event, no warning, no warning report.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: keep the silent-row skip; add the assertion. Production: none beyond the skip. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 19. A silent layout row before a warned row keeps its place in the row numbering

Design constraints:
- The warning for a row after a silent layout row names raw row 2, not filtered row 1.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: number diagnostics by raw table position. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 20. An unmatched departure and a later warning are reported in plan order

Design constraints:
- An orphan departure error discovered at the end is sorted before a later-row warning; the departure error is the `Reason:` line.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: sort diagnostics by row; choose the first error for `Reason:`. Production: `crates/duty2cal-core/src/application/mod.rs`, `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 21. An unrecognized activity code with an unqualified window across midnight keeps it as the following UTC day

Design constraints:
- The fallback reuses `row_interval`: 23:10–01:05 unqualified becomes 23:10 → next-day 01:05 UTC with a warning.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: use `row_interval` for the fallback. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 22. An unrecognized activity code with an equal printed window rejects the conversion

Design constraints:
- Equal clocks are never repaired; the fallback row is a non-positive-interval error; no calendar.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: `row_interval`'s positive check applies to the fallback. Production: none beyond task 21. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 23. A mis-paired travel leg reports both the arrival error and the orphan departure

Design constraints:
- A mismatched arrival and an orphan departure produce two errors, in row order.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: report the arrival mismatch and every remaining pending departure. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 24. An unrecognized activity code's activity carries no description

Design constraints:
- The fallback publishes the code, station and interval only; no `DESCRIPTION` line, because the source prints no approved descriptive field on that row.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: `description_fields()` is empty for `UnrecognizedActivity`; assert the absence. Production: `crates/duty2cal-core/src/domain/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 25. The warning report identifies the source and carries a warnings section

Design constraints:
- The warning report begins `Source PDF: <name>`, carries `Warnings:`, one entry per warned row, LF, trailing newline.

- [ ] RED: single-scenario run fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: assert the exact report header and section. Production: none beyond `warning_report_text`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 26. The page states how many rows were converted with warnings

Design constraints:
- Ready-with-warnings status uses the exact singular/plural sentence and count from the result; the ready-state way-back button is visible.

- [ ] RED: browser scenario fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: render the count sentence and the ready-state reset. Production: `web/app.js`, `web/index.html`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 27. The page offers a warning report that names the source and carries the warnings

Design constraints:
- The offered `2026-06.parse-warnings.txt` content is the exact warning report text: source name and `Warnings:` section.

- [ ] RED: browser scenario fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: the page offers `warningReport` unchanged from the converter. Production: `web/app.js`, `ffi/wasm/src/lib.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 28. A second attempt clears the previous attempt's warning list

Design constraints:
- A second attempt clears the previous warning list and offers before reading the new plan; a warning-free second plan shows no warning list.

- [ ] RED: browser scenario fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: `clearAttempt` empties and hides both list containers and withdraws prior offers. Production: `web/app.js`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

### 29. The page keeps the calendar and the warnings when it cannot offer the warning report

Design constraints:
- A failed warning-report offer keeps the calendar offer and the visible warnings, and shows `No warning report was offered.`; no failure state.

- [ ] RED: browser scenario fails on the undefined `Given`.
  ```
  PENDING
  ```
- [ ] GREEN: catch only the warning-report offer failure. Production: `web/app.js`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  PENDING
  ```
- [ ] REFACTOR: no behaviour change.
  ```
  PENDING
  ```
- [ ] COMMIT: body names this scenario. Revision `PENDING`.

## Closeout

- [ ] Every step phrase bound by the delta is bound in the permanent test tree (no delta step copy).
- [ ] Full suite `./run-tests.sh` exits 0.
- [ ] E2E suite `./run-tests.sh --e2e` exits 0; scenario count strictly less than the full run.
- [ ] Browser suite `./run-tests.sh --browser` exits 0 and commits the declared evidence.
- [ ] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `givn lint --change report-row-diagnostics` pass.
