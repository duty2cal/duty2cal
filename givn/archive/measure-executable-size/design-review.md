# Design review — measure-executable-size

- **Change:** `measure-executable-size`
- **Date:** 2026-10-18
- **Method:** a fresh-context grilling subagent read the full plan (proposal,
  delta, design, `arc42.md`, the permanent use case, the ideation seed,
  `givn/personas/*`, all 12 `docs/arc42/` chapters, the ADR index,
  `givn/commands.yaml`, `.github/workflows/ci.yml`, `cliff.toml`, the renderer,
  packaging and suite scripts, `features_runner.rs`, the existing e2e step
  files, `Cargo.toml`, `.cargo/config.toml`, `REUSE.toml` and the
  license-header script), walked every required branch, and verified the
  cargo-bloat 0.12.1 claims against the published crate and its v0.12.1
  source. The grilling questions and findings are summarized below.
- **Second phase:** this hardening pass applied every finding and the operator
  decision; this file records the outcome.

## Operator question (closed)

### Question: should an unreadable measurement fail the Windows build job or be isolated?

- **Status:** closed 2026-10-18 (user).
- **Blocks:** the CI wiring, the risk register (whether a generator regression
  can block release evidence), and the sign-off of the "Failure is visible"
  promise.
- **Context:** the measurement runs inside `windows-gnu-build`, whose artifact
  `windows-smoke` consumes and whose checks gate publication. The command
  exits non-zero for `unreadable`; GitHub runs `run:` under `bash -e`, so a
  non-zero command would skip the report append and the separate upload step.
  The choice was whether that non-zero status should propagate.

Options put to the operator:

1. **Hard gate** — fail the measurement step (and therefore the job) when no
   attribution can be produced; publish the report first.
   - Meaning: a broken measurement is treated like a failing check.
   - Gain: a generator or toolchain regression cannot be silently ignored.
   - Cost/example: a cargo-bloat or toolchain regression blocks the Windows
     execution evidence and, because publication waits for checks, a tagged
     release; the fix is urgent rather than scheduled.
2. **Evidence only, job green** — the step always exits zero, appends the
   report (or a note that none was written) to the step summary, and uploads
   `size-attribution/` with `if: always()`; the Windows smoke and the release
   path continue.
   - Meaning: the report is the deliverable; the automation does not decide.
   - Gain: Windows execution evidence and the release path survive a
     measurement regression, and the reason is still published.
   - Cost/example: a broken measurement is green and can be ignored until
     someone reads the report.
3. **Separate measurement job** — a job that needs `windows-gnu-build`,
   downloads its artifact and uploads the report.
   - Meaning: the coupling is removed with plumbing.
   - Gain: the smoke is never blocked by the measurement.
   - Cost/example: a second checkout, artifact download and upload; the job
     either still blocks if required or is unenforced if not, so it only moves
     the original question.

**Resolution:** option 2, **evidence only, job stays green**. The measurement
runs in `windows-gnu-build`; a failure is reported in `attribution.md` and the
step summary, the step exits zero, `attribution.md` is always appended and the
artifact is uploaded with `if: always()`; no revision or release is refused.
The command itself may still exit non-zero for `unreadable` for local callers,
but the CI step does not propagate that status.

**Rejected:** the hard gate (a cargo-bloat/toolchain regression would block
Windows execution evidence and a tagged release) and a separate measurement
job (extra plumbing; still a gate if required, or unenforced if not). The
accepted residual risk is recorded as R-034 in chapter 11 and in
`design.md` §Risks.

## Grilling questions and their resolutions

