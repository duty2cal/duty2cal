# Change seed: build-browser-converter

Status: proposed by build-browser-converter

- Use-case IDs: `export-duty-plan-calendar` (browser capability)
- Topic: `public-website`
- Personas: `crew-member--personal-calendar-owner`,
  `project-maintainer--release-owner`, `contributor--web-reviewer`
- Suggested start: 1; no dependency. Can run in parallel with the prior
  topic's `publish-site-plumbing`.
- Handoff: confirmed by the user on 2026-10-19 (Q9).

## Accepted decisions

### The browser is a supported conversion route

- Context: the converter is distributed only as a Windows x86_64 executable;
  the site is a placeholder; the privacy rule forbids a server-side converter.
  The conversion boundary already takes bytes and returns text.
- Options:
  1. **Supported browser route** — the page converts in current evergreen
     browsers with the same completeness and explicit-failure guarantees as
     the executable.
     In practice: a macOS crew member opens the URL, converts, and gets the
     same calendar the executable would produce.
     Gain: the unsupported-platform gap closes without a Rust toolchain.
     Cost: the browser bundle, the page's failure behaviour and a browser test
     become support surface.
  2. **Best-effort convenience** — the page converts but only Windows remains
     supported.
     Gain: no browser-matrix gate.
     Cost: a user cannot distinguish a defect from a limitation.
  3. **Landing page only** — no conversion.
     Gain: smallest scope.
     Cost: the unsupported-platform gap stays.
- Chosen: **Supported browser route** because a browser failure is a defect
  and the privacy promise reaches every platform. Rejected: **Best-effort**
  because it advertises conversion without backing it; **Landing page** because
  it leaves the gap and drops the draft's value.

### The page offers; the browser saves

- Context: the permanent use case promises a file "beside the source" under an
  unused numbered name and a numbered diagnostic. A browser has no source
  directory and cannot observe the save.
- Options:
  1. **Browser-owned save** — the page offers `<source-stem>.ics` and
     `<source-stem>.parse-error.txt`; it writes nothing, never claims a save,
     and the browser handles the save location and name collisions.
     In practice: saving twice yields the browser's own suffix; the page never
     overwrites the first file.
     Gain: works in every supported browser; no page-side filesystem access.
     Cost: no promised unused numbered name; the page cannot observe the save.
  2. **File System Access API** — the page writes into a chosen directory and
     allocates the name.
     Gain: closer to the executable contract.
     Cost: unavailable in Safari and Firefox; breaks the supported set.
  3. **Server-side conversion** — upload and return.
     Gain: executable file semantics.
     Cost: violates the no-upload privacy rule.
- Chosen: **Browser-owned save** because it preserves completeness,
  no-write and explicit failure everywhere. Rejected: **File System Access**
  because it breaks the supported browsers; **server-side** because it
  violates privacy.

### Capability-based browser support and 20 MiB plans

- Context: the WASM parser runs in the tab; the project has no browser farm.
  An explicit version matrix would promise tests the Linux CI cannot run.
- Options:
  1. **Capability-based** — current browsers with WebAssembly and ES modules;
     Chromium tested in CI; Firefox and Safari checked manually; plans up to
     20 MiB; unsupported capability refused by name.
     Gain: a testable claim that does not rot with version numbers.
     Cost: "current evergreen" is a moving target; manual checks each release.
  2. **Explicit brand/version matrix** — named versions per browser.
     Gain: precise statement.
     Cost: a matrix to maintain; Safari cannot be tested in CI.
  3. **Chromium-only** — Chrome and Edge.
     Gain: matches CI.
     Cost: excludes current Safari and Firefox for no technical reason.
- Chosen: **Capability-based** because it is testable and keeps the confirmed
  route. Rejected: **Explicit matrix** because the evidence cannot cover it;
  **Chromium-only** because it narrows the route without a constraint.

### The no-network claim is proven, not asserted

- Context: the page must not send the plan; the draft supplies a self-only CSP
  and no third-party assets.
- Options:
  1. **Self-only CSP and a browser check** — `connect-src 'none'`, no
     third-party asset; the check observes every request after load and
     refuses any request to another origin.
     Gain: the claim has evidence.
     Cost: a browser check in the gate; a third-party asset is a defect.
  2. **CSP only** — no runtime check.
     Gain: cheaper.
     Cost: a CSP typo or an unobserved request is not caught.
- Chosen: **Self-only CSP and a browser check** because the page's central
  promise is privacy. Rejected: **CSP only** because it is an assertion, not
  evidence.

### Licensing follows the project

- Context: the repository is MPL-2.0; `org/**` is CC0-1.0; the draft recommends
  MPL-2.0 for the site code and CC0-1.0 for non-code assets.
- Options:
  1. **MPL-2.0 code, CC0-1.0 assets** — one project-wide code licence; no
     attribution burden on screenshots and logos.
     In practice: `web/*.js` and `ffi/wasm/**` carry MPL-2.0; `web/assets/**`
     and site prose are CC0-1.0; `LICENSES/` keeps MPL-2.0 and CC0-1.0.
     Gain: consistent with ADR-0001 and REUSE.
     Cost: a REUSE override for the assets.
  2. **GPL-3.0-or-later for `web/`** — the draft's earlier question.
     Gain: strong copyleft for the page.
     Cost: a second code licence; `deny.toml` and REUSE complexity; no
     practical benefit for a few dozen lines of glue.
  3. **MIT/0BSD for `web/`** — permissive.
     Gain: maximum reuse.
     Cost: a second licence for no practical advantage.
- Chosen: **MPL-2.0 code, CC0-1.0 assets** because it matches the project and
  ADR-0001. Rejected: **GPL** and **MIT/0BSD** because they add licence
  surface without a benefit.

### Inherit the permanent conversion contract

- Context: the permanent use case owns classification, timing, identity,
  descriptions and RFC 5545 output. The browser adds a channel.
- Options:
  1. **Inherit and generalize only the publication/diagnosis wording** — the
     conversion rules stay identical; the browser delta offers the files and
     writes nothing; the executable keeps its beside-the-source realization.
     Gain: one conversion contract; no fork.
     Cost: the permanent use-case wording needs a channel-neutral revision.
  2. **A separate browser contract** — duplicate the rules for the browser.
     Gain: no edit to the permanent contract.
     Cost: two conversion contracts that can diverge.
- Chosen: **Inherit and generalize** because the conversion rules must not
  fork. Rejected: **Separate contract** because it duplicates the domain.

## Open questions

None. The browser check's tooling and the manual Firefox/Safari checks are
tasks, not open decisions.
