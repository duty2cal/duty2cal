# Proposal: report-row-diagnostics

## Release note

Every row of a supplied duty plan is reported in one run: an activity code the converter does not recognize is converted from its printed times and station with a warning, a row that cannot be represented at all still fails the conversion, and both lists appear in the report file and on the conversion page.

## Use-Case Context

- Use-case ID: `export-duty-plan-calendar`
- Ideation topic: `duty-plan-calendar-conversion`
- Seed path: none
- Confirmed Personas: `crew-member--personal-calendar-owner`, `contributor--web-reviewer`

## Problem / Opportunity

One unfamiliar activity code defeats the whole conversion today. The converter reads a supplied duty plan row by row and stops at the first row it cannot classify, so a single code it has never seen hides every row after it and produces no calendar at all.

That is what happened to the operator's plan for November 2026: of its thirty dated rows, six carry an activity code the converter does not know. All six print station FRA and an explicit UTC start and end, so the times, the station and the code are all present in the source. Every other row is recognized. The conversion nevertheless fails on the first such row and publishes nothing.

The failure is also silent about the rest of the plan. Because the first error aborts the run, a crew member cannot tell whether the plan has one unfamiliar row or twenty, cannot see which dates are affected, and cannot judge whether the calendar would otherwise be complete. The report file carries a single reason, and the page carries the same single reason, so neither tells the whole story.

The opportunity is to treat an unfamiliar row the way the source actually reads: a row whose times and station are printed can still become a calendar entry, and the fact that its code is unfamiliar is evidence worth reporting rather than a reason to discard the whole plan.

## Proposed Solution

- **Every row is processed and every diagnostic is reported in one run.** The conversion no longer stops at the first row it cannot resolve. Each row that cannot be represented and each row that was represented by a fallback is collected, and all of them are reported together.
- **An activity code the converter does not know is converted from its printed evidence.** When the row supplies both a start and an end, the crew member receives a timed activity titled with the printed code, located at the printed station, using the printed UTC window. When the row supplies neither a start nor an end, the crew member receives an all-day activity titled with the printed code. Either outcome carries a warning.
- **An unfamiliar code that supplies only one endpoint is an error.** The row cannot be placed on the calendar without inventing an interval, and the source prints a time, so the converter does not claim the day is free.
- **A recognized code keeps exactly its current meaning.** Only an unexpected code warns. A row that carries a route remains travel, including a flight number. A date-only layout row stays silent: it produces no event and no warning. A row that prints neither an activity code nor a route is still an error.
- **Severity is explicit.** An error means the row cannot be represented: the conversion fails and no calendar is published. A warning means the row was represented by a documented fallback: the calendar is published and the warning is reported.
- **The report files separate the two severities.** A conversion with warnings publishes the calendar and a warning report named `<stem>.parse-warnings.txt`, numbered `<stem>.parse-warnings-1.txt` and so on when that name is taken. A conversion with errors publishes no calendar and a failure report named `<stem>.parse-error.txt`, numbered as today, which carries every error and every warning of the run in `Errors:` and `Warnings:` sections after the existing source and reason lines. A failure that is not about a row — an unreadable source, an unsupported plan, a publication failure, an unnameable source — keeps today's two-line report.
- **Both realizations show the diagnostics.** The executable prints every warning and every error line and names the report it wrote, and exits successfully when the calendar was published with warnings. The page lists the warnings in the ready state and lists the errors and the warnings in the failure state, so a crew member never has to open a file to see them. The page offers the warning report when the calendar is ready with warnings and the failure report on failure, named from the source stem.
- **Nothing is invented and nothing is silently dropped.** Every warning names the detail row it came from and its date when the row states one. The calendar text of every already-recognized activity is unchanged, and a plan without warnings publishes exactly the calendar and no report it publishes today.

## Capability Routing

`givn spec route --change report-row-diagnostics` reports no signal: the top candidates tie. The decision is therefore recorded here, and editing this table is the declaration `givn check` compares against the delta.

