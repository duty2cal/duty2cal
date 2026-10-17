# Public-home infrastructure — Frame

Status: **confirmed by the user on 2026-10-17 (Q1, option 1)**, including the
site-publishing plumbing. Research observations stay findings, not
requirements; supplied direction from the draft remains supplied.

## Problem

The organization and its repositories are configured by hand in the GitHub
web interface. The chapters state what the public home should be, but the
intended configuration exists nowhere as a declaration: it cannot be reviewed,
diffed, previewed, or rebuilt from the history, and a change such as the
2026-10-17 repository transfer happens as a sequence of clicks whose
consequences are discovered afterwards — the transfer silently dropped every
published artifact attestation, and the public text promising them had to be
repaired in a follow-up revision.

## Vision

The maintainer changes the project's GitHub presence by changing `infra/` in
the monorepo: the organization's profile and member defaults, both
repositories, the protection rulesets for the monorepo's `main` and for the
pages repository, the security features, and the Pages configuration with the
key and secret that publish to it. A preview shows the difference before
anything changes; the change is reviewed like any other revision; the
maintainer applies it from one workstation. A scheduled and per-push
configuration drift report states whether the live organization still matches
what the repository declares.

Example: enabling a newly added CI job as a required check becomes one line in
`infra/rulesets.tf`, a preview that names the check, and an apply — not a walk
through two settings pages whose result nobody can diff.

## Proposed scope

Inside:

- The organization `duty2cal`: the profile and member-default settings the
  provider exposes, including default repository permission and who may create
  repositories.
- Both repositories: `duty2cal/duty2cal` and `duty2cal/duty2cal.github.io`,
  including description, topics, features and merge strategies.
- The protection rulesets: the monorepo's `main`, and the pages repository,
  whose only writer is the publish deploy key.
- The security features the provider exposes: secret scanning, push
  protection, Dependabot alerts and automated security updates.
- The pages repository's Pages configuration and the write-enabled deploy key
  plus the monorepo secret through which automation publishes the site.
- Adopting today's hand-made state into the declaration — both repositories,
  the organization settings already set, and the Pages configuration already
  serving the stock site — and the one-time sequence that rebuilds what GitHub
  allows to be rebuilt.
- The configuration drift report over the declared configuration.

Outside:

- Creating the GitHub organization and transferring a repository into it:
  GitHub exposes no creation endpoint on github.com and no transfer resource;
  both are one-time manual actions, already performed.
- Release immutability: a repository setting, not a pipeline outcome. The
  provider cannot declare it, so it stays manual and is reported as a gap.
- Artifact attestations and every other release-time fact: pipeline outcomes,
  never configuration.
- Private vulnerability reporting and any other setting the provider cannot
  declare: manual, reported as gaps.
- The website's content, build and styling, the WASM UI, and the browser
  converter: this topic only supplies the plumbing that publishes a site. The
  pages repository keeps its stock Jekyll site until that separate topic
  changes it.
- Team access and organization membership: currently the owner is the only
  member, so no member-default stakeholder exists beyond that owner. Who joins
  later is a separate authority decision.
- Any change to the product executable, its release workflow, or the
  conversion specification.

## Stakeholders

These are stakeholder types, not approved Persona biographies or Gherkin
Actors. Personas are proposed only after the Frame is confirmed.

| Stakeholder type | Interest | Concrete example |
|---|---|---|
| Maintainer / organization owner | Configuration that can be reviewed, diffed and rebuilt; no click-through memory as the only record of how the public home is set up. | A ruleset change appears as a reviewed line in `infra/rulesets.tf`, not as an unrecorded settings edit. |
| Contributor / reviewer | Can read the intended configuration and see an infrastructure change in the same history as the code it supports; never holds the administration token or the state file. | A pull request that adds a required check shows the ruleset revision beside the workflow revision. |
| Crew member downloading a release | The public home keeps serving the release channel, and a downloaded archive can still be traced to its tagged revision and the automation that built it, while its configuration changes. | A ruleset apply does not lock `main` in a way that blocks the release tag, and the archive's provenance still verifies after the change. |
| Future site visitor | The published site is produced from the monorepo by the declared automation, not by hand; the deployment cannot publish a site nobody built. The site's content belongs to a separate topic. | A failed build leaves the previous site in place instead of publishing an empty page. |

## Supplied direction from the draft

`private/opentofu.md` supplies the following; each is treated as a direction
unless the user revises it:

- OpenTofu with `integrations/github`; `infra/` in the monorepo, MPL-2.0.
- Local state, kept out of Git; a local apply from Linux with an owner PAT.
- No apply in GitHub Actions: the token that administers the repository must
  not be stored in the repository it administers.
- One-time preconditions and import sequence: organization (manual), transfer
  (manual), import of the existing objects, then apply.
- A read-only configuration drift report in CI. It must be able to read the
  applied state, so local-only state is insufficient for it; an encrypted
  remote backend or an exported state is required. This is the open
  state-backend question (Q4).
- Dependabot `terraform` ecosystem for `infra/`.
- Rejected: `safe-settings` (a deployable GitHub App, no Pages management) and
  raw API scripts (no preview or configuration-drift detection) as the primary
  mechanism.

## Research protocol and current evidence

1. Read the project's state before asking what it already answers:
   `givn status` shows no active change, `givn spec tree` two permanent use
   cases, neither owning organization configuration.
