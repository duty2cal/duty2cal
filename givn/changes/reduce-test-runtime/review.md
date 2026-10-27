# Review: reduce-test-runtime

Operational maintenance change. `specs` is skipped in `.givn-skip`; no
capability scenario is added, modified or removed. The review therefore applies
the fabrication, conformance and coverage checks to the test-scope, CI and
documentation work, not to a product capability delta.

## Fabrication audit

- **Delta `@e2e` tags:** no delta `.feature` file exists. The permanent
  product feature keeps its two `@e2e` scenarios; the browser one adds
  `@browser` and is not downgraded. Result: no tag removal.
- **Empty step bodies:** `0 empty or no-op step bodies found across 4 step
  definition files` (`apps/cli/tests/steps/{browser,export_duty_plan_calendar}.rs`,
  `steps/e2e/{export_duty_plan_calendar,mod}.rs`, `steps/{mod,support}.rs`).
  The deleted operational step files are gone, not stubbed.
- **Task commits:** every checked task records
  `e2218486aeb34f1df7a533b966d43b705273d7bb`. That revision touches the test
  harness, CI workflows, scripts and arc42 (not product source) — this is a
  maintenance change whose implementation is the harness/CI change itself.
  The task record commit is `08e825726f98cc4ab7626b67fec802ad810ec814`.
- **Promised components:** `apps/cli/tests/golden.rs`,
  `scripts/check-drift.sh`, `scripts/tests/publish-site.bats`, the assembler
  `@documentation` skip, the runner phase lines and guards, the CI
  `test`/`browser`/`scripts` jobs, the main-only coverage gate, and the
  Windows budget all exist and are documented in `design.md`.
- **Strict mode:** `.fail_on_skipped()` remains on the cucumber runner
  (`features_runner.rs`); the task record carries the 84-scenario product run
  and the 1-scenario e2e run.
- **`@e2e` Then steps:** the CLI e2e asserts the real subprocess's exit status,
  stdout path and published file; the browser e2e asserts the rendered page,
  the captured download and the visual baseline. No repository-only
  downgrade.
- **Browser driver:** the browser step file uses `BrowserDriver` / Playwright
  actions (`start`, `set_file`, `save_offer`, `read_state`); no HTTP or
  `fetch` substitute.
- **E2E command binding:** `givn/commands.yaml` sets
  `verify.command: ./run-tests.sh` and
  `verify.e2e_command: ./run-tests.sh --e2e`; the values differ. The e2e
  command selects `@e2e and not @browser`; the browser e2e is selected by the
  dedicated `--browser --name` job and by `scripts/site-evidence.sh`. The
  recorded counts are regular 84 scenarios and e2e 1 scenario; e2e is strictly
  smaller. The split is deliberate and recorded in the design and R-070.
- **Normed E2E scope:** the two inventory actions each have one `@e2e`
  scenario (executable conversion, browser save). The browser variants are
  regular scenarios and are not additional actions.
- **Design conformance:** the runner, step file layout, browser driver, coverage
  filters, site golden step and Windows budget match `design.md`; no
  unreviewed deviation.
- **Interaction coverage cross-reference:**

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| supply a duty plan for conversion through the executable | yes | Crew member converts a supported duty plan and receives a calendar file | real `duty2cal` subprocess |
| supply a duty plan through the converter page and save the offered calendar | yes | Crew member converts a supported duty plan in the browser and saves the offered calendar | Playwright driver |

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Product-only `cargo test` | 02, 05, 08 | yes | test scope and runner | task 1 | `run-tests.sh`, assembler, trimmed harness | yes |
| `@documentation` features are skipped | 02, 05, 08 | yes | marker and assembler | task 1 | `scripts/assemble_features.py`, 8 tagged features | yes |
| Browser smoke in its own job | 02, 05, 07 | yes | browser job | task 1 | `ci.yml` `browser` job | yes |
| Golden test before site build | 07 | yes | site workflow | task 3 | `site.yml` golden step | yes |
| 2,097,152 B executable budget | 02, 07, 10, 11 | yes | D3, budget | task 3 | `ci.yml` budget step; local build 2,013,696 B | yes |
| Per-push attribution stays | 02, 05, 06, 08, 10 | yes | D6 | task 3 | `ci.yml` attribution before budget | yes |
| Local drift, no CI job | 02, 07, 11 | yes | D1 | task 4 | `scripts/check-drift.sh`; no CI drift job | yes |
| Publication early refusals | 02, 05, 07, 10 | yes | bats | task 4 | `scripts/tests/publish-site.bats` | yes |
| Operational features are documentation | 06, 08, 11 | chapter 6 Yes | test scope | task 5 | chapter amendments, R-067 | yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

