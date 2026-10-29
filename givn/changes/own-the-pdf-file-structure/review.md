# Review: own-the-pdf-file-structure

Fresh-context review of the completed change: the two delta scenarios
(`8b7d660` accepted-spelling outline, `b61e519` bad-pointer refusal), the Gate A
replacement (`f83dae7`), the build evidence (`cb9b95d`), the e2e evidence path
(`0c18bd8`), the closeout (`619003c`), the remediation of the previous review's
five findings (`8ab4806`) and this review's F6 remediation (`a634668`: two added
differential cases, the doc list and the design-review re-review). `givn status`
reports 23/23 tasks.

Everything below was re-derived from the tree, not from the previous review or
the task list's pasted evidence. Read in full: both delta files and the
permanent spec, the owning use case, the three persona files, `design.md`,
`design-review.md`, `tasks.md`, `proposal.md`, `arc42.md`, the affected
`docs/arc42` chapters and ADR-0008, all step definitions, the structure
implementation, the fixture generator, the differential suite, the runner, the
dependency and measurement scripts, `run-tests.sh`, `givn/commands.yaml`, and
the committed transcript. No production code, spec, design, task, doc or
proposal was edited; only this file.

## Fabrication Audit

### Step definition scan

```
$ grep -nE '^\s*(\{\s*\}|\(\s*\)\s*\{\s*\})\s*$|^\s*(pass|return)\s*$|unimplemented!|todo!' \
    apps/cli/tests/steps/own_the_duty_plan_extraction.rs \
    apps/cli/tests/steps/e2e/own_the_duty_plan_extraction.rs \
    apps/cli/tests/steps/support.rs
exit=1 (no match)
```

0 empty or no-op step bodies found across 3 step-definition files:

| File | Step bindings | Empty/no-op bodies |
|---|---:|---:|
| `apps/cli/tests/steps/own_the_duty_plan_extraction.rs` | 28 | 0 |
| `apps/cli/tests/steps/e2e/own_the_duty_plan_extraction.rs` | 4 | 0 |
| `apps/cli/tests/steps/support.rs` | 0 (helpers only) | 0 |

Every body was read. The new delta Givens build a generated fixture (or fail
the step naming an unknown spelling — `own_the_duty_plan_extraction.rs:117-145`
panics on any fifth string, no default builder); the Whens run
`env!("CARGO_BIN_EXE_duty2cal")` through `std::process::Command`
(`:31-42`); the Thens assert on the published `.ics`, the failure report, the
exit status and the warning report. No `unimplemented!`, `todo!`, bare `pass`/
`return`, or deleted-to-go-green step remains. The `{}` matches in a raw grep
are `format!`/`eprintln!` placeholders, not bodies.

### Commit cross-check

Every implementation/scenario task has one reachable revision naming its
scenario or task in the body:

| Task (tasks.md) | Revision | Touches |
|---|---|---|
| Setup — runner, strict mode, baseline | no source revision promised (evidence task) | `tasks.md` evidence prose only; no component was promised |
| Scenario 1 — accepted-spelling outline | `8b7d660` | `crates/duty2cal-pdf-fixtures/src/lib.rs` (`PredictedXrefStream`, `IncrementalUpdate`, `LayoutOptions`), both regular step files, `steps/support.rs`, docs/arc42 |
| Scenario 2 — bad-pointer refusal | `b61e519` | `crates/duty2cal-pdf-fixtures/src/lib.rs` (`build_bad_startxref_plan`), the new Given, docs |
| Gate A — owned file structure | `f83dae7` | `reader/structure/{mod,object,parser,xref,filter}.rs`, rewritten `reader/document.rs`, `font.rs`/`content.rs`, `reader/mod.rs`, `differential_tests.rs`, `Cargo.toml`/`Cargo.lock` |
| Gate A — measurement record | `f83dae7` (report in `tasks.md`) | measurement record only; no component promised |
| Build evidence — dependency check + both measurement scripts | `cb9b95d` | `scripts/check-extraction-dependency.sh`, `scripts/measure_extraction_backend.py`, `scripts/measure_browser_bundle.py`, `.github/workflows/ci.yml` |
| E2E evidence path | `0c18bd8` | `run-tests.sh`, `apps/cli/tests/features_runner.rs` (`calendar_matched`), both e2e step files, transcript rename |
| Closeout | `619003c` | `tasks.md` only (bookkeeping) |

The setup, measurement and closeout boxes are evidence/record tasks that
promise no code; they are not fabricated implementation. The two scenario
revisions touch the characterisation harness (fixture generator and step
definitions) because the delta's scenarios are the harness, and Gate A carries
the production reader. Each `- [x] COMMIT:` line now names its revision
(`tasks.md:222,284,389,437,479`), and `givn check review` integrity passes — the
previous review's F1 is closed. Each of the two checked delta scenarios carries
exactly one revision (`8b7d660`, `b61e519`); no revision is shared by two
scenarios, none is a placeholder, and both are reachable from `HEAD`. No real
plan or plan-derived PDF is committed: `git ls-files | grep -Ei '\.pdf$|^private/'`
is empty, and the change's `git diff --name-only d8a46f8..HEAD` contains no
`private/` path and no `.pdf`. The review-remediation commit `a634668` (two
differential cases, the `design.md`/chapter/R-list wording and the
design-review re-review) is a test-only follow-up with no `tasks.md` task; it
changes no scenario, tag or production behaviour.