| Q | Branch | Resolution |
|---|---|---|
| Q1 | Risk / Scope — the `unreadable` report was never published under `bash -e` | Operator decision above; CI step rewritten (evidence-only, `set +e`, always append, `exit 0`, `if: always()` upload). `design.md`; chapter 6/7. |
| Q2 | Scope / use-case fidelity — the report named only the resolved SHA, while the use case promises `v0.3.0` | Both are recorded: `requested` (ref as given) and `revision` (resolved 40-hex SHA), in JSON and Markdown; e2e asserts the requested ref. `design.md`; feature; use case; chapter 8/12. |
| Q3 | Scope — section sizes were extra scope with no scenario | Kept (the Frame confirms them) and made executable: a regular scenario for the `.text` row and one for an absent `objdump`. `design.md`; feature; traceability. |
| Q4 | Testability — "different number" did not bind labels to quantities; the strip-only rule was unproven | The e2e asserts the file-size line equals the fixture executable's byte count, `total < executable.bytes`, and the attribution build's `file-size > executable.bytes`; `attribution.build_file_size` added. `design.md`; feature. |
| Q5 | Missing scenarios — build identity recorded but never asserted | `## Build identity` asserted: rustc line, `cargo-bloat 0.12.1`, 64-hex digests of `Cargo.toml` and `.cargo/config.toml`. `design.md`; feature; traceability. |
| Q6 | Missing scenarios / Testability — the crate/function split asserted only as table presence | The e2e asserts the crate `fixture_support` and the function `fixture_support::fixture_alpha`. `feature`. |
| Q7 | Testability — the build-fails scenario could pass for the wrong reason; a missing generator was mislabelled | The build-fails assertion names the fixture's `fixture_compile_error`; every toolchain-dependent Given preflights its tools; a missing generator gets a distinct reason. `design.md`; feature. |
| Q8 | Risk — the GNU-PE symbol premise is unverified | Added as the top implementation risk with a first-task spike, the `-C link-dead-code` fixture fallback, and a stop-and-re-decide gate for the real project PE. `design.md`; chapter 11. |
| Q9 | Ubiquitous language — glossary "not an estimate" contradicted the design | Glossary contrast corrected to "not a byte-exact file size"; estimate caveat added to the Markdown contract and to chapters 8/11. `docs/arc42/12`, `08`, `11`; `design.md`; `arc42.md`. |
| Q10 | Missing scenarios / Risk — the cadence has no executable assertion | Accepted as a recorded limitation: the first pushed run's `size-attribution` artifact and summary are the evidence; no Gherkin scenario is invented for GitHub Actions. `design.md`; this file's Deferrals. |
| Q11 | Risk — appending to `windows-gnu-build` couples the measurement to the smoke and publication gate | Resolved by the operator decision; residual risk R-034. `design.md`; chapter 11. |
| Q12 | Missing scenarios — an unresolved revision is specified but unasserted | Regular scenario added: non-zero exit, the ref named, no report written. `feature`; `design.md`; traceability. |
| Q13 | Tech choices — `cargo install` vs `taiki-e/install-action` | Kept `cargo install --locked cargo-bloat@0.12.1`: the published crate ships `Cargo.lock` and the repo already uses `cargo install` for cargo-deny/cargo-llvm-cov. No change. |
| Q14 | Ubiquitous language — "symbol-bearing build" / "attribution build" were undefined | Standardized to the glossary's "a build that retains the names the measurement needs", with "attribution build" defined once as local shorthand; no new glossary term. `design.md`; chapters 2, 4, 6, 8, 12; `arc42.md`. |

## Findings and resolutions

