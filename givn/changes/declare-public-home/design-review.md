# Design review — declare-public-home

- Change: `declare-public-home` (use case `adopt-existing-configuration`).
- Reviewed: 2026-10-17, against `proposal.md`, `specs/adopt-existing-configuration/`
  and `design.md`, with `arc42.md`, the twelve chapters, the two ADRs and the
  repository scripts (`scripts/render-changelog.sh`, `cliff.toml`,
  `measure-coverage.sh`) checked as evidence.
- Method: a fresh grilling pass produced twenty-five ranked questions; the
  hardener applied every disposition. No question required a new user decision,
  so no question was deferred and nothing was left to a later operator answer.
- Verdict: `DESIGN-REVIEW: PASS` (see the end of this file).

## Ranked question log

### G1 (blocker) — when does the new capability enter the change-log allow-list?

- **Question:** the archive creates `givn/specs/adopt-existing-configuration/`,
  but `cliff.toml`'s capability allow-list is compared against `givn/specs/`
  only. Should the capability be added to the allow-list now, or at archive?
- **Disposition:** do **not** add it now. Record an explicit archive-time
  obligation: when the archive creates the capability directory, add
  `adopt-existing-configuration` to the marked allow-list in `cliff.toml`
  **and** to the per-type parser rows that repeat the list, in the same
  revision. Until then the `changelog` gate stays green.
- **Source:** renderer evidence (`scripts/render-changelog.sh` refuses an
  allow-list differing from the use-case directories under `givn/specs/`) and
  the repository's own change-log gate.
- **Rejected:** adding the capability now. The renderer would refuse every push
  between this revision and the archive because the directory does not exist
  yet.
- **Correction to the grilling record:** the griller was right about the need
  and wrong about the timing; the obligation is recorded, the file is not
  edited.

### G2 (blocker) — can the preflight validate on a fresh checkout?

- **Question:** `tofu validate` needs an initialized provider and `.terraform/`;
  the command copied the declaration only after declaring the declaration
  "valid". A fresh checkout has no `.terraform/`, so the order was wrong.
- **Disposition:** stage the declaration first, then run
  `tofu init -backend=false` and `tofu validate` inside the staged copy; a
  fresh checkout is the normal case, not an exception.
- **Source:** repository evidence (OpenTofu `init`/`validate` semantics) and the
  change's own offline-suite constraint.
- **Rejected:** validating in `infra/` before staging — it fails on a fresh
  checkout and writes an untracked `.terraform/` into the repository.

### G3 (blocker) — what proves the adoption against the real organization?

- **Question:** the fake API proves the command's logic, not that the provider's
  real reads and import identifiers match `duty2cal`. Where is the real
  evidence, and does this change claim a real drift check?
- **Disposition:** add a **required post-GREEN step**: the maintainer runs
  `scripts/adopt-public-home.sh` once against the real organization, records the
  transcript under `givn/changes/declare-public-home/evidence/real-adoption/`,
  commits the resulting encrypted `infra/terraform.tfstate` and backup, and
  review verifies the five bound addresses with `tofu state list`. Remove the
  ADR-0005/ADR-0006 confirmation claims that this change runs a real drift
  check; drift reporting is out of scope (`report-configuration-drift`).
- **Source:** the use case's success guarantee ("every already-existing object
  ... bound in the applied state") cannot be proven by the fake alone; D1's
  scope boundary excludes drift reporting.
- **Rejected:** treating the fake-backed `@e2e` scenario as evidence about the
  real organization; treating the local drift check as this change's
  confirmation.

### G4 (major) — are the repository feature booleans neutral when omitted?

- **Question:** the declaration omitted feature booleans, but the provider sends
  `false` for an omitted Optional-only boolean, so omission silently decides the
  value. Which values are intended?
