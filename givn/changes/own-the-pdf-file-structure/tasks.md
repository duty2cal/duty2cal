# Tasks: own-the-pdf-file-structure

Release note (commit subject for every scenario revision): the converter reads
the supported duty plan's file structure itself, so the Windows download and the
browser bundle no longer carry a third-party PDF library while every published
calendar, warning, report and refusal stays the same.

Owning use case: [Own the reading of the supported duty plan](specs/own-the-duty-plan-extraction/usecase.md)
(`own-the-duty-plan-extraction`). Confirmed Personas:
`project-maintainer--release-owner` (the Actor and the Gate A ruler),
`crew-member--personal-calendar-owner` (calendar, refusal text and privacy
lens), `contributor--web-reviewer` (browser bundle lens).

Commands used below: single scenario `./run-tests.sh --name "<scenario title>"`;
full suite `./run-tests.sh`; `@e2e` suite `./run-tests.sh --e2e`. The delta's
Scenario Outline shares one name and the script refuses a name that selects
several scenarios, so the outline is targeted with
`./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"` while only
it has lost `@wip` — the same convention the preceding extraction change used.
A passing baseline before the first scenario is
`./run-tests.sh --tags "not @wip"`.

## Rules that apply to every task in this file

- One scenario = one commit. The subject carries the release note; the body
  carries `Scenario: <title>` and one sentence of why. Record the hash in
  `COMMIT:`.
- Never add, remove or move an `@e2e` tag. The delta adds no `@e2e` scenario;
  the capability's existing one is re-run in the e2e task.
- Fixtures are generated at test time; no PDF is committed. `private/*.pdf` is
  read in place, never copied. The committed accepted calendars stay
  byte-identical.
- A step body is never empty. An unimplemented step is
  `unimplemented!("<step text>")`.
- Compile before every GREEN: `cargo fmt --all --check && cargo clippy
  --workspace --all-targets -- -D warnings`.
- No `unsafe`; every parse step returns `Result`; no `unwrap` on parsed data;
  no indexing without a bounds check.
- Source-level failures keep the two-line `Source PDF: <name>` /
  `Reason: <reason>` report, the numbered report and the page suffix, and never
  contain printed plan content.
- The comparison oracle (`pdfplumber = "=0.4.1"`) is a `[dev-dependencies]`
  entry throughout; it never appears in a shipped graph.

## Interaction coverage pre-flight (checked before writing tasks)

| Inventory entry (`usecase.md` §Interactions) | Matrix row (`design.md`) | `@e2e` scenario | Driver |
|---|---|---|---|
| convert the supported duty plan through the built executable and read the calendar it publishes | present — CLI (terminal) | `Release maintainer converts the supported plan through the executable and receives the accepted calendar` (permanent spec) | real `std::process::Command` on `env!("CARGO_BIN_EXE_duty2cal")` |

Result: no gap. One normalized inventory entry, one matrix row, one existing
`@e2e` scenario. The delta's two scenarios are variants and failure edges of
that action, so neither adds an inventory entry nor an `@e2e` scenario.

## Setup findings recorded before writing tasks

1. **Baseline under `@wip`:** both delta scenarios carry `@wip`, so
   `./run-tests.sh` is red by construction until each section removes its own
   tag. `./run-tests.sh --tags "not @wip"` is the passing baseline.
2. **The outline needs a tag filter, not a name:** its four examples share one
   name, and `run-tests.sh` refuses a name that selects zero or more than one
   scenario. With only the target scenario untagged, the filter above selects
   exactly it.
3. **`givn/commands.yaml` needs no change:** `verify.command` is
   `./run-tests.sh`, `verify.e2e_command` is `./run-tests.sh --e2e`; they are
   distinct and the second is a strict subset.
4. **Gate A is stop-able:** the two scenarios are the characterisation harness
   and are committed first, against the current reader. Gate A then replaces
   the reader and re-runs them; a host measurement above the recorded bound
   stops the change before any production commit.
5. **The oracle shares the previous decoder:** `pdfplumber-parse 0.4.1` depends
   on `lopdf 0.44`, so its predictor and checksum behaviour is the comparison
   target. The owned filter mirrors it: PNG predictors 10–15 applied, every
   other value passed through, raw-deflate retry on a failed zlib checksum.
