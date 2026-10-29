# Proposal: own-the-pdf-file-structure

## Release note

The converter reads the supported duty plan's file structure itself, so the Windows download and the browser bundle no longer carry a third-party PDF library while every published calendar, warning, report and refusal stays the same.

## Use-Case Context

- Use-case ID: `own-the-duty-plan-extraction` — the existing permanent leaf whose one capability this change extends. Its Goal, Actors, guarantees, Rules and inventory are preserved; no new use case and no new capability is created.
- Ideation topic: none — this change has no ideation seed. It takes up the reopen condition that ADR-0008 and TD-030 already record for the file-structure layer inside that use case.
- Seed path: none
- Confirmed Personas: `project-maintainer--release-owner`, `crew-member--personal-calendar-owner`, `contributor--web-reviewer` — all three exist in `givn/personas/`; none is invented or promoted. The release owner decides and answers for the frozen subset, the prototype gate and the measured saving; the crew member is the accuracy and privacy lens for the calendar, the refusal text and the download; the web reviewer is the lens for the browser bundle and the unchanged browser route.

### How this change sits inside the owning use case

- The use case's Rule "the dependency split is mechanical" already requires that the release and browser graphs carry no third-party PDF extractor and that the test graph keeps the oracle. This change makes that split stronger: the shipped graphs carry no third-party PDF library at all, and only a pure-Rust decompressor remains in the reading path.
- The use case's Rule "measurement is on the shipped artifacts" already forbids host numbers and the published binary as evidence. This change reuses the two same-revision measurement commands the preceding change created; it invents no new baseline.
- The use case's Rule about the frozen subset is unchanged: no construct is added to it and none is renamed. The change replaces *who reads* the already-frozen file-structure constructs, not what is supported.
- ADR-0008's reopen condition and TD-030's resolution are decided outside this change: the `arc42-docs` artifact carries the ADR amendment, TD-030's closure and the glossary revisions. This proposal only requires the gate and states that it is not met yet.

## Problem / Opportunity

Both shipped artifacts still compile a general-purpose PDF object and cross-reference library, although the one supported duty plan uses a narrow, stable slice of it and the rest of the reading is already project code.

Measured host attribution at this revision (`cargo bloat --release -p duty2cal --crates`; host `.text`, not shipped evidence):

