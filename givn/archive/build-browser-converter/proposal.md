# Proposal: build-browser-converter

## Release note

A crew member can convert one supported duty plan to a complete calendar in a
current browser from the built Site output, with the plan never leaving the
tab, or receive an explicit failure with an offered diagnostic. The public
publication of that page is the separate `publish-public-website` change; this
change delivers the browser capability and its evidence, not the public URL.

## Use-Case Context

- Use-case ID: `export-duty-plan-calendar` (browser capability under the permanent leaf)
- Ideation topic: `public-website`
- Seed path: `givn/ideation/public-website/changes/build-browser-converter.md`
- Confirmed Personas: `crew-member--personal-calendar-owner`, `project-maintainer--release-owner`, `contributor--web-reviewer`

The permanent use case owns the conversion contract: the supported Released
Roster detail-table family, activity classification, timing, identities,
descriptions and RFC 5545 output. This change adds the browser realization of
the goal's publication and diagnosis. It does not create a second conversion
goal, a new source family or a server-side conversion route.

## Problem / Opportunity

The converter has one supported route: download the Windows x86_64 executable
and run it locally. A crew member on macOS, Linux, a phone or a managed machine
without administrator rights has no route unless they build the source with a
Rust toolchain, and a crew member who does not want to install software has
none. The project's privacy rule forbids a server-side converter, so the
missing route cannot be closed by uploading the plan.

The conversion boundary already accepts bytes and returns text, so the missing
piece is a browser-local realization: the plan is processed inside the tab, the
page offers the result, and the browser saves it.

## Proposed Solution

- A current evergreen browser with WebAssembly and ES modules loads the
  converter page served from the built Site output. A browser without those
  capabilities sees an explicit "cannot run" message and no file picker. A
  module-load or instantiation failure shows the same startup treatment, never
  a server or upload fallback.
- The crew member selects or drops one duty plan of at most 20 MiB. The bound
  is checked by the page before it reads the file. A larger plan, a source
  outside the supported family, or a plan with an unresolved required activity
  is refused by name and no calendar is offered.
- A supported plan produces a complete RFC 5545 calendar. The page offers it
  for saving as `<source-stem>.ics`; the page writes nothing, never claims a
  save it did not observe, and the browser owns the save location and name
  collisions.
- Every attempt clears the offer and state left by the previous attempt before
  it reads the new file, so a failed attempt withdraws any calendar an earlier
  attempt offered. A second successful conversion replaces the earlier offer.
- On failure the page shows the reason and offers a diagnostic text as
  `<source-stem>.parse-error.txt`, identifying the source filename and the
  reason. If the diagnostic cannot be offered, the primary failure remains
  visible and the page says no diagnostic was offered. A mid-conversion trap or
  out-of-memory abort is reported as a generic failure, never as a partial
  success, and discards the converter instance so the next attempt starts
  fresh.
- The page sends no request to another origin after it loads, loads no
  third-party asset, and stores no plan or derived data in persistent browser
  storage or a navigable URL or history entry. An ephemeral in-memory object URL
  for the download offer is allowed and is revoked when the page unloads.
- The conversion rules are inherited unchanged from the permanent use case;
  only the offer-and-save and diagnostic realization is browser-specific. For a
  source that carries its Created revision metadata, the executable and the
  browser produce byte-identical calendars; without it, only the
  construction-time DTSTAMP fallback differs.
- A real-browser test drives the rendered page, converts a supported plan,
  observes the rendered success state, captures the saved calendar, and proves
  that no other-origin request or persistent storage occurred.

## Capability Routing

`givn spec route --change build-browser-converter` reported no signal — the top
candidates tie at 13.00 — so no recommendation column is recorded. The decision
follows the placement test: the browser advances the Crew member's existing
calendar goal; channel is not a goal.