6. **`Cargo.lock` keeps `lopdf`** through the oracle's dev graph;
   `THIRD-PARTY-NOTICES.md` does not move and is checked by the existing drift
   job.
7. **Every new phrase is bound in the permanent step file:**
   `apps/cli/tests/steps/own_the_duty_plan_extraction.rs` is the durable home,
   so the archive binding check finds all four phrases.
8. **Lint disposition for the end-to-end reference:** decision D7 makes the
   capability's permanent `@e2e` scenario — not a delta scenario — the
   end-to-end evidence for this change, so `givn lint` reports
   "@e2e scenarios referenced in tasks.md but none found in specs/". No tag was
   removed or moved; the reference is the permanent scenario re-run below. The
   warning is expected at every stage of this change and is dispositioned as
   `variant` evidence of the one declared action, not as a missing scenario.

## Setup task — runner, strict mode, baseline

Design constraints (from `design.md`):
- Runner: `apps/cli/tests/features_runner.rs`, cucumber-rs 0.23,
  `.fail_on_skipped()`; `./run-tests.sh` assembles `givn/specs/**` and
  `givn/changes/*/specs/**` with `scripts/assemble-features.sh`.
- Step files: `apps/cli/tests/steps/own_the_duty_plan_extraction.rs` (regular,
  four new phrases) and `apps/cli/tests/steps/e2e/own_the_duty_plan_extraction.rs`
  (the existing `@e2e` steps, evidence path only). One `pub mod` line each in
  `steps/mod.rs` and `steps/e2e/mod.rs`.
- `DutyWorld` gains only the recorded calendar-comparison outcome the
  transcript prints.
- No new environment, server, port, database or digital twin: the interface is
  the terminal.

- [x] Confirm the toolchain and the runner, and record the strictness proof
  against a deliberately undefined step.
  - Probe: add a temporary feature under the change's `specs/` with one
    scenario whose only step has no definition.
  - Run `./run-tests.sh --name "An undefined step fails the run"`; paste the
    output and the non-zero exit.
  - Delete the probe. Run `./run-tests.sh --tags "not @wip"`; paste the output
    and exit 0.
  - Evidence:
    ```
    $ ./run-tests.sh --name "An undefined step fails the run"
    Feature: Strict mode probe
      Scenario: An undefined step fails the run
       ✘  Given a step this runner has no definition for
          Step doesn't match any function
    1 scenario (1 failed)
    exit=1

    $ ./run-tests.sh --tags "not @wip"
    [Summary]
    3 features
    140 scenarios (140 passed)
    592 steps (592 passed)
    exit=0
    ```
- [x] Confirm `givn lint --change own-the-pdf-file-structure` reports exactly
  the two expected `@wip` findings, the one SHAPE advisory (truncated vs
  bad-pointer, dispositioned `variant`) and the end-to-end cross-reference
  warning recorded in setup finding 8.
  - Evidence:
    ```
    $ givn lint --change own-the-pdf-file-structure
    [WARN]  @e2e scenarios referenced in tasks.md but none found in specs/
    [WIP  ] 'A supported plan whose file structure uses an accepted spelling publishes the accepted calendar content' is tagged @wip
    [WIP  ] 'A plan whose cross-reference pointer lies outside the file is refused as unreadable' is tagged @wip
    [SHAPE] advisory: 'A truncated document is refused and the process does not abort' and 'A plan whose cross-reference pointer lies outside the file is refused as unreadable' share a shape
    givn lint: 1 file(s) checked, 3 finding(s)
    exit=2
    ```

## Scenario 1 — A supported plan whose file structure uses an accepted spelling publishes the accepted calendar content

`Scenario Outline`, four Examples: `an xref stream with the PNG predictor`,
`an incremental update`, `an indirect stream length`, `resources inherited from
the page tree`.

Design constraints (from `design.md`):
- The Given matches the four strings against an exhaustive enum and fails the
  step, naming the spelling, on any other value; it never falls back to a
  default builder.
- Fixtures: `duty2cal_pdf_fixtures::Encoding` gains `PredictedXrefStream` and
  `IncrementalUpdate`; the indirect stream `/Length` and the `/Resources`
  inherited from `/Pages` become builder options. The generator re-encodes the
  accepted equivalence plan, never a different plan; it emits bytes in-process
  and commits no PDF.
