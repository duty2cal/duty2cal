# 9. Architecture Decisions

This file is the chapter-09 register/index for architecture decisions. One
architecturally significant decision gets exactly one ADR, stored as a separate
Markdown file under `docs/arc42/adr/ADR-NNNN-short-slug.md`.

## Register

| ADR | Title | Status | Date | File |
|---|---|---|---|---|
| ADR-0001 | Licence the project under MPL-2.0 (per-file weak copyleft) | accepted | 2026-10-12 | [ADR-0001-project-license.md](adr/ADR-0001-project-license.md) |
| ADR-0002 | Enforce the hexagonal layer model with crate boundaries and four policy levels | accepted | 2026-10-12 | [ADR-0002-layer-enforcement.md](adr/ADR-0002-layer-enforcement.md) |
| ADR-0003 | Publish downloadable releases for Windows x86_64 from one public repository | accepted | 2026-10-13 (amended 2026-10-17) | [ADR-0003-release-channel-and-platforms.md](adr/ADR-0003-release-channel-and-platforms.md) |
| ADR-0004 | Publish releases immutably from an inspected draft | accepted | 2026-10-13 | [ADR-0004-immutable-draft-first-publication.md](adr/ADR-0004-immutable-draft-first-publication.md) |
| ADR-0005 | Declare the public home's GitHub configuration with OpenTofu and the GitHub provider | accepted | 2026-10-17 | [ADR-0005-declare-the-public-home-with-opentofu.md](adr/ADR-0005-declare-the-public-home-with-opentofu.md) |
| ADR-0006 | Keep the applied state local, passphrase-encrypted and committed, and check configuration drift locally | accepted | 2026-10-17 (amended 2026-10-19) | [ADR-0006-encrypted-local-applied-state.md](adr/ADR-0006-encrypted-local-applied-state.md) |
| ADR-0007 | Run the conversion in the browser through a WASM facade over the unchanged application | accepted | 2026-10-19 | [ADR-0007-browser-local-conversion-facade.md](adr/ADR-0007-browser-local-conversion-facade.md) |

ADR-0001 and ADR-0002 were minted by the change to export released roster
calendars (export-released-roster-calendar); ADR-0003 and ADR-0004 were minted
by the change to publish a public release (publish-public-release). ADR-0003 was
**amended on 2026-10-17** by the windows-only-release change, which narrowed its
platform set to Windows x86_64; it was amended in place, not superseded, and
ADR-0004 is unaffected. ADR-0005 and ADR-0006 were minted by the change to
declare the public home (declare-public-home): the mechanism that declares the
configuration, and where the applied state lives and how it is protected. They
are separate records because they are separate boundaries — the tool that
declares the configuration does not decide where the state lives, and the state
decision does not depend on the tool choice beyond its encryption support.
ADR-0007 was minted by the change to build the browser converter
(build-browser-converter): one record for the browser-local conversion facade
(the new driving adapter, the `wasm32` target and the no-server deployment
boundary). The browser-owned save is observable workflow behavior routed to the
capability specification and `usecase.md`; the Playwright/JSON-lines test
driver is test tooling routed to `design.md`. Every other design choice in
those changes is a supplied constraint (chapter 2), a local mechanism
(`design.md`), or product behaviour owned by the capability specification and
the glossary, and did not pass the qualification gate.

ADR-0006 was **amended on 2026-10-19** by the change to create the pages
publishing plumbing (publish-site-plumbing): the state it protects now holds
the only copy of a live credential — the pages deploy key's private half, which
GitHub never returns — and losing the passphrase therefore means replacing the
credential rather than re-adopting. It was amended in place, not superseded.
That change's other candidates did not qualify: the pages rule set's
`DeployKey` bypass is protection policy and workflow behaviour owned by the
capability specification and `design.md` (the bypass is by actor type, not a
component or deployment boundary), and the choice to generate the key pair with
`tls_private_key` inside the declaration is a local mechanism owned by
`design.md` once ADR-0006 fixes that the state holds it.

The change that publishes the Public website (publish-public-website) minted
no ADR and amended none. Four candidates were assessed and routed; the serving
check was assessed in the design-review hardening of 2026-10-25:

- **The site's publishing topology** — an external Pages repository written
  from the monorepo by a write-enabled deploy key, with the project site and a
  hand-edited target rejected. It fails `lower_level_artifact`: the choice,
  its alternatives and their costs were already decided and recorded in the
  ideation seed's accepted decision, and the durable position is a deployment
  fact recorded in chapter 7 and `docs/arc42/07-deployment-view.md`. Routing:
  `CANONICAL_ARTIFACT` → `docs/arc42/07-deployment-view.md`.
- **The release's coupling to the site deploy** — a version publishes only
  after the site publication for its tag succeeded and the hosting target's
  branch tip carries the tagged Built revision; the site publication is a
  `site` job that calls the reusable workflow, and the `publish` job
  (`needs: site`) refuses before any `gh` call otherwise. It fails
  `architectural_impact` and `durable_consequence` as a boundary: it orders two
  existing capabilities and adds a read of the target the deployment view
  already fixes, reversal is one change's workflow and command edit, and the
  behaviour is owned by the release capability's specification. Routing:
  `CANONICAL_ARTIFACT` →
  `givn/specs/publish-public-release/publish-public-release.feature`.
- **The publication mechanism** — one command, a reusable workflow wrapper, an
  assembled `dist/site/` output of nine members including the vector mark and
  its raster tab alternate, a build-time `git describe` stamp and a local git
  target as the test twin. It fails `architectural_impact` and
  `durable_consequence`: the wrapper and the command choose build-and-publish
  tooling inside one boundary the deployment view already fixes, and reversing
  them costs a script and workflow edit.
  Routing: `CANONICAL_ARTIFACT` → `givn/changes/publish-public-website/design.md`.
- **The serving check** — the publication fetches the required `--verify-url`
  address after the replacement and refuses unless the served page carries the
  same Built revision text, with bounded retries; leaving serving unchecked, a
  separate post-deploy job and trusting the host's reported deployment were
  rejected. It fails `lower_level_artifact`: the refusal and its naming are
  observable workflow behaviour owned by the capability specification, and it
  moves no component, authority or deployment boundary — the write target and
  the credential are unchanged, and the check only reads an address the use
  case already names. It also fails the product-behaviour falsification test for
  the same reason. Routing: `CANONICAL_ARTIFACT` →
  `givn/specs/publish-public-website/publish-public-website.feature`; the
  retry bound and the fetch are ordinary design detail in `design.md`.

The change that adds the vector brand assets (add-vector-brand-assets) minted no
ADR and amended none. The publication-mechanism summary above now counts nine
members, including the vector mark and its raster tab alternate. Its one
candidate — the mark's vector master with a raster tab fallback — returned
`NOT_QUALIFIED` and routes to exactly one canonical artifact:
`givn/changes/add-vector-brand-assets/design.md`.

## Individual ADR contract

Written and tailored from `docs/arc42/adr/adr-template.md`; MADR sections,
frontmatter, and `ADR-QUALIFICATION` verdict required. A failed candidate is
routed exactly once to its canonical feature, design, Arc42, process,
project-documentation, or code location.

## Archive

Historical ADRs move to `docs/arc42/adr/archive/` preserving ID and filename;
both indexes are updated on any move. `givn/archive/<change>/` is not an ADR
location.
