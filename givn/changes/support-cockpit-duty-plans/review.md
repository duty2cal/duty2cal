# Review: support-cockpit-duty-plans

```text
Review kind: self-review
Author: omp
Reviewer context: self (same context)
```

README-IMPACT: updated - the supported-plan statement, its boundary and the refusal sentence

Reviewed from the tree after the two behavior batches: the reading rules in
`crates/duty2cal-pdf/src/table.rs`, the three cockpit fixtures and steps, the
support-statement constants, carriers, evidence record, check and bats suite,
the affected arc42 chapters, and this change's artifacts. No production code,
spec, design, task, doc or proposal was edited by this review; only this file
was written.

## Implementation evidence

### Step-definition scan

```
$ grep -nE '^\s*(\{\s*\}|\(\s*\)\s*\{\s*\})\s*$|^\s*(pass|return)\s*$|unimplemented!|todo!' \
    apps/cli/tests/steps/export_duty_plan_calendar.rs \
    apps/cli/tests/steps/support.rs
exit=1 (no match)
```

No empty, `unimplemented!`, `todo!`, bare `pass` or bare `return` body exists in
the changed step files. Every new Given builds a generated fixture through
`support.rs`; the shared When runs the built executable; the new Thens inspect
the published `VEVENT` bodies for a date-bound `SUMMARY`/`DTSTART` or for
`DTSTART;VALUE=DATE`, and the footer scenario asserts the process exit status,
the all-day end date and `DTSTAMP`.

### Commit cross-check

| Task | Batch | Revision | Evidence in the revision |
|---|---|---|---|
| Scenario 1 — previous-month tail | cockpit-plan-reading | `f26522e` | `table.rs` adjacent-month resolution, `cockpit_prior_month_plan`, the Given/Then steps, the delta scenario |
| Scenario 2 — next-month tail | cockpit-plan-reading | `f26522e` | same production rule, `cockpit_next_month_tail_plan`, the Given/Then steps, the delta scenario |
| Scenario 3 — close footer | cockpit-plan-reading | `f26522e` | `is_revision_footer` and the footer stop, `footer_beside_last_row_plan`, the Given, the assertions, the delta scenario |
| Scenario 4 — support check across carriers | crew-support-statement | `5536488` | `support-statement.env`, both site checks, all carriers, the shell check, the delta scenario |
| Scenario 5 — evidence names both fixtures | crew-support-statement | `5536488` | `docs/evidence/supported-duty-plan.md`, the check's cockpit requirements, the bats refusal test, the delta scenario |

All revisions are reachable from `HEAD`; each names its batch in the body and
contains the scenario's implementation or regression test. `869a83c`,
`674e591`, `ef08d2c`, `b5285d3` and `8819a36` are bookkeeping (task records,
configuration lock, evidence transcript, gate scope and coverage), not claimed
as scenario evidence. The setup and closeout boxes promise no code.

### Design conformance

- `design.md` named `table.rs`, `apps/cli/tests/steps/export_duty_plan_calendar.rs`,
  `apps/cli/tests/steps/support.rs`, `scripts/support-statement-check.sh` and
  `scripts/tests/support-statement.bats`; all exist and carry the named work.
- The runner and commands are unchanged: `./run-tests.sh` for regular and
  `./run-tests.sh --e2e` for the scoped E2E run. The runner result scope honors
  the gate's `GIVN_RESULT_SCOPE` (recorded in `b5285d3`); the receipt says
  `scope: "change"` with 148 passed.
- No declared contract changed; `RosterSource` still has one implementing
  component, `PdfRosterSource`, and no consumer imports an implementation.

## Interaction coverage

| Inventory entry | Design matrix row | Real `@e2e` scenario | Driver |
|---|---|---|---|
| executable conversion | present | Crew member converts a supported duty plan and receives a calendar file | `std::process::Command` on `CARGO_BIN_EXE_duty2cal` |
| browser conversion and save | present | Crew member converts a supported duty plan in the browser and saves the offered calendar | Playwright driver |
| support-statement check | present | Release owner runs the support-statement check and every carrier carries the statement | `scripts/support-statement-check.sh` |

The cockpit scenarios are variants of the executable action; no new user action
and no second E2E scenario is introduced. `./run-tests.sh --e2e` ran 2
scenarios, both passing, with `--e2e` as the explicit scope selection.

## Coverage

