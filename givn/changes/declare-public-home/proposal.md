# Proposal: declare-public-home

## Release note

The public home's existing organization settings, its two repositories, the
protection rule for the default branch and the site source become part of the
project's reviewed configuration and are bound to what already exists on GitHub
without changing it, so the home can be reviewed, diffed and rebuilt instead of
remembered.

## Use-Case Context

- Use-case ID: `adopt-existing-configuration` — "Adopt the existing public-home
  configuration". New: the permanent corpus holds `export-duty-plan-calendar`
  and `publish-public-release`; neither owns a configuration declaration or an
  adoption. The delta creates the permanent leaf
  `givn/specs/adopt-existing-configuration/`.
- Ideation topic: `public-home-infrastructure`.
- Seed path: `givn/ideation/public-home-infrastructure/changes/declare-public-home.md`.
  Its status line already reads `Status: proposed by declare-public-home`; the
  seed's original text is unchanged.
- Confirmed Personas:
  - `project-maintainer--organization-owner` — the administration review lens;
    holds the authority, the token and the passphrase, and judges a batch
    adoption and its recorded state.
  - `contributor--infrastructure-reviewer` — the review-without-authority lens;
    reads the declaration, the batch and the recorded state, and cannot apply.
  - `crew-member--personal-calendar-owner` — the release-channel and future-site
    lens; nothing this change does alters what a crew member downloads.
  Persona IDs are recorded for review only; none is a Gherkin Actor biography.

The use case this change realizes is the ideation contract
[Adopt the existing public-home configuration](../../ideation/public-home-infrastructure/use-cases/adopt-existing-configuration.md)
in the topic [Public-home infrastructure](../../ideation/public-home-infrastructure/frame.md).
Its guarantee is unchanged: place the already-existing objects of the public
home under the declared configuration without changing them.

**Frame refinement recorded here.** The topic's Frame promised "a scheduled and
per-push configuration drift report" from CI. The state decision (D1) and the
requirement that the administer token never live in the repository it
administers withdraw that promise: the configuration drift check stays local
and is run by the maintainer. The Frame's vision otherwise stands.

## Problem / Opportunity

The organization and its repositories are configured by hand in the GitHub web
interface. The chapters state what the public home should be, but the intended
configuration exists nowhere as a declaration: it cannot be reviewed, diffed,
previewed, or rebuilt from the history.

The 2026-10-17 repository transfer is the realized instance. It happened as a
sequence of clicks, and it silently dropped every published artifact
attestation; the public text promising them had to be repaired in a follow-up
revision. Nobody could have reviewed the change before it happened or diffed
its consequence afterwards, because neither side existed as text.

The opportunity has two halves, and this change takes only the first:

- **Declare what is there.** The organization's profile and member defaults,
  both repositories' settings, the protection rule on the default branch and
  the site source are described once, in the repository, in the same history as
  the code they support.
- **Bind what already exists.** Those objects exist on GitHub today.
  Declaration alone is not enough: a first configuration change would either
  fail on objects it did not create or, worse, try to create duplicates. The
  already-existing objects have to be adopted — bound to their declared
  counterparts without being created, modified or deleted.

Adoption is also where the project's data-safety question becomes concrete: the
binding has to be recorded somewhere the local preview and drift check can read,
and that record must never be a public copy of live settings or, later, of a
publishing key.

## Proposed Solution

### One declared configuration

The public home is described in the repository, in the same reviewable form as
the rest of the project: the organization's profile and member-default
settings, both repositories with their description, topics, features and merge
strategies, the protection rule that governs the default branch, and the
published site's source. A reader can diff a change to it, and a reviewer can
see an infrastructure change beside the workflow or document it supports.

The declared values are chosen, not inherited. Where today's project still
matches the supplied draft, the draft's values are kept: projects off, squash
merges only, delete-branch-on-merge on, wiki and discussions off, auto-merge
off, forking allowed, the current description and topics kept. Where the draft
described something the project does not have yet — a browser interface in the
organization's profile wording — today's value stays. The first configuration
change, not the adoption, is where those declared values reach GitHub, and its
preview names every one of them.

