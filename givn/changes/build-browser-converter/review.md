# Review — build-browser-converter

Stage: `tasks-done` → `review`.
Scope: the browser realization of `export-duty-plan-calendar` — the `ffi/wasm`
facade, the `web/` page, the Playwright/cucumber-rs browser harness, the
committed visual evidence, and the recorded suite/coverage results.
The full suite was not re-run in this pass; the recorded final verification in
`tasks.md` and the generated Cobertura reports in `coverage/` are the execution
evidence. That record also reports `cargo fmt --all --check`, `cargo clippy
--workspace --all-targets -- -D warnings`, `cargo deny check`, `reuse lint`,
the licence-header script, the notices regeneration and `pii-guard` clean;
this review relies on those recorded runs and did not repeat them.

**Verdict: `REVIEW: FAIL`.** The implementation is real and the e2e evidence is
not fabricated, but six unresolved design/arc42/record mismatches remain, plus
one mechanical task-record check. They are listed as F1–F7 and must be
remediated (or the reviewed design corrected and re-reviewed) before sign-off.

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | `git log -p` over the delta feature; current tags | **CLEAN.** The delta scenario at `specs/export-duty-plan-calendar/export-duty-plan-calendar.feature:4` still carries `@givn.added @e2e`; the only tag change in history is the removal of `@wip` (4e0d466). No `@e2e` tag was removed or moved. |
| 1 | Empty/no-op step bodies | Grep all 10 step-definition files under `apps/cli/tests/steps/**/*.rs` for `{}`, bare `pass`/`return`, `unimplemented!`, `todo!`; the shared `browser.rs` client inspected separately | **CLEAN — 0 empty step bodies found across 10 step-definition files.** Every new browser step body calls the driver or asserts an observed value; `unimplemented!`/`todo!` have no matches. |
| 2 | Every checked task has a commit touching production | Task `Hash:` records vs `git show --stat` | **FINDING (F7).** Eight scenario commits (`afd900b`, `8b33b34`, `b078df5`, `c6eab05`, `66593de`, `78ad4e4`, `5a82432`, `43e916d`) have no production-file diff; they change tests, the driver/test tooling, the spec and `tasks.md`. The records for scenarios 4, 5, 6, 8 and 10 state that production was already committed, and scenarios 2/3 name the earlier page commit, but the mechanical check is unmet and scenarios 2/3 still say “production files modified”. No checked task is a stub, but the commit record does not match the check. |
| 3 | Every component `design.md` promises exists | Changed-files table vs tree | **CLEAN.** `ffi/wasm` (`Cargo.toml`, `src/lib.rs`, `clippy.toml`), `web/` (`index.html`, `capability.js`, `app.js`, `styles.css`, `.wasm-pack-version`, `pkg/` generated output), `web/e2e/` (driver, package files, `.nvmrc`), `scripts/build-web.sh`, `scripts/run-web.sh`, `docs/design/design-system.md`, `docs/design/reference/converter-reference.html`, the changed step files/runner, `Cargo.lock`, `THIRD-PARTY-NOTICES.md`, `run-tests.sh`, `measure-coverage.sh`, CI, `.gitignore` and the use case are all present. |
| 4 | Strict-mode proof present with output | `tasks.md` setup and e2e setup sections | **CLEAN.** The setup proof ran a temporary undefined step through the exact runner and exited 101 (`RUNNER_EXIT=101`); the e2e proof ran an undefined `@e2e` step and exited 101 (`E2E_PROBE_EXIT=101`, with the still-RED browser stub also failing). `.fail_on_skipped()` is in `apps/cli/tests/features_runner.rs`; the driver fails closed on unknown actions and the Rust client panics on non-`ok` results. |
| 5 | `@e2e` Then steps assert the real interface | Read the delta scenario and every e2e step | **CLEAN.** The Then steps assert the rendered empty/ready states, the offered `2026-06.ics`, the browser-captured download (VCALENDAR shell, CRLF, complete/count-balanced `VEVENT` blocks, exact reference-fixture event match), the request log, the visible no-save/no-account text and the visual baseline. No Then asserts only a repository/database. |
| 6 | Browser steps actually drive the browser | Read every browser step and `web/e2e/driver.mjs` | **CLEAN.** `set_file` uses Playwright `page.setInputFiles('#plan', …)`; `save_offer` clicks the real `<a download>` and captures `page.waitForEvent('download')`; `dismiss_save` cancels the browser download; `reload` uses `page.reload`; observations use the rendered DOM, request log, storage snapshot and screenshots. No step sends an HTTP request for the plan or calls page-side `fetch`/`XMLHttpRequest`. |
| 7 | Exact files `verify.e2e_command` invokes, and parallel implementations | `givn/commands.yaml` + `run-tests.sh` + tree search (including untracked) | **CLEAN.** `verify.e2e_command` is `./run-tests.sh --e2e`; `run-tests.sh` calls `scripts/assemble-features.sh` and then `cargo test -p duty2cal --test features_runner -- --tags "@e2e"`. The invoked implementation is `apps/cli/tests/features_runner.rs` → `steps/mod.rs` → `steps/e2e/export_duty_plan_calendar.rs` (+ shared steps in `steps/export_duty_plan_calendar.rs`) → `steps/browser.rs` → `web/e2e/driver.mjs`. `grep`/`find` found no second browser implementation; `git status --porcelain` is empty. |
| 8 | Normalized E2E scope | `usecase.md` `## Interactions` vs delta feature | **CLEAN.** The inventory has two distinct actions (executable supply; browser supply-and-save). The browser delta has exactly one `@e2e` scenario; size, family, unresolved activity, stale offer, drop, capabilities, startup, offer failures, cancelled save, stem and persistence are regular scenarios. Drop and equivalence are stated variants/invariants, not additional end-to-end actions. |
| 9 | `verify.e2e_command` is scoped, not a copy of `verify.command` | Literal `givn/commands.yaml` values; recorded counts | **CLEAN.** `verify.command` is `./run-tests.sh`; `verify.e2e_command` is `./run-tests.sh --e2e` (different strings). `tasks.md` records 166 regular scenarios and 9 e2e scenarios; 9 < 166, and the e2e scope consists of the 8 pre-existing `@e2e` scenarios plus the one browser scenario. No archive verification receipt exists yet because the archive step has not run; the final-verification record is the available execution evidence. |
| 10 | Implementation vs `design.md` commands/layout/framework and stated facts | File tree, manifests, scripts, page source, durable docs | **FINDING.** The commands/layout/framework mostly match (wasm-pack 0.15.0 and `ffi/wasm --release --target web --out-dir ../../web/pkg`; Playwright 1.64.1; the two capability step files; the JSON-lines driver; `run-tests.sh --e2e`), but the implementation silently diverges from reviewed design facts: the trap/discard rule is absent (F1), the privacy-note content is incomplete (F2), the declared breakpoints/typography are not implemented (F3), the state inventory omits the rendered `cannot-run` state (F4), and the `Offer` code symbol does not exist (F5). The design and chapter 11 also claim a privacy-guard exclusion set that the guard does not have (F6). |
| 11 | Interaction coverage cross-reference | `usecase.md`, design matrix, feature, step files | **CLEAN — full table below.** Two inventory entries, two matrix rows, one matching `@e2e` scenario each, and the promised driving mechanism is the one implemented. |
| 12 | Findings from 1–11 | — | F1–F7 are unfinished work, not coverage gaps. No pass marker is written while they remain. |

