# Design review — build-browser-converter

- Change: `build-browser-converter`
- Review stage: design complete, tasks not written, implementation not started.
- Grilling: fresh-context reviewer (`DesignGrill`) read the proposal, delta
  feature, design, permanent use case and feature, ideation seed/frame/event
  storming/terms/personas, `docs/arc42/*`, the ADR register/template,
  `Cargo.toml`, `rust-toolchain.toml`, `givn/commands.yaml`, `run-tests.sh`,
  `measure-coverage.sh`, CI, and the CLI/core/pdf code. It edited nothing and
  wrote no `design-review.md`.
- Dispositions: the operator answered under the explicit instruction "go ahead
  and do not ask again; stop only for something hard to change". The recorded
  decisions were captured in `design-review-decisions.md`; this file is their
  durable review record (the scratch file was removed after capture).
- Hardening: applied by `DesignHardener`. No production code, no `tasks.md`.

## Grilling questions and dispositions

| # | Branch | Finding / question | Disposition |
|---|---|---|---|
| 1 | Visual design contract | No approved rendered direction; the reference was deferred to future `@e2e` screenshots; `docs/design/design-system.md` was declared but absent. | **Resolved — operator approved.** The recorded token/direction is the approved contract; `docs/design/design-system.md` and the design-only reference under `docs/design/reference/` exist; only empty/ready/failure are declared states; converting is transient; the "45%+" typo is fixed to the token ratios. |
| 2 | Arc42 / deferrals | `arc42.md` absent and chapter 4 omitted; pre-browser claims remained in chapters 1, 2, 3, 5, 7, 10, 11 and 12. | **Resolved.** `arc42.md` has the full 12-row table with `STATUS: DONE`; chapters 1–12 (including 4) are updated. |
| 3 | ADR qualification | The planned ADR bundled three decisions, had no five-gate verdict and no existing-ADR check. | **Resolved — operator directed one qualified record.** ADR-0007 records only the browser-local conversion facade with the five gates and an ADR-0002/0003 duplicate check. The browser-owned save is routed to the capability spec and `usecase.md`; the Playwright driver is routed to `design.md`. |
| 4 | Deferrals / coverage | The `duty2cal-wasm` coverage exclusion was an unanswered operator question. | **Resolved — operator approved.** The wasm-only wrapper lines are excluded from the native Cobertura denominator; the shared core remains covered by the CLI rows; the browser scenario is the facade evidence; the residual risk and revisit condition are recorded. No native duplicate. |
| 5 | Missing scenarios / testability | Trap/OOM behavior was design-only with no operator decision. | **Resolved.** The startup/module-instantiation failure is a regular scenario (cannot-run treatment, no picker, no calendar, no network fallback). A mid-conversion trap/OOM that cannot be forced deterministically is an operator-accepted manual check with a concrete release/version condition, recorded in design and chapter 11. |
| 6 | Missing scenarios | A failed attempt could leave an earlier calendar offered. | **Resolved.** `acceptFile` clears the prior offer/state before reading; the scenario "A failed attempt withdraws a calendar offered by an earlier attempt" pins it; the use case records the rule. |
| 7 | Missing scenarios | The drop path was in the proposal and driver protocol but untested. | **Resolved.** The regular scenario "A supported plan dropped onto the drop zone is converted" drives `drop_file`; the picker remains the single `@e2e` action. |
| 8 | Missing scenarios | The ES-module capability branch was unasserted. | **Resolved.** The regular scenario "A browser without ES-module support shows the cannot-run message" forces the `supports`/`noModule` branch. |
| 9 | Missing scenarios | Startup/module-load/instantiation failure was described but unasserted. | **Resolved.** The regular scenario "A converter module that cannot be instantiated shows the startup failure" forces the path and asserts no picker, no calendar and no other-origin request. |
| 10 | Testability | The persistence scenario could pass without a successful conversion. | **Resolved.** It now asserts the ready state and the calendar offer before the address/history/storage assertions. |
| 11 | Scope / spec precision | The equivalence scenario overclaimed byte identity without the Created-footer precondition. | **Resolved.** The Given now names the supported plan whose Created footer supplies its revision timestamp. |
| 12 | Testability | The `@e2e` "complete RFC 5545 calendar" was not concrete. | **Resolved.** The scenario adds "the saved calendar matches the reference calendar for `2026-06.pdf`"; `design.md` names the step's concrete checks (events, CRLF, UID, DTSTAMP, no extra/partial event). |
| 13 | Testability / e2e mechanics | `read_state` could not return diagnostic Blob text under `connect-src 'none'`. | **Resolved.** The driver captures the diagnostic download via `save_offer`; the Rust step reads the file and matches `Source PDF: …\nReason: …\n`; `read_state` returns only the visible reason. |
| 14 | Tech choices | The changed-files table named `scripts/run-tests.sh` and `scripts/measure-coverage.sh`, which do not exist. | **Resolved.** Both are corrected to the root `run-tests.sh` and `measure-coverage.sh`. |
| 15 | Scope / CI | `Cargo.lock` and `THIRD-PARTY-NOTICES.md` were missing from the change set. | **Resolved.** Both are in the changed-files table and the setup obligation regenerates the notices with the pinned `cargo-about@0.9.0`. |
| 16 | Visual contract | `docs/design/design-system.md` was declared but absent. | **Resolved.** Created with tokens, primitives, breakpoints, accessibility, capture rules and references. |
| 17 | Tech choices | Node floated at "latest LTS at every run", contrary to D6. | **Resolved.** Resolve once, then pin in `web/e2e/.nvmrc` and the CI `node-version`; the resolved version is committed. |
| 18 | Tech choices | "Playwright is the project's default browser driver" was factually wrong. | **Resolved.** Reworded to "the chosen driver, not an existing project default"; the Selenium/WebDriverIO rejection rationale remains. |
| 19 | Tech choices | `wasm-bindgen` and `js-sys` were normal dependencies, risking native/Windows workspace builds. | **Resolved.** `wasm-bindgen`, `js-sys` and `getrandom` are target-scoped under `cfg(target_arch = "wasm32")`; the native rlib keeps core/pdf/time. |
| 20 | Ubiquitous language | "Published"/"written" were used for browser offers, contradicting the glossary. | **Resolved.** The browser events are **Calendar export offered** and **Conversion diagnostic offered**; the executable keeps published/written; the glossary lifecycle rows name both realizations and define **Offer**. |
| 21 | Ubiquitous language | "Supported platform" could be read as denying the browser route. | **Resolved.** The glossary, chapters 1/2/4/12, the proposal and the use case state that supported platform means downloadable archive platforms; the browser is a supported conversion route with no archive. |
| 22 | Language / e2e fidelity | "Public website" was used for a loopback-served page. | **Resolved.** Browser scenarios and the design use "the converter page served from the built Site output"; "Public website" is reserved for the published page. |
| 23 | Use-case scope | The 20 MiB bound was listed as a precondition but is checked in the flow. | **Resolved.** The capability/startup requirement stays the precondition; the 20 MiB bound is a flow check in main-flow step 1 and the browser realization. |
| 24 | Use-case scope | The trigger and success guarantee named channels. | **Resolved.** The trigger is channel-neutral; the success guarantee keeps one outcome plus one clearly marked realization clause. |
| 25 | Use-case consistency | Traceability conflated the calendar-offer failure with the diagnostic-offer failure. | **Resolved.** Separate rows for "calendar offer cannot be created" and "diagnostic offer cannot be created". |
| 26 | Visual contract | Contrast "45%+" was not a ratio; converting/failure evidence did not match the declared states. | **Resolved.** Fixed to the token ratios; empty/ready/failure are declared states; converting is transient; all three states have a committed design-only reference. |
| 27 | Scope | The release note implied the page was already public. | **Resolved.** It says the capability is available from the built Site output; public publication is the `publish-public-website` change. |
| 28 | Decision ledger | Proposal Q2 omitted the confirmed visitor lens and its assignment. | **Resolved.** Q2 now records the confirmed `visitor--project-reader` lens assigned to `publish-public-website`, so this change records no visitor impact. |

