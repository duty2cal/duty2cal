# Design Review: replace-pdf-extraction-backend

- Date: 2026-10-28
- Plan under review: `proposal.md`, `specs/own-the-duty-plan-extraction/`
  (use case and `.feature`), `specs/export-duty-plan-calendar/usecase.md`,
  `design.md`, `arc42.md`, `docs/arc42/adr/ADR-0008-own-the-duty-plan-extraction.md`
  and the chapters it changed.
- Method: a fresh-context grilling subagent that did not write the plan,
  followed by a fresh-context hardening subagent; the orchestrator verified every
  edit and the domain decisions. No production code, `tasks.md` or `proposal.md`
  was edited. The single `@e2e` tag was preserved.

## Grilling questions

### Q1 — Does the size gate survive landing?

- Status: closed 2026-10-28 (operator)
- Blocks: the acceptance rule in the measurement decision (D6) and the
  traceability claim that the measurement is CI evidence.
- Context: the design measured the landed Windows executable and browser bundle
  against "the same revision rebuilt with the previous extraction restored from a
  base revision", refused a base with no extractor, and resolved a push's base as
  `git merge-base HEAD origin/main`. Every CI job runs on every push and pull
  request. Once this change is on main, the base carries no `pdfplumber`, so both
  `windows-gnu-build` and `browser` would fail on main and on every later pull
  request; only the introducing review could ever produce the number. Tightening
  the 2,097,152-byte budget was excluded because the proposal's Out of Scope
  forbids a size ceiling.
- Question: what should the measurement do after this change has landed?
- Options:
  1. **Measure only while the base carries the extractor.** The script inspects
     the base tree's `crates/duty2cal-pdf/Cargo.toml`; with `pdfplumber` present
     it builds the baseline and writes `status: "measured"`, without it writes
     `status: "not applicable"` and exits 0.
     In practice: the introducing review reports base, landed and saved bytes;
     the next pull request reports `not applicable`.
     Gain: CI stays green and no later change is blamed on the extraction.
     Cost: after landing there is no automated size gate, so a regression under
     the existing budget is silent.
  2. **Pin the pre-removal revision.** Every push re-measures against a recorded
     revision that still carries the extractor.
     In practice: six months later the report mixes this change with later work.
     Gain: the delta after a later regression is visible.
     Cost: unrelated later changes land in the number; the baseline is ever older.
  3. **Measure once, outside CI.** Produce the report once for the introducing
     revision and record it with the change.
     In practice: a manual report, no CI step.
     Gain: no CI time or baseline-resolution logic.
     Cost: nothing regenerates the evidence and the landed revision has none.
- Resolution: option 1. Chosen: the measurement reports `status: "measured"` or
  `status: "not applicable"` and the introducing revision's report accepts a
  positive saved-bytes figure. Rejected: the pinned revision, because the delta
  stops attributing to the extraction; measuring once outside CI, because the
  landed revision would carry no automated evidence at all. The consequence is
  recorded as R-079 in chapter 11 and in the use case's measurement rule; the
  proposal's ceiling exclusion is untouched.

### Q2 — Is the use case one actor goal?

- Status: closed 2026-10-28 (operator)
- Blocks: the use-case scope rejection criteria and the design's traceability
  table.
- Context: the use case named seven main-flow steps (freeze, build and measure,
  read, refuse, review, accept/reject, land), a five-outcome success guarantee,
  and a trigger naming a mechanism ("behind the existing source port"). The
  review instruction fails a use case whose main flow names three or more actor
  goals, whose success guarantee names two outcomes, or whose goal, trigger or
  guarantees name a mechanism.
- Question: should the use case own the build/measure/accept/land steps, or only
  the reading?
- Options:
  1. **Shrink to the reading goal.** Four main-flow steps: freeze the subset;
     read a supported plan into the accepted calendar, warnings and diagnostics;
     refuse a construct outside the subset by name; review and accept or reject.
     Building and measuring both targets, the dependency split and landing stay
     as Rules, which already stated them.
     In practice: one page tells a maintainer what "supported" means and what
     evidence must exist before landing.
     Gain: one leaf that passes every criterion; the proposal's routing is intact.
     Cost: the measurement is no longer an actor step; its evidence lives in the
     Rules and the review artifacts.
  2. **Split the measurement into its own capability** under
     `measure-distributed-executable-size`, with an inventory row and an `@e2e`.
     In practice: a second consumer action whose e2e invokes the measurement
     script.
     Gain: each leaf is one goal and the size claim gets an owner.
     Cost: a second capability and routing row, and an e2e for a two-build
     measurement the Gherkin runner cannot run inside the browser job.
