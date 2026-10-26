# Proposal: publish-public-website

## Release note

The Public website at `https://duty2cal.github.io/` now serves the browser converter built from a named Built revision, states the browser and executable routes with the source, licence and privacy terms before it asks for a file, and is published only after the page-content, licensing, browser and conversion evidence passes.

## Use-Case Context

- Use-case ID: `publish-public-website` — **new permanent leaf** ([the contract](../../ideation/public-website/use-cases/publish-public-website.md), level `!`, primary Actor *Release maintainer*). The same change carries a delta under two existing leaves: `apply-configuration-change` (the corrected organization profile copy, applied after the site is live) and `publish-public-release` (the platform wording and the release-workflow site gate).
- Ideation topic: `public-website` — the topic whose Frame confirmed a supported browser conversion route (its question Q1, answered by the user on 2026-10-19).
- Seed path: `givn/ideation/public-website/changes/publish-public-website.md`. Its status line now reads `Status: proposed by publish-public-website`; no other seed text changed.
- Confirmed Personas, recorded as review lenses only (no biographies copied):
  - `project-maintainer--release-owner` — primary lens; judges the published claims, the Built revision and the release coupling.
  - `contributor--web-reviewer` — judges the CSP, the no-network proof, the built output and the gate; holds no publishing authority.
  - `visitor--project-reader` — judges whether the page explains itself and reaches the source and the release before it asks for a file.
  - `crew-member--personal-calendar-owner` — the browser route and the file it receives through the published page.
  - `project-maintainer--organization-owner` — supporting lens for the deploy key/secret prerequisite and the profile-copy apply in the same change seed; not an Actor of this use case.

The new leaf's goal is the Release maintainer's: *Make the Public website at `https://duty2cal.github.io/` serve the Site output built from a named Built revision* — the goal, guarantees, rules, Actors and Personas of the ideation contract are preserved in the change's use-case document. The browser conversion itself was delivered by the archived `build-browser-converter` change; this change publishes that page and makes the public copy true. The deploy key and its `PAGES_DEPLOY_KEY` secret are consumed, not created: the archived `publish-site-plumbing` change applied them live, so the credential is a prerequisite.

One structural correction to a permanent document is made in the same pass. The permanent `givn/specs/export-duty-plan-calendar/usecase.md` already failed `givn lint` at `HEAD` with a **blocking** `UCDOC` finding — the paragraph that explains the interaction inventory sat after the `## Interactions` table, and the use-case parser accepts that section only as a table. The paragraph is moved below the `## Diagram` mermaid block, where the parser accepts prose, with every word preserved. This changes no behaviour, no rule, no scenario and no wording; it restores a document the lint gate can parse, so the archive merge of this change's inventory is not blocked by an unrelated file.

## Problem / Opportunity

The browser converter exists and is only reachable from a local server. `https://duty2cal.github.io/` serves a stock placeholder from `duty2cal/duty2cal.github.io`, and the monorepo has no path that publishes into it: no workflow, no gate, no Built revision on the page. A colleague who is sent the URL learns nothing about the tool, and a crew member on macOS, Linux or a managed machine has no reachable route unless they build the source.

The public copy makes the gap worse in the other direction. `org/profile/README.md` says the website "does not host the converter today", and the durable claims — chapter 1's truthful-copy goal, quality scenario QS-022, risk R-032 — require the public copy to say there is **no** browser application. Those statements are true only while the page is unpublished.

Two consequences follow. First, the page cannot carry the statements its own contract requires until it is published: what it does, both conversion routes, the source, the stable latest-release address, the licence, the privacy rule, the non-affiliation statement, the as-is statement, and a footer naming and linking the revision it was built from. Second, the release pipeline is independent of the site, so a version can become public while the page is stale or names a different revision — the state the user asked to remove on 2026-10-19.

## Proposed Solution

