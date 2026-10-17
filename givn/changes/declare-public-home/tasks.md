# Tasks: declare-public-home

Release note (every scenario commit carries it as the message subject; the
scenario title goes in the body):
The public home's existing organization settings, its two repositories, the
protection rule for the default branch and the site source become part of the
project's reviewed configuration and are bound to what already exists on GitHub
without changing it, so the home can be reviewed, diffed and rebuilt instead of
remembered.

Change constraints (apply to every scenario): the adoption command
`scripts/adopt-public-home.sh` is the only interface; the real `tofu` binary and
the pinned `integrations/github` provider are used; tests drive the local fake
GitHub API on loopback; no scenario touches `api.github.com`; adoption never
writes to GitHub; the applied state is encrypted and committed; the passphrase
is test-only in the suite.

## Setup task — runner, strict-mode proof, step skeletons, test infrastructure

Design constraints:
- Runner `./run-tests.sh` (already `verify.command`); single scenario
  `./run-tests.sh --name "<title>"`; e2e `./run-tests.sh --e2e`; strict mode is
  `Cucumber::fail_on_skipped()` in `apps/cli/tests/features_runner.rs`.
- Step files: `apps/cli/tests/steps/adopt_existing_configuration.rs` and
  `apps/cli/tests/steps/e2e/adopt_existing_configuration.rs`; declare both in
  the two `mod.rs` files; add the world fields the design names (declaration
  directory, batch, fake API child, transcript, state).
- Test infrastructure: `scripts/setup-infra-tests.sh` checks OpenTofu 1.13.1
  from `infra/.opentofu-version`, mirrors the provider, writes the
  `terraformrc`; `scripts/fake-github/serve.py` starts on loopback with
  fixtures, journal, faults, unknown-path 599 marker.
- Fixture proof: one real `tofu import` per object against the fake succeeds,
  and a `-refresh-only` plan on an unchanged fake is a no-op; extend fixtures
  until both hold.
- `.github/workflows/ci.yml` installs the pinned OpenTofu and runs the setup
  script before `./run-tests.sh`.

Evidence:
- [x] Strict-mode proof: run one `@wip` scenario; paste the non-zero output
  showing the undefined/skipped step fails. Command and output:
  ```
  $ ./run-tests.sh --name "An invalid declaration refuses before any object is read"
  ...
    Scenario: An invalid declaration refuses before any object is read
     ✘  Given the declared configuration fails validation
        Step failed:
        Defined: tmp/tmp.FGwT1lBuk1/f0003.feature:16:5
        Step doesn't match any function
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  error: test failed, to rerun pass `-p duty2cal --test features_runner`
  exit status: 101
  ```
- [x] Step files, modules and world fields created; the crate compiles
  (`cargo check -p duty2cal --tests`); output pasted.
  ```
  $ cargo check -p duty2cal --tests
      Checking cucumber v0.23.0
      Checking duty2cal v0.1.0 (/home/dan/dev/PRIVATE/duty2cal/apps/cli)
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 17.55s
  ```
- [x] Fixture proof: one import per object and the no-op refresh; commands and
  output pasted. The proof is part of `scripts/setup-infra-tests.sh`; the fake
  serves the fixtures under `scripts/fake-github/fixtures/adopt-all/` and the
  provider's `/api/v3` base-URL prefix is normalized before the fixture lookup.
  ```
  $ ./scripts/setup-infra-tests.sh
  tofu 1.13.1 (/etc/profiles/per-user/dan/bin/tofu)
  Python 3.14.7
  provider integrations/github 6.13.0 for linux_amd64
  mirror /home/dan/.cache/duty2cal/terraform-provider-mirror
  terraformrc /home/dan/.cache/duty2cal/terraformrc
  export TF_CLI_CONFIG_FILE=/home/dan/.cache/duty2cal/terraformrc
  import ok github_organization_settings.org 343978778
  import ok github_repository.mono duty2cal
  import ok github_repository.pages duty2cal.github.io
  import ok github_repository_ruleset.main duty2cal:24981082
  import ok github_repository_pages.pages duty2cal.github.io
  refresh-only plan on the unchanged fake: No changes.
  unknown paths served: 0
  fixtures scripts/fake-github/fixtures/adopt-all proven for 5 objects
  exit=0
  ```
  The same workspace's explicit `tofu plan -refresh-only` (the adoption's
  verification step) prints:
  ```
  No changes. Your infrastructure still matches the configuration.

  OpenTofu has checked that the real remote objects still match the result of
  your most recent changes, and found no differences.
  ```
  OpenTofu 1.13.1's `-detailed-exitcode` returns 2 for that same refresh-only
  plan with `change_summary {'add': 0, 'change': 0, 'import': 0, 'remove': 0,
  'forget': 0, 'operation': 'plan'}` — the recorded upstream defect
  opentofu/opentofu#3015. The adoption therefore treats the JSON report as
  authoritative: status 2 with no non-no-op `planned_change`/`resource_drift`
  is a no-op, and status 2 with a named address is the refusal scenario 7
  needs.