| ID | Finding | Resolution | Files changed |
|---|---|---|---|
| F1 | `scripts/measure_executable_size.py` does not exist. | Expected: the design declares it new. | — |
| F2 | `python3` is already a tooling dependency (`assemble-features.sh`). | Claim confirmed. | — |
| F3 | `scripts/prepare-release.sh --binaries` trust-model analogy is accurate. | Claim confirmed. | — |
| F4 | The transcript placeholder convention is `<scenario-dir>`, not `<workdir>`. | Replaced all three occurrences. | `design.md` |
| F5 | `./run-tests.sh --e2e` selects the whole e2e suite, not one scenario. | The single-scenario command is `--name "<title>"`; `--e2e` kept for the suite. | `design.md` |
| F6 | `features_runner.rs` is strict (`.fail_on_skipped()`, `.run_and_exit`). | Claim confirmed; strict-mode proof retained. | — |
| F7 | `givn/commands.yaml` verify/e2e commands are as claimed. | Claim confirmed. | — |
| F8 | The fixture directory does not exist. | Expected: declared new. | — |
| F9 | CI job facts (permissions, job names, Windows target, artifact) are as claimed. | Claim confirmed; used by the rewiring. | — |
| F10 | The shallow-checkout mitigation holds (reproduced). | Claim confirmed. | — |
| F11 | The e2e scenarios run in `test` and `coverage`, not in `windows-gnu-build`. | CI-cost paragraph corrected. | `design.md` |
| F12 | The capability allow-list omits `measure-distributed-executable-size`, so the renderer would refuse. | The design plans the capability's entry in the marked allow-list (applied at implementation time). | `design.md` |
| F13 | An allow-list-only edit leaves the capability to the catch-all `skip = true`, contradicting the file's own invariant. | The design plans the breaking/new/fix/perf/revert parser rows for the capability (applied at implementation time). | `design.md` |
| F14 | The release-note rationale named the wrong operative mechanism (path filter vs catch-all skip). | Rationale corrected; parser rows make a later user-receiving revision announceable. | `design.md` |
| F15 | cargo-bloat 0.12.1 exists and its publication date is right. | Claim confirmed. | — |
| F16 | `--crates`, `--message-format json`, `-n 0` are real flags. | Claim confirmed. | — |
| F17 | "Its CLI has no `--locked` pass-through" is false; 0.12.1 forwards `--locked`. | Claim removed; `--locked` added to both invocations so the committed lockfile is honoured. | `design.md` |
| F18 | The strip and PE-read claims are true; a GNU PE without a PDB yields no fallback. | Claim confirmed; the unverified premise became the top risk (Q8). | `design.md`; chapter 11; `arc42.md` |
| F19 | The generator JSON keys (`size`, `file-size`, `crate?`) differ from the report schema and the mapping was unstated. | Mapping stated; `size → bytes`, `file-size → build_file_size`; `crate` optional and `[Unknown]` rendered. | `design.md` |
| F20 | `cargo install --locked cargo-bloat@0.12.1` is viable (the crate ships `Cargo.lock`). | Claim confirmed. | — |
| F21 | The project's release profile and `.cargo/config.toml` are as claimed. | Claim confirmed. | — |
| F22 | The license-header gate is `scripts/license-headers.sh`, not `reuse lint`; `Cargo.lock` is covered by the aggregate `REUSE.toml`. | Rationale corrected. | `design.md` |
| F23 | All 12 arc42 chapter files exist, are non-placeholder, and use Mermaid only. | Integrity check passed; walked independently (below). | — |
| F24 | No qualified ADR is missing for the measurement. | Confirmed; chapter 9 stays No. | — |
| F25 | Chapter content matched the pre-hardening design; chapters 6/7/8/10/11/12 needed the evidence-only, requested/resolved, estimate and risk updates. | Chapters updated and `arc42.md` row summaries refreshed. | `docs/arc42/02,04,06,07,08,10,11,12`; `arc42.md` |
| F26 | Chapters 3 and 9 are correctly marked No. | Confirmed; unchanged. | — |
| F27 | The environment paragraph undercounted the scenarios needing the Windows target. | Corrected: seven regular scenarios and the e2e need the target; five regular and the e2e need `cargo-bloat`; the unresolved case needs neither. | `design.md` |
| F28 | "two fixture builds" was wrong twice (where they run and how many). | CI-cost statement rewritten. | `design.md` |
| F29 | "the scenario alone is `./run-tests.sh --e2e`" is false. | Corrected to `--name`; `--e2e` is the suite. | `design.md` |
| F30 | The foreign-artifact refusal's report was ambiguous. | Refusal writes the `unreadable` report (status, reason naming the platform, `attribution: null`) and then exits non-zero; scenario asserts the report exists. | `design.md`; `feature` |
| F31 | The ideation seed still said the cadence was unresolved. | Replaced with the D1 outcome (every pushed revision; on request for any named revision). | seed |
| F32 | The use-case Trigger was a state/desire, not an event. | Reworded: "A revision is pushed, or the release maintainer requests a measurement for a named revision." | `usecase.md` |
| F33 | The "cannot be read" Given relied on a nonexistent path / `chmod 000`. | Reworded to "the supported platform's executable at the named path cannot be read"; the step uses a missing path and the design records why `chmod` is avoided. | `feature`; `design.md` |
| F34 | `givn lint` was clean apart from per-scenario `@wip` findings. | Re-run after hardening: exit 2 (findings-only), nine `@wip` findings, one file checked. | `feature` |

No finding is left unresolved.

## Branch verdicts

