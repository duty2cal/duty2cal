# arc42 Documentation Update: own-the-pdf-file-structure

> This is the change-level arc42 **impact assessment template**. It is not an
> ADR template and must not be used to draft, store, or combine architecture
> decisions. The architecturally significant candidate is recorded as an
> **amendment of the existing** MADR file at
> `docs/arc42/adr/ADR-0008-own-the-duty-plan-extraction.md`; chapter 09 is only
> its register/index and no second ADR file exists for this boundary.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | Chapter 01 is unchanged: the extraction goal already points at ADR-0008, whose amendment is only proposed; the quality-scenario edit (QS-048) is row 10. | — |
| 2 | 02 architecture-constraints | Yes | The shipped dependency constraint changes: `lopdf` leaves the Windows and WASM normal graphs, `miniz_oxide` becomes the only third-party crate in the reading path, and the oracle keeps its own library copy in the dev graph. | Rewrote the `lopdf` and `pdfplumber` rows and added the `miniz_oxide` pin row, each marked as the ADR-0008 amendment proposed 2026-10-29 (pending Gate A). |
| 3 | 03 context-and-scope | No | No external actor, interface, public surface or use-case inventory entry changes: the proposal adds no consumer action (D7) and the design states the public surface (`PdfRosterSource`, `RosterSource`, `SourceDocument`, `SourceError`, `Construct`, positioned words) does not move. | — |
| 4 | 04 solution-strategy | Yes | The PDF-reading strategy changes from a third-party object/xref layer to a project-owned file structure with one pure-Rust decompressor. | Updated the pure-core bullet, the PDF-reading technology row, the ADR summary paragraph, and the maintainability and release-integrity quality approaches. |
| 5 | 05 building-block-view | Yes | New `reader/structure/` modules (object model, parser, xref, filters) inside `duty2cal-pdf`, an updated dependency graph, new fixture variants and a changed measurement predicate in the release toolbox. | Added the file-structure element row and a Mermaid reading-path diagram, updated the `duty2cal-pdf` responsibility, the level-1 row, the fixture generator, the step row and the measurement/graph script rows. |
| 6 | 06 runtime-view | Yes | The successful-conversion flow now reads the file structure in project code, and the backend-measurement flow's base-manifest predicate is the removed library; Gate A and Gate B are named. | Updated the conversion step, the measurement diagram and steps, and the notes that name Gate A (entry, not met) and Gate B (landing). |
| 7 | 07 deployment-view | No | The runner classes, supported platform, packaging, publication topology and byte budget are unchanged; the measurement runs in the existing Windows and browser jobs. | — |
| 8 | 08 crosscutting-concepts | Yes | The owned-extraction concept gains the file structure, the no-repair rule, the decode bound and the predictor policy (PNG 10–15 applied, every other value passed through), the oracle's own library copy, and the parser's panic discipline. | Rewrote "Owned extraction and the frozen supported subset", extended the panic rule, and rewrote "Measured backend replacement" for the new predicate and the two gates. |
| 9 | 09 architecture-decisions | Yes | ADR-0008 is amended in place for the file-structure ownership decision, proposed pending Gate A; the register and the ADR directory index record it. | Register row status now reads "accepted (amendment proposed 2026-10-29)"; a register note and an ADR-index note describe the amendment and its gate; no new ADR file was created. |
| 10 | 10 quality-requirements | Yes | QS-048's acceptance names the compared layer, whose base-manifest predicate changes from the extractor to the removed library. | Reworded QS-048 (stimulus, not-applicable case and the graph check); no new quality scenario or tree node was invented. |
| 11 | 11 risks-and-technical-debt | Yes | The amendment's "Bad, because" consequences are standing risks and the reopen attempt belongs to TD-030; the parser and the gate outcomes must be recorded. | Recorded TD-030's reopen attempt and gate (debt stays open), refined R-077 and R-078, and added R-082 (spellings), R-083 (parser discipline, the remaining two recorded differences from the previous reader and the checksum recovery) and R-084 (the replacement may not pay). |
| 12 | 12 glossary | Yes | A new domain term names the layer the decision moves, and three extraction terms described `lopdf` as part of the owned reading. | Added **File structure**; refined **Owned extraction**, **Frozen supported subset** and **Test oracle**; extended the tally paragraph; added **File-structure spelling**. |