- [x] CI job updated; `actionlint`-clean.
  ```
  $ /tmp/gobin/actionlint -color .github/workflows/ci.yml
  exit=0
  ```

## 1. An invalid declaration refuses before any object is read

Design constraints:
- Preflight runs `tofu init -backend=false` and `tofu validate` inside the
  staged copy before any import; a validation failure names the declaration
  error and exits 2.
- No live object is read: the fake's journal stays empty; no object is bound.
- Refusal format `refused <object>: <reason>`.

- [x] RED — remove `@wip` from this scenario only; steps stubbed/undefined;
      run the single-scenario command; paste output (must be non-zero).
      ```
      $ ./run-tests.sh --name "An invalid declaration refuses before any object is read"
      ...
        Scenario: An invalid declaration refuses before any object is read
         ✘  Given the declared configuration fails validation
            Step failed:
            Defined: tmp/tmp.hNsARf15M2/f0003.feature:16:5
            Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      exit status: 101
      ```
- [x] GREEN — real assertions and the minimum production code; list production
      files created/modified; run the single scenario; paste output (zero).
      Production files created/modified:
      `scripts/adopt-public-home.sh` (preflight, staging, `tofu init
      -backend=false`, `tofu validate`, the `refused declaration:` refusal);
      `infra/versions.tf`, `infra/main.tf`, `infra/repos.tf`,
      `infra/rulesets.tf`, `infra/pages.tf`, `infra/adoption-batch.txt`,
      `infra/.opentofu-version`, `infra/.terraform.lock.hcl` (the declaration
      the command validates); `apps/cli/tests/steps/adopt_existing_configuration.rs`,
      `apps/cli/tests/steps/e2e/adopt_existing_configuration.rs` and the two
      `mod.rs` files (steps); `apps/cli/tests/features_runner.rs` (world fields,
      fake child cleanup); `scripts/setup-infra-tests.sh`,
      `scripts/fake-github/serve.py`, `.gitignore`, `.github/workflows/ci.yml`
      (setup infrastructure).
      ```
      $ ./run-tests.sh --name "An invalid declaration refuses before any object is read"
      ...
      Feature: Adopt the existing public-home configuration
        Scenario: An invalid declaration refuses before any object is read
         ✔  Given the declared configuration fails validation
         ✔  When the organization owner adopts the batch
         ✔  Then the adoption refuses the batch and names the declaration error
         ✔  And no object is read from the live organization
         ✔  And no object in the batch is bound
      [Summary]
      1 feature
      1 scenario (1 passed)
      5 steps (5 passed)
      exit status: 0
      ```
- [x] REFACTOR — clean up without behaviour change; re-run; paste output (zero).
      ```
      $ ./run-tests.sh --name "An invalid declaration refuses before any object is read"
      ...
      1 feature
      1 scenario (1 passed)
      5 steps (5 passed)
      exit status: 0
      ```
- [x] COMMIT — `feat(adopt-existing-configuration): <release note>` with the
      scenario title in the body; record the revision.
      Revision: `2bd68e3bc0d4ac0a7f0a290513039da57104d07f`

## 2. A malformed batch entry refuses before any import

Design constraints:
- Preflight parses every batch line before staging; a line without an import
  identifier is refused naming the line; no import runs.
- The batch format is `address<TAB>import-id`, `#` comments.

- [x] RED — remove `@wip` from this scenario only; run the single scenario;
      paste output (non-zero).
      ```
      $ ./run-tests.sh --name "A malformed batch entry refuses before any import"
      ...
        Scenario: A malformed batch entry refuses before any import
         ✘  Given the selected batch has a line without an import identifier
            Step failed:
            Defined: tmp/tmp.9MJfJp8iGb/f0003.feature:24:5
            Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      exit status: 101
      ```