### Promised-architecture cross-check

| Component (design.md) | Exists? | Path / evidence |
|---|---|---|
| Object model, parser, xref, filter, module surface | yes | `crates/duty2cal-pdf/src/reader/structure/{object,parser,xref,filter,mod}.rs` |
| Rewritten orchestration (`/Resources` inherited, boxes from the page dictionary) | yes | `reader/document.rs:57-78,158-186`; only `/Resources` walks `/Parent` |
| `miniz_oxide` direct dependency, `lopdf` out of shipped graphs, oracle dev-only | yes | `crates/duty2cal-pdf/Cargo.toml`; `Cargo.lock:1052,1193` |
| Differential corpus (14 comparison cases; 19 passing, 1 ignored) | yes | `differential_tests.rs`; `fixtures/src/lib.rs` (`Encoding::{PredictedXrefStream,IncrementalUpdate,HybridXrefStream,CompactXrefStream}`, `LayoutOptions`, `build_leading_junk_plan`, `build_corrupt_checksum_plan`, `build_bad_startxref_plan`) |
| Shared `accepted_calendar()`, `normalise()`, `content_apart_from_identity()` | yes | `apps/cli/tests/steps/support.rs` |
| `DutyWorld` comparison field | yes | `apps/cli/tests/features_runner.rs:48` (`calendar_matched`) |
| Evidence-path resolution and derived transcript | yes | `run-tests.sh:142-146`; `steps/e2e/own_the_duty_plan_extraction.rs:80-113` |
| Dependency check and both measurement predicates | yes | `scripts/check-extraction-dependency.sh:48`; both measurement scripts match `lopdf` in `[dependencies]` |
| Graph check, bootstrap export visibility, CI wording | yes | `steps/e2e/export_duty_plan_calendar.rs` (`scenario_slug`), `.github/workflows/ci.yml` |

Nothing design.md promises is missing. The F2 file-location error from the
previous review is corrected in `design.md`'s Files and Step Definitions tables
(`features_runner.rs`, plus the changed export e2e file); F3's Flate API is now
the streaming one in the design and chapter 2; F4's header slicing is
implemented (`structure/mod.rs:76` slices at `xref::header_offset`, and
`header_offset` validates the version and the line ending); F5's accepted-path
corpus was extended, and F6's remediation adds the last two cases: 19 passing
differential cases, 1 ignored.

### Strict-mode proof

Present in the setup task: a deliberately undefined step exits non-zero
(`✘ … Step doesn't match any function`, `1 scenario (1 failed)`, `exit=1`), the
probe is deleted, and the `not @wip` baseline exits 0. The runner enforces it:
`apps/cli/tests/features_runner.rs:83` calls `.fail_on_skipped()`. The two
scenario RED runs in `tasks.md` were also produced by `unimplemented!` stubs
that panicked, not by skipped steps. Not re-run here: the review reads the
recorded proof, the runner source and the strict RED/GREEN evidence.

### Downgraded e2e scenario scan

The capability's one `@e2e` scenario is the permanent
`Release maintainer converts the supported plan through the executable and
receives the accepted calendar` (`givn/specs/own-the-duty-plan-extraction/own-the-duty-plan-extraction.feature:3-9`).

| @e2e Scenario | Real-interface assertion present? | Driven via real interface? | Downgraded? |
|---|---|---|---|
| Release maintainer converts the supported plan through the executable and receives the accepted calendar | Yes — published `.ics` carries `BEGIN/END:VCALENDAR`, stdout carries `Written:`, the calendar equals the committed accepted calendar after `normalise`, and the process exit status is 0 (`steps/e2e/own_the_duty_plan_extraction.rs:36-77`) | Yes — `std::process::Command` on `env!("CARGO_BIN_EXE_duty2cal")` (`steps/own_the_duty_plan_extraction.rs:31-42`); no in-process shortcut, no HTTP/fetch | No |

The transcript writer is part of the success step and derives both values from
scenario state (`exit_status`, `calendar_matched`), never from literals.

### E2e scope check

| Capability | @e2e scenario count | Distinct actions | Over-produced? |
|---|---:|---|---|
| `own-the-duty-plan-extraction` | 1 | one normalized inventory action (convert a supported plan through the built executable and read the calendar) | No |

The delta's outline (four accepted spellings) is input variation of that
action, and the bad-pointer scenario is its failure edge; neither adds an
inventory entry or an `@e2e` scenario (`proposal.md` Capability Routing,
design D7). The assembled corpus confirms the `@e2e and not @browser` set is
exactly two scenarios: this capability's and the export capability's CLI one.

### `@e2e` tag integrity