### Interaction coverage cross-reference (check 11)

| Inventory entry (`givn/specs/export-duty-plan-calendar/usecase.md`) | Design matrix row | `@e2e` scenario | e2e step file(s) | Driving mechanism promised / found | Match? |
|---|---|---|---|---|---|
| supply a duty plan for conversion through the executable | supply … through the executable (CLI, terminal) | `Crew member converts a supported duty plan and receives a calendar file` (permanent feature) | `apps/cli/tests/steps/export_duty_plan_calendar.rs`; `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` | `std::process::Command` on the built `duty2cal` binary / the same `std::process::Command` | MATCH |
| supply a duty plan through the converter page and save the offered calendar | supply … through the converter page and save the offered calendar (Web UI) | `Crew member converts a supported duty plan in the browser and saves the offered calendar` (delta feature) | `apps/cli/tests/steps/export_duty_plan_calendar.rs`; `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`; `apps/cli/tests/steps/browser.rs` | Playwright 1.64.1 → `web/e2e/driver.mjs` → Chromium, loopback-served `web/` / the same Playwright driver with `setInputFiles`, offer click and download capture | MATCH |

No missing or extra matrix row; no unmapped interaction; no `reqwest` or other
HTTP client substitutes Playwright.

## Arc42 implementation conformance

