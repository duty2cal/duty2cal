# Proposal: add-nightly-ci

## Release note

A nightly CI run exercises every browser variant and the full product suite, so the coverage that is now on demand is measured on a nightly schedule instead of waiting for a release.

## Use-Case Context

- Use-case ID: none. This is an operational maintenance change.
- Ideation topic: none.
- Seed path: none.
- Confirmed Personas: none — no product surface changes; the change runs the existing suite on a schedule.

## Problem / Opportunity

The archived `reduce-test-runtime` change left 15 browser variants on demand. The per-push CI runs one browser smoke, so a regression in the size bound, the diagnostic offer, the capability gate, the drop zone, the cancelled save, the naming or the persistence variants is visible only when a developer remembers `./run-tests.sh --browser`. The risk is recorded as R-070, with no scheduled run.

There is no `schedule:` trigger in any workflow. A quiet `main` can therefore go days without a full browser pass, and a dependency or runner-image change that breaks a variant is found late.

The opportunity is a nightly run that exercises the whole runnable suite — the 84 product scenarios (which include the CLI e2e scenario) and the 16 browser variants — and the dependency-advisory check, without putting Playwright or a long run on every push.

## Proposed Solution

- **A scheduled workflow runs nightly and on manual dispatch.** `.github/workflows/nightly.yml` runs at 03:17 UTC and through `workflow_dispatch`; it holds `contents: read`, uses a `nightly` concurrency group, and sets per-job timeouts of 15 (product), 30 (browser) and 10 (deny) minutes.
- **The product job runs the product suite.** `./run-tests.sh` runs the 84 product scenarios — including the CLI e2e scenario — the unit/property/architecture tests and the golden test.
- **The browser job runs every browser variant.** `./run-tests.sh --browser` builds the bundle and Site output and runs all 16 `@browser` scenarios, including the one smoke.
- **The dependency job refreshes the advisory database.** `cargo deny check` runs with the pinned tool through `taiki-e/install-action`, so a new advisory is reported without waiting for a push.
- **The run is observable.** The product and browser jobs write their transcript (and phase lines) to the job summary and an artifact; the dependency job uploads its run log. A red nightly is visible in the Actions run.
- **The local drift check stays out.** ADR-0006 still forbids the passphrase and administer token in CI. The nightly does not apply configuration, publish a site, or publish a release; those remain workflow- or maintainer-owned.
- **The schedule has a known limit.** GitHub disables a scheduled workflow after 60 days without repository activity. A manual dispatch or any push re-enables it; R-071 records the limitation so a quiet repository does not silently lose the signal.

## Capability Routing

`givn spec route --change add-nightly-ci` has no capability signal. This change adds no product behaviour, so `specs` is skipped in `.givn-skip`.

| Proposed capability | Decision | Rationale |
|---|---|---|
| none | No capability delta | The workflow runs the existing product scenarios; no observable product behaviour changes. |

## Out of Scope

- **Configuration drift or apply in CI.** ADR-0006 and chapter 2 keep the passphrase and administer token local.
- **Site or release publication in the nightly.** The tag and default-branch workflows own them.
- **A second platform matrix.** The Windows cross-build and smoke already run per push.
- **New product scenarios.** The 16 browser scenarios already exist and run on demand.
- **Issue creation or chat notification.** A failed scheduled run is visible in Actions and through GitHub's workflow notifications; no new write credential is added.

## Open Questions

None. The schedule, the jobs, and the `contents: read` boundary follow the accepted test-scope decision and ADR-0006.
