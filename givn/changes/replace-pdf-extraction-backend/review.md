# Review: replace-pdf-extraction-backend

## Fabrication audit

### Step-body check

0 empty or no-op step bodies across 2 delta step-definition files:

| File | Bound steps | Empty bodies |
|---|---|---|
| `apps/cli/tests/steps/own_the_duty_plan_extraction.rs` | 24 | 0 |
| `apps/cli/tests/steps/e2e/own_the_duty_plan_extraction.rs` | 4 | 0 |

The registration files (`steps/mod.rs`, `steps/e2e/mod.rs`) carry only `pub mod`
lines. No `unimplemented!`, `todo!`, bare `pass`/`return`, or empty body remains
in the delta step tree.

### `@e2e` tag integrity

One `@e2e` scenario is planned and present:

- `Release maintainer converts the supported plan through the executable and receives the accepted calendar`
  (`@givn.added @e2e`). The tag was not removed; the outline, refusal,
  multi-page and warning scenarios are regular scenarios.

### Checked task → commit

| Scenario | Commit | Implementation touched |
|---|---|---|
| 1. object-stream equivalence | `3673bee` | reader modules, fixtures crate, table split, differential tests, adapter, support re-exports |
| 2. named refusals outline | `4d297fc` | generated refusal variants, outline step bindings (typed mapping landed in `3673bee`) |
| 3. encrypted refusal | `ef57e13` | `reader/document.rs` `/Encrypt` refusal, step bindings |
| 4. unsupported family | `b4833a4` | permanent step implementation (regression guard; reader family path landed in `3673bee`) |
| 5. truncated | `a7e462e` | permanent step implementation (regression guard; loader mapping landed in `3673bee`) |
| 6. privacy | `ee4411a` | permanent step implementation (regression guard; Type3 refusal landed in `3673bee`) |
| 7. multi-page | `f4b8730` | permanent step implementation and fixture (page iteration landed in `3673bee`) |
| 8. warning | `f1b00a5` | permanent step implementation (regression guard; warning path unchanged) |
| 9. `@e2e` accepted calendar | `5bc4e7e` | e2e steps, transcript, feature |
| wiring | `a42d56d` | graph guard, measurement scripts, CI, WASM test, notices, README |

No checked task records a spec-only or placeholder-only revision. The
regression-guard scenarios are design-approved to add no production change;
their commits carry the real assertions in the permanent test tree, and the
production behaviour each protects is in `3673bee`.

### Promised components

Present: `reader/{mod,document,content,font,cmap,words}.rs`, `table.rs`,
`differential_tests.rs`, `duty2cal-pdf-fixtures`, `Construct` /
`SourceError::UnsupportedConstruct`, the three scripts, the CI wiring, the WASM
facade test, both delta step files, the transcript, ADR-0008, and the arc42
chapter updates.

### Strict-mode proof

