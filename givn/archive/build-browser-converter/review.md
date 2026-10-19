# Review — build-browser-converter

Stage: `tasks-done` → `review` (final re-review after the F1–F7 and R1–R3
remediations).
Scope: the browser realization of `export-duty-plan-calendar` — the `ffi/wasm`
facade, the `web/` page, the Playwright/cucumber-rs browser harness, the
committed visual evidence, and the recorded suite/coverage results.

The full suite was not re-run in this pass. The available execution evidence is
the final-verification record in `tasks.md`, including its post-fix
`givn check tasks` run (166/166 scenarios, 744/744 steps, all 16 browser
scenarios targeted) and `./run-tests.sh --e2e` (9/9 scenarios, 71/71 steps).
The measured coverage evidence is `coverage/cobertura-coverage.xml`
(880/894 lines, merged from the distinct non-e2e and e2e reports). `givn lint`
is recorded clean. This review re-verified the implementation, the records and
the visual evidence by inspection and by the focused gates named below.

**Verdict: `## Archive-phase evidence path correction (2026-10-19)

The archive gate moves the change before running the suite. The e2e step now
resolves the evidence directory from the active change or the archived copy, so
the archive run writes the four screenshots into the candidate instead of
recreating `givn/changes/`. This is a test-evidence path fix; the product
behavior, assertions and reviewed evidence are unchanged. The review remains
PASS.

REVIEW: PASS`.** F1–F7 are fixed. The residual findings from the
preceding re-reviews are resolved: the design state inventory and driver
protocol now name five observed states including `cannot-run` and the design
review records the correction (R1); the reuse-scenario record names the
production commit for all ten test-only scenario commits (R2); every
`- [x] COMMIT:` line leads with its unique hash, so the review hook's
task-integrity check passes (R3); and the four evidence screenshots now use
the contract viewport names `mobile`/`desktop`, so the visual-evidence check
passes (R4). No production code, spec, design or task was changed in this
review, and no archive action was taken.

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | Delta feature tags plus `git log -p` over the delta feature | **CLEAN.** `specs/export-duty-plan-calendar/export-duty-plan-calendar.feature:4` still carries `@givn.added @e2e`; the only historical tag change is the removal of `@wip` (`4e0d466`). No `@e2e` tag was removed or moved. |
| 1 | Empty/no-op step bodies | Grep all 10 step-definition files (`apps/cli/tests/steps/*.rs` capability files and `apps/cli/tests/steps/e2e/*.rs`) for `{}`, bare `pass`/`return`, `unimplemented!`, `todo!` | **CLEAN — 0 empty step bodies found across 10 step-definition files.** The `{}` hits are Rust match arms and `format!` braces in `support.rs`; no step body is empty or a no-op. |
| 2 | Every checked task has a commit touching production | `git show --name-only` for all 16 scenario hashes; reuse record cross-check | **CLEAN.** Scenarios 1, 7, 12, 13, 14 and 15 have production diffs. The ten test-only scenario commits (`afd900b`, `8b33b34`, `b078df5`, `c6eab05`, `66593de`, `78ad4e4`, `82cbe13`, `5a82432`, `257f662`, `43e916d`) are all in the reuse-scenario production record with their production commit (`4e0d466`, `63974a1` or `30008f0`). Every `- [x] COMMIT:` line leads with its unique hash; no task is a stub. |
| 3 | Every component `design.md` promises exists | Changed-files table vs tree | **CLEAN.** `ffi/wasm` (`Cargo.toml`, `src/lib.rs`, `clippy.toml`), `web/` (`index.html`, `capability.js`, `app.js`, `styles.css`, `.wasm-pack-version`, generated `pkg/`), `web/e2e/` (driver, package files, `.nvmrc`), `scripts/build-web.sh`, `scripts/run-web.sh`, `docs/design/design-system.md`, `docs/design/reference/converter-reference.html`, the changed step files/runner, `Cargo.lock`, `THIRD-PARTY-NOTICES.md`, `run-tests.sh`, `measure-coverage.sh`, CI, `.gitignore` and the use case are all present. |
| 4 | Strict-mode proof present with output | `tasks.md` setup and e2e setup sections | **CLEAN.** The setup proof ran a temporary undefined step and exited 101 (`RUNNER_EXIT=101`); the e2e proof ran an undefined `@e2e` step and exited 101 (`E2E_PROBE_EXIT=101`). `.fail_on_skipped()` is in `apps/cli/tests/features_runner.rs`; the driver fails closed on unknown actions and the Rust client panics on non-`ok` results. |
| 5 | `@e2e` Then steps assert the real interface | Delta scenario and every e2e step | **CLEAN.** The Then steps assert the rendered empty/ready states, the offered `2026-06.ics`, the browser-captured download (VCALENDAR shell, CRLF, balanced `VEVENT` blocks, exact reference-fixture event match), the request log, the visible no-save/no-account text and the visual baseline. No Then asserts only a repository/database. |
| 6 | Browser steps actually drive the browser | Every browser step and `web/e2e/driver.mjs` | **CLEAN.** `set_file` uses Playwright `page.setInputFiles('#plan', …)`; `save_offer` clicks the real `<a download>` and captures `page.waitForEvent('download')`; `dismiss_save` cancels the browser download; `reload` uses `page.reload`; observations use the rendered DOM, request log, storage snapshot and screenshots. No browser step sends an HTTP request for the plan or calls page-side `fetch`/`XMLHttpRequest`. |
| 7 | Exact files `verify.e2e_command` invokes, and parallel implementations | `givn/commands.yaml` + `run-tests.sh` + tree search including untracked files | **CLEAN.** `verify.e2e_command` is `./run-tests.sh --e2e`; `run-tests.sh` calls `scripts/assemble-features.sh` and then `cargo test -p duty2cal --test features_runner -- --tags "@e2e"`. The invoked implementation is `apps/cli/tests/features_runner.rs` → `steps/mod.rs` → `steps/e2e/export_duty_plan_calendar.rs` (+ shared steps in `steps/export_duty_plan_calendar.rs`) → `steps/browser.rs` → `web/e2e/driver.mjs`. A tree search for `setInputFiles`/`waitForEvent('download')`/`chromium` found only `web/e2e/driver.mjs`; no parallel implementation exists. |
| 8 | Normalized E2E scope | `usecase.md` `## Interactions` vs delta feature | **CLEAN.** The inventory has two distinct actions (executable supply; browser supply-and-save). The browser delta has exactly one `@e2e` scenario; size, family, unresolved activity, stale offer, drop, capabilities, startup, offer failures, cancelled save, stem and persistence are regular scenarios. Drop and equivalence are stated variants/invariants, not additional end-to-end actions. |
| 9 | `verify.e2e_command` is scoped, not a copy of `verify.command` | Literal `givn/commands.yaml`; recorded per-scope counts | **CLEAN.** `verify.command` is `./run-tests.sh`; `verify.e2e_command` is `./run-tests.sh --e2e` (different strings). `tasks.md` records 166 regular scenarios and 9 e2e scenarios; 9 < 166. No archive verification receipt exists yet because the archive step has not run; the recorded final verification is the available execution evidence and will be superseded by the receipt. |
| 10 | Implementation vs `design.md` commands/layout/framework and stated facts | File tree, manifests, scripts, page source, durable docs | **CLEAN.** wasm-pack 0.15.0 with `--target web --out-dir ../../web/pkg`; Playwright 1.64.1; two capability step files; the JSON-lines driver; `run-tests.sh --e2e`. The state inventory now says five observed states, the driver protocol includes `cannot-run`, the reference descriptions include the startup view, and `design-review.md` records four reference states with the post-review correction. |
| 11 | Interaction coverage cross-reference | `usecase.md`, design matrix, feature, step files | **CLEAN — full table below.** Two inventory entries, two matrix rows, one matching `@e2e` scenario each, and the promised driving mechanism is the one implemented. |
| 12 | Findings from 1–11 | — | No unfinished work remains. No pass marker is written while a finding remains; none does. |