- **Disposition:** state them explicitly. Monorepo: `has_issues = true` (live,
  kept), `has_projects = false`, `has_wiki = false`,
  `has_discussions = false`. Pages repository: `has_issues = false`,
  `has_wiki = false` (draft), `has_projects = true` (live; the draft does not
  specify it). Update the declaration table, the first-plan difference
  description and D2.
- **Source:** provider evidence (`Optional` booleans default to `false` on the
  wire) and confirmed decision D2 (seed Q5).
- **Rejected:** leaving them omitted and calling the result "today's live
  values" — the wire value would be `false`, not live.

### G5 (major) — how does the already-bound scenario obtain its primed state?

- **Question:** the scenario says "already bound" but the design did not say how
  the state gets that binding without fabricating it.
- **Disposition:** prime with a successful **monorepo-only** batch against the
  same fake, then adopt the full batch; the refusal names
  `duty2cal/duty2cal`.
- **Source:** the no-duplicate-binding rule and the use case's success
  guarantee; every binding in a test must be written by the real command.
- **Rejected:** hand-writing a state fixture; priming with the full batch (it
  would bind the object whose duplicate the scenario tests).

### G6 (major) — are declaration and batch preconditions observed?

- **Question:** two refusals named in the use-case extensions and design
  failure table — an invalid declaration and a malformed batch line — had no
  scenario.
- **Disposition:** add two regular `@givn.added @wip` scenarios: (a) an invalid
  declaration refuses before any object is read and names the declaration
  error; (b) a malformed batch entry refuses before any import and names the
  batch line. The `## Interactions` inventory does not change because the
  consumer action is the same.
- **Source:** the use case's main-flow validation step and its extensions; the
  minimal guarantee.
- **Rejected:** covering them only in design prose — an unobserved refusal can
  regress silently.

### G7 (major) — can a missing fixture masquerade as a refusal?

- **Question:** an unknown path returned 404, which is also a legitimate
  provider error. A refusal scenario could pass because the fake was
  incomplete.
- **Disposition:** unknown paths return a distinctive **599** marker (with an
  `X-Fake-Unknown-Path` header) and journal an unknown-path entry; every
  refusal step asserts the journal contains no unknown-path entry. Update the
  test seam and the matrix's driving mechanism.
- **Source:** testability evidence (the fake is the digital twin; 404 is
  ambiguous) and the executable-spec strictness requirement.
- **Rejected:** keeping 404-only, which cannot distinguish a missing fixture
  from a missing live object.

### G8 (major) — where may the applied state file live?

- **Question:** the command reads `--state` but never passes `-state` to
  `tofu`; a state path outside the staged workspace would be silently ignored.
- **Disposition:** `--state` must lie inside `--declaration`; preflight refuses
  otherwise. Staging copies the state to the staged workspace's root
  `terraform.tfstate` (the local backend's path). Update usage, steps and the
  exception fix.
- **Source:** repository evidence (local backend state path) and the adoption
  guarantee that the binding is recorded at the named path.
- **Rejected:** passing `-state` (deprecated/conditional across commands);
  allowing arbitrary state paths.

### G9 (major) — is a state encrypted with another passphrase observed?

- **Question:** the passphrase refusal was described, but no scenario drove an
  applied state that cannot be decrypted, and the failure table had no row.
- **Disposition:** add a regular `@givn.added @wip` scenario: a state encrypted
  with a different passphrase refuses before any binding and names the applied
  state. Add the preflight/decrypt row: `tofu state list` in the staged
  workspace before any import.
- **Source:** confirmed decision D1 (encrypted state, passphrase) and the
  minimal guarantee.
- **Rejected:** treating decryption failure as the generic "state cannot be
  written" case — it is a distinct cause with a distinct message.

### G10 (minor) — are the new stakeholders in chapter 1?

- **Question:** chapter 1's stakeholder table listed the release owner but not
  the organization owner or the contributor reviewer.
- **Disposition:** add both rows, naming their confirmed Persona IDs and their
  authority boundaries.
- **Source:** confirmed Personas (topic Q7) and arc42 chapter 1 coverage.
- **Rejected:** relying on the use case's Persona list alone (chapter 1 is the
  durable stakeholder view).

