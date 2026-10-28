# Proposal: replace-pdf-extraction-backend

## Release note

The converter now carries its own extraction for the supported duty plan, so the Windows download and the browser bundle shrink substantially while the calendar a crew member receives stays the same.

## Use-Case Context

- Use-case ID: `own-the-duty-plan-extraction` (new; the permanent leaf this change creates), `export-duty-plan-calendar` (document-only: its delivery-constraint sentence), `reduce-distributed-executable-size` and `measure-distributed-executable-size` (unchanged: both keep excluding backend replacement, which this use case now owns)
- Ideation topic: none — this change has no ideation seed. It redraws the boundary recorded by two topics, as described below.
- Seed path: none
- Confirmed Personas: `project-maintainer--release-owner`, `crew-member--personal-calendar-owner`, `contributor--web-reviewer` — all three exist in `givn/personas/`; no Persona is invented or promoted. The release owner decides and answers for the frozen subset and the measured saving; the crew member is the accuracy and privacy lens for the calendar, the refusal text and the download; the web reviewer is the lens for the browser bundle and the unchanged browser route.

### How the boundary is redrawn

- `release-executable-size` excluded "replacing the PDF backend" as a product decision with its own ADR (`givn/ideation/release-executable-size/frame.md:61`), and both size use cases list backend replacement out of scope. That exclusion stays true for a **size lever**, which is a build-configuration change; it does not say the project never replaces the backend. This change is that separate product decision and now owns it in its own use case. Neither size use case is extended.
- `generalized-duty-plan-input` inherits "one pinned PDF extraction dependency, no native C/C++ extraction dependency" (`givn/ideation/generalized-duty-plan-input/frame.md:168-170`). For work after this change the inherited constraint reads: one project-owned extraction implementation behind the source port, with the third-party extractor kept only as the test oracle, and no native C/C++ extraction dependency. The topic's next session re-reads its Frame; this change does not edit the ideation record.
- `export-duty-plan-calendar`'s delivery constraints name the exact third-party extractor pin. This change carries a document-only delta of that use-case document, replacing that phrase with the owned-extraction constraint. Its Goal, Actors, conversion rules, scenarios and guarantees are untouched.

## Problem / Opportunity

The distributed executable and the browser bundle are dominated by a general-purpose PDF extraction stack, of which the one supported duty plan uses a narrow slice.

Measured at revision `30bbcb9` on the host (`x86_64-unknown-linux-gnu`, committed release profile):

| Build | File size |
|---|---:|
| current | 2,233,584 B |
| adapter stubbed, extractor removed from the dependency graph | 295,560 B |
| a small PDF object layer with a stubbed adapter | 779,016 B |

The extraction stack is 1,938,024 B, or 86.8% of the executable. `cargo bloat --crates` attributes 141.1 KiB of `.text` to the object layer, 222.8 KiB to the extractor's own three crates, and `duty2cal-pdf` itself only 31.5 KiB. Of 968,863 B of `.rodata`, 589,240 B (60.8%) is the extractor's Adobe CID→Unicode tables for GB1, CNS1, Korea1 and Japan1, plus 37,680 B of a CJK encoding table — tables for scripts the supported roster never prints. The published Windows executable is 2,014,208 B, and the browser bundle is built from the same graph, so a crew member downloads and the browser parses the same unused stack.

The six supplied roster months (private, gitignored) prove how narrow the used slice is: three Type0/Identity-H fonts with per-font `ToUnicode` CMaps, Flate content, `Tm`-only positioning, classic xref and xref streams, and object streams. No encryption, no rotation, no Type3 font, no inline image, no `TJ`.

Impact: 86.8% of the executable is a stack the one supported plan barely uses, and the browser route pays the same cost before it can offer a calendar. Nothing else about the conversion is wrong.

## Proposed Solution

