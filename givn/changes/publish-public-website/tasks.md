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
  - Revision: ed51e5db4f0ea9be9fb9f9a8ca8f6d322faf2a28

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
  - Revision: 58cecd3d18157a7753e557cbf351b37594e5ed25

### T07 — A page missing a public statement is refused and the statement is named

- Design constraints: `scripts/check-site-content.sh <out>` must require one
  element per statement name and refuse a missing `data-site-statement="privacy"`
  by naming the privacy statement. A duplicate element for one name is also a
  refusal. The check reads the assembled output, not the page source.
- Domain constraints: K4 — the page states what it does, both routes, source,
  licence, privacy, non-affiliation and as-is before it asks for a file.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A page missing a public statement is refused and the statement is named"`
    → `✘ And a source revision whose page carries no privacy statement` /
    `Step panicked. Captured output: not implemented: a source revision whose page carries no privacy statement`,
    `1 scenario (1 failed)`; exit 101.
- [x] GREEN — the content check with per-statement names.
  - Production files: `scripts/check-site-content.sh` (the name list, the
    per-name count that survives `pipefail` when a name is absent, the
    non-empty-text requirement and the duplicate refusal),
    `apps/cli/tests/steps/support.rs` (`site_commit_drop_line` and the fixture
    trim that keeps the gate's conversion suite to one capability),
    `apps/cli/tests/steps/publish_public_website.rs` (the scenario's steps).
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; the publication printed `check-site-content: page content check
    failed: the page carries no privacy statement` and the target still served
    the earlier page. A manual duplicate probe printed `the page carries the
    privacy statement 2 times; exactly one element per statement name is
    required` (exit 1), and an empty element printed `the page's privacy
    statement carries no text` (exit 1).
- [x] REFACTOR — extract the name list; re-run scoped; exit 0.
  - Evidence: the names are one `statement_names` list read by
    `statement_count`/`statement_text`, and the absent-name exit is handled in
    `statement_count` instead of aborting under `pipefail`; scoped re-run
    `1 scenario (1 passed)`, `5 steps (5 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): name a missing public statement`
      (body: the scenario title).
  - Revision: bc5ab02149e2ab5c72ea3e1c6f22197f2f98093a

### T08 — A page whose script requests another origin after it loads is refused and the browser check is named

- Design constraints: the fixture page script attempts a request after load;
  the driver records `requestfailed` (CSP `connect-src 'none'` blocks it); the
  browser check (`./run-tests.sh --name "<browser @e2e title>"` with
  `GIVN_RESULT_FILE`) refuses and the publication names `browser check`. The
  CSP is not weakened.
- Domain constraints: K5 — after load the page sends no request to another
  origin; the whole-web-reviewer lens holds no publishing authority to weaken
  it.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A page whose script requests another origin after it loads is refused and the browser check is named"`
    → `✘ And a source revision whose page's script attempts a request to another origin after the page loads` /
    `Step panicked. Captured output: not implemented: a source revision whose page's script attempts a request to another origin after the page loads`,
    `1 scenario (1 failed)`; exit 101.
- [x] GREEN — driver request observation and named browser-check refusal.
  - Production files: `web/e2e/driver.mjs` (serves the directory named by
    `DUTY2CAL_SITE_ROOT` and fails closed when it is missing, records
    `securitypolicyviolation` attempts in the request log the steps read),
    `scripts/site-evidence.sh` (the browser check's driver tree, `node` and
    Chromium prerequisites refuse it by name instead of skipping, the output
    path is made absolute for the driver, the licensing check refuses a missing
    `reuse` by name), `apps/cli/tests/steps/publish_public_website.rs` (the
    scenario's steps).
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; the publication printed `site-evidence: the browser check failed`
    and the gate's log carried the browser scenario's own
    `the page requested another origin "https://beacon.example/fixture-request"`
    assertion. The CSP was not changed: `web/index.html` still carries
    `connect-src 'none'`.
    Prerequisite probes (each exit 1, each naming the check): with `reuse`
    absent from `PATH`, `site-evidence: the licensing check cannot run: 'reuse'
    is not installed`; with `node` absent, `site-evidence: the browser check
    cannot run: 'node' is not installed`; with `CHROMIUM_PATH` naming no
    executable, `site-evidence: the browser check cannot run: Chromium is not
    installed; …`; against a clean worktree with no installed driver tree,
    `site-evidence: the browser check has no driver tree at
    '<worktree>/web/e2e/node_modules'; run 'npm ci --prefix web/e2e' at this
    revision`. Content and licensing were exercised first in each probe, so the
    refusal came from the check under test.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: checks 3–4 share one `run_revision_suite` helper that owns
    `GIVN_RESULT_FILE`, `DUTY2CAL_SITE_ROOT` and `DUTY2CAL_SKIP_WEB_BUILD=1`;
    scoped re-run `1 scenario (1 passed)`, `5 steps (5 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): refuse a page that requests another origin`
      (body: the scenario title).
  - Revision: 391a59d25d45a92d0ce1c3811a4e1e708186050c

### T09 — A page that loads a third-party asset is refused and the page-content check is named

- Design constraints: a static `<img src="https://…">` (or equivalent) is
  refused by `scripts/check-site-content.sh`, naming `page content` and the
  foreign asset; this is distinct from a runtime attempt (T08).
- Domain constraints: K5 and the self-contained-page rule.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A page that loads a third-party asset is refused and the page-content check is named"`
    → `✘ And a source revision whose page loads an asset from another origin` /
    `Step panicked. Captured output: not implemented: a source revision whose page loads an asset from another origin`,
    `1 scenario (1 failed)`; exit 101.
- [x] GREEN — foreign-asset refusal in the content check.
  - Production files: `scripts/check-site-content.sh` (the assembled
    `index.html` is the only input; `src`/`srcset`/`poster` and `<link href>`
    naming another origin refuse, the asset is printed, navigation `<a href>`
    links stay allowed), `apps/cli/tests/steps/publish_public_website.rs` (the
    scenario's steps).
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; the publication printed `check-site-content: page content check
    failed: the page content loads an asset from another origin:
    src="https://tracker.example/fixture.png"` and the target still served the
    earlier page. This is the static half of K5; the runtime half is T08.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: the two foreign-asset patterns are one `foreign_asset_patterns`
    list read by one loop; scoped re-run `1 scenario (1 passed)`,
    `5 steps (5 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): refuse a page that loads a foreign asset`
      (body: the scenario title).
  - Revision: 0effcaf4785775d6454226a66b3688b313f5ea6d

### T10 — A failing conversion check is refused and the conversion suite is named

- Design constraints: the revision's conversion suite fails; the gate runs it
  with `DUTY2CAL_SITE_ROOT`, `DUTY2CAL_SKIP_WEB_BUILD=1` and
  `GIVN_RESULT_FILE`, fails fast, and the publication names `conversion suite`.
  A `total == 0` selection is a refusal, not a pass.
- Domain constraints: only a working page becomes public; the success guarantee
  rests on complete-or-explicit-failure evidence.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A failing conversion check is refused and the conversion suite is named"`
    → `✘ And a source revision whose conversion of a supported plan file fails its checks` /
    `Step panicked. Captured output: not implemented: a source revision whose conversion of a supported plan file fails its checks`,
    `1 scenario (1 failed)`; exit 101.
- [x] GREEN — conversion gate step and result-file count contract.
  - Production files: `scripts/site-evidence.sh` (check 4 runs the revision's
    suite with `GIVN_RESULT_FILE`, `DUTY2CAL_SITE_ROOT` and
    `DUTY2CAL_SKIP_WEB_BUILD=1` after the browser check, fails fast, and refuses
    a selection that records no scenario), `apps/cli/tests/steps/publish_public_website.rs`
    (the scenario's steps: the fixture revision drops the `OFF` activity code
    from `crates/duty2cal-core/src/application/mod.rs`, which the browser
    scenario's plan does not use).
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; the publication printed `site-evidence: the conversion suite
    failed` and the gate's log named the failing scenario
    `All-day codes ignore printed clocks`. The browser check passed first with
    the same revision, so the refusal is the conversion suite's own.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: both checks read their count through one `require_selection`
    contract (`exactly-one` for the browser check, `at-least-one` for the
    conversion suite), so `total == 0` refuses in one place; scoped re-run
    `1 scenario (1 passed)`, `5 steps (5 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): name a failing conversion check`
      (body: the scenario title).
  - Revision: 22e731d6e6f49c0301790507dfd83b380d5b5996

### T11 — A revision the publication is asked to compare against cannot be resolved

- Design constraints: a supplied `--since 9999999` that the repository does not
  hold refuses and names that base; it is never read as "no change".
- Domain constraints: the default-branch trigger decision is a command
  argument; an unknown base must not silently suppress a publication.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A revision the publication is asked to compare against cannot be resolved"`
    → `✘ And a revision "def5678" whose site inputs changed` /
    `Step panicked. Captured output: not implemented: a revision whose site
    inputs changed`, `1 scenario (1 failed)`; exit 101.
- [x] GREEN — base resolution refusal.
  - Production files: `scripts/publish-site.sh` (a `--since` base the
    repository cannot resolve refuses with `cannot resolve the comparison base
    '<base>'; an unknown base is not 'no change'` before the diff, so an
    unknown base can never print `no site input changed`; the diff itself
    fails closed too), `apps/cli/tests/steps/publish_public_website.rs` (the
    scenario's steps: the fixture revision appends a stylesheet comment — a
    site input — and records `def5678`).
  - Evidence: scoped run → `1 scenario (1 passed)`, `6 steps (6 passed)`,
    exit 0; the refusal printed `cannot resolve the comparison base
    '9999999'` and the target still served the earlier page.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: the comparison base and the Built revision resolve through one
    `resolve_commit` helper, so the two refusals cannot drift; scoped re-run
    `1 scenario (1 passed)`, `6 steps (6 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): refuse an unresolvable comparison base`
      (body: the scenario title).
  - Revision: 7a000990310cfa8e4509c220f88533da7be208e8

### T12 — A driver package tree installed from a different lockfile is refused and the lockfile is named

- Design constraints: compare the worktree's `web/e2e/package-lock.json` with
  the caller's; when they differ, refuse and name the lockfile and `npm ci
  --prefix web/e2e` at the revision; symlink the caller's `node_modules` only
  when they match, and refuse by name when no installed tree exists.
- Domain constraints: the gate runs the revision's own pinned Playwright and
  axe, never versions from another lockfile.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A driver package tree installed from a different lockfile is refused and the lockfile is named"`
    → `✘ And a source revision that pins different driver packages than the working copy installs` /
    `Step panicked. Captured output: not implemented: a source revision that pins different driver packages than the working copy installs`,
    `1 scenario (1 failed)`; exit 101.
- [x] GREEN — lockfile comparison, symlink/refusal path.
  - Production files: `scripts/publish-site.sh` (the worktree's
    `web/e2e/package-lock.json` is compared with the caller's; a difference —
    including a file present on only one side — refuses naming the lockfile,
    both paths and `npm ci --prefix web/e2e` at the revision, before any build;
    the caller's installed tree is symlinked only after the comparison, and a
    caller with no installed tree is refused by name),
    `apps/cli/tests/steps/publish_public_website.rs` (the scenario's steps: the
    caller pins Playwright 1.64.1 while the published revision pins 1.63.0).
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; the refusal printed `the driver lockfile
    'web/e2e/package-lock.json' differs between the revision '<sha>'
    (.../tree/web/e2e/package-lock.json) and the caller (...); run 'npm ci
    --prefix web/e2e' at revision '<sha>' before publishing it`, and the target
    still served the earlier page.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: both driver-tree refusals name one `prepare_driver` command, so
    the lockfile and the missing-tree paths cannot drift apart; scoped re-run
    `1 scenario (1 passed)`, `5 steps (5 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): refuse a driver tree from a different lockfile`
      (body: the scenario title).
  - Revision: f23a944ea2d921109c62f04447a0e2daf3281ebe

### T13 — An asset without licence information is refused and the asset is named

- Design constraints: `reuse lint` fails on a new site asset without licence
  information; the gate names `licensing` and the offending file. The icon's
  `docs/assets/**` CC0-1.0 override is part of `REUSE.toml` in GREEN for the
  publication path, and the copied `LICENSE.txt` is a build output.
- Domain constraints: MPL-2.0 for code, CC0-1.0 for non-code assets,
  Unicode-3.0 attribution for third-party data.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "An asset without licence information is refused and the asset is named"`
    → `✘ And a source revision that adds a site asset with no licence information` /
    `Step panicked. Captured output: not implemented: a source revision that adds a site asset with no licence information`,
    `1 scenario (1 failed)`; exit 101.
- [x] GREEN — licensing gate step and named failure.
  - Production files: `scripts/site-evidence.sh` (check 2 refuses a missing
    `reuse` by name, runs `reuse lint` inside the revision's worktree, prints
    its report — which names the offending file — and then the `licensing`
    refusal), `apps/cli/tests/steps/publish_public_website.rs` (the scenario's
    steps: the revision adds `web/fixture-glyph.svg` naming a licence whose text
    does not exist, while `REUSE.toml`'s `docs/assets/**` CC0-1.0 override and
    the copied `LICENSE.txt` stay as they are).
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; `reuse lint` reported `MISSING LICENSES 'LicenseRef-Fixture-Proprietary'
    found in: * web/fixture-glyph.svg` and the gate printed
    `site-evidence: the licensing check failed; the offending file is named
    above`. The project's own `reuse lint` still exits 0.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: every refused check goes through one `refuse` helper, so the
    `site-evidence: …` prefix and the exit status cannot drift between the four
    checks; scoped re-run `1 scenario (1 passed)`, `5 steps (5 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): name an unlicensed site asset`
      (body: the scenario title).
  - Revision: 699b9cca902c18233b42ddd7527b65adcd345c27

### T14 — Republishing an unchanged revision reports the target as current and writes nothing

- Design constraints: after the gate, compare the fetched target tip's tree
  with the output; when equal, print `already current`, exit 0, stage nothing,
  commit nothing, push nothing. Assert the target history holds no new commit.
- Domain constraints: a maintainer re-running the publication must not create a
  second identical commit; manual dispatch is a supported trigger.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "Republishing an unchanged revision reports the target as current and writes nothing"`
    → `✘ Given a hosting target holding the Site output built from "abc1234"` /
    `Step panicked. Captured output: not implemented: a hosting target holding
    the Site output built from a revision`, `1 scenario (1 failed)`; exit 101.
- [x] GREEN — idempotent comparison and exit.
  - Production files: `scripts/publish-site.sh` (the fetched tip's tree is
    compared with the assembled output through `target_holds_output`; on
    equality the command stages, commits and pushes nothing, still re-reads the
    address through one `verify_served` — so a re-run of a served revision
    re-verifies the host — and prints `already current`, exit 0),
    `apps/cli/tests/steps/support.rs` (`site_target_history`),
    `apps/cli/tests/steps/publish_public_website.rs` (the scenario's steps; the
    fixture target's history is recorded before the re-run).
  - Evidence: scoped run → `1 scenario (1 passed)`, `4 steps (4 passed)`,
    exit 0; the publication printed `already current`, the target's tip and
    its whole `rev-list` were unchanged, and the rebuilt output compared equal
    to the target's tree.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: the idempotent decision and the serving check are single
    functions (`target_holds_output`, `verify_served`) shared by both outcomes,
    so the re-run path cannot skip the address; scoped re-run
    `1 scenario (1 passed)`, `4 steps (4 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): report an unchanged target as current`
      (body: the scenario title).
  - Revision: 468023861b959338b8571c527b6d244e3e0bf1b7

### T15 — An address that serves an earlier revision refuses the publication and is named

- Design constraints: the loopback twin keeps serving the `abc1234` tree after
  the push; `--verify-url` reads it with bounded retries, then refuses naming
  the address and the revision it serves; the target still holds `def5678`
  (the check runs after the replacement and never restores the previous page).
- Domain constraints: the served page must name the Built revision; a serving
  refusal is named after the replacement and is never reported as a working
  page.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "An address that serves an earlier revision refuses the publication and is named"`
    → `✘ And an address that keeps serving the page built from "abc1234" after the hosting target's content is replaced` /
    `Step panicked. Captured output: not implemented: an address that keeps serving the page built from an earlier revision`,
    `1 scenario (1 failed)`; exit 101.
- [x] GREEN — bounded serving check and named refusal.
  - Production files: `apps/cli/tests/steps/support.rs` (`site_serve_earlier`
    starts the loopback twin over the earlier tree under `/earlier/` and points
    `--verify-url` at it; `site_server_port` is the one server spawn),
    `apps/cli/tests/steps/publish_public_website.rs` (the scenario's steps and
    the revision-aware `the hosting target holds the Site output built from`
    assertion).
  - Evidence: scoped run → `1 scenario (1 passed)`, `6 steps (6 passed)`,
    exit 0; the gate passed (the target received the new output), the
    replacement landed, and the publication then refused with
    `the address 'http://127.0.0.1:<port>/earlier/' serves the Built revision
    'v0.5.0' instead of 'v0.5.0-1-g<sha>'; the target holds the complete output
    built from v0.5.0-1-g<sha>`. The target's footer named
    `v0.5.0-1-g<sha>`, i.e. `def5678`'s own history.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: both loopback servers — the target's branch tip and the earlier
    tree — start through one `site_server_port` helper; scoped re-run
    `1 scenario (1 passed)`, `6 steps (6 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): name an address that serves an earlier revision`
      (body: the scenario title).
  - Revision: a2a79fe8d04ed9f5525c69eac46b486858dc03a4

### T16 — A hand edit in the hosting target is removed by the whole-content replacement

- Design constraints: one commit whose tree is exactly the eight members;
  fast-forward, single parent; the hand-added file disappears; the earlier
  page stays in the target's history.
- Domain constraints: the page is never edited in the target; whole-content
  replacement is the only write path.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A hand edit in the hosting target is removed by the whole-content replacement"`
    → `✘ And a file added to the hosting target by hand` / `Step panicked.
    Captured output: not implemented: a file added to the hosting target by
    hand`, `1 scenario (1 failed)`; exit 101.
- [x] GREEN — tree replacement, fast-forward push.
  - Production files: `apps/cli/tests/steps/support.rs`
    (`site_add_hand_file` pushes a real hand commit into the bare target,
    `site_output_members`, `site_target_git`, `site_target_parents`),
    `apps/cli/tests/steps/publish_public_website.rs` (the scenario's steps:
    the hand-added file is gone, the target's members equal the output's, and
    the new tip is one commit whose single parent is the target's previous tip)
  - Evidence: scoped run → `1 scenario (1 passed)`, `7 steps (7 passed)`,
    exit 0; the publication replaced the tree without a force push, the
    hand-added file disappeared, and the commit built from `abc1234` stayed in
    the history with its own footer text.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: every target read (`members`, `tip`, `history`) goes through one
    `site_target_git` helper, so the step bodies cannot spell the bare
    repository differently; scoped re-run `1 scenario (1 passed)`, `7 steps (7
    passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): replace the hosting target's content as a whole`
      (body: the scenario title).
  - Revision: 13eda8a79f238641106f2d98e09ee33a3308c8b4

### T17 — A refused publication into the hosting target leaves the previous page

- Design constraints: a pre-receive hook refuses the push; the publication
  exits non-zero naming the hosting target, and the target still serves the
  `abc1234` page. No force push, no retry that bypasses the hook.
- Domain constraints: minimal guarantee; the ruleset forbids non-fast-forward
  pushes.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "A refused publication into the hosting target leaves the previous page"`
    → `✘ And a hosting target that refuses the incoming content` /
    `Step panicked. Captured output: not implemented: a hosting target that
    refuses the incoming content`, `1 scenario (1 failed)`; exit 101.
- [x] GREEN — push failure propagation and named refusal.
  - Production files: `apps/cli/tests/steps/support.rs`
    (`site_refuse_pushes` now returns the hook it installed so the scenario can
    assert it, `site_page_revision` commits the page-changing revision under a
    scenario label, `site_target_url` names the target),
    `apps/cli/tests/steps/publish_public_website.rs` (the scenario's steps).
    `scripts/publish-site.sh` needed no change: step 6 already propagates the
    push failure, prints the target's own rejection log and refuses with `the
    hosting target '<target>' refused the publication`; this scenario is that
    behaviour's regression test.
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; the refusal carried the target's path and the hook's message
    `the hosting target refuses the incoming content`, and the target still
    served the `abc1234` page with its history unchanged.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: the page-changing revision is one `site_page_revision` helper
    used by the given step and the publication step, so the two fixtures cannot
    drift; scoped re-run `1 scenario (1 passed)`, `5 steps (5 passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): name a refused push and keep the previous page`
      (body: the scenario title).
  - Revision: 692011639d042734d9241ebbe5602d226c6f36c1

### T18 — Two publications that begin together are serialized and neither is refused

- Design constraints: an exclusive `flock` spans fetch/replace/push; two
  concurrent runs of the same revision both exit 0; the target ends at
  `def5678`; neither fails as a non-fast-forward. The workflow adds the
  concurrency group in `site.yml` (recorded, exercised on the host).
- Domain constraints: two publications must not race; a refused loser would
  break the trigger promise.
- [x] RED — scoped run; exit non-zero.
  - Evidence: `DUTY2CAL_SKIP_WEB_BUILD=1 ./run-tests.sh --name "Two publications that begin together are serialized and neither is refused"`
    → `✘ And two publications of the revision "def5678" that begin together`
    / `Step panicked. Captured output: not implemented: two publications of a
    revision that begin together`, `1 scenario (1 failed)`; exit 101.
- [x] GREEN — flock and serialized publish path.
  - Production files: `apps/cli/tests/steps/support.rs`
    (`site_publication_command` is the one invocation both a single run and the
    pair use, `site_publish_together` starts two publications together with
    their own output directories and the default lock and records both logs and
    exit statuses, `site_default_lock` names `.git/publish-site.lock`),
    `apps/cli/tests/steps/publish_public_website.rs` (the scenario's steps).
    `scripts/publish-site.sh` needed no change: the exclusive `flock` already
    spans fetch, the current-target comparison, the replacement and the push,
    and its default path is `$(git rev-parse --absolute-git-dir)/publish-site.lock`.
  - Evidence: scoped run → `1 scenario (1 passed)`, `5 steps (5 passed)`,
    exit 0; the pair exited 0/0, exactly one log printed
    `publish-site: published …` and the other `already current`, neither log
    carried a non-fast-forward/rejected push, the target ended at `def5678` as
    one commit on the previous tip, and `.git/publish-site.lock` existed —
    the default path, with no `--lock` argument passed by the test.
  - Unreconciled: `.github/workflows/site.yml` (and its caller `web.yml`) does
    not exist in the tree and no task in this plan creates it, so the
    `concurrency` group design.md assigns to it could not be recorded here; the
    serialization this scenario proves is the command's `flock`. Reported to
    the change owner rather than invented.
- [x] REFACTOR — re-run scoped; exit 0.
  - Evidence: a concurrent publication's log and exit status are read through
    one `site_publication_path` helper, and the single and concurrent runs share
    one command builder; scoped re-run `1 scenario (1 passed)`, `5 steps (5
    passed)`, exit 0.
- [x] COMMIT — `feat(publish-public-website): serialize concurrent publications`
      (body: the scenario title).
  - Revision: 94f725779bf3765ee1373c3b7dda70b6f130010c

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