- **The page is published from the monorepo, never edited in the target.** The Release maintainer publishes the site from a source revision; the page is built only from the repository into the Site output — the page, its script and the WASM bundle, plus the licence file — and that output replaces the hosting target's content as a whole. A file somebody added to the hosting target by hand is removed by the next publication and is never a source.
- **The page states the project's public terms before it asks for a file.** The published page states what the tool does, both conversion routes, the source, the stable latest-release address, the licence statement, the privacy rule, the non-affiliation statement and the as-is statement above the file picker.
- **The footer names the Built revision.** The footer names the revision the page was built from and links it, using the nearest release tag where one exists (`v0.5.0`) and the tag-plus-distance form otherwise (`v0.5.0-3-gabc1234`). A revision that cannot be resolved, or a working tree that is not clean at that revision, is refused rather than published.
- **Only evidence-gated content becomes public.** A Site publication happens only after the page-content check, the licensing/REUSE check, the browser check (the page sends no request to another origin after it loads) and the conversion evidence pass. A failed check is named, and the previous page stays in place.
- **The site is republished on every relevant revision and on every release.** A default-branch revision that changes the site's inputs (`crates/**`, `ffi/wasm/**`, `web/**`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `LICENSES/MPL-2.0.txt`, or the site's workflow) republishes the page; a revision that changes none of them starts no Site publication. The publication command makes that decision against the previous default-branch revision, so the same rule is exercised when a maintainer runs the publication by hand. Every `v*` release tag republishes the site from the tagged revision — the tree is built from that revision even when the default branch is newer — so the footer's tag and the release agree.
- **A release publishes its archive only after the site deploy for its tag succeeded.** A failed site publication leaves the previous page and publishes no release archive.
- **The public copy is corrected in the same change.** The organization profile states that a current browser converts a plan on the Public website, the root README states the browser route beside the one downloadable archive platform, and the durable claims stop describing the page as unpublished. The profile copy is applied through `apply-configuration-change` after the site is live.
- **The site is a user-receiving path.** `web/**` and `ffi/wasm/**` join the user-receiving path list and `publish-public-website` joins the capability allow-list, so a site change is announced in a release's notes.

## Capability Routing

`givn spec route --change publish-public-website` reported **no signal** — the top candidates tie at 13.00, and no ranked list is printed without signal. No recommendation column is recorded; each decision below carries its own rationale. Every decision is the result of the placement test: which actor goal does completing the capability advance? Channel is never an answer.

| Proposed capability | Delta file | Decision | Rationale |
|---|---|---|---|
| `publish-public-website` | `specs/publish-public-website/publish-public-website.feature` | `NEW in publish-public-website` | The goal — make the Public website serve the Site output built from a named Built revision — advances the Release maintainer's goal for the published page. No existing leaf owns it: the conversion leaf owns the calendar the crew member obtains, the release leaf owns the Windows archive, and the configuration leaf owns live GitHub settings. The capability is goal-shaped, not channel- or component-shaped, and it is the ideation contract's own new leaf. |
| `publish-organization-profile` | `specs/apply-configuration-change/publish-organization-profile.feature` | `EXTEND publish-organization-profile` | The profile content assertion is extended, not replaced: the published profile must now state the browser route and link the Public website, which reverses the permanent "no claim that the converter runs in a browser" step. The owner stays `apply-configuration-change`; the goal (make the live configuration match the declared revision) is unchanged, and the interaction inventory gains no row because no new consumer action appears. |
| `publish-public-release` | `specs/publish-public-release/publish-public-release.feature` | `EXTEND publish-public-release` | Two obligations of the release goal change and no new goal appears: the platform wording must distinguish the browser route from the downloadable archive platform, and a version's archive is published only after the site publication for its tag succeeded. The release's own interface (the preparation the maintainer runs) is unchanged, so the inventory gains no row. |

The delta files are therefore `specs/publish-public-website/publish-public-website.feature`, `specs/apply-configuration-change/publish-organization-profile.feature`, and `specs/publish-public-release/publish-public-release.feature`.

## Out of Scope

- **Creating, rotating or removing the deploy key or its `PAGES_DEPLOY_KEY` secret.** The archived `publish-site-plumbing` change created them, its proposal D1 closed the pages rule-set bypass, and its evidence records the live apply. This change consumes the credential.
- **The Pages source and hosting settings.** The Pages source stays `main` at `/` with `build_type = "legacy"`; no declared object under `infra/` changes, and no configuration change is run for the hosting target.
- **Changing the conversion contract.** No conversion rule, activity meaning, supported source family, 20 MiB bound or diagnostic format changes; the page's offer-and-save behaviour is inherited unchanged.
- **A hosted converter, an upload endpoint, an account, analytics or telemetry.** The published page has no backend, and the publication command never runs in the visitor's browser.
- **The Windows release archive contract.** Its platform, name, format, checksum, provenance, bill of materials and immutability are unchanged; the change log's grouping and filtering rules are unchanged apart from adding the site's paths and capability.
- **A custom domain and the project-site redirect.** Q6 keeps both out of scope; the canonical address is `https://duty2cal.github.io/`.
- **PWA/offline use, localization, native applications, calendar-client import and reconciliation.**
- **A review gate in the apply flow.** The profile apply is reviewed through the givn workflow and run by the operator after GREEN, as the prior infrastructure changes record; prior-topic question Q10 stays open in its own topic log.
- **Repairing a published page in place after a serving failure.** A later publication replaces the content as a whole; the project command does not drive the host's serving.
- **Any product behaviour in the converter.** The executable and the page never contact GitHub; only the project's publication automation does.

