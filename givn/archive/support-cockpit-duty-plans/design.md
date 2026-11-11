# Design: support-cockpit-duty-plans

Owning capabilities: [export-duty-plan-calendar](../../specs/export-duty-plan-calendar/usecase.md) and [state-the-support-statement](../../specs/state-the-converter-support-statement/usecase.md).

## Context

The cockpit-crew plans use the same Jeppesen detail table as the cabin-crew plans. The reading gap is local to two rules:

1. A `DD Ddd` date cell is resolved only inside the heading month. A plan whose first dated rows are the previous month's tail (`30 Thu`, `31 Fri`) is rejected before interpretation.
2. The table-end rule is a vertical gap greater than 20 pt. A `Created` footer printed 10 pt below the last row is inside that gap, becomes a table row, and `Created` is then reported as an unresolved date.

The public claim is a separate, project-owned sentence carried by the website, README, organization profile, architecture chapter 3, both issue forms, the organization description and the evidence record. It currently excludes cockpit duties.

## Architecture Impact

No module, port, adapter or data-model boundary changes.

| Component | Change |
|---|---|
| `crates/duty2cal-pdf/src/table.rs` | Date resolution admits the adjacent previous and next month; the detail-table reader stops at a parsed `Created … (UTC)` footer. |
| `crates/duty2cal-core` | None. The interpretation, calendar writer, UIDs, `DTSTAMP` fallback and reports are unchanged. |
| `apps/cli` and `ffi/wasm` | None. Both facades use the same `PdfRosterSource`. |
| Public copy, evidence and its checks | The support sentence and boundary change; the evidence record adds the cockpit fixture; the support-statement check and its bats evidence are extended. |

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| `RosterSource` (bytes in, `SourceDocument` out; unchanged) | export a duty-plan calendar | `ConvertRoster` | `PdfRosterSource` in `crates/duty2cal-pdf` | A second publication representation that cannot fill `RawRosterTable` would require a second adapter, not a split of this one. |

This change refines the existing adapter's reading inside the contract; it adds no contract and no component.

## Reading design

### Adjacent-month date resolution

`resolve_date` receives the heading month and the last date it resolved. It parses the day and weekday as today, then builds the candidate dates in the heading month and in the adjacent previous and next months (month arithmetic carries the year). A candidate is kept only when it is a real calendar date whose weekday equals the printed weekday.

The choice is deterministic:

- If a previous dated row exists, choose the earliest candidate at or after that row's date. Plan rows are chronological, so this continues the sequence and does not jump a month backwards. If no candidate is at or after it, the date is unresolved rather than silently dated backwards.
- With no previous row, choose the candidate nearest the first day of the heading month.

If no candidate exists, the date cell is left unresolved and the existing interpretation reports `unresolved date`. The weekday check therefore still refuses a contradictory cell, and a real adjacent-month date is accepted.

The last resolved date is carried across pages in `table::build`, so a page boundary does not reset the sequence. Continuation rows with an empty date cell do not change it.

Worked example, all values generated: heading `Month: August 2099`; rows `30 Thu`, `31 Fri`, `01 Sat`. Candidates for `30 Thu` are 30 July, 30 August (Sunday) and 30 September; only 30 July matches Thursday, so the row dates 30 July. `31 Fri` chooses 31 July after the previous date. `01 Sat` chooses 1 August.

### Footer is not a detail row

Before a clustered row is considered for the table, the reader joins its words and checks whether the line starts with `Created` and parses as the family's revision footer (`Created DDMonYYYY HH:MM (UTC) by …`). When it does, the reader ends the page's detail rows. The footer was already read from the page text for the source revision, so `DTSTAMP` is unaffected.

This is stronger than the 20 pt gap: it also covers a footer printed inside the row pitch, which is the August 2026 case. It does not rely on the row's x position or on a page-number token.

## Test design

### Test runner and strict mode

- Full suite: `./run-tests.sh` (cargo unit/property/architecture/golden tests plus the assembled Gherkin corpus).
- E2E-only: `./run-tests.sh --e2e` (`@e2e and not @browser`).
- Single scenario: `./run-tests.sh --name "<scenario title>"`.
- Strict mode: `apps/cli/tests/features_runner.rs` calls `.fail_on_skipped()` on the cucumber-rs builder; an undefined or skipped step fails the run. The change adds no runner configuration.

