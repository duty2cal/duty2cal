# Evidence record — the supported duty plan

duty2cal converts a Lufthansa crew duty plan published as a Jeppesen PDF
export into one calendar file. This record states what backs that sentence. It
is the evidence authority the support-statement check reads; no real plan and no
personal identifying value is committed here.

## What the claim rests on

- **Fixture builder:** `apps/cli/tests/steps/support.rs` builds the generated
  detail table through `detail_header_row`, `equivalence_plan`,
  `cockpit_prior_month_plan`, `footer_beside_last_row_plan` and `build_pdf`;
  the reference plan's rows are assembled by `reference_plan` in
  `apps/cli/tests/steps/export_duty_plan_calendar.rs`. No PDF is committed.
- **Roster month:** the cabin fixture uses the fictional month `2099-06`,
  following the project's fictional-year convention. The cockpit fixture uses
  the fictional heading `August 2099` and prints `30 Thu`, `31 Fri` and `01 Sat`
  as its first three dated rows, so the first two belong to the previous month.
  The fixtures' own creation footers are not reproduced as calendar properties.
- **Conversion:** the scenario `Crew member converts a supported duty plan and
  receives a calendar file` in
  `givn/specs/export-duty-plan-calendar/export-duty-plan-calendar.feature`
  exercises the cabin fixture through the unchanged reading, interpretation and
  calendar writer. Two cockpit-plan scenarios exercise the same path:
  `A detail date printed in the previous month is resolved by its weekday`
  converts `cockpit_prior_month_plan`, and
  `A Created footer printed directly below the last detail row is not a detail
  row` converts `footer_beside_last_row_plan`. The browser smoke uses the same
  builder.
- **Expected calendar:** `apps/cli/tests/fixtures/expected/2026-06.ics` is the
  committed reference for the generated cabin plan; it is generated data, not a
  converted real plan.

## What the fixtures print

The generated detail table carries the column labels `Date (LT)`, `Trip`,
`Report (LT)`, `Pos`, `Activity`, `From`, `To`, `Start (UTC)`, `End (UTC)`,
`A/C`, `Blocktime`, `Timezone Difference`, `Layover` and `Tags`.

The generated fixtures print the activity codes `FREE`, `OFF`, `U`, `FZ`, `SB`,
`SBX`, `RE`, `TK`, `DTUSA` and `EM`, and the travel leg `LU742`. The cockpit
fixture prints a cockpit position (`CP`) on its travel rows and an unrecognized
all-day code (`U1`); the latter keeps the documented unknown-code fallback and
produces a warning, so the fixture does not claim a meaning for it.

Separately, the conversion accepts the operator-confirmed standby shift codes
`SB_F_CF`, `SB_M_CF` and `SB_S_CF`. No generated plan prints them, so this
fixture and the committed expected calendars do not exercise them; their
evidence is the capability's Gherkin scenarios, which convert a plan printing
each code through the real executable.

The close-footer fixture prints the complete family footer (`Created`, its date,
its clock, `UTC`, `by`, `Jeppesen`) ten points below its last detail row, so the
footer is inside the table's row pitch. Its scenario proves the footer supplies
the revision timestamp and no dated activity.

## Privacy rule

The private plans stay local and ignored. No real plan, no personal identifying
value and no committed sample enters the repository; the evidence is the
generated fixtures, and the expected calendar follows the fictional date
convention.

## If the evidence changes

If a generated fixture stops converting because it prints an unknown activity
code or omits a column, the conversion refuses and names what could not be read.
The suite is red, so no carrier is published as agreeing; the release owner
revises the carriers and the constants in `scripts/support-statement.env` to the
shrunk sentence and records the failure here. The statement never outlives its
evidence.