- [x] GREEN — assertions and production code; list files; paste output (zero).
      The batch preflight in `scripts/adopt-public-home.sh` parses every line
      before staging and refuses a line without a tab-separated import
      identifier, naming the line number and quoting the line;
      `apps/cli/tests/steps/adopt_existing_configuration.rs` adds the scenario's
      Given and the two Then assertions (journal empty, no state recorded);
      `givn/changes/declare-public-home/specs/adopt-existing-configuration/adopt-existing-configuration.feature`
      drops this scenario's `@wip`.
      ```
      $ ./run-tests.sh --name "A malformed batch entry refuses before any import"
      ...
      Feature: Adopt the existing public-home configuration
        Scenario: A malformed batch entry refuses before any import
         ✔  Given the selected batch has a line without an import identifier
         ✔  When the organization owner adopts the batch
         ✔  Then the adoption refuses the batch and names the malformed batch line
         ✔  And no object is imported
         ✔  And no object in the batch is bound
      [Summary]
      1 feature
      1 scenario (1 passed)
      5 steps (5 passed)
      exit status: 0
      ```
- [x] REFACTOR — cleanup; re-run; paste output (zero).
      `cargo fmt --all` normalised the step module; no behaviour changed.
      ```
      $ ./run-tests.sh --name "A malformed batch entry refuses before any import"
      ...
      1 feature
      1 scenario (1 passed)
      5 steps (5 passed)
      exit status: 0
      ```
- [x] COMMIT — release note subject, scenario title body; record the revision.
      Revision: `4a654b3f144b29c374390128b891508ceb26ce22`

## 3. A declared object that does not exist live refuses the whole batch

Design constraints:
- The provider's read returns 404 for `duty2cal/example`; the import stops; the
  staged state is discarded; the real state is untouched.
- The refusal names `duty2cal/example` and states it does not exist live.
- Nothing is created: the fake journal has no refused write or mutation.

- [x] RED — remove `@wip`; run the single scenario; paste output (non-zero).
      ```
      $ ./run-tests.sh --name "A declared object that does not exist live refuses the whole batch"
      Feature: Adopt the existing public-home configuration
        Scenario: A declared object that does not exist live refuses the whole batch
         ✘  Given the declared configuration names an additional repository "duty2cal/example"
            Step failed:
            Defined: tmp/tmp.e8d6vVOD8w/f0003.feature:32:5
            Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      exit status: 101
      ```
- [x] GREEN — assertions and production code; list files; paste output (zero).
      Production files created/modified:
      `scripts/fake-github/serve.py` (normalize the provider's `/api/v3`
      base-URL prefix to the fixture key); `scripts/setup-infra-tests.sh` (the
      recorded fixture proof); `scripts/fake-github/fixtures/adopt-all/**`
      (organization, both repositories, the rule set and the Pages source);
      `scripts/adopt-public-home.sh` (404/already-managed/401-403 refusals
      state the live identity and the reason; a status-2 refresh plan with no
      reported change is a no-op, see setup evidence);
      `apps/cli/tests/steps/adopt_existing_configuration.rs` (base fixture
      copy, fake restart, the scenario's Givens and assertions, the
      unknown-path journal assertion); `apps/cli/tests/features_runner.rs`
      (the named missing repository); the feature file drops this scenario's
      `@wip`.
      ```
      $ ./run-tests.sh --name "A declared object that does not exist live refuses the whole batch"
      Feature: Adopt the existing public-home configuration
        Scenario: A declared object that does not exist live refuses the whole batch
         ✔  Given the declared configuration names an additional repository "duty2cal/example"
         ✔  And the selected batch names that repository
         ✔  And it does not exist live
         ✔  When the organization owner adopts the batch
         ✔  Then the adoption refuses the batch and names "duty2cal/example"
         ✔  And the refusal states that the repository does not exist live
         ✔  And no object in the batch is bound
         ✔  And nothing is created in the live organization
      [Summary]
      1 feature
      1 scenario (1 passed)
      8 steps (8 passed)
      exit status: 0
      ```
- [x] REFACTOR — cleanup; re-run; paste output (zero).
      `cargo fmt --all` only reflowed the new step code; no behaviour changed.
      ```
      $ ./run-tests.sh --name "A declared object that does not exist live refuses the whole batch"
      1 feature
      1 scenario (1 passed)
      8 steps (8 passed)
      exit status: 0
      ```
