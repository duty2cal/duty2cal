# Review: windows-only-release

Scope: the release platform set narrowed from five platforms to Windows x86_64
(`publish-public-release`). The change adds no production Rust; its production
changes are the preparation script, the release workflow, the public text and
its gate, the feature-assembly helper, the durable documentation and the
release step definitions. This review covers all 58 checked tasks in the
`tasks-done` stage.

## Fabrication Audit

**Step definition scan** — every step-definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, `todo!()`/`unimplemented!()` with no
other work, or a body with no assertion/subprocess/domain reference):

- Files scanned: `apps/cli/tests/steps/publish_public_release.rs`,
  `apps/cli/tests/steps/e2e/publish_public_release.rs`,
  `apps/cli/tests/steps/export_duty_plan_calendar.rs`,
  `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`.
- Result: **0 empty or trivial step bodies across 4 files.** Every publish step
  calls the real preparation/rendering subprocess (`run_command`), reads the
  staged files, or asserts with `assert!`/`assert_eq!`; the conversion steps
  assert on the published calendar and diagnostics.

**Commit cross-check** — every checked `[x]` task in `tasks.md` has a recorded
revision that exists and touches code, tooling, or the public surface promised
by that task. The 58 boxes are grouped by task; sub-boxes share their task's
commit.

| Task (boxes) | Commit | Touches |
|---|---|---|
| 0 — setup, strict-mode proof, assembly helper (8) | `49e52c0` | `scripts/assemble_features.py`, `scripts/assemble-features.sh` (tooling) |
| B1 — public text and gate (5) | `880eed0` | README, issue form, SECURITY, `scripts/public-text-check.sh` |
| B2 — workflow narrowing (4) | `614ec8e` | `.github/workflows/release.yml` |
| B3 — durable docs + R-012 correction (3) | `1fa18f5` | 14 `docs/arc42` files including ADR-0003 |
| C1 — one-archive scenario (5) | `7dac3d1` | `scripts/prepare-release.sh` (production), shared/test steps |
| C2 — missing executable scenario (5) | `4f8feee` | step definitions only; re-baselined retained production refusal |
| C3 — checksum scenario (5) | `7b86604` | step definitions only; re-baselined retained checksum behavior |
| C4 — unreadable scenario (5) | `ba665ca` | step definitions only; re-baselined retained readability refusal |
| C5 — withdrawn-platform scenario (5) | `d237476` | step definitions only; the narrowing landed in C1 |
| C6 — e2e scenario (5) | `0e26453` | e2e steps, shared release-text step, transcript evidence |
| Final acceptance (3) | `0e0c43d`, `220cba3` | formatting evidence + task acceptance |
| Coverage receipt (1) | `19d4aee` | `coverage/*.xml` reports |

C1 is the production RED/GREEN driver for the platform narrowing; C2–C4 are
explicitly re-baselined keep-true evidence in `design.md` §"RED drivers and
re-baselined evidence", and C5's production refusal landed with C1's
`PLATFORMS='windows-x86_64'`. Their commits are not spec-only stubs: they carry
the step definitions that make the delta executable. No checked task lacks a
revision.

**Promised-architecture cross-check** — every component `design.md` said the
change would create or change exists in the tree:

| Component (from `design.md`) | Exists? | Path |
|---|---|---|
| Gherkin-aware assembly helper | yes | `scripts/assemble_features.py` |
| Stable assembly entry point | yes | `scripts/assemble-features.sh` (delegates, prints root) |
| One-platform preparation | yes | `scripts/prepare-release.sh` (`PLATFORMS='windows-x86_64'`) |
| Public-text presence/negative gate | yes | `scripts/public-text-check.sh` (14 promises) |
| One-entry release matrix + Windows smoke | yes | `.github/workflows/release.yml` |
| Windows-only public text | yes | `README.md`, `.github/ISSUE_TEMPLATE/bug.yml`, `SECURITY.md` |
| Durable documentation + amended ADR-0003 | yes | 12 chapters, `adr/README.md`, ADR-0003 |
| Publish in-process steps | yes | `apps/cli/tests/steps/publish_public_release.rs` |
| Publish e2e steps | yes | `apps/cli/tests/steps/e2e/publish_public_release.rs` |
| E2e transcript | yes | `evidence/visual/maintainer-prepares-a-windows-release-and-inspects-the-archived-executable/transcript.txt` |