### Interaction coverage cross-reference (check 11)

| Inventory entry (`givn/specs/export-duty-plan-calendar/usecase.md`) | Design matrix row | `@e2e` scenario | e2e step file(s) | Driving mechanism promised / found | Match? |
|---|---|---|---|---|---|
| supply a duty plan for conversion through the executable | supply … through the executable (CLI, terminal) | `Crew member converts a supported duty plan and receives a calendar file` (permanent feature) | `apps/cli/tests/steps/export_duty_plan_calendar.rs`; `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` | `std::process::Command` on the built `duty2cal` binary / the same `std::process::Command` | MATCH |
| supply a duty plan through the converter page and save the offered calendar | supply … through the converter page and save the offered calendar (Web UI) | `Crew member converts a supported duty plan in the browser and saves the offered calendar` (delta feature) | `apps/cli/tests/steps/export_duty_plan_calendar.rs`; `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`; `apps/cli/tests/steps/browser.rs` | Playwright 1.64.1 → `web/e2e/driver.mjs` → Chromium, loopback-served `web/` / the same Playwright driver with `setInputFiles`, offer click and download capture | MATCH |

No missing or extra matrix row; no unmapped interaction; no `reqwest` or other
HTTP client substitutes Playwright.

### Previous-finding verification (F1–F7)

