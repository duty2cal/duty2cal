# Design review: export-released-roster-calendar

Fresh-context grilling of the complete plan (proposal, spec delta, design, arc42 assessment), then hardening. The grilling subagent read every planning artifact, the ideation seed/contract, both Personas, the 12 arc42 chapters and the ADR template, and independently re-derived the arc42 chapter set before diffing `arc42.md`.

## Grilling — branches walked

| Branch | Outcome |
|---|---|
| Scope | One gap: the spec did not assert whole-plan completeness for a multi-activity plan, the EM rule, the DESCRIPTION field order, or footer records. Fixed (see hardening). |
| Tech choices | Clean. The supplied stack (Rust edition 2021, pinned `pdfplumber = 0.1.0`, hand-written RFC 5545, cucumber-rs strict, `cargo llvm-cov`, Windows GNU cross-build) fits every scenario; no simpler alternative satisfies the observable behaviour without violating a supplied non-negotiable. |
| Missing scenarios | The four scope gaps above, plus a missing exit-status assertion and a missing Trip ID row in the omission outline. Fixed. |
| Testability | One abstract Then ("Complementary optional values …") and one Given that implied an indistinguishable race ("A calendar name claimed concurrently …"). Fixed. No true duplicates among the remaining scenarios. |
| E2E fidelity | Clean. Exactly one `@e2e` scenario is correct for this single-action CLI; the Interaction Coverage Matrix row is valid and non-empty; the real driver is a subprocess of the built binary. |
| Visual contract | Clean. `Interface: terminal` is correct and the opposite (browser) cannot hold — no HTML/CSS/JS, no rendered screen; the transcript evidence rule is satisfied. |
| Interface classification | Confirmed terminal; no browser driver is required or valid. |
| Persona disposition | Clean. Both confirmed Personas are recorded as review lenses and neither is a Gherkin actor; no Persona invented or auto-promoted. |
| Deferrals | None silent. The absent `run-tests.sh`/`measure-coverage.sh`/`merge-coverages.sh` are declared setup tasks; the Windows build, licence, and privacy gates are named pre-release gates. |
| Decision ledger | Complete. D1/D2/D3 resolve in `design.md` with their seed references; D2/D3 carry a named reopening owner. |
| Risk | Single most likely failure: the exact-pinned extraction crate (declares edition 2024/Rust 1.85; its 0.1.0 API does not match the draft snippets). Mitigation: the setup task proves compile + extraction before conversion logic is written, and reports a conflict rather than substituting. Recorded as R-001. |
| ADR qualification | Confirmed: no decision passes all five mandatory dimensions; the snapshot-scoped UID identity is the nearest miss whose complete rationale is already owned by the spec + glossary. |
| arc42 re-derivation | 11 of 12 rows matched; row 12 was the mismatch. Four concrete arc42 defects found and fixed (see hardening). |
| Ubiquitous language | Row 12 issue plus the glossary-scope question below. |

## Grilling — findings resolved from the repository (no operator input needed)

1. **Exit-status contract not asserted.** The design and chapter 05 declare "Non-zero on rejection; zero only on real publication", but no scenario asserted it. Resolved by applying the recommended option: the `@e2e` happy path now asserts success through the process exit status, and the unsupported-source rejection asserts a non-zero exit alongside the failure text. This is hardening of an already-declared contract, not a new promise.
2. **Spec gaps for already-fixed rules.** Resolved by applying the recommended option: added scenarios for EM timed non-flight duty, footer records supplying no dated activities, a mixed plan representing every required activity (whole-plan completeness), and the approved DESCRIPTION field order; added Trip ID to the omission outline; made the complementary-values Then concrete; removed the indistinguishable concurrent-claim scenario (a concurrently claimed path behaves exactly like an occupied one, already covered by the occupied-name scenario).
3. **ASCII-art quality tree in chapter 10.** Converted to a Mermaid `flowchart`.
4. **Missing `docs/arc42/adr/README.md`.** Created as the ADR directory index.
5. **`arc42.md` row 1 factual error** ("four quality goals" vs five). Corrected to "five".
6. **`design.md` referenced the deleted concurrent-claim scenario.** Traceability row updated.

`givn lint --change export-released-roster-calendar` exits 2 (expected: all scenarios carry `@wip`; remaining SHAPE/SUBST entries are advisories on rule-distinct scenarios).

## Operator questions

### D-Q3 — Glossary scope for the change's remaining vocabulary