No grilling question remains unresolved.

## Operator decisions (recorded verbatim enough to be auditable)

1. **Visual contract — approved with a rendered reference.** Approve the
   recorded token direction; create `docs/design/design-system.md`; capture one
   rendered reference view for empty, ready and failure from the page source,
   or a design-only static reference if the page is not built; declare only the
   states the evidence captures; treat converting as transient; fix the
   "45%+" typo; record the approval in `design.md`.
2. **Coverage exclusion — approved.** Exclude `duty2cal-wasm`'s wasm-only
   wrapper lines from the native Cobertura denominator; keep the shared core
   covered by the CLI rows; evidence the facade through the browser scenario;
   record the residual risk and revisit condition; do not add a native
   duplicate.
3. **ADR — one qualified record.** Write one ADR for the browser-local
   conversion facade with the five mandatory gates and an ADR-0002/0003
   duplicate check. Route browser-owned save to the spec and `usecase.md`; route
   Playwright/cucumber-rs to `design.md`. If the facade failed
   `lower_level_artifact`, create no ADR; it passes, so ADR-0007 is created.
4. **Trap/OOM — deterministic startup failure plus a recorded manual
   deferral.** Add the startup/instantiation failure scenario; record an
   operator-accepted manual check with a concrete condition for a
   mid-conversion trap/OOM; name any test seam as a design fix, not production
   behavior.
