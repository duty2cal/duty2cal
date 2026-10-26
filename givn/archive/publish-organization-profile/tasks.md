# Tasks: publish-organization-profile

Release note (commit subject on every scenario revision):

> feat(apply-configuration-change): The organization's public profile and its default security policy are published from the reviewed declaration, so a visitor reads an accurate description of the local Windows converter and a contributor finds the confidential reporting route.

Use-case contract: `givn/specs/apply-configuration-change/usecase.md` —
"Make the live configuration match the declared configuration revision the
apply accepts." The capability `publish-organization-profile` is a new
capability under that use case. Guarantees: the apply creates the declared
objects that are absent live, records them, and changes nothing else; the
preview names every difference among the previewed objects; a refusal changes
nothing; a partial failure names the object and marks the state stale.
Personas: `project-maintainer--organization-owner` (Actor and administration
lens), `contributor--infrastructure-reviewer` (review lens), and
`crew-member--personal-calendar-owner` (prospective reader). Their
biographies are not Gherkin actors.

## Bundle context — applies to every scenario

- **Runner:** `./run-tests.sh` (verify.command) →
  `cargo test -p duty2cal --test features_runner` over the assembled feature
  root; `./run-tests.sh --e2e` (verify.e2e_command) is a strict subset.
  Strict mode is `DutyWorld::cucumber().fail_on_skipped()`; an undefined or
  pending step fails. The not-implemented stub is
  `unimplemented!("<scenario>: <step>")`.
- **Single-scenario command:** `./run-tests.sh --name "<title>"` for both
  regular and `@e2e` scenarios. Pinned cucumber-rs 0.23.0 rejects `--tags`
  together with `--name`; the `@e2e` scope is proven separately by
  `./run-tests.sh --e2e`.
- **Declaration:** `infra/org_profile.tf` declares
  `github_repository.dot_github`, `github_repository_file.org_profile_readme`,
  `github_repository_file.org_security`, `variable "org_source_dir"` (default
  empty) and `locals.org_source_dir` = `${path.module}/../org` when empty.
  Both file resources read `file("${local.org_source_dir}/...")`.
  `auto_init = true`, `visibility = "public"`, `has_issues = false`,
  `has_projects = false`, `has_wiki = false`, `has_discussions = false`.
- **Sources:** `org/profile/README.md`, `org/SECURITY.md`, licensed
  CC0-1.0; add `LICENSES/CC0-1.0.txt`; `REUSE.toml` gains an `org/**`
  annotation with `precedence = "override"` and `SPDX-License-Identifier =
  "CC0-1.0"`. The reviewed copy is the published copy.
- **Workspace integration:** `scripts/adopt-public-home.sh` excludes
  `org_profile.tf` from the staged declaration;
  `scripts/setup-infra-tests.sh` removes `org_profile.tf` from the import-proof
  and `security-applied` base workspaces;
  `apps/cli/tests/steps/enable_repository_security.rs`
  (`ensure_base_adoption`) removes it with `security.tf` for the base
  declaration and restores it after the base apply. The new profile step
  module copies `org/` beside the scenario's `infra/` (or sets
  `TF_VAR_org_source_dir` to the copy).
- **Fake GitHub API:** `scripts/fake-github/serve.py` in `--writes` mode.
  Add exactly the calls the provider 6.13.0 makes:
  `POST /orgs/{owner}/repos` (201, full repository body, overlay);
  create-follow-up `PATCH /repos/{owner}/{repo}`;
  `GET /repos/{owner}/{repo}`; `GET /repos/{owner}/{repo}/vulnerability-alerts`
  (204 for a created repository);
  `GET /repos/{owner}/{repo}/branches/{branch}`;
  `GET`/`PUT /repos/{owner}/{repo}/contents/{path}`;
  `GET /repos/{owner}/{repo}/commits/{sha}`. The `PUT` response carries a flat
  `commit`; the commit read carries a nested
  `{"sha":…,"commit":{"message":…,"committer":{"name":"fake","email":"fake@example.test"}}}`.
  The file body carries `type`, `encoding: "base64"`, `size`, `name`, `path`,
  `content`, `sha`, and a `url` ending `?ref=<branch>`. Blob SHA is
  `sha1(bytes)`; commit SHA is deterministic from repository path, file path,
  blob SHA and the applied-write sequence. The overlay is lock-guarded and
  journaled with `applied_write`; unknown paths still return 599.
- **Base state:** copy `scripts/fake-github/fixtures/security-applied/` into
  the scenario; write three 404 descriptors:
  `repos/duty2cal/.github/contents/profile/README.md.json`,
  `repos/duty2cal/.github/contents/SECURITY.md.json`, and
  `repos/duty2cal/duty2cal.github.io/contents/SECURITY.md.json` (each
  `{"__fake_response__": {"status": 404}}`), plus the absent profile
  repository descriptor `repos/duty2cal/.github.json`. Adopt both existing
  batches, then run the base declaration's real `tofu init`/`plan`/`apply`
  with `org_profile.tf` removed. `security.tf` and the scanning block remain,
  so that base apply performs the security change's scanning write and the
  inherited state matches the security-applied tree. Restore `org_profile.tf`
  and the profile sources. Set `journal_offset` after the base apply. The
  security capability's own base priming is separate: it removes
  `security.tf`/the scanning block to reproduce the pre-security state. The
  fake runs in `--writes` mode; kill it with `DutyWorld`.
