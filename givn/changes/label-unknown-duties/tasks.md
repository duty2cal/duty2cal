# Tasks: label-unknown-duties

Release note (commit subject): Every calendar entry for an unrecognized duty code is prefixed with `unknown duty: <code>`, and the browser's ready page lists those duties and offers the calendar and the publisher warning with clear save labels.

Owning use case: [Export a duty-plan calendar](specs/export-duty-plan-calendar/usecase.md) (`export-duty-plan-calendar`). Confirmed Personas: `crew-member--personal-calendar-owner` (accuracy lens for the imported calendar and the page) and `contributor--web-reviewer` (the page's copy, labels and structure). `project-maintainer--release-owner` is unaffected: no release artifact, dependency or public claim changes.

## Environment

This machine's Nix Rust has no `wasm32-unknown-unknown` target and its Playwright Chromium is dynamically linked. The runs below therefore use a test-only dispatch wrapper and overrides; they are not production changes.

- Non-browser: `RUSTC=/tmp/duty2cal-tc/rustc ./run-tests.sh --name "<scenario>"`
- Browser: build the bundle and Site output once with the rustup toolchain and the Nix `wasm-ld`, then
  `CHROMIUM_PATH=/nix/store/r0f9i35n3r7rng8162xih4c1dag0s1vw-chromium-156.0.8078.47/bin/chromium RUSTC=/tmp/duty2cal-tc/rustc DUTY2CAL_SKIP_WEB_BUILD=1 DUTY2CAL_SITE_ROOT=$PWD/dist/site ./run-tests.sh --browser --name "<scenario>"`
- `/tmp/duty2cal-tc/rustc` dispatches wasm compilations to the rustup toolchain (which has the wasm std and the Nix `wasm-ld` override) and native compilations to the Nix rustc.

Single-scenario command from the design: `./run-tests.sh --name "<scenario title>"`. Full suite: `./run-tests.sh`. E2E-only: `./run-tests.sh --e2e`. Browser: `./run-tests.sh --browser`.

## Use-case constraints carried into every scenario task

- An unrecognized code is represented from its printed evidence, never from its letters: both printed endpoints and no route make a timed activity; neither makes an all-day activity; exactly one endpoint is a conversion error.
- Every such activity is titled `unknown duty: <printed code>`; the station, printed UTC window and empty-description rule stay as they are.
- A conversion warning is reported for each fallback, names its detail row and date, and never withdraws the calendar; the warning report and the executable's lines keep their current text and filename.
- The recognized standby shifts `SB_F_CF`, `SB_M_CF` and `SB_S_CF` keep their bare titles and produce no warning; every other code keeps the fallback.
- The page's ready state lists the unknown duties in plan order, one line per represented event, and offers the calendar and the warning report; it writes nothing and claims no save it did not observe.
- The failure state, its count sentence, its `Errors`/`Warnings` lists and its `Choose another plan` way back are unchanged.
- Private plans are never committed; every fixture is generated.

## Shared design constraints (from design.md)

- Domain title: `DutyEntry::UnrecognizedActivity`'s `summary()` arm, `crates/duty2cal-core/src/domain/mod.rs`.
- Structured fact: `UnknownDuty { date, start, code }` and `ConversionOutcome.unknown_duties`, projected from the represented entries in `crates/duty2cal-core/src/application/mod.rs`. A timed fallback's date is the interval's UTC start date; an all-day fallback's date is the entry's duty-plan date.
- Facade: `ffi/wasm/src/lib.rs` adds `unknownDuties` as `{date: "DD-MM-YYYY", start: "HH:MM"|null, code}`; every helper is `#[cfg(target_arch = "wasm32")]`.
- Page: `web/index.html` order is heading, status, `Unknown duties` list, save-choice copy, offers, note, way back; `web/app.js` composes `DD-MM-YYYY [HH:MM UTC ]unknown duty: <code>`, labels the warning offer, and keeps the warning-report/`#warnings` gate on `warnings.length`.
- Driver and CSS are unchanged. Step definitions live in `apps/cli/tests/steps/export_duty_plan_calendar.rs`; the fixture `all_day_warned_plan()` lives in `crates/duty2cal-pdf-fixtures/src/lib.rs` and is re-exported by `apps/cli/tests/steps/support.rs`.
- The golden oracles `apps/cli/tests/fixtures/expected/2098-06.ics` and `2026-06.ics` stay byte-identical; neither plan prints an unrecognized code.

Interaction coverage (checked before writing tasks): the permanent inventory holds two entries — supply a duty plan to the executable, and supply a duty plan through the page and save the offered calendar — matched by the permanent `@e2e` scenarios named in design.md's Interaction Coverage Matrix. This change adds no consumer action; the page list is regular `@browser` coverage of the existing page action.

## Setup task — environment, runner, strict mode

- [x] Confirm the toolchain, runner, strict mode and the pre-implementation baseline.
  - `apps/cli/tests/features_runner.rs` calls `DutyWorld::cucumber().fail_on_skipped().run_and_exit(root)`.
  - `verify.command: "./run-tests.sh"`; `verify.e2e_command: "./run-tests.sh --e2e"` (strict subset).
  - Strictness proofs:
    - Targeted behavioral RED before any production edit:
      ```
      RUSTC=/tmp/duty2cal-tc/rustc ./run-tests.sh --name "An unrecognized activity code with a printed window becomes a timed activity with a warning"
      1 scenario (1 failed) / 3 steps (2 passed, 1 failed)
      Step panicked: the timed activity does not carry "SUMMARY:unknown duty: ZZ_M_CF": … SUMMARY:ZZ_M_CF …
      EXIT=1
      ```
    - Full baseline: `4 features / 140 scenarios (135 passed, 5 failed) / 578 steps (573 passed, 5 failed)`; the five failures are exactly the delta's CLI scenarios still `@wip`; no permanent scenario regressed.
  - Browser baseline (before the page edit): `1 scenario (1 passed) / 12 steps (12 passed)` for "Crew member converts a supported duty plan in the browser and saves the offered calendar".

## E2E setup task

- [x] Confirm the e2e filter and counts before the browser scenarios.
  - `verify.e2e_command` is `./run-tests.sh --e2e`, which selects `@e2e and not @browser`; the permanent inventory's CLI `@e2e` scenario is "Crew member converts a supported duty plan and receives a calendar file". The browser `@e2e` scenario is exercised by the browser smoke, not this filter.
  - This change adds no `@e2e` scenario and removes none; the two inventory actions keep one `@e2e` each.
  - Browser smoke command and environment are recorded in `## Environment`; it drives the real page through Playwright.

## Scenario tasks

### 1. An unrecognized activity code with a printed window becomes a timed activity with a warning

Design constraints:
- The timed fallback is titled `unknown duty: ZZ_M_CF`, keeps its station ABC and 08:00-15:30 UTC window, warns once, publishes the calendar and the warning report, and exits zero.
- First production increment: the timed fallback only; the all-day arm is task 2.
- Files: `crates/duty2cal-core/src/domain/mod.rs`; test expectation in the delta feature.

- [x] RED: remove `@wip` from this scenario only; run the targeted command; paste output (must fail on the bare title).
  ```
  Scenario: An unrecognized activity code with a printed window becomes a timed activity with a warning
   ✔  Given … / ✔  When …
   ✘  Then the calendar contains one timed activity titled "unknown duty: ZZ_M_CF" …
      Matched: apps/cli/tests/steps/export_duty_plan_calendar.rs:2970:1
      Step panicked: the timed activity does not carry "SUMMARY:unknown duty: ZZ_M_CF": … SUMMARY:ZZ_M_CF …
  1 scenario (1 failed) / 3 steps (2 passed, 1 failed)
  EXIT=1
  ```
- [x] GREEN: prefixed the timed fallback's title in `DutyEntry::summary()`; run the targeted command; paste output (must pass).
  ```
  Scenario: An unrecognized activity code with a printed window becomes a timed activity with a warning
   ✔ × 8 steps
  1 scenario (1 passed) / 8 steps (8 passed)
  EXIT=0
  ```
- [x] REFACTOR: `cargo fmt --all`; no behaviour change; run again; paste output.
  ```
  1 scenario (1 passed) / 8 steps (8 passed)
  EXIT=0
  ```
- [x] COMMIT: one revision, release note as subject, scenario title in the body; record the hash.
  ```
  COMMIT: 5d94ee0
  ```

### 2. An unrecognized activity code without printed times becomes an all-day activity with a warning

Design constraints:
- The all-day fallback is titled `unknown duty: ZZ_M_CF`, carries no interval, warns once and publishes the warning report.
- Second production increment: make the prefix cover the all-day arm too.
- Files: `crates/duty2cal-core/src/domain/mod.rs`.

- [x] RED: removed `@wip`; run the targeted command; paste output (failed on the bare all-day title).
  ```
  Scenario: An unrecognized activity code without printed times becomes an all-day activity with a warning
   ✔  Given … / ✔  When …
   ✘  Then the calendar contains one all-day activity for 2099-06-07 titled "unknown duty: ZZ_M_CF"
      Step panicked: … SUMMARY:ZZ_M_CF
  1 scenario (1 failed) / 3 steps (2 passed, 1 failed)
  EXIT=1
  ```
- [x] GREEN: unified the `UnrecognizedActivity` arm; run the targeted command; paste output (passed).
  ```
  1 scenario (1 passed) / 6 steps (6 passed)
  EXIT=0
  ```
- [x] REFACTOR: `cargo fmt --all`; run again; no behaviour change.
  ```
  1 scenario (1 passed) / 6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: one revision; record the hash.
  ```
  COMMIT: ed6cb4b
  ```

### 3. An unrecognized activity code with an unqualified window across midnight keeps it as the following UTC day

Design constraints:
- The overnight timed fallback is titled `unknown duty: ZZ_M_CF`; its interval stays 23:10 on 2099-06-07 to 01:05 on 2099-06-08; one warning.
- Guard: the title rule from task 1 already covers it; no further production change.
- Files: delta feature expectation.

- [x] RED: removed `@wip`; the guard's title rule from task 1 is already in place, so the targeted run is green. Recorded honestly as guard evidence; the RED failure this scenario guards against (a bare override title) is covered by task 1.
  ```
  1 scenario (1 passed) / 4 steps (4 passed)
  EXIT=0
  ```
- [x] GREEN: no production change; the overnight interval and title pass.
  ```
  ✔  Then the calendar contains one timed activity titled "unknown duty: ZZ_M_CF" from 23:10 on 2099-06-07 to 01:05 on 2099-06-08 UTC
  1 scenario (1 passed) / 4 steps (4 passed)
  ```
- [x] REFACTOR: run again; no behaviour change.
  ```
  1 scenario (1 passed) / 4 steps (4 passed)
  EXIT=0
  ```
- [x] COMMIT: one revision; record the hash.
  ```
  COMMIT: cf8f0ab
  ```

### 4. An unrecognized activity code's activity carries no description

Design constraints:
- The timed fallback is titled `unknown duty: ZZ_M_CF`, carries its station and window, and carries no description even though the row prints Trip ID T42.
- Guard: `description_fields()` and `summary()` are separate; no production change beyond the title rule.
- Files: delta feature expectation.

- [x] RED: removed `@wip`; the title rule from task 1 is already in place, so the guard's targeted run is green. Recorded honestly as guard evidence.
  ```
  1 scenario (1 passed) / 5 steps (5 passed)
  EXIT=0
  ```
- [x] GREEN: no production change; the title, station, window and empty description pass.
  ```
  1 scenario (1 passed) / 5 steps (5 passed)
  ```
- [x] REFACTOR: run again; no behaviour change.
  ```
  1 scenario (1 passed) / 5 steps (5 passed)
  EXIT=0
  ```
- [x] COMMIT: one revision; record the hash.
  ```
  COMMIT: PENDING4
  ```

### 5. A standby-like code outside the three shift codes still warns

Design constraints:
- `SB_X_CF` keeps the fallback title `unknown duty: SB_X_CF`, the 08:00-15:30 window at FRA, one warning naming the code and the printed window; the three recognized shift codes stay bare.
- Guard: the recognition set is unchanged; the title rule covers this code.
- Files: delta feature expectation.

- [x] RED: removed `@wip`; the title rule from task 1 is already in place, so the guard's targeted run is green. Recorded honestly as guard evidence.
  ```
  1 scenario (1 passed) / 6 steps (6 passed)
  EXIT=0
  ```
- [x] GREEN: no production change; the fallback title, window and warning sentence pass.
  ```
  1 scenario (1 passed) / 6 steps (6 passed)
  ```
- [x] REFACTOR: run again; no behaviour change.
  ```
  1 scenario (1 passed) / 6 steps (6 passed)
  EXIT=0
  ```
- [x] COMMIT: one revision; record the hash.
  ```
  COMMIT: PENDING5
  ```

### 6. The page shows the warnings and offers both files when a plan converts with warnings

Design constraints:
- Ready-with-warnings state; heading `Calendar ICS ready`; status `Your calendar is ready to save. 1 row was converted with an unknown duty.`; list heading `Unknown duties`; item `07-06-2099 08:00 UTC unknown duty: ZZ_M_CF`; calendar offer `Save 2026-06.ics`; warning offer `2026-06.parse-warnings.txt` labelled `Save unknown duty warning for publisher`; copy `You can choose where to save the file.`; way back `Open new/next duty plan PDF`; no save claim and no cross-origin request.
- Production: `UnknownDuty` + `ConversionOutcome.unknown_duties` + wasm `unknownDuties` + `web/index.html` + `web/app.js`; the shared step bindings.
- Files: `crates/duty2cal-core/src/application/mod.rs`, `ffi/wasm/src/lib.rs`, `web/index.html`, `web/app.js`, `apps/cli/tests/steps/export_duty_plan_calendar.rs`.

- [ ] RED: remove `@wip`; run the browser command; paste output (must fail: the page still shows the old heading/copy and no structured list). The driver must be reachable.
- [ ] GREEN: implement the structured fact, facade array, page order/copy/labels and list composition; run the browser command; paste output (must pass).
- [ ] REFACTOR: no behaviour change; run again; paste output.
- [ ] COMMIT: one revision; record the hash.

### 7. The page lists an all-day unknown duty with its date and code

Design constraints:
- Ready-with-warnings; item `07-06-2099 unknown duty: ZZ_M_CF` (no `HH:MM UTC`); `all_day_warned_plan()` fixture and its Given binding.
- Production: the `start: null` path of the same facade/list.
- Files: `crates/duty2cal-pdf-fixtures/src/lib.rs`, `apps/cli/tests/steps/support.rs`, `apps/cli/tests/steps/export_duty_plan_calendar.rs`.

- [ ] RED: remove `@wip`; run the browser command; paste output (must fail: the Given binding is absent).
- [ ] GREEN: add the fixture and binding; run the browser command; paste output (must pass).
- [ ] REFACTOR: run again; paste output.
- [ ] COMMIT: one revision; record the hash.

### 8. The page shows no warning list when the plan converts without warnings

Design constraints:
- Plain ready state; heading `Calendar ICS ready`; copy `You can choose where to save the file.`; way back `Open new/next duty plan PDF`; no list; calendar offer; no warning offer.
- Production: the empty `unknownDuties` path; the ready heading/copy/label bindings.
- Files: `web/app.js`, `apps/cli/tests/steps/export_duty_plan_calendar.rs`.

- [ ] RED: remove `@wip`; run the browser command; paste output (must fail on the heading/copy/way-back).
- [ ] GREEN: ensure the empty path hides the list and the copy is static; run the browser command; paste output (must pass).
- [ ] REFACTOR: run again; paste output.
- [ ] COMMIT: one revision; record the hash.

### 9. The page states how many rows were converted with warnings

Design constraints:
- Two unknown duties; status `Your calendar is ready to save. 2 rows were converted with unknown duties.`; list order `07-06-2099 08:00 UTC unknown duty: ZZ_M_CF` then `08-06-2099 09:00 UTC unknown duty: ZZ_TST`.
- Production: the plural count sentence and plan-order list.
- Files: `web/app.js`, `apps/cli/tests/steps/export_duty_plan_calendar.rs`.

- [ ] RED: remove `@wip`; run the browser command; paste output (must fail on the old warning wording or order).
- [ ] GREEN: use `unknownDuties.length` for the count and keep entry order; run the browser command; paste output (must pass).
- [ ] REFACTOR: run again; paste output.
- [ ] COMMIT: one revision; record the hash.

### 10. The page keeps the calendar and the warnings when it cannot offer the warning report

Design constraints:
- Ready-with-warnings; the list remains; the calendar offer remains; `No warning report was offered.`; the page does not become a failure; the warning-report offer gate is `warnings.length`.
- Production: no new code beyond the offer-failure path and the list; the state and copy bindings.
- Files: `web/app.js`, `apps/cli/tests/steps/export_duty_plan_calendar.rs`.

- [ ] RED: remove `@wip`; run the browser command; paste output (must fail on the old list/copy).
- [ ] GREEN: keep the calendar and list when the offer fails; run the browser command; paste output (must pass).
- [ ] REFACTOR: run again; paste output.
- [ ] COMMIT: one revision; record the hash.

## Closeout

- [ ] Every delta scenario is GREEN by a targeted run; `./run-tests.sh`, `./run-tests.sh --e2e` and the browser smoke exit 0.
- [ ] Permanent step bindings exist for every delta phrase; no delta step copy remains.
- [ ] Visual evidence committed under `givn/changes/label-unknown-duties/evidence/visual/` with the capture override.