| ID | Verified state | Evidence | Verdict |
|---|---|---|---|
| F1 | Trap/OOM recovery discards and reinstantiates | `web/app.js:82-89` (`ensureInstance` re-runs `init(decodeBase64(wasmBase64))` when `!instanceReady || needsReinit`); the `convert` catch at `web/app.js:171-181` sets `needsReinit = true` and offers no partial calendar; `start()` calls `ensureInstance()`. The generated `web/pkg/duty2cal_wasm.js` `__wbg_init` re-runs load/finalize and replaces the instance. | **FIXED** |
| F2 | Footer states no storage and no request after load | `web/index.html:47`: “Nothing is stored, and the page sends no request after it loads.” Visible in all four screenshots. | **FIXED** |
| F3 | 480/720/1200 breakpoints, 720 px column, 36 px h1 | `web/styles.css:24-58`: 16 px base padding, 24 px at `min-width: 480px`, 32 px and 36 px `h1` at `min-width: 720px`, `max-width: 720px`; the 1200 px rule is the unchanged centered column, as the contract states. Screenshots show 28 px/16 px at 360 and 36 px/32 px in a 720 px column at 1920. | **FIXED** |
| F4 | `cannot-run` in design inventory, driver protocol, design-system and reference; visual contract distinguishes it | `design.md:223` now says five observed states (four conversion-flow states, one transient, plus the `cannot-run` startup state) above the five-row table; `design.md:446` lists `empty/converting/ready/failure/cannot-run`; `design.md:105` and `:688` include the startup reference view; `docs/design/design-system.md:11-13` and `docs/design/reference/converter-reference.html:120-124` declare it; `design-review.md:194` says four reference states and the post-review correction section records the change. | **FIXED** |
| F5 | `offerCalendar` symbol exists and is used by `showReady` | `web/app.js:105-113` defines `offerCalendar`; `web/app.js:117` calls it from `showReady`; `design.md:571,578` and `docs/arc42/12-glossary.md:74` name it. | **FIXED** |
| F6 | Privacy guard excludes only the guard file/node_modules/state; docs record the standard dirs and three content exclusions | `scripts/pii-guard.sh:40-62`: the tree scan excludes `.git`, `target`, `private`, `node_modules`, `terraform.tfstate*` and `pii-guard.sh` by name — no `*.sh`; the history scan excludes `scripts/pii-guard.sh`, `infra/terraform.tfstate*` and the superseded `apps/cli/tests/fixtures/expected/2026-06.ics`. `design.md:418-424`, `docs/arc42/11…` R-045 and `tasks.md` final verification state the same three content exclusions plus the standard directories, and explicitly state that every other path, including shell scripts, is scanned. | **FIXED** |
| F7 | Reuse-scenario production record names the production commit for each test-only scenario | `tasks.md:1773-1795` now lists all ten test-only scenario commits, including scenario 9 (`82cbe13` → `30008f0`) and scenario 11 (`257f662` → `30008f0`); their GREEN headings say “reuse of production code introduced by `30008f0` (no new production file)”. | **FIXED** |

