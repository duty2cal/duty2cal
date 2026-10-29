---
status: accepted
date: 2026-10-28
amended: 2026-10-29
decision-makers: operator (kressnerd)
consulted: none
informed: none
---

# Own the extraction of the one supported duty plan and keep the third-party extractor as a test-only oracle

> **Amended 2026-10-29** by the object-layer bump: `lopdf` moves from 0.44.0 to
> 0.46.0, adding the brotli decompressor. The differential tests (classic xref,
> xref stream, object streams, multipage plan) stay green against the test
> oracle; the Windows executable grows from 841,728 B to 1,038,336 B and the
> saving over the extractor's 2,233,584-byte host stack falls from 61–63% to
> 53.5%. The decision to own the interpretation behind the object layer is
> unchanged.

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

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

The decision fixes the shipped dependency and ownership boundary of the PDF
reading path: the project owns the interpretation of a frozen supported subset,
and the third-party extractor survives only in the test graph. The word seam
and the measurement commands are local mechanisms owned by `design.md`; the
observable conversion and refusal behaviour is owned by the
`own-the-duty-plan-extraction` capability and its use case. This ADR owns the
durable boundary, the frozen subset and the dependency split. The cheaper home
is recorded explicitly: the frozen subset's construct list and the
refusal/report behaviour are owned by the capability specification, the use
case's Rules and `design.md`; this ADR owns only the dependency-and-ownership
boundary none of them can state.

The existing-ADR check searched the active register (ADR-0001 licence,
ADR-0002 layer model, ADR-0003 release channel and platforms, ADR-0004
immutable publication, ADR-0005 declaration mechanism, ADR-0006 applied state,
ADR-0007 browser-local conversion facade) and the archived register (there is
no `docs/arc42/adr/archive/` directory). ADR-0002's layer model is applied,
not amended: the owned extraction stays a driven adapter behind
`RosterSource`. ADR-0003's archive platforms and ADR-0007's browser facade are
unaffected. No existing record owns the extraction dependency or the supported
PDF subset, so the route is `NEW_ADR`.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would re-add the extractor's
      1,938,024-byte host stack to the shipped graphs, restore the previous
      reader crate and its tests, and re-open the differential and fixture
      strategy; this is broad code and interface work, not one file.
- [x] **Shapes quality attributes** - It removes 86.8% of the host
      executable's PDF stack from the Windows executable and the browser
      bundle, setting a new floor for download size and browser parse cost.
- [x] **Structural** - It defines the dependency direction of the reading
      path: `duty2cal-pdf` owns the content, font, CMap and positioning
      interpretation, `lopdf` owns the file structure, and the extractor is a
      dev-only oracle rather than a fallback.
- [ ] **Must be shared** - Supporting only: a contributor must know that the
      supported subset is frozen and the oracle is test-only, but the graph
      check and the capability's refusal scenarios enforce both, so this item
      is not a gate.

## Context and Problem Statement

The distributed executable is 2,014,208 B and the browser bundle is built from
the same dependency graph. On the host at revision `30bbcb9` the current build
is 2,233,584 B, of which 1,938,024 B (86.8%) leaves when the third-party
extraction stack is removed; the extractor's Adobe CID→Unicode tables alone
occupy 589,240 B of the 968,863 B of `.rodata`. The six supplied reference
plans use one narrow profile — three Type0/Identity-H fonts with per-font
`ToUnicode` CMaps, Flate content, `Tm`-only positioning, classic xref, xref
streams and object streams — so the project pays for a general PDF stack and
its CJK tables to read a stable Latin roster. No feature or profile lever
removes that stack: the extractor exposes no feature that drops its tables and
the already-landed `panic = "abort"` is the last cheap lever.

## Decision Drivers

- The Windows executable's file size and the browser bundle's size are the
  quantities a crew member receives.
- The newer reference files already use xref streams and object streams, so a
  replacement must not take on xref repair as its first risk.
- The accepted calendar, warnings and diagnostics must not change for the same
  source snapshot, and an unsupported construct must fail explicitly rather
  than silently produce an empty calendar.
- The extractor must remain available to prove equivalence in the test graph.
- The single-file, static, no-native-dependency Windows build and the WASM
  route must survive.

## Considered Options

- **Keep the current extractor** (`pdfplumber = "=0.4.1"` unchanged).
- **Own the interpretation behind a small object layer** (`lopdf` 0.46.0 for
  objects, pages, decoded content, xref tables/streams and object streams;
  project-owned tokenizer, text state, fonts, CMaps, positioning and refusals).
- **Own the file structure as well** (xref tables and streams, object streams,
  filters and content; only a decompressor remains).
- **Vendor `pdfplumber-parse` 0.4.1 and stub its four Adobe CID→Unicode tables**
  (GB1, CNS1, Korea1, Japan1), keeping the rest of the extractor.

## Decision Outcome

Chosen option: **Own the interpretation behind a small object layer**, because
it removes the third-party extractor from both shipped graphs while the
measured object-layer floor left a 61–63% saving at 0.44.0 (the 0.46.0 object
layer lands at 1,038,336 B, a 53.5% saving), and because the corpus's xref
streams and object streams stay with `lopdf` instead of becoming the project's
repair problem.

