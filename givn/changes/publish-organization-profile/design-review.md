# Design review — `publish-organization-profile`

Fresh-context grilling of the full planning layer before `tasks.md` is written.
The question log below is the intake record; `## Resolutions` at the end records
the disposition of every question, and the final verdict is PASS.
The review read `proposal.md`, the delta feature, `usecase.md`, `design.md`,
`arc42.md`, all twelve `docs/arc42/` chapters, the implementation surface the
design relies on (`infra/*.tf`, the adoption batches, `scripts/adopt-public-home.sh`,
`scripts/fake-github/serve.py`, `scripts/setup-infra-tests.sh`, the adoption and
security step modules, `features_runner.rs`, `run-tests.sh`, `REUSE.toml`,
`LICENSES/`), and the pinned provider source at `integrations/github` `v6.13.0`
with `google/go-github` `v88`.

At intake, no file outside this review was edited; no `tasks.md` was written; no
project test suite or project apply was run. The post-implementation amendment
review at the end records the later factual corrections to `design.md`.

- **Intake verdict (superseded):** NOT READY — **4 blockers**, 4 high
  questions, 7 medium questions and 1 low question (16 ranked questions). The
  plan was not signed off at intake; the resolutions below close every
  question.
- **Most serious pattern:** the new declaration writes `file("${path.module}/../org/...")`,
  but the repository has three execution shapes for the declaration (`infra/`,
  the adoption command's `.adoption-staging/`, and the test/setup workspace
  copies). The design only evaluated the first. This is a certain, reproducible
  break in the existing adoption and security suites and in
  `scripts/setup-infra-tests.sh`.
- **Existing-artifact context:** the preceding security change left a
  full-suite seam (`scripts/adopt-public-home.sh` staging + refresh-only plan,
  the security base-state priming, and the setup import proof) that this design
  did not re-walk after adding `infra/org_profile.tf`.

## Ranked question log

### Q1 — The `org/` source path breaks the adoption staging, the setup proof, and the security priming

- **Severity:** Blocker (P1); branches Tech choices, Testability, E2E fidelity, Risk.
- **Context:** `design.md:47,139,148` sources the two published files with
  `file("${path.module}/../org/...")`. The declaration is not only planned from
  `infra/`:
  - `scripts/adopt-public-home.sh:199` stages every declaration file into
    `<declaration>/.adoption-staging/` and runs `tofu plan -refresh-only` over
    the whole staged declaration at `:314`;
  - `scripts/setup-infra-tests.sh:160,206,238` copies `infra/*.tf` into fresh
    workspaces and runs refresh-only / full plans (`:188` and the base plan);
  - `apps/cli/tests/steps/enable_repository_security.rs:311` copies the whole
    `infra/` tree and runs a full base plan; `:346` removes only `security.tf`;
  - `apps/cli/tests/steps/adopt_existing_configuration.rs:171` copies the whole
    declaration for every adoption scenario.
- **Finding:** In the staging workspace `path.module` is
  `<declaration>/.adoption-staging`, so `../org` resolves to
  `<declaration>/org`, which does not exist; in the setup and security
  workspaces `path.module` is the workspace root, so `../org` resolves outside
  the copied tree. OpenTofu evaluates `file()` while building the plan: a
  throwaway OpenTofu 1.13.1 configuration with `file("${path.module}/../does-not-exist/README.md")`
  fails `tofu plan -refresh-only` with `Invalid function argument: no file
  exists at ...`, so every adoption run, the setup import proof, and the
  security base plan fail before the first scenario assertion. If `org/` were
  made visible, the security base plan would also contain the three profile
  creates, and the existing `enable_repository_security.rs` base-plan text
  check (`:350-365`) panics on any `+` attribute line. The design's file table
  (`design.md:485-504`) omits `scripts/adopt-public-home.sh`,
  `scripts/setup-infra-tests.sh`, and `apps/cli/tests/steps/enable_repository_security.rs`,
  and `design.md:430` claims the setup proof is unchanged.
- **Recommended answer:** Make the source path workspace-shape-independent. The
  cleanest option is to stage the adoption declaration as a sibling of `org/`
  (for example `<declaration>/../.adoption-staging`) so `${path.module}/../org`
  holds in both `infra/` and staging, and to copy `org/` next to the setup and
  security workspaces or remove `org_profile.tf` from the base-declaration
  workspaces that are not proving the profile. Add the adoption script, the
  setup script and `enable_repository_security.rs` to the changed-file table.
  Do not leave the existing suites to discover this at implementation time.
- **Answerable from:** repository (the break is reproducible); the exact fix is
  an authoring decision, not a user question.

### Q2 — The excluded provider name would be committed in new public test source

- **Severity:** Blocker (P1); branches Scope, Testability, Use-case/Persona
  constraints.
- **Context:** `design.md:519` says “the invariant step's test-only deny-list
  holds the excluded name”; `proposal.md:227` repeats it. The deny-list is
  planned in `apps/cli/tests/steps/publish_organization_profile.rs` (new), and
  the whole repository is public.
- **Finding:** The owner's binding constraint is that the third-party provider
  name must not appear in **new public content**. A new Rust test source file is
  new public content; a literal deny-list puts the excluded name in the
  repository history even though the published `org/**` text is clean. The
  design has therefore satisfied the profile rule and violated the content
  rule in the same plan. (The name already exists in pre-change documents such
  as `givn/ideation/duty-plan-calendar-conversion/frame.md`; the proposal
  explicitly does not edit those. That does not license a new copy.)
- **Recommended answer:** Do not store the literal. Either (a) compare the
  published bytes to the reviewed `org/profile/README.md` (and to the reviewed
  `org/SECURITY.md`) and keep the “no provider” negative check review-based, or
  (b) make the mechanical check token-wise against a hash of the excluded name
  (or a salted digest) so the name is never stored in plaintext. If the owner
  intends test source to be exempt from the constraint, that must be an
  explicit operator decision recorded in this review; otherwise the literal is
  a leak.
- **Answerable from:** user (choose the exemption or the hash/comparison
  mechanism); the factual leak is repository-answerable.

### Q3 — The default security policy names a private-reporting route that the risk register says is off

- **Severity:** Blocker (P1); branches Scope, Risk, Use-case context, Truth of
  public copy.
- **Context:** `design.md:269-273` makes the new `org/SECURITY.md` name
  `https://github.com/duty2cal/duty2cal/security/advisories/new` as the private
  vulnerability reporting route. `docs/arc42/11-risks-and-technical-debt.md:36`
  (R-030) says: “Private vulnerability reporting is off and cannot be declared
  by the pinned provider”. The existing monorepo `SECURITY.md` names the same
  route, but that does not make the new organization-wide copy true.