| Proposed capability | Decision | Rationale |
|---|---|---|
| Report every row's errors and warnings | `EXTEND export-duty-plan-calendar` | The behaviour changes who is represented and how a failure reads, inside the existing goal of exporting one supplied plan as a calendar. The browser is a second realization of the same action, not a second goal, so the delta belongs to `export-duty-plan-calendar`. |

## Out of Scope

- Deciding what any unfamiliar code means in the operator's roster system, expanding it, or mapping it onto a known activity kind. The printed code, station and printed UTC window are the whole representation.
- Reclassifying a known code, changing which codes are all-day or timed, or adding a new known code.
- Inventing a missing endpoint, repairing a contradictory date, or relaxing the rule that a timed interval must end after it starts.
- A single combined report file, a machine-readable report format, or a change to the two-line report used by failures that are not about a row.
- Any new capability, any new interaction, and any change to the browser's privacy rules: no network, no persistent storage, no new dependency, and the content security policy is unchanged.
- Reconciling a revised plan, importing the calendar, or judging duty/rest legality from an unfamiliar code.

## Open Questions

Every question below was settled by the operator in this change's brief before specs were written. None is carried or open; the design's Decisions ledger resolves each ID.

### D1 — What do an error and a warning mean, and which one still stops the calendar?

- Question: Should a row the converter cannot classify produce the same outcome as a row it represents by a fallback?
- Options:
  1. **Two severities, errors fatal** — an error means the row cannot be represented, so the conversion fails and no calendar is published; a warning means the row was represented by a documented fallback, so the calendar is published. Gain: one vocabulary matches the operator's own words and one calendar is never partial. Cost: two report shapes and two page states to keep distinct; in practice a known-only plan reports nothing and a plan with six unfamiliar-but-printable rows publishes a calendar plus six warnings.
  2. **One severity, everything fatal** — any unfamiliar row still fails the conversion. Gain: the smallest possible change. Cost: the operator's plan still converts to nothing, which is the reported problem; in practice six printable rows would discard the whole month.
  3. **One severity, everything a warning** — every row is reported but the calendar is always published. Gain: no failure path. Cost: a row that cannot be represented would vanish from a calendar that looks complete, which is the failure the project exists to prevent; in practice a row with one endpoint would silently become nothing.
  4. **`failure` / `notice` wording for the same two levels** — the same model under other names. Gain: a softer tone. Cost: the operator asked for "errors and warnings", and two names for one thing would have to be kept in step across reports, the page and the spec.
- Disposition: answered — the operator's brief settles option 1 and rejects the one-severity models; realized in design.md D1 and in the use case's severity rule.

### D2 — How is an unfamiliar activity code represented?

- Question: When a detail row carries an activity code outside the known sets and no route, what should the converter do with the printed evidence?
- Options:
  1. **Convert from the printed evidence** — both endpoints become a timed activity titled by the printed code at the printed station; neither endpoint becomes an all-day activity titled by the printed code; exactly one endpoint is an error. Gain: the printed content is preserved and the uncertainty is reported. Cost: the converter asserts a timed or all-day kind it cannot verify from the code's meaning; in practice station FRA with 08:00–15:30 becomes a timed activity, and the same code with blank time cells becomes an all-day activity.
  2. **All unfamiliar codes become all-day** — Gain: no interval risk. Cost: printed times are discarded and a crew member would miss a duty window; in practice 08:00–15:30 would become a free day.
  3. **Reject every unfamiliar code** — Gain: no unverified representation. Cost: the reported problem stays; in practice the whole month still fails.
  4. **Trust the code's letters, for example treat `SB...` as standby** — Gain: a plausible known shape. Cost: it invents semantics the source does not state and would silently change behavior for any future code sharing a prefix; in practice `SBX` is a known code and `SB_M_CF` is not, and a prefix rule would conflate them.