- **Environment:** `GITHUB_TOKEN=test-token`,
  `GITHUB_BASE_URL=http://127.0.0.1:<port>`,
  `TF_VAR_state_passphrase=test-passphrase-0123456789`,
  `TF_VAR_billing_email=billing@example.test`, `TF_CLI_CONFIG_FILE` from
  `scripts/setup-infra-tests.sh`, `TF_VAR_org_source_dir` = the scenario's
  `org/` copy when not beside `infra/`.
- **Targeted preview/apply:** the saved plan is
  `tofu plan -input=false -no-color -target=github_repository.dot_github
  -target=github_repository_file.org_profile_readme
  -target=github_repository_file.org_security -out=plan.bin`;
  `tofu show -json plan.bin`; the read-only
  `tofu plan -refresh-only -detailed-exitcode -json` with the same targets;
  `tofu apply -input=false plan.bin`. The preview scope is the revision's
  objects, not a rationale about unrelated drift.
- **State assertions:** `tofu state list` records the three addresses;
  `tofu show -json`/`tofu state pull` for values. Read the fake's repository,
  contents and commits with raw loopback HTTP after the apply; the primary
  `@e2e` assertion is the API response, not repository-only state.
- **Evidence:** each `@e2e` scenario commits a scrubbed transcript under
  `givn/changes/publish-organization-profile/evidence/visual/<slug>/transcript.txt`
  with the scenario directory, port, token and passphrase replaced by
  placeholders.

## 0. Setup — runner, strict mode, stubs, baseline

- [x] Create the two step-definition modules and declare them in
      `apps/cli/tests/steps/mod.rs` and
      `apps/cli/tests/steps/e2e/mod.rs`. Every step body for the six scenarios
      starts as `unimplemented!("<scenario>: <step>")`. Add
      `DutyWorld.source_tree: Option<PathBuf>` and
      `DutyWorld.journal_offset: usize`. Do not change production behaviour.
      Created `apps/cli/tests/steps/publish_organization_profile.rs` and
      `apps/cli/tests/steps/e2e/publish_organization_profile.rs`; declared
      both modules; extended `DutyWorld`. No commit (setup task).
- [x] Prove strict mode: run the apply scenario with the stubs,
      `./run-tests.sh --name "Organization owner applies the previewed revision and the organization profile is published"`.
      Paste the non-zero output and the undefined/stub failure.

      ```text
      $ ./run-tests.sh --name "Organization owner applies the previewed revision and the organization profile is published"
      Feature: Publish the organization profile
        Scenario: Organization owner applies the previewed revision and the organization profile is published
         ✘  Given the applied state binds every declared object except the organization profile repository and its two files
            Step panicked. Captured output: not implemented: A profile repository that already exists live and is unbound refuses the preview: the applied state binds every declared object except the profile
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      ...
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      EXIT:101
      ```
- [x] Prove the baseline: `./run-tests.sh --tags 'not @wip'` exits 0 with the
      existing implemented scenarios. This baseline was re-run after the four
      non-e2e scenarios were GREEN, so the 148 count includes them; the two
      `@e2e` scenarios were still `@wip` at that point. Paste the summary.

      The same command is the final non-e2e verification after scenarios 1-4;
      the two `@e2e` scenarios remain `@wip` and are excluded:

      ```text
      $ ./run-tests.sh --tags 'not @wip'
      [Summary]
      6 features
      148 scenarios (148 passed)
      631 steps (631 passed)
      EXIT:0
      ```
- [x] Confirm `verify.command` and `verify.e2e_command` are unchanged and the
      e2e command is a strict subset; paste both commands from
      `givn/commands.yaml`.

      ```yaml
      verify:
        command: "./run-tests.sh"            # project-owned test script
        e2e_command: "./run-tests.sh --e2e"  # same script, @e2e filter
      ```

      `run-tests.sh` turns `--e2e` into `--tags @e2e`, so the e2e command is
      the full runner restricted to a tag; it is a strict subset (the count
      proof belongs to the e2e setup task).

Design constraints: strict mode `fail_on_skipped()`; no empty step bodies;
the stub is `unimplemented!`; the runner is the existing cucumber-rs runner;
the e2e command is `./run-tests.sh --e2e`.

No commit for this setup task.

### Non-e2e verification (after scenarios 1-4)

The shared `tofu` helper resolves `TF_VAR_org_source_dir` to the profile
scenario's revised `org/` copy, or to the repository's reviewed copy in every
other planning workspace, so the security guard plan and the profile plan read
the same source without a workspace-shaped `file()` failure. Recorded in the
test commit `cc386cb70e14484d59e3ec204844efd4e573dcda`.

```text
$ ./run-tests.sh --tags 'not @wip'
[Summary]
6 features
148 scenarios (148 passed)
631 steps (631 passed)
EXIT:0
```

```text
$ ./scripts/setup-infra-tests.sh
...
base declaration apply: 0 live changes (follow-up plan: No changes.)
refresh-only plan (security-applied-security-adopted) on the unchanged fake: No changes.
unknown paths served: 0
fixtures scripts/fake-github/fixtures/security-applied proven for 5 base + 2 security objects
SETUP_EXIT:0
```

`bash scripts/license-headers.sh`, `bash scripts/public-text-check.sh`,
`bash scripts/pii-guard.sh`, `cargo fmt --all --check` and
`cargo clippy --workspace --all-targets -- -D warnings` pass. `reuse lint`
could not run in this environment (`reuse: command not found`); the
`org/**` annotation and `LICENSES/CC0-1.0.txt` are present for the project's
REUSE check. `givn lint --change publish-organization-profile` reports only
the two `@e2e` scenarios still tagged `@wip`, which this bundle does not start.