### Step definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| export-duty-plan-calendar | `apps/cli/tests/steps/export_duty_plan_calendar.rs` | same file |
| export-duty-plan-calendar (fixture builders) | `apps/cli/tests/steps/support.rs` | same file |
| state-the-support-statement | `scripts/support-statement-check.sh` and `scripts/tests/support-statement.bats` (the `@documentation` feature is not executed by `run-tests.sh`) | same files |

The two new regular steps are `Then the calendar contains a travel activity on <date> titled "<title>"` and `Then the calendar contains an all-day activity on <date>`. They read the published `roster.ics` through the existing `calendar(w)` helper and inspect `VEVENT` blocks. No browser step and no E2E step is added.

`support.rs` gains two generated fixture builders:
`cockpit_prior_month_plan()` (heading `August 2099`, rows `30 Thu`, `31 Fri`, `01 Sat`, full revision footer) and `footer_beside_last_row_plan()` (one `2099-06-07` all-day row followed 10 pt later by a full `Created … (UTC) by …` footer).

### Test technique

- The prior-month scenario drives the real executable and observes the published calendar, so it exercises the same reader the product uses.
- The footer scenario's fixture prints the complete footer words (`Created`, date, time, `(UTC)`, `by`, `Jeppesen`) so the footer row reaches the date cell under the current code instead of being skipped as a silent layout row. Its primary assertion is the process exit status plus the all-day end dates; the revision timestamp assertion proves the footer was still read as metadata.
- Old code fails both: the prior-month row reports `unresolved date: "30 Thu"`, and the footer row reports `unresolved date: "Created"`.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| `export-duty-plan-calendar`: supply a duty plan for conversion through the executable | Crew member converts a supported duty plan and receives a calendar file | CLI (terminal) | `std::process::Command` runs `env!("CARGO_BIN_EXE_duty2cal")` on the generated plan and reads the published `roster.ics` and transcript |
| `export-duty-plan-calendar`: supply a duty plan through the converter page and save the offered calendar | Crew member converts a supported duty plan in the browser and saves the offered calendar | Web UI | Playwright driver (`web/e2e/driver.mjs`) navigates the built Site output, sets/drops the file, clicks the offer and reads the saved download |
| `state-the-support-statement`: run the support-statement check before publication | Release owner runs the support-statement check and every carrier carries the statement | CLI (shell check) | `scripts/support-statement-check.sh` over a temporary carrier tree; `scripts/tests/support-statement.bats` is its executable evidence |

The change adds no interaction. The two cockpit reading scenarios are variants of the first action, not a new user action, so they are regular scenarios and no second `@e2e` is added.

## Visual Design Contract

```
- Interface: web
- Viewports: mobile (360x780), desktop (1920x1080)
- Design system: docs/design/design-system.md
```

- **Screens covered:** the existing converter page's statements panel, copy-only: the `what-it-does` statement and the lede change to the new sentence. No component, layout, state, control or interaction changes.
- **Design tokens, primitives, breakpoints, accessibility and motion:** unchanged from `docs/design/design-system.md`. The existing browser baseline assertions (headings, labels, duplicated content, reduced motion, axe) remain in force.
- **Rendered reference:** `docs/design/reference/converter-reference.html` is updated to the new statement; it is the reference view for the text change.
- **Capture mechanism and evidence path:** no new `@e2e` browser scenario is introduced. The existing browser scenario remains the real-interface evidence and captures its screenshots at the paths the archived `build-browser-converter` evidence already names. A copy-only change does not add a state.

Persona impact: `contributor--web-reviewer` reads the page copy; `crew-member--personal-calendar-owner` reads whether their plan is in scope; `project-maintainer--release-owner` answers for the claim's evidence.

## External boundary cases

