# Change seed: publish-public-website

Status: proposed by publish-public-website

- Use-case IDs: `publish-public-website`, `apply-configuration-change`
  (profile copy), `publish-public-release` (platform wording and the
  release-workflow site gate)
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
  key and `PAGES_DEPLOY_KEY` secret. That seed is now the
  `publish-site-plumbing` change; its proposal D1 closes the ruleset bypass,
  and the credential is not yet applied.
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

- Context: the site publishes from path-filtered `main`, where no release tag
  necessarily exists, and from release tags; the footer must name a resolvable
  revision in both cases.
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

### Path-filtered `main` plus release-tag build trigger

- Context: the site publishes from `main`, where no release tag necessarily
  exists. The user's decision of 2026-10-19 requires every release to deploy
  the site as well. A main-only publish leaves the release independent of the
  site and cannot stamp a tag created after the main build; a tag-only publish
  would delay page fixes. The two triggers serve different purposes and both
  are required.
- Options:
  1. **Both path-filtered `main` and release tag** — the site builds on a
     default-branch push that changes the site's inputs for freshness, and on
     every `v*` tag from the tagged revision for the release stamp and release
     consistency. One reusable build-and-deploy workflow serves both callers;
     the release workflow publishes the archive only after the site deploy
     succeeds; a concurrency group serializes site deploys.
     In practice: a `web/` fix on `main` publishes immediately; tagging
     `v0.5.0` republishes the site from the tag, the footer shows `v0.5.0`,
     and the release is published only after the site is live.
     Gain: the site is current between releases, every release lands with a
     tag-stamped site, and a site failure blocks the release instead of
     leaving the two out of step.
     Cost: one extra site build and deploy per release; a tag on an older
     revision republishes that revision's site, so the maintainer tags the
     intended site; main and tag deploys must be serialized.
  2. **Release tag only** — the site builds only when a version is tagged.
     In practice: a page fix stays unpublished until the next tag.
     Gain: one deploy per release; the site always matches a release.
     Cost: page fixes wait; the site is stale between releases; this reverses
     the accepted main-deploy decision.
  3. **Path-filtered `main` plus a release gate, no tag deploy** — the site
     builds on `main`; the release workflow refuses to publish when the live
     site is not current for the tag.
     In practice: a tag with a stale site blocks the release until someone
     redeploys the site.
     Gain: no duplicate deploy.
     Cost: the release does not deploy the site; a pre-tag main build cannot
     show the tag in the footer; a site outage blocks the release.
- Chosen: **Both path-filtered `main` and release tag** because the user
  requires every release to deploy the site without giving up page-fix
  freshness. Rejected: **Release tag only** because page fixes would wait for
  a release; **main plus a release gate** because the release would not deploy
  the site and the footer would not name the tag.

### The release workflow gates on the site deploy

- Context: the user requires every release to deploy the website. The release
  workflow already publishes the Windows executable after its evidence gate;
  the site publication is a separate use case.
- Options:
  1. **Release gates on the site publication** — the release workflow calls
     the reusable site build-and-deploy workflow and publishes the executable
     only after it succeeds.
     In practice: a failed site build or deploy leaves the previous site in
     place and stops the release; no archive is published.
     Gain: the release and the site land together; a site failure cannot hide
     behind a published release.
     Cost: a site failure blocks the release; the release critical path grows
     by the site build and evidence gate.
  2. **Release proceeds independently** — the release publishes the archive;
     the site deploy runs separately and may fail.
     Gain: a site failure does not block the release.
     Cost: the release can land while the site is stale or shows the wrong
     revision, which is the state the user asked to remove.
- Chosen: **Release gates on the site publication** because the user wants
  every release to deploy the site. Rejected: **Release proceeds
  independently** because it leaves the release and the site out of step.

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

Closed by the `publish-site-plumbing` change's proposal D1 on 2026-10-19: the
pages rule set carries deletion, non-fast-forward and linear-history rules and
names the `DeployKey` actor type as an `always` bypass. The site publish
consumes the credential once that change is applied; the ruleset decision is
not reopened here.

### Does review gate the configuration apply? (prior topic's Q10)

The profile copy is applied through `apply-configuration-change`; the prior
topic's review-gating decision still applies.
