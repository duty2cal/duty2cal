# Tasks: list-only-changes-a-user-receives

Release note (commit subject): A version's notes list only changes a user can receive — a released revision that changed none of the paths a user's copy is made of is left out, so the notes never present the project's own engineering work as something a reader can act on.

Owning use case: [Publish a public release](specs/publish-public-release/usecase.md) (`publish-public-release`). Confirmed Personas: `project-maintainer--release-owner` (the Actor who prepares a release), `crew-member--personal-calendar-owner` (the reader the notes promise serves). Actors in scenarios stay distinct from Personas.

Single-scenario run command: `./run-tests.sh --name "<scenario title>"`. Full suite: `./run-tests.sh`. E2E-only: `./run-tests.sh --e2e`.

Use-case constraints carried into every scenario task (from the owning use case's rules):

- A version's notes name the changes a user can see; housekeeping, test, build and ci revisions are never listed, and neither is a revision whose words name the project's own automation.
- A released revision must also have changed a path a user receives (the rule this change adds).
- Each entry states the change in the release-note words the revision carries, never a rewritten summary, and identical subjects collapse to one entry.
- A version whose notes would name no change a user can see is refused rather than published.
- The version being released already has its section in the committed change log before its tag exists; a committed change log that differs from a fresh render is refused.
- The configuration's marked lists cannot drift from what they describe: a capability the repository holds and the allow-list omits, a machinery term the configuration does not skip, or (new) a listed user-receiving path the tree does not hold is refused by name.
- Private duty plans and real identifying values never enter the public history or a public asset; fixture repositories are generated.

Interaction coverage (checked before writing tasks): the use case's inventory holds two entries — "prepare the release for a version" and "generate the change log for a version" — each matched by an `@e2e` scenario and each named in `design.md`'s Interaction Coverage Matrix with the real command as its driving mechanism. This change adds no consumer action, so no new inventory entry, no new `@e2e` scenario and no new matrix row.

## Setup task — environment, runner, strict mode, fixture facts

- [x] Confirm the toolchain, the pinned generator, strict mode and the baseline.
  - The toolchain is the nix-store Rust 1.98.1 with its clippy and a gcc 15.3.0 linker; `git-cliff 2.14.2` (the only version the renderer accepts) is placed on `PATH` for every run.
  - Strict mode is `.fail_on_skipped()` in `apps/cli/tests/features_runner.rs`; the step files exist, one per capability.
  - `givn/commands.yaml` already names `verify.command: ./run-tests.sh` and `verify.e2e_command: ./run-tests.sh --e2e`.
  - Fixture facts the scenarios rest on: `build_history` wrote one file per revision and created only `cliff.toml` and `givn/specs/<capability>/`; the renderer collapsed identical `^- ` lines inside the same `awk` that takes the version's section; the committed `CHANGELOG.md` held the `0.2.0` section this change makes stale.

  Baseline before any change, with the pinned generator on `PATH` — the delta's three scenarios with their steps absent, every permanent scenario green:

  ```
  ./run-tests.sh
  2 features
  100 scenarios (97 passed, 3 failed)
  1 step failed: Step doesn't match any function
  exit=101
  ```

## Scenario tasks

### 1. The path list and the filter

Two scenarios, one pipeline change: the marked list the renderer checks, and the filter that drops an entry whose revision changed none of those paths.

- [x] RED, "A configured path a user receives that the repository does not hold is refused": remove `@wip`; add its `Given` step; run the single-scenario command with the production change reverted; the scenario must fail because the render currently accepts the list.

  ```
  ✘  Then the generation fails and names the path missing from the repository
     Step panicked. Captured output: assertion `left != right` failed: the generation did not
     fail: rendered version 0.2.0 into /tmp/duty2cal-release-1514343-0/CHANGELOG.md
  1 scenario (1 failed)
  ```
- [x] RED, "A version's notes list only the changes a user can receive": same reverted state; the scenario must fail because the machinery-only revision is announced today.

  ```
  ✔  Then the section for version 0.2.0 names the change to the converter
  ✔  And the section for version 0.2.0 names the change that touched both
  ✘  And the section for version 0.2.0 names no change to the release machinery
     Step panicked. Captured output: the section names a revision that changed only the
     machinery: ## [0.2.0] - 2026-01-03
  1 scenario (1 failed)
  ```
- [x] GREEN: the marked `user-receiving-paths` line in `cliff.toml`; the renderer reads it, refuses a listed path the tree does not hold, takes the version's section without collapsing, strips each entry's revision marker, drops the entries that changed none of the listed paths, and only then collapses identical subjects; the fixture builder creates every listed path in a scaffold revision and gives each revision the paths its subject implies, with an override for a history that needs the opposite. Production files: `cliff.toml`, `scripts/render-changelog.sh`, `apps/cli/tests/steps/publish_public_release.rs`. Both scenarios, then the whole suite:

  ```
  ./run-tests.sh --name "A configured path a user receives that the repository does not hold is refused"
  1 scenario (1 passed) / 3 steps (3 passed)
  ./run-tests.sh --name "A version's notes list only the changes a user can receive"
  1 scenario (1 passed) / 5 steps (5 passed)
  ./run-tests.sh
  3 features
  103 scenarios (103 passed)
  390 steps (390 passed)
  ```
- [x] REFACTOR: the renderer's pipeline comment states why the section is taken without collapsing, and the fixture builder's comment states what a revision's own subject decides. The whole suite stays green (above).
- [x] COMMIT: `fix(publish-public-release): <release note>` with both scenario titles in the body; record the revision identifier.

### 2. A version's notes name a change to the packaging a user receives *(guard)*

- [x] RED: remove `@wip`; add its `Given` step; run the single-scenario command with the step file stashed, so the step is undefined and strict mode fails the run.

  ```
  ✘  Given a history whose released revisions changed the packaging
     Step doesn't match any function
  1 scenario (1 failed)
  ```
- [x] GREEN: no production change is required — `scripts/prepare-release.sh` is on the marked list, which is the boundary the operator's accepted cost depends on (a change to what an archive holds stays announceable). Record that finding and run the single-scenario command.

  ```
  ./run-tests.sh --name "A version's notes name a change to the packaging a user receives"
  1 scenario (1 passed) / 3 steps (3 passed)
  ```
- [x] REFACTOR: none; the whole suite is green (103 scenarios).
- [x] COMMIT: `test(publish-public-release): <release note>` with this scenario's title in the body; record the revision identifier.

## Documentation and release task

- [x] State the authorship rule in the release runbook (`README.md`, "Releasing a version"): a released subject says what a crew member can act on, and engineering work takes a housekeeping type so it is never released. The use case's rule and example are already edited in the design phase.
- [x] Re-render the committed change log for the untagged version: `scripts/render-changelog.sh --version 0.2.0 --repo . --out CHANGELOG.md`, and confirm the `0.2.0` section now holds the travel-title fix and neither engineering entry, that every earlier section is byte-identical, and that `--check` is clean.

  ```
  rendered version 0.2.0 into CHANGELOG.md
  ## [0.2.0] - 2026-10-14

  ### Fixes
  - A travel activity's calendar title prints the crew position and the flight number as two words separated by a space — an operating leg reads `FB 123 ABC-DEF` instead of `FB123 ABC-DEF`, while positioning travel keeps reading `DH 007 ABC-DEF`.
  [0.1.0...0.2.0](https://github.com/kressnerd/duty2cal/compare/v0.1.0...v0.2.0)
  change log is current for version 0.2.0
  ```

  The diff is two deleted lines — the two engineering entries — and nothing else; the commit carries the released revision's own committer date so the tagged revision will state the date the section states.
- [x] Record the revision identifier: revision `d569697` (`docs(publish-public-release): state the authorship rule in the runbook and re-render the untagged version's section.`).

## Verification task

- [x] `./run-tests.sh` → every scenario passes; paste the summary.

  ```
  [Summary]
  3 features
  103 scenarios (103 passed)
  390 steps (390 passed)
  ```
- [x] `./run-tests.sh --e2e` → every `@e2e` scenario passes; paste the summary.

  ```
  [Summary]
  2 features
  3 scenarios (3 passed)
  16 steps (16 passed)
  ```
- [x] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `GIVN_FEATURES=$(scripts/assemble-features.sh) cargo test --workspace` → clean; paste.

  ```
  cargo fmt --all --check            -> clean
  cargo clippy --workspace --all-targets -- -D warnings
    Checking duty2cal v0.1.0 … Finished `dev` profile … in 2.55s
  GIVN_FEATURES=$(scripts/assemble-features.sh) cargo test --workspace
    3 features / 103 scenarios (103 passed)
    running 3 tests / test result: ok. 3 passed
  ```

  The first `cargo fmt --all --check` failed on the new step's attribute line; `cargo fmt --all` fixed it and the check is clean.
- [x] `scripts/render-changelog.sh --version 0.2.0 --repo . --out CHANGELOG.md --check` → `change log is current for version 0.2.0`; paste. (Pasted above, re-run after the change-log commit.)
- [x] `givn lint --change list-only-changes-a-user-receives`, `scripts/pii-guard.sh` → clean; paste.

  ```
  givn lint: 1 file(s) checked — clean
  pii-guard: clean
  ```
