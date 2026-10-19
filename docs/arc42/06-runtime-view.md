# 6. Runtime View

## Scenario: Successful conversion

**Trigger:** A crew member supplies a supported duty plan.

```mermaid
sequenceDiagram
  participant CLI
  participant App as ConvertRoster
  participant PDF as PDF extraction
  participant Dom as Domain
  participant FS as Filesystem publication
  CLI->>App: convert(source path)
  App->>PDF: read source, build snapshot
  PDF-->>App: positioned content + revision footer
  App->>Dom: classify, resolve dates, compose descriptions
  Dom-->>App: complete Roster (or rejection)
  App->>FS: allocate unused calendar path + write
  FS-->>App: published path
  App-->>CLI: Published { path }
  CLI-->>CLI: print the new path, exit 0
```

**Steps:**

1. The CLI reads the source path and calls the orchestrator.
2. The orchestrator recognizes the family, builds the source snapshot, and extracts positioned content and the revision footer.
3. The domain classifies activities, resolves each UTC endpoint's date, composes descriptions and events, and enforces completeness.
4. On completeness, the orchestrator asks the publication port for an unused calendar path and writes the calendar.
5. The CLI prints the actual new path and exits zero.

**Notes:** No side effect occurs before completeness; the source file is never modified.

## Scenario: Interpretation failure with a diagnostic

**Trigger:** A required activity, date, fragment, or source family cannot be resolved.

```mermaid
sequenceDiagram
  participant CLI
  participant App as ConvertRoster
  participant Dom as Domain
  participant Diag as Diagnostic writer
  CLI->>App: convert(source path)
  App->>Dom: resolve
  Dom-->>App: rejection reason
  App->>Diag: allocate unused numbered report + write
  Diag-->>App: report path (or failure)
  App-->>CLI: Rejected { reason, diagnostic }
  CLI-->>CLI: print failure, exit non-zero
```

**Steps:**

1. The orchestrator reaches a rule that cannot be resolved.
2. It returns the primary reason and attempts a numbered diagnostic naming the source and reason.
3. The CLI prints the failure; when a report could not be saved, it states that no new report was written and never presents an older report as current.

**Notes:** Existing calendars and reports are unchanged. The diagnostic path is reported only after a successful write.

## Scenario: Publication failure

**Trigger:** Activities resolve, but the complete calendar cannot be written.

```mermaid
sequenceDiagram
  participant CLI
  participant App as ConvertRoster
  participant FS as Filesystem publication
  participant Diag as Diagnostic writer
  CLI->>App: convert(source path)
  App->>FS: publish complete calendar
  FS-->>App: write failure
  App->>Diag: attempt numbered diagnostic
  Diag-->>App: report path or failure
  App-->>CLI: Rejected { reason, diagnostic }
  CLI-->>CLI: print failure, exit non-zero
```

**Steps:**

1. The calendar is composed but cannot be published.
2. The orchestrator exposes the failure and independently attempts a diagnostic.
3. The CLI reports the failure and the diagnostic outcome; existing files are untouched.

**Notes:** Completeness and publication are distinct: a completed composition that fails to publish is not a success.

## Scenario: A version tag becomes a verified published release

**Trigger:** The maintainer decides a version is ready and pushes its `v*` tag.

```mermaid
flowchart TB
  tag["Maintainer pushes a v* tag"] --> rel["release.yml"]
  rel --> build["build on ubuntu-latest: one Windows x86_64 executable, cross-built for x86_64-pc-windows-gnu"]
  rel --> smoke["smoke-windows on windows-latest: start the cross-built duty2cal.exe"]
  build --> prep["prepare on ubuntu-latest: render and stage the change log and the release text, then run scripts/prepare-release.sh over the one Windows binary"]
  smoke --> prep
  prep -->|"incomplete set, no visible change, or change-log drift"| nostage["nothing staged, no release"]
  prep --> pub["publish on ubuntu-latest: resolve the release-workflow check suites, then wait for the tagged revision's checks"]
  pub -->|"non-success, no success, no evidence, unreadable source, or the bound"| nopub["nothing public"]
  pub -->|"every check concluded, at least one success"| stage["generate the SBOM, attest every archive, create and inspect the draft, then publish"]
  stage --> ver["verify: download the assets back, compare checksums, run gh attestation verify, assert immutable"]
  ver --> done["verified immutable release, reachable at the stable latest address"]
```