No `@e2e` tag was removed or moved. The delta never carried one: the Feature is
tagged `@givn.delta @own-the-duty-plan-extraction` and both scenarios
`@givn.added`. The permanent `@e2e` scenario is unchanged
(`git diff d8a46f8..HEAD -- givn/specs/own-the-duty-plan-extraction/own-the-duty-plan-extraction.feature`
is empty). `givn lint`'s
`@e2e scenarios referenced in tasks.md but none found in specs/` warning is the
D7 disposition (the permanent scenario is re-run, not re-authored), not a
removed tag.

### Duplicate-implementation check

`verify.e2e_command` is `./run-tests.sh --e2e`, which resolves to
`scripts/assemble-features.sh` plus
`cargo test -p duty2cal --test features_runner -- --tags "@e2e and not @browser"`
(`run-tests.sh:106`). The files it actually invokes for this capability are
`apps/cli/tests/features_runner.rs`,
`apps/cli/tests/steps/e2e/own_the_duty_plan_extraction.rs` and the shared When
in `apps/cli/tests/steps/own_the_duty_plan_extraction.rs`. A tree search
(`grep -rn "accepted calendar is recorded|write_transcript|DUTY2CAL_EVIDENCE_DIR"`)
found no second/parallel e2e implementation; the worktree is clean apart from
this review file and the gate-generated `verification/` receipt, and no untracked step file exists. The other permanent
`@e2e` scenario belongs to `export-duty-plan-calendar`, a different capability
and a different action, not a rival implementation.

### Local environment check

One-shot CLI with no server, port, database, queue, container or credential —
the design's `## Local runnability & digital twins` states there is no digital
twin. The run evidence is the committed transcript and the e2e run recorded in
`tasks.md` (`--e2e`: 2 scenarios, 2 passed); the targeted CLI runs in this
review also execute the built binary. Nothing was started for this review.

### E2e isolation check