| Crate | `.text` |
|---|---:|
| `lopdf` (the file-structure library) | 140.4 KiB |
| `sha2` (its hashing, reached only through it) | 30.2 KiB |
| `aes` (its encryption stack, reached only through it) | 13.1 KiB |
| `weezl` | 8.3 KiB |
| `stringprep` | 3.5 KiB |
| `hashbrown` | 3.2 KiB |
| `rand` + `chacha20` + `getrandom` | ≈4.8 KiB |
| `duty2cal-pdf` (the project's own extraction) | 62.9 KiB |
| `miniz_oxide` (Flate, would remain) | 20.4 KiB |
| `std` | 229.5 KiB |

The library's host cost was already measured for ADR-0008 as 483,456 B over the no-PDF floor (779,016 B against 2,233,584 B). No feature of it removes that stack: `default-features = false` still compiles its encryption and encoding layers.

The cost moves with every update of the library rather than with the converter. Commit `67f4a2e` "chore(deps): keep lopdf at 0.44.0" reverted a bump because 0.45.0 and 0.46.0 make a Brotli decompressor a non-optional dependency, growing the Windows GNU executable from 841,728 B to 1,038,336 B (+196,608 B, +23.4%). The project is pinned to an old release of a library it does not need in full.

What the supported plan actually exercises is a short list: classic cross-reference tables, cross-reference streams, object streams, Flate content, the page tree with its inherited resources and boxes, and `/MediaBox` plus `/Resources` lookups. The interpretation on top of that — content tokenizer, text state, Type0/Identity-H fonts, `/W` advances, `/ToUnicode` CMaps, glyph boxes, word assembly and every named refusal — is already project code in `crates/duty2cal-pdf/src/reader/`.

Impact: a crew member's download and the browser's parse cost carry a general PDF stack for one narrow roster layout, and any update of that stack can enlarge what they receive without a single conversion rule changing.

## Proposed Solution

- The project reads the supported plan's file structure itself, inside the same reading boundary that already produces positioned words. What the project reads is the object syntax, the cross-reference table or stream, object streams, the trailer, the page tree with inherited boxes and resources, and the decoding of Flate streams. The detail-table construction and every conversion rule stay untouched.
- The third-party PDF library leaves the Windows executable's and the browser bundle's dependency graphs. The only third-party piece left in the reading path is a pure-Rust Flate decompressor — no native code, no companion runtime, no network.
- The comparison oracle is unchanged and remains test-only, as it is today. It keeps its own copy of the PDF library inside the test graph; that copy is never shipped.
- Nothing a crew member reads changes. For the same source snapshot the calendar text, the conversion warnings, the detail-row diagnostics, the numbered warning and failure reports, the calendar refusals, the no-clobber publication, the privacy rules and the no-network rule are the same as today. A plan that converts today converts to the same accepted calendar; a construct outside the frozen subset is refused by the same name, naming the source PDF, the construct and, when one is known, the page; a document that is not a readable PDF keeps the unreadable refusal and never panics.
- Equivalence is judged against the oracle before anything is accepted: the owned reading's positioned words match the oracle's word text exactly and its left edge and top within 0.5 point over the generated corpus, the shared table construction then yields the same table and source document, and the six supplied plans convert identically in the opt-in local differential.
- The dependency split is checked mechanically: neither the Windows nor the browser normal graph names a third-party PDF library or extractor, while the test graph still carries the oracle. A later revision cannot reintroduce one silently.
- The saving is measured where it ships — the Windows GNU executable and the browser bundle — against the same revision rebuilt with the previous file-structure layer and lock restored. A non-positive saving rejects the replacement. Host numbers are not evidence, and the published binary is not the baseline.
- Before any of this lands, the gate ADR-0008 and TD-030 record must be met: a measured host prototype at or below the ADR's [INFERENCE] ≈450 KB upper bound, with a green differential over the six-file corpus. That gate is **not** met by this proposal and is not claimed; the design makes it the first, stop-able step, and the ADR amendment itself belongs to the `arc42-docs` artifact.
- Unchanged: the single-file static executable, the Windows GNU cross-build on Linux/CI, the browser conversion route, the support statement, the 2,097,152-byte budget and the public copy of every surface.

## Capability Routing

`givn spec route --change own-the-pdf-file-structure` reports no signal — the top candidates tie at 13.00 — and prints no ranked list, so no recommendation column is recorded. This table is the declaration `givn check review` compares against the delta.

| Proposed capability | Decision | Rationale |
|---|---|---|
| own-the-duty-plan-extraction | `EXTEND own-the-duty-plan-extraction` | The actor goal is already this capability's: the release maintainer freezes the supported subset, proves the delivered calendar, warnings and diagnostics equivalent, refuses every construct outside the subset by name, keeps the dependency split mechanical and measures the shipped artifacts. The file structure is a layer *inside* that reading, so a second capability would be component-shaped (`own-the-pdf-file-structure` names a PDF layer, not an actor goal) and would either duplicate the conversion action or introduce a second inventory action that does not exist. The use case's own Rules already name the dependency split and the shipped-artifact measurement, so the extension needs no new use case, no new capability and no inventory row. |

No consumer action is added: the release maintainer still converts a plan through the executable and the browser visitor still uses the page. The owning use case's `## Interactions` inventory therefore stays exactly as it is, and the delta adds no `@e2e` scenario — its scenarios are variants and failure edges of the one already-declared action.

## Out of Scope

- Any change to conversion rules: activity-code meanings, date and timing rules, travel pairing, calendar representation, warning and error semantics for detail rows, and the report formats.
- Widening or renaming the frozen supported subset (R-077). No construct is added, removed or renamed; the closed refusal vocabulary keeps its names.
- Encryption: an encrypted document stays a named refusal; the project implements no decryption.
- Removing or upgrading the test oracle, or removing the PDF library from the test graph; the oracle keeps its own copy.
- A general-purpose PDF library, a PDF writer, or any native C/C++ dependency.
- A size ceiling or a release refusal on size, other size levers, and the build profile.
- Publishing a release, its archive, packaging or platform set; the browser page's copy, design, interaction or save behaviour; the site publication.
- Committing a real plan or any plan-derived fixture. The private corpus stays local, gitignored and opt-in.
- The ADR-0008 amendment, TD-030's closure, the arc42 chapters and the glossary revisions — the `arc42-docs` artifact owns them. This change supplies their evidence.
- The one-shot nature of the size measurement after landing (the standing 2,097,152-byte budget remains the only bound).

## Open Questions

Each question is resolved by an entry of the same ID in the design's Decisions ledger, or is explicitly carried with an owner. The measured evidence is named where it exists; no figure is invented, and every `[INFERENCE]` stays labelled.

### D1 — What remains of the third-party reading path?

- Question: With the file-structure library gone, does the project depend directly on a small pure-Rust Flate decompressor, keep a decompression crate that brings more than deflate, or write its own inflate?
- Options:
  1. **Depend on `miniz_oxide` directly** — the crate is already in the tree (through the fixtures crate and, today, through the library's own Flate path); `=0.9.1` is pinned by both. Gain: one small pure-Rust crate with `default-features = false` plus `with-alloc`, no `crc32fast`/gzip wrappers, wasm-safe, and 20.4 KiB of host `.text` today. Cost: the shipped reading path still has one third-party crate in it, so "no third-party code at all" is not what this change claims. In practice: a Flate content stream decodes through `decompress_to_vec_zlib_with_limit`.
  2. **Keep `flate2`** — the library's current Flate front end. Gain: no new direct dependency; the sync/async wrappers and the zlib/gzip/raw variants are available. Cost: `crc32fast` and the extra wrappers ship for a decoder that only needs zlib, and the crate is an indirect dependency rather than a declared one. In practice: ~2 more crates in the shipped graph than option 1 for no conversion benefit.
  3. **Write own inflate** — Gain: the reading path would carry no third-party crate. Cost: a second parser to own and fuzz with no measured saving; ADR-0008's estimate for the whole self-owned cut was [INFERENCE] ≈400–450 KB and already assumed a decompressor crate. In practice: the largest new risk surface in the change would be the least related to the roster.
- Disposition: answered — design D1 selects option 1 (`miniz_oxide = { version = "=0.9.1", default-features = false, features = ["with-alloc"] }`), with a bounded decode.

### D2 — What does the owned reading do with a damaged or unusual file structure?

- Question: When the cross-reference pointer lies outside the file, a section cannot be parsed, or a stream's declared length runs past the end, does the owned reading refuse the document as unreadable, reconstruct the cross-reference by scanning the file, or attempt a full repair?
- Options:
  1. **Refuse as unreadable, no repair** — the document is not a readable PDF for this reader; the two-line report keeps its shape and no calendar is published. Gain: today's observable behaviour on the supported plan and every file the frozen subset covers (the current reader follows `startxref`, then `/Prev`, with `/XRefStm`, and has no reconstruction fallback), the smallest parser, and no invented interpretation of a broken file. Cost: a real plan whose cross-reference is damaged does not convert — today it does not convert either, so nothing regresses; if a real plan ever needs repair it becomes a named decision with evidence. In practice: a truncated file keeps the unreadable refusal it already gets.
  2. **Bounded reconstruction** — scan for `N G obj` markers when the cross-reference is unusable. Gain: some damaged files would convert. Cost: size and a new interpretation the current reader never makes — documents that are refused today would convert after the change, which is a behaviour change in the permissive direction, and a reconstructed object set can differ from the trailer's intent. In practice: a file that reports "not a readable PDF" today would suddenly produce a calendar, possibly a partial one.
  3. **Full repair** — Gain: the broadest tolerance. Cost: a repair engine is the part of a PDF library the project explicitly declined to own, and it makes the equivalence claim unprovable. In practice: the project owns xref repair for every future file.
- Disposition: answered — design D2 selects option 1 and enumerates the accepted file-structure spellings (classic table, cross-reference stream with predictors, incremental update, object stream, direct or indirect stream length, hybrid `/XRefStm`) that keep a supported document readable.

### D3 — Does the owned reading need a new named refusal?

- Question: The owned reading now decides constructs the library used to decide. Does it add refusal names for them (an unsupported predictor, a damaged structure, an unparsable section), or keep the closed vocabulary the frozen subset already defines?
- Options:
  1. **Keep the closed vocabulary** — a stream filter outside the subset keeps `unsupported stream filter`; every structural failure keeps the unreadable refusal; no `Construct` variant is added, removed or renamed. Gain: the refusal contract, the report wording and every refusal scenario stay valid; R-077's rule that widening or renaming the subset is an ADR change holds. Cost: a structural failure is reported as "not a readable PDF" rather than by a construct name, which is what happens today for the same files. In practice: a file with a wrong cross-reference pointer is reported as not readable, byte-for-byte the same class of refusal as now.
  2. **Add names for each new decision** — Gain: a finer refusal vocabulary. Cost: new refusal names are a subset change with its own ADR and differential evidence; the same file would report differently before and after the change, breaking the "observable behaviour unchanged" rule. In practice: a document that reads "not a readable PDF" today would read "unsupported predictor" tomorrow.
- Disposition: answered — design D3 keeps option 1 and publishes the refusal mapping: which construct each existing name still covers, and which structural failures map to the unreadable refusal.

### D4 — Does the oracle's own copy of the library stay?

- Question: The oracle is a third-party extractor that itself depends on the PDF library. Does the test graph keep that copy, is it vendored away, or is the oracle dropped?
- Options:
  1. **Keep it, test-only** — the dev graph keeps the extractor and, transitively, the PDF library; the normal graphs of the Windows and browser targets name neither. Gain: equivalence evidence is unchanged and the differential keeps its judge; the dependency-split check already has this two-mode shape. Cost: `Cargo.lock` and the third-party notices keep the library, so a reader of the lock sees it — a fact the check and the glossary must state rather than hide. In practice: the check's dev mode still requires the oracle, and its normal mode refuses both names.
  2. **Vendor a table-stubbed fork** — rejected by ADR-0008 with its measurement (1,642,384 B against 2,233,584 B) and it removes nothing from the shipped graph. In practice: a fork to maintain for no shipping benefit.
  3. **Drop the oracle** — Gain: no third-party PDF code anywhere. Cost: no equivalence evidence for a change whose whole acceptance is equivalence; the use case's Preconditions require the oracle in the test graph. In practice: the differential disappears.
- Disposition: answered — design D4 selects option 1 and states the check's two modes, the lock and notices consequence.

### D5 — Which measurement evidence, and which command?

- Question: The saving must be measured on the shipped artifacts against the same revision with the previous layer restored. Do the two existing same-revision commands get reused with their base-manifest predicate switched, is a third command added, or is the measurement taken on the host?
- Options:
  1. **Reuse both existing commands, predicate switched** — `scripts/measure_extraction_backend.py` and `scripts/measure_browser_bundle.py` already restore `crates/duty2cal-pdf/` and `Cargo.lock` from the base revision, refuse a difference outside those paths, require the landed artifact to be smaller, and record `not applicable` once the base no longer carries the previous layer. Only their base-manifest predicate changes (from the extractor to the removed library) and their wording. Gain: no new command, no new CI job, the same one-shot rule and report fields. Cost: the script names still say "extraction backend" although the compared layer is now the file-structure library. In practice: the Windows job's uploaded `extraction-backend.md` reads `status: measured` with positive saved bytes.
  2. **A third command** — Gain: a name that matches the compared layer. Cost: a third copy of the worktree/restore/compare logic and a third CI job, diverging from two that already work. In practice: three scripts that must be kept in step.
  3. **Host measurement** — Gain: cheapest and fastest. Cost: host bytes are not evidence about the Windows executable or the browser bundle; the use case forbids substituting them. In practice: a Windows-only regression is invisible.
- Disposition: answered — design D5 selects option 1 and records both commands, their CI jobs, their acceptance rule and the one-shot status.

### D6 — What is the entry gate, and who accepts it?

- Question: ADR-0008 and TD-030 reopen the self-owned file structure only "with a measured prototype at or below the [INFERENCE] ≈450 KB upper bound and a green differential over the six-file corpus". Is that gate satisfied before implementation, after landing, or not at all — and who decides?
- Options:
  1. **Prototype gate before implementation, decided by the release maintainer** — the first design step builds a host prototype of the owned reading behind the existing entry point, measures the host release executable, runs the six-file differential, and stops the change if the bound is exceeded. Gain: the change honours the recorded reopen condition instead of asserting it; a negative measurement costs one prototype rather than a full replacement. Cost: a stop-able first step and an ADR amendment that can conclude "not reopened". In practice: a 470 KB host prototype ends the change with the measurement recorded.
  2. **Land first, measure after** — Gain: no early stop. Cost: the ADR's own reopen condition is bypassed, which is exactly what the condition forbids; a negative result would leave the library removed with no way back that the record justifies. In practice: the record and the code disagree.
  3. **Not at all** — Gain: none. Cost: an accepted ADR is contradicted. In practice: TD-030 is closed without its own reopen condition.
- Disposition: carried — owner: `project-maintainer--release-owner`; the evidence is produced by the design's prototype gate and the ADR-0008 amendment is decided in the `arc42-docs` artifact. The gate is **not met** at proposal time and this proposal does not claim it.

### D7 — Does the change need a new consumer action?

- Question: Does removing the file-structure library add an interaction a consumer performs through the real interface — a new command, a new option, a new page action — or does the existing inventory cover it?
- Options:
  1. **No new action** — the release maintainer converts a plan through the executable exactly as before; the browser visitor takes the offered calendar exactly as before. Gain: the owning use case's inventory stays at one entry, no `@e2e` scenario is duplicated, and no flag or output changes. Cost: the change's own evidence is the existing scenarios plus the differential, the dependency check and the measurement. In practice: the delta adds variants and failure edges of the one declared action, not a second one.
  2. **A new build-evidence action** — Gain: the dependency split and the size measurement would appear in Gherkin. Cost: they are build-only claims, not conversion behaviour; a second inventory entry would need a permanent `@e2e` scenario that drives a script rather than the product, and the project's existing convention keeps build claims as scripts, CI jobs and documentation-level features. In practice: the use case would declare a "consumer action" no consumer performs.
- Disposition: answered — design D7 selects option 1: the inventory is untouched, the delta adds no `@e2e`, and the build claims stay with the dependency check, the measurement commands and their CI jobs.
