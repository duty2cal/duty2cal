# arc42 impact assessment — export-released-roster-calendar

Change: export-released-roster-calendar. Assessed against `proposal.md`, `specs/`, and `design.md` (including the operator's external brief, reconciled conflict-by-conflict — see `design-review.md`).

| # | Change introduces or modifies | Affected | Reason / what changed |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | Yes | Filled requirements overview, five quality goals, and the three stakeholders in `01-introduction-and-goals.md`; product renamed to `duty2cal`. |
| 2 | Constraints (legal, tech, org) | Yes | Recorded the supplied constraints plus the reconciled ones in `02-architecture-constraints.md`: `pdfplumber = "=0.4.1"`, the two-crate workspace, the core clippy policy, `archunit`, project-wide MPL-2.0 licensing with `deny.toml`/`REUSE.toml`/DCO, and the Windows GNU target. |
| 3 | External systems, interfaces, context, new user-facing surface | Yes | Business/technical context and scope boundary in `03-context-and-scope.md`; CLI-only, no external system. |
| 4 | Major technical strategy or approach | Yes | `04-solution-strategy.md`: pure core behind ports, staged pipeline with a completeness gate, snapshot-scoped identity, create-new publication with enumerated retry; ADR-0001/0002 referenced. |
| 5 | New building blocks, modules, components | Yes | `05-building-block-view.md`: the workspace (`duty2cal-core`, `duty2cal-pdf`, `apps/cli`) and its modules; Phase-2/3 blocks listed as deferred. |
| 6 | New runtime flows, sequences | Yes | `06-runtime-view.md`: successful conversion, interpretation failure, publication failure (unchanged by the reconciliation). |
| 7 | Deployment changes | Yes | `07-deployment-view.md`: single self-contained `duty2cal` executable, Windows GNU cross-build, no runtime dependency. |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | `08-crosscutting-concepts.md`: complete-or-explicit-failure, no-clobber publication, snapshot identity/determinism, privacy, pure rules (unchanged by the reconciliation). |
| 9 | Architecture decisions (tradeoffs, ADRs) | Yes | Two ADRs now qualify and are minted: `ADR-0001` (project licence, MPL-2.0) and `ADR-0002` (layer enforcement), registered in chapter 09 and `adr/README.md`; their `Bad, because…` consequences appear in chapter 11 (R-007, R-008). |
| 10 | New quality scenarios | Yes | `10-quality-requirements.md`: QS-001…QS-006 (completeness/rejection, no-clobber naming, RFC 5545 conformance, determinism, privacy, timing boundaries). |
| 11 | New risks or technical debt | Yes | `11-risks-and-technical-debt.md`: R-001…R-008 (pin API, Windows build, coverage scripts, unvalidated expert rules, privacy, parked questions, MPL weak-copyleft scope, archunit limits) and TD-001…TD-004. |
| 12 | New domain terms | Yes | `12-glossary.md` carries the thirteen Handoff terms plus **Released Roster detail-table family**, **Layover**, **EM duty**, **Travel leg** and **Availability duty**; the reconciled activity rules reuse them unchanged. |

## Status

STATUS: DONE