Read literally from `givn/commands.yaml` (the project's single source for
commands; the template's `config.yaml` pointer is stale):

| `verify.command` | `verify.e2e_command` | Identical? | Regular scope | E2e scope | Strictly smaller? |
|---|---|---|---:|---:|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 145 (recorded full run, `tasks.md`) | 2 (recorded `--e2e` run, `tasks.md`) | Yes (2 < 145) |

No archive receipt exists for this change yet. The change-level task receipt
generated by `givn check tasks` for this review records the regular scope at
`HEAD` 8ab4806: 145 scenarios, 145 passed, 0 failed, 0 skipped
(`givn/changes/own-the-pdf-file-structure/verification/tasks-verification.json`).
The e2e scope count is the recorded `--e2e` run (2 passed) and was verified
independently by assembling the corpus and counting: `@e2e` without `@browser`
is exactly the two scenarios named above. The nearest archived per-scope
receipt, `replace-pdf-extraction-backend` (regular 140, e2e 2), is consistent
with the 5 delta scenarios this change adds.

### Design.md conformance check

| Decision (design.md) | design.md says | Actually built | Match? |
|---|---|---|---|
| `verify.command` / `verify.e2e_command` | `./run-tests.sh` / `./run-tests.sh --e2e` (`@e2e and not @browser`) | exactly those values in `givn/commands.yaml`; `run-tests.sh:106` | yes |
| Runner / strict mode | cucumber-rs 0.23, `.fail_on_skipped()` | `features_runner.rs`, `.fail_on_skipped()` at `:83` | yes |
| E2E step file | `apps/cli/tests/steps/e2e/own_the_duty_plan_extraction.rs` (transcript path + derived values) | same file, `write_transcript` derives both values | yes |
| Regular step file | `apps/cli/tests/steps/own_the_duty_plan_extraction.rs`, four new phrases | same file, 28 bindings, exhaustive spelling map | yes |
| `DutyWorld` location | `apps/cli/tests/features_runner.rs` | `calendar_matched` at `features_runner.rs:48` | yes |
| Flate decode | `miniz_oxide::inflate::stream::{inflate, InflateState}` with the 64 MiB bound and raw-deflate retry | `structure/filter.rs:13,111-131,134-160` | yes |
| Header rule | buffer sliced at the first `%PDF-`, offsets relative to the sliced view | `structure/mod.rs:76`, `structure/xref.rs:105-125` | yes |
| Structure module layout | `reader/structure/{mod,object,parser,xref,filter}.rs`, `document.rs` rewritten, font/content signatures only | present as listed | yes |
| Dependency manifest | `miniz_oxide` direct, `lopdf` removed, `pdfplumber` dev-only | `crates/duty2cal-pdf/Cargo.toml` | yes |
| Measurement predicate | both scripts match `lopdf` in `[dependencies]`; check refuses `lopdf` and `pdfplumber` in shipped graphs | scripts as designed | yes |
| Evidence path | `DUTY2CAL_EVIDENCE_DIR` for the single active change; temp fallback, never `givn/archive/` | `run-tests.sh:142-146`, `write_transcript` | yes |

The design changes since the first review are the F2–F5 corrections of
`8ab4806` and the F6 corpus enumeration of `a634668`; `design-review.md`
records a re-review for both. No silently overridden decision remains.

### Interaction coverage verification

| Use-case inventory entry (`usecase.md` §Interactions) | Design matrix row | `@e2e` scenario | Promised driving mechanism | Actual driving mechanism | Match? |
|---|---|---|---|---|---|
| convert the supported duty plan through the built executable and read the calendar it publishes | present — CLI (terminal) | `Release maintainer converts the supported plan through the executable and receives the accepted calendar` | `std::process::Command` on `env!("CARGO_BIN_EXE_duty2cal")` | `std::process::Command` on the built binary; asserts the published `.ics`, `Written:` line and exit status | yes |

One inventory entry, one matrix row, one matching `@e2e` scenario, and the
promised driver in the step file. The delta's two scenarios add no inventory
entry.

### Gate advisory: proposal routing column shape (recorded, not a finding)

`givn check review` prints a warning:
`proposal.md has no filled '## Capability Routing' table — the delta
capabilities [own-the-duty-plan-extraction] are undeclared`. The table exists
and records the correct decision (`EXTEND own-the-duty-plan-extraction`,
`proposal.md:64-66`), and the placement is verified: the delta lives under
`specs/own-the-duty-plan-extraction/`, the tag is `@own-the-duty-plan-extraction`,
and the permanent use case exists. The installed givn parser reads the decision
from the third cell and requires at least four cells
(`routing_table.rs:44-47`); this proposal's table has the three-column form
(`Proposed capability | Decision | Rationale`), so the machine check cannot read
it and degrades to the advisory. This does not affect the routing: the delta's
capability is unchanged and the interaction inventory is untouched. Recorded
for the archive owner; a later proposal edit adding the `Route's recommendation`
column (empty or `—`) would let the gate read the decision mechanically. It is
not a fabrication, coverage or conformance finding.

FABRICATION AUDIT: CLEAN — no `@e2e` tag removed, 0 empty step bodies, every
implementation task revision verified, every promised component present,
strict-mode proof recorded, no downgraded e2e scenario, no second e2e
implementation, e2e scope a strict subset, implementation matching the reviewed
design.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | The delta asserts the accepted calendar content and the unreadable refusal; the refusal contract (two-line `Source PDF:`/`Reason:`, page suffix when known, no printed plan content, non-zero exit, no calendar) is untouched; the e2e compares the accepted calendar | yes |
| Capability ownership and typed relationships preserved | `own-the-duty-plan-extraction` remains the single capability; `Includes`/`Extends` are `none`/`none`; the proposal's `EXTEND` routing matches the delta location and adds no consumer action | yes |
| Confirmed Persona-relevant outcomes preserved | Release owner: the Gate A ruling and Gate B job; calendar owner: byte-identical calendar and unchanged refusals; web reviewer: the WASM bundle claim is build evidence, not a screen | yes |
| Actors remain distinct from Persona biographies | Gherkin names the release maintainer; no Persona biography is used as an actor | yes |
| Migration ledger and coverage evidence complete, if applicable | Not applicable — no migrating capability or alias set | yes |

Design-review corrections that the review re-checked in the tree:

- **Q1 (one-shot measurement predicate)** — the durable rule now names the
  replaced layer with `lopdf` in parentheses
  (`givn/specs/own-the-duty-plan-extraction/usecase.md:55`), and both
  measurement scripts use the same `lopdf` predicate
  (`scripts/measure_extraction_backend.py:38`,
  `scripts/measure_browser_bundle.py:39`). The rule and the code agree.
- **Q2 (file-structure spelling)** — the glossary row exists
  (`docs/arc42/12-glossary.md:116`) and matches the spec's Given and the
  design's exhaustive mapping.
- **Inheritance correction** — the code inherits only `/Resources` (page
  dictionary plus `/Resources` references up the `/Parent` chain,
  `document.rs:158-186`); `/MediaBox`, `/CropBox` and `/Rotate` are read from
  the page dictionary (`document.rs:51-78`). Design, proposal, chapter 5 and
  the ADR use the same wording.

Scope-rejection criteria: single-verb goal; primary actor named by the
trigger; the capability passes the placement test; no mutation under an
observing goal; extensions reference their main-flow steps; preconditions are
supplied; names are goal-shaped; the leaf is not oversized; relationships are
used correctly; the goal's delivery clause is pre-existing.

PERSONAS: project-maintainer--release-owner
PERSONAS: crew-member--personal-calendar-owner
PERSONAS: contributor--web-reviewer

## Deferred mandatory checks

None. The two shipped-artifact measurements are Gate B's automation in the
existing Windows and browser CI jobs — a landing gate, not a check postponed
for this review. The private-corpus differential is opt-in by the design's
interface-obstacle fix (CI cannot read `private/`), and the generated corpus
backs it in CI. No `DEFERRAL:`/`OPERATOR-DECISION:` pair is recorded because
nothing is deferred.

## Visual Review

Contract: `Interface: terminal`; the evidence kind is one transcript per
`@e2e` scenario. One transcript exists for the one `@e2e` scenario:

| @e2e Scenario | Viewport | Evidence file | Present and non-empty? |
|---|---|---|---|
| Release maintainer converts the supported plan through the executable and receives the accepted calendar | terminal (n/a) | `givn/changes/own-the-pdf-file-structure/evidence/visual/release-maintainer-converts-the-supported-plan-through-the-executable-and-receives-the-accepted-calendar/transcript.txt` | yes (4 lines: command, `Written:` path, exit status 0, normalised comparison identical) |

The transcript seen: `$ duty2cal <scenario-dir>/roster.pdf`, `Written:
<scenario-dir>/roster.ics`, `exit status: 0`, `normalised comparison:
identical`. No host path, user name or clock value survives; `<scenario-dir>`
is substituted. No screenshots exist because no page is rendered.

This review is the fresh context that saw the transcript; it wrote none of the
artifacts. One lens per confirmed Persona:

| Confirmed Persona | Goal | Finding | Disposition |
|---|---|---|---|
| project-maintainer--release-owner | decide the replacement on the delivered calendar and the recorded measurement | none — the transcript shows the accepted calendar published from the built executable with exit 0 | — |
| crew-member--personal-calendar-owner | receive the accepted calendar through the supported route | none — the published file is the complete, normalised accepted calendar; no diagnostic is needed | — |
| contributor--web-reviewer | see that the browser route keeps the same conversion | none — the transcript is the shared conversion path; the browser-side claim is the build evidence, not a screen | — |

**Baseline:** the visible text carries no internal identifier, no placeholder,
no duplicated content, a real command path and, for the failure state, the
two-line report (covered by the regular refusal scenarios). No contract rule
and no Persona goal fails; no pixel or coordinate finding is offered.

VISUAL-REVIEW: PASS

## Retirements

N/A — the change retires no capability (`@givn.retired` appears nowhere in the
delta).

## Arc42 implementation conformance

Chapters independently derived as affected from the completed change:
02 (dependency constraints), 04 (solution strategy), 05 (building blocks),
06 (runtime), 08 (crosscutting concepts), 09 (ADR register), 10 (quality
scenarios), 11 (risks/debt), 12 (glossary), plus ADR-0008. Chapters 01, 03 and
07 are independently confirmed **not** affected: chapter 01's quality-goal
sentence is already true and untouched (git history shows no edit in this
change), chapter 03 gains no actor, interface or inventory entry, and chapter
07's runner, platform, packaging and budget are unchanged (only CI step wording
moved).

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 01 quality goal 5: the reading path is the project's own | `01-introduction-and-goals.md:28` | not affected | D1–D3 | Gate A | sentence already true; no edit needed | yes |
| 02 dependency constraints: no `lopdf`/`pdfplumber` in shipped graphs, `miniz_oxide` only third-party in the reading path, oracle dev-only | `02-architecture-constraints.md:17-19` | affected | Technology decisions; D1/D4 | Gate A, build evidence | `crates/duty2cal-pdf/Cargo.toml`, `check-extraction-dependency.sh:48`, `Cargo.lock:1052,1193`; streaming inflate API named | yes |
| 03 context/scope: no new actor, interface or inventory entry | `03-context-and-scope.md` | not affected | D7 | scenarios | delta adds variants only; assembled inventory unchanged | yes |
| 04 solution strategy: owned file structure, one pure-Rust decompressor, ADR-0008 boundary | `04-solution-strategy.md:5,20,37,47` | affected | D1–D5 | Gate A | same content; only `miniz_oxide` third-party | yes |
| 05 building blocks: `reader/structure/`, fixture variants, step files, graph and measurement scripts | `05-building-block-view.md:25,85-90,186,219-221` | affected | Files table | all | modules and scripts exist as listed | yes |
| 06 runtime: conversion reads the project's file structure; Gate A/Gate B measurement flow | `06-runtime-view.md:36,124,534-563` | affected | D2/D5/D6 | Gate A, build evidence | `document.rs`, the two scripts | yes |
| 07 deployment: unchanged runner/platform/budget | `07-deployment-view.md` | not affected | Delivery constraints | all | CI unchanged apart from wording | yes |
| 08 crosscutting: owned extraction, no repair, 64 MiB bound, predictor policy, parser panic rule | `08-crosscutting-concepts.md:15-68,741-751` | affected | D1–D3, Risks | Gate A, build evidence | `filter.rs`, `xref.rs`, no `unsafe`/`unwrap` on parsed data | yes |
| 09 ADR register: ADR-0008 amended in place, no second ADR | `09-architecture-decisions.md:18,54-62` | affected | ADR action | Gate A | `ADR-0008…md` frontmatter `status: accepted`, `amended: 2026-10-29`; register row and index note | yes |
| 10 QS-048 names the removed layer and the `not applicable` case | `10-quality-requirements.md:115` | affected | D5 | build evidence | both scripts' `names_previous_layer`; `status: not applicable` path | yes |
| 11 TD-030 closed; R-082–R-084 record the corpus, the parser risks and the measured outcome | `11-risks-and-technical-debt.md:93-101,135` | affected | D2/D6, Risks | Gate A | TD-030 closed on 548,432 B vs 926,104 B (−40.8%); R-078/R-082 list the full corpus | yes |
| 12 glossary: **File structure**, **File-structure spelling**, refined **Owned extraction**, **Frozen supported subset**, **Test oracle** | `12-glossary.md:55,61-63,115-118,137,166` | affected | Glossary impact | all | rows present; source paths current (`reader/structure/`) | yes |
| ADR-0008 decision and amendment: own the file structure, keep the test oracle | `docs/arc42/adr/ADR-0008…md:1-11,12-31,224-429` | registered/amended | D1–D6 | Gate A | amendment accepted 2026-10-29; Gate A measurement recorded; Gate B still the landing gate | yes |

