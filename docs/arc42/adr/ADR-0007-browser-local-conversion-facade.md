---
status: accepted
date: 2026-10-19
decision-makers: operator (kressnerd)
consulted: none
informed: none
---

# Run the conversion in the browser through a WASM facade over the unchanged application

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

The decision fixes a new driving-adapter and deployment boundary: the browser
route is a `wasm32-unknown-unknown` facade over the existing application rather
than a server, an upload or a second conversion contract. The behavior of the
offer is owned by the capability specification and `usecase.md`; the mechanism
and test driver are owned by `design.md`. The existing-ADR check searched the
active register (ADR-0001 licence, ADR-0002 layer model, ADR-0003 release
channel and platforms, ADR-0004 immutable publication, ADR-0005 declaration
mechanism, ADR-0006 applied state) and the archived register (there is no
`docs/arc42/adr/archive/` directory). ADR-0002 owns the layer model this facade
applies, so it is not amended; ADR-0003 owns downloadable archive platforms,
which the browser route is not, so it is neither amended nor superseded.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort
      because: it would remove a supported conversion route and re-shape the
      workspace member, the WASM build, the browser page, the CI jobs that
      build and drive it, and the coverage denominator.
- [x] **Shapes quality attributes** - It materially affects privacy,
      portability and testability because: the plan never leaves the tab, the
      route works on platforms that receive no archive, and the page is proven
      through a real browser driver rather than an HTTP client.
- [x] **Structural** - It defines a boundary, contract, or dependency direction
      because: a new driving adapter depends inward on the unchanged
      application, and the deployment boundary is a static page with no server.
- [x] **Must be shared** - Developers unfamiliar with it would systematically
      violate the architecture because: they would add a backend or upload
      route, a second conversion contract, or a page-side filesystem write
      where the contract requires an offer.

## Context and Problem Statement

The converter was distributed only as a Windows x86_64 executable, so a crew
member on another platform or without installation rights had no supported
route. The privacy rule forbids a server-side converter, so the missing route
cannot be closed by uploading the plan. The application boundary already takes
bytes and returns text, which makes an in-tab driving adapter possible without
changing the conversion rules.

## Decision Drivers

- The plan must never leave the browser tab.
- The conversion contract must not fork; completeness, timing, identity and
  RFC 5545 output stay owned by the existing use case.
- A browser without the required capabilities must be refused explicitly
  rather than degrade into a partial result.
- The page must make no request after load and load no third-party asset.
- The browser route must be testable through the real rendered page.
- The build and test supply chain must be reproducible and pinned.

## Considered Options

- Browser-local facade over the unchanged application (`ffi/wasm` + static page)
- Server-side conversion with an upload endpoint
- Executable-only distribution, with no browser route

## Decision Outcome

Chosen option: "Browser-local facade over the unchanged application", because
it closes the platform gap without receiving the plan and without a second
conversion contract.

### Consequences

- Good, because the same `ConvertRoster` runs in the tab; the facade performs no
  filesystem, network or environment I/O and the plan never leaves the page.
- Good, because the page needs no backend, account or runtime service, and the
  route works on any current browser with WebAssembly and ES modules.
- Good, because the page can keep `connect-src 'none'` with an inline WASM
  bundle, so the no-network claim is structural and observable in the driver's
  request log.
- Bad, because the browser bundle cannot be LLVM-profiled by this toolchain, so
  `duty2cal-wasm`'s wasm-only wrapper lines are outside the native Cobertura
  denominator; the facade is browser-evidenced instead (chapter 11, R-039).
- Bad, because the page cannot observe or claim the save and cannot promise an
  unused numbered name; the browser owns the save location and collisions
  (chapter 11, R-041).
- Bad, because a browser bundle, a Playwright/Chromium test surface and a Node
  supply chain become support and evidence surface (chapter 11, R-038, R-044).
- Bad, because Firefox and Safari are manual release checks; CI drives Chromium
  only (chapter 11, R-040).

### Confirmation

The `@e2e` scenario "Crew member converts a supported duty plan in the browser
and saves the offered calendar" drives the rendered page through
Playwright/Chromium, asserts the empty and ready states and the request log,
captures the saved calendar and compares it with the reference fixture. The
regular browser scenarios cover the size bound, unsupported family, unresolved
activity, stale offer, drop, WebAssembly/ES-module gates, startup failure,
offer failures, cancelled save, naming and persistence. Strict mode is
`.fail_on_skipped()`; the driver fails closed on an unknown action. The design's
first setup milestone is a minimal real-Chromium instantiation under the real
CSP.

## Pros and Cons of the Options

### Browser-local facade over the unchanged application

- Good, because it reuses the one conversion contract and the pure core.
- Good, because the privacy promise is structural: no request, no upload, no
  persistent storage.
- Good, because the browser route needs no server to operate or secure.
- Bad, because the WASM bundle and browser test supply chain add build, size,
  coverage and browser-matrix surface.
- Bad, because the page cannot reproduce the executable's beside-the-source
  write or unused-name guarantee; it offers and the browser saves.

### Server-side conversion with an upload endpoint

- Good, because it could reuse the executable's file semantics and a simpler
  client.
- Bad, because it receives the plan, violating the project's privacy rule.
- Bad, because it introduces a service to operate, secure and pay for.

### Executable-only distribution, with no browser route

- Good, because there is no new build, test or browser support surface.
- Bad, because a crew member on another platform or without installation
  rights still has no supported route.
- Bad, because the source-build instruction remains the only fallback, and that
  route is not tested by CI for those platforms.

## More Information

The `build-browser-converter` change's `design.md`, proposal and delta
`export-duty-plan-calendar` specification; chapters 3 (the browser route), 4
(solution strategy), 5 (`ffi/wasm` and `web/`), 6 (browser runtime flows), 7
(the static output and its runners), 8 (the no-network/offer concepts), 11
(R-038 to R-044) and 12 (Browser conversion, Offer, Supported platform);
ADR-0002 (the layer model this adapter applies) and ADR-0003 (archive platforms,
which the browser route is not). The seed is
`givn/ideation/public-website/changes/build-browser-converter.md`; the
feasibility probe is `givn/ideation/public-website/evidence/wasm-probe.txt`
(compilation only, not browser instantiation).
