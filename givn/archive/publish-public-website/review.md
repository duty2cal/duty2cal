# Review: publish-public-website

Reviewed 2026-10-26 against `proposal.md`, `specs/**`, `design.md`,
`design-review.md`, `arc42.md`, `tasks.md`, the implementation tree, the
committed evidence and the archive-stage instructions. The change is at stage
`tasks-done`; every task box is checked.

## Findings

The first pass raised four findings. All four are resolved; no finding remains.

| Finding | First-pass defect | Resolution | Independent verification |
|---|---|---|---|
| F1 | `run-tests.sh` did not assemble `dist/site` or export `DUTY2CAL_SITE_ROOT`, and the driver fell back to `web/`, contradicting `design.md:135-136` | `a5419b5` adds the assembly/export (`run-tests.sh:31-45`) and requires `DUTY2CAL_SITE_ROOT` with a fail-closed root check (`web/e2e/driver.mjs:17-31,81-93`) | The change verification at `cf14afa` ran 216/216 scenarios, exit 0 (receipt below), which includes the ordinary export `@e2e`; the scoped export e2e is recorded green (`1 scenario`, `12 steps`, exit 0). The runner and driver now match the unchanged design rows. |
| F2 | Arc42 chapters 2 and 7 omitted `docs/assets/**` and `.github/workflows/site.yml` | `a5419b5` adds both to `02-architecture-constraints.md:27` and `07-deployment-view.md:86` | Re-read both chapters: the lists now hold all ten entries; every Arc42 row matches. |
| F3 | T18 kept a stale "Unreconciled" note that `site.yml`/`web.yml` did not exist | `a5419b5` replaces it with the T22b (`6b9e4c5`) resolution | Re-read T18: it now names T22b, and the files/group exist. |
| F4 | The proposal's three-column Capability Routing table was not parsed | `a5419b5` changes it to four columns (capability, delta file, decision, rationale) | The routing parser reads all three decisions; the review gate no longer reports the routing warning. |

### F1 resolution detail

- `design.md:135-136` (unchanged) says `run-tests.sh` builds the Site output and
  exports `DUTY2CAL_SITE_ROOT`, and the driver serves it and fails closed. The
  implementation now conforms: `run-tests.sh:31-45` calls
  `scripts/build-site.sh --repo . --revision "$(git describe --tags --always
  --match 'v*')" --out dist/site` and exports `DUTY2CAL_SITE_ROOT` unless the
  caller already set it; `web/e2e/driver.mjs:17-31` throws without the variable,
  and `requireServedRoot` refuses a missing or non-directory root. No `WEB_ROOT`
  fallback remains. This is the design-conformant repair, so no `design.md`
  update or design-review re-run was required.

### F2 resolution detail

- `docs/arc42/02-architecture-constraints.md:27` now lists `docs/assets/**`,
  `.github/workflows/web.yml` and `.github/workflows/site.yml`.
- `docs/arc42/07-deployment-view.md:86` now lists `docs/assets/**` and "the two
  site workflows"; both workflow files are named in the same section.
- `ARC42 CONFORMANCE: CLEAN` is recorded below because every row now matches.

### F3 resolution detail

- `tasks.md` T18 now reads "Later resolved: T22b (`6b9e4c5`) created
  `.github/workflows/site.yml` with the design's concurrency group and its
  `web.yml` caller"; the group is at `site.yml:27-29`.

### F4 resolution detail

- `proposal.md` `## Capability Routing` now has
  `| Proposed capability | Delta file | Decision | Rationale |`; the parser
  reads the decision from cell 3 and `givn check review` no longer prints the
  routing warning.

### Non-blocking documentation note

- `design.md:540` still says "Column 1 is the single path the integrity hook
  resolves" while column 1 is now the capability (`f229db6`). The promised-files
  hook resolves the paths from the table and passes, so this is a wording nit,
  not a coverage or conformance finding.

## Fabrication Audit (do this first)

### Step 0 — `@e2e` tag integrity

Every delta `.feature` file was grepped and its history inspected
(`git log -S'@e2e' -- givn/changes/publish-public-website/specs/`). The only
`@e2e` occurrence is on the new leaf's publication scenario
(`specs/publish-public-website/publish-public-website.feature:15`), introduced
with the feature in `b094056`; no scenario's tags were changed from including
`@e2e` to excluding it. The two extended capabilities add no `@e2e` scenario
because they add no distinct action (the inherited matrix rows below), and the
release delta's three added scenarios are variants/error cases of the existing
actions. Tag integrity: clean.

### Step 1 — Empty/trivial step bodies

Scan command (all step-definition files):

```
grep -rln '#\[given\|#\[when\|#\[then' apps/cli/tests/steps/   # 18 files
grep -rn 'unimplemented!\|todo!\|TODO\|FIXME' apps/cli/tests/steps/
grep -rnE '\)\s*\{\s*\}|=>\s*\{\s*\}|fn [a-z_]+\([^)]*\)\s*\{\s*\}' apps/cli/tests/steps/
```

