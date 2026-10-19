# Change seed: publish-public-website

Status: ready

- Use-case IDs: `publish-public-website`, `apply-configuration-change`
  (profile copy), `publish-public-release` (platform wording)
- Topic: `public-website`
- Personas: `project-maintainer--release-owner`,
  `contributor--web-reviewer`, `visitor--project-reader`,
  `crew-member--personal-calendar-owner`,
  `project-maintainer--organization-owner` (supporting)
- Suggested start: 2; depends on `build-browser-converter` and the prior
  topic's `publish-site-plumbing` seed.
- Handoff: confirmed by the user on 2026-10-19 (Q9).

## Accepted decisions

### Publish from the monorepo into the Pages repository

- Context: the Pages repository exists and serves the stock site; the
  declaration sets its Pages source to `main` at `/`. The site must be built
  from the monorepo, not hand-edited.
- Options:
  1. **External Pages repository** — CI builds the Site output and publishes
     it into `duty2cal/duty2cal.github.io` with a write-enabled deploy key;
     the target holds only built output.
     In practice: the canonical URL `https://duty2cal.github.io/` serves the
     built page; a hand edit is removed by the next whole-content publication.
     Gain: the short canonical URL is the page; one source.
     Cost: a deploy key and a second repository to publish into.
  2. **Project site under `duty2cal.github.io/duty2cal`** — deploy into the
     monorepo's own Pages site.
     Gain: no deploy key.
     Cost: the canonical URL is the long path; the org site would only
     redirect, and Pages has no server redirects.
  3. **Hand-edited Pages repository** — edit the site in the target.
     Gain: no workflow.
     Cost: no source, no gate, no traceability.
- Chosen: **External Pages repository** because the short URL is the product
  address and the target stays output-only. Rejected: **Project site** because
  it makes the canonical URL a redirect; **Hand-edited** because it loses the
  source and the gate.

### Consume the prior deploy key and secret

- Context: the prior topic's `publish-site-plumbing` seed creates the deploy
  key and `PAGES_DEPLOY_KEY` secret. That seed is `ready`, not implemented; its
  pages-ruleset bypass question is open.
- Options:
  1. **Consume the prior seed's credential** — this change only uses it.
     Gain: one owner for the credential; no duplicate creation.
     Cost: a dependency on an unimplemented seed and its open ruleset
     decision.
  2. **Create the credential here** — duplicate the prior seed.
     Gain: one change end to end.
     Cost: two owners for the same credential; conflicts with the prior
     topic's boundary.
- Chosen: **Consume** because the prior seed owns the credential. Rejected:
  **Create here** because it duplicates ownership.

### `git describe` version stamp

- Context: the site publishes from `main`, where no release tag necessarily
  exists; the footer must name a resolvable revision.
- Options:
  1. **`git describe`** — `v0.5.0` on a tagged revision,
     `v0.5.0-3-gabc1234` otherwise, linked to the commit.
     Gain: one resolvable string for every build.
     Cost: the `-N-gSHA` form is less readable.
  2. **Latest release tag only** — readable, but can name a revision that did
     not build the page.
     Gain: readability.
     Cost: false provenance.
  3. **SHA only** — exact, but no release relationship.
     Gain: precision.
     Cost: less useful to a reader.
- Chosen: **`git describe`** because it is resolvable and names the nearest
  tag. Rejected: **Latest tag** because it can misstate the build; **SHA only**
  because it loses the release relationship.

### Path-filtered `main` build trigger

- Context: building on every push publishes on documentation-only changes;
  building only on tags leaves page fixes unreleased.
- Options:
  1. **Path-filtered `main`** — `crates/**`, `ffi/wasm/**`, `web/**`,
     `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`,
     `LICENSES/MPL-2.0.txt`, `.github/workflows/web.yml`.
     Gain: the site is current without needless publishes.
     Cost: a maintained path list; a missed path leaves the site stale.
  2. **Every push** — no path list.
     Gain: simplicity.
     Cost: publishes on docs-only changes.
  3. **Release tag only** — the site matches a release.
     Gain: release relationship.
     Cost: page fixes wait for a release.