| Branch | Verdict |
|---|---|
| **Scope** | **Resolved.** The delta covers exactly the proposal's revision/platform, split, two-number rule and refusal paths, plus the section table the Frame confirms (now asserted). The cadence and the strip-only rule now have recorded evidence and observable assertions; the requested `v0.3.0` ref is observable. |
| **Tech choices** | **Resolved.** cargo-bloat 0.12.1, Python 3, two builds, separate target directories, temporary worktree, PE header check, JSON+Markdown all stand. The false `--locked` claim is corrected and the flag used; the generator JSON mapping is explicit. `taiki-e/install-action` was considered and rejected in favour of the repo's `cargo install` pattern. |
| **Missing scenarios** | **Resolved.** Added: build identity, concrete crate/function, requested ref, unresolved revision, section table, absent `objdump`; strengthened: build-fails reason marker, foreign-artifact report existence, incomplete Given. |
| **Testability** | **Resolved.** Every assertion binds a label to a quantity or a name; the two-number proof is arithmetic on the fixture's own bytes; strict mode, stub pattern and `--name` single-scenario command are named; toolchain preflights prevent false-green runs; environment counts corrected. |
| **E2E fidelity** | **Resolved.** One inventory entry → one `@e2e` scenario, real subprocess and real generator, primary assertion on the report, no duplicate action; `<scenario-dir>` used; `--name` is the single-scenario command; the `@e2e` tag is preserved and strengthened. |
| **Interaction Coverage** | **No issue found.** The use case has one inventory entry and the design has exactly that row; no extra or missing rows. |
| **Risk** | **Resolved.** The unverified GNU-PE premise is now the top implementation risk with a spike and fallbacks; the evidence-not-a-gate residual is R-034; the fixture LTO risk is retained; the CI wiring no longer discards the report. |
| **Use-case and Persona** | **Resolved.** All eight rejection criteria pass (below); the Trigger is now an event; the `v0.3.0` example is executable; Personas remain review lenses with recorded dispositions and no new Persona. |
| **Decision ledger** | **No issue found.** D1–D4 resolve or carry a named owner with seed references intact. |
| **Visual design contract** | **No issue found.** Terminal interface, no viewports, no screens, transcript evidence per `@e2e`, path and scrubbing rule; the opposite classification is now explicitly rejected. |
| **Deferrals** | **No issue found.** No mandatory check is deferred. The cadence's non-Gherkin evidence is recorded in `design.md` and is not an inventory action. |
| **Persona disposition** | **No issue found.** Per-persona relevance and evidence are recorded; a user-visible surface has confirmed Personas. |
| **Interface classification** | **Resolved.** The design now states why it is not a Web UI (text/files, no HTML/CSS/JS, no browser driver). |
| **ADR qualification** | **No issue found.** All four candidates are `NOT_QUALIFIED`, each routed once to `design.md`; no new ADR, register row or index entry. |
| **Arc42** | **Resolved.** The independent 12-row walk matches; chapters 3 and 9 stay No; the affected chapters were updated for the hardening decisions. |
| **Ubiquitous language** | **Resolved.** The glossary contrast is corrected, both generator runs are named in `Where`, the estimate caveat is in the report contract, and the build phrase is the glossary's; no unconfirmed term added. |

## Use-case scope rejection criteria

| Criterion | Evidence | Result |
|---|---|---|
| Goal conjunction | One verb phrase ("Obtain a size attribution …"), one object. | PASS |
| Missing primary actor | The Release maintainer is the primary Actor and is named in the Trigger. | PASS (trigger wording fixed) |
| Placement failure | The capability advances the measurement goal; it is not a conversion capability. | PASS |
| Mutation under an observing goal | The use case declares itself observing; scenarios assert report content, not state. | PASS |
| Extension without a step reference | Extensions reference steps 2, 2, 4, 4; no new success outcome. | PASS |
| Inconsistent preconditions | Preconditions are pre-existing facts and are not established by the flow. | PASS |
| Mechanism vocabulary | Goal/Trigger/guarantees avoid mechanisms; `strip` appears only under Rules. | PASS |
| Component/channel-shaped name | Use case and capability are goal-shaped. | PASS |

## Independent arc42 walk

Derived from the proposal, delta and design before opening `arc42.md`; the
last column is the comparison.

| # | Derivation | `arc42.md` | Result |
|---|---|---|---|
| 1 | Yes — new stakeholder segment | Yes | Match |
| 2 | Yes — new constraint (per-revision attribution, two builds, two numbers, pinned generator) | Yes | Match |
| 3 | No — GitHub remains the only external system; build tooling is not a product interface | No | Match |
| 4 | Yes — new technical strategy and release-integrity approach | Yes | Match |
| 5 | Yes — new script, report files, step files, fixture; `ci.yml` row | Yes | Match |
| 6 | Yes — new runtime flow, evidence-only failure | Yes | Match |
| 7 | Yes — deployment: attribution beside the run; evidence not a gate; suite jobs install the toolchain | Yes | Match |
| 8 | Yes — cross-cutting two-build/two-number/refusal/estimate rule | Yes | Match |
| 9 | No — no qualified ADR | No | Match |
| 10 | Yes — new quality scenario and tree node; no refusal on size | Yes | Match |
| 11 | Yes — R-032 (unverified premise), R-033, R-034, TD-014 | Yes | Match |
| 12 | Yes — refined size terms | Yes | Match |