The protection declaration carries one object: the protection rule that already
governs the default branch, with deletion, non-fast-forward and linear-history
rules and no bypass actor. The weaker duplicate legacy protection is not
declared; retiring it is a live change and belongs to the first configuration
change.

### One adoption action, all or none

The maintainer selects the objects that already exist and asks for them to be
adopted. Every object in the batch is bound or none is:

- The organization, both repositories, the protection rule and the site source
  are each bound to their declared counterpart by identity.
- Nothing live changes: no create, no update, no delete, no permission change,
  no feature toggle, no rebuild of the site. The declared values may differ
  from live; that difference is deliberately left for the first preview to name.
- A refusal names the object and the reason, and the whole batch is refused:
  an object that cannot be read, an object already bound, a declared object that
  does not exist live, a selected object the declaration does not name, a live
  object that changed after it was read, or a failure to record the binding.
- The site deploy key and the publishing secret do not exist and are never
  adopted; the change that publishes the site creates them.

### A binding that is safe to keep

The binding is recorded in an applied state that is part of the repository and
encrypted at rest with a passphrase only the maintainer holds. The passphrase
never has a default, never enters a repository secret and never reaches
automation. There is no remote state service and no lock: the project assumes
one maintainer applies at a time, and records that assumption rather than
pretending to enforce it. Because the automation that could run a drift report
would need the passphrase — and thereby the material that will later include
the publishing key — the configuration drift check stays on the maintainer's
machine.

### How the outcome is observed

The maintainer runs the adoption and reads its terminal output: one line per
bound object, or a refusal naming the object. The repository shows the
declaration and the recorded binding; GitHub shows the same settings it showed
before, which is the point.

## Capability Routing

`givn spec route --change declare-public-home` reported
`EXTEND export-duty-plan-calendar` (score 13, confidence: signal), from word
overlap alone — its matches fall on the word `route` and on generic
goal/interaction words shared with the conversion use case. That recommendation
is not followed: the conversion use case converts a duty plan into a calendar
and has no configuration, adoption or GitHub behaviour that this change could
extend.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `adopt-existing-configuration` | `EXTEND export-duty-plan-calendar` | `NEW in adopt-existing-configuration` | The goal — place already-existing public-home objects under the declared configuration without changing them — belongs to the organization owner's administration goal, not to the crew member's conversion goal. The route's signal is lexical: `route` and generic goal words, with no shared object or rule. The owning use case is an ideation contract for exactly this goal, mapped by the Handoff to its own permanent leaf. |

## Out of Scope

- **Creating the organization and transferring a repository into it.** GitHub
  exposes no creation endpoint for `github.com` and no transfer resource for
  the provider; both were one-time manual actions, already performed.
- **Any change to the live configuration.** This change runs no apply and
  changes no GitHub setting. The first configuration change — the security
  features, the removal of the duplicate legacy protection, the deploy key and
  the publishing secret, the pages repository's protection rule — is a separate
  change with its own preview.
- **The site's content and publishing workload.** The stock Jekyll site stays as
  it is; the site source is adopted, not rebuilt. The deploy key and the
  `PAGES_DEPLOY_KEY` secret belong to `publish-site-plumbing`.
- **The security features.** Secret scanning, push protection, Dependabot
  alerts and automated security updates belong to `enable-repository-security`.
- **Automated drift reporting.** No scheduled or per-push CI drift report: the
  state passphrase would decrypt the future publishing key, so the drift check
  stays local (D1).
- **Team access and organization membership.** The organization has one member,
  its owner; who joins later is a separate authority decision (Q6).
- **Release immutability, artifact attestations and private vulnerability
  reporting.** Settings and outcomes the provider cannot declare, or pipeline
  outcomes; they stay manual or belong to the release pipeline.
- **The product executable, its release workflow and the conversion
  specification.** No product behaviour changes; the converter never contacts
  GitHub.

## Open Questions