## Notes

- **The amendment is proposed, not accepted.** Every chapter that describes the
  new boundary says so and names **Gate A** — a measured host prototype at or
  below ADR-0008's [INFERENCE] ≈450 KB upper bound with a green differential
  over the six-file corpus. Gate A is **not met** at design time (no prototype
  exists; the design's arithmetic leaves roughly 7–18 KB of headroom). No
  chapter claims Gate A or Gate B succeeded, and none claims that `lopdf` has
  already left the shipped graphs: the accepted boundary today is still the
  object layer plus the test-only oracle.
- **No second ADR.** The candidate refines the same extraction/dependency
  boundary ADR-0008 already owns and that record already listed "Own the file
  structure as well" as a considered option with a reopen condition. The
  amendment was written inside that file with its own qualification verdict,
  MADR checklist evidence, status, consequences and confirmation; the
  frontmatter keeps `status: accepted` for the current decision and adds
  `amended: 2026-10-29`: the object-layer boundary and the oracle are accepted,
  the file-structure ownership is proposed. See
  "Amendment — 2026-10-29: own the PDF file structure too (proposed, pending
  Gate A)".
- **Falsification tests.** The candidate is not reversible inside one component
  and one change: reversal restores the manifest and the lock, re-opens the
  differential and the measurement evidence across the Windows, WASM and dev
  graphs, and re-edits the graph check and the measurement predicate. It
  constrains at least two future changes and two components — every dependency
  update of the library (the reverted Brotli bump grew the Windows executable by
  196,608 B) and the oracle's dev graph. It is not product behaviour: the
  observable calendar, the refusals and the reports are owned by the capability;
  the accepted spellings and mechanisms are `design.md`. So it passes the
  `architectural_impact` and `durable_consequence` dimensions rather than being
  excluded by kind.
- **TD-030 stays open.** The chapter-11 row records the 2026-10-29 reopen
  attempt and its two gates and explicitly keeps the debt open; a later revision
  closes it only after Gate A's recorded ruling and Gate B's positive measured
  delta. The ADR's own "Bad, because" consequence is unchanged in kind.
- **Quality goal 5 is left as accepted.** Chapter 01's sentence "the reading
  path is the project's own" is not widened here: it becomes fully true only
  once the amendment clears Gate A, and chapter 2 states plainly that `lopdf`
  supplies the object/xref layer today. Changing it now would claim a gate
  result that does not exist.
- **Diagrams.** The one diagram added is a Mermaid fenced block (chapter 5's
  reading path); every diagram edited in chapters 5 and 6 remains Mermaid. No
  ASCII art was introduced.
- **Frozen supported subset.** The design's D2 spellings (PNG predictors 10–15
  applied with every other value passed through, `/Prev` updates, hybrid
  `/XRefStm`, direct or indirect stream length) are
  spellings of constructs the previous reader already accepted; the glossary row
  records that they add no construct and that a form outside them is the
  unreadable refusal, so R-077's meaning and the fifteen `Construct` names are
  unchanged.
- `docs/arc42/README.md` needed no edit: it holds the chapter list and the
  storage/status rules, not an ADR status table; its links were verified
  unchanged and it agrees with both ADR indexes.

This impact marker does not replace the ADR qualification or the individual
ADR file.

```text
ADR action: AMEND_ADR
ADR target: docs/arc42/adr/ADR-0008-own-the-duty-plan-extraction.md

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: AMEND_ADR
canonical_artifact: null
target_adr: ADR-0008
replacement_adr: null
```

---

## Status

STATUS: DONE
