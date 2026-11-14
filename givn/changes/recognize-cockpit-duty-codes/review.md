# Review: recognize-cockpit-duty-codes

```text
Review kind: self-review
Author: omp
Reviewer context: self (same context)
```

README-IMPACT: none

## Implementation evidence

### Step-definition scan

```
$ grep -nE '^\s*(\{\s*\}|\(\s*\)\s*\{\s*\})\s*$|^\s*(pass|return)\s*$|unimplemented!|todo!' \
    apps/cli/tests/steps/export_duty_plan_calendar.rs \
    apps/cli/tests/steps/support.rs
exit=1 (no match)
```

No empty or no-op step body exists in the changed step files. The new Then
inspects one `VEVENT` for the exact `SUMMARY` and the matching `DTSTART`/`DTEND`
on the named date; the existing all-day and warning steps carry the other
assertions.

### Commit cross-check

| Task | Batch | Revision | Evidence |
|---|---|---|---|
| 1. All-day codes ignore printed clocks | cockpit-code-recognition | `b01b3be` | `ALL_DAY_CODES` gains `U1`/`OFF_2`; the modified outline's six examples |
| 2. Medical and training duties use their printed UTC window | cockpit-code-recognition | `b01b3be` | `NON_FLIGHT_CODES` gains `MED`/`EMCRM`; `NON_FLIGHT_PREFIXES` and the predicate; the eight-example outline |
| 3. A code outside the confirmed training families still warns | cockpit-code-recognition | `b01b3be` | the near-miss outline for `WBTX`/`EBTF4A` |

`5df8d17` (managed AGENTS blocks), `acb12f1` (configuration lock and evidence),
`a51a894` (task record), `4f305cb` (verification receipt) and `79d6783`
(coverage) are bookkeeping; none is claimed as scenario evidence.

### Design conformance

- The changed production file is exactly `crates/duty2cal-core/src/application/mod.rs`;
  no port, adapter, ICS writer or facade was touched.
- `NON_FLIGHT_CODES` is `TK`, `DTUSA`, `EM`, `MED`, `EMCRM`;
  `NON_FLIGHT_PREFIXES` is `WBT_`, `EBTF3`; `classify` calls the predicate.
- The step file is the one `design.md` named; the new Then is the one new
  binding. No new fixture builder was needed.
- The runner and commands are unchanged; the receipt at `acb12f2`/`4f305cb`
  is scope `change`, 160 passed.

## Interaction coverage

| Inventory entry | Matrix row | Real `@e2e` scenario | Driver |
|---|---|---|---|
| executable conversion | present | Crew member converts a supported duty plan and receives a calendar file | `std::process::Command` on `CARGO_BIN_EXE_duty2cal` |
| browser conversion and save | present | Crew member converts a supported duty plan in the browser and saves the offered calendar | Playwright |
| support-statement check | present | Release owner runs the support-statement check and every carrier carries the statement | shell check |

No new action and no new `@e2e` scenario. `./run-tests.sh --e2e` ran 2 scenarios,
both passing.

## Coverage

`./measure-coverage.sh` and `./merge-coverages.sh` merged **18 files, 3,334 lines
(2,792 covered)**. The changed file `application/mod.rs` has 488 instrumented
lines and 12 uncovered: the construct-page formatter, the `ConvertError` display
and source fallbacks, the `merge_travel` error arm, and the missing-carried-date
arm. All are pre-existing error/format branches; the new all-day membership,
non-flight membership and prefix predicate are covered by the new outlines.

## Persona disposition

PERSONAS: crew-member--personal-calendar-owner
PERSONAS: project-maintainer--release-owner

| Persona | Surface | Finding |
|---|---|---|
| `crew-member--personal-calendar-owner` | the cockpit calendars | `U1`, `OFF_2`, `MED`, `WBT_*`, `EMCRM` and `EBTF3*` no longer carry `unknown duty:` or a warning; the event title is the printed code and the timed window/station are the row's own. The printed station remains the location; the user's “at home”/“at the company” is not turned into an invented venue. |
| `project-maintainer--release-owner` | the evidence record and statement check | The record names the confirmed codes and the three scenarios; the check requires them, and the bats suite refuses a record that drops `MED`. |

## Interface classification and visual review

The change adds no screen, state, control or layout. The only page-visible effect
is that a cockpit plan with no remaining unknown code may show fewer entries in
the existing `Unknown duties` list; the page's structure and states are
unchanged. No new `@e2e` browser scenario or screenshot state is owed.

VISUAL-REVIEW: NOT OWED — no new or changed screen state.

## Arc42, language and structure

- `arc42.md` marks chapters 11 and 12 affected; the other ten unaffected. An
  independent walk agrees. R-074 now names the confirmed prefixes and TD-029
  records the 2026-11-14 meanings; the glossary adds **Day off**, **Medical
  appointment duty** and **Training duty** and updates **Vacation day** and the
  code invariant.
- No ADR is qualified: the change edits constants and a predicate inside the
  existing classifier; ADR-0008 and the label-unknown-duties decision remain
  accurate.
- The project's durable vocabulary is `docs/arc42/12-glossary.md`; no
  `docs/glossary.md` exists. No structural relationship moved, so no
  `docs/domain-model.md` update is owed.

## Gate checks

- `./run-tests.sh` → 4 features, 160 scenarios (160 passed), 679 steps.
- `./run-tests.sh --e2e` → 2 features, 2 scenarios (2 passed), 10 steps.
- `scripts/support-statement-check.sh` → clean; `bats scripts/tests/support-statement.bats` → 9/9.
- `givn check tasks --change recognize-cockpit-duty-codes` → receipt passed at
  `acb12f1`, scope `change`, 160 passed.
- `givn lint --change recognize-cockpit-duty-codes` → clean.
- `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` → clean.
- Six cabin-crew calendars in `private/mk/` reconverted byte-identical; all five
  cockpit plans exit 0; August and September publish no warning report, July
  keeps `AMV`/`REP`, October keeps `S5`, November keeps `SBAO`.

## Sign-off

- [x] Fabrication audit complete; no empty or no-op step body.
- [x] Every checked scenario carries the batch revision.
- [x] Design's named files and commands exist; no silent deviation.
- [x] Interaction inventory, matrix and `@e2e` scenarios agree.
- [x] Coverage measured and every changed-file gap classified.
- [x] Persona disposition recorded.
- [x] Arc42, language and structure conformance recorded.
- [x] Both runners green; support check and bats green; cabin calendars unchanged.

REVIEW: PASS