## 1. A revised profile content updates only the profile file

Design constraints:
- Add `infra/org_profile.tf`, the two `org/` sources, `LICENSES/CC0-1.0.txt`
  and the `REUSE.toml` override. Add the fake-server overlays and the
  workspace integration fixes from the bundle context.
- The scenario's Given primes the applied state with the base batch, the
  `security-applied` base apply and the second batch, then restores
  `org_profile.tf` and runs the targeted apply that creates the profile
  repository and both files.
- A revised `org/profile/README.md` in the scenario's source tree is the
  candidate revision. The targeted preview names only
  `github_repository_file.org_profile_readme` as changed; the apply updates
  it; the repository and `SECURITY.md` are unchanged; the applied state
  records the revised content.
- The assertion reads the fake's `GET /repos/duty2cal/.github/contents/profile/README.md?ref=main`,
  decodes base64, and compares it with the scenario's revised source. The
  repository's `GET /repos/duty2cal/.github` body and the `SECURITY.md`
  content are unchanged before and after.
- The profile-content source is the only source of the published bytes;
  `overwrite_on_create = true` and the provider's SHA handling replace an
  existing file as an update.

- [x] RED — remove `@wip` from this scenario only; run
      `./run-tests.sh --name "A revised profile content updates only the profile file"`;
      paste the non-zero output.

      ```text
      Feature: Publish the organization profile
        Scenario: A revised profile content updates only the profile file
         ✘  Given the applied state binds the organization profile repository and both files
            Step panicked. Captured output: not implemented: A revised profile content updates only the profile file: the applied state binds the organization profile repository and both files
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      ...
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      EXIT:101
      ```
- [x] GREEN — implement the production/declaration/fake/test files; list every
      file created/modified; run the same single-scenario command; paste the
      zero-exit output.

      Files created: `infra/org_profile.tf`, `org/profile/README.md`,
      `org/SECURITY.md`, `LICENSES/CC0-1.0.txt`,
      `apps/cli/tests/steps/publish_organization_profile.rs`,
      `apps/cli/tests/steps/e2e/publish_organization_profile.rs`.
      Files modified: `infra/repos.tf`, `REUSE.toml`,
      `scripts/fake-github/serve.py`, `scripts/adopt-public-home.sh`,
      `scripts/setup-infra-tests.sh`,
      `apps/cli/tests/steps/enable_repository_security.rs`,
      `apps/cli/tests/steps/mod.rs`, `apps/cli/tests/steps/e2e/mod.rs`,
      `apps/cli/tests/features_runner.rs`, and the delta feature.

      ```text
      Feature: Publish the organization profile
        Scenario: A revised profile content updates only the profile file
         ✔  Given the applied state binds the organization profile repository and both files
         ✔  And a previewed revision whose profile content differs from the published profile content
         ✔  When the organization owner applies the previewed revision
         ✔  Then the apply reports the profile content as the only changed object
         ✔  And the organization profile repository serves the revised profile content
         ✔  And the default security policy served by the organization profile repository is unchanged
         ✔  And the organization profile repository itself is unchanged
      [Summary]
      1 feature
      1 scenario (1 passed)
      7 steps (7 passed)
      EXIT:0
      ```
- [x] REFACTOR — clean up without behaviour change; re-run; paste the
      zero-exit output.

      ```text
      Feature: Publish the organization profile
        Scenario: A revised profile content updates only the profile file
         ✔  Given the applied state binds the organization profile repository and both files
         ✔  And a previewed revision whose profile content differs from the published profile content
         ✔  When the organization owner applies the previewed revision
         ✔  Then the apply reports the profile content as the only changed object
         ✔  And the organization profile repository serves the revised profile content
         ✔  And the default security policy served by the organization profile repository is unchanged
         ✔  And the organization profile repository itself is unchanged
      [Summary]
      1 feature
      1 scenario (1 passed)
      7 steps (7 passed)
      EXIT:0
      ```
- [x] COMMIT: `ef18cd860f28dc2a5524b1cfa96ff5e4c2f6a718` — `feat(apply-configuration-change): <release note>` with the scenario title in the body; staged only this scenario's files.

## 2. The published profile content states only supported claims and requests no third-party resource

Design constraints:
- Use the same published-state helper as scenario 1 (or apply the profile
  declaration once in the Given). The scenario reads the published
  `profile/README.md` from the fake and compares it byte-for-byte with the
  reviewed `org/profile/README.md` in the scenario's source tree.
- Assert the exact claims: local tool reading a crew duty plan and writing a
  calendar; Windows x86_64 as the only distributed platform; no browser
  claim; the stable release address
  `https://github.com/duty2cal/duty2cal/releases/latest`; the source and site
  repository links; the non-affiliation sentence; the Unicode-3.0
  attribution; every link absolute; no image, badge or third-party resource.
- The absence of a specific third-party provider name is a review control
  (R-032), not a literal deny-list. Do not store the name in test source.
- The copy itself is the design's profile block; add the Unicode-3.0 sentence
  and its absolute `LICENSES/Unicode-3.0.txt` link if not already present.

- [x] RED — remove `@wip` from this scenario only; run
      `./run-tests.sh --name "The published profile content states only supported claims and requests no third-party resource"`;
      paste the non-zero output.

      ```text
      Feature: Publish the organization profile
        Scenario: The published profile content states only supported claims and requests no third-party resource
         ✘  Given the organization profile repository is published
            Step panicked. Captured output: not implemented: the published profile content scenario: the organization profile repository is published
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      ...
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      EXIT:101
      ```