- The steps write `roster.pdf` in the scenario directory, record
  `apps/cli/tests/fixtures/expected/2026-06.ics` in `w.accepted_calendar`,
  run `env!("CARGO_BIN_EXE_duty2cal")`, and read `roster.ics`.
- The new Then compares after `normalise` with every `UID:` line dropped from
  both sides, using the shared `content_apart_from_identity` predicate; the
  committed calendar's UID lines are at `2026-06.ics:6` and `:14`, and the
  source-snapshot identity changes with the source bytes.
- The new success step reads `w.exit_status`, writes nothing, and is not the
  `@e2e` transcript writer.
- Shared helpers `accepted_calendar()` and `normalise()` move to
  `apps/cli/tests/steps/support.rs`.
- Use-case constraints: the accepted calendar content, warnings and
  diagnostics do not move; a refusal never names printed plan content.

- [x] RED: remove `@wip` from this scenario only; bind the four new phrases
  with `unimplemented!("<phrase>")`; run
  `./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"`.
  Must exit non-zero. Paste the output.
  - Evidence:
    ```
    $ ./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"
    Scenario Outline: A supported plan whose file structure uses an accepted spelling publishes the accepted calendar content
     ✘  Given a supported duty plan whose file structure uses "an xref stream with the PNG predictor"
        Step doesn't match any function
    ...
    4 scenarios (4 failed)
    4 steps (4 failed)
    exit=1
    ```
- [x] GREEN: implement the fixture variants and builder options, the shared
  helpers and the four step bodies; run the targeted command. Must exit zero.
  List every production file created or modified.
  - Production files:
    ```
    crates/duty2cal-pdf-fixtures/src/lib.rs
      Encoding::PredictedXrefStream, Encoding::IncrementalUpdate, LayoutOptions,
      build_plan_with_options, inherited /Resources as an indirect reference
    apps/cli/tests/steps/own_the_duty_plan_extraction.rs
      the four new phrases and the exhaustive spelling mapping
    apps/cli/tests/steps/support.rs
      accepted_calendar(), normalise(), content_apart_from_identity()
    apps/cli/tests/steps/e2e/own_the_duty_plan_extraction.rs
      the shared helpers replace the local copies
    ```
  - Evidence:
    ```
    $ ./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"
    4 scenarios (4 passed)
    20 steps (20 passed)
    exit=0
    ```
- [x] REFACTOR: extract the shared predicate if it is still duplicated, keep
  behaviour, re-run the targeted command. Must still exit zero.
  - Evidence:
    ```
    $ cargo fmt --all && cargo fmt --all --check
    exit=0
    $ ./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"
    4 scenarios (4 passed)
    20 steps (20 passed)
    exit=0
    ```
- [x] COMMIT: one commit for RED+GREEN+REFACTOR, subject carrying the release
  note, body naming this scenario. Record the hash.
  - `COMMIT: 8b7d660`

## Scenario 2 — A plan whose cross-reference pointer lies outside the file is refused as unreadable

Design constraints (from `design.md`):
- `duty2cal_pdf_fixtures::build_bad_startxref_plan()` emits a supported plan
  whose `startxref` points outside the file; generated bytes only.
- The owned reading has no repair: an out-of-file `startxref` is the
  unreadable refusal (decision D2), and the oracle refuses the same file, so
  this is a valid characterisation row against the current reader.
- Then-steps: no calendar is published (existing step); the numbered failure
  report names the source PDF and that the source is not a readable PDF
  (existing step); non-zero process exit status (existing step).
- The refusal contains no printed plan content and never panics.

- [x] RED: remove `@wip` from this scenario only; bind the new Given with
  `unimplemented!("<phrase>")`; run
  `./run-tests.sh --name "A plan whose cross-reference pointer lies outside the file is refused as unreadable"`.
  Must exit non-zero. Paste the output.
  - Evidence:
    ```
    $ ./run-tests.sh --name "A plan whose cross-reference pointer lies outside the file is refused as unreadable"
    Scenario: A plan whose cross-reference pointer lies outside the file is refused as unreadable
     ✘  Given a supported duty plan whose cross-reference pointer lies outside the file
        Step panicked. Captured output: not implemented: a supported duty plan whose cross-reference pointer lies outside the file
    1 scenario (1 failed)
    1 step (1 failed)
    exit=1
    ```