No domain term is added or redefined. `@documentation`, `@browser`, "browser
smoke", "phase line" and "size budget" are engineering vocabulary described in
chapter 8; the glossary's "Size attribution" remains accurate. No finding.

## Coverage classification

- **Deleted operational step definitions and fixtures → dead code (bucket 1).**
  Their subject was operational tooling, not the product; the deleted files
  and the size-attribution fixture are removed.
- **Deleted configuration preview/apply scenarios → legitimately hard to test
  (bucket 3).** They required the make-believe GitHub API, `tofu` and a
  primed encrypted state; the real evidence is the maintainer's preview/apply
  against the live organization. The operator's decision is recorded in D1 and
  R-067; the feature files remain as documentation.
- **Browser variants not in CI → covered by the on-demand suite, with the CI
  gap recorded (bucket 3).** The 16 browser scenarios remain in the product
  feature; CI runs one smoke, and the other 15 run with `./run-tests.sh
  --browser`. D2 and R-070 record the deliberate narrowing. This is not a
  missing scenario: the scenarios exist and pass on demand.
- **Site publication behaviour → bats covers the early refusals (bucket 1 for
  the deleted step code); the workflow is the executable gate.** The full
  local twin was deleted with the step definitions; R-067 records it.

No coverage percentage is claimed; the archive receipt carries the measured
result. The last product run is 84 scenarios / 303 steps passing; the e2e run
is 1 / 5; the browser smoke is 1 / 12; bats is 10/10.

## E2E coverage

The browser page is a Web UI: a rendered page is the interface, so the opposite
classification (an HTTP client) does not hold. The e2e browser scenario drives
Playwright and asserts the rendered state, the captured download and the visual
baseline. The CLI e2e drives the real subprocess and asserts its output and the
published calendar. `verify.e2e_command` is `./run-tests.sh --e2e`, not a copy
of `verify.command`; its 1 scenario is strictly less than the regular 84.

## Visual review

No screen is introduced or visibly changed. The existing browser smoke
traverses the empty and ready states at 360x780 and 1920x1080 and asserts the
visual baseline; its frozen screenshots remain under
`givn/archive/build-browser-converter/evidence/visual/crew-member-converts-a-supported-duty-plan-in-the-browser-and-saves-the-offered-calendar/`.
The smoke passed in this change. No new screenshot is required because the
contract is unchanged.

VISUAL-REVIEW: PASS

## Use-case and Persona conformance

The permanent `export-duty-plan-calendar` use case and its two inventory
entries are unchanged; the browser save and the CLI conversion keep their
goals, guarantees and typed relationships. The confirmed
`crew-member--personal-calendar-owner` remains the review lens for the browser
smoke. No Persona is invented or promoted to a Gherkin actor.

## Deferrals

- **Configuration drift in CI** — deferred by the operator on 2026-10-27
  (D1; ADR-0006). The local `scripts/check-drift.sh` is the control.
- **Browser variants in CI** — deferred by the operator (D2). The smoke runs
  in CI; the variants run on demand. R-070 records the gap.
- **Operational configuration scenarios** — deferred by the change's premise
  and the operator's analysis; R-067 records the remaining gap. No
  undispositioned mandatory check remains.

## Overlap dispositions

No delta scenario exists, so there is no removed/added pair or change-shaped
overlap to disposition. The `givn lint` advisories live in `@documentation`
features and are not executed; they are not change scenarios.

## Split-or-keep

No delta scenario exists; no long scenario is introduced by this change.
Decision: none.

## README impact

The Development section now lists `./run-tests.sh`, `./run-tests.sh --e2e` and
`./run-tests.sh --browser`, and the pull-request template names the browser
run.

README-IMPACT: updated - Development

## Sign-off

- Fabrication audit: clean.
- Every checked task records the implementation revision; the change modifies
  no product source by design.
- Every promised component exists.
- Strict-mode proof present (`.fail_on_skipped()`).
- Execution evidence recorded per scope: regular 84, e2e 1, browser smoke 1.
- Coverage gaps classified: deleted operational code (dead), configuration
  scenarios (hard to test, operator decision), browser variants (on-demand,
  R-070).
- No `@wip` tags; no implementation detail in the (unchanged) product spec.
- Canonical E2E policy read; two actions, two `@e2e`; variants regular.
- Real-interface assertions and Playwright driving verified.
- Local run command starts the product suite with no external service.
- `verify.e2e_command` differs from `verify.command`; counts prove the scope.
- Implementation matches `design.md`.
- Interaction coverage and arc42 conformance clean.
- No invented fourth-bucket exception.

REVIEW: PASS
