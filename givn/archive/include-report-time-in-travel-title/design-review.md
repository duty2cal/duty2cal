# Design review: include-report-time-in-travel-title

Grilling phase only (fresh context; no artifact was authored here). Read in full:
`givn instructions design-review`, `specs`, `arc42-docs`, `review` and
`domain-modeling`; `proposal.md`, the delta feature, `design.md`, the owning
`givn/specs/export-duty-plan-calendar/usecase.md` and its permanent feature; the
step definitions and fixture support; `crates/duty2cal-core/src/domain/mod.rs`,
`application/mod.rs`, `adapters/ics/mod.rs`; the golden fixture; all twelve
arc42 chapters and the ADR register. Ran `givn status`, `givn lint --change
include-report-time-in-travel-title`, the three delta scenarios by name, and the
golden-oracle scenario. No production code, spec, design or use case was
modified.

## Grilling findings

Ranked by implementation risk. Every finding is answerable from the repository
or from the operator decisions already settled in the proposal; none requires a
new user decision.

### F1 (High) — The "title unchanged" assertions are substring checks, so the `Differing` guard cannot catch the most likely wrong implementation

**Question.** Do the delta's Then-steps assert the *exact* title, so that an
implementation which appends the departure (or arrival) report value for
`FieldValue::Differing` fails a scenario?

**Plan says.** Scenario 3 asserts `that activity's title is "LU742 ABC-DEF"`;
scenario 2 asserts the same for an unsupplied report. `design.md` says
`Differing` and `None` append no suffix, and reuses the existing title step
unchanged.

**Verified.** `apps/cli/tests/steps/export_duty_plan_calendar.rs:616-621`
implements the reused step as
`calendar.contains(&format!("SUMMARY:{title}"))` — a substring check. A wrong
implementation that appends the departure value emits
`SUMMARY:LU742 ABC-DEF Report 07:30 LT`, which still *contains*
`SUMMARY:LU742 ABC-DEF`, so scenario 3 passes. No other scenario closes the
hole: the permanent "Differing optional values are both retained with their
fragment roles" asserts only the description, the golden reference plan supplies
no report cells at all, and the `@e2e` fixture supplies none either. The
`None` path is indirectly protected because an unconditional suffix would break
the golden's exact whole-file comparison (`apps/cli/tests/steps/
export_duty_plan_calendar.rs:2111-2125`), but scenario 2's own assertion is
still weak. This is exactly the testability failure the design-review
instruction names: the guard cannot fail for the reason it exists.

**Recommended answer.** Tighten the shared `that activity's title is` step to
an exact `SUMMARY` line match (unfold continuations, strip the trailing `\r`,
require one line equal to `SUMMARY:<title>`), or add a dedicated exact-title
step used by the three delta scenarios. With an exact step, a `Differing`
suffix becomes a RED failure and scenario 2 asserts the invariant it claims.
If the shared step is tightened, add `apps/cli/tests/steps/
export_duty_plan_calendar.rs` to `design.md`'s changed-files table and correct
the "steps unchanged / no new step definition" statements.

**Requires user:** No.

### F2 (High) — The delta does not pin the clause "a single fragment, or equal values on both fragments"

**Question.** Does the spec pin every case of the title rule the use case now
states — a whole-row value, one fragment only, equal values on both fragments,
no value, and differing values?

**Plan says.** The proposal: "One supplied report time produces one suffix. A
value supplied equally by the departure and arrival fragments of one leg also
produces one suffix." The added use-case rule says "(a single fragment, or
equal values on both fragments)". `design.md` says `combine()` collapses equal
values to `Single`, that the no-report scenario pins "single-value scope", and
that no new step definition is introduced.

