# Evidence record — the supported duty plan

duty2cal converts a Lufthansa cabin-crew duty plan published as a Jeppesen PDF
export into one calendar file. This record states what backs that sentence. It
is the evidence authority the support-statement check reads; no real plan and no
personal identifying value is committed here.

## What the claim rests on

- **Fixture builder:** `apps/cli/tests/steps/support.rs` builds the detail table
  at test time through `detail_header_row`, `equivalence_plan` and
  `reference_plan`; no PDF is committed.
- **Roster month:** the fixture uses the fictional month `2099-06`, following the
  project's fictional-year convention. The fixture's own creation footer is not
  reproduced as a calendar property.
- **Conversion:** the scenario `Crew member converts a supported duty plan and
  receives a calendar file` in
  `givn/specs/export-duty-plan-calendar/export-duty-plan-calendar.feature`
  exercises the fixture through the unchanged reading, interpretation and
  calendar writer. The browser smoke uses the same builder.
- **Expected calendar:** `apps/cli/tests/fixtures/expected/2026-06.ics` is the
  committed reference for the generated plan; it is generated data, not a
  converted real plan.

## What the fixture prints

The generated detail table carries the column labels `Date (LT)`, `Trip`,
`Report (LT)`, `Pos`, `Activity`, `From`, `To`, `Start (UTC)`, `End (UTC)`,
`A/C`, `Blocktime`, `Timezone Difference`, `Layover` and `Tags`.

The conversion accepts the activity codes `FREE`, `OFF`, `U`, `FZ`, `SB`, `SBX`,
`RE`, `TK`, `DTUSA` and `EM`, and the travel leg `LU742`.

## Privacy rule

The private plans stay local and ignored. No real plan, no personal identifying
value and no committed sample enters the repository; the evidence is the
generated fixture, and the expected calendar follows the fictional date
convention.

## If the evidence changes

If the generated fixture stops converting because it prints an unknown activity
code or omits a column, the conversion refuses and names what could not be read.
The suite is red, so no carrier is published as agreeing; the release owner
revises the carriers and the constants in `scripts/support-statement.env` to the
shrunk sentence and records the failure here. The statement never outlives its
evidence.
