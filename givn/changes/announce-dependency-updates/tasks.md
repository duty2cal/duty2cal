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

Baseline before implementation (the delta's nine scenarios with their steps absent; every permanent scenario green; strict mode rejects the undefined steps):

```
[Summary]
3 features
112 scenarios (103 passed, 9 failed)
399 steps (390 passed, 9 failed)
```

## Setup task — toolchain, pinned generator, strict mode, fixture facts

- [ ] Confirm the toolchain, the pinned generator, strict mode and the fixture baseline.
  - Toolchain: `nix-shell -p cargo` provides `cargo 1.98.0`; `nix-shell -p git-cliff` provides `git-cliff 2.14.2`, the only version the renderer accepts. Every scenario run must have both on `PATH`.
  - `givn/commands.yaml` already names `verify.command: ./run-tests.sh` and `verify.e2e_command: ./run-tests.sh --e2e`.
  - Strict mode is `.fail_on_skipped()` in `apps/cli/tests/features_runner.rs`; the baseline above proves an undefined step fails the process. Record the exact command and the non-zero exit in this task.
  - Fixture facts: `build_history` delegates to `build_history_with`, creates every `RECEIVED_PATHS` entry (including `Cargo.lock` and `.github/workflows/release.yml`), maps every `chore(…)` subject to `MACHINERY` by default, and accepts explicit per-revision path overrides. Every dependency fixture must override its `chore(deps)` revisions to `["Cargo.lock"]` (or the CI/release-workflow path the scenario names); otherwise the path filter drops the entry and the scenario tests the wrong thing.
  - The delta's nine scenarios are `@wip` and fail only because their steps do not exist. Remove `@wip` from one scenario at a time, per task.

## Scenario tasks

### 1. A version whose only changes are dependency updates is announced

**Design constraints:** Add the exact parser row `{ message = "^chore\\(deps\\):", group = "<!-- 5 -->Dependencies" }` after the Reversals parsers and before the catch-all; keep the machinery skip parsers first. The path filter already keeps a `Cargo.lock` change. Fixture `DEPENDENCY_ONLY_HISTORY`: two `chore(deps)` revisions with distinct subjects, plus `chore(ci)` and `docs(readme)`, both dependency revisions overridden to `["Cargo.lock"]`, tag `v0.2.0` on the last revision. Step file `apps/cli/tests/steps/publish_public_release.rs`; define `a_history_of_dependency_updates`, `notes_name_the_dependency_updates` (both update texts present), and `notes_name_no_housekeeping_revision` (neither `chore(ci)` nor `docs(readme)` subject appears).

- [ ] RED: remove `@wip` from this scenario; the new steps are absent/stubbed; run `./run-tests.sh --name "A version whose only changes are dependency updates is announced"`. Paste the non-zero output.
- [ ] GREEN: add the parser row and the fixture/steps. Production files: `cliff.toml`, `apps/cli/tests/steps/publish_public_release.rs`. Run the single-scenario command; paste the zero-exit output.
- [ ] REFACTOR: keep the parser comments accurate (the catch-all is no longer every non-capability revision) and the fixture override comment explicit. Re-run the single scenario; paste the zero-exit output.
- [ ] COMMIT: `feat(publish-public-release): A version whose only changes are dependency updates can now be published, with those updates named in its release notes.` Scenario title in the body. Record the revision identifier.

### 2. Identical dependency update subjects collapse to one entry

**Design constraints:** The existing collapse (`awk '/^- / { if (seen[$0]++) next }'`) runs after the path filter. Fixture `DEPENDENCY_COLLAPSE_HISTORY`: two revisions carrying exactly `chore(deps): bump serde from 1.0.0 to 1.0.1`, plus `chore(ci)`, both dependency revisions overridden to `["Cargo.lock"]`, tag `v0.2.0` on the last revision. Define `a_history_with_repeated_dependency_subjects` and `dependency_update_named_once` (count bullets containing "bump serde" = 1).

- [ ] RED: remove `@wip`; run the single-scenario command with the steps stubbed; paste the non-zero output.
- [ ] GREEN: add the fixture/steps; no production change beyond task 1. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none expected; re-run and paste.
- [ ] COMMIT: `test(publish-public-release): A version whose only changes are dependency updates can now be published, with those updates named in its release notes.` Scenario title in the body. Record the revision identifier.

### 3. A dependency-only version can be prepared

**Design constraints:** `scripts/prepare-release.sh` is unchanged: it calls the renderer and stages the five archives, checksums, `CHANGELOG.md` and `release-notes.md`. The scenario needs the five platform stand-ins from `an executable built for every supported platform` and `DEPENDENCY_ONLY_HISTORY`; define `prepared_release_holds_every_platform` (exit zero and one archive per `SUPPORTED` platform).

- [ ] RED: remove `@wip`; run the single-scenario command with the step stubbed; paste the non-zero output.
- [ ] GREEN: add the step; no production change beyond task 1. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none expected; re-run and paste.
- [ ] COMMIT: `test(publish-public-release): A version whose only changes are dependency updates can now be published, with those updates named in its release notes.` Record the revision identifier.

### 4. The Dependencies group follows Reversals

**Design constraints:** Group order is parser position: the dependency parser sits after the Reversals parsers, so Dependencies renders last. Fixture `DEPENDENCY_ORDER_HISTORY`: a capability-scoped fix, a `revert(publish-public-release): …`, and two `chore(deps)` revisions, dependency revisions overridden to `["Cargo.lock"]`, tag `v0.2.0` on the last revision. The `revert` preprocessor rewrites the subject to `xrevert`; no fixture machinery change. Define `a_history_with_a_reversal_and_dependency_updates`, `fix_precedes_dependency_update`, `reversal_precedes_dependency_update`, and `dependency_updates_in_history_order`.

- [ ] RED: remove `@wip`; run the single-scenario command with the steps stubbed; paste the non-zero output.
- [ ] GREEN: add the fixture/steps; no production change beyond task 1. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none expected; re-run and paste.
- [ ] COMMIT: `test(publish-public-release): A version whose only changes are dependency updates can now be published, with those updates named in its release notes.` Record the revision identifier.

### 5. A dependency update that changes only the project's checks is not listed

**Design constraints:** Fixture `DEPENDENCY_CI_HISTORY`: a converter fix and a `chore(deps): bump actions/checkout …` revision overridden to `[".github/workflows/ci.yml"]`, plus `chore(ci)`, tag `v0.2.0`. The path filter drops the dependency bullet; the renderer's new empty-heading prune removes the `### Dependencies` heading. Define `a_history_with_check_only_dependency_updates`, `notes_name_the_converter_fix`, `notes_name_no_dependency_update`, and `notes_name_no_dependencies_group` (no `### Dependencies` heading in the written section). This is a preservation guard: it passes before the parser exists; its RED against a parser-only implementation comes from the no-heading assertion.

- [ ] RED: remove `@wip`; run the single-scenario command with the steps stubbed; paste the non-zero output.
- [ ] GREEN: add the fixture/steps and the renderer prune in `scripts/render-changelog.sh`: after collapse and before the visible-count check, remove a `### ` heading whose next non-blank line is another heading or the section end. Production files: `scripts/render-changelog.sh`, `apps/cli/tests/steps/publish_public_release.rs`. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: comment the prune with why it runs before the visible count. Re-run and paste.
- [ ] COMMIT: `fix(publish-public-release): A version whose only changes are dependency updates can now be published, with those updates named in its release notes.` Record the revision identifier.

### 6. A dependency update that changes the release workflow is listed

**Design constraints:** Fixture `DEPENDENCY_RELEASE_WORKFLOW_HISTORY`: `chore(deps): bump actions/upload-artifact …` overridden to `[".github/workflows/release.yml"]`, plus `chore(ci)`, tag `v0.2.0`. `.github/workflows/release.yml` is a user-receiving path, so the bullet survives. Define `a_history_with_a_release_workflow_dependency_update` and `notes_name_the_dependency_update`.

- [ ] RED: remove `@wip`; run the single-scenario command with the steps stubbed; paste the non-zero output.
- [ ] GREEN: add the fixture/steps; no production change beyond tasks 1 and 5. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none expected; re-run and paste.
- [ ] COMMIT: `test(publish-public-release): A version whose only changes are dependency updates can now be published, with those updates named in its release notes.` Record the revision identifier.

### 7. A version that changes nothing a user can see is refused *(modified, guard)*

**Design constraints:** Reuse the existing generation and refusal steps; the scenario replaces the permanent title and uses `INVISIBLE_WITH_CI_DEPENDENCY_HISTORY` (a CI-only dependency bump, `chore(ci)` and `docs(readme)`, tag `v0.2.0`). Define `a_history_with_no_announceable_entries`. The refusal is reached only when no capability entry and no dependency entry survives; the empty-heading prune runs first.

- [ ] RED: remove `@wip`; run the single-scenario command with the Given stubbed; paste the non-zero output.
- [ ] GREEN: add the Given; no further production change. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none expected; re-run and paste.
- [ ] COMMIT: `test(publish-public-release): A version whose only changes are dependency updates can now be published, with those updates named in its release notes.` Record the revision identifier.

### 8. A dependency update typed with a capability is announced once

**Design constraints:** Fixture `DEPENDENCY_CAPABILITY_HISTORY`: `fix(export-duty-plan-calendar): Upgrade the PDF parser` overridden to `["crates/duty2cal-core/src/lib.rs", "Cargo.lock"]`, a `chore(deps): bump serde …` overridden to `["Cargo.lock"]`, and `chore(ci)`, tag `v0.2.0`. The capability parser runs first, so the fix stays in Fixes; Dependencies holds only the unscoped serde update. Define `a_history_with_a_capability_scoped_dependency_update`, `capability_scoped_update_once` (exactly one bullet contains "Upgrade the PDF parser"), and `dependencies_group_holds_only_unscoped_updates` (group contains serde, not the parser upgrade).

- [ ] RED: remove `@wip`; run the single-scenario command with the steps stubbed; paste the non-zero output.
- [ ] GREEN: add the fixture/steps and the `group_of`/`position_of` helpers; no production change beyond tasks 1 and 5. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none expected; re-run and paste.
- [ ] COMMIT: `test(publish-public-release): A version whose only changes are dependency updates can now be published, with those updates named in its release notes.` Record the revision identifier.

### 9. The release text and the change log state the same dependency updates

**Design constraints:** `prepare-release.sh` stages `CHANGELOG.md` and `release-notes.md` from one render; the dependency-only fixture must carry the tag and the five platform stand-ins. Define `release_text_names_the_dependency_updates`; reuse `the staged change log holds the section the release text states`.

- [ ] RED: remove `@wip`; run the single-scenario command with the step stubbed; paste the non-zero output.
- [ ] GREEN: add the step; no production change beyond task 1. Run the single scenario; paste the zero-exit output.
- [ ] REFACTOR: none expected; re-run and paste.
- [ ] COMMIT: `test(publish-public-release): A version whose only changes are dependency updates can now be published, with those updates named in its release notes.` Record the revision identifier.

## Documentation and release task

- [ ] Amend the `### Releasing a version` paragraph in `README.md`: the blanket “so it is never released at all” gains the `chore(deps):` exception; state that a dependency-only version is releasable, that its updates appear under **Dependencies**, and that a bump that changed only the checks stays unannounced.
- [ ] Confirm the committed `CHANGELOG.md` is still current for the newest tag (`scripts/render-changelog.sh --version 0.2.0 --repo . --out CHANGELOG.md --check`); the parser and prune must not change the `0.2.0` section or any earlier section.
- [ ] Remove the now-unused `INVISIBLE_HISTORY` constant and the old `Given("a history whose released revisions name only work no user can see")` step after the delta merges at archive (the delta replaces the permanent scenario). Until archive, both features run together, so keep them.
- [ ] Record the revision identifiers for the documentation commit(s).

## Verification task

- [ ] `./run-tests.sh` → every scenario passes; paste the summary.
- [ ] `./run-tests.sh --e2e` → every `@e2e` scenario passes; paste the summary.
- [ ] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `GIVN_FEATURES=$(scripts/assemble-features.sh) cargo test --workspace` → clean; paste.
- [ ] `scripts/render-changelog.sh --version 0.2.0 --repo . --out CHANGELOG.md --check` → `change log is current for version 0.2.0`; paste.
- [ ] `givn lint --change announce-dependency-updates` → clean after `@wip` removal; `scripts/pii-guard.sh` → clean; paste.
- [ ] `./measure-coverage.sh` and `./merge-coverages.sh` → the merged report is non-zero and the new step code is exercised; paste the summary.