- **Finding:** The new public security policy would tell a contributor to use a
  confidential route the project's own risk register records as disabled. The
  scenario then asserts the route is named (`publish-organization-profile.feature:56`),
  so the suite would lock in the claim without checking that the route exists.
- **Recommended answer:** Decide and record the prerequisite: either enable
  private vulnerability reporting manually before publishing the profile (and
  add it to chapter 7's deployment consequences and to the operator's post-GREEN
  steps), or name a route that is actually available (for example a security
  contact address) and adjust the scenario. Do not publish a route the project
  has recorded as off.
- **Answerable from:** user (the enablement decision); R-030 is
  repository-answerable evidence of the conflict.

### Q4 — The shared refusal/partial branches are deferred without an operator decision

- **Severity:** Blocker (P1); branches Deferrals, Missing scenarios, Use-case
  context.
- **Context:** `design.md:562` (Use-case traceability) says: “Not re-asserted
  under this capability: the sibling `enable-repository-security` scenarios
  already exercise the refusal and partial-failure branches of the shared
  routine.”
- **Finding:** The design-review instruction permits a mandatory-check deferral
  only after an operator question with the decision recorded. The plan records a
  rationale, not an operator decision. It also leaves the capability without any
  error-path scenario for its own create (for example a refused repository
  create or a failed file write), even though the proposal's apply can fail on
  exactly those objects.
- **Recommended answer:** Either (a) put the question to the operator and record
  the acceptance of the sibling-capability evidence (including which scenario
  covers which branch), or (b) add one profile-specific refusal/partial
  scenario. Record the decision in this file and the design; do not leave it as
  an unexplained “not re-asserted” row.
- **Answerable from:** user (accept the deferral or require the scenario); the
  current lack of an operator decision is repository-answerable.

### Q5 — The use-case success guarantee carries an open question marker

- **Severity:** High (P2); branch Use-case and Persona context.
- **Context:** `givn/specs/apply-configuration-change/usecase.md:74`: the
  success guarantee ends with `(open: Q11)`. The use case's out-of-scope at
  `:112` says Q11 is “removing a declared object”, and this capability removes
  nothing.
- **Finding:** The review instruction's use-case rejection criteria include an
  unqualified guarantee whose prerequisite is open. The guarantee is qualified
  by “every object the revision retains”, so Q11 is arguably not a prerequisite,
  but the marker is still present and the use case's own text does not close it.
  The design-review branch explicitly asks to run those criteria.
- **Recommended answer:** Close Q11 in the use case with the out-of-scope
  disposition (removal never occurs in this capability) and remove the open
  marker, or record the operator's explicit acceptance of the qualified
  guarantee. Do not leave the marker in a signed-off contract.
- **Answerable from:** user (the Q11 disposition); the marker is
  repository-answerable.

### Q6 — The content scenarios do not assert the links and policy clauses the proposal promises

- **Severity:** High (P2); branches Scope, Missing scenarios, Testability.
- **Context:** `proposal.md:75` promises the profile “points the download at the
  stable release address, links the source and the site”. The feature's
  content scenario (`publish-organization-profile.feature:39-48`) asserts the
  local converter, Windows-only platform, no browser claim, no provider name,
  non-affiliation, absolute links, and no third-party resources, but not the
  stable release address, the source link, or the site link. The proposal's
  security-policy paragraph also promises that `SECURITY.md` “forbids attaching
  a real duty plan”; the security-policy scenario (`:51-58`) asserts the route,
  the latest-release support, and the absent issue tracker, but not that clause.
- **Finding:** The proposed “published text is asserted, not trusted” promise is
  incomplete: a future edit could change the download link away from the stable
  release address, drop the source/site links, or delete the plan-attachment
  prohibition without failing a scenario. The design's copy contains those
  claims, so the gap is in the executable specification, not in the copy.
- **Recommended answer:** Add concrete Then-steps for the stable
  `releases/latest` address, the source and site links, and the
  “never attach a real plan or converted calendar” clause. This is a
  specification addition, not a design rewrite.
- **Answerable from:** repository.

### Q7 — The fake's `GET /commits/{sha}` body shape is under-specified and can stop the follow-up plan converging

- **Severity:** High (P2); branches Tech choices, Testability, Risk.
- **Context:** `design.md:350-351` says the provider reads the commit's message
  and committer; `design.md:409` lists the deterministic commit body as
  “`sha`, `message`, `author` and `committer`”, which reads as a flat object.
  Provider `v6.13.0` `resource_github_repository_file.go:328,340,344-345`
  calls `Repositories.GetCommit` and then `commit.GetSHA()` plus
  `commit.Commit.GetCommitter()...`. go-github's `GET /repos/{owner}/{repo}/commits/{sha}`
  returns a `RepositoryCommit` with a top-level `sha` and a nested `commit`
  object.
- **Finding:** If the fake returns the flat create-response shape for
  `GET /commits`, `commit.Commit` is nil, the provider sets `commit_message`
  to the empty string, and the next plan shows a diff against the configured
  `commit_message` (`design.md:145`). The follow-up “no differences” assertion
  then fails, or a later apply loops on an update. The design does not
  distinguish the two response shapes.
- **Recommended answer:** Specify both shapes in the fake model: the `PUT`
  response carries a flat `commit` object with `sha`/`message`/`author`/
  `committer`; `GET /commits/{sha}` returns
  `{"sha": <sha>, "commit": {"message": <commit_message>, "committer": {"name":
  "fake", "email": "fake@example.test"}}}`. Add the nested shape to the
  obstacle list.
- **Answerable from:** repository (provider source at `v6.13.0` /
  `go-github` `v88`).

### Q8 — The profile capability's preview interaction is missing from the inventory and matrix

- **Severity:** High (P2); branches Interaction Coverage, E2E fidelity, Use-case
  context.
- **Context:** `usecase.md:89-95` has one `publish-organization-profile` row:
  “apply the previewed revision and read the outcome”. The design's Interaction
  Coverage Matrix (`design.md:635-637`) has the same one row. The feature has a
  separate preview scenario (`publish-organization-profile.feature:18-27`), and
  the sibling capability's inventory has a separate preview row
  (`usecase.md:91`).
- **Finding:** Preview is a distinct consumer action in the sibling capability
  and in the proposal (“the owner previews and applies as usual”), but it is not
  an inventory entry here. The matrix therefore maps only the apply action.
  If preview is deliberately folded into the apply action, the design must say
  so; if not, the inventory is incomplete and the preview scenario is a regular
  scenario with no `@e2e` evidence entry.
