# Proposal: include-report-time-in-travel-title

## Release note

A travel activity's calendar title now ends with its report time in local time — `LU 742 Tychon-Mareum Report 07:30 LT` — so the briefing time is visible without opening the event, while the event details keep the labelled value.

## Use-Case Context

- Use-case ID: `export-duty-plan-calendar` ("Export a duty-plan calendar", primary Actor Crew member)
- Ideation topic: `duty-plan-calendar-conversion` ("Duty-plan calendar conversion")
- Seed path: none — no ideation seed proposes this addition; it extends the shipped conversion of the permanent use case above.
- Confirmed Personas: `crew-member--personal-calendar-owner` ("Crew member exporting their own duty plan") — the title is the text the crew member reads at a glance in their own calendar, which is what this change alters. `project-maintainer--release-owner` ("Maintainer responsible for a public release") is unaffected: the change alters no release artifact, dependency, or public claim.

## Problem / Opportunity

The report column of a supported duty plan prints the local briefing time of a travel assignment. The converter already carries that value into the event details as `Report (LT)`, but the calendar title contains only the printed crew position, the flight number, and the route.

Calendar clients show only the title in month, week, and day views. A crew member scanning a travel day therefore has to open the event to learn when they must report. On a day with several travel legs that is a click per event, and a briefing time that is operationally decisive is effectively hidden.

The correction is additive: the briefing time stays in the event details, and the title also ends with it. The title must show that the value is local time, because the event's clock times are UTC and an unqualified time at the end of a title would be ambiguous.

## Proposed Solution

A travel activity's calendar title ends with a space, the word `Report`, the plan's report value as printed, and the marker `LT`:

```
LU 742 Tychon-Mareum  +  report time 07:30  ->  LU 742 Tychon-Mareum Report 07:30 LT
```

- The suffix is exactly ` Report <value> LT`: one space before `Report`, one space between the parts, no parentheses and no other label. The value keeps the plan's printed form.
- `LT` states that the value is local time. The event's own start and end remain UTC instants; the suffix never becomes one of them.
- One supplied report time produces one suffix. A value supplied equally by the departure and arrival fragments of one leg also produces one suffix.
- When the plan supplies no report time, the title is unchanged.
- When the two fragments of one leg supply different report times, the title carries no report suffix: there is no single briefing time. The event details retain both labelled values (`Report (LT) 07:30 (departure fragment)`, `Report (LT) 08:00 (arrival fragment)`), exactly as they do today.
- The rule applies to travel activities only. All-day absences, availability windows, non-flight duties, and layovers keep their current titles.
- The suffix changes no activity date, no UTC interval, no description field or order, no location, and no component identifier.

## Capability Routing

`givn spec route --change include-report-time-in-travel-title` reports signal: recommendation `EXTEND export-duty-plan-calendar` (use case "export-duty-plan-calendar", score 13.00).

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| export-duty-plan-calendar | EXTEND export-duty-plan-calendar | `EXTEND export-duty-plan-calendar` | The travel title is one observable detail of the existing whole-plan conversion ("Export a duty-plan calendar"). No new actor goal, capability, or use case is introduced. |

## Out of Scope

- The description's `Report (LT)` label, its position in the approved field order, and its fragment-role wording.
- Deriving any activity instant, date, or interval from the report time; the report time remains descriptive.
- Deciding whether a plan supplies a report time, or repairing a missing one.
- The other title components — crew position, flight number, and route — and the failure message that names an unmatched travel fragment.
- The titles of all-day absences, availability windows, non-flight duties, and layovers.
- Calendar alarms, reminders, notifications, or any alarm derived from the report time.
- Calendar-client rendering, import behavior, and reconciliation after a plan revision.
- Regenerating the expected calendar: the reference plan supplies no report time, so its titles stay byte-identical.

## Open Questions

None. The suffix's shape (`Report <value> LT`), its position at the end of the title, and the omission when no single report time exists are settled by the operator's request. No question blocks the spec.
