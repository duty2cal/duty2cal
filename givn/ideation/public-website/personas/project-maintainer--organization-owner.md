# Persona: project-maintainer--organization-owner (reused, supporting)

Reuse: `project-maintainer--organization-owner`, base 2026-10-17; this file adds record entries only.

## Running record

- 2026-10-19 — Supporting lens for public website. The deploy key on the Pages
  repository and the `PAGES_DEPLOY_KEY` secret are the prior topic's
  `publish-site-plumbing` seed; this topic's publish path consumes them. No new
  authority and no new segment.
- 2026-10-19 — Confirmed by the user as a supporting lens for this topic (Q2).

- 2026-10-19 — Event Storming reaction: the deploy key and
  `PAGES_DEPLOY_KEY` secret are the prior `publish-site-plumbing` seed's
  output; the site's publish command consumes them. The Pages source stays
  `main` at `/`. No new authority is granted here. Advisory only.

- 2026-10-19 — Use Case Elaboration reaction: the deploy key/secret are the
  prior `publish-site-plumbing` seed's output; the profile copy is applied
  through the declared configuration after the site is live; the Pages source
  stays `main` at `/`. Advisory only.