**Strict-mode proof** — present with captured output in `tasks.md` §0.1/§0.2:
`.fail_on_skipped()` is on the `Cucumber` builder (`apps/cli/tests/features_runner.rs:57`);
running the then-undefined Given `an executable presented as the previously
offered platform "linux-x86_64"` produced `exit=101` with "Step doesn't match
any function", and the restored targeted run exited 0. The strict mode is
effective for this change.

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps were read:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Maintainer prepares a Windows release and inspects the archived executable (delta, added) | yes — reported archive and three-way SHA-256 equality, printed release text names the version, `tar -tzf` member list is exactly `duty2cal.exe`, extracted bytes equal the staged executable, exit status 0 | n/a — CLI (terminal), no browser interface | no |
| Maintainer generates the change log for a version and reads its newest section (permanent) | yes — reads the change log the real renderer wrote and asserts the version section and released change | n/a — CLI | no |
| Crew member converts a supported duty plan and receives a calendar file (permanent, other capability) | yes — runs the production CLI subprocess and asserts the published calendar content | n/a — CLI | no |

No e2e step sends HTTP or calls `fetch()`/`XMLHttpRequest`; the publish e2e
drives `scripts/prepare-release.sh` with `std::process::Command`, exactly as
the design names.

**E2e scope check** — the change's inventory has two distinct actions, and each
has one `@e2e` scenario; variants and refusals are regular scenarios:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| `publish-public-release` | 2 (prepare a release; generate the change log) | yes — one per distinct consumer action | no |
| `export-duty-plan-calendar` (permanent, untouched) | 1 | yes | no |

**Duplicate-implementation check** — `verify.e2e_command` is literally
`./run-tests.sh --e2e`; that script assembles the corpus through
`scripts/assemble-features.sh` and invokes
`cargo test -p duty2cal --test features_runner -- --tags @e2e`. The assembled
`@e2e` scenarios bind to
`apps/cli/tests/steps/e2e/publish_public_release.rs` and
`apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`. A tree search
including `git status` (clean, no untracked files) found no second/parallel
implementation of either capability; the in-process file shares helpers with
the e2e file but is not a second e2e driver. The strongest implementation is
the one being reviewed.

**Local environment check** — the local run command is `./run-tests.sh`
(feature assembly + cucumber-rs + the real preparation/render subprocess and a
throwaway filesystem). There is no server, database, queue, or digital twin to
start; `design.md` §"Local runnability & digital twins" records that GitHub is
the only external service and only GitHub's own automation talks to it. The
completed-suite facts supplied to this review: `./run-tests.sh` 121/121
scenarios exit 0; `./run-tests.sh --e2e` 3/3 exit 0.

**E2e isolation check** — values read literally from `givn/commands.yaml`:

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | no | 121 | 3 | yes (3 < 121) |

The counts are the final acceptance recorded in `tasks.md`; the change has not
yet been archived, so no `verification.json` receipt exists (see
§"Scenario execution evidence").

## Step Definitions

| Capability | Step definition file(s) |
|---|---|
| `publish-public-release` (in-process) | `apps/cli/tests/steps/publish_public_release.rs` |
| `publish-public-release` (e2e) | `apps/cli/tests/steps/e2e/publish_public_release.rs` |
| `export-duty-plan-calendar` (e2e, permanent) | `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` |

Runner: `./run-tests.sh` (`verify.command`). Single scenario:
`./run-tests.sh --name "<scenario title>"`.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | The delta narrows the supported platform set in Goal, Main flow, Extensions, Rules and Success guarantee; the release/immutability/change-log/publication guarantees are unchanged. | yes |
| Capability ownership and typed relationships preserved | Only `publish-public-release`; `Includes: none`, `Extends: none`; the proposal records `EXTEND publish-public-release`. | yes |
| Confirmed Persona-relevant outcomes preserved | Both confirmed Personas are in the proposal and use case; the change gives the maintainer one build/start/archive and the crew member the Windows route plus the source-build statement for other platforms. | yes |
| Actors remain distinct from Persona biographies | The only Actor is `Release maintainer`; the two Persona ids are review lenses, not Actors. | yes |
| Migration ledger and coverage evidence | Not applicable: no capability migration or retirement. | n/a |

