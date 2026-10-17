# Design Review: migrate-0-8-4-to-0-9-1

## Method

Fresh-context grilling subagent `GrillMigrationDesign` read `givn instructions design-review`, the proposal, the design (including the appended inventory), the arc42 marker, `givn/config.yaml`, the generated targets, and the retained pre-upgrade session evidence. It independently walked all 12 arc42 rows, all 12 chapter files, and every required design-review branch, and returned a conditional FAIL with four hardening findings and two questions. The orchestrator answered the questions, applied the hardening, and re-ran the verification.

## Questions and resolutions

### Q1 — Is the inventory complete enough to close?

- **Status:** closed 2026-10-17 (operator; answer: "could be the only one, i do not know anymore").
- **Blocks:** whether the migration records no extension or restores displaced text.
- **Context:** the generated rule files were untracked (`rules: local-only`) and were overwritten by the 0.9.1 sync. The scan found one modifying change: the `givn-ideation` `## Diagram` heading was replaced with `**The actor-goal diagram.**` in all four targets as a validator workaround. No other write event appears in the retained omp transcripts or `opencode.db`, and the managed `AGENTS.md` block is unchanged.
- **Options:**
  1. **Accept the evidence boundary (chosen).** Record the scan method, the four affected targets, the `v0.8.5` baseline comparison, and the no-extension disposition. Gain: the change closes with a defensible, evidence-backed disposition. Cost: a manual edit that never appeared in a retained session is unknowable after the rewrite.
  2. **Restore an extension.** Add rule text to `givn/config.yaml` under `extensions:`. Gain: a particular rule survives. Cost: no rule text exists to restore; the only evidenced change is stale and contradicts the served template and the project's own use cases.
- **Resolution:** option 1. The ideation workaround is dropped as stale because the served 0.9.1 `ideation` instruction carries `## Diagram`, and `givn/specs/export-duty-plan-calendar/usecase.md`, `givn/specs/publish-public-release/usecase.md`, the ideation contract, and the archived change all use `## Diagram`. Rejected: inventing or restoring rule text without evidence, which would violate the proposal's "must not invent domain behavior, tests, architecture decisions, or provenance" and the migration's "when a disposition is not derivable, ask" rule.

### Q2 — Does the migration vocabulary belong in the arc42 glossary?

- **Status:** closed 2026-10-17 (orchestrator, conservative choice).
- **Blocks:** chapter 12 and row 12 of the arc42 marker.
- **Context:** the design introduces Givn tooling terms ("displaced project rule", "instruction id", "trigger file"). No extension entry survives, so the project will not use them after archive.
- **Options:**
  1. **No glossary entries (chosen).** The terms are defined by the served 0.9.1 instructions and confined to the archived migration design; the project's ubiquitous language remains the duty-plan domain. Gain: no tooling jargon enters durable domain language. Cost: a reader of `design.md` must consult the served instructions.
  2. **Record the terms.** Add "Instruction extension" and "Displaced project rule" to chapter 12 and mark row 12 Yes. Gain: the archived design is self-contained. Cost: durable tooling terms that no product code uses.
- **Resolution:** option 1, recorded in `arc42.md` row 12 and `## Notes`.

## Branch verdicts

| Branch | Verdict |
|---|---|
| Scope | **F1 applied.** The bundle said the migration edits `givn/config.yaml`; the inventory finds no surviving rule. `design.md` now states Phase 2 is a no-op and this change edits no project file. The `.givn-skip` entry (`specs`) and `givn status` confirm the spec skip. |
| Tech choices | No issue found. The only technology is the existing `extensions:` map; no stack choice exists and no config edit is needed. |
| Missing scenarios | No issue found. No product behavior and no `.feature` delta; the proposal's completion criteria are inventory and verification criteria. |
| Testability | **F2 applied.** `design.md` Phase 3 now names exact checks: 15 trigger-only skills and 15/15/0/0 commands per target, zero `## Project extension` sections, a fresh `givn skills sync` checksum diff, `givn lint`, and `./run-tests.sh`. |
| E2E fidelity | No issue found. `specs` is skipped; no normalized action scope, interface declaration, or `@e2e` delta exists to redefine, and no existing tag is touched. |
| Interaction Coverage | No issue found. No use case owns this change, so there is no `usecase.md` §Interactions inventory to map. |
| Risk | **Mitigation applied.** The residual risk is an invisible manual edit (Q1); the design records the evidence boundary and the operator answer. The zero-edit change is verified by the fresh-sync checksum diff and the green suite. |
| Use-case and Persona context | No issue found. No use case is linked; the use-case rejection criteria are N/A; no Persona is invented or promoted. |
| Decision ledger | No issue found. Neither the proposal nor the bundle contains a `D<n>` or seed question ID. |
| Visual design contract | No issue found. No screen is introduced or changed. |
| Deferrals | No issue found. No mandatory check is deferred. The `specs` skip is a change-type exemption recorded in `.givn-skip`, not a deferral. |
| Persona disposition | **F3 applied.** No Persona is confirmed because this maintenance change has no user-visible surface; the operator initiated it; no Gherkin actor uses Persona biography. |
| Interface classification | No issue found. No screen, CLI action, HTTP surface, or library contract is added or changed. |
| ADR qualification | No issue found. The only decision — drop the stale ideation workaround — has no real alternatives, no architectural boundary, and no durable consequence. Existing ADRs 0001–0004 cover unrelated boundaries. `arc42.md` records `ADR action: NONE`; the canonical home is `design.md`. |
| Architecture documentation | **F4 applied.** An independent 12-row pass found no missing rows. All 12 chapter files exist with substantive content and no ASCII-art diagrams. `arc42.md` row 8 and row 12 reasons were corrected; chapter 11 needs no new entry. |
| Ubiquitous language | Q2 resolved: no glossary entries; rationale recorded in `arc42.md`. |

## Verification evidence

- Installed files: 15 skills in each of `.opencode`, `.agents`, `.claude`, `.omp`; 15 commands in `.opencode` and `.omp`; every file carries trigger/direction and no rule copy. `grep -rn '## Project extension'` across all targets returns nothing.
- `givn skills sync` against a pre-sync SHA-256 manifest of 95 generated files: byte-identical.
- All 16 served instruction ids (`proposal`, `specs`, `design`, `design-review`, `tasks`, `review`, `archive`, `explore`, `ideation`, `steps`, `reverse-engineer`, `characterize`, `domain-modeling`, `dev-principles`, `grill-me`, `grill-with-docs`) return exit 0 and zero `## Project extension` sections.
- `givn lint --change migrate-0-8-4-to-0-9-1` exits 0.
- `nix-shell -p rustc cargo git-cliff --run './run-tests.sh'`: 2 features, 103 scenarios, 390 steps, all passing.

## Sign-off

All grilling questions are resolved, all hardening edits are applied, and the verification evidence is recorded. No blocker remains.

DESIGN-REVIEW: PASS