Chapter integrity: all 12 files exist, contain decision-specific content, use
Mermaid only, and contain no placeholder templates. Chapter 3 remains No
(GitHub is the only external system; the command is repository build tooling,
consistent with `prepare-release.sh`). Chapter 9 remains No (no qualified ADR).

## ADR qualification summary

| Candidate | Verdict | Canonical artifact |
|---|---|---|
| Measurement command (one command, two builds, worktree, artifact input) | NOT_QUALIFIED | `design.md` |
| `cargo-bloat` 0.12.1 as generator | NOT_QUALIFIED | `design.md` |
| Report shape (JSON + Markdown, statuses, separate fields) | NOT_QUALIFIED | `design.md` |
| CI cadence (every pushed revision, artifact beside the run) | NOT_QUALIFIED | `design.md` |

`alternatives` and `existing_adr_check` PASS; the other three dimensions FAIL
with decision-specific evidence; each routes exactly once to `design.md`.
ADR-0001…0006 were searched; no archive exists; no `NEW_ADR`, so no cheaper
home is owed. Chapter 9 has no missing MADR, and chapter 11 reflects the
consequences. No ADR file, register row or index entry was created.

## Ubiquitous language disposition

- **Size attribution** — contract refined in the glossary: requested and
  resolved revision, build identity, cadence, command, both generator runs,
  and the corrected contrast ("not a byte-exact file size"; the numbers are
  estimates of `.text`).
- **Size attribution report** — names the requested and resolved revision and
  the build's identity; statuses unchanged.
- The design's build phrase is the glossary's "a build that retains the names
  the measurement needs"; "attribution build" is defined once as a local
  shorthand. No unconfirmed term was added, and no Persona biography is used
  as a Gherkin actor.

## Deferrals

None. The cadence ("produced for every pushed revision") cannot be driven by
GitHub Actions from a local Gherkin run; its non-Gherkin evidence — the first
pushed run's `size-attribution` artifact and step summary, plus `actionlint` on
the workflow definition — is recorded in `design.md` and this file. That is a
recorded evidence limitation, not a deferred mandatory check and not an
inventory action.

## Hardening edits applied

- `givn/changes/measure-executable-size/design.md` — CI wiring (evidence-only),
  requested/resolved revision, build identity, concrete split assertions,
  two-number proof and `build_file_size`, `--locked`, JSON mapping,
  foreign-artifact refusal order, section sizes, unresolved revision,
  environment counts, single-scenario command, `<scenario-dir>`, interface
  classification, UL phrase, `cliff.toml` parser rows and release-note
  rationale, license-header rationale, workstation claim, top risks, use-case
  traceability.
- `givn/changes/measure-executable-size/specs/measure-distributed-executable-size/measure-distributed-executable-size.feature`
  — the `@e2e` scenario strengthened (requested `v0.3.0`, resolved commit,
  build identity, concrete crate/function, byte-count equality, `total <
  file size`, `build file size > file size`); earlier-revision scenario uses
  `v0.3.0`; build-fails asserts `fixture_compile_error`; foreign-artifact
  asserts the report exists and names the platform; incomplete Given reworded;
  three regular scenarios added; every new scenario tagged
  `@givn.added @wip`; `@e2e` preserved.
- `givn/specs/measure-distributed-executable-size/usecase.md` — event-shaped
  delivery trigger; requested/resolved and refusal rules; extension 4;
  minimal and success guarantees; examples.
- `givn/ideation/release-executable-size/changes/measure-executable-size.md` —
  the cadence open question resolves to D1.
- `docs/arc42/02,04,06,07,08,10,11,12` and
  `givn/changes/measure-executable-size/arc42.md` — evidence-not-a-gate,
  estimate caveat, requested/resolved revision, the unverified GNU-PE premise
  with its spike, R-034, glossary contrast and `Where`, and the assessment rows.
- `design.md` records the planned `cliff.toml` edit (the marked allow-list
  and the matching breaking/new/fix/perf/revert parser rows); the config file
  itself is not edited in this planning step.

`givn lint --change measure-executable-size` exits 2 (findings-only; nine
expected `@wip` findings across one file). `givn status` reports `design-review-done` with `tasks` as the next
required artifact; `tasks.md` was not written, `proposal.md` is unchanged,
and no production code, script, workflow step or fixture was implemented.

DESIGN-REVIEW: PASS