- **Recommended answer:** Add the preview interaction row and an `@e2e` preview
  scenario, or record the explicit normalization (“preview is a precondition of
  this capability's single inventory action; the preview is driven inside the
  `@e2e` apply scenario”) in the use case and design. The former matches the
  sibling capability and the proposal; the latter is acceptable only if
  recorded.
- **Answerable from:** repository.

### Q9 — The `security-applied` fixture is described as post-change but records the pre-apply scanning state

- **Severity:** Medium (P2/P3); branches Testability, E2E fidelity, Risk.
- **Context:** `design.md:418-419` calls `scripts/fake-github/fixtures/security-applied/`
  “the tree that records the live state the security change left”. The fixture's
  monorepo body (`fixtures/security-applied/repos/duty2cal/duty2cal.json`) has
  `secret_scanning` and `secret_scanning_push_protection` `disabled`; the
  security change's post-apply evidence has them `enabled`. The preceding
  security design-review's G5 records that the tree is derived from the
  pre-apply 2026-10-18 live read.
- **Finding:** The profile scenario's base priming (adopt both batches, then run
  the full base declaration's apply) will enable scanning as a priming write,
  not merely “record the state the security change left”. The design's
  `journal_offset` hides those writes, so the e2e assertion can still pass, but
  the narrative and the fixture provenance are wrong, and the design does not
  say the base apply is a real security-change apply.
- **Recommended answer:** Correct the description to “the inherited pre-security
  base tree”; state explicitly that the base apply performs the security
  change's scanning write and that `journal_offset` is taken after it; or start
  from a post-security fixture. Also state whether the security scenarios'
  guard plan includes the profile declaration (see Q1).
- **Answerable from:** repository.

### Q10 — No not-implemented stub pattern is named for the RED/GREEN loop

- **Severity:** Medium (P3); branch Testability.
- **Context:** `design.md:453-460` names `fail_on_skipped()`, the five `@wip`
  scenarios, and a manual “rename one step in a throwaway copy” proof. It does
  not name a not-implemented stub pattern.
- **Finding:** The design-review instruction requires a concrete strict-mode
  mechanism, a **not-implemented stub pattern**, and a single-scenario command.
  The first and third are present and correct; the stub pattern is absent, so
  an implementer could write empty or permissive step bodies and rely on
  `@wip`/undefined-step failure alone. The manual rename proof is an
  implementation task, not a design-time mechanism.
- **Recommended answer:** Name the pattern (for example every new step body
  starts with `unimplemented!("<scenario>: <step>")`, or `todo!()`), and add the
  existing mechanical audit (a grep for empty bodies) as the backstop. Keep the
  rename proof as the independent strict-mode check.
- **Answerable from:** repository.

### Q11 — The ADR qualification names more than one canonical home, and the licence candidate has no verdict

- **Severity:** Medium (P3); branches ADR qualification, Arc42.
- **Context:** `design.md:788-816` gives the main candidate a structured verdict
  with `canonical_artifact: docs/arc42/07-deployment-view.md`, and then says the
  rationale also lives in chapter 8 and this design. `arc42.md:15` says the
  candidate is “routed to chapter 7 and the design”. The `org/**` CC0 licence
  choice is mentioned in prose as failing `architectural_impact`, routed to
  `REUSE.toml` and the design, without a structured verdict.
- **Finding:** The review instruction requires a non-qualifying choice to name
  exactly one canonical feature/design/Arc42/process/project-documentation/code
  destination. The verdict names one, the prose names two or three. The licence
  candidate has no verdict at all (or the design should state that it is not a
  candidate).
- **Recommended answer:** Pick one home for the main candidate (chapter 7 for
  the deployed-object inventory is defensible; the design is also defensible)
  and use it everywhere. Either add the licence verdict or state explicitly
  that the licence is a constraint/configuration choice, not an ADR candidate.
  Chapter 9 stays unchanged either way.
- **Answerable from:** repository.

### Q12 — The glossary does not hold two central terms of this change

- **Severity:** Medium (P3); branches Ubiquitous language, Arc42 chapter 12.
- **Context:** The design and feature use “default security policy” and “profile
  content” throughout; `docs/arc42/12-glossary.md` adds five terms (line 48, 56,
  63, 64, 75) but not “default security policy” or “profile content”. It does
  define “Reviewed public copy”, while the spec/design say “reviewed profile
  content” / “reviewed security policy”.
- **Finding:** The design introduces terms the glossary does not hold, and the
  “reviewed” term is used inconsistently. The chapter 12 promotion prose claims
  the five terms are the profile change's contribution; the central policy term
  is missing.
- **Recommended answer:** Add “Default security policy” (or fold it explicitly
  into the “Default community health files” entry), define or avoid “profile
  content”, and use “reviewed public copy” consistently (or add the shorter
  phrases as aliases). Update chapter 12 and the promotion prose.
- **Answerable from:** repository.

### Q13 — The Pages-repository Given in the default-security scenario has no observable consequence

- **Severity:** Medium (P3); branches Missing scenarios, Testability.
- **Context:** `publish-organization-profile.feature:53` gives “the pages
  repository has no security policy of its own”. The Thens (`:55-58`) assert the
  profile repository carries `SECURITY.md`, its route, its support statement and
  no public issue tracker. No Then reads the Pages repository.
- **Finding:** The Given is inert. The scenario title says the profile repository
  “carries the organization's default security policy”, but it never asserts the
  inheritance premise that makes it a *default*. The design records the fallback
  as GitHub's behavior (R-033), which is fine, but the scenario does not even
  assert the observable premise it sets up.
