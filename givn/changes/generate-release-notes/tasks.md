# Tasks: generate-release-notes

Release note (commit subject): Record what each released version of the tool changed in a change log a reader can scan, group the released changes by kind, and refuse to publish a version whose notes say nothing a user can see.

Owning use case: [Publish a public release](specs/publish-public-release/usecase.md) (`publish-public-release`). Confirmed Personas: `crew-member--personal-calendar-owner` (reader lens: whether the notes let them tell, from the notes alone, whether a version contains something they want), `project-maintainer--release-owner` (author lens: whether a release can be prepared and published without hand-editing any text, and whether the page and the repository can ever disagree). Actors in scenarios stay distinct from Personas.

Single-scenario run command (used by every RED/GREEN/REFACTOR below): `./run-tests.sh --name "<scenario title>"`.
Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`.

Use-case constraints carried into every scenario task: the section is rendered from the repository's history by the project's own generator, whose version is recorded beside its configuration (`cliff.toml`), and `scripts/render-changelog.sh` refuses a generator whose version differs; the capability allow-list is the use-case ids under `givn/specs/`, repeated per type in `cliff.toml`, and a capability the repository holds and the configuration leaves out is refused; a section is dated with the released revision's own committer date and, where an earlier version exists, carries the literal comparison `https://github.com/kressnerd/duty2cal/compare/<previous>...<this>`, none for the first version; identical release-note subjects collapse to one entry; a version whose notes would name no visible change is refused; a render replaces its own version's section and prepends a version not yet present, keeping every earlier section verbatim; and, under `--check`, a committed change log missing the released version's section, or differing from a fresh render, is refused.

## Setup task — generator pin, configuration, fixtures

- [x] Record the pinned generator where the design says: `cliff.toml` at the repository root carries the pinned version on the line `# git-cliff version: 2.14.2` and the capability allow-list on the line marked `<!-- capability-allow-list: … -->`, which `scripts/render-changelog.sh` reads. Both are the design's own mechanism, so no second pin file is invented.
  ```
  $ grep -n 'git-cliff version\|capability-allow-list' cliff.toml
  13:# git-cliff version: 2.14.2
  47:# <!-- capability-allow-list: export-duty-plan-calendar publish-public-release -->
  ```
  Recorded in `cliff.toml` (committed with scenario 1, `b6dc609`); the renderer reads both lines with `sed`/`grep` and refuses when either is absent, when the running `git-cliff` differs from the recorded version, or when the allow-list differs from the use-case ids under `givn/specs/`. No second pin file exists.
- [x] Confirm the environment: `git-cliff` 2.14.2 through `nix-shell -p git-cliff` (crates.io `max_version` and nixpkgs agree), Rust through `nix-shell -p rustc cargo`, and that the suite runs the renderer as a real subprocess with no network, no credentials and no GitHub. Record the pinned version and the confirmed suite counts here.
  ```
  $ nix-shell -p git-cliff --run 'git-cliff --version'
  git-cliff 2.14.2
  $ nix-shell -p rustc cargo git-cliff --run './run-tests.sh'
  3 features
  95 scenarios (95 passed)
  358 steps (358 passed)
  $ nix-shell -p rustc cargo git-cliff --run './run-tests.sh --e2e'
  3 features
  3 scenarios (3 passed)
  16 steps (16 passed)
  ```
  Pinned version: **git-cliff 2.14.2**, which nixpkgs supplies and which the renderer's pin refusal accepts against the line in `cliff.toml`. Rust comes from `nix-shell -p rustc cargo`. The suite drives `scripts/render-changelog.sh` and `scripts/prepare-release.sh` as real subprocesses against fixture repositories built under `/tmp`, so a run touches only the filesystem, the pinned binary and local git — no network, no credentials, no GitHub; the `@e2e` render scenario's transcript is the committed proof. The design resolved crates.io `max_version` 2.14.2 on 2026-10-13; re-checking that here would need the network this environment does not carry, so it is recorded as the design's resolution and confirmed locally against nixpkgs.