PERSONAS: project-maintainer--release-owner
PERSONAS: crew-member--personal-calendar-owner

Use-case scope rejection criteria were checked individually and none holds:

- Goal conjunction: `## Goal` states one outcome (a release obtainable and
  traceable for Windows x86_64); `## Success guarantee` states that one
  outcome.
- Primary actor: `Release maintainer` is the primary Actor and appears in
  `## Trigger`.
- Placement: `publish-public-release` is the release goal; it does not host a
  converter or documentation capability.
- Mutation under observation: the use case declares "This is a changing use
  case: it issues a release"; its guarantees describe the issued release.
- Extensions: every extension is prefixed by its main-flow step (1, 1–2, 3, 5)
  and none adds a new success outcome.
- Preconditions: they are environmental/decision preconditions, not outcomes
  established by the flow; the trigger is the maintainer's event, not a state;
  no guarantee has an open qualifier.
- Mechanism vocabulary: `## Goal`, `## Trigger` and the guarantees name no
  command, workflow, runner or file.
- Component/channel-shaped name: `publish-public-release` names the goal, not
  a component or channel.
- Oversized leaf: `## Main flow` has seven numbered steps of one release flow,
  not three or more actor goals.
- Relationship misuse: no `Extends`/`Includes` edges.

## Visual Review

The design declares `Interface: terminal` (no rendered screen, no viewports,
no design system). Evidence kind: one committed transcript per `@e2e`
scenario.

| @e2e Scenario | Viewport | Evidence file | Present and non-empty? |
|---|---|---|---|
| Maintainer prepares a Windows release and inspects the archived executable | n/a (terminal) | `evidence/visual/maintainer-prepares-a-windows-release-and-inspects-the-archived-executable/transcript.txt` | yes (22 lines; command, archive/digest, release-text section, extraction, `tar -tzf` listing, exit status) |

A fresh-context judge reviewed the transcript once per confirmed Persona (two
separate reviewers; no Persona invented). States seen by both: the
`scripts/prepare-release.sh --version 0.1.0` invocation and its output (archive
`duty2cal-0.1.0-windows-x86_64.tar.gz` with digest, change-log/version lines),
the `cat release-notes.md` section, the `tar -xzf` extraction command, the
`tar -tzf` listing (`duty2cal.exe`), and `exit status: 0`; the scenario
directory appears only as `<scenario-dir>`.

| Confirmed Persona | Goal | Finding | Disposition |
|---|---|---|---|
| `project-maintainer--release-owner` | Prepare and inspect a release whose public claims match its evidence. | The staged release text's New capabilities entry says the release offers "a downloadable, provenance-verifiable executable for Windows, Linux and macOS", while the staged archive is Windows-only. | **Accepted as historical, not a change defect.** The transcript stages the committed `0.1.0` section, and `0.1.0` is an already-published immutable release that shipped five archives; its original text truthfully describes those assets. The change's rule (use case; README) is that only future releases are Windows-only. The scenario's asserted fact is the version line, which is present. No code or text change is owed to the future-release path. |
| `crew-member--personal-calendar-owner` | On Windows, obtain one archive that runs without installation; on another platform, read that no download is offered and the source can be built. | Same historical `0.1.0` platform sentence; otherwise the archive name, checksum, member and README route are correct. | **Accepted as historical, not a change defect**, for the same reason. The README's live route states the unsupported-platform source-build and platform-return rules; the `0.1.0` asset set is immutable. |

Baseline: the transcript shows the real command and its real output, the
scenario directory is the only redaction, the exit status is explicit, and the
release-text version line is present. No internal identifier, duplicated
content, invented error state, or motion concern applies. Both findings are
dispositioned, so the block passes.

VISUAL-REVIEW: PASS

## Retirements

N/A: the change retires no capability. The one `@givn.removed` block retires a
scenario (`Maintainer prepares a release and runs the archived executable`),
not a capability; the replacement `@e2e` scenario is the retitled
archive-inspection interaction, and the execution evidence stays with the
`windows-latest` smoke job.

## Arc42 implementation conformance

