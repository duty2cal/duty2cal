# Design Review: own-the-pdf-file-structure

Stress-test of the complete plan — proposal, delta specification and design —
before the task list is written. The grilling ran in a fresh context; the
hardening was applied after both operator questions were answered.

## Grilling branch verdicts

| # | Branch | Verdict | Evidence and disposition |
|---|---|---|---|
| 1 | Scope | finding — resolved | The delta carries exactly the proposal's two observables: four accepted file-structure spellings and one damaged-pointer refusal; the dependency split, the saving and the ADR gate are build claims routed out by decision D7. The "nothing a crew member reads changes" claim was over-broad where the design changed a malformed input's outcome; the predictor and checksum paths now mirror the oracle exactly, and the remaining differences (64 MiB bound, `/Kids` cycle, `Unreadable` detail wording) are stated with their direction. |
| 2 | Tech choices | finding — resolved | `miniz_oxide 0.9.1` is already in the lock, `default = ["with-alloc"]` and `with-alloc` are real, `decompress_to_vec_zlib_with_limit` exists, the toolchain is pinned `1.99.1`, cucumber is `0.23` with `.fail_on_skipped()`, wasm-pack is `0.15.0`. The design's TIFF-2 support and its refusal of invalid `/Predictor` values contradicted its own equivalence rule because the comparison oracle (`pdfplumber-parse 0.4.1`) depends on the same `lopdf 0.44` and passes every non-PNG predictor through; both were removed, and the previous reader's raw-deflate retry on a corrupt checksum was added. |
| 3 | Missing scenarios | finding — resolved | No happy path or refusal is missing. A corrupt-checksum fixture now joins the differential suite, because the oracle recovers such a stream and the owned reader would otherwise refuse it. The two predictor rows disappeared with the parity fix, so no scenario is owed for them. |
| 4 | Testability | finding — resolved | Strict mode (`.fail_on_skipped()`), the single-scenario command (`./run-tests.sh --name`, which refuses zero or several matches) and the real CLI driver were already concrete. Added: the `unimplemented!` stub pattern and its RED command, the exhaustive spelling enum whose unknown value fails the step, the shared helper location for `accepted_calendar()`/`normalise()`, and the `differential_tests.rs` correction — the design had called it unchanged while also promising new differential variants. |
| 5 | E2E fidelity | finding — resolved | One inventory action, one `@e2e` scenario, real subprocess driver, primary assertion on the published calendar and exit status; no HTTP-for-browser substitution and no duplicate e2e. The committed transcript printed `exit status: 0` and `normalised comparison: identical` as literals; the design now requires both values to be read from the scenario state. No `@e2e` tag is added, removed or moved. |
| 6 | Interaction Coverage | clean | The owning use case's one `## Interactions` row maps to the design's one matrix row, the same CLI/terminal interface and the same `std::process::Command` mechanism the step code implements. |
| 7 | Risk | finding — carried | The single most likely failure is Gate A: the estimate lands at 442,648 B before the replacing parser's own bytes, leaving 7–18 KB below an "≈450 KB" bound whose unit is itself approximate. Mitigation is the design's own: the prototype is the first, stop-able step, the measurement decides, and a stop reverts the proposed `arc42-docs` amendment. No user question was needed; the design records the stop rule and the owner. |
| 8 | Use-case and Persona context | finding — resolved | Goal, primary actor, trigger, extensions, relationships, preconditions and mechanism vocabulary pass the scope rejection criteria. One durable rule was stale: the one-shot measurement rule named `pdfplumber` as the stop predicate while the design moves both scripts to `lopdf`; corrected by operator decision Q1. |
| 9 | Decision ledger | clean | Proposal questions D1–D7 all resolve: D1–D5 and D7 in the design's `## Decisions`, D6 split into a decided mechanism and a carried outcome owned by `project-maintainer--release-owner`. No seed reference is owed; the proposal records "Seed path: none". |
| 10 | Visual design contract | clean | `Interface: terminal`, viewports n/a, evidence kind transcript. The opposite classification (Web UI) is tested and rejected: no markup, copy or page interaction changes, and the only browser claim is a smaller WASM module. |
| 11 | Deferrals | clean | No mandatory check is deferred. Gate A is a recorded, stop-able gate with a named owner; the private corpus's absence from CI is handled by the interface-obstacle rule with a concrete local fix. |
| 12 | Persona disposition | clean | All three personas are confirmed in `givn/personas/`; the design records a relevance and evidence line for each. The browser reviewer is justified by the bundle measurement, not by a screen. |
| 13 | Interface classification | clean | CLI/terminal is correct; the delta drives the built process, not an in-process call. |
| 14 | ADR qualification | finding — resolved | The amendment is routed `AMEND_ADR` to ADR-0008 with a complete existing-ADR check, a named cheaper home (feature spec, use case, `design.md`, code) and `must_be_shared: SUPPORTING`; no second ADR file. The non-standard `amended:` frontmatter key is now documented in both ADR READMEs instead of remaining an undocumented extension. |
| 15 | Architecture documentation | finding — resolved | Independent derivation of the twelve chapter rows (1 No, 2 Yes, 3 No, 4 Yes, 5 Yes, 6 Yes, 7 No, 8 Yes, 9 Yes, 10 Yes, 11 Yes, 12 Yes) matches `arc42.md` row for row. All twelve chapter files exist, carry real content, use Mermaid only and contain no box-drawing art or placeholders. Fixed: row 1's reason contradicted row 10; chapters 5, 8, 11, the ADR amendment and the change-level notes carried the retired predictor policy; they now match `design.md`. |
| 16 | Ubiquitous language | finding — resolved | The glossary already held **File structure**, **Owned extraction**, **Frozen supported subset** and **Test oracle**. The load-bearing word "spelling" had no row; added by operator decision Q2. The frozen-subset row's predictor sentence was corrected. |

