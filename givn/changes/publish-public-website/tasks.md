# Tasks: publish-public-website

Change: publish the Public website from a named Built revision, gate it on the
page-content, licensing, browser and conversion evidence, and couple the
release to it. Plan: `proposal.md`, `specs/**`, `design.md`, `arc42.md`;
review: `design-review.md` (`DESIGN-REVIEW: PASS`, stage `design-review-done`).

## Preconditions and interaction coverage

- `design-review` is done (`givn status --change publish-public-website`).
- Interaction inventory ↔ matrix ↔ `@e2e` pre-flight: the new leaf's one entry
  ("publish the site from a source revision") maps to the matrix row and the
  existing `@e2e` "Release maintainer publishes the site and the hosting target
  holds the built page"; the four inherited entries (`publish-public-release`
  ×2, `publish-organization-profile` ×2) are in the matrix with their existing
  `@e2e` titles. No gap, no missing `@e2e`; no task below invents one.
- Order: all non-`@e2e` scenarios first (site, then the two extended
  capabilities), then the E2E setup, then the one `@e2e` scenario last.
- Never remove an `@e2e` tag. `verify.command` is `./run-tests.sh`;
  `verify.e2e_command` is `./run-tests.sh --e2e` — a strict subset, proved in
  T23.
- One atomic `givn commit` per scenario, RED+GREEN+REFACTOR together; the
  subject is the release note, the body carries the scenario title.

## Shared design contract (applies to every scenario below)

- Command: `scripts/publish-site.sh --repo DIR [--revision REV] [--since COMMIT]
  --target URL --out DIR --verify-url URL [--key FILE] [--branch main]
  [--lock PATH]`.
- Steps: (1) refuse an unclean caller tree with
  `git status --porcelain --untracked-files=no`; (2) when `--since` is given,
  diff `since..rev` against the site-input list and print
  `no site input changed` on an empty intersection, refusing an unresolvable
  base; (3) resolve `rev-parse --verify "<rev>^{commit}"` and
  `describe --tags --always --match 'v*'`; (4) create a detached worktree,
  run `scripts/build-web.sh` then `scripts/build-site.sh --repo <worktree>
  --revision <rev> --out <out>`; (5) gate with `scripts/site-evidence.sh`;
  (6) publish under `flock`, idempotent, one fast-forward commit, no force;
  (7) read the pushed tree back and compare member lists; (8) fetch
  `--verify-url` with bounded retries and refuse unless the served page carries
  the same Built revision text, naming the address and the served revision.
- Site-input paths owned by the command: `crates/**`, `ffi/wasm/**`, `web/**`,
  `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`,
  `LICENSES/MPL-2.0.txt`, `.github/workflows/web.yml`, `docs/assets/**`,
  `.github/workflows/site.yml`.
- Site output, exactly eight members: `index.html`, `app.js`, `capability.js`,
  `styles.css`, `icon.png` (from `docs/assets/duty2cal.png`), then the page
  carries `pkg/duty2cal_wasm.js`, `pkg/wasm-inline.js` and `LICENSE.txt` (from
  `LICENSES/MPL-2.0.txt`). Nothing else.
- Statements, exactly one element per name: above the converter panel
  (heading "Before you choose a plan") `what-it-does`, `browser-route`,
  `archive-route`, `privacy`, `non-affiliation`, `as-is`; below the converter
  panels (heading "Source and licence") `source`, `license`; footer
  `built-revision` plus prose (no statement attributes). `archive-route` says
  "Downloads for various operating systems are available on request; the
  release page carries the published builds", linking
  `https://github.com/duty2cal/duty2cal/releases`.
- Header: 48×48 icon left of the `h1` and subheadline, alt "duty2cal",
  `<link rel="icon" href="./icon.png">` replacing `href="data:, "`.