Result: **0 empty step bodies found across 18 step-definition files** (9
regular and 9 `e2e`). The only `{}` matches are match arms
(`support.rs:173`, `e2e/export_duty_plan_calendar.rs:341`,
`enable_repository_security.rs:297`, `publish_organization_profile.rs:262`) and
a JavaScript string inside a fixture (`publish_public_website.rs:417`); no
substituted body panics with `unimplemented!()`/`todo!()` or is assertion-free.

### Step 2 — Checked task → commit cross-check

`tasks.md` carries 26 `COMMIT:` lines (the visual-review follow-up `1251699`
is an additional fix commit, recorded in T24's REFACTOR evidence). Every
scenario task's commit touches production source or the runner/CI deliverable;
none is a spec-only or stub-only diff. `git show --stat` for each:

| Task | Commit | Production / deliverable files touched |
|---|---|---|
| T00 setup | `b094056` | step skeletons, `steps/mod.rs`, `steps/e2e/mod.rs`, `support.rs`, `site_server.py` (the setup deliverable; not spec-only) |
| T01 | `ed51e5d` | `scripts/publish-site.sh`, `scripts/build-site.sh`, `web/index.html`, `web/styles.css` (+ steps) |
| T02 | `605f7be` | `scripts/publish-site.sh`, `build-site.sh`, `site-evidence.sh`, `check-site-content.sh`, `build-web.sh`, `run-web.sh`, `web/*`, `.gitignore`, `REUSE.toml`, `pii-guard.sh` |
| T03 | `7a22d40` | `scripts/publish-site.sh` |
| T04 | `84ac95a` | `scripts/publish-site.sh` (+ steps) |
| T05 | `4b4f8e5` | `scripts/publish-site.sh` |
| T06 | `58cecd3` | `scripts/publish-site.sh` |
| T07 | `bc5ab02` | `scripts/check-site-content.sh` |
| T08 | `391a59d` | `scripts/site-evidence.sh`, `web/e2e/driver.mjs` |
| T09 | `0effcaf` | `scripts/check-site-content.sh` |
| T10 | `22e731d` | `scripts/site-evidence.sh` |
| T11 | `7a00099` | `scripts/publish-site.sh` |
| T12 | `f23a944` | `scripts/publish-site.sh` |
| T13 | `699b9cc` | `scripts/site-evidence.sh` |
| T14 | `4680238` | `scripts/publish-site.sh` |
| T15 | `a2a79fe` | scenario steps + `support.rs` (the support helper is the fixture deliverable) |
| T16 | `13eda8a` | scenario steps + `support.rs` |
| T17 | `6920116` | scenario steps + `support.rs` (the production refusal path already existed; the scenario is its regression test — recorded in the task) |
| T18 | `94f7257` | scenario steps + `support.rs` (the `flock` already existed; the scenario proves it — recorded in the task) |
| T19 | `e13103b` | `cliff.toml`, `scripts/render-changelog.sh` |
| T20 | `2f0557c` | `README.md`, `org/profile/README.md`, `scripts/public-text-check.sh` |
| T21 | `f5309fb` | `scripts/publish-release.sh` |
| T22 | `ba98451` | `org/profile/README.md` (+ steps) |
| T22b | `6b9e4c5` | `.github/workflows/{site,web,release,ci}.yml` |
| T23 | `24d4797` | `run-tests.sh`, `steps/browser.rs`, `steps/support.rs` (E2E environment deliverable) |
| T24 | `e35aaa0` | `steps/e2e/publish_public_website.rs`, feature `@wip` removal, four committed screenshots |
| visual follow-up | `1251699` | `web/index.html`, e2e step, permanent export assertion, four re-captured screenshots |

Checks: every hash exists, is an ancestor of `HEAD`, and each scenario's
commit body names the scenario title (verified for all 26 by
`git show --format=%B`; `givn check review` also re-checks this). No fabricated
task found. The commits of T15–T18 that touch no script are the tasks whose
recorded production path already existed and whose deliverable is the regression
scenario; the task text says so explicitly.

### Step 3 — Promised components exist

| Component (from `design.md`) | Exists | Path |
|---|---|---|
| Site assembler | yes | `scripts/build-site.sh` |
| Page-content check | yes | `scripts/check-site-content.sh` |
| Evidence gate | yes | `scripts/site-evidence.sh` |
| Publication command | yes | `scripts/publish-site.sh` |
| Release command | yes | `scripts/publish-release.sh` |
| Bundle build (changed) | yes | `scripts/build-web.sh` |
| Local run (changed) | yes | `scripts/run-web.sh` |
| Runner (changed) | yes; conforms after `a5419b5` | `run-tests.sh` |
| Driver (changed) | yes; conforms after `a5419b5` | `web/e2e/driver.mjs` |
| Page + styles | yes | `web/index.html`, `web/styles.css` |
| Reusable workflow | yes | `.github/workflows/site.yml` |
| Trigger workflow | yes | `.github/workflows/web.yml` |
| Release workflow | yes | `.github/workflows/release.yml` |
| CI workflow | yes | `.github/workflows/ci.yml` |
| Change-log data | yes | `cliff.toml` |
| Public text | yes | `README.md`, `org/profile/README.md` |
| Design system + rendered reference | yes | `docs/design/design-system.md`, `docs/design/reference/converter-reference.html` |
| Licensing/guard changes | yes | `REUSE.toml`, `.gitignore`, `scripts/pii-guard.sh` |
| Shared step helpers | yes | `apps/cli/tests/steps/support.rs`, `steps/browser.rs`, `apps/cli/tests/support/site_server.py` |
| Step definition files | yes | `steps/publish_public_website.rs`, `steps/e2e/publish_public_website.rs`, extended `publish_public_release.rs`, `publish_organization_profile.rs` |
| Use cases + delta features | yes | `specs/publish-public-website/`, `specs/publish-public-release/`, `specs/apply-configuration-change/` |
| Icon asset | yes | `docs/assets/duty2cal.png` |

No promised component is missing.

### Step 4 — Strict-mode proof

`.fail_on_skipped()` is present in `apps/cli/tests/features_runner.rs:143`
followed by `.run_and_exit(root)`. `tasks.md` T00 records the deliberately
undefined site step exiting `101` with `1 scenario (1 failed)`; T23 records the
deliberately pending `@e2e` step exiting `101` with `1 scenario (1 failed)` and
then the implemented scenario at `14 steps (14 passed)`, exit 0. The
`run_revision_suite` gate also refuses a selection whose result file records
`total == 0` (`scripts/site-evidence.sh:79-105`) and the browser check requires
exactly one selected scenario. Strict-mode proof: present and passing.

### Step 5–6 — `@e2e` Then assertions and the real driver

Only the delta's one `@e2e` was authored by this change; the four inherited
matrix rows keep their permanent `@e2e` scenarios unchanged.

| `@e2e` scenario | Real-interface primary assertion | Driven through a real browser? | Downgraded? |
|---|---|---|---|
| Release maintainer publishes the site and the hosting target holds the built page | The publication is a real `std::process::Command` (`support.rs:987-1000`) whose stdout names the Built revision; the hosting target's tree and the **served address** are read back; the rendered page's header, both statement panels, footer revision/commit link, empty→ready state change and no-foreign-request log are asserted | Yes — `BrowserDriver::start_serving` (`steps/e2e/publish_public_website.rs:67`) with `DUTY2CAL_SITE_ROOT` pointing at the target checkout (`support.rs:914-930`); Chromium via Playwright, `page.setInputFiles` for the plan, `page.reload` for the second state | No |

No delta Then asserts only against a repository/database: the repository
assertions are secondary to the served address and the Chromium-rendered page.
The step implementation contains no `fetch`/`XMLHttpRequest` against the
application; the only direct HTTP read is `support::site_served_page`
(`curl` at `support.rs:1233-1239`) reading the address the command itself
promised to verify, which is the CLI capability's observable interface, not a
substitute for the browser interaction.

### Step 7 — Files `verify.e2e_command` invokes, and the parallel-implementation check

`givn/commands.yaml:32` is `verify.command: "./run-tests.sh"` and
`givn/commands.yaml:37` is `verify.e2e_command: "./run-tests.sh --e2e"`. The
script reads `--e2e` into `--tags "@e2e"` (`run-tests.sh:39-43`) and runs
`cargo test -p duty2cal --test features_runner`, which compiles every module
declared in `steps/e2e/mod.rs`; for this capability the invoked implementation
is `apps/cli/tests/steps/e2e/publish_public_website.rs` (shared with
`steps/publish_public_website.rs` for the flow steps).

Search for a parallel implementation: `find . -path ./target -prune -o
-iname '*publish*website*' -print` returns only the two step files, the delta
feature, the change/usecase documents and the ideation seed; `git status`
shows no untracked e2e file. No second, stronger implementation exists.

| Capability | File(s) `verify.e2e_command` invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| publish-public-website | `apps/cli/tests/steps/e2e/publish_public_website.rs` (with `steps/publish_public_website.rs` flow steps, `steps/browser.rs`, `support.rs`) | none | N/A — the only implementation is the real-subprocess + Chromium one |

### Step 8 — Normalized action scope

The owning use case's `## Interactions` has one entry; the matrix reproduces
the four inherited entries. Each normalized action has exactly one `@e2e`
scenario; variants and error cases are regular scenarios.

| Capability | Distinct action | `@e2e` scenarios | Over-produced? |
|---|---|---|---|
| publish-public-website | publish the site from a source revision | 1 | no |
| publish-public-release | prepare the release for a version | 1 (permanent) | no |
| publish-public-release | generate the change log for a version | 1 (permanent) | no |
| publish-organization-profile | preview the declared configuration | 1 (permanent) | no |
| publish-organization-profile | apply the previewed revision | 1 (permanent) | no |

The tag build, unchanged-revision no-op, unclean/unresolvable revision,
worktree failure, build failure, each evidence refusal, lockfile, unlicensed
asset, hand edit, refused push, serving mismatch and serialization are all
variants/error cases of the one publication action; none adds an entry. The
release delta's site-announcement, public-text and target-verification
scenarios are variants of the two existing release actions.

### Step 9 — E2E isolation

| `verify.command` | `verify.e2e_command` | Identical? | Regular scope count | E2E scope count | Strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | no | 216 | 14 | yes (14 < 216) |

The counts are the committed closeout evidence in `tasks.md` (full suite
`11 features`, `216 scenarios (216 passed)`, `1065 steps`, exit 0; `--e2e`
`9 features`, `14 scenarios (14 passed)`, `126 steps`, exit 0). The archive
receipt will record the per-scope counts for this candidate; review does not
re-run them.

### Step 10 — `design.md` conformance

| Decision named by `design.md` | `design.md` says | Actually built | Matches? | If not: design.md updated + design-review re-run? |
|---|---|---|---|---|
| Publication command | `scripts/publish-site.sh` with the documented flags and steps | present; flags, order, unclean check, `--since`, worktree, gate, flock, fast-forward, read-back, `--verify-url` match | yes | — |
| Site output | exactly eight members, icon from `docs/assets/duty2cal.png`, `LICENSE.txt` | `build-site.sh` copies exactly those eight; `SITE_MEMBERS` (`support.rs:1250`) lists the same eight | yes | — |
| Gate | `site-evidence.sh`: content, licensing, browser, conversion, fail-fast, result-file count | matches; `@site-publication` excluded from the conversion suite | yes | — |
| Serving check | required `--verify-url`, bounded retries, names address + served revision | matches (`publish-site.sh:verify_served`) | yes | — |
| Step files | one regular and one `e2e` file, e2e asserting the rendered page | both exist; the e2e's page assertions live in the e2e file; the publication `When` is shared flow glue bound in the regular file, which cucumber-rs binds globally | yes (shared glue noted) | — |
| Driver/framework | Playwright 1.64.1 + Chromium through the JSON-lines driver | `web/e2e/package-lock.json`, `web/e2e/driver.mjs` (Playwright, axe 4.14.0) | yes | — |
| `verify.e2e_command` | `./run-tests.sh --e2e` | same | yes | — |
| Runner behavior | `run-tests.sh` builds the Site output and exports `DUTY2CAL_SITE_ROOT` | `run-tests.sh:31-45` builds `dist/site` and exports the root unless the caller set it | yes | — |
| Driver root | serve `DUTY2CAL_SITE_ROOT` instead of `web/`; fail closed when absent | requires the variable (`driver.mjs:17-31`) and refuses a missing/non-directory root (`:81-93`) | yes | — |
| Site-input list | ten entries including `docs/assets/**`, `.github/workflows/site.yml` | matches | yes | — |
| Workflow layout | reusable `site.yml`, trigger `web.yml` (no `v*`), release `site` + `publish(needs: site)`, `fetch-depth: 0` | matches (verified in the four workflow files; `actionlint` recorded in T22b) | yes | — |
| Release coupling | `publish-release.sh` verifies the target's tagged Built revision before any `gh`, excludes `release-notes.md` | matches | yes | — |
| Icon/licence | header mark + tab icon `./icon.png`, MPL-2.0 `LICENSE.txt`, CC0-1.0 `docs/assets/**` override | matches (`web/index.html:9,70`, `REUSE.toml`) | yes | — |

### Step 11 — Interaction coverage cross-reference

Inventory source: `givn/changes/publish-public-website/specs/publish-public-website/usecase.md`
`## Interactions` (one row) plus the inherited rows in the two extended copies;
matrix: `design.md` `## Interaction Coverage Matrix` (five rows).

| Inventory entry | Matrix row exists? | `@e2e` scenario exists? | Promised driving mechanism | Actual driving mechanism | Match? |
|---|---|---|---|---|---|
| publish-public-website · publish the site from a source revision | yes | "Release maintainer publishes the site and the hosting target holds the built page" | CLI publication via `std::process::Command` against a local bare target, then Playwright/Chromium serving the target's checked-out tree | `support::site_publication_command`/`site_publish` (real subprocess); `BrowserDriver::start_serving(target-tree)`; Chromium navigation, `setInputFiles`, request log, screenshots | yes |
| publish-public-release · prepare the release for a version | yes (unchanged) | "Maintainer prepares a Windows release and inspects the archived executable" | real `scripts/prepare-release.sh` subprocess (+ `gh` stand-in only where the host is third-party) | `publish_public_release.rs:242` runs `scripts/prepare-release.sh`; `gh_stand_in` on `PATH` (`:836-862`) | yes |
| publish-public-release · generate the change log for a version | yes (unchanged) | "Maintainer generates the change log for a version and reads its newest section" | real `scripts/render-changelog.sh` subprocess | `publish_public_release.rs:521` / `:1718` | yes |
| publish-organization-profile · preview the declared configuration | yes (unchanged) | "Organization owner previews the profile declaration and only the three creations are named" | real `tofu` against the loopback fake GitHub API | `steps/e2e/publish_organization_profile.rs:188-225` (`tofu init`, targeted `plan`, `tofu show`) | yes |
| publish-organization-profile · apply the previewed revision | yes (unchanged) | "Organization owner applies the previewed revision and the organization profile is published" | real `tofu apply` against the fake API, then read the fake's repository/contents/journal | same e2e module, `tofu apply` + `fake_request`/journal assertions | yes |

No inventory entry lacks a matrix row, an `@e2e` scenario or its promised
mechanism; no matrix row lacks an inventory entry.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case goal, guarantees and rules preserved | The new leaf keeps the ideation contract's goal (one verb phrase: make the Public website serve the Site output built from a named Built revision), its minimal/success guarantees, K3/K4/K5, the statement panels, the icon rule, the trigger rule and the release coupling; the release copy adjusts only the platform wording and adds the site-before-archive order; the profile copy keeps the same capability and inventory | yes |
| Capability ownership and typed relationships preserved | `publish-public-website` owns the publication; the release waits for it and claims nothing about page content; the profile apply owns the copy; `Includes`/`Extends` stay `none`; the matrix maps only these | yes |
| Confirmed Persona-relevant outcomes preserved | Release owner: Built revision, tag coupling, site failure blocks release; web reviewer: CSP/no-network, artifact = gated output, one credential; visitor: terms before picker; crew member: unchanged conversion and offer; organization owner: credential consumed, profile apply after site live | yes |
| Actors remain distinct from Persona biographies | Gherkin names `Release maintainer`; Personas are review lenses in the use case and design, never scenario actors | yes |
| Migration ledger and coverage evidence complete, if applicable | No storage/schema migration; the publication writes only the target's git tree; coverage boundary recorded in `design.md` | yes |
| Use-case scope rejection criteria | Goal is one verb phrase; primary Actor named in `## Trigger`; one capability passes the placement test; mutation matches the changing goal; every extension names its main-flow step; preconditions are not flow-established and the trigger is an event; mechanism vocabulary is absent from Goal/Trigger/guarantees; no component/channel-shaped name; the leaf's main flow names one actor goal; no `Extends` misuse | yes |

Persona dispositions (all confirmed, none invented):

| Persona | Goal | Disposition |
|---|---|---|
| `project-maintainer--release-owner` | Publish the claims the page can back and couple the release to its revision | primary lens; satisfied by the footer/tag/release-gate scenarios and the visual review |
| `contributor--web-reviewer` | Read the CSP, no-network proof, built artifact and gate | satisfied by the request log, the gate's artifact and the lockfile guard |
| `visitor--project-reader` | Understand the page and reach source/release before a file is requested | satisfied by the "Before you choose a plan" panel and the source/licence panel |
| `crew-member--personal-calendar-owner` | Reach the browser route and receive the calendar | satisfied by the unchanged ready-state offer through the published page |
| `project-maintainer--organization-owner` | Know the credential is a prerequisite and the profile apply follows the live site | supporting lens; satisfied by the use-case prerequisite and the profile delta |

PERSONAS: project-maintainer--release-owner

## Deferred mandatory checks

None. The `gh` host is an explicit recording twin (a design decision, not a
deferred check), and the size-capability corpus trim in the site fixture
removes two `@e2e` feature files that need an `x86_64-pc-windows-gnu` target
library this host does not carry; the fixture keeps both use cases so the
change-log allow-list still names them, and the gate runs the revision's own
trimmed suite. That is fixture shaping, not a deferral of a mandatory check.
The change-log allow-list entry for `publish-public-website` is a recorded
post-archive obligation (`design.md` file table, T19): the renderer refuses an
allow-list naming a capability whose permanent use-case directory does not yet
exist, so the entry is added when the archive makes `givn/specs/` hold it, as
the earlier archived infrastructure changes record the same obligation. Counts:
0 `DEFERRAL:` lines, 0 `OPERATOR-DECISION:` lines.

## Visual Review

The design declares `Interface: web`, viewports mobile (360x780) and desktop
(1920x1080), and the states empty and ready. Evidence is the four committed,
full-page screenshots under
`evidence/visual/release-maintainer-publishes-the-site-and-the-hosting-target-holds-the-built-page/`;
each is non-empty and was opened in this review.

| `@e2e` scenario | Viewport | State | Evidence file | Present and non-empty? |
|---|---|---|---|---|
| Release maintainer publishes the site and the hosting target holds the built page | 360x780 | empty | `mobile--empty.png` (360x1401) | yes |
| same | 360x780 | ready (final) | `mobile.png` (360x1559) | yes |
| same | 1920x1080 | empty | `desktop--empty.png` (1920x1080) | yes |
| same | 1920x1080 | ready (final) | `desktop.png` (1920x1212) | yes |

States seen: the empty state ("No plan selected.") and the ready state ("Your
calendar is ready to save.", `Save roster.ics`) at both viewports, with the
header icon left of the heading, the "Before you choose a plan" panel above the
converter, the "Source and licence" panel below it, and the prose footer naming
and linking the Built revision (`v0.5.0-3-g4e50f0c`, the fixture's own
`git describe`).

Five fresh-context vision reviews, one per confirmed Persona, saw the four
committed full-page screenshots:

| Confirmed Persona | Goal | Finding | Disposition |
|---|---|---|---|
| project-maintainer--release-owner | Trace the page to its revision and trust the release coupling | PASS | no finding |
| contributor--web-reviewer | Read the CSP/no-network/artifact claims from the page | PASS | no finding |
| visitor--project-reader | Understand the page and reach source/release before the picker | PASS | no finding |
| crew-member--personal-calendar-owner | Reach the browser offer and the file through the published page | PASS | no finding |
| project-maintainer--organization-owner | See the credential prerequisite and the published profile claim | PASS | no finding |

Two findings were raised across those reviews and both are fixed in `1251699`:
the captures were scrolled (only the viewport, not the whole page) — fixed by
`fullPage: true`, re-captured; and the footer sentence differed from the
approved reference ("No account, email, or upload is used" vs "No account,
email or upload is needed") — fixed in `web/index.html` and in the permanent
export assertion, and the scoped `@e2e` re-ran green (`14 steps (14 passed)`,
exit 0). The advisory about link text ("release page", "licence text", "the
source") is dispositioned as the operator-approved reference wording of
2026-10-25; each link's accessible name is non-empty and the wording is the
approved rendered reference's.

Baseline: every control is labelled and keyboard-focusable, the page has a real
title and one visible `h1`, no duplicated visible text, `lang="en"`, reduced
motion computed at `0ms`, axe clean in both traversed states, and the
site-specific assertion confirms no request to another origin after load. No
internal identifier is visible on the published page; the `data-site-statement`
hooks are attributes, not text.

VISUAL-REVIEW: PASS

## Arc42 implementation conformance

The affected chapters were independently derived from the completed
implementation before `arc42.md` was accepted: 01 (requirements/stakeholders/
quality goal 8), 02 (constraints), 03 (external surface), 04 (solution
strategy), 05 (site toolbox), 06 (runtime flow), 07 (deployment), 08
(crosscutting rules), 09 (no ADR), 10 (quality scenarios), 11 (risks/debt), 12
(glossary). All twelve durable chapters exist and are substantive; no
placeholder or ASCII-art diagram was found (the site diagrams are Mermaid).

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 01 requirements/quality goal 8 | `01-introduction-and-goals.md:7,31,37-41` | Yes: goal 8 no longer describes the page as unpublished; personas named | Scope, persona impact | T20/T22 | `README.md`, `org/profile/README.md`, profile/public-text scenarios | yes |
| 02 publication constraint: Pages target, deploy key, whole-content commit, lock | `02-architecture-constraints.md:21-24` | Yes | D1, D2, D4 | T02, T16–T18, T22b | `publish-site.sh`, `site.yml`, `release.yml` | yes |
| 02 Site-input path list | `02-architecture-constraints.md:27` | Yes: single owner for the list | step 2; list of ten entries | T01, T11 | `publish-site.sh:38-40`; the chapter now lists all ten entries | yes |
| 02 stamping/unclean/lockfile facts | `02-architecture-constraints.md:25-26,28-31` | Yes | D3, D5, D6 | T03–T13, T23 | `publish-site.sh`, `build-site.sh`, `site-evidence.sh` | yes |
| 03 external surface/partners | `03-context-and-scope.md:19-45,61-64,107-108,128-168` | Yes | Scope/technology | T20, T22 | Page, publication, release, profile | yes |
| 04 published page from one gated command; release coupling; no ADR | `04-solution-strategy.md:11,28-29,37,49` | Yes | D1, D2, D7; ADR verdicts | T01–T24 | commands/workflows | yes |
| 05 site toolbox and eight-member output | `05-building-block-view.md:22-24,58-104,267` | Yes | Architecture impact, output table | T01–T24 | files exist; `build-site.sh` copies eight | yes |
| 06 site publication and release runtime scenarios | `06-runtime-view.md:171-270` | Yes | Steps 1–8; release coupling | T01–T24 | scripts match the flow (unclean → since → resolve → worktree → build → gate → replace → read back → serve) | yes |
| 07 deployment facts (address, target, credential, command, gate, serving check, trigger, coupling, twin) | `07-deployment-view.md:75-99,298` | Yes | D1–D7 | T01–T23 | workflows and commands | yes |
| 07 trigger enumeration | `07-deployment-view.md:86` | Yes: `--since` trigger | step 2 | T01, T02, T11 | `publish-site.sh:38-40`; the chapter now lists `docs/assets/**` and the two site workflows | yes |
| 08 crosscutting rules | `08-crosscutting-concepts.md:203-270` | Yes | Gates/D7 | T02–T13, T24 | scripts/steps | yes |
| 09 register: no ADR, four routings | `09-architecture-decisions.md:52-91` | Yes, four `NOT_QUALIFIED` verdicts | Decisions | — | No new ADR file; `docs/arc42/adr/` unchanged | yes |
| 10 QS-022/031/035–039 | `10-quality-requirements.md:75,84-92` | Yes | Traceability | T01–T23 | scenarios named | yes |
| 11 R-043/R-053 closed, R-055 corrected, R-057–R-066, TD-022–TD-024 | `11-risks-and-technical-debt.md:49-67,100-102` | Yes | Risks section | T02, T12, T18, T23 | mitigated by the named checks | yes |
| 12 glossary: no new term; four site terms realized; serving check is a step | `12-glossary.md:30-35,95,108,260-275` | Yes | Glossary references | T01–T22 | code symbols match (`publish-site.sh`, `build-site.sh`, `dist/site`, `site.yml`, footer attribute) | yes |

Sign-off sub-checks:

- [x] Every chapter marked affected by either assessment was opened and checked.
- [x] Every relevant Arc42 fact is represented in `design.md` and mapped to tasks.
- [x] The completed implementation follows the facts; the first-pass F1
      implementation deviation is repaired in `a5419b5`.
- [x] Implementation-discovered changes are recorded in the affected chapters;
      the first-pass F2 list omissions are fixed.
- [x] No mismatch remains; the F2 lists are complete and the F1 implementation
      now matches `design.md`.

ARC42 CONFORMANCE: CLEAN

Result: CLEAN. The two first-pass list omissions are fixed in `a5419b5`, and
the runner/driver implementation now matches the reviewed design.

## Ubiquitous language conformance

The durable glossary (`docs/arc42/12-glossary.md`) holds **Public website**,
**Site publication**, **Site output** and **Built revision**. The specs, design,
use case, code and evidence reuse those exact terms and their code symbols:

| Term | Spec/design use | Code symbol | Consistent? |
|---|---|---|---|
| Public website | the published page at `https://duty2cal.github.io/` | `web/index.html`, `duty2cal.github.io` | yes |
| Site publication | the gated build-and-publish process | `scripts/publish-site.sh`, `site.yml` | yes |
| Site output | the assembled eight-member artifact | `dist/site/` from `scripts/build-site.sh` | yes |
| Built revision | the revision named and linked in the footer | `git describe` in `build-site.sh`; `data-site-statement="built-revision"` | yes |

No new term is introduced; the glossary's promotion boundary says so and this
change's design confirms it. Anti-terms are respected (the archive is not the
Site output; the profile is not the Public website). No ubiquitous-language
finding remains.

## Coverage

No archive receipt exists for this candidate yet (the archive gate produces the
merged measured result). This classification uses the change verification
receipt (216/216 scenarios, exit 0, `cf14afa`), the committed per-task evidence,
the `@e2e` closeout in `tasks.md` (14/14, exit 0) and static inspection. The product's coverage
denominator is unchanged; the publication tooling is build tooling outside it,
as `design.md`'s coverage-process boundary records for `prepare-release.sh` and
`render-changelog.sh`, and the gate's `run-tests.sh` exercises the same
conversion path as the product rows.

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| Ordinary browser `@e2e` (`export-duty-plan-calendar`) served `web/` source instead of the assembled Site output (first-pass F1) | `run-tests.sh:31-45`, `web/e2e/driver.mjs:17-31` | 2 — missing test coverage | Resolved in `a5419b5`; the ordinary suite now assembles `dist/site` and the driver refuses to run without it. Final verification: 216/216 scenarios, exit 0 |
| Site publication tooling (`scripts/publish-site.sh`, `build-site.sh`, `site-evidence.sh`, `check-site-content.sh`, `publish-release.sh`) | whole files | 3 — outside the product denominator by the recorded coverage boundary | No product line is added; the same boundary as `prepare-release.sh`; the gate's product suite is covered by the existing rows |
| Publication/release refusal paths | all scenarios | 2 — covered by the delta scenarios (19 site + 3 release + 1 profile) | No gap: each refusal is a scenario |
| Coverage measurement | merged report | not re-run by review | The archive receipt will carry the measured merged result |

No dead code identified for deletion; no redundant unit test was added (the
`.feature` scenarios are the surface, and the change adds no parallel unit
test); no fourth-bucket classification is used.

## Scenario execution evidence

Review does not re-run the suite. The change verification receipt exists at
`givn/changes/publish-public-website/verification/tasks-verification.json`:
`schema 2`, `status: passed`, `head: cf14afa4c5699b6c61af394810660d2651a23c47`,
`command: ./run-tests.sh`, `runner_exit: 0`, `result: {scope: regular, total:
216, passed: 216, failed: 0, skipped: 0}`. The committed `tasks.md` closeout
records the same full suite (`11 features`, `216 scenarios (216 passed)`,
`1065 steps`, exit 0) and the `@e2e` scope (`9 features`, `14 scenarios
(14 passed)`, `126 steps`, exit 0), with `14 < 216`. The archive gate will
produce the fresh merged per-scope receipt.

**Receipt status:** proven candidate at `cf14afa`; the review gate currently
reports the receipt stale only because untracked, unrelated ideation files
(`givn/ideation/generalized-duty-plan-input/**`,
`docs/ideation/generalized-duty-plan-input.md`) appeared after the receipt was
written (06:17:04 receipt, 06:17:09 files) and changed the tested-content
digest. Those files belong to another change; regenerating the receipt is the
orchestrator's step once that work settles. No change-owned file changed after
the receipt.

## Overlap dispositions

The deterministic gate reported no shape-match and no long-scenario finding
for this change's delta. The first-pass routing warning is gone after the
proposal table fix (F4). The gate's change-receipt check is covered in
"Scenario execution evidence"; its current staleness is environmental, not a
review.md disposition.

## Split-or-keep

No scenario in the delta exceeds the 14-step threshold (the longest is the
`@e2e` at exactly 14 steps).

## README impact

`README.md` was edited in the change (commit `2f0557c`) in the
"Download a release" section: the browser route at the canonical address and
the published-archive/on-request wording. No open README placeholder is
present.

README-IMPACT: updated - Download a release

## Sign-off

- [x] Every checked task has a verified commit touching production code or its
      stated harness/CI deliverable; none is spec-only; each commit body names
      its scenario.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence is recorded per scope: the change
      verification receipt (`216/216`, exit 0, `cf14afa`) and the `tasks.md`
      `@e2e` scope (14/14); review did not re-run the suite; the archive
      receipt will carry the merged scopes.
- [x] Every coverage gap is classified in the three buckets; the one ordinary
      browser gap was bucket 2 and is resolved in `a5419b5`.
- [x] Dead code deleted / none; missing test added (F1's fix); no
      hard-to-test gap asserted beyond the recorded coverage boundary.
- [x] Redundant unit tests: none added.
- [x] No `@wip` tags remain; the delta spec carries no implementation-layer
      detail beyond externally observable artifact members.
- [x] Canonical E2E policy read from the resolved specs instruction; every
      normalized inventory action has one `@e2e` scenario; variants are regular
      scenarios.
- [x] Every `@e2e` scenario has a real-interface primary assertion; the browser
      scenarios use Chromium through the driver, never HTTP/fetch as a
      substitute for the UI.
- [x] The local run command (`./scripts/run-web.sh`) builds and serves
      `dist/site/`; the scenarios start their own local targets and loopback
      servers (T23's driver smoke and the green `@e2e` run).
- [x] The exact file(s) `verify.e2e_command` invokes were identified
      (`features_runner` → `steps/e2e/publish_public_website.rs`) and the tree
      (including untracked files) was searched for a parallel implementation;
      none exists.
- [x] `verify.e2e_command` differs from `verify.command`; per-scope counts
      (216 vs 14) prove isolation.
- [x] Implementation matches `design.md` for every named command, file layout
      and framework/driver; the first-pass F1 deviation is repaired in
      `a5419b5`.
- [x] Interaction coverage verified: every inventory entry maps to a matrix row,
      an existing `@e2e` scenario and the promised mechanism.
- [x] No finding is excused with a classification outside the three buckets.
- [x] `design.md` has no `## Internal Primitives` section requirement beyond
      the new commands; no internal struct/fn relied on by steps is omitted.
- [x] Arc42 slug mapping verified: all twelve chapter slugs in `arc42.md` map to
      real `docs/arc42/*.md` files; the first-pass chapter 2/7 list omission
      (F2) is fixed and every row matches.
- [x] No invented exception language is used in this report.

No unresolved finding remains. All four first-pass findings are fixed in
`a5419b5` (with `f229db6` for the step-file table and `cf14afa` for the
refreshed screenshots), the visual fixes from `1251699` hold, and
`ARC42 CONFORMANCE: CLEAN` is recorded. The review gate's only non-content
complaint is the environmentally stale change-verification receipt described
above; the orchestrator must regenerate it after the unrelated ideation work
settles.

REVIEW: PASS