## Arc42 implementation conformance

The durable `docs/arc42/` chapters and the change-level `arc42.md` were read.
The affected chapters were independently re-derived from the completed
implementation, `design.md` and `tasks.md` before accepting the change-level
claims. All twelve chapters are affected.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 1 — browser conversion route and truthful-copy goal | `docs/arc42/01-introduction-and-goals.md` | affected: second realization; truthful copy | Scope, release note | setup, 1–16 | `web/`, `ffi/wasm`, `web/e2e/` | yes |
| 2 — `wasm32`, target-scoped deps, no-I/O facade, CSP/inline bundle, 20 MiB bound, pinned test supply chain | `docs/arc42/02-architecture-constraints.md` | affected: constraints | Technology decisions, WASM facade | setup, 1–3, 16 | `ffi/wasm/Cargo.toml`, `web/index.html`, `web/.wasm-pack-version`, `.nvmrc`, `package.json` (Playwright 1.64.1) | yes |
| 3 — browser boundary, loopback Site output, Site-publication boundary | `docs/arc42/03-context-and-scope.md` | affected: context | Architecture impact, scope | setup, 16 | `driver.mjs` loopback server, page code | yes |
| 4 — browser-local delivery strategy and ADR-0007 | `docs/arc42/04-solution-strategy.md` | affected: strategy | Decisions, architecture impact | setup | ADR-0007, `ffi/wasm`, `web/` | yes |
| 5 — `ffi/wasm`, `web/`, build/evidence toolbox | `docs/arc42/05-building-block-view.md` | affected: building blocks | new/changed files | setup, 1–16 | tree inspection | yes |
| 6 — browser conversion/offer, failure/diagnostic, capability/startup flows | `docs/arc42/06-runtime-view.md` | affected: runtime | state machine, runtime tables, `read_state` protocol | 1–16 | `web/app.js`, `capability.js`, driver actions; five observed states including `cannot-run` | yes |
| 7 — static Site output, loopback serving, browser runtime, test-only tooling | `docs/arc42/07-deployment-view.md` | affected: deployment | local runnability, file layout | setup, 16 | `scripts/run-web.sh`, `driver.mjs` | yes |
| 8 — CSP, inline WASM, browser offer/no-write, browser E2E, version pinning | `docs/arc42/08-crosscutting-concepts.md` | affected: crosscutting | network/CSP, accessibility, test runner | setup, 1–16 | `web/index.html`, `web/app.js`, driver, lockfiles | yes |
| 9 — ADR-0007 register entry and indexes | `docs/arc42/09-architecture-decisions.md`, `adr/README.md` | affected: ADR | ADR routing | setup | `docs/arc42/adr/ADR-0007-…md` | yes |
| 10 — browser equivalence/privacy/failure/capability/input/evidence quality scenarios | `docs/arc42/10-quality-requirements.md` | affected: quality | traceability, matrix | 1–16 | `@e2e` + regular scenarios | yes |
| 11 — R-036…R-045 and TD-016…TD-018 | `docs/arc42/11-risks-and-technical-debt.md` | affected: risks/debt | risks, coverage boundary | final verification | R-042/R-044/TD-018 match the `needsReinit`/`ensureInstance` recovery; R-045 matches the revised guard exclusions | yes |
| 12 — **Offer**, **Supported platform**, **Site output**, lifecycle events | `docs/arc42/12-glossary.md` | affected: glossary | traceability, decisions | 2, 5, 7 | glossary names `offerCalendar`/`acceptFile`; both exist | yes |
| `design.md` state inventory and driver protocol | `design.md` state machine; driver protocol | — | five observed states; `read_state` includes `cannot-run` | 9–11, 16 | `web/e2e/driver.mjs:155-163` returns `cannot-run` | yes |
| Reference descriptions and design-review artifact | `design.md:105,688`; `design-review.md:194,233-241` | — | reference descriptions include the startup view; four reference states recorded with the post-review correction | — | `converter-reference.html` has four reference views | yes |

