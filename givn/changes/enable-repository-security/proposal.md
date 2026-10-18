# Proposal: enable-repository-security

## Release note

Secret scanning and push protection are enabled on the public repository and the
Dependabot security settings are brought under the reviewed configuration, so a
credential can no longer enter the public history unnoticed.

## Use-Case Context

- Use-case ID: `apply-configuration-change` — "Apply a configuration change".
  New: the permanent corpus holds `adopt-existing-configuration`,
  `export-duty-plan-calendar` and `publish-public-release`; none owns applying a
  configuration change. The delta creates the permanent leaf
  `givn/specs/apply-configuration-change/` and carries its `usecase.md` under
  `specs/apply-configuration-change/`. This is the stable use case the topic's
  Handoff maps to its own leaf.
- Ideation topic: `public-home-infrastructure`.
- Seed path: `givn/ideation/public-home-infrastructure/changes/enable-repository-security.md`.
  Its status line already reads `Status: proposed by enable-repository-security`;
  the seed's original text is unchanged.
- Confirmed Personas:
  - `project-maintainer--organization-owner` — the administration review lens;
    holds the authority, the token and the passphrase, previews and applies the
    revision, and answers for the outcome.
  - `contributor--infrastructure-reviewer` — the review-without-authority lens;
    reads the declaration and the preview, and holds no token or passphrase.
  Persona IDs are recorded for review only; neither is a Gherkin Actor.

The use case this change realizes is the ideation contract
[Apply a configuration change](../../ideation/public-home-infrastructure/use-cases/apply-configuration-change.md)
in the topic [Public-home infrastructure](../../ideation/public-home-infrastructure/frame.md).
Its goal, guarantees and rules are preserved in the delta's use-case document;
this is the security change that follows the earlier configuration change,
applied through the declaration adopted by `declare-public-home`. The earlier
apply (`c87a34d`) reached live and changed the predecessor's projects, merge
and delete-branch fields; the authenticated post-apply read of 2026-10-18
under `evidence/live-read/` is the base this change starts from.

**Frame refinement recorded here.** The topic's Frame puts "the security
features the provider exposes" inside the declared configuration. This change
realizes that for the monorepo: secret scanning, push protection, Dependabot
alerts and Dependabot security updates become declared values, and the two
scanning settings are changed live. Private vulnerability reporting and release
immutability remain outside every declaration because the configuration
mechanism cannot express them; they stay manual and are recorded as gaps
(D6). The confirmed Frame itself is not rewritten.

## Problem / Opportunity

The public repository at `duty2cal/duty2cal` is where the project's history, its
contributor route and its release channel live, and every writer can push to it.
Two settings that protect that history are off today: an authenticated read on
2026-10-18 reports `secret_scanning` disabled and
`secret_scanning_push_protection` disabled. Nothing stops a credential from
being committed, and nothing reports it afterwards if it is.

The rest of the security surface is no better recorded. Dependabot alerts and
Dependabot security updates are on, but no declaration names them, so a change
to either is invisible to the reviewed configuration and to the local drift
check — exactly the failure the public-home topic exists to prevent. Private
vulnerability reporting is off and cannot be declared at all.

The opportunity is the security change after the earlier configuration change:
it works against the state that apply left, closes the two open settings,
brings the already-on Dependabot settings under the declaration, and proves the
preview-and-apply flow end to end: the difference is named before anything
changes, a refusal changes nothing, and a partial failure is reported with the
object that failed and a stale applied state.

## Proposed Solution

- **The declaration names the monorepo's security settings.** Secret scanning
  and push protection are declared as enabled; Dependabot alerts and Dependabot
  security updates are declared as enabled. The pages repository's scanning is
  not declared, because the seed's guarantee names the monorepo.
- **The preview names only what changes.** With Dependabot alerts and security
  updates already on and bound to their declared counterparts, a preview of the
  revision names exactly the two scanning settings as changes and proposes no
  object to create or delete. The preview reads; nothing live changes.
- **The apply enables the two settings.** The previewed revision is applied as
  it was previewed, the repository reports secret scanning and push protection
  enabled, the applied state records the values that were applied, and a later
  preview reports no differences. No other declared object changes.
- **A refusal changes nothing.** When the authority cannot read the repository's
  settings, the preview fails and names the repository. When it cannot change
  them, the apply fails and names the repository; the live configuration and the
  applied state are unchanged, and a later preview names the same change again.
- **A partial failure is stated honestly.** When an earlier setting is applied
  and a later one fails, the outcome reports the settings that were applied, and
  names the setting that failed; the applied state never claims the failed
  setting and is marked stale beside itself, so the next preview refuses until a
  green apply records the whole revision.
- **The already-on settings become drift-managed.** Because Dependabot alerts
  and security updates are declared and bound, a later manual change to either
  is a difference the local drift check can report; before this change nothing
  named them.