Resolved 2026-10-12 (operator): **promote the domain-level terms.** Chapter 12 now holds **Released Roster detail-table family**, **Layover**, and **EM duty** as confirmed terms, and `arc42.md` row 12 is "Yes". Raw source-column labels (A/C, Blocktime, Report (LT), Timezone Difference, Tags) and the FREE/OFF codes stay verbatim source labels, not confirmed terms. Rejected: promoting every label the spec uses (fills the glossary with non-domain column names and weakens the term signal); keeping the 13-term glossary (leaves the spec's own vocabulary undefined and row 12 unverifiable).

## Re-review after design amendment (tasks/setup phase)

`design.md` was amended during setup: cucumber-rs 0.23 takes a single feature root, so the runner mechanism changed to `GIVN_FEATURES` assembly by `./run-tests.sh` (plus `run_and_exit`, `--e2e` tag filter, single-scenario via `./run-tests.sh --name`). design-review was re-run in a fresh context.

Result: **PASS**. The amended mechanism was independently reproduced (`./run-tests.sh` 50 scenarios, `./run-tests.sh --e2e` 1 scenario, both RED at exit 101); it satisfies the executable-spec invariant and the canonical specs instruction; `tasks.md` is consistent; the 12 arc42 rows were re-walked with no mismatch (test tooling is not an arc42 chapter change); no ADR qualification outcome changed. Ten low/medium hygiene findings were raised and fixed: temp-root leak from `exec` discarding its `EXIT` trap; missing `/target` ignore; stale chapter-11 R-003 and arc42 row 11 wording; `--e2e` only honoured as first argument; the design's "latest from crates.io" cucumber wording (now pins 0.23); the coverage scripts' dependence on `GIVN_FEATURES` (design now says they reuse the runner's assembly); the missing `tests/steps/export_duty_plan_calendar.rs` skeleton; the tasks label calling the full run "non-e2e"; and untracked `Cargo.lock` (to be committed with the change).

## Brief reconciliation (external "AGENT BRIEF — DutyPlan2Ical")

The operator supplied an external brief containing new material and 20 conflicts with this committed change. Operator decision (2026-10-12): reconcile conflict by conflict, applying each answer before the next question.

Resolutions:

- **Conflict 1 — all-day determination: keep the code-driven rule (committed).** U, FZ, FREE and OFF are all-day on their duty-plan date with printed clocks ignored; TK, DTUSA, EM, SB, RE and SBX are timed. The brief's `00:00–00:00` rule is rejected: the inspected corpus has blank time cells on FREE/OFF and its `00:00` values sit in accounting columns (LSW/BZW). This also settles **conflict 8** (standby is not all-day) and **conflict 9** (U and FZ remain confirmed all-day codes).
- **Conflict 10 — deadhead visibility: keep DH in the travel title (committed).** A Pos-DH positioning row is one event titled `DH 007 ABC-DEF`; zero accounting Blocktime and a missing aircraft do not erase it (Q19, Q20). The brief's uniform `DESIGNATOR FROM-TO` title and its DESCRIPTION alternative are rejected; a deadhead case must be added to the golden evidence.
- **Conflict 16 — endpoint date resolution: keep independent endpoint resolution with adjacent-day UTC qualifiers (committed).** Each endpoint's date is resolved on its own; an explicit parenthesised day marker maps to the unique previous/same/next Gregorian date carrying month/year; the narrow unqualified overnight rollover stays (Q3, Q21, Q30). The brief's "DTEND date = arrival block" rule is rejected — it misdates the corpus's `(12)`-qualified split (25 h 50 m instead of 1 h 50 m) and year boundaries.
- **Conflict 11 — component identity: keep the committed snapshot-scoped identity with an opaque discriminator.** Identity is scoped to the source snapshot and the represented activity, globally unique, deterministic for the identical snapshot, independent of counters and titles (Q25). The brief's readable date/flight/route recipe is rejected as a whole because it is not globally unique (two crew members or two identical legs on one date collide) and it re-introduces the cross-revision correspondence that Q25 explicitly rejected.
- **Conflict 12 — DTSTAMP provenance: keep the source Created-footer preference with a construction-time fallback, realised through the brief's Clock port.** The footer supplies DTSTAMP when usable; the injected Clock supplies the construction moment otherwise; revision metadata never supplies activity dates (Q27). The brief's Clock-only rule and a footer-only-with-failure rule are rejected.
- **Conflict 13 — calendar naming: keep the enumerated unused name and never overwrite (committed).** The ordinary base name is used when free, otherwise an enumerated suffix; occupied or concurrently claimed paths are never replaced; the actual new path is reported (Q10). The brief's fixed `<name>.ics` with replacement, and a fixed-name-refuse-on-collision variant, are rejected.
- **Conflict 14 — diagnostics naming: keep numbered, preserved reports (committed).** Every failure attempts a new numbered report; occupied names are never replaced; the new path is reported only after a successful write, and a stale report is never presented as current (Q22, Q24). The brief's fixed `<name>.parse-error.txt` (with replacement, or written-only-if-absent) is rejected.
- **Conflict 15 — CLI arity: keep one plan per conversion (committed); batch deferred.** Passing more than one path stays out of Phase 1 and is recorded as a Phase-2 item. The brief's `dutyplan2ical <pdf>...` shape and both multi-path failure models (continue-and-aggregate, fail-fast) are not adopted now.
- **Conflict 17 — test oracle: express the whole-file golden comparison as a Gherkin scenario (committed executable-spec invariant).** `verify.command` continues to run the `.feature` files; a scenario converts the reference PDF and compares the produced calendar with the expected file after CRLF→LF and folding normalisation. The brief's separate `tests/golden.rs` is rejected as a parallel hand-maintained oracle.
- **Conflict 18 — fixture provenance: generated fixtures only; real plans stay local and uncommitted.** The committed `reference_roster.pdf` and golden calendar are independently generated with fictional values; real duty plans are used only for local validation. Committing an anonymised real roster is rejected because it needs the source-use rights the parked question Q5 has not established.
- **Conflict 20 — phases/scope: keep Phase 1 scope (CLI + one Windows executable); record Phase 2 (WASM/UniFFI) and Phase 3 (native UIs) as roadmap.** No `ffi/*` crates are created now (placeholders would be stubs) and no WASM facade is implemented in this change.
- **Conflict 19 — event model (additive, adopted).** The brief's in-process, synchronous, ordered `DomainEvent` model with an `EventSubscriber` port is adopted as an addition to the design (no broker, no async channel); it does not conflict with the committed contract and adds the CLI logger / future progress surface.
- **Conflict 1 — product name: rename to `duty2cal` (operator input, not the brief's `dutyplan2ical`).** The operator chose `duty2cal` because the product is not limited to iCalendar output in future. The rename (crate/binary/docs) and the calendar PRODID product token follow the new name (`-//kressnerd//duty2cal//EN`); the rename is applied in the hardening pass after all questions.
- **Conflict 2 — extraction pin: adopt the brief's `pdfplumber = "=0.4.1"`.** The setup task re-proves the build on the resolved toolchain before extraction code is written; the earlier 0.1.0 compile proof is superseded.
- **Conflict 3 — Windows target: keep `x86_64-pc-windows-gnu`, cross-compiled on Linux/CI (committed).** The brief's `x86_64-pc-windows-msvc` on windows-latest is not adopted; static linking keeps the single self-contained executable.
- **Conflict 4 — licensing: adopt the brief's split now.** `crates/**` (domain, application, adapters, ffi) are MPL-2.0; `apps/**` (the CLI) are GPL-3.0-or-later; `deny.toml` (allowlist adds MPL-2.0), `REUSE.toml`, `cargo-about` notices, per-file SPDX headers and DCO sign-off are adopted. The supplied MIT/Apache-2.0 constraint is superseded; the GPL-CLI/App-Store constraint is recorded for the deferred Phase 3. **Amended 2026-10-12:** the operator stated the licence change exists because the iOS UI will be published in the App Store; since a GPL application layer cannot ship there, all `apps/**` are now MPL-2.0 as well and ADR-0001 records a single project-wide licence (see the amendment section below).
- **Conflict 5 — repo shape: adopt the operator's consolidated reduced workspace (§4 replacement).** Cargo workspace with `crates/dutyplan-core` (modules `domain/`, `application/`, `adapters/ics/`; deps `thiserror` + `time`; no I/O, no clock, wasm32-safe), `crates/dutyplan-pdf` (the only crate depending on `pdfplumber`), `apps/cli` (driving adapter; filesystem as a module), and Phase-2 `ffi/{wasm,uniffi}`. Layer rules are enforced at four levels: crate boundaries (compiler); `deny.toml` plus a core `clippy.toml` `disallowed-types`/`disallowed-methods` list; `archunit = "=0.0.2"` tests (`cargo test --test architecture`); and review conventions. The per-adapter crate list is superseded. Two consequences: (a) crate prefix `dutyplan-*` vs the chosen product name `duty2cal` — asked next; (b) §4.2's `AllDayKind /* Free, Off, Standby */` is adjusted to the resolved all-day rule (U/FZ all-day; SB/RE/SBX timed).
- **Conflict 5a — crate names: `duty2cal-core` and `duty2cal-pdf`.** The crates follow the chosen product name; the binary/package is `duty2cal`; `ffi/wasm`, `ffi/uniffi` follow. The §4 text's `dutyplan-*` names are superseded.
- **Conflict 6 — ADR process/location: mint the qualifying ADRs in the project's arc42 home.** `docs/arc42/adr/ADR-0001-project-license.md` (project licence; amended to MPL-2.0 project-wide) and `docs/arc42/adr/ADR-0002-layer-enforcement.md` (crate boundaries + `deny.toml` + core `clippy.toml` + `archunit`) are created, registered in chapter 09 and `adr/README.md`; the brief's separate `docs/adr/` path is rejected.

All 20 conflicts plus the crate-naming consequence are now resolved. Hardening: the decisions are applied to `design.md`, the arc42 chapters and ADRs, the spec delta, `tasks.md`, and the code scaffold; then design-review is re-run.

## Re-review after reconciliation hardening

The reconciliation decisions were applied: `design.md` rewritten (name `duty2cal`, two-crate workspace, four-level layer enforcement, `pdfplumber = "=0.4.1"`, project-wide MPL-2.0 licensing, event model, adjusted `DutyEntry`, golden-oracle scenario); `docs/arc42/{02,04,05,07,09,11}.md`, `arc42.md`, `09-architecture-decisions.md` and `adr/README.md` updated; ADR-0001 and ADR-0002 minted; the spec gained the non-`@e2e` golden scenario; `tasks.md` setup and scenario list updated; the scaffold restructured to the workspace (`crates/duty2cal-core`, `crates/duty2cal-pdf`, `apps/cli`) with `deny.toml`, `REUSE.toml`, `clippy.toml`, `LICENSES/`, SPDX headers and the archunit architecture test; `run-tests.sh` retargeted to `-p duty2cal`.

A fresh-context re-review returned **PASS** on all six checks (decision fidelity for all 20 conflicts + the crate-naming consequence; contradiction greps clean; both ADRs complete and defensible with ADR-0001's licensing exception recorded; scaffold/tasks/tests/arc42 consistent; golden scenario correctly non-`@e2e` with exactly one `@e2e` preserved; verification reproduces `cargo test -p duty2cal-core --test architecture` 3 passed, `./run-tests.sh` 51 RED, `./run-tests.sh --e2e` 1 RED).

Fixes applied: chapters 6 and 8 renamed `ConvertDutyPlan`/`DutyPlanCalendar` to `ConvertRoster`/`Roster`; the README licence section now states MPL-2.0 project-wide. Noted then: the pre-reconciliation first draft `temp_first_idea.md` still carried the superseded name/pin/licence — since resolved by deleting it (operator-confirmed; superseded by the frame/contract/seed/change artifacts; its reusable mechanical details were preserved first — see the ideation session record). The forward-looking E2E step skeleton, `application/src/testing.rs` port fakes, coverage scripts and CI workflows remain tracked unchecked tasks.

## Licence amendment (2026-10-12, operator)

The operator stated the reason for the licence change: **the iOS UI will be published in the App Store**. Since a GPL-3.0-or-later application layer cannot ship there, and the adopted DCO/no-CLA model means a single external contribution would permanently close the copyright-holder-exception route, the operator chose to licence **all** `apps/**` MPL-2.0 as well. The result is a single project-wide MPL-2.0 licence; the brief's GPL application layer is dropped.

Applied: ADR-0001 rewritten as "Licence the project under MPL-2.0 (per-file weak copyleft)" and renamed to `ADR-0001-project-license.md`, with its original split recorded as the amended outcome, the iOS goal added as a driver and the strong-copyleft driver removed, and routing recorded as `AMEND_ADR`; `apps/cli/Cargo.toml` and the six `apps/**` source headers moved to MPL-2.0; `Cargo.toml` carries a single `license = "MPL-2.0"` in `[workspace.package]` with members inheriting it; `LICENSES/GPL-3.0-or-later.txt` deleted; `deny.toml` comment, chapter 02/04/09/11, the chapter-09 register, `adr/README.md`, `README.md`, `proposal.md`, `design.md`, `arc42.md` and `tasks.md` updated; chapter 11's R-007 replaced (the App-Store blocker is gone; the residual is that MPL is file-level, so a redistributor of a modified product owes only the modified MPL files).

Fresh-context re-review: **PASS** on all six checks (no live GPL claim; licence statement consistent across manifests, headers, docs and indexes; ADR-0001 complete and correctly routed as an amendment with the qualification gate re-derived; chapter 11 and the register agree; `cargo build --workspace`, `cargo test -p duty2cal-core --test architecture` 3 passed, `./run-tests.sh` 51 RED, `./run-tests.sh --e2e` 1 RED; no relicensing/strong-copyleft claims anywhere). One low finding — chapter 01 still said "permissively licensed" — was fixed to "licensed MPL-2.0 project-wide (weak file-level copyleft; App-Store distributable)".

**Operator reaffirmation (2026-10-12):** asked whether a permissive project licence remained possible given the extraction dependency, the operator confirmed copyleft is intended and declined the `MIT OR Apache-2.0` alternative. `pdfplumber 0.4.1` was verified as Apache-2.0 with a permissive-only shipped closure, so no licence question arises from the dependency either way; the project stays MPL-2.0 project-wide.

## Coverage-scope fix and final confirmation (implementation complete)

Design and reality diverged during implementation and were reconciled, with a fresh-context review each time:

- `./measure-coverage.sh` used `cargo llvm-cov -p duty2cal`, which instrumented only the CLI binary and left the core and PDF crates (including the design's named non-e2e probe `domain/timing.rs`) uncovered. The script and the design's coverage rows now use `--workspace`; the reports cover all seven source files (694 lines, 662 covered) and both probes are non-zero.
- The same review found that the merge tool mutated one input report, keeping that report's other packages and duplicating six of seven classes. It now builds a fresh document: one package, one class per file, every merged line equal to the sum of the two reports (verified exhaustively, 0 mismatches).
- design.md was corrected where it described a committed reference PDF/Typst source that never existed (the plan is built at test time; only the expected calendar is committed), where the coverage rows named `./run-tests.sh` instead of `./measure-coverage.sh`, and where the `@e2e` transcript had to carry the process exit status (it now does).
- The `DUTY2CAL_CONSTRUCTION_CLOCK` seam is recorded in the design instead of being undocumented.

Final confirmation (fresh context): all checks pass — merged coverage structurally clean and arithmetically correct, golden-oracle wording matching the implementation, `./run-tests.sh` 51/51 and `./run-tests.sh --e2e` 1/1, `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` clean (re-run in a fresh target dir), and the architecture tests green.

## Status

DESIGN-REVIEW: PASS — every branch walked, every question resolved, all hardening edits applied; the implementation-phase divergences above were reconciled and re-confirmed.


## Re-run after the producer-layout rework

The reader was re-targeted from fixtures invented for this change to the producer's own export layout, and
the fixtures were re-shaped and their data invented. This re-run covers that rework; the branches were
walked against `design.md` (`Reading the producer's own layout`), the reader code, the fixtures and the
local validation script.

Findings and dispositions:

| Finding | Severity | Disposition |
|---|---|---|
| The producer-layout reading rules were not covered by any committed scenario; only the out-of-CI local corpus checked them. | blocking (Testability, Missing scenarios) | Fixed. The fixture builder now prints the wrapped two-row header, the `DD Ddd` date spelling and the heading month; the reference plan prints a continuation row without a date, the `Layover: <station>` marker instead of an activity code, and the producer's own revision footer; the all-day scenarios print the `==` marker and assert it never reaches the calendar. |
| `header_areas` required all three anchor labels in one extracted row, so a wrapped header whose anchors land on different print rows would be rejected as an unsupported family. | blocking (Risk) | Fixed. Recognition now accumulates the header rows and recognises the family once the anchor labels are present across them. |
| arc42 chapter 11 `R-003` claimed the coverage scripts were absent and named a package-scoped coverage tool. | blocking (arc42) | Fixed: the entry is closed, naming the workspace-wide measurement design.md records. |
| The claim "nothing above the header is a detail row" did not state the guard for material that follows the table. | advisory | Fixed: the design now names the row-pitch gap for what follows the table and the body-column test as the guard for placement. |
| The spec's Given steps write ISO dates while the fixture builder prints the family's spelling, so the family's date spelling is invisible in the feature text. | advisory | Accepted: the layout is a design-level reading rule, recorded in `design.md`; the scenarios pin behaviour, and the corpus script pins the real spelling. |
| The re-drive RED claim was not reproducible because the reader and the fixtures landed in one commit. | advisory | Fixed: `tasks.md` records the fault-injection commands and the observed `41 steps failed`. |

Branches with no issue found: technology choices, use-case and Persona context, the decision ledger
(`D1`–`D3`, seed references kept), the visual design contract, interface classification, and ADR
qualification.

Operator question and answer: does the converter serve one producer template or several? Answered
**one template now, a second expected soon** — the reader keeps rejecting an unrecognised header explicitly,
`design.md` records the single supported template, and recognising the second is a later change with its own
committed fixture and its own local plans.

All blocking findings are fixed, every branch has been walked, and the operator question is answered.

DESIGN-REVIEW: PASS