## Open Questions

Every question below is carried to `design.md`'s Decisions ledger, where each one is resolved with its rejected alternatives. No question needs the operator's authority: the public commitments (the browser route, both conversion routes on the page, the licence and privacy terms, the release coupling) were confirmed in ideation, and each remaining choice is a technical mechanism.

### D1 — How is the Site publication executed and gated so that it runs locally without GitHub?

- Context: the target is a different repository writable only with the deploy key; the gate must refuse unverified content; and the whole path must be runnable and testable on a workstation, because a scenario may not depend on a live third-party service.
- Question: should the site be published by one project-owned command whose evidence checks run as real subprocesses, by a reusable GitHub workflow whose jobs gate each other, or by a third-party deploy action configured in the workflow?
- Options:
  1. **One project-owned publication command with an extracted gate.**
     Meaning: a command decides whether the named revision is a site input by comparing a supplied base revision, builds the Site output from that revision, runs the page-content, licensing, browser and conversion checks as real subprocesses, then replaces the target's content in one fast-forward commit.
     In practice: the maintainer runs the same command against a local bare repository and gets the same refusals and the same commit shape as CI; a documentation-only revision prints that nothing changed and writes nothing.
     Gain: every promise is executable locally, the workflow becomes a thin trigger, and no write logic or path list lives in YAML.
     Cost: the command owns more than a workflow step normally does, the CI job must install the full toolchain, and every default-branch push starts a runner that may exit after the comparison.
  2. **A reusable GitHub workflow with job dependencies.**
     Meaning: the site build and deploy is a workflow both triggers call; the release workflow's job depends on it.
     In practice: the gate exists only as job ordering on GitHub, so nothing about the refusal or the replacement can be exercised locally.
     Gain: the smallest amount of new project code.
     Cost: the gate and the replacement are untestable without pushing to GitHub, and YAML ordering is the only enforcement.
  3. **A third-party deploy action in the workflow.**
     Meaning: an off-the-shelf action pushes a directory into the Pages repository.
     In practice: the push behaviour belongs to a dependency the project does not test.
     Gain: little project code.
     Cost: a third-party action in the write path of the public site, with behaviour, pinning and failure reporting outside the project's control.
- Disposition: carried — owner: `design.md` (Decisions ledger D1) decides the command, the extracted gate, the `--since` trigger decision, the path-list ownership and the local twin.

### D2 — How is the release coupled to the site deploy as testable subprocess behaviour?

- Context: the user requires every release to deploy the site (seed, amended Q8), and the coupling must be provable without inspecting workflow text or reaching live GitHub.
- Question: should the release's publication step run the site publication itself and then publish the archive, should the release job merely depend on a site job, or should the release stay independent and rely on review?
- Options:
  1. **The release publication command performs the site publication first.**
     Meaning: one command publishes the site for the tagged revision and, only if that succeeded, publishes the prepared archive.
     In practice: a failed site publication leaves the previous page and no archive exists afterwards; the test supplies a local Pages target and a recording stand-in for the release host.
     Gain: the ordering is executable behaviour a scenario can observe.
     Cost: the release host's calls move into project code and need a stand-in in the tests.
  2. **A release job that depends on a site job.**
     Meaning: the host's job graph enforces the order.
     In practice: the coupling is visible only in a tagged run, and nothing else can observe it.
     Gain: no project code for the ordering.
     Cost: the promise is unprovable locally and a mis-wired job silently decouples the release.
  3. **Keep the release independent.**
     Meaning: the site deploy runs beside the release and may fail.
     In practice: a version can be public while the page is stale or names another revision.
     Gain: a site failure never blocks a release.
     Cost: exactly the state the user's 2026-10-19 decision removed.
- Disposition: carried — owner: `design.md` (Decisions ledger D2) decides the ordering command and the stand-in for the release host.

### D3 — How does the footer obtain and name the Built revision?