- **Recommended answer:** Add a Then that the Pages repository serves no
  `SECURITY.md` of its own (observable from the fake's contents/fixtures), and
  keep the GitHub-renderer limitation in R-033. Or remove the Given and record
  that the fallback is out of the capability's observable surface.
- **Answerable from:** repository.

### Q14 — “A reader opens the organization profile” is an API read in the plan

- **Severity:** Medium (P3); branches E2E fidelity, Interface classification,
  Visual Design Contract.
- **Context:** `publish-organization-profile.feature:41` says “When a reader
  opens the organization profile”. The design's interface classification
  (`design.md:596-616`) says the project does not render the page; GitHub does;
  the content is read through the REST API. The Visual Design Contract declares
  `Interface: terminal`.
- **Finding:** The design's classification is defensible: the system's boundary
  is the declaration + GitHub REST API, and the project owns no rendered screen;
  a browser test would test GitHub's renderer. But the scenario's wording
  implies a browser action, and the `Then` steps are API-response assertions.
  The review instruction requires testing the opposite classification and
  saying why it does not hold; the design does say why, but the scenario wording
  is a silent redefinition of “opens”.
- **Recommended answer:** Keep the API-only classification and no browser
  driver, but reword the scenario/step to “When a reader reads the published
  profile content” (or equivalent) and record that “opens” means the published
  API response, not a rendered page. No browser driver is required.
- **Answerable from:** repository.

### Q15 — The targeted preview contradicts the use-case “names every difference” rule, and the billing-drift rationale is not true of the fixture

- **Severity:** Medium (P3); branches Use-case context, Scope, Arc42 chapter 6.
- **Context:** `usecase.md:30` (main flow 2) says the preview “names every
  difference by object and field”. `usecase.md:66` (the profile example) says
  “The targeted preview names the three creations and no other change.”
  `design.md:437` and `docs/arc42/06-runtime-view.md:305-312` justify the
  targeted preview by “the pre-existing organization billing drift”. The
  `security-applied` fixture's `orgs/duty2cal.json` has the matching
  `billing@example.test`; the drift is introduced only by the security test's
  `ensure_base_state`, not by the profile scenario.
- **Finding:** The use case contains an internal contradiction, and the design's
  stated reason for targeting is true of the live organization (R-030's
  context) but not of the profile scenario's fixture. A targeted preview is
  still a reasonable design, but it must be recorded as a scoping rule rather
  than as a consequence of drift that the test does not have.
- **Recommended answer:** Amend the use case's main flow to “names every
  difference among the previewed objects” (or explicitly record the targeted
  scope), and either introduce the same out-of-scope drift in the profile
  scenario (as the security scenario does) or correct the rationale in the
  design and chapter 6. Keep the targeted plan.
- **Answerable from:** repository.

### Q16 — The profile copy's licence sentence omits the third-party Unicode licence the README attributes

- **Severity:** Low (P3); branches Scope, Truth of public copy.
- **Context:** `design.md:235` publishes “The project is licensed under
  MPL-2.0; this page is CC0-1.0.” The README's public-text gate
  (`scripts/public-text-check.sh`) requires the `Unicode-3.0` attribution and
  link, and `LICENSES/Unicode-3.0.txt` exists.
- **Finding:** The sentence is not false for the project's own code, but it
  omits the third-party data licence the repository publishes. This is the kind
  of public-claim completeness the proposal's “current truth” constraint
  targets; the README already treats the attribution as a promise.
- **Recommended answer:** Add the Unicode-3.0 attribution (or say “MPL-2.0
  except third-party data, including Unicode-3.0”) and, if the profile is meant
  to carry the same completeness claims as the README, extend
  `scripts/public-text-check.sh` or the content-invariant scenario rather than
  leaving the profile unchecked.
- **Answerable from:** repository / user (the wording decision).

## Branch-by-branch findings

### Scope — findings (Q3, Q4, Q6, Q16)

The five scenarios cover the proposal's repository creation, public feature
set, reviewed content, default policy, preview, update, and follow-up preview.
They do not assert the stable release/source/site links, the “never attach a
real plan” clause, or the truth of the private-reporting route. The licence
sentence is a low-scope truth gap. “Done” is otherwise unambiguous.

### Tech choices — findings (Q1, Q7)

The stack is right and minimal: the existing OpenTofu declaration, the pinned
`integrations/github` 6.13.0, `github_repository_file`, and the existing Python
fake. The provider call sequence was independently checked against
`v6.13.0`/go-github `v88`:

- create: `POST /orgs/duty2cal/repos`, `PATCH /repos/duty2cal/.github` (with a
  possible second visibility-only `PATCH`), `GET /repos/duty2cal/.github`,
  `GET /vulnerability-alerts` — correct (`resource_github_repository.go:758,785,922,961,1016,1024,906`);
- file create: repo `GET`, branch `GET`, contents `GET` 404, contents `PUT`,
  contents `GET`, commit `GET` — correct
  (`resource_github_repository_file.go:181,192,240,268,318,328,340,344`);
- file update: contents `PUT` with SHA, contents `GET`, commit `GET` — correct
  (`:385`);
- 204 → vulnerability alerts enabled (`go-github` `parseBoolResponse`) —
  correct.
The remaining tech-choice defects are the `file()` workspace path (Q1) and the
commit response shape (Q7). A minor observation: `branch = "main"` is hard-coded
rather than using the created repository's computed `default_branch`; the live
repo's default is `main`, so this is not a finding today, but the design should
say why the constant is safe.

### Missing scenarios — findings (Q4, Q6, Q13)

Missing Then coverage for the stable release/source/site links, the
plan-attachment prohibition, and the Pages repository's absence of its own
policy. The refusal/partial branches are deferred without an operator decision
(Q4). The preview action is missing from the inventory (Q8).

### Testability — findings (Q1, Q7, Q10)

Every scenario can fail RED via `fail_on_skipped()` and the undefined-step
contract; the Thens are concrete copy/API/state assertions. The `file()`
workspace break (Q1) makes the scenario setup, not the assertion, fail. The
commit shape (Q7) can make a correct implementation look non-convergent. The
not-implemented stub pattern is missing (Q10). The single-scenario command is
correct: `./run-tests.sh --name "<title>"` and `--e2e --name`.

### E2E fidelity — findings (Q1, Q8, Q9, Q14)

The one `@e2e` scenario drives the real `tofu` binary and reads the GitHub REST
API response back through raw loopback HTTP; the primary assertion is on the
published API response, which is the project's real boundary. The fake is the
digital twin, the provider is from the local mirror, and no scenario reaches
`api.github.com`. The browser classification is correctly rejected for the
project-owned surface (GitHub renders the page). The preview inventory (Q8),
the base-tree description (Q9) and the “opens” wording (Q14) are the gaps.
No second `@e2e` scenario duplicates the apply action.

### Interaction Coverage — finding (Q8)

The owning use case's inventory has one profile row, and the design matrix has
exactly that one row with the matching `@e2e` title; the driving mechanism is
the real CLI/API path. The preview interaction is the missing entry (Q8).

### Risk — findings (Q1, Q3, Q7, Q9)

The most likely implementation failure is Q1: the new `file()` source breaks
the adoption staging and every full-declaration plan before the profile
scenarios run. The design's risk list (fake-vs-policy, GitHub fallback, pins,
disclaimer) does not record it, and does not record the PVR route truth (Q3),
the deny-list leak (Q2), or the commit-shape non-convergence (Q7). The fake's
limits are otherwise correctly recorded as risks, not proven behavior.

### Use-case and Persona context — findings (Q4, Q5, Q15)