The durable chapters and `arc42.md` were read; the affected chapters were
re-derived from the completed implementation, `design.md` and `tasks.md` before
accepting the change-level claims. All twelve chapters are affected.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 1 — browser conversion route and truthful-copy goal | `docs/arc42/01-introduction-and-goals.md` | affected: second realization; truthful copy | Scope, release note | setup, 1–16 | `web/`, `ffi/wasm`, `web/e2e/` | yes |
| 2 — `wasm32`, target-scoped deps, no-I/O facade, CSP/inline bundle, 20 MiB bound, pinned test supply chain | `docs/arc42/02-architecture-constraints.md` | affected: constraints | Technology decisions, WASM facade | setup, 1–3, 16 | `ffi/wasm/Cargo.toml`, `web/index.html`, `web/.wasm-pack-version`, `.nvmrc`, `package-lock.json` | yes |
| 3 — browser boundary, loopback Site output, Site-publication boundary | `docs/arc42/03-context-and-scope.md` | affected: context | Architecture impact, scope | setup, 16 | `driver.mjs` loopback server, page code | yes |
| 4 — browser-local delivery strategy and ADR-0007 | `docs/arc42/04-solution-strategy.md` | affected: strategy | Decisions, architecture impact | setup | ADR-0007, `ffi/wasm`, `web/` | yes |
| 5 — `ffi/wasm`, `web/`, build/evidence toolbox | `docs/arc42/05-building-block-view.md` | affected: building blocks | new/changed files | setup, 1–16 | tree inspection | yes |
| 6 — browser conversion/offer, failure/diagnostic, capability/startup flows | `docs/arc42/06-runtime-view.md` | affected: runtime | state machine, runtime tables | 1–16 | `web/app.js`, `capability.js`, driver actions | yes for the flows; the design state inventory itself is F4 |
| 7 — static Site output, loopback serving, browser runtime, test-only tooling | `docs/arc42/07-deployment-view.md` | affected: deployment | local runnability, file layout | setup, 16 | `scripts/run-web.sh`, `driver.mjs` | yes |
| 8 — CSP, inline WASM, browser offer/no-write, browser E2E, version pinning | `docs/arc42/08-crosscutting-concepts.md` | affected: crosscutting | network/CSP, accessibility, test runner | setup, 1–16 | `web/index.html`, `web/app.js`, driver, lockfiles | yes |
| 9 — ADR-0007 register entry and indexes | `docs/arc42/09-architecture-decisions.md`, `adr/README.md` | affected: ADR | ADR routing | setup | `docs/arc42/adr/ADR-0007-…md` | yes |
| 10 — browser equivalence/privacy/failure/capability/input/evidence quality scenarios | `docs/arc42/10-quality-requirements.md` | affected: quality | traceability, matrix | 1–16 | `@e2e` + regular scenarios | yes |
| 11 — R-036…R-045 and TD-016…TD-018 | `docs/arc42/11-risks-and-technical-debt.md` | affected: risks/debt | risks, coverage boundary | final verification | **R-042, R-044 and TD-018 claim a catch/discard/fresh-retry rule that `web/app.js` does not implement (F1); R-045 lists three exclusions while `scripts/pii-guard.sh` also excludes `*.sh` in the tree scan (F6).** | **NO** |
| 12 — **Offer**, **Supported platform**, **Site output**, lifecycle events | `docs/arc42/12-glossary.md` | affected: glossary | traceability, decisions | 2, 5, 7 | **The `Offer` row names `web/app.js` (`offerCalendar`, `acceptFile`); no `offerCalendar` exists — the functions are `addOffer` and `showReady` (F5).** | **NO** |
| `design.md` state inventory | design state machine; driver protocol | — | four observed states (empty/converting/ready/failure) | 9–11, 16 | `readState` and the page also render/observe `cannot-run`; the visual contract declares only empty/ready/failure (F4) | **NO** |
| `design.md` privacy note and breakpoints | `docs/design/design-system.md`; `design.md` Visual Design Contract | — | no upload/no storage/no request; 480/720 breakpoints, 720 px column, 36 px h1 | 16 | `web/index.html` footer and `web/styles.css` omit the storage/request claim and the breakpoints (F2, F3) | **NO** |
| Privacy-gate exclusion claim | `design.md` Validation decisions; `docs/arc42/11…` R-045; `tasks.md` final verification | affected: risks | "only three recorded cases… No other path is excluded" | 392d2a6, final verification | `scripts/pii-guard.sh` tree scan also uses `--exclude='*.sh'` (and standard `.git`/`target`/`private` dirs) (F6) | **NO** |