## Capability Routing

`givn spec route --change enable-repository-security` reported
`EXTEND export-duty-plan-calendar` (score 13, confidence: signal). That
recommendation is not followed: its matches fall on the word `route` and on
generic goal and interaction words shared with the conversion use case, which
converts a duty plan into a calendar and shares no object, rule or outcome with
a repository security setting.

| Proposed capability | Route's recommendation | Decision | Rationale (if we deviated from route) |
|---|---|---|---|
| `enable-repository-security` | `EXTEND export-duty-plan-calendar` | `NEW in apply-configuration-change` | The goal — preview a revision of the declared configuration that enables the repository's security settings, apply it, and report the outcome — belongs to the organization owner's administration goal, not to the crew member's conversion goal. The route's signal is lexical (`route` and generic words) and names no shared object. The owning use case is the ideation contract for exactly this goal, mapped by the Handoff to `givn/specs/apply-configuration-change/`. |

## Out of Scope

- **Requiring pull requests on the default branch (seed Q3).** The protection
  ruleset is not modified; the security settings do not depend on the landing
  routine.
- **A runtime review gate (seed Q10).** This change is reviewed through the givn
  review, and the operator applies the declaration after GREEN. No approval
  record or runtime gate is introduced.
- **Deleting anything (seed Q11).** This change removes no object and disables
  no setting. The duplicate legacy branch protection was already removed before
  this change, and the declared ruleset is untouched.
- **Secret-scanning non-provider patterns and validity checks.** The seed names
  secret scanning and push protection only; these two stay as they are (D2).
- **Security settings on the pages repository.** The seed's guarantee names the
  monorepo; the pages repository is written only by the future publish path,
  and adding its scanning would extend the guarantee without a decision.
- **Advanced security on the public repository.** Advanced security is reported
  enabled for a public repository and cannot be supplied; declaring it would be
  a false claim (D2).
- **Private vulnerability reporting.** The configuration mechanism cannot
  express it; it stays manual and is recorded as a gap (D6).
- **Release immutability, artifact attestations and organization-level security
  defaults.** Settings or pipeline outcomes the configuration cannot declare, or
  that belong to the release pipeline.
- **Configuration drift reporting.** The local drift check already exists as a
  contract; running it on a schedule is `report-configuration-drift`.
- **Any product behaviour, release workflow or conversion behaviour.** The
  converter never contacts GitHub.

## Open Questions

Every question the seed leaves open is dispositioned here before the specs are
written. `D1`, `D2` and `D6` are decisions this proposal takes; `D3`–`D5` are
the intersecting seed questions, recorded so nothing enters the specification by
inheritance.

### D1 — Are the Dependabot settings declared and bound, or left unmanaged?

- Question: Dependabot alerts and Dependabot security updates are already on.
  Should the declaration name them and bind them to what exists, name them
  without binding them, or leave them outside the declaration?
- Options:
  1. **Declare both and bind them to the live settings** — the declaration names
     `enabled` for each, and each is bound to the repository it already belongs
     to, so neither appears as a change in the preview.
     Gain: both settings become drift-managed and the preview names no
     difference that is not real; the applied state is a truthful record.
     Cost: two more bound settings, and an operator step before the apply: the
     settings must be adopted first, exactly as the repositories were.
     In practice: the preview reports the two scanning settings as the only
     change; a manual disabling of Dependabot security updates later is a
     difference the next preview can name.
  2. **Declare both without binding them** — the declaration names them, and the
     first apply enables settings that are already enabled.
     Gain: no extra adoption step, and the apply carries several changes, so a
     partial failure is reachable without further setup.
     Cost: the preview claims two creations of settings that already exist,
     which contradicts the adoption rule that an existing object is never
     recreated; the operator reads a difference that is not there.
     In practice: the preview says two settings are to be created while both are
     already enabled.
  3. **Leave both unmanaged** — the declaration names neither.
     Gain: the smallest change and no adoption step.
     Cost: Dependabot alerts and security updates stay outside the reviewed
     configuration, so a manual change to either is invisible — the problem this
     topic exists to solve — and the seed's guarantee that the settings are
     "declared" is not met.
     In practice: someone turns Dependabot security updates off by hand and no
     preview ever names it.
- Disposition: **answered** — option 1, decided by this proposal on 2026-10-18.
  Both settings are declared with `enabled = true` and bound by the existing
  adoption action through a second batch, which is data rather than a new
  command; the apply then carries exactly one live change. Rejected: option 2,
  because it makes the preview claim a creation of something that exists and
  contradicts the rule that an existing object is adopted before it is updated;
  and option 3, because it leaves the settings outside the declaration and fails
  the seed's guarantee. Realized in `design.md` ("The declaration", "The
  adoption batch") and in the delta's scenarios.

### D2 — Are non-provider patterns, validity checks and advanced security in this change?