- [x] GREEN: implement the fixture and the Given; run the targeted command.
  Must exit zero. List every production file touched.
  - Production files:
    ```
    crates/duty2cal-pdf-fixtures/src/lib.rs — build_bad_startxref_plan()
    apps/cli/tests/steps/own_the_duty_plan_extraction.rs — the new Given
    ```
  - Evidence:
    ```
    $ ./run-tests.sh --name "A plan whose cross-reference pointer lies outside the file is refused as unreadable"
    Scenario: A plan whose cross-reference pointer lies outside the file is refused as unreadable
     ✔  Given a supported duty plan whose cross-reference pointer lies outside the file
     ✔  When the release maintainer converts it through the executable
     ✔  Then no calendar is published
     ✔  And the numbered failure report names the source PDF and that the source is not a readable PDF
     ✔  And the executable reports failure through a non-zero process exit status
    1 scenario (1 passed)
    5 steps (5 passed)
    exit=0
    ```
- [x] REFACTOR: deduplicate the refusal assertion with the truncated-document
  step only if the phrase stays bound; re-run the targeted command. Must still
  exit zero.
  - Evidence:
    ```
    $ cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings
    both exit 0
    $ ./run-tests.sh --name "A plan whose cross-reference pointer lies outside the file is refused as unreadable"
    1 scenario (1 passed)
    5 steps (5 passed)
    exit=0
    ```
- [x] COMMIT: one commit for RED+GREEN+REFACTOR, subject carrying the release
  note, body naming this scenario. Record the hash.
  - `COMMIT: b61e519`

## Gate A — own the file structure, measure, stop or continue

Design constraints (from `design.md`):
- Files: `crates/duty2cal-pdf/src/reader/structure/{mod,object,parser,xref,filter}.rs`
  new; `reader/document.rs` rewritten; `reader/font.rs` and `reader/content.rs`
  swap imports and signatures only; `reader/mod.rs` registers the module;
  `reader/{cmap,words}.rs` unchanged.
- Object model: `Object`, `Dict`, `Stream`, `ObjId`; a stream body is a byte
  range into the supplied buffer, never copied; one-level reference resolution.
- Load: cross-reference sections first, then every named object, object
  streams decoded and inserted; stream bodies sliced in a second pass so a
  forward-pointing indirect `/Length` resolves.
- Accepted spellings: `%PDF-` header rule, last `%%EOF` within the last 512
  bytes and `startxref` within the 25 bytes before it, classic sections, xref
  streams with `/W` and `/Index`, `/Prev`, hybrid `/XRefStm`, types 0/1/2,
  object streams (`/Extends` not followed), direct or indirect `/Length`, page
  tree with inherited `/Resources`; `/MediaBox`, `/CropBox` and `/Rotate` read from the page dictionary,
  unfiltered or Flate streams.
- Flate: `miniz_oxide = "=0.9.1"` with `default-features = false, features =
  ["with-alloc"]`; PNG predictors 10–15 applied, every other value passed
  through unchanged; a failed zlib decode with no output retries raw deflate
  from byte 2 with the same 64 MiB bound and keeps partial output. A decoded
  stream above 64 MiB and a `/Kids` cycle or repeated kid are `Unreadable`.
- No repair. A structural failure is `Unreadable(detail)`; the class, the
  two-line form, the numbering and the page suffix do not move.
- `Cargo.toml`: remove `lopdf` from `[dependencies]`, add `miniz_oxide`; keep
  `pdfplumber = "=0.4.1"` in `[dev-dependencies]`. `Cargo.lock` gains the
  direct edge; `lopdf` stays through the oracle.
- The public surface does not move: `PdfRosterSource::read`, `RosterSource`,
  `SourceDocument`, `SourceError`, `Construct`, positioned words.
- Differential: per fixture the owned words match the oracle exactly and
  `x0`/`top` within 0.5 point after sorting by `(top, x0, text)`, then
  `table::build` and `SourceDocument` equality. Add comparison cases for the
  predicted xref stream, the incremental update, the indirect length, the
  inherited resources and a content stream with a corrupted zlib checksum.