The five arc42 sign-off checks: (1) affected chapters were independently
derived — yes; (2) every affected chapter is current — no, chapters 11 and 12
carry the contradictions above, and the design state inventory is incomplete;
(3) each fact was cross-referenced to design/tasks/implementation — yes;
(4) no omission or stale contradiction — no (F1–F6); (5) the clean marker is
recorded only after all rows match — **not recorded**.

**ARC42 CONFORMANCE: FINDINGS**

## Ubiquitous language conformance

The specs, design and code reuse the durable terms **Browser conversion**,
**Offer**, **Site output**, **Supported platform**, and the lifecycle events
**Calendar export offered** / **Conversion diagnostic offered** consistently at
the concept level. One code-symbol mismatch is a finding:

- `docs/arc42/12-glossary.md` (Offer row) and `design.md:563,570` name
  `web/app.js` (`offerCalendar`, `acceptFile`) / a `try/catch` around
  `offerCalendar`. The page has no `offerCalendar`; the actual functions are
  `addOffer` (creates the Blob/object URL/link) and `showReady` (the ready
  state). The glossary's `Code symbol` cell is therefore wrong, and the design
  traceability points at a non-existent symbol.

No other term drift was found: the spec uses “converter page served from the
built Site output”, not “Public website”; “published/written” stays with the
executable and “offered” with the browser; “Supported platform” still names
downloadable archive platforms.

**LANGUAGE CONFORMANCE: FINDING** — F5.

## Coverage classification

The most recent measured evidence is `coverage/cobertura-coverage.xml`
(merged from the two distinct instrumented runs recorded in `tasks.md`):
**880 of 894 production lines covered (line-rate 0.984340)**, 14 uncovered.
The non-e2e report is 880/894 and the e2e report is 614/894 with a different
md5; the reports are distinct, and `duty2cal-wasm` is absent from the native
denominator as designed. The browser scenarios ran GREEN in the recorded final
verification (166/166 regular, 9/9 e2e).

Every uncovered region is classified below; no region is left unclassified.

| File | Lines | What the uncovered code is | Bucket | Disposition |
|---|---|---|---|---|
| `crates/duty2cal-pdf/src/lib.rs` | 288, 365, 383, 389, 407, 408 | The reader guards for input the supported family cannot print: a missing date band, the `assign_cells` closing path when `areas` is empty, an out-of-range footer date, a non-UTC footer zone, and the compact-date month/day conversions. | 3 — legitimately hard to test | A fixture of the supported family cannot present these shapes; the behaviours they guard are pinned by the unusable-stamp/no-date-band scenarios. Pre-existing and recorded as TD-007. |
| `crates/duty2cal-core/src/domain/mod.rs` | 184–186 | The exhaustive-match arms of `DutyEntry::date()` for travel, availability and non-flight duty. | 3 — legitimately hard to test | Every one of those variants is exercised; the accessor is only asked for a date by the all-day and layover paths, and LLVM attributes the multi-line `|` pattern's continuation lines as uncovered. Pre-existing, TD-007. |
| `apps/cli/src/fs.rs` | 47, 54 | The retry arm when another process claims a candidate between the existence check and `create_new`, and the `Ok(None)` return after the bounded name enumeration is exhausted. | 3 — legitimately hard to test | The retry needs a second process to interleave; the exhaustion needs 10 000 occupied names. The occupied-name behaviour itself is covered. Pre-existing, TD-007. |
| `crates/duty2cal-core/src/adapters/ics/mod.rs` | 88, 89 | Escaping a raw newline and dropping a carriage return. | 3 — legitimately hard to test | A PDF text cell cannot carry either character; the sink guard is defensive. Pre-existing, TD-007. |
| `apps/cli/src/main.rs` | 27 | The fall-through when `DUTY2CAL_CONSTRUCTION_CLOCK` is set but unparsable. | 3 — legitimately hard to test | The seam is a test affordance; both real outcomes (a usable instant and the system clock) are covered. Pre-existing, TD-007. |
| `web/app.js`, `web/capability.js`, `ffi/wasm` wasm-only wrapper | behaviour-level | The page and the wasm wrapper have no native line-coverage signal: the Chromium runtime has no file-based LLVM/JS coverage collector in this toolchain, and `duty2cal-wasm` is outside the native Cobertura denominator. | 3 — legitimately hard to test (measurement boundary) | Every observable behaviour is driven by the 16 browser scenarios and the one browser `@e2e` scenario; the shared core stays in the native denominator. This is the operator-approved D3 boundary recorded in the proposal/design/design-review and chapter 11. |