**Verified.** `combine()` (`crates/duty2cal-core/src/application/mod.rs:393-401`)
implements the one-fragment and equal-value branches, and `summary()` does not
branch on the origin of `Single`, so the production intent is sound. The delta
covers only whole-row `Single` (scenario 1), `None` (scenario 2) and `Differing`
(scenario 3). No scenario exercises a split leg with a report value on one
fragment only, or an equal value on both fragments; no registered step builds
either fixture (the only report-fragment step is the hard-coded differing
given at `export_duty_plan_calendar.rs:1244`). The Decisions ledger's claim that
the no-report scenario pins the equal-value case is therefore false: that
scenario pins `None`, not `combine()`. Pinning the missing clause requires a new
`Given` step, so the design's "adds no new step definition" and changed-files
table are also incomplete.

**Recommended answer.** Add one `@givn.added @wip` scenario (or a small outline)
for a split leg whose two fragments supply the same report value, asserting the
exact title with one suffix; if an outline is used, include a one-fragment row
so both `combine()` shapes are exercised. Register the supporting `Given` step,
and update `design.md`'s step-location, changed-files and Decisions sections.
If the team deliberately leaves the combine path unpinned, record that decision
and its reason in `design.md` and remove the claim that the no-report scenario
pins it.

**Requires user:** No. The observable behavior is already settled; this is
spec completeness.

### F3 (Blocking) — The arc42-docs artifact is absent, and the design's chapter list omits chapter 12

**Question.** Is the arc42 assessment complete and consistent with the change?

**Plan says.** `design.md` names chapters 5, 8 and 10 as affected, "recorded by
the separate arc42 artifact". No `arc42.md` exists in the change directory.

**Verified.** `givn status --change include-report-time-in-travel-title` reports
stage `design-done` with `arc42-docs` as the next required artifact and
`design-review` after it. `givn/changes/include-report-time-in-travel-title/
arc42.md` is absent, and `git status` shows no `docs/arc42/` edit. An
independent 12-row derivation (below) gives **Yes** on rows 5, 8, 10 **and 12**.
Row 12 is material: the glossary's **Travel leg** entry says the title is
"printed crew position and activity, then the route", which the new suffix makes
stale. All twelve chapter files exist and carry real content (38–273 lines); a
scan for box-drawing characters and pseudo-diagram blocks found none (markdown
table separators excluded). No chapter-09 MADR is required (ADR verdict below),
so the register stays unchanged.

**Recommended answer.** Produce the arc42 artifact: write `arc42.md` with the
full 12-row table, marking rows 5, 8, 10 and 12 Yes with what changed and row 11
No with the reason that the differing-value omission is specified behaviour, not
technical debt. Update chapters 05 (domain responsibility now composes the title
suffix), 08 ("Printed identification of a travel leg" must state the suffix and
its omission), 10 (extend QS-013's printed-fidelity expectation) and 12 (see
F4). Record the NOT_QUALIFIED ADR verdict. Design-review cannot sign off while
the artifact is absent; the graph already orders it before this review.

**Requires user:** No.

### F4 (High) — The ubiquitous language is stale for `Travel leg` and unrecorded for the report time

**Question.** Does the durable glossary hold the terms this change uses in a
contract role?

**Plan says.** The use case and design use "report time" and the title suffix;
`design.md` names no chapter-12 change.

**Verified.** `docs/arc42/12-glossary.md:75` defines **Travel leg** as "One
timed travel activity titled by the printed crew position and activity, then
the route" — the title now also ends with ` Report <value> LT` when one report
time is supplied, so the glossary's contract use is stale. The glossary has no
**Report time** term, and line 165 explicitly says raw source-column labels
(including `Report (LT)`) "are not confirmed domain terms". The added use-case
rule gives the report time a contract use in the title, so leaving it unglossed
means the spec and the glossary disagree about what is domain language.

**Recommended answer.** Refine **Travel leg**'s contract use to include the
conditional suffix, and record **Report time** (definition: the local briefing
time the plan prints in its Report column; contrast: not an activity instant,
offset, or accounting duration; code symbol `DutyEntry::Travel.report_lt`). If
the team prefers not to promote the raw label, extend the **Travel leg** entry
with the suffix rule and keep the raw-label note, but do not leave the new
contract use unglossed. Update the promotion-boundary paragraph to name this
change.

