# Tasks: build-browser-converter

Release note (commit subject):

    feat(export-duty-plan-calendar): A crew member can convert one supported duty plan to a complete calendar in a current browser from the built Site output, with the plan never leaving the tab, or receive an explicit failure with an offered diagnostic. The public publication of that page is the separate publish-public-website change; this change delivers the browser capability and its evidence, not the public URL.

Owning use case: [Export a duty-plan calendar](../../specs/export-duty-plan-calendar/usecase.md)
(`export-duty-plan-calendar`). Confirmed Personas: `crew-member--personal-calendar-owner`
(accuracy/privacy lens), `project-maintainer--release-owner` (release-evidence lens),
`contributor--web-reviewer` (CSP/no-network/fallback lens). Actors in scenarios stay
distinct from Personas.

Single-scenario run command (used by every RED/GREEN/REFACTOR below):
`./run-tests.sh --name "<scenario title>"`.

Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`.
Coverage: `./measure-coverage.sh && ./merge-coverages.sh` (after GREEN, not per scenario).

## Rules that apply to every task in this file

- **`@e2e` tags are immutable.** The `@e2e` tag on `Crew member converts a supported
  duty plan in the browser and saves the offered calendar` is never removed — not during
  RED, not to bypass a gate, not when e2e infrastructure is blocked. If the e2e run is
  blocked, fix `verify.e2e_command`; removing the tag is a procedure violation that
  `givn lint --change build-browser-converter` and the review `verify-e2e` hook detect.
- **No step body is ever empty.** Unimplemented steps use the cucumber-rs stub
  `unimplemented!("<step name>")`. A bare `{}` or a no-op body is indistinguishable from
  PASS under `.fail_on_skipped()` and is banned in every phase.
- **Strict mode is `apps/cli/tests/features_runner.rs`'s `.fail_on_skipped()` plus
  `.run_and_exit(root)`.** An undefined or skipped step makes the process non-zero; a
  panicking stub makes it non-zero with exit 101.
- **One commit per scenario** (`RED+GREEN+REFACTOR`), created with
  `givn commit -m "<subject>\n\nScenario: <title>\n\n<phase summary>"` (a raw Git
  commit is stamped by the hook; a raw Fossil check-in is not). The subject is the
  release note above (for the `@e2e` scenario the subject is
  `test(e2e): <release note>`); the body names the scenario exactly; the revision id is
  recorded in the COMMIT task.
- **Tick each checkbox immediately when the phase completes**, never in a batch.
- **Never infer RED/GREEN from whole-suite output.** Every phase runs the targeted
  single-scenario command and pastes that transcript.

## Evidence rules (canonical for this change)

- `@e2e` visual evidence: after the final assertion, `web/e2e/driver.mjs` resizes to
  **360x780 (mobile)** and **1920x1080 (desktop)**, emulates `prefers-reduced-motion:
  reduce` and a light color scheme, waits for `document.fonts.ready` and two animation
  frames, and calls `page.screenshot({ animations: 'disabled' })`.
- Committed evidence paths:
  `givn/changes/build-browser-converter/evidence/visual/crew-member-converts-a-supported-duty-plan-in-the-browser-and-saves-the-offered-calendar/<viewport>.png`
  for the **ready** state and `<viewport>--empty.png` for the **empty** state. Exactly
  the two declared states the scenario traverses are captured. Intermediate captures go
  to `reports/visual/` and are never review evidence. The failure state's approved
  reference is the design-only render at
  `docs/design/reference/converter-reference.html`.
- Every `@e2e` display assertion also runs the visual baseline: visible label on every
  control; no internal identifier or state name in visible text (denylist `ok`,
  `failure`, `status`, `WASM`, `blob:`, bare `parse-error`, component names — offered
  filenames are the intended exception); a real title and `h1`; no duplicated visible
  text block; failure heading plus `Choose another plan`; `lang="en"`; visible keyboard
  focus; reduced-motion computed value `0ms`; `@axe-core/playwright` clean after each
  rendered state.
- Calendar download capture: `save_offer` clicks the real `<a download>` link and waits
  for `page.waitForEvent('download')`, saving the captured download to the scenario
  directory. The step compares the captured file with
  `apps/cli/tests/fixtures/expected/2026-06.ics`: one `VEVENT` per expected activity
  with the fixture's `SUMMARY`, `DTSTART` and `DTEND`, CRLF line endings, the expected
  `UID` values, the fixture's `DTSTAMP`, and no extra or partial event. A
  `BEGIN:VCALENDAR`/`END:VCALENDAR` stub is not evidence.
- Diagnostic capture: the same `save_offer` path captures `<stem>.parse-error.txt`; the
  Rust step reads that file and matches `Source PDF: <filename>\nReason: <reason>\n`.
  `read_state` never returns Blob contents (`connect-src 'none'` forbids fetching it); no
  page-side test hook is added.
- `read_state` returns the visible state (`empty`/`converting`/`ready`/`failure`),
  visible text, offers, visible failure reason, storage snapshot, address/history and the
  request log. The request log must show no other-origin request after load and no
  `.wasm` request (the WASM bytes are inlined).
- Fixtures are generated at test time with `apps/cli/tests/steps/support.rs`
  (`build_pdf`, `at`, `detail_header_row`); no real plan and no real identifying value
  is committed. The committed calendar fixture contains generated/anonymized values only.

## Interaction coverage pre-flight (checked against `usecase.md` §Interactions)

| Inventory entry (`givn/specs/export-duty-plan-calendar/usecase.md`) | Matrix row (`design.md` §Interaction Coverage Matrix) | Existing `@e2e` scenario | Driver |
|---|---|---|---|
| supply a duty plan for conversion through the executable | present — CLI (terminal) | `Crew member converts a supported duty plan and receives a calendar file` (permanent feature) | real `std::process::Command` on the built `duty2cal` binary (unchanged) |
| supply a duty plan through the converter page and save the offered calendar | present — Web UI | `Crew member converts a supported duty plan in the browser and saves the offered calendar` (delta feature) | Playwright 1.64.1 → `web/e2e/driver.mjs` → Chromium, loopback-served `web/` |

Result: **no gap.** Two normalized inventory entries, two matrix rows, and exactly one
existing `@e2e` scenario per entry. Dropping a plan is a variant of the same supply
action and the equivalence scenario is a distinct invariant of the same action, so
neither adds an inventory entry or an `@e2e` scenario. This plan invents no `@e2e`
scenario.

## Setup findings recorded before writing tasks

1. **Interaction coverage:** no gap (table above).
2. **Baseline under `@wip`:** all 16 delta scenarios still carry `@wip`, so
   `./run-tests.sh` is RED by construction until the last section removes its tag (this
   matches every prior change). The implemented baseline proven in Setup is
   `./run-tests.sh --tags "not @wip"` → exit 0. This is a phase-state fact, not a design
   change.
3. **`design.md` step-location omission:** the regular-browser step list names the size,
   family, unresolved-activity, stale-offer, drop, capability, startup, offer-failure,
   cancelled-save, stem and persistence scenarios but omits the equivalence scenario by
   name. The design's own rule is one step file per capability with the regular browser
   variants in `apps/cli/tests/steps/export_duty_plan_calendar.rs`, so this plan puts the
   equivalence steps there. Recorded as a documentation gap; no design decision changes.
4. **`measure-coverage.sh` does not match D3 yet:** it currently runs
   `cargo llvm-cov --workspace`; the design's D3 decision requires
   `--package duty2cal-core --package duty2cal-pdf --package duty2cal` and one
   `scripts/build-web.sh` call. Setup fixes it.
5. **`run-tests.sh` does not build the web bundle yet:** both suite modes must call
   `scripts/build-web.sh` unless `DUTY2CAL_SKIP_WEB_BUILD=1`; Setup fixes it.
6. **Toolchain is absent:** `wasm-pack` is not installed, `web/e2e/node_modules` and the
   Playwright Chromium build do not exist, and `web/pkg` is not built. Setup installs
   and pins them; the first milestone is a real-Chromium `init(bytes)` proof under the
   real CSP (design risk R-036).

## Setup task — toolchain, build, runner, strict mode, step skeletons

- [x] Resolve and pin the browser-test toolchain once (design D6 / Version freshness):
  - Resolve the latest Node LTS once (`nvm install --lts && node --version`), record the
    resolved major in `web/e2e/.nvmrc`, and use `node-version-file: web/e2e/.nvmrc` in
    `.github/workflows/ci.yml`. Do not leave a floating `lts/*`.
  - Create `web/e2e/package.json` with `playwright@1.64.1` and
    `@axe-core/playwright@4.14.0` (test-only), run `npm install --prefix web/e2e`, and
    commit `web/e2e/package-lock.json`.
  - Install the driver and browser:
    ```
    npm ci --prefix web/e2e
    npx --prefix web/e2e playwright install --with-deps chromium
    ```
  - Record the resolved `wasm-pack` version in `web/.wasm-pack-version` (design pins
    0.15.0; the setup task confirms and records the resolved value).
  - Evidence: the host resolves the latest LTS as Node v24.21.0; `web/e2e/.nvmrc`
    records `24`, and both CI jobs read it with `node-version-file`. `npm install`
    installed Playwright 1.64.1 and `@axe-core/playwright` 4.14.0 and wrote the
    committed `web/e2e/package-lock.json`. `wasm-pack --version` resolves to 0.15.0,
    recorded in `web/.wasm-pack-version`; `scripts/build-web.sh` installs exactly that
    version when the host has none or a different one.
- [x] Add the WASM member and build path exactly as design.md names it:
  - `Cargo.toml`: add `ffi/wasm` as a workspace member.
  - `ffi/wasm/Cargo.toml`: package `duty2cal-wasm`, `crate-type = ["cdylib", "rlib"]`,
    dependencies `duty2cal-core`, `duty2cal-pdf`, `time`; `wasm-bindgen`, `js-sys` and
    `getrandom = { version = "0.4", features = ["wasm_js"] }` under
    `[target.'cfg(target_arch = "wasm32")'.dependencies]`.
  - `ffi/wasm/src/lib.rs`: the facade skeleton with the `#[cfg(target_arch = "wasm32")]`
    `convert(filename, bytes)` / `diagnostic_text(...)` exports and panicking bodies
    where conversion is not yet wired, plus `BrowserClock` (wasm) and `NativeClock`
    (native) so `cargo clippy --workspace --all-targets` compiles the crate on Linux.
  - `ffi/wasm/clippy.toml`: the core's `disallowed-types`/`disallowed-methods`
    (`std::fs`, `std::net`, `std::time::SystemTime`, `std::env`, `std::process::exit`).
  - `scripts/build-web.sh`: install exactly the pinned `wasm-pack`, run
    `wasm-pack build ffi/wasm --release --target web --out-dir ../../web/pkg`, generate
    `web/pkg/wasm-inline.js` (one exported base64 string of `duty2cal_wasm_bg.wasm`), and
    copy `LICENSES/MPL-2.0.txt` to `web/LICENSE.txt`. It never weakens the CSP and never
    fetches the `.wasm` file.
  - Regenerate `THIRD-PARTY-NOTICES.md` with the pinned `cargo-about@0.9.0` after the
    member joins, so the `notices` CI job's `git diff --exit-code` passes.
  - `.gitignore`: `/web/pkg/`, `web/e2e/node_modules/`, `web/e2e/test-results/`,
    `reports/visual/`.
  - Evidence: `cargo check --workspace --all-targets` compiles the new member natively,
    and `scripts/build-web.sh` produced `web/pkg` for `wasm32-unknown-unknown` with
    `wasm-pack 0.15.0`; `web/pkg/wasm-inline.js` is the single base64 export. Rust 1.99
    emits bulk-memory operations, which the binaryen bundled with wasm-pack 0.15.0
    rejects, so `ffi/wasm/Cargo.toml` sets
    `wasm-opt = ["-O", "--enable-bulk-memory"]`; the CSP is unchanged. `Cargo.lock`
    records `wasm-bindgen 0.2.129`, `js-sys 0.3.106` and `getrandom 0.4.3`, and
    `THIRD-PARTY-NOTICES.md` was regenerated with `cargo-about 0.9.0`. `.gitignore`
    also ignores the build-generated `/web/LICENSE.txt` so the copied licence is not
    committed twice.
- [x] Wire the runner and the two suite modes:
  - `run-tests.sh`: call `scripts/build-web.sh` before the runner unless
    `DUTY2CAL_SKIP_WEB_BUILD=1`; keep `--e2e` (`--tags "@e2e"`), `--name` and `--tags`.
  - `measure-coverage.sh`: call `scripts/build-web.sh` once, then run
    `cargo llvm-cov --package duty2cal-core --package duty2cal-pdf --package duty2cal
    --test features_runner` for the non-e2e (`--tags "not @e2e"`) and e2e
    (`--tags "@e2e"`) reports (design D3 coverage exclusion).
  - `givn/commands.yaml`: set/confirm `verify.command: "./run-tests.sh"` and
    `verify.e2e_command: "./run-tests.sh --e2e"`. They are distinct; the e2e command is a
    strict subset (proved in the E2E setup task). Never point both at the same command.
  - `.github/workflows/ci.yml`: the `test` and `coverage` jobs install Node from
    `web/e2e/.nvmrc` and Chromium; `fmt`, `clippy`, `deny`, `notices`, `reuse`,
    `license-headers`, `pii-guard` and `windows-gnu-build` see the new member and files.
    No publish workflow is added (the sibling `publish-public-website` change owns it).
  - Evidence: `./run-tests.sh --tags "not @wip"` ran the built bundle and the suite to
    that exit (transcript below). `measure-coverage.sh` now builds once and passes the
    three D3 packages; `givn/commands.yaml` keeps the two distinct commands it already
    had (`./run-tests.sh` and `./run-tests.sh --e2e`). CI's `test` and `coverage` jobs
    install the pinned Node, npm dependencies and Chromium, and request the wasm target.
- [x] Create the step-definition skeletons, **one file per capability layer** exactly as
  design.md's "Step definition locations" names:
  - `apps/cli/tests/steps/browser.rs` — the shared `BrowserDriver` client: spawns
    `web/e2e/driver.mjs`, one JSON command per line, one JSON result per line, fails
    closed on an unknown command or a non-`ok` result, owns the child and kills it on
    drop. This is support code, not a step-definition file.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the regular browser variant
    steps, alongside the existing CLI steps (including the equivalence steps, see Setup
    finding 3).
  - `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` — the browser `@e2e` steps,
    separate from the regular file.
  - `apps/cli/tests/steps/mod.rs` — add `pub mod browser;`.
  - `apps/cli/tests/features_runner.rs` — add `browser: Option<BrowserDriver>` to
    `DutyWorld` and kill the child in `Drop`.
  - Every new step body starts as `unimplemented!("<step name>")`; the driver has no
    path that reports an unperformed action as success (`{ "ok": false, "error": ... }`
    on an unknown action or missing observation).
  - Evidence: `apps/cli/tests/steps/browser.rs` owns the `node web/e2e/driver.mjs` child,
    rejects an action outside the ten-action protocol before writing, parses each
    result line with `serde_json`, and panics on a missing line or any non-`ok` result;
    `Drop` kills and reaps the child. `steps/mod.rs` registers the module,
    `DutyWorld.browser` holds it and `Drop` releases it. All 52 distinct delta step
    texts are defined across the two capability files (42 stubs in the regular file,
    7 `@e2e`-only stubs in the e2e file, and the parameterised patterns that serve the
    remaining texts); every new body is `unimplemented!` naming the step.
- [x] Prove strict mode with a deliberately undefined step, using the exact runner
  configuration:
  1. Add a temporary feature `givn/changes/build-browser-converter/specs/export-duty-plan-calendar/strict-mode-probe.feature`:
     ```
     @givn.delta @export-duty-plan-calendar
     Feature: Strict-mode probe

       Scenario: An undefined step fails the run
         Given a step that has no definition
     ```
  2. Run the targeted probe and paste the non-zero output:
     ```
     $ ./run-tests.sh --name "An undefined step fails the run"
     Feature: Strict-mode probe
       Scenario: An undefined step fails the run
        ✘  Given a step that has no definition
           Step failed:
           Defined: tmp/tmp.WyaIdxbhvy/f0007.feature:5:5
           Step doesn't match any function
     [Summary]
     1 feature
     1 scenario (1 failed)
     1 step (1 failed)
     RUNNER_EXIT=101
     ```
  3. Confirm the exit is non-zero. If it is 0, stop: the runner is not strict and GREEN
     would be meaningless.
  4. Remove the probe feature and return to the passing implemented baseline:
     ```
     $ ./run-tests.sh --tags "not @wip"
     [Summary]
     5 features
     150 scenarios (150 passed)
     648 steps (648 passed)
     BASELINE_EXIT=0
     ```
     The unfiltered `./run-tests.sh` stays RED only because the 16 delta scenarios still
     carry `@wip`; that is the expected phase state, not a skipped check.
- [x] Prove the WASM bundle instantiates in a real browser under the real CSP before any
  page or scenario work (design risk R-036 / chapter 11):
  - Build `web/pkg` with `scripts/build-web.sh`.
  - Use a throwaway probe page carrying the exact CSP
    `default-src 'self'; connect-src 'none'; script-src 'self' 'wasm-unsafe-eval';
    style-src 'self'; img-src 'self' data:; object-src 'none'; base-uri 'none';
    form-action 'none'`, importing `./pkg/duty2cal_wasm.js` and `./pkg/wasm-inline.js`,
    decoding the base64 into a `Uint8Array` and calling `init(bytes)`.
  - Serve it on loopback and drive it with Playwright/Chromium; assert
    `WebAssembly.instantiate` succeeded, no `.wasm` request was made and every request
    origin is the page origin. Paste the output, e.g.:
    ```
    console: warning: using deprecated parameters for the initialization function; pass a single object instead
    probe: init(bytes) = ok
    probe: requests = ["http://127.0.0.1:40517/probe.html","http://127.0.0.1:40517/probe.js","http://127.0.0.1:40517/pkg/duty2cal_wasm.js","http://127.0.0.1:40517/pkg/wasm-inline.js"]
    probe: .wasm requests = none
    probe: other-origin requests = none
    PROBE_EXIT=0
    ```
  - Delete the probe page and probe script. If instantiation fails, fix the
    `getrandom/wasm_js` configuration or revisit the inline/CSP decision with the
    operator — never silently weaken `connect-src`.
  - Evidence: the proof ran against the NixOS Chromium 155.0.8059.79 binary through
    Playwright's `executablePath` (CI uses the Playwright-installed Chromium). The
    probe page loads an external `probe.js`: the exact CSP forbids inline scripts, as
    the real page's external module does. `init(bytes)` resolved, no `.wasm` request was
    made, and every request stayed on the loopback page origin. The pages and script
    were deleted after the run.

### Setup verification

- `cargo check --workspace --all-targets` passes natively and the wasm build passes
  through `scripts/build-web.sh`.
- `cargo fmt --all --check` exits 0, and
  `cargo clippy --workspace --all-targets -- -D warnings` reports no warning. Host
  workaround: the rustup proxies for `cargo-fmt`/`cargo-clippy` cannot execute on this
  NixOS host, so the commands ran with the NixOS-provided `rustfmt 1.98.1`/`clippy
  0.1.98` on PATH; CI runs the pinned stable equivalents.
- `givn lint --change build-browser-converter` reports exactly the 16 expected `@wip`
  findings and nothing else.

Setup commit: `30008f0a8b71d57ae7d6b13a3860ad1f21c7acf1`.

## Scenarios (non-`@e2e` first, in feature order)

Each GREEN starts with the compile step
`cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings`
(and `scripts/build-web.sh` for anything the page needs); a compile failure stops the
phase before the scenario run.

### 1. The browser offers the same calendar the executable produces for the same plan

Use-case constraint: the success guarantee is one complete RFC 5545 calendar; the
browser realization and the executable inherit the same conversion rules. The
equivalence holds only when the source supplies its revision timestamp.

**Design constraints (from design.md):**

- `ffi/wasm/src/lib.rs` exports `convert(filename: &str, bytes: &[u8]) -> JsValue`
  wiring `ConvertRoster::new(PdfRosterSource, IcsSink, BrowserClock).execute(bytes)`.
- Success is a plain JS object `{ status: "ok", calendar: "<complete RFC 5545 text>" }`;
  failure is `{ status: "failure", reason, diagnostic }`. No path crosses the WASM
  boundary and the facade performs no filesystem or network I/O.
- The source's Created footer remains the preferred revision timestamp; `BrowserClock`
  (`js_sys::Date::now()` + `OffsetDateTime::from_unix_timestamp_nanos`) is only the
  fallback. The equivalence fixture carries a Created footer, so the CLI and the browser
  derive the same `DTSTAMP`.
- The scenario runs the built CLI as a real subprocess (`std::process::Command`) for the
  same fixture, alongside the browser run.
- The page supplies `File.name` and bytes through the real `<input type="file">`; the
  driver's `set_file` action does not synthesize a fetch or an in-process call.
- `IcsSink` and `ConvertRoster` are unchanged; there is no second conversion path.
- The WASM bytes are inlined as base64 and passed to `init(bytes)`; the CSP
  `connect-src 'none'` is never weakened and no `.wasm` request is made.

- [ ] RED — remove `@wip` from this scenario only; add `unimplemented!()` stubs for its
  five steps in `apps/cli/tests/steps/export_duty_plan_calendar.rs` (helper hooks in
  `apps/cli/tests/steps/browser.rs`); run the single-scenario command; paste the
  non-zero output (expected shape):
  ```
  $ ./run-tests.sh --name "The browser offers the same calendar the executable produces for the same plan"
  Feature: Export a duty-plan calendar
    Scenario: The browser offers the same calendar the executable produces for the same plan
     ✘  Given the supported duty plan "2026-06.pdf" whose Created footer supplies its revision timestamp
        Step panicked. Captured output: not implemented: build the Created-footer equivalence fixture
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — replace the stubs with real assertions; minimum production code. Production
  files created/modified:
  - `ffi/wasm/src/lib.rs` — `convert` mapping, `BrowserClock`, native `NativeClock`,
    structured success/failure.
  - `web/index.html`, `web/capability.js`, `web/app.js`, `web/styles.css` — the converter
    screen, capability gate, ES-module init, `acceptFile`, conversion routing, states,
    privacy note.
  - `web/e2e/driver.mjs` — `start`, `set_file`, `read_state`, `save_offer`, `stop`.
  Test support (not production): `apps/cli/tests/steps/browser.rs` (JSON-lines client),
  `apps/cli/tests/steps/export_duty_plan_calendar.rs` (equivalence steps: browser run +
  real CLI subprocess), `apps/cli/tests/features_runner.rs` (`browser` field and `Drop`
  kill), `apps/cli/tests/steps/support.rs` (Created-footer PDF builder, e.g. a
  `Created <stamp>` footer row), and `apps/cli/tests/fixtures/expected/2026-06.ics`
  (generated from the CLI output after the byte-identity assertion passes;
  generated/anonymized values only).
  Run the single-scenario command; paste the zero-exit output (expected shape):
  ```
  $ ./run-tests.sh --name "The browser offers the same calendar the executable produces for the same plan"
  Feature: Export a duty-plan calendar
    Scenario: The browser offers the same calendar the executable produces for the same plan
     ✔  Given the supported duty plan "2026-06.pdf" whose Created footer supplies its revision timestamp
     ✔  And the converter page served from the built Site output
     ✔  And the executable has published the calendar for that plan
     ✔  When the crew member supplies that plan through the file picker
     ✔  Then the offered calendar is byte-identical to the executable's calendar
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — review the page/facade for dead weight (no framework, no bundler, no
  third-party runtime asset), keep the facade free of filesystem/network access, and
  re-run the single-scenario command; paste the zero-exit output:
  ```
  cargo fmt --all --check -> FMT OK
  cargo clippy --workspace --all-targets -- -D warnings -> clean
  $ ./run-tests.sh --name "The browser offers the same calendar the executable produces for the same plan"
  1 scenario (1 passed), 5 steps (5 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: one commit for RED+GREEN+REFACTOR; subject is the release note above
  (`feat(export-duty-plan-calendar): …`), body contains
  `Scenario: The browser offers the same calendar the executable produces for the same plan`; Hash: `<record the revision identifier here>`

### 2. A plan at the supported size bound is accepted

Use-case constraint: main-flow step 1 refuses a browser-supplied plan larger than
20 MiB before reading it; a plan at the bound is accepted.

**Design constraints (from design.md):**

- `web/app.js` owns `MAX_BYTES = 20 * 1024 * 1024`; the check is
  `file.size > MAX_BYTES` **before** `await file.arrayBuffer()`.
- The exactly-20-MiB fixture builder appends a PDF comment/padding after a valid
  generated detail table, so the bytes reach exactly 20 MiB and `PdfRosterSource` still
  recognizes the family. A builder-to-reader check runs before the size-bound scenarios
  are written; a fixture the reader rejects would prove the wrong behavior.
- Ready state: `Calendar ready` heading, `Save 2026-06.ics` offer, picker enabled,
  privacy note; the offer is an in-memory object URL plus `<a download="2026-06.ics">`.
- No numbered name is promised; the browser owns the save location and collisions.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (the shared page Given stays implemented); run:
  ```
  $ ./run-tests.sh --name "A plan at the supported size bound is accepted"
  Feature: Export a duty-plan calendar
    Scenario: A plan at the supported size bound is accepted
     ✔  Given the converter page served from the built Site output
     ✘  When the crew member supplies the supported duty plan "2026-06.pdf" of exactly 20 MiB
        Step panicked. Captured output: not implemented: build and supply the exactly-20-MiB fixture
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions and minimum production code; production files modified:
  - `web/app.js` — `MAX_BYTES`, the pre-read size guard (accept path), ready-state
    rendering and the `Save 2026-06.ics` offer.
  - Test support: `apps/cli/tests/steps/support.rs` — a `pad_to`/comment-padding helper
    that produces exactly 20 MiB; the builder-to-reader check asserts the CLI accepts the
    padded fixture.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the size-bound steps.
  Run:
  ```
  $ ./run-tests.sh --name "A plan at the supported size bound is accepted"
  Feature: Export a duty-plan calendar
    Scenario: A plan at the supported size bound is accepted
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" of exactly 20 MiB
     ✔  Then the page shows that the calendar is ready
     ✔  And the page offers the file "2026-06.ics"
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — keep the fixture padding in test support (never a committed binary),
  re-run:
  ```
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A plan at the supported size bound is accepted`; Hash: `<record the revision identifier here>`

### 3. A plan larger than the supported size bound is refused by name

Use-case constraint: an oversized plan is refused by name before it is read; no calendar
is offered; a diagnostic offer is attempted.

**Design constraints (from design.md):**

- The page rejects `file.size > 20 * 1024 * 1024` before reading, with a reason naming
  the 20 MiB bound.
- The page calls the facade's `diagnostic_text(filename, reason)` for that failure, so
  the size path uses the same two-line diagnostic as a conversion failure.
- D4: the pure `diagnostic_text(source_name: &str, reason: &str) -> String` lives in
  `duty2cal-core/src/application/mod.rs`; the CLI's `report()` and the WASM facade call
  it. The only duplicated spelling is the page's last-resort trap fallback.
- The diagnostic text is exactly `Source PDF: <filename>\nReason: <reason>\n`; the page
  offers it as `<stem>.parse-error.txt`.
- The failure panel is `role="alert"` with a `Conversion failed` heading, the reason, the
  optional diagnostic offer and `Choose another plan`; no calendar offer.
- The 25 MiB fixture is rejected before parsing.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "A plan larger than the supported size bound is refused by name"
  Feature: Export a duty-plan calendar
    Scenario: A plan larger than the supported size bound is refused by name
     ✔  Given the converter page served from the built Site output
     ✘  When the crew member supplies the plan "2026-06.pdf" of 25 MiB
        Step panicked. Captured output: not implemented: build and supply the 25-MiB fixture
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions and minimum production code; production files modified:
  - `crates/duty2cal-core/src/application/mod.rs` — `pub fn diagnostic_text(...)`.
  - `apps/cli/src/main.rs` — `report()` uses the shared function (no format change).
  - `ffi/wasm/src/lib.rs` — the `diagnostic_text` wasm export and the failure object's
    `diagnostic` field.
  - `web/app.js` — size refusal, failure-panel rendering, diagnostic offer
    (`Save 2026-06.parse-error.txt`), reset control.
  - Test support: `apps/cli/tests/steps/support.rs` — a 25 MiB padded fixture.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the size-refusal steps,
    including the captured diagnostic download.
  Run:
  ```
  $ ./run-tests.sh --name "A plan larger than the supported size bound is refused by name"
  Feature: Export a duty-plan calendar
    Scenario: A plan larger than the supported size bound is refused by name
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the plan "2026-06.pdf" of 25 MiB
     ✔  Then the page shows a failure heading and a failure naming the 20 MiB bound
     ✔  And the page offers no calendar
     ✔  And the page offers the diagnostic text "2026-06.parse-error.txt"
     ✔  And the diagnostic text identifies "2026-06.pdf" and the size failure
     ✔  And the page offers a way to choose another plan
  [Summary]
  1 feature
  1 scenario (1 passed)
  7 steps (7 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — confirm the CLI and facade share one diagnostic format (no second
  spelling), re-run:
  ```
  1 scenario (1 passed), 7 steps (7 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A plan larger than the supported size bound is refused by name`; Hash: `<record the revision identifier here>`

### 4. A source outside the supported family is rejected with a diagnostic offer

Use-case constraint: Extension 2 — a source that is not a supported duty-plan export
fails explicitly; no calendar is offered and a diagnostic is attempted.

**Design constraints (from design.md):**

- `PdfRosterSource` performs family recognition; `SourceError::UnsupportedFamily` is the
  reason. The facade maps the error to `{ status: "failure", reason, diagnostic }`.
- The page never offers a calendar for a failure, and always attempts the diagnostic
  offer with the shared text.
- The scenario supplies a readable PDF outside the family (not a size failure), so the
  family check — not the page guard — is what refuses it.
- The captured diagnostic download is read by the Rust step and matches
  `Source PDF: notes.pdf\nReason: <unsupported-family reason>\n`.
- No request leaves the page.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "A source outside the supported family is rejected with a diagnostic offer"
  Feature: Export a duty-plan calendar
    Scenario: A source outside the supported family is rejected with a diagnostic offer
     ✔  Given the converter page served from the built Site output
     ✘  When the crew member supplies the file "notes.pdf" that is not a supported duty plan
        Step panicked. Captured output: not implemented: build the unsupported-family PDF and supply it
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions; production files:
  - None created. Exercised unchanged: `crates/duty2cal-pdf/src/lib.rs`
    (`PdfRosterSource` recognition), `crates/duty2cal-core/src/application/mod.rs`
    (`SourceError::UnsupportedFamily`), `ffi/wasm/src/lib.rs` (error mapping), and the
    failure/diagnostic path in `web/app.js` completed by the previous scenario.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the family steps and the
    captured diagnostic download.
  Run:
  ```
  $ ./run-tests.sh --name "A source outside the supported family is rejected with a diagnostic offer"
  Feature: Export a duty-plan calendar
    Scenario: A source outside the supported family is rejected with a diagnostic offer
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the file "notes.pdf" that is not a supported duty plan
     ✔  Then the page shows a failure heading and a failure naming "notes.pdf"
     ✔  And the page offers no calendar
     ✔  And the page offers the diagnostic text "notes.parse-error.txt"
     ✔  And the diagnostic text identifies "notes.pdf" and the unsupported-family reason
     ✔  And the page offers a way to choose another plan
  [Summary]
  1 feature
  1 scenario (1 passed)
  7 steps (7 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — assert a fresh unsupported PDF truly reports the family reason (not a
  read error), re-run:
  ```
  1 scenario (1 passed), 7 steps (7 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A source outside the supported family is rejected with a diagnostic offer`; Hash: `<record the revision identifier here>`

### 5. An unresolved required activity is rejected with a diagnostic offer

Use-case constraint: an unresolved required activity is rejected, never omitted; the
failure is visible and a diagnostic is attempted.

**Design constraints (from design.md):**

- `ConvertRoster`/`interpret` keep the inherited rule: every required source activity is
  accounted for, and an unresolved code or timing yields `ConvertError` instead of a
  partial calendar.
- The facade returns the `ConvertError` Display as `reason` and the shared diagnostic as
  `diagnostic`; `IcsSink` is never asked to render a partial calendar.
- The page shows the failure heading and reason, offers no calendar, offers
  `2026-06.parse-error.txt`, and keeps `Choose another plan`.
- The diagnostic download is captured and read; no Blob is fetched by the page.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "An unresolved required activity is rejected with a diagnostic offer"
  Feature: Export a duty-plan calendar
    Scenario: An unresolved required activity is rejected with a diagnostic offer
     ✔  Given the converter page served from the built Site output
     ✘  When the crew member supplies the supported duty plan "2026-06.pdf" containing a required activity the converter cannot resolve
        Step panicked. Captured output: not implemented: build the unresolved-activity fixture and supply it
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions; production files:
  - None created. Exercised unchanged: `crates/duty2cal-core/src/application/mod.rs`
    (`interpret`, `ConvertError`), `crates/duty2cal-core/src/domain/` (classification),
    `ffi/wasm/src/lib.rs` (failure mapping), `web/app.js` (failure rendering).
  - Test support: `apps/cli/tests/steps/support.rs` — a generated plan carrying an
    unknown required code.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the unresolved-activity steps
    and diagnostic capture.
  Run:
  ```
  $ ./run-tests.sh --name "An unresolved required activity is rejected with a diagnostic offer"
  Feature: Export a duty-plan calendar
    Scenario: An unresolved required activity is rejected with a diagnostic offer
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" containing a required activity the converter cannot resolve
     ✔  Then the page shows a failure heading and the reason
     ✔  And the page offers no calendar
     ✔  And the page offers the diagnostic text "2026-06.parse-error.txt"
     ✔  And the diagnostic text identifies "2026-06.pdf" and the reason
     ✔  And the page offers a way to choose another plan
  [Summary]
  1 feature
  1 scenario (1 passed)
  7 steps (7 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — confirm no partial calendar can be rendered down this path, re-run:
  ```
  1 scenario (1 passed), 7 steps (7 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: An unresolved required activity is rejected with a diagnostic offer`; Hash: `<record the revision identifier here>`

### 6. A failed attempt withdraws a calendar offered by an earlier attempt

Use-case constraint: rule — each browser attempt clears the prior offer and state before
reading the new plan, so a failed attempt withdraws the calendar an earlier attempt
offered.

**Design constraints (from design.md):**

- Every `acceptFile` call begins by revoking the live calendar and diagnostic object URLs,
  removing the offer links and resetting the status region **before** it validates or
  reads the new file.
- A failed attempt therefore withdraws the earlier calendar; a second success replaces the
  offer and never leaves two calendar offers on the page.
- `URL.revokeObjectURL` is not called before the browser has had the chance to start the
  download; every live URL is revoked on `pagehide`.
- The stale offer's object URL is not retained after the second attempt.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "A failed attempt withdraws a calendar offered by an earlier attempt"
  Feature: Export a duty-plan calendar
    Scenario: A failed attempt withdraws a calendar offered by an earlier attempt
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
     ✔  Then the page shows that the calendar is ready
     ✔  And the page offers the file "2026-06.ics"
     ✔  When the crew member supplies the file "notes.pdf" that is not a supported duty plan
     ✔  Then the page shows a failure heading and a failure naming "notes.pdf"
     ✘  And the page offers no calendar and no longer offers "2026-06.ics"
        Step panicked. Captured output: not implemented: assert the earlier offer was withdrawn
  [Summary]
  1 feature
  1 scenario (1 failed)
  8 steps (7 passed, 1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions and minimum production code; production files modified:
  - `web/app.js` — clear-first `acceptFile`, offer revocation/tracking, status reset,
    single-offer invariant.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the two-attempt steps (the
    driver keeps one browser context and calls `set_file` twice).
  Run:
  ```
  $ ./run-tests.sh --name "A failed attempt withdraws a calendar offered by an earlier attempt"
  Feature: Export a duty-plan calendar
    Scenario: A failed attempt withdraws a calendar offered by an earlier attempt
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
     ✔  Then the page shows that the calendar is ready
     ✔  And the page offers the file "2026-06.ics"
     ✔  When the crew member supplies the file "notes.pdf" that is not a supported duty plan
     ✔  Then the page shows a failure heading and a failure naming "notes.pdf"
     ✔  And the page offers no calendar and no longer offers "2026-06.ics"
     ✔  And the page offers the diagnostic text "notes.parse-error.txt"
  [Summary]
  1 feature
  1 scenario (1 passed)
  8 steps (8 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — ensure no object URL leaks across attempts; re-run:
  ```
  1 scenario (1 passed), 8 steps (8 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A failed attempt withdraws a calendar offered by an earlier attempt`; Hash: `<record the revision identifier here>`

### 7. A calendar that cannot be offered is not reported as ready

Use-case constraint: Extension 6 — if the complete calendar cannot be offered, expose
failure and attempt a diagnostic; success is never reported without an offer.

**Design constraints (from design.md):**

- `web/app.js` wraps calendar offer creation (`Blob` + `URL.createObjectURL` + `<a
  download>`) in `try/catch`. On failure it does **not** enter the ready state: it shows
  the failure panel with the reason `the calendar could not be offered for saving`, offers
  no calendar, and attempts the diagnostic offer with the same reason.
- The driver injects an init script that throws only for a `text/calendar` object URL,
  after the page loads; the `text/plain` diagnostic path is unaffected.
- The failure panel keeps its heading, the reason and `Choose another plan`.
- The diagnostic offer is `2026-06.parse-error.txt`; the step captures and reads it.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "A calendar that cannot be offered is not reported as ready"
  Feature: Export a duty-plan calendar
    Scenario: A calendar that cannot be offered is not reported as ready
     ✘  Given the converter page served from the built Site output that cannot create a download offer for the calendar
        Step panicked. Captured output: not implemented: start the page with the calendar-offer failure injection
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions and minimum production code; production files modified:
  - `web/app.js` — offer-creation `try/catch`, failure reason, diagnostic fallback with the
    same reason, no ready state.
  - `web/e2e/driver.mjs` — the init-script mode (`context.addInitScript` applied before
    navigation, as design.md's `set_file` mode specifies) that throws for `text/calendar`
    object URLs.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the offer-failure steps.
  Run:
  ```
  $ ./run-tests.sh --name "A calendar that cannot be offered is not reported as ready"
  Feature: Export a duty-plan calendar
    Scenario: A calendar that cannot be offered is not reported as ready
     ✔  Given the converter page served from the built Site output that cannot create a download offer for the calendar
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
     ✔  Then the page shows a failure heading and states that the calendar could not be offered
     ✔  And the page offers no calendar
     ✔  And the page offers the diagnostic text "2026-06.parse-error.txt"
     ✔  And the page offers a way to choose another plan
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — ensure the ready heading is never rendered before the offer exists,
  re-run:
  ```
  1 scenario (1 passed), 6 steps (6 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A calendar that cannot be offered is not reported as ready`; Hash: `<record the revision identifier here>`

### 8. A diagnostic that cannot be offered does not hide the primary failure

Use-case constraint: Extension 2–6 — if the diagnostic offer cannot be created, the
primary failure remains visible and the page states that no diagnostic was offered.

**Design constraints (from design.md):**

- If `URL.createObjectURL` or `Blob` construction throws for the diagnostic,
  `web/app.js` catches it, keeps the primary failure panel and its reason, and adds
  `No diagnostic was offered.` It never replaces the primary failure with the diagnostic
  failure.
- The driver injects an init script that throws only for a `text/plain` object URL; the
  calendar offer path is unaffected (and there is no calendar on this failure path).
- The failure heading and the primary reason naming `notes.pdf` stay visible; there is
  no calendar offer and `Choose another plan` remains.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "A diagnostic that cannot be offered does not hide the primary failure"
  Feature: Export a duty-plan calendar
    Scenario: A diagnostic that cannot be offered does not hide the primary failure
     ✘  Given the converter page served from the built Site output that cannot create a download offer for the diagnostic
        Step panicked. Captured output: not implemented: start the page with the diagnostic-offer failure injection
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions and minimum production code; production files modified:
  - `web/app.js` — diagnostic-offer `try/catch`, `No diagnostic was offered.` copy, primary
    failure retention.
  - `web/e2e/driver.mjs` — the init-script mode that throws for `text/plain` object URLs.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the diagnostic-unavailable
    steps.
  Run:
  ```
  $ ./run-tests.sh --name "A diagnostic that cannot be offered does not hide the primary failure"
  Feature: Export a duty-plan calendar
    Scenario: A diagnostic that cannot be offered does not hide the primary failure
     ✔  Given the converter page served from the built Site output that cannot create a download offer for the diagnostic
     ✔  When the crew member supplies the file "notes.pdf" that is not a supported duty plan
     ✔  Then the page shows a failure heading and the primary failure naming "notes.pdf"
     ✔  And the page states that no diagnostic was offered
     ✔  And the page offers no calendar
     ✔  And the page offers a way to choose another plan
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — confirm the visible reason is the primary one (not the offer error),
  re-run:
  ```
  1 scenario (1 passed), 6 steps (6 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A diagnostic that cannot be offered does not hide the primary failure`; Hash: `<record the revision identifier here>`

### 9. A browser without WebAssembly shows the cannot-run message

Use-case constraint: the browser route requires WebAssembly and a module that
initializes; a missing capability is refused by name with no file picker and no calendar.

**Design constraints (from design.md):**

- `web/capability.js` is a classic deferred script; it checks
  `typeof WebAssembly === 'object'` and
  `typeof WebAssembly.instantiate === 'function'`, and sets
  `document.documentElement.dataset.duty2calSupported`.
- When the check fails it renders the cannot-run message and leaves the picker and drop
  zone `hidden`; `app.js` reveals them only after the capability check passes and the WASM
  module has initialized.
- The driver's init-script mode runs `context.addInitScript` before navigation to
  delete or shadow `window.WebAssembly` (with a configurable-property fallback), as
  design.md's `set_file` init-script mode specifies.
- There is no server or upload fallback and no calendar offer.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "A browser without WebAssembly shows the cannot-run message"
  Feature: Export a duty-plan calendar
    Scenario: A browser without WebAssembly shows the cannot-run message
     ✘  Given the converter page served from the built Site output in a browser without WebAssembly
        Step panicked. Captured output: not implemented: start Chromium with WebAssembly removed
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions and minimum production code; production files modified:
  - `web/capability.js` — the classic capability gate and cannot-run rendering.
  - `web/index.html` — the classic deferred script, the hidden picker/drop zone and the
    cannot-run region.
  - `web/styles.css` — the cannot-run panel.
  - `web/e2e/driver.mjs` — the init-script mode removing `window.WebAssembly`.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the no-WebAssembly steps.
  Run:
  ```
  $ ./run-tests.sh --name "A browser without WebAssembly shows the cannot-run message"
  Feature: Export a duty-plan calendar
    Scenario: A browser without WebAssembly shows the cannot-run message
     ✔  Given the converter page served from the built Site output in a browser without WebAssembly
     ✔  Then the page shows a cannot-run heading and states that WebAssembly is required
     ✔  And the page offers no file picker
     ✔  And the page offers no calendar
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — confirm the picker is never briefly visible on a failed gate, re-run:
  ```
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A browser without WebAssembly shows the cannot-run message`; Hash: `<record the revision identifier here>`

### 10. A browser without ES-module support shows the cannot-run message

Use-case constraint: the browser route requires ES modules; a missing capability is
refused by name with no file picker and no calendar.

**Design constraints (from design.md):**

- `web/capability.js` checks ES-module support via
  `HTMLScriptElement.supports?.('module')` with a `noModule` fallback when `supports` is
  absent, and takes the ES branch when the check fails.
- The same cannot-run treatment applies: message, no picker, no calendar.
- The driver's init-script mode shadows `HTMLScriptElement.supports` before navigation so
  the capability gate takes the ES branch.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "A browser without ES-module support shows the cannot-run message"
  Feature: Export a duty-plan calendar
    Scenario: A browser without ES-module support shows the cannot-run message
     ✘  Given the converter page served from the built Site output in a browser without ES-module support
        Step panicked. Captured output: not implemented: start Chromium with ES-module support shadowed
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions and minimum production code; production files modified:
  - `web/capability.js` — the ES-module branch and the `noModule` fallback.
  - `web/e2e/driver.mjs` — the init-script mode shadowing
    `HTMLScriptElement.supports`.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the ES-module steps.
  Run:
  ```
  $ ./run-tests.sh --name "A browser without ES-module support shows the cannot-run message"
  Feature: Export a duty-plan calendar
    Scenario: A browser without ES-module support shows the cannot-run message
     ✔  Given the converter page served from the built Site output in a browser without ES-module support
     ✔  Then the page shows a cannot-run heading and states that ES modules are required
     ✔  And the page offers no file picker
     ✔  And the page offers no calendar
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — verify both the `supports` and `noModule` branches are asserted
  (separate cases if the shadowing cannot exercise both in one), re-run:
  ```
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A browser without ES-module support shows the cannot-run message`; Hash: `<record the revision identifier here>`

### 11. A converter module that cannot be instantiated shows the startup failure

Use-case constraint: precondition — the converter module initializes; a startup failure
never falls back to a server or an upload.

**Design constraints (from design.md):**

- `web/app.js` imports `./pkg/duty2cal_wasm.js` and `./pkg/wasm-inline.js`, decodes the
  base64 into a `Uint8Array` and calls the generated `init(bytes)`. The loader accepts an
  `ArrayBuffer`/`Uint8Array` and never calls `fetch`, which is required by
  `connect-src 'none'`.
- A module-load or instantiation failure shows a startup failure with the cannot-run
  treatment and no picker; it never falls back to a server or an upload.
- The CSP is `script-src 'self' 'wasm-unsafe-eval'`; the page is loaded with the real CSP
  and the E2E fails if instantiation is blocked (no test-only CSP override).
- The driver's init-script mode replaces the module import/`init` entry with a rejecting
  function after the capability gate.
- The request log shows no other-origin request.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "A converter module that cannot be instantiated shows the startup failure"
  Feature: Export a duty-plan calendar
    Scenario: A converter module that cannot be instantiated shows the startup failure
     ✘  Given the converter page served from the built Site output whose converter module cannot be instantiated
        Step panicked. Captured output: not implemented: start the page with a rejecting module init
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions and minimum production code; production files modified:
  - `web/app.js` — the import/`init(bytes)` `try/catch` and the startup-failure state.
  - `web/e2e/driver.mjs` — the init-script mode replacing the module entry with a
    rejecting function.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the startup-failure steps and
    the request-log assertion.
  Run:
  ```
  $ ./run-tests.sh --name "A converter module that cannot be instantiated shows the startup failure"
  Feature: Export a duty-plan calendar
    Scenario: A converter module that cannot be instantiated shows the startup failure
     ✔  Given the converter page served from the built Site output whose converter module cannot be instantiated
     ✔  Then the page shows a cannot-run heading and states that the converter could not start
     ✔  And the page offers no file picker
     ✔  And the page offers no calendar
     ✔  And the page has sent no request to another origin since it loaded
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — confirm the startup failure is the cannot-run treatment (not the generic
  conversion failure), re-run:
  ```
  1 scenario (1 passed), 5 steps (5 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A converter module that cannot be instantiated shows the startup failure`; Hash: `<record the revision identifier here>`

### 12. A supported plan dropped onto the drop zone is converted

Use-case constraint: the drop zone is the same supply action as the picker; it accepts one
plan and produces the same offer.

**Design constraints (from design.md):**

- The drop zone accepts one file and prevents the browser's default navigation; both the
  picker and the drop handler call one `acceptFile(file)` function.
- Drop is a thin alias and a variant of the same supply action, not a second end-to-end
  action and not a second inventory entry; the picker remains the single `@e2e` action.
- The ready state and the `Save 2026-06.ics` offer are identical to the picker path.
- The driver's `drop_file` action drives the real drag-and-drop path.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "A supported plan dropped onto the drop zone is converted"
  Feature: Export a duty-plan calendar
    Scenario: A supported plan dropped onto the drop zone is converted
     ✔  Given the converter page served from the built Site output
     ✘  When the crew member drops the supported duty plan "2026-06.pdf" onto the drop zone
        Step panicked. Captured output: not implemented: drive the real drop action
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions and minimum production code; production files modified:
  - `web/index.html` — the drop-zone markup and its visible label.
  - `web/app.js` — the `drop` handler, default-navigation prevention, and the shared
    `acceptFile(file)` call.
  - `web/e2e/driver.mjs` — the `drop_file` action.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the drop steps.
  Run:
  ```
  $ ./run-tests.sh --name "A supported plan dropped onto the drop zone is converted"
  Feature: Export a duty-plan calendar
    Scenario: A supported plan dropped onto the drop zone is converted
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member drops the supported duty plan "2026-06.pdf" onto the drop zone
     ✔  Then the page shows that the drop was accepted and the calendar is ready
     ✔  And the page offers the file "2026-06.ics"
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — confirm the drop path adds no second conversion implementation, re-run:
  ```
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A supported plan dropped onto the drop zone is converted`; Hash: `<record the revision identifier here>`

### 13. A cancelled save is not reported as a save

Use-case constraint: the page writes nothing, never claims a save it did not observe, and
the browser owns the save; a dismissed save leaves the offer visible and the page
unchanged.

**Design constraints (from design.md):**

- The page never renders the word `saved` as a claim about its own work; it offers and the
  browser saves.
- A dismissed save leaves the offer visible and the page unchanged.
- A page cannot observe the save, so the step asserts only what the page can know: the
  offer remains and no save claim appears. The driver either does not click the offer or
  calls the captured download's cancel.
- The page uses no File System Access API, `showSaveFilePicker`, or directory access.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "A cancelled save is not reported as a save"
  Feature: Export a duty-plan calendar
    Scenario: A cancelled save is not reported as a save
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
     ✘  And the crew member dismisses the save of the offered calendar
        Step panicked. Captured output: not implemented: dismiss the captured download
  [Summary]
  1 feature
  1 scenario (1 failed)
  3 steps (2 passed, 1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions; production files:
  - `web/index.html` / `web/app.js` — offer copy that names the file and states the browser
    chooses the location; no save claim, no write API.
  - `web/e2e/driver.mjs` — the `dismiss_save` action.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the cancelled-save steps.
  Run:
  ```
  $ ./run-tests.sh --name "A cancelled save is not reported as a save"
  Feature: Export a duty-plan calendar
    Scenario: A cancelled save is not reported as a save
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
     ✔  And the crew member dismisses the save of the offered calendar
     ✔  Then the page still offers the calendar
     ✔  And the page has not reported that it saved a file
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — grep the page's visible copy for a save claim, re-run:
  ```
  1 scenario (1 passed), 5 steps (5 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A cancelled save is not reported as a save`; Hash: `<record the revision identifier here>`

### 14. The offered names use the source file's stem

Use-case constraint: rule — the page offers `<source-stem>.ics` and
`<source-stem>.parse-error.txt`; the page derives the names and never returns a path.

**Design constraints (from design.md):**

- The page derives the stem: strip the final extension only when a dot is present after
  the first character; `2026-06.pdf` becomes `2026-06`, `roster.v2.pdf` becomes
  `roster.v2`.
- The calendar name is `<stem>.ics`; the diagnostic name is `<stem>.parse-error.txt`.
- The facade never returns a path and never chooses a save location.
- This scenario uses an unsupported source so the diagnostic name is asserted without a
  conversion; no calendar is offered.

- [ ] RED — remove `@wip` from this scenario only. The page's stem rule for a dotted
  filename is not implemented yet, so the scenario's name assertion is the failing test
  (the shared Given/When steps already exist). Run the single-scenario command; paste the
  non-zero output:
  ```
  $ ./run-tests.sh --name "The offered names use the source file's stem"
  Feature: Export a duty-plan calendar
    Scenario: The offered names use the source file's stem
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the file "roster.v2.pdf" that is not a supported duty plan
     ✘  Then the page offers the diagnostic text "roster.v2.parse-error.txt"
        Step panicked. Captured output: the offered diagnostic name was "roster.parse-error.txt", expected "roster.v2.parse-error.txt"
  [Summary]
  1 feature
  1 scenario (1 failed)
  3 steps (2 passed, 1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions and minimum production code; production files modified:
  - `web/app.js` — the stem rule and the two derived names.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the stem steps.
  Run:
  ```
  $ ./run-tests.sh --name "The offered names use the source file's stem"
  Feature: Export a duty-plan calendar
    Scenario: The offered names use the source file's stem
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the file "roster.v2.pdf" that is not a supported duty plan
     ✔  Then the page offers the diagnostic text "roster.v2.parse-error.txt"
     ✔  And the page offers no calendar
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — cover the edge cases in the helper (`2026-06.pdf`, `roster.v2.pdf`,
  a leading-dot name) without adding an unasserted path, re-run:
  ```
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: The offered names use the source file's stem`; Hash: `<record the revision identifier here>`

### 15. A browser conversion leaves no plan in persistent storage or a navigable address

Use-case constraint: rule — the page sends no request to another origin after load, loads
no third-party asset, and stores no plan or derived data in persistent browser storage or
a navigable URL/history entry; an ephemeral object URL is allowed and revoked on unload.

**Design constraints (from design.md):**

- The scenario asserts the ready state and the calendar offer **first**, then the
  address/history/storage assertions, so it cannot pass without a successful conversion.
- The page never uses `localStorage`, `sessionStorage`, IndexedDB, cookies,
  `document.cookie`, `history.pushState`, `location.hash`, or a form submission.
- The plan bytes live in a local variable for the duration of the attempt; the derived
  calendar/diagnostic lives in the object URL until it is revoked; every live URL is
  revoked on `pagehide`.
- The page's CSP is the exact meta tag in design.md: `connect-src 'none'` forbids fetch,
  XHR, WebSocket, EventSource and `sendBeacon`; `script-src 'self' 'wasm-unsafe-eval'` is
  required for `WebAssembly.instantiate`; no third-party asset; inline empty favicon.
- `read_state` returns the storage snapshot, address/history and request log; `reload`
  returns the page to the empty state.

- [ ] RED — remove `@wip` from this scenario only; add its new step bodies as
  `unimplemented!()` stubs (previously implemented shared steps stay implemented); run:
  ```
  $ ./run-tests.sh --name "A browser conversion leaves no plan in persistent storage or a navigable address"
  Feature: Export a duty-plan calendar
    Scenario: A browser conversion leaves no plan in persistent storage or a navigable address
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
     ✔  Then the page shows that the calendar is ready
     ✔  And the page offers the file "2026-06.ics"
     ✘  And the page's address is unchanged and the browser history has no new entry
        Step panicked. Captured output: not implemented: read the address/history snapshot
  [Summary]
  1 feature
  1 scenario (1 failed)
  5 steps (4 passed, 1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — real assertions and minimum production code; production files modified:
  - `web/index.html` — the exact CSP meta in the head before any resource, `lang="en"`,
    the inline empty favicon.
  - `web/app.js` — no storage/history APIs, object-URL revocation on `pagehide`, plan
    bytes never persisted.
  - `web/e2e/driver.mjs` — the `read_state` storage/address/history snapshot and
    `reload`.
  - `apps/cli/tests/steps/export_duty_plan_calendar.rs` — the persistence steps.
  Run:
  ```
  $ ./run-tests.sh --name "A browser conversion leaves no plan in persistent storage or a navigable address"
  Feature: Export a duty-plan calendar
    Scenario: A browser conversion leaves no plan in persistent storage or a navigable address
     ✔  Given the converter page served from the built Site output
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
     ✔  Then the page shows that the calendar is ready
     ✔  And the page offers the file "2026-06.ics"
     ✔  And the page's address is unchanged and the browser history has no new entry
     ✔  And the page has stored nothing in persistent browser storage
     ✔  When the crew member reloads the page
     ✔  Then the page shows the empty state and offers no calendar
  [Summary]
  1 feature
  1 scenario (1 passed)
  8 steps (8 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — confirm no `.wasm` request and no other-origin request in the log,
  re-run:
  ```
  1 scenario (1 passed), 8 steps (8 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: subject `feat(export-duty-plan-calendar): <release note>`; body
  `Scenario: A browser conversion leaves no plan in persistent storage or a navigable address`; Hash: `<record the revision identifier here>`

## E2E setup task — local environment, browser driver, e2e skeleton, subset proof

- [ ] Bring up the local environment from design.md's "Local runnability & digital twins":
  run `./scripts/run-web.sh` (it builds `web/pkg` if needed and serves `web/` at
  `http://127.0.0.1:8000/`). Confirm it starts cleanly, then check reachability:
  ```
  $ ./scripts/run-web.sh &
  $ curl -fsS http://127.0.0.1:8000/ >/dev/null && echo "web reachable"
  web reachable
  ```
  There is no application server, database, queue, container, credential or external
  service; the page has no digital twin because it has no runtime external dependency.
  Stop the manual server before running the driver (the driver starts its own loopback
  server on `127.0.0.1:0`).
- [ ] Install the browser driver and browser exactly as design.md names (re-run is
  idempotent; Node is the pinned `web/e2e/.nvmrc`):
  ```
  npm ci --prefix web/e2e
  npx --prefix web/e2e playwright install --with-deps chromium
  ```
- [ ] Confirm the e2e step skeleton is in the exact design location and separate from the
  regular file: `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` (the browser
  `@e2e` steps, alongside the existing CLI e2e steps), using the shared
  `apps/cli/tests/steps/browser.rs`. Every new body is
  `unimplemented!("<step name>")`.
- [ ] Prove strict mode for the e2e runner with a deliberately undefined `@e2e` step:
  1. Add a temporary `@e2e` probe scenario under
     `givn/changes/build-browser-converter/specs/export-duty-plan-calendar/`.
  2. Run `./run-tests.sh --e2e` and paste the non-zero output:
     ```
     $ ./run-tests.sh --e2e
     Feature: Strict-mode probe
       Scenario: An undefined e2e step fails the run
        ✘  Given an e2e step that has no definition
           Step doesn't match any function
     [Summary]
     <features> features
     3 scenarios (<passed> passed, <failed> failed)
     E2E_PROBE_EXIT=101
     ```
     cucumber-rs 0.23 refuses `--tags` together with `--name`, so `--e2e` (which supplies
     `--tags "@e2e"`) cannot be combined with a name filter; the probe is selected by the
     `@e2e` filter instead. This is a framework constraint, recorded, not a workaround:
     the tag is untouched and the runner still exits non-zero.
  3. Remove the probe scenario.
- [ ] Prove `verify.e2e_command` is a strict subset, never equal to `verify.command`:
  - Run the full suite and record the count:
    ```
    $ ./run-tests.sh
    [Summary]
    <features> features
    <full_total> scenarios (<passed> passed, <failed> failed)
    <steps> steps (<passed> passed, <failed> failed)
    FULL_EXIT=<non-zero while @wip scenarios remain; record the count>
    ```
  - Run the e2e suite and record its count:
    ```
    $ ./run-tests.sh --e2e
    [Summary]
    <features> features
    2 scenarios (<passed> passed, <failed> failed)
    <steps> steps (<passed> passed, <failed> failed)
    E2E_EXIT=<non-zero while the new @e2e scenario is @wip; record the count>
    ```
  - The e2e count is **2** (the existing CLI `@e2e` scenario plus the new browser
    `@e2e` scenario) and is strictly less than `<full_total>`. Neither command is equal
    to the other. Paste both counts into this task. If `e2e_total == full_total`, stop:
    the filter is not real and the e2e gate is meaningless.
- [ ] Confirm the e2e driver can reach the running local environment before writing any
  scenario steps: exercise the driver's `start` action, which serves `web/` on an
  ephemeral loopback port, launches Chromium, opens the page and returns the initial
  rendered state plus the request log. Paste the result: `ok: true`, state `empty`, every
  request origin = the loopback page origin. The driver has no path that reports an
  unperformed action as success (unknown action → `{ "ok": false, "error": "unknown
  action" }` and non-zero).

## @e2e scenario

### 16. Crew member converts a supported duty plan in the browser and saves the offered calendar (`@e2e`)

Use-case constraint: the primary assertion is on what the real browser rendered and what
it saved; the plan never leaves the tab. One inventory entry (the browser supply action)
has this single `@e2e` scenario.

**Design constraints (from design.md):**

- Real interface: the rendered page in Chromium, driven by Playwright 1.64.1 through
  `web/e2e/driver.mjs`. One browser context per scenario, one driver process per scenario,
  one in-process `http.Server` on `127.0.0.1:0` serving `web/`; no database, container,
  port reservation, credential or external service.
- Step location: `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`, separate from
  the regular step file, using the shared `apps/cli/tests/steps/browser.rs`. The driver
  contains actions/observations only; no scenario text and no step assertions.
- The scenario traverses the **empty** and **ready** rendered states, asserts the picker
  is the accessible primary path, sets the file input to the generated `2026-06.pdf`
  fixture, clicks the `2026-06.ics` offer, and captures the browser's download with
  `page.waitForEvent('download')`.
- The captured calendar is compared with `apps/cli/tests/fixtures/expected/2026-06.ics`:
  one `VEVENT` per expected activity with the fixture's `SUMMARY`, `DTSTART`, `DTEND`,
  CRLF line endings, the expected `UID` values, the fixture's `DTSTAMP`, and no extra or
  partial event. "Complete RFC 5545 calendar" is the Then phrase; the step asserts the
  fixture.
- The page has not claimed a save: no visible `saved` claim about its own work.
- Request log: every request origin is the local page origin; no other-origin request
  after load; no `.wasm` request (bytes are inlined).
- Storage/address: no plan in persistent storage, no navigable URL or history entry.
- The page asks for no account, email or upload control.
- Visual baseline at every displayed state plus `@axe-core/playwright`; screenshots per
  viewport for the empty and ready states (see "Evidence rules").
- `@e2e` tag stays on this scenario for the entire change (immutable).

- [ ] RED — remove `@wip` from this scenario only (tags become `@givn.added @e2e`); write
  the e2e steps with `unimplemented!()` bodies in the design's exact e2e file; run the
  single-scenario command (`./run-tests.sh --name "<title>"`, because `--e2e` cannot be
  combined with `--name` in cucumber-rs 0.23); paste the non-zero output:
  ```
  $ ./run-tests.sh --name "Crew member converts a supported duty plan in the browser and saves the offered calendar"
  Feature: Export a duty-plan calendar
    Scenario: Crew member converts a supported duty plan in the browser and saves the offered calendar
     ✔  Given the converter page served from the built Site output
     ✘  Then the page shows the empty state and offers no calendar
        Step panicked. Captured output: not implemented: assert the initial empty rendered state
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  RED_EXIT=101
  ```
- [ ] GREEN — replace the stubs with real driver actions and assertions; production files
  created/modified:
  - `web/e2e/driver.mjs` — the remaining actions used by this scenario (`screenshot`,
    `axe`, the request-log and storage observations) and the download capture.
  - `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` — the real e2e steps.
  - `apps/cli/tests/steps/browser.rs` — any client extension the e2e steps need.
  - Evidence committed:
    `givn/changes/build-browser-converter/evidence/visual/crew-member-converts-a-supported-duty-plan-in-the-browser-and-saves-the-offered-calendar/<viewport>.png`
    and `<viewport>--empty.png` for 360x780 and 1920x1080.
  Run the single-scenario command and paste the zero-exit output:
  ```
  $ ./run-tests.sh --name "Crew member converts a supported duty plan in the browser and saves the offered calendar"
  Feature: Export a duty-plan calendar
    Scenario: Crew member converts a supported duty plan in the browser and saves the offered calendar
     ✔  Given the converter page served from the built Site output
     ✔  Then the page shows the empty state and offers no calendar
     ✔  When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
     ✔  Then the page shows that the calendar is ready
     ✔  And the page offers the file "2026-06.ics"
     ✔  When the crew member saves the offered calendar
     ✔  Then the browser has saved a complete RFC 5545 calendar
     ✔  And the saved calendar matches the reference calendar for "2026-06.pdf"
     ✔  And the page has not reported that it saved a file
     ✔  And the page has sent no request to another origin since it loaded
     ✔  And the page asks for no account, email, or upload
     ✔  And the page meets the visual baseline at every displayed state
  [Summary]
  1 feature
  1 scenario (1 passed)
  12 steps (12 passed)
  GREEN_EXIT=0
  ```
- [ ] REFACTOR — confirm the e2e steps assert only through the real interface (no
  in-process call, no fetch substitute), the screenshots are the committed evidence (not
  debug captures), and the `@e2e` tag is still present; run the whole e2e suite:
  ```
  $ ./run-tests.sh --e2e
  [Summary]
  <features> features
  2 scenarios (2 passed)
  <steps> steps (<steps> passed)
  E2E_SUITE_EXIT=0
  ```
  and re-run the single-scenario command:
  ```
  1 scenario (1 passed), 12 steps (12 passed)   REFACTOR_EXIT=0
  ```
- [ ] COMMIT: one commit for RED+GREEN+REFACTOR; subject
  `test(e2e): A crew member can convert one supported duty plan to a complete calendar in a current browser from the built Site output, with the plan never leaving the tab, or receive an explicit failure with an offered diagnostic.`
  (the release note above prefixed with `test(e2e):`); body contains
  `Scenario: Crew member converts a supported duty plan in the browser and saves the offered calendar`; Hash: `<record the revision identifier here>`

## Final verification (after every section is GREEN)

- [ ] `./run-tests.sh` — full suite exits 0; record the scenario count.
- [ ] `./run-tests.sh --e2e` — e2e suite exits 0; record the scenario count (it is a
  strict subset of the full run).
- [ ] `./measure-coverage.sh && ./merge-coverages.sh` — two distinct Cobertura reports
  from distinct instrumentation runs, merged; `duty2cal-wasm` is outside the native
  denominator per the approved D3 decision, and the shared core remains covered by the
  CLI rows.
- [ ] `givn lint --change build-browser-converter` — exit 0 with no `@wip` findings and
  no shape/subset advisories.
- [ ] `givn check tasks --change build-browser-converter` — exit 0.
- [ ] `grep -c '@wip' givn/changes/build-browser-converter/specs/export-duty-plan-calendar/export-duty-plan-calendar.feature`
  → `0`; the `@e2e` tag is still on
  `Crew member converts a supported duty plan in the browser and saves the offered calendar`;
  `grep -rn 'unimplemented!' apps/cli/tests/steps/` → no output.
- [ ] Non-scenario checks (design changed-files table): `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check`,
  `reuse lint`, `bash scripts/license-headers.sh`, `bash scripts/pii-guard.sh`, and the
  regenerated `THIRD-PARTY-NOTICES.md` diff.