The five arc42 sign-off checks: (1) affected chapters were independently
derived — yes; (2) every affected chapter is current — yes; (3) each fact was
cross-referenced to design/tasks/implementation — yes; (4) no omission or
stale contradiction remains — yes; (5) the clean marker is recorded only after
all rows match — yes.

**ARC42 CONFORMANCE: CLEAN**

## Ubiquitous language conformance

The specs, design and code reuse the durable terms **Browser conversion**,
**Offer**, **Site output**, **Supported platform**, and the lifecycle events
**Calendar export offered** / **Conversion diagnostic offered** consistently at
the concept level.

- The `Offer` row in `docs/arc42/12-glossary.md:74` names
  `web/app.js` (`offerCalendar`, `acceptFile`); both symbols exist, and
  `design.md:571,578` points at `offerCalendar`.
- “Published/written” stays with the executable and “offered” with the
  browser; “Supported platform” still names downloadable archive platforms;
  the spec says “converter page served from the built Site output”, not
  “Public website”.
- The state vocabulary is consistent across `design.md`,
  `docs/design/design-system.md`, the reference and `web/e2e/driver.mjs`
  (`empty`, `converting`, `ready`, `failure`, `cannot-run`); no `@wip` or
  implementation-layer state name leaks into the spec.

**LANGUAGE CONFORMANCE: CLEAN**

## Coverage classification

The most recent measured evidence is `coverage/cobertura-coverage.xml`
(merged from two distinct instrumented runs recorded in `tasks.md`):
**880 of 894 production lines covered (line-rate 0.984340)**, 14 uncovered.
The non-e2e report is 880/894 and the e2e report is 614/894 with a different
md5 (`9ca7744e…` vs `1bcb9e49…`); `duty2cal-wasm` is absent from the native
denominator as designed. The browser scenarios ran GREEN in the recorded final
verification (166/166 regular, 9/9 e2e).

Every uncovered region is classified below; no region is left unclassified.

| File | Lines | What the uncovered code is | Bucket | Disposition |
|---|---|---|---|---|
| `crates/duty2cal-pdf/src/lib.rs` | 288, 365, 383, 389, 407, 408 | Reader guards for input the supported family cannot print: a missing date band, the `assign_cells` closing path when `areas` is empty, an out-of-range footer date, a non-UTC footer zone, and the compact-date month/day conversions. | 3 — legitimately hard to test | A fixture of the supported family cannot present these shapes; the behaviours they guard are pinned by the unusable-stamp/no-date-band scenarios. Pre-existing and recorded as TD-007. |
| `crates/duty2cal-core/src/domain/mod.rs` | 184–186 | The exhaustive-match arms of `DutyEntry::date()` for travel, availability and non-flight duty. | 3 — legitimately hard to test | Every one of those variants is exercised; the accessor is only asked for a date by the all-day and layover paths, and LLVM attributes the multi-line `|` pattern's continuation lines as uncovered. Pre-existing, TD-007. |
| `apps/cli/src/fs.rs` | 47, 54 | The retry arm when another process claims a candidate between the existence check and `create_new`, and the `Ok(None)` return after the bounded name enumeration is exhausted. | 3 — legitimately hard to test | The retry needs a second process to interleave; the exhaustion needs 10 000 occupied names. The occupied-name behaviour itself is covered. Pre-existing, TD-007. |
| `crates/duty2cal-core/src/adapters/ics/mod.rs` | 88, 89 | Escaping a raw newline and dropping a carriage return. | 3 — legitimately hard to test | A PDF text cell cannot carry either character; the sink guard is defensive. Pre-existing, TD-007. |
| `apps/cli/src/main.rs` | 27 | The fall-through when `DUTY2CAL_CONSTRUCTION_CLOCK` is set but unparsable. | 3 — legitimately hard to test | The seam is a test affordance; both real outcomes (a usable instant and the system clock) are covered. Pre-existing, TD-007. |
| `web/app.js`, `web/capability.js`, `ffi/wasm` wasm-only wrapper | behaviour-level | The page and the wasm wrapper have no native line-coverage signal: the Chromium runtime has no file-based LLVM/JS coverage collector in this toolchain, and `duty2cal-wasm` is outside the native Cobertura denominator. | 3 — legitimately hard to test (measurement boundary) | Every observable behaviour is driven by the 16 browser scenarios and the one browser `@e2e` scenario; the shared core stays in the native denominator. This is the operator-approved D3 boundary recorded in the proposal/design/design-review and chapter 11. The `needsReinit`/`ensureInstance` branch is behaviour-evidenced through the manual trap/OOM release check (R-042, TD-018). |

