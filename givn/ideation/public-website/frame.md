# Public website — Frame

Status: **confirmed by the user on 2026-10-19 (Q1, option 1)**: the browser is
a supported conversion route for current evergreen browsers, with the same
completeness and explicit-failure guarantees as the executable; Windows x86_64
stays available and the public copy states both. Research observations stay
findings, not requirements; the supplied draft remains supplied direction.

## Problem

The converter has exactly one supported route: download the Windows x86_64
executable and run it locally. A crew member on macOS, Linux, a phone or a
managed machine has no route except building the source with a Rust toolchain,
and a crew member who does not want to install software has none at all. The
project's public site exists but serves a stock placeholder, so the URL a
colleague shares states nothing about the tool.

The privacy promise is the constraint that shapes any solution: a duty plan
carries identifying data, and the project's own rule is that no file leaves the
crew member's device. A server-side converter is therefore not an option.

## Vision

A crew member opens `https://duty2cal.github.io/` in any modern browser, drops
or selects one supported duty-plan PDF, and receives either a downloadable
`.ics` file or an explicit failure that names the reason. Every byte is
processed inside the browser tab: no upload, no account, no storage, no
network request after the page has loaded.

Example: a crew member on a company laptop without administrator rights opens
the URL, selects `2026-06.pdf`, and downloads `2026-06.ics`; opening the page
again later issues no request that carries the plan.

The page states what it does, links the source and the Windows release, shows
the exact source revision it was built from, and carries the licence, privacy
and as-is statements. The maintainer publishes it from the monorepo's `main`
branch: the build runs in CI, and only the built output lands in
`duty2cal/duty2cal.github.io`. The output is never edited by hand.

## Proposed scope

Inside:

- Browser-local conversion of one supported duty plan, reusing the permanent
  conversion contract (`export-duty-plan-calendar`): the supported source
  family, complete-or-explicit-failure behaviour, and the RFC 5545 output.
  The browser is not a distributed executable platform: the release use
  case's supported-platform promise stays Windows x86_64 for archives, and
  the public copy states the browser and executable routes separately. The
  underlying concepts are the candidate terms **Browser conversion** and the
  existing **Distributed executable**; "conversion channel/route" is
  explanatory prose, not a term.
- A minimal static page: file selection or drag-and-drop, the result or the
  failure, and the public statements (what it does, source link, release link,
  version, licence, privacy, non-affiliation, as-is).
- A WASM facade (`ffi/wasm`) over the existing application boundary and the
  static source under `web/`.
- The browser build and the publish workflow from the monorepo into the Pages
  repository, consuming the deploy key and `PAGES_DEPLOY_KEY` secret created
  by the prior topic's `publish-site-plumbing` seed. Creating, rotating or
  adopting that credential stays with that seed; its open pages-ruleset
  bypass question is a prerequisite (Q2 of the prior topic, not this one).
- The technical no-network statement: a Content-Security-Policy served or
  declared with the page that forbids network requests, and no third-party
  asset.
- Licensing and REUSE coverage for `web/` and `ffi/wasm/`: MPL-2.0 for code,
  CC0-1.0 for non-code assets.
- The public copy and durable claims that make the browser route true:
  - chapter 1's truthful-copy goal (no browser application), chapter 2's
    "test interface is the GitHub REST API, not a browser driver" sentence,
    chapter 3's scope boundary, chapter 5's deferred `ffi/wasm` note, chapter
    7's "does not publish a browser converter" consequence, quality scenario
    QS-022 and risk R-032's content-invariant mitigation;
  - the platform wording in the release use case (`publish-public-release`),
    which currently says nothing else is supported, tested or offered;
  - the organization profile and the repository README.
  The profile change is applied through the declared configuration
  (`org/profile/README.md` → `infra/org_profile.tf`), not by editing the
  profile repository; the organization owner's apply authority is the one the
  prior topic established.

Outside:

- Changing the conversion rules, supporting a new PDF family, or giving
  unexplained activity codes new meanings.
- A hosted converter, an upload endpoint, accounts, analytics, telemetry, or
  any server-side processing.
- Calendar-client import, automatic reconciliation with an imported calendar,
  or calendar editing.
- The Windows release path, its archive, its provenance, and the supported
  platform declaration for the executable.
- A custom domain, the optional project-site redirect, a PWA or offline use,
  localization, and native mobile applications.
- Team access and organization membership.

## Prerequisites owned elsewhere

- The prior topic's `publish-site-plumbing` seed creates the Pages deploy key
  and the `PAGES_DEPLOY_KEY` secret. It is `ready`, not implemented, and its
  open pages-ruleset bypass question must close before the publish path can
  use the key. This topic consumes the credential; it does not create it.
- The declared configuration's Pages source stays `main` at `/`; the site
  publish path replaces the target's content and does not change the Pages
  build type.
- The Windows release path is unchanged; the browser route adds no archive
  and no platform to the release matrix.

## Stakeholders

These are stakeholder types, not approved Persona biographies or Gherkin
Actors. Personas are proposed only after the Frame is confirmed.

| Stakeholder type | Interest | Concrete example |
|---|---|---|
| Crew member converting their own plan | A usable calendar on any browser without installing anything, with the same completeness and failure behaviour as the executable, and no plan data leaving the machine. | On a macOS laptop, the crew member converts their plan and gets the same calendar the Windows executable would produce. |
| Project maintainer / release owner | One site source, one build, one publish path; the page's claims are true and the version it shows is traceable to a revision. | A push to `main` publishes a site whose footer names the commit it was built from. |
| Contributor reviewer | Can review the page, the WASM facade and the publish workflow in the same history as the converter; can check the no-network claim and the licence headers. | A pull request that adds a font from a third-party host is refused because it breaks the CSP and the privacy statement. |
| Visitor who does not convert | Understands what the project does, reaches the source and the Windows release, and is not asked to upload anything. | A colleague opens the URL, reads the privacy statement and follows the release link. |

## Supplied direction from the draft

`private/website-idea.md` supplies the following; each is treated as a
direction unless the user revises it:

- The site is generated from the monorepo and published into
  `duty2cal/duty2cal.github.io`; the Pages repository holds only build output.
- The publish path uses a write-enabled deploy key, not `GITHUB_TOKEN`,
  because the target is a different repository.
- `web/` holds `index.html` and a minimal ES module; `pkg/` is generated and
  ignored; a failing test gates the deploy.
- The page carries a footer with the source link, the version built from the
  tag and commit, and the as-is and privacy statements.
- A Content-Security-Policy forbids network requests (`connect-src 'none'`).
- Code under `web/` is MPL-2.0; non-code assets are CC0-1.0; `LICENSES/` keeps
  `MPL-2.0.txt` and `CC0-1.0.txt`.
- A hybrid redirect from the project site to the canonical org URL is out of
  scope (Q6).

## Research protocol and current evidence

1. Read the project's state before asking what it already answers:
   `givn status` shows no active change; `givn spec tree` shows four permanent
   use cases.
2. Read the durable context: `README.md`; arc42 chapters 1 (goals), 2
   (constraints), 3 (context), 5 (building blocks), 7 (deployment), 10
   (quality), 11 (risks) and 12 (glossary).
   Chapter 2 already says the use-case boundary takes bytes and returns text,
   so one use case can serve CLI, WASM and UniFFI. Chapter 5 lists `ffi/wasm`
   as deferred Phase 2 work. Chapter 1's truthful-copy goal and QS-022
   currently require the public copy to say there is **no** browser
   application; this topic would revise that claim.
3. Read the prior topic's Frame, use-case definitions, session state and the
   `publish-site-plumbing` seed. That seed owns the deploy key and secret; the
   site's content, build and WASM UI were explicitly outside that topic.
