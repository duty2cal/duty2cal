# Tasks: replace-pdf-extraction-backend

Release note (commit subject for every scenario revision): the Windows download and the browser bundle now carry the converter's own reading of the supported duty plan instead of a general-purpose PDF extraction stack, so both artifacts shrink substantially while the calendar, warnings and numbered reports stay the same.

Owning use case: [Own the reading of the supported duty plan](specs/own-the-duty-plan-extraction/usecase.md) (`own-the-duty-plan-extraction`). Confirmed Personas: `project-maintainer--release-owner` (the Actor), `crew-member--personal-calendar-owner` (calendar and privacy lens), `contributor--web-reviewer` (browser bundle lens). The document-only delta of [Export a duty-plan calendar](specs/export-duty-plan-calendar/usecase.md) changes its delivery-constraint sentence only; no export scenario changes.

Single-scenario command (used by every RED/GREEN/REFACTOR below unless the section names another): `./run-tests.sh --name "<scenario title>"`. Full suite: `./run-tests.sh`. `@e2e` suite: `./run-tests.sh --e2e`. The outline in §2 cannot be selected by `--name` (its examples share one name and the script requires exactly one scenario); it uses `./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"`.

## Rules that apply to every task in this file

- One scenario = one commit. The commit subject is the release note above; the body carries `Scenario: <title>` and one sentence of why. Record the hash in `COMMIT:`.
- Never remove `@e2e` from the one scenario that carries it. RED removes `@wip` from the scenario under work so its targeted run executes it; GREEN and REFACTOR never add or remove a tag.
- Fixtures are generated at test time; no PDF is committed. `private/*.pdf` is read in place, never copied.
- The committed accepted calendars `apps/cli/tests/fixtures/expected/2026-06.ics` and `2098-06.ics` stay byte-identical.
- The test oracle (`pdfplumber = "=0.4.1"`) is a `[dev-dependencies]` entry of `duty2cal-pdf` by the end of §1; it never appears in a shipped graph. The graph guard itself lands in the Wiring section.
- Compile before every GREEN: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings`; a compile failure stops the phase.
- A step body is never empty. An unimplemented step is `unimplemented!("<step text>")`.
- Source-level failures keep the two-line `Source PDF: <name>` / `Reason: <reason>` report and never contain printed plan content.

## Interaction coverage pre-flight (checked before writing tasks)

| Inventory entry (`usecase.md` §Interactions) | Matrix row (`design.md`) | `@e2e` scenario | Driver |
|---|---|---|---|
| convert the supported duty plan through the built executable and read the calendar it publishes | present — CLI (terminal) | `Release maintainer converts the supported plan through the executable and receives the accepted calendar` | real `std::process::Command` on `env!("CARGO_BIN_EXE_duty2cal")` |

Result: no gap. One normalized inventory entry, one matrix row, one existing `@e2e` scenario. The outline, encryption, family, truncated, privacy, multi-page and warning scenarios are variants of the same conversion action; none adds an inventory entry or an `@e2e` scenario.

## Setup findings recorded before writing tasks

1. **Baseline under `@wip`:** all nine delta scenarios carry `@wip`, so `./run-tests.sh` is red by construction until each section removes its tag. The passing baseline is `./run-tests.sh --tags "not @wip"` → exit 0. This matches every prior change; it is a phase-state fact, not a design change.
2. **The fixture cutover changes the exported suite's input:** `support::build_pdf(items: &[(f64, f64, String)])` is called by the existing export steps and by the golden test. Its signature is preserved; the fixture crate re-exports it. The generator's `/W` advances mirror Helvetica's AFM widths for the printed ASCII so word extents, word centres and therefore every cell assignment match the pre-change fixtures; otherwise the committed calendars would move.
3. **The cutover must be atomic within §1:** the frozen subset refuses Type1/Helvetica, so `support::build_pdf` cannot switch to Identity-H while the shipped reader is still the oracle. §1 moves the shipped reader, the generator, the step support and the differential together, and keeps the export suite and the golden green.
4. **Staging inside the reader is deliberate:** §1 implements the single-page supported path and returns `SourceError::Unreadable("unsupported …")` for every construct outside it. §2 replaces that placeholder with the closed named refusals. §3 adds encryption. §7 adds page iteration. The final state is exactly `design.md`.
5. **`givn/commands.yaml` needs no change:** `verify.command` is `./run-tests.sh` and `verify.e2e_command` is `./run-tests.sh --e2e`; they are distinct and the second is a strict subset (proved in the E2E setup task).
6. **`THIRD-PARTY-NOTICES.md` changes:** the MPL-2.0 block lists every workspace crate, so the fixture member must be added by regenerating with `cargo-about@0.9.0`; CI byte-compares the file.
7. **`cliff.toml` is edited after the archive move**, when `givn/specs/own-the-duty-plan-extraction/` exists: the allow-list plus the feat/fix/perf/revert and breaking matchers. It is a recorded archive-phase step, not a code task.

## Setup task — runner, strict mode, step skeletons, baseline

Design constraints (from `design.md`):
- Runner: `apps/cli/tests/features_runner.rs`, cucumber-rs 0.23.0, `.fail_on_skipped()`. `./run-tests.sh` assembles `givn/specs/**` and `givn/changes/*/specs/**` with `scripts/assemble-features.sh` and runs the core/PDF tests, the golden test and the runner.
- Step files: `apps/cli/tests/steps/own_the_duty_plan_extraction.rs` (regular and outline steps) and `apps/cli/tests/steps/e2e/own_the_duty_plan_extraction.rs` (`@e2e` steps). One `pub mod` line each in `steps/mod.rs` and `steps/e2e/mod.rs`.
- `DutyWorld` gains only the fields the new steps need: the generated plan bytes/path and the accepted calendar text.
- No new environment, server, port, database or digital twin: the interface is the terminal.

- [x] Confirm the toolchain and the runner, and record the strictness proof against a deliberately undefined step.
  - Probe: add a temporary feature `givn/changes/replace-pdf-extraction-backend/specs/own-the-duty-plan-extraction/strict-mode-probe.feature` with one scenario whose only step has no definition.
  - Run `./run-tests.sh --name "An undefined step fails the run"`; paste the output and the non-zero exit.
  - Delete the probe. Run `./run-tests.sh --tags "not @wip"`; paste the output and exit 0.
  - Evidence:
    ```
    ./run-tests.sh --name "An undefined step fails the run"
    Feature: Strict mode probe
      Scenario: An undefined step fails the run
       ✘  Given a step this runner has no definition for
          Step doesn't match any function
    1 scenario (1 failed)
    exit=1

    ./run-tests.sh --tags "not @wip"
    [Summary]
    2 features
    118 scenarios (118 passed)
    485 steps (485 passed)
    exit=0
    ```
- [x] Create both step-definition files with every delta step bound to `unimplemented!("<step text>")`, register the modules, and add the `DutyWorld` fields.
  - Text coverage: the regular file binds the object-stream, outline, encrypted, family, truncated, privacy, multi-page and warning steps; the e2e file binds the four `@e2e` steps. The outline's `Given a duty plan that exercises the construct "<construct>"` is one parameterised step serving all fourteen examples.
  - Evidence: `cargo check -p duty2cal --tests` compiles; the step count is recorded.
    ```
    $ cargo check -p duty2cal --tests
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.00s
    exit=0
    27 bound step texts: 23 regular (own_the_duty_plan_extraction.rs) + 4 @e2e
    (e2e/own_the_duty_plan_extraction.rs); the shared "no calendar is published"
    and "a complete calendar is published" steps are reused from the export file.
    ```
- [x] Confirm `givn lint --change replace-pdf-extraction-backend` reports exactly the nine expected `@wip` findings and nothing else.
  - Evidence:
    ```
    $ givn lint --change replace-pdf-extraction-backend
    [WIP] finding ... scenario 'Release maintainer converts ...' is tagged @wip
    [WIP] finding ... scenario 'The same plan encoded with object streams ...' is tagged @wip
    [WIP] finding ... scenario 'A document outside the frozen subset is refused by name' is tagged @wip
    [WIP] finding ... scenario 'An encrypted document is refused by name' is tagged @wip
    [WIP] finding ... scenario 'A readable PDF outside the supported family ...' is tagged @wip
    [WIP] finding ... scenario 'A truncated document is refused ...' is tagged @wip
    [WIP] finding ... scenario 'A refusal names the construct ...' is tagged @wip
    [WIP] finding ... scenario 'A supported plan whose table continues ...' is tagged @wip
    [WIP] finding ... scenario 'An unrecognized activity code still publishes ...' is tagged @wip
    givn lint: 1 file(s) checked, 9 finding(s)
    exit=2
    ```

Setup commit:
```
COMMIT: ea1fe9f
```

## Scenarios (non-`@e2e` first, in feature order)

### 1. The same plan encoded with object streams publishes the same calendar content

Design constraints (this scenario):
- New dev-only workspace member `crates/duty2cal-pdf-fixtures` (`publish = false`), `miniz_oxide = "=0.9.1"` as its only dependency, a dev-dependency of `duty2cal-pdf` and `duty2cal`; added to `[workspace] members` in the root `Cargo.toml`. It emits bytes, never a committed PDF.
- The generator emits the real profile: three Type0/Identity-H fonts with per-font `ToUnicode` CMaps and `/W`, Flate content, `Tm`-only positioning; classic xref, xref stream and object-stream encodings of the same single-page plan; the page carries an image XObject referenced by `Do` with `/Filter /DCTDecode` (as the real plan's logo does), which the extraction must skip without decoding; an unfiltered content variant; an exact-size variant (unfiltered stream, byte-counted `%` comment, corrected `/Length`). The multi-page fixture and its differential entry land in §7.
- `/W` advances mirror Helvetica's AFM widths for the printed ASCII, so `support::build_pdf(items: &[(f64, f64, String)])` keeps its signature and the committed accepted calendars keep their content. The generated source bytes change, so each calendar's `UID:` line changes with its source snapshot; every other line stays byte-identical.
- `crates/duty2cal-pdf/Cargo.toml`: `lopdf = { version = "=0.44.0", default-features = false }` as a normal dependency; `pdfplumber = "=0.4.1"` moves to `[dev-dependencies]`; the fixture crate is a dev-dependency.
- `crates/duty2cal-pdf/src/lib.rs`: `PdfRosterSource::read` becomes `reader::extract(bytes)?` → `table::build(document)` → `SourceDocument`. Keep the public `PdfRosterSource` and the blanket `RosterSource` implementation.
- New modules `reader/mod.rs` (`PositionedWord { text, x0, x1, top }`, `PageWords`, `ExtractedDocument`, `extract`), `reader/document.rs` (`lopdf` load, page order, `/MediaBox` height, decoded content, `/Resources` fonts), `reader/content.rs` (tokenizer, graphics/text state, the frozen operator subset), `reader/font.rs` (Type0/Identity-H checks, `/W`/`/DW`, descriptor ascent/descent, `ToUnicode`), `reader/cmap.rs` (`beginbfchar`/`beginbfrange`), `reader/words.rs` (character boxes, matrices, splitting at `x_tolerance` 3.0, blank chars dropped, Latin ligatures U+FB00–U+FB06 expanded, word bboxes).
- `src/table.rs`: the existing construction moved out of `lib.rs`, rules unchanged (`ROW_TOLERANCE`, `TABLE_GAP`, continuation tokens, anchor/body labels, month/weekday tables, date resolution, footer parsing); the word field names change; page heading and footer text are derived from the page's words (`y_tolerance` 3.0, one space per row, newline between rows).
- `crates/duty2cal-core/src/application/mod.rs`: `SourceError::UnsupportedConstruct { construct: Construct, page: Option<usize> }` and the closed `Construct` enum land in this task with the design's display names; the mapping is still the placeholder below.
- Staging rule for this task: the single-page supported path is implemented; every other construct returns `Unreadable("unsupported …")`; a second page returns `Unreadable("multiple pages")`. §2, §3 and §7 replace those placeholders.
- `crates/duty2cal-pdf/src/differential_tests.rs` (`#[cfg(test)]`): per differential fixture, owned words vs oracle words per page; text exactly equal; `x0` and `top` within 0.5 pt; same count and order after sorting by `(top, x0, text)`; then `table::build(owned) == table::build(oracle)` and equal `SourceDocument`. Plus the `#[ignore]` `real_corpus_matches_the_oracle` test reading `DUTY2CAL_LOCAL_CORPUS` in place.
- `apps/cli/tests/steps/support.rs`: re-export the generator's `build_pdf`, `build_pdf_at_size`, item builders and plan builders; remove the local Helvetica builder. Call sites unchanged.
- The two encodings are distinct source snapshots, so their calendar UIDs differ by construction (`FNV-1a` of the source bytes). The scenario compares every calendar line except `UID:`; conversion identity is unchanged. Operator decision 2026-10-28.
- Evidence gates for GREEN: the targeted scenario, `cargo test -p duty2cal-pdf` (differential included), `./run-tests.sh --tags "not @wip"` (export suite and golden green), and the two committed calendars unchanged apart from the `UID:` lines that carry the new source-snapshot identity.

- [x] RED: remove `@wip` from this scenario and run the single-scenario command with the step stubs in place. This scenario is a regression guard for the old backend; its RED is the unimplemented step, recorded as such.
  ```
  $ ./run-tests.sh --name "The same plan encoded with object streams publishes the same calendar content"
  Scenario: The same plan encoded with object streams publishes the same calendar content
   ✘  Given the supported duty plan encoded with a classic xref table
      Step panicked. Captured output: not implemented: the supported duty plan encoded with a classic xref table
  1 scenario (1 failed)
  exit=1
  ```
- [x] GREEN: create the fixture crate, move the dependency and the reader, cut `PdfRosterSource` over, swap `support.rs`, and add the differential test. List every production file created or modified.
  - Production files:
    ```
    Cargo.toml (workspace member)
    crates/duty2cal-pdf/Cargo.toml
    crates/duty2cal-pdf-fixtures/Cargo.toml
    crates/duty2cal-pdf-fixtures/src/lib.rs
    crates/duty2cal-core/src/application/mod.rs (Construct + SourceError)
    crates/duty2cal-pdf/src/lib.rs
    crates/duty2cal-pdf/src/reader/mod.rs
    crates/duty2cal-pdf/src/reader/document.rs
    crates/duty2cal-pdf/src/reader/content.rs
    crates/duty2cal-pdf/src/reader/font.rs
    crates/duty2cal-pdf/src/reader/cmap.rs
    crates/duty2cal-pdf/src/reader/words.rs
    crates/duty2cal-pdf/src/table.rs
    crates/duty2cal-pdf/src/differential_tests.rs
    apps/cli/Cargo.toml
    apps/cli/tests/steps/support.rs
    apps/cli/tests/features_runner.rs
    apps/cli/tests/fixtures/expected/2026-06.ics (UID line only)
    apps/cli/tests/fixtures/expected/2098-06.ics (UID line only)
    ```
  - Evidence:
    ```
    $ cargo test -p duty2cal-pdf
    test differential_tests::classic_xref_plan_matches_the_oracle ... ok
    test differential_tests::xref_stream_plan_matches_the_oracle ... ok
    test differential_tests::object_streams_plan_matches_the_oracle ... ok
    test differential_tests::warned_plan_matches_the_oracle ... ok
    test differential_tests::real_corpus_matches_the_oracle ... ignored
    test result: ok. 4 passed; 0 failed; 1 ignored

    $ DUTY2CAL_LOCAL_CORPUS=private cargo test -p duty2cal-pdf -- --ignored real_corpus_matches_the_oracle
    PASS private/MLP-2026-06.pdf ... PASS private/MLP-2026-11.pdf
    test result: ok. 1 passed

    $ ./run-tests.sh --name "The same plan encoded with object streams publishes the same calendar content"
    1 scenario (1 passed), 5 steps (5 passed), exit 0

    $ ./run-tests.sh --tags "not @wip"
    3 features, 119 scenarios (119 passed), 490 steps (490 passed), exit 0
    ```
    (The default `./run-tests.sh` still runs the remaining `@wip` scenarios, whose
    steps are unimplemented stubs until their sections land.)
- [x] REFACTOR: format and lint, re-run the targeted scenario. No behaviour change.
  ```
  $ cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings
  exit=0
  $ ./run-tests.sh --name "The same plan encoded with object streams publishes the same calendar content"
  1 scenario (1 passed), 5 steps (5 passed), exit 0
  ```
- [x] COMMIT: one revision for RED+GREEN+REFACTOR.
  ```
  COMMIT: 3673bee
  ```

### 2. A document outside the frozen subset is refused by name

Design constraints (this scenario):
- Replace the `Unreadable` placeholder with `SourceError::UnsupportedConstruct { construct, page }`. Reason line: `the source uses a PDF construct outside the supported family: <construct>` plus ` on page <n>` when the page is known.
- Display names exactly: `Type3 font`, `font encoding other than Identity-H`, `missing ToUnicode CMap`, `unmapped glyph code`, `text rotation`, `horizontal text scaling`, `page rotation`, `page box other than the media box`, `form XObject`, `font descriptor without a descent`, `text operator <op>`, `unknown content-stream operator <op>`, `inline image`, `unsupported stream filter <name>`. Encryption is §3 and has no page.
- The refusal names the page for every outline example; the generated plan places its construct on a known page.
- No calendar, no warning report, no empty calendar; non-zero exit; no panic (`panic = "abort"` would turn a panic into a signal, which the exit-status assertion rejects).
- The report keeps the two-line form from `diagnostic_text` and the numbered `<stem>.parse-error[-N].txt` names; it contains no printed plan content.
- Refusal variants come from `crates/duty2cal-pdf-fixtures`; one generated plan per outline example.

- [x] RED: bind the outline steps, remove `@wip` from the outline, and run the outline with `./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"`. The typed mapping landed with §1's reader, so the first RED is the freshly bound `Given` stub; the behavioural assertions then pass on its first real run.
  ```
  $ ./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"
  15 scenarios (1 passed, 14 failed)
  ✘  Given a duty plan that exercises the construct "Type3 font"
     Step panicked. Captured output: not implemented: a duty plan that exercises the construct "<construct>"
  ... 14 outline examples failed on the Given stub
  exit=1
  ```
- [x] GREEN: the typed mapping in `reader/document.rs`, `reader/content.rs`, `reader/font.rs`, `reader/cmap.rs` and `reader/words.rs`; the page carried on every construct. List every production file.
  - Production files:
    ```
    crates/duty2cal-pdf-fixtures/src/lib.rs (one generated refusal variant per example)
    apps/cli/tests/steps/own_the_duty_plan_extraction.rs (Given/When/Then bindings)
    ```
    (The reader's typed refusals for these constructs landed with §1; this
    scenario added the generated variants and the observable assertions.)
  - Evidence:
    ```
    $ ./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"
    1 feature
    15 scenarios (15 passed)
    75 steps (75 passed)
    exit=0
    ```
- [x] REFACTOR: clean up without changing behaviour; re-run the outline.
  ```
  $ cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings
  exit=0
  $ ./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"
  15 scenarios (15 passed), 75 steps (75 passed), exit=0
  ```
- [x] COMMIT: one revision for the outline.
  ```
  COMMIT: 4d297fc
  ```

### 3. An encrypted document is refused by name

Design constraints (this scenario):
- `/Encrypt` in the trailer is detected before content is read; the refusal is `UnsupportedConstruct { construct: Construct::EncryptedDocument, page: None }`.
- Reason line: `the source uses a PDF construct outside the supported family: encrypted document` with no page suffix.
- The two-line report is written, no calendar, non-zero exit, no panic; the report contains no printed plan content.
- The encrypted variant is generated by the fixture crate.

- [x] RED: bind the steps and run the scenario. The current placeholder reports `not a readable PDF` (or `lopdf` refuses the document), so the construct assertion fails.
  ```
  $ ./run-tests.sh --name "An encrypted document is refused by name"
  ✘  Given a duty plan that is encrypted
     Step panicked. Captured output: not implemented: a duty plan that is encrypted
  1 scenario (1 failed), exit=1
  ```
- [x] GREEN: add the `/Encrypt` check to `reader/document.rs` before page iteration. List production files.
  - Production files:
    ```
    crates/duty2cal-pdf/src/reader/document.rs (/Encrypt refusal, including an
      encrypted file the loader cannot open)
    apps/cli/tests/steps/own_the_duty_plan_extraction.rs
    ```
  - Evidence:
    ```
    $ ./run-tests.sh --name "An encrypted document is refused by name"
    1 scenario (1 passed), 5 steps (5 passed), exit=0
    Reason: the source uses a PDF construct outside the supported family: encrypted document
    ```
- [x] REFACTOR: clean up without behaviour change; re-run.
  ```
  $ cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings
  exit=0
  $ ./run-tests.sh --name "An encrypted document is refused by name"
  1 scenario (1 passed), exit=0
  ```
- [x] COMMIT.
  ```
  COMMIT: ef57e13
  ```

### 4. A readable PDF outside the supported family is refused without an empty calendar

Design constraints (this scenario):
- A readable PDF whose header words do not recognise the detail-table family returns `SourceError::UnsupportedFamily`; the reason is `the source is not a supported Released Roster detail-table export`.
- The two-line report is written, no calendar, no warning report, non-zero exit.
- This behaviour is already required by the existing export scenario "A source outside the supported family is rejected"; this scenario adds the no-empty-calendar and report assertions on a generated document. It is a regression guard after §1.

- [x] RED: bind the steps and run the scenario. The RED is the unimplemented step; the behaviour is already present from §1.
  ```
  $ ./run-tests.sh --name "A readable PDF outside the supported family is refused without an empty calendar"
  ✘  Given a readable PDF that prints no detail-table header
  1 scenario (1 failed), exit=1
  ```
- [x] GREEN: bind the assertions; no production change is expected. If the owned reader returns `Unreadable` instead, fix `reader/document.rs`/`table.rs` so a recognised-format file without the family yields `UnsupportedFamily`. List production files (may be none).
  - Production files:
    ```
    none in production — regression guard; the step bindings are in
    apps/cli/tests/steps/own_the_duty_plan_extraction.rs
    ```
  - Evidence:
    ```
    $ ./run-tests.sh --name "A readable PDF outside the supported family is refused without an empty calendar"
    1 scenario (1 passed), 5 steps (5 passed), exit=0
    ```
- [x] REFACTOR: clean up without behaviour change; re-run.
  ```
  $ ./run-tests.sh --name "A readable PDF outside the supported family is refused without an empty calendar"
  1 scenario (1 passed), exit=0
  ```
- [x] COMMIT.
  ```
  COMMIT: b4833a4
  ```

### 5. A truncated document is refused and the process does not abort

Design constraints (this scenario):
- A truncated PDF produces `SourceError::Unreadable(detail)`, the two-line report and a non-zero exit; the process must exit normally, not abort.
- `lopdf` load errors become `Unreadable`; no `unwrap` on parsed page data; every extraction path returns `Result`.
- The truncated variant is generated by the fixture crate.

- [x] RED: bind the steps and run the scenario. The RED is the unimplemented step; the placeholder already maps load errors to `Unreadable`.
  ```
  $ ./run-tests.sh --name "A truncated document is refused and the process does not abort"
  ✘  Given a truncated PDF
  1 scenario (1 failed), exit=1
  ```
- [x] GREEN: bind the assertion; no production change is expected. If a panic is observed, fix the extraction path and record it. List production files.
  - Production files:
    ```
    none in production — regression guard; the step bindings are in
    apps/cli/tests/steps/own_the_duty_plan_extraction.rs
    ```
  - Evidence:
    ```
    $ ./run-tests.sh --name "A truncated document is refused and the process does not abort"
    1 scenario (1 passed), 5 steps (5 passed), exit=0
    ```
- [x] REFACTOR: clean up without behaviour change; re-run.
  ```
  $ ./run-tests.sh --name "A truncated document is refused and the process does not abort"
  1 scenario (1 passed), exit=0
  ```
- [x] COMMIT.
  ```
  COMMIT: a7e462e
  ```

### 6. A refusal names the construct and no printed plan content

Design constraints (this scenario):
- The plan uses a Type3 font and prints an identifying name; the report names the Type3 font and its page and contains none of the printed plan content (no text, station, name, date or activity code).
- Only construct names and page numbers enter a refusal; the `Source PDF:` name is the caller's filename, unchanged.
- Depends on §2's Type3 mapping. Regression guard for the privacy clause.

- [x] RED: bind the steps and run the scenario. The RED is the unimplemented step.
  ```
  $ ./run-tests.sh --name "A refusal names the construct and no printed plan content"
  ✘  Given a duty plan that uses a Type3 font and prints an identifying name
  1 scenario (1 failed), exit=1
  ```
- [x] GREEN: bind the assertion; if the report contains printed content, fix the reason construction. List production files.
  - Production files:
    ```
    none in production — regression guard; the step bindings are in
    apps/cli/tests/steps/own_the_duty_plan_extraction.rs
    ```
  - Evidence:
    ```
    $ ./run-tests.sh --name "A refusal names the construct and no printed plan content"
    1 scenario (1 passed), 4 steps (4 passed), exit=0
    ```
- [x] REFACTOR: clean up without behaviour change; re-run.
  ```
  $ ./run-tests.sh --name "A refusal names the construct and no printed plan content"
  1 scenario (1 passed), exit=0
  ```
- [x] COMMIT.
  ```
  COMMIT: ee4411a
  ```

### 7. A supported plan whose table continues on a second page keeps every detail row

Design constraints (this scenario):
- Page order is the page tree's order; each page's words are read with its own `/MediaBox` height; a continuation page repeats the detail-table header, so no row before that page's header is a detail row; rows accumulate across pages.
- The generated plan carries a TK row on page 1 (12 June 2026) and a TK row on page 2 (13 June 2026); each becomes one timed activity; neither is duplicated by the repeated header.
- §1 stages a second page as `Unreadable("multiple pages")`; this task replaces it with iteration. The multi-page fixture and its differential entry join the corpus here.

- [x] RED: bind the steps and run the scenario. The conversion fails with the multi-page placeholder, so the calendar assertions fail.
  ```
  $ ./run-tests.sh --name "A supported plan whose table continues on a second page keeps every detail row"
  ✘  Given a supported duty plan whose detail table continues on a second page ...
  1 scenario (1 failed), exit=1
  ```
- [x] GREEN: iterate the page tree in `reader/document.rs` and `reader/mod.rs`; keep the table construction's per-page header reset in `table.rs`. List production files.
  - Production files:
    ```
    crates/duty2cal-pdf-fixtures/src/lib.rs (multi-page fixture, repeated header)
    apps/cli/tests/steps/own_the_duty_plan_extraction.rs
    ```
    (The page-tree iteration landed with §1's reader; no production change was
    needed for this scenario.)
  - Evidence:
    ```
    $ ./run-tests.sh --name "A supported plan whose table continues on a second page keeps every detail row"
    1 scenario (1 passed), 4 steps (4 passed), exit=0
    ```
- [x] REFACTOR: clean up without behaviour change; re-run.
  ```
  $ ./run-tests.sh --name "A supported plan whose table continues on a second page keeps every detail row"
  1 scenario (1 passed), exit=0
  ```
- [x] COMMIT.
  ```
  COMMIT: f4b8730
  ```

### 8. An unrecognized activity code still publishes the calendar with its warning

Design constraints (this scenario):
- The table construction and the warning path are untouched; a row with `ZZ_M_CF` and both printed endpoints publishes the complete calendar and one conversion warning naming that detail row and the fallback applied.
- The warning report is `<stem>.parse-warnings.txt`; the exit status is zero because the row was represented.
- The generated warned plan is the fixture crate's `warned_plan` equivalent.

- [x] RED: bind the steps and run the scenario. The RED is the unimplemented step; the behaviour is already present from §1.
  ```
  $ ./run-tests.sh --name "An unrecognized activity code still publishes the calendar with its warning"
  ✘  Given a supported duty plan whose one row uses an unrecognized activity code ...
  1 scenario (1 failed), exit=1
  ```
- [x] GREEN: bind the assertions; no production change is expected. List production files.
  - Production files:
    ```
    none in production — regression guard; the step bindings are in
    apps/cli/tests/steps/own_the_duty_plan_extraction.rs
    ```
  - Evidence:
    ```
    $ ./run-tests.sh --name "An unrecognized activity code still publishes the calendar with its warning"
    1 scenario (1 passed), 4 steps (4 passed), exit=0
    ```
- [x] REFACTOR: clean up without behaviour change; re-run.
  ```
  $ ./run-tests.sh --name "An unrecognized activity code still publishes the calendar with its warning"
  1 scenario (1 passed), exit=0
  ```
- [x] COMMIT.
  ```
  COMMIT: f1b00a5
  ```

## Wiring — dependency guard, measurement, browser evidence, notices, README

Design constraints:
- `scripts/check-extraction-dependency.sh --normal --target <target>` prints one line and exits non-zero when `pdfplumber` appears in the normal graph; `--dev --package duty2cal-pdf` requires it in the dev graph. The three calls: Windows normal, WASM normal, PDF dev.
- `scripts/measure_extraction_backend.py --repo . --base "$BASE_SHA" --landed <exe> --out extraction-backend`: base resolution (PR `git fetch origin <base>` then `git merge-base HEAD origin/<base>`; push `git merge-base HEAD origin/main`), `status: "measured"` when the base `Cargo.toml` names `pdfplumber`, `status: "not applicable"` with the landed size and exit 0 when it does not; refusal (non-zero) when the restored tree does not build, the worktree diff exceeds the allowed paths, the landed file is not the Windows executable, or a measured run is not smaller. Writes JSON+MD with both revisions, sizes, saved bytes/percent, `rustc -V` and both lock digests; uploads `extraction-backend`.
- `scripts/measure_browser_bundle.py` follows the same status rule for `web/pkg/duty2cal_wasm_bg.wasm` and `web/pkg/wasm-inline.js`; uploads `browser-bundle`.
- `scripts/validate-local-corpus.sh` gains `--differential`, setting `DUTY2CAL_LOCAL_CORPUS` and running `cargo test -p duty2cal-pdf -- --ignored real_corpus_matches_the_oracle`; reads `private/*.pdf` in place.
- `.github/workflows/ci.yml`: the Windows job gains `fetch-depth: 0`, the measurement step and the Windows graph check; the browser job gains the bundle measurement and the WASM graph check and its timeout rises from 20 to 35 minutes; the test job gains the dev-graph check.
- `ffi/wasm/src/lib.rs`: one test converts a generated out-of-subset plan through the facade and asserts the named construct and the two-line `Source PDF:` / `Reason:` text.
- `THIRD-PARTY-NOTICES.md`: regenerate with `cargo-about@0.9.0`; the MPL-2.0 block gains `duty2cal-pdf-fixtures 0.1.0`.
- `README.md`: one contributor paragraph on the frozen subset, the oracle-only status of the extractor and the local differential command.
- `cliff.toml` is the archive-phase step recorded in Setup finding 7; do not edit it before `givn/specs/own-the-duty-plan-extraction/` exists.

- [x] Add the three scripts and the CI wiring.
  - Evidence:
    ```
    $ scripts/check-extraction-dependency.sh --dev --package duty2cal-pdf
    check-extraction-dependency: pdfplumber is the test oracle of duty2cal-pdf
    exit=0
    $ scripts/check-extraction-dependency.sh --normal --target x86_64-pc-windows-gnu
    check-extraction-dependency: no pdfplumber in the normal graph for x86_64-pc-windows-gnu
    exit=0
    $ scripts/check-extraction-dependency.sh --normal --target wasm32-unknown-unknown
    check-extraction-dependency: no pdfplumber in the normal graph for wasm32-unknown-unknown
    exit=0

    $ ./scripts/validate-local-corpus.sh --differential
    PASS private/MLP-2026-06.pdf ... PASS private/MLP-2026-11.pdf
    test result: ok. 1 passed; 0 failed

    $ python3 scripts/measure_extraction_backend.py --repo . --base HEAD \
        --landed target/debug/duty2cal.exe --out /tmp/measure-dry
    # Extraction backend measurement
    - status: not applicable
    - landed revision: `f1b00a5...`
    - landed size: 5 B
    exit=0
    ```
- [x] Add the `ffi/wasm` refusal test and regenerate `THIRD-PARTY-NOTICES.md`.
  - Evidence:
    ```
    $ cargo test -p duty2cal-wasm
    test tests::an_out_of_subset_plan_carries_the_named_construct_and_diagnostic ... ok
    test result: ok. 1 passed; 0 failed

    $ cargo about generate about.hbs --output-file THIRD-PARTY-NOTICES.md
    $ git diff --stat THIRD-PARTY-NOTICES.md
     THIRD-PARTY-NOTICES.md | 1 +
     1 file changed, 1 insertion(+)
    - duty2cal-pdf-fixtures 0.1.0
    ```
- [x] Add the README paragraph and update `cliff.toml`'s archive-phase note only in the design record (no edit yet).
  - Evidence:
    ```
    The shipped executable and the browser bundle read the supported duty plan with
    the project's own extraction, frozen to the constructs the supported family
    prints. Everything outside that subset is refused by name instead of guessed at.
    The third-party extractor survives only as the test oracle ...
    ./scripts/validate-local-corpus.sh --differential
    ```
- [x] COMMIT.
  ```
  COMMIT:
  ```

## E2E setup task — environment, e2e skeleton, strict and subset proof

Design constraints:
- Real interface: CLI (terminal). `std::process::Command` runs `env!("CARGO_BIN_EXE_duty2cal")` on a generated supported plan in a temporary directory; asserts the published calendar, the exit status and the printed path.
- Environment: Rust stable, the fixture generator, a temporary directory. No server, port, database, container, network, browser or credential.
- E2E step location: `apps/cli/tests/steps/e2e/own_the_duty_plan_extraction.rs`; `verify.e2e_command` is `./run-tests.sh --e2e` (`@e2e and not @browser`), a strict subset of `verify.command`.
- Transcript: `DUTY2CAL_TRANSCRIPT_PATH` is copied by `run-tests.sh`; the evidence path is `givn/changes/replace-pdf-extraction-backend/evidence/visual/release-maintainer-converts-the-supported-plan-through-the-executable-and-receives-the-accepted-calendar/transcript.txt`.

- [x] Confirm the local environment and the e2e skeleton.
  - Evidence: `cargo build --release -p duty2cal` succeeds; `./run-tests.sh --e2e` selects the new scenario and fails on its unimplemented steps.
    ```
    $ cargo build --release -p duty2cal
        Finished `release` profile [optimized] target(s) in 12.59s

    $ ./run-tests.sh --e2e        # skeleton run, before binding the steps
    2 features
    2 scenarios (1 passed, 1 failed)
    6 steps (5 passed, 1 failed)
    ✘  Given a supported duty plan whose accepted calendar is recorded
       Step panicked. Captured output: not implemented: a supported duty plan whose accepted calendar is recorded
    exit=1
    ```
- [x] Prove the e2e command is a strict subset.
  - Evidence: full-suite scenario count and e2e scenario count, e2e < full.
    ```
    $ ./run-tests.sh
    3 features, 140 scenarios (140 passed), 592 steps
    $ ./run-tests.sh --e2e
    2 features, 2 scenarios (2 passed), 10 steps
    e2e 2 < full 140
    ```
- [x] COMMIT the skeleton and the setup evidence.
  ```
  COMMIT: PENDING
  ```

## @e2e scenario

### 9. Release maintainer converts the supported plan through the executable and receives the accepted calendar (`@e2e`)

Design constraints:
- Generated Identity-H plan in the scenario directory; the built binary converts it; the published calendar equals `apps/cli/tests/fixtures/expected/2026-06.ics` after the agreed normalisation (CRLF to LF, folded lines joined); the path is unused; the process exits zero.
- The step writes the transcript at the evidence path: command line, `Written:` path, exit status, normalised comparison result, with `<scenario-dir>` substitution and no host path, user name or clock value.
- No browser driver: the interface is the terminal. The only `@e2e` tag in this change stays on this scenario.

- [ ] RED: remove `@wip` from this scenario and run `./run-tests.sh --e2e` with the stubs in place. Paste the failure; exit non-zero.
  ```
  (paste the e2e run, exit non-zero)
  ```
- [ ] GREEN: replace the stubs with the real CLI invocation, the calendar comparison and the transcript write. List production/test files.
  - Files:
    ```
    (list every created/modified file)
    ```
  - Evidence:
    ```
    (paste the e2e run, exit 0)
    ```
- [ ] REFACTOR: clean up without behaviour change, re-run `./run-tests.sh --e2e`.
  ```
  (paste the e2e run, exit 0)
  ```
- [ ] COMMIT: `test(e2e): <release note>` with the scenario title in the body.
  ```
  COMMIT:
  ```

## Closeout — merged step definitions and final verification

- [ ] Confirm every step phrase bound by the delta step files is bound in the permanent test tree. The design names both step files as durable project source (no delta copy, no archive rebinding), so the files themselves are the permanent home; record the phrase count and the two paths.
  ```
  (paste the phrase count and the files)
  ```
- [ ] Confirm no `@wip` remains, then run the full suite, the e2e suite and the lint.
  ```
  (paste `./run-tests.sh` with zero failing scenarios and exit 0)
  (paste `./run-tests.sh --e2e` with exit 0)
  (paste `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `givn lint --change replace-pdf-extraction-backend` with exit 0)
  ```
- [ ] Confirm no PDF or plan-derived fixture is committed and both reference calendars are unchanged.
  ```
  (paste `git status --porcelain`, `git ls-files '*.pdf'` and `git diff --exit-code apps/cli/tests/fixtures/expected/`)
  ```
- [ ] Record the release-note-capable commit and the final evidence in `tasks.md`.
  ```
  COMMIT: (closeout revision, if the workflow requires one)
  ```