5. **Stale offer — add a scenario and a rule.** A failed attempt clears the
   prior offer; `acceptFile` and the state machine specify it.
6. **Drop path — add a regular scenario.** Drop a supported plan and assert the
   ready state and the offer; the picker remains the single `@e2e` action.
7. **ES-module branch — add a regular scenario** forcing the capability check
   false and asserting cannot-run, no picker, no calendar.
8. **Startup/instantiation failure — add a regular scenario** forcing the
   module/`WebAssembly.instantiate` rejection and asserting startup failure, no
   picker, no network fallback.
9. **Persistence scenario — assert the ready state first**, then the offer,
   then the address/storage assertions.
10. **Equivalence — state the Created-footer precondition.**
11. **`@e2e` completeness — assert the reference fixture.** The Then phrase
    stays; the step asserts the fixture's events, CRLF, UID, DTSTAMP and the
    absence of extra/partial events.
12. **Diagnostic content — capture the download.** Click the diagnostic offer,
    capture and read it in the Rust step; `read_state` returns only the visible
    reason; no Blob fetch.
13. **Script paths — fix** to `run-tests.sh` and `measure-coverage.sh` at the
    repository root.
14. **Lock and notices — add** `Cargo.lock` and `THIRD-PARTY-NOTICES.md` to the
    changed files and setup.
15. **Node — pin after resolution** in `web/e2e/.nvmrc` and the CI
    `node-version`; record the resolved version.
16. **Playwright — correct the wording.** Chosen driver, not an existing
    default; keep the rejection rationale.
17. **WASM-only dependencies — target-scope** `wasm-bindgen`, `js-sys` and
    `getrandom`; keep the native rlib on core/pdf/time.
18. **Event language — distinguish offer from publish/write.** Browser events
    are "Calendar export offered" and "Conversion diagnostic offered"; the
    executable keeps published/written; generalize the glossary lifecycle
    definitions.
19. **Supported platform — clarify** it means downloadable archive platforms;
    the browser is a supported conversion route with no archive; coordinate
    the release-use-case wording with `publish-public-website`.
20. **Public website term — reserve it for the published page.** Browser
    scenarios use "the converter page served from the built Site output"; the
    E2E serves `web/` on loopback.
21. **Use-case preconditions — move the 20 MiB bound into the flow.**
22. **Use-case trigger/success — make the trigger channel-neutral**; keep the
    channel realization in the rules; the success guarantee may keep one
    clearly marked realization clause.
