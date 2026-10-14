# arc42 impact assessment — list-only-changes-a-user-receives

Change: list-only-changes-a-user-receives. Assessed against `proposal.md`, the delta under `specs/publish-public-release/`, and `design.md` (the marked list of user-receiving paths, the per-entry revision marker, the renderer's filter, and the release-runbook rule).

| # | Change introduces or modifies | Affected | Reason / what changed |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | No | `01-introduction-and-goals.md` needs no edit: requirement 5 already promises that a release's notes state "the changes a user can see", and the release-integrity goal already names the change log. This change makes that promise true by construction rather than changing it. |
| 2 | Constraints (legal, tech, org) | Yes | `02-architecture-constraints.md`: added the marked list of the paths a user receives — read by the renderer, checked against the tree — as the third maintained list beside the capability allow-list and the machinery phrases, with the line between the two kinds of tooling stated. |
| 3 | External systems, interfaces, context, new user-facing surface | No | `03-context-and-scope.md` unchanged: no external system, no new interface. The release text and the change log keep their surface; only which revisions they name changes. |
| 4 | Major technical strategy or approach | Yes | `04-solution-strategy.md`: the release strategy's sentence now says the newest section is filtered to the changes a user receives, and states that a revision touching none of the paths a user's copy is made of is left out. |
| 5 | New building blocks, modules, components | Yes | `05-building-block-view.md`: the renderer's row gains the path-list check, the path-resolution refusal and the drop; the `cliff.toml` row gains the marked path list and the per-entry revision marker. No block, module or file is added. |
| 6 | New runtime flows, sequences | Yes | `06-runtime-view.md`: the version-rendering flow's pre-render check names the path list, and the render step now resolves each surviving entry's revision and drops one that changed no user-receiving path. |
| 7 | Deployment changes | No | `07-deployment-view.md` unchanged: no artifact, target, platform or release asset is added or altered. |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | `08-crosscutting-concepts.md`: "What a reader sees" now states that a released change requires a change to a path a user receives, and names the renderer's diff-based resolution. The existing concepts are unchanged. |
| 9 | Architecture decisions (tradeoffs, ADRs) | No | `09-architecture-decisions.md` unchanged: no ADR is minted. The one candidate — filtering the notes by the change rather than by the words — fails the qualification gate and is routed to `design.md` (verdict below). |
| 10 | New quality scenarios | Yes | `10-quality-requirements.md`: QS-011's scenario and metric gain the machinery-only revision and the path rule, so the releasability criterion states what a version's notes may name. No new node in the quality tree. |
| 11 | New risks or technical debt | Yes | `11-risks-and-technical-debt.md`: TD-008 is recorded as **partly closed** — the removal half is mechanical now, the authorship half stays, and the list's completeness is a new, named judgement (a user-receiving change through an unlisted path is not announced). |
| 12 | New domain terms | Yes | `12-glossary.md`: added **User-receiving path**; refined **Released change** and **Release text** so the third condition is part of the term; the promotion boundary names this change. |

## ADR qualification

One candidate was judged by the instruction's mandatory dimensions and falsification tests: deciding a revision's visibility from the paths it changed rather than from the words of its subject.

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

- **alternatives** — PASS: the design records a real choice between filtering by the diff, reporting at render time, refusing the release, and keeping the phrase list; the proposal holds each option's meaning, gain, cost and example.
- **architectural_impact** — FAIL: no component, ownership, deployment or compatibility boundary moves; the rule lives inside an existing script and its configuration.
- **durable_consequence** — FAIL: it is implementable and reversible within this change (remove the filter and re-render the change log), so the single-change-reversibility falsification test applies.
- **lower_level_artifact** — FAIL: the complete rationale — the path list, the line between the two kinds of tooling and the accepted cost — already lives in `design.md` ("The rule") and in the proposal's D1 resolution, so that home is not incomplete.
- **existing_adr_check** — PASS: the register was searched; ADR-0003 owns the release channel and platform set, ADR-0004 draft-first immutable publication, and neither covers what a version's notes may name.
- **must_be_shared** — FAIL: a contributor reading `cliff.toml` and the release runbook has what they need; the decision does not govern anyone else's component.

Verdict: **NOT_QUALIFIED**, routed exactly once to `design.md`. No ADR file, chapter-09 register row or ADR-index entry was created.

## Status

STATUS: DONE