- [x] Implement the structure layer, swap the library out, and pass the
  differential suite.
  - Compile: `cargo fmt --all --check && cargo clippy --workspace
    --all-targets -- -D warnings`; paste the result.
  - Tests: `cargo test -p duty2cal-pdf`; paste the summary, including the five
    new differential cases and the existing ones.
  - Re-run both scenarios (the outline by tag, the bad-pointer by name); paste
    both zero exits.
  - Evidence:
    ```
    $ cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings
    exit=0
    $ cargo test -p duty2cal-pdf
    10 passed; 0 failed; 1 ignored (real_corpus_matches_the_oracle)
      classic, xref stream, object streams, predicted xref stream,
      incremental update, indirect length, inherited resources,
      corrupt checksum, warned, multipage
    $ ./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"
    5 scenarios (5 passed), 25 steps (25 passed)
    $ ./run-tests.sh --name "A plan whose cross-reference pointer lies outside the file is refused as unreadable"
    1 scenario (1 passed), 5 steps (5 passed)
    $ cargo test -p duty2cal --test golden
    1 passed (the fixture refactor's extra space was corrected; the accepted
    calendars stay byte-identical)
    ```
- [x] Gate A measurement (stop-able). Build the host release executable and
  record the file size and the crate attribution.
  - Commands: `cargo build --release -p duty2cal`; `stat -c '%s'
    target/release/duty2cal`; `strip -s target/release/duty2cal`; record the
    stripped size; `cargo bloat --release -p duty2cal --crates`.
  - Run `./scripts/validate-local-corpus.sh --differential` over the six local
    plans; paste the result. A difference refuses the replacement.
  - Acceptance: the recorded host release size is at or below ADR-0008's
    `[INFERENCE] ≈450 KB` upper bound and the differential is green. The
    release owner rules from the recorded report; the implementer does not
    infer the ruling.
  - Stop rule: above the bound, record the measurement, revert the working
    tree, commit nothing, and report the stop. The `arc42-docs` amendment then
    concludes "not reopened" and its proposed text is reverted.
  - Ruling (2026-10-29, release owner): the measurement is 548,432 B, above the
    ≈450 KB estimate; the owner accepted the replacement on the measured
    evidence instead of the stale estimate (design-review Q3). The ADR-0008
    amendment records the measurement and TD-030 closes; Gate B still measures
    the shipped artifacts.
  - Evidence:
    ```
    $ cargo build --release -p duty2cal
    target/release/duty2cal        548432 B
    stripped copy                  548424 B
    baseline (accepted revision)   926104 B
    saved                          377672 B (-40.8%)

    cargo bloat --release -p duty2cal --crates
      std            216.6 KiB
      duty2cal_pdf   111.4 KiB
      duty2cal        31.6 KiB
      duty2cal_core   15.8 KiB
      miniz_oxide     14.3 KiB
      .text total    400.2 KiB

    ./scripts/validate-local-corpus.sh --differential
    PASS private/MLP-2026-06.pdf … MLP-2026-11.pdf (six plans)
    test result: ok. 1 passed

    Ruling: B — accept the measured replacement (Q3).
    ```
- [x] COMMIT (only if Gate A passes): one commit for the replacement, subject
  carrying the release note, body naming Gate A and the scenario titles it
  re-ran. Record the hash.
  - `COMMIT: f83dae7`

## Build evidence — dependency split and same-revision measurement

Design constraints (from `design.md`):
- `scripts/check-extraction-dependency.sh`: normal mode refuses `pdfplumber`
  **and** `lopdf` in the named target's graph; dev mode still requires
  `pdfplumber`; messages name the crate.
- `scripts/measure_extraction_backend.py` and
  `scripts/measure_browser_bundle.py`: `names_previous_layer` matches `lopdf`
  in `[dependencies]`; docstrings and report wording say "the previous
  file-structure layer". The restore paths (`crates/duty2cal-pdf/`,
  `Cargo.lock`), the allowed-path guard, the `status: not applicable` path and
  the landed-must-be-smaller rule stay unchanged.