23. **Traceability — separate the calendar-offer and diagnostic-offer
    failures.**
24. **Release note — do not imply publication.** Say available from the built
    Site output; public publication is the sibling change.
25. **Proposal Q2 — record the confirmed visitor lens** and its assignment to
    `publish-public-website`.
26. **arc42 — produce the artifact.** Full 12-row table, `STATUS: DONE`,
    chapters 1–12 updated, chapter 4 included, glossary event/supported
    platform wording, no ASCII-art diagrams.
27. **Strict mode and single-scenario command** are confirmed; keep
    `.fail_on_skipped()` and the named command.
28. **Residual risks** stay in design/chapter 11: WASM link/instantiation under
    the real CSP is the first setup milestone; 20 MiB fixture validity; the
    Node/Chromium supply chain must fail loudly.

## Hardening edits applied

- `proposal.md`: corrected release note; added the startup-failure, stale-offer
  and trap/OOM behavior to the proposed solution; recorded the approved D3
  coverage exclusion and the D6 Node pin; corrected seed Q2; clarified that the
  browser route is not an archive platform.
- `specs/export-duty-plan-calendar/export-duty-plan-calendar.feature`: added
  the stale-offer, drop, WebAssembly, ES-module and startup-failure scenarios;
  added the ready/offer assertion to the persistence scenario; added the
  Created-footer precondition to the equivalence scenario; added the concrete
  reference-fixture assertion to the `@e2e` scenario; replaced "Public website"
  with "the converter page served from the built Site output"; kept exactly one
  `@e2e` scenario per normalized action.
- `design.md`: recorded the approved visual contract and reference; created the
  design-system reference; declared only empty/ready/failure states; fixed the
  contrast typo; replaced the coverage question with the approved decision;
  replaced the bundled ADR claim with the five-gate verdict; added the
  startup/trap decisions; added the stale-offer clearing rule; updated the drop,
  ES and startup behavior; changed diagnostic capture to a download; corrected
  script paths; added `Cargo.lock`, `THIRD-PARTY-NOTICES.md` and
  `web/e2e/.nvmrc`; pinned Node; corrected Playwright wording; target-scoped
  the WASM dependencies; renamed the browser events; clarified supported
  platform versus archive; fixed the precondition/trigger/traceability rows and
  separated the offer failures; recorded the release-note limitation; expanded
  the risk register; recorded the 12-chapter impact and ADR routing.
- `givn/specs/export-duty-plan-calendar/usecase.md`: precondition now names the
  browser capability and initialization; the trigger is channel-neutral; the
  20 MiB bound is a flow check; the success guarantee is one outcome plus a
  marked realization; the browser events are named; the interaction inventory
  uses the converter page and notes drop is a variant; the minimal guarantee
  includes stale-offer withdrawal; delivery constraints distinguish supported
  platforms from conversion routes.
- `docs/arc42/12-glossary.md`: generalized the lifecycle events; added
  **Offer**; clarified **Supported platform**, **Browser conversion**, **Site
  output**, **Profile content** and **Organization profile**.
