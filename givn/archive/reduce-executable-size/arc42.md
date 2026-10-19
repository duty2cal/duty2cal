# arc42 impact assessment — reduce-executable-size

Change: reduce-executable-size. Assessed against `proposal.md`, the saving
under `specs/reduce-distributed-executable-size/`, the permanent use case,
and `design.md` (the measured lever, the cumulative report, the failure
outcome, and the landed `panic = "abort"`).

| # | Change introduces or modifies | Affected | Reason / what changed |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | No | No new goal or stakeholder. The reduction serves the existing release-integrity goal; the build-configuration reviewer stakeholder row already exists. |
| 2 | Constraints (legal, tech, org) | Yes | `02-architecture-constraints.md`: the release profile carries `panic = "abort"` as a measured lever (83,968 B smaller on the supported platform), with the failure consequence recorded; the reduction command is build tooling outside the workspace. |
| 3 | External systems, interfaces, context, new user-facing surface | No | `03-context-and-scope.md` unchanged: the command is repository build tooling; GitHub remains the only external system. |
| 4 | Major technical strategy or approach | Yes | `04-solution-strategy.md`: the size-attribution approach gains the applied lever and the cumulative baseline/lever measurement. |
| 5 | New building blocks, modules, components | Yes | `05-building-block-view.md`: the release toolbox gains `scripts/measure_size_lever.py`, the lever report, and the lever step files; the profile setting. |
| 6 | New runtime flows, sequences | Yes | `06-runtime-view.md`: a new scenario, "A size lever is measured and landed", records the refusal, baseline, cumulative lever builds, the saving, the failure outcome and the landed profile. |
| 7 | Deployment changes | Yes | `07-deployment-view.md`: the Windows executable is built from the profile with `panic = "abort"`; the release build and Windows smoke are unchanged; the executable is smaller. |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | `08-crosscutting-concepts.md`: the panic outcome and the preserved conversion diagnostic are recorded as the cross-cutting failure rule. |
| 9 | Architecture decisions (tradeoffs, ADRs) | No | No qualified ADR: the profile lever is owned by `design.md` and the capability spec; the qualification is below. |
| 10 | New quality scenarios | Yes | `10-quality-requirements.md`: QS-025 records the measured lever, the rejected larger lever and the landed size. |
| 11 | New risks or technical debt | Yes | `11-risks-and-technical-debt.md`: R-039 (abort loses unwinding) and R-040 (the host/Windows failure-path boundary), plus the CI-cost debt row. |
| 12 | New domain terms | Yes | `12-glossary.md`: **Size lever** is now applied, with its measured contract and the panic outcome; **Baseline**, **Published binary** and **Saved bytes** record the reduction's quantities. |

## ADR qualification

The candidate is the `panic = "abort"` profile lever. Alternatives: unwind
(current), abort (applied), `opt-level = "s"` (rejected by measurement), and a
feature audit (no measured target). The decision is recorded in `design.md`
and the capability spec; it is a build-configuration choice, not a component,
ownership, dependency, deployment or compatibility boundary. Reversing it is a
one-line profile change and a new build; no migration or cross-component
coordination is required. `alternatives` PASS; `architectural_impact`,

`durable_consequence` and `lower_level_artifact` FAIL; `existing_adr_check`
PASS (ADR-0001…0006 hold no build-profile decision; no archived ADR directory exists).
Routing: `CANONICAL_ARTIFACT` — `design.md`. No ADR file or register row is
created.

## Status

STATUS: DONE