- **Bucket 1 (dead code):** none found in the delivered code.
- **Bucket 2 (missing coverage):** none found in the delivered code; no
  downgraded or missing `@e2e` scenario exists for either normalized action.
- **Bucket 3:** the rows above.

This change introduces no uncovered line in the native conversion crates. The
browser page/facade are not native-coverage candidates; their behaviour is
pinned by real-browser evidence, and the measurement boundary is the recorded
operator decision.

## E2E coverage

The capability is a **Web UI**: the page is rendered in Chromium and the e2e
scenario drives it. The opposite classification — “no browser driver is
needed, the capability is only a CLI” — does not hold for the browser
realization: the capability renders a page with a picker, status, offer and
failure panels, and a terminal/HTTP assertion could not observe those states.
The executable row remains a CLI interface.

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
| Mid-conversion trap/out-of-memory is a manual release check, not a Gherkin scenario. | **Present** — design-review operator decision 4 (2026-10-19); design “Risks”, `docs/arc42/11` R-042/TD-018. The decision covers the test forcing mechanism; it does not waive the catch/discard behaviour. That behaviour is implemented in `web/app.js` (F1). | The check is repeated at each release and whenever Chromium or the WASM bundle changes; R-044 records the below-bound memory risk. |
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

### Persona dispositions

| Confirmed Persona | Disposition |
|---|---|
| `crew-member--personal-calendar-owner` | Primary accuracy/privacy lens. The browser scenario proves the same calendar as the executable for the same plan, an explicit rejection with an offered diagnostic, no plan leaving the tab, and browser-owned save with no page save claim. The screenshots show the complete privacy note: no storage and no request after load. |
| `project-maintainer--release-owner` | Release-evidence lens. The e2e scenario, the pinned toolchain and the recorded versions are reviewable; the page does not publish or claim a public URL, which the sibling change owns. |
| `contributor--web-reviewer` | CSP/no-network/fallback/WASM-bundle lens. The CSP, inline bundle, request log, storage snapshot, capability gate and fail-closed driver are in the same history as the page. The `cannot-run` state has a declared inventory row, driver-protocol value and reference view. |
| `visitor--project-reader` | Confirmed for the topic but assigned to `publish-public-website`; this change records no visitor impact and invents no visitor Persona. |

No migration ledger applies: this change adds a browser realization, not a
data/configuration migration.

## Visual review

Interface: **web**. Contract viewports: mobile 360x780 and desktop 1920x1080.
Declared conversion-screen states: empty, ready and failure; the separate
`cannot-run` startup state is declared as well.

PERSONAS: crew-member--personal-calendar-owner
PERSONAS: project-maintainer--release-owner
PERSONAS: contributor--web-reviewer

### Evidence inspected