Use-case identity, capability ownership, typed relationships (none), and
Persona coverage are correct; the three confirmed Personas are the owning use
case's lenses and the Gherkin Actor remains the organization owner. The
success guarantee's `(open: Q11)` (Q5), the refusal/partial deferral (Q4), and
the targeted-preview rule versus main flow 2 (Q15) are contract gaps. No
Persona is invented or silently promoted; the “reader” in the content scenario
is a domain role for the crew-member/contributor lenses, not a new Persona.

### Decision ledger — no issue found

All proposal questions D1–D9 resolve in `design.md:507-547` as
`Decides D<n>` with the rejected alternatives and the realization. The
proposal records no ideation seed, so no `(seed Q<n>)` reference is required.
D1–D4 are realized in the copy and the content scenario; D5–D6 in the
declaration/REUSE; D7 in chapter 7; D8–D9 in the declaration/copy and chapter
11.

### Visual design contract — no issue found

The design contains the required `## Visual Design Contract`
(`design.md:590-616`) with `Interface: terminal`, no project-owned screen, a
transcript evidence path and the placeholder scrubbing rule. The alternative
(browser test against GitHub's renderer) is correctly rejected. The change
introduces a public *copy* surface, not a project-rendered screen; the reviewed
copy block is the reference. If the orchestrator treats the organization page
as a Web UI despite GitHub rendering it, that classification decision must be
recorded and would change the evidence requirement; the design's stated reason
is currently defensible.

### Deferrals — finding (Q4)

The refusal/partial deferral is the only mandatory-check deferral without an
operator decision. The pinned-repository step, the GitHub renderer fallback,
the fake's policy limits and the operator's real apply are recorded as
out-of-scope/manual controls, not as a silent weakening.

### Persona disposition — no issue found

`design.md:549-565` records the impact for all three confirmed Personas:
organization owner (Actor, revision/apply/pin, answers for the claims),
infrastructure reviewer (reads the declaration and copy without a token), and
crew member (prospective user reads the truth). The proposal confirms no new
Persona. The Gherkin Actor stays the organization owner; Persona biographies
are not used as actors.

### Interface classification — no issue found (with Q14 wording)

The system exposes a CLI/declaration and the GitHub REST API; the profile page
is GitHub's rendering, not a project screen. The design says why a browser
driver does not apply; no browser test would prove the project's declarations.
The content scenario should read the API, which is what the design plans (Q14).

### ADR qualification — finding (Q11)

The main candidate's five mandatory gates are present with a NOT_QUALIFIED
verdict, and the existing-ADR check names ADR-0005/0006/0003. The canonical
destination is not named exactly once across the verdict and prose, and the
`org/**` licence choice lacks its own structured verdict or an explicit “not a
candidate” statement. No `NEW_ADR` is proposed; chapter 9 and the ADR indexes
stay unchanged, which is correct.

### Architecture documentation (arc42) — findings (Q9, Q15, Q12, Q11)

Independent chapter-set derivation (walking all 12 rows before opening
`arc42.md`, after reading the proposal/spec/design): rows 1, 2, 3, 5, 6, 7, 8,
10, 11, 12 are affected; rows 4 and 9 are not. `arc42.md` matches that exactly,
including the reasons, so there is **no omitted row**. All twelve chapter files
exist and contain decision-specific content; no file is scaffold-only. No ASCII
box-drawing or `+`/`|` pseudo-diagrams were found in any chapter; all diagrams
are Mermaid. Chapter 9's register is unchanged and correctly has no new ADR.
The content mismatches are the base-tree/billing rationale (Q9, Q15), the
glossary gap (Q12), and the canonical-home wording (Q11). `STATUS: DONE` was
verified against the files, not trusted.

### Ubiquitous language — finding (Q12)

“Organization profile”, “Organization profile repository”, “Pinned
repositories”, “Default community health files” and “Reviewed public copy” are
in `docs/arc42/12-glossary.md`. “Default security policy” and “profile content”
are not, and the design alternates between “reviewed public copy” and “reviewed
profile content”. The glossary must hold the terms the delta and design use
before sign-off.

## Blockers and non-blocking improvements

The intake blockers and improvements below are all resolved in
`## Resolutions`; they are kept as the record of what the grilling found.

**Blockers (4):**

1. **Q1** — the `file()` source path breaks `scripts/adopt-public-home.sh`
   staging, `scripts/setup-infra-tests.sh` and the existing security priming;
   the design's changed-file table omits them.
2. **Q2** — the excluded provider name would be committed in new public test
   source as a deny-list literal.
3. **Q3** — the new default security policy names a private-reporting route
   that R-030 records as off.
4. **Q4** — the shared refusal/partial branches are deferred without an
   operator decision.

**Non-blocking improvements (must be resolved before sign-off, but do not by
themselves block the task list once the blockers are fixed):**

- Q5: close or qualify the `(open: Q11)` success-guarantee marker.
- Q6: add the missing stable-release/source/site link and plan-attachment Thens.
- Q7: specify the nested `GET /commits` response shape.
- Q8: add or explicitly normalize the preview interaction.
- Q9: correct the `security-applied` provenance and the base-apply narrative.
- Q10: name the not-implemented stub pattern.
- Q11: name exactly one ADR canonical home; add the licence verdict or state
  it is not a candidate.
- Q12: add “default security policy” / “profile content” and align “reviewed
  public copy”.
- Q13: give the Pages-repository Given an observable Then or remove it.
- Q14: reword “opens” to the API read; keep the no-browser-driver decision.
- Q15: align the targeted-preview rule with use-case main flow 2 and the
  fixture's actual drift.
- Q16: complete the profile's licence sentence with the Unicode-3.0
  attribution or record the intended simplification.

**Verified clean (no issue found):** REUSE `precedence = "override"` and the
CC0-1.0 text are valid per REUSE 3.3 (the spec's own example uses exactly
`precedence = "override"` with `CC0-1.0`); the `org/**` table is last, so it is
the matching table for those paths; `LICENSES/CC0-1.0.txt` is the canonical
SPDX text. The pinned-repository manual step is correct: provider 6.13.0 has no
pinned-repository resource (no `pinned` hits in the provider schema). The live
read's `public_repos: 2` supports “the profile repository does not exist live”.

## Verification performed

- `givn instructions design-review|specs|design|arc42-docs|review --change publish-organization-profile`
  served and followed; every design-review branch was walked.
- `givn lint --change publish-organization-profile`: exit 0, five `@wip`
  advisories, no errors.
- Provider source: `integrations/github` tag `v6.13.0` fetched and diffed
  against the local source; `go-github` `v88` `repos.go`/`repos_contents.go`
  read; create/read/update/commit and 204 semantics confirmed.
- `tofu` behavior check in `/tmp` only: a `file()` pointing at a missing path
  fails a refresh-only plan with `Invalid function argument`; no project file
  was applied.
- Arc42 scan: all 12 chapter files exist, non-scaffold, no box-drawing/pseudo
  ASCII diagrams; all diagrams are Mermaid; chapter 9 and the ADR indexes are
  unchanged.
- No test suite, project apply, commit, `tasks.md`, or sign-off was produced.

## Verdict

At intake, four blockers and twelve unresolved questions remained; the plan was
not ready to harden into a task list until the questions were answered and the
design, specification, and integration files were corrected. Those corrections
are recorded below.

## Resolutions

Each record states the decision, its evidence or source, the files changed and
whether the question was answerable from the repository or was settled by the
owner's binding instruction in the hardening brief. No question is reopened.

### Q1 — workspace shapes and the existing suites (blocker)

- **Decision:** `infra/org_profile.tf` declares `variable "org_source_dir"`
  (default `""`, meaning `${path.module}/../org`) and resolves it through
  `locals.org_source_dir`; both `file()` calls read
  `${local.org_source_dir}/...`. The adoption command stages the declaration
  without `org_profile.tf`; the setup import proof and the `security-applied`
  base workspace omit it; `enable_repository_security.rs` removes it together
  with `security.tf` for the base declaration, restores it after the base
  apply, and sets `TF_VAR_org_source_dir` to the repository root's `org/` for
  the profile helper. The profile step module copies `org/` beside the
  scenario's `infra/` (or sets the variable), and `DutyWorld.source_tree` is
  that copy.
- **Source:** repository-answerable break (the missing `org/` fails
  `tofu plan` in the staging and copied workspaces) and the owner's binding
  instruction for the exact fix.
- **Files changed:** `design.md` (declaration, changed-file table, new
  "Workspace shapes and the existing suites", fixtures, step definitions,
  walkthrough, obstacles); `docs/arc42/02-architecture-constraints.md`,
  `05-building-block-view.md`, `07-deployment-view.md`,
  `08-crosscutting-concepts.md`; `arc42.md`.
- **Status:** resolved. The design states that the existing adoption and
  security suites remain green.

### Q2 — excluded provider name (blocker)

- **Decision:** no literal is stored in any new file. The invariant compares the
  published `profile/README.md` and `SECURITY.md` bytes with the reviewed
  `org/` source and asserts the positive claims (local conversion, Windows
  x86_64 only, no browser claim, crew-duty-plan input, non-affiliation,
  absolute links, no third-party embeds). The absence of the provider name is
  a review control (R-032), not a deny-list.
- **Source:** the owner's binding instruction (no name in new public content);
  the leak was repository-answerable.
- **Files changed:** `proposal.md`; `design.md`; `arc42.md`;
  `docs/arc42/11-risks-and-technical-debt.md` (R-032); the feature (the
  negative Then removed).
- **Status:** resolved. No new file contains the name.

### Q3 — private-reporting route (blocker)

- **Decision:** the policy keeps naming the private vulnerability reporting
  route, and the owner enables private vulnerability reporting on
  `duty2cal/duty2cal` by hand before the profile apply. The setting remains
  outside the declaration, the state and the drift check; the fake and the
  suite cannot prove it.
- **Source:** the owner's binding instruction (enable rather than change the
  route); R-030 is repository evidence of the conflict.
