# Public-home infrastructure — question log

## Q1 — Adopt the Frame, and does the site-publishing plumbing belong to it?

- Status: closed 2026-10-17 (user)
- Blocks: Persona proposals and Event Storming. Neither can start from an
  unagreed problem, boundary, or stakeholder list.
- Context: the user's draft `private/opentofu.md` proposes OpenTofu for the
  GitHub organization and its repositories. Research on 2026-10-17 found that
  the repository transfer is already done, that the pages repository already
  exists with only an initial commit and the default Jekyll site, and that the
  monorepo has no site source (`web/`, `ffi/wasm/` are absent). The draft also
  supplies a deploy key and a `PAGES_DEPLOY_KEY` secret for a publishing
  workflow that does not exist yet. The Frame in `frame.md` therefore has one
  genuine boundary fork: whether that dormant publishing plumbing is part of
  this topic or waits for the website topic. Everything else in the Frame is
  the problem, scope, and stakeholder list shown there.
- Question: adopt the Frame for the project's public-home infrastructure as
  drafted — the organization and both repositories declared in `infra/`, the
  `main` protection ruleset, the security features, the Pages configuration,
  the deploy key and its secret, the import of today's state, and the drift
  report — or revise a named part before Personas and Event Storming?

  The fork the evidence forces: the draft's deploy key and secret have no
  consumer until the site's build workflow exists. The rest of the Frame does
  not depend on that answer.

- Options:
  1. **Adopt the Frame as drafted, including the publishing plumbing.**
     Meaning: `infra/` declares the pages repository's Pages configuration,
     the write-enabled deploy key on it, and the `PAGES_DEPLOY_KEY` secret on
     the monorepo now; the publishing workflow arrives later.
     In practice: the first apply imports both repositories, sets the
     organization profile and defaults, the `main` ruleset and the security
     features, enables Pages on the pages repository, and stores a deploy key
     and a secret that nothing reads yet.
     Gain: one bootstrap and one declaration for the whole public home; the
     website topic starts from plumbing that is already reviewed and applied.
     Cost: a credential exists before its consumer; rotating or removing it
     later means touching a topic that has already handed off; the secret's
     plaintext sits in the state until the site work begins.
     Authorization: topic-local ideation only.
  2. **Adopt the Frame but exclude the publishing plumbing.**
     Meaning: the Frame covers the organization, both repositories' settings,
     the rulesets, and the security features; the deploy key, the secret and
     the Pages source setting wait for the website topic, which adds them with
     the workflow that uses them.
     In practice: the first apply imports both repositories and sets their
     settings and rulesets, but registers no deploy key and creates no
     `PAGES_DEPLOY_KEY`; the pages repository keeps its default Jekyll site
     until the site topic changes it.
     Gain: every credential arrives with its consumer; this topic's handoff
     does not depend on a product surface that does not exist.
     Cost: the public-home topic is revisited when the site starts, and the
     Pages configuration is then reviewed in a second topic; the boundary
     between "repository settings" and "publishing plumbing" must be stated.
     Authorization: topic-local ideation only.
  3. **Revise another named part of the Frame.**
     Meaning: change the problem, the vision, a scope boundary, or a
     stakeholder interest before Personas and Event Storming derive from it.
     In practice: for example, say that drift reporting is out of scope and
     the declaration is applied only on demand, or that organization
     membership and teams are in scope from the start.
     Gain: exploration starts from the correct objective.
     Cost: the affected phases wait for the correction.
     Authorization: topic-local ideation only.
- Later-change cost: the Frame is the basis for Persona proposals, the event
  chain, and every use-case contract. Changing it after those exist means
  revisiting the affected Persona reviews, events, rules and guarantees. It
  is a coherent basis, not an irreversible commitment. Adopting the Frame
  authorizes topic-local ideation only: no Persona is confirmed by it, no
  `infra/` file is written, no provider state is created, no GitHub setting is
  changed, and no change or Handoff is started.
- Resolution: **Adopt the Frame as drafted, including the publishing plumbing** (option 1) because the user answered "1." The deploy key, the `PAGES_DEPLOY_KEY` secret and the Pages configuration belong to this topic; the publishing workflow that consumes them arrives later. Rejected: **Adopt without the publishing plumbing** because the user did not choose the narrower boundary; and **Revise another named part** because the user named no revision. The same answer authorizes topic-local ideation only; it confirms no Persona, writes no `infra/` file, creates no state, and changes no GitHub setting.