**Steps:**

1. The tag starts `release.yml`; the build job cross-builds the one Windows x86_64 executable (`x86_64-pc-windows-gnu`) on `ubuntu-latest`, and the Windows smoke job starts that executable on `windows-latest`.
2. The `prepare` job installs the pinned generator, renders the change log and the release text (the newest section of that change log) and runs `scripts/prepare-release.sh` over the one Windows binary; it refuses and stages nothing when that executable is missing or unreadable, when an executable is presented for a platform outside the supported set (including a previously offered Linux or macOS target), when the render would name no change a user receives, or when the committed change log differs from the released revision.
3. The `publish` job resolves every check suite the release workflow created for the tagged revision and invokes `scripts/await-green-checks.sh`; the gate waits, bounded, for the remaining check runs to conclude. It refuses a concluded non-success, a set with no successful check, no check outside the release-workflow suites, an unreadable check-run source, or a check still running at the bound, naming the checks. Nothing later in the job runs on a refusal.
4. After the gate passes, the `publish` job generates the SBOM, attests the archive, creates a draft release with the Windows archive, the release text and the change log, checks the draft's asset set against the staged set, and publishes it.
5. The `verify` job reads the published release back: it compares the checksum, runs `gh attestation verify` on the archive, and asserts the release reports `immutable: true`.

**Notes:** Publication cannot be undone, so every check that can precede publication does, and the one that cannot — reading the public record — fails the run loudly.

## Scenario: A version's section is rendered from the released history

**Trigger:** The maintainer generates the change log for a version.

```mermaid
flowchart TB
  hist["Repository history: conventional-commit subjects, v* tags"] --> render["scripts/render-changelog.sh --version <v> --repo <dir> --out <path>"]
  pin["pinned git-cliff 2.14.2 recorded beside cliff.toml"] --> tool{"the generator is the pinned version?"}
  corpus["use-case ids under givn/specs/"] --> allow{"the cliff.toml allow-list matches the corpus?"}
  config["cliff.toml: group order, skip rules (allow-list and machinery), templates"] --> cliff
  config --> machinery{"the marked machinery skip list is realized in the configuration?"}
  tool -->|"no"| refuse["refuse, naming the cause; output untouched"]
  allow -->|"no"| refuse
  machinery -->|"no"| refuse
  render --> released["the released revision: the v<version> tag, or HEAD before the tag exists"]
  released --> cliff["git-cliff renders the history with the committed configuration, ordered by history; the renderer extracts the released version's section"]
  cliff --> sect["Newest section: version, its own date, comparison link, grouped released changes"]
  sect -->|"names no change a user receives"| refuse
  sect --> log["CHANGELOG.md: new section prepended, earlier sections verbatim"]
  sect --> text["release-notes.md: the newest section alone"]
  log --> drift{"--check: fresh render equals the committed change log?"}
  drift -->|"differs"| refuse
  drift -->|"matches"| ok["drift check passes"]
```

**Steps:**

