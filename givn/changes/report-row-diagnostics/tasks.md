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

- [x] RED: run the single-scenario command. The `Given` is undefined, so strict mode fails.
  ```
  Feature: Export a duty-plan calendar
    Scenario: An unrecognized activity code with a printed window becomes a timed activity with a warning
     ✘  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF at station ABC and printed UTC start 08:00 and end 15:30
        Step doesn't match any function
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)

  thread 'main' panicked ...: 1 step failed
  error: test failed, to rerun pass `-p duty2cal --test features_runner`
  EXIT=1
  ```
- [x] GREEN: implement the diagnostics model, the fallback, the warning report, and the CLI output. Production files: `crates/duty2cal-core/src/application/mod.rs`, `crates/duty2cal-core/src/domain/mod.rs`, `crates/duty2cal-core/src/adapters/ics/mod.rs` (accessor arms only), `apps/cli/src/fs.rs`, `apps/cli/src/main.rs`. Test file: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: An unrecognized activity code with a printed window becomes a timed activity with a warning
     ✔  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF at station ABC and printed UTC start 08:00 and end 15:30
     ✔  When the crew member converts it
     ✔  Then the calendar contains one timed activity titled "ZZ_M_CF" from 08:00 to 15:30 UTC on 2099-06-07 at ABC
     ✔  And the converter reports one conversion warning naming detail row 1 and the date 2099-06-07
     ✔  And the warning names the activity code "ZZ_M_CF"
     ✔  And the converter publishes a warning report beside the source
     ✔  And the converter reports no conversion error
     ✔  And the invocation reports success through its process exit status
  [Summary]
  1 feature
  1 scenario (1 passed)
  8 steps (8 passed)
  EXIT=0
  ```
  The success phrase was already bound in `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`; its
  transcript evidence write moved to that file's `the converter reports the path of the newly published
  calendar` step so the phrase has one permanent binding.
- [x] REFACTOR: no behaviour change; targeted run still zero.
  ```
  1 feature
  1 scenario (1 passed)
  8 steps (8 passed)
  EXIT=0
  ```
- [x] COMMIT: release-note subject; body names this scenario. Revision `f90cea2`.

### 2. An unrecognized activity code without printed times becomes an all-day activity with a warning

Design constraints:
- Code-only row with no endpoints becomes an all-day `UnrecognizedActivity`; summary `ZZ_M_CF`, `DTSTART;VALUE=DATE` and non-inclusive next-day end.
- One warning at the row; no interval is invented.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: An unrecognized activity code without printed times becomes an all-day activity with a warning
     ✘  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and no printed UTC times and no route
        Step doesn't match any function
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: add the all-day fallback arm and the `Given` fixture. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and no printed UTC times and no route
   ✔  When the crew member converts it
   ✔  Then the calendar contains one all-day activity for 2099-06-07 titled "ZZ_M_CF"
   ✔  And that activity carries no timed interval
   ✔  And the converter reports one conversion warning naming detail row 1 and the date 2099-06-07
   ✔  And the converter publishes a warning report beside the source
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `d23996b`.

### 3. An unrecognized activity code with a single printed endpoint rejects the conversion

Design constraints:
- One printed endpoint and no route is an error, message names the code and the single endpoint; no calendar; failure report lists the row under `Errors:`; exit non-zero.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: An unrecognized activity code with a single printed endpoint rejects the conversion
     ✘  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and a printed UTC start but no end and no route
        Step doesn't match any function
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: add the one-endpoint error arm, the failure-report builder, and the CLI row-failure path. Production: `crates/duty2cal-core/src/application/mod.rs`, `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and a printed UTC start but no end and no route
   ✔  When the crew member converts it
   ✔  Then no calendar is published
   ✔  And the converter reports a conversion failure naming "ZZ_M_CF"
   ✔  And the failure report lists that row under "Errors:"
   ✔  And the invocation reports failure through a non-zero process exit status
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0

  (First attempt failed on the step's own section parser: the phrase quotes `"Errors:"`, so the
  title needed its trailing colon trimmed.)
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `04792e3`.

### 4. Several unrecognized rows are all reported in one conversion

