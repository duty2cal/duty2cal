# Design Review: label-unknown-duties

## Scope

The review covered the proposal, the delta feature, the design, the arc42 artifact and the amended durable design contract. A fresh grilling pass walked every mandated branch; this file records its findings, their dispositions, and the hardening applied.

## Grilling findings and dispositions

### Resolved blockers

1. **A permanent scenario would fail through an unlisted changed step.** `the page still offers the calendar` asserted the removed copy `Your complete calendar is complete…`; it is used by the unmodified permanent scenario "A cancelled save is not reported as a save". Hardened: the design's changed-binding list now names that step, and it asserts `You can choose where to save the file.`
2. **The arc42 assessment under-reported chapter 5.** The wasm result shape and the two `web/` block rows change, but `arc42.md` marked row 5 "No". Hardened: row 5 is "Yes" with the changed blocks named; `05-building-block-view.md` now updates the `ffi/wasm`, `index.html` and `app.js` rows. Row 4 was also marked "Yes" because `04-solution-strategy.md`'s browser clause was factually corrected.

### Resolved should-fix findings

3. **The ready-heading step could not read a heading.** The driver exposes no `#offer h2` observation. Hardened: the delta uses the generic visible-text step for the heading; no driver change is needed, and the changed ready bindings assert the heading.
4. **The evidence plan named no capture command and the wrong intermediate path.** Hardened: the design records `DUTY2CAL_VISUAL_EVIDENCE_DIR=label-unknown-duties DUTY2CAL_UPDATE_VISUAL_EVIDENCE=1`, `fullPage: true`, and that runs without the override capture into the scenario's temporary directory.
5. **The reference and the design's child order disagreed.** Hardened: the reference's two ready panels and the durable design contract now use heading → status → list → copy → offers → note → way back; the contract names the child-order-only layout change and corrects `#ready-reset` to `#reset-ready`.
6. **Three negative guards would become vacuous after the heading rename.** Hardened: the changed-binding list records the three `!text.contains("Calendar ICS ready")` updates.
7. **Nothing asserted plan order or one item per row.** Hardened: the two-row scenario now lists both expected lines in order.
8. **The line's date was ambiguous for a day-qualified window.** Decision: a timed fallback lists the interval's UTC start date (the calendar event's date); an all-day fallback lists the duty-plan date. Recorded in the design, the glossary and the runtime view, so the page and calendar can be read side by side.
9. **The report offer could silently inherit a new predicate.** Decision: the offer and `#warnings` visibility follow `warnings.length > 0`; `unknownDuties.length` drives the list and the count sentence. The equivalence is stated, not assumed.
10. **The matrix overclaimed the CLI `@e2e` scenario.** Hardened: the matrix now says the `@e2e` action proves the real publication path; the title is proven by the non-e2e calendar scenarios.
11. **The use case's extensions still said "titled by the printed code".** Hardened: both extension sentences now say `unknown duty: <printed code>`.

### Accepted observations

- The all-day `@browser` scenario is thin but distinct: it exercises the all-day line form. It is not `@e2e` and adds no interaction.
- Two identical unknown-duty rows could collide with the baseline's duplicate-text rule. The baseline runs on fixed generated plans; no such plan is used. No rule change is made.
- A duplicate safe list is not required: one line per represented row is the contract.
- Every new facade helper carries `#[cfg(target_arch = "wasm32")]`, so the native build stays free of dead code.

## Branch coverage

| Branch | Result |
|---|---|
| Scope | Two blockers and one wording finding resolved; the layout reorder is recorded as the operator's settled child order |
| Tech choices | No issue: one domain accessor, a borrowed projection, existing `time` dependency; `cfg` gate recorded |
| Missing scenarios | Resolved: two-row order assertions and every contradicted binding listed |
| Testability | Resolved: heading via visible text, concrete list/offer assertions, `.fail_on_skipped()`, panic stubs, named single-scenario command |
| E2E fidelity | No issue: two inventory actions, real CLI and Playwright, no new `@e2e`, matrix claim corrected |
| Interaction Coverage | No issue: both inventory entries have a matrix row and one `@e2e` each |
| Risk | Mitigated: verbatim list assertions and the full-suite runner catch copy/glue drift; the plan-level binding gaps are closed |
| Use-case and Persona context | Resolved: extension wording fixed; confirmed lenses unchanged, no invented Persona |
| Decision ledger | No issue: D1–D3 resolved and realized; D1 now records the date rule |
| Visual design contract | Resolved: order, capture command, evidence paths and stale id corrected |
| Deferrals | No issue: no mandatory check deferred |
| Persona disposition | No issue: two confirmed lenses for the web surface; release owner recorded as unaffected |
| Interface classification | No issue: rendered page, Web UI, Playwright through the real driver |
| ADR qualification | No issue: `NOT_QUALIFIED` → `CANONICAL_ARTIFACT`, all five gates evidenced, no ADR minted |
| Architecture documentation | Resolved: independent 12-row derivation now matches; chapter 5 and 4 corrected; glossary consistent |
| Ubiquitous language | Resolved: **Unknown duty** in the glossary; `unknown duty: <code>` used across proposal, spec, design and use case |

## Hardening applied

- `givn/changes/label-unknown-duties/design.md`: date rule, DOM order, changed bindings, matrix wording, evidence command, `cfg` gate, report-offer predicate.
- `givn/changes/label-unknown-duties/specs/export-duty-plan-calendar/export-duty-plan-calendar.feature`: generic heading step, two-row order assertions.
- `givn/specs/export-duty-plan-calendar/usecase.md`: both extension sentences name the new title.
- `givn/changes/label-unknown-duties/arc42.md` and `docs/arc42/{04,05,06,12}`: chapter 5 and 4 corrected; date rule recorded.
- `docs/design/design-system.md`, `docs/design/reference/converter-reference.html`: child order, stale id, rendered ready panels.

`givn lint --change label-unknown-duties` exits 0 (advisory subset notices only).

DESIGN-REVIEW: PASS
