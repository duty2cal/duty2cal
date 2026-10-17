# Persona: project-maintainer--organization-owner

## Stable core

- Name: Organization owner maintaining the public home's configuration.
- Role: owns the `duty2cal` organization and administers its repositories.
- Authority boundary: decides and applies the declared configuration; holds
  the administration token and the state; may change organization and
  repository settings. Does not decide product behavior, release readiness, or
  the conversion contract.
- Evaluation capability: judges a preview, a configuration diff, a ruleset's
  effect, and a configuration drift report; can tell whether the declaration
  matches the intended public home and whether a change is safe to apply.
  Cannot judge the converter's output or a release's evidence.
- Own vocabulary: declared configuration, configuration drift, protection
  ruleset, import, apply, state, deploy key.
- Success criterion: the public home's configuration is reviewable and
  reproducible from the repository, an apply changes only what the preview
  named, and the configuration drift report is clean.

## Running record

- 2026-10-17 — Proposed after the Frame confirmation (Q1). Distinguishing
  criterion against `project-maintainer--release-owner`: authority boundary
  (applies organization and repository configuration, including access to the
  token) and evaluation capability (preview and configuration drift, not
  release evidence). Same human, different segment; confirmed by the user on
  2026-10-17 (Q7).

## Topic stances

- None yet.
