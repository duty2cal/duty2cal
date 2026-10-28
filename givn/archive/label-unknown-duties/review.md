# Review: label-unknown-duties

## Fabrication audit

- **Empty or trivial step bodies:** 0 found across `apps/cli/tests/steps/export_duty_plan_calendar.rs`, `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`, `apps/cli/tests/steps/browser.rs`, `apps/cli/tests/steps/support.rs` and the two e2e submodules. No `unimplemented!`, `todo!`, empty body or assertion-free step remains.
- **Checked tasks with a production-touching commit:** every scenario commit was inspected with `git show --stat`.
  - `5d94ee0` — `crates/duty2cal-core/src/domain/mod.rs` (timed title prefix).
  - `ed6cb4b` — `crates/duty2cal-core/src/domain/mod.rs` (uniform arm).
  - `cf8f0ab`, `6463fd5`, `3359aa5` — guard scenarios; their modified delta scenario text and tasks are the regression evidence, production already carried the rule from `5d94ee0`/`ed6cb4b`.
  - `fdd485b` — `crates/duty2cal-core/src/application/mod.rs`, `ffi/wasm/src/lib.rs`, `web/app.js`, `web/index.html`, the fixture, the support re-export and the step file.
  - `e09fcbe`, `e512428`, `c24f16e`, `7374983` — guard/page scenarios; their committed scenario text and evidence, production already carried the behaviour from `fdd485b`.
  - `d923599` — the permanent multi-row step assertion fix (`SUMMARY:unknown duty: <code>`), design changed-binding record and the full browser evidence set.
- **Promised components exist:** `UnknownDuty`/`ConversionOutcome.unknown_duties` (`crates/duty2cal-core/src/application/mod.rs`), `set_unknown_duties` and the two page formatters (`ffi/wasm/src/lib.rs`), `unknownDutyLine`/`showReady` (`web/app.js`), the ready markup (`web/index.html`), `all_day_warned_plan` (`crates/duty2cal-pdf-fixtures/src/lib.rs`), and the new step bindings (`apps/cli/tests/steps/export_duty_plan_calendar.rs`). None missing.
- **Strict-mode proof:** `.fail_on_skipped()` in `apps/cli/tests/features_runner.rs`; tasks.md carries the targeted behavioral RED (`SUMMARY:ZZ_M_CF`) and the full pre-implementation baseline (`135 passed, 5 failed`).
- **E2E Then assertions:** the CLI `@e2e` scenario asserts the published calendar and its bytes through the real subprocess; the browser `@e2e` scenario asserts the rendered page, the saved calendar, the request log and the persisted-storage snapshot through Playwright. Neither asserts repository state alone.
- **Browser E2E driving:** `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` calls the shared `driver()` (Playwright through `web/e2e/driver.mjs`); no step sends an HTTP request or calls `fetch` in the page context.
- **Files `verify.e2e_command` invokes:** `givn/commands.yaml` sets `verify.e2e_command: "./run-tests.sh --e2e"`, which runs `cargo test -p duty2cal --test features_runner` over the assembled feature root with the `@e2e and not @browser` filter. The e2e step implementation is the single file `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`; no parallel e2e implementation exists in the tree, tracked or untracked.
- **E2E scope:** the inventory has two distinct actions (supply through the executable; supply through the page and save the offered calendar), each with exactly one `@e2e` scenario. The new all-day scenario is `@browser` regular coverage, not a third action. No excess `@e2e`.
- **Command separation:** `verify.command` is `./run-tests.sh`; `verify.e2e_command` is `./run-tests.sh --e2e`. They are not identical. The measured counts are 140 regular scenarios and 2 e2e scenarios, so the e2e scope is strictly smaller.
- **Design conformance:** the runner, step-file layout, fixtures, driver and unchanged CSS match `design.md`. The only design line that is not a runnable command is the quality-gate shorthand `cargo test --workspace`; the design's own runner section names `./run-tests.sh`, which passes, and the design names the wasm unit test separately. No reviewed decision was silently overridden.

## Interaction coverage

| Inventory entry (usecase.md) | Matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| supply a duty plan for conversion through the executable | yes | Crew member converts a supported duty plan and receives a calendar file | `std::process::Command` runs the built `duty2cal` binary; reads the transcript, exit status and published calendar |
| supply a duty plan through the converter page and save the offered calendar | yes | Crew member converts a supported duty plan in the browser and saves the offered calendar | Playwright through `web/e2e/driver.mjs`: file picker, download capture, request log, storage snapshot and rendered state |

