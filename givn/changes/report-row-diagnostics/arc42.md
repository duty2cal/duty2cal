# arc42 impact: report-row-diagnostics

## Assessment

| # | Chapter | Affected | Reason and change |
|---|---|---|---|
| 1 | Introduction and Goals | Yes | Requirement 2 and quality goal 1 now distinguish a conversion error from a conversion warning, and the crew member's expectation names the reported warning list. |
| 2 | Architecture Constraints | No | No technical, legal or organisational constraint changes; the supplied `thiserror`/typed-error and no-new-dependency rules already cover the new types. |
| 3 | Context and Scope | Yes | The executable and the page gain a second output (the warning report) and the page exposes error and warning lists; the business-context table, diagram and technical-interface table were updated. |
| 4 | Solution Strategy | Yes | The staged pipeline is no longer "all-or-nothing on the first error": it collects row diagnostics, and publication now covers a second report kind; the correctness approach and browser bullet were updated. |
| 5 | Building Block View | Yes | `domain` gains `UnrecognizedActivity`, `application` gains `Diagnostic`/`Severity`, the report builders and `diagnostic_line`, `adapters/ics` renders the new variant unchanged through the existing accessors (no arm), `apps/cli::fs` gains `Kind::Warning`, the WASM facade's result shape grows, and the page gains a state, list containers and the diagnostic-list styling. |
| 6 | Runtime View | Yes | A new "Conversion with warnings publishes a warning report" flow was added; the interpretation-failure flow now processes every row and writes the sectioned report; the browser success and failure flows show lists and offer the matching report. |
| 7 | Deployment View | No | No infrastructure, target, port, credential, hosting or packaging change; the executables and the Site output are deployed exactly as before. |
| 8 | Cross-cutting Concepts | Yes | "Complete-or-explicit-failure" now defines severity, a new "Row diagnostics and two report kinds" concept was added, and the explicit-failure, no-clobber and classification sections were updated. |
| 9 | Architecture Decisions | Yes | One candidate (the severity model and the two report kinds) was assessed and returned `NOT_QUALIFIED`, routed to the use case, with mechanics in `design.md`; the register, ADR index and ADR files are unchanged. |
| 10 | Quality Requirements | Yes | QS-001, QS-026 and QS-029 were rewritten for severity and the sectioned report; QS-045 (diagnostics aggregation) and QS-046 (browser warning visibility) were added, and the quality tree gained a correctness branch. |
| 11 | Risks and Technical Debt | Yes | R-074 records that the fallback's kind is an assumption, R-075 records that an unsaved warning report leaves the warnings only in the output/page, TD-029 records the accepted inference, and R-070/R-071 record that the new `@browser` states are verified by the nightly job rather than the per-push gate. Chapter 9 minted no ADR, so no ADR consequence needed a line. |
| 12 | Glossary | Yes | **Detail row**, **Conversion error**, **Conversion warning**, **Failure report** and **Warning report** were added; the **Duty-plan calendar** aggregate, one invariant and the lifecycle events were revised. |

## Notes

- The change adds no new capability. The use case `export-duty-plan-calendar` owns the behaviour and its `usecase.md` was updated in place, including the explicit replacement of the old unknown-code rule.
- The golden calendar `apps/cli/tests/fixtures/expected/2098-06.ics` is deliberately unchanged: no recognized activity's calendar text changes.
- No new diagrams outside the Mermaid policy; the change's own design carries the data-flow and sequence diagrams.

## Status

STATUS: DONE
