# Persona: contributor--web-reviewer

Name: Contributor reviewing the public website

Status: confirmed by the user on 2026-10-19 (Q2); promoted to the project collection by confirmed Handoff on 2026-10-19 (Q9).

Origin topic: Public website (`public-website`). Relative artifact references in the running record refer to `givn/ideation/public-website/`.

## Role

A contributor who reviews the public page, the WASM facade and the publish
workflow without publishing the site.

## Authority boundary

May review and approve a website change. May not publish the site, apply
infrastructure, hold the deploy key, or weaken the no-network claim.

## Evaluation capability

Judges the Content-Security-Policy and its browser evidence, the provenance of
the WASM bundle, the page's behaviour in the supported browsers, and whether
the page's statements match the built output. Cannot judge conversion
semantics beyond the established contract, or release evidence.

## Own vocabulary

CSP, WASM bundle, deploy key, no-network proof, browser test, site version.

## Success criterion

A website change is reviewable in the same history as the converter, the
no-network claim has reproducible evidence, and every public statement matches
the built output.

## Running record

- 2026-10-19 — Promoted to the project collection by the user's confirmed Handoff (Q9). The topic-local file stays as historical record.

- 2026-10-19 — Created as a topic-local segment for `public-website`. It shares
  the review-only authority of `contributor--infrastructure-reviewer` and
  `contributor--build-configuration-reviewer`; it differs in evaluation
  capability (CSP, browser behaviour, WASM bundle) and own vocabulary.
- 2026-10-19 — Confirmed by the user (Q2). No promotion.

## Topic stances

- 2026-10-19 — `public-website`: a third-party request from the page is a
  review finding even if the conversion still works; the no-network claim is
  proven by the browser test, not by reading the CSP.

## Review record
- 2026-10-19 — Use Case Elaboration reaction: the browser check must observe
  every request after load, not only plan-carrying ones; the whole-content
  replacement removes hand edits; the licence file is a separate built output;
  the evidence gate includes the REUSE check. Advisory only.

- 2026-10-19 — Event Storming reaction: K5 needs a browser check that observes
  no request carrying the plan, not only a CSP read; the CSP must still allow
  WASM instantiation. K1/K2 make a hand edit to the Pages repository a defect,
  not a recovery path. The build failure (E11) and publication failure (E15)
  must stay separate so a broken deploy does not look like a broken
  conversion. Advisory only.