**Requires user:** No. The name is already fixed by the settled wording
("report column", `Report <value> LT`); this is a glossary-maintenance decision,
not an observable-behaviour tradeoff.

### F5 (Medium) — Only scenario 1 is RED-first; scenarios 2 and 3 already pass, contrary to the design's blanket claim

**Question.** Can each scenario fail RED for the intended reason with production
code unchanged?

**Plan says.** "Because every step resolves, the RED phase fails on the
assertion rather than on an undefined step", implying all three fail before
implementation.

**Verified.** `./run-tests.sh --name "report time"` (all three delta scenarios)
reported **3 scenarios (2 passed, 1 failed)**. Scenario 1 failed exactly on
`Then that activity's title is "LU742 ABC-DEF Report 07:30 LT"`; the actual
calendar carried `SUMMARY:LU742 ABC-DEF` and the description carried
`Report (LT) 07:30`. Scenario 2 (`None`) and scenario 3 (`Differing`) passed
with production code unchanged — they are regression guards, not RED-first
behaviour. All ten steps resolved (9 passed, 1 failed); no undefined step.
Scenario 3 nonetheless becomes a meaningful guard once F1 is fixed, because it
then fails a wrong `Differing` suffix; it still passes pre-change.

**Recommended answer.** Record in `tasks.md` that scenario 1 is the RED-first
behaviour and scenarios 2 and 3 are regression guards (the archived
`separate-position-marker-and-flight-number` design-review R3 is the precedent).
Narrow the blanket RED claim in `design.md` accordingly; keep the strict-mode
mechanism (`fail_on_skipped()`), the no-stub statement and the `--name` command,
which are all accurate.

**Requires user:** No.

### F6 (Low) — The Visual Design Contract names an evidence path that nothing writes

**Question.** Does the contract point at the real terminal evidence?

**Plan says.** `Interface: terminal`; no new `@e2e` scenario, so no new
transcript is owed; the capture path is
`givn/changes/include-report-time-in-travel-title/evidence/visual/
<scenario-slug>/transcript.txt`.

**Verified.** The committed transcript path in the e2e steps is
`givn/archive/export-released-roster-calendar/evidence/visual/
crew-member-converts-a-supported-duty-plan/transcript.txt`
(`apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs:14-20`). The change-dir
evidence directory does not exist and no step writes it. The change adds no
`@e2e` scenario, so the design's conclusion (no new evidence owed) is correct;
only the named path is wrong.

**Recommended answer.** Correct the contract to name the archived transcript, or
state that the existing terminal evidence is unchanged and remove the
unwritten change-dir path. Do not add or remove any `@e2e` tag.

**Requires user:** No.

### F7 (Low) — `givn lint` emits 14 advisory lines beyond the three `@wip` findings

**Question.** Are there unexpected lint findings?

**Plan says.** The delta is expected to be lint-clean apart from the `@wip`
warnings; no step changes.

**Verified.** `givn lint --change include-report-time-in-travel-title` exits 2
with **3 `[WIP]` findings** (the expected three) and **14 advisory lines: 7
`[SHAPE]` and 7 `[SUBST]`**. They pair the delta title scenarios with each
other and with permanent title scenarios (for example, scenario 1 is reported
as a shape-subset of "Deadhead positioning travel keeps one event and marks the
assignment in its title"; scenarios 1 and 2 are reported as subsets of scenario
3). They are advisories, not findings, and they reflect the intentional
one-observable-per-scenario split. They still require review-time
`## Overlap dispositions` rows.

**Recommended answer.** Keep the three `@wip` warnings as expected. Carry the
14 advisories into `review.md` and disposition each pair as `variant` or
`boundary` (the DH title, no-position title, non-flight title, `None`-report
title and `Differing`-report title are distinct observables). Re-run lint after
the F1/F2 step changes and re-check.

**Requires user:** No.

### F8 (Optional, Low) — The use-case Examples section has no title-suffix example

**Question.** Should the permanent use case illustrate the new title?

**Plan says.** Only the Rules block is edited; the Examples section is
unchanged.

