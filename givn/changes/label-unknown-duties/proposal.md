# Proposal: label-unknown-duties

## Release note

Every calendar entry for an unrecognized duty code is prefixed with `unknown duty: <code>`, and the browser's ready page lists those duties and offers the calendar and the publisher warning with clear save labels.

## Use-Case Context

- Use-case ID: `export-duty-plan-calendar` — the capability that converts one supplied duty plan into a calendar and, when a row used a fallback, the warning report of the same export.
- Ideation topic: `duty-plan-calendar-conversion` (the use case's own source topic; this change has no ideation change seed)
- Seed path: none
- Confirmed Personas: `crew-member--personal-calendar-owner` — the accuracy and privacy lens of this use case, who reads the calendar and its page; `contributor--web-reviewer` — the confirmed lens for the browser realization's page, its copy and its offers. Neither Persona is invented or promoted.

## Problem / Opportunity

A crew member who imports the exported calendar meets an unrecognized duty code as a bare event title: `ZZ_M_CF`. Nothing in the calendar says the code came from a fallback, so the entry reads like a recognized activity with a strange name. The only explanation lives in the warning report or the page's diagnostics, which the calendar no longer accompanies once it is imported.

The page's ready state has the matching problem in the other direction. Its heading and copy describe the old behaviour ("Your complete calendar is ready. The browser chooses where the file is saved."), its warning list is a list of row-number diagnostics rather than a list of the calendar entries a crew member will actually see, and its warning-report offer carries the same generic `Save <file>` label as the calendar. A reader cannot tell at a glance which entries are fallbacks, in which order they occur, or which of the two offered files is the calendar and which is the note for the publisher.

The generated fixture fixes the concrete shape: a plan whose row prints `ZZ_M_CF` with a printed window lists `07-06-2099 08:00 UTC unknown duty: ZZ_M_CF` under `Unknown duties`, in plan order, beside the two clearly labelled saves. The operator's attached November plan is a stale example: its three standby shift codes `SB_F_CF`, `SB_M_CF` and `SB_S_CF` are recognized, so it currently converts with no unknown duties; recognition is not reverted, and the scenarios use generated fixtures.

## Proposed Solution

The calendar and the browser's ready states change; the conversion outcome, the warning report and the failure state do not.

**Calendar entries.** Every activity represented from an unrecognized code is titled `unknown duty: <printed code>`, for both the timed fallback and the all-day fallback. Location, printed UTC window and the rule that the fallback carries no description stay exactly as they are. A calendar of recognized activities is unchanged.

**Both ready states.**

| Element | Ready | Ready with unknown duties |
|---|---|---|
| Heading | `Calendar ICS ready` | `Calendar ICS ready` |
| Status | `Calendar ready for <source>.` | `Your calendar is ready to save. <n> rows were converted with unknown duties.` — `Your calendar is ready to save. 1 row was converted with an unknown duty.` for one row |
| List heading | absent | `Unknown duties` |
| List entry, timed | — | `DD-MM-YYYY HH:MM UTC unknown duty: <code>` |
| List entry, all-day | — | `DD-MM-YYYY unknown duty: <code>` |
| Offer copy | `You can choose where to save the file.` | `You can choose where to save the file.` |
| Calendar offer | `Save <source-stem>.ics` | `Save <source-stem>.ics` |
| Warning-report offer | absent | `Save unknown duty warning for publisher` |
| Way back | `Open new/next duty plan PDF` | `Open new/next duty plan PDF` |

The list is in plan order, one entry per converted row, so the page and the calendar can be read side by side. The warning report's filename stays `<source-stem>.parse-warnings.txt`; only the offer's label becomes specific.

**Unchanged.** The page's failure state keeps its heading, its count-based reason sentence, its `Errors` and `Warnings` lists, its `Choose another plan` way back and its failure-report offer. The warning report's contents and filename, the executable's transcript and warning lines, every diagnostic sentence, the calendar's dates, times, descriptions and identities, and the two `@e2e` inventory actions are unchanged.

## Capability Routing

`givn spec route --change label-unknown-duties` reports no signal: the top candidates tie, so it prints no ranked list. The decision is recorded with its rationale.

| Proposed capability | Decision | Rationale |
|---|---|---|
| `export-duty-plan-calendar` | `EXTEND export-duty-plan-calendar` | The actor, the goal, the source, the publication and the two actions stay the same; only the presentation of a conversion warning in the calendar title and in the page's ready states changes. A new capability would split one goal across two names for no new actor outcome, and no new capability name is goal-shaped here. |

The two interaction-inventory actions in `usecase.md` remain exactly as they are: supplying a plan through the executable, and supplying a plan through the converter page and saving the offered calendar. No new user action is introduced, and no new `@e2e` scenario is added.

## Out of Scope

- The page's failure state, its heading, reason sentence, lists, way back and failure-report offer.
- The warning report's filename, its `Source PDF:`/`Warnings:` text and the row-number diagnostics it carries.
- The executable's transcript lines, its warning lines and its exit status.
- Changing which codes are recognized. `SB_F_CF`, `SB_M_CF` and `SB_S_CF` remain recognized standby shifts and produce no warning.
- The calendar's RFC 5545 shape, dates, UTC windows, description fields, component identities, revision timestamp and file naming.
- A new user action, a new `@e2e` scenario or a new browser state: the ready state and the ready-with-unknown-duties state keep their existing structure.
- Any visual token, layout, breakpoint or dependency change.

## Open Questions

### D1 — Where do the ready page's unknown-duty facts come from?

- Question: The ready list needs a date, an optional interval start and a code per unknown duty, but the conversion outcome exposes only the calendar text and warnings whose message is one sentence. Should the page parse those sentences, or should the conversion produce a structured fact?
- Options:
  1. **Derive a structured fact from the represented activities** — the conversion emits, in plan order, each unknown duty's duty-plan date, its UTC interval start when timed, and its printed code; the page composes the visible line from those fields. Gain: the line is deterministic, testable through the real page and independent of the warning sentence's wording. Cost: one small application type and one new field on the browser result. In practice: the warning still reads `row 1 (2099-06-07): unrecognized activity code …`, while the page shows `07-06-2099 08:00 UTC unknown duty: ZZ_M_CF` from the structured fields.
  2. **Parse the diagnostic message in the page** — the page extracts a code from the warning text and uses the diagnostic's date. Gain: no conversion change. Cost: the page is coupled to an English sentence, loses the interval start (the message carries it only as `08:00-15:30`), and breaks when the sentence is reworded. In practice: a message change silently produces an empty or malformed list.
- Disposition: carried — `design.md` decides the shape of the structured fact and its browser exposure; the code, date and optional start are the settled minimum.

### D2 — Does the new title cover every unrecognized activity, including a standby-shaped code?

- Question: The standby-shaped code `SB_X_CF` is not one of the three recognized shift codes, so it takes the unrecognized-code fallback and warns. Should its calendar title also be prefixed?
- Options:
  1. **Yes, every activity represented from an unrecognized code** — one rule, one home, and the title states the same fallback the warning reports; three further permanent scenarios whose expected title is the bare code change with it. Gain: the calendar never presents a fallback as a recognized activity. Cost: every such entry gains the `unknown duty: ` prefix. In practice: `unknown duty: SB_X_CF`.
  2. **Only codes without a standby shape** — keeps `SB_X_CF` looking like the recognized `SB_F_CF` family. Gain: no change to the standby-like scenario. Cost: two titles for one fallback, and a reader must know the recognition rule to tell them apart. In practice: `SB_X_CF` looks recognized while warning in the report.
- Disposition: answered — the operator's settled interpretation for this change: the title applies to every `DutyEntry::UnrecognizedActivity`, timed or all-day. The three further scenarios are modified rather than left contradicting the rule.

### D3 — Does the ready-page change reach the failure state or the non-browser output?

- Question: The page derives both ready states and the failure state from the same result, and the executable prints the same warnings. Should the failure state's list and the executable's warning lines adopt the unknown-duty wording too?
- Options:
  1. **No: the ready states are the whole presentation change** — the failure state keeps row-number diagnostics because it explains why no calendar exists, and the executable's lines remain the report's entries. Gain: one canonical warning sentence for the report, the transcript and the failure list; nothing reworded twice. Cost: the page uses two vocabularies, one per state. In practice: a ready list reads `07-06-2099 08:00 UTC unknown duty: ZZ_M_CF`, a failure list still reads `row 1 (2099-06-07): unrecognized activity code …`.
  2. **Yes, reword everywhere** — one vocabulary on every surface. Gain: consistency. Cost: the warning report, its assertions and the executable's contract change without the operator asking, and a report reader loses the row number that locates the source row. In practice: the publisher's warning file changes text.
- Disposition: answered — the operator's settled interpretation for this change: the failure state, the warning report, the row diagnostics and the executable's warning lines stay as they are; only the ready states' presentation changes.
