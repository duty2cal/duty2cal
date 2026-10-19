# Arc42 assessment — build-browser-converter

Independent assessment against all twelve rows of the arc42-docs chapter
selection table. The change adds a second driving adapter and a static browser
page, so almost every chapter is affected.

| # | Chapter | Affected? | Reason / one-line summary |
|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The converter now has two realizations; the truthful-copy goal no longer claims "no browser application", and "supported platform" is scoped to downloadable archives. |
| 2 | 02 architecture-constraints | Yes | Added `wasm32-unknown-unknown`, the target-scoped `getrandom`/`wasm-bindgen`/`js-sys` policy, the no-I/O facade, the CSP/inline bundle, the 20 MiB bound and the pinned browser-test supply chain; scoped the profile's "not a browser driver" sentence. |
| 3 | 03 context-and-scope | Yes | Added the browser conversion route and its browser boundary (no backend, no other-origin request), plus the loopback-served Site output and the Site-publication boundary. |
| 4 | 04 solution-strategy | Yes | Added the browser-local delivery channel, the WASM facade, the browser build/test choices and ADR-0007; kept the browser route distinct from archive platforms. |
| 5 | 05 building-block-view | Yes | Added `ffi/wasm`, `web/`, the build/evidence toolbox and their modules; moved `ffi/wasm` out of the deferred list. |
| 6 | 06 runtime-view | Yes | Added the browser conversion/offer, failure/diagnostic-offer and capability/startup flows. |
| 7 | 07 deployment-view | Yes | Added the static Site output, loopback serving, the browser runtime and test-only tooling; scoped the "does not publish a browser converter" claim to the Site publication. |
| 8 | 08 crosscutting-concepts | Yes | Added browser-local conversion/no-network, browser offer/no-write, capability/startup gate, input bound and browser-toolchain pinning. |
| 9 | 09 architecture-decisions | Yes | Registered ADR-0007 (browser-local conversion facade) and recorded the routing of the browser-owned save and the Playwright driver to their canonical artifacts. |
| 10 | 10 quality-requirements | Yes | Added browser equivalence, privacy, explicit failure, capability/startup, input-bound, diagnostic-fidelity, save-semantics and evidence-reproducibility scenarios; scoped QS-022. |
| 11 | 11 risks-and-technical-debt | Yes | Added the WASM instantiation, fixture validity, browser supply chain, coverage exclusion, browser matrix, save semantics, manual trap check, publication-boundary and bundle-size risks, the targeted privacy-gate exclusions (R-045), plus three debt items; reflected ADR-0007's "Bad, because…" consequences. |
| 12 | 12 glossary | Yes | Generalized the lifecycle events for the browser offer, added **Offer**, and clarified **Supported platform** and the **Site output**/**Browser conversion** symbols. |

## Chapter decisions

- Chapters 1–3 describe the second realization and its boundaries without
  leaking implementation detail into the specification layer.
- Chapter 4 records the delivery-channel strategy; chapter 5 the blocks;
  chapter 6 the flows; chapter 7 the static output and evidence environment.
- Chapter 8 records the cross-cutting no-network, offer, capability and pinning
  rules.
- Chapter 10 uses the real browser scenario as the evidence for browser quality
  scenarios; chapter 11 records every accepted residual risk.
- Chapter 12 is the durable vocabulary: the executable keeps
  "published"/"written"; the browser realization records "offered", and the
  lifecycle rows name both.

## ADR routing

One candidate was assessed: **browser-local conversion facade**.

- Routing: `NEW_ADR` — ADR-0007.
- Five gates: alternatives PASS (server-side, executable-only, browser-local
  facade); architectural impact PASS (new driving adapter, `wasm32` and
  no-server deployment boundary, inward dependency); durable consequence PASS
  (workspace member, build/test toolchain, coverage boundary); lower-level
  artifact PASS (behavior is spec-owned, the component/deployment boundary is
  not); existing-ADR check PASS (ADR-0002 is applied, not amended; ADR-0003
  covers archive platforms, which the browser route is not; no archived record).
- Routed elsewhere: the browser-owned save → capability specification and
  `givn/specs/export-duty-plan-calendar/usecase.md`; the Playwright/JSON-lines
  driver → the change's `design.md`.

## Status

STATUS: DONE