2. Read the durable context: `README.md`; arc42 chapters 1 (goals), 2
   (constraints), 3 (context), 7 (deployment), 9 (ADR-0003 and ADR-0004), 11
   (risks) and 12 (glossary).
   Chapter 2 owns the constraints the declaration must preserve: the public
   repository at `duty2cal/duty2cal`, immutable releases and never-reused
   version tags, the DCO contribution terms, the published contributor routes,
   and no private material in public assets. The glossary confirms **Public
   home**, **Default branch**, **Gate (on landing)**, **Immutable release** and
   **Drift check**; the last one names the change-log comparison and collides
   with any new "drift" term — see `domain-terms.md`.
3. Read the user's draft `private/opentofu.md` and `private/website-idea.md`
   without editing them; the draft is supplied direction, the HCL is
   hypothesis.
4. Inventory the live state on 2026-10-17 without writing anything, first
   unauthenticated and then with the machine's authenticated `gh` (`repo`,
   `read:org`): the organization exists on the free plan with empty profile
   fields, default repository permission `read`, members allowed to create
   public and private repositories, and one member; the monorepo is public and
   serves releases; `duty2cal.github.io` exists with one initial commit, the
   stock Jekyll site built from `main` at `/`, and its default features on; a
   repository ruleset `main protection` exists alongside classic branch
   protection; secret scanning and push protection are off; Dependabot
   security updates are on; private vulnerability reporting is off; no
   repository secrets or deploy keys exist. Organization Actions policy could
   not be read (needs `admin:org`).
5. Verify provider capability against `integrations/github` 6.13.0 on the
   Terraform and OpenTofu registries instead of trusting the draft's links.
6. Keep every finding below distinct from confirmed direction; the user
   arbitrates.

## Findings — draft claims checked against live state and provider

| Claim in the draft | Finding |
|---|---|
| The repository is still to be transferred. | Already done on 2026-10-17; the bootstrap step is historical, not a pending action. |
| `duty2cal.github.io` is to be created. | It already exists (one initial commit, stock content). It must be imported, not created; creating it would fail or, worse, replace it. |
| Pages is to be enabled. | Already enabled: the stock Jekyll site is served. Import `github_repository_pages` with the existing source (legacy, `main`, `/`) rather than enabling it anew. |
| Organization settings are to be created fresh. | The organization exists with hand-made settings; they must be adopted by import, not recreated. |
| Pages is configured through the `pages` block in `github_repository`. | That block is deprecated in provider 6.13.0; the documented form is `github_repository_pages` with `lifecycle { ignore_changes = [pages] }` on `github_repository`. |
| The required check context is `ci`. | No check run is named `ci`; the gate reports fifteen job-named checks (`fmt`, `test`, `clippy`, `reuse`, …). A ruleset requiring `ci` would require a check that never appears. |
| The organization can be created from code. | No github.com endpoint; manual, as the draft's own table says. |
| Immutable releases are configurable from code. | Not in provider 6.13.0; a repository setting that stays manual. |
| Private vulnerability reporting is configurable from code. | No resource in 6.13.0; manual. |
| A deploy private key can live in the state file safely. | The state holds it in plaintext; an encrypted backend or an encrypted, restorable backup is required. The same state access is what the CI configuration drift report needs (Q4). |
| The monorepo already has the site source. | It does not: no `web/` and no `ffi/wasm/` exist; only `apps/cli` is present. The publishing workflow is future work, so the deploy key and secret have no producer yet. |
| The stock Jekyll site must be replaced now. | It stays until the website topic; the site's content is outside this Frame. |
| The pages repository needs the same protection as `main`. | It needs its own ruleset, but its only writer is the publish deploy key; a ruleset that does not bypass that key would block publishing (Q2). |
| No protection exists on `main`. | A repository ruleset `main protection` (deletion, non-fast-forward, linear history) and classic branch protection with the same rules both exist. Whether both enter the declaration or one is retired is Q9. |
| Secret scanning and push protection are enabled. | Both are off on the public repository; enabling them is part of the declaration. Dependabot alerts and security updates are on. |
| Private vulnerability reporting is enabled. | Off, and the provider cannot declare it; it stays manual. |
| Attestations survived the transfer. | Confirmed absent: an authenticated lookup returns 404 and `gh attestation verify` on the v0.3.0 Windows archive fails, so the README repair stands. |
| Organization profile fields are set. | Name, description, blog and email are empty; the draft's description and blog would be new values. |

## Draft consequences to carry forward

- The transfer's attestation loss is a realized instance of the problem: a
  manual configuration change with an unrecorded, invisible consequence.
- The draft's HCL, if applied as written, would (a) fail on the already
  existing pages repository, (b) import the monorepo with settings that differ
  from today's (for example `has_projects` is currently true), and (c) create
  a ruleset requiring a check name that never reports.
- Reading the live settings requires an authenticated token; the Frame's later
  phases must record which settings were verified and which remained
  unreadable.

## Open questions

Not yet asked; recorded in `questions.md` as they become blocking:

- The pages repository's ruleset must bypass its only writer, the publish
  deploy key, or restrict itself to what cannot block a push (Q2).
- Whether the monorepo's ruleset requires pull requests or keeps an owner
  bypass, so direct pushes continue (Q3).
- Which state access makes both the local apply and the CI configuration drift
  report possible (Q4).
- Which current repository settings are intended values and which were copied
  defaults (Q5).
- Whether team access arrives with this topic or waits (Q6).
- Whether the classic branch protection and the ruleset both enter the
  declaration, or the classic protection is retired in favour of the ruleset
  (Q9).