- Disposition: answered — the operator's brief settles option 1; realized in design.md D2 and pinned by the delta's timed, all-day and one-endpoint scenarios.

### D3 — Which rows stay silent, and which bare row stays an error?

- Question: Should the layout rows that carry only a date, and the rows that carry only a station, change behaviour?
- Options:
  1. **Date-only rows stay silent; station-only rows stay errors** — a date with no code and no route produces nothing and no warning, because it is table layout; a row that prints a station but neither an activity code nor a route is still a conversion error. Gain: no warning noise for layout, no invented activity for an unclassifiable row. Cost: two bare-row shapes must be distinguished in one place; in practice the reference plan's eight date-only rows stay silent while an unknown station with no code fails.
  2. **Warn on date-only rows too** — Gain: every row is visibly accounted for. Cost: layout rows would flood every plan with warnings the crew member can do nothing about; in practice the reference plan alone would gain eight warnings.
  3. **Treat a station-only row as an all-day activity** — Gain: no failure. Cost: the station alone does not state a day off, and the converter would invent an absence.
- Disposition: answered — the operator's brief settles option 1; realized in design.md D3 and pinned by the "date-only row is not turned into an activity" and station-only scenarios.

### D4 — Do recognized codes change?

- Question: Does the new fallback alter any code the converter already knows, or any row that carries a route?
- Options:
  1. **Known codes unchanged; a route still means travel** — FREE/OFF/U/FZ, SB/RE/SBX, TK/DTUSA/EM and Layover keep exactly today's rules, and a row with From and To and both endpoints remains travel whatever its code, including a flight number. Gain: no regression risk, and no warning for an ordinary flight. Cost: none identified; in practice flight LU742 stays a silent travel leg and SB stays timed standby.
  2. **Warn on every travel row too** — Gain: maximal reporting. Cost: every normal flight would carry a warning, which makes warnings meaningless.
  3. **Re-derive known codes from the fallback rule** — Gain: one rule. Cost: it would change known semantics and could alter calendar bytes for existing plans.
- Disposition: answered — the operator's brief settles option 1; realized in design.md D4 and pinned by the unchanged golden calendar and the existing known-code scenarios.

### D5 — What files are written, under which names, and what does each report contain?

- Question: How should a run with warnings and a run with errors leave evidence beside the source?
- Options:
  1. **One report kind per outcome, mirroring today's naming** — warnings publish `<stem>.parse-warnings.txt` (numbered on collision) beside the calendar; errors publish no calendar and `<stem>.parse-error.txt` (numbered as today) carrying every error and warning in `Errors:` and `Warnings:` sections after `Source PDF:` and `Reason:`. Gain: the file name states the outcome and the failure report tells the whole run. Cost: two names and two builders to keep consistent; in practice `roster.parse-warnings-1.txt` is the second warning report beside `roster.parse-warnings.txt`.
  2. **One combined `<stem>.report.txt` for both outcomes** — Gain: one name. Cost: a reader cannot tell a success from a failure by the file name, and every existing `parse-error` expectation changes; in practice a published calendar and a report would sit beside each other with no severity in the name.
  3. **Warnings only on the terminal and page, never a file** — Gain: fewer files. Cost: the operator asked for a report file with the warnings, and a crew member could not keep the evidence; in practice closing the page would lose the list.
  4. **`<stem>.warnings.txt` instead of `<stem>.parse-warnings.txt`** — Gain: shorter. Cost: it breaks the naming family that `<stem>.parse-error.txt` established, so a reader would have to remember two unrelated suffixes.
- Disposition: answered — the operator's brief settles option 1; realized in design.md D5 with the exact report text, and pinned by the warning-report, collision and severity-section scenarios.

### D6 — Where are the diagnostics shown?

