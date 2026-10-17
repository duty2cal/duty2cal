# Tasks: migrate-0-8-4-to-0-9-1

This is an aggregate maintenance change with no product specification: `specs` is skipped in `.givn-skip` and there are no `.feature` scenarios. The scenario-by-scenario RED/GREEN/REFACTOR loop therefore has no scenario to run. The task list instead records the ordered migration work from the design's inventory rows, the Phase 3 verification, and the required commits. The runner is already configured and its strictness was proven by the existing suite; the baseline below is the proof that the setup task is inherited and still passes.

## 1. Baseline setup and runner

- [x] Confirm the runner, strict mode, and current baseline.
  - Design constraints: `verify.command` is `./run-tests.sh`; `verify.e2e_command` is `./run-tests.sh --e2e`; the toolchain is `nix-shell -p rustc cargo git-cliff`; `specs` is skipped; no product scenario belongs to this change.
  - RED: N/A — no scenario is added by this maintenance change.
  - GREEN: `nix-shell -p rustc cargo git-cliff --run './run-tests.sh'` → 2 features, 103 scenarios, 390 steps, all passing.
  - REFACTOR: N/A — no production code changes.
  - E2E: `nix-shell -p rustc cargo git-cliff --run './run-tests.sh --e2e'` → 2 features, 3 scenarios, 16 steps, all passing; 3 is strictly less than the full 103.
  - Commit: `acb9d311cab9863277fb9e1f28658e1117e9337d`.

## 2. Inventory the displaced project rules

- [x] Recover and scan the pre-upgrade generated rules, classify every displaced statement, and record the result.
  - Design constraints: pre-upgrade rules are untracked (`rules: local-only`); the baseline is givn `v0.8.5`; the only edit event is the `givn-ideation` `## Diagram` workaround; the managed `AGENTS.md` block is unchanged; record the evidence boundary.
  - Evidence: `git ls-tree -r --name-only 76f1602771c6bf7c4430ac691657cff1e0a5b86c^` returns no generated rule paths; `git log --all -- .opencode .agents .claude .omp` is empty; recovered bodies match `v0.8.5`; `opencode.db` and every omp transcript contain no other generated-path write.
  - Result: `design.md` `## Phase 1 inventory — displaced project rules`; four ideation files, one stale workaround, no surviving project rule.
  - Commit: `acb9d311cab9863277fb9e1f28658e1117e9337d`.

## 3. Phase 2 — extension disposition (no-op)

- [x] Confirm no project rule needs an `extensions:` entry and no project file is edited.
  - Design constraints: Phase 2 is a no-op; the ideation displacement is stale because the served `ideation` instruction and the project's permanent use cases carry `## Diagram`; no rule text may be invented.
  - Evidence: `givn/config.yaml` has no active `extensions:` key; `git status` shows no config change; every served instruction has zero `## Project extension` sections.
  - Commit: `acb9d311cab9863277fb9e1f28658e1117e9337d`.

## 4. Phase 3 — installed files and served instructions

- [x] Verify the installed files in every configured target carry only trigger/direction.
  - Design constraints: targets are `opencode`, `agents`, `claude`, `omp`; expected shape is 15 skills per target and 15 commands for `opencode`/`omp`, 0 for `agents`/`claude`.
  - Evidence: 15/15/15/15 skills; 15/15/0/0 commands; all trigger-only; `grep -rn '## Project extension'` returns nothing.
  - Commit: `acb9d311cab9863277fb9e1f28658e1117e9337d`.

- [x] Verify a fresh `givn skills sync` leaves every generated file byte-identical.
  - Design constraints: include skills, commands, attribution plugins/hooks, `.claude/settings.local.json`, and `.githooks/commit-msg`; no extension text may appear.
  - Evidence: SHA-256 manifest of 95 generated files before and after `givn skills sync` → `diff` reports no differences.
  - Commit: `acb9d311cab9863277fb9e1f28658e1117e9337d`.

- [x] Verify every served instruction id is extension-free.
  - Design constraints: ids `proposal`, `specs`, `design`, `design-review`, `tasks`, `review`, `archive`, `explore`, `ideation`, `steps`, `reverse-engineer`, `characterize`, `domain-modeling`, `dev-principles`, `grill-me`, `grill-with-docs`.
  - Evidence: each returns exit 0 and `grep -c '^## Project extension'` = 0.
  - Commit: `acb9d311cab9863277fb9e1f28658e1117e9337d`.

## 5. Phase 3 — lint, suite, and archive

- [x] Run the static check and the full suite.
  - Design constraints: `givn lint --change migrate-0-8-4-to-0-9-1`; `./run-tests.sh` under the pinned toolchain; no product code is changed.
  - Evidence: lint exits 0 with "no .feature files found"; suite is 103 scenarios / 390 steps passing; e2e is 3 scenarios / 16 steps passing.
  - Commit: `acb9d311cab9863277fb9e1f28658e1117e9337d`.

- [ ] Commit the tasks artifact and complete the review/archive gates.
  - Design constraints: one attributed `givn commit` for the tasks evidence; then `givn check tasks`, the review artifact, and `givn archive`.
  - Evidence: committed tasks revision recorded below; review and archive must pass their gates.
  - Commit: pending.

## Commit log

| Revision | Purpose |
|---|---|
| `acb9d311cab9863277fb9e1f28658e1117e9337d` | Migration inventory, arc42 assessment, and design review evidence. |