| Declared state | Screenshot(s) / reference seen | Observations |
|---|---|---|
| empty | `mobile--empty.png`, `desktop--empty.png` | Heading, explanation, empty status, picker, drop zone, footer. At mobile (360x780): 16 px page padding and a 28 px two-line `h1`; at desktop (1920x1080): a 720 px centered column, 32 px padding and a 36 px `h1`. All controls labelled; no offer. |
| ready | `mobile.png`, `desktop.png` | Ready status, picker/drop zone, `Calendar ready` heading, `Save 2026-06.ics`, browser-chooses-location note; no save claim. Same viewport scale as empty. |
| failure | No `@e2e` screenshot; the approved design-only failure section in `docs/design/reference/converter-reference.html` was inspected. | The regular failure scenarios assert the heading, reason, optional diagnostic offer and `Choose another plan`; the visual contract names this reference. |
| cannot-run | No committed screenshot; the startup section at `docs/design/reference/converter-reference.html:120-124` was inspected, and scenarios 9–11 assert the rendered panel. | The page renders the cannot-run panel with the heading and reason and no picker; the design-system, state inventory and visual contract declare it. |

The four evidence PNGs are at the required 360x780 and 1920x1080 resolutions, all
non-blank, and the e2e baseline step asserts `lang="en"`, a real title, one
visible `h1`, visible labels and keyboard focus on every control, no duplicated
visible text, the reduced-motion computed value `0ms`, and `axe` cleanliness.
The privacy note is present in all four captures; the layout, labels and focus
baseline hold.

The four evidence files are present in the working tree (to be committed with
the change) at the contract’s viewport names
(`mobile.png`, `mobile--empty.png`, `desktop.png`, `desktop--empty.png`), so
the evidence convention
`evidence/visual/<scenario-slug>/<viewport>.png` (final state) and
`<viewport>--<state>.png` (additional declared state) holds.

### Persona findings and dispositions

| Persona | Verdict | Findings |
|---|---|---|
| `crew-member--personal-calendar-owner` | Goal reachable: choose/drop a plan, see `Calendar ready`, save `2026-06.ics`; a rejected plan shows a failure and a diagnostic. | None. The privacy note states that nothing is stored and no request is sent after load. |
| `project-maintainer--release-owner` | Goal reachable: the screen and its evidence are committed; the e2e scenario and pinned versions are reviewable. | None on the screen. |
| `contributor--web-reviewer` | Goal reachable: the CSP/inline-WASM/no-network/fallback design is inspectable, and the empty/ready captures show no third-party asset. | None on the screen; the `cannot-run` state is declared. |

VISUAL-REVIEW: PASS

`VISUAL-## Archive-phase evidence path correction (2026-10-19)

The archive gate moves the change before running the suite. The e2e step now
resolves the evidence directory from the active change or the archived copy, so
the archive run writes the four screenshots into the candidate instead of
recreating `givn/changes/`. This is a test-evidence path fix; the product
behavior, assertions and reviewed evidence are unchanged. The review remains
PASS.

REVIEW: PASS` — the four evidence images exist at the contract
viewport paths, the privacy-note, breakpoint/typography rules and Persona
goals hold, and no finding names a pixel-level or coordinate defect.

## Deterministic gate dispositions

`givn lint --change build-browser-converter` is recorded clean (post-fix) and
reports no shape match, no subset advisory and no overlap finding involving
this change. The review’s mandatory hook, `givn check review --change
build-browser-converter`, exits 0: it passes the task integrity check (every
`- [x] COMMIT:` line leads with its unique 40-character hash), the run
declaration, the overlap dispositions, the Persona disposition and the
visual-evidence check (`mobile.png`, `mobile--empty.png`, `desktop.png`,
`desktop--empty.png`). Its only output beyond the passes is an advisory
warning that `proposal.md`’s Capability Routing table was not recognised as
declaring `export-duty-plan-calendar`; the table does record
`EXTEND export-duty-plan-calendar` and the proposal’s Use-Case Context names
the same capability, so this is a formatting advisory, not an undeclared
capability and not a review blocker. No finding pair exists to disposition.

## Overlap dispositions

No finding pair exists to disposition. The delta adds 16 scenarios, the
permanent feature is unchanged, and the use case adds the browser interaction
row without removing a scenario. No `duplicate`, `variant` or `boundary` row is
owed, and there is no removed+added pair whose supersession must be recorded.

