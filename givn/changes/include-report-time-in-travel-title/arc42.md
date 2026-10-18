# arc42 impact assessment — include-report-time-in-travel-title

Change: include-report-time-in-travel-title. Assessed against `proposal.md`, the four-scenario delta under `specs/export-duty-plan-calendar/` and `design.md` (the travel title's existing printed identification and route gaining a conditional ` Report <value> LT` suffix when one report value is supplied, `FieldValue::Single` appending it while `None` and `Differing` append nothing, and the description keeping its labelled `Report (LT)` values).

| # | Change introduces or modifies | Affected | Reason / what changed |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | No | `01-introduction-and-goals.md` needs no edit: requirement 1 ("faithful conversion") already promises that a plan's activities are represented faithfully, and this change makes the briefing time visible towards that promise rather than changing it. No goal, stakeholder or priority moves; the measurable case is QS-021 (row 10). |
| 2 | Constraints (legal, tech, org) | No | `02-architecture-constraints.md` unchanged: no new dependency, tool, platform, licence or distribution constraint. The change edits one function in the core's domain and the tests that drive it. |
| 3 | External systems, interfaces, context, new user-facing surface | No | `03-context-and-scope.md` unchanged: no external system, no new interface, no new surface. The calendar title is text the crew member already reads; the suffix adds a value the event details already carried. The `.ics` consumer stays outside the boundary. |
| 4 | Major technical strategy or approach | No | `04-solution-strategy.md` unchanged: the pure domain behind ports, snapshot-scoped identity and no-clobber publication are untouched. The suffix is composed in-process by the existing single title rule before serialization; no second formatting path is introduced. |
| 5 | New building blocks, modules, components | Yes | `05-building-block-view.md`: the `domain` element's responsibility now says it composes the travel title, including the conditional ` Report <value> LT` report suffix. No block, module, crate or file is added, moved or removed. |
| 6 | New runtime flows, sequences | No | `06-runtime-view.md` unchanged: the successful-conversion, failure-with-diagnostic and publication flows keep their participants and steps. The change alters the text one step emits, not the sequence. |
| 7 | Deployment changes | No | `07-deployment-view.md` unchanged: no artifact, target, platform or release asset is added or altered; the executable is rebuilt as usual and the golden calendar stays byte-identical. |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | `08-crosscutting-concepts.md`: extended "Printed identification of a travel leg" so the printed-identification/route title and the conditional report suffix are described together — one space before `Report<value> LT`, `LT` as literal local-time text that never becomes an instant, omission for no value and for differing fragment values while the description keeps both labelled values, equal values collapsing to one suffix, and other activity kinds unchanged. |
| 9 | Architecture decisions (tradeoffs, ADRs) | No | `09-architecture-decisions.md` unchanged: no ADR is minted. The one candidate — composing the suffix in the domain title rule rather than in the ICS adapter — fails the qualification gate and is routed to `design.md` (verdict below). |
| 10 | New quality scenarios | Yes | `10-quality-requirements.md`: added QS-021 (a supported travel leg supplies report time 07:30, so the published title ends with ` Report 07:30 LT`; a leg with no report time keeps its previous title, and differing fragment values leave the title without a suffix while the description keeps both labelled values). No new node in the quality tree; it is a measurable case of Correctness / complete representation. |
| 11 | New risks or technical debt | No | `11-risks-and-technical-debt.md` unchanged: the omission of a suffix for differing report values is specified behaviour, not technical debt, and the description still carries both fragment-labelled values. No new risk is introduced; the title remains non-identity, and the unchanged golden oracle covers the no-report path. |
| 12 | New domain terms | Yes | `12-glossary.md`: refined **Travel leg**'s contract use to name the conditional report suffix, added **Report time** with its code symbols and non-meanings, named this change in the promotion boundary, and removed `Report (LT)` from the unconfirmed raw-label list because that label now names a confirmed term. |

## ADR qualification

One candidate was judged by the instruction's mandatory dimensions and falsification tests: compose the report suffix in `DutyEntry::summary()` rather than in the ICS adapter (the rejected alternative being a second formatting rule in `adapters/ics`, and the weaker alternative of deriving the title text in `apps/cli`).

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

- **alternatives** — PASS: `design.md` records a real choice between composing the suffix in the existing domain title rule and formatting it in the ICS adapter (or in the CLI), and rejects the second because it would create a second formatting rule.
- **architectural_impact** — FAIL: no component, ownership, dependency, deployment or compatibility boundary moves; the choice is local formatting inside one crate and one function.
- **durable_consequence** — FAIL: it is fully implementable and reversible within this one change (move the suffix between two functions), so the single-change-reversibility falsification test applies.
- **lower_level_artifact** — FAIL: the complete rationale — one composition point, the drift argument, the differing-value omission — already lives in `design.md` ("The title composition rule", "Why the rule lives in `summary()`"), so that home is not incomplete.
- **existing_adr_check** — PASS: the register was searched; ADR-0001 owns licensing, ADR-0002 the hexagonal layer model (untouched here), ADR-0003/0004 the release channel and immutable publication, and ADR-0005/0006 the public home's declaration and state. None owns title composition or any boundary this change moves.
- **must_be_shared** — FAIL: no other change, contributor or team needs this decision before acting; the use-case rule, the glossary and the executable spec carry what a reader of the domain needs.

Verdict: **NOT_QUALIFIED**, routed exactly once to `design.md`. No ADR file, chapter-09 register row or ADR-index entry was created.

## Status

STATUS: DONE