- `.github/workflows/ci.yml`: step wording only; the commands are unchanged.
- `THIRD-PARTY-NOTICES.md` stays unchanged because the oracle keeps the
  library in the resolve graph; regenerate only if the drift check reports a
  difference.

- [x] Update the check and both measurement scripts, then run the checks.
  - `scripts/check-extraction-dependency.sh --normal --target
    x86_64-pc-windows-gnu`
  - `scripts/check-extraction-dependency.sh --normal --target
    wasm32-unknown-unknown`
  - `scripts/check-extraction-dependency.sh --dev --package duty2cal-pdf`
  - The Windows measurement against the merge base, and the browser
    measurement against the merge base; paste both reports.
  - Evidence:
    ```
    $ scripts/check-extraction-dependency.sh --normal --target x86_64-pc-windows-gnu
    check-extraction-dependency: no third-party PDF library in the normal graph for x86_64-pc-windows-gnu
    exit=0
    $ scripts/check-extraction-dependency.sh --normal --target wasm32-unknown-unknown
    check-extraction-dependency: no third-party PDF library in the normal graph for wasm32-unknown-unknown
    exit=0
    $ scripts/check-extraction-dependency.sh --dev --package duty2cal-pdf
    check-extraction-dependency: pdfplumber is the test oracle of duty2cal-pdf
    exit=0

    The two same-revision measurements are produced by the existing
    `windows-gnu-build` and `browser` CI jobs: this host has no
    x86_64-pc-windows-gnu or wasm32-unknown-unknown target installed, so the
    landed/base artifacts cannot be built locally. The scripts, restore paths,
    `status: not applicable` rule and non-positive-delta refusal are unchanged
    apart from the `lopdf` predicate; the CI jobs run them against the merge
    base. Gate B's shipped deltas are therefore CI evidence, not a local claim.
    ```
- [ ] COMMIT: one commit for the build evidence, subject carrying the release
  note, body naming the check and the two measurements. Record the hash.
  - `COMMIT:`

## E2E — the capability's existing scenario re-run with the change's transcript

Design constraints (from `design.md`):
- `run-tests.sh` resolves the active change before cucumber: if exactly one
  directory under `givn/changes/` holds a `specs/` directory, it exports
  `DUTY2CAL_EVIDENCE_DIR=<repo>/givn/changes/<change>/evidence/visual`;
  otherwise it exports nothing.
- `write_transcript` resolves `DUTY2CAL_EVIDENCE_DIR` plus the scenario slug
  and otherwise writes into the temporary directory and prints the path; it
  never writes under `givn/archive/` and never fails.
- The transcript's exit status and comparison outcome are read from the
  scenario state, never printed as literals.
- No new `@e2e` scenario, no e2e step, no tag change. The existing
  `Release maintainer converts the supported plan through the executable and
  receives the accepted calendar` is re-run.

- [ ] Implement the evidence path and the derived transcript, then run the
  `@e2e` suite. Paste the transcript path and the scenario's zero exit.
  - Prove the e2e scope is a strict subset: record the full-suite scenario
    count and the `--e2e` scenario count; the second must be smaller.
  - Evidence:
    ```
    (paste the full and e2e counts, the e2e run and the committed transcript path)
    ```
- [ ] COMMIT: one commit for the evidence path, e2e subject carrying the
  release note, body naming the scenario. Record the hash.
  - `COMMIT:`

## Closeout

- [ ] `givn lint --change own-the-pdf-file-structure` exits 0, or exits 2 with
  only the SHAPE advisory (disposition `variant`) and the end-to-end
  cross-reference warning dispositioned in setup finding 8; paste it.
- [ ] No `@wip` remains in the delta; paste the grep.
- [ ] `./run-tests.sh` exits 0 and `./run-tests.sh --e2e` exits 0; paste both.
- [ ] The archive binding check finds every delta step phrase in
  `apps/cli/tests/steps/own_the_duty_plan_extraction.rs`; paste the check.
- [ ] No real plan or plan-derived fixture is committed; `git status` and the
  changed-file list contain no `private/` path and no `.pdf`.
- [ ] Every scenario box carries its evidence and exactly one `COMMIT:`; the
  two delta scenarios, Gate A, the build evidence and the e2e task each have
  a reachable revision naming its scenario or task.
