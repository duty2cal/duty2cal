# Review: add-nightly-ci

Operational maintenance change. `specs` is skipped in `.givn-skip`; no
capability scenario is added, modified or removed. The review applies the
fabrication, conformance and coverage checks to the workflow and documentation.

## Fabrication audit

- **Delta `@e2e` tags:** no delta `.feature` file exists. The permanent product
  feature keeps its two `@e2e` scenarios; the nightly runs the existing ones.
- **Empty step bodies:** no step definition is added or changed. The existing
  product step files are unchanged; 0 new empty bodies.
- **Task commits:** every checked task records
  `a258e675279e9295639b482079af9590ca621f08`, which adds the workflow, the
  actionlint entry and chapters 2/7/8/10/11. The task record is committed
  separately.
- **Promised components:** `.github/workflows/nightly.yml` exists with
  `product`, `browser` and `deny` jobs; `.github/workflows/ci.yml` lists the new
  file in actionlint; the five chapters carry the nightly boundary. No missing
  component.
- **Strict mode:** unchanged; `.fail_on_skipped()` remains.
- **`@e2e` Then steps:** the existing CLI and browser e2e assertions are
  unchanged; the nightly adds no weaker path.
- **Browser driver:** unchanged; the nightly uses the existing Playwright
  driver.
- **E2E command binding:** `verify.command` is `./run-tests.sh`;
  `verify.e2e_command` is `./run-tests.sh --e2e`; the values differ. The
  product run includes the CLI e2e; the browser run includes the browser e2e.
  The nightly adds no third e2e command.
- **Normed E2E scope:** two inventory actions, two `@e2e` scenarios; unchanged.
- **Design conformance:** the workflow matches the design's three jobs,
  timeouts, permissions, concurrency and log artifacts; no silent deviation.
- **Interaction coverage cross-reference:**

| Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| supply a duty plan for conversion through the executable | yes | Crew member converts a supported duty plan and receives a calendar file | real `duty2cal` subprocess |
| supply a duty plan through the converter page and save the offered calendar | yes | Crew member converts a supported duty plan in the browser and saves the offered calendar | Playwright driver |

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Nightly schedule and manual dispatch | 02, 07 | yes | D1 | task 1 | `nightly.yml` `schedule` + `workflow_dispatch` | yes |
| Read-only, no secrets | 02, 07 | yes | D2, permissions | task 1 | `contents: read`; no `secrets.`/`tofu`/deploy key | yes |
| Product suite nightly | 02, 07, 10 | yes | product job | task 2 | `nightly.yml` `./run-tests.sh` | yes |
| All 16 browser variants nightly | 02, 07, 08, 10 | yes | browser job | task 3 | `nightly.yml` `./run-tests.sh --browser`; local 16/96 pass | yes |
| Fresh advisory check | 02 | yes | deny job | task 4 | `cargo deny check` + install action | yes |
| 60-day schedule limit | 02, 08, 10, 11 | yes | D4 | task 5 | R-071, workflow header, QS-042, R-070 | yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

No domain term is added. "Nightly", "browser variant", "product suite" and
"advisory database" are engineering vocabulary used consistently in the
proposal, design and chapters. No finding.

## Coverage classification

- **New workflow (bucket 3: hard to test locally as a scheduled trigger).**
  The three jobs are executed by the same commands locally: the product suite
  passed 84/84, the browser suite 16/16. The schedule itself cannot run until
  the workflow is on the default branch; the first run is a manual
  `workflow_dispatch`. `actionlint` in CI validates the workflow syntax.
- **No new product code, no missing product coverage.** The existing 84
  product and 16 browser scenarios keep their evidence.
- **Dependency job (bucket 3).** `cargo-deny` is not runnable in the local
  shell; CI installs the pinned tool and runs it. The workflow captures the
  log for the failure case.

No coverage percentage is claimed for this change; the archive receipt carries
the measured result for the unchanged product suite.

## E2E coverage

The browser page is a Web UI; the e2e scenario drives Playwright and asserts
the rendered page and the captured download. The CLI e2e drives the real
subprocess. `verify.e2e_command` remains distinct from `verify.command`; the
e2e count is 1 and the regular count is 84.

## Visual review

No screen is introduced or changed, and no visual design contract is declared
by this change. The existing browser screenshots remain the archived evidence;
the nightly runs the same assertions. No visual finding.

## Use-case and Persona conformance

No use case or Persona changes. The two inventory actions remain as recorded.
The proposal declares no Persona because this change has no product surface.

## Deferrals

- **The scheduled trigger itself** cannot be exercised before the workflow is
  on the default branch; the first run is a manual dispatch, and actionlint
  validates the definition in CI. This is a deployment fact, not a deferred
  mandatory product check.
- **Configuration preview/apply and publication** remain out of CI by
  ADR-0006; the nightly does not reopen them.

## Overlap dispositions

No delta scenario exists; no removed/added pair or change-shaped overlap. The
`givn lint` advisories remain in `@documentation` features and are not change
scenarios.

## Split-or-keep

No delta scenario exists; no long scenario is introduced. Decision: none.

## README impact

This change does not touch a public command, flag, output or README section.

README-IMPACT: none

## Sign-off

- Fabrication audit: clean.
- Checked tasks record the implementation revision.
- Promised components exist.
- Strict-mode proof unchanged.
- Execution evidence: product 84/84, browser 16/16 locally; deny and
  actionlint delegated to CI.
- Coverage gaps classified as hard to test (scheduled trigger and local
  cargo-deny), with the concrete command/CI justification.
- No `@wip` tags; no spec change.
- E2E policy, real interface and driver unchanged.
- `verify.e2e_command` is distinct; counts are 84/1.
- Implementation matches `design.md`.
- Interaction coverage and arc42 conformance clean.
- No invented fourth-bucket exception.

REVIEW: PASS
