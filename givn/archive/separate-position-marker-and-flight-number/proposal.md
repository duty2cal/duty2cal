# Proposal: separate-position-marker-and-flight-number

## Release note

> A travel activity's calendar title prints the crew position and the flight number as two words separated by a space — an operating leg reads `FB 123 ABC-DEF` instead of `FB123 ABC-DEF`, while positioning travel keeps reading `DH 007 ABC-DEF`.

## Use-Case Context

- Use-case ID: export-duty-plan-calendar ("Export a duty-plan calendar", primary Actor Crew member)
- Ideation topic: duty-plan-calendar-conversion ("Duty-plan calendar conversion")
- Seed path: none — no ideation seed proposes this correction; it corrects the shipped conversion of the permanent use case above.
- Confirmed Personas: `crew-member--personal-calendar-owner` ("Crew member exporting their own duty plan") — the event title is text the crew member reads at a glance in their own calendar, which is what this correction changes. `project-maintainer--release-owner` ("Maintainer responsible for a public release") is unaffected: this change alters no release artifact or claim.

## Problem / Opportunity

A supported published plan prints two separate detail columns for a travel row: the **position** column carries the crew position the plan assigns (in the operator's examples `FB`, Flugbegleiter — German for flight attendant — and `DH` for positioning travel), and the **activity** column carries the **flight number**.

The converter's travel title splices the two into one token. Against the operator's local plans (redacted), every operating leg yields a title of the form `<position><number> <from>-<to>`, for example `FB123 ABC-DEF`. A reader cannot tell where the crew position ends and the flight number begins, and the run-together text reads like a single flight designator — an airline code followed by a number — rather than a crew position followed by a flight number.

Positioning travel does not share the defect: it already reads `DH 007 ABC-DEF`, because the position marker is printed ahead of the flight number there. So the two travel titles disagree in shape, and the operating one is the reported defect.

The operator's correction, in their words: the numbers behind `FB` or `DH` are flight numbers, and there must be a space between the letters and the numbers.

## Proposed Solution

A travel activity's calendar title prints the crew position and the flight number as two words separated by a single space.

- An operating leg whose plan prints position `FB` and flight number `123` reads `FB 123 ABC-DEF`; the same leg currently reads `FB123 ABC-DEF`.
- Positioning travel is unchanged in appearance: position `DH`, flight number `007` reads `DH 007 ABC-DEF`.
- The rule follows the plan, not a fixed list of markers. Whatever the position column prints (`FB`, `DH`, `AK`, …) is kept verbatim, separated from the flight number by one space. A marker the converter does not recognise is still printed as the plan prints it; no position value selects an activity kind or a timing.
- A row that prints no position keeps its printed activity text unchanged, exactly as today: the flight number alone reads `123 ABC-DEF`, and an all-activity-cell code such as `LU900` is not split.
- Printed leading zeroes survive, as the permanent use case already requires: `DH 007 ABC-DEF`.
- The separation holds wherever the converter prints the two together, including a failure message that names an unmatched travel fragment, so no output splices them.
- A position that is not a crew position stays as the plan prints it: the `EM` row of the operator's plans prints position `AK` beside activity `EM`, and its title remains `EM`.

Vocabulary used here: the printed letters are the **crew position**; the printed digits are the **flight number**. Where a plan prints them in two columns, a calendar title prints them as two words. The permanent use case's rule that printed designators — "including leading zeroes" — are preserved and never converted into a numeric quantity is retained, with its wording corrected to name the crew position and the flight number it actually protects.

Unchanged by this change: which rows become activities, their classification, their dates and UTC intervals, the route text in the title, the calendar location value, the description fields and their order, and component identities — a title is not identity, and the same source snapshot keeps its component identifiers.

## Capability Routing

`givn spec route --change separate-position-marker-and-flight-number` reports signal: recommendation `EXTEND export-duty-plan-calendar` (use case "export-duty-plan-calendar", score 13.00).

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| export-duty-plan-calendar | EXTEND export-duty-plan-calendar | `EXTEND export-duty-plan-calendar` | The travel title is one observable detail of the existing whole-plan conversion ("Export a duty-plan calendar"). No new actor goal, capability, or use case is introduced. |

## Out of Scope

- Which plan rows become activities, how they are classified, and which codes are recognised.
- Activity dates, UTC intervals, overnight resolution, and the evidence that supplies timing.
- The route text in a title (`ABC-DEF`) and the calendar location value.
- Component identifiers: the title is not identity, and this correction changes no UID.
- The description fields, their labels, their order, and their fragment-role wording.
- The supported source family and the detail columns it must print.
- The meaning of deadhead duty and its placement in the title, which the permanent use case already records.
- Failure-message wording other than naming the crew position and flight number separately.
- Importing an exported calendar, and reconciling a calendar already imported after the plan is revised.

## Open Questions

None. The required rendering is settled by the operator's correction, and the layout it applies to is verified against the operator's local plans: the position and the flight number are two printed columns, so the separation has one unambiguous form. No question blocks the spec.