Independently derived affected chapters from the implementation, `design.md`
and `tasks.md`: all twelve chapters plus ADR-0003 are affected (the published
platform set appears in goals/constraints/context/strategy/blocks/runtime/
deployment/crosscutting/decisions/quality/risks/glossary). This matches the
change-level `arc42.md` table (12 rows, all **Yes**).

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? |
|---|---|---|---|---|---|---|
| One Windows x86_64 archive; other platforms unsupported, source-build route | `docs/arc42/01-introduction-and-goals.md:13,25-26,32-34` | Requirement 5, quality goals 5–6, stakeholder rows narrowed | Use-case traceability + Persona impact | B3 verifies the docs | commit `1fa18f5`; README/use case agree | yes |
| Windows-only target; Linux is CI/test/cross-build host, not a platform | `docs/arc42/02-architecture-constraints.md:17,18` | Constraint rows replaced; Linux host recorded | Technology decisions + Architecture impact | B3 + B2 | `prepare-release.sh` + `release.yml` build on `ubuntu-latest`; smoke on Windows | yes |
| One release-channel edge, no per-platform asset set | `docs/arc42/03-context-and-scope.md:20,28,33` | Chapter 03 affected and rewritten | Architecture impact | B3 | README one-row download table; release workflow one artifact | yes |
| Release strategy: cross-build on Linux, smoke on Windows, package once | `docs/arc42/04-solution-strategy.md:10,22,35` | Strategy and distribution rows narrowed | The preparation command / release workflow | B2, C1 | `release.yml` `build`/`smoke-windows`/`prepare`; script `PLATFORMS` | yes |
| `prepare-release.sh` refuses missing/unreadable/withdrawn platforms and stages one archive | `docs/arc42/05-building-block-view.md:52,58` | Building-block responsibilities narrowed | New/changed files table | C1–C5 | script + delta scenarios | yes |
| One-build + Windows-smoke runtime flow | `docs/arc42/06-runtime-view.md:96,110` | Flow updated | Architecture impact diagram | B2 | workflow `needs: [build, smoke-windows]` | yes |
| Deployment artifact table, runner pair, Windows crew-member OS | `docs/arc42/07-deployment-view.md:5,10,20,38,51,57` | Deployment chapter narrowed; macOS signing removed | New/changed files table | B3 | workflow + README | yes |
| Release determinism over the one Windows executable; completeness refusal names Windows | `docs/arc42/08-crosscutting-concepts.md:33,51` | Determinism and refusal narrowed | Public text / preparation sections | C1 | `tar` fixed inputs; `PLATFORMS`; prepared scenarios | yes |
| ADR-0003 amended in place (`AMEND_ADR`), register + index updated | `docs/arc42/09-architecture-decisions.md:13,20`; `adr/README.md:13,24-26` | Register row carries amended title/date | ADR qualification routing | B3 | ADR frontmatter `amended: 2026-10-17`, qualification `AMEND_ADR` | yes |
| Quality tree node, QS-007 Windows completeness, QS-008 one executable twice | `docs/arc42/10-quality-requirements.md:21,41-42` | QS-007/QS-008 narrowed | Use-case traceability | B3 | scenarios "A prepared release…", "Preparing the same release twice…" | yes |
| R-010/TD-006 closed; R-009/R-013 rewritten; R-019 source-build risk added | `docs/arc42/11-risks-and-technical-debt.md:15-16,19,25,36` | Risks updated, new R-019 | Risks and follow-ups | B3 | ADR consequences cite R-019/R-013/R-009 | yes |
| Glossary: Supported platform = Windows x86_64 only; Release archive = one per version | `docs/arc42/12-glossary.md:47,62` | Both terms narrowed | Ubiquitous language | B3 | spec/design/code use both terms; `PLATFORMS` symbol | yes |

arc42 slug mapping verified: every chapter slug in `arc42.md`
(`01-introduction-and-goals` … `12-glossary`) maps to a real
`docs/arc42/<slug>.md`; no chapter is a placeholder and no `.md` file is
missing.

- [x] Every chapter marked affected by either assessment was opened and checked
      for stale, contradictory, or placeholder content.
- [x] Every relevant Arc42 fact/decision is represented in `design.md` and
      mapped to tasks `B1`–`B3`, `C1`–`C6`.