- [x] GREEN — add the assertions and any missing copy; list files; run the
      same command; paste the zero-exit output.

      Files modified: `apps/cli/tests/steps/publish_organization_profile.rs`.
      The reviewed copy was already the design's block (scenario 1), including
      the Unicode-3.0 sentence and its absolute licence link.

      ```text
      Feature: Publish the organization profile
        Scenario: The published profile content states only supported claims and requests no third-party resource
         ✔  Given the organization profile repository is published
         ✔  When a reader reads the published profile content
         ✔  Then the profile content describes the converter as a local tool that reads a crew duty plan and writes a calendar
         ✔  And the profile content names Windows x86_64 as the only distributed platform
         ✔  And the profile content makes no claim that the converter runs in a browser
         ✔  And the profile content links the stable latest-release address "https://github.com/duty2cal/duty2cal/releases/latest"
         ✔  And the profile content links the source repository "https://github.com/duty2cal/duty2cal" and the site repository "https://github.com/duty2cal/duty2cal.github.io"
         ✔  And the profile content states that the project is not affiliated with any duty-plan provider or airline
         ✔  And the profile content attributes the third-party Unicode data to Unicode-3.0
         ✔  And every link in the profile content is absolute
         ✔  And the profile content embeds no image, badge or other third-party resource
      [Summary]
      1 feature
      1 scenario (1 passed)
      11 steps (11 passed)
      EXIT:0
      ```
- [x] REFACTOR — clean up; re-run; paste the zero-exit output.

      ```text
      Feature: Publish the organization profile
        Scenario: The published profile content states only supported claims and requests no third-party resource
         ✔  Given the organization profile repository is published
         ✔  When a reader reads the published profile content
         ✔  Then the profile content describes the converter as a local tool that reads a crew duty plan and writes a calendar
         ✔  And the profile content names Windows x86_64 as the only distributed platform
         ✔  And the profile content makes no claim that the converter runs in a browser
         ✔  And the profile content links the stable latest-release address "https://github.com/duty2cal/duty2cal/releases/latest"
         ✔  And the profile content links the source repository "https://github.com/duty2cal/duty2cal" and the site repository "https://github.com/duty2cal/duty2cal.github.io"
         ✔  And the profile content states that the project is not affiliated with any duty-plan provider or airline
         ✔  And the profile content attributes the third-party Unicode data to Unicode-3.0
         ✔  And every link in the profile content is absolute
         ✔  And the profile content embeds no image, badge or other third-party resource
      [Summary]
      1 feature
      1 scenario (1 passed)
      11 steps (11 passed)
      EXIT:0
      ```
- [x] COMMIT: `f90447ca1e2e3dbd7d465cfff9c1b132ae9ed31f` — same release-note subject, scenario title in the body; staged only this scenario's files.

## 3. The profile repository carries the organization's default security policy

Design constraints:
- Use the published-state helper. Assert the fake serves
  `GET /repos/duty2cal/.github/contents/SECURITY.md?ref=main` with the
  reviewed `org/SECURITY.md` content.
- Assert the policy names the private vulnerability reporting route
  `https://github.com/duty2cal/duty2cal/security/advisories/new`, forbids
  attaching a real duty plan or a converted calendar, states that only the
  latest published release is supported, and that the profile repository
  offers no public issue tracker.
- Assert the pages repository serves no `SECURITY.md` of its own: the
  scenario tree writes the 404 descriptor for
  `repos/duty2cal/duty2cal.github.io/contents/SECURITY.md.json`; the step
  reads 404 from the fake.
- The operator enables private vulnerability reporting manually before the
  real apply; the suite asserts the policy text, not the live setting
  (R-030). The default-health-file fallback itself is GitHub's behaviour
  (R-033).

- [x] RED — remove `@wip` from this scenario only; run
      `./run-tests.sh --name "The profile repository carries the organization's default security policy"`;
      paste the non-zero output.

      ```text
      Feature: Publish the organization profile
        Scenario: The profile repository carries the organization's default security policy
         ✔  Given the organization profile repository is published
         ✘  And the pages repository has no security policy of its own
            Step panicked. Captured output: not implemented: The profile repository carries the default security policy: the pages repository has no security policy of its own
      [Summary]
      1 feature
      1 scenario (1 failed)
      2 steps (1 passed, 1 failed)
      ...
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      EXIT:101
      ```
- [x] GREEN — add the assertions and the 404 descriptor; list files; run the
      same command; paste the zero-exit output.

      Files modified: `apps/cli/tests/steps/publish_organization_profile.rs`.
      The `security-applied` fixture descriptor for the pages repository's
      `SECURITY.md` was already written by the profile scenario's base state
      (`repos/duty2cal/duty2cal.github.io/contents/SECURITY.md.json`).

      ```text
      Feature: Publish the organization profile
        Scenario: The profile repository carries the organization's default security policy
         ✔  Given the organization profile repository is published
         ✔  And the pages repository has no security policy of its own
         ✔  When a contributor reads the organization's default security policy
         ✔  Then the organization profile repository carries the default security policy as "SECURITY.md"
         ✔  And the default security policy names the private vulnerability reporting route
         ✔  And the default security policy forbids attaching a real duty plan or a converted calendar
         ✔  And the default security policy states that only the latest published release is supported
         ✔  And the organization profile repository offers no public issue tracker for security reports
         ✔  And the pages repository serves no security policy of its own
      [Summary]
      1 feature
      1 scenario (1 passed)
      9 steps (9 passed)
      EXIT:0
      ```