- [x] COMMIT — release note subject, scenario title body; record the revision.
      Revision: `cd89ada1de3ff5bb03fc866b8886b92bb04abcd6`

## 4. An object already bound is not bound again

Design constraints:
- Prime with a monorepo-only batch, then adopt the full batch; the staged state
  is a copy of the applied state; the provider refuses the already-managed
  monorepo; the refusal names `duty2cal/duty2cal`.
- The applied state still binds the monorepo once.

- [x] RED — remove `@wip`; run the single scenario; paste output (non-zero).
      ```
      $ ./run-tests.sh --name "An object already bound is not bound again"
      Feature: Adopt the existing public-home configuration
        Scenario: An object already bound is not bound again
         ✘  Given the monorepo is already bound in the applied state
            Step failed:
            Defined: tmp/tmp.11VkaX6Iom/f0003.feature:43:5
            Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      exit status: 101
      ```
- [x] GREEN — assertions and production code; list files; paste output (zero).
      Production files created/modified:
      `apps/cli/tests/steps/adopt_existing_configuration.rs` (the priming
      monorepo-only adoption, the "adopts again" When, the `tofu state list`
      assertion against the applied state); `apps/cli/tests/features_runner.rs`
      (the passphrase override, needed by scenario 8 and used by the state
      reader); the feature file drops this scenario's `@wip`. The refusal
      itself needed no new command code: `tofu import` already names the
      already-managed address, and the live-name mapping from scenario 3 makes
      it print `duty2cal/duty2cal`.
      ```
      $ ./run-tests.sh --name "An object already bound is not bound again"
      Feature: Adopt the existing public-home configuration
        Scenario: An object already bound is not bound again
         ✔  Given the monorepo is already bound in the applied state
         ✔  When the organization owner adopts the batch again
         ✔  Then the adoption refuses the batch and names "duty2cal/duty2cal"
         ✔  And the applied state binds the monorepo once
      [Summary]
      1 feature
      1 scenario (1 passed)
      4 steps (4 passed)
      exit status: 0
      ```
- [x] REFACTOR — cleanup; re-run; paste output (zero).
      `cargo fmt --all` only reflowed the new step code; no behaviour changed.
      ```
      $ ./run-tests.sh --name "An object already bound is not bound again"
      1 feature
      1 scenario (1 passed)
      4 steps (4 passed)
      exit status: 0
      ```
- [x] COMMIT — release note subject, scenario title body; record the revision.
      Revision: `7519c272529ba9d316cadcd243bfbbc607c0b8f7`

## 5. An unreadable object refuses the whole batch

Design constraints:
- The fake returns 401/403 for the organization settings; the refusal names the
  object and states it cannot be read; no binding; the journal has no write.

- [x] RED — remove `@wip`; run the single scenario; paste output (non-zero).
      ```
      $ ./run-tests.sh --name "An unreadable object refuses the whole batch"
      Feature: Adopt the existing public-home configuration
        Scenario: An unreadable object refuses the whole batch
         ✘  Given the organization settings cannot be read with the owner's authority
            Step failed:
            Defined: tmp/tmp.YxI9OAZdh3/f0003.feature:50:5
            Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      exit status: 101
      ```