- [x] The completed implementation follows those facts; evidence names the
      built file/command/test above.
- [x] Architecture changes discovered in the implementation are recorded in
      the durable chapters and `arc42.md`.
- [x] No mismatch remains; no documentation, design, or task change was needed.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The durable glossary (`docs/arc42/12-glossary.md`) holds every term the delta
use case, design, and code reuse: **Supported platform**, **Release archive**,
**Release preparation**, **Release text**, **Change log**, **Checksum**,
**Build provenance**, **Software bill of materials**, **Immutable release**,
**Version tag**, **Public home**, and **Stable latest-release address**. The
spec, design, README, and implementation use "Windows x86_64" and "supported
platform" consistently; the canonical value is `PLATFORMS` in
`scripts/prepare-release.sh`. No artifact introduces a rival term
("distribution platform" is absent; the old "machine platform" helpers are
removed). No glossary edit was needed by this change.

LANGUAGE CONFORMANCE: CLEAN

## Coverage

**Coverage result:** the most recent measured evidence is the committed merged
Cobertura report `coverage/cobertura-coverage.xml` (commit `19d4aee`):
**871 of 886 lines covered (98%)**, branch coverage n/a (0/0). The two source
reports are distinct instrumentation runs of the same 886-line production
denominator: non-e2e 871 covered, e2e 612 covered. The e2e report is valid
because the permanent conversion e2e spawns the production CLI
(`apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`), so the e2e scope
measures production lines even though the publish e2e itself drives a shell
script. The change adds no production Rust line, so the denominator is
unchanged.

## Coverage gap classification

15 production lines are uncovered. They are the same 15 defensive/attribution
lines recorded in durable risk **TD-007** (`docs/arc42/11-risks-and-technical-
debt.md`), which this change's documentation task verified and retained. All
three buckets were considered; none is dead bolted-on code this review may
delete, and none needs a scenario to exercise a new behavior introduced here.

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `apps/cli/src/fs.rs` | 47 | **Bucket 3** — the `AlreadyExists && exists` race arm needs a second process to claim the name between the check and `create_new`; a single-process suite cannot interleave a writer. | Justified; TD-007's two hard-to-reach filesystem arms. |
| `apps/cli/src/fs.rs` | 54 | **Bucket 3** — enumeration exhaustion requires 10 000 occupied candidate names. | Justified; TD-007. |
| `apps/cli/src/main.rs` | 27 | **Bucket 3** — closing brace of the controlled-clock early return; line 26 is covered 5 times, so the behavior is exercised and LLVM attributes the brace to the uncovered continuation. | Attribution residue; TD-007's construction-clock fall-through. |
| `crates/duty2cal-core/src/adapters/ics/mod.rs` | 88, 89 | **Bucket 3** — raw `\n` and `\r` in a PDF text cell; the supported family cannot print them, so the sink's guard is defensive. | Justified; TD-007's sink newline/CR arms. |
| `crates/duty2cal-core/src/application/mod.rs` | 398 | **Bucket 3** — the equal-value arm of the split-fragment merge; the supported producer layout prints each optional field on one fragment, not both. A source variation would need a new fixture. | Justified; TD-007's defensive conversion-reader arms. |
| `crates/duty2cal-core/src/domain/mod.rs` | 184–186 | **Bucket 3** — continuation lines of the exhaustive multi-line `date()` match pattern; `date()` is called for every entry, and LLVM attributes the pattern continuations uncovered. | Attribution residue; TD-007's exhaustive-match arms. |
| `crates/duty2cal-pdf/src/lib.rs` | 288 | **Bucket 3** — the no-date-band guard; a table without a Date column is rejected by the application's required-column check, and the guard is the reader's defensive arm. | Justified; TD-007's missing-date-band guard. |
| `crates/duty2cal-pdf/src/lib.rs` | 365 | **Bucket 3** — closing brace of the nearest-area guard; `areas` is non-empty for every recognised detail table, and LLVM attributes the brace to the uncovered `None` continuation. | Attribution residue; TD-007. |
| `crates/duty2cal-pdf/src/lib.rs` | 383 | **Bucket 3** — the invalid-calendar-date arm of the compact footer date; the supported family prints a valid footer date, and a malformed stamp is already refused by the unusable-stamp scenario. | Justified; TD-007's out-of-range footer-date arm. |
| `crates/duty2cal-pdf/src/lib.rs` | 389 | **Bucket 3** — the non-UTC-zone return; the zoneless-footer scenario exercises the behavioral outcome (construction clock) and line 388 is hit, so the return's line attribution is the residue. | Attribution residue; TD-007's out-of-range zone arm. |
| `crates/duty2cal-pdf/src/lib.rs` | 407, 408 | **Bucket 3** — infallible conversions: a month index taken from the 12-name table, and a 1–2 digit day; the `?` failure arms cannot be reached by a printed footer. | Justified; TD-007's compact-date conversion arms. |