1. The version names a released revision: the `v<version>` tag when it exists, and otherwise `HEAD`, the revision the tag will point at. The renderer refuses a leading `v`, and a repository with no `HEAD` to take the section's date from; the date it stamps is that revision's own committer date, never the clock, which is what lets a section be rendered before its tag exists.
2. The pinned generator, the capability allow-list, the machinery skip list and the list of paths a user receives are checked before anything is rendered: a `git-cliff` whose version differs from the pinned one, a `cliff.toml` allow-list that differs from the use-case ids under `givn/specs/`, a marked machinery skip list whose terms the configuration does not realize, or a listed user-receiving path the tree does not hold refuses and leaves the output untouched.
3. The generator renders the history — from the previous `v*` tag to the released revision, or the whole history for the first version — ordered by history, and the renderer extracts the released version's section from it, with each entry stating the release-note words its subject carries. A version that is not tagged yet is rendered with `--tag v<version>` for the newest release, which the generator cannot name otherwise.
4. The section states the released revision's own date and the comparison against the previous version (none for the first), groups the changes a user receives (breaking changes, new capabilities, fixes, performance, reversals, dependency updates) and skips housekeeping — except a dependency update that changed a path a user receives — `test`, `build`, `ci`, every scope outside the allow-list and every revision whose words name the project's own tooling. Each surviving entry is then resolved to its revision, and one whose revision changed none of the paths a user receives — the converter, its build inputs, and the packaging and release workflow that decide what an archive holds — is left out. A `### ` group heading whose next non-blank line is another heading or the section end is then pruned, so a group emptied by the path filter does not appear. A render that names no change a user receives refuses the version as not releasable.
5. The change log is written with the new section prepended and every earlier section kept verbatim, and the release text is the newest section alone — one render, so the page and the repository cannot disagree. `--check` renders in memory and, on a committed file that differs, refuses and names the version whose section differs.

**Notes:** The render is deterministic: ordering is history order, the date is the released revision's own, and no working-directory path, user name or runner name enters the text, so the same history renders the same bytes on any machine.

## Scenario: A pushed change reaches the gated default branch

**Trigger:** A change is pushed, or a pull request is opened.

```mermaid
flowchart TB
  push["Maintainer pushes a change or opens a pull request"] --> ci["ci.yml: formatting, lint, tests, dependency licences, SPDX headers, third-party notices drift, change-log drift, private-data guard, coverage, Windows cross-build, public text, actionlint; dco on pull requests"]
  ci -->|"any check fails"| blocked["the change cannot land"]
  ci -->|"all checks pass"| rules{"required ruleset on main"}
  rules -->|"checks green"| land["the change lands on the default branch"]
  rules -->|"maintainer bypass"| land
```

**Steps:**

1. Every push and pull request runs the project's checks; the pull-request-only `dco` job additionally checks sign-off over the commit range.
2. A change whose checks are not green cannot land: the default branch's ruleset requires the CI status checks and blocks force pushes and deletion.
3. The maintainer keeps an explicit bypass route for an urgent correction.

**Notes:** The gate is enforced by the repository, not by habit, and a reader sees the outcome on the revision they are looking at.

## Scenario: A pushed revision gains a size attribution

**Trigger:** A revision is pushed or a pull request is opened; the release maintainer may request the same measurement for any named revision.

```mermaid
flowchart TB
  rev["Named revision: the pushed revision, or a revision the maintainer names"] --> wt["temporary worktree at that revision"]
  wt --> ship["shipped build: the committed build configuration (strip = true)"]
  ship --> exe["stripped duty2cal.exe: file size, digest, section sizes"]
  wt --> attr["build retaining the names: strip disabled for this build only, separate target directory"]
  attr --> tool{"the build fails, or the generator reads no names?"}
  tool -->|"yes"| none["report: no attribution; names what could not be read; the report is published and the job continues"]
  tool -->|"no"| split["crate split, function split, .text total"]
  exe --> report["attribution.json + attribution.md: revision, platform, build identity"]
  split --> report
  report -->|"every pushed revision"| art["the run's size-attribution artifact and step summary"]
  exe -.->|"file size cannot be read"| incomplete["report: incomplete; states the file size is unavailable; never uses the split's total"]
```

**Steps:**

1. The measurement resolves the named revision into a temporary worktree, so it never describes the working tree, and names the supported platform `x86_64-pc-windows-gnu`.
2. It builds the shipped artifact with the committed build configuration and reads that stripped executable's file size, digest and section sizes.
3. It builds the same revision again with `strip` disabled for that build only, in a separate target directory, and reads the crate split, the function split and that build's `.text` total.
4. It writes one report — a machine-readable file and a human rendering of the same data — naming the requested revision, the resolved revision, the platform and the build's identity, with the split and the stripped executable's file size kept as separate numbers.
5. The Windows cross-build job produces the report for the revision it built and publishes it beside that run; the release maintainer can run the same command for any named revision.