Every question the seed leaves open is dispositioned here before the specs are
written. `D1`–`D3` are the seed's own questions; `D4`–`D8` are the intersecting
questions the topic log carries, recorded so that nothing enters the
specification by inheritance.

### D1 — Where does the applied state live? (seed Q4)

- Question: where is the binding recorded so the local preview and the drift
  check can read it, while the material it will later hold stays out of the
  public repository — a remote backend, an exported snapshot, or local only?
- Options:
  1. **Encrypted remote backend** — state lives in an access-controlled service
     and automation reads it with a read-only credential; gain: one source of
     state and a CI drift report; cost: a service to operate or pay for, and
     another credential that can read the state.
  2. **Local state with an encrypted copy exported for automation** — gain: no
     backend service; cost: the exported snapshot is stale by definition, so
     drift since the export is invisible, and the export is easy to forget; in
     practice: a setting changed after the last export is reported as no drift.
  3. **Local state only; the drift check runs locally** — gain: no service and
     no extra credential; cost: the configuration can diverge whenever nobody
     looks, which is the problem this topic exists to solve.
- Disposition: **answered** — the user decided on 2026-10-17 that the applied
  state is local, encrypted at rest with a passphrase-derived key, and
  committed to the repository (the earlier decision to keep it out of Git is
  revoked). The passphrase, at least 16 characters, lives only in the
  maintainer's password manager and the environment of the applying shell;
  there is no default and no repository secret. There is no locking: one
  maintainer applies at a time, recorded rather than enforced. The drift check
  stays local, so the Frame's CI drift report is withdrawn. Rejected: options 1
  and 2 — the user established that GitHub offers no managed state service and
  that a stale exported snapshot reports drift that is not there.

### D2 — Which declared values are intended? (seed Q5)

- Question: does the declaration adopt the supplied draft's repository settings,
  preserve today's values exactly, or decide field by field?
- Options:
  1. **Adopt the draft's values** — gain: one intended state; cost: changes
     unrelated to the infrastructure goal appear in the first plan; in practice:
     projects turn off at the first configuration change.
  2. **Preserve today's values exactly** — gain: the first plan is empty for
     those fields; cost: settings nobody chose are ratified by code; in
     practice: projects stay on because the import copied them.
  3. **Decide field by field** — gain: no field enters by inertia; cost: a
     longer first pass.
- Disposition: **answered** — the user decided on 2026-10-17, field by field,
  to adopt the draft's values where they still describe today's project:
  `has_projects = false`; merge commits and rebase merges off, squash merges
  on; `delete_branch_on_merge = true`; today's description ("Convert a duty
  plan to a calendar") and today's topics kept; `has_wiki = false`,
  `has_discussions = false`, `allow_auto_merge = false`, `allow_forking = true`;
  no licence template on the imported repository; Dependabot alerts and
  security updates through their dedicated resources. The first configuration
  change therefore changes `has_projects`, `allow_merge_commit`,
  `allow_rebase_merge` and `delete_branch_on_merge`, and its preview names them.
  The organization's profile and member-default fields keep today's values,
  because the draft's profile wording describes a browser interface the project
  does not have yet; no draft profile text is adopted. Rejected:
  options 1 and 2 as whole policies — ratifying today's hand-made values and
  changing everything the draft proposed are both unrecorded choices.

### D3 — Do the legacy protection and the rule set both become declarations? (seed Q9)

- Question: `main` carries a repository ruleset and a classic branch protection
  with the same rules; should both be declared, consolidated into the rule set,
  or should only the rule set be declared while the legacy object stays
  unmanaged?
- Options:
  1. **Adopt both** — gain: no live change; cost: the duplication persists and
     every rule change must be mirrored or consciously left different.
  2. **Consolidate into the rule set; retire the legacy protection** — gain: one
     object, one place to review; cost: the first configuration change alters
     live protection.
  3. **Declare the rule set only; leave the legacy protection unmanaged** —
     gain: smallest declaration; cost: a manual change to the legacy protection
     is invisible to the drift check, which is the problem this topic exists to
     solve.