**Verified.** The use-case diff is exactly **+1 line** (git diff: 1 insertion,
0 deletions); the inserted rule is quoted in "Use-case edit" below. It sits
next to the other title rule and does not contradict "Timed event instants come
from the printed UTC time evidence … not Report (LT)" because the suffix is
literal text and no timing code reads it. The Examples section's "Differing
optional fragment values" example does not mention the title, and no example
shows the suffix.

**Recommended answer.** Optional: add one generated example
(`… ABC-DEF Report 07:30 LT`) for parity with the other title examples. The rule
is authoritative and the delta scenarios are the executable evidence; this is a
documentation nicety, not a sign-off condition.

**Requires user:** No.

## Use-case edit

The use-case change is one inserted rule in "Timing and context", next to the
existing printed-identification title rule, plus one generated example added
after the deadhead example (advisory F8):

> - A travel activity's title ends with the plan's report time in local time, written ` Report <value> LT`, when the source supplies one report time (a single fragment, or equal values on both fragments). The description keeps the fully labelled Report (LT) value. No report time supplies no suffix; differing report values on the two fragments also supply no suffix because there is no single briefing time, while the description retains both labelled values.

Assessment: surgical and correct. It states the same suffix shape as the
proposal and scenario 1, preserves the description contract and the
descriptive-not-temporal rule, and names the combine cases that F2 shows the
delta does not yet pin. It introduces the vocabulary that F4 requires the
glossary to record.

## Finding dispositions (applied hardening)

Every finding was answered in the artifacts; no user decision was needed. This
section records the disposition, so the findings above are historical evidence,
not open defects.

| Finding | Disposition | Where it now lives |
|---|---|---|
| **F1** substring title check | **Fixed.** `design.md` requires the shared `that activity's title is "..."` step to become an exact `SUMMARY` line match (`calendar.lines()`, trailing `\r` and folded continuation trimmed, exactly one matching line). The step file is listed as changed. Test code was not edited in this phase. | `design.md` "Step definition locations"; changed-files table |
| **F2** equal-value combine path unpinned | **Fixed.** A fourth scenario, "Equal report times on both fragments produce one title suffix", uses a new equal-value `Given` and asserts one suffix. The Decisions ledger no longer claims the no-report scenario pins the combine path; a separate equal-fragment decision does. | delta spec; `design.md` Decisions and Use-case traceability |
| **F3** chapter 12 omitted / artifact ownership | **Recorded.** The design's downstream list is Yes on 5, 8, 10, and 12; No on 1–4, 6, 7, 9, and 11. `givn/changes/include-report-time-in-travel-title/arc42.md` is written by the arc42-docs artifact, not by this review. | `design.md` Architecture impact; arc42 derivation below |
| **F4** stale glossary | **Recorded, not edited.** **Travel leg** now names the conditional report-time suffix; a **Report time** term is added (printed local briefing time, descriptive only, `Report (LT)` in the description and ` Report <value> LT` in the title, never an instant or activity date). The arc42-docs artifact applies it. | `design.md` Architecture impact |
| **F5** RED-first claim overstated | **Fixed.** Scenarios 2, 3, and 4 are recorded as guards, each with its own new `Given` wording so the pre-implementation state is an undefined step; scenario 1 remains the RED-first suffix assertion. The blanket "every step resolves" claim is removed. | `design.md` Step definition locations / RED mechanics |
| **F6** unwritten evidence path | **Fixed.** The Visual Design Contract now names the durable archived transcript `givn/archive/export-released-roster-calendar/evidence/visual/crew-member-converts-a-supported-duty-plan/transcript.txt` and states that no new transcript is owed. | `design.md` Visual Design Contract |
| **F7** lint advisories | **Disposition recorded.** The `[WIP]` findings are expected until implementation (one per scenario). The `[SHAPE]` and `[SUBST]` advisories must be reviewed at implementation time for overlap with the permanent scenarios and are accepted only if they add no duplicate behaviour; the title variants (DH, no position, non-flight, no-report, differing-report, equal-report) are distinct observables. No code change now. | This file, "Lint advisory disposition" |
| **F8** no title-suffix example | **Fixed.** One generated example was added to the permanent use case: a travel leg with Report (LT) 07:30 yields the suffix `Report 07:30 LT`, and the value stays descriptive without moving the UTC interval. | `givn/specs/export-duty-plan-calendar/usecase.md` Examples |

