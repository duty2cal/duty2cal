# Persona: contributor--infrastructure-reviewer

## Stable core

- Name: Contributor reviewing an infrastructure change.
- Role: a contributor who reads and reviews changes to the project, including
  `infra/`.
- Authority boundary: may review and approve a change; may not apply it, read
  the state or the administration token, or change repository settings.
- Evaluation capability: can read the declared configuration and judge whether
  a change matches its stated intent and the project's constraints; cannot
  preview it against the live organization.
- Own vocabulary: declared configuration, protection ruleset, review, preview
  shown in the pull request.
- Success criterion: an infrastructure change is reviewable in the same
  history as the code it supports, and no hidden state decides what it does.

## Running record

- 2026-10-17 — Proposed after the Frame confirmation (Q1). New segment, not a
  reuse: no project Persona holds a reviewing-but-not-applying authority
  boundary. Distinguishing criterion against
  `project-maintainer--organization-owner`: authority boundary (review only,
  no token, no state, no apply). Confirmed by the user on 2026-10-17 (Q7).

## Topic stances

- None yet.