- Gate order (`scripts/site-evidence.sh`, fail fast): page content
  (`scripts/check-site-content.sh`, one element per name, no foreign asset),
  licensing (`reuse lint`), browser check (`./run-tests.sh --name "<browser
  @e2e title>"`), conversion suite (`./run-tests.sh --tags "not @e2e and not
  @site-publication"`). Checks 3–4 set `GIVN_RESULT_FILE`; `total == 0`
  refuses, and the browser check requires exactly one selected scenario.
- `@site-publication` stays on every non-`@e2e` site and release-gate scenario;
  the site feature tag alone is not enough (the archive merge drops it).
- Fixture: real git clone with a `v0.5.0` tag and three commits after it; local
  bare target; pre-receive hook when a push must be refused; loopback servers
  serving the target tree or an earlier tree; recording `gh` stand-in.
- Step files: `apps/cli/tests/steps/publish_public_website.rs` (new,
  non-`@e2e`), `apps/cli/tests/steps/e2e/publish_public_website.rs` (new,
  `@e2e`), `apps/cli/tests/steps/publish_public_release.rs` (extend),
  `apps/cli/tests/steps/publish_organization_profile.rs` (replace one step),
  `apps/cli/tests/steps/support.rs` (fixtures). Declare each new module in
  `steps/mod.rs` / `steps/e2e/mod.rs`.
- Use-case constraints carried by every task: the minimal guarantee (a refusal
  before the replacement leaves the previous page and names the failure; after
  the replacement the target holds the complete content and a serving refusal
  names the address, never claiming restoration); success guarantee (the
  published page serves the named Built revision and states its terms before
  the picker); rules K3 (named revision), K4 (terms before picker), K5 (no
  request to another origin after load); a release tag always builds the
  tagged tree.

---

## T00 — Setup: runner configuration, strict-mode proof, step skeletons

- [x] Read the owning use case (`specs/publish-public-website/usecase.md`) and
      `design.md` §Test runner/§Step Definitions; confirm the runner is
      `cucumber-rs` with `.fail_on_skipped()` in
      `apps/cli/tests/features_runner.rs` and that `givn/commands.yaml` carries
      `verify.command: ./run-tests.sh`, `verify.e2e_command:
      ./run-tests.sh --e2e`.
- [x] Create the non-`@e2e` step file
      `apps/cli/tests/steps/publish_public_website.rs` and declare it in
      `apps/cli/tests/steps/mod.rs`; create the `@e2e` step file
      `apps/cli/tests/steps/e2e/publish_public_website.rs` and declare it in
      `apps/cli/tests/steps/e2e/mod.rs`. Every body starts as
      `unimplemented!()` — never `{}`, bare `return`, or a comment.
- [x] Add the shared helpers to `apps/cli/tests/steps/support.rs`: fixture git
      repository (real `v0.5.0` tag + three commits), local bare target,
      pre-receive hook, recording `gh` stand-in on `PATH`, loopback static
      servers over the target tree and over an earlier tree, and the
      lockfile-installed `web/e2e/node_modules` symlink path.
- [x] Proof of strictness: temporarily bind one step of an existing `@wip`
      scenario in the new file to `unimplemented!()`, or leave one of its steps
      undefined; run
      `./run-tests.sh --name "<that scenario's title>"` and confirm a non-zero
      exit. Paste the command and the failing output.
      - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A default-branch revision that changes none of the site's inputs starts no Site publication"`
        → `✘ Given a hosting target serving the Site output built from "abc1234"` /
        `Step panicked. Captured output: not implemented: a hosting target serving the Site output built from a revision` /
        `1 scenario (1 failed)`; exit 101. The probe was the new step's
        `unimplemented!()` body; the scenario's RED replaces it.
- [x] Remove the probe, re-run the runner to exit 0 with the real scenarios
      still `@wip`, and paste the output.
      - Evidence: the probe was the step stub itself; with the change's
        scenarios still `@wip`, the permanent baseline
        `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A source outside the supported family is rejected"`
        → `2 scenarios (2 passed)`, `13 steps (13 passed)`, exit 0.
