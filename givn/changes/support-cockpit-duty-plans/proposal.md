# Proposal: support-cockpit-duty-plans

## Release note

> Convert a Lufthansa crew duty plan whose detail table begins with days from the previous month, and state the supported plan as one roster month of one Lufthansa crew member without changing the cabin-crew calendars.

## Use-Case Context

- Use-case ID: `export-duty-plan-calendar`; `state-the-converter-support-statement`
- Ideation topic: `generalized-duty-plan-input` (the support statement's source topic; its general-intake direction was withdrawn on 2026-10-27)
- Seed path: none — this change is a follow-on to the implemented `state-the-supported-duty-plan`, not a new seed.
- Confirmed Personas: `crew-member--personal-calendar-owner` (accuracy/privacy lens), `project-maintainer--release-owner` (release-evidence lens), `contributor--web-reviewer` (the page's public statements).

## Review Context

- Review kind: self-review
- Reason: the change corrects two reading rules and one public sentence; it introduces no new external boundary, storage, protocol or deployment commitment. The existing ADR register covers the affected architecture (ADR-0008's owned extraction).

## Problem / Opportunity

The five cockpit-crew Released Roster PDFs in the private corpus use the same Jeppesen detail table as the cabin-crew plans, but they do not all convert:

- August and November print their first two or three dated rows with the day and weekday of the previous month — for example `30 Thu` under `Month: August 2026`. The converter resolves a `DD Ddd` date only inside the heading month, so it rejects `30 Thu` instead of dating it 30 July, and the whole month fails.
- August prints its `Created … (UTC) by …` revision footer close enough to the last detail row that the table-end gap does not separate them. The footer becomes a detail row; `Created` is then read as an unresolved date and the conversion fails.
- The public support statement still says “One roster month of cabin crew; not cockpit duties”. Once these plans convert, every carrier states a boundary the tool crosses.

Three of the five cockpit plans already publish a calendar, but with the documented unknown-code fallback for cockpit-specific activity codes such as `U1`, `OFF_2`, `REP` and `EMCRM`. The operator decided that fallback stays: this change does not assign meanings to new codes.

## Proposed Solution

- A detail date printed as a day and weekday resolves to the unique matching date in the heading month or in the adjacent previous or next month, continuing the plan's row order. A cockpit plan that starts with `30 Thu` and `31 Fri` under an August heading therefore publishes its travel on 30 and 31 July, while a row that matches no nearby date is still refused.
- A `Created … (UTC)` revision footer is metadata and never becomes a detail row, even when it is printed within the table's row pitch. Its revision instant still supplies the calendar's `DTSTAMP`.
- The support statement becomes “duty2cal converts a Lufthansa crew duty plan published as a Jeppesen PDF export into one calendar file.” with the boundary “One roster month of one crew member; not other Lufthansa duty plans, other Jeppesen exports or another operator.” The website, README, organization profile, architecture chapter 3, both issue forms, the organization description and the evidence record carry it, and the evidence record names the generated cockpit fixture and the scenario that exercises it.
- The cabin-crew calendars in `private/mk/` are unchanged; the existing expected calendar and every recognized-code rule stay as they are.

## Capability Routing

`givn spec route --change support-cockpit-duty-plans` reports no signal (the top candidates tie at 13.00). The decisions are recorded directly.

| Proposed capability | Decision | Rationale |
|---|---|---|
| export-duty-plan-calendar | `EXTEND export-duty-plan-calendar` | The actor, goal, calendar contract, publication and reports are unchanged; only two reading rules for dates and footer material change, so the same capability gains the cockpit rows. |
| state-the-support-statement | `EXTEND state-the-support-statement` | The capability already owns the one public sentence and its five carriers; the sentence's noun phrase and boundary change, and that use case is the durable home of both. |

## Out of Scope

- Assigning meanings to `U1`, `OFF_2`, `S5`, `REP`, `SBAO`, `MED`, `AMV`, `WBT_*`, `EMCRM` or `EBTF3*`. They keep the documented unknown-code fallback and warning report. The operator chose this on 2026-11-11.
- Changing any calendar representation rule: all-day/timed classification, UTC endpoint resolution, travel pairing, titles, descriptions, UIDs, `DTSTAMP` fallback, file naming or reports.
- Any other operator, producer or Jeppesen export, and any non-PDF duty plan.
- Importing calendars, reconciling revisions, or editing a plan.
- A new screen, state, control or interaction. The website's existing statement text changes; its layout and behaviour do not.
- A release, version bump or changelog entry.

## Open Questions

### D1 — How should cockpit-specific activity codes be represented?

- Question: Should the converter learn the meanings of `U1`, `OFF_2`, `S5`, `REP`, `SBAO`, `MED`, `AMV`, `WBT_*`, `EMCRM` and `EBTF3*`, or keep the documented unknown-code fallback once the reading succeeds?
- Options:
  1. **Recognize the codes** — `U1`/`OFF_2`/`S5` as all-day; `REP`/`SBAO` as timed availability; `MED`/`AMV`/training codes as timed non-flight duty; titles stay the printed code and no warning is emitted. Gain: warning-free calendars that read like the cabin-crew calendars. Cost: the converter asserts meanings for codes whose only evidence is the operator's word, and each needs a scenario and fixture evidence; in practice, `REP` on 23 July 02:00–21:00 UTC becomes `SUMMARY:REP` with no warning.
  2. **Keep the documented fallback** — fix only the dates and footer; every unknown code keeps `unknown duty: <code>` and a warning. Gain: no new semantic claim, and the existing use case already specifies this fallback. Cost: every cockpit month carries a warning report and visibly labelled fallback events; in practice, the same `REP` row becomes `SUMMARY:unknown duty: REP`.
- Disposition: answered — the operator selected option 2 on 2026-11-11: keep the fallback. Rejected: option 1, because it would add unconfirmed code meanings in a change whose subject is reading the same table for cockpit plans.

### D2 — Which public support sentence replaces the cabin-crew-only claim?

- Question: Once cockpit plans convert, what should the website, README, organization profile, architecture chapter 3, both issue forms and the organization description say the converter supports?
- Options:
  1. **Name both roles explicitly** — “a Lufthansa cabin-crew or cockpit-crew duty plan”; boundary “One roster month of cabin or cockpit crew”. Gain: precise about both observed roles. Cost: the sentence grows and names roles that are still one producer template; in practice, a reader sees both words and the boundary repeats them.
  2. **Say “Lufthansa crew duty plan”** — boundary “One roster month of one crew member”. Gain: short, accurate for the two observed plans, and does not need another edit for the next crew role. Cost: the reader no longer sees the role precision the project adopted on 2026-10-27; in practice, the evidence record carries the cabin and cockpit fixtures that back the generic claim.
  3. **Leave the cabin-only claim** — no carrier changes. Gain: no public-copy work. Cost: five carriers and the organization description would state a boundary the tool crosses, contradicting the project's truthfulness constraint and its support-statement check; in practice, the check would still pass while the page said cockpit duties are out of scope.
- Disposition: answered — the operator selected option 2 on 2026-11-11: “Lufthansa crew duty plan” and “One roster month of one crew member”. Rejected: option 1 for unnecessary role repetition; option 3 because the published claim would be false.
