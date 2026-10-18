# Tasks: include-report-time-in-travel-title

Release note (commit subject): A travel activity's calendar title now ends with its report time in local time — `LU 742 Tychon-Mareum Report 07:30 LT` — so the briefing time is visible without opening the event, while the event details keep the labelled value.

Owning use case: [Export a duty-plan calendar](specs/export-duty-plan-calendar/usecase.md) (`export-duty-plan-calendar`). Confirmed Personas: `crew-member--personal-calendar-owner` (accuracy lens — the title and details a crew member reads), `project-maintainer--release-owner` (release-evidence lens — no release artifact changes). Actors in scenarios stay distinct from Personas.

Single-scenario run command (used by every RED/GREEN/REFACTOR below): `./run-tests.sh --name "<scenario title>"`.
Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`.

Use-case constraints carried into every scenario task (from the owning use case's Rules):

- A travel title preserves the printed crew position and flight number as two words and appends the route; this change adds the report suffix after the route and changes nothing else in that title.
- The report time is descriptive: it never supplies an activity date or instant. The event's `DTSTART`/`DTEND` stay UTC.
- An unsupplied optional field is omitted; no report time is manufactured.
- Differing optional fragment values publish one event, retain both labelled values in the description, and never merge conflicting required values.
- The title is not component identity; UIDs and snapshot scoping are untouched.
- Every conversion failure still names the source and the reason with a numbered diagnostic; this change adds no failure path.
- Private plans are never committed: all fixtures are generated.

Interaction coverage (checked before writing tasks): the permanent inventory holds one entry — "supply a duty plan for conversion" — matched by the `@e2e` scenario "Crew member converts a supported duty plan and receives a calendar file". `design.md`'s Interaction Coverage Matrix holds that row with the CLI subprocess as its driving mechanism. This change adds no consumer action, so no new `@e2e` scenario and no new matrix row; the delta's four scenarios are regular scenarios of that one action.

## Setup task — environment, runner, strict mode

- [x] Confirm the toolchain, runner, strict mode, and the pre-implementation baseline.
  - `cargo 1.98.0`, `rustc 1.98.1`, `git-cliff 2.14.2` on `PATH`.
  - Strict mode is `.fail_on_skipped()` in `apps/cli/tests/features_runner.rs`, so an undefined step fails the run.
  - `./run-tests.sh` before any scenario work: `5 features / 144 scenarios (140 passed, 4 failed) / 590 steps (586 passed, 4 failed)`. The four failures are exactly the delta scenarios: scenario 1 fails on the missing title suffix; scenarios 2–4 fail as undefined `Given` steps. Exit 101.

  ```
  Scenario: A travel activity's title ends with its report time in local time
   ✘  Then that activity's title is "LU742 ABC-DEF Report 07:30 LT"
      Step panicked. Captured output: the activity title is not "LU742 ABC-DEF Report 07:30 LT": ...
      SUMMARY:LU742 ABC-DEF
  Scenario: A travel activity without a report time keeps its title unchanged
   ✘  Given a supported duty plan travel leg without a report time
      Step doesn't match any function
  Scenario: Differing report times leave the title without a report suffix
   ✘  Given a compatible departure and arrival fragment of one travel leg supplying different report times 07:30 and 08:00
      Step doesn't match any function
  Scenario: Equal report times on both fragments produce one title suffix
   ✘  Given a compatible departure and arrival fragment of one travel leg supplying the same report time 07:30
      Step doesn't match any function
  [Summary]
  5 features
  144 scenarios (140 passed, 4 failed)
  590 steps (586 passed, 4 failed)
  ```

## Scenario tasks

### 1. A travel activity's title ends with its report time in local time

- [x] RED: run `./run-tests.sh --name "A travel activity's title ends with its report time in local time"`; the existing `Given` resolves and the title assertion fails on the missing suffix.

  ```
  ✔  Given a supported duty plan travel leg supplying Trip ID T42, Report (LT) 07:30, A/C ABC, Blocktime 02:00, Timezone Difference +01, Layover duration 03:00, and Tags 5_MFL
  ✔  When the crew member converts it
  ✘  Then that activity's title is "LU742 ABC-DEF Report 07:30 LT"
     Step panicked. Captured output: the activity title is not "LU742 ABC-DEF Report 07:30 LT": ...
     SUMMARY:LU742 ABC-DEF
  1 scenario (1 failed) / 3 steps (2 passed, 1 failed)
  ```
- [x] GREEN: appended ` Report <value> LT` in `DutyEntry::summary()` for `FieldValue::Single`, and strengthened the shared title `Then` step to require an exact `SUMMARY` line. Production file: `crates/duty2cal-core/src/domain/mod.rs`. Test file: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.

  ```
  Scenario: A travel activity's title ends with its report time in local time
  ✔  Then that activity's title is "LU742 ABC-DEF Report 07:30 LT"
  1 scenario (1 passed) / 3 steps (3 passed)

  ./run-tests.sh (whole suite)
  5 features
  144 scenarios (141 passed, 3 failed)
  590 steps (587 passed, 3 failed)
  failing: the three delta guards whose Givens are still undefined — no permanent scenario regressed.
  ```
- [x] REFACTOR: no behaviour change; code and comments stay truthful.

  ```
  1 scenario (1 passed) / 3 steps (3 passed)
  ```
- [x] COMMIT: `givn commit -m "feat(export-duty-plan-calendar): <release note>"`, body naming this scenario. Revision `64a4ee0`.

### 2. A travel activity without a report time keeps its title unchanged *(guard)*

- [x] RED: run `./run-tests.sh --name "A travel activity without a report time keeps its title unchanged"`; the new `Given` is undefined, so strict mode fails.

  ```
  ✘  Given a supported duty plan travel leg without a report time
     Step doesn't match any function
  1 scenario (1 failed) / 1 step (1 failed)
  ```
- [x] GREEN: added the `Given a supported duty plan travel leg without a report time` step (generated fixture: `LU742` `ABC`→`DEF`, 09:00–10:00, no `Report` cell). Production files: none — the title path appends nothing for `None`. Test file: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.

  ```
  Scenario: A travel activity without a report time keeps its title unchanged
  ✔  Given a supported duty plan travel leg without a report time
  ✔  When the crew member converts it
  ✔  Then that activity's title is "LU742 ABC-DEF"
  1 scenario (1 passed) / 3 steps (3 passed)
  ```
- [x] REFACTOR: no behaviour change.

  ```
  1 scenario (1 passed) / 3 steps (3 passed)
  ```
- [x] COMMIT: same release-note subject, body naming this scenario. Revision `667dabf`.

### 3. Differing report times leave the title without a report suffix *(guard)*

- [x] RED: run `./run-tests.sh --name "Differing report times leave the title without a report suffix"`; the new `Given` is undefined, so strict mode fails.

  ```
  ✘  Given a compatible departure and arrival fragment of one travel leg supplying different report times 07:30 and 08:00
     Step doesn't match any function
  1 scenario (1 failed) / 1 step (1 failed)
  ```
- [x] GREEN: added the `Given a compatible departure and arrival fragment of one travel leg supplying different report times 07:30 and 08:00` step (departure 07:30, arrival 08:00), sharing one fixture helper with the permanent differing-values step. Production files: none — `FieldValue::Differing` appends no suffix in `summary()`. Test file: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.

  ```
  Scenario: Differing report times leave the title without a report suffix
  ✔  Given a compatible departure and arrival fragment of one travel leg supplying different report times 07:30 and 08:00
  ✔  When the crew member converts it
  ✔  Then that activity's title is "LU742 ABC-DEF"
  ✔  And the travel activity's description states "Report (LT) 07:30 (departure fragment)" and "Report (LT) 08:00 (arrival fragment)"
  1 scenario (1 passed) / 4 steps (4 passed)
  ```
- [x] REFACTOR: extracted `report_time_fragments()` so the permanent and delta fixture steps share one definition; no behaviour change.

  ```
  1 scenario (1 passed) / 4 steps (4 passed)
  ```
- [x] COMMIT: same release-note subject, body naming this scenario. Revision `6198868`.

### 4. Equal report times on both fragments produce one title suffix *(guard)*

- [x] RED: run `./run-tests.sh --name "Equal report times on both fragments produce one title suffix"`; the new `Given` is undefined, so strict mode fails.

  ```
  ✘  Given a compatible departure and arrival fragment of one travel leg supplying the same report time 07:30
     Step doesn't match any function
  1 scenario (1 failed) / 1 step (1 failed)
  ```
- [x] GREEN: added the `Given a compatible departure and arrival fragment of one travel leg supplying the same report time 07:30` step (both fragments print 07:30). Production files: none — `combine()` already collapses equal values to `FieldValue::Single`, so `summary()` appends one suffix. Test file: `apps/cli/tests/steps/export_duty_plan_calendar.rs`.

  ```
  Scenario: Equal report times on both fragments produce one title suffix
  ✔  Given a compatible departure and arrival fragment of one travel leg supplying the same report time 07:30
  ✔  When the crew member converts it
  ✔  Then that activity's title is "LU742 ABC-DEF Report 07:30 LT"
  1 scenario (1 passed) / 3 steps (3 passed)
  ```
- [x] REFACTOR: no behaviour change.

  ```
  1 scenario (1 passed) / 3 steps (3 passed)
  ```
- [x] COMMIT: same release-note subject, body naming this scenario. Revision recorded below after the commit.

## Documentation task

- [x] Update the durable architecture documentation for this change: `docs/arc42/05-building-block-view.md` (the `domain` responsibility names the title suffix), `08-crosscutting-concepts.md` (title composition and the conditional report suffix), `10-quality-requirements.md` (QS-021), `12-glossary.md` (**Travel leg** contract use and the new **Report time** term), plus `givn/changes/include-report-time-in-travel-title/arc42.md`. Record the revision identifier.

## Verification task

- [ ] `./run-tests.sh` → every scenario passes; paste the summary line.
- [ ] `./run-tests.sh --e2e` → the `@e2e` scenario passes; paste the summary line.
- [ ] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` → clean; paste.
- [ ] `givn lint --change include-report-time-in-travel-title` → exit 0 (no `@wip` left); paste.
- [ ] `git diff --exit-code -- apps/cli/tests/fixtures/expected/2098-06.ics` → the golden oracle is byte-identical; paste the empty result.