4. Inspect the repository: `web/` and `ffi/wasm/` are absent; `ci.yml` and
   `release.yml` are the only workflows; `infra/pages.tf` declares the existing
   legacy Pages source and no deploy key; `infra/repos.tf` declares both
   repositories.
5. Inspect public copy: `org/profile/README.md` says the site "does not host
   the converter today"; the README distributes Windows x86_64 only. A browser
   converter changes both statements.
6. Check WASM feasibility empirically instead of assuming it. The default
   check fails on `getrandom` 0.4's wasm policy; a scratch consumer with
   `getrandom = { version = "0.4", features = ["wasm_js"] }` compiles the
   whole extraction chain for `wasm32-unknown-unknown`. The exact manifest,
   command, toolchain and output are recorded in
   [evidence/wasm-probe.txt](evidence/wasm-probe.txt). That evidence covers
   compilation only: no link, no `wasm-pack` bundle, and no browser run yet.
7. Read `private/website-idea.md` without editing it. Its external claims
   (GitHub Pages redirect behaviour, the peaceiris action's inputs, the MPL
   obligations) are supplied direction, not verified here.

## Findings — draft claims and current repository state

| Claim in the draft | Finding |
|---|---|
| A second free personal account violates GitHub's terms, so use an organization. | Already resolved: `duty2cal` exists and the monorepo was transferred there before this topic. |
| The Pages repository is to be created. | It exists and serves the stock site; the publish path replaces its content, never creates the repository. |
| The deploy key and `PAGES_DEPLOY_KEY` secret are still to be created. | Correct: `infra/` declares neither. They are the prior topic's ready `publish-site-plumbing` seed, not this topic's decision. |
| The Pages source is "deploy from a branch", `main`, root. | Already declared: `github_repository_pages.pages` has `build_type = "legacy"`, branch `main`, path `/`. |
| `web/` and `ffi/wasm/` exist in the monorepo. | They do not; both are new. |
| The site can promise "no network request". | Technically reachable with a self-only CSP and no third-party asset; the browser test must prove it. The draft's `wasm-unsafe-eval` requirement is supplied, not verified. |
| `LICENSES/GPL-3.0-or-later.txt` must be removed. | Stale: the file does not exist. `LICENSES/` holds `MPL-2.0.txt`, `CC0-1.0.txt` and `Unicode-3.0.txt`. |
| The site's version comes from a tag plus commit. | The site publishes on `main` pushes, where no release tag necessarily exists; the version rule is resolved as `git describe` plus a commit link (Q5). |
| A project-site redirect is recommended. | It would need a second Pages site on the monorepo, which the declaration does not enable; out of scope (Q6). |
| The browser converter's publication guarantees are the executable's. | The permanent use case promises a file "beside the source" under an unused numbered name and a numbered diagnostic file. A browser download has neither a source directory nor a filesystem write; the variant offers the file and the browser saves it (Q4). |
| The deploy key and secret can be created by the site change. | They belong to the prior topic's `publish-site-plumbing` seed; this topic consumes them. The seed is `ready`, not implemented, and its pages-ruleset bypass question is open. |
| The browser route leaves the release platform statement untouched. | The release use case currently says nothing else is supported, tested, distributed or offered for download. The browser is not an archive platform; the wording must distinguish the browser and executable routes without changing the Windows archive contract. |
| A compile check proves browser support. | It proves only that the Rust crates compile for `wasm32-unknown-unknown` with `getrandom/wasm_js`. Linking, `wasm-pack` output, a real-browser run, and the no-network claim remain unverified. |

## Open questions

Recorded in `questions.md`:

No open question remains. Q1–Q8 are closed: the Frame (Q1), the Persona lenses
(Q2), capability-based browser support and 20 MiB plans (Q3), browser-owned
save and diagnostic text (Q4), `git describe` versioning (Q5), redirect and
custom domain out of scope (Q6), one change seed for the site and its public
copy (Q7), and path-filtered `main` (Q8).
