# arc42 impact assessment — separate-position-marker-and-flight-number

Change: separate-position-marker-and-flight-number. Assessed against `proposal.md`, the delta under `specs/export-duty-plan-calendar/` and `design.md` (the travel value carrying the plan's two printed fields, one join rule for the title and for the failure message, pairing by the printed pair, the regenerated golden titles, and the reworded use-case rule).

| # | Change introduces or modifies | Affected | Reason / what changed |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | No | `01-introduction-and-goals.md` needs no edit: requirement 1 ("faithful conversion") already promises that a plan's activities are represented faithfully, and this change corrects the conversion towards that promise rather than changing it. No stakeholder, priority or quality goal moves. |
| 2 | Constraints (legal, tech, org) | No | `02-architecture-constraints.md` unchanged: no new dependency, tool, platform, licence or distribution constraint. The change edits two existing crates and the tests that drive them. |
| 3 | External systems, interfaces, context, new user-facing surface | No | `03-context-and-scope.md` unchanged: no external system, no new interface, no new surface. The calendar title is text the crew member already reads; only its spelling changes. |
| 4 | Major technical strategy or approach | No | `04-solution-strategy.md` unchanged: the strategy (pure domain behind ports, snapshot-scoped identity, no-clobber publication) is untouched. Its statement that a title is never identity stays true. |
| 5 | New building blocks, modules, components | Yes | `05-building-block-view.md`: the `domain` element's responsibility now names the one rule that reads a travel leg's two printed fields, and the `application` element's responsibility reads "classifies activities and pairs travel fragments by their two printed fields". No block, module or file is added, moved or removed. |
| 6 | New runtime flows, sequences | No | `06-runtime-view.md` unchanged: the successful-conversion, failure-with-diagnostic and publication flows keep their steps and participants. The change alters the text a step emits and which rows pair, not the sequence. |
| 7 | Deployment changes | No | `07-deployment-view.md` unchanged: no artifact, target, platform or release asset is added or altered; the executable is rebuilt as usual. |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | `08-crosscutting-concepts.md`: added "Printed identification of a travel leg" — the plan's two printed columns are kept apart, one pure rule reads them together for both the title and the failure message, and fragments pair only when both printed fields are equal (refusing rather than guessing). The existing concepts are unchanged. |
| 9 | Architecture decisions (tradeoffs, ADRs) | No | `09-architecture-decisions.md` unchanged: no ADR is minted. The one candidate — keeping the two printed fields apart, the single join rule, and the pairing key — fails the qualification gate and is routed to `design.md` (verdict below). |
| 10 | New quality scenarios | Yes | `10-quality-requirements.md`: added QS-013 (a travel title prints the crew position and the flight number as separate words, a leg without a printed position keeps the flight number alone, and a crew position printed beside a non-flight activity stays out of that activity's title), with no new node in the quality tree — it is a measurable case of Correctness / complete representation. |
| 11 | New risks or technical debt | Yes | `11-risks-and-technical-debt.md`: added TD-010 — fragments now pair only when both printed fields are equal, so a source variant that prints a crew position on one row of a split leg is refused where the previous suppression tolerated it; accepted deliberately (matching by flight number alone is ambiguous when a designator repeats) and pinned by a scenario. R-014 and TD-007 are neither widened nor closed. |
| 12 | New domain terms | Yes | `12-glossary.md`: added **Crew position** and **Flight number** with their code symbols, reworked **Travel leg**'s contract use, and corrected **Deadhead duty**'s contract use and code symbol (the deleted `Position` variant); the promotion boundary names the two terms and this change. |

## ADR qualification

One candidate was judged by the instruction's mandatory dimensions and falsification tests: the travel value keeps the plan's two printed fields apart with one join rule, rather than carrying one pre-joined string (the rejected alternatives being the pre-joined string with a renamed symbol, and splitting letters from digits inside a single cell).

| Field | Value |
|---|---|
| qualification | NOT_QUALIFIED |
| alternatives | PASS |
| architectural_impact | FAIL |
| durable_consequence | FAIL |
| lower_level_artifact | FAIL |
| existing_adr_check | PASS |
| must_be_shared | FAIL |
| routing | CANONICAL_ARTIFACT |
| canonical_artifact | `design.md` |
| target_adr | null |
| replacement_adr | null |

- **alternatives** — PASS: the design records a real choice between keeping one pre-joined string, keeping the two printed fields apart, and splitting letters from digits inside a single activity cell.
- **architectural_impact** — FAIL: no component, ownership, deployment or compatibility boundary moves; the change reshapes one value in the core and its two readers.
- **durable_consequence** — FAIL: it is fully implementable and reversible within this one change (rejoin the fields and re-render the fixture), so the single-change-reversibility falsification test applies.
- **lower_level_artifact** — FAIL: the complete rationale — the source columns, the pairing evidence and the two operator decisions — already lives in `design.md` ("The two printed fields and the one join rule"), so that home is not incomplete.
- **existing_adr_check** — PASS: the register was searched; ADR-0002 owns layer enforcement and covers none of this, and ADR-0001/0003/0004 own licensing and the release channel.
- **must_be_shared** — FAIL: no other change, contributor or team needs this decision before acting; the reworded use-case rules and the glossary carry what a reader of the domain needs.

Verdict: **NOT_QUALIFIED**, routed exactly once to `design.md`. No ADR file, chapter-09 register row or ADR-index entry was created.

## Status

STATUS: DONE
