# arc42 impact assessment — export-released-roster-calendar

Change: export-released-roster-calendar. Assessed against `proposal.md`, `specs/`, and `design.md`.

| # | Change introduces or modifies | Affected | Reason / what changed |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | Yes | First product change: filled requirements overview, five quality goals, and the three stakeholders (crew member, plan owner/privacy, release maintainer) in `01-introduction-and-goals.md`. |
| 2 | Constraints (legal, tech, org) | Yes | Recorded the supplied constraints (Rust edition 2021, Windows GNU cross-build, single executable, exact `pdfplumber = 0.1.0` pin, no native extraction dependency, hand-written RFC 5545, permissive licences, no network) plus the privacy/organisational constraints and Clean Architecture convention in `02-architecture-constraints.md`. |
| 3 | External systems, interfaces, context, new user-facing surface | Yes | New user-facing CLI surface: recorded the business and technical context, all filesystem interfaces, and the scope boundary in `03-context-and-scope.md`. No external system. |
| 4 | Major technical strategy or approach | Yes | Recorded the pure-domain-behind-ports strategy, the staged pipeline with a completeness gate, snapshot-scoped identity, create-new publication, and explicit failure in `04-solution-strategy.md`. |
| 5 | New building blocks, modules, components | Yes | Documented the new CLI adapter, conversion orchestrator, six domain modules, and four driven adapters in `05-building-block-view.md`. |
| 6 | New runtime flows, sequences | Yes | Added the successful-conversion, interpretation-failure, and publication-failure flows in `06-runtime-view.md`. |
| 7 | Deployment changes | Yes | Recorded the single self-contained Windows executable, its cross-build, and the absence of runtime dependencies in `07-deployment-view.md`. |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | Added complete-or-explicit-failure, no-clobber publication, explicit failure reporting, snapshot-scoped identity/determinism, privacy, and pure classification/timing in `08-crosscutting-concepts.md`. |
| 9 | Architecture decisions (tradeoffs, ADRs) | No | Every decision candidate is a supplied constraint owned by chapter 2 (hand-written RFC 5545 writer, exact pin, distribution) or product behaviour owned by the capability specification and the glossary (snapshot-scoped UID identity). Applying the qualification gate, none records a project-chosen alternative with a durable cross-change boundary that is not already owned lower down, so no ADR is minted and the chapter-09 register stays empty. |
| 10 | New quality scenarios | Yes | Added measurable scenarios QS-001…QS-006 (completeness/rejection, no-clobber naming, RFC 5545 conformance, determinism, privacy, timing boundaries) in `10-quality-requirements.md`. |
| 11 | New risks or technical debt | Yes | Added risks R-001…R-006 (pin/API feasibility, unproven Windows build, missing coverage scripts, unvalidated expert rules, privacy leakage, parked questions) and debt TD-001…TD-003 in `11-risks-and-technical-debt.md`. R-001's build half is retired by the setup task (the pinned crate compiles). |
| 12 | New domain terms | Yes | Added three confirmed domain terms to `12-glossary.md` — **Released Roster detail-table family**, **Layover**, and **EM duty** — because the capability specification asserts observable behaviour about them; raw source-column labels and the FREE/OFF codes remain verbatim source labels, not confirmed terms. |

## Status

STATUS: DONE