- The project reads the supported duty plan itself, inside the converter's existing reading boundary. The executable and the browser bundle carry no third-party PDF extractor, and the Windows download and the browser bundle become substantially smaller.
- Nothing a crew member reads from the calendar, a warning or a diagnostic changes: for the same source snapshot the converted calendar, the conversion warnings, the detail-row diagnostics and the numbered failure-report format are the same as the previously accepted extractor's. A supported plan that would differ is refused rather than accepted with a different result.
- The executable and the browser keep producing their activities through the same table rules as today, so the two routes cannot diverge; only the reading changes.
- A document that uses a PDF construct outside the frozen supported subset is refused by name: encryption, a Type3 font, text rotation, horizontal text scaling, an inline image, or a stream filter outside the subset. The refusal names the source PDF, the construct and, when one is known, the page; it writes the numbered failure report, publishes no calendar, and never panics. A readable PDF outside the supported family keeps the existing unsupported-family refusal; a non-PDF keeps the unreadable refusal. No refusal produces an empty calendar or a partial month.
- The supported subset is frozen by a recorded architecture decision (ADR-0008). Widening it is a decision with its own evidence, not an incidental code change.
- The third-party extractor remains only as a test oracle: the release and browser dependency graphs no longer contain it, the test graph keeps it, and a mechanical check holds that split.
- The saving is measured where it ships: the Windows GNU executable and the browser bundle, against the same revision rebuilt with the previous extraction. Host numbers are not evidence, and the published binary is not used as the baseline.
- The single-file static executable, the Windows GNU cross-build, the browser conversion route, the no-network and privacy rules, and the public support statement are unchanged.

## Capability Routing

`givn spec route --change replace-pdf-extraction-backend` reports no signal — the top candidates tie at 13.00 — and prints no ranked list, so no recommendation column is recorded. This table is the declaration `givn check review` compares against the delta.

| Proposed capability | Decision | Rationale |
|---|---|---|
| `own-the-duty-plan-extraction` | `NEW in own-the-duty-plan-extraction` | The actor goal is the release maintainer's: read the one supported plan with an extraction the project owns, prove the delivered calendar and diagnostics equivalent, refuse every construct outside a frozen subset by name, and measure the shipped artifacts. None of the existing use cases owns that goal. `reduce-distributed-executable-size` defines a size lever strictly as a build-configuration change with a cumulative measured saving and excludes backend replacement; `measure-distributed-executable-size` observes a build and changes nothing; `export-duty-plan-calendar` is the crew member's export and its two inventory actions are the executable and browser conversions, so a second capability there would duplicate an action or name a component. |
| `export-duty-plan-calendar` (document only) | `EXTEND export-duty-plan-calendar` | Only the delivery-constraint sentence changes: the exact third-party extractor pin becomes one project-owned extraction implementation behind the source port. No conversion rule, scenario, Actor or guarantee of that use case changes. |

## Out of Scope

- Any change to conversion rules: activity-code meanings, date and timing rules, travel pairing, calendar representation, warning and error semantics for detail rows, and the report formats.
- Supporting another plan, operator, generator or PDF layout; encrypted PDFs; a general-purpose PDF library, a PDF writer, or any native C/C++ dependency.
- A size ceiling or a release refusal on size (the ideation question stays parked), other size levers, and the build profile.
- Publishing a release, its archive, packaging or platform set; the browser page's copy, design or interaction; the site publication.
- Committing a real plan or any plan-derived fixture. The private corpus stays local, gitignored and opt-in.
- Editing the ideation records of `release-executable-size` or `generalized-duty-plan-input`; this proposal records the redraw, and their next session re-reads it.
- The size use cases' own measurement report shape and cadence.

## Open Questions

Each question carries its option treatment into `design.md`, whose Decisions ledger records the resolution; D5 is already answered by the Capability Routing table. The measured evidence is named where it exists; no figure is invented, and every `[INFERENCE]` stays labelled.

### D1 — Which layer of the PDF stack does the project own?

