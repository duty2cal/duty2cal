# Public-home infrastructure

Handoff published with the user's confirmation on **2026-10-17**. This records
the approved domain contract and planning input, not an implemented or
validated configuration. No `infra/` file, provider state, GitHub setting, or
change exists because of this document.

## Planning records

- [Topic and resume state](../../givn/ideation/public-home-infrastructure/session.md).
- [Confirmed Frame](../../givn/ideation/public-home-infrastructure/frame.md).
- [Event Storming and rendered canvases](../../givn/ideation/public-home-infrastructure/eventstorming.md).
- [Use-case definition](../../givn/ideation/public-home-infrastructure/use-case-definitions.md).
- [Use cases: adoption](../../givn/ideation/public-home-infrastructure/use-cases/adopt-existing-configuration.md),
  [configuration change](../../givn/ideation/public-home-infrastructure/use-cases/apply-configuration-change.md),
  [configuration drift](../../givn/ideation/public-home-infrastructure/use-cases/check-configuration-drift.md).
- [Full decisions, alternatives, and open questions](../../givn/ideation/public-home-infrastructure/questions.md).
- [Confirmed Handoff, mapping, promotions, and seeds](../../givn/ideation/public-home-infrastructure/handoff.md).
- [Durable vocabulary](../arc42/12-glossary.md).

The normal proposal flow has **not** started. The four seeds are input to
`/givn-propose`; Handoff creates no proposal, change scaffold, permanent spec
leaf, tasks, code, or provider state.

## Problem and vision

The organization and its repositories are configured by hand in the GitHub web
interface. The chapters state what the public home should be, but the intended
configuration exists nowhere as a declaration: it cannot be reviewed, diffed,
previewed, or rebuilt from the history. A change such as the 2026-10-17
repository transfer happens as a sequence of clicks whose consequences are
discovered afterwards — the transfer silently dropped every published artifact
attestation, and the public text promising them had to be repaired in a
follow-up revision.

The vision: the maintainer changes the public home by changing `infra/` in the
monorepo. A preview names every difference; the change is reviewed like any
other revision; the maintainer applies it from one workstation; a configuration
drift report states whether the live organization still matches what the
repository declares.

## Scope

Inside:

- The organization `duty2cal`: profile fields, default repository permission,
  and members' repository-creation rights.
- Both repositories: description, topics, features, merge strategies,
  delete-branch-on-merge, and the security features the provider exposes.
- The protection rulesets: the monorepo's `main` and the pages repository.
- The Pages configuration on `duty2cal/duty2cal.github.io`, the write-enabled
  deploy key, and the monorepo `PAGES_DEPLOY_KEY` secret.
- Adopting today's hand-made state, and the configuration drift report.

Outside:

- Creating the organization and transferring a repository into it.
- Release immutability and artifact attestations (manual or pipeline outcomes).
- Private vulnerability reporting (no provider resource).
- The website's content, build, and WASM UI.
- Team access and organization membership.
- Product code, the release workflow, and the conversion contract.

## What was verified on 2026-10-17

Live reads used the machine's authenticated `gh` (`repo`, `read:org`); the
provider claims were checked against `integrations/github` 6.13.0.

| Fact | State |
|---|---|
| Organization | Free plan; profile fields empty; default repository permission `read`; members may create public and private repositories; one member |
| `duty2cal/duty2cal` | Projects on, wiki and discussions off; all three merge methods allowed; delete-branch-on-merge off; secret scanning and push protection off; Dependabot security updates on; no repository secrets |
| Protection | Ruleset `main protection` (deletion, non-fast-forward, linear history) and classic branch protection with the same rules |
| `duty2cal/duty2cal.github.io` | Issues, projects and wiki on; Pages built from `main` at `/`; no deploy keys |
| Release integrity | Releases immutable; attestations absent for the pre-transfer releases (authenticated lookup returns 404) |
| Provider gaps | No organization creation, repository transfer, immutable-release setting, or private-vulnerability-reporting resource |

## Vocabulary

Six terms were confirmed for this topic and promoted to the glossary:
**Declared configuration**, **Configuration drift**, **Protection ruleset**,
**Deploy key**, **Adoption**, and **Configuration change**. **Configuration
drift** always carries the qualifier; the glossary's **Drift check** is the
change-log comparison. "Applied state", "live configuration", "preview" and
"configuration drift report" are artifact labels, not domain terms.

## Review lenses

- [Organization owner](../../givn/personas/project-maintainer--organization-owner.md)
  (`project-maintainer--organization-owner`) — applies configuration, holds the
  token and state, judges a preview and a drift report.
- [Contributor reviewer](../../givn/personas/contributor--infrastructure-reviewer.md)
  (`contributor--infrastructure-reviewer`) — reviews without applying; holds no
  token or state.
- [Crew member](../../givn/personas/crew-member--personal-calendar-owner.md)
  (`crew-member--personal-calendar-owner`) — the release channel and the future
  site surface; no authority over publishing.

## The three contracts

### Adopt the existing public-home configuration (`adopt-existing-configuration`)

One-time changing goal: place the already-existing objects of the public home
under the declared configuration without changing them. A batch is adopted or
none is; the applied state binds an identity, not a copy of the values; any
difference between declared and live is left for the first preview. The deploy
key and secret do not exist and are never adopted.

### Apply a configuration change (`apply-configuration-change`)

Changing goal: make the live configuration match the declared configuration
revision the apply accepts. The flow records a revision, previews it, takes an
optional review, and applies only the previewed revision. A refusal before the
first write changes nothing; a failure after the first write is reported as
partial with a stale state. No repository, organization, or published release
is deleted.

### Check the configuration drift (`check-configuration-drift`)

Observing goal: obtain a report of every difference between the declared
configuration and the live configuration. The report names differences, marks
unreadable fields unchecked, and states the applied state's availability and
age. It never concludes "no configuration drift" on an unavailable state.

## Change seeds

| Seed | Guarantee | Depends on |
|---|---|---|
| `declare-public-home` | The declaration names the live home and the existing objects are bound in the applied state; live unchanged. | — |
| `enable-repository-security` | Secret scanning, push protection and Dependabot security updates are on through a previewed, reviewed apply. | `declare-public-home` |
| `publish-site-plumbing` | A write-enabled deploy key and the matching `PAGES_DEPLOY_KEY` secret exist; the stock site is untouched. | `declare-public-home` |
| `report-configuration-drift` | A report names every difference it can compare and never calls an unavailable state "no drift". | `declare-public-home`; CI needs Q4 |

## Decision history

- **Q1 — Frame.** Confirmed including the site-publishing plumbing. Rejected:
  excluding the deploy key and secret until the site exists.
- **Q2–Q6 — open.** Pages ruleset deploy-key bypass; monorepo pull requests;
  state access; intended setting values; team access.
- **Q7 — Personas.** Confirmed the three review lenses. Rejected: revising a
  named lens.
- **Q8 — Terms.** Confirmed all six, with the mandatory qualifier on
  configuration drift. Rejected: revisiting a named term.
- **Q9 — open.** Classic protection and ruleset both declared, consolidated, or
  one left unmanaged.
- **Q10 — open.** Review gating versus advisory.
- **Q11 — open.** What a configuration change may delete.
- **Q12 — Handoff.** Confirmed the mapping, terms, promotions and seeds
  published here.

## What this does not authorize

No `infra/` directory, no provider initialization, no `tofu plan` or `apply`
against the organization, no GitHub settings change, no Persona approval of a
release, and no change start. Each seed enters the normal proposal flow when
the user runs it.