### Consequences

- Good, because the Windows executable and the browser bundle lose the
  extractor's 1,938,024-byte host stack; the object layer's measured floor is
  779,016 B against 2,233,584 B today, and the landed Windows executable
  measured 841,728 B at 0.44.0 and 1,038,336 B at 0.46.0 (amended 2026-10-29),
  still inside the 2,097,152-byte budget.
- Good, because the supported plan's calendar, warnings and row diagnostics are
  compared with the extractor as a test-only oracle rather than migrated by
  inspection.
- Bad, because `lopdf` 0.44.0 cost 483,456 B over the no-PDF floor, the 0.46.0
  bump adds the brotli decompressor and raises the Windows executable from
  841,728 B to 1,038,336 B (measured 2026-10-29), and it still compiles its
  encryption stack, with no feature to drop it; going below that requires
  owning xref and streams. Recorded as TD-030 in chapter 11.
- Bad, because the frozen subset may refuse a later Jeppesen plan whose
  construct is outside it; the refusal is explicit, but the plan does not
  convert until a later ADR widens the subset with evidence. Recorded as R-077.
- Bad, because the CI corpus reproduces the real profile's shape, not the HPDF
  generator's exact bytes, so generator-specific divergence is caught only by
  the opt-in local real-corpus differential. Recorded as R-078.
- Bad, because the owned extraction can disagree with the oracle on a word's
  text or position; the differential fails rather than a silent conversion.
  Recorded as R-076.

### Confirmation

- `cargo test -p duty2cal-pdf` runs the word-level differential over the
  generated Identity-H, xref-stream and object-stream corpus: exact text, and
  `x0`/`top` within 0.5 point, then equal `RawRosterTable` and
  `SourceDocument`.
- `scripts/check-extraction-dependency.sh` refuses the extractor in the
  `x86_64-pc-windows-gnu` and `wasm32-unknown-unknown` normal graphs and
  refuses a missing oracle in `cargo tree -e dev -p duty2cal-pdf`.
- `scripts/measure_extraction_backend.py` and
  `scripts/measure_browser_bundle.py` record the landed and same-revision
  previous-extraction sizes in CI; a non-positive delta rejects the
  replacement.
- The capability scenarios in
  `givn/specs/own-the-duty-plan-extraction/` assert the accepted calendar, the
  named construct refusals, the no-plan-content rule and the unchanged warning
  path.

## Pros and Cons of the Options

### Keep the current extractor

- Good, because it changes nothing and carries no migration risk.
- Bad, because it saves 0 B; the extractor's CJK tables and general layers
  remain in both shipped graphs.
- Bad, because no feature lever reduces the stack and the remaining stable
  profile lever has already landed.

### Own the interpretation behind a small object layer

- Good, because the measured floor is 779,016 B (a 65.1% reduction from
  2,233,584 B) and the landed Windows executable measured 841,728 B at 0.44.0
  (a 61–63% saving) and 1,038,336 B at 0.46.0 (a 53.5% saving), amended
  2026-10-29.
- Good, because `lopdf` keeps xref streams, object streams and filters out of
  the project's code while the project owns the narrow interpretation the plan
  actually uses.
- Bad, because the object layer's cost remains and grew with 0.46.0: 483,456 B
  over the no-PDF floor at 0.44.0, and a 1,038,336-byte Windows executable at
  0.46.0 (amended 2026-10-29), including its encryption stack.

### Own the file structure as well

- Good, because [INFERENCE] ≈400–450 KB total would be roughly 380 KB below
  the chosen cut, an 80–82% saving.
- Bad, because the project would own xref repair and object-stream parsing for
  files that already use them (2026-08/09 have xref streams, 2026-10/11 add
  object streams), and no measured prototype exists.
- Bad, because the reachable saving is an estimate, not a measurement.

### Vendor `pdfplumber-parse` 0.4.1 and stub its four Adobe CID→Unicode tables

- Good, because it was measured on the host at revision `30bbcb9`:
  1,642,384 B against 2,233,584 B, saving 591,200 B (26.5%); `.rodata` falls
  from 968,839 B to 378,447 B; all six private plans convert and the 2026-11
  calendar is byte-identical (the measurement records the SHA-256 prefix
  `40cdf1d`).
- Bad, because it keeps the extractor's layers and adds a vendored fork to
  maintain, while the chosen cut removes the extractor from both shipped
  graphs.
- Bad, because approximately 120 KB of `encoding_rs` CJK tables remain, and the
  resulting executable is still about 1.64 MB.
- Rejected as an interim alternative, not as a second decision. If the project
  ever needs it, the measurement above is the starting evidence; no fork is
  built by this change.

## More Information

- Proposal, specs and design:
  `givn/changes/replace-pdf-extraction-backend/`.
- The measured host sizes, crate attribution, `.rodata` split and the six-plan
  corpus findings are recorded in that change's briefing; the Windows and
  browser-bundle numbers remain to be measured in CI.
- The fully self-owned candidate reopens only with a measured prototype at or
  below the [INFERENCE] ≈450 KB upper bound and a green differential over the
  six-file corpus.
- Related: ADR-0002 (the driven-adapter layer this decision keeps), ADR-0007
  (the browser facade that ships the same reader).