- [x] REFACTOR — clean up; re-run; paste the zero-exit output.

      ```text
      Feature: Publish the organization profile
        Scenario: The profile repository carries the organization's default security policy
         ✔  Given the organization profile repository is published
         ✔  And the pages repository has no security policy of its own
         ✔  When a contributor reads the organization's default security policy
         ✔  Then the organization profile repository carries the default security policy as "SECURITY.md"
         ✔  And the default security policy names the private vulnerability reporting route
         ✔  And the default security policy forbids attaching a real duty plan or a converted calendar
         ✔  And the default security policy states that only the latest published release is supported
         ✔  And the organization profile repository offers no public issue tracker for security reports
         ✔  And the pages repository serves no security policy of its own
      [Summary]
      1 feature
      1 scenario (1 passed)
      9 steps (9 passed)
      EXIT:0
      ```
- [x] COMMIT: `17336d2da50d56a18440eebf902a8c3b821dd696` — same release-note subject, scenario title in the body; staged only this scenario's files.

## 4. A profile repository that already exists live and is unbound refuses the preview

Design constraints:
- Prime the same base state, then add a `.github` repository body to the
  scenario's live fake (a repository fixture or overlay) without binding it
  in the applied state.
- Run the targeted preview of the three profile resources. The existing
  apply guard refuses a create for an object that already exists live and is
  unbound; the refusal names `github_repository.dot_github` (domain name:
  the organization profile repository).
- Assert no object in the preview is bound, the applied state is unchanged,
  and the live configuration is unchanged. No write is issued.
- This is the profile-specific refusal scenario required in place of a
  blanket deferral of error paths; the shared partial-failure branch remains
  covered by the sibling `enable-repository-security` scenarios.

- [x] RED — remove `@wip` from this scenario only; run
      `./run-tests.sh --name "A profile repository that already exists live and is unbound refuses the preview"`;
      paste the non-zero output.

      ```text
      Feature: Publish the organization profile
        Scenario: A profile repository that already exists live and is unbound refuses the preview
         ✘  Given the applied state binds every declared object except the organization profile repository and its two files
            Step panicked. Captured output: not implemented: A profile repository that already exists live and is unbound refuses the preview: the applied state binds every declared object except the profile
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      ...
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      EXIT:101
      ```
- [x] GREEN — add the fake fixture/overlay, the preview guard assertion and
      the no-write assertions; list files; run the same command; paste the
      zero-exit output.

      Files modified: `apps/cli/tests/steps/publish_organization_profile.rs`.
      The preview guard is implemented in the profile preview: it evaluates
      the targeted plan's creates and reads
      `GET /repos/duty2cal/.github`; the live fixture body is written in the
      scenario. The journal check reads only lines after `journal_offset` and
      asserts no `applied_write`/`refused_write`.

      ```text
      Feature: Publish the organization profile
        Scenario: A profile repository that already exists live and is unbound refuses the preview
         ✔  Given the applied state binds every declared object except the organization profile repository and its two files
         ✔  And the live organization already has the organization profile repository
         ✔  And the declaration adds the organization profile repository, its profile content and its default security policy
         ✔  When the organization owner previews the declaration
         ✔  Then the preview is refused and names the organization profile repository
         ✔  And no object in the preview is bound
         ✔  And the live configuration is unchanged
      [Summary]
      1 feature
      1 scenario (1 passed)
      7 steps (7 passed)
      EXIT:0
      ```
- [x] REFACTOR — clean up; re-run; paste the zero-exit output.

      ```text
      Feature: Publish the organization profile
        Scenario: A profile repository that already exists live and is unbound refuses the preview
         ✔  Given the applied state binds every declared object except the organization profile repository and its two files
         ✔  And the live organization already has the organization profile repository
         ✔  And the declaration adds the organization profile repository, its profile content and its default security policy
         ✔  When the organization owner previews the declaration
         ✔  Then the preview is refused and names the organization profile repository
         ✔  And no object in the preview is bound
         ✔  And the live configuration is unchanged
      [Summary]
      1 feature
      1 scenario (1 passed)
      7 steps (7 passed)
      EXIT:0
      ```
- [x] COMMIT: `3b4cf5b778a796165d5e791f2da6aba2b51098c2` — same release-note subject, scenario title in the body; staged only this scenario's files.

## E2E setup — before the first @e2e scenario

- [x] Bring up the local environment from the design's Local Runnability
      section: run `./scripts/setup-infra-tests.sh` (provider mirror,
      `terraformrc`, fixture proof) and confirm the fake starts on an
      ephemeral loopback port with `--writes`; no container or host service.

      ```text
      $ ./scripts/setup-infra-tests.sh
      jq-1.8.2
      provider integrations/github 6.13.0 for linux_amd64
      mirror <cache>/terraform-provider-mirror
      terraformrc <cache>/terraformrc
      ...
      refresh-only plan (adopt-all) on the unchanged fake: No changes.
      unknown paths served: 0
      fixtures scripts/fake-github/fixtures/adopt-all proven for 5 objects
      ...
      base declaration apply: 0 live changes (follow-up plan: No changes.)
      refresh-only plan (security-applied-security-adopted) on the unchanged fake: No changes.
      unknown paths served: 0
      fixtures scripts/fake-github/fixtures/security-applied proven for 5 base + 2 security objects
      EXIT:0
      ```

      The fake child starts on an ephemeral loopback port in `--writes` mode
      from `start_fake_with(..., true)` and is killed by `DutyWorld`'s `Drop`.