- **Files changed:** `proposal.md`; `design.md`;
  `docs/arc42/07-deployment-view.md`; `docs/arc42/11` (R-030);
  `docs/arc42/06-runtime-view.md`, `08-crosscutting-concepts.md`,
  `01-introduction-and-goals.md`, `02-architecture-constraints.md`,
  `03-context-and-scope.md`, `10-quality-requirements.md`; `arc42.md`.
- **Status:** resolved.

### Q4 — refusal/partial deferral (blocker)

- **Decision:** add the regular scenario "A profile repository that already
  exists live and is unbound refuses the preview". The design records that
  this capability adds the unbound-profile refusal; the shared partial-failure
  branch remains covered by the sibling capability and is not re-asserted, with
  no deferral of a mandatory profile-specific check.
- **Source:** the owner's binding instruction; the missing operator decision
  was repository-answerable.
- **Files changed:** the feature; `design.md` (use-case traceability).
- **Status:** resolved. The feature is 2 `@e2e` + 4 regular scenarios.

### Q5 — success-guarantee marker

- **Decision:** `(open: Q11)` is removed from the success guarantee; the use
  case adds a "No removal" rule and closes Q11 in the existing out-of-scope
  removal item. No removal behaviour is invented.
- **Source:** the owner's binding instruction (close with the out-of-scope
  disposition); the marker was repository-answerable.
- **Files changed:** `givn/specs/apply-configuration-change/usecase.md`.
- **Status:** resolved.

### Q6 — missing content assertions

- **Decision:** the content scenario asserts the stable latest-release address
  `.../releases/latest`, the source and site repository links, the
  "forbids attaching a real duty plan or a converted calendar" clause and the
  Unicode-3.0 attribution. The design's copy and assertions match.
- **Source:** repository.
- **Files changed:** the feature; `design.md`;
  `docs/arc42/10-quality-requirements.md`, `01`, `02`, `08`; `arc42.md`.
- **Status:** resolved.

### Q7 — fake commit response shape

- **Decision:** `PUT .../contents/...` returns a flat `commit`
  (`sha`/`message`/`author`/`committer`); `GET /repos/{owner}/{repo}/commits/{sha}`
  returns a top-level `sha` with a nested `commit`. The provider's
  `commit.Commit.GetCommitter()` readback and the no-diff follow-up are
  recorded as the obstacle.
- **Source:** repository (provider `v6.13.0` and go-github v88).
- **Files changed:** `design.md`.
- **Status:** resolved.

### Q8 — preview interaction

- **Decision:** the owning use case gains a preview row and retains the apply
  row; the preview scenario is retitled
  "Organization owner previews the profile declaration and only the three
  creations are named" and tagged `@e2e`. The design matrix gains the preview
  row, every "one `@e2e`" statement becomes "two", and the second transcript
  path is recorded.
