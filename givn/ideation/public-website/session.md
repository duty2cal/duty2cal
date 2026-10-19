# Public website — session state

- Topic: `public-website`.
- Topic identity: selected by the agent on 2026-10-19 from the user's draft
  `private/website-idea.md`; the user confirmed the topic and its Handoff
  through Q9 without a separate naming decision.
- Phase: Handoff complete — published 2026-10-19 with the user's confirmation (Q9). The readable document, two ready seeds, five glossary terms and two Persona promotions are published.
- Next separate workflow: `build-browser-converter` is implemented and archived. The next seed is the prior topic's `publish-site-plumbing` (deploy key and secret), then `publish-public-website`; the pages-ruleset decision in the plumbing seed is still open.
- This completed ideation authorizes no change start, no `web/` or `ffi/wasm/` file, no workflow, no `infra/` edit, no GitHub write and no Handoff beyond the records published here.
- Completed authority: topic state, the readable topic record, the two change
  seeds, five glossary terms and two Persona promotions only. No change
  started, no `web/` or `ffi/wasm/` file, no workflow, no `infra/` edit and no
  GitHub write.

## Confirmed inputs

- Q1, the Frame, confirmed by the user on 2026-10-19 through option 1: the
  browser is a supported conversion route for current evergreen browsers, with
  the same completeness and explicit-failure guarantees as the executable;
  Windows x86_64 remains available; public copy states both. Rejected: a
  best-effort convenience and a landing page only.
- The supplied draft is user work, not confirmed direction.
- `private/website-idea.md` supplies the direction: a static site published
  from the monorepo into `duty2cal/duty2cal.github.io`; browser-local
  conversion; MPL-2.0 for the site's code and CC0-1.0 for its non-code assets;
  a footer with source, version, privacy and disclaimer; a CSP that forbids
  network requests; a redirect from the project site as an optional hybrid.

## Research completed

- `givn status`: no active changes; 13 archived changes. The archived changes
  include `declare-public-home`, `enable-repository-security` and
  `publish-organization-profile`.
- `givn spec tree`: four permanent use cases — `adopt-existing-configuration`,
  `apply-configuration-change`, `export-duty-plan-calendar`,
  `publish-public-release`. The conversion use case is channel-independent and
  already owns the complete-or-explicit-failure contract; the release use case
  owns the Windows x86_64 release.
- Read `README.md`; arc42 chapters 1, 2, 3, 5, 7, 10, 11 and 12; `infra/`
  resources; the prior topic's frame, use-case definitions, session and
  `publish-site-plumbing` seed; `org/profile/README.md`; the permanent
  conversion use case.
- Read `private/website-idea.md` without editing it. `private/opentofu.md`,
  `private/smaller-rust.md` and `private/pdf-extractor.md` were read for
  context; none is changed.
- Live/current state: `web/` and `ffi/wasm/` do not exist; the only workflows
  are `ci.yml` and `release.yml`; the Pages repository serves the stock site
  from `main` at `/`; `infra/` declares the Pages source only — no deploy key
  and no `PAGES_DEPLOY_KEY` secret; the prior topic's
  `publish-site-plumbing` seed is `ready` but not implemented.
- Public copy currently contradicts a browser converter: the organization
  profile says the site "does not host the converter today", chapter 1's
  truthful-copy goal says "no browser application", and QS-022 enforces that
  claim.
- WASM feasibility measured on 2026-10-19: the default check fails because
  `getrandom` 0.4 refuses wasm32-unknown-unknown; a scratch consumer enabling
  `getrandom/wasm_js` compiles the whole extraction chain for that target.
  The manifest, command, toolchain and output are recorded in
  `evidence/wasm-probe.txt`. This is compile feasibility only: no link, no
  `wasm-pack` bundle, no browser run, and no no-network proof.
- Licensing inventory: the repository is MPL-2.0 project-wide, `org/**` is
  CC0-1.0, and `LICENSES/` holds `MPL-2.0.txt`, `CC0-1.0.txt` and
  `Unicode-3.0.txt`; there is no GPL-3.0 file to remove.

## Persona confirmation

Q2 closed on 2026-10-19 with "Reuse two, add two": the crew member and the
release owner are reused; `contributor--web-reviewer` and
`visitor--project-reader` are new; `project-maintainer--organization-owner` is
a supporting lens for the deploy key and secret. The five topic-local files
are confirmed and carry record entries only (the reused ones) or the stable
core (the new ones).

## Persona reuse inventory

Inventoried 2026-10-19 against the project collection
(`crew-member--personal-calendar-owner`, `project-maintainer--release-owner`,
`project-maintainer--organization-owner`,
`contributor--infrastructure-reviewer`,
`contributor--build-configuration-reviewer`).