- Resolution: option 1. Chosen: the use case keeps the reading goal, one outcome,
  and a de-mechanised goal and trigger; the measurement, dependency split and
  landing stay as Rules. Rejected: the split, because it turns a build operation
  into a consumer interaction and its e2e cannot execute the measurement.

### Q3 — What numeric result accepts the replacement?

- Status: closed 2026-10-28 (operator)
- Blocks: the acceptance rule in D6 and the use case's success guarantee.
- Context: the design accepted on "a positive saved-bytes figure" while the
  proposal says the artifacts become "substantially smaller" and the measured
  expectation is 61–63 percent. A positive-only rule would accept a one-kilobyte
  saving.
- Question: what numeric result accepts the replacement?
- Options:
  1. **Relative floor of 50 percent** on the Windows executable and the wasm
     module. In practice: 860,000 B against 2,233,584 B passes at 61.5 percent;
     1,900,000 B rejects at 14.9 percent. Gain: expresses "substantially".
     Cost: a number to defend and revisit.
  2. **Absolute 1,000,000 bytes saved on Windows** and any positive wasm delta.
     In practice: 1,033,584 B saved passes at 46 percent. Gain: stated in
     delivered bytes. Cost: brittle as unrelated code grows.
  3. **Positive saved bytes only.** In practice: a 3,584-byte saving passes.
     Gain: no number to defend. Cost: "done" stays ambiguous.
- Resolution: option 3. Chosen: a positive saved-bytes figure on the introducing
  revision's measured run accepts; zero or negative reopens the backend decision.
  Rejected: the 50-percent floor and the 1,000,000-byte floor, because the
  operator chose not to introduce a numeric ceiling into this change; the
  measured expectation stays evidence, not a gate.

### Q4 — Is local-only real-corpus equivalence acceptable?

- Status: closed 2026-10-28 (operator)
- Blocks: the deferral of a mandatory equivalence check and the D3 evidence chain.
- Context: the design makes the comparison against the six real plans an opt-in
  `#[ignore]` test run by the maintainer before a release; CI compares only a
  generated corpus that reproduces the shape, not the generator's bytes. The
  design states the residual risk (R-078) but recorded no operator decision.
- Question: is the release-time local run acceptable as the equivalence evidence?
- Options:
  1. **Accept with a recorded decision and a release-checklist step.** In
     practice: before tagging, the maintainer runs
     `./scripts/validate-local-corpus.sh --differential` and records its output.
     Gain: the plans never leave the machine and no infrastructure is added.
     Cost: generator-specific drift is caught near release, not on the push.
  2. **Add a private-corpus CI runner** that mounts `private/` and runs the
     ignored test. In practice: a self-hosted job fails a push whose extraction
     drifts on a real plan. Gain: real bytes on every push. Cost: a second runner
     class, corpus access control, and a wider privacy blast radius; fork pull
     requests cannot run it.
- Resolution: option 1. Chosen: the local run is the release evidence, recorded
  in D3 with the exact command. Rejected: the private-corpus runner, because the
  privacy rule's blast radius grows for a check that the release already
  performs.

### Q5 — How is the browser path's named refusal evidenced?

- Status: closed 2026-10-28 (operator)
- Blocks: the failure contract's "each proved by a scenario or a test" claim and
  the interface-classification branch.
- Context: the design asserted that the WASM result carries the same reason and
  diagnostic text, but the only new `@e2e` scenario drives the CLI and the
  existing `@browser` scenario converts a supported plan. Nothing exercised the
  browser half of the new refusal.