- **Source:** repository (the sibling capability's pattern) and the owner's
  binding instruction.
- **Files changed:** `usecase.md`; the feature; `design.md`.
- **Status:** resolved.

### Q9 — fixture provenance

- **Decision:** `security-applied` is the inherited pre-security base tree;
  the base apply performs the security change's scanning write; the profile
  scenario takes `journal_offset` after that base apply. The base workspaces
  omit `org_profile.tf`, so the profile objects are not in the base state.
- **Source:** repository (the fixture body and the security change's
  evidence).
- **Files changed:** `design.md`; `docs/arc42/06-runtime-view.md`,
  `08-crosscutting-concepts.md`.
- **Status:** resolved.

### Q10 — not-implemented stub pattern

- **Decision:** every new step body starts as
  `unimplemented!("<scenario>: <step>")` (or `todo!()`); the GREEN replaces it.
  A review-time grep for `unimplemented!`/`todo!()` in the new step files is
  the backstop. `fail_on_skipped()` and the rename-a-step proof remain
  independent strictness checks.
- **Source:** repository.
- **Files changed:** `design.md`.
- **Status:** resolved.

### Q11 — ADR canonical home

- **Decision:** the single canonical home is
  `docs/arc42/07-deployment-view.md`; chapter 8 and the design record the
  realization, not additional homes. The `org/**` CC0-1.0 choice is not an ADR
  candidate; its canonical home is `REUSE.toml` (the design records the
  implementation). No ADR is added.
- **Source:** repository.
- **Files changed:** `design.md`; `arc42.md`; `docs/arc42/02`.
- **Status:** resolved.

### Q12 — glossary terms

- **Decision:** add **Default security policy** and **Profile content**;
  **Reviewed public copy** is the umbrella term for both. Chapter 12's
  promotion prose now names seven public-profile terms, and the feature and
  design use the terms.
- **Source:** repository.
- **Files changed:** `docs/arc42/12-glossary.md`, `05`, `08`; `design.md`;
  the feature.
- **Status:** resolved.

### Q13 — inert Pages-repository Given

- **Decision:** the default-security scenario asserts "the pages repository
  serves no security policy of its own"; the tree gains the 404 descriptor at
  `repos/duty2cal/duty2cal.github.io/contents/SECURITY.md.json` and the step
  asserts the fake returns 404. R-033 keeps the GitHub-renderer limitation.
- **Source:** repository.
- **Files changed:** the feature; `design.md`.
- **Status:** resolved.

### Q14 — "opens" wording

- **Decision:** the content scenario's When becomes "When a reader reads the
  published profile content"; the design records that "reads" means the
  published GitHub REST API response, not a rendered page. The API-only
  interface classification and the absence of a browser driver stand.
- **Source:** repository.
- **Files changed:** the feature; `design.md`.
- **Status:** resolved.

### Q15 — targeted preview and the use-case rule

- **Decision:** use-case main flow 2 names "every difference among the
  previewed objects", and a rule fixes the preview scope to the objects the
  revision adds or changes. The design replaces the billing-drift
  justification with that scoping rule and records that the
  `security-applied` fixture has no billing drift; chapter 6's note matches.
- **Source:** repository.
- **Files changed:** `usecase.md`; `design.md`;
  `docs/arc42/06-runtime-view.md`.
- **Status:** resolved.

### Q16 — licence completeness

- **Decision:** the profile copy adds "Third-party Unicode data is licensed
  under Unicode-3.0" with
  `https://github.com/duty2cal/duty2cal/blob/main/LICENSES/Unicode-3.0.txt`;
  the content scenario asserts the attribution; the page remains CC0-1.0 for
  its own prose.
- **Source:** repository (the README's public-text gate and the existing
  licence text) and the owner's binding instruction for the wording.
- **Files changed:** `design.md`; the feature; `docs/arc42/01`, `02`, `07`,
  `08`, `10`; `arc42.md`.
- **Status:** resolved.

### Verification after the resolutions

- `givn lint --change publish-organization-profile`: 6 findings, every one
  `kind: wip`, `level: warning`; **no errors**. The six are the expected
  `@wip` advisories for the six scenarios (the CLI exits 2 when findings exist).
- `givn status --change publish-organization-profile`: stage
  `design-review-done`; `tasks` is the next required artifact.

Every question is resolved and the planning artifacts carry the edits. The
implementation, `tasks.md` and the commit remain out of scope for this step.

## Post-implementation amendment review

Fresh-context re-review after implementation exposed three seams: the pinned
runner's argument conflict, the shared preview/apply step text, and the two
different base-priming paths. Scope read: this file, `design.md` (including
"Workspace shapes and the existing suites", "Fixtures and the scenario tree",
"Why the preview and the follow-up are targeted", "Test runner, strict mode,
and single-scenario command", "Step definitions", "E2E smoke test
infrastructure", "Interaction Coverage Matrix"), the delta feature
`specs/apply-configuration-change/publish-organization-profile.feature`, both
`steps/publish_organization_profile.rs` modules,
`steps/enable_repository_security.rs`, `scripts/adopt-public-home.sh`,
`scripts/setup-infra-tests.sh`, and the `tasks.md` evidence. No live apply, no
`tasks.md` write, no commit.

### Amendment 1 — single-scenario command and the e2e scope

- `run-tests.sh` rewrites only `--e2e` into `--tags "@e2e"` and passes every
  other argument (`--name`, `--tags`) through unchanged. The pinned
  `cucumber` 0.23.0 CLI declares `conflicts_with = "name"` on `--tags`
  (`~/.cargo/registry/src/index.crates.io-*/cucumber-0.23.0/src/cli.rs:113`),
  so `--e2e --name` fails by construction and `--name` alone is the correct
  single-scenario form. The amendment is accurate.
- Counts recomputed from the assembled root, not trusted: 141
  `Scenario`/`Scenario Outline` declarations plus the two outlines' 4 and 7
  example rows give 150 scenarios; the `@e2e` tag count is 8. This matches
  `design.md` and the tasks' final evidence (150 full / 8 e2e).
- `verify.command` and `verify.e2e_command` remain distinct in
  `givn/commands.yaml`; the e2e command is the full runner restricted by tag,
  and a `--name` run selects the same scenario without changing the scope
  proof. No downgrade: the e2e evidence is still produced by the tag-filtered
  run.
- The earlier Testability finding's `--e2e --name` form is superseded by this
  amendment.

### Amendment 2 — the shared step text is bound once

- Exactly one binding exists for `the organization owner previews the
  declaration` and one for `the organization owner applies the previewed
  revision`, both in `steps/enable_repository_security.rs` (`:789`, `:800`).
- The dispatch predicate is `DutyWorld.source_tree.is_some()`; the only setters
  are `ensure_source_tree` calls on profile paths (`prime_base_state`, the
  e2e apply `Given`, and the "declaration adds" `Given`). No security scenario
  sets `source_tree`, so no security scenario is silently redefined.
- The profile module owns the targeted `plan -target=... -out`, `show -json`,
  the refresh-only preflight, the apply, and the assertions; the security
  handler only dispatches. `source_tree` is the scenario's copied `org/`, and
  the published bytes are compared to that copy, so no parallel fixture
  substitutes for the declaration.
- Verified against the feature: the two shared `When` steps drive the two
  `@e2e` scenarios; the four regular scenarios add profile-specific steps and
  reuse only the shared preview/apply `When` where applicable. No `@e2e` tag
  was removed.

### Amendment 3 — the two priming paths and the drift

- The apply `@e2e` scenario's first `Given` is bound to `prime_base_state`
  (`steps/publish_organization_profile.rs:97`): it copies the unchanged
  `security-applied` tree, adopts both batches, removes `org_profile.tf`, runs
  the base declaration's real init/plan/apply (with `security.tf` present, so
  that apply performs the scanning write), restores `org_profile.tf`, and takes
  `journal_offset`. It does not rewrite the organization body, so no billing
  drift exists. The revised-content, invariant, policy and refusal scenarios
  also use this path.
- The preview `@e2e` scenario's "the applied state binds every declared object"
  is the security module's `ensure_base_state`
  (`enable_repository_security.rs:748`), which runs `ensure_base_adoption`,
  adopts the security batch, and then rewrites
  `orgs/duty2cal.json`'s billing address to `live-drift@example.test`
  (`:475-485`); the e2e `Given` writes the four descriptors and takes the
  offset afterwards.
- The preview, preflight and follow-up are `TARGETS`-scoped at the three
  profile resources. The preview `Then`s assert exactly those three creates
  and no other change, zero applied writes, and an unchanged state snapshot;
  the drift is therefore outside the plan and is not asserted away. The
  design's targeted section states the two paths accurately.

### E2E fidelity

- Apply: primary assertions are the saved plan's JSON, the CLI apply transcript
  ("3 added"), the fake's `GET /repos/duty2cal/.github` body, the contents API
  bytes compared byte-for-byte with the reviewed source, the state addresses,
  and the journal set (one `POST`, the create's `PATCH`es, exactly two `PUT`s,
  no refused/unknown path). The transcript is committed.
- Preview: the plan JSON, the empty applied-write journal, and the unchanged
  state snapshot.
- Both drive the real `tofu` binary; the fake is the HTTP boundary, not a
  provider mock. Repository-state assertions are supplementary to the API and
  CLI assertions, so there is no repository-only downgrade.

### Factual corrections applied to `design.md`

The amendments themselves were accurate, but three factual statements did not
match the implementation and were corrected rather than left as hidden
contradictions:

1. The fixture list, the step-definition row, and the manual walkthrough said
   three 404 descriptors. `write_profile_descriptors` writes four, including
   `repos/duty2cal/.github.json`; without it the preview's live-object `GET`
   would be a 599 unknown path instead of the 404 the guard requires. All three
   places now list the fourth descriptor and explain its role.
2. "their workspaces never see the profile declaration, and the profile
   scenario alone plans it" contradicted `enable_repository_security.rs`:
   `ensure_base_adoption` restores `org_profile.tf`, and the security preview's
   full-declaration guard plan therefore includes the three profile creates
   (the guard filter selects only the two security settings), while the shared
   `tofu` helper supplies `TF_VAR_org_source_dir` for both the scenario's
   `org/` copy and the repository's reviewed `org/`. The workspace section and
   the changed-file row now state this.
3. "The profile scenario primes the applied state..." described only the
   apply-side path. The fixtures section now says "the apply-side profile
   scenarios" use `prime_base_state` and points the preview to its sibling
   priming.

No decision, scope, scenario, assertion, or evidence requirement changed in
those corrections. Reassessment found no new blocker.

### Findings left unedited (outside this review's edit scope)

- `tasks.md`'s bundle-context line "for an `@e2e` scenario add `--e2e`" is stale
  and wrong under the pinned runner. The tasks' own single-scenario evidence
  and this design use `--name` alone; the line should be corrected when
  `tasks.md` is next edited. Non-blocking: it is not the executed command and
  the implementation evidence followed the amended form.
- The baseline bullet's first sentence, "the six new scenarios are still
  `@wip`", is stale relative to its own output: 148 passed is the count after
  four regular scenarios were implemented with the two `@e2e` scenarios still
  excluded. The pasted count is consistent with the later non-e2e verification
  and the final 150; the sentence is a caption error, not a fabricated result.

### Non-blocking test-strength finding (implementation follow-up)

- The design's journal note says no applied write names another repository, the
  organization or a settings endpoint. The apply `@e2e` PATCH check
  (`steps/e2e/publish_organization_profile.rs:300-309`) filters the applied
  `PATCH` paths through `unique | .[0]` and compares that first path with
  `/repos/duty2cal/.github`; its message claims "every follow-up write targets
  the created profile repository". An additional `PATCH` to another repository
  whose path sorts after `.github` (for example
  `/repos/duty2cal/duty2cal`) would not fail the check. The one `POST` and the
  exact two `PUT` paths are asserted exactly, and the applied plan is targeted
  at the three creates, so the residual risk is low and the profile outcome
  remains proven; this is not a capability blocker. Tightening the jq to assert
  the full `PATCH` path set is an implementation change, outside this review's
  edit scope; record it as a test-hardening follow-up before archive.

### Verification performed

- Ran `scripts/assemble-features.sh` and counted from the assembled root: 141
  `Scenario`/`Scenario Outline` declarations, two outlines with 4 and 7 example
  rows (150 scenarios), and 8 `@e2e` scenarios.
- Read `run-tests.sh` (the `--e2e` → `--tags "@e2e"` rewrite) and the pinned
  `cucumber` 0.23.0 `src/cli.rs` (`--tags` conflicts with `--name`).
- Read both profile step modules, `enable_repository_security.rs`
  (`is_profile_scenario`, `ensure_base_state`, `ensure_base_adoption`,
  `tofu`, the shared `When` bindings), `scripts/adopt-public-home.sh` (staging
  exclusion), `scripts/setup-infra-tests.sh` (proof/base workspace exclusions),
  `DutyWorld`'s fields, `givn/commands.yaml`, and the delta feature.
- Checked `tasks.md`'s evidence: the 150/8 final summaries, the single-scenario
  runs, and the committed preview and apply transcripts (sampled the real
  `tofu` output). No suite, live apply, commit, or `tasks.md` write was
  performed by this review.

### Disposition

All three amendments are accurate and verified against the pinned runner, the
implementation, and the evidence. The three design corrections align the
document with the implemented seam; they do not weaken any check. The
`tasks.md` wording issues and the `PATCH`-path assertion strength are
non-blocking follow-ups outside this review's edit scope. No blocker remains.

DESIGN-REVIEW: PASS