- Question: the repository's scanning surface also exposes non-provider patterns,
  validity checks and advanced security. Does this change enable any of them?
- Options:
  1. **Enable them with secret scanning** — one revision turns on every scanning
     feature at once.
     Gain: the widest protection in one change.
     Cost: it extends the seed's guarantee beyond what the seed names, and
     advanced security cannot be supplied for a public repository, so the
     declaration would contain a setting that cannot be honoured.
     In practice: the preview would carry differences nobody decided.
  2. **Leave them unchanged** — only secret scanning and push protection change.
     Gain: the declaration states exactly what the seed guarantees; no setting
     changes without a decision.
     Cost: non-provider patterns and validity checks stay off, and a later
     change must add them if wanted.
     In practice: the preview names the two settings only, and the others stay
     as they are.
- Disposition: **answered** — option 2, decided by this proposal on 2026-10-18,
  because the seed names only secret scanning and push protection and advanced
  security cannot be supplied on a public repository. Rejected: option 1 for the
  reason above. The pinned provider 6.13.0 cannot declare the two remaining
  checks either: its update path sends only advanced security, secret scanning
  and push protection, and `secret_scanning_validity_checks` is not in its
  schema. A later change that wants them needs a provider upgrade or another
  mechanism, plus its own preview and decision; a declaration edit alone
  cannot reach them.

### D3 — Does the default branch's protection rule require pull requests? (seed Q3)

- Question: should the rule set require pull requests for every change, allow
  the owner to bypass them, or require only the protection rules without a
  pull-request rule?
- Options:
  1. **Require pull requests, no bypass** — gain: one path for every change;
     cost: the maintainer waits for a pull request on a one-line fix.
  2. **Require pull requests, owner bypass** — gain: keeps the current routine;
     cost: the required checks are advisory for the owner.
  3. **Protection only, no pull-request rule** — gain: matches today's practice;
     cost: no review step at landing.
- Disposition: **out of scope** — this change modifies no rule set and the
  security settings do not depend on the landing routine. The question stays
  open in the topic log (`questions.md`, Q3) and is decided by a change that
  revises the ruleset.

### D4 — Does a review gate the apply? (seed Q10)

- Question: should the apply refuse a revision the reviewer has not approved,
  record the review but let the owner apply anyway, or not model review in the
  apply flow at all?
- Options:
  1. **Gating: no approval, no apply** — gain: the reviewer's authority is real;
     cost: a solo maintainer blocks on a second person.
  2. **Advisory: review is recorded, the owner decides** — gain: no blocked solo
     workflow; cost: the reviewer cannot stop a change.
  3. **No review step in the apply flow** — gain: simplest flow; cost: an
     unreviewed change can be applied.
- Disposition: **out of scope** — this change is reviewed through the givn
  review and its operator applies the declaration after GREEN; no approval
  record, gate or bypass is built. The question stays open in the topic log
  (Q10) and is decided by a change that models review in the apply flow.

### D5 — What may an apply delete? (seed Q11)

- Question: which declared objects may an apply delete, and which must it
  refuse to delete?
- Options:
  1. **Never delete a repository, the organization or a published release;
     allow lesser objects** — gain: the destructive class is bounded; cost: a
     repository can only be removed by hand.
  2. **Delete any declared object, with the preview naming every deletion** —
     gain: one rule; cost: a mistaken edit can destroy the public home.
  3. **Never delete; removal only stops managing the object** — gain: the apply
     is non-destructive; cost: cleanup is manual.
- Disposition: **out of scope** — this change deletes nothing, disables nothing
  and removes no declaration. The duplicate legacy protection whose removal the
  predecessor deferred is already gone (authenticated read 2026-10-18). The
  question stays open in the topic log (Q11) and is decided before a change
  removes an object from the declaration.

### D6 — How is private vulnerability reporting handled?

- Question: private vulnerability reporting is off, and the configuration
  mechanism cannot express it. Should this change adopt another mechanism for it,
  or keep it manual and record the gap?
- Options:
  1. **Keep it manual and record the gap** — the contributor route in
     `SECURITY.md` stays the reporting path, and the fact that the setting is
     undeclarable is recorded.
     Gain: no unmanaged automation and no new credential; cost: a manual click
     can change the setting and no preview names it.
     In practice: a reviewer reads the recorded gap and checks the setting by
     hand.
  2. **Add a separate mechanism for it** — a script or an extra tool calls the
     API.
     Gain: the setting is covered; cost: a second write path with its own
     credential, outside the declaration and the reviewed preview.
     In practice: two mechanisms now write security settings.
- Disposition: **answered** — option 1, decided by this proposal on 2026-10-18,
  because the seed's guarantee does not name private vulnerability reporting and
  a second write path would contradict the one-declaration rule. Rejected:
  option 2. The gap is recorded in the design and in the arc42 risk chapter.