- Question: how should the browser path's named refusal be evidenced?
- Options:
  1. **Test the WASM result.** One Rust test in `ffi/wasm/src/lib.rs` converts a
     generated out-of-subset plan and asserts the named construct and the
     two-line text. In practice: `wasm_convert` on a Type3 plan returns
     `Reason: the source uses a PDF construct outside the supported family:
     Type3 font on page 1`. Gain: cheap, runs in the existing test job, and the
     claim becomes true. Cost: proves the value, not the rendered page.
  2. **Add a `@browser` scenario.** In practice: the page's failure state is
     asserted for a rotated plan. Gain: proves the rendered surface. Cost: a
     second bundle build and Playwright run inside the 20-minute browser job.
  3. **Record the page as unaffected** and delete the WASM claim. Gain: no work.
     Cost: the new wording reaches the browser with no evidence.
- Resolution: option 1. Chosen: the `ffi/wasm` test is the browser-path evidence
  and the design names it; the page's markup, copy and interaction are unchanged.
  Rejected: the browser scenario, because the rendering is not what changed and
  the browser job's cost is real; declaring the page unaffected without a test,
  because the obligation would then be unproved.

## Branch findings and dispositions

| Branch | Finding | Severity | Disposition |
|---|---|---|---|
| Scope | The refusal scenarios did not assert the page the proposal and use case promise. | major | Fixed: outline Then asserts source, construct and page; encryption split into its own scenario, which has no page. |
| Scope | Acceptance ("positive saved bytes") left "substantially smaller" ambiguous. | major | Fixed by operator decision Q3; rejected floors recorded in D6. |
| Scope | `page rotation` was in the spec but not the proposal's illustrative list. | none | No issue: the use case's Rules own the closed construct list. |
| Tech choices | `lopdf` 0.44 kept although 0.46 exists. | none | No issue: 0.46 removes no encryption stack, and 0.44 is the measured version. |
| Tech choices | `default-features = false` and Flate availability. | none | No issue: `flate2`/`miniz_oxide` are unconditional; `pdfplumber-parse` already uses the same setting. |
| Tech choices | The exact-size fixture mechanism was unspecified; the old builder pads an uncompressed content stream. | major | Fixed: sized fixtures emit an unfiltered stream with a byte-counted comment and corrected `/Length`; both reference calendars stay unchanged. |
| Tech choices | `THIRD-PARTY-NOTICES.md` was "expected unchanged". | major | Fixed: row is `changed`; the MPL-2.0 block lists every workspace crate and `cargo about` is byte-compared in CI. |
| Tech choices | `cliff.toml` changes named only the allow-list. | major | Fixed: the row names the allow-list plus the feat/fix/perf/revert and breaking matchers, and the `.*` skip consequence. |
| Missing scenarios | Eight named refusals were asserted only by unit tests outside every coverage scope. | major | Fixed: eight Examples rows added; all named constructs now go through the CLI. |
| Missing scenarios | Multi-page Thens were not concrete. | major | Fixed: exactly two TK events, dated 12 and 13 June 2026, no duplicate. |
| Missing scenarios | The graph split and the size claim had no scenario. | major | Resolved by Q2: the measurement and split are Rules with script evidence, not actor steps; the traceability table says so. |
| Testability | Five scenarios already hold for the old backend. | major | Fixed: the design labels them as regression guards and names the stub pattern that gives every scenario a behavioural RED. |
| Testability | No not-implemented stub pattern. | major | Fixed: `reader::extract` returns `UnsupportedConstruct { UnknownOperator("unimplemented"), page: Some(1) }` first. |
| Testability | `--name` cannot select one Scenario Outline example. | minor | Fixed: the design names the `--tags` workaround and the exactly-one guard. |
| E2E fidelity | The comparison basis was unstated. | minor | Fixed: "identical … after the agreed normalisation". |
| E2E fidelity | Inventory row, `@e2e` title and matrix row match; the driver is the real CLI. | none | No issue. |
| Interaction coverage | One inventory row, one matrix row; no duplicate action. | none | No issue. |
| Risk | The most likely failure is the owned word geometry never converging and the tolerance being loosened. | none | Mitigation already stated: fix the extraction or refuse; never loosen. Recorded as R-076. |
| Use case / Personas | Oversized leaf, composite guarantee, mechanism vocabulary. | major | Fixed by Q2. |
| Use case / Personas | All three Personas exist; none invented or promoted; no biography is an actor. | none | No issue. |
| Decision ledger | D1–D6 resolved; no seed, so no seed reference owed. | none | No issue. |
| Visual design contract | `Interface: terminal` was asserted without arguing the opposite classification. | major | Fixed: the contract now states why no Web UI classification holds. |
| Deferrals | Real-corpus equivalence deferred without a recorded decision. | major | Fixed by Q4 and D3. |
| Persona disposition | Per-persona relevance existed without a named evidence artifact. | minor | Fixed: each bullet names its artifact. |
| ADR qualification | The five gates pass; the cheaper-home statement was implicit. | minor | Fixed: ADR-0008 names the cheaper home explicitly. |
| Architecture documentation | All 12 rows were re-derived and agree; all chapter files exist with real content; no ASCII art; every "Bad, because" consequence is in chapter 11. | none | No issue. |
| Ubiquitous language | `Positioned word`, `Named refusal` and `Differential` were used without glossary rows. | minor | Fixed: three glossary rows added. |
| Measurement baseline | The gate was red on every push after landing. | blocker | Fixed by Q1: `status: not applicable` rule, explicit base resolution, browser timeout raised to 35 minutes. |
| Measurement standing protection | The 2,097,152-byte budget leaves the saving unguarded after the one-shot gate. | major | Accepted by Q1/Q3 and recorded as R-079; the proposal forbids a size ceiling. |
| Lock / workspace member | Lock movement and the fixture crate's workspace entry. | none | No issue; the design's parenthetical covers it. |
| `cargo tree` target checks | Valid flags; targets installed; `deny.toml` wrapper rule stays satisfied. | none | No issue. |

