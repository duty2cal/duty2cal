# arc42 impact — support-cockpit-duty-plans

## Assessment

| # | Chapter | Affected? | Reason |
|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The requirements overview and the faithful-conversion goal name the plan as a Lufthansa crew duty plan; quality goal 8 drops cockpit duties from the boundary. |
| 2 | 02 architecture-constraints | Yes | The public-copy constraint carries the new support sentence and boundary; its rationale records the 2026-11-11 broadening. |
| 3 | 03 context-and-scope | Yes | The context diagrams, both input rows, the support-statement subsection, the visitor row and the scope boundary name the crew duty plan and the two generated fixtures. No new external system or surface. |
| 4 | 04 solution-strategy | No | No strategy, dependency or delivery decision changes; the same owned extraction and calendar writer remain. |
| 5 | 05 building-block-view | Yes | The PDF-adapter responsibility now names adjacent-month `DD Ddd` resolution and the revision-footer stop; the web block's plan name changes. No new building block. |
| 6 | 06 runtime-view | Yes | The support-statement runtime scenario names the crew duty plan. The conversion flow itself is unchanged and needs no new sequence. |
| 7 | 07 deployment-view | No | No platform, packaging, network or hosting change. |
| 8 | 08 crosscutting-concepts | Yes | The public-copy section carries the new sentence and boundary; the owned-extraction section records the adjacent-month date rule and the footer stop. |
| 9 | 09 architecture-decisions | Yes | The register's existing note on the support-statement decision uses the new noun. No new ADR: the reading rules are owned by the capability specification and design; the sentence is product copy owned by its use case. |
| 10 | 10 quality-requirements | No | QS-022 already says “crew duty plan input” and QS-043 asserts carrier agreement, not the sentence's noun. No quality scenario changes. |
| 11 | 11 risks-and-technical-debt | Yes | R-032's description of the input changes to the crew duty plan. No new risk is introduced: the date rule is bounded to three months and the footer rule keeps the existing gap fallback. |
| 12 | 12 glossary | Yes | **Crew position** now names `CP` (Copilot) alongside `FB`; **Supported duty plan**, **Profile content** and **Organization profile** carry the new plan name and boundary. |

## ADR qualification

- **Architectural commitment:** none. The date and footer rules are table-construction details behind the unchanged `RosterSource` port; the support sentence is observable public copy owned by its use case and its check.
- **Real alternatives:** the date rule has alternatives (reject adjacent months, or use a wider window), but they are local mechanisms already framed by the existing timing rule; the public wording was decided by the operator and recorded in the proposal.
- **Durable consequence:** reversing either reading rule changes one adapter function and its scenarios; no stored data, interface or deployment boundary moves.
- **Existing decisions:** ADR-0008 owns the extraction boundary and remains accurate. No `NEW_ADR`, `AMEND_ADR` or `SUPERSEDE_ADR` is qualified.

## Status

STATUS: DONE