- [x] Confirm `givn lint --change publish-public-website` exits 0 or 2.
      - Evidence: `givn lint --change publish-public-website` → `3 file(s) checked, 28 finding(s)` (advisory persona references and `@wip` notes), exit 2.
- [x] COMMIT — `chore(publish-public-website): prepare the site publication steps`
      (body: setup task).
      - Revision: b09405664342e9da4c6104aafcfbbbae74f6e3d0

---

## Non-@e2e scenarios

### T01 — A default-branch revision that changes none of the site's inputs starts no Site publication

- Design constraints: `--repo <fixture>` `--revision def5678` `--since
  abc1234` `--target <bare>` `--out <dir>` `--verify-url <loopback>`; the diff
  intersects the owned list, prints exactly `no site input changed`, exits 0,
  and writes nothing to the target or the output directory. A docs-only
  revision (`docs/**`) is the fixture's shape.
- Domain constraints: trigger rule for default-branch revisions; the current
  Public website stays; protects the current-site minimal guarantee.
- [x] RED — remove `@wip` from this scenario only; run
      `./run-tests.sh --name "A default-branch revision that changes none of the site's inputs starts no Site publication"`;
      must exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A default-branch revision that changes none of the site's inputs starts no Site publication"`
    → `✘ Given a hosting target serving the Site output built from "abc1234"` /
    `not implemented: a hosting target serving the Site output built from a revision`,
    `1 scenario (1 failed)`; exit 101.
- [x] GREEN — implement the trigger decision in `scripts/publish-site.sh`
      (site-input list, `--since` resolution, `no site input changed`).
  - Production files: `scripts/publish-site.sh` (argument parsing, the owned
    site-input list, the `--since` diff decision and `no site input changed`),
    `scripts/build-site.sh` (the eight-member assembler the fixture's base page
    needs), `web/index.html` and `web/styles.css` (the stamped footer anchor and
    the statement panels), `apps/cli/tests/steps/support.rs` (the fixture base
    output), `apps/cli/tests/steps/publish_public_website.rs` (the T01 steps).
  - Evidence: scoped run → all six steps pass,
    `1 scenario (1 passed)`, `6 steps (6 passed)`, exit 0.
- [x] REFACTOR — remove duplication in the path-list handling; re-run scoped;
      exit 0.
  - Evidence: the site inputs are one list iterated by `site_input()`;
    scoped re-run `1 scenario (1 passed)`, `6 steps (6 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): skip publication when a revision changes no site input`
      (body: the scenario title).
  - Revision:

### T02 — A release tag republishes the site from the tagged revision and the footer names the tag

- Design constraints: `--revision v0.5.0` with no `--since`; build inside the
  detached worktree at that commit; `git describe --tags --always --match 'v*'`
  yields `v0.5.0`; the footer's `built-revision` text is `v0.5.0` linked to the
  tagged commit and contains no `-N-g` form; the target holds the eight-member
  output built from `abc1234`.
- Domain constraints: a release tag always starts a publication from the tagged
  revision even when it changes no site input; the footer's tag and the release
  agree.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A release tag republishes the site from the tagged revision and the footer names the tag"`
    → `✘ And the release tag "v0.5.0" pushed on a revision "abc1234" ...` /
    `not implemented: the release tag pushed on a revision that changes none of the site's inputs`,
    `1 step failed`; exit 101.