No `@wip` tags remain (`grep` prints nothing). No delta scenario is missing or
downgraded, and no unit test duplicates a `.feature` scenario: the change adds
only step definitions.

## Scenario execution evidence

**Receipt status:** not archived yet — the change has no
`givn/archive/windows-only-release/verification.json`. The final acceptance in
`tasks.md` records the completed candidate: regular scope `121 scenarios
(121 passed)`, `{"total":121,"passed":121,"failed":0,"skipped":0}`; e2e scope
`3 scenarios (3 passed)`, `{"total":3,"passed":3,"failed":0,"skipped":0}`;
both exit 0. The review does not run the suite; the archive is the full-suite
authority.

## Overlap dispositions

Every shape-match finding involving the change's delta. All pairs share one
capability interaction shape but pin a different cause or input class
(missing build, unreadable build, withdrawn platform, never-supported
platform); none is a duplicate that would lose coverage if merged.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A platform whose executable was not built stops the release | An unreadable executable stops the preparation | variant |
| A platform whose executable was not built stops the release | An executable for the previously offered Linux platform is refused | variant |
| An unreadable executable stops the preparation | An executable for the previously offered Linux platform is refused | variant |
| An executable for an unrecognised platform is refused | A platform whose executable was not built stops the release | variant |
| An executable for an unrecognised platform is refused | An unreadable executable stops the preparation | variant |
| An executable for an unrecognised platform is refused | An executable for the previously offered Linux platform is refused | variant |

The permanent copy of each `@givn.modified` scenario is omitted at assembly, so
the permanent/delta pairs the lint reports do not run twice.

## Split-or-keep

No scenario in this change exceeds the gate threshold (more than 14 steps).
The longest delta scenario is the e2e scenario with 7 steps; the longest
permanent scenario involved in a shape pair has 9 steps. No split-or-keep row
is owed.

## Design.md conformance check

| Decision (from `design.md`) | `design.md` says | Actually built | Matches? | If not: design.md updated + design-review re-run? |
|---|---|---|---|---|
| Supported platform value | `PLATFORMS='windows-x86_64'` in `scripts/prepare-release.sh` | `PLATFORMS='windows-x86_64'` | yes | n/a |
| E2E runner | `./run-tests.sh --e2e` | `./run-tests.sh --e2e` → `--tags @e2e` | yes | n/a |
| E2E step file | `apps/cli/tests/steps/e2e/publish_public_release.rs` | same file, one per capability | yes | n/a |
| Driver | `std::process::Command` on the real preparation script | `Command::new("scripts/prepare-release.sh")` in the shared When, e2e assertions on its files/transcript | yes | n/a |
| Test assembly | `scripts/assemble_features.py` behind the stable `scripts/assemble-features.sh` | same, invoked by `run-tests.sh` / `measure-coverage.sh` | yes | n/a |
| Release matrix | one `windows-x86_64` entry; no `linker`, no musl env, no non-Windows start step | exactly that; `smoke-windows` remains | yes | n/a |
| Public-text gate | 11 presence checks + 3 fixed statements + 2 negative checks | implemented as 14 promises; clean | yes | n/a |
| README route | one Windows row, Windows extraction/run, `Get-FileHash`, source-route/history statements | implemented and gated | yes | n/a |
| Evidence path/slug | `evidence/visual/maintainer-prepares-a-windows-release-and-inspects-the-archived-executable/transcript.txt` | same, non-empty | yes | n/a |
| Product code | none | no production crate/module/port changed | yes | n/a |