- **Bucket 1 (dead code):** none found in the delivered code.
- **Bucket 2 (missing coverage):** none found in the delivered code; no
  downgraded or missing `@e2e` scenario exists for either normalized action.
- **Bucket 3:** the rows above.

This change introduces no uncovered line in the native conversion crates. The
browser page/facade are not native-coverage candidates; their behaviour is
pinned by real-browser evidence, and the measurement boundary is the recorded
operator decision, not an invented exception.

## E2E coverage

The capability is a **Web UI**: the page is rendered in Chromium and the e2e
scenario drives it. The opposite classification — “no browser driver is needed,
the capability is only a CLI” — does not hold for the browser realization: the
capability renders a page with a picker, status, offer and failure panels, and
a terminal/HTTP assertion could not observe those states. The executable row
remains a CLI interface.

- **Real driver:** Playwright 1.64.1 launches Chromium via
  `web/e2e/driver.mjs`; the scenario uses the real file input
  (`page.setInputFiles`), a real offer click and a real browser download
  (`page.waitForEvent('download')`), plus DOM observations, request log,
  storage snapshot, address/history and screenshots.
- **Primary assertions:** the captured calendar is checked for the VCALENDAR
  shell, `VERSION`/`PRODID`, CRLF throughout, balanced `VEVENT` blocks, and an
  exact event-by-event match with
  `apps/cli/tests/fixtures/expected/2026-06.ics`; the page states, offered
  filename, no-origin-request log, no-save claim, no-account/email/upload
  controls, storage emptiness, unchanged address/history, and the visual
  baseline at both displayed states are all asserted through the rendered
  page.
- **Command/scoping:** `verify.e2e_command` is `./run-tests.sh --e2e`, filtering
  `@e2e`; it is not a copy of `verify.command`. The e2e scope is 9 scenarios
  (the 8 pre-existing plus this browser scenario), strictly fewer than the 166
  regular scenarios. No archive receipt exists yet; the archive will record the
  receipt.
- **Inventory:** two normalized actions, two matrix rows, exactly one `@e2e`
  scenario each (table in Step 0). Regular variants do not add an inventory
  entry.
- **No downgrade:** no e2e Then step asserts only a repository/database, and no
  e2e step replaces the browser with an HTTP or fetch call.

## Deferrals

| Deferral | Recorded operator decision | Evidence and residual risk |
|---|---|---|
| `duty2cal-wasm`'s wasm-only wrapper lines excluded from the native Cobertura denominator; the facade is evidenced through the browser scenario; no native duplicate test. | **Present** — design-review operator decision 2 (2026-10-19); proposal D3, design D3, `docs/arc42/11` R-039/TD-016. | The shared core remains covered by the CLI rows; the browser scenario asserts the facade's result. Residual risk and revisit condition are recorded (R-039, TD-016). |
| Mid-conversion trap/out-of-memory is a manual release check, not a Gherkin scenario. | **Present** — design-review operator decision 4 (2026-10-19); design “Risks”, `docs/arc42/11` R-042/TD-018. The decision covers the *test forcing mechanism*; it does not waive implementing the catch/discard behaviour. That behaviour is missing — F1. | The check is to be repeated at each release and whenever Chromium or the WASM bundle changes; R-044 records the below-bound memory risk. |
| Firefox and Safari are manual release checks; CI drives Chromium only. | **Present** — the confirmed capability-based support decision in the ideation seed (2026-10-19) and the design-review residual risks; `docs/arc42/11` R-040/TD-017. | Support is capability-based, not a version matrix; a browser-specific defect can persist between manual checks. Recorded, not hidden. |

No other mandatory check is deferred. The absence of an archive verification
receipt is a workflow state (the archive has not run), not a deferral; the
recorded per-scope counts are in `tasks.md`.

## Use-case and Persona conformance

The owning `usecase.md`, the proposal’s use-case context and the ideation seed
were read. The implementation preserves the permanent conversion contract:
supported Released Roster family, activity classification, timing, identities,
RFC 5545 output, complete-or-explicit-failure, and the browser realization’s
offer/no-write/diagnostic rules. No conversion rule was forked.

### Use-case scope rejection criteria