## Split-or-keep

The lint gate reports no scenario longer than the threshold. The longest
browser scenarios are the persistence and cancelled-save flows; each is a short
step sequence with one invariant and is not flagged. No `split`/`keep` decision
is required.

## README impact decision

README-IMPACT: none

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
| Fabrication audit clean | CLEAN — 0 empty step bodies, every test-only commit recorded with its production commit, and the hook’s task-integrity check passes. |
| Every checked task has a verified commit touching production code | CLEAN — production diffs for scenarios 1, 7, 12, 13, 14 and 15; the other ten are reuse scenarios whose production commits are named in `tasks.md`. |
| Visual evidence complete for the declared viewports | CLEAN — the four images exist at `mobile.png`, `mobile--empty.png`, `desktop.png` and `desktop--empty.png`, hold the baseline, and the visual-evidence integrity check passes. |
| Every promised component exists | CLEAN. |
| Strict-mode proof present and passing | CLEAN — both runner proofs exit 101 on the deliberate undefined step; `.fail_on_skipped()` is active. |
| Scenario execution evidence recorded per scope | CLEAN as available evidence — `tasks.md` records 166/166 regular and 9/9 e2e; no archive receipt exists yet. |
| Coverage classification uses the most recent measured evidence | CLEAN — merged 880/894, 14 bucket-3 lines, browser boundary recorded. |
| Every coverage gap classified and resolved | CLEAN — bucket 3 rows above; no bucket 1/2 gap found. |
| Dead code deleted; missing tests added; hard-to-test gaps justified | CLEAN for the delivered code; the bucket-3 reasons are concrete and pre-existing TD-007. |
| Redundant unit tests removed | CLEAN — no unit test was added for the page/driver/facade; the `.feature` file is the source of truth. |
| No `@wip` tags; no implementation-layer detail in the spec | CLEAN — recorded `givn lint` exit 0; grep found none. |
| Canonical E2E policy read; every normalized action has one `@e2e` scenario | CLEAN — two actions, two rows, one `@e2e` each; variants regular. |
| Every E2E scenario has a real-interface primary assertion and the reviewed driver | CLEAN — captured download/VEVENT compare and rendered-state assertions through Playwright. |
| Local run command starts the stack cleanly | CLEAN — `PORT=8125 scripts/run-web.sh` served `/index.html` and `/app.js` with HTTP 200 during this review; there are no digital twins. |
| Exact `verify.e2e_command` files identified; tree searched for a parallel implementation | CLEAN — files named in Step 0 item 7; no parallel implementation in the tree or untracked files. |
| `verify.e2e_command` differs from `verify.command`; per-scope counts prove the filter | CLEAN — `./run-tests.sh` vs `./run-tests.sh --e2e`; 166 vs 9. |
| Implementation matches every explicitly named design command/layout/framework | CLEAN — commands/layout/framework match; the state inventory, driver protocol and reference/design-review text now match. |
| Interaction coverage verified end to end | CLEAN — table in Step 0. |
| No finding excused with a classification outside the three buckets | CLEAN — the bucket-3 rows are the only coverage classifications; no exception outside the three buckets is used. |

## Blocking findings and exact remediation

No blocking findings remain. The F1–F7 fixes, the R1–R3 remediations and the
R4 evidence-path correction are verified in the current tree. The four
screenshots now live at `mobile.png`, `mobile--empty.png`, `desktop.png` and
`desktop--empty.png`; the dimension-named files are removed; and the `@e2e`
scenario re-ran with all 12 steps passing. No production code was changed in
this review, and no archive action was taken.

## Archive-phase evidence path correction (2026-10-19)

The archive gate moves the change before running the suite. The e2e step now
resolves the evidence directory from the active change or the archived copy, so
the archive run writes the four screenshots into the candidate instead of
recreating `givn/changes/`. This is a test-evidence path fix; the product
behavior, assertions and reviewed evidence are unchanged. The review remains
PASS.

REVIEW: PASS
