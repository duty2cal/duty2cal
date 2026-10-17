# arc42 impact assessment — wait-for-green-checks-before-publish

Change: wait-for-green-checks-before-publish. Assessed against `proposal.md`, the
delta under `specs/publish-public-release/`, and `design.md` (the extracted
`scripts/await-green-checks.sh`, the repeatable release-suite exclusion, the
bounded poll, and the publish-job step replacement).

| # | Change introduces or modifies | Affected | Reason / what changed |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | No | `01-introduction-and-goals.md` needs no edit: the release-integrity goal already promises that no published promise outruns its evidence. The new release-gate criterion belongs to chapter 10. |
| 2 | Constraints (legal, tech, org) | Yes | `02-architecture-constraints.md`: the change-log-drift constraint's rationale quoted the old publish behaviour (“refuses only checks that already concluded non-green”). It now states the gate waits and refuses a concluded non-success or the bound, while the preparation's drift check remains independent. |
| 3 | External systems, interfaces, context, new user-facing surface | No | `03-context-and-scope.md` unchanged: GitHub already owns the checks and the release; the gate consumes the same check-runs interface. |
| 4 | Major technical strategy or approach | No | `04-solution-strategy.md` unchanged: the gate is a local mechanism inside the existing publish job, not a strategy change. |
| 5 | New building blocks, modules, components | Yes | `05-building-block-view.md`: the release toolbox gains `scripts/await-green-checks.sh`, its invocation edge from the publish job and its subprocess edge from the suite; the `release.yml` row names the wait. |
| 6 | New runtime flows, sequences | Yes | `06-runtime-view.md`: the release scenario gains the gate between preparation and the draft, with the refusal edge to “nothing public”; the numbered steps now include the gate and renumber publish/verify. |
| 7 | Deployment changes | Yes | `07-deployment-view.md`: the publish node and the `ubuntu-latest` row name the bounded wait; no runner, asset or permission changes. |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | `08-crosscutting-concepts.md`: a new paragraph records the publication gate as the release-time counterpart of the landing gate — bounded wait, release-suite exclusion, skipped/neutral allowed, unknown conclusions fail closed, nothing published before it passes. |
| 9 | Architecture decisions (tradeoffs, ADRs) | No | `09-architecture-decisions.md` unchanged: the extracted-command and timing choices are `NOT_QUALIFIED` and routed to `design.md`; the behavioural rules belong to the use case and Gherkin. |
| 10 | New quality scenarios | Yes | `10-quality-requirements.md`: the quality tree gains “Publication waits for every check to conclude”, and QS-015 states the release-gate currency criterion. |
| 11 | New risks or technical debt | Yes | `11-risks-and-technical-debt.md`: TD-009 closes, with the host-only proof and the 1000-suite cap recorded as residuals. |
| 12 | New domain terms | Yes | `12-glossary.md`: added **Check run**, **Check suite** and **Publication gate**; refined **Gate (on landing)** to name its release-time counterpart. |

## ADR qualification

One candidate: extracting the wait into `scripts/await-green-checks.sh` and
choosing its polling policy.

| Field | Value |
|---|---|
| qualification | NOT_QUALIFIED |
| alternatives | PASS — extracted command vs inline shell; 15 s/1800 s vs shorter/longer bounds |
| architectural_impact | FAIL — no component, ownership, deployment or compatibility boundary moves; the gate is a step inside the existing publish job |
| durable_consequence | FAIL — reversible within one change by inlining or deleting the script; the single-change-reversibility test applies |
| lower_level_artifact | FAIL — `design.md` and the delta use case own the rationale |
| existing_adr_check | PASS — ADR-0003 covers the release channel and platforms; ADR-0004 covers draft-first immutable publication; neither covers this gate |
| must_be_shared | FAIL — a contributor reading the workflow and this design has the rule |
| routing | CANONICAL_ARTIFACT |
| canonical_artifact | `design.md` |
| target_adr | null |
| replacement_adr | null |

Verdict: **NOT_QUALIFIED**, routed exactly once to `design.md`. No ADR file,
chapter-09 register row or ADR-index entry was created.

## Status

STATUS: DONE
