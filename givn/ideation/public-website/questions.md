# Public website — questions

## Q1 — What does the website promise?

- Status: closed 2026-10-19 (user)
- Blocks: the Frame; therefore every Persona, event, use case and change seed
  in this topic. Specifically it decides whether the browser becomes a
  supported conversion route and whether the public copy may say so.
- Context: the converter is distributed only as a Windows x86_64 executable.
  The organization profile currently says the site "does not host the
  converter today", chapter 1's truthful-copy goal says "no browser
  application", and the site serves a stock placeholder. The draft
  (`private/website-idea.md`) builds a WASM converter into the page, which
  would make a browser a second conversion route. The conversion contract
  itself is already channel-independent: its boundary takes bytes and returns
  text, and chapter 5 lists `ffi/wasm` as deferred Phase 2 work. WASM
  feasibility was measured: the PDF extraction chain compiles for
  `wasm32-unknown-unknown` once `getrandom/wasm_js` is enabled. What remains
  open is the promise the project makes, not whether the code can exist.
- Question: should the browser be a supported conversion route with the same
  guarantees as the executable, or a best-effort convenience while Windows
  x86_64 remains the only supported route — or should the site remain a
  landing page with no conversion at all?
- Options:
  1. **A supported browser route** — the page is a first-class conversion
     surface for current evergreen browsers; the Windows executable stays
     available; the public copy states both.
     In practice: a crew member on macOS opens the URL, converts their plan,
     and expects the same complete-or-explicit-failure behaviour as on
     Windows; the project tests the page in a real browser and treats a broken
     browser conversion as a defect.
     Gain: the unsupported-platform gap closes without a Rust toolchain; the
     privacy promise reaches every platform.
     Cost: the browser matrix, the WASM bundle and the page's failure
     behaviour become support surface; the profile, README, quality goals and
     release claims must change; a browser test becomes part of the gate.
     Example consequence: the profile sentence "it does not host the converter
     today" is replaced, and a page that fails in a current browser is a defect
     rather than an accepted limitation.
  2. **A best-effort browser convenience** — the page offers conversion, but
     only Windows x86_64 remains a supported platform; the page says the
     browser route is not a support promise.
     In practice: the same macOS crew member can convert, but the project does
     not promise the result across browsers and does not gate a release on a
     browser test.
     Gain: no browser-matrix or support obligation; the page can ship earlier.
     Cost: the page's own claim is weaker than its usefulness; a user cannot
     tell which failures are defects; the conversion contract's guarantees are
     advertised but not backed by a support commitment.
     Example consequence: a Safari-only failure is closed as "not supported"
     while the page still offers the button.
  3. **A landing page only** — the site explains the project and links the
     source and the Windows release; it performs no conversion.
     In practice: the macOS crew member reads that the tool is Windows-only and
     that the source can be built with a Rust toolchain; nothing converts in
     the tab.
     Gain: the smallest scope; no WASM facade, no browser matrix, no new
     failure semantics, no public-copy reversal.
     Cost: the unsupported-platform gap stays exactly as it is; the page adds
     no conversion value; the draft's build workflow and privacy proof are
     unnecessary.
     Example consequence: the site links the release page and the source, and
     the profile's "does not host the converter" sentence stays true.
- Resolution: **A supported browser route** because the page's usefulness and
  the project's privacy promise reach every platform only if a browser
  failure is a defect rather than an accepted limitation, and because the
  conversion contract is already channel-independent. Rejected: **A best-effort
  browser convenience** because it advertises conversion without backing it,
  leaving a user unable to distinguish a defect from a limitation; **A landing
  page only** because it leaves the unsupported-platform gap exactly as it is
  and drops the draft's conversion value.

## Q2 — Which Persona lenses review this topic?

- Status: closed 2026-10-19 (user)
- Blocks: the review lenses for the Frame, the event storming, and every use
  case; the user confirms every Persona before it is used.
- Context: the Frame names four stakeholder types. The project collection
  already holds a crew-member lens, a release-owner lens, an organization-owner
  lens, an infrastructure-reviewer lens and a build-configuration-reviewer
  lens. The website adds a review surface (page clarity, CSP, no-network
  claim, WASM bundle) and a reader who does not convert; the prior public-home
  topic treated that reader as the crew-member lens because it supplied only
  plumbing, while this topic owns the page.