- [x] GREEN — assertions and production code; list files; paste output (zero).
      Production files created/modified:
      `apps/cli/tests/steps/adopt_existing_configuration.rs` (the 401 fault for
      `GET /orgs/duty2cal`, the fake restart, the refusal assertions and the
      shared no-write journal check); the feature file drops this scenario's
      `@wip`. The command already turns a 401 read into `the live object
      343978778 cannot be read` (scenario 3's refusal mapping).
      ```
      $ ./run-tests.sh --name "An unreadable object refuses the whole batch"
      Feature: Adopt the existing public-home configuration
        Scenario: An unreadable object refuses the whole batch
         ✔  Given the organization settings cannot be read with the owner's authority
         ✔  When the organization owner adopts the batch
         ✔  Then the adoption refuses the batch and names the organization settings
         ✔  And the refusal states that the organization settings cannot be read
         ✔  And no object in the batch is bound
         ✔  And the live configuration is unchanged
      [Summary]
      1 feature
      1 scenario (1 passed)
      6 steps (6 passed)
      exit status: 0
      ```
- [x] REFACTOR — cleanup; re-run; paste output (zero).
      `cargo fmt --all` and the shared `assert_no_live_write` helper; no
      behaviour changed.
      ```
      $ ./run-tests.sh --name "An unreadable object refuses the whole batch"
      1 feature
      1 scenario (1 passed)
      6 steps (6 passed)
      exit status: 0
      ```
- [x] COMMIT — release note subject, scenario title body; record the revision.
      Revision: `53f32674dc6e9c3db703759d1c2ed2f42978c8b9`

## 6. A batch entry the declaration does not name refuses the whole batch

Design constraints:
- `tofu import` refuses a target absent from the configuration; the refusal
  names the archived repository; no object is bound.

- [x] RED — remove `@wip`; run the single scenario; paste output (non-zero).
      ```
      $ ./run-tests.sh --name "A batch entry the declaration does not name refuses the whole batch"
      Feature: Adopt the existing public-home configuration
        Scenario: A batch entry the declaration does not name refuses the whole batch
         ✘  Given the selected batch names the archived repository, which the declared configuration does not name
            Step failed:
            Defined: tmp/tmp.ChppLXV3zY/f0003.feature:59:5
            Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      exit status: 101
      ```
- [x] GREEN — assertions and production code; list files; paste output (zero).
      Production files created/modified:
      `apps/cli/tests/steps/adopt_existing_configuration.rs` (the batch naming
      `github_repository.archived`, which the declaration does not name, and
      the refusal assertion); the feature file drops this scenario's `@wip`.
      `tofu import` refuses the undeclared target before any read, so no
      command change was needed; the refusal names the address and the import
      identifier.
      ```
      $ ./run-tests.sh --name "A batch entry the declaration does not name refuses the whole batch"
      Feature: Adopt the existing public-home configuration
        Scenario: A batch entry the declaration does not name refuses the whole batch
         ✔  Given the selected batch names the archived repository, which the declared configuration does not name
         ✔  When the organization owner adopts the batch
         ✔  Then the adoption refuses the batch and names the archived repository
         ✔  And no object in the batch is bound
      [Summary]
      1 feature
      1 scenario (1 passed)
      4 steps (4 passed)
      exit status: 0
      ```
- [x] REFACTOR — cleanup; re-run; paste output (zero).
      `cargo fmt --all`; no behaviour changed.
      ```
      $ ./run-tests.sh --name "A batch entry the declaration does not name refuses the whole batch"
      1 feature
      1 scenario (1 passed)
      4 steps (4 passed)
      exit status: 0
      ```
- [x] COMMIT — release note subject, scenario title body; record the revision.
      Revision: `b558e4d934839e7519bf4b85f93059d5cf7545f4`

## 7. A live object that changes after it was read refuses the batch

Design constraints:
- The fake mutates deterministically after an explicit request count
  (`after_requests`) or a step-controlled trigger, never after an implicit
  first read.
- The post-import `tofu plan -refresh-only -detailed-exitcode -json` reports a
  non-no-op change for the monorepo; the refusal names it; nothing is bound.

- [x] RED — remove `@wip`; run the single scenario; paste output (non-zero).
      ```
      $ ./run-tests.sh --name "A live object that changes after it was read refuses the batch"
      Feature: Adopt the existing public-home configuration
        Scenario: A live object that changes after it was read refuses the batch
         ✘  Given the monorepo's live settings change after the adoption reads them
            Step failed:
            Defined: tmp/tmp.eZqD9b1Wp5/f0003.feature:66:5
            Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      exit status: 101
      ```
- [x] GREEN — assertions and production code; list files; paste output (zero).
      Production files created/modified:
      `apps/cli/tests/steps/adopt_existing_configuration.rs` (the
      `mutate.json` deterministic change after one request, the fake restart
      and the scenario's assertions); the feature file drops this scenario's
      `@wip`; `scripts/adopt-public-home.sh` now reads the refresh-only report's
      `resource_drift` message, whose change is `{"action": "update"}` rather
      than an `actions` array, and refuses by address. The recorded run:
      ```
      $ ./run-tests.sh --name "A live object that changes after it was read refuses the batch"
      Feature: Adopt the existing public-home configuration
        Scenario: A live object that changes after it was read refuses the batch
         ✔  Given the monorepo's live settings change after the adoption reads them
         ✔  When the organization owner adopts the batch
         ✔  Then the adoption refuses the batch and names "duty2cal/duty2cal"
         ✔  And no object in the batch is bound
      [Summary]
      1 feature
      1 scenario (1 passed)
      4 steps (4 passed)
      exit status: 0
      ```
- [x] REFACTOR — cleanup; re-run; paste output (zero).
      `cargo fmt --all`; no behaviour changed.
      ```
      $ ./run-tests.sh --name "A live object that changes after it was read refuses the batch"
      1 feature
      1 scenario (1 passed)
      4 steps (4 passed)
      exit status: 0
      ```
- [x] COMMIT — release note subject, scenario title body; record the revision.
      Revision: `PENDING-SCENARIO-7-HASH`

## 8. An applied state encrypted with another passphrase refuses before any binding

Design constraints:
- The state is encrypted with a different passphrase; `tofu state list` in the
  staged workspace fails before any import; the refusal names the applied state;
  no object is bound; live unchanged.

- [ ] RED — remove `@wip`; run the single scenario; paste output (non-zero).
- [ ] GREEN — assertions and production code; list files; paste output (zero).
- [ ] REFACTOR — cleanup; re-run; paste output (zero).
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## 9. A failure to record the applied state binds nothing and changes nothing

Design constraints:
- The staged import succeeds but the final rename/write fails (read-only target
  or injected failure); the refusal names the applied state; no binding; live
  unchanged; the real state is untouched.

- [ ] RED — remove `@wip`; run the single scenario; paste output (non-zero).
- [ ] GREEN — assertions and production code; list files; paste output (zero).
- [ ] REFACTOR — cleanup; re-run; paste output (zero).
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## 10. Only the selected batch is bound

Design constraints:
- The declaration names both repositories; the batch names only the
  organization and the monorepo; the adoption reports those two bound and does
  not bind the site repository.

- [ ] RED — remove `@wip`; run the single scenario; paste output (non-zero).
- [ ] GREEN — assertions and production code; list files; paste output (zero).
- [ ] REFACTOR — cleanup; re-run; paste output (zero).
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## E2E setup task — before the first @e2e scenario

Design constraints:
- `verify.e2e_command` is `./run-tests.sh --e2e`, a strict subset of
  `./run-tests.sh`; prove the counts (e2e strictly less than full).
- E2E steps live in `apps/cli/tests/steps/e2e/adopt_existing_configuration.rs`;
  the fake server is started per scenario and killed at scenario end.
- The e2e scenario asserts the command's output (the five `adopted` lines) and
  the journal's no-write condition; transcript evidence is recorded.

Evidence:
- [ ] Full-suite scenario count and e2e scenario count pasted; e2e count is
  strictly less.
- [ ] `./run-tests.sh --e2e` reaches the fake on loopback (smoke) before the
  scenario steps are written.

## 11. Maintainer adopts the existing public home and every object is bound without changing it (@e2e)

Design constraints:
- Real subprocess `scripts/adopt-public-home.sh` against the fake; five objects
  (organization, both repositories, the ruleset, the Pages source) bound; one
  `adopted <address> <import-id>` line each.
- The journal has no refused write and no mutation; a GraphQL query POST is a
  read.
- Transcript committed under
  `givn/changes/declare-public-home/evidence/real-adoption/` for the later real
  run; the scenario itself uses the fake.

- [ ] RED — remove `@wip` from this scenario only; run the single scenario;
      paste output (non-zero).
- [ ] GREEN — assertions and production code; list files; paste output (zero).
- [ ] REFACTOR — cleanup; re-run; paste output (zero).
- [ ] COMMIT — `test(e2e): <release note>` with the scenario title in the body;
      record the revision.

## Final verification

- [ ] Every scenario box checked with pasted output and a revision.
- [ ] `./run-tests.sh` exits 0 over the full suite.
- [ ] `./run-tests.sh --e2e` exits 0 with a strictly smaller scenario count.
- [ ] `givn lint --change declare-public-home` has no errors.
- [ ] Post-GREEN operator step recorded: the maintainer runs the real adoption
  once against the organization with the real passphrase, commits the encrypted
  state and backup, and `tofu state list` shows the five addresses. (This step
  awaits the operator and does not block the suite.)
- [ ] Archive-time allow-list obligation: when the archive creates
  `givn/specs/adopt-existing-configuration/`, add the capability to
  `cliff.toml` and its parser rows in the same revision.