No inventory entry is unmapped, no matrix row is extra, and the step files use the promised drivers.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Domain title | ch.12 **Unknown duty**, ch.10 QS-001 | row 12 Yes | `DutyEntry::summary()` | tasks 1–2 | `summary()` arm returns `unknown duty: {code}`; scenarios 1–5 pass | yes |
| Structured ready-list facts | ch.05 `ffi/wasm`, ch.06 browser sequence | rows 5, 6 Yes | `UnknownDuty`/`unknown_duties`/`unknownDuties` | task 6 | `application/mod.rs` projection, wasm array, `web/app.js` composition; scenario 6 passes | yes |
| Ready-state copy and order | ch.05 `index.html`/`app.js`, ch.08, ch.10 QS-046 | rows 3, 5, 8, 10 Yes | Page DOM/JavaScript section | tasks 6–10 | `web/index.html` order and copy; 26 browser scenarios pass; screenshots inspected | yes |
| Failure state and reports unchanged | ch.06, ch.08, ch.11 | rows 6, 8, 11 Yes | D3 | tasks 6, 10 | `showFailure`, `diagnostic_line`, report builders and CLI untouched; permanent failure scenarios pass | yes |
| Date rule for timed list lines | ch.06 step 4, ch.12 **Unknown duty** | row 6 Yes | D1 | task 6 | projection uses `interval.start().date()`; all-day uses the entry date; both line forms asserted | yes |
| No new ADR | ch.09 register | row 9 Yes, NOT_QUALIFIED | Decisions ledger | — | register and `adr/README.md` unchanged; no new MADR file | yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language

`docs/arc42/12-glossary.md` holds **Unknown duty** (calendar activity represented from an unrecognized code and titled `unknown duty: <printed code>`), and **Conversion warning** points at both presentations. The spec, design, use case and code use `unknown duty` / `UnknownDuty` / `UnrecognizedActivity` consistently: the diagnostic wording keeps `unrecognized activity code` as the anti-term, and **Standby shift** still records that `SB_F_CF`, `SB_M_CF` and `SB_S_CF` are recognized and produce no unknown duty. No finding.

## Coverage

No measured coverage report exists for this candidate yet; the archive will carry the merged result. Static classification:

- **Dead code:** none. The intermediate timed-only arm was replaced by the uniform arm in the next commit; no unused helper or fixture remains.
- **Missing test coverage:** none found. The domain title, the structured projection, the wasm array, the page line composition, the empty path, the two-row order and the offer-failure path are each exercised by a scenario through the real interface. The JavaScript is not line-instrumented (browser scenarios are outside the coverage runs); its evidence is the committed screenshots and the 26 passing browser scenarios.
- **Legitimately hard to test:** none claimed.

## E2E coverage

Web UI classification holds: the converter renders HTML, CSS and JavaScript in a browser, so Playwright through `web/e2e/driver.mjs` is the real driver; an HTTP client would not be valid. Two normalized actions, one `@e2e` each, no downgraded scenario, no excess scenario. The browser `@e2e` run is the browser smoke, separate from `--e2e`; both pass.

## Visual review

- Evidence complete: both ready states at 360x780 and 1920x1080, the two-row and all-day variants, the unchanged failure state and the empty state.
- States seen: ready desktop/mobile, ready-with-warnings one-row desktop/mobile, ready-with-warnings two-row desktop, all-day-only desktop, failure desktop, empty desktop.
- `crew-member--personal-calendar-owner`: state recognition, singular/plural count sentence, unknown-duty readability, both offer labels and the way back are reachable at both viewports; failure and empty states unchanged. No finding. `VISUAL-REVIEW: PASS`.
- `contributor--web-reviewer`: approved copy, heading hierarchy, real list semantics, distinct offer labels, no internal identifier, no superseded copy. One non-finding observation: the two offers share the same button treatment; the contract specifies labels and order, not visual priority. No finding. `VISUAL-REVIEW: PASS`.

VISUAL-REVIEW: PASS

## Overlap dispositions

| Finding pair | Decision |
|---|---|
| "The page lists an all-day unknown duty with its date and code" / "The page shows the warnings and offers both files when a plan converts with warnings" | variant — the all-day line form (`start: null`) is the distinct invariant; the two-row order is asserted separately |
| "The page states how many rows were converted with warnings" / "The page shows the warnings and offers both files when a plan converts with warnings" | variant — the plural count and the two-line plan order are distinct from the single-row state |
| "The page keeps the calendar and the warnings when it cannot offer the warning report" / "The page shows the warnings and offers both files when a plan converts with warnings" | variant — the failed-offer path is a distinct end state |

## Split-or-keep

No scenario exceeds the gate's long-scenario threshold; nothing to split.

## README impact

README-IMPACT: none

The README describes the executable's inputs, the published calendar beside the source and the warning report; it does not enumerate calendar titles or the browser ready-state copy, so no named section is stale.

## Sign-off

- Fabrication audit clean; every checked task has a real revision naming its scenario; the permanent multi-row assertion was corrected and committed.
- Every promised component exists; strict-mode proof present.
- `./run-tests.sh` 140/140, `./run-tests.sh --e2e` 2/2, `./run-tests.sh --browser` 26/26, `clippy -D warnings` and `fmt --check` pass.
- Coverage classified from inspection; no dead code, no missing scenario, no hard-to-test claim.
- No `@wip` tags remain; no implementation detail in the delta spec.
- Both inventory actions have one real-interface `@e2e` each; no downgraded or excess scenario; `verify.e2e_command` is a strict subset.
- Arc42 conformance and ubiquitous language clean; both Persona vision reviews pass.
- No deferral and no invented exception classification.

PERSONAS: crew-member--personal-calendar-owner
PERSONAS: contributor--web-reviewer

REVIEW: PASS