- Question: should the topic reuse the two existing lenses and add a web
  reviewer and a non-converting reader, or review with the existing lenses
  only?
- Options:
  1. **Reuse two, add two** — reuse `crew-member--personal-calendar-owner` and
     `project-maintainer--release-owner`; add
     `contributor--web-reviewer` and `visitor--project-reader`; keep
     `project-maintainer--organization-owner` as a supporting lens for the
     deploy key and secret.
     In practice: the web reviewer challenges the CSP and browser test; the
     reader challenges whether the page asks for data or assumes a plan.
     Gain: the page's two new failure modes (privacy claim, reader trust) get a
     dedicated critic.
     Cost: two more Persona files to confirm and consult; the contributor
     stakeholder type reaches its cap of three segments.
     Example consequence: a page that loads a third-party font is caught by
     the web reviewer before the reader ever sees it.
  2. **Reuse only** — review with the existing five lenses; the reader is the
     crew-member lens, as the prior topic decided.
     In practice: the crew-member lens judges clarity only while holding a
     plan; the build-configuration reviewer judges the WASM bundle.
     Gain: no new files; the prior position stands.
     Cost: the non-converting reader's trust interest and the no-network
     proof have no dedicated lens; the crew-member lens assumes a plan.
     Example consequence: the page could ask for an email before revealing the
     converter button and still pass the crew-member lens.
  3. **Add the web reviewer only** — reuse the existing crew-member and
     release-owner lenses; add `contributor--web-reviewer`; no visitor lens.
     In practice: the privacy claim and browser behaviour are reviewed; the
     reader's clarity is judged by the crew-member lens.
     Gain: the technical claim gets an expert critic without a new
     stakeholder type.
     Cost: the page's explanation and trust surface stay unreviewed by a
     reader who has no plan.
     Example consequence: a page whose first screen is a file picker passes;
     a colleague cannot tell what the project is before choosing a file.
  4. **Add the reader only** — add `visitor--project-reader`; no web reviewer.
     In practice: the page is reviewed for clarity and trust; the CSP and
     browser test are judged by the build-configuration reviewer.
     Gain: the page's public surface gets a reader lens.
     Cost: no lens owns the no-network proof and the browser matrix; the
     build-configuration reviewer judges size, not privacy.
     Example consequence: the page explains itself well but a CSP typo breaks
     conversion in one browser.
- Resolution: **Reuse two, add two** because the page's privacy claim and
  browser behaviour need a reviewer whose evaluation is not declared
  configuration or size, and the non-converting reader's trust interest is
  material once the page itself is the subject. Rejected: **Reuse only**
  because the crew-member lens assumes a reader with a plan and cannot judge
  the page's trust surface; **Add the web reviewer only** because the page's
  explanation would stay unreviewed by a reader without a plan; **Add the
  reader only** because no lens would own the no-network proof and the
  browser matrix. The organization owner remains a supporting lens for the
  deploy key and secret, which the prior topic owns.

## Q3 — Which browsers and which plan sizes are supported?