**Notes:** A failed build or a build that keeps no names yields no attribution and names what could not be read; the report is still written, appended to the step summary and uploaded, and the job continues — the measurement is evidence, not a gate. A stripped executable whose file size cannot be read leaves the attribution explicitly incomplete and never presents the split's total as the file size. No number is inferred from the release archive or from a host build, and the shipped build configuration is unchanged. Scenario "A pushed change reaches the gated default branch" still decides whether the revision lands; the attribution does not refuse a revision or a release.

## Scenario: The existing public home is adopted into the declared configuration

**Trigger:** The maintainer asks to adopt the selected batch of objects that already exist on GitHub.

```mermaid
flowchart TB
  owner["Organization owner"] --> cmd["scripts/adopt-public-home.sh --declaration infra --batch infra/adoption-batch.txt --state infra/terraform.tfstate"]
  cmd --> pre{"tofu/python3 present, passphrase valid, batch well formed, state inside declaration?"}
  pre -->|"no"| refuse["refuse; name the cause; nothing read from GitHub"]
  pre -->|"yes"| stage["stage the declaration and state; tofu init -backend=false; tofu validate; tofu state list decrypts the state"]
  stage -->|"invalid declaration or wrong passphrase"| refuse
  stage --> imp["tofu import each batch address in order (reads only, against GitHub)"]
  imp -->|"already bound, missing live, undeclared, unreadable or unwritable"| refuseobj["discard the staged state; refuse; name the object; live unchanged"]
  imp --> verify["tofu plan -refresh-only -detailed-exitcode"]
  verify -->|"a bound object changed since the import"| refuseobj
  verify -->|"no change"| bind["move the staged state over infra/terraform.tfstate; create the first backup as a copy, else keep the previous state"]
  bind --> out["one 'adopted <address> <id>' line per object"]
```

**Steps:**

1. The command checks that `tofu` and `python3` exist, that the passphrase is present and at least 16 characters, that the batch is well formed, and that `--state` lies inside `--declaration`. Nothing is read from GitHub and nothing in the repository is written before these pass.
2. It copies the declaration into a staging workspace and the current encrypted state to that workspace's root `terraform.tfstate`, so `infra/` is untouched while the batch runs. It then runs `tofu init -backend=false` and `tofu validate` inside the staged copy — a fresh checkout has no `infra/.terraform/`, so init comes first. An invalid declaration discards the staging directory and names the declaration error.
3. `tofu state list` in the staged workspace proves the state decrypts before any import; a state encrypted with a different passphrase refuses here and names the applied state.
4. Each `address<TAB>import-id` line is imported in order. `tofu import` reads the live object; it creates, modifies and deletes nothing. An object already managed by the copied state, an import target the declaration does not name, a live object that does not exist, an object the token cannot read, or a state that cannot be written stops the batch: the staged state is discarded and the command refuses, naming the object and the reason.
5. A refresh-only plan over the staged state compares the imported objects with GitHub again; a non-no-op change means a live object changed after it was read, so the batch is refused and the object named. With no change, the staged state and the live objects agree.
6. The staged state is moved over `infra/terraform.tfstate`; on the first adoption `<state>.backup` is created as a copy of the new state, and on later runs the previous state is kept as the backup. The command prints one `adopted` line per object.

**Notes:** Adoption is read-only against GitHub by construction; the suite proves it from the fake API's request journal: a refused write or a GraphQL mutation would be the evidence of a write, while a query-only POST is a read. Every refusal step also asserts that the journal holds no unknown-path entry, so a refusal is never the fake's own missing fixture. The declaration revision is the repository commit, so the command never edits the declaration.

## Scenario: The declared security settings are previewed and applied

**Trigger:** The maintainer has declared the monorepo's security settings and
asks for a preview, then applies the previewed revision.