### Q1 evidence update 2026-10-17 (Frame boundary review)

The adoption covers the Frame text, which names both protection rulesets
(the monorepo's `main` and the pages repository) and the publishing plumbing.
Refinements recorded without reopening the decision:

- The organization settings already exist and must be imported; they are not
  created fresh.
- Pages is already enabled and already serves the stock Jekyll site; import
  `github_repository_pages` in its existing form (legacy, `main`, `/`), do not
  enable it anew.
- The stock site stays until the separate website topic; site content is
  outside this Frame.
- The configuration drift report requires read access to the applied state, so
  local-only state cannot support it (now Q4).

## Q2 — May the pages repository's ruleset bypass the publish deploy key?

- Status: open
- Blocks: the pages repository's protection contract and the use case that
  changes repository rules.
- Context: the pages repository's only writer is the write-enabled deploy key
  through which the monorepo publishes the site. A protection ruleset that
  requires pull requests, or that admits only changes passing checks, would
  block that push. GitHub rulesets support a `DeployKey` bypass actor
  (`bypass_mode = "always"`); the provider exposes it. The Frame places a
  ruleset on the pages repository without stating how publishing survives it.
- Question: should the pages repository's ruleset grant the publish deploy key
  a bypass, restrict itself to rules that cannot block a push, or be omitted?
- Options:
  1. **Bypass the deploy key; protect everything else.**
     Meaning: deletion, non-fast-forward and any required checks apply to
     humans and other actors; the deploy key may always push.
     In practice: a manual force push is refused, while the publishing workflow
     still writes `main`.
     Gain: the repository is protected against accidental human damage without
     breaking publishing.
     Cost: the key's own pushes are unreviewed by definition; its compromise is
     not contained by the ruleset.
  2. **Protect only what cannot block a push.**
     Meaning: the ruleset forbids deletion and non-fast-forward but imposes no
     pull-request or check requirement.
     In practice: a human cannot delete `main`; the deploy key pushes as
     before.
     Gain: simpler, no bypass actor to maintain.
     Cost: nothing forces a review of a human change to the pages repository.
  3. **No ruleset on the pages repository.**
     Meaning: the pages repository keeps today's unprotected state.
     In practice: `main` can be force-pushed or deleted by any writer.
     Gain: nothing to configure or drift-check.
     Cost: a bad publish or a mistake destroys the only copy of the site.
- Resolution: pending.

## Q3 — Does the monorepo's `main` ruleset require pull requests?

- Status: open
- Blocks: the monorepo ruleset contract and the maintainer's landing routine.
- Context: `main` is protected today by deletion and force-push rules, and the
  maintainer pushes directly. A ruleset that requires pull requests, or that
  requires status checks without a bypass, ends that routine: every change
  then waits for a pull request and its checks. The archived release work
  deferred the required-check ruleset until the repository became public; it
  is public now.
- Question: should the ruleset require pull requests for every change, allow
  the owner to bypass them, or only require the protection rules without a
  pull-request rule?
- Options:
  1. **Require pull requests, no bypass.**
     Meaning: every change reaches `main` through a pull request with the
     required checks.
     In practice: the maintainer opens a pull request for a one-line docs fix
     and waits for the checks.
     Gain: one path for every change; the gate cannot be skipped by its author.
     Cost: slower small fixes; the maintainer's own workflow changes.
  2. **Require pull requests, owner bypass `always`.**
     Meaning: pull requests are the default, but the owner may push directly.
     In practice: routine fixes land directly; outside contributions use pull
     requests.
     Gain: keeps the current routine while giving contributors a defined path.
     Cost: the required checks are advisory for the owner; two landing paths
     exist.
  3. **Protection only, no pull-request rule.**
     Meaning: deletion, force-push and linear-history rules apply; anyone with
     write access pushes directly.
     In practice: the maintainer pushes as today; a contributor with write
     access could too.
     Gain: simplest ruleset matching current practice.
     Cost: no review step and no required checks at landing.
- Resolution: pending.

## Q4 — Which state access supports both the apply and the configuration drift report?

- Status: closed 2026-10-17 (user; supersedes the earlier option set)
- Blocks: the drift report's feasibility and the state-security rule.
- Context: the draft keeps state locally and out of Git, and adds a read-only
  `tofu plan -detailed-exitcode` job in CI. A plan needs the applied state to
  tell a difference from a fresh creation; a CI job cannot read the
  maintainer's local file. The state also holds the deploy private key in
  plaintext.
- Question: where does the applied state live so that the local apply and the
  CI configuration drift report both work, while the deploy private key stays
  out of the repository?
- Options:
  1. **Encrypted remote backend.**
     Meaning: state lives in an access-controlled backend; CI reads it with a
     read-only credential.
     In practice: `tofu plan` in CI compares against the same state the
     maintainer applied.
     Gain: honest drift reporting, one source of state, encrypted at rest.
     Cost: a backend to operate or pay for; another credential.
  2. **Local state with an encrypted copy exported for CI.**
     Meaning: the maintainer applies locally and publishes an encrypted state
     snapshot the CI job can read.
     In practice: the drift job uses the last exported snapshot; drift since
     that export is invisible.
     Gain: no backend service.
     Cost: a stale snapshot reports stale drift; the export step is manual and
     easy to forget.
  3. **Local drift only; no CI report.**
     Meaning: the maintainer runs the plan locally; CI checks nothing.
     In practice: configuration drift is found only when someone looks.
     Gain: no extra credential or service.
     Cost: the declared configuration can diverge silently, which is the
     problem this topic exists to solve.
- Resolution: **Local backend with the state encrypted at rest and committed to the repository; the configuration drift check runs locally, not in CI.** The user answered on 2026-10-17 after establishing that GitHub has no managed state service and none of its features provides the GET/POST/LOCK semantics of the `http` backend. Chosen mechanics: OpenTofu's `encryption` block with PBKDF2 deriving an AES-GCM key from a passphrase, `state` and `plan` both `enforced = true`, so plaintext state cannot be written; `infra/terraform.tfstate` and its backup are committed (the earlier `.gitignore` rule is revoked), protected at rest by that encryption. The passphrase (at least 16 characters) lives only in the maintainer's password manager and `TF_VAR_state_passphrase`; never a default and never a GitHub secret. No locking: acceptable under a single-maintainer assumption. A CI drift report is withdrawn because the passphrase in CI would make the deploy private key in the state decryptable; the drift check stays local. `required_version = ">= 1.7"` records the state-encryption floor. Rejected: **encrypted remote backend** because it adds an external account and token for roughly eight resources; **Cloudflare R2** because its S3 compatibility needs an extra worker for locking; **HCP Terraform** because it is a vendor dependency OpenTofu does not guarantee; **self-hosted TACOS** because it is oversized here; and **a CI passphrase secret** because it would decrypt the deploy key. The Frame's scheduled/per-push configuration drift report is withdrawn by this decision; the topic document remains historical.

## Q5 — Which current repository settings are intended values?

- Status: closed 2026-10-17 (user)
- Blocks: the repository declarations and the first plan's expected diff.
- Context: the draft supplies values (no wiki, no projects, squash-only
  merges, a description and license template) that differ from the live
  repository in at least one observable way: projects are currently enabled.
  Authenticated reads on 2026-10-17 establish the rest: all three merge
  methods are allowed, delete-branch-on-merge is off, discussions are off, and
  the organization profile fields are empty. The first apply will change
  whatever differs, so each difference needs a decision rather than an
  inherited default.
- Question: should the declaration adopt the draft's repository settings,
  preserve the live values, or decide setting by setting?
- Options:
  1. **Adopt the draft's values.**
     Meaning: the first apply turns projects off, restricts merges to squash,
     and rewrites the description.
     In practice: the repository's settings change at the first apply.
     Gain: one intended state, matching the draft.
     Cost: the changes are unrelated to the infrastructure goal and appear in
     the first plan.
  2. **Preserve the live values exactly.**
     Meaning: the declaration records today's settings, whatever they are.
     In practice: the first plan is empty for those fields; the declaration
     captures the status quo.
     Gain: no incidental change; the import proves itself.
     Cost: settings nobody chose (projects on) are ratified by code.
  3. **Decide setting by setting.**
     Meaning: each field with a difference is named and chosen.
     In practice: projects off, wiki stays off, merges decided explicitly.
     Gain: no setting enters the declaration by inertia.
     Cost: a longer first pass.
- Resolution: **Adopt the draft's values where they still describe today's project** (option 2), because the user wants the declaration as current and early as possible. The declared values: projects off; merge commits and rebase merges off, squash merges only; delete-branch-on-merge on; today's description ("Convert a duty plan to a calendar") and topics kept; wiki off; discussions off; auto-merge off; forking allowed; `license_template` omitted because it has no effect on an imported repository; Dependabot alerts and security updates through their dedicated resources. The first apply will therefore change `has_projects`, `allow_merge_commit`, `allow_rebase_merge` and `delete_branch_on_merge`; the security seed's preview names them, and its guarantee is about the security fields, not about changing nothing else. Rejected: **preserve live values** because it ratifies unused settings; **decide field by field** because the user chose the draft's values as a set.
- Update 2026-10-18 (user, after archive): the squash-only part is superseded. The declaration now allows rebase merges and forbids squash and merge commits, so the history keeps every commit and stays linear; the first apply changes `allow_merge_commit`, `allow_squash_merge` and `delete_branch_on_merge`, while `allow_rebase_merge` already matches live. The pages repository's projects are also off: projects are not needed anywhere. The original resolution stays as the record of what was decided before this update.

## Q6 — Is team access part of this topic?

- Status: open
- Blocks: whether the organization declaration includes teams and repository
  roles, or only the owner's own access.
- Context: the organization currently has one member, its owner. The frame
  keeps membership and team access outside, but the organization's
  member-default settings are inside. If a second maintainer or reviewer is
  invited, access should not be a separate undocumented click.
- Question: should this topic declare teams and their repository roles, or
  leave access to a later decision?
- Options:
  1. **Leave access out.**
     Meaning: the organization and repository settings are declared; who may
     write is decided in the web interface when it happens.
     In practice: the first topic ships without a team; inviting someone is a
     manual act.
     Gain: smaller scope; no token can grant access by accident.
     Cost: the one configuration with security consequences stays outside the
     reviewed declaration.
  2. **Include a maintainer team and repository roles.**
     Meaning: the declaration creates the team and grants its repository roles.
     In practice: accepting a maintainer means adding a person to the team, not
     editing repository settings.
     Gain: access is reviewed and reproducible.
     Cost: the state and token can change who can write; the topic grows.
- Resolution: pending.

## Q7 — Confirm the Persona review lenses

- Status: closed 2026-10-17 (user)
- Blocks: Persona reviews of the Event Storming and the use-case contracts.
- Context: the Frame names four stakeholder types. The project collection held
  two Personas; only the crew-member lens matched without a new segment. The
  organization owner and the contributor reviewer need segments whose
  authority boundaries differ from the existing release owner's. The user was
  shown each lens's role, authority boundary, evaluation capability and the
  concrete artifact it would refuse.
- Question: confirm the three review lenses — organization owner
  (`project-maintainer--organization-owner`), contributor reviewer
  (`contributor--infrastructure-reviewer`), and the reused crew member
  (`crew-member--personal-calendar-owner`) — or name one to revisit?
- Options:
  1. **Confirm all three.**
     Meaning: all three become advisory review lenses for this topic.
     In practice: the owner lens refuses an event chain that recreates existing
     objects; the contributor lens refuses a use case that assumes a reviewer
     can apply; the crew-member lens refuses a ruleset that blocks the release
     tag.
     Gain: every stakeholder type has a reviewer for later artifacts.
     Cost: three lenses to consult at each phase boundary; their findings need
     arbitration.
     Authorization: topic-local review lenses only; no project promotion.
  2. **Revise one named lens.**
     Meaning: drop or merge the named Persona before use.
     In practice: dropping the contributor lens because nobody contributes yet
     leaves infrastructure changes without a reviewer lens until a later topic.
     Gain: one less lens to consult.
     Cost: the affected stakeholder type has no reviewer; a later contribution
     topic adds it.
- Resolution: **Confirm all three** because the user answered "confirmed." with
  no revision. Rejected: **Revise one named lens** because the user named none.
  The confirmation authorizes advisory review lenses only; it does not promote
  a Persona to the project collection, create a Gherkin Actor, write `infra/`,
  touch GitHub, or start a change.

## Q8 — Event Storming term check

- Status: closed 2026-10-17 (user)
- Blocks: the event chain and canvas, and every later use-case contract that uses these nouns.
- Context: six core terms emerged from the Frame and the draft and are needed before the canvas can rely on them. Each was described in one sentence with its recorded collision check; `Configuration drift` must stay qualified because the glossary's **Drift check** is the change-log comparison, and `Adoption` collides in prose with the prior topic's contract adoption.
- Question: confirm all six terms — **Declared configuration**, **Configuration drift**, **Protection ruleset**, **Deploy key**, **Adoption**, **Configuration change** — or name the terms to revisit?
- Options:
  1. **Confirm all six.**
     Meaning: the names enter the topic record and later contracts.
     In practice: the canvas reads "declared configuration" and "configuration drift"; a use case says a deploy key bypasses the pages ruleset.
     Gain: one vocabulary across canvas, use cases and seeds.
     Cost: none beyond adopting the recorded definitions.
     Authorization: topic-local vocabulary only; the durable glossary is untouched until Handoff.
  2. **Revisit named terms.**
     Meaning: a named term gets its own naming dialog before confirmation; the other terms stand.
     In practice: renaming "Declared configuration" to "Public-home declaration" would change the canvas and every contract that uses it.
     Gain: a name that fits better.
     Cost: the affected artifacts wait for the naming dialog.
- Resolution: **Confirm all six** because the user answered "confirmed." with no term named. Rejected: **Revisit named terms** because the user named none. The terms are topic-local; no durable glossary entry is written by this confirmation.

## Q9 — Do the classic protection and the ruleset both become declarations?

- Status: closed 2026-10-17 (user)
- Blocks: the monorepo protection declaration and the first plan's expected
  live changes.
- Context: `main` currently has two overlapping protections: a repository
  ruleset `main protection` (deletion, non-fast-forward, linear history) and
  classic branch protection with the same rules. The provider can declare
  both (`github_repository_ruleset`, `github_branch_protection`). Declaring
  both preserves today's duplication; declaring one and retiring the other
  changes the live repository.
- Question: should the declaration adopt both protections, consolidate them
  into one ruleset and retire the classic protection, or adopt only the
  ruleset and leave the classic protection unmanaged?
- Options:
  1. **Adopt both as they are.**
     Meaning: both objects enter the declaration with their current rules.
     In practice: the first plan shows no protection change; two objects stay
     in the repository settings.
     Gain: no live change; the import proves itself.
     Cost: the duplication persists and every ruleset change must be mirrored
     or consciously left different.
  2. **Consolidate into the ruleset; retire the classic protection.**
     Meaning: the ruleset carries the rules; the classic protection is removed
     by the same apply that declares the ruleset.
     In practice: one place defines `main`'s protection; the apply removes the
     legacy object.
     Gain: one object, one place to review.
     Cost: the first apply changes live protection; classic settings not
     represented in the ruleset (for example enforce-admins) must be mapped or
     consciously dropped.
  3. **Declare only the ruleset; leave classic protection unmanaged.**
     Meaning: the ruleset is declared and drift-checked; the classic protection
     is whatever it is.
     In practice: a manual change to classic protection is invisible to the
     configuration drift report.
     Gain: smallest declaration.
     Cost: an unmanaged protection path remains, which is the problem this
     topic exists to solve.
- Resolution: **Consolidate into the ruleset and retire the classic protection** (option 2), because the user chose it and explicitly wants linear history retained. The consolidated ruleset carries deletion, non-fast-forward and `required_linear_history`. The live ruleset has no bypass actors, so it already binds the owner, including the linear-history rule; the classic protection's `enforce_admins: false` is the weaker duplicate and is **not** migrated. No bypass actor is added. Removing the classic protection is a live change and therefore cannot happen during adoption: the declaration omits the classic object, adoption binds the ruleset, and the first apply (`enable-repository-security`) removes the classic protection. **Update 2026-10-18 (observed; supersedes this sentence):** the classic protection was removed by hand on 2026-10-18, before `enable-repository-security` was applied — the authenticated read `GET /repos/duty2cal/duty2cal/branches/main/protection` returns 404 "Branch not protected" (`givn/changes/enable-repository-security/evidence/live-read/classic-protection.json`). The security change therefore does not remove it; it verifies it stays gone and declares nothing for it. The ruleset remains the only protection object. Q3 (whether to add a pull-request or required-check rule) stays carried to that apply seed and does not change this declaration. Rejected: **adopt both** because the duplication persists; **declare only the ruleset and leave classic unmanaged** because an unmanaged protection path remains. Linear history is possible and already enforced by the ruleset; direct pushes stay possible because the rule forbids merge commits, not pushes.

## Q10 — Does the reviewer's decision gate the apply?

- Status: open
- Blocks: the apply flow's rule for an unreviewed or rejected revision, and the
  reviewer Persona's authority.
- Context: the Frame gives the contributor reviewer a review step, and the
  contributor Persona says it may approve a change. The apply use case treats
  review as a supporting step, but nothing yet decides whether a rejected or
  unreviewed revision may be applied. The answer also decides whether the
  apply's success guarantee can require approval.
- Question: should the apply refuse a revision the reviewer has not approved,
  record the review but let the owner apply anyway, or not model review in the
  apply flow at all?
- Options:
  1. **Gating: no approval, no apply.**
     Meaning: the apply refuses an unreviewed or rejected revision.
     In practice: the owner cannot apply a one-line settings change until the
     reviewer approves it.
     Gain: the reviewer's authority is real; every applied change has a second
     reader.
     Cost: a solo maintainer blocks on a second person; the opening state
     (reviewer is the owner) needs a bypass or the rule is empty.
  2. **Advisory: review is recorded, the owner decides.**
     Meaning: a review can approve or reject; the owner may apply either way.
     In practice: a rejected revision can still be applied, with the rejection
     visible in the history.
     Gain: no blocked solo workflow; the review is evidence.
     Cost: the reviewer cannot stop a change; the Persona's "may approve" is
     weaker than it reads.
  3. **No review step in the apply flow.**
     Meaning: review happens only if the ruleset's pull-request path requires
     it (Q3).
     In practice: with direct pushes allowed, a change reaches `main` and the
     live configuration without a review.
     Gain: simplest flow.
     Cost: the contributor lens has nothing to gate; the apply contract says
     nothing about review.
- Resolution: pending.

## Q11 — What may a configuration change delete?

- Status: open
- Blocks: the apply contract's deletion rule and the declaration's protection.
- Context: a configuration change that removes an object from the declaration
  would delete it. The declaration covers both repositories, the organization
  settings, rulesets, the Pages source, the deploy key and the secret.
  Deleting a repository destroys the public home and its history; deleting a
  deploy key or a secret revokes publishing. The Frame says nothing about
  deletion.
- Question: which declared objects may an apply delete, and which must it
  refuse to delete?
- Options:
  1. **Never delete a repository, the organization, or a published release;
     allow lesser objects.**
     Meaning: removing a deploy key, secret, ruleset or a setting from the
     declaration removes it live; removing a repository is refused.
     In practice: deleting the pages repository from the declaration fails the
     apply and names it; removing the deploy key succeeds.
     Gain: the destructive class is bounded while ordinary cleanup works.
     Cost: a repository can only be removed by hand, outside the declaration;
     the drift report then names it undeclared.
  2. **Delete any declared object, with the preview naming every deletion.**
     Meaning: the declaration is the whole truth; removing an object deletes
     it.
     In practice: dropping the pages repository from the declaration deletes
     the site and its history on the next successful apply.
     Gain: one rule, no special cases.
     Cost: a mistaken edit can destroy the public home; the state cannot
     restore a deleted repository.
  3. **Never delete; removal from the declaration only stops managing the
     object.**
     Meaning: an apply never deletes anything; an object removed from the
     declaration becomes unmanaged and the drift report names it undeclared.
     In practice: removing a secret leaves it live; the owner deletes it by
     hand.
     Gain: the apply is non-destructive by construction.
     Cost: cleanup is manual, and the undeclared object stays in the drift
     report until removed.
- Resolution: pending.

## Q12 — Confirm the Handoff records

- Status: closed 2026-10-17 (user)
- Blocks: publishing the readable topic document, the four change seeds, the two Persona promotions and the six glossary terms.
- Context: the Handoff prepared in `handoff.md` maps each `!` use case to its own permanent leaf, proposes six durable terms, promotes two Persona segments and cuts four deliverable seeds. Publication is the last step of ideation; it creates no change.
- Question: confirm the Handoff records — the use-case mapping, the six terms, the two Persona promotions and the four seeds — or name a record to revise?
- Options:
  1. **Confirm all.** Publish the topic document, write the four seeds, promote the two Personas, add the six terms to the glossary.
     In practice: `/givn-propose declare-public-home` finds a ready seed with its guarantees and open questions.
     Gain: the topic closes; each seed enters the normal proposal flow.
     Cost: the published topic document is historical; seeds carry open questions.
     Authorization: publication/promotion only; no change start, no `infra/`, no GitHub write.
  2. **Revise a named record.** Publication waits for the correction; the other records stand.
     In practice: dropping `publish-site-plumbing` until the site exists, or keeping a Persona topic-local.
     Gain: the record matches the intent.
     Cost: the Handoff waits.
- Resolution: **Confirm all** because the user answered "confirmed." with no revision. Rejected: **Revise a named record** because the user named none.
