# Persona: contributor--infrastructure-reviewer

Name: Contributor reviewing an infrastructure change

Status: confirmed by the user on 2026-10-17; promoted to the project collection by confirmed Handoff on 2026-10-17 (Q12).

Origin topic: Public-home infrastructure (`public-home-infrastructure`). Relative artifact references in the running record refer to `givn/ideation/public-home-infrastructure/`.

## Role

A contributor who reads and reviews changes to the project, including `infra/`.

## Authority boundary

May review and approve a change; may not apply it, read the applied state or the administration token, or change repository settings.

## Evaluation capability

Reads the declared configuration and judges whether a change matches its stated intent and the project's constraints; cannot preview it against the live organization.

## Own vocabulary

Declared configuration, protection ruleset, review, preview shown in the pull request.

## Success criterion

An infrastructure change is reviewable in the same history as the code it supports, and no hidden state decides what it does.

## Running record

- 2026-10-17 — Proposed after the Frame confirmation (Q1). New segment, not a reuse: no project Persona held a reviewing-but-not-applying authority boundary. Distinguishing criterion against `project-maintainer--organization-owner`: authority boundary (review only, no token, no state, no apply). Confirmed by the user on 2026-10-17 (Q7).
- 2026-10-17 — Reviewed the Event Storming and the use-case contracts; the review lens found that a reviewer's approval cannot be assumed to gate an apply, recorded as Q10.

## Topic stances

### 2026-10-17 — Review gating is open

- Topic: `public-home-infrastructure`.
- Whether the reviewer's decision gates the apply or is advisory is Q10; this Persona does not assume either.
- The persona's "may review and approve" describes the act, not a guaranteed gate.

## Promotion record

### 2026-10-17 — Confirmed ideation Handoff

- The user confirmed project-level promotion through the Handoff decision (Q12).
- The stable core and the topic's dated records were copied unchanged from the origin topic; the topic file remains historical.
- Promotion grants a reusable advisory review lens only, not a Gating authority, a token, or a product Actor.
