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
- [ ] Fixture proof: one import per object and the no-op refresh; commands and
  output pasted. (Left to bundle B: the fixture set and its one-import proof
  are the next task; this bundle creates the fake server and setup script that
  proof runs through.)
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
      Revision: `PENDING-SCENARIO-2-HASH`

## 3. A declared object that does not exist live refuses the whole batch

Design constraints:
- The provider's read returns 404 for `duty2cal/example`; the import stops; the
  staged state is discarded; the real state is untouched.
- The refusal names `duty2cal/example` and states it does not exist live.
- Nothing is created: the fake journal has no refused write or mutation.

- [ ] RED — remove `@wip`; run the single scenario; paste output (non-zero).
- [ ] GREEN — assertions and production code; list files; paste output (zero).
- [ ] REFACTOR — cleanup; re-run; paste output (zero).
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## 4. An object already bound is not bound again

Design constraints:
- Prime with a monorepo-only batch, then adopt the full batch; the staged state
  is a copy of the applied state; the provider refuses the already-managed
  monorepo; the refusal names `duty2cal/duty2cal`.
- The applied state still binds the monorepo once.

- [ ] RED — remove `@wip`; run the single scenario; paste output (non-zero).
- [ ] GREEN — assertions and production code; list files; paste output (zero).
- [ ] REFACTOR — cleanup; re-run; paste output (zero).
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## 5. An unreadable object refuses the whole batch

Design constraints:
- The fake returns 401/403 for the organization settings; the refusal names the
  object and states it cannot be read; no binding; the journal has no write.

- [ ] RED — remove `@wip`; run the single scenario; paste output (non-zero).
- [ ] GREEN — assertions and production code; list files; paste output (zero).
- [ ] REFACTOR — cleanup; re-run; paste output (zero).
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## 6. A batch entry the declaration does not name refuses the whole batch

Design constraints:
- `tofu import` refuses a target absent from the configuration; the refusal
  names the archived repository; no object is bound.

- [ ] RED — remove `@wip`; run the single scenario; paste output (non-zero).
- [ ] GREEN — assertions and production code; list files; paste output (zero).
- [ ] REFACTOR — cleanup; re-run; paste output (zero).
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## 7. A live object that changes after it was read refuses the batch

Design constraints:
- The fake mutates deterministically after an explicit request count
  (`after_requests`) or a step-controlled trigger, never after an implicit
  first read.
- The post-import `tofu plan -refresh-only -detailed-exitcode -json` reports a
  non-no-op change for the monorepo; the refusal names it; nothing is bound.

- [ ] RED — remove `@wip`; run the single scenario; paste output (non-zero).
- [ ] GREEN — assertions and production code; list files; paste output (zero).
- [ ] REFACTOR — cleanup; re-run; paste output (zero).
- [ ] COMMIT — release note subject, scenario title body; record the revision.

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