| Proposed capability | Decision | Rationale |
|---|---|---|
| `export-duty-plan-calendar` (browser realization) | `EXTEND export-duty-plan-calendar` | The goal is unchanged; the browser is a driving channel under the Crew member's calendar goal. A separate capability would be channel-shaped and would fork the conversion contract. The delta adds the browser realization to the existing capability. |

## Out of Scope

- Publishing the Public website, the Site output, the deploy key, the Pages
  workflow, the Built revision, and the full public copy/footer. Those belong
  to the `publish-public-website` site publication; this change builds the
  browser capability and its evidence.
- Changing the conversion rules, the supported source family, activity
  meanings, or the executable's beside-the-source behavior.
- A hosted converter, an upload endpoint, accounts, analytics, telemetry, or
  any server-side processing.
- File System Access API writes, page-side directory access, saving on the
  Crew member's behalf, or a numbered diagnostic name the page cannot
  guarantee.
- Importing the calendar into a client, reconciling a revised plan, or
  calendar editing.
- A browser version matrix or a browser farm. Support is capability-based;
  Chromium is driven in CI, and Firefox/Safari checks are release work.
- The Windows release archive, its platform declaration, and its release
  pipeline. The browser is a supported conversion route, not a downloadable
  archive platform: "supported platform" continues to name the platforms a
  release offers as archives, and this change adds no archive for the browser.
- A custom domain, a project-site redirect, a PWA/offline mode, localization,
  or native applications.

## Open Questions

### D1 — Which WASM build tool produces the browser bundle, and how is its version pinned?

- Context: the browser bundle must be produced from the existing Rust
  workspace; the project has no JavaScript build tool, and the chosen tool's
  version must be recorded under the version-freshness rule.
- Question: should the browser bundle be produced by `wasm-pack`, by a manual
  `cargo build` plus `wasm-bindgen-cli` sequence, or by a bundler such as
  `trunk`?
- Options:
  1. **`wasm-pack`** — one command produces the ES-module glue and the
     matching `wasm-bindgen` CLI; gain: a standard, documented `--target web`
     output and no hand-written glue; cost: one extra build tool whose version
     must be recorded; in practice: `wasm-pack build ffi/wasm --release
     --target web --out-dir ../../web/pkg`.
  2. **`wasm-bindgen-cli` only** — build with Cargo and run `wasm-bindgen`
     manually; gain: fewer layers; cost: fragile glue and copy steps that must
     be maintained; in practice: a shell sequence that must match the crate's
     `wasm-bindgen` version.
  3. **`trunk`/a bundler** — gain: a development server and asset pipeline;
     cost: a second build system and a larger supply chain for a static page
     that needs neither; in practice: `trunk serve` wrapping the same bundle.
- Disposition: carried — owner: `design.md` (Decisions ledger D1 and the
  Version freshness section) decides the tool and the pin/record policy.

### D2 — How is the real-browser E2E harness integrated with the Gherkin runner?

- Context: the project's Gherkin runner is cucumber-rs with
  `.fail_on_skipped()`; the browser must be driven for real, and the project has
  no Node test runner today.
- Question: should the browser scenarios extend the existing cucumber-rs
  runner with a Playwright driver, use a separate cucumber-js/Playwright
  runner, or keep a hand-maintained Playwright test file?
- Options:
  1. **Extend cucumber-rs with a Playwright driver** — the existing
     `DutyWorld` steps start a real Chromium driver and assert in Rust; gain:
     one Gherkin runner, `.fail_on_skipped()` remains, and the `.feature` file
     stays the executable spec; cost: a Node driver process and a JSON
     protocol between Rust and Playwright; in practice: a step calls the
     driver, which returns rendered state and download evidence.
  2. **A separate cucumber-js/Playwright runner** — gain: native Playwright
     BDD glue; cost: a second Gherkin runner, a second strict-mode contract,
     and two ways to run the suite; in practice: `npx cucumber-js` for browser
     features.
  3. **A hand-maintained Playwright test file** — gain: straightforward
     Playwright code; cost: a parallel test surface that duplicates the
     `.feature` text and violates the executable-spec invariant; in practice:
     browser assertions written twice.
