# Proposal: reduce-test-runtime

## Release note

The test suite now runs only the product's behaviour, so a local run and CI finish in minutes instead of hours.

## Use-Case Context

- Use-case ID: none. This is an operational maintenance change; it changes the verification surface, not a product capability.
- Ideation topic: none.
- Seed path: none.
- Confirmed Personas: `crew-member--personal-calendar-owner` — the review lens for the browser smoke; no product surface changes.

The domain contract is unchanged. The conversion capability `export-duty-plan-calendar` keeps its goal, scenarios, and step behaviour; the operational capabilities keep their Gherkin files as documentation but stop being executed by the cucumber runner.

## Problem / Opportunity

The `test` CI job takes 1h28m in run #87. Every other job finishes within ten minutes, including the Windows cross-build at 2m03s. A red run costs 41 minutes to 2h37m, and the ten publication scenarios alone failed in #87. A local full run has the same shape.

The job spends its time on operational specifications: release preparation, site publication, organization profile, repository security, adoption, and executable-size measurement. They drive real `tofu`, real `git worktree`, real cross-builds, and real subprocess suites from inside the cucumber runner. Those are good checks, but `cargo test` is the wrong place for them: they need toolchains the product does not need, they duplicate checks the owning workflow performs, and one failure spends the whole suite before reporting it.

The test job also lacks a Rust cache and a timeout, compiles `cargo-bloat` from source on every run, and installs `mingw-w64`, `git-cliff`, and OpenTofu for tests that no longer need them.

The opportunity is to make `cargo test` mean "the product works" and to put each operational guarantee where it is cheap, real, and fails in the right job.

## Proposed Solution

- **The cucumber runner executes product behaviour only.** The domain feature `export-duty-plan-calendar` remains runnable. The eight operational feature files stay in `givn/specs` as documentation, marked `@documentation`, and the assembler leaves them out of the runner root. Their step definitions are removed.
- **One browser smoke check remains.** The existing end-to-end browser scenario stays runnable as the WASM facade's smoke check, tagged `@browser`. Browser scenarios are excluded from the default run; a separate CI job runs the smoke check, and the remaining browser variants stay available on demand.
- **Operational guarantees move to their owner.**
  - The Windows build job enforces a byte budget on the cross-built executable and keeps the per-push size attribution; the `test` job no longer installs the generator.
  - The reusable site publication workflow runs the conversion golden test, builds, gates, and only then deploys; a failure deploys nothing.
  - The release publication keeps waiting for the tagged revision's checks to conclude green before publishing.
  - Configuration drift stays a local check, as the accepted local-state decision (ADR-0006) requires; no passphrase or administer token enters CI.
  - `publish-site.sh`'s refusal logic is covered by `bats` tests in a dedicated `scripts` job, without a cargo build.
- **CI stops paying for tools it does not use.** The test job gets a Rust cache, a 20-minute timeout, and no `mingw-w64`, `cargo-bloat`, `git-cliff`, OpenTofu, or browser toolchain. `cargo-deny`, `cargo-bloat`, and `cargo-llvm-cov` come from the pinned install action instead of source builds. Coverage runs on `main` only.
- **The runner reports its phases.** `run-tests.sh` prints `phase=<name> seconds=<n>` and the cucumber concurrency, and CI appends the lines to the step summary. The target is ten minutes in CI and five minutes locally with `--no-fail-fast`.

## Capability Routing

`givn spec route --change reduce-test-runtime` reported **no signal**. The change introduces no capability: it removes operational scenarios from execution, keeps their Gherkin as documentation, and moves their guarantees to CI jobs and a local drift check. No delta `.feature` file is authored; the `specs` artifact is skipped in `.givn-skip` because there is no product behaviour to add, modify, or remove.

| Proposed capability | Decision | Rationale |
|---|---|---|
| none | No capability delta | The product's observable behaviour is unchanged. The only Gherkin edits are execution tags on existing browser scenarios and `@documentation` on operational features. |

## Out of Scope

- **Product behaviour.** The converter, the browser page, and the CLI output are unchanged.
- **Deleting operational specifications.** Their Gherkin files stay in `givn/specs` as documentation; only their binding and execution are removed.
- **Re-binding an operational feature.** It needs a new ADR naming the defect no other mechanism would catch.
- **Configuration drift in CI.** The accepted local-state decision (ADR-0006) forbids the passphrase in any workflow. The drift check stays local.
- **No new browser coverage.** The existing smoke scenario is kept; no new UI behaviour is specified.
- **The release tag ruleset.** It remains repository configuration owned by the applied declaration; this change does not apply it.

## Open Questions

### D1 — Where does configuration drift run?

- Question: should configuration drift be detected by a CI job or by the maintainer's local check?
- Options:
  1. **Local check** — the maintainer runs `scripts/check-drift.sh` with the passphrase and a read-only token in their environment. Gain: preserves the accepted local-state boundary; no secret can decrypt the pages publishing key. Cost: drift is found when the maintainer looks, not on every push. In practice: a hand-disabled secret scanning setting is reported by the next local run, not by CI.
  2. **CI drift job** — store the state passphrase and a read-only token as repository secrets and run `tofu plan -detailed-exitcode` on every push. Gain: drift is caught automatically. Cost: any workflow compromise can decrypt the pages publishing key, reversing the accepted boundary. In practice: a malicious workflow change could read the passphrase secret.
- Disposition: answered by the user on 2026-10-27 — **local check**. ADR-0006 and chapter 2 stay in force.

### D2 — How much browser coverage runs by default?

- Question: should the default `cargo test` run the browser variants, or only one smoke scenario?
- Options:
  1. **One smoke by default, variants on demand** — the default run excludes `@browser`; a separate CI job runs the existing end-to-end browser scenario by name; `./run-tests.sh --browser` runs the remaining variants when needed. Gain: the default suite is fast and still proves the rendered page works. Cost: a regression in an untested browser variant is caught only by the on-demand run. In practice: a change to the save prompt is caught by the smoke; a change to the size-bound error is caught when the variants run.
  2. **All browser variants by default** — keep every browser scenario in the default run. Gain: maximum coverage. Cost: the default suite needs Node, Chromium, and the bundle on every run, which is the runtime this change removes. In practice: every local run pays the browser startup cost.
- Disposition: answered in this proposal — **one smoke by default**. The variants remain in the feature and are runnable with `--browser`.

### D3 — What is the executable size budget?

- Question: what byte ceiling should the Windows build job enforce?
- Options:
  1. **2,097,152 bytes (2 MiB)** — the last published distributed executable's file size; a local cross-build of this revision measured 2,013,696 B. Gain: catches growth beyond the reviewed size without failing on the current build. Cost: only ~83,456 B (81.5 KiB) of headroom, so a large feature fails the job. In practice: the job fails when `duty2cal.exe` exceeds 2 MiB.
  2. **2,200,000 bytes** — about 5% above the last published executable. Gain: more room for ordinary dependency growth. Cost: a 180 KiB regression passes. In practice: the job fails only above 2.2 MB.
  3. **No budget** — keep attribution only. Gain: no false failures. Cost: silent growth. In practice: the executable doubles without a red job.
- Disposition: answered in this proposal — **2,097,152 bytes**, to be tightened after a fresh cross-build. The budget is a ceiling, not a target. The per-push attribution stays in the `windows-gnu-build` job; the `test` job no longer installs the generator.