- Question: Does the project own only the interpretation of the supported plan's content — text operators, fonts, CMaps, positioning and refusals — while keeping a small third-party PDF object/xref layer, or does it own the file structure (cross-reference tables and streams, object streams, filters) as well, or does it keep the current extractor?
- Options:
  1. **Own interpretation, keep a small object layer** — the project reads objects, pages and decoded content through a small library and owns everything from the content stream onwards. Gain: the object layer's floor is measured at 779,016 B with a stubbed adapter against 2,233,584 B today (−65.1%); the corpus already exercises xref streams and object streams, so the riskiest parsing stays outside. Cost: ≈483,456 B of object-layer code and its encryption dependencies remain; the landed build is [INFERENCE] ≈820–860 KB, a 61–63% saving rather than a larger one. In practice: a `%PDF-1.5` plan with an xref stream and object streams reads through the layer, and the project's code starts at the content stream.
  2. **Own the file structure too** — the project reads xref tables and streams, object streams, filters and content itself; only a decompressor remains. Gain: [INFERENCE] ≈400–450 KB total, roughly 380 KB below option 1, an 80–82% saving. Cost: the project owns xref repair and object-stream parsing for the exact files that already use them, and no measured prototype of this floor exists. In practice: a damaged xref becomes the project's repair problem instead of the library's.
  3. **Keep the current extractor** — no layer change. Gain: no risk. Cost: no feature or profile lever remains; the extractor exposes no feature that drops its CMap tables, and `panic = "abort"` has already landed. In practice: the download stays at roughly its current size.
- Disposition: carried — owner: the design author; decided in `design.md` (D1) with the measured floors, and option 2 is recorded either as rejected or as a later candidate with its measured precondition.

### D2 — Where is the seam between the owned extraction and the table rules?

- Question: Is the new boundary "bytes → positioned words", with one table implementation and two extractors behind it, or a second complete source implementation, or an in-place swap behind the existing port?
- Options:
  1. **Word-level seam** — the owned extraction produces positioned words; the existing detail-table construction consumes them unchanged, and the oracle produces the same word payload in tests. Gain: one copy of the table rules, a differential test on the primitive that can localize a drift to reading or to table construction, and the executable and browser share the table layer. Cost: the owned extraction must reproduce word text and positions within the agreed tolerance, and the page heading and footer must be derived from the same words instead of a separate page-text extraction. In practice: a one-point drift in `x0` fails the differential test before any calendar differs.
  2. **Second complete source** — two independent pipelines, each producing detail rows. Gain: the oracle path is independent end to end. Cost: two copies of column bands, clustering, dates and footer rules; a fix lands twice or the copies drift, and a calendar difference cannot be attributed to reading or to table construction. In practice: a row-clustering fix is made twice.
  3. **In-place swap behind the port** — the owned extraction replaces the current code inside the same function with no word payload. Gain: fewest moving parts. Cost: the oracle can only compare final calendars, which hides the failing layer; the table rules cannot be exercised without a PDF. In practice: a regression localizes to "the conversion" rather than to reading.
- Disposition: carried — owner: the design author; decided in `design.md` (D2), including the payload type and the tolerance.

### D3 — How does the differential test get a corpus in CI?

- Question: The oracle must leave the release and browser graphs but stay in the test graph, and the real plans are private and gitignored. Which corpus proves equivalence on every push, and which risk remains uncovered?
- Options:
  1. **Synthetic corpus in CI plus an opt-in real-corpus run** — the committed test builder emits the real shape (Type0/Identity-H with `ToUnicode`, Flate content, classic xref, xref stream, object streams), and the existing local-corpus script gains an opt-in word-level differential over `private/`. Gain: every push exercises the real font path rather than a standard-font shortcut, the private plans never enter the repository, and the exact generator output is covered locally. Cost: a builder reproduces the shape but not the generator's exact bytes, and CI alone cannot prove generator-specific behaviour. In practice: CI fails when a word position drifts on the synthetic plan; the maintainer's local run catches generator-specific drift.
  2. **One sanitized real plan committed** — Gain: CI sees the exact generator output. Cost: a real plan in public history, a privacy review that can miss a field, and a standing re-identification risk; the privacy rule forbids it. In practice: a sanitized file still carries the generator's structure.
  3. **Local-only differential** — Gain: no fixture work. Cost: CI has no equivalence gate, so a later revision can regress the extraction silently. In practice: the gate runs only on one workstation.