| Stakeholder type | Existing matches | Disposition |
|---|---|---|
| Crew member converting their own plan | `crew-member--personal-calendar-owner` | Reused: same authority (supplies the plan, judges usability), evaluation (compares activities, dates, report time), and vocabulary (duty plan, flight leg, report time). The browser adds a channel, not a new segment; the topic file holds record entries only. |
| Project maintainer / release owner | `project-maintainer--release-owner` | Reused: same authority (accepts or rejects the public claims), evaluation (release evidence, truthful copy). The topic adds a stance: publishes a site from `main`, judges the version stamp and the no-network claim. `project-maintainer--organization-owner` is a supporting lens for the deploy key and secret, which the prior topic owns. |
| Contributor reviewer | `contributor--infrastructure-reviewer`, `contributor--build-configuration-reviewer` | New segment `contributor--web-reviewer`: same review-only authority, but a different evaluation capability (the no-network claim, CSP, browser behaviour, WASM bundle) and own vocabulary (CSP, WASM, bundle, deploy key). The two existing contributor segments judge declared configuration and size claims. |
| Visitor who does not convert | none | New segment `visitor--project-reader`: new stakeholder type; no authority, evaluation of clarity and trust (does the page ask for data, can the source and release be reached), vocabulary (source, release, privacy, licence). The prior topic said the future site visitor was the crew-member lens because it supplied only plumbing; this topic owns the page, so the non-converting reader is a material lens. |

## Canvas render check

All three Mermaid canvases rendered on 2026-10-19 with
`@mermaid-js/mermaid-cli` 12.0.0 (Chromium 155): three non-empty SVGs were
produced from the extracted blocks. The per-node kind prefixes and shapes
carry the distinction; the first draft's `classDef` declarations were removed
because Mermaid's parser rejects `var(--accent)` in a class definition.

## Handoff state

- Published 2026-10-19 after Q9: `docs/ideation/public-website.md`; two seed
  files under `changes/`; five terms added to `docs/arc42/12-glossary.md`
  (**Browser conversion**, **Public website**, **Site publication**, **Site
  output**, **Built revision**); `contributor--web-reviewer` and
  `visitor--project-reader` promoted to `givn/personas/` with merged records.
  The reused Personas stay topic-local and historical.
- The use-case mapping: `export-duty-plan-calendar` gains the browser
  capability; `publish-public-website` is a new permanent leaf;
  `publish-public-release` and `apply-configuration-change` are affected
  amendments. No change started.

## Boundary reviews

- Frame/Persona review, 2026-10-19: four findings. Applied: Q1/Q2 status
  records corrected; deploy key/secret ownership moved back to the prior
  topic's `publish-site-plumbing` seed with its pages-ruleset question named
  as a prerequisite; the affected durable claims and the declared-configuration
  apply path for the profile were added to scope; the WASM probe was
  reproduced from a clean target and recorded as compile-only evidence. No
  finding changed a confirmed direction, so none returned to the user beyond
  this report.
- Event Storming review, 2026-10-19: rewritten around the confirmed aggregate
  vocabulary, a single element inventory (IDs shared by the table and the
  canvases), explicit failure partners for the build and publication, policies
  separated from constraints and drawn as governance edges, and the target
  repository separated from GitHub Pages. Rejected one finding: E4/E5 remain
  in the aggregate boundary because the glossary's aggregate table confirms
  `Publish calendar export` and `Calendar export published`; the browser's
  realization is Q4, now closed. The first canvases' `classDef` styles
  were already removed and all three render. Q8 records the build trigger and
  is closed as path-filtered `main`.
- Use Case Definition review, 2026-10-19: two blocking and six advisory
  findings applied. The site success guarantee no longer absorbs the profile
  copy; the trigger is the path-filtered push/event, not a decision; the
  guarantees were made implementation-independent; the browser success
  guarantee no longer promises an unused name; the README is a source
  revision, not an apply; `publish-public-release` and
  `apply-configuration-change` are recorded as affected existing leaves; K4
  is in the site contract; and the event storming's closed-question markers
  were corrected. The definition now records the prerequisite seed's status
  and the Pages source.
- Use Case Elaboration review, 2026-10-19: nine blocking/advisory findings applied. K4 is gated by a content check; whole-content replacement and failure outcomes are consistent; the Site output term and the separate licence file are aligned; the trigger includes the licence source and names `.github/workflows/web.yml`; the browser offer names and diagnostic content are explicit; extensions name guarantees; Q7's seed-vs-use-case boundary is reconciled; durable claims are owned by the seed; K5 covers every third-party request; ephemeral offer URLs are distinguished from persistent storage; the trigger/precondition and mechanism-vocabulary issues were corrected.

## Open questions

- Q1 — Frame (closed 2026-10-19, option 1).
- Q2 — the Persona lenses (closed 2026-10-19: reuse two, add two; organization owner supporting).
- Q3 — closed: capability-based evergreen support, 20 MiB plans.
- Q4 — closed: browser-owned save; page offers, browser saves and handles
  collisions; diagnostic offered as text, never claimed as saved.
- Q5 — closed: `git describe` revision, linked to the commit.
- Q6 — closed: redirect and custom domain out of scope.
- Q7 — closed: one seed with the site; site publish before the profile apply.
- Q8 — closed: path-filtered `main`.
- Q9 — Handoff confirmation (closed 2026-10-19: confirm all; five terms and
  two Personas promoted, two seeds ready).
