# Security policy

`duty2cal` reads a released duty-plan PDF and writes one calendar file beside
it. The material that must never leak is the plan itself and the people in it.

## What is in scope

- Anything that could expose private plan data: a duty plan, a calendar
  converted from one, a diagnostic report, or the identifying values in them —
  through the repository, a release asset, a committed fixture, a log or a
  diagnostic file.
- A conversion defect that silently misrepresents a plan's content rather than
  failing: an activity dropped, an instant invented, an interval widened, or a
  failure that reports success.
- The release channel: an archive that does not match its published checksum or
  its build provenance, a published release whose assets or tag can still be
  changed, an asset attached by hand, or a release published from a revision
  whose checks were not green.
- A credential or secret that could enter the repository or the published
  history, and any bypass of push protection or secret scanning.
- A defect in the project's own automation — the workflows and the scripts under
  `scripts/` — that weakens one of the promises above.

## What is out of scope

- A plan the converter does not support: it exits non-zero and writes a numbered
  diagnostic, which is the documented behaviour. Report a missing or wrong
  mapping as an ordinary issue, with a stripped sample.
- Vulnerabilities in third-party crates: report those to the crate's authors.
  This repository enables Dependabot alerts and code scanning, so findings
  appear as reviewable alerts here as well.

## How to report, privately

Use the repository's private vulnerability reporting:

<https://github.com/duty2cal/duty2cal/security/advisories/new>

(Security tab → **Report a vulnerability**.) That is the only confidential route
this project names, and it keeps the report private between you and the
maintainer until an advisory is published.

Do not open a public issue for a plan-exposure report, and never attach a real
duty plan or a calendar converted from one to any report. A stripped sample —
the page structure with every real name, employee number, date, station and
route removed — is enough to reproduce most defects.

For a defect that cannot expose private data, a normal
[issue](https://github.com/duty2cal/duty2cal/issues) is fine and reaches the
maintainer faster.

## How a report is handled

- The report is acknowledged, then assessed; if it is in scope, a fix and a
  release follow and the advisory is published with credit unless you prefer to
  stay anonymous.
- Please give the maintainer a reasonable window before disclosing publicly.
  These are the project's own promises, not a commercial support contract: there
  is no bounty.

## Supported versions

| Version | Supported |
|---|---|
| The latest published release | Yes |
| Any earlier release | No |

Published releases are immutable, so a correction is always the next version
rather than a change to the one you already downloaded (see
[ADR-0004](docs/arc42/adr/ADR-0004-immutable-draft-first-publication.md)).