- Status: closed 2026-10-19 (agent, under the user's "go ahead" authority)
- Blocks: the support claim in Q1, the browser test, and the page's stated
  limits.
- Context: a WASM PDF parser runs inside the tab, so memory and browser
  capabilities bound the input. The project has no browser farm: CI runs
  Linux, and only Chromium is available there. A brand-and-version matrix
  would promise tests the project cannot run.
- Question: which browser families and minimum versions form the supported
  set, and what is the largest plan the page promises to handle?
- Options:
  1. **Capability-based evergreen support** — current browsers with
     WebAssembly and ES modules; the page detects a missing capability and
     says so; CI tests Chromium, Firefox and Safari are checked manually
     before a release; plans up to 20 MiB.
     In practice: a current Safari on macOS converts; an old browser without
     WebAssembly sees "this browser is not supported" instead of a broken
     button.
     Gain: the claim is testable and does not rot with version numbers; no
     browser farm is required.
     Cost: "current evergreen" is a moving target; manual Firefox/Safari
     checks are release work; 20 MiB is a public limit.
     Example consequence: a 25 MiB plan is refused with a named size failure,
     not silently exhausted.
  2. **Explicit brand and version matrix** — e.g. Chrome/Edge 120+,
     Firefox 120+, Safari 17+.
     In practice: the page names versions; the project must test each.
     Gain: precise support statement.
     Cost: a version matrix to maintain; Safari cannot run in the Linux CI,
     so the claim outruns the evidence.
     Example consequence: Safari 17.1 on an older macOS is "supported" but
     never tested by the project.
  3. **Chromium-only** — support Chrome and Edge; other browsers are
     best-effort.
     In practice: Firefox and Safari users see a warning.
     Gain: matches the CI browser exactly.
     Cost: narrows the Q1 choice and excludes current Safari and Firefox for
     no technical reason; the conversion code is not Chromium-specific.
     Example consequence: a current Firefox user is told the page is
     unsupported even though it works.
- Resolution: **Capability-based evergreen support** because it states a
  testable capability, keeps the Q1 promise without a browser farm, and
  refuses an unsupported browser explicitly. Rejected: **Explicit matrix**
  because the project cannot test the versions it would promise; **Chromium-
  only** because it narrows the confirmed Q1 route without a technical
  constraint. Plans are limited to 20 MiB; a larger plan is refused by name.

## Q4 — How does the browser preserve the output and diagnostic guarantees?

- Status: closed 2026-10-19 (agent, under the user's "go ahead" authority)
- Blocks: the use-case contract for the browser variant.
- Context: the permanent conversion use case promises a complete calendar
  "beside the source" under an unused numbered name and a numbered diagnostic
  file on failure. A browser download has no source directory and cannot write
  beside the file; the browser, not the page, decides where a download lands
  and how a name collision is handled.
- Question: which observable guarantees replace "beside the source under an
  unused name" and "a numbered diagnostic file" in the browser, without
  weakening completeness, no-overwrite behaviour, or explicit failure?
- Options:
  1. **Browser-owned save** — the page offers the complete calendar as a
     download named `<source-stem>.ics`; the browser owns the save location
     and collision handling; the page never writes and never claims a save.
     On failure the page shows the reason and offers a diagnostic text named
     `<source-stem>.parse-error.txt`; if it cannot produce that text, it keeps
     the primary failure visible and says no diagnostic was offered.
     In practice: the crew member clicks the offered file, chooses a save
     location, and the browser adds a suffix if the name exists.
     Gain: no page-side filesystem access; the no-destruction rule holds at
     the page boundary; works in every supported browser.
     Cost: the page cannot guarantee an unused numbered name or observe the
     save; the diagnostic may be saved under a browser-chosen name.
     Example consequence: saving `2026-06.ics` twice yields the browser's
     `2026-06 (1).ics`; the page never overwrites the first file.
  2. **File System Access API** — the page asks the user for a directory and
     writes the calendar there, checking collisions itself.
     In practice: the page can allocate an unused numbered name.
     Gain: closer to the executable contract.
     Cost: not available in Safari or Firefox; it would break the supported
     set and add a permission prompt.
     Example consequence: a Safari user cannot convert at all.
  3. **Server-side conversion and download** — the plan is uploaded and the
     calendar returned.
     Gain: the executable's file semantics.
     Cost: violates the no-upload privacy rule and the Frame's outside scope.
     Example consequence: a private duty plan leaves the machine.
- Resolution: **Browser-owned save** because it preserves completeness, the
  page-level no-write rule and explicit failure in every supported browser.
  Rejected: **File System Access API** because it is unavailable in Safari and
  Firefox; **server-side conversion** because it violates the confirmed
  privacy rule. The permission/unsaved-file semantics become a use-case rule,
  not a hidden assumption.

## Q5 — Which version does a site built from `main` show?

- Status: closed 2026-10-19 (agent, under the user's "go ahead" authority)
- Blocks: the footer's source-revision reference and the licence statement.
- Context: the draft stamps the version from a tag plus commit, but the site
  publishes on `main` pushes, where no release tag necessarily exists. The
  footer must name a revision the repository can resolve.
- Question: should the footer name the latest release version, the built
  commit, or both, and what happens when the built revision is not a released
  tag?
- Options:
  1. **`git describe` revision** — `v0.5.0` on a tagged revision,
     `v0.5.0-3-gabc1234` otherwise; the footer links the commit.
     In practice: a `main` build three commits after `v0.5.0` shows
     `v0.5.0-3-gabc1234`.
     Gain: one resolvable string for every build; no separate version state.
     Cost: the `-N-gSHA` form is less readable to a non-developer; it is a
     build-derived string.
     Example consequence: the footer links
     `https://github.com/duty2cal/duty2cal/commit/abc1234`.
  2. **Latest release tag only** — the footer shows `v0.5.0` even when the
     page was built from a later `main` commit.
     Gain: readable version.
     Cost: the footer can name a revision that did not build the page; the
     provenance claim is false.
     Example consequence: a page fix merged after `v0.5.0` still shows
     `v0.5.0`.
  3. **Commit SHA only** — the footer shows `abc1234`.
     Gain: exact provenance.
     Cost: no human-readable version; a reader cannot tell whether the page
     is older or newer than their release.
     Example consequence: the footer says `abc1234` and nothing about the
     nearest release.
- Resolution: **`git describe` revision** because it is resolvable for every
  build and names the nearest tag when one exists. Rejected: **Latest release
  tag only** because it can misstate the built revision; **Commit SHA only**
  because it loses the release relationship a reader needs. The exact string
  is a build detail; the rule is "resolvable and linked".

## Q6 — Do the project-site redirect and a custom domain belong here?

- Status: closed 2026-10-19 (agent, under the user's "go ahead" authority)
- Blocks: the site's address and publishing scope.
- Context: the draft recommends a redirect from
  `duty2cal.github.io/duty2cal` to `duty2cal.github.io`, which would need a
  Pages site on the monorepo that the declaration does not enable. A custom
  domain is mentioned only as a future `cname`.
- Question: should this topic publish the redirect page or a custom domain, or
  leave both to a later change?
- Options:
  1. **Out of scope** — the canonical address is
     `https://duty2cal.github.io/`; no redirect and no custom domain.
     In practice: README and profile links point at the canonical address.
     Gain: one Pages site, one publish path, no DNS or TLS commitment.
     Cost: an old project-site link does not redirect; a custom domain waits.
     Example consequence: a reader who follows the monorepo's Pages link gets
     no page, not a redirect.
  2. **Project-site redirect** — publish a static redirect from the monorepo
     Pages site.
     Gain: old links land on the canonical page.
     Cost: a second Pages site on the monorepo, which the declaration does not
     enable; the redirect is a client-side meta refresh.
     Example consequence: enabling Pages on the monorepo adds a declared
     object and a second publish path.
  3. **Custom domain** — serve the site from a project-owned domain.
     Gain: memorable address independent of GitHub.
     Cost: DNS, TLS, a `cname` file and a domain budget; the canonical URL
     changes.
     Example consequence: the Pages repository gains a `CNAME` and the
     deploy workflow sets `cname`.
- Resolution: **Out of scope** because the browser route needs neither a
  redirect nor a custom domain, and both add a second public surface or a DNS
  commitment. Rejected: **Project-site redirect** because it needs a second
  Pages site the declaration does not enable; **Custom domain** because it is
  a separate identity decision with DNS/TLS cost.

## Q7 — Which public-copy changes accompany the site?

- Status: closed 2026-10-19 (agent, under the user's "go ahead" authority)
- Blocks: the truthful-copy goal, the organization profile, the README, and
  the change seeds.
- Context: chapter 1, QS-022 and the organization profile currently state that
  there is no browser application and that the site does not host the
  converter. A browser converter makes those statements false. The profile is
  published through the declared configuration (`org/` → `infra/`), not by
  editing the profile repository.
- Question: are the profile and README updates part of the site's change, a
  separate seed, or a separate topic?
- Options:
  1. **One seed with the site** — the site publish and the public-copy
     correction share one change; tasks order the site publish before the
     profile apply, so a failed copy update leaves a stale "no browser" claim,
     never a promise of a page that does not exist.
     In practice: the change is not done until the page is live and the
     profile/README state it.
     Gain: no window in which the public copy promises a page that is absent.
     Cost: one larger change touching the site, the profile and the durable
     claims.
     Example consequence: the site publishes, then the profile apply replaces
     "does not host the converter today".
  2. **Separate copy seed after the site** — the site ships first, then a
     copy change.
     Gain: smaller site change.
     Cost: between the two changes the profile is false in the other
     direction, saying no browser exists while one does.
     Example consequence: a reader sees a working page and a profile that
     denies it.
  3. **Separate topic** — the website topic publishes; another topic owns the
     copy.
     Gain: narrow topic.
     Cost: the false public copy can persist indefinitely; no owner is named
     by the site's guarantee.
     Example consequence: the profile keeps the "no browser application"
     claim after the page ships.
- Resolution: **One seed with the site** because the change seed's guarantee
  must include the public copy that describes the site, while the site use
  case's success is the site alone. Rejected: **Separate copy seed** because
  it creates a window of false copy; **Separate topic** because it leaves the
  copy unowned and can persist.

## Q8 — Which source changes trigger a site build?

- Status: closed 2026-10-19 (agent, under the user's "go ahead" authority)
- Blocks: the build trigger in the publish workflow and policy P3.
- Context: the draft's workflow runs on pushes to `main` that change
  `crates/**`, `ffi/wasm/**`, `web/**` or the workflow itself. The project has
  no confirmed build-trigger decision; a path missed by the filter leaves the
  published site stale, while building on every push publishes on unrelated
  documentation changes.
- Question: should the site build on a change to the paths that affect it, on
  every push to `main`, or only on a release tag?
- Options:
  1. **Path-filtered `main`** — build when the site's inputs change:
     `crates/**`, `ffi/wasm/**`, `web/**`, `Cargo.toml`, `Cargo.lock`,
     `rust-toolchain.toml` and the site workflow.
     In practice: a docs-only commit publishes nothing; a change to the
     converter's crates rebuilds the site.
     Gain: the site is current without publishing on every commit.
     Cost: the path list is a maintained contract; a missed path leaves the
     site stale until the next matching change.
     Example consequence: adding a new site input outside the list needs a
     reviewed workflow change.
  2. **Every push to `main`** — build unconditionally.
     In practice: every merge publishes, including a change log or a document.
     Gain: no path list to maintain; the site is always the current `main`.
     Cost: needless publishes and deploy-key use; the Pages history churns.
     Example consequence: a typo fix in `docs/` republishes the site.
  3. **Release tag only** — build when a version is tagged.
     In practice: the site shows the latest released version, not `main`.
     Gain: the site always matches a released version.
     Cost: an unreleased page fix is invisible until the next release; a
     release can be delayed by a site-only defect.
     Example consequence: a broken footer waits for the next version tag.
- Resolution: **Path-filtered `main`** because it keeps the site current
  without publishing on unrelated changes and matches the supplied workflow
  shape. Rejected: **Every push** because it republishes on documentation-only
  changes; **Release tag only** because a page fix would wait for a release and
  the site would not reflect the source the footer names.

## Q9 — Should the Handoff records be published and promoted?

- Status: closed 2026-10-19 (user)
- Blocks: writing the durable glossary and the project Persona collection; the
  Handoff itself. No change start.
- Context: the topic has a readable record, two change seeds, five confirmed
  terms and two new Persona candidates. Promotion makes the terms and lenses
  durable for later topics; keeping them topic-local preserves the option to
  rename or replace them without touching the project collection.
- Question: should the Handoff publish the record and seeds, promote the five
  terms, and promote the two new Personas?
- Options:
  1. **Confirm all** — publish the record and seeds; promote the five terms
     and both Personas; the reused Personas stay topic-local.
     In practice: later topics cite **Browser conversion** and **Public
     website** and reuse the web-review and reader lenses.
     Gain: one durable vocabulary and review set; no re-confirmation later.
     Cost: the names and lenses enter the project collection; changing them
     later needs a merged record change.
     Example consequence: a later site change cites
     `contributor--web-reviewer` instead of recreating the lens.
  2. **Terms, no Persona promotion** — publish the record and seeds; promote
     the terms; keep both Personas topic-local.
     Gain: durable vocabulary without committing the lenses.
     Cost: a later topic must re-confirm the web-review and reader lenses.
     Example consequence: the reader lens is recreated, not cited.
  3. **Topic-local only** — publish the record and seeds; promote neither.
     Gain: no project-collection change.
     Cost: the names and lenses are not reusable; a later topic can coin a
     different name for the page.
     Example consequence: the glossary has no browser-website vocabulary.
  4. **Revise a named record** — name a use case, term, Persona or seed to
     change before publishing.
     Gain: the Handoff can correct the record first.
     Cost: the Handoff waits; no file is written.
- Resolution: **Confirm all** because the terms and lenses are already
  confirmed for this topic and later work should reuse them rather than
  re-litigate them. Rejected: **Terms, no Persona promotion** because the two
  new lenses would be recreated; **Topic-local only** because the durable
  vocabulary would stay fragmented; **Revise a named record** because no
  record was named for revision.
