# arc42 assessment — reduce-test-runtime

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and goals | No | The product goals and stakeholders are unchanged. |
| 2 | Architecture constraints | Yes | The test-scope boundary, the size budget, and the removal of operational toolchains from `cargo test` are constraints. |
| 3 | Context and scope | No | No external system or product surface changes. |
| 4 | Solution strategy | No | The per-push size attribution and the Windows release strategy are unchanged; the byte budget is an added CI gate recorded in chapters 2 and 7. |
| 5 | Building block view | Yes | The test harness loses the operational step modules; the browser and site toolboxes keep their build responsibilities. |
| 6 | Runtime view | Yes | No product flow changes, but the operational runtime scenarios' evidence moved from the cucumber suite to the local planning proof, workflows and bats; chapter 6 records the amendment. The pushed-revision attribution flow is unchanged. |
| 7 | Deployment view | Yes | The CI job set changes: `test`, `browser`, `coverage` on `main`, `scripts`, and the Windows size budget. |
| 8 | Cross-cutting concepts | Yes | A new "test scope and phase reporting" concept governs what the suite executes. |
| 9 | Architecture decisions | No | No new ADR; the drift decision follows ADR-0006 and the test-scope choice is a process/design decision. |
| 10 | Quality requirements | Yes | New quality scenarios for the suite's wall-clock ceiling and the executable size budget. |
| 11 | Risks and technical debt | Yes | Operational Gherkin is no longer executed; the size budget is a fixed ceiling; the publication's shell refusals are tested, its full behaviour is not. |
| 12 | Glossary | No | No new domain term; "test scope" is described in chapter 8 and design.md. |

## Status

STATUS: DONE

The operational runtime scenarios in chapter 6 are flows whose executable
evidence is now the local planning proof, the owning workflow, the bats tests,
or the maintainer's preview/apply; the chapter carries the test-scope amendment
at its head and names the new owner at each affected note.
