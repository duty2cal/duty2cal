# Public website — ideation

Status: Handoff complete — published 2026-10-19 with the user's confirmation
(Q9). Five terms promoted to the glossary; two Personas promoted; two change
seeds ready. No change started. This document is the readable
record of the `public-website` ideation topic. It authorizes no change, no
`web/` or `ffi/wasm/` file, no workflow, no infrastructure apply and no
publication.

## Problem

The converter has one supported route: download the Windows x86_64 executable
and run it locally. A crew member on macOS, Linux, a phone or a managed
machine has no route except building the source with a Rust toolchain, and a
crew member who does not want to install software has none at all. The
project's public site exists but serves a stock placeholder, while the
organization profile says the site "does not host the converter today" and the
durable truthful-copy goal says there is no browser application.

The privacy rule excludes a server-side converter: a duty plan carries
identifying data and must not leave the crew member's machine.

## Vision

A crew member opens `https://duty2cal.github.io/` in any current evergreen
browser, selects one supported duty-plan PDF, and receives a complete calendar
file through the browser's save. The page writes nothing, claims no save, and
sends no plan or other communication to another origin. If the plan cannot be
converted, the page shows the reason and offers a diagnostic text.

The maintainer publishes the page from the monorepo's `main` branch. The build
is gated on the conversion tests, a browser check, licensing and the page
content; only a built Site output reaches the Pages repository. The footer
names and links the Built revision. The public copy, the durable claims and
the release platform wording are corrected in the same change seed, after the
site is live.

## Scope

Inside:

- Browser-local conversion under the existing conversion contract: the
  supported source family, complete-or-explicit-failure behaviour, RFC 5545
  output and source-snapshot identities.
- `ffi/wasm`, `web/`, and the build and publish path into
  `duty2cal/duty2cal.github.io`.
- The CSP no-network claim and the browser check that proves it.
- The site's public statements, footer, licence and Built revision.
- The profile, README, durable-claim and release-platform wording that the
  browser route makes false.

Outside:

- Changing conversion rules, adding a PDF family or reinterpreting activity
  codes.
- A hosted converter, upload, account, analytics or telemetry.
- The Windows release archive and its workflow.
- A custom domain and the project-site redirect.
- PWA/offline use, localization and native mobile applications.
- Creating, rotating or adopting the deploy key/secret; the prior topic's
  `publish-site-plumbing` seed owns it.

## What was verified

- WASM feasibility: the PDF extraction chain (`duty2cal-core`,
  `duty2cal-pdf`, `pdfplumber`, `lopdf`) compiles for
  `wasm32-unknown-unknown` once `getrandom/wasm_js` is enabled. The manifest,
  command, toolchain and output are in
  `givn/ideation/public-website/evidence/wasm-probe.txt`. This is compile
  feasibility only: no link, no `wasm-pack` bundle, no browser run and no
  no-network proof yet.
- The repository has no `web/` or `ffi/wasm/`; the only workflows are
  `ci.yml` and `release.yml`; `infra/pages.tf` declares the existing legacy
  Pages source and no deploy key.
- The prior topic's `publish-site-plumbing` seed is `ready`, not implemented;
  its pages-ruleset bypass decision is still open.
- The organization profile says the site "does not host the converter today";
  chapter 1, QS-022 and R-032 require "no browser application". A browser route
  makes those claims false.
- `LICENSES/` holds `MPL-2.0.txt`, `CC0-1.0.txt` and `Unicode-3.0.txt`; the
  draft's instruction to remove a GPL file is stale.

## Decisions

| Question | Decision | Rejected |
|---|---|---|
| Q1 — what the site promises | A supported browser route for current evergreen browsers; Windows x86_64 stays available; public copy states both. | Best-effort convenience; landing page only. |
| Q2 — review lenses | Reuse the crew member and release owner; add a web reviewer and a non-converting reader; organization owner supporting. | Reuse only; web reviewer only; reader only. |
| Q3 — browsers and sizes | Capability-based evergreen support (WebAssembly + ES modules); plans up to 20 MiB; unsupported capability refused by name. | Explicit brand/version matrix; Chromium-only. |
| Q4 — output and diagnostics | Browser-owned save: the page offers `<source-stem>.ics`, writes nothing, never claims a save; diagnostic offered as `<source-stem>.parse-error.txt` naming source and reason. | File System Access API; server-side conversion. |
| Q5 — version stamp | `git describe` output (`v0.5.0` or `v0.5.0-3-gabc1234`), linked to the commit. | Latest release tag only; SHA only. |
| Q6 — redirect and domain | Out of scope; canonical address is `https://duty2cal.github.io/`. | Project-site redirect; custom domain. |
| Q7 — public copy | One change seed with the site; the seed's guarantee covers the profile, README, durable claims and release wording; the profile apply follows the live site. | Separate copy seed; separate topic. |
| Q8 — build trigger | Path-filtered `main`: `crates/**`, `ffi/wasm/**`, `web/**`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `LICENSES/MPL-2.0.txt`, `.github/workflows/web.yml`. | Every push; release tag only. |