| Criterion | Evidence | Verdict |
|---|---|---|
| Goal conjunction | `## Goal`: “Export one published duty plan as an importable personal calendar file.” One outcome. | PASS |
| Missing primary actor | `## Actors` identifies the Crew member; `## Trigger` names them. | PASS |
| Placement failure | `## Capabilities` contains `export-duty-plan-calendar`; the browser is the same goal’s channel, not a second capability. | PASS |
| Mutation under an observing goal | The goal is export (a change), not observation; the scenarios assert the offered calendar. | PASS |
| Extension without a step reference | Every `## Extensions` entry names main-flow steps 2–6 and adds no new success outcome. | PASS |
| Inconsistent preconditions | Capability/initialization are preconditions; size, family, completeness and output availability are flow checks; no precondition is established by the flow and no guarantee is assumed. | PASS |
| Mechanism vocabulary | Goal/trigger do not name a mechanism; the success guarantee’s browser/executable language is the expressly marked realization clause. | PASS |
| Component- or channel-shaped name | `export-duty-plan-calendar` is domain-shaped; “browser conversion” is not a capability name. | PASS |
| Oversized leaf | The main flow names one actor goal (export) and its sub-actions. | PASS |
| Relationship misuse | `## Extends` is none; no conditionless extension. | PASS |

### Personas

| Confirmed Persona | Disposition |
|---|---|
| `crew-member--personal-calendar-owner` | Primary accuracy/privacy lens. The browser scenario proves the same calendar as the executable for the same plan, an explicit rejection with an offered diagnostic, no plan leaving the tab, and browser-owned save with no page save claim. The visual review records the privacy-copy content gap (F2). |
| `project-maintainer--release-owner` | Release-evidence lens. The e2e scenario, the pinned toolchain and the recorded versions are reviewable; the page does not publish or claim a public URL, which the sibling change owns. |
| `contributor--web-reviewer` | CSP/no-network/fallback/WASM-bundle lens. The CSP, inline bundle, request log, storage snapshot, capability gate and fail-closed driver are in the same history as the page. |
| `visitor--project-reader` | Confirmed for the topic but assigned to `publish-public-website`; this change records no visitor impact and invents no visitor Persona. |

No migration ledger applies: this change adds a browser realization, not a
data/configuration migration.

## Visual review

Interface: **web**. Contract viewports: mobile 360x780 and desktop 1920x1080.
Declared screen states: empty, ready and failure; “converting” is transient.

### Evidence inspected

| Declared state | Screenshot(s) seen | Observations |
|---|---|---|
| empty | `360x780--empty.png`, `1920x1080--empty.png` | Heading, explanation, empty status, picker, drop zone, footer; all controls labelled; no offer. |
| ready | `360x780.png`, `1920x1080.png` | Ready status, picker/drop zone, `Calendar ready` heading, `Save 2026-06.ics`, browser-chooses-location note; no save claim. |
| failure | No `@e2e` screenshot; the approved design-only reference section in `docs/design/reference/converter-reference.html` was inspected. | The regular failure scenarios assert the heading, reason, optional diagnostic offer and `Choose another plan`; the visual contract names this reference. |
| cannot-run | No committed screenshot; asserted by the regular WebAssembly/ES-module/startup scenarios. | The page renders a `cannot-run` panel; the design state inventory and visual contract do not declare it (F4). |

The four committed PNGs are the required 360x780 and 1920x1080 images, all
non-blank, and the e2e baseline step asserts `lang="en"`, a real title, one
visible `h1`, visible labels and keyboard focus on every control, no duplicated
visible text, the reduced-motion computed value `0ms`, and `axe` cleanliness.
The layout, labels and focus baseline hold.

### Persona dispositions

| Persona | Verdict | Findings |
|---|---|---|
| `crew-member--personal-calendar-owner` | Goal reachable: choose/drop a plan, see `Calendar ready`, save `2026-06.ics`; a rejected plan shows a failure and a diagnostic. | The privacy note does not state that the page stores nothing or makes no request after load (F2). This is a contract-rule finding, not a failure to reach the save goal. |
| `project-maintainer--release-owner` | Goal reachable: the screen and its evidence are committed; the e2e scenario and pinned versions are reviewable. | None on the screen. |
| `contributor--web-reviewer` | Goal reachable: the CSP/inline-WASM/no-network/fallback design is inspectable, and the empty/ready captures show no third-party asset. | The `cannot-run` state is not in the declared state inventory (F4); no goal failure. |

