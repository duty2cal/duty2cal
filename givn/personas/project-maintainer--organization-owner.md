# Persona: project-maintainer--organization-owner

Name: Organization owner maintaining the public home's configuration

Status: confirmed by the user on 2026-10-17; promoted to the project collection by confirmed Handoff on 2026-10-17 (Q12).

Origin topic: Public-home infrastructure (`public-home-infrastructure`). Relative artifact references in the running record refer to `givn/ideation/public-home-infrastructure/`.

## Role

Owns the `duty2cal` organization and administers its repositories.

## Authority boundary

Decides and applies the declared configuration; holds the administration token and the applied state; may change organization and repository settings. Does not decide product behavior, release readiness, or the conversion contract.

## Evaluation capability

Judges a preview, a configuration diff, a protection ruleset's effect, and a configuration drift report; can tell whether the declared configuration matches the intended public home and whether a change is safe to apply. Cannot judge the converter's output or a release's evidence.

## Own vocabulary

Declared configuration, configuration drift, protection ruleset, adoption, apply, applied state, deploy key.

## Success criterion

The public home's configuration is reviewable and reproducible from the repository, an apply changes only what the preview named, and the configuration drift report is clean.

## Running record

- 2026-10-17 — Proposed after the Frame confirmation (Q1). Distinguishing criterion against `project-maintainer--release-owner`: authority boundary (applies organization and repository configuration, including access to the token) and evaluation capability (preview and configuration drift, not release evidence). Same human, different segment; confirmed by the user on 2026-10-17 (Q7).
- 2026-10-17 — Reviewed the Frame, Event Storming and use-case contracts as the administration lens; the topic's findings (no live change during adoption, partial apply reported as partial, drift never claimed on an unavailable state) were applied.

## Topic stances

### 2026-10-17 — Site-publishing plumbing included

- Topic: `public-home-infrastructure`.
- The user included the deploy key and `PAGES_DEPLOY_KEY` secret in the topic through Q1, option 1, even though no site source or publishing workflow exists.
- This stance does not authorize creating the credential or writing `infra/`; it records the confirmed scope.

### 2026-10-17 — Adoption is identity-only

- The adoption contract binds existing objects without changing live values; differences wait for the first preview.
- This stance does not authorize applying the differences.

## Promotion record

### 2026-10-17 — Confirmed ideation Handoff

- The user confirmed project-level promotion through the Handoff decision (Q12).
- The stable core and the topic's dated records were copied unchanged from the origin topic; the topic file remains historical.
- Promotion grants a reusable advisory administration lens only, not a product Actor, a token, or authorization to apply anything.