- Context: the site publishes from a default-branch revision that may be newer than the last tag, and from release tags; the footer must name a revision the repository can resolve in both cases; the page's policy forbids a request to another origin after load.
- Question: should the Built revision be resolved and stamped when the Site output is built, fetched by the page at load time, or maintained as a written value in the page source?
- Options:
  1. **Resolved and stamped at build time.**
     Meaning: the build checks out the revision it is building into a detached worktree, resolves the nearest tag and the commit there, and writes both into the assembled page.
     In practice: a page built three revisions after `v0.5.0` names `v0.5.0-3-gabc1234` and links the commit, and a tag publication builds the tagged tree even when the default branch is newer.
     Gain: the string and the assembled content are produced from the same revision, so a published page cannot name a revision that did not build it.
     Cost: the assembled page differs per revision, so it is not committed, and the publication needs a scratch checkout it must clean up.
  2. **Fetched by the page at load time.**
     Meaning: the page asks the hosting service which revision it serves.
     In practice: the request would be blocked by the page's own policy, and the claim would depend on a service being reachable.
     Gain: no build-time step.
     Cost: it breaks the no-request-after-load promise and cannot work under `connect-src 'none'`.
  3. **A hand-written value in the page source.**
     Meaning: the revision is typed into the page and updated by hand.
     In practice: the value goes stale silently and the footer states false provenance.
     Gain: nothing to implement.
     Cost: the claim is unverifiable, which is the failure the release owner's lens exists to prevent.
- Disposition: carried — owner: `design.md` (Decisions ledger D3) decides the resolution, the detached-worktree build and the refusal rules.

### D4 — What is the Site output as an artifact, and how do the checks and the publication read it?

- Context: the page source is a source tree with ignored build directories and a licence file copied at build time; the publication needs one artifact to gate and to replace the target with, and the browser evidence must exercise that artifact rather than the source tree.
- Question: should the published artifact be an assembled output directory, the page source tree as it stands, or a tarball produced for the occasion?
- Options:
  1. **An assembled output directory.**
     Meaning: the build assembles one directory holding the page, its scripts, the WASM bundle, the licence file and the stamped footer.
     In practice: the content check, the browser check and the publication all read the same directory.
     Gain: one artifact to gate, publish and inspect; the source tree keeps source-only content.
     Cost: one more generated path, which must be ignored by version control.
  2. **The page source tree as it stands.**
     Meaning: publish the source directory including everything present.
     In practice: the ignored build output and the copied licence file would have to be present anyway, and the source and the artifact stay the same thing.
     Gain: no assembly step.
     Cost: nothing distinguishes the published artifact from editable source, and the footer cannot be stamped without dirtying source files.
  3. **A tarball produced for the occasion.**
     Meaning: the artifact is an archive that the target unpacks.
     In practice: the target is a git repository, so an archive is an extra layer to unpack and re-commit.
     Gain: a single file to move.
     Cost: the archive adds a format the target does not need and a second thing to verify.
- Disposition: carried — owner: `design.md` (Decisions ledger D4) decides the assembled output's shape and path.

### D5 — Where does the page-content check live?

- Context: K4 requires the page to state its terms before it asks for a file; the statement set is asserted by the browser scenario, but the publication also needs to refuse a built page that is missing one, with the missing statement named.
- Question: should the page-content check be a project-owned command over the built artifact, the browser scenario's assertions alone, or a copy of the statement list in the workflow?
- Options:
  1. **A project-owned command over the built artifact.**
     Meaning: a command reads the assembled output and names each missing statement.
     In practice: the publication refuses a page without a privacy statement and names that statement.
     Gain: the refusal the use case promises is executable, and its input is the artifact that would be published.
     Cost: the statement list is maintained in one more place, next to the page it checks.
  2. **The browser scenario's assertions alone.**
     Meaning: the visited page is the only check.
     In practice: the browser scenario asserts the rendered page, but the publication command itself cannot refuse a page missing a statement.
     Gain: one place to assert the page.
     Cost: the gate cannot name a missing statement, and a publication could not refuse on it.
  3. **A statement list in the workflow.**
     Meaning: the workflow greps the built output.
     In practice: the check exists only on the host, so the refusal is untestable locally.
     Gain: no new command.
     Cost: YAML-held logic, which is the failure this change's real-interface rule excludes.
- Disposition: carried — owner: `design.md` (Decisions ledger D5) decides the check's input, its named failures and its place in the gate's order.

### D6 — In what order does the change land the site and the corrected public copy?

