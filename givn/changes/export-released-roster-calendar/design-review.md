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

## Status

DESIGN-REVIEW: PASS — every branch walked, every question resolved, all hardening edits applied.
