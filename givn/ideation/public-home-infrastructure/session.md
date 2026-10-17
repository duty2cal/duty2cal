# Public-home infrastructure — session state

- Topic: `public-home-infrastructure`.
- Topic identity: selected by the agent on 2026-10-17 from the user's draft `private/opentofu.md`; no user naming decision has been taken. Rename on request before any Handoff.
- Phase: Handoff complete — published 2026-10-17 with the user's confirmation (Q12). Topic state, the readable document, four seeds, two Persona promotions and six glossary terms are published.
- Next separate workflow: the user invokes normal proposal planning per seed, starting with `declare-public-home`. This completed ideation authorizes no change start, no `infra/` file, no provider state, and no GitHub write.
- Tooling now available: `gh` authenticated as `kressnerd` (`repo`, `read:org`) and `tofu` 1.13.1. All live reads below are authenticated unless a scope is named.
- Completed authority: topic-local ideation state only. Frame, Personas and terms are confirmed; no change artifacts, no `infra/` files, no GitHub writes, no provider state, no Handoff.

## Confirmed inputs

- Frame confirmed by the user on 2026-10-17 through Q1, option 1: the organization, both repositories, the `main` ruleset, the security features, the Pages configuration, the deploy key and its secret, the import of today's state, and the drift report are in scope; organization creation, repository transfer, release immutability, attestations, private vulnerability reporting, the website's content, membership, and product code are out.
- The publishing plumbing is included even though no site source or build workflow exists yet.

## Supplied direction (not yet confirmed as Frame)

`private/opentofu.md` is the user's first draft. It stays unedited user work.
Its directions, to be treated as supplied unless the user revises them:

- OpenTofu with the `integrations/github` provider.
- `infra/` lives in the monorepo, MPL-2.0, with local state kept out of Git.
- `tofu apply` runs locally from the maintainer's workstation with an owner PAT; never in GitHub Actions.
- Bootstrap order: create the organization manually, transfer the repository manually, import the repository, then apply.
- A read-only drift report in CI (`tofu plan -detailed-exitcode`).
- Dependabot gains a `terraform` ecosystem entry for `infra/`.
- `safe-settings` and raw `gh api` scripts are rejected as the primary mechanism.

The draft's HCL is a hypothesis, not a verified configuration. See
`frame.md` for the claims checked against the live repository and the
provider.

## Research completed

- `givn status`: no active changes; nine archived changes.
- `givn spec tree`: two permanent use cases — export a duty-plan calendar
  (`export-duty-plan-calendar`) and publish a public release
  (`publish-public-release`). Neither owns organization or repository
  configuration.
- Read `README.md` and arc42 chapters 1, 2, 3, 7, 9, 11 and 12. Chapter 2
  owns the public repository address, immutable releases, contributor routes
  and no-private-material constraint; chapter 3 names GitHub as the project's
  one external system; chapter 7 owns the deployment view; chapter 9 owns
  ADR-0003 and ADR-0004; chapter 12 holds the confirmed **Public home**,
  **Default branch**, **Gate (on landing)**, **Immutable release** and
  **Drift check** terms.
- Read the previous topic's frame, question log, use-case definitions and
  Personas as format and reuse material.
- Read `private/opentofu.md` and `private/website-idea.md` as user notes; both
  remain unedited and untracked.
- Live GitHub state read on 2026-10-17 with authenticated `gh`: the
  organization is on the free plan, its profile fields are empty, default
  repository permission is `read`, members may create public and private
  repositories, and the only member is `kressnerd`; the monorepo has projects
  on, wiki and discussions off, all three merge methods allowed,
  delete-branch-on-merge off, secret scanning and push protection off,
  Dependabot security updates on; a repository ruleset `main protection`
  (deletion, non-fast-forward, linear history) exists alongside classic branch
  protection with the same rules; the pages repository has issues, projects
  and wiki on, Pages built from `main` at `/`, no deploy keys; the monorepo has
  no repository secrets; private vulnerability reporting is off; Actions allows
  all actions with no SHA-pinning requirement; the org Actions policy needs
  `admin:org` and was not read.
- Attestation loss verified independently: authenticated lookup for the v0.3.0
  Windows archive digest returns 404 and `gh attestation verify` fails.
- Provider research against `integrations/github` 6.13.0 on both the Terraform
  and OpenTofu registries: resource coverage and the four gaps recorded in
  `frame.md`.
- Unauthenticated GitHub API quota was exhausted during research; org profile,
  branch protection, Dependabot and Actions settings still need an
  authenticated read in a later phase.

## Persona reuse inventory

Inventory performed 2026-10-17 against the project collection
(`crew-member--personal-calendar-owner`, `project-maintainer--release-owner`)
and the one sibling topic's Personas.

| Stakeholder type | Existing matches | Disposition |
|---|---|---|
| Maintainer / organization owner | `project-maintainer--release-owner` | New segment `project-maintainer--organization-owner`: differs in authority boundary (applies configuration, holds the token and state) and evaluation capability (preview and configuration drift, not release evidence). |
| Contributor / reviewer | none | New segment `contributor--infrastructure-reviewer`: review-only authority, no token or state. |
| Crew member downloading a release | `crew-member--personal-calendar-owner` | Reused; topic-local file holds record entries only. |
| Future site visitor | `crew-member--personal-calendar-owner` | No new segment: same authority, evaluation and vocabulary; the site's content is a separate topic. |

All three Persona files were confirmed by the user on 2026-10-17 (Q7) and are advisory review lenses for this topic.

## Decision history

- Q4, the state-access decision, is closed on 2026-10-17 by the user: local backend, state encrypted with OpenTofu's `encryption` block (PBKDF2 to AES-GCM, `enforced = true` for state and plan) and committed to `infra/`; passphrase local only; no locking under a single-maintainer assumption; the CI configuration drift report is withdrawn, so the drift check stays local. This supersedes the Frame's scheduled/per-push report; the published topic document remains historical.

- Q7, the Persona decision (three review lenses), is closed on 2026-10-17 with the user's answer "confirmed." Its full question and rejected alternative remain in `questions.md`.

- Q1, the Frame decision (problem, vision, scope, stakeholders, and inclusion of the publishing plumbing), is closed on 2026-10-17 with the user's answer "1". Its full question and rejected alternatives remain in `questions.md`.

## Handoff state

- Published 2026-10-17 after Q12: `docs/ideation/public-home-infrastructure.md`; four seed files under `changes/`; `project-maintainer--organization-owner` and `contributor--infrastructure-reviewer` promoted with merged diffs; six terms added to `docs/arc42/12-glossary.md`.
- The crew-member topic file remains historical; no promotion was needed.
- No change started. The seeds are input to `/givn-propose`, not proposals.

## Phase reports

- Frame: confirmed 2026-10-17 (Q1); boundary review run and applied.
- Event Storming: drafted, reviewed (six blocking findings applied), canvases rendered (three diagrams).
- Use Case Definition: drafted, reviewed (six blocking findings applied), cut fixed at three leaves.
- Use Case Elaboration: three contracts drafted 2026-10-17; boundary review pending. New open questions: Q10 (review gating), Q11 (deletion).