- [x] Confirm the e2e step module
      `apps/cli/tests/steps/e2e/publish_organization_profile.rs` exists and
      every e2e step starts as `unimplemented!`.

      ```text
      $ git show cc32948:apps/cli/tests/steps/e2e/publish_organization_profile.rs | grep -c 'unimplemented!'
      11
      ```

      One stub per step of the two `@e2e` scenarios (7 + 4). The module is
      declared in `apps/cli/tests/steps/e2e/mod.rs` as
      `pub mod publish_organization_profile;`.
- [x] Prove `verify.e2e_command` is a strict subset: run `./run-tests.sh`
      (with the new scenarios still `@wip`, or record the current full count)
      and `./run-tests.sh --e2e`; paste both scenario counts; the e2e count
      must be strictly less than the full count.

      ```text
      $ ./run-tests.sh --tags 'not @wip'
      [Summary]
      6 features
      149 scenarios (149 passed)
      640 steps (640 passed)
      EXIT:0
      ```

      ```text
      $ ./run-tests.sh --tags '@e2e and not @wip'
      [Summary]
      5 features
      7 scenarios (7 passed)
      51 steps (51 passed)
      EXIT:0
      ```

      The e2e count is strictly smaller; the e2e command is the full runner
      restricted to `@e2e`. The pinned cucumber-rs 0.23.0 rejects
      `--tags` together with `--name` ("the argument '--tags <tagexpr>' cannot
      be used with '--name <regex>'"), so each targeted single-scenario run
      below is `./run-tests.sh --name "<title>"`; the name filter selects only
      that scenario, which carries `@e2e`.
- [x] Confirm the e2e runner reaches the fake: a targeted e2e run of the apply
      scenario with stubs fails at the first step, not at infrastructure.
      Paste the output.

      ```text
      $ ./run-tests.sh --name "Organization owner applies the previewed revision and the organization profile is published"
      Feature: Publish the organization profile
        Scenario: Organization owner applies the previewed revision and the organization profile is published
         ✔  Given the applied state binds every declared object except the organization profile repository and its two files
         ✘  And a previewed revision that declares the organization profile repository, its profile content and its default security policy
            Step panicked. Captured output: not implemented: Organization owner applies the previewed revision: a previewed revision that declares the profile objects
      [Summary]
      1 feature
      1 scenario (1 failed)
      2 steps (1 passed, 1 failed)
      ...
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      EXIT:101
      ```

      The priming Given (adoption batches, the base declaration's real
      `tofu init`/`plan`/`apply`) passed before the stub panicked, so the
      run reached the fake and the real binary, not an infrastructure fault.

Design constraints: e2e steps live in the separate `steps/e2e/` module; the
transcript path is
`givn/changes/publish-organization-profile/evidence/visual/<slug>/transcript.txt`;
the transcript is scrubbed of the scenario directory, port, token and
passphrase; the fake is killed with `DutyWorld`.

No commit for this setup task.

## 5. @e2e Organization owner applies the previewed revision and the organization profile is published

Design constraints:
- Drive the real `tofu init`, targeted `plan -out`, `show -json`, targeted
  `refresh-only` preflight and `apply plan.bin` against the fake in
  `--writes` mode. The base state is primed by the adoption batches and the
  base apply; `journal_offset` excludes those writes.
- Primary assertion: the fake's post-apply `GET /repos/duty2cal/.github`,
  the two contents responses, and the journal's applied-write set (one
  `POST /orgs/duty2cal/repos`; the create's `PATCH`es; exactly two `PUT`s,
  one per contents path). The repository is public, has issues/projects/wiki
  disabled, and the files carry the reviewed bytes. The applied state records
  the three addresses. A targeted follow-up preview reports no differences.
- No refused write and no unknown path in the scenario's journal. The two
  `PUT`s may be in either order. Commit the scrubbed transcript.
- Do not remove the `@e2e` tag.

- [x] RED — remove `@wip` from this scenario only; run
      `./run-tests.sh --name "Organization owner applies the previewed revision and the organization profile is published"`;
      paste the non-zero output. (The pinned runner rejects `--tags` with
      `--name`; the name alone selects the `@e2e` scenario.)

      ```text
      $ ./run-tests.sh --name "Organization owner applies the previewed revision and the organization profile is published"
      Feature: Publish the organization profile
        Scenario: Organization owner applies the previewed revision and the organization profile is published
         ✔  Given the applied state binds every declared object except the organization profile repository and its two files
         ✘  And a previewed revision that declares the organization profile repository, its profile content and its default security policy
            Step failed:
            Defined: tmp/tmp.acUfv05qWT/f0006.feature:8:5
            Matched: apps/cli/tests/steps/e2e/publish_organization_profile.rs:12:1
            Step panicked. Captured output: not implemented: Organization owner applies the previewed revision: a previewed revision that declares the profile objects
      [Summary]
      1 feature
      1 scenario (1 failed)
      2 steps (1 passed, 1 failed)
      ...
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      EXIT:101
      ```