The full question records, with gains, costs, examples and rejected
alternatives, are in
`givn/ideation/public-website/questions.md`.

## Stakeholders and Personas

| Stakeholder type | Persona lens |
|---|---|
| Crew member converting their own plan | `crew-member--personal-calendar-owner` (reused) |
| Release maintainer | `project-maintainer--release-owner` (reused) |
| Organization owner applying the deploy key and profile copy | `project-maintainer--organization-owner` (reused, supporting) |
| Contributor reviewing the page, WASM and workflow | `contributor--web-reviewer` (new) |
| Visitor who does not convert | `visitor--project-reader` (new) |

## Use cases

### Export a duty-plan calendar (`export-duty-plan-calendar`) — affected

The permanent goal and conversion rules are unchanged. The browser capability
adds a channel: the page offers the complete calendar and the diagnostic; the
browser owns the save; the page writes nothing and never claims a save. The
permanent "beside the source under an unused name" wording is generalized for
the browser channel only; the executable keeps it.

### Publish the public website (`publish-public-website`) — new leaf

The Release maintainer's goal: the Public website at
`https://duty2cal.github.io/` serves the Site output built from the named
Built revision. The evidence gate covers conversion tests, the browser check
(every request after load), licensing/REUSE and the page-content check. A
failed build, gate or publish leaves the previous site in place; a successful
replacement is whole-content and removes hand edits; the footer names and
links the Built revision.

### Affected existing leaves

- `publish-public-release` — platform wording adjusted; the Windows x86_64
  archive contract unchanged.
- `apply-configuration-change` — the corrected profile copy from `org/` is
  applied through the declared configuration after the site is live.

The root `README.md` correction is an ordinary source revision in the change
seed, not an apply.

## Domain terms

Five terms confirmed by the user on 2026-10-19:

| Term | Definition | Contrast |
|---|---|---|
| Browser conversion | A conversion of one duty plan inside the browser tab; the plan never leaves the machine. | Not executable conversion; not a hosted conversion. |
| Public website | The published static page at `duty2cal.github.io` offering browser conversion and the project's public statements. | Not the Public home (source repository); not the Organization profile. |
| Site publication | The build-and-publish process turning a source revision into the Public website. | Not a Release; not a Configuration change. |
| Site output | The built page, script and WASM bundle written into the Pages repository. | Not a Release archive; not the source. |
| Built revision | The repository revision the Public website was built from, named in its footer. | Not the Source revision timestamp; not necessarily a Version tag. |

The collision records and exact commands are in
`givn/ideation/public-website/domain-terms.md`.

## Change seeds

Two seeds, in start order:

| Seed | Guarantee | Depends on |
|---|---|---|
| `build-browser-converter` | The browser conversion capability exists in the monorepo and converts the supported family in a browser; the page writes nothing, sends no plan, and passes the conversion and no-network checks. No public site change. | — (can run parallel with `publish-site-plumbing`) |
| `publish-public-website` | The Public website serves the browser converter from the Built revision, the evidence gate passes, and the public copy and durable claims state the browser route truthfully. | `build-browser-converter`; the prior `publish-site-plumbing` seed and its pages-ruleset decision |

Seed files are under `givn/ideation/public-website/changes/`. They are input
to `/givn-propose`, not proposals.

## Prerequisites and open items

- The prior topic's `publish-site-plumbing` seed is `ready` but not
  implemented. It creates the deploy key and `PAGES_DEPLOY_KEY` secret; this
  topic consumes them. Its pages-ruleset bypass decision remains open and must
  close before the publish path can use the key.
- The Pages source stays `main` at `/`; no custom domain and no project-site
  redirect.
- The browser check's browser matrix is capability-based; Chromium is tested in
  CI, Firefox and Safari are checked manually before a release.
- The site's release notes require adding the site paths to the user-receiving
  path list in `cliff.toml` and the new capability to the allow-list; the seed
  records this.

## What this does not authorize

No change is started by this Handoff. No `web/` or `ffi/wasm/` file, no
workflow, no `infra/` edit, no GitHub write, no site publication and no release
is authorized. The seeds are input to the normal proposal flow, one at a time,
when the user starts them.
