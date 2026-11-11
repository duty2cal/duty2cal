# Tasks: support-cockpit-duty-plans

Release note (commit subject): Convert a Lufthansa crew duty plan whose detail table begins with days from the previous month, and state the supported plan as one roster month of one Lufthansa crew member without changing the cabin-crew calendars.

Owning capabilities: [Export a duty-plan calendar](../../specs/export-duty-plan-calendar/usecase.md) and [State the support statement](../../specs/state-the-converter-support-statement/usecase.md).

Single-scenario run command: `./run-tests.sh --name "<scenario title>"`.
Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`.

## Setup task — runner, strict mode, baseline

- [x] Confirm the existing Gherkin runner and strict mode; this change adds no runner configuration.
  - `apps/cli/tests/features_runner.rs` sets `.fail_on_skipped()` on the cucumber-rs builder, so an undefined or skipped step fails the run.
  - `./run-tests.sh` assembles the permanent corpus and the active change's specs via `scripts/assemble-features.sh`; `@documentation` features are not executed.
  - Single-scenario command: `./run-tests.sh --name "<title>"`.
  - Baseline after the change: `./run-tests.sh` -> 4 features, 148 scenarios (148 passed); `./run-tests.sh --e2e` -> 2 features, 2 scenarios (2 passed). The E2E count is strictly less than the full count.
  - `./run-tests.sh --name "A detail date printed in the previous month is resolved by its weekday"` -> 1 scenario (1 passed).
  - `givn lint --change support-cockpit-duty-plans` -> clean (only the advisory subset notices remain); no `@wip` scenario remains.

## Behavior batch: cockpit-plan-reading

Design constraints:
- `crates/duty2cal-pdf/src/table.rs` resolves a `DD Ddd` date against the heading month and its adjacent previous and next months. It uses the printed weekday, chooses the earliest candidate at or after the previous dated row (or the candidate nearest the heading month's first day when there is no previous row), and leaves the cell unresolved when no candidate qualifies.
- The last resolved date is carried across pages in `table::build`; continuation rows do not reset it.
- A row whose raw text starts with `Created` and parses as `Created DDMonYYYY HH:MM (UTC) …` ends the page's detail rows. The footer is still read as the source revision for `DTSTAMP`.
- The interpretation, calendar writer, UIDs, descriptions and reports are unchanged.
- Fixtures: `cockpit_prior_month_plan()`, `cockpit_next_month_tail_plan()` and `footer_beside_last_row_plan()` in `apps/cli/tests/steps/support.rs`.
- Steps: the two date/title and all-day assertions live in `apps/cli/tests/steps/export_duty_plan_calendar.rs`; no new step file and no browser driver.

### 1. A detail date printed in the previous month is resolved by its weekday

Use-case constraint: every dated detail row is accounted for; a cabin or cockpit plan may begin with the previous month's tail, and the row's day and weekday must date it without inventing an August event.

- [x] GREEN — fixture and assertion in place; production rule in `table.rs`; scenario passes.
- [x] COMMIT: f26522e -- Batch: cockpit-plan-reading

### 2. A detail date printed in the next month is resolved by its weekday

Use-case constraint: the adjacent-month rule is symmetric; a row after the heading month's last day resolves to the next month when the row order continues.

- [x] GREEN — fixture and assertion in place; production rule in `table.rs`; scenario passes.
- [x] COMMIT: f26522e -- Batch: cockpit-plan-reading

### 3. A Created footer printed directly below the last detail row is not a detail row

Use-case constraint: footer records are not dated activities, even when the footer sits inside the table's row pitch; the revision instant still supplies `DTSTAMP`.

- [x] GREEN — fixture prints the complete `Created … (UTC) by …` footer 10 pt below the last row; the reader stops at it; the scenario passes with the revision timestamp intact.
- [x] COMMIT: f26522e -- Batch: cockpit-plan-reading

## Behavior batch: crew-support-statement

Design constraints:
- `scripts/support-statement.env` is the one wording source: `SUPPORT_PROJECT`, `SUPPORT_SITE`, `BOUNDARY`, `SUPPORT_SHRUNK` and `SUPPORT_SITE_SHRUNK`.
- Five carriers: `web/index.html` (`what-it-does` and lede), `README.md`, `org/profile/README.md`, the `docs/arc42/03-context-and-scope.md` subsection, and both `.github/ISSUE_TEMPLATE/{bug,proposal}.yml`; the organization description (`infra/main.tf`, `infra/README.md`, the two fake fixtures) carries the same plan phrase.
- `docs/evidence/supported-duty-plan.md` names `cockpit_prior_month_plan`, `cockpit_next_month_tail_plan`, `footer_beside_last_row_plan` and the three cockpit scenarios.
- `scripts/support-statement-check.sh` requires the cockpit fixture and scenarios; `scripts/tests/support-statement.bats` covers the clean run and the missing-cockpit-fixture refusal.
- These scenarios are `@documentation`, so the Gherkin runner skips them; the executable evidence is the shell check and its bats suite.

### 4. Release owner runs the support-statement check and every carrier carries the statement

Use-case constraint: the statement names a Lufthansa crew duty plan, its boundary and the non-affiliation sentence; the same statement is carried by all five carriers and the evidence record; a disagreeing carrier is named.

- [x] GREEN — `scripts/support-statement-check.sh` exits zero and prints the clean line; all five carriers agree; the check names no refusal.
- [x] COMMIT: 5536488 -- Batch: crew-support-statement

### 5. The evidence record names both crew-role fixtures that exercise the claim

Use-case constraint: a plan is named only with exercised evidence; the cabin and cockpit fixtures both convert through the unchanged conversion.

- [x] GREEN — the evidence record names `cockpit_prior_month_plan`, `cockpit_next_month_tail_plan`, `footer_beside_last_row_plan` and their scenarios; the bats suite refuses a record that drops the cockpit fixture.
- [x] COMMIT: 5536488 -- Batch: crew-support-statement

## Existing E2E coverage

- [x] No new `@e2e` scenario is added. The cockpit cases are variants of the existing executable conversion action; the browser route and the support-statement check retain their existing `@e2e` scenarios. `./run-tests.sh --e2e` runs 2 scenarios and passes.

## Closeout

- [x] `./run-tests.sh` and `./run-tests.sh --e2e` exit 0.
- [x] `scripts/support-statement-check.sh` exits 0 and `bats scripts/tests/support-statement.bats` passes.
- [x] `givn lint --change support-cockpit-duty-plans` exits clean.
- [x] The six cabin-crew calendars in `private/mk/` are byte-identical after reconversion, and all five cockpit-crew plans in `private/hs/` exit 0.