| Boundary | Adversarial case | Expected behaviour |
|---|---|---|
| PDF date cell | weekday matches only the previous or next month | resolved in that month |
| PDF date cell | weekday matches no date in the three months | existing `unresolved date` error |
| PDF date cell | day absent from all three months (e.g. 31 in a 30-day heading month) | existing `unresolved date` error |
| PDF revision footer | footer within the table's row pitch | not a detail row; revision still parsed |
| PDF revision footer | line starts with `Created` but does not parse as the family's stamp | remains table material and keeps the existing date error |
| Support carriers | one carrier differs | check refuses and names it |
| Evidence record | cockpit fixture missing | check refuses and names the record |

## Local runnability

- Local run command: `./run-tests.sh`; the change needs no server, database, container or third-party service.
- Digital twins: none; the conversion is local and the browser route is served by a local Playwright driver.
- Interface obstacle: none. The CLI is driven as a subprocess; the browser route is unchanged and needs no new driver capability.

## Coverage process boundaries

Coverage is enabled. This change adds no production process; it changes existing code inside `crates/duty2cal-pdf` and adds two Gherkin scenarios.

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| `features_runner` | `cargo llvm-cov --test features_runner` from `./measure-coverage.sh` | the instrumented test binary | cargo-llvm-cov's per-process profile | `./measure-coverage.sh` writes `coverage/non-e2e-cobertura.xml`; `./merge-coverages.sh` merges | `crates/duty2cal-pdf/src/table.rs` (adjacent-month candidate selection) |
| `duty2cal` CLI child | the `@e2e` and conversion steps through `env!("CARGO_BIN_EXE_duty2cal")` | the cargo-built, instrumented CLI binary | `LLVM_PROFILE_FILE` inherited from the runner; flushed on exit | the same `measure-coverage.sh` run folds it into the matching scope | `crates/duty2cal-pdf/src/table.rs` (footer stop) |

Measured population and exclusions: production source under `crates/` and `apps/cli/src`; tests, fixtures, the browser bundle and generated code excluded as `measure-coverage.sh` already declares. The support-statement shell check is not part of the Rust coverage population; its evidence is the bats file.

## Use-case traceability

| Use-case guarantee, rule or event | Technical decision | Evidence |
|---|---|---|
| Every dated detail row is accounted for; a previous-month row is dated, never refused | adjacent-month candidate resolution (above) | Scenario `A detail date printed in the previous month is resolved by its weekday` |
| Overview, statistics and footer material supply no dated activities | footer stop (above) | Scenario `A Created footer printed directly below the last detail row is not a detail row`; the existing footer scenarios remain |
| A date whose weekday matches no candidate is still refused | candidate filter returns none | Existing scenario `A date cell whose weekday contradicts the day is not accepted as activity timing` |
| The public statement names one plan, its boundary and the non-affiliation sentence across five carriers | D2 | Scenario `Release owner runs the support-statement check and every carrier carries the statement` |
| The claim rests on exercised evidence | evidence record names the cabin and cockpit fixtures | Scenario `The evidence record names both crew-role fixtures that exercise the claim` |
| Unrecognized codes are represented from printed evidence with a warning | D1 (unchanged fallback) | Existing unknown-code scenarios remain; no new scenario for a code meaning |

## Decisions

- **Decides D1:** keep the documented unknown-code fallback; no cockpit code gains a meaning. Realized by leaving `ALL_DAY_CODES`, `AVAILABILITY_CODES` and `NON_FLIGHT_CODES` unchanged and by the proposal's out-of-scope list.
- **Decides D2:** the public sentence becomes “duty2cal converts a Lufthansa crew duty plan published as a Jeppesen PDF export into one calendar file.”; the boundary becomes “One roster month of one crew member; not other Lufthansa duty plans, other Jeppesen exports or another operator.” Realized in `scripts/support-statement.env`, every carrier, the organization description and `docs/evidence/supported-duty-plan.md`.

## Black-Box-First policy

Both new scenarios drive the built executable and read the published calendar and its exit status. No internal unit test is added: the date-resolution arithmetic and footer recognition are observed through the real interface. The existing domain unit tests and the golden calendar stay unchanged.

## Risks

- The candidate rule could accept a typo that happens to match an adjacent month. The sequence rule and the three-month window bound this; the existing contradictory-weekday scenario remains the negative evidence.
- The footer recognition depends on the family's `Created` spelling. A future producer spelling would fall back to the 20 pt gap and the existing date error, rather than silently inventing an activity. The evidence record names the spelling.