`./measure-coverage.sh` wrote distinct non-e2e and e2e reports and
`./merge-coverages.sh` merged **18 files, 3,328 lines (2,786 covered)** into
`coverage/cobertura-coverage.xml`.

Changed production file `crates/duty2cal-pdf/src/table.rs`: 342 instrumented
lines, 7 uncovered. The 7 are the pre-existing empty-page guard (`page_text`),
the defensive `Month::try_from` and `Date::from_calendar_date` `continue` arms
in `adjacent_dates`, the non-empty-cell separator in `assign_cells`, and the
`parse_iso_date` / `parse_clock` / zone fallback arms of `created_footer`. The
new candidate selection, footer stop and sequence rule are covered by the three
delta scenarios. No new behavior is uncovered; the change adds no threshold or
population change.

## Persona disposition

PERSONAS: crew-member--personal-calendar-owner
PERSONAS: project-maintainer--release-owner
PERSONAS: contributor--web-reviewer

| Persona | Surface | Finding |
|---|---|---|
| `crew-member--personal-calendar-owner` | the cockpit calendars | The five private plans now each publish a complete calendar; prior-month rows are dated in their own month and the footer produces no phantom row. The unknown-code fallback remains visible as the operator chose. |
| `project-maintainer--release-owner` | the support claim | The statement, boundary, organization description and evidence record agree; the check and bats suite refuse a disagreeing carrier or a record without the cockpit fixture. |
| `contributor--web-reviewer` | the page copy | `what-it-does` and the lede carry the new sentence; `scripts/check-site-content.sh dist/site` passes on the built page. |

## Interface classification and visual review

The change touches a rendered Web UI's copy but introduces no screen, control,
state, layout or interaction change. The design's contract is `Interface: web`
for that existing page; `docs/design/reference/converter-reference.html` and the
page were updated, and the built site passes the page-content check. No new
`@e2e` browser scenario or screenshot state is owed. The browser smoke could
not be rebuilt on this host because the Nix Rust sysroot lacks the
`wasm32-unknown-unknown` target; that is an environment limit on an unchanged
route, not a deferred behavior of this change.

VISUAL-REVIEW: NOT OWED — copy-only, no new or changed screen state.

## Arc42, language and structure

- `arc42.md` marks chapters 01, 02, 03, 05, 06, 08, 09, 11 and 12 affected and
  04, 07 and 10 unaffected; an independent walk of the chapter table agrees.
  Each affected chapter contains real, matching text; no ASCII-art diagram was
  found in the 12 chapter files.
- No ADR is qualified: the reading rules are one adapter's local mechanism, and
  the sentence is public copy owned by its use case. ADR-0008 remains accurate.
- The project's durable vocabulary is `docs/arc42/12-glossary.md`; it now names
  `CP` in **Crew position** and the crew duty plan in **Supported duty plan**,
  **Profile content** and **Organization profile**. `docs/glossary.md` does not
  exist, and creating a second authority was deliberately avoided.
  `docs/domain-model.md` does not exist; the change moves no aggregate,
  boundary or relationship, so no structural update is owed.

## Gate checks

- `./run-tests.sh` → 4 features, 148 scenarios (148 passed), 631 steps.
- `./run-tests.sh --e2e` → 2 features, 2 scenarios (2 passed), 10 steps.
- `scripts/support-statement-check.sh` → clean; `bats scripts/tests/support-statement.bats` → 8/8.
- `givn check tasks --change support-cockpit-duty-plans` → receipt `passed` at
  `b5285d3`, scope `change`, 148 passed.
- `givn lint --change support-cockpit-duty-plans` → clean (only advisory subset notices).
- `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` → clean.
- Six cabin-crew calendars in `private/mk/` reconverted byte-identical; all
  five cockpit-crew plans in `private/hs/` exit 0 and publish a warning report
  for their unrecognized codes.
- `scripts/build-site.sh` and `scripts/check-site-content.sh dist/site` → pass.

## Sign-off

- [x] Fabrication audit complete; no empty or no-op step body.
- [x] Every checked scenario carries one reachable batch revision.
- [x] Design's named files and commands exist; no silent deviation.
- [x] Interaction inventory, matrix and real `@e2e` scenarios agree.
- [x] Coverage measured and every changed-file gap classified.
- [x] Persona disposition recorded.
- [x] Arc42, language and structure conformance recorded.
- [x] Both runners green; support check and bats green; cabin calendars unchanged.

REVIEW: PASS
