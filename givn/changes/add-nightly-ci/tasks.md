# Tasks: add-nightly-ci

Operational maintenance change; `specs` is skipped in `.givn-skip`, and no
product scenario is added, modified or removed. The task list records the
workflow, the actionlint wiring, the documentation, and the local verification.

Design constraints: the nightly is read-only (`contents: read`), holds no state
passphrase, administer token or deploy key, and does not apply configuration,
publish a site or publish a release. The product job runs `./run-tests.sh`
(84 scenarios including the CLI e2e); the browser job runs
`./run-tests.sh --browser` (16 scenarios); the deny job runs `cargo deny check`
with the install action.

## 1. Nightly workflow

- [x] Add `.github/workflows/nightly.yml` with schedule `17 3 * * *` and `workflow_dispatch`.
  - Evidence: the workflow has `contents: read`, `concurrency: group: nightly`, and jobs `product` (15 min), `browser` (30 min), `deny` (10 min). No `tofu`, `secrets.`, `git push`, `gh`, deploy key or publication step.
  - Commit: `a258e675279e9295639b482079af9590ca621f08`.

- [x] Run the full product suite in the `product` job.
  - Evidence: `./run-tests.sh` locally → 84 scenarios / 303 steps, all passing; the run includes the CLI `@e2e` scenario by the default `not @browser` filter.
  - Commit: `a258e675279e9295639b482079af9590ca621f08`.

- [x] Run every browser variant in the `browser` job.
  - Evidence: `./run-tests.sh --browser` locally → 16 scenarios / 96 steps, all passing; `phase=site-build seconds=18`, `phase=cucumber seconds=36`; the local run used `CHROMIUM_PATH` (CI uses the pinned Playwright Chromium).
  - Commit: `a258e675279e9295639b482079af9590ca621f08`.

- [x] Add the dependency-advisory job and its log artifact.
  - Evidence: `deny` uses `taiki-e/install-action@v2` with `cargo-deny`; `cargo deny check` is piped to `nightly-deny.log` under `pipefail` and uploaded with `if: always()`.
  - Commit: `a258e675279e9295639b482079af9590ca621f08`.

## 2. CI and documentation

- [x] Add `nightly.yml` to the actionlint file list.
  - Evidence: `.github/workflows/ci.yml` lists `.github/workflows/nightly.yml` with the other workflows.
  - Commit: `a258e675279e9295639b482079af9590ca621f08`.

- [x] Record the nightly boundary, schedule limit and quality scenario.
  - Evidence: chapters 2, 7, 8, 10 (QS-042) and 11 (R-070/R-071) are updated; `arc42.md` marks chapters 2, 7, 8, 10 and 11 affected and ends `STATUS: DONE`; R-071 records the 60-day inactivity disable.
  - Commit: `a258e675279e9295639b482079af9590ca621f08`.

## 3. Verification

- [x] Verify the product and browser suites locally.
  - Evidence: product 84/303 passed; browser 16/96 passed; `git diff --check` clean; `bash -n run-tests.sh` clean; `givn lint` exit 0.
  - Commit: `a258e675279e9295639b482079af9590ca621f08`.

- [x] Confirm the scheduled-only checks are delegated to CI.
  - Evidence: `actionlint` checks the workflow in CI; `cargo deny check` and the scheduled run are CI jobs, not local (the local `cargo-deny` proxy cannot find cargo). The design review ended `DESIGN-REVIEW: PASS`.
  - Commit: `a258e675279e9295639b482079af9590ca621f08`.

## Commit log

| Revision | Purpose |
|---|---|
| `a258e675279e9295639b482079af9590ca621f08` | Nightly workflow, actionlint entry, and chapters 2/7/8/10/11. |
| this revision | The completed task record and its evidence; archived with the change. |