- [x] GREEN — implement the e2e steps, the API reads and the transcript;
      list files; run the same command; paste the zero-exit output.

      Files modified: `apps/cli/tests/steps/e2e/publish_organization_profile.rs`
      (scenario 5 steps, journal reads, transcript commit),
      `apps/cli/tests/steps/publish_organization_profile.rs` (`source_tree`
      and `published_file` exposed as `pub(crate)` only), and the delta
      feature's `@wip` removal. The transcript is
      `givn/changes/publish-organization-profile/evidence/visual/organization-owner-applies-the-previewed-revision-and-the-organization-profile-is-published/transcript.txt`.

      ```text
      $ ./run-tests.sh --name "Organization owner applies the previewed revision and the organization profile is published"
      Feature: Publish the organization profile
        Scenario: Organization owner applies the previewed revision and the organization profile is published
         ✔  Given the applied state binds every declared object except the organization profile repository and its two files
         ✔  And a previewed revision that declares the organization profile repository, its profile content and its default security policy
         ✔  When the organization owner applies the previewed revision
         ✔  Then the apply reports the organization profile repository and both files as created
         ✔  And the organization profile repository is public and offers no issue tracker, projects or wiki
         ✔  And the organization profile repository serves the reviewed profile content as "profile/README.md"
         ✔  And the organization profile repository serves the default security policy as "SECURITY.md"
         ✔  And the applied state records the organization profile repository and both files
         ✔  And a later targeted preview of the organization profile reports no differences
      [Summary]
      1 feature
      1 scenario (1 passed)
      9 steps (9 passed)
      EXIT:0
      ```
- [x] REFACTOR — clean up; re-run; paste the zero-exit output.

      Removed the unused `tofu` import and the unused plan-change filter (both
      belong to scenario 6, which reintroduces the filter when it is
      implemented). The real-binary, journal and transcript behaviour is
      unchanged.

      ```text
      $ ./run-tests.sh --name "Organization owner applies the previewed revision and the organization profile is published"
      Feature: Publish the organization profile
        Scenario: Organization owner applies the previewed revision and the organization profile is published
         ✔  Given the applied state binds every declared object except the organization profile repository and its two files
         ✔  And a previewed revision that declares the organization profile repository, its profile content and its default security policy
         ✔  When the organization owner applies the previewed revision
         ✔  Then the apply reports the organization profile repository and both files as created
         ✔  And the organization profile repository is public and offers no issue tracker, projects or wiki
         ✔  And the organization profile repository serves the reviewed profile content as "profile/README.md"
         ✔  And the organization profile repository serves the default security policy as "SECURITY.md"
         ✔  And the applied state records the organization profile repository and both files
         ✔  And a later targeted preview of the organization profile reports no differences
      [Summary]
      1 feature
      1 scenario (1 passed)
      9 steps (9 passed)
      EXIT:0
      ```
- [x] COMMIT: `ebe14aea0fec62c6898d0db8a00c31be60087675` — `test(e2e): <release note>` with the scenario title in the body; staged only this scenario's files.

Post-review strengthening (2026-10-18): the apply journal assertion now
requires the unique `PATCH` path set to be exactly
`/repos/duty2cal/.github`, not just the first sorted path. The single-scenario
re-run passes 9/9 and the transcript is refreshed. Revision:
`3322b38ae7109b50d8968f0b2f989c41f641efcb`.

## 6. @e2e Organization owner previews the profile declaration and only the three creations are named

Design constraints:
- Drive the real targeted `tofu plan -out` and `tofu show -json`; assert the
  plan's `resource_changes` contains exactly the three creates
  (`github_repository.dot_github`, `github_repository_file.org_profile_readme`,
  `github_repository_file.org_security`), no update or delete, and no live
  write. The live fake and the applied state are unchanged.
- The preview scope is the revision's objects; it is not justified by
  unrelated drift. The base fixture's organization billing value matches the
  declaration.
- Commit the scrubbed transcript. Do not remove the `@e2e` tag.

- [x] RED — remove `@wip` from this scenario only; run
      `./run-tests.sh --name "Organization owner previews the profile declaration and only the three creations are named"`;
      paste the non-zero output. (Same `--tags`/`--name` runner limitation as
      scenario 5.)

      ```text
      $ ./run-tests.sh --name "Organization owner previews the profile declaration and only the three creations are named"
      Feature: Publish the organization profile
        Scenario: Organization owner previews the profile declaration and only the three creations are named
         ✔  Given the applied state binds every declared object
         ✘  And the live organization has no organization profile repository
            Step failed:
            Defined: tmp/tmp.TdSyYddeAR/f0006.feature:20:5
            Matched: apps/cli/tests/steps/e2e/publish_organization_profile.rs:310:1
            Step panicked. Captured output: not implemented: Organization owner previews the profile declaration:
                     the live organization has no profile repository
      [Summary]
      1 feature
      1 scenario (1 failed)
      2 steps (1 passed, 1 failed)
      ...
      error: test failed, to rerun pass `-p duty2cal --test features_runner`
      EXIT:101
      ```
- [x] GREEN — implement the e2e preview steps and transcript; list files; run
      the same command; paste the zero-exit output.

      Files modified: `apps/cli/tests/steps/e2e/publish_organization_profile.rs`
      (scenario 6 plan filters, journal and state invariants, transcript
      commit), `apps/cli/tests/steps/publish_organization_profile.rs`
      (`write_profile_descriptors` exposed as `pub(crate)` only), and the delta
      feature's `@wip` removal. The transcript is
      `givn/changes/publish-organization-profile/evidence/visual/organization-owner-previews-the-profile-declaration-and-only-the-three-creations-are-named/transcript.txt`.

      ```text
      $ ./run-tests.sh --name "Organization owner previews the profile declaration and only the three creations are named"
      Feature: Publish the organization profile
        Scenario: Organization owner previews the profile declaration and only the three creations are named
         ✔  Given the applied state binds every declared object
         ✔  And the live organization has no organization profile repository
         ✔  And the declaration adds the organization profile repository, its profile content and its default security policy
         ✔  When the organization owner previews the declaration
         ✔  Then the preview names the organization profile repository as the only object to create
         ✔  And the preview names the profile content and the default security policy as the only files to create
         ✔  And the preview names no change other than the three creations
         ✔  And the live configuration is unchanged
      [Summary]
      1 feature
      1 scenario (1 passed)
      8 steps (8 passed)
      EXIT:0
      ```