- `docs/arc42/01..11`: updated the browser route, constraints, context,
  solution strategy, building blocks, runtime flows, deployment, cross-cutting
  concepts, ADR register, quality scenarios and risks/debt; removed the stale
  pre-browser claims (including "no browser application", "not created in this
  change" and "does not publish a browser converter" where it was too broad).
- `docs/arc42/09-architecture-decisions.md`, `docs/arc42/adr/README.md` and
  `docs/arc42/adr/ADR-0007-browser-local-conversion-facade.md`: one qualified
  ADR and both indexes.
- `docs/design/design-system.md` and
  `docs/design/reference/converter-reference.html`: the durable design system
  and the approved design-only reference for empty, ready and failure.
- `givn/changes/build-browser-converter/arc42.md`: full 12-row assessment,
  chapter decisions, ADR routing and `STATUS: DONE`.

## Branch coverage

| Branch | Result |
|---|---|
| Scope | No extra scope found; the missing drop/ES/startup/trap/stale-offer behaviors and the release-note/precondition precision are fixed. |
| Tech choices | Stack fits; Node pin, Playwright wording, target-scoped wasm deps, script paths, lock/notices and the design-system doc are fixed. |
| Missing scenarios | Added stale offer, drop, ES modules, startup/instantiation and the trap/OOM decision. |
| Testability | Ready-state precondition, concrete reference fixture, diagnostic-download capture and the trap manual check are recorded; strict mode and single-scenario command are concrete. |
| E2E fidelity | Real Chromium/Playwright, real file input, real download capture, one `@e2e` per normalized action, no HTTP/fetch substitute. |
| Interaction coverage | `usecase.md` has two normalized inventory entries; the design matrix has two matching rows; each has exactly one matching `@e2e` scenario. Drop is a variant, not an entry. |
| Risk | WASM instantiation under the real CSP is the first setup milestone; the 20 MiB fixture is validated builder-to-reader; the supply chain fails loudly. |
| Use-case/Persona | Goal, primary actor, placement, mutation, naming, leaf size and relationships pass. The precondition/trigger/mechanism findings are fixed; three confirmed Personas are dispositioned, visitor is assigned to the sibling change. |
| Decision ledger | D1–D6 resolve; D3 is approved, D6 is pinned; seed Q1–Q9 are dispositioned; Q2 is corrected. |
| Visual design contract | `Interface: web`, both viewports, approved direction, durable design system, three reference states, capture mechanics and state alignment. |
| Deferrals | Coverage exclusion approved; trap/OOM manual check accepted with a concrete condition; no other mandatory check is deferred. |
| Persona disposition | Three confirmed lenses with relevance/evidence; visitor explicitly assigned to `publish-public-website`; no invented or silently promoted Persona. |
| Interface classification | The converter screen is a Web UI (`Interface: web`) driven by Playwright/Chromium; the executable remains a CLI. The opposite classification (no browser driver) does not hold because the capability renders a page. |
| ADR qualification | One qualified `NEW_ADR` (ADR-0007); browser-owned save and Playwright driver routed to their canonical artifacts; existing-ADR check completed. |
| Arc42 documentation | 12-row table complete; chapters 1–12 updated; chapter 4 included; all 12 chapter files exist with real content and no box-drawing/ASCII-art diagrams. |
| Ubiquitous language | Offer vs publish/write, supported platform vs conversion route, and Public website vs built Site output are consistent; the glossary holds them. |

## Verification

- `givn check proposal --change build-browser-converter` — exit 0.
- `givn check specs --change build-browser-converter` — exit 0.
- `givn check design --change build-browser-converter` — exit 0.
- `givn check arc42-docs --change build-browser-converter` — exit 0.
- `givn check design-review --change build-browser-converter` — exit 0.
- `givn lint --change build-browser-converter` — exit 2 with 16 findings, all
  `[WIP]` for the 16 delta scenarios. No shape/subset advisories remain.
- `givn status --change build-browser-converter` — proposal, specs, design and
  arc42-docs complete; design-review is the next artifact.
- Chapter integrity: all 12 files exist with decision-specific content; no
  box-drawing or pseudo-diagram characters were found.
- Scratch `design-review-decisions.md` removed after its content was captured
  here.

## Residual risks and conditions (not blockers)

- **WASM instantiation under the real CSP** is proven first by the setup task's
  minimal real-Chromium `init(bytes)` milestone, before page/E2E work
  (design risk register; chapter 11 R-036).
- **Mid-conversion trap/OOM** is an operator-accepted manual check at each
  release and whenever Chromium or the WASM bundle changes (chapter 11 R-042,
  TD-018).
- **Coverage exclusion** is revisited when the toolchain can collect a wasm
  profile (chapter 11 R-039, TD-016).
- **Firefox/Safari** remain manual release checks; support stays
  capability-based (chapter 11 R-040, TD-017).
- **Publication** of the page, its Built revision and its public copy belongs to
  `publish-public-website`; this change must not imply the public URL is live.

DESIGN-REVIEW: PASS