### Contract findings

- **F2 — privacy note content.** The design system’s Privacy note primitive
  requires “processing is local, no upload, no storage, no request after load”;
  the approved reference footer says “The page stores nothing and sends no
  request after it loads.” The rendered footer
  (`web/index.html:47`) says “Processing happens in your browser. No account,
  email, or upload is used.” — local processing and no upload are present; no
  storage and no request after load are absent. This is a contract rule, not a
  pixel finding.
- **F3 — breakpoints/typography.** The contract declares 24 px page padding at
  480 px, 32 px at 720 px, a 720 px maximum column, and a 36 px `h1` at
  ≥ 720 px. `web/styles.css` uses `max-width: 44rem` (704 px), `padding:
  var(--space)` (16 px) at every width, `h1` 1.75 rem (28 px) at every width,
  and has no 480/720 media queries. The 1920 screenshot shows the 28 px heading
  and 16 px padding. The baseline does not hold against the approved scale.
- **F4 — state inventory.** The design’s state machine says “four observed
  states” and the visual contract declares empty/ready/failure; the page and
  `web/e2e/driver.mjs:155-163` also render/observe a fifth `cannot-run` state,
  asserted by scenarios 9–11. The state inventory and the declared visual
  states are therefore stale, and the cannot-run treatment has no declared
  reference/evidence policy.

`VISUAL-REVIEW: FAIL` — the four required screenshots are present and the
Persona goals are reachable, but the approved visual contract’s privacy-note,
breakpoint/typography and state-inventory rules do not hold in the
implementation, and the reviewed design must be corrected or the implementation
conformed before sign-off.

## Deterministic gate dispositions

## Overlap dispositions

`givn lint --change build-browser-converter` exits 0 and reports no shape
match, no subset advisory and no overlap finding involving this change. There
is no finding pair to disposition, and no removed+added scenario pair: the
delta adds 16 scenarios, the permanent feature is unchanged, and the use case
adds the browser interaction row without removing a scenario. No `duplicate`,
`variant` or `boundary` row is owed.

## Split-or-keep

The lint gate reports no scenario longer than the threshold, so no split/keep
row is owed. The longest browser scenarios are the persistence and cancelled-save
flows; each is a short step sequence with one invariant, and the lint gate does
not flag them. No `split`/`keep` decision is required.

## README impact decision

`README-IMPACT: none`

The public README face remains the executable route. This change builds and
serves the converter page from the built Site output; it does not publish the
Public website, its URL, the Built revision or the public copy. The README’s
Description, Quick start, Download/Prerequisites/Usage sections continue to
describe the supported release route. The public-site publication is the
sibling `publish-public-website` change; no README section was edited here and
none is stale for this change.

## Sign-off checklist

| Item | Result |
|---|---|
| Fabrication audit clean | **FINDING** — F7 (commit/task record) and F1–F6 (design/arc42 facts) remain. |
| Every checked task has a verified commit touching production code | **FINDING** — the eight verification-only commits listed in Step 0 item 2. |
| Every promised component exists | CLEAN. |
| Strict-mode proof present and passing | CLEAN — both runner proofs exit 101 on the deliberate undefined step; `.fail_on_skipped()` is active. |
| Scenario execution evidence recorded per scope | CLEAN as available evidence — `tasks.md` records 166/166 regular and 9/9 e2e; no archive receipt exists yet. |
| Coverage classification uses the most recent measured evidence | CLEAN — merged 880/894, 14 bucket-3 lines, browser boundary recorded. |
| Every coverage gap classified and resolved | CLEAN — bucket 3 rows above; no bucket 1/2 gap found. |
| Dead code deleted; missing tests added; hard-to-test gaps justified | CLEAN for the delivered code; the bucket-3 reasons are concrete and pre-existing TD-007. |
| Redundant unit tests removed | CLEAN — no unit test was added for the page/driver/facade; the `.feature` file is the source of truth. |
| No `@wip` tags; no implementation-layer detail in the spec | CLEAN — `givn lint` exit 0; grep found none. |
| Canonical E2E policy read; every normalized action has one `@e2e` scenario | CLEAN — two actions, two rows, one `@e2e` each; variants regular. |
| Every E2E scenario has a real-interface primary assertion and the reviewed driver | CLEAN — captured download/VEVENT compare and rendered-state assertions through Playwright. |
| Local run command starts the stack cleanly | CLEAN — `PORT=8123 ./scripts/run-web.sh` served `web/index.html` with HTTP 200; there are no digital twins. |
| Exact `verify.e2e_command` files identified; tree searched for a parallel implementation | CLEAN — files named in Step 0 item 7; no parallel implementation in the tree or untracked files. |
| `verify.e2e_command` differs from `verify.command`; per-scope counts prove the filter | CLEAN — `./run-tests.sh` vs `./run-tests.sh --e2e`; 166 vs 9. |
| Implementation matches every explicitly named design command/layout/framework | **FINDING** — commands/layout/framework match, but F1–F5 are stated design facts that the implementation or docs do not match. |
| Interaction coverage verified end to end | CLEAN — table in Step 0. |
| No finding excused outside the three buckets | CLEAN — the bucket-3 rows are the only classifications; no exception outside the three buckets is used. |