- Disposition: carried — owner: the design author; decided in `design.md` (D3), including the tolerance, the fixture generator's location, the local command, and the exact risk each choice leaves uncovered.

### D4 — How is an unsupported construct carried to the crew member?

- Question: Should an unsupported PDF construct be a typed, named variant of the existing source failure, a separate extraction error type, or a free-text unreadable message?
- Options:
  1. **A named variant on the existing source failure** — the source error gains one variant carrying a closed construct name and the page; the existing two-line report keeps `Source PDF:` and `Reason:` and the numbered report names. Gain: the caller distinguishes "not a PDF", "not the family" and "a construct outside the frozen subset" without parsing text, and the report contract stays. Cost: one core type and its names become part of the contract. In practice: a Type3 plan reports `the source uses a PDF construct outside the supported family: Type3 font on page 1`.
  2. **A separate extraction error type** — Gain: the extraction owns its vocabulary. Cost: the source port must map it, a second failure vocabulary reaches the CLI and the browser, and the existing contract splits. In practice: two failure types with two wordings to keep aligned.
  3. **Free-text unreadable message** — Gain: no type change. Cost: "unreadable" conflates a corrupt file with a valid plan the project refuses, and the distinction can only be tested by string. In practice: a valid PDF is reported as "not a readable PDF", which is false.
- Disposition: carried — owner: the design author; decided in `design.md` (D4), including the closed construct names and the rule that no printed plan content enters the message.

### D5 — Does this extend a size use case or become its own use case?

- Question: Which permanent use case owns reading the supported plan with an extraction the project owns, and how is the existing out-of-scope boundary redrawn?
- Options:
  1. **A new use case `own-the-duty-plan-extraction`** — Gain: the actor goal, the frozen-subset decision, the differential-equivalence contract and the shipped-artifact measurement live in one place; the size use cases keep their narrow lever definition. Cost: a new use case and capability, a document-only delta of the export use case, and a redraw note for the two ideation topics. In practice: the release maintainer reads one use case to know what "supported" means for the extraction.
  2. **Extend `reduce-distributed-executable-size`** — Gain: no new permanent leaf; the saving is measured there anyway. Cost: its Rules define a lever as a build-configuration change reviewed by the build-configuration reviewer, and its Out of scope names backend replacement; an extractor replacement has no equivalence or refusal contract there, so its Goal and Rules would have to be rewritten for a source change. In practice: a source-code replacement is recorded as if it were a Cargo profile lever.
  3. **A new capability under `export-duty-plan-calendar`** — Gain: no new use case. Cost: the actor goal there is the crew member's export with exactly two inventory actions; the new capability would either duplicate a conversion action or name a component, and the use case's delivery constraint pins the extractor. In practice: the same CLI action is covered twice, or a component-shaped capability appears.
- Disposition: answered — the Capability Routing table above and "How the boundary is redrawn" under Use-Case Context; `design.md` records the same decision in its ledger as D5.

### D6 — Which build is the measurement evidence, and what is the baseline?

- Question: The published artifact is the Windows GNU executable and the browser bundle, while the local measurements above are a Linux host. What does CI measure, and against which baseline, so the saving is attributable to this change?
- Options:
  1. **Same-revision restore on both shipped targets** — build the landed revision for Windows and for the browser; rebuild the same tree with the extraction crate and lock restored from the base revision; record both sizes and the delta in CI. Gain: the delta attributes to the extraction alone, on the shipped platform and the browser target. Cost: a new measurement command and one extra build per target. In practice: the Windows job uploads a report naming the landed and baseline sizes and the delta.
  2. **Base revision rebuild** — build the base revision as the baseline. Gain: simpler command. Cost: unrelated changes between base and landed are attributed to the extraction, so the number is not the extraction's. In practice: an unrelated dependency bump lands in the delta.
  3. **Host measurement only** — Gain: cheapest. Cost: host bytes are not evidence about the supported platform, and the size use cases already forbid substituting them. In practice: a Windows-only regression goes unseen.
- Disposition: carried — owner: the design author; decided in `design.md` (D6) with the exact CI commands, the report artifact and the acceptance rule.
