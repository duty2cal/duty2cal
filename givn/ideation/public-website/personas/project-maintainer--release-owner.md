# Persona: project-maintainer--release-owner (reused)

Reuse: `project-maintainer--release-owner`, base 2026-10-12; this file adds record entries only.

## Running record

- 2026-10-19 — Reused for public website. The release owner's interest here is
  one site source, one build and one publish path; the page's public
  statements must match what it ships; the Windows release channel must not
  regress. The site publishes from path-filtered `main` (where no release tag
  necessarily exists) and from release tags (Q5, Q8 amended 2026-10-19).
- 2026-10-19 — Release-coupling stance: every release tag deploys the site
  from the tagged revision before the release workflow publishes the Windows
  archive. The archive contract itself is unchanged; a site failure leaves the
  previous site and blocks the release.
- 2026-10-19 — Topic stance: a site claim about the version it was built from
  is release evidence, not page copy; the footer must name a revision the
  repository can resolve.
- 2026-10-19 — Confirmed by the user as a review lens for this topic (Q2).

- 2026-10-19 — Event Storming reaction: the site's gate (P5) must refuse a
  failed conversion test, a failed browser check or a failed licensing check
  and leave the previous site in place; a footer version (K3) without a
  resolvable revision is not release evidence. The Windows release channel is
  unchanged by the browser route. Advisory only.

- 2026-10-19 — Use Case Elaboration reaction: the site's success includes the
  K4 content and the Built revision; a missing privacy statement or any
  third-party request refuses publication; the previous site stays on failure.
  The Windows archive contract is unchanged. Advisory only.