Design constraints:
- Three code-only rows with windows become three activities and three warnings in one run; no early stop; report has one entry per warned row.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: Several unrecognized rows are all reported in one conversion
     ✘  Given a supported duty plan with three detail rows carrying unrecognized activity codes, each with printed UTC start and end
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: make the interpretation collect every row and warning. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan with three detail rows carrying unrecognized activity codes, each with printed UTC start and end
   ✔  When the crew member converts it
   ✔  Then the calendar contains one activity for each of the three rows
   ✔  And the converter reports three conversion warnings
   ✔  And the converter reports no conversion error
   ✔  And the warning report lists one entry for each warned row
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `f4522a8`.

### 5. Several failing rows are all reported with the warnings in one failure report

Design constraints:
- Two unrepresentable rows plus one representable unknown row: no calendar, two errors, one warning, all three in the failure report; exit non-zero.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: Several failing rows are all reported with the warnings in one failure report
     ✘  Given a supported duty plan with two detail rows that cannot be represented and one unrecognized row that can
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: collect every error and warning; discard entries when any error exists. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan with two detail rows that cannot be represented and one unrecognized row that can
   ✔  When the crew member converts it
   ✔  Then no calendar is published
   ✔  And the converter reports two conversion errors
   ✔  And the converter reports one conversion warning
   ✔  And the failure report lists both errors and the warning
   ✔  And the invocation reports failure through a non-zero process exit status
  [Summary]
  1 feature
  1 scenario (1 passed)
  7 steps (7 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  7 steps (7 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `d8a12b9`.

### 6. Error and warning entries are distinguishable in the failure report

Design constraints:
- Failure report carries `Errors:` with one entry and `Warnings:` with one entry; `Source PDF:` and `Reason:` present; the two sections are ordered errors first.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: Error and warning entries are distinguishable in the failure report
     ✘  Given a supported duty plan with one detail row that cannot be represented and one unrecognized row that can
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: implement `failure_report_text` section ordering and the step assertions. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan with one detail row that cannot be represented and one unrecognized row that can
   ✔  When the crew member converts it
   ✔  Then the failure report carries an "Errors:" section listing one entry
   ✔  And the failure report carries a "Warnings:" section listing one entry
   ✔  And the failure report identifies the source PDF and the primary reason
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `fef3107`.

### 7. A second conversion with warnings uses the next unused warning-report name

Design constraints:
- Existing `roster.parse-warnings.txt` and `roster.parse-warnings-1.txt` stay unchanged; the new report is `roster.parse-warnings-2.txt`; `Kind::Warning` enumeration uses create-new.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: A second conversion with warnings uses the next unused warning-report name
     ✘  Given a supported duty plan with an unrecognized activity code beside existing reports roster.parse-warnings.txt and roster.parse-warnings-1.txt
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: add `Kind::Warning` and its spelling. Production: `apps/cli/src/fs.rs`, `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan with an unrecognized activity code beside existing reports roster.parse-warnings.txt and roster.parse-warnings-1.txt
   ✔  When the crew member converts it
   ✔  Then the warning report is written as roster.parse-warnings-2.txt
   ✔  And roster.parse-warnings.txt and roster.parse-warnings-1.txt are unchanged
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  4 steps (4 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `8da8473`.

### 8. The executable prints each warning beside the new calendar path

Design constraints:
- Transcript names the calendar path, carries one warning line per warned row, and names the warning report; exit zero.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: The executable prints each warning beside the new calendar path
     ✘  Given a supported duty plan with two detail rows carrying unrecognized activity codes
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: print `Warning: <diagnostic_line>` for every warning and `Warning report: <path>`. Production: `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan with two detail rows carrying unrecognized activity codes
   ✔  When the crew member converts it
   ✔  Then the transcript names the newly published calendar
   ✔  And the transcript carries one warning line for each warned row
   ✔  And the transcript names the warning report
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `8a96826`.

### 9. A conversion without warnings publishes no warning report

Design constraints:
- Recognized-only plan publishes the calendar and no `parse-warnings` file; transcript reports no warning; exit zero.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: A conversion without warnings publishes no warning report
     ✘  Given a supported duty plan whose every dated activity carries a recognized activity code
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: write the warning report only when warnings exist. Production: `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan whose every dated activity carries a recognized activity code
   ✔  When the crew member converts it
   ✔  Then a complete calendar is published
   ✔  And no warning report is published
   ✔  And the transcript reports no conversion warning
   ✔  And the invocation reports success through its process exit status
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `7afb005`.

### 10. A warning report that cannot be published keeps the calendar and the warnings visible

Design constraints:
- Calendar publication succeeds; warning-report publication fails (dangling symlink yields `EEXIST`); exit zero; transcript still carries the warning line and says no new warning report was saved.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: A warning report that cannot be published keeps the calendar and the warnings visible
     ✘  Given a supported duty plan with an unrecognized activity code and no writable destination for a new warning report
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: make warning-report failure non-fatal while keeping the warning visible. Production: `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan with an unrecognized activity code and no writable destination for a new warning report
   ✔  When the crew member converts it
   ✔  Then the calendar is published
   ✔  And the invocation reports success through its process exit status
   ✔  And the transcript carries the warning line
   ✔  And the transcript states that no new warning report was saved
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `8f6bd27`.

### 11. The page shows the warnings and offers both files when a plan converts with warnings

Design constraints:
- Reference as drawn: `Calendar ready`; plural copy; status `Your calendar is ready to save. 1 row was converted with a warning.`; warning list; calendar and `2026-06.parse-warnings.txt` offers; ready-state `Choose another duty plan`; no save claim; no cross-origin request.
- Visual baseline, axe, and the `ready-with-warnings` screenshot per viewport.

- [x] RED: `./run-tests.sh --browser --name "<title>"` fails on the undefined `When`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: The page shows the warnings and offers both files when a plan converts with warnings
     ✔  Given the converter page served from the built Site output
     ✘  When the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
        Step doesn't match any function
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  EXIT=1
  ```
- [x] GREEN: extend the WASM result, the page DOM/JS/CSS, and the driver. Production: `ffi/wasm/src/lib.rs`, `web/index.html`, `web/app.js`, `web/styles.css`, `web/e2e/driver.mjs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`, `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`. Fixtures: `apps/cli/tests/steps/support.rs` (generated warned plans). The scenario title reaches the evidence helper through a `before` hook in `apps/cli/tests/features_runner.rs`.
  ```
   ✔  Given the converter page served from the built Site output
   ✔  When the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
   ✔  Then the page shows that the calendar is ready with warnings
   ✔  And the page lists one warning naming the activity code
   ✔  And the page offers the file "2026-06.ics"
   ✔  And the page offers the warning report "2026-06.parse-warnings.txt"
   ✔  And the page has not reported that it saved a file
   ✔  And the page has sent no request to another origin since it loaded
   ✔  And the page meets the visual baseline in the ready-with-warnings state
  [Summary]
  1 feature
  1 scenario (1 passed)
  9 steps (9 passed)
  EXIT=0

  Evidence committed: `evidence/visual/the-page-shows-the-warnings-and-offers-both-files-when-a-plan-converts-with-warnings/{mobile,desktop}--ready-with-warnings.png`.
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  9 steps (9 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `1e99782`.

### 12. The page lists the errors and the warnings when a plan cannot be converted

Design constraints:
- Failure heading; count-based reason; `Errors` and `Warnings` lists; no calendar; `2026-06.parse-error.txt` offer; `Choose another plan`; visual baseline, axe, failure screenshot.

- [x] RED: the failure state is not yet a state the shared visual baseline accepts.
  ```
  Feature: Export a duty-plan calendar
    Scenario: The page lists the errors and the warnings when a plan cannot be converted
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" with a row that cannot be represented and a row that was converted with a warning
     ✔  Then the page shows a failure heading and the count-based reason sentence
     ✔  And the page lists the error naming the unrepresentable row
     ✔  And the page lists the warning naming the converted row
     ✔  And the page offers no calendar
     ✔  And the page offers the diagnostic text "2026-06.parse-error.txt"
     ✔  And the page offers a way to choose another plan
     ✘  And the page meets the visual baseline in the failure state
        Step panicked. Captured output: the failure page shows the internal identifier "parse-error": ...
  [Summary]
  1 feature
  1 scenario (1 failed)
  9 steps (8 passed, 1 failed)
  EXIT=1
  ```
- [x] GREEN: failure lists, count-based reason, diagnostic offer. Production: `web/index.html`, `web/app.js`, `web/styles.css`, `ffi/wasm/src/lib.rs`, `web/e2e/driver.mjs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`, `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`. `apps/cli/tests/steps/e2e/visual_baseline.rs` now ignores the download names the page offers when it looks for internal identifiers: the offer prints the declared file name the browser will save.
  ```
   ✔  Given the converter page served from the built Site output
   ✔  When the crew member supplies the supported duty plan "2026-06.pdf" with a row that cannot be represented and a row that was converted with a warning
   ✔  Then the page shows a failure heading and the count-based reason sentence
   ✔  And the page lists the error naming the unrepresentable row
   ✔  And the page lists the warning naming the converted row
   ✔  And the page offers no calendar
   ✔  And the page offers the diagnostic text "2026-06.parse-error.txt"
   ✔  And the page offers a way to choose another plan
   ✔  And the page meets the visual baseline in the failure state
  [Summary]
  1 feature
  1 scenario (1 passed)
  9 steps (9 passed)
  EXIT=0

  Evidence committed: `evidence/visual/the-page-lists-the-errors-and-the-warnings-when-a-plan-cannot-be-converted/{mobile,desktop}--failure.png`.
  ```
- [x] REFACTOR: no behaviour change; rerun zero.
  ```
  1 scenario (1 passed)
  9 steps (9 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `c261491`.

### 13. The page shows no warning list when the plan converts without warnings

Design constraints:
- Ready state, no warning list, no warning-report offer; calendar offer only.

- [x] RED: the scenario's `When` was held as `panic!("not yet implemented: <step phrase>")` for the RED run, as the design declares for every new step.
  ```
  Feature: Export a duty-plan calendar
    Scenario: The page shows no warning list when the plan converts without warnings
     ✔  Given the converter page served from the built Site output
     ✘  When the crew member supplies the supported duty plan "2026-06.pdf" whose every dated activity is recognized
        Step panicked. Captured output: not yet implemented: the crew member supplies the supported duty plan "<name>" whose every dated activity is recognized
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  EXIT=1
  ```
- [x] GREEN: hide the warning block and skip the warning offer when there are no warnings. Production: `web/app.js` (already carried by the ready-with-warnings work; `renderList` hides an empty list and `showReady` offers the warning report only when warnings exist). Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given the converter page served from the built Site output
   ✔  When the crew member supplies the supported duty plan "2026-06.pdf" whose every dated activity is recognized
   ✔  Then the page shows that the calendar is ready
   ✔  And the page shows no warning list
   ✔  And the page offers the file "2026-06.ics"
   ✔  And the page offers no warning report
   ✔  And the page meets the visual baseline in the ready state
  [Summary]
  1 feature
  1 scenario (1 passed)
  7 steps (7 passed)
  EXIT=0

  Evidence committed: `evidence/visual/the-page-shows-no-warning-list-when-the-plan-converts-without-warnings/{mobile,desktop}--ready.png`.
  ```
- [x] REFACTOR: no behaviour change; rerun zero.
  ```
  1 scenario (1 passed)
  7 steps (7 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `786fa0e`.

### 14. A timed activity code with a route and an incomplete printed interval rejects the conversion

Design constraints:
- Known timed code `TK` + route + only Start reaches the final catch-all error `a route row carries an incomplete printed interval`; no calendar; report lists it.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: A timed activity code with a route and an incomplete printed interval rejects the conversion
     ✘  Given a supported duty plan dated 2099-06-07 with activity code TK, a printed route from ABC to DEF, a printed UTC start 08:00 and no end
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: add the exhaustive catch-all error arm. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan dated 2099-06-07 with activity code TK, a printed route from ABC to DEF, a printed UTC start 08:00 and no end
   ✔  When the crew member converts it
   ✔  Then no calendar is published
   ✔  And the conversion failure names "a route row carries an incomplete printed interval"
   ✔  And the failure report lists that row under "Errors:"
   ✔  And the invocation reports failure through a non-zero process exit status
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0

  The recognized timed arms now route an interval they cannot build on a route row to the catch-all
  sentence instead of returning the bare timing error.
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `1f95728`.

### 15. An unrecognized activity code with a route and a single printed endpoint rejects the conversion

Design constraints:
- Unrecognized code + route + one endpoint reaches the same route-incomplete error; no calendar.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: An unrecognized activity code with a route and a single printed endpoint rejects the conversion
     ✘  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF, a printed route from ABC to DEF, a printed UTC start 08:00 and no end
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: the catch-all covers this shape; add the fixture. Production: none beyond task 14. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF, a printed route from ABC to DEF, a printed UTC start 08:00 and no end
   ✔  When the crew member converts it
   ✔  Then no calendar is published
   ✔  And the conversion failure names "a route row carries an incomplete printed interval"
   ✔  And the failure report lists that row under "Errors:"
   ✔  And the invocation reports failure through a non-zero process exit status
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `6048f3b`.

### 16. A known timed activity code that also prints a route stays a timed non-flight activity

Design constraints:
- Known codes win over the route branch: `TK` + ABC→DEF + 08:00–12:00 is a timed non-flight duty, not travel; no warning.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: A known timed activity code that also prints a route stays a timed non-flight activity
     ✘  Given a supported duty plan dated 2099-06-07 with activity code TK, a printed route from ABC to DEF and printed UTC times 08:00 and 12:00
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: keep the known-code branch before the route branch; add the fixture and assertion. Production: none beyond the classifier order. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan dated 2099-06-07 with activity code TK, a printed route from ABC to DEF and printed UTC times 08:00 and 12:00
   ✔  When the crew member converts it
   ✔  Then the calendar contains one timed non-flight activity from 08:00 to 12:00 UTC on 2099-06-07 lasting 4 hours
   ✔  And the calendar contains no travel activity on 2099-06-07
   ✔  And the converter reports no conversion warning
   ✔  And no warning report is published
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `1b4a1a2`.

### 17. A travel leg with a flight number produces no warning

Design constraints:
- Route-bearing flight leg converts silently; no `parse-warnings.txt`; transcript carries no warning.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: A travel leg with a flight number produces no warning
     ✘  Given a supported duty plan with a travel leg printing flight number LU742 from ABC to DEF with printed UTC times 09:00 and 10:00 and no crew position
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: route-before-fallback order; assert no warning. Production: none beyond the classifier. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan with a travel leg printing flight number LU742 from ABC to DEF with printed UTC times 09:00 and 10:00 and no crew position
   ✔  When the crew member converts it
   ✔  Then the calendar contains one travel activity from 09:00 to 10:00 UTC on the duty-plan date
   ✔  And the converter reports no conversion warning
   ✔  And no warning report is published
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `0091e91`.

### 18. A silent layout row produces no warning

Design constraints:
- A row with no activity code and no route is skipped before date reading; no event, no warning, no warning report.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: A silent layout row produces no warning
     ✔  Given a supported duty plan with a detail row that carries only a date and no activity code
     ✔  When the crew member converts it
     ✘  Then the calendar contains no absence or layover for that date
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: keep the silent-row skip; add the assertion. Production: none beyond the skip. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan with a detail row that carries only a date and no activity code
   ✔  When the crew member converts it
   ✔  Then the calendar contains no absence or layover for that date
   ✔  And the converter reports no conversion warning
   ✔  And no warning report is published
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `b1569cd`.

### 19. A silent layout row before a warned row keeps its place in the row numbering

Design constraints:
- The warning for a row after a silent layout row names raw row 2, not filtered row 1.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: A silent layout row before a warned row keeps its place in the row numbering
     ✘  Given a supported duty plan whose first detail row is a silent layout row carrying only a date and whose second detail row carries the unrecognized activity code ZZ_M_CF with printed UTC times 08:00 and 15:30
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: number diagnostics by raw table position. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan whose first detail row is a silent layout row carrying only a date and whose second detail row carries the unrecognized activity code ZZ_M_CF with printed UTC times 08:00 and 15:30
   ✔  When the crew member converts it
   ✔  Then the converter reports one conversion warning naming detail row 2 and the date 2099-06-07
   ✔  And the warning names the activity code "ZZ_M_CF"
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  4 steps (4 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `69ad15a`.

### 20. An unmatched departure and a later warning are reported in plan order

Design constraints:
- An orphan departure error discovered at the end is sorted before a later-row warning; the departure error is the `Reason:` line.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: An unmatched departure and a later warning are reported in plan order
     ✘  Given a supported duty plan with an unmatched departure fragment on an earlier detail row and an unrecognized activity code with printed UTC times on a later detail row
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: sort diagnostics by row; choose the first error for `Reason:`. Production: `crates/duty2cal-core/src/application/mod.rs`, `apps/cli/src/main.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan with an unmatched departure fragment on an earlier detail row and an unrecognized activity code with printed UTC times on a later detail row
   ✔  When the crew member converts it
   ✔  Then no calendar is published
   ✔  And the failure report lists the departure row's error before the later row's warning
   ✔  And the failure report's reason line is the departure row's error
   ✔  And the invocation reports failure through a non-zero process exit status
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0

  The reason line carries the first error's message, not the entry's row prefix; the step compares it
  with the message part of the first `Errors:` entry.
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `ef83378`.

### 21. An unrecognized activity code with an unqualified window across midnight keeps it as the following UTC day

Design constraints:
- The fallback reuses `row_interval`: 23:10–01:05 unqualified becomes 23:10 → next-day 01:05 UTC with a warning.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: An unrecognized activity code with an unqualified window across midnight keeps it as the following UTC day
     ✘  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF, one local-date block and unqualified printed UTC clocks 23:10 and 01:05
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: use `row_interval` for the fallback. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF, one local-date block and unqualified printed UTC clocks 23:10 and 01:05
   ✔  When the crew member converts it
   ✔  Then the calendar contains one timed activity titled "ZZ_M_CF" from 23:10 on 2099-06-07 to 01:05 on 2099-06-08 UTC
   ✔  And the converter reports one conversion warning naming detail row 1 and the date 2099-06-07
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  4 steps (4 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `39d99a9`.

### 22. An unrecognized activity code with an equal printed window rejects the conversion

Design constraints:
- Equal clocks are never repaired; the fallback row is a non-positive-interval error; no calendar.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: An unrecognized activity code with an equal printed window rejects the conversion
     ✘  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and equal printed UTC clocks 10:00 and 10:00
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: `row_interval`'s positive check applies to the fallback. Production: none beyond task 21. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and equal printed UTC clocks 10:00 and 10:00
   ✔  When the crew member converts it
   ✔  Then no calendar is published
   ✔  And the conversion failure names "a timed interval must end after it starts"
   ✔  And the invocation reports failure through a non-zero process exit status
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `6deba99`.

### 23. A mis-paired travel leg reports both the arrival error and the orphan departure

Design constraints:
- A mismatched arrival and an orphan departure produce two errors, in row order.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: A mis-paired travel leg reports both the arrival error and the orphan departure
     ✘  And the failure report lists the departure row's error before the arrival row's error
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: report the arrival mismatch and every remaining pending departure. Production: `crates/duty2cal-core/src/application/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan whose departure fragment prints crew position DH and flight number 124 and whose arrival fragment prints flight number 124 without a crew position
   ✔  When the crew member converts it
   ✔  Then no calendar is published
   ✔  And the converter reports two conversion errors
   ✔  And the failure report lists the departure row's error before the arrival row's error
   ✔  And the invocation reports failure through a non-zero process exit status
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `af910a2`.

### 24. An unrecognized activity code's activity carries no description

Design constraints:
- The fallback publishes the code, station and interval only; no `DESCRIPTION` line, because the source prints no approved descriptive field on that row.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: An unrecognized activity code's activity carries no description
     ✘  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF at station ABC, printed UTC times 08:00 and 15:30, and a printed Trip ID T42
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: `description_fields()` is empty for `UnrecognizedActivity`; assert the absence. Production: `crates/duty2cal-core/src/domain/mod.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF at station ABC, printed UTC times 08:00 and 15:30, and a printed Trip ID T42
   ✔  When the crew member converts it
   ✔  Then the calendar contains one timed activity titled "ZZ_M_CF" from 08:00 to 15:30 UTC on 2099-06-07 at ABC
   ✔  And that activity carries no description
   ✔  And the converter reports one conversion warning naming detail row 1 and the date 2099-06-07
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `d0c0d6d`.

### 25. The warning report identifies the source and carries a warnings section

Design constraints:
- The warning report begins `Source PDF: <name>`, carries `Warnings:`, one entry per warned row, LF, trailing newline.

- [x] RED: single-scenario run fails on the undefined `Given`.
  ```
  Feature: Export a duty-plan calendar
    Scenario: The warning report identifies the source and carries a warnings section
     ✔  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and printed UTC times 08:00 and 15:30
     ✔  When the crew member converts it
     ✘  Then the warning report's first line states the source PDF "roster.pdf"
        Step doesn't match any function
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: assert the exact report header and section. Production: none beyond `warning_report_text`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and printed UTC times 08:00 and 15:30
   ✔  When the crew member converts it
   ✔  Then the warning report's first line states the source PDF "roster.pdf"
   ✔  And the warning report carries a "Warnings:" section listing one entry
   ✔  And that entry names detail row 1 and the date 2099-06-07
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] REFACTOR: no behaviour change.
  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `d9ba755`.

### 26. The page states how many rows were converted with warnings

Design constraints:
- Ready-with-warnings status uses the exact singular/plural sentence and count from the result; the ready-state way-back button is visible.

- [x] RED: the scenario's `When` was held as `panic!("not yet implemented: <step phrase>")` for the RED run.
  ```
  Feature: Export a duty-plan calendar
    Scenario: The page states how many rows were converted with warnings
     ✔  Given the converter page served from the built Site output
     ✘  When the crew member supplies the supported duty plan "2026-06.pdf" containing two rows with unrecognized activity codes and printed windows
        Step panicked. Captured output: not yet implemented: the crew member supplies the supported duty plan "<name>" containing two rows with unrecognized activity codes and printed windows
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  EXIT=1
  ```
- [x] GREEN: render the count sentence and the ready-state reset. Production: `web/app.js`, `web/index.html` (`showReady` writes the singular/plural sentence from the warning count; `#reset-ready` is the ready-state way back). Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given the converter page served from the built Site output
   ✔  When the crew member supplies the supported duty plan "2026-06.pdf" containing two rows with unrecognized activity codes and printed windows
   ✔  Then the page shows that the calendar is ready with warnings
   ✔  And the page states "Your calendar is ready to save. 2 rows were converted with warnings."
   ✔  And the page meets the visual baseline in the ready-with-warnings state
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0

  Evidence committed: `evidence/visual/the-page-states-how-many-rows-were-converted-with-warnings/{mobile,desktop}--ready-with-warnings.png`.
  ```
- [x] REFACTOR: no behaviour change; rerun zero.
  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `f217209`.

### 27. The page offers a warning report that names the source and carries the warnings

Design constraints:
- The offered `2026-06.parse-warnings.txt` content is the exact warning report text: source name and `Warnings:` section.

- [x] RED: the report-reading step was held as `panic!("not yet implemented: <step phrase>")` for the RED run.
  ```
  Feature: Export a duty-plan calendar
    Scenario: The page offers a warning report that names the source and carries the warnings
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
     ✔  Then the page offers the warning report "2026-06.parse-warnings.txt"
     ✘  And the offered warning report begins with "Source PDF: 2026-06.pdf" and carries a "Warnings:" section
        Step panicked. Captured output: not yet implemented: the offered warning report begins with "<first line>" and carries a "<section>" section
  [Summary]
  1 feature
  1 scenario (1 failed)
  4 steps (3 passed, 1 failed)
  EXIT=1
  ```
- [x] GREEN: the page offers `warningReport` unchanged from the converter. Production: `web/app.js`, `ffi/wasm/src/lib.rs`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs` (the step reads the captured download back and compares its header, section and entry).
  ```
   ✔  Given the converter page served from the built Site output
   ✔  When the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
   ✔  Then the page offers the warning report "2026-06.parse-warnings.txt"
   ✔  And the offered warning report begins with "Source PDF: 2026-06.pdf" and carries a "Warnings:" section
   ✔  And the offered warning report's entry names the detail row and its date
   ✔  And the page meets the visual baseline in the ready-with-warnings state
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0

  Evidence committed: `evidence/visual/the-page-offers-a-warning-report-that-names-the-source-and-carries-the-warnings/{mobile,desktop}--ready-with-warnings.png`.
  ```
- [x] REFACTOR: no behaviour change; rerun zero.
  ```
  1 scenario (1 passed)
  6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `66da742`.

### 28. A second attempt clears the previous attempt's warning list

Design constraints:
- A second attempt clears the previous warning list and offers before reading the new plan; a warning-free second plan shows no warning list.

- [x] RED: with the warning clearing removed from `clearAttempt`/`showReady` (the pre-change page), the second attempt inherited the first attempt's list.
  ```
  Feature: Export a duty-plan calendar
    Scenario: A second attempt clears the previous attempt's warning list
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
     ✔  And the crew member supplies the supported duty plan "2026-07.pdf" whose every dated activity is recognized
     ✘  Then the page shows that the calendar is ready
        Step panicked. Captured output: the page did not reach the ready state: { ... "lists":{"errors":[],"warnings":["row 1 (2099-06-07): unrecognized activity code \"ZZ_M_CF\"; ..."]}, "offers":[{"kind":"calendar","name":"2026-07.ics", ...}] ... }
        left: Some("ready-with-warnings")
        right: Some("ready")
  [Summary]
  1 feature
  1 scenario (1 failed)
  4 steps (3 passed, 1 failed)
  EXIT=1
  ```
- [x] GREEN: `clearAttempt` empties and hides both list containers and withdraws prior offers. Production: `web/app.js`. Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs` (the same test steps apply the file picker action twice in one scenario).
  ```
   ✔  Given the converter page served from the built Site output
   ✔  When the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
   ✔  And the crew member supplies the supported duty plan "2026-07.pdf" whose every dated activity is recognized
   ✔  Then the page shows that the calendar is ready
   ✔  And the page shows no warning list
   ✔  And the page offers no warning report
   ✔  And the page offers the file "2026-07.ics"
   ✔  And the page meets the visual baseline in the ready state
  [Summary]
  1 feature
  1 scenario (1 passed)
  8 steps (8 passed)
  EXIT=0

  Evidence committed: `evidence/visual/a-second-attempt-clears-the-previous-attempt-s-warning-list/{mobile,desktop}--ready.png`.
  ```
- [x] REFACTOR: no behaviour change; rerun zero.
  ```
  1 scenario (1 passed)
  8 steps (8 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `9b7f0fa`.

### 29. The page keeps the calendar and the warnings when it cannot offer the warning report

Design constraints:
- A failed warning-report offer keeps the calendar offer and the visible warnings, and shows `No warning report was offered.`; no failure state.

- [x] RED: the driver's fault injection was absent, so the `Given` could not start the page.
  ```
  Feature: Export a duty-plan calendar
    Scenario: The page keeps the calendar and the warnings when it cannot offer the warning report
     ✘  Given the converter page served from the built Site output that cannot create a download offer for the warning report
        Step panicked. Captured output: the browser driver failed "start": unknown start mode: warning-offer-failure
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  EXIT=1
  ```
- [x] GREEN: catch only the warning-report offer failure. Production: `web/app.js` (the warning-report offer has its own `try`; its failure sets `No warning report was offered.` and leaves the calendar and the list), `web/e2e/driver.mjs` (`warning-offer-failure` refuses the conversion's first plain-text object URL, which is the warning report on the success path). Test: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.
  ```
   ✔  Given the converter page served from the built Site output that cannot create a download offer for the warning report
   ✔  When the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
   ✔  Then the page shows that the calendar is ready with warnings
   ✔  And the page lists one warning naming the activity code
   ✔  And the page still offers the calendar
   ✔  And the page states that no warning report was offered
   ✔  And the page meets the visual baseline in the ready-with-warnings state
  [Summary]
  1 feature
  1 scenario (1 passed)
  7 steps (7 passed)
  EXIT=0

  Evidence committed: `evidence/visual/the-page-keeps-the-calendar-and-the-warnings-when-it-cannot-offer-the-warning-report/{mobile,desktop}--ready-with-warnings.png`.
  ```
- [x] REFACTOR: no behaviour change; rerun zero.
  ```
  1 scenario (1 passed)
  7 steps (7 passed)
  EXIT=0
  ```
- [x] COMMIT: body names this scenario. Revision `f880e20`.

## Closeout

- [x] Every step phrase bound by the delta is bound in the permanent test tree: all new phrases live in `apps/cli/tests/steps/export_duty_plan_calendar.rs` and `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`; no delta step copy exists.
- [x] Full suite `./run-tests.sh` exits 0 (114 scenarios, 461 steps, plus the unit, golden and architecture tests).
- [x] E2E suite `./run-tests.sh --e2e` exits 0 (1 scenario, strictly fewer than the full run).
- [x] Browser suite `./run-tests.sh --browser` exits 0 (25 scenarios, 178 steps) and commits the declared evidence.
- [x] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `givn lint --change report-row-diagnostics` pass.