- [x] REFACTOR — clean up; re-run; paste the zero-exit output.

      `cargo fmt --all` applied the workspace style to the new module; the
      real-binary plan, journal, state and transcript behaviour is unchanged.

      ```text
      $ ./run-tests.sh --name "Organization owner previews the profile declaration and only the three creations are named"
      Feature: Publish the organization profile
        Scenario: Organization owner previews the profile declaration and only the three creations are named
         ✔  Given the applied state binds every declared object
         ✔  And the live organization has no organization profile repository
         ✔  And the declaration adds the organization profile repository, its profile content and its default security policy
         ✔  When the organization owner previews the declaration
         ✔  Then the preview names the organization profile repository as the only object to create
         ✔  And the preview names the profile content and the default security policy as the only files to create
         ✔  And the preview names no change other than the three creations
         ✔  And the live configuration is unchanged
      [Summary]
      1 feature
      1 scenario (1 passed)
      8 steps (8 passed)
      EXIT:0
      ```
- [x] COMMIT: `6c46ce006c91e7d4fa5d900122acf31f360c89c2` — `test(e2e): <release note>`, scenario title in the body; staged only this scenario's files.

## Final verification

- [x] `./run-tests.sh` exits 0 over the full suite; paste the summary.

      ```text
      $ ./run-tests.sh
      [Summary]
      6 features
      150 scenarios (150 passed)
      648 steps (648 passed)
      EXIT:0
      ```
- [x] `./run-tests.sh --e2e` exits 0 with a strictly smaller scenario count;
      paste the summary.

      ```text
      $ ./run-tests.sh --e2e
      [Summary]
      5 features
      8 scenarios (8 passed)
      59 steps (59 passed)
      EXIT:0
      ```

      The e2e count (8) is strictly smaller than the full count (150).
- [x] `givn lint --change publish-organization-profile` has no errors; paste
      the output.

      ```text
      $ givn lint --change publish-organization-profile
      givn lint: 1 file(s) checked — clean
      EXIT:0
      ```
- [x] `bash scripts/license-headers.sh`, `reuse lint`,
      `bash scripts/public-text-check.sh`, `bash scripts/pii-guard.sh`,
      `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings`
      pass (or record the project's actual full check set and any
      environment-only skip).

      ```text
      $ bash scripts/license-headers.sh
      license-headers: clean
      EXIT:0

      $ nix-shell -p reuse --run 'reuse lint'
      # SUMMARY
      * Bad licenses: 0
      * Deprecated licenses: 0
      * Licenses without file extension: 0
      * Missing licenses: 0
      * Unused licenses: 0
      * Used licenses: CC0-1.0, MPL-2.0, Unicode-3.0
      * Read errors: 0
      * Invalid SPDX License Expressions: 0
      * Files with copyright information: 348 / 348
      * Files with license information: 348 / 348

      Congratulations! Your project is compliant with version 3.3 of the REUSE Specification :-)
      EXIT:0

      $ bash scripts/public-text-check.sh
      public-text-check: clean - all 14 public-text promises present in <repo>/README.md
      EXIT:0

      $ bash scripts/pii-guard.sh
      pii-guard: clean
      EXIT:0

      $ cargo fmt --all --check
      EXIT:0

      $ cargo clippy --workspace --all-targets -- -D warnings
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.95s
      EXIT:0
      ```

      No environment-only skip: `nix-shell -p reuse --run 'reuse lint'`
      fetched and ran REUSE 3.3 in this environment.
- [x] `./scripts/setup-infra-tests.sh` passes after the `org_profile.tf`
      exclusions; paste the fixture proof output.

      ```text
      $ ./scripts/setup-infra-tests.sh
      ...
      refresh-only plan (adopt-all) on the unchanged fake: No changes.
      unknown paths served: 0
      fixtures scripts/fake-github/fixtures/adopt-all proven for 5 objects
      ...
      refresh-only plan (security-applied-adopted) on the unchanged fake: No changes.
      base declaration apply: 0 live changes (follow-up plan: No changes.)
      import ok github_repository_vulnerability_alerts.mono duty2cal
      import ok github_repository_dependabot_security_updates.mono duty2cal
      refresh-only plan (security-applied-security-adopted) on the unchanged fake: No changes.
      unknown paths served: 0
      fixtures scripts/fake-github/fixtures/security-applied proven for 5 base + 2 security objects
      EXIT:0
      ```
- **Operator step (archive-time obligation, not a task):** the maintainer
  enables private vulnerability reporting on the monorepo, runs the real
  targeted apply with the passphrase, and commits `infra/terraform.tfstate`
  and its backup only after a green apply. The operator also pins
  `duty2cal` and `duty2cal.github.io` in the organization settings. This
  cannot be performed in CI or without the passphrase; it is recorded in
  the change review and the deployment chapter, not ticked as a task.