- [x] GREEN — tag handling (no `--since`), worktree checkout, footer stamping.
  - Production files: `scripts/publish-site.sh` (revision resolution, detached
    worktree and trap, driver-tree link, the revision's `scripts/build-web.sh`
    and `scripts/build-site.sh`, the evidence gate, the `flock` publication,
    read-back and bounded serving check), `scripts/build-site.sh` (the
    eight-member output with the stamped footer), `scripts/site-evidence.sh`
    and `scripts/check-site-content.sh` (the four gate checks and the
    `GIVN_RESULT_FILE` count), `scripts/build-web.sh` and `scripts/run-web.sh`
    (the licence leaves the web tree, the local run serves `dist/site/`),
    `web/index.html` and `web/styles.css` (the two statement panels, the
    header icon and the footer anchor), `.gitignore`, `REUSE.toml` and
    `scripts/pii-guard.sh` (the assembled output's ignore, the icon's
    CC0-1.0 override, the generated tree's exclusion).
  - Evidence: scoped run → `1 scenario (1 passed)`, `8 steps (8 passed)`,
    exit 0; the gate ran `check-site-content.sh`, `reuse lint`, the browser
    scenario and the conversion suite, and the target's footer read
    `v0.5.0`.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: the three logged build/gate failures share `fail_with_log`;
    scoped re-run `1 scenario (1 passed)`, `8 steps (8 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): build a tagged publication from the tagged revision`
      (body: the scenario title).
  - Revision: 605f7be9ff5d9d3ddc44c06e7f5cd81b5cb0bbbc

### T03 — A Built revision that cannot be resolved refuses the publication

- Design constraints: `rev-parse --verify "<rev>^{commit}"` fails; refuse
  before any build, naming the Built revision text the caller supplied; the
  target is unchanged.
- Domain constraints: rule K3 (a page cannot name a revision that did not build
  it); minimal guarantee.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A Built revision that cannot be resolved refuses the publication"`
    → `✘ And a source revision whose history the repository does not hold` /
    `not implemented: a source revision whose history the repository does not hold`,
    `1 step failed`; exit 101.
- [x] GREEN — revision resolution and named refusal.
  - Production files: `scripts/publish-site.sh` (`rev-parse --verify
    "<rev>^{commit}"` and `describe` failures refuse, naming the Built
    revision, before any build), `apps/cli/tests/steps/support.rs`
    (`site_resolves`), `apps/cli/tests/steps/publish_public_website.rs` (the
    scenario's steps).
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; the publication printed `cannot resolve the Built revision
    '9999999'` and the target still served `v0.5.0-3-g…`.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: scoped re-run `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0.
- [x] COMMIT — `feat(publish-public-website): refuse an unresolvable Built revision`
      (body: the scenario title).
  - Revision: 7a22d40607a877fadbb87bcbc397c28aac610388

### T04 — A source revision whose working tree carries changes it does not hold is refused

- Design constraints: the fixture edits a tracked file in the caller repository;
  `git status --porcelain --untracked-files=no` is non-empty; the refusal names
  the unclean revision and writes nothing. Untracked files must NOT refuse.
- Domain constraints: the publication builds only committed revisions; a
  maintainer must not believe uncommitted edits were published.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A source revision whose working tree carries changes it does not hold is refused"`
    → `✘ And a working tree at the revision "abc1234" carrying edits to files the revision tracks` /
    `not implemented: a working tree carrying edits to files the revision tracks`,
    `1 step failed`; exit 101.
- [x] GREEN — cleanliness check with `--untracked-files=no` and the named
      refusal.
  - Production files: `scripts/publish-site.sh` (the tracked-change check
    before the trigger decision), `apps/cli/tests/steps/support.rs`
    (`site_checkout`, `site_append`, `site_write_untracked`),
    `apps/cli/tests/steps/publish_public_website.rs` (the scenario's steps).
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; the refusal named the unclean revision and the target still
    served the earlier page.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: scoped re-run `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0.
- [x] COMMIT — `feat(publish-public-website): refuse an unclean caller tree`
      (body: the scenario title).
  - Revision: 84ac95a3e5099d0ebdecfb18eee0314cc72ea752

### T05 — A revision whose tree cannot be checked out is refused and the worktree is named

- Design constraints: `git worktree add --detach` fails (missing object or
  unwritable scratch path); refuse naming the worktree and the cause; the trap
  removes any worktree on every exit path, including this refusal.
- Domain constraints: minimal guarantee; no stale registration.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A revision whose tree cannot be checked out is refused and the worktree is named"`
    → `✘ And a source revision "def5678" the repository cannot check out` /
    `not implemented: a source revision the repository cannot check out`,
    `1 step failed`; exit 101.
- [x] GREEN — worktree creation, trap, named refusal.
  - Production files: `scripts/publish-site.sh` (the scratch path's failure and
    `git worktree add` fail closed, naming the worktree and the cause; the trap
    removes any worktree on every exit path),
    `apps/cli/tests/steps/publish_public_website.rs` (the scenario's steps).
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; the refusal named the worktree and the target still served the
    earlier page.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: scoped re-run `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0.
- [x] COMMIT — `feat(publish-public-website): name a worktree that cannot be created`
      (body: the scenario title).
  - Revision: 4b4f8e55081593a20a00be9615d4041464786dc0

### T06 — A browser build that fails refuses the publication and names the build step

- Design constraints: inside the worktree, `scripts/build-web.sh` fails (the
  fixture makes the pinned toolchain step fail); refuse naming the revision's
  browser build step; the assembler's missing-bundle refusal
  (`scripts/build-site.sh` naming `scripts/build-web.sh`) stays a defensive
  check but is not the scenario's path.
- Domain constraints: a failed build publishes nothing and leaves the previous
  page; the failed step is named.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A browser build that fails refuses the publication and names the build step"`
    → `✘ And a source revision whose browser build fails` /
    `not implemented: a source revision whose browser build fails`,
    `1 step failed`; exit 101.
- [x] GREEN — build invocation and named failure propagation.
  - Production files: `scripts/publish-site.sh` (the worktree's
    `scripts/build-web.sh` failure is refused by name),
    `apps/cli/tests/steps/publish_public_website.rs` (the scenario's steps).
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; the refusal named `browser build`, `scripts/build-web.sh` and the
    revision, and the target still served the earlier page.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: scoped re-run `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0.
- [x] COMMIT — `feat(publish-public-website): name the browser build step when it fails`
      (body: the scenario title).
  - Revision:

### T07 — A page missing a public statement is refused and the statement is named

- Design constraints: `scripts/check-site-content.sh <out>` must require one
  element per statement name and refuse a missing `data-site-statement="privacy"`
  by naming the privacy statement. A duplicate element for one name is also a
  refusal. The check reads the assembled output, not the page source.
- Domain constraints: K4 — the page states what it does, both routes, source,
  licence, privacy, non-affiliation and as-is before it asks for a file.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — the content check with per-statement names.
  - Production files:
  - Evidence:
- [ ] REFACTOR — extract the name list; re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): name a missing public statement`
      (body: the scenario title).
  - Revision:

### T08 — A page whose script requests another origin after it loads is refused and the browser check is named

- Design constraints: the fixture page script attempts a request after load;
  the driver records `requestfailed` (CSP `connect-src 'none'` blocks it); the
  browser check (`./run-tests.sh --name "<browser @e2e title>"` with
  `GIVN_RESULT_FILE`) refuses and the publication names `browser check`. The
  CSP is not weakened.
- Domain constraints: K5 — after load the page sends no request to another
  origin; the whole-web-reviewer lens holds no publishing authority to weaken
  it.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — driver request observation and named browser-check refusal.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): refuse a page that requests another origin`
      (body: the scenario title).
  - Revision:

### T09 — A page that loads a third-party asset is refused and the page-content check is named

- Design constraints: a static `<img src="https://…">` (or equivalent) is
  refused by `scripts/check-site-content.sh`, naming `page content` and the
  foreign asset; this is distinct from a runtime attempt (T08).
- Domain constraints: K5 and the self-contained-page rule.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — foreign-asset refusal in the content check.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): refuse a page that loads a foreign asset`
      (body: the scenario title).
  - Revision:

### T10 — A failing conversion check is refused and the conversion suite is named

- Design constraints: the revision's conversion suite fails; the gate runs it
  with `DUTY2CAL_SITE_ROOT`, `DUTY2CAL_SKIP_WEB_BUILD=1` and
  `GIVN_RESULT_FILE`, fails fast, and the publication names `conversion suite`.
  A `total == 0` selection is a refusal, not a pass.
- Domain constraints: only a working page becomes public; the success guarantee
  rests on complete-or-explicit-failure evidence.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — conversion gate step and result-file count contract.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): name a failing conversion check`
      (body: the scenario title).
  - Revision:

### T11 — A revision the publication is asked to compare against cannot be resolved

- Design constraints: a supplied `--since 9999999` that the repository does not
  hold refuses and names that base; it is never read as "no change".
- Domain constraints: the default-branch trigger decision is a command
  argument; an unknown base must not silently suppress a publication.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — base resolution refusal.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): refuse an unresolvable comparison base`
      (body: the scenario title).
  - Revision:

### T12 — A driver package tree installed from a different lockfile is refused and the lockfile is named

- Design constraints: compare the worktree's `web/e2e/package-lock.json` with
  the caller's; when they differ, refuse and name the lockfile and `npm ci
  --prefix web/e2e` at the revision; symlink the caller's `node_modules` only
  when they match, and refuse by name when no installed tree exists.
- Domain constraints: the gate runs the revision's own pinned Playwright and
  axe, never versions from another lockfile.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — lockfile comparison, symlink/refusal path.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): refuse a driver tree from a different lockfile`
      (body: the scenario title).
  - Revision:

### T13 — An asset without licence information is refused and the asset is named

- Design constraints: `reuse lint` fails on a new site asset without licence
  information; the gate names `licensing` and the offending file. The icon's
  `docs/assets/**` CC0-1.0 override is part of `REUSE.toml` in GREEN for the
  publication path, and the copied `LICENSE.txt` is a build output.
- Domain constraints: MPL-2.0 for code, CC0-1.0 for non-code assets,
  Unicode-3.0 attribution for third-party data.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — licensing gate step and named failure.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): name an unlicensed site asset`
      (body: the scenario title).
  - Revision:

### T14 — Republishing an unchanged revision reports the target as current and writes nothing

- Design constraints: after the gate, compare the fetched target tip's tree
  with the output; when equal, print `already current`, exit 0, stage nothing,
  commit nothing, push nothing. Assert the target history holds no new commit.
- Domain constraints: a maintainer re-running the publication must not create a
  second identical commit; manual dispatch is a supported trigger.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — idempotent comparison and exit.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): report an unchanged target as current`
      (body: the scenario title).
  - Revision:

### T15 — An address that serves an earlier revision refuses the publication and is named

- Design constraints: the loopback twin keeps serving the `abc1234` tree after
  the push; `--verify-url` reads it with bounded retries, then refuses naming
  the address and the revision it serves; the target still holds `def5678`
  (the check runs after the replacement and never restores the previous page).
- Domain constraints: the served page must name the Built revision; a serving
  refusal is named after the replacement and is never reported as a working
  page.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — bounded serving check and named refusal.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): name an address that serves an earlier revision`
      (body: the scenario title).
  - Revision:

### T16 — A hand edit in the hosting target is removed by the whole-content replacement

- Design constraints: one commit whose tree is exactly the eight members;
  fast-forward, single parent; the hand-added file disappears; the earlier
  page stays in the target's history.
- Domain constraints: the page is never edited in the target; whole-content
  replacement is the only write path.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — tree replacement, fast-forward push.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): replace the hosting target's content as a whole`
      (body: the scenario title).
  - Revision:

### T17 — A refused publication into the hosting target leaves the previous page

- Design constraints: a pre-receive hook refuses the push; the publication
  exits non-zero naming the hosting target, and the target still serves the
  `abc1234` page. No force push, no retry that bypasses the hook.
- Domain constraints: minimal guarantee; the ruleset forbids non-fast-forward
  pushes.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — push failure propagation and named refusal.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): name a refused push and keep the previous page`
      (body: the scenario title).
  - Revision:

### T18 — Two publications that begin together are serialized and neither is refused

- Design constraints: an exclusive `flock` spans fetch/replace/push; two
  concurrent runs of the same revision both exit 0; the target ends at
  `def5678`; neither fails as a non-fast-forward. The workflow adds the
  concurrency group in `site.yml` (recorded, exercised on the host).
- Domain constraints: two publications must not race; a refused loser would
  break the trigger promise.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — flock and serialized publish path.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-website): serialize concurrent publications`
      (body: the scenario title).
  - Revision:

### T19 — A revision that changes the public website is announced in the release notes

- Design constraints: `cliff.toml` (a) adds `web/` and `ffi/wasm/` to the
  user-receiving paths, and (b) adds the capability to the allow-list only when
  the use case is permanent under `givn/specs/publish-public-website/` (the
  archive merge), because `scripts/render-changelog.sh` refuses an allow-list
  naming a capability the repository does not hold; the release-announcement
  fixture's capability list gains `publish-public-website` in the same pass.
  The docs-only revision stays out of the notes.
- Domain constraints: a site change is a user-receiving change; the release
  notes state what changed.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — `cliff.toml` data and the fixture's capability list.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-release): announce public website changes`
      (body: the scenario title).
  - Revision:

### T20 — The public text states the browser route beside the downloadable archive platform

- Design constraints: `org/profile/README.md`, `README.md` and the checked
  public text state that a current browser converts a plan on the Public
  website, that Windows x86_64 is the only **published** archive platform, that
  builds for other operating systems are available on request, and that the
  source can be built with a Rust toolchain; `scripts/public-text-check.sh`
  enforces it; no archive for another platform is named.
- Domain constraints: truthful public copy; the profile copy is applied only
  after the site is live (T22).
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — public text sources and the checker.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-release): state the browser route beside the published archive`
      (body: the scenario title).
  - Revision:

### T21 — A release whose hosting target does not hold the tagged site revision is refused and publishes no release

- Design constraints: `scripts/publish-release.sh --version V --tag TAG
  --staged DIR --repo OWNER/NAME --site-repo DIR --target URL [--key FILE]`
  reads the target's branch tip; when it does not carry the tagged Built
  revision, it refuses, names the revision the target holds, runs no `gh`
  command at all, and leaves the staged set unchanged. The asset comparison
  excludes `release-notes.md`. The recording `gh` stand-in proves the absence
  of any invocation.
- Domain constraints: a version never becomes public while the page is stale;
  the release's archive contract is otherwise unchanged.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — target verification, `--site-repo`, gh stand-in wiring.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(publish-public-release): publish no release when the site target is stale`
      (body: the scenario title).
  - Revision:

### T22 — The published profile content states only supported claims and requests no third-party resource

- Design constraints: `org/profile/README.md` gains the browser route and the
  Public website link, states Windows x86_64 as the only published archive with
  builds for other operating systems on request, keeps absolute links, and
  embeds no third-party resource; `apps/cli/tests/steps/publish_organization_profile.rs`
  replaces the browser-claim step in place (the scenario is `@givn.modified`).
  The apply runs through `apply-configuration-change` after the site is live
  (D6); this task changes the copy and the step, not the live apply.
- Domain constraints: the profile copy is applied only after the site is live;
  the organization owner is a supporting lens, not an actor of the site change.
- [ ] RED — scoped run; exit non-zero.
  - Evidence:
- [ ] GREEN — profile copy and step replacement.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped; exit 0.
  - Evidence:
- [ ] COMMIT — `feat(apply-configuration-change): state the browser route in the organization profile`
      (body: the scenario title).
  - Revision:

---

## E2E setup

### T23 — E2E environment, driver, and strict-subset proof

- [ ] Configure the e2e run: `verify.e2e_command: ./run-tests.sh --e2e`
      (distinct from `verify.command: ./run-tests.sh`), driver served through
      `DUTY2CAL_SITE_ROOT`, Chromium from the Playwright package (or
      `CHROMIUM_PATH` on NixOS), Node from `web/e2e/.nvmrc`, `web/e2e/node_modules`
      installed from the lockfile.