- Disposition: **answered** — the user chose option 2 on 2026-10-17 and wants
  linear history retained. The declaration carries the existing rule set with
  deletion, non-fast-forward and linear-history rules, no bypass actor (the live
  rule set already binds the owner). The legacy protection is not declared and
  is not migrated: its weaker admin exemption is deliberately dropped. Removing
  it is a live change, so it belongs to the first configuration change
  (`enable-repository-security`), not to adoption; the declaration simply omits
  it. Rejected: option 1 (keeps the weaker duplicate) and option 3 (leaves an
  unmanaged protection path).

### D4 — May the pages repository's protection rule bypass the publish deploy key? (seed Q2)

- Question: should the pages repository's protection rule grant the publishing
  deploy key a bypass, restrict itself to rules that cannot block a push, or be
  omitted?
- Options:
  1. **Bypass the deploy key** — gain: protected against human damage while
     publishing continues; cost: the key's own pushes are unreviewed.
  2. **Protect only what cannot block a push** — gain: simpler, no bypass; cost:
     nothing forces review of a human change.
  3. **No protection rule** — gain: nothing to configure; cost: a bad publish or
     a mistake destroys the only copy of the site.
- Disposition: **carried** — owner: the user; decided by
  `publish-site-plumbing`, which creates the key and therefore knows its
  identity. This change declares no pages-repository protection rule: none
  exists live today, so the adoption batch is complete without it.

### D5 — Does the default branch's protection rule require pull requests? (seed Q3)

- Question: should the rule set require pull requests for every change, allow
  the owner to bypass them, or require only the protection rules?
- Options:
  1. **Require pull requests, no bypass** — gain: one path for every change;
     cost: the maintainer waits for a pull request on a one-line fix.
  2. **Require pull requests, owner bypass** — gain: keeps the current routine;
     cost: two landing paths, and the required checks are advisory for the
     owner.
  3. **Protection only, no pull-request rule** — gain: matches today's practice;
     cost: no review at landing.
- Disposition: **carried** — owner: the user; decided by
  `enable-repository-security`, whose first configuration change is the one that
  may change live protection. This change declares the rule set exactly as it
  is today, which adds no pull-request rule.

### D6 — Does the reviewer's decision gate the apply? (seed Q10)

- Question: should an apply refuse an unreviewed or rejected revision, record
  the review but let the owner decide, or not model review in the apply flow?
- Options:
  1. **Gating** — gain: a second reader for every applied change; cost: a solo
     maintainer blocks, and the opening state has nobody else.
  2. **Advisory** — gain: no blocked workflow; cost: the reviewer cannot stop a
     change.
  3. **No review step** — gain: simplest flow; cost: no review contract.
- Disposition: **carried** — owner: the user; decided by the apply seeds
  (`enable-repository-security` first), because this change records a binding
  and applies nothing, so no decision about gating arises here.

### D7 — What may a configuration change delete? (seed Q11)

- Question: which declared objects may an apply delete, and which must it
  refuse to delete?
- Options:
  1. **Never delete the organization, a repository or a published release;
     allow lesser objects** — gain: the destructive class is bounded; cost: a
     repository is removed by hand.
  2. **Delete any declared object, named by the preview** — gain: one rule;
     cost: a mistaken edit can destroy the public home.
  3. **Never delete; unmanaging is the only effect** — gain: non-destructive by
     construction; cost: cleanup is manual.
- Disposition: **carried** — owner: the user; decided by the apply seeds. This
  change deletes nothing: adoption creates no object and removes none.

### D8 — Is team access part of the topic? (seed Q6)

- Question: should the declaration include teams and their repository roles, or
  leave access to a later decision?
- Options:
  1. **Leave access out** — gain: a smaller scope; cost: the one
     security-relevant setting stays outside the declaration.
  2. **Include a maintainer team** — gain: access is reviewed and reproducible;
     cost: the declaration can grant and revoke write access.
- Disposition: **out of scope** — the Frame keeps team access and organization
  membership outside the public-home topic, and the organization has one member,
  its owner. No declared object in this change is a team or a membership, so
  nothing enters or leaves the specification by inheritance.