- Disposition: carried — owner: `design.md` (Decisions ledger D2) decides the
  integration and names the step-definition files and the strict-mode proof.

### D3 — How is the `ffi/wasm` facade covered by the project's coverage gate?

- Context: the coverage gate measures production paths, but the browser bundle
  runs on a WASM target and the project's LLVM coverage tooling cannot collect
  a profile from the browser tab.
- Question: should the WASM facade be instrumented in the browser, covered by
  a native duplicate test, or excluded from the native coverage denominator
  and evidenced by the browser scenario?
- Options:
  1. **Exclude the facade, evidence it through the browser** — gain: no false
     coverage and no duplicate test; cost: the thin facade's Rust lines are
     not in the Cobertura denominator; in practice: coverage runs over the
     native packages only (`duty2cal-core`, `duty2cal-pdf` and `duty2cal`),
     while the browser E2E exercises the facade.
  2. **Instrument the WASM bundle** — gain: production coverage from the
     browser; cost: the browser target has no file-based LLVM profile writer
     in this toolchain, so the profile cannot be collected and merged; in
     practice: an unsupported instrumentation path.
  3. **Add a native duplicate test** — gain: native coverage of the facade;
     cost: a weaker duplicate of the browser scenario and a second path to
     maintain; in practice: a Rust test that calls the same conversion and
     asserts the same outcome.
- Disposition: decided — the operator approved excluding the wasm-only wrapper
  lines of `duty2cal-wasm` from the native Cobertura denominator and evidencing
  the facade through the browser scenario (design review, 2026-10-19). The
  shared core stays covered by the CLI rows. The residual risk and the
  condition for revisiting the exclusion are recorded in `design.md` and
  chapter 11. No native duplicate of the browser scenario is added.

### D4 — How is the diagnostic text kept identical between the executable and the browser?

- Context: the executable writes a two-line diagnostic naming the source and
  reason; the browser must offer the same text, but the format currently lives
  in the executable's failure report.
- Question: should the two-line diagnostic text be composed by a shared pure
  function, duplicated in the page, or moved into the conversion core?
- Options:
  1. **A shared pure function** — a function in the application layer composes
     `Source PDF: <name>\nReason: <reason>\n`; the CLI and the WASM facade call
     it; gain: one format, no drift; cost: a small refactor of the CLI's
     failure report; in practice: both routes produce byte-identical
     diagnostic text.
  2. **Duplicate the format in the page** — gain: no core change; cost: two
     formats that can diverge; in practice: the page and the CLI write the
     same lines twice.
  3. **Move publication into the core** — gain: one path; cost: the core would
     learn about files and download offers, breaking the bytes-in/text-out
     boundary; in practice: a filesystem or Blob dependency in the core.
- Disposition: carried — owner: `design.md` (Decisions ledger D4) decides the
  shared function and the page's remaining offer mechanics.

### D5 — How is the permanent use-case contract generalized without forking it?

- Context: the permanent use case promises beside-the-source publication under
  an unused numbered name and a numbered diagnostic; a browser has no source
  directory and cannot observe the save, so the wording needs a channel-neutral
  realization without forking the conversion rules.
- Question: should the permanent use case's publication/diagnostic wording be
  generalized and given the browser interaction, should a separate browser
  contract be written, or should the browser delta stay only in the change?
- Options:
  1. **Generalize the permanent wording and add the browser inventory row** —
     gain: one conversion contract, one durable inventory; cost: a reviewed
     edit to the permanent use case; in practice: the success and minimal
     guarantees name both realizations, and the interactions table gains the
     browser action.
  2. **A separate browser contract** — gain: no permanent edit; cost: two
     conversion contracts that can diverge; in practice: the browser rules
     restated in a second document.
  3. **A delta-only note** — gain: the smallest edit; cost: the permanent use
     case still promises only the executable's beside-the-source publication
     and the inventory omits the browser action; in practice: the browser
     behavior has no durable home.
