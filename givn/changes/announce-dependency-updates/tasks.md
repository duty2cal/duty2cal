# Tasks: announce-dependency-updates

Release note (commit subject): A version whose only changes are dependency updates can now be published, with those updates named in its release notes.

Owning use case: [Publish a public release](specs/publish-public-release/usecase.md) (`publish-public-release`). Confirmed Personas: `project-maintainer--release-owner` (the Actor who prepares and publishes a release) and `crew-member--personal-calendar-owner` (the reader the notes serve). Actors in scenarios stay distinct from Personas.

Single-scenario run command: `./run-tests.sh --name "<scenario title>"`. Full suite: `./run-tests.sh`. E2E-only: `./run-tests.sh --e2e`; this change adds no `@e2e` scenario and removes none.

Use-case constraints carried into every scenario task (from the owning use case's rules):

- A released change is a capability-scoped revision or a dependency update (`chore(deps): …`) whose words do not name the project's own tooling.
- A released revision must have changed a path a user receives; `Cargo.lock`, `Cargo.toml`, the converter crates and `.github/workflows/release.yml` are on the marked list; `.github/workflows/ci.yml` is not.
- Dependency updates are grouped as **Dependencies**, after Reversals, in history order; identical subjects collapse to one entry; a capability-scoped dependency update stays in its capability group and is not repeated.
- A version whose only announced changes are dependency updates is releasable; a version whose notes name nothing at all is refused.
- The change log and the release text come from one render; the committed change log must match a fresh render of the released revision.
- The empty-heading prune runs after path filtering and collapse and before the visible-count check, so a group emptied by the filter never reaches the change log or release text.
- Private duty plans and real identifying values never enter the public history; fixture repositories are generated.

Interaction coverage (checked before writing tasks): the use case's inventory holds two entries — "prepare the release for a version" and "generate the change log for a version" — each already matched by an `@e2e` scenario and each named in `design.md`'s Interaction Coverage Matrix. The delta's scenarios are variants of those two actions, so no new inventory entry, `@e2e` scenario or matrix row is owed.

The production change is one atomic slice (parser plus renderer prune), so its scenarios share revision `e2a6efd`; the per-scenario entries below record that shared revision rather than pretending nine independent production commits. Test-only scenarios add no production file.

## Setup task — toolchain, pinned generator, strict mode, fixture facts

- [x] Confirm the toolchain, the pinned generator, strict mode and the fixture baseline.
  - Toolchain: `nix-shell -p cargo` provides `cargo 1.98.0`; `nix-shell -p git-cliff` provides `git-cliff 2.14.2`. `cargo clippy` needs `nix-shell -p cargo clippy`; coverage needs `cargo cargo-llvm-cov` plus `LLVM_COV`/`LLVM_PROFDATA` from `/nix/store/62xr80awfwywf5zggcx3gc0cxnpnsr24-llvm-21.1.8/bin`.
  - `givn/commands.yaml` names `verify.command: ./run-tests.sh` and `verify.e2e_command: ./run-tests.sh --e2e`.
  - Strict mode is `.fail_on_skipped()` in `apps/cli/tests/features_runner.rs`; the baseline below fails every delta scenario with `Step doesn't match any function`.
  - Fixture facts: `build_history` delegates to `build_history_with`, creates every `RECEIVED_PATHS` entry, maps every `chore(…)` subject to `MACHINERY` by default, and accepts per-revision path overrides. Every dependency fixture overrides its `chore(deps)` revisions to `Cargo.lock`, `Cargo.toml`, `.github/workflows/ci.yml` or `.github/workflows/release.yml` as the scenario names; the repeated-subject fixture uses `Cargo.lock` then `Cargo.toml` so the second revision is not an empty commit.
  - Baseline before implementation:

    ```
    [Summary]
    3 features
    112 scenarios (103 passed, 9 failed)
    399 steps (390 passed, 9 failed)
    ```

## Scenario tasks

### 1. A version whose only changes are dependency updates is announced

**Design constraints:** Parser row `{ message = "^chore\\(deps\\):", group = "<!-- 5 -->Dependencies" }` after Reversals and before the catch-all; machinery skips stay first. `DEPENDENCY_ONLY_HISTORY`: two distinct `chore(deps)` revisions, `chore(ci)` and `docs(readme)`, indices 0–1 overridden to `["Cargo.lock"]`, tag `v0.2.0` on the last revision. Steps `a_history_of_dependency_updates`, `notes_name_the_dependency_updates`, `notes_name_no_housekeeping_revision`.

- [x] RED: baseline run; the scenario failed because its steps did not exist. Evidence:

  ```
  112 scenarios (103 passed, 9 failed)
  Step doesn't match any function
  ```
- [x] GREEN: parser row and fixture/steps added. Production files: `cliff.toml`, `apps/cli/tests/steps/publish_public_release.rs`. Targeted run:

  ```
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  ```
- [x] REFACTOR: `cargo fmt --all --check` clean; the targeted run stays green.
- [x] COMMIT: revision `e2a6efd` (shared production change).

### 2. Identical dependency update subjects collapse to one entry

**Design constraints:** Existing collapse after the path filter. `DEPENDENCY_COLLAPSE_HISTORY`: two identical `chore(deps): bump serde from 1.0.0 to 1.0.1` revisions, the first overridden to `["Cargo.lock"]`, the second to `["Cargo.toml"]`, plus `chore(ci)`, tag `v0.2.0`. Steps `a_history_with_repeated_dependency_subjects`, `dependency_update_named_once`.

- [x] RED: baseline run failed with undefined steps (same `112 scenarios (103 passed, 9 failed)`).
- [x] GREEN: fixture/steps added; no production change beyond task 1.

  ```
  1 scenario (1 passed)
  3 steps (3 passed)
  ```
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `e2a6efd`.

### 3. A dependency-only version can be prepared

**Design constraints:** `scripts/prepare-release.sh` unchanged; `DEPENDENCY_ONLY_HISTORY` plus five platform stand-ins; step `prepared_release_holds_every_platform`.

- [x] RED: baseline run failed with undefined steps.
- [x] GREEN: step added; no production change beyond task 1.

  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  ```
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `e2a6efd`.

### 4. The Dependencies group follows Reversals

**Design constraints:** Group order is parser position; the dependency parser is last. `DEPENDENCY_ORDER_HISTORY`: capability fix, `revert(publish-public-release): …`, serde update, clap update; indices 2–3 overridden to `["Cargo.lock"]`; tag `v0.2.0`. Steps `a_history_with_a_reversal_and_dependency_updates`, `fix_precedes_dependency_update`, `reversal_precedes_dependency_update`, `dependency_updates_in_history_order`.

- [x] RED: baseline run failed with undefined steps.
- [x] GREEN: fixture/steps added; no production change beyond task 1.

  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  ```
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `e2a6efd`.

### 5. A dependency update that changes only the project's checks is not listed

**Design constraints:** `DEPENDENCY_CI_HISTORY`; the `chore(deps)` revision is overridden to `[".github/workflows/ci.yml"]`, so the path filter drops it. The renderer prune removes the empty `### Dependencies` heading after collapse and before the visible count. Steps `a_history_with_check_only_dependency_updates`, `notes_name_the_converter_fix`, `notes_name_no_dependency_update`, `notes_name_no_dependencies_group`. Preservation guard whose RED against a parser-only implementation comes from the no-heading assertion.

- [x] RED: baseline run failed with undefined steps.
- [x] GREEN: fixture/steps and the renderer prune added. Production file: `scripts/render-changelog.sh`.

  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  ```
- [x] REFACTOR: the prune is commented with its position and purpose; targeted run stays green.
- [x] COMMIT: revision `e2a6efd`.

### 6. A dependency update that changes the release workflow is listed

**Design constraints:** `DEPENDENCY_RELEASE_WORKFLOW_HISTORY`; the `chore(deps)` revision is overridden to `[".github/workflows/release.yml"]`, a user-receiving path. Steps `a_history_with_a_release_workflow_dependency_update`, `notes_name_the_dependency_update`.

- [x] RED: baseline run failed with undefined steps.
- [x] GREEN: fixture/steps added; no production change beyond tasks 1 and 5.

  ```
  1 scenario (1 passed)
  3 steps (3 passed)
  ```
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `e2a6efd`.

### 7. A version that changes nothing a user can see is refused *(modified, guard)*

**Design constraints:** Reuse the generation and refusal steps; `INVISIBLE_WITH_CI_DEPENDENCY_HISTORY` (CI-only dependency, `chore(ci)`, `docs(readme)`, tag `v0.2.0`); step `a_history_with_no_announceable_entries`. The prune runs before the visible count.

- [x] RED: baseline run failed with undefined steps.
- [x] GREEN: Given added; no further production change.

  ```
  2 features
  2 scenarios (2 passed)
  6 steps (6 passed)
  ```
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `e2a6efd`.

### 8. A dependency update typed with a capability is announced once

**Design constraints:** `DEPENDENCY_CAPABILITY_HISTORY`; the capability fix is overridden to `["crates/duty2cal-core/src/lib.rs", "Cargo.lock"]`, the serde update to `["Cargo.lock"]`. Steps `a_history_with_a_capability_scoped_dependency_update`, `capability_scoped_update_once`, `dependencies_group_holds_only_unscoped_updates`, plus `group_of`.

- [x] RED: baseline run failed with undefined steps.
- [x] GREEN: fixture/steps/helpers added; no production change beyond tasks 1 and 5.

  ```
  1 scenario (1 passed)
  4 steps (4 passed)
  ```
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `e2a6efd`.

### 9. The release text and the change log state the same dependency updates

**Design constraints:** `prepare-release.sh` stages both files from one render; `DEPENDENCY_ONLY_HISTORY` and five platform stand-ins. Step `release_text_names_the_dependency_updates`.

- [x] RED: baseline run failed with undefined steps.
- [x] GREEN: step added; no production change beyond task 1.

  ```
  1 scenario (1 passed)
  5 steps (5 passed)
  ```
- [x] REFACTOR: none; targeted run stays green.
- [x] COMMIT: revision `e2a6efd`.

## Documentation and release task

- [x] Amend the `### Releasing a version` paragraph in `README.md`: the blanket “so it is never released at all” now names the `chore(deps):` exception, the dependency-only releasability, the **Dependencies** group and the checks-only exclusion. Revision `e2a6efd`.
- [x] Confirm the committed `CHANGELOG.md` is still current for the newest tag: `change log is current for version 0.2.0`. The parser and prune did not change the `0.2.0` section or any earlier section.
Archive-time cleanup (not a task, because it cannot be performed before the merge): after the delta replaces the permanent refusal scenario and the archive merges it, remove the now-unused `INVISIBLE_HISTORY` constant and the old `Given("a history whose released revisions name only work no user can see")` step. Before archive both features run together, so the step is still used. Recorded here and in `design.md`.

## Verification task

- [x] `./run-tests.sh`:

  ```
  [Summary]
  3 features
  112 scenarios (112 passed)
  427 steps (427 passed)
  ```
- [x] `./run-tests.sh --e2e`:

  ```
  [Summary]
  2 features
  3 scenarios (3 passed)
  16 steps (16 passed)
  ```
- [x] `cargo fmt --all --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean; `GIVN_FEATURES=$(scripts/assemble-features.sh) cargo test --workspace` exited 0.
- [x] `scripts/render-changelog.sh --version 0.2.0 --repo . --out CHANGELOG.md --check` → `change log is current for version 0.2.0`.
- [x] `givn lint --change announce-dependency-updates` clean; `scripts/pii-guard.sh` clean.
- [x] `./measure-coverage.sh` produced both Cobertura reports; `./merge-coverages.sh` → `merged 7 files, 886 lines (871 covered) -> coverage/cobertura-coverage.xml`.