## Blocking findings and exact remediation

| ID | Finding | Exact remediation |
|---|---|---|
| F1 | `design.md` (WASM facade section) and `docs/arc42/11` R-042/R-044/TD-018 promise that a trap/OOM/non-structured exception discards the WASM instance so the next attempt starts from a fresh one. `web/app.js:155-165` catches the exception, shows the generic failure and attempts the diagnostic, but never discards or reinitializes the instance; `start()` calls `init` only once. | Add an instance-lifecycle reset in `web/app.js`: on the catch, mark the instance unusable/discard it and re-run `init(decodeBase64(wasmBase64))` before the next `convert` (or re-import the module), keeping no partial calendar and the same diagnostic. Re-run the operator-accepted manual trap/OOM check and record the fresh-retry result in the release evidence. If the discard rule is withdrawn instead, update `design.md`, R-042/R-044/TD-018 and the manual-check wording, then re-run design-review. |
| F2 | The design system’s Privacy note and the approved reference require “no storage, no request after load”; the rendered footer omits both. | Restore the approved privacy-note content in `web/index.html` (e.g. “No account, email, or upload is needed. The page stores nothing and sends no request after it loads.”), regenerate the four committed screenshots and re-run the `@e2e` scenario. Or update the design system/contract/reference to the intended shorter copy and re-run design-review; the current docs and implementation disagree. |
| F3 | The declared 480/720 breakpoints, 720 px column and 36 px `h1` at ≥720 px are not implemented; `web/styles.css` is 44 rem/16 px/28 px with no breakpoints. | Implement the declared breakpoints and scale in `web/styles.css` and regenerate the screenshots; or change the approved contract and reference and re-run design-review. |
| F4 | The design state machine declares four observed states and the visual contract declares empty/ready/failure, but the page/driver render and observe `cannot-run` (scenarios 9–11) with no declared reference or evidence policy. | Add `cannot-run` to the design’s state inventory, driver protocol and Visual Design Contract (or explicitly declare it a capability-gate treatment outside the visual state machine) and record its reference/evidence rule; re-run design-review because `design.md` changes. |
| F5 | The glossary `Offer` row and `design.md` traceability name `offerCalendar`, which does not exist; the code has `addOffer` and `showReady`. | Correct the glossary `Code symbol` to `addOffer`/`showReady` and correct `design.md:563,570`; re-run design-review because `design.md` changes. No production code change is needed. |
| F6 | `design.md` Validation decisions says the privacy guard excludes “only three recorded cases… No other path is excluded”; `tasks.md` final verification repeats that. `scripts/pii-guard.sh`’s tree scan also excludes `*.sh` (and the standard `.git`, `target`, `private` directories). | Either narrow the tree-scan exclusion to `scripts/pii-guard.sh` (the self-matching file) and re-run `bash scripts/pii-guard.sh`, or record the `*.sh` exclusion and its justified no-leak rationale in `design.md`, R-045 and the task verification, and re-run design-review if `design.md` changes. Do not leave the “only three/no other path” claim in place. |
| F7 | Eight checked scenario tasks have commits with no production diff; scenarios 2/3 still say “production files modified” while the diff is tests/spec/tasks only. The behaviour is not fabricated (it was committed in the scenario-1/7/12/14/15 commits), but the mechanical task→production check fails. | Correct the task records to name the commit that delivered the production behaviour for each verification-only scenario (2, 3, 4, 5, 6, 8, 10, 16), and remove the “production files modified” wording where the diff does not touch production. If the process requires each task’s own production commit, re-open the task and commit the production change under its own hash. |

No production code was changed in this review, and no archive action was taken.

REVIEW: FAIL