Present in `tasks.md`: an undefined step exits 1 ("Step doesn't match any
function"), the probe is deleted, and the `not @wip` baseline exits 0. The runner
calls `.fail_on_skipped()`.

### E2E real-interface check

The single `@e2e` scenario's Then steps assert the published `.ics` content, its
equality with the committed accepted calendar after CRLF/folding normalisation,
the `Written:` line, and the process exit status. It drives
`env!("CARGO_BIN_EXE_duty2cal")` through `std::process::Command`. No
repository-only assertion and no HTTP shortcut.

### Browser-UI classification, considered in the opposite direction

A rendered page would make this a Web UI. It does not hold: the change renders
no page, changes no markup or copy, and its interface is the terminal. The one
browser-side claim — the WASM result carries the same named refusal — is proved
by the native facade test, not a screen.

### E2E command and scope

`givn/commands.yaml`:

- `verify.command: "./run-tests.sh"`
- `verify.e2e_command: "./run-tests.sh --e2e"`

They are distinct. The e2e command assembles the corpus and runs
`apps/cli/tests/features_runner.rs` with `@e2e and not @browser`. The tree
contains exactly one `@e2e` step implementation for this capability
(`apps/cli/tests/steps/e2e/own_the_duty_plan_extraction.rs`); no untracked or
parallel implementation exists. Recorded per-scope counts: regular 140, e2e 2
(e2e is a strict subset).

### Design conformance

`design.md` names two step files, the fixture crate, the reader modules, the
`lopdf`/dev-oracle split, the three scripts and the CI wiring; all exist where
named. The only `design.md` edit during review is the Step Definitions table
reordered to put the file path in the second column, because the integrity hook
resolves that column; no decision, command, framework or file choice changed.
`givn status` still records `design-review` as done.

### Interaction coverage

| Use-case inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| convert the supported duty plan through the built executable and read the calendar it publishes | present — CLI (terminal) | Release maintainer converts the supported plan through the executable and receives the accepted calendar | `std::process::Command` on `env!("CARGO_BIN_EXE_duty2cal")`; asserted against the published file and exit status |

One normalized action, one matrix row, one `@e2e` scenario. The refusal,
encryption, family, truncation, privacy, multi-page and warning scenarios are
variants of the same conversion action and stay regular.

FABRICATION AUDIT: CLEAN

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 01 requirement 6: owned reading, test-only oracle | `01-introduction-and-goals.md` | affected | D1/D3 | 1, wiring | reader modules; `[dev-dependencies]` oracle | yes |
| 02 `lopdf` + oracle + frozen subset constraints | `02-architecture-constraints.md` | affected | Technology decisions | 1, wiring | `Cargo.toml`, graph check | yes |
| 04 PDF-reading strategy changed | `04-solution-strategy.md` | affected | D1–D4 | 1–9 | reader + differential | yes |
| 05 `duty2cal-pdf` level-2 reader blocks | `05-building-block-view.md` | affected | New files table | 1 | `reader/`, `table.rs` | yes |
| 06 construct refusal and backend measurement flows | `06-runtime-view.md` | affected | D4, D6 | 2–3, wiring | refusal path; measurement scripts | yes |
| 08 owned extraction / measured replacement concepts | `08-crosscutting-concepts.md` | affected | D1–D6 | all | code + scripts | yes |
| 09 ADR-0008 registered, no ADR body in chapter | `09-architecture-decisions.md` | affected | ADR section | wiring | `docs/arc42/adr/ADR-0008-…` + index row | yes |
| 10 QS-047/QS-048 and quality nodes | `10-quality-requirements.md` | affected | D3/D6 | 1–9 | differential + measurement | yes |
| 11 risks R-076…R-079, TD-030 | `11-risks-and-technical-debt.md` | affected | D6 | wiring | one-shot measurement behaviour | yes |
| 12 glossary terms | `12-glossary.md` | affected | reader sections | all | glossary rows | yes (two source-path/descriptor corrections made during review) |
| ADR-0008 decision: own the frozen subset, keep object/xref layer | `docs/arc42/adr/ADR-0008-own-the-duty-plan-extraction.md` | registered | D1 | 1 | `lopdf` + project interpretation | yes |

Sign-off checks:

- [x] Every chapter the impact assessment marks affected has current content.
- [x] Every chapter independently derived from the implementation is marked affected.
- [x] ADR-0008 is registered and linked from the index.
- [x] The glossary records every new term and every source path is current.
- [x] No contradiction remains between durable docs, `arc42.md`, `design.md`, `tasks.md` and the implementation.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

| Term | Specs | Design | Code | Glossary |
|---|---|---|---|---|
| Owned extraction | yes | yes | `crates/duty2cal-pdf/src/reader/` | `12-glossary.md` |
| Test oracle | yes | yes | `[dev-dependencies]` oracle + graph check | yes |
| Frozen supported subset | yes | yes | `Construct`, reader checks | yes |
| Positioned word | yes | yes | `PositionedWord` | yes |
| Named refusal | yes | yes | `SourceError::UnsupportedConstruct` | yes |
| Differential | yes | yes | `differential_tests.rs` | yes |

No unrecorded synonym replaces a glossary term in the delta specs, design, tasks
or code. `Word` (the oracle type) appears only in the test oracle, as the
glossary's exclusion states.

UBIQUITOUS LANGUAGE: CLEAN

## Coverage

No archive receipt exists yet for this change; classification is from static
inspection, and the archive will publish the measured result.

1. Dead code: none found. `PageWords.page` is read by the differential
   diagnostics; no unused reader module or helper remains.
2. Missing test coverage: one gap found and remediated in this review — the
   design promised a multi-page differential entry in §7, and
   `multipage_plan_matches_the_oracle` was missing. It was added to
   `crates/duty2cal-pdf/src/differential_tests.rs` and passes.
3. Legitimately hard to test: the real-corpus differential reads `private/*.pdf`
   in place, which cannot be committed or read by CI. It stays an opt-in local
   release-evidence command and is backed by the generated corpus in CI.

Redundant unit tests: none added. The differential tests are the design's
internal equivalence evidence, not duplicates of a Gherkin scenario.

## E2e coverage

Canonical policy read from `givn instructions specs --change
replace-pdf-extraction-backend`. One normalized inventory action, one `@e2e`
scenario, real CLI driver, real-interface assertions, and `verify.e2e_command`
strictly scoped to `@e2e`. Variants (refusals, encryption, family, truncation,
privacy, multi-page, warning) are regular scenarios.

## Use-case and Persona conformance

- Goal: one verb phrase (own the reading of the supported duty plan).
- Primary actor: release maintainer, named by the trigger.
- Capability placement: the reading capability owns the extraction; the export
  capability keeps its two conversion actions.
- No mutation under an observing goal; every extension references its main-flow
  step and adds no new success outcome.
- Preconditions are supplied, not established by the flow; the success guarantee
  is not assumed.
- Names are outcome-shaped, not component/channel-shaped.
- Personas: project-maintainer--release-owner (Actor lens), crew-member--personal-calendar-owner (calendar/privacy lens), contributor--web-reviewer (browser-bundle lens). No invented Persona was promoted.

PERSONAS: project-maintainer--release-owner
PERSONAS: crew-member--personal-calendar-owner
PERSONAS: contributor--web-reviewer

## Visual review

Contract: `Interface: terminal`; evidence is the committed transcript at
`givn/changes/replace-pdf-extraction-backend/evidence/visual/release-maintainer-converts-the-supported-plan-through-the-executable-and-receives-the-accepted-calendar/transcript.txt`.

Fresh-context Persona reviews:

| Persona | Evidence seen | Goal reachable? | Finding | Disposition |
|---|---|---|---|---|
| project-maintainer--release-owner | transcript (command, `Written:` path, exit status, comparison) | yes | none | — |
| crew-member--personal-calendar-owner | transcript + Then-step semantics | yes | none | — |
| contributor--web-reviewer | transcript + shared-path semantics | yes | none | — |

State evidence seen: the terminal conversion state (command, published path,
success exit, normalised comparison). No screenshots exist because no page is
rendered.

VISUAL-REVIEW: PASS

## Overlap dispositions

No overlap finding was reported for the delta scenarios.

| Finding pair | Decision |
|---|---|

## Split-or-keep

No delta scenario exceeds the gate's long-scenario threshold.

| Scenario | Decision |
|---|---|

## README impact decision

README-IMPACT: updated - Development

The Development section now states the frozen subset, the test-oracle-only
status of the extractor, the graph guard and the local differential command.

## Sign-off checklist

- [x] Fabrication audit clean; all findings remediated.
- [x] Every checked task has a verified commit touching implementation source.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Per-scope execution evidence recorded (regular 140, e2e 2).
- [x] Coverage classified from inspection; archive will publish measurement.
- [x] Every coverage gap resolved; no dead code; no redundant unit test.
- [x] No `@wip` remains; no implementation detail in the spec.
- [x] Canonical E2E policy read; one normalized action, one `@e2e` scenario.
- [x] Every `@e2e` scenario asserts the real CLI output; no HTTP/fetch shortcut.
- [x] Local run command starts the converter cleanly (terminal, no services).
- [x] `verify.e2e_command` files identified and read; no parallel e2e implementation.
- [x] `verify.e2e_command` differs from `verify.command`; e2e 2 < regular 140.
- [x] Implementation matches `design.md`; the only review edit is the table column order required by the integrity hook.
- [x] Interaction coverage verified with the promised driver.
- [x] No finding excused outside the three coverage buckets.

REVIEW: PASS