- Context: Q7 confirmed one seed for the site and its public copy so that no window exists in which the public copy promises a page that is absent, or denies a page that works. The profile copy reaches GitHub only through an apply the operator runs; the README is an ordinary source revision.
- Question: should the site be published before the profile copy is applied, after it, or in the same step?
- Options:
  1. **Site first, then the README revision, then the profile apply.**
     Meaning: the page is live before any copy claims it, and the change's evidence records the live page before the apply.
     In practice: a failed profile apply leaves a working page and a stale profile; the failure is named by the profile change and the change is not complete.
     Gain: the false window Q7 rejected cannot occur in either direction.
     Cost: the change completes in two operator steps with a gap between them.
  2. **Profile copy first.**
     Meaning: the copy is corrected, then the site is published.
     In practice: between the two, the profile promises a page that returns nothing.
     Gain: one ordering decision.
     Cost: the window Q7 rejected, now in the more damaging direction.
  3. **Both in one step.**
     Meaning: the site publication and the apply are one operation.
     In practice: the publication is a workflow and the apply is a workstation command with a passphrase the workflow does not hold.
     Gain: no gap.
     Cost: it would put the state passphrase in CI, which the state-encryption decision forbids.
- Disposition: carried — owner: `design.md` (Decisions ledger D6) decides the ordering, the evidence that the site is live, and where the README revision sits.

### Seed and ideation question disposition

Every intersecting question is dispositioned. A disposition names its source; "resolved in `design.md`" alone is never used.

| Question | Disposition |
|---|---|
| Q1 — what the website promises | **answered** — a supported browser route for current evergreen browsers, with Windows x86_64 staying the only downloadable archive platform and the public copy stating both; source: user confirmation on 2026-10-19 (Q1, option 1). Realized by the published page and the corrected public copy. |
| Q2 — which Persona lenses review this topic | **answered** — reuse the crew member and the release owner, add the web reviewer and the non-converting reader, keep the organization owner as a supporting lens; source: user confirmation on 2026-10-19 (Q2, option 1). Recorded in Use-Case Context; the Persona files were promoted by Q9. |
| Q3 — which browsers and plan sizes are supported | **answered** — capability-based evergreen support (WebAssembly and ES modules) and plans up to 20 MiB; source: agent under the user's "go ahead" authority on 2026-10-19 (Q3, option 1). Realized by the page that was built by `build-browser-converter`; this change publishes it unchanged and keeps Chromium in CI with Firefox/Safari as release checks. |
| Q4 — how the browser preserves output and diagnostic guarantees | **answered** — browser-owned save, the page offering `<source-stem>.ics` and `<source-stem>.parse-error.txt` and writing nothing; source: agent under the user's "go ahead" authority on 2026-10-19 (Q4, option 1). Already realized; this change makes the page public and does not change the offer. |
| Q5 — which version a site built from `main` shows | **answered** — `git describe` output linked to the commit; source: agent under the user's "go ahead" authority on 2026-10-19 (Q5, option 1). Realized by the footer's Built revision (D3). |
| Q6 — the project-site redirect and a custom domain | **out of scope** — the canonical address is `https://duty2cal.github.io/`; a redirect needs a second Pages site the declaration does not enable, and a custom domain is a separate DNS/TLS identity commitment; source: closed 2026-10-19 (Q6, option 1). Recorded in Out of Scope. |
| Q7 — which public-copy changes accompany the site | **answered** — one change seed covering the site, the profile copy, the README revision and the durable claims, with the profile applied after the site is live; source: agent under the user's "go ahead" authority on 2026-10-19 (Q7, option 1), confirmed by the seed's accepted decision. The ordering is D6. |
| Q8 — which source changes trigger a site build | **answered** — a default-branch revision that changes the site's inputs **plus every release tag**; source: the user's 2026-10-19 amendment to Q8 (accepted decision: both triggers, one build-and-deploy implementation, the release gated on the site deploy). Realized by D1/D2 and the release delta; the path list is evaluated by the publication command (`--since`) rather than by the workflow, which the user's decision does not constrain. |
| Q9 — should the Handoff records be published and promoted | **answered** — the record and both seeds were published, the five terms and both new Personas promoted; source: user confirmation on 2026-10-19 (Q9, option 1). This change cites the promoted terms and Personas and adds no term. |
| Prior topic Q2 — may the pages rule set bypass its publish deploy key | **answered** — yes: the `pages protection` rule set carries deletion, non-fast-forward and linear-history rules and names the `DeployKey` actor type as an `always` bypass; source: the archived `publish-site-plumbing` proposal D1 (user decision 2026-10-19, applied live). Consumed here as a prerequisite; not reopened. |
| Prior topic Q10 — does review gate the configuration apply | **out of scope** — this change adds no approval record, gate or bypass to the apply flow; the declaration is reviewed through the givn workflow and the operator applies it after GREEN, as the prior infrastructure changes recorded. The question stays open in `givn/ideation/public-home-infrastructure/questions.md` and is decided by a change that models review in the apply flow. |