- [ ] Bring up the local environment: a fixture repository clone with a real
      `v0.5.0` tag and three commits after it; a local bare target; loopback
      static servers over the target tree and over the earlier tree; the
      recording `gh` stand-in on `PATH`.
- [ ] Prove strict mode for the e2e runner: one deliberately undefined e2e step
      → scoped e2e run exits non-zero; paste it, remove the probe, re-run to 0.
      - Evidence:
- [ ] Prove the e2e filter is a strict subset: run `./run-tests.sh` and record
      its scenario count; run `./run-tests.sh --e2e` and record its count; the
      e2e count must be strictly less.
      - Full count:
      - E2E count:
- [ ] Confirm the driver reaches the loopback environment with a smoke
      navigation before any scenario step is written.
      - Evidence:
- [ ] COMMIT — `test(e2e): prepare the site publication browser environment`
      (body: e2e setup).
      - Revision:

---

## @e2e scenario

### T24 — Release maintainer publishes the site and the hosting target holds the built page

- Design constraints: drive `scripts/publish-site.sh --repo <fixture>
  --revision <sha> --target <bare> --out <dir> --verify-url <loopback over the
  target's checked-out tree>` as a real subprocess; then serve the target's
  checked-out tree through the Playwright driver and assert, in Chromium:
  the eight members; the earlier page replaced as a whole; the footer and the
  served address carry the same Built revision that the fixture's own
  `git describe` yields, linked to the commit; the header icon (48×48, alt
  "duty2cal") left of the heading and subheadline and the same tab icon; the
  "Before you choose a plan" panel with the six statements and the
  operator's archive-route wording; the "Source and licence" panel with the
  source and licence; the footer's prose terms without duplicate statement
  attributes; no request to another origin after load; the empty state before a
  plan and the ready state after a supported plan; the visual baseline in those
  two states. Capture 360×780 and 1920×1080 screenshots under
  `givn/changes/publish-public-website/evidence/visual/release-maintainer-publishes-the-site-and-the-hosting-target-holds-the-built-page/`.
- Domain constraints: success guarantee; K3, K4, K5; the crew member receives
  the calendar through the published page unchanged; the visitor reads the
  terms before the picker.
- [ ] RED — remove `@wip` from this scenario only; run
      `./run-tests.sh --e2e --name "Release maintainer publishes the site and the hosting target holds the built page"`;
      must exit non-zero.
  - Evidence:
- [ ] GREEN — the e2e steps, driver root, assertions and screenshots.
  - Production files:
  - Evidence:
- [ ] REFACTOR — re-run scoped e2e; exit 0.
  - Evidence:
- [ ] COMMIT — `test(e2e): publish the site and observe the built page`
      (body: the scenario title).
  - Revision:

---

## Closeout

- [ ] Every scenario box is checked with pasted evidence and exactly one
      reachable `COMMIT:` revision naming the scenario in its body.
- [ ] Merged step definitions: every phrase bound by the delta step files is
      also bound in the permanent per-capability step files; archive is not
      used to move them.
- [ ] Full `./run-tests.sh` exits 0 and `./run-tests.sh --e2e` exits 0; both
      counts recorded.
- [ ] `givn lint --change publish-public-website` exits 0 or 2; `givn status
      --change publish-public-website` shows tasks complete.