### G11 (minor) — is the "local filesystem only" claim true for the adoption tooling?

- **Question:** chapter 3 said "all interfaces are the local filesystem", but
  the adoption tooling reads the GitHub REST API.
- **Disposition:** scope that claim to the converter and add the adoption
  tooling's read-only GitHub REST API interface to the technical-context table.
- **Source:** repository evidence (chapter 3's own public-home section) and the
  adoption design.
- **Rejected:** leaving the unconditional claim, which the adoption tooling
  falsifies.

### G12 (minor) — what does the glossary say about the pages rule set?

- **Question:** the glossary stated the pages repository's rule set "must
  bypass its publish deploy key", but D4 is carried to
  `publish-site-plumbing` and no pages rule set exists yet.
- **Disposition:** state that the pages rule set, when it exists, must not block
  the publish deploy key, and that whether it bypasses the key is the carried
  question decided by `publish-site-plumbing`.
- **Source:** carried decision D4 (seed Q2) and confirmed decision D3.
- **Rejected:** recording the bypass as settled here.

### G13 (minor) — is the OpenTofu version actually pinned?

- **Question:** the design said "latest stable at setup", which makes the tool
  version whatever a runner resolves.
- **Disposition:** pin OpenTofu **1.13.1** (verified installed on this machine)
  in `infra/.opentofu-version`; CI and setup install exactly that version; keep
  `required_version = ">= 1.7"` for users. Update the technology row, file
  table, chapter 2/4 and the setup/CI descriptions.
- **Source:** repository/live evidence (tofu 1.13.1 installed) and the
  reproducibility goal.
- **Rejected:** resolving "latest stable" at setup.

### G14 (minor) — what exactly does the no-write assertion say?

- **Question:** "no non-GET request" is wrong because the provider reads
  through GraphQL query POSTs.
- **Disposition:** align design test seam, chapter 6 notes and QS-016 to "no
  refused write and no mutation (a GraphQL query POST is a read)".
- **Source:** provider evidence (GraphQL reads are POST) and testability.
- **Rejected:** an assertion that would fail on a legitimate query.

### G15 (minor) — does the fake serve the provider's configure read?

- **Question:** the provider's configure step calls `Users.Get(owner)`, which
  the endpoint list and fixture tree omitted.
- **Disposition:** add `GET /users/{owner}` to the expected endpoints and
  `users/<owner>.json` to the fixture tree.
- **Source:** provider evidence (`Users.Get(owner)` during configure).
- **Rejected:** discovering the missing fixture only through a failed import.

### G16 (minor) — where is the live-read evidence committed?

- **Question:** the design said "`results/`", which is not an evidence path for
  this change.
- **Disposition:** capture it under
  `givn/changes/declare-public-home/evidence/live-read/` and add it to the file
  table.
- **Source:** the change's evidence discipline and the live read recorded in the
  topic session.
- **Rejected:** leaving the vague `results/` path.

### G17 (minor) — does the first adoption create both committed state files?

- **Question:** step 5 kept "the previous state" as `<state>.backup`, but on the
  first adoption there is no previous state, so the backup would be absent.
- **Disposition:** on the first adoption create `<state>.backup` as a copy of
  the new state; later runs preserve the previous state. Update step 5, the
  applied-state section and the file table.
- **Source:** confirmed decision D1 (both state files committed).
- **Rejected:** committing an absent backup after the first adoption.

### G18 (minor) — what is the real coverage command?

- **Question:** the coverage table showed `measure-coverage.sh --tags`, but the
  script takes no `--tags`; it produces both reports itself.
- **Disposition:** describe the real command: `./measure-coverage.sh` takes no
  `--tags` and produces both the non-e2e and e2e reports.
- **Source:** repository evidence (`measure-coverage.sh`).
- **Rejected:** documenting flags the script does not accept.

### G19 (minor) — does the missing-object Given say the batch selects it?

- **Question:** the scenario's Given named the declaration but not the batch,
  so "selected but missing" was not established.
- **Disposition:** the Given states that the selected batch names the missing
  object.
- **Source:** the minimal guarantee (the batch is the selected set).
- **Rejected:** relying on the declaration alone to imply selection.

### G20 (minor) — do the refusal scenarios assert a reason, not only a name?

- **Question:** the unreadable and missing-object scenarios asserted only the
  object name.
- **Disposition:** add a Then asserting the refusal reason to both; align
  chapter 8 to say every refusal names the object and the reason.
- **Source:** testability (a reason is observable output) and the minimal
  guarantee.
- **Rejected:** object-only assertions, which a generic failure message would
  satisfy.

### G21 (minor) — who performs the strict-mode proof?

- **Question:** the design said the setup script proves strict mode by mutating
  a step, but setup is not the implementation task.
- **Disposition:** record the proof as an implementation task for `tasks.md`,
  not a setup-script mutation; update design and the E2E section.
- **Source:** repository evidence (`fail_on_skipped()` is a runner property) and
  task ownership.
- **Rejected:** mutating the repository from a setup script.

### G22 (minor) — is the fake's mutation deterministic?

- **Question:** `mutate.json` replaced a body "after its first read", which
  races the provider's read order.
- **Disposition:** mutate after an explicit per-path request count
  (`after_requests: N`) or a step-controlled trigger.
- **Source:** testability (determinism) and the live-change scenario.
- **Rejected:** first-read mutation.

### G23 (minor) — is the Pages precondition stated?

- **Question:** the use case adopts the Pages source but its Preconditions did
  not state that the Pages site exists.
- **Disposition:** add "The Pages site exists on the pages repository."
- **Source:** the use-case contract and the adopted `github_repository_pages`
  resource.
- **Rejected:** relying on "both repositories exist" to imply a built site.

### G24 (minor) — where is the Frame's "state kept out of Git" superseded?

- **Question:** the confirmed Frame's supplied direction says local state is
  kept out of Git; D1 commits it encrypted.
- **Disposition:** record in the proposal's Frame-refinement paragraph and in
  the design's applied-state section that D1 supersedes that direction; do not
  rewrite the confirmed Frame.
- **Source:** confirmed decision D1 and the rule that a confirmed Frame is not
  rewritten.
- **Rejected:** editing the Frame document.

### G25 (minor) — does ADR-0006's existing-ADR check name ADR-0005?

- **Question:** ADR-0006 said only that the register holds no record about
  state storage; it did not name ADR-0005 or state the distinct boundary.
- **Disposition:** name ADR-0005 and state that ADR-0006 owns the distinct
  boundary (state storage, secrecy, where drift runs); update the verdict
  evidence in ADR-0006 and the change's `arc42.md`.
- **Source:** the ADR qualification procedure's existing-ADR gate.
- **Rejected:** treating ADR-0005 as covering the state decision.

## Hardening edits by file

- `givn/changes/declare-public-home/design.md`
  - Technology decisions: OpenTofu pinned to 1.13.1 via
    `infra/.opentofu-version`; CI/setup use it; `required_version = ">= 1.7"`
    kept for users.
  - File table: `.opentofu-version`, `evidence/live-read/`,
    `evidence/real-adoption/transcript.txt`, and the archive-only `cliff.toml`
    obligation.
  - Declaration: explicit monorepo/pages booleans; the provider's `false` for an
    omitted Optional-only boolean; the first-plan difference list; live-read
    evidence path.
  - Adoption command: state containment, staging before
    `tofu init -backend=false`/`tofu validate`, decrypt check via
    `tofu state list`, per-reason failure rows, first-adoption backup copy.
  - Applied state: Frame supersession by D1 and the backup rule.
  - Test seam: 599 unknown-path marker, `GET /users/{owner}`, deterministic
    `mutate.json`, GraphQL wording.
  - Runner/steps: strict proof is a task; monorepo-only priming for the
    already-bound scenario.
  - E2E, coverage table, local runnability and interface obstacles updated
    (fresh-checkout init, state path, pinned version).
  - New sections: required post-GREEN real adoption and archive-time allow-list
    obligation.
  - Risks: unknown-path marker, pages-repository first-plan differences, drift
    reporting out of scope.
- `givn/changes/declare-public-home/specs/adopt-existing-configuration/adopt-existing-configuration.feature`
  - Added `@givn.added @wip` scenarios for invalid declaration, malformed batch
    entry and wrong-passphrase state.
  - Missing-object Given now names the selected batch; unreadable and
    missing-object Thens assert the reason.
  - `@e2e` preserved on the main scenario; all scenarios remain `@wip`.
- `givn/changes/declare-public-home/specs/adopt-existing-configuration/usecase.md`
  - Precondition: the Pages site exists on the pages repository. The
    `## Interactions` inventory is unchanged.
- `givn/changes/declare-public-home/proposal.md`
  - Frame refinement records the D1 supersession of "local state, kept out of
    Git"; D2 states the explicit booleans, the pages-repository values and the
    revised first-plan difference list. No scope change.
- `docs/arc42/01-introduction-and-goals.md`
  - Stakeholder rows for the organization owner and contributor reviewer.
- `docs/arc42/02-architecture-constraints.md`
  - OpenTofu pin in the constraint; Optional-only boolean exception.
- `docs/arc42/03-context-and-scope.md`
  - "Local filesystem only" scoped to the converter; read-only GitHub REST API
    row for the adoption tooling.
- `docs/arc42/04-solution-strategy.md`
  - OpenTofu pin in the technology table.
- `docs/arc42/06-runtime-view.md`
  - Adoption scenario: input preflight, staged init/validate, decrypt check,
    backup rule, refusal-reason notes, GraphQL/unknown-path journal wording.
- `docs/arc42/08-crosscutting-concepts.md`
  - All-or-none adoption states that every refusal names the object and the
    reason.
- `docs/arc42/10-quality-requirements.md`
  - QS-016 uses the journal wording; QS-018 includes an undecryptable state and
    the reason.
- `docs/arc42/12-glossary.md`
  - **Protection ruleset** contract use no longer asserts a bypass; D4 is
    carried to `publish-site-plumbing`.
- `docs/arc42/adr/ADR-0005-declare-the-public-home-with-opentofu.md`
  - Confirmation: validate inside the staged copy after init; no real drift
    claim; post-GREEN real adoption is the evidence.
- `docs/arc42/adr/ADR-0006-encrypted-local-applied-state.md`
  - Confirmation: drift reporting out of scope; post-GREEN real adoption,
    transcript and committed state are the evidence. Existing-ADR check names
    ADR-0005 and the distinct boundary.
- `givn/changes/declare-public-home/arc42.md`
  - ADR-0006 verdict evidence names ADR-0005.
- `cliff.toml`
  - **Not edited.** The archive-time obligation is recorded in `design.md` and
    no capability is added before `givn/specs/adopt-existing-configuration/`
    exists.

## Recorded deferrals

None. Every question is resolved by evidence, a confirmed decision, or a
requirement of the use case's guarantee; no operator decision was needed.

## Verification

- `givn lint --change declare-public-home`: exit 2 — 11 `@wip` findings and two
  advisories (a shared scenario shape between the wrong-passphrase and
  state-write refusals, and the live-change scenario as a subset of the
  missing-object scenario). No errors. The advisories are accepted: the two
  state refusals have different causes and different messages, and the
  live-change scenario drives a distinct detection step.
- `givn status --change declare-public-home`: `arc42-docs-done`; design-review is
  the next required artifact and is completed by this file.
- No `@e2e` tag was removed. `tasks.md` was not written. The seed, the published
  topic document and the topic question log were not edited.

DESIGN-REVIEW: PASS