```mermaid
flowchart TB
  owner["Organization owner"] --> adopt["adopt the two Dependabot settings (read-only)"]
  adopt -->|"a batch object is unreadable, already bound, or not live"| stopnone["refuse; name the setting; nothing bound, nothing live changed"]
  adopt -->|"whole-state refresh sees unrelated drift"| direct["import the two settings directly by repository-name ids"]
  adopt --> plan["tofu plan -out=plan.bin"]
  direct --> plan
  plan -->|"cannot read a setting, wrong passphrase, no authority"| stopnone2["refuse; name the object; nothing written"]
  plan --> guard["preview guard (full-declaration plan): no create for a setting already live and unbound"]
  guard -->|"an unadopted live setting would be created"| stopguard["refuse; name the setting; adopt it first; nothing written"]
  guard --> read["the plan names the two scanning statuses and no other difference"]
  read --> preflight["read-only refresh-only preflight on the applied state"]
  preflight -->|"a manual edit changed a live object"| stophange["refuse; name the object; the saved plan is not applied"]
  preflight --> apply["tofu apply plan.bin"]
  apply -->|"an earlier setting applied and a later one failed"| partial["exit non-zero; name the applied settings and the failed address; write terraform.tfstate.stale; do not commit the partial state"]
  apply --> patch["one PATCH: secret scanning and push protection enabled"]
  patch --> back["the provider reads the repository back"]
  back -->|"the read fails"| err["the apply reports the repository and the reason; live is changed, the state is not reported as applied"]
  back --> recorded["the encrypted state records both statuses enabled"]
  recorded --> again["targeted tofu plan: no differences"]
  again --> commit["commit the declaration and the state in one revision"]
```

**Steps:**

1. Before anything else, a `terraform.tfstate.stale` marker is checked; while
   it exists the preview and the apply refuse and name the stale state. The two
   Dependabot settings are then adopted with the second batch: the existing
   adoption command binds them read-only, so they are not reported as
   creations. A refusal binds nothing. If that command refuses because its
   whole-state refresh sees a pre-existing drift elsewhere, the two settings
   are imported directly by their repository-name ids, which still satisfies
   adopt-before-create.
2. The targeted security preview reads the encrypted state, the declaration
   and the live configuration. On this declaration it names the two scanning
   statuses as the only live difference and proposes nothing to create or
   delete. A `create`
   for a setting that already exists live and is unbound is refused by address
   before any write. It writes nothing.
3. The previewed revision is saved to `plan.bin` (encrypted, git-ignored). A
   read-only `tofu plan -refresh-only -detailed-exitcode -json
   -target=github_repository.mono` immediately before the apply names a live
   edit made since the preview and refuses; its JSON report is authoritative,
   not its exit status. The scope is the previewed object, so the pre-existing
   organization billing drift outside it neither refuses the apply nor marks
   the state stale. With no change in the
   security scope, `tofu apply plan.bin`
   applies exactly the previewed plan.
4. The provider charges the declared change to GitHub with one repository
   `PATCH`, reading the repository back afterwards. Success enables both
   settings; the failed read is reported as a failure of the repository and the
   state does not claim the statuses.
5. The encrypted state records the enabled statuses; a second plan targeted to
   the security scope reports no differences, the plan file and the stale
   marker are removed, and the declaration and state are committed together.
   The full preview still names the pre-existing organization billing drift, a
   separate issue outside this change; the full configuration is not clean.

**Notes:** The suite drives steps 2–4 against the fake with a write sandbox and
asserts the journal's single applied write and its body keys; the live API's
acceptance of push protection is proven only by the operator's real apply,
which is a required step after GREEN. A partial failure is reported by resource
address, writes the stale marker beside the state, and the state is
deliberately not committed.

## Scenario: The organization profile is previewed, applied and published

**Trigger:** The maintainer has declared the organization profile repository and
its two reviewed files and asks for a preview, then applies the previewed
revision.