- Question: Which surfaces show the errors and warnings, and how does a crew member obtain the reports?
- Options:
  1. **Executable output and page, plus the report files** — the executable prints every error and warning line and names the report path; the page lists warnings in the ready state and errors plus warnings in the failure state, and offers the matching report. Gain: no file needs to be opened to understand a run. Cost: two realizations to keep identical in meaning, and a new page state; in practice a browser user sees six warning lines and can save the calendar and the warning report.
  2. **Report files only** — Gain: one surface. Cost: the operator's fourth requirement is not met; in practice a browser user has to download and open a text file.
  3. **Page only** — Gain: nothing to open. Cost: the executable's user loses the evidence and the report requirement is unmet.
- Disposition: answered — the operator's brief settles option 1; realized in design.md D6 and pinned by the CLI-output, ready-with-warnings and failure-list scenarios.

### D7 — Does the calendar output change for activities that already convert?

- Question: May the new representation alter the calendar bytes of a recognized activity?
- Options:
  1. **Byte-identical for recognized activities** — the new timed and all-day fallbacks are ordinary timed and all-day events, component identity stays snapshot plus ordinal, and the reference calendar is not regenerated. Gain: existing users' calendars do not change and the golden test keeps its meaning. Cost: the fallback must fit the existing event shapes rather than a bespoke one; in practice the reference calendar stays byte-identical.
  2. **A new event shape for fallback activities** — Gain: the fallback is visibly special in the calendar. Cost: it invents a presentation the operator did not ask for and risks changing shared rendering; in practice a calendar client would show a new title convention.
- Disposition: answered — the operator's brief settles option 1; realized in design.md D7 and pinned by the unchanged golden calendar scenario.

### D8 — How does the permanent domain contract change?

- Question: The use case currently says an unknown activity code produces failure rather than an apparently complete partial calendar. What replaces that rule?
- Options:
  1. **Replace the rule** — an unrecognized activity code is converted using its printed evidence and reported as a warning; only a row that cannot be represented at all is an error and prevents publication. The owning use case's rules, extensions, guarantees and examples are updated and the replaced rule is recorded. Gain: spec and behaviour agree and the change is auditable. Cost: a substantive use-case edit beyond the delta; in practice the old sentence is named as replaced.
  2. **Leave the use case and change only the delta** — Gain: a smaller edit. Cost: the permanent contract would contradict the executable spec, and archive would merge a delta against a rule it denies.
- Disposition: answered — the operator's brief settles option 1; realized in `givn/specs/export-duty-plan-calendar/usecase.md` and recorded in design.md D8.

### D9 — Is a new capability created?

- Question: Should the diagnostics behaviour live in a new capability rather than the existing conversion capability?
- Options:
  1. **Extend `export-duty-plan-calendar`** — Gain: one goal, one inventory, one delta file. Cost: that capability's rules grow; in practice the delta file already holds failure behaviour.
  2. **A new capability for reporting** — Gain: a named home for diagnostics. Cost: `givn spec route` reports no signal, the behaviour is the same actor goal, and a second capability would split one action across two specs.
- Disposition: answered — `givn spec route --change report-row-diagnostics` reports no signal (top candidates tie), and the operator's brief settles option 1; the decision is recorded in the Capability Routing table and in design.md D9.

### D10 — Do the interaction-inventory actions change?

- Question: Does a new consumer action appear, requiring a new end-to-end interaction?
- Options:
  1. **Keep the two actions; add regular browser scenarios** — supplying a plan to the executable and supplying a plan through the page stay the only inventory entries, and warning/error visibility is regular coverage of them. Gain: no new `@e2e` action and no duplicate end-to-end evidence. Cost: the new page state is asserted by regular `@browser` scenarios rather than the one `@e2e` scenario.
  2. **Add a "read the warnings" interaction** — Gain: an explicit entry. Cost: reading a rendered list is not a separate consumer action with its own state change; it would over-cover one action with two `@e2e` scenarios, which the interaction policy treats as a defect.
- Disposition: answered — the operator's brief settles option 1; realized in design.md D10 and the unchanged Interaction Coverage Matrix.