- [x] Add the `DutyWorld` fields the new steps need (fixture repository, change log path, command transcript as the render scenario's evidence) in `apps/cli/tests/features_runner.rs`; the step files already declared in `apps/cli/tests/steps/mod.rs` and `apps/cli/tests/steps/e2e/mod.rs` are extended, never replaced (one step-definition file per capability).
  ```
  $ grep -n 'changelog' apps/cli/tests/features_runner.rs
  47:    pub changelog: Option<PathBuf>,
  $ git diff --stat b6dc609^..HEAD -- apps/cli/tests/steps/mod.rs apps/cli/tests/steps/e2e/mod.rs
  (no output)
  ```
  `DutyWorld` gained the `changelog` field (the change log a scenario renders) with the other fields the new steps need already present — `fixture_repo` (the history the renderer reads), `command_transcript` and `scenario_transcript` (the render scenario's evidence). Added in scenario 1's commit `b6dc609`. Both step files are extended in place and neither `mod.rs` changed, which is the one-file-per-capability rule.
- [x] Build the fixture repositories inside the steps: `git init`, commits with fixed `GIT_COMMITTER_DATE`/`--date` values and conventional subjects, `v*` tags, and a copy of the committed `cliff.toml` plus the capability directories under `givn/specs/` that the allow-list check reads. No real repository and no clock are involved.
  ```
  $ grep -n 'fn git_dated\|fn build_history\|^fn git(' apps/cli/tests/steps/publish_public_release.rs
  317:fn git_dated(repo: &Path, date: &str, args: &[&str]) {
  337:pub(crate) fn build_history(w: &mut DutyWorld, revisions: &[(&str, &str, &str)]) {
  400:fn git(repo: &Path, args: &[&str]) {
  ```
  `build_history` copies the committed `cliff.toml` and creates the capability directories under `givn/specs/`, then `git init`s and commits every revision through `git_dated`, which sets both `GIT_AUTHOR_DATE` and `GIT_COMMITTER_DATE` to the fixture's fixed date, and tags the revisions the fixture marks with their `v*` tag. No real repository and no clock are involved: every render scenario asserts a heading date that comes from a fixed committer date, and the drift check passes against the real checkout only because the real history's dates are already fixed in it.

## Interaction coverage (checked before writing scenarios)

| Inventory entry (usecase.md) | Matrix row (design.md) | `@e2e` scenario in the delta | Driver |
|---|---|---|---|
| generate the change log for a version | present, one row | `Maintainer generates the change log for a version and reads its newest section` | real subprocess: `scripts/render-changelog.sh --version 0.1.0 --repo <fixture repo> --out <path>` |
| prepare the release for a version | present, one row | `Maintainer prepares a release and runs the archived executable` (permanent) | real subprocess: `scripts/prepare-release.sh`, then the executable extracted from the produced archive |

Every inventory entry has a matrix row, the row names a real subprocess driver, and the `@e2e` title matches the feature file exactly. No `@e2e` scenario exists that the inventory does not name.

## Scenarios (non-@e2e first, in feature order)

Each scenario's production code is `scripts/render-changelog.sh` (new) and, for the two preparation scenarios, `scripts/prepare-release.sh` (changed); `cliff.toml` (new) supplies the grouping, the skip rules and the allow-list. No Rust production code changes in this change — the only Rust added is step definitions.

### 1. A version's notes group the changes a reader can see

Use-case constraint: a new capability groups above a fix; work no user can see is listed nowhere.

- [x] RED — remove `@wip` from this scenario only; write the step definitions with `unimplemented!(...)` bodies; run the single-scenario command; paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: A version's notes group the changes a reader can see
     ✘  Given a history whose released revisions name a new capability, a fix, and work no user can see
        Step panicked. Captured output: not implemented: build a history naming a new capability, a fix, and work no user can see
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
  Step file extended in RED: `apps/cli/tests/steps/publish_public_release.rs` (the fixture helpers `git_dated`, `build_history`, `change_log_path`, `run_render` and the five stubs); `apps/cli/tests/features_runner.rs` gained the `changelog` field the render scenarios carry.
- [x] GREEN — the renderer, the configuration and the fixture; run the single-scenario command; paste the passing output. List every production file touched:
  ```
  Feature: Publish a public release
    Scenario: A version's notes group the changes a reader can see
     ✔  Given a history whose released revisions name a new capability, a fix, and work no user can see
     ✔  When the maintainer generates the change log for version 0.2.0
     ✔  Then the notes for version 0.2.0 name the new capability and the fix
     ✔  And the notes name the new capability before the fix
     ✔  And the notes name no revision whose work no user can see
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  GREEN_EXIT=0
  ```
  Production files touched: `scripts/render-changelog.sh` (new — the argument contract, the pinned-version refusal, the capability allow-list against `givn/specs/`, the range and heading date, the collapse of identical subjects, the replace-or-prepend write, the visibility refusal and `--check`), `cliff.toml` (new — the per-type allow-list, the group order, the skip rules and the section template), `CHANGELOG.md` (new, generated). The steps build the fixture history (`git init`, six revisions with fixed committer dates and conventional subjects, `git tag v0.2.0`) and the committed `cliff.toml` plus the two capability directories it copies, then drive `scripts/render-changelog.sh` as a real subprocess. The renderer is invoked through the pinned tool, so the run command is `nix-shell -p rustc cargo --run "nix-shell -p git-cliff --run './run-tests.sh --name \"<title>\"'"`; `./run-tests.sh` alone leaves `git-cliff` off `PATH` and the renderer refuses with its own message.
- [x] REFACTOR — tidy without changing behaviour; run again; paste the output:
  ```
  rustfmt --edition 2021 --check apps/cli/tests/steps/publish_public_release.rs
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 5 steps (5 passed)   REFACTOR_EXIT=0
  ```
  No production change. The `git` helper was left for the permanent scenario's un-dated history; the new fixture path uses `git_dated`, so the two do not share a body.
- [x] COMMIT: `b6dc609` — subject is the release note, body names this scenario:
  ```
  feat(generate-release-notes): Record what each released version of the tool changed in a change log a reader can scan, group the released changes by kind, and refuse to publish a version whose notes say nothing a user can see.

  Scenario: A version's notes group the changes a reader can see

  RED: the five steps existed as unimplemented!() stubs; the single-scenario run failed with exit 101 under strict mode. The step file gained the fixture helpers (git_dated, build_history, change_log_path, run_render) and the world gained the changelog field.
  GREEN: scripts/render-changelog.sh renders one version from the history with the pinned git-cliff 2.14.2 and the committed cliff.toml, checking the pin and the capability allow-list against givn/specs before it renders; cliff.toml carries the per-type allow-list, the group order, and the skip rules, and the renderer collapses identical release-note subjects. The steps build the fixture history with fixed committer dates and run the renderer as a real subprocess.
  REFACTOR: the step file is rustfmt-clean and the single scenario passes 5/5 steps with exit 0.
  ```

### 2. A version that changes nothing a user can see is refused

Use-case constraint: a version whose notes would name no change a user can see is refused rather than published.

- [x] RED — remove `@wip` from this scenario only; write the steps with `unimplemented!(...)` bodies; paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: A version that changes nothing a user can see is refused
     ✘  Given a history whose released revisions name only work no user can see
        Step panicked. Captured output: not implemented: build a history naming only work no user can see
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — the empty-section refusal; paste the passing output. List every production file touched:
  ```
  Feature: Publish a public release
    Scenario: A version that changes nothing a user can see is refused
     ✔  Given a history whose released revisions name only work no user can see
     ✔  When the maintainer generates the change log for version 0.2.0
     ✔  Then the generation fails and states that the version changes nothing a user can see
  [Summary]
  1 feature
  1 scenario (1 passed)
  3 steps (3 passed)
  GREEN_EXIT=0
  ```
  Production files touched: none new — the empty-section refusal in `scripts/render-changelog.sh` (a render whose section names no `- ` entry) already covers this; the steps build a history of `chore`, `docs` and `build` subjects tagged `v0.2.0` and assert the refusal names the version and that no output was written.
- [x] REFACTOR — paste the output:
  ```
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 3 steps (3 passed)   REFACTOR_EXIT=0
  ```
  No tidy needed: the two steps are one fixture builder and one assertion, and the file is rustfmt-clean.
- [x] COMMIT: `53e0cbc` — subject is the release note, body names this scenario:
  ```
  feat(generate-release-notes): Record what each released version of the tool changed in a change log a reader can scan, group the released changes by kind, and refuse to publish a version whose notes say nothing a user can see.

  Scenario: A version that changes nothing a user can see is refused

  RED: the two steps existed as unimplemented!() stubs; the single-scenario run failed with exit 101.
  GREEN: no production code new — scripts/render-changelog.sh already refuses a render that names no change a user can see (an empty visible-change set), so this scenario pins the refusal: a history of chore, docs and build subjects tagged v0.2.0 fails with "version 0.2.0 changes nothing a user can see" and writes nothing.
  REFACTOR: no tidy needed; the step file is rustfmt-clean and the single scenario passes 3/3 steps with exit 0.
  ```

### 3. The change log keeps the earlier versions beside the new one

Use-case constraint: a render replaces its own version's section and places a new version's section above the earlier ones, which stay verbatim.

- [x] RED — remove `@wip` from this scenario only; write the steps with `unimplemented!(...)` bodies; paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: The change log keeps the earlier versions beside the new one
     ✘  Given a change log whose newest section is version 0.1.0
        Step panicked. Captured output: not implemented: build a change log whose newest section is version 0.1.0
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — the replace-or-prepend write; paste the passing output. List every production file touched:
  ```
  Feature: Publish a public release
    Scenario: The change log keeps the earlier versions beside the new one
     ✔  Given a change log whose newest section is version 0.1.0
     ✔  When the maintainer generates the change log for version 0.2.0
     ✔  Then version 0.2.0 is the newest section of the change log
     ✔  And the section for version 0.1.0 is unchanged
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
  Production files touched: `scripts/render-changelog.sh` (the `awk` write: a render whose version already has a section replaces it in place and one not yet present is prepended, earlier sections verbatim).
- [x] REFACTOR — paste the output:
  ```
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
- [x] COMMIT: `8b83037` — subject is the release note, body names this scenario:
  ```
  feat(generate-release-notes): Record what each released version of the tool changed in a change log a reader can scan, group the released changes by kind, and refuse to publish a version whose notes say nothing a user can see.

  Scenario: The change log keeps the earlier versions beside the new one

  RED: the three steps existed as unimplemented!() stubs; the single-scenario run failed with exit 101.
  GREEN: the renderer replaces the section that already carries the released version and prepends a version not yet present, keeping every earlier section verbatim. The fixture builds two tagged releases with fixed committer dates, each tag on its own release commit, so the earlier section is unchanged and each version has one section.
  DEVIATION: the design prescribes the range <previous v* tag>..<released revision> and never --tag. git-cliff 2.14.2 mis-groups a render carried by --tag under the newest tag when a later tag exists (reproduced on the fixtures and the real tree), and a positional range shares that flaw; the renderer renders the whole history once and takes the released version's section from it, stamping the heading from the released revision's own commit date. The observable contract is unchanged and the production case renders exactly its range. Recorded above.
  REFACTOR: the step file is rustfmt-clean and the single scenario passes 4/4 steps with exit 0.
  ```

  **Deviation recorded (design.md "Range").** The design says the render "never passes `--tag`" and expresses the released range as `<previous v* tag>..<released revision>`. Rendering with `--tag` was reproduced to mis-group a whole history under the newest tag when a later tag exists — the fixtures and the real tree both showed a release's own section replaced by the newest release's. Passing the range to the generator positionally (or `--current`) shares that flaw. The renderer therefore renders the whole history once with the committed configuration and takes the released version's section from it, stamping the heading from the released revision's own commit date. The observable contract the design fixes is unchanged: the section holds exactly the released version's released changes, dated from its own revision, and the case that matters in production — the version being released, which is the newest — renders exactly its range. This is the design correction the grilling's F4 branch was aimed at; the design was wrong in the form it prescribed, so it is recorded here rather than improvised.

### 4. A change log that does not match the released revision is refused

Use-case constraint: a committed change log differing from a fresh render of the released revision is refused, naming the version whose section differs.

- [x] RED — remove `@wip` from this scenario only; write the steps with `unimplemented!(...)` bodies; paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: A change log that does not match the released revision is refused
     ✘  Given a committed change log whose newest section differs from what the history states
        Step panicked. Captured output: not implemented: write a committed change log whose newest section differs from the history
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — `--check`'s render-and-compare; paste the passing output. List every production file touched:
  ```
  Feature: Publish a public release
    Scenario: A change log that does not match the released revision is refused
     ✔  Given a committed change log whose newest section differs from what the history states
     ✔  When the change log is checked against the released revision
     ✔  Then the check fails and names the version whose section differs
  [Summary]
  1 feature
  1 scenario (1 passed)
  3 steps (3 passed)
  GREEN_EXIT=0
  ```
  Production files touched: `scripts/render-changelog.sh` (the `--check` mode: render in memory, `diff` the whole file against the committed change log, refuse naming the version).
- [x] REFACTOR — paste the output:
  ```
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 3 steps (3 passed)   REFACTOR_EXIT=0
  ```
  No tidy needed: the check step shares the renderer invocation helper and the file is rustfmt-clean.
- [x] COMMIT: `c9437ad` — subject is the release note, body names this scenario:
  ```
  feat(generate-release-notes): Record what each released version of the tool changed in a change log a reader can scan, group the released changes by kind, and refuse to publish a version whose notes say nothing a user can see.

  Scenario: A change log that does not match the released revision is refused

  RED: the three steps existed as unimplemented!() stubs; the single-scenario run failed with exit 101.
  GREEN: the renderer check mode renders in memory and compares the rendered whole file against the committed change log, writing nothing; the steps commit a change log whose newest section states a different note and assert the check fails naming version 0.2.0.
  REFACTOR: the step file is rustfmt-clean and the single scenario passes 3/3 steps with exit 0.
  ```

### 5. Each version's section is dated and compared with the one before it

Use-case constraint: a section states its own date (the released revision's own committer date) and the comparison against the immediately preceding version.

- [x] RED — remove `@wip` from this scenario only; write the steps with `unimplemented!(...)` bodies; paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: Each version's section is dated and compared with the one before it
     ✘  Given a change log holding version 0.1.0 and a history releasing version 0.2.0
        Step panicked. Captured output: not implemented: build a change log holding version 0.1.0 and a history releasing version 0.2.0
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — the date stamp and the literal comparison link; paste the passing output. List every production file touched:
  ```
  Feature: Publish a public release
    Scenario: Each version's section is dated and compared with the one before it
     ✔  Given a change log holding version 0.1.0 and a history releasing version 0.2.0
     ✔  When the maintainer generates the change log for version 0.2.0
     ✔  Then the section for version 0.2.0 states its own date
     ✔  And the section for version 0.2.0 states the comparison against version 0.1.0
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
  Production files touched: none new — `scripts/render-changelog.sh` already carries both mechanisms this scenario pins: the heading date is stamped from the released revision's own committer date (`git log -1 --format=%cd --date=short "$released"`, substituted for the template's `@@DATE@@`), and the literal comparison link is appended to a non-first section as `https://github.com/kressnerd/duty2cal/compare/$previous...v$version` with the remote integration never enabled. The steps build the two-release fixture with fixed committer dates, render version 0.1.0 into the change log and then render version 0.2.0, and assert the 0.2.0 heading states `2026-02-02` and the section states the `v0.1.0...v0.2.0` comparison URL.
- [x] REFACTOR — paste the output:
  ```
  rustfmt --edition 2021 --check apps/cli/tests/steps/publish_public_release.rs
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
  No tidy needed: the three steps are two fixture lines and two assertions on the rendered section, and the file is rustfmt-clean.
- [x] COMMIT: `663d25d` — subject is the release note, body names this scenario.

### 6. The first released version is recorded from the history

Use-case constraint: the first version of all renders the whole history it contains, one section, no comparison against an earlier version.

- [x] RED — remove `@wip` from this scenario only; write the steps with `unimplemented!(...)` bodies; paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: The first released version is recorded from the history
     ✘  Given a history whose released revisions name a new capability and work no user can see
        Step panicked. Captured output: not implemented: build a history naming a new capability and work no user can see
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — the first-release range and the heading date; paste the passing output. List every production file touched:
  ```
  Feature: Publish a public release
    Scenario: The first released version is recorded from the history
     ✔  Given a history whose released revisions name a new capability and work no user can see
     ✔  And no change log exists yet
     ✔  When the maintainer generates the change log for version 0.1.0
     ✔  Then the change log holds one section, for version 0.1.0
     ✔  And that section names the new capability
     ✔  And that section states no comparison against an earlier version
  [Summary]
  1 feature
  1 scenario (1 passed)
  6 steps (6 passed)
  GREEN_EXIT=0
  ```
  Production files touched: none new — `scripts/render-changelog.sh` already renders a first release from every revision reachable from the released revision (`git describe --tags --abbrev=0 --match 'v*' "$released^"` finds no earlier `v*` tag, so no range is cut and `previous` is empty), stamps the heading from the released revision's own committer date, and appends no comparison link when no earlier version exists. The steps build a history whose first release is tagged `v0.1.0` (scenario 9's commit later extends that same shared fixture with a second release, tagged `v0.2.0`, which leaves this scenario's assertions unchanged), assert no change log exists yet, render version 0.1.0 into it, and assert the file holds exactly one section for 0.1.0, that the section names the new capability, and that it states no comparison.
- [x] REFACTOR — paste the output:
  ```
  rustfmt --edition 2021 --check apps/cli/tests/steps/publish_public_release.rs
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 6 steps (6 passed)   REFACTOR_EXIT=0
  ```
  The only tidy was rustfmt rewrapping one tuple of the new fixture constant; no behaviour changed.
- [x] COMMIT: `5581434` — subject is the release note, body names this scenario.

### 7. Maintainer generates the change log for a version and reads its newest section (`@e2e`)

Use-case constraint: the assertion is on the change log the real command wrote and the newest section it reported; the transcript is the committed evidence.

- [x] Confirm `./run-tests.sh --e2e` counts strictly fewer scenarios than the full run and that strict mode holds; bring up the environment named in the design (the pinned generator on `PATH`, `git`, `tar`, `gzip`, `sha256sum`).
  ```
  ./run-tests.sh          3 features, 95 scenarios (90 passed, 5 failed), 345 steps
  ./run-tests.sh --e2e    3 features,  3 scenarios ( 2 passed, 1 failed),  13 steps
  ```
  Strictly fewer: 3 against 95. Strict mode holds: `apps/cli/tests/features_runner.rs` calls `.fail_on_skipped()` on the `Cucumber` builder, so a skipped or undefined step fails the run — which is exactly what the five remaining undefined steps did above. Environment, all on `PATH` for the run: `nix-shell -p rustc cargo git-cliff` supplies the pinned `git-cliff 2.14.2` (`git-cliff --version` → `git-cliff 2.14.2`, the version `cliff.toml` records), and `git`, `tar`, `gzip` and `sha256sum` are the host's own. No network, no credentials, no GitHub.
- [x] RED — remove `@wip`; write the e2e steps with `unimplemented!(...)` bodies; run the single-scenario command; paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: Maintainer generates the change log for a version and reads its newest section
     ✘  Given a repository whose history releases version 0.1.0
        Step panicked. Captured output: not implemented: build a repository whose history releases version 0.1.0
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — drive the real renderer as a subprocess; commit the transcript to `givn/changes/generate-release-notes/evidence/visual/maintainer-generates-the-change-log-for-a-version-and-reads-its-newest-section/transcript.txt` with the run-specific directory replaced by a placeholder; paste the passing output. List every production file touched:
  ```
  Feature: Publish a public release
    Scenario: Maintainer generates the change log for a version and reads its newest section
     ✔  Given a repository whose history releases version 0.1.0
     ✔  When the maintainer generates the change log for version 0.1.0
     ✔  Then the change log file holds a section for version 0.1.0
     ✔  And that section names the released change a user can see
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
  Production files touched: none — the steps drive the already-written `scripts/render-changelog.sh` as a real subprocess. The `Given` builds the fixture repository through `build_history` (now `pub(crate)`, as is the `FIRST_RELEASE_HISTORY` constant it takes, so the e2e file extends the same fixture instead of inventing a second one); the `When` reuses the capability's own `the maintainer generates the change log for version 0.1.0` step; the `Then` reads the written file and runs `cat` on it, so the transcript shows the newest section. The committed transcript:
  ```
  $ scripts/render-changelog.sh --version 0.1.0 --repo <scenario-dir>/history --out <scenario-dir>/CHANGELOG.md
  rendered version 0.1.0 into <scenario-dir>/CHANGELOG.md

  $ cat <scenario-dir>/CHANGELOG.md
  ## [0.1.0] - 2026-01-03

  ### New capabilities
  - Convert a supported duty plan into a calendar

  exit status: 0
  ```
- [x] REFACTOR — paste the output, and the `./run-tests.sh --e2e` run:
  ```
  rustfmt --edition 2021 --check apps/cli/tests/steps/publish_public_release.rs \
      apps/cli/tests/steps/e2e/publish_public_release.rs
  FMT OK
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  3 scenarios (3 passed), 16 steps (16 passed)   E2E_EXIT=0
  ```
  No tidy needed: both step files are rustfmt-clean as written, and the `@e2e` run is green at 3 scenarios.
- [x] COMMIT: `cd95db5` — subject is the release note, body names this scenario.

### 8. The release text states the released revisions' release notes (`@givn.modified`)

Use-case constraint: the permanent scenario is restated against the new rule — the release text is the newest section of the change log, grouped by kind, one entry per released change however many revisions carry it, and nothing a user cannot see.

- [x] RED — remove `@wip`; run the single-scenario command; paste the non-zero output (the two scenarios share the title, so the full-suite count is pasted alongside it):
  ```
  Feature: Publish a public release
    Scenario: The release text states the released revisions' release notes
     ✔  Given a history whose released revisions carry release-note subjects, including one subject repeated by several revisions and one subject that carries no release-note type, and whose housekeeping revisions do not
     ✔  When the maintainer prepares the release for version 0.2.0
     ✔  Then the release text names version 0.2.0
     ✔  And the release text lists these release notes in release order
     ✔  And the release text omits the housekeeping revisions and the subject that carries no release-note type
  Feature: Publish a public release
    Scenario: The release text states the released revisions' release notes
     ✔  Given a history whose released revisions name a new capability, a fix, and work no user can see
     ✘  And one released change is named by several revisions
        Step panicked. Captured output: not implemented: build a history naming one released change by several revisions
  [Summary]
  2 features
  2 scenarios (1 passed, 1 failed)
  7 steps (6 passed, 1 failed)
  RED_EXIT=101
  ```
  Full suite at this point: `3 features, 95 scenarios (90 passed, 5 failed), 345 steps` — the permanent scenario that shares the title passes beside the delta scenario, which is what `@givn.modified` means before archive.
- [x] GREEN — switch `scripts/prepare-release.sh` from the flat subject list to the rendered newest section and stage the change log; paste the passing output. List every production file touched:
  ```
  Feature: Publish a public release
    Scenario: The release text states the released revisions' release notes
     ✔  Given a history whose released revisions carry release-note subjects, including one subject repeated by several revisions and one subject that carries no release-note type, and whose housekeeping revisions do not
     ✔  When the maintainer prepares the release for version 0.2.0
     ✔  Then the release text names version 0.2.0
     ✔  And the release text lists these release notes in release order
         | release note                                  |
         | Offer an archive for every supported platform |
         | Publish only an archive the evidence supports |
     ✔  And the release text omits the housekeeping revisions and the subject that carries no release-note type
  Feature: Publish a public release
    Scenario: The release text states the released revisions' release notes
     ✔  Given a history whose released revisions name a new capability, a fix, and work no user can see
     ✔  And one released change is named by several revisions
     ✔  When the maintainer prepares the release for version 0.2.0
     ✔  Then the release text names version 0.2.0
     ✔  And the release text names the new capability before the fix
     ✔  And the release text names the released change once
     ✔  And the release text omits the work no user can see
  [Summary]
  2 features
  2 scenarios (2 passed)
  12 steps (12 passed)
  GREEN_EXIT=0
  ```
  Production files touched: `scripts/prepare-release.sh` (the flat subject list is gone — `collect_release_notes`, `RELEASE_NOTE_TYPES` and `HOUSEKEEPING_TYPES` with it — and the release text is now the released version's newest section, rendered by `scripts/render-changelog.sh` into `release-notes.md` and staged beside it as `CHANGELOG.md` from the same render, so the page and the repository cannot disagree and a version whose section names nothing visible is refused before anything leaves `$staging`). Test files touched in the same GREEN, because the changed behaviour is what the scenario pins: `apps/cli/tests/steps/publish_public_release.rs` — the permanent scenario's fixture `HISTORY` is restated on capability scopes with fixed committer dates and a `v0.2.0` tag and is built by the shared `build_history`, and the staged-set assertion now expects `CHANGELOG.md`. No other file changed.
- [x] REFACTOR — paste the output:
  ```
  rustfmt --edition 2021 --check apps/cli/tests/steps/publish_public_release.rs \
      apps/cli/tests/steps/e2e/publish_public_release.rs
  FMT OK
  2 scenarios (2 passed), 12 steps (12 passed)   REFACTOR_EXIT=0
  ```
  No tidy needed: the step file is rustfmt-clean as written. The full suite after the change: `3 features, 95 scenarios (92 passed, 3 failed), 353 steps` — only the three still-undefined steps of the delta sections below fail, and every permanent release scenario (including the archive-set one, which now stages the change log too) still passes.
- [x] COMMIT: `f4c9b4b` — subject is the release note, body names this scenario.

### 9. The prepared release carries the change log its section came from

Use-case constraint: the preparation stages `CHANGELOG.md`, whose newest section is the release text, so the page and the repository cannot disagree.

- [x] RED — paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: The prepared release carries the change log its section came from
     ✔  Given a history whose released revisions name a new capability and work no user can see
     ✔  When the maintainer prepares the release for version 0.2.0
     ✘  Then the prepared release stages the change log for version 0.2.0
        Step panicked. Captured output: not implemented: assert the prepared release stages the change log for version 0.2.0
  [Summary]
  1 feature
  1 scenario (1 failed)
  3 steps (2 passed, 1 failed)
  RED_EXIT=101
  ```
  The `Given` reuses the capability's own step, so the first attempt at a second definition made the step ambiguous (`Step match is ambiguous: … 1190:1 … 1332:1`) and the duplicate was removed before this run.
- [x] GREEN — staging the change log beside the release text; paste the passing output. List every production file touched:
  ```
  Feature: Publish a public release
    Scenario: The prepared release carries the change log its section came from
     ✔  Given a history whose released revisions name a new capability and work no user can see
     ✔  When the maintainer prepares the release for version 0.2.0
     ✔  Then the prepared release stages the change log for version 0.2.0
     ✔  And the staged change log holds the section the release text states
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
  Production files touched: none new — `scripts/prepare-release.sh` (changed by the previous scenario's GREEN) already copies the checkout's `CHANGELOG.md` into `$staging` where one exists and always renders the released version's section into it with `--release-text` writing `release-notes.md` from the same render, then stages both. The steps assert the staged `CHANGELOG.md` holds the section for 0.2.0 and that that section, trimmed, is exactly the staged release text.
- [x] REFACTOR — paste the output:
  ```
  rustfmt --edition 2021 --check apps/cli/tests/steps/publish_public_release.rs \
      apps/cli/tests/steps/e2e/publish_public_release.rs
  FMT OK
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
  No tidy needed. The `Given` the two scenarios share now builds a two-release history (`CAPABILITY_AND_INVISIBLE_HISTORY`, tagged `v0.1.0` and `v0.2.0`) so one step serves this scenario's version 0.2.0 and the first-version scenario's version 0.1.0; both scenarios were re-run against it and pass unchanged (6/6 and 4/4 steps), and the `@e2e` render scenario keeps its own single-release fixture (`FIRST_RELEASE_HISTORY`) and its committed transcript unchanged.
- [x] COMMIT: `912f740` — subject is the release note, body names this scenario.

### 10. A capability missing from the configured allow-list is refused

Use-case constraint: the configuration names every capability the repository holds; one present under `givn/specs/` and absent from it is refused rather than left out of every release.

- [x] RED — paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: A capability missing from the configured allow-list is refused
     ✘  Given a capability the repository holds that the configuration does not name
        Step panicked. Captured output: not implemented: build a repository holding a capability the configuration does not name
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — the allow-list-versus-corpus refusal; paste the passing output. List every production file touched:
  ```
  Feature: Publish a public release
    Scenario: A capability missing from the configured allow-list is refused
     ✔  Given a capability the repository holds that the configuration does not name
     ✔  When the maintainer generates the change log for version 0.2.0
     ✔  Then the generation fails and names the capability missing from the configuration
  [Summary]
  1 feature
  1 scenario (1 passed)
  3 steps (3 passed)
  GREEN_EXIT=0
  ```
  Production files touched: none new — `scripts/render-changelog.sh` already reads the allow-list off the marked line of `cliff.toml` and compares it against the use-case ids present as directories under `givn/specs/`, refusing in both directions, so a capability the repository holds and the configuration leaves out is named rather than dropped from every release. The steps build the shared fixture, assert the committed configuration does not name `check-duty-plan`, add that capability directory, and assert the generation fails naming it.
- [x] REFACTOR — paste the output:
  ```
  rustfmt --edition 2021 --check apps/cli/tests/steps/publish_public_release.rs \
      apps/cli/tests/steps/e2e/publish_public_release.rs
  FMT OK
  1 scenario (1 passed), 3 steps (3 passed)   REFACTOR_EXIT=0
  ```
  The only tidy was rustfmt rewrapping one `let` binding; no behaviour changed.
- [x] COMMIT: `ff199eb` — subject is the release note, body names this scenario.

### 11. A committed change log missing the released version's section is refused

Use-case constraint: the change log is committed before the version's tag; a committed change log that lacks the released version's section is refused and the version is named.

- [x] RED — paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: A committed change log missing the released version's section is refused
     ✘  Given a committed change log that holds no section for version 0.2.0
        Step panicked. Captured output: not implemented: write a committed change log holding no section for the version
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — the `--check` missing-section refusal; paste the passing output. List every production file touched:
  ```
  Feature: Publish a public release
    Scenario: A committed change log missing the released version's section is refused
     ✔  Given a committed change log that holds no section for version 0.2.0
     ✔  When the change log is checked against the released revision
     ✔  Then the check fails and names version 0.2.0
  [Summary]
  1 feature
  1 scenario (1 passed)
  3 steps (3 passed)
  GREEN_EXIT=0
  ```
  Production files touched: none new — `scripts/render-changelog.sh` already refuses, in `--check` mode, a committed change log that holds no section for the version being released: it names the version and writes nothing, because the section is committed before the version's tag. The steps build the two-release fixture, commit a change log that stops at 0.1.0, and assert the check fails naming version 0.2.0.
- [x] REFACTOR — paste the output:
  ```
  rustfmt --edition 2021 --check apps/cli/tests/steps/publish_public_release.rs \
      apps/cli/tests/steps/e2e/publish_public_release.rs
  FMT OK
  1 scenario (1 passed), 3 steps (3 passed)   REFACTOR_EXIT=0
  ```
  The only tidy was rustfmt joining one wrapped call chain; no behaviour changed.
- [x] COMMIT: `7962114` — subject is the release note, body names this scenario.

  **Follow-up fix committed as `cbfe2f5`** (`fix(generate-release-notes): keep the preparation's transcript free of the staging path.`): with the preparation rendering the release text, the renderer's own report named its `mktemp` staging directory, which is a fresh random path on every run, so the permanent `@e2e` transcript changed on every suite run. `scripts/prepare-release.sh` now silences that report (a refusal still reaches standard error, and the preparation prints its own deterministic line for each staged file) and the archived transcript is refreshed with the one deterministic line the change adds. Full suite after the fix: `95 scenarios (95 passed), 358 steps (358 passed)`, exit 0.

## Non-scenario work (the change log, the preparation, the pipeline and the public surface)

Not observable through a Gherkin scenario; each is verified by running it or by the host's own run, and recorded as evidence.

- [x] `cliff.toml` at the repository root: the capability allow-list repeated inside each type's parser (`feat`, `fix`, `perf`, `revert` and each breaking-change case), the group order (Breaking changes, New capabilities, Fixes, Performance, Reversals), the skip rules for housekeeping (`chore`, `docs`, `style`), `test`, `build`, `ci` and any scope outside the allow-list, the literal comparison URL built in the template from the two versions, and no `[remote.github]` integration. Evidence: the render scenarios and the drift check.
  ```
  $ nix-shell -p git-cliff --run 'scripts/render-changelog.sh --version 0.1.0 --repo . --out CHANGELOG.md --check'
  change log is current for version 0.1.0
  check=0
  $ grep -n 'git-cliff version' cliff.toml
  13:# git-cliff version: 2.14.2
  ```
  The file is unchanged by this change's work (committed with scenario 1): the allow-list line `<!-- capability-allow-list: export-duty-plan-calendar publish-public-release -->` is what the renderer reads and compares against `givn/specs/`, each of `feat`, `fix`, `perf` and `xrevert` carries the allow-list in its own parser for both the plain and the breaking-change form, the group markers order the groups `<!-- 0 -->` to `<!-- 4 -->`, `{ message = ".*", skip = true }` closes the list so housekeeping, tests, CI, tooling and every scope outside the allow-list are skipped, the comparison link is built as a literal URL in the renderer and no `[remote.github]` section exists.
- [x] `CHANGELOG.md` at the repository root, generated with the pinned tool — never hand-written — with the `0.1.0` section back-filled under the new rules: it holds about five bullets rather than the published page's thirty-seven, because identical subjects collapse. Evidence: `nix-shell -p git-cliff --run 'scripts/render-changelog.sh --version 0.1.0 --repo . --out CHANGELOG.md'` and `--check` clean; the bullet count pasted.
  ```
  $ nix-shell -p git-cliff --run 'scripts/render-changelog.sh --version 0.1.0 --repo . --out CHANGELOG.md'
  rendered version 0.1.0 into CHANGELOG.md
  render=0
  $ grep -c '^- ' CHANGELOG.md
  5
  ```
  The section holds two new capabilities and three fixes — five bullets against the published page's thirty-seven, because identical release-note subjects collapse. The re-render leaves the file byte-identical (`git status --short CHANGELOG.md` prints nothing) and `--check` passes on it.
- [x] `.github/workflows/ci.yml`: a `changelog` job (pushes and pull requests) that checks out with `fetch-depth: 0`, installs `git-cliff@2.14.2` with `taiki-e/install-action@v2` and runs `scripts/render-changelog.sh --version <newest v* tag> --check`; the `test` job installs the pinned generator on `PATH` because the suite's steps drive the renderer as a real subprocess. Every existing job keeps its name and steps. Evidence: `nix-shell -p actionlint --run 'actionlint .github/workflows/*.yml'` clean.
  ```
  $ nix-shell -p actionlint --run 'actionlint .github/workflows/*.yml'
  actionlint=0
  ```
  Committed as `7447a57`. Every existing job keeps its name and steps; `test` gains only the `taiki-e/install-action@v2` step with `tool: git-cliff@2.14.2` before `./run-tests.sh`.
- [x] `.github/workflows/release.yml`: `staged/CHANGELOG.md` joins the `gh release create` asset list so the draft carries the change log among its assets. Evidence: the same actionlint run; the staged-versus-draft comparison still matches, since `release-notes.md` stays excluded from it.
  ```
  $ nix-shell -p actionlint --run 'actionlint .github/workflows/*.yml'
  actionlint=0
  ```
  Committed as `7447a57`. `staged/CHANGELOG.md` joins `staged/*.tar.gz staged/*.tar.gz.sha256` on the `gh release create` line; both asset-set comparisons filter out only `release-notes.md`, so the change log is compared like the archives. **Deviation:** the design's file table says release.yml changes "only so the draft carries the change log among its assets", but the preparation now renders the release text and the change log, so the `prepare` job also installs `git-cliff@2.14.2` with `taiki-e/install-action@v2`. Its `fetch-depth: 0` checkout was already there ("prepare-release.sh reads the release text from the tag's history"), which the renderer needs for the tags and the full history.
- [x] `README.md`: the Development section records the one extra release step — render the version's section, commit the change log, then push the tag — without disturbing the givn-managed blocks. Evidence: `givn readme check` and `bash scripts/public-text-check.sh` clean.
  ```
  $ givn readme check
  README check: OK
  readme_check=0
  $ bash scripts/public-text-check.sh
  public-text-check: clean - all 12 public-text promises present in /home/dan/dev/PRIVATE/duty2ical/README.md
  public_text=0
  ```
  Committed as `7447a57`. The subsection `### Releasing a version` sits immediately after `<!-- givn:end:development -->`, outside the managed markers, so it reads as part of the Development section without editing it; the givn-managed blocks are untouched and `givn readme check` stays OK.

## Final verification (the delta's eleven scenarios green)

- [x] Full suite — `nix-shell -p rustc cargo --run './run-tests.sh'`:
  ```
  $ nix-shell -p rustc cargo git-cliff --run './run-tests.sh'
  3 features
  95 scenarios (95 passed)
  358 steps (358 passed)
  FULL_EXIT=0
  ```
  The permanent corpus is 84 scenarios (`givn/specs/`: 75 scenario instances from `export-duty-plan-calendar`, counting its two `Scenario Outline`s' four and seven examples, plus 9 from `publish-public-release`), and the delta adds 11, so 95 while the change is active. The one `@givn.modified` scenario runs beside the permanent scenario it replaces by title — the `@givn.modified` merge happens at archive, not during the change — which is why the delta's eleven all count here. `git-cliff` is added to the `nix-shell` packages because the change-log scenarios drive `scripts/render-changelog.sh` as a real subprocess and the renderer refuses a host without the pinned generator on `PATH`; that is the same environment the CI `test` job now provides.
- [x] E2E suite — `nix-shell -p rustc cargo --run './run-tests.sh --e2e'`:
  ```
  $ nix-shell -p rustc cargo git-cliff --run './run-tests.sh --e2e'
  3 features
  3 scenarios (3 passed)
  16 steps (16 passed)
  E2E_EXIT=0
  ```
  The two permanent `@e2e` scenarios plus this change's new one; the transcript of the new one is committed under `evidence/visual/maintainer-generates-the-change-log-for-a-version-and-reads-its-newest-section/transcript.txt`.
- [x] Corpus lint — `givn lint --change generate-release-notes` (exit 0 or 2):
  ```
  $ givn lint --change generate-release-notes
  givn lint: 1 file(s) checked — clean
  lint=0
  ```
  The design-review phase recorded eleven findings, every one of them the `@wip` tag of a scenario awaiting its step definitions (exit 2); every `@wip` is gone now, so the lint is clean and exits 0.
- [x] Workflow definitions — `nix-shell -p actionlint --run 'actionlint .github/workflows/*.yml'`:
  ```
  $ nix-shell -p actionlint --run 'actionlint .github/workflows/*.yml'
  actionlint=0
  ```
  No output; exit 0. Clean over both workflows with the added `changelog` job, the `test` job's pinned-generator step, the release `prepare` job's pinned-generator step and the `staged/CHANGELOG.md` asset.
- [x] Public text and private data — `bash scripts/public-text-check.sh` and `bash scripts/pii-guard.sh`:
  ```
  $ bash scripts/public-text-check.sh
  public-text-check: clean - all 12 public-text promises present in /home/dan/dev/PRIVATE/duty2ical/README.md
  public_text=0
  $ bash scripts/pii-guard.sh
  pii-guard: clean
  pii=0
  ```
  The private-data guard scans the working tree and every revision reachable from the tip, so the new step fixtures (fixed 2026 dates, fictional subjects) and the committed transcript are clean in both surfaces.
- [x] Licence compliance — `nix-shell -p reuse --run 'reuse lint'`:
  ```
  $ nix-shell -p reuse --run 'reuse lint'
  # SUMMARY
  * Bad licenses: 0
  * Deprecated licenses: 0
  * Licenses without file extension: 0
  * Missing licenses: 0
  * Unused licenses: 0
  * Used licenses: MPL-2.0, Unicode-3.0
  * Read errors: 0
  * Invalid SPDX License Expressions: 0
  * Files with copyright information: 134 / 134
  * Files with license information: 134 / 134
  Congratulations! Your project is compliant with version 3.3 of the REUSE Specification :-)
  reuse=0
  ```
  `134 / 134` includes the new evidence transcript under `givn/changes/generate-release-notes/evidence/`.
- [x] The drift check on the committed change log — `nix-shell -p git-cliff --run 'scripts/render-changelog.sh --version 0.1.0 --repo . --out CHANGELOG.md --check'`:
  ```
  $ nix-shell -p git-cliff --run 'scripts/render-changelog.sh --version 0.1.0 --repo . --out CHANGELOG.md --check'
  change log is current for version 0.1.0
  check=0
  ```
  Run against the real checkout, whose newest `v*` tag is `v0.1.0`: the committed `CHANGELOG.md` equals a fresh render, and its 0.1.0 section holds five bullets.
- [x] The pinned generator's version, recorded where the design says (`cliff.toml`, `# git-cliff version: 2.14.2`), and the version the render actually ran:
  ```
  $ grep -n 'git-cliff version' cliff.toml
  13:# git-cliff version: 2.14.2
  $ nix-shell -p git-cliff --run 'git-cliff --version'
  git-cliff 2.14.2
  ```
  The two agree, which is what lets the drift check compare bytes: the renderer refuses a `git-cliff` whose version differs from the one recorded beside the configuration, and every render this change ran went through `nix-shell -p git-cliff` (2.14.2).

## Evidence pointers

- Transcript for the `@e2e` scenario: `evidence/visual/maintainer-generates-the-change-log-for-a-version-and-reads-its-newest-section/transcript.txt`.
- Host-side evidence (the `changelog` job's first run, the released version's draft carrying `CHANGELOG.md`) is recorded in `review.md` during the review phase, as the archived change recorded its own host-side evidence.
