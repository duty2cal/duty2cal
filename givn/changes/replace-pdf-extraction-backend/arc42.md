# arc42 Documentation Update: replace-pdf-extraction-backend

> This is the change-level arc42 **impact assessment template**. It is not an
> ADR template and must not be used to draft, store, or combine architecture
> decisions. The architecturally significant decision is one separate MADR file
> at `docs/arc42/adr/ADR-0008-own-the-duty-plan-extraction.md`; chapter 09 is only
> its register/index.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The distributable-release requirement and the maintainability/portability quality goal now state that the shipped artifacts carry an owned extraction and no general-purpose PDF extractor. | Requirement 6 names the owned reading and the test-only oracle; quality goal 5 points to ADR-0008. |
| 2 | 02 architecture-constraints | Yes | The shipped extractor pin becomes a test-only oracle, the object layer becomes a direct dependency, and the frozen subset, fixture crate and same-revision measurement are constraints. | Replaced the `pdfplumber` pin row with `lopdf` + oracle rows; added the fixture-crate, wasm-safe-extraction and backend-measurement constraints. |
| 3 | 03 context-and-scope | No | The external actors, supplied plan, calendar/diagnostic outputs, support statement, browser route and public-home surfaces are unchanged; the new use case is a maintainer goal served by the same conversion. | — |
| 4 | 04 solution-strategy | Yes | The PDF-reading strategy and technology table change from a pinned extractor to an owned interpretation plus a test-only oracle. | Updated the pure-core bullet, the PDF-reading technology row, the ADR register note, and the maintainability and release approaches. |
| 5 | 05 building-block-view | Yes | `duty2cal-pdf` gains the reader modules and the table split; a dev-only fixture crate, a differential test, and the measurement/graph scripts are new blocks. | Updated the level-1 diagram and table, the level-2 `duty2cal-pdf` section, the test harness and the release toolbox (all Mermaid). |
| 6 | 06 runtime-view | Yes | The conversion participant is now the owned extraction, and the named-refusal and backend-measurement flows are new. | Renamed the participant; added "A construct outside the frozen subset is refused by name" and "A backend replacement is measured against the previous extraction"; added the construct refusal to the failure step. |
| 7 | 07 deployment-view | No | The runner classes, supported platform, packaging and publication topology are unchanged; the new measurement runs inside the existing Windows and browser jobs and the byte budget stays. | — |
| 8 | 08 crosscutting-concepts | Yes | The owned extraction, the frozen subset, the named refusal, the test-only oracle and the same-revision measurement are cross-cutting rules. | Added "Owned extraction and the frozen supported subset" and "Measured backend replacement"; updated the non-row failure list, the pure-rules wording, and the panic rule. |
| 9 | 09 architecture-decisions | Yes | ADR-0008 records the shipped extraction boundary and its considered alternatives, including the rejected vendored fork. | Registered ADR-0008 in the chapter-09 register and the ADR directory index; created the MADR file. |
| 10 | 10 quality-requirements | Yes | The change introduces an equivalence/named-refusal scenario and a backend-measurement scenario, and the quality tree gains their nodes. | Added `c4`, `m3` and `ri10` to the tree and QS-047/QS-048 to the scenarios. |
| 11 | 11 risks-and-technical-debt | Yes | ADR-0008's "Bad, because" consequences are standing risks or debt; two stale shipped-extractor risks are closed, the oracle debt is updated, and the one-shot measurement is a standing risk. | Closed R-001 and R-013; updated TD-004; added R-076, R-077, R-078, R-079 and TD-030. The rejected fork is not standing debt and gets no debt line. |
| 12 | 12 glossary | Yes | Three extraction terms are confirmed and referenced by the decision, the scenarios and the code, and the reading boundary, refusal and comparison names are project vocabulary. | Added **Owned extraction**, **Test oracle**, **Frozen supported subset**, **Positioned word**, **Named refusal** and **Differential**. |

## Notes

- The vendor-fork measurement (1,642,384 B host, −591,200 B; `.rodata`
  968,839 → 378,447 B; six private plans convert; the 2026-11 calendar is
  byte-identical, SHA-256 prefix `40cdf1d`) is recorded as a considered
  alternative inside ADR-0008, not as a second decision; no fork is built.
- The ADR directory and both indexes were updated together; the chapter-09
  register links the MADR file and carries no ADR body.
- No diagram was added as ASCII art; every new or changed diagram is a Mermaid
  fenced block.

This impact marker does not replace the ADR qualification or the individual
ADR file.

```text
ADR action: NEW_ADR
ADR target: docs/arc42/adr/ADR-0008-own-the-duty-plan-extraction.md

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null
```

---

## Status

STATUS: DONE