### Lint advisory disposition

`givn lint --change include-report-time-in-travel-title` exits 2 with exactly
**4 findings**, all `[WIP]`, one per delta scenario. That is the expected
pre-implementation state and is the only finding class. The 12 `[SHAPE]` and 9
`[SUBST]` advisories pair the four scenarios with each other and with the
permanent title scenarios; they reflect the intentional one-observable-per-scenario
split. They are accepted provisionally and must be re-checked at implementation
time: a pair is accepted only if it adds no duplicate behaviour, otherwise the
scenarios must be merged. No spec or code change is made for the advisories now.

## Branch walk

The rows below record the assessment at review time. Every defect they name
(F1, F2, F5, F6, F8) has since been fixed in the artifacts, and F3/F4 are
recorded for the arc42-docs artifact, as the dispositions above state.

| Required branch | Disposition |
|---|---|
| Scope | No issue beyond **F2**. The settled behaviour is exactly what was asked: suffix ` Report <value> LT`, one space before `Report`, value verbatim, after the route; differing values omit it; no value leaves the title unchanged. No extra behaviour is asserted. |
| Tech choices | No issue found. `DutyEntry::summary()` is the single title composition point; `adapters/ics/mod.rs:44` writes `SUMMARY` verbatim; no dependency, module, driver or version changes. The alternative (format in the adapter) would create a second formatting rule and is correctly rejected in `design.md`. |
| Missing scenarios | **F2** for the combine `Single` path. Considered and rejected as input variation or already covered: position + report ordering (permanent DH/title scenarios plus scenario 1 prove the two joins), non-travel titles (permanent `EM`, layover, all-day and availability assertions), empty report cell (same `optional()` path as omission), error paths (the report value is optional free text; no failure path is introduced), interval/description-order/UID invariants (permanent scenarios using the same report fixtures; golden exact file). |
| Testability | **F1** and **F5**. Verified: strict mode is `fail_on_skipped()` (`apps/cli/tests/features_runner.rs`); no stub step is added; the single-scenario command is supported (`run-tests.sh` passes `--name` through, and the named run above executed exactly the three scenarios); every reused step exists and produces the stated fixture (`LU742`, route `ABC-DEF`, report `07:30`); the Then values are concrete titles. Only the substring mechanism (F1) and the pre-change GREEN status (F5) are defects. |
| E2E fidelity | No issue found. One inventory action, one `@e2e` scenario; the driver is the built binary via `std::process::Command`; no `@e2e` tag was added or removed. The e2e fixture supplies no Report cell and its asserted title `LU742 ABC-DEF` remains correct. The e2e assertion is also substring-based, but the golden's exact whole-file comparison protects the `None` path; no new e2e variant is owed for a single value variant. |
| Interaction Coverage | No issue found. `usecase.md` §Interactions has one entry ("supply a duty plan for conversion"); the design matrix has exactly that row, mapped to the existing `@e2e` title "Crew member converts a supported duty plan and receives a calendar file" and the real CLI mechanism. |
| Risk | The most likely failure is **F1**: a `Differing` suffix silently ships because every current scenario passes. The second is **F2**: the one-fragment/equal-value path regresses untested. Mitigations are the exact-title assertion and the added combine scenario. The golden oracle is a genuine exact check and, unlike the prior title change, needs no regeneration (the reference plan supplies no `Report` cells); verified by `./run-tests.sh --name "The reference roster converts to the expected calendar"` → 1 scenario passed. |
| Use-case and Persona context | No issue found. Identity and contract are preserved; the edit is one inserted rule (F8 quotes it); typed relationships remain none; Personas are confirmed review lenses and Gherkin uses the domain Actor "Crew member". Scope-rejection criteria re-run: no goal conjunction, primary actor present and named by the trigger, placement valid, mutation is a new export (not an observing goal), every extension names its step, preconditions consistent, no mechanism vocabulary in goal/trigger/guarantees, no component/channel name, leaf size fine, no Includes/Extends misuse. |
| Decision ledger | No issue found except the misstatement folded into **F2**. The proposal records no `D<n>`, so there is nothing unresolved; the four settled decisions are realized in the design and map to the scenarios (with F2's equal-value claim corrected). |
| Visual design contract | **F6**. The section exists with `Interface: terminal`, viewports n/a, motion n/a, no new screen, a baseline, and a real-interface justification. No `@e2e` tag or evidence is owed for this variant. |
| Deferrals | None. No mandatory check is deferred; no operator deferral decision is recorded or needed. |
| Persona disposition | No issue found. `crew-member--personal-calendar-owner` is the confirmed accuracy lens for the title change; `project-maintainer--release-owner` is confirmed and explicitly dispositioned as unaffected (no release artifact, dependency or public claim changes; golden byte-identical). No invented or promoted Persona appears. |
| Interface classification | No issue found. Terminal/CLI is correct: the product renders no HTML/CSS/JS; the `.ics` is consumed by an external calendar client outside the system boundary. The opposite classification (Web UI) does not hold, so no browser driver is required. |
| ADR qualification | No issue found. Candidate: compose the suffix in `DutyEntry::summary()` rather than in `adapters/ics`. Verdict: `NOT_QUALIFIED` — alternatives PASS; architectural impact FAIL (local formatting inside one crate, no boundary or contract); durable consequence FAIL (reversible in one change); lower-level artifact FAIL (`design.md` + code own the rationale); existing-ADR check PASS (ADR-0001..0006 searched; ADR-0002's layering model is untouched). Routing `CANONICAL_ARTIFACT` → `design.md`; no MADR, chapter 09 unchanged. |
| Architecture documentation (arc42) | **F3**. Independent derivation below; chapter files and diagram policy checked; no ADR entry required. |
| Ubiquitous language | **F4**. `Travel leg`'s contract use is stale; the report time has no glossary term despite its new title contract. |

## Independent arc42 chapter derivation

Derived from the proposal, delta, design and use case **before** comparing with
the design's claim. `arc42.md` does not exist yet; it is written by the
arc42-docs artifact, not by this review, and every row below is a required row
of that marker file. The design now carries the same selection: Yes on 5, 8, 10,
and 12; No on 1–4, 6, 7, 9, and 11.

| # | Change introduces or modifies | Yes/No | Reason |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | No | Requirement 1 already promises faithful conversion; no goal, stakeholder or priority moves. The measurable case is QS-013 (row 10). |
| 2 | Constraints | No | No new dependency, tool, platform, licence or distribution constraint; one function changes. |
| 3 | External systems, interfaces, context, new user-facing surface | No | No external system or interface; the calendar title already exists. The `.ics` consumer is outside the boundary. |
| 4 | Major technical strategy | No | The pure-domain/ports strategy is unchanged; the suffix is computed in-process before serialization. |
| 5 | New building blocks, modules, components | **Yes** | No new block, but the `domain` element's responsibility now composes the title with the report suffix (`05-building-block-view.md:106-110`). |
| 6 | New runtime flows, sequences | No | The successful-conversion, failure and publication flows keep their participants and steps; only emitted text changes. |
| 7 | Deployment changes | No | No artifact, target, platform or release asset changes. |
| 8 | Cross-cutting concepts | **Yes** | `08-crosscutting-concepts.md:27-29` ("Printed identification of a travel leg") describes the title rule and must state the suffix and its omission. |
| 9 | Architecture decisions | No | The only candidate fails the gate and is routed to `design.md`; no ADR, register unchanged. |
| 10 | New quality scenarios | **Yes** | QS-013 (Correctness — printed fidelity, `10-quality-requirements.md:53`) extends to the report suffix: one suffix for one/equal value, none for absent/differing. |
| 11 | New risks or technical debt | No | No new risk or debt. The differing-value title omission is specified behaviour with the description retaining both values; if the author chooses to record it as an accepted usability limitation, chapter 11 is the correct home, but it is not required. |
| 12 | New domain terms | **Yes** | Refine **Travel leg**'s contract use (`12-glossary.md:75`) and record **Report time** / the suffix rule; update the promotion-boundary paragraph. |

## Verification evidence

| Check | Result |
|---|---|
| `givn status --change include-report-time-in-travel-title` | at review time: stage `design-done`; `arc42-docs` next required; design-review was pending. The arc42 artifact remains owned by the arc42-docs stage; this review does not write it. |
| `givn lint --change include-report-time-in-travel-title` (before hardening) | exit 2: 3 `[WIP]` findings (the three delta scenarios) + 14 advisories (7 `[SHAPE]`, 7 `[SUBST]`). |
| `givn lint --change include-report-time-in-travel-title` (final) | exit 2: **4 findings, all `[WIP]`** (one per delta scenario) + 21 advisories (12 `[SHAPE]`, 9 `[SUBST]`). No other finding class. |
| `./run-tests.sh --name "report time"` (before hardening) | 3 scenarios: **2 passed, 1 failed**; 10 steps: 9 passed, 1 failed. Scenario 1 failed on the suffix; actual `SUMMARY:LU742 ABC-DEF`. Scenarios 2 and 3 passed. No undefined steps. |
| RED mechanics after hardening (design-level, no test edit) | Scenario 1 keeps its existing Given and fails the strengthened suffix assertion. Scenarios 2–4 use the three new `Given` wordings, which do not exist yet, so strict mode makes them fail as undefined steps until implementation defines them. |
| `./run-tests.sh --name "The reference roster converts to the expected calendar"` | 1 scenario passed, 3 steps passed; expected `2098-06.ics` has **0** occurrences of `Report`, and the reference-plan builder never writes a `REPORT_X` cell. |
| Golden / e2e fixture inspection | `apps/cli/tests/fixtures/expected/2098-06.ics` has 26 `SUMMARY` lines, all unsuffixed; the e2e fixture supplies date/activity/route/endpoints only, so `SUMMARY:LU742 ABC-DEF` stays correct. |
| Step existence | Scenario 1 Given exists at `export_duty_plan_calendar.rs:1191` (activity `LU742`, `ABC`→`DEF`, report `07:30`); the description step used by scenario 3 exists at `:1270`; the title Then exists at `:616` and is to be strengthened. The three new Givens for scenarios 2–4 do not exist yet and are added by the implementation task, which is what makes those guards RED. |
| Code paths | `combine()` at `application/mod.rs:393-401`; `report_lt` typing at `application/mod.rs:360,424`; `summary()` at `domain/mod.rs:202-217`; description `push_value` at `domain/mod.rs:54-64,219-241`; ICS `SUMMARY` at `adapters/ics/mod.rs:44`. |
| Arc42 integrity | all 12 chapter files exist (38–273 lines); no box-drawing or pseudo-diagram blocks; chapter 09 is a register with ADR-0001..0006. |

No question in this review genuinely needs the user. The two operator decisions
that matter — the exact suffix shape and the no-suffix rule for differing report
values — are already settled in the proposal and are not reopened here.

DESIGN-REVIEW: PASS

All fourteen required branches were walked. The design names the exact changed
files, the strengthened title `Then` step, the three new `Given` steps, and the
arc42 chapter set Yes on 5, 8, 10, 12 / No on 1–4, 6, 7, 9, 11. The delta has
four scenarios covering one value, no value, differing values with both
description values retained, and equal values with exactly one suffix. The only
lint findings are the four expected `[WIP]` entries; the `[SHAPE]`/`[SUBST]`
advisories carry an implementation-time disposition and no duplicate behaviour
is claimed. No user decision is outstanding.