## Operator questions

### Q1 — the one-shot measurement rule's stop predicate

- Status: closed 2026-10-29 (operator chose option A)
- Blocks: whether the owning use case's durable rule is edited in this change.
- Context: the use case for owning the duty-plan extraction
  (`own-the-duty-plan-extraction`) says the same-revision measurement is
  repeated only while the base revision's
  `crates/duty2cal-pdf/Cargo.toml` "still names `pdfplumber` in its
  dependencies". After this change the replaced layer is the file-structure
  library (`lopdf`) in `[dependencies]`; `pdfplumber` remains only in
  `[dev-dependencies]` as the comparison oracle. The design moves both
  measurement scripts to `lopdf`, and the architecture chapters already
  describe that predicate. Left unchanged, the rule would describe a condition
  that never stops being true while the scripts stop on a condition the rule
  never states.
- Options:
  1. **Generalise the predicate** — "the base's
     `crates/duty2cal-pdf/Cargo.toml` names the layer being replaced in
     `[dependencies]` (`lopdf` for this change)". Example: a later replacement
     whose base still names its removed layer measures; a base with no removed
     layer records `not applicable`. Gain: one rule serves every replacement
     and matches both scripts. Cost: one sentence of the durable use case
     changes although the proposal says its Rules are preserved.
  2. **Name `lopdf` explicitly** — identical now, but the rule must be edited
     again at the next replacement. Gain: concrete. Cost: hard-codes one crate.
  3. **Leave unchanged** — the scripts check `lopdf`, the rule names
     `pdfplumber`, which the dev-only oracle never stops naming. Gain: no edit.
     Cost: the rule contradicts the design and the architecture chapters, and
     the use-case conformance check fails.
- Resolution: option A. Chosen: generalised rule naming the replaced layer with
  `lopdf` in parentheses. Rejected: option 2 because it re-opens the durable
  rule at every replacement; option 3 because it makes the one-shot status
  underivable from the rule and leaves the change's use-case conformance
  failing.

### Q2 — "file-structure spelling" as a glossary term

- Status: closed 2026-10-29 (operator chose option A)
- Blocks: the merged permanent spec's vocabulary and the ubiquitous-language
  sign-off.
- Context: the delta's Examples column is `| spelling |`, its Given phrase is
  "a supported duty plan whose file structure uses \"<spelling>\"", the design
  uses the word throughout, and the risk register says the accepted spellings
  are spellings of constructs the previous reader already accepted. Two
  glossary rows use the word inside their contract text but none defines it.
- Options:
  1. **Add `File-structure spelling`** — "one concrete encoding of a
     file-structure construct that the owned reading accepts without widening
     the frozen subset". Example: "an incremental update" is an accepted
     spelling of the cross-reference table. Gain: one definition for the spec,
     the design, the risk register and future changes. Cost: one row plus a
     tally line; the term enters the permanent vocabulary.
  2. **Reuse "accepted form"** and fold the definition into the frozen-subset
     row. Gain: no new term. Cost: changes the feature text, the Examples
     column, the design and the risk register, and the merged permanent spec
     keeps the changed step text.
  3. **Leave informal** — gain: no edit; cost: the glossary rule blocks
     sign-off and the word stays undefined.
- Resolution: option A. Chosen: the new glossary row and tally entry.
  Rejected: option 2 because the churn reaches the permanent step text for no
  gain; option 3 because the term is load-bearing and the glossary rule
  forbids signing off without it.

## Findings resolved without a question