- Chosen: **Path-filtered `main`** because it matches the supplied workflow
  and keeps the site current. Rejected: **Every push** because of needless
  publishes; **Release tag only** because of the delay.

### Evidence gate and whole-content replacement

- Context: the site's public claims are only true if the built output passes
  the conversion, browser, licensing and content checks; a partial publish
  would mix old and new content.
- Options:
  1. **Gate and whole-content replacement** — publish only after the
     conversion tests, the browser check (every request after load), the
     licensing/REUSE check and the K4 content check pass; replace the target's
     content as a whole, removing extra files; a failure leaves the previous
     site or a complete new page, never a mixed one.
     Gain: the published page is the gated one; no partial state.
     Cost: a larger gate; the deploy replaces the target's content.
  2. **Convert-and-publish without the browser/content gate** — faster.
     Gain: less gate work.
     Cost: a privacy or copy defect can become public.
  3. **Incremental file writes** — update changed files only.
     Gain: smaller writes.
     Cost: a hand edit survives; a partial state is possible.
- Chosen: **Gate and whole-content replacement** because the public claim is
  the product. Rejected: **No gate** because it publishes unverified claims;
  **Incremental writes** because they leave hand edits and partial states.

### One change seed for the site and its public copy

- Context: the profile, README, durable claims and release platform wording
  currently say there is no browser application. A site that ships before the
  copy is corrected leaves false public copy; a copy change before the site
  promises a page that does not exist.
- Options:
  1. **One seed** — the site publish and the copy correction share one
     change; the profile apply follows the live site; the README is a source
     revision; the durable claims and release wording are revised; the seed is
     not complete until all are true.
     Gain: no window of false public copy; one owner.
     Cost: a larger change touching the site, the profile and documents.
  2. **Separate copy seed after the site** — smaller site change, but a window
     of false copy.
     Gain: smaller change.
     Cost: the profile denies a working page.
  3. **Separate topic** — the copy can persist false.
     Gain: narrow topic.
     Cost: no owner; false copy can remain.
- Chosen: **One seed** because the public state must be true end to end.
  Rejected: **Separate seed** because of the false window; **Separate topic**
  because it leaves the copy unowned.

### The site is a user-receiving path

- Context: the change log announces only revisions that changed a
  user-receiving path. The site reaches users but `web/**` and `ffi/wasm/**`
  are not on the marked list.
- Options:
  1. **Add the site paths and the capability** — add `web/**` and
     `ffi/wasm/**` to the user-receiving path list in `cliff.toml`, and add
     `publish-public-website` to the capability allow-list.
     Gain: a site change is announced; the allow-list matches the corpus.
     Cost: the site paths join the maintained path list.
  2. **Leave the list unchanged** — the browser release may be filtered out.
     Gain: no config change.
     Cost: a user-facing change is not announced.
- Chosen: **Add the paths and the capability** because the site is a
  user-receiving surface. Rejected: **Leave unchanged** because it hides the
  release.

### Redirect and custom domain out of scope

- Context: the canonical address is `https://duty2cal.github.io/`; a redirect
  would need a second Pages site and a custom domain would need DNS/TLS.
- Options:
  1. **Out of scope** — link the canonical address from the README and
     profile.
     Gain: one site, one publish path.
     Cost: old project-site links do not redirect.
  2. **Project-site redirect** — add a second Pages site.
     Gain: old links work.
     Cost: a second Pages site and publish path.
  3. **Custom domain** — DNS/TLS and a `cname`.
     Gain: a project-owned address.
     Cost: a separate identity decision and budget.
- Chosen: **Out of scope** because neither is needed for the browser route.
  Rejected: **Redirect** because it adds a second site; **Custom domain**
  because it is a separate identity commitment.

## Open questions

### May the pages ruleset bypass its publish deploy key? (prior topic's Q2)

The prior `publish-site-plumbing` seed carries this open decision. The site
publish cannot use the deploy key until it closes. It is not reopened here.

### Does review gate the configuration apply? (prior topic's Q10)

The profile copy is applied through `apply-configuration-change`; the prior
topic's review-gating decision still applies.
