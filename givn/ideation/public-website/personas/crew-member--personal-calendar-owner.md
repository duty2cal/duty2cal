# Persona: crew-member--personal-calendar-owner (reused)

Reuse: `crew-member--personal-calendar-owner`, base 2026-10-12; this file adds record entries only.

## Running record

- 2026-10-19 — Reused for public website. The crew member's interest here is a
  conversion route on any browser, with the same completeness and
  explicit-failure behaviour as the executable, and no plan data leaving the
  machine. No new authority over publishing.
- 2026-10-19 — Topic stance: a browser download is the page's publication; the
  no-overwrite promise means the page must never silently replace an existing
  download, while the browser, not the page, chooses the download location.
  Q4 is closed: the page offers the file and writes nothing; the browser owns the save and its collision handling; the page never claims a save.
- 2026-10-19 — Confirmed by the user as a review lens for this topic (Q2).

- 2026-10-19 — Event Storming reaction: the chain must distinguish a completed
  calendar (E2) from a published one (E4) and from a failed publication (E5);
  a page that says "done" before the file is available repeats the failure
  this project exists to avoid. The browser's realization of "published" is
  Q4, not an assumption. Advisory only.

- 2026-10-19 — Use Case Elaboration reaction: the browser offer must not claim
  a save; the diagnostic must name the source and the reason; a plan over
  20 MiB must be refused by name. The permanent "beside the source" wording is
  generalized only for the browser channel, not for the executable. Advisory
  only.