Sign-off checks:

- [x] Every chapter marked affected by either assessment was opened and
      checked; none is a placeholder or a stale contradiction.
- [x] Every architecture fact relevant to this change is represented in
      `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts; the evidence names the
      built file, command, test or observable result.
- [x] Changes discovered in the completed implementation are recorded in the
      durable chapters and in `arc42.md` (the F3 Flate API, the F4 header slice
      and the F5 corpus were folded in by `8ab4806`).
- [x] No mismatch remains. The Gate A measurement is the accepted one:
      548,432 B host (548,424 B stripped) against the 926,104 B baseline,
      −377,672 B (−40.8%), with the six private plans green. The "10 cases"
      wording in the Gate A receipt sentences is the measurement-time count;
      the current corpus is 19 cases (verified: `cargo test -p duty2cal-pdf`
      runs 20, 19 passed, 1 ignored), and the corpus enumerations in
      `design.md`, chapter 5 and R-078/R-082 list every added case.

ARC42 CONFORMANCE: CLEAN

**Result:** CLEAN.

## Ubiquitous language conformance

| Term (glossary) | Delta spec | Design | Code | Glossary row |
|---|---|---|---|---|
| File structure | feature text and titles | throughout | `crates/duty2cal-pdf/src/reader/structure/` | `12-glossary.md:115` |
| File-structure spelling | Given `"<spelling>"`, Examples column | D2, exhaustive mapping | fixture `Encoding`/`LayoutOptions` | `12-glossary.md:116` |
| Owned extraction | "the project itself" | throughout | `reader/` modules | `12-glossary.md:137` |
| Frozen supported subset | "supported plan", "accepted spelling" | D2/D3 | `Construct`/`SourceError` | `12-glossary.md:118` |
| Test oracle | not named in the delta (build claim, D7) | D4 | `[dev-dependencies]` | `12-glossary.md:166` |
| Named refusal / unreadable | bad-pointer Then, refusal outline | D3 | `SourceError::{UnsupportedConstruct,Unreadable}` | `12-glossary.md:183` |
| Source snapshot | "source-snapshot identity" Then | corpus section | `SnapshotId` / `UID` | `12-glossary.md:159` |
| Differential | design's equivalence evidence | Differential and corpus | `differential_tests.rs` | `12-glossary.md:182` |

The coined terms the spec and design rely on are the glossary's rows, including
the new **File-structure spelling**; no undefined load-bearing term and no
invented synonym replaces a glossary term. The general **File structure**
definition names the page tree's inheritable boxes as PDF structure, while its
contract column matches the implemented `/Resources`-only inheritance.

UBIQUITOUS LANGUAGE: CLEAN

## Coverage

No archive receipt exists yet for this change; the coverage XMLs in
`./coverage/` predate the candidate and cover the previous reader. This
classification is from static inspection, and the archive will publish the
measured result. There is no percentage threshold.

| Location | Lines | Classification | Action |
|---|---|---|---|
| `structure/filter.rs:246` (`decode_row` filter 0, None) and `:257-266` (filter 3, Average) | the two row-filter arms of the PNG predictor | **2 — missing test coverage (resolved)** | F6: `png_none_filter_xref_stream_matches_the_oracle` (`png_filter: 0`) and `png_average_filter_xref_stream_matches_the_oracle` (`png_filter: 3`) now drive both arms through `compare_with_oracle`; `cargo test -p duty2cal-pdf` reports 19 passed, 1 ignored, and the six-plan differential stays green. |
| Malformed-input guards: page-tree cycle/repeated kid (`document.rs:135-139,175`), 64 MiB decode bound (`filter.rs:142,213,232`), missing `/Root`, unsupported `/W`, negative pointers, missing `%%EOF`/`startxref` | defensive arms for documents no supplied plan can produce | **3 — legitimately hard to test** | No supplied or generated plan reaches them (the supported family is the six-plan corpus); their shared observable — the unreadable refusal with the two-line report and the non-zero exit without an abort — is pinned by the bad-pointer and truncated scenarios. |
| Real private corpus (`private/*.pdf`) | six plans, read in place | **3 — legitimately hard to test** | Cannot be committed or read by CI (privacy rule); the opt-in `./scripts/validate-local-corpus.sh --differential` is the release evidence and the generated corpus is the CI substitute. This is the design's recorded interface-obstacle fix. |
| Dead code | — | **1 — none found** | The structure modules are reachable from `Document::load`/`document::extract`; `Document::trailer()` is used at `document.rs:105`; `as_f64` is used by `font.rs`/`document.rs:307`; every `LengthSpec` arm is handled; every fixture variant is used by the differential suite. |

Redundant unit tests: none added. The differential suite is the design's
internal equivalence evidence (every word, every page), not a duplicate of a
Gherkin scenario; the golden test is pre-existing. The line between the two
buckets above is deliberate: the malformed-input guards reject documents the
supported family cannot supply, and their observable is the already-pinned
refusal, so they are bucket 3; the PNG row-filter arms are accepted encodings a
valid plan may carry, so they are bucket 2. No finding is excused by a category
outside the three buckets; the one bucket-2 gap (F6) was remediated in
`a634668` and re-verified.

## Scenario execution evidence

This change has not been archived, so no archive receipt exists. `givn check
review` requires the change-level task receipt, which `givn check tasks` wrote
for this review:

```
$ givn check tasks --change own-the-pdf-file-structure
[Summary]
4 features
145 scenarios (145 passed)
617 steps (617 passed)
  ✓ change verification receipt: 145 scenario(s), 0 failed, 0 skipped at 8ab4806e9c5fa2c140f7a9f5e5f8d8cdf8bb4926
  ✓ verify passed
```

`verification/tasks-verification.json`: `scope: regular`, `total: 145`,
`passed: 145`, `failed: 0`, `skipped: 0`, `head: 8ab4806`. The e2e scope is the
recorded run in `tasks.md`:

```
$ ./run-tests.sh --e2e
2 features / 2 scenarios (2 passed) / 10 steps
exit=0
```

The archive will merge both scopes into the verification receipt. The nearest
archived per-scope receipt (`replace-pdf-extraction-backend`: regular 140,
e2e 2, proven) is consistent.

Verification run in this review (targeted, not the full-suite authority):

```
$ cargo test -p duty2cal-pdf
test result: ok. 19 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

$ ./scripts/validate-local-corpus.sh --differential
test differential_tests::real_corpus_matches_the_oracle ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 19 filtered out

$ DUTY2CAL_LOCAL_CORPUS=$PWD/private cargo test -p duty2cal-pdf -- --ignored --nocapture
PASS …/MLP-2026-06.pdf … MLP-2026-11.pdf   (six plans)
test differential_tests::real_corpus_matches_the_oracle ... ok

$ ./run-tests.sh --tags "@own-the-duty-plan-extraction and not @wip"
1 feature / 5 scenarios (5 passed) / 25 steps (25 passed)
exit=0

$ ./run-tests.sh --name "A plan whose cross-reference pointer lies outside the file is refused as unreadable"
1 feature / 1 scenario (1 passed) / 5 steps (5 passed)
exit=0
```

**Receipt status:** the on-disk change-level receipt (145/145 at 8ab4806)
predates `a634668`; because that commit changes tested content (two test cases
and documentation, no scenario or production behaviour), it is stale by
construction until this review artifact is committed. The orchestrator then
runs `givn check tasks --change own-the-pdf-file-structure` to regenerate it;
the archive receipt will merge the regular and e2e scopes.

## Overlap dispositions

`givn lint --change own-the-pdf-file-structure` reports exactly one shape match,
between the permanent truncated-document scenario and the delta's bad-pointer
scenario:

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A truncated document is refused and the process does not abort | A plan whose cross-reference pointer lies outside the file is refused as unreadable | variant |

The two have the same refusal class as their observable but different
malformed inputs (a truncated file versus a cross-reference pointer outside the
file); both are kept as distinct variants of the same refusal behaviour.
Neither duplicates the `@e2e` action, and no `@givn.removed` scenario exists, so
there is no removed+added pair to classify.

## Split-or-keep

No delta scenario exceeds the gate's long-scenario threshold: `givn lint`
reports no long-scenario finding (only the DISPOSITIONED shape advisory and the
D7 `@e2e` cross-reference warning). The outline's four Examples are input
variation of one behaviour, not four overlong scenarios.

| Scenario | Decision |
|---|---|

## README impact decision

README-IMPACT: none

`git diff d8a46f8..HEAD -- README.md` is empty. The public support statement,
the CLI invocation, its flags and its outputs are unchanged; the third-party
notices and the lockfile still name `lopdf` through the dev oracle, and the
README does not enumerate dependencies at that level. No README section needs
refreshing.

## Findings and remediation

**F6 — the PNG predictor's None and Average row-filter arms were unpinned
(bucket 2) — RESOLVED (committed as `a634668`).** `decode_row`
(`crates/duty2cal-pdf/src/reader/structure/filter.rs:242-266`) implements all
five PNG row filters. At review time the differential corpus exercised Up (2,
the default in every predicted fixture), Sub (1,
`png_sub_filter_xref_stream_matches_the_oracle`) and Paeth (4,
`png_paeth_filter_xref_stream_matches_the_oracle`); the None (0) and Average (3)
arms were executed by no committed test, and the six-plan private corpus could
not reach them either — its only predicted stream
(`private/MLP-2026-10.pdf`, `/DecodeParms /Columns 5 /Predictor 12`) uses row
filter 2 on every row. Both are valid encodings the reader accepts, so this was
missing test coverage, not a hard-to-test exception, and the remediation commit
`8ab4806` had added Sub and Paeth only.

The remediation is committed and was re-verified here:

- `png_none_filter_xref_stream_matches_the_oracle` (`png_filter: 0`) and
  `png_average_filter_xref_stream_matches_the_oracle` (`png_filter: 3`) in
  `crates/duty2cal-pdf/src/differential_tests.rs:328-349`, both through the
  existing `compare_with_oracle`. The two arms of `decode_row` are now pinned.
- `cargo test -p duty2cal-pdf`: 19 passed, 0 failed, 1 ignored — both new cases
  green.
- `./scripts/validate-local-corpus.sh --differential`: the six private plans
  stay green (one test passed, 19 filtered out).
- The docs list all four filters: `design.md:121` (Files row) and `:406`
  (Differential and corpus), `docs/arc42/05-building-block-view.md:88`,
  R-078 (`11-risks-and-technical-debt.md:94`) and R-082 (`:99`).
- The implementer reports `cargo fmt --check` and clippy clean; not re-run
  here.

The design-review re-review is recorded in the same commit (`design-review.md`,
`## Re-review after the F6 coverage finding`: the two arms verified against the
implementation, the corpus now names None, Sub, Average and Paeth, verdict
PASS), so the changed `design.md` is re-reviewed. The only remaining mechanical
step is the verification receipt: `a634668` changed the tested content, so the
on-disk receipt (written at `8ab4806`) is stale until this review artifact is
committed and the orchestrator runs `givn check tasks --change
own-the-pdf-file-structure` and then `givn check review --change
own-the-pdf-file-structure`.

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, every implementation task
      revision verified, every promised component exists, no downgraded e2e
      scenario, no HTTP/fetch substitution, no over-produced e2e scope, no
      `@e2e` tag removed).
- [x] Scenario execution evidence recorded per scope: the change-level receipt
      had regular 145/145 at 8ab4806 and the e2e scope (2) is recorded in
      `tasks.md` and verified against the assembled corpus; `a634668` is
      test-only, so the counts hold, and `givn check tasks` regenerates the
      stale receipt after this review artifact is committed. The full suite was
      not re-run by this review beyond the gate's own `givn check tasks`.
- [x] Every coverage gap classified and resolved — F6's two arms are pinned by
      the added cases (19 passed, 1 ignored); the malformed-input arms remain
      bucket 3 with their recorded reason.
- [x] No `@wip` tags remain in the delta (`grep -rn "@wip"` finds none).
- [x] The delta spec contains no implementation-layer detail (domain terms:
      supported plan, file structure, accepted spelling, published calendar,
      failure report, exit status).
- [x] Local run command starts the full stack cleanly — N/A for a one-shot CLI
      with no digital twin; the committed transcript is the run evidence.
- [x] The exact files `verify.e2e_command` invokes were identified and read;
      the tree (including untracked files) was searched for a parallel e2e
      implementation — none exists.
- [x] `verify.e2e_command` differs from `verify.command`; the e2e scope is a
      strict subset (2 vs 145, and the assembled-corpus count is 2).
- [x] Implementation (commands, file layout, framework/driver) matches
      `design.md`; the F2–F5 and F6 design corrections were each re-reviewed
      (`design-review.md`), never kept silently.
- [x] Interaction coverage verified: one inventory entry → one matrix row → one
      existing `@e2e` scenario → the promised `std::process::Command` driver.
- [x] Design.md enumerates the internal primitives this change introduces (the
      object model row, the Files table and the structure module surface); no
      primitive the steps rely on is omitted.
- [x] arc42 slug-mapping verified: every slug in `arc42.md` maps to a real
      `docs/arc42/<slug>.md` file, and ADR-0008 is the only ADR touched.
- [x] No finding is excused with a category outside the three coverage buckets.
- [x] Design review re-run for the changed `design.md` — the F6 re-review is
      recorded in `design-review.md` in `a634668`.

Gate result at review time (the review artifact is not yet committed, so the
receipt from `8ab4806` is stale by construction):

```
$ givn check review --change own-the-pdf-file-structure
Mandatory checks for 'review':
  ✓ integrity passed
  ✓ run declaration passed
net delta: 2 added, 0 modified, 0 removed
  ✓ overlap dispositions passed
  ⚠ proposal.md has no filled '## Capability Routing' table — the delta
    capabilities [own-the-duty-plan-extraction] are undeclared; record
    decisions (EXTEND <capability> / NEW in <usecase-id>)
error: change verification receipt is stale for 'own-the-pdf-file-structure':
  - tested content changed …
Run `givn check tasks --change own-the-pdf-file-structure` again.
exit=1
```

Everything else in the gate passes; the stale receipt is regenerated by
`givn check tasks` once this review artifact is committed. The routing warning
is the recorded three-column advisory above, not an error.

REVIEW: PASS