| Finding | Evidence | Hardening applied |
|---|---|---|
| TIFF-2 decoding and invalid-predictor refusal contradict oracle equivalence | `pdfplumber-parse 0.4.1` depends on `lopdf 0.44`; `lopdf` applies only predictors 10–15 and passes every other value through (`object.rs`, `decompress_predictor`) | Predictor row, accepted list, refused list, difference rows and D3 paragraph in `design.md` now mirror the oracle; chapters 5, 8, 11, the ADR amendment and `arc42.md` agree |
| A corrupt zlib checksum would be refused where the oracle recovers it | `lopdf`'s `decompress_zlib` retries raw deflate from byte 2 when the zlib decode produced nothing; `miniz_oxide` returns `Err(Adler32Mismatch)` | Filter row and differences table require the same retry and partial-output rule; a corrupt-checksum fixture joins the differential suite |
| `differential_tests.rs` was "unchanged" while the design promised new differential variants | `differential_tests.rs` has one test per existing builder; new variants need new comparison cases | File-table row now says "changed" and lists the cases; the corpus section and chapters 5 and 11 agree |
| New steps cannot reach the e2e module's private `accepted_calendar_path()` and `normalise()` | Both are private to `apps/cli/tests/steps/e2e/own_the_duty_plan_extraction.rs` | `apps/cli/tests/steps/support.rs` gains both as `pub(crate)`; the Files and Step Definitions rows match |
| The spelling Given had no mapping or failure mode | Spec Examples are free text | Exhaustive enum mapping; an unknown spelling fails the step and never falls back |
| No not-implemented stub pattern was named | Review instruction requires one; the design only said "`@wip` until the steps exist" | `unimplemented!` stub pattern with its recorded RED command added to Step Definitions |
| The committed transcript printed literals | `write_transcript` formats `exit status: 0` and `normalised comparison: identical` | Design requires both values from the scenario state; `DutyWorld` gains the recorded comparison outcome; the e2e row and evidence contract say so |
| `--tags @own-the-duty-plan-extraction` selects both delta scenarios, not only the outline | The tag sits on the Feature line | Wording corrected in both places |
| `Unreadable(detail)` text changes and was not stated | `SourceError::Unreadable` carries the library's message today | Differences table row added: wording only; class, two-line form, numbering and page suffix unchanged |
| Gate A's stop rule did not say what happens to the proposed `arc42-docs` edits | Every chapter marks the amendment "proposed, pending Gate A" | Gate A now says a stop reverts the amendment block and the affected statements |
| `arc42.md` row 1 marked "No" while row 10 is "Yes" | Row 1 covers chapters 1 and 10 and claimed no quality attribute changes | Reason now says chapter 01 is unaffected and QS-048 is row 10 |
| `amended:` frontmatter key was an undocumented extension | `docs/arc42/README.md` defines the five status values and no other keys | The key is documented in `docs/arc42/README.md` and `docs/arc42/adr/README.md`: it never replaces `status` and never encodes a pending state |

## Overlap disposition

`givn lint` reports one SHAPE advisory pairing the new bad-pointer scenario with
the permanent truncated-document scenario. Disposition: **variant**. A
truncated file and an out-of-file cross-reference pointer are different
malformed classes required by decision D2; both assert the same refusal class,
and neither duplicates the `@e2e` action. No scenario is merged or deleted.

## Hardening applied

- `givn/changes/own-the-pdf-file-structure/design.md`: predictor parity,
  corrupt-checksum retry, difference-table corrections (`Unreadable` wording,
  64 MiB, `/Kids`), corpus and differential-test rows, shared helper row,
  exhaustive spelling mapping, stub pattern, transcript derivation, Gate A
  revert note, tag-selection wording, glossary-impact bullets.
- `givn/specs/own-the-duty-plan-extraction/usecase.md`: one-shot measurement
  rule now names the layer being replaced, with `lopdf` in parentheses.
- `docs/arc42/12-glossary.md`: new **File-structure spelling** row, tally
  update, corrected **Frozen supported subset** predictor sentence.
- `docs/arc42/05-building-block-view.md`, `08-crosscutting-concepts.md`,
  `11-risks-and-technical-debt.md`, `adr/ADR-0008-own-the-duty-plan-extraction.md`:
  corrected to the parity policy and the checksum recovery.
- `givn/changes/own-the-pdf-file-structure/arc42.md`: rows 1, 8, 11 and 12 and
  the frozen-subset note corrected.
- `docs/arc42/README.md`, `docs/arc42/adr/README.md`: `amended:` key documented.

## Verification

- `givn lint --change own-the-pdf-file-structure` exits 2 with exactly the two
  intended `@wip` findings and the one SHAPE advisory; no new finding.
- The oracle's decoder was read from the resolved registry sources in this
  session: `pdfplumber-parse 0.4.1`'s manifest names `lopdf 0.44`;
  `lopdf 0.44.0`'s predictor and checksum code was read directly;
  `miniz_oxide 0.9.1`'s zlib and raw-deflate entry points and its
  `DecompressError` payload were read directly.
- Repository facts claimed by the design were checked: the measurement scripts'
  restore paths and `not applicable` status, the dependency check's two modes,
  the differential tolerance and corpus, the fixture builders, the runner's
  zero/one name guard and strict mode, the e2e helper visibility, the archived
  evidence path, the persona confirmations and the `2026-06.ics` UID lines.
- No scenario was removed, no `@e2e` tag was changed, and `tasks.md` was not
  written.

DESIGN-REVIEW: PASS