- Disposition: carried — owner: `design.md` (Decisions ledger D5) and the
  specs artifact update `givn/specs/export-duty-plan-calendar/usecase.md`.

### D6 — Which browser/test tool versions are recorded, and from where?

- Context: version freshness forbids remembering a current version; the
  browser route adds Node.js, Playwright, a Chromium build, `wasm-pack` and
  `wasm-bindgen` to the test/build surface.
- Question: should the browser driver and build tools be looked up and pinned
  in lockfiles, always resolved to the latest at run time, or avoided by not
  adding a Node test dependency?
- Options:
  1. **Looked-up versions pinned in lockfiles** — gain: reproducible runs and
     a recorded source for every version; cost: a periodic upgrade change; in
     practice: `web/e2e/package-lock.json` and `web/.wasm-pack-version`.
  2. **Latest at every run** — gain: no upgrade work; cost: a moving test
     surface and non-reproducible failures; in practice: `npm install` without
     a lockfile.
  3. **No Node test dependency** — gain: no Node/Playwright supply chain; cost:
     no real browser evidence, which violates the Web UI policy; in practice:
     an HTTP client cannot click or read the rendered page.
- Disposition: decided — the setup task resolves the latest Node LTS once and
  then pins it in `web/e2e/.nvmrc` and the CI `node-version`; the resolved
  version is recorded. Playwright and the axe package are pinned in
  `web/e2e/package-lock.json`, `wasm-pack` in `web/.wasm-pack-version`, and the
  Rust crates in `Cargo.lock`. `design.md` (Decisions ledger D6 and the Version
  freshness section) owns the pin mechanism.

### Seed question disposition

| Seed question | Disposition |
|---|---|
| Q1 — What does the website promise? | answered — a supported browser route with the executable's completeness and explicit-failure guarantees; source: user confirmation on 2026-10-19. Realized by this change. |
| Q2 — Which Persona lenses review this topic? | answered — reuse `crew-member--personal-calendar-owner` and `project-maintainer--release-owner`, add `contributor--web-reviewer`; `visitor--project-reader` is confirmed for the page but assigned to `publish-public-website` by the use-case definitions, so this change records no visitor impact. Source: user confirmation on 2026-10-19. Recorded in Use-Case Context. |
| Q3 — Which browsers and plan sizes are supported? | answered — capability-based evergreen support (WebAssembly and ES modules), plans up to 20 MiB; source: agent under the user's "go ahead" authority, closed 2026-10-19. Realized by the page guard and the browser scenarios. |
| Q4 — How does the browser preserve output and diagnostic guarantees? | answered — browser-owned save; the page offers `<stem>.ics` and `<stem>.parse-error.txt`, writes nothing, and never claims a save; source: agent under the user's "go ahead" authority, closed 2026-10-19. Realized by the offer/no-write scenarios. |
| Q5 — Which version does a site built from `main` show? | out of scope — the Built revision and footer belong to the `publish-public-website` site publication, not to the browser conversion capability. |
| Q6 — Do the project-site redirect and a custom domain belong here? | out of scope — both are site publication decisions; the browser converter needs neither. |
| Q7 — Which public-copy changes accompany the site? | out of scope — the profile/README/public copy corrections belong to the site publication seed; this change builds the capability and its evidence. |
| Q8 — Which source changes trigger a site build? | out of scope — the path-filtered publish workflow belongs to the site publication; this change's build is the local/CI browser-evidence build. |
| Q9 — Should the Handoff records be published and promoted? | answered — the Handoff was confirmed on 2026-10-19 and the five terms and two Personas were promoted; source: user confirmation. This change uses the promoted Personas and terms. |