## Hardening applied

- `design.md`: D3 records the operator decision on the local corpus; D6 is
  rewritten for the one-shot measurement, explicit base resolution, the
  `measured`/`not applicable` status, the 35-minute browser timeout and the
  positive-only acceptance with the rejected floors; the changed-files table
  corrects `THIRD-PARTY-NOTICES.md` and `cliff.toml`, adds `ffi/wasm/src/lib.rs`
  and `2098-06.ics`, and names the exact-size fixture mechanism; the test-runner
  section names the stub pattern, the outline `--name` guard and the RED and
  regression labels; the failure contract names the WASM test; the use-case
  traceability is re-anchored to the four-step flow and the Rules; the Persona
  impact names each artifact; the Visual Design Contract argues the interface
  classification.
- `specs/own-the-duty-plan-extraction/usecase.md`: goal and trigger de-mechanised,
  main flow reduced to four steps, extensions renumbered, one-outcome success
  guarantee, and the one-shot measurement rule added to the Rules.
- `specs/own-the-duty-plan-extraction/own-the-duty-plan-extraction.feature`:
  refusal Then asserts the page; encryption has its own scenario; eight Examples
  rows added for the remaining named constructs; the e2e comparison names the
  normalisation; the multi-page Thens assert concrete dates and a count.
- `docs/arc42/adr/ADR-0008-own-the-duty-plan-extraction.md`: the cheaper home is
  named in `## Qualification`.
- `docs/arc42/12-glossary.md`: `Positioned word`, `Named refusal` and
  `Differential` rows added.
- `docs/arc42/06-runtime-view.md`: the measurement flow gains the
  not-applicable branch and the one-shot consequence.
- `docs/arc42/10-quality-requirements.md`: QS-047 names the page when one is
  known; QS-048 records the not-applicable rule.
- `docs/arc42/11-risks-and-technical-debt.md`: R-079 records the one-shot
  measurement and the absent standing size gate.

## Verification

- `givn lint --change replace-pdf-extraction-backend`: exit 2, nine findings,
  every one the expected `[WIP]` tag on the new capability's scenarios.
- The one `@e2e` scenario remains tagged; no tag was added or removed.
- `tasks.md` was not written; `proposal.md` was not changed.
- The use case now has four main-flow steps, one success outcome, no mechanism in
  its goal or trigger, and every extension references an existing step.

## Residual limits (recorded, not hidden)

- The real-generator corpus is compared only by the opt-in local run before a
  release (Q4, R-078).
- The size measurement applies to the introducing revision; after landing no
  automated size gate remains (Q1, R-079).
- A positive saved-bytes figure accepts; a one-kilobyte saving would pass (Q3).
- The synthetic corpus reproduces the real profile's shape, not its bytes.

DESIGN-REVIEW: PASS