```mermaid
flowchart TB
  owner["Organization owner"] --> plan["targeted tofu plan -out=plan.bin; tofu show -json"]
  plan -->|"the declaration cannot be read"| stop["refuse; name the declaration problem; nothing written"]
  plan -->|"the profile repository is absent live"| creates["the plan names three creates: the repository, profile/README.md, SECURITY.md"]
  creates --> preflight["read-only refresh-only preflight"]
  preflight -->|"a live edit since the preview"| stop
  preflight --> apply["tofu apply plan.bin"]
  apply --> post["POST /orgs/duty2cal/repos"]
  post --> patch["PATCH /repos/duty2cal/.github (create follow-up, possibly a visibility-only second PATCH)"]
  patch --> read["GET /repos/duty2cal/.github (+ vulnerability alerts)"]
  read --> files["GET branch; GET content (404); PUT content; GET content; GET commit — for each file"]
  files -->|"a write or read fails"| partial["report the applied objects and the failed object; mark the state stale; do not commit"]
  files --> state["the encrypted state records the repository and both files"]
  state --> again["targeted follow-up plan: no differences"]
  again --> pin["owner pins duty2cal and duty2cal.github.io once, by hand"]
```

**Steps:**

1. The owner records the declaration revision that adds
   `github_repository.dot_github`, `github_repository_file.org_profile_readme`
   and `github_repository_file.org_security`; the two file contents are read
   from `org/` at plan time.
2. A `tofu plan -out=plan.bin` targeted at the profile repository and its two
   files reads the applied state and the live configuration. The profile
   repository does not exist and is not bound, so the preview names three
   creates and no update or delete; a declaration that cannot be read is
   refused. The preview writes nothing. The plan is targeted to the revision's
   scope — the three objects it adds — matching the sibling capability's
   preview rule. The apply `@e2e` scenario primes from the unchanged
   `security-applied` fixture, whose organization body carries
   `billing@example.test`. The preview `@e2e` scenario reuses the sibling's
   base priming, which introduces the known organization billing drift after
   its base apply; that drift is outside the three targets, so the targeted
   plan names no unrelated difference.
3. The read-only refresh-only preflight, targeted at the same three objects,
   compares them with live and refuses a manual edit by address; the saved plan
   is not applied.
4. `tofu apply plan.bin` creates the repository: `POST /orgs/duty2cal/repos`,
   the provider's create follow-up `PATCH`, and the repository read. The
   provider then creates `profile/README.md` and `SECURITY.md`: for each file
   it reads the repository and the `main` branch, reads the absent path,
   writes the reviewed content, and reads the file and its commit back. The two
   file creations are independent and may run concurrently.
5. The apply reports the three creations; the encrypted state records the
   repository and both files; a follow-up plan targeted at the three objects
   reports no differences. The
   profile repository is public, with its issue tracker, projects and wiki
   disabled, and GitHub now serves the profile page and the organization's
   default security policy from it.
6. Before the apply, the owner has enabled private vulnerability reporting on
   `duty2cal/duty2cal` by hand, because the published policy names its route and
   the provider cannot declare it. Once, after the apply, the owner pins
   `duty2cal` and `duty2cal.github.io` in the organization's web settings. No
   declaration, preview or drift check covers either step; chapter 7 records
   both.

**Notes:** The suite drives steps 2–5 against the fake in `--writes` mode and
asserts the API responses, the journal's applied-write set and the state; the
real GitHub acceptance and rendering are proven only by the operator's apply
after GREEN. The state is primed by adopting both existing batches and applying
the inherited pre-security base tree; that base apply performs the security
change's scanning write, and the scenario takes the journal offset after it, so
only the three profile creations are asserted. The apply scenario's base
omits `org_profile.tf` and copies `org/` beside its `infra/`; the preview
scenario reuses the sibling's base priming and writes its own 404 descriptors
before the targeted plan, which excludes the introduced billing drift.
A write failure after the repository creation is reported as a partial outcome
and the state is not committed; the stale marker rules are the shared
configuration-change rules, not re-asserted by this capability.

