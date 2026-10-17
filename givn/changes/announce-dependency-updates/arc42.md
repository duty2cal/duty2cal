# arc42 impact assessment — announce-dependency-updates

Change: announce-dependency-updates. Assessed against `proposal.md`, the delta
under `specs/publish-public-release/`, and `design.md` (the `chore(deps):`
parser and Dependencies group, the unchanged path filter and visible-count
refusal, and the dependency-only release scenarios).

| # | Change introduces or modifies | Affected | Reason / what changed |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | Yes | `01-introduction-and-goals.md`: the distributable-release goal and the release-integrity quality goal now say a version's notes state the changes a user receives, including dependency updates, rather than only the changes a user can see. |
| 2 | Constraints (legal, tech, org) | Yes | `02-architecture-constraints.md`: the capability allow-list row now says it decides capability scopes only; a dependency update is released through its own parser below that allow-list. |
| 3 | External systems, interfaces, context, new user-facing surface | Yes | `03-context-and-scope.md`: the release-page sentence now names capability changes and dependency updates as what the newest change-log section states. No external system or interface is added. |
| 4 | Major technical strategy or approach | Yes | `04-solution-strategy.md`: the release strategy now says a dependency update that changed the dependency graph is announced under Dependencies, beside the existing path filter. |
| 5 | New building blocks, modules, components | Yes | `05-building-block-view.md`: the `cliff.toml` row now names the dependency-update parser and the Dependencies group; the renderer row's refusal wording now says "no change a user receives" and records the empty-group prune. No block or file is added. |
| 6 | New runtime flows, sequences | Yes | `06-runtime-view.md`: the prepare job's refusal and the render flow now name the changes a user receives, include dependency updates in the group list, state the housekeeping exception for a dependency update that changed a user-receiving path, and record the empty-heading prune before the visible count. |
| 7 | Deployment changes | No | `07-deployment-view.md` unchanged: no target, platform, archive, asset or provenance step changes. A dependency-only version uses the same five-platform preparation and publication path. |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | `08-crosscutting-concepts.md`: "What a reader sees" now defines a released change as either a capability-scoped revision or a dependency update (`chore(deps): …`) that changed a user-receiving path, names Dependencies in the group order, and states that an emptied group does not appear. |
| 9 | Architecture decisions (tradeoffs, ADRs) | No | `09-architecture-decisions.md` unchanged: the admitting rule fails the qualification gate and is routed to `design.md` (verdict below). No ADR is minted. |
| 10 | New quality scenarios | Yes | `10-quality-requirements.md`: the quality tree gains "Dependency updates are releasable and announced", and QS-014 states the dependency-only release criterion, the checks-only exclusion and the unchanged empty-version refusal. |
| 11 | New risks or technical debt | Yes | `11-risks-and-technical-debt.md`: R-018 records that every Cargo dependency update is announced, including build- and test-only crates; TD-011 records the coupling to Dependabot's `chore(deps):` prefix. |
| 12 | New domain terms | Yes | `12-glossary.md`: added **Dependency update**; refined **Housekeeping revision**, **Released change**, **Release preparation** and **Release text** for the dependency case. |

## ADR qualification

One candidate was judged by the instruction's mandatory dimensions and
falsification tests: admitting a dependency update to a version's notes and
allowing a dependency-only release.

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

- **alternatives** — PASS: the proposal records real choices: announce every
  dependency update, only those that changed a user-receiving path, or only
  Cargo updates (D1); place Dependencies before capabilities, after reversals,
  or inside Fixes (D2); announce a capability-scoped update twice or once (D3).
- **architectural_impact** — FAIL: no component, ownership, deployment or
  compatibility boundary moves. The rule is one parser entry in an existing
  configuration, read by an existing script.
- **durable_consequence** — FAIL: the change is reversible within one change by
  removing the parser row and re-rendering; the single-change-reversibility
  falsification test applies.
- **lower_level_artifact** — FAIL: the complete rationale — the group
  placement, the path-filter reuse, the parser prefix and the accepted noise —
  is owned by the release use case and `design.md`; neither home is incomplete.
- **existing_adr_check** — PASS: the register was searched. ADR-0003 owns the
  release channel and platform set; ADR-0004 owns draft-first immutable
  publication. Neither covers what a version's notes may state.
- **must_be_shared** — FAIL: a contributor reading `cliff.toml` and the release
  runbook has the rule; the decision governs no other component.

Verdict: **NOT_QUALIFIED**, routed exactly once to `design.md`. No ADR file,
chapter-09 register row or ADR-index entry was created.

## Index

`docs/arc42/README.md` lists chapters, not change-local decisions; no chapter
was added or removed, so the index is unchanged.

## Status

STATUS: DONE