No deviation was implemented without updating `design.md`; `design.md` is the
reviewed design (design-review PASS).

## Interaction coverage verification

The inventory is the owning use case's `## Interactions` table (two rows).

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| prepare the release for a version | yes | Maintainer prepares a Windows release and inspects the archived executable | CLI: `std::process::Command` runs `scripts/prepare-release.sh --version 0.1.0 --binaries <fixture>/binaries --out <fixture>/out`, asserts archive/digest, release text, `tar -tzf` member, extracted bytes | shared `run_prepare` uses `Command::new("scripts/prepare-release.sh")`; e2e file uses `std::process::Command` for `tar`/`cat` and asserts the staged files | yes |
| generate the change log for a version | yes | Maintainer generates the change log for a version and reads its newest section | CLI: `std::process::Command` runs `scripts/render-changelog.sh --version 0.1.0 --repo <fixture> --out <path>` and reads the file | same; permanent steps drive the renderer and read the written change log | yes |

No missing or extra matrix rows; no over-produced e2e. The opposite interface
classification (a rendered page is a Web UI) does not hold: the product is one
CLI binary, the publish interaction is a terminal command, and the release page
and README are GitHub-rendered text, not this project's UI. No browser driver
is valid or expected.

## README impact

README-IMPACT: updated - Download a release, Verify what you received

The public face changed for a consumer: the download table is Windows-only, the
Windows extraction/run and `Get-FileHash` examples replace the five-platform and
macOS instructions, and the source-build/platform-return/history statements are
added. `scripts/public-text-check.sh` reports "clean - all 14 public-text
promises present" and the old README fails five promises in the negative
control recorded in `tasks.md`. The issue form and `SECURITY.md` were also
updated; the design records that they are reviewed by hand rather than gated.

## Sign-off

- [x] Fabrication audit: CLEAN — 0 empty step bodies across 4 files, every
      checked task has an existing revision, every promised component exists,
      no downgraded e2e, no e2e driven by HTTP/fetch in place of the CLI, no
      over-produced e2e scope.
- [x] Scenario execution evidence recorded per scope in the completed candidate
      (archive receipt not yet produced; the change is not archived).
- [x] Every coverage gap is classified and justified from the measured merged
      report plus static inspection; 15 lines, all bucket 3 via TD-007.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the whole testable system; no digital twins
      exist to start.
- [x] The exact files `verify.e2e_command` invokes were identified; the tree
      including untracked files was searched for a parallel e2e implementation.
- [x] `verify.e2e_command` is not identical to `verify.command`; 3 < 121.
- [x] Implementation (commands, file layout, framework/driver) matches
      `design.md` as reviewed; no silent deviation.
- [x] Interaction coverage verified: 2 inventory entries ↔ 2 matrix rows ↔
      existing `@e2e` scenarios ↔ the promised CLI mechanisms.
- [x] `design.md`'s internal-primitives requirement is satisfied in substance:
      the design states "Product code: None" and lists only test-step helpers;
      no production struct/fn/enum is introduced.
- [x] arc42 slug mapping verified: every chapter slug maps to a real
      `docs/arc42/<slug>.md`.
- [x] No finding is excused with a classification outside the three buckets.

## Integrity finding (resolved after review)

**`givn check review` initially failed the integrity check: `tasks.md` had no
machine-readable `- [x] COMMIT:` lines.** The six scenario tasks recorded
`- [x] COMMIT (revision recorded below)` and a prose `Revision: \`<hash>\``
line, which the review integrity hook does not read. The commits themselves
exist and were verified above (`7dac3d1`, `4f8feee`, `7b86604`, `ba665ca`,
`d237476`, `0e26453`); this was a record-format defect, not missing work.

Remediation applied: every scenario task now carries an explicit
`- [x] COMMIT: <hash>` line (C1 `7dac3d1`, C2 `4f8feee`, C3 `7b86604`,
C4 `ba665ca`, C5 `d237476`, C6 `0e26453`).

**Gate result after remediation:** `givn check review --change windows-only-release` exit **0**:

```
✓ integrity passed
✓ run declaration passed
net delta: 2 added, 4 modified, 1 removed
✓ overlap dispositions passed
```

No other finding remains. Sign-off:

REVIEW: PASS
