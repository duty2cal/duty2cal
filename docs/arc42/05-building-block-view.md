# 5. Building Block View

## Level 1 — System overview

```mermaid
flowchart TB
  CLI["apps/cli — driving adapter (argv → use case → exit code); fs module publishes"]
  WASM["ffi/wasm — wasm-bindgen driving adapter over the same use case"]
  WEB["web/ — static page, ES modules, offers; no backend"]
  PDF["crates/duty2cal-pdf — pdfplumber → RosterSource"]
  CORE["crates/duty2cal-core — lib"]
  CLI --> CORE
  CLI --> PDF
  WASM --> CORE
  WASM --> PDF
  WEB -->|"inline WASM bytes"| WASM
  PDF --> CORE
```

| Building block | Responsibility |
|---|---|
| `apps/cli` (package `duty2cal`) | Parse the invocation, invoke the conversion, print the reported path or failure, set the exit status, and publish the calendar or the diagnostic (`fs` module). |
| `ffi/wasm` (package `duty2cal-wasm`) | A thin `wasm-bindgen` driving adapter: accepts the filename and bytes, wires the unchanged `ConvertRoster` with a browser clock, and returns calendar text or a structured failure. Target-scoped to `wasm32-unknown-unknown`; performs no filesystem, network or environment I/O. |
| `web/` | The converter page: a header with the project icon as a 48×48 mark left of the heading (and as the favicon via `icon.png`), a statements panel above the converter panel (what it does, the browser route, downloads for various operating systems on request with the release page linked, privacy, non-affiliation, as-is), capability gate, file picker and drop zone, 20 MiB bound, conversion call, status/failure panels, Blob offers, a "Source and licence" panel below the converter panels, and a full footer carrying the Built revision and the plain-prose non-affiliation, as-is and privacy statements. No backend, no upload, no persistent storage. |
| `crates/duty2cal-pdf` | The only crate that depends on `pdfplumber`; produces the `RawRosterTable` anti-corruption input. |
| `crates/duty2cal-core` | Pure domain, application ports + `ConvertRoster`, and the dependency-free ICS adapter. |

## Browser build and evidence toolbox (outside the product crates)

The browser route adds a build step and a real-browser test driver. They are
test/build tooling; the product page and facade remain the blocks above.

```mermaid
flowchart TB
  BUILD["scripts/build-web.sh — pinned wasm-pack, web/pkg, base64 inline module, licence copy"]
  RUN["scripts/run-web.sh — loopback static server for manual use"]
  DRIVER["web/e2e/driver.mjs — Playwright + Chromium, JSON-lines actions"]
  STEPS["apps/cli/tests/steps/browser.rs — driver client"]
  E2E["steps/e2e/export_duty_plan_calendar.rs — the one browser @e2e scenario"]
  REG["steps/export_duty_plan_calendar.rs — regular browser variants"]
  BUILD -->|"produces"| WEB["web/pkg"]
  RUN -->|"serves"| WEB
  E2E --> STEPS
  REG --> STEPS
  STEPS -->|"spawns"| DRIVER
  DRIVER -->|"navigates, sets files, clicks, captures"| WEB
```

| Block | Responsibility |
|---|---|
| `scripts/build-web.sh` | Install the pinned `wasm-pack`, build `web/pkg`, and generate the base64 `wasm-inline.js`. |
| `scripts/run-web.sh` | Build the Site output and serve `dist/site/` on loopback for manual use. |
| `web/e2e/driver.mjs` | Serve the Site output named by `DUTY2CAL_SITE_ROOT` on loopback and drive Chromium: perform actions, read rendered state/request log/storage, capture downloads and screenshots, and fail closed on an unknown action or a missing root. |
| `apps/cli/tests/steps/browser.rs` | The Rust client for the driver protocol; shared by the `@e2e` and regular browser step files. |
| `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` | The one browser `@e2e` scenario, alongside the existing CLI e2e steps. |
| `apps/cli/tests/steps/export_duty_plan_calendar.rs` | The regular browser variants (size, family, stale offer, drop, capability, startup, offer failures, cancelled save, naming, persistence). |

## Site publication toolbox (outside the product crates)

The Public website's publication adds blocks beside the workspace. They are
build-and-publish tooling: they change nothing about the product blocks above,
and the page they publish is the same `web/` page.

```mermaid
flowchart TB
  ASSEMBLE["scripts/build-site.sh — assemble dist/site (page, icon, script, capability gate, stylesheet, bundle, licence), stamp the Built revision"]
  CONTENT["scripts/check-site-content.sh — name each missing public statement or foreign asset"]
  EVIDENCE["scripts/site-evidence.sh — content, licensing, browser, conversion"]
  PUBLISH["scripts/publish-site.sh — status/--since decision, worktree, build, gate, replace, read back, verify the served page"]
  RELEASE["scripts/publish-release.sh — verify the target's tag, then the release"]
  WEBWF[".github/workflows/web.yml — main push (--since) and manual dispatch; no tag trigger"]
  SITEWF[".github/workflows/site.yml — reusable: toolchain, key to a 600 file, publish-site.sh"]
  RELWF[".github/workflows/release.yml — site job, then publish job (needs: site)"]
  WORKTREE["detached worktree at the publication revision"]
  SERVED["GitHub Pages: the served page (--verify-url)"]
  STEPS["apps/cli/tests/steps/publish_public_website.rs + steps/e2e/publish_public_website.rs"]
  WEBWF -->|"calls"| SITEWF
  RELWF -->|"site job calls"| SITEWF
  SITEWF -->|"runs"| PUBLISH
  RELWF -->|"publish job runs"| RELEASE
  RELEASE -->|"git ls-remote / fetch of the target"| TARGET[("Pages repository")]
  PUBLISH -->|"git worktree add --detach"| WORKTREE
  WORKTREE -->|"build-web.sh, then"| ASSEMBLE
  PUBLISH -->|invokes| ASSEMBLE
  PUBLISH -->|invokes| EVIDENCE
  EVIDENCE -->|invokes| CONTENT
  EVIDENCE -->|"real subprocess, inside the worktree"| RUNNER["run-tests.sh → features_runner → Playwright driver"]
  STEPS -->|"real subprocess"| PUBLISH
  STEPS -->|"serves the target's checked-out tree"| DRIVER["web/e2e/driver.mjs"]
  ASSEMBLE --> OUT["dist/site — the Site output"]
  PUBLISH -->|"one fast-forward commit"| TARGET
  TARGET --> SERVED
  PUBLISH -->|"read with bounded retries"| SERVED
```

| Block | Responsibility |
|---|---|
| `scripts/build-site.sh` | Assemble `dist/site/` from the checkout it is pointed at (the publication passes the revision's detached worktree) and those `web/pkg/` members, copy the project icon to `icon.png`, stamp the footer anchor from `--revision`, and copy `LICENSES/MPL-2.0.txt` as `LICENSE.txt`; the output holds exactly eight members — `index.html`, `app.js`, `capability.js`, `styles.css`, `icon.png`, `pkg/duty2cal_wasm.js`, `pkg/wasm-inline.js` and `LICENSE.txt`; refuse a missing converter bundle and name `scripts/build-web.sh`. |
| `scripts/check-site-content.sh` | Read the assembled page and name every missing public statement, a missing header icon or favicon, a missing Built revision link or a missing licence file; refuse an asset loaded from another origin. |
| `scripts/site-evidence.sh` | Run the four checks as real subprocesses in fail-fast order and name the failing one; it runs inside the revision's worktree, so the conversion suite and browser scenario are that revision's own. |
| `scripts/publish-site.sh` | Refuse an unclean caller tree (`git status --porcelain --untracked-files=no`), decide `--since` against the site-input list it owns, resolve the Built revision, create a detached worktree at it, build and gate there, report the target as current and write nothing when republishing content the target already holds, replace the hosting target's content in one fast-forward commit under an exclusive lock, read the pushed tree back, and verify the page served at `--verify-url` names the same Built revision before reporting success. |
| `scripts/publish-release.sh` | Read the hosting target's branch tip (`git ls-remote`, then a fetch) and refuse before any `gh` call unless it carries the tagged Built revision; then create, inspect and publish the draft release through `gh`, comparing the asset set while excluding `release-notes.md`; it takes `--site-repo DIR` for the target it reads. |
| `.github/workflows/web.yml` | The trigger workflow only: every default-branch push (passing `github.event.before` as `--since`) and manual dispatch, with no `v*` tag trigger; it calls the reusable `site.yml` with `fetch-depth: 0` and holds no build, gate or publish step of its own. |
| `.github/workflows/site.yml` | The reusable publication workflow (`on: workflow_call`, inputs `revision` and `since`, secret `PAGES_DEPLOY_KEY`): prepares the gate's toolchain, writes the key to a `600` file and runs `scripts/publish-site.sh` with `--verify-url`; its concurrency group is the site publication's serialization. |
| `.github/workflows/release.yml` | A `site` job that calls `site.yml` for the tag (`fetch-depth: 0`, `revision` = the tag, no `since`), then a `publish` job (`needs: site`) that keeps the SBOM and attestation steps and runs `scripts/publish-release.sh`; the `publish` job no longer publishes the site itself. |
| `apps/cli/tests/steps/publish_public_website.rs`, `apps/cli/tests/steps/e2e/publish_public_website.rs` | The capability's step definitions: the regular refusal and replacement scenarios, and the one `@e2e` that publishes into a local target and drives the published tree in Chromium. |

## Release toolbox (outside the product crates)

The release channel adds blocks that sit beside the workspace, not inside it. They are build tooling and repository operation; they change nothing about the product blocks above.

```mermaid
flowchart TB
  PREP["scripts/prepare-release.sh — completeness, deterministic archives, checksums, release text"]
  RENDER["scripts/render-changelog.sh — renders a version's section and the change log; --check refuses drift"]
  CONFIG["cliff.toml — capability allow-list, machinery phrases, user-receiving paths, group order, section templates"]
  LOG["CHANGELOG.md — generated change log; its newest section is the release text"]
  DCO["scripts/dco-check.sh — sign-off over a pull request's commit range"]
  GATE["scripts/await-green-checks.sh — bounded wait for the tagged revision's checks"]
  CI[".github/workflows/ci.yml — the gate; actionlint, dco, public-text and changelog jobs"]
  REL[".github/workflows/release.yml — build matrix, Windows smoke, prepare, publish, verify"]
  COMM["community files — CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, issue and PR templates"]
  STEPS["apps/cli/tests/steps/publish_public_release.rs + steps/e2e/publish_public_release.rs"]
  MEASURE["scripts/measure_executable_size.py — two builds, strip disabled for the attribution only"]
  REPORT["attribution.json + attribution.md — revision, platform, crate/function split, stripped file size"]
  STEPS9["apps/cli/tests/steps/measure_distributed_executable_size.rs + steps/e2e/..."]
  REDUCE["scripts/measure_size_lever.py — baseline and cumulative lever builds"]
  LEVER["size-lever.json + size-lever.md — baseline, cumulative delta, status"]
  STEPS10["apps/cli/tests/steps/reduce_distributed_executable_size.rs + steps/e2e/..."]
  REL -->|invokes| PREP
  PREP -->|invokes| RENDER
  REL -->|publish job invokes| GATE
  CONFIG -->|configures| RENDER
  RENDER -->|writes| LOG
  CI -->|invokes| DCO
  CI -->|invokes| RENDER
  CI -->|"windows-gnu-build invokes"| MEASURE
  CI -->|actionlint validates| REL
  STEPS -->|real subprocess| PREP
  STEPS -->|real subprocess| RENDER
  STEPS -->|real subprocess| GATE
  STEPS9 -->|real subprocess| MEASURE
  MEASURE -->|writes| REPORT
  STEPS10 -->|real subprocess| REDUCE
  REDUCE -->|writes| LEVER
  REDUCE -->|"measures the release profile"| PRODUCT["Cargo.toml: panic = abort"]
```

| Release block | Responsibility |
|---|---|
| `scripts/prepare-release.sh` | The preparation contract: refuse a set missing the Windows x86_64 executable or carrying an executable for a platform outside the supported set, stage exactly one deterministic Windows `.tar.gz` archive and its checksum, and stage the change log and the release text the renderer produces. |
| `scripts/render-changelog.sh` | Render one version's section and the change log (and the release text) from the repository's history with the pinned generator; `--check` renders in memory and refuses a committed change log that differs from the released revision, naming the version whose section differs. The section's date is the released revision's own committer date — the `v<version>` tag when it exists, and otherwise `HEAD`, the revision the tag will point at — never the clock, so a section is renderable before its tag exists. It also refuses a version with a leading `v`, a repository with no `HEAD` to date the section from, a generator that is not the pinned version, an allow-list that differs from `givn/specs/`, a marked machinery skip list the configuration does not realize, a listed user-receiving path the tree does not hold, an announced revision whose changed paths cannot be read, and a render that names no change a user receives. An entry whose revision changed no user-receiving path is left out of the section, and a `### ` group heading left without a bullet is pruned before the visible count and the written section. |
| `cliff.toml` | The one configuration the project owns: the capability allow-list (the released capability scopes), the dependency-update parser for `chore(deps):` revisions, the marked machinery skip list (the project's own tooling words the allow-list cannot see), the marked list of the paths a user receives (which decides what a released revision must have changed to be announced), the group order (including the Dependencies group for `chore(deps):` updates), the skip rules and the templates that render a section, including the per-entry revision marker the renderer resolves and strips. Passed to the generator explicitly so a developer's global configuration cannot change a render. |
| `CHANGELOG.md` | The generated change log: the newest section is the version being released, every earlier section stays verbatim, and the first section back-fills `0.1.0`; published among a release's assets. |
| `scripts/dco-check.sh` | Check DCO sign-off over a pull request's commit range; run by the `dco` CI job on pull requests only. |
| `scripts/await-green-checks.sh` | The publication gate: resolve every release-workflow check suite on the tagged revision, wait (bounded) for the remaining check runs to conclude, and refuse a concluded non-success, no success, no check evidence, an unreadable source, or the bound, naming the checks. |
| `scripts/measure_executable_size.py` | The measurement: resolve a named revision into a temporary worktree, build the shipped artifact with the committed configuration, build the same revision again with `strip` disabled for that build only in a separate target directory, attribute it by crate and by function with the pinned generator, read the stripped artifact's file size, digest and section sizes, and write one report naming the revision, the platform and the build's identity. A failed build or a build that keeps no names yields no attribution; an unreadable file size leaves the report incomplete; nothing is inferred from the archive or a host build. |
| `scripts/measure_size_lever.py` | The reduction measurement: resolve the revision into a worktree, rebuild the unchanged release configuration (`panic = "unwind"`, `opt-level = "z"`) as the baseline, build each candidate lever cumulatively, measure the stripped executable's file size, record the delta and status, refuse a nightly/UPX/dynamic-linking proposal before a build, report the published binary's size separately, and preserve the accepted executable for the failure-path run. |
| `size-lever.json` and `size-lever.md` | The reduction report: baseline, cumulative rows, applied/rejected statuses, the published binary's size and the recorded panic/conversion outcome. |
| `apps/cli/tests/steps/reduce_distributed_executable_size.rs`, its e2e file and the `@e2e` transcript | The reduction scenarios drive the real command, assert the report, and run a host lever build on an unsupported source to prove the numbered diagnostic; the Windows executable is measured and started by the Windows smoke job. |
| `.github/workflows/release.yml` | On a `v*` tag: cross-build the Windows x86_64 executable on Linux, smoke that executable on Windows, prepare the staged set over the one binary, wait for the tagged revision's checks to conclude green, publish the immutable release with provenance and a bill of materials, then verify the published release by reading it back. |
| `.github/workflows/ci.yml` | The gate on landing; the publishing change added the `actionlint` and `dco` jobs, and the change to generate release notes adds the `changelog` job (the pinned generator plus the drift check) and puts the pinned generator on the `test` job's `PATH`, because the suite's change-log scenarios drive the renderer as a real subprocess. The size-attribution change adds the measurement step to the `windows-gnu-build` job and puts the Windows target, `mingw-w64` and the pinned generator on the `test` and `coverage` jobs so the suite's `@e2e` scenario drives the same command. |
| `attribution.json` and `attribution.md` | The report a measurement writes, not a committed file: the machine-readable and human renderings of one dataset. They name the revision, the platform and the build's identity, keep the crate/function split and the stripped executable's file size as separate fields, and carry the status `complete`, `incomplete` or `unreadable`. The gate's Windows job uploads the pair beside the run for the revision it built; the release maintainer can request the same report for any named revision. |
| Community files (`CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `.github/ISSUE_TEMPLATE/*`, `.github/pull_request_template.md`) | The contributor routes: contribution and sign-off rules, conduct, the confidential reporting route, and issue and pull-request forms that ask for the export family and a stripped sample, never a real plan. |
| `apps/cli/tests/steps/publish_public_release.rs` and `apps/cli/tests/steps/e2e/publish_public_release.rs` | The step definitions for this capability; the `@e2e` file drives `scripts/prepare-release.sh` and `scripts/render-changelog.sh` as real subprocesses, asserts the staged Windows archive and checksum, extracts it and inspects its only member and bytes, and reads the newest section out of the written change log. The real Windows executable is started by the `smoke-windows` job, not by the Linux suite. |
| `apps/cli/tests/steps/measure_distributed_executable_size.rs`, `apps/cli/tests/steps/e2e/measure_distributed_executable_size.rs`, and `apps/cli/tests/fixtures/size-attribution/` | The measurement capability's step definitions and its two-crate fixture workspace. The scenarios drive the real command as a subprocess against real git repositories, real cross-builds and the real generator; the `@e2e` scenario commits its terminal transcript. |

## Public-home declaration toolbox (outside the product crates)

The public home's configuration adds blocks beside the workspace. They are
repository operation and its tests, not product blocks.

```mermaid
flowchart TB
  DECL["infra/ — declaration (organization, repositories, both rule sets, Pages source, security settings, pages publish key/secret), two adoption batches, encrypted state"]
  PROFILE["infra/org_profile.tf — profile repository + profile and security-policy file resources"]
  PUBLISH["infra/pages_publish.tf — generated key pair, pages deploy key, PAGES_DEPLOY_KEY secret, pages rule set"]
  ORG["org/ — reviewed profile content and default security policy (CC0-1.0)"]
  ADOPT["scripts/adopt-public-home.sh — preflight, staged imports, refresh verification, atomic binding"]
  APPLY["tofu plan -out=plan.bin → tofu apply plan.bin — the preview and the configuration change"]
  SETUP["scripts/setup-infra-tests.sh — tool checks, provider mirror, fixture proof"]
  FAKE["scripts/fake-github/serve.py — fixtures, request journal, fault injection, opt-in write sandbox (create, contents, key, secret and rule-set overlays)"]
  STEPS5["steps/adopt_existing_configuration.rs + steps/e2e/..."]
  STEPS6["steps/enable_repository_security.rs + steps/e2e/..."]
  STEPS7["steps/publish_organization_profile.rs + steps/e2e/..."]
  STEPS8["steps/publish_site_plumbing.rs + steps/e2e/..."]
  TOFU5["tofu + integrations/github and hashicorp/tls providers"]
  ORG -->|"file() at plan time"| PROFILE
  ADOPT --> TOFU5
  APPLY --> TOFU5
  APPLY --> PROFILE
  APPLY --> PUBLISH
  ADOPT --> DECL
  APPLY --> DECL
  STEPS5 -->|real subprocess| ADOPT
  STEPS6 -->|"real subprocess (init, plan, apply)"| APPLY
  STEPS7 -->|"real subprocess (init, plan, apply)"| APPLY
  STEPS8 -->|"real subprocess (guard reads, init, plan, apply)"| APPLY
  STEPS5 -->|starts| FAKE
  STEPS6 -->|"starts with --writes"| FAKE
  STEPS7 -->|"starts with --writes"| FAKE
  STEPS8 -->|"starts with --writes"| FAKE
  STEPS7 -->|"raw TCP reads"| FAKE
  STEPS8 -->|"raw TCP reads (guard)"| FAKE
  TOFU5 -->|"GITHUB_BASE_URL"| FAKE
  SETUP -->|mirrors providers| TOFU5
```

| Block | Responsibility |
|---|---|
| `infra/` | The declared configuration: `github_organization_settings`, the monorepo and Pages `github_repository` resources (the monorepo's `security_and_analysis` included), the profile repository and its two `github_repository_file` resources in `org_profile.tf` (source resolved by `variable "org_source_dir"`, whose default is `../org`), `github_repository_ruleset.main` (the existing `main protection` rule set), `github_repository_pages.pages`, `github_repository_vulnerability_alerts.mono` / `github_repository_dependabot_security_updates.mono`, and — in `pages_publish.tf` — `tls_private_key.pages_publish`, `github_repository_deploy_key.pages_publish`, `github_actions_secret.pages_publish` and `github_repository_ruleset.pages_publish`; the two adoption batches; the encrypted state and its backup; the two provider lock entries. |
| `org/` | The reviewed public copy: the profile content (`profile/README.md`) and the default security policy (`SECURITY.md`), both CC0-1.0 and covered by a `REUSE.toml` override. The profile repository is output; these files are the source. |
| `scripts/adopt-public-home.sh` | The adoption interface: validate the declaration and passphrase, import the selected batch into a staged copy of the state, verify the batch with a refresh-only plan, then bind atomically and report one line per object, or refuse and name the object. It stages the declaration without `org_profile.tf`, because the profile repository is created, never adopted, so the refresh-only verification never evaluates the profile files. |
| The apply | The declaration's normal mechanism, no wrapper: a read-only creation guard (`gh api` reads of the pages repository's deploy keys and rule sets and the monorepo's Actions secrets, with `jq` verdicts), `tofu init`, `tofu plan -out=plan.bin` (the preview; `plan.bin` is encrypted and git-ignored), a read-only refresh-only report, the guard again immediately before the write, `tofu apply plan.bin` (the configuration change), then a fresh plan that must be empty. A create-only revision is classified after a failure by live reads, not by the failed state. The state is committed only after a green apply. |
| `scripts/setup-infra-tests.sh` | Prepares the offline test environment: `tofu`, `python3` and `jq` checks, the mirror and `terraformrc` for both locked providers (`integrations/github`, `hashicorp/tls`), and the one-import proof that the fixture set satisfies the provider. The proof workspaces omit `org_profile.tf`, because they cover only the existing/adopted objects; the inherited-base workspaces also omit `pages_publish.tf`, because the plumbing is created, not inherited. |
| `scripts/fake-github/serve.py` and its fixtures | The digital twin of GitHub's REST API: deterministic JSON fixtures per scenario, a journal of every request, and fault injection for unreadable, missing and changed objects, including faults that fire only after a stated number of requests on a path; `--writes` adds an opt-in sandbox that accepts the repository `PATCH`, the vulnerability-alerts and Dependabot-security-updates `PUT`s, organization-repository creation, repository-contents `PUT`s, the deploy-key create, the Actions-secret write and the rule-set create, with branch, commit, key, secret and rule-set overlays served back to the provider's reads. |
| `apps/cli/tests/steps/adopt_existing_configuration.rs`, `apps/cli/tests/steps/e2e/adopt_existing_configuration.rs` | The adoption capability's step definitions; they drive the adoption command as a real subprocess against the fake. |
| `apps/cli/tests/steps/enable_repository_security.rs`, `apps/cli/tests/steps/e2e/enable_repository_security.rs` | The security-settings capability's step definitions; they prime the state with the real adoption command, then drive `tofu init`, `plan` and `apply` as real subprocesses against the fake in `--writes` mode. |
| `apps/cli/tests/steps/publish_organization_profile.rs`, `apps/cli/tests/steps/e2e/publish_organization_profile.rs` | The profile capability's step definitions; they copy `org/` beside the scenario's `infra/`, prime the same state, remove `org_profile.tf` (and now `pages_publish.tf`) for the base priming and restore it, drive the preview and the apply as real subprocesses, and read the published repository and contents back from the fake with raw loopback requests. |
| `apps/cli/tests/steps/publish_site_plumbing.rs`, `apps/cli/tests/steps/e2e/publish_site_plumbing.rs` | The publishing-plumbing capability's step definitions; they prime the inherited state (both adoption batches plus the inherited declaration's real apply), restore `pages_publish.tf`, compute the creation guard's verdicts from the fake's deploy-key, secret and rule-set lists, then drive the targeted plan, the refresh-only report and the apply as real subprocesses and read the created key, sealed secret and rule set back from the fake. |

## Level 2 — Key building blocks

### `duty2cal-core` modules

**Responsibility:** everything that is host-independent and testable without I/O.

| Element | Responsibility |
|---|---|
| `domain` | `Roster` aggregate, `DutyEntry` values, the one rule that reads a travel leg's printed crew position and activity together and composes the travel title (including the conditional ` Report <value> LT` report suffix), and the pure timing rules (`timing`). No I/O, no clock. |
| `application` | Ports (`RosterSource`, `CalendarSink`, `Clock`), the `ConvertRoster` use case, `RawRosterTable` as the anti-corruption intermediate, and the row interpretation that classifies activities and pairs travel fragments by their two printed fields. |
| `adapters/ics` | RFC 5545 serializer — the `CalendarSink` used by every facade. |

### `duty2cal-pdf`

**Responsibility:** header detection → X bands → Y clustering → `RawRosterTable`; error mapping to a crate-local `SourceError` (the orphan rule forbids `From<pdfplumber::Error>`).

### `apps/cli`

**Responsibility:** composition root and native-only concerns.

| Element | Responsibility |
|---|---|
| `main` | argv handling, wiring, exit status. |
| `fs` | unused-path allocation for calendars and numbered diagnostics. |

### `ffi/wasm`

**Responsibility:** the browser driving adapter over the unchanged application.

| Element | Responsibility |
|---|---|
| `convert(filename, bytes)` (wasm export) | Wire `ConvertRoster::new(PdfRosterSource, IcsSink, BrowserClock).execute(bytes)` and return `{ status: "ok", calendar }` or `{ status: "failure", reason, diagnostic }`. |
| `diagnostic_text(filename, reason)` (wasm export) | Call the shared `duty2cal_core::application::diagnostic_text`, so the page's pre-conversion size failure uses the same two lines as the executable. |
| `BrowserClock` / `NativeClock` | Implement the existing `Clock` port with `js_sys::Date` or `OffsetDateTime::now_utc`, so the crate lints natively and the source's Created footer stays preferred. |

The crate is `cdylib` + `rlib`; `wasm-bindgen`, `js-sys` and `getrandom` are
target-scoped, and the native `rlib` compiles for fmt, clippy and the Windows
workspace build.

### `web/`

**Responsibility:** the rendered converter screen and its browser rules.

| Element | Responsibility |
|---|---|
| `index.html` | Title, a flex header row carrying the project icon as a 48×48 mark left of the `h1` and subheadline with the alt text `duty2cal` and a `<link rel="icon" href="./icon.png">` favicon, the statements panel above the converter panel (what the tool does, the browser route, downloads for various operating systems on request with `https://github.com/duty2cal/duty2cal/releases` linked, privacy, non-affiliation, as-is), drop zone and picker markup hidden until ready, status and failure panels, a "Source and licence" panel below the converter panels, and a full footer carrying the Built revision and the plain-prose statements; meta CSP including `img-src 'self'`. |
| `capability.js` | Classic deferred check for WebAssembly and ES modules; renders the cannot-run message and leaves the picker hidden. |
| `app.js` | ES module: `init(bytes)` from the inline bundle, `acceptFile` (clear prior offer, size bound, read, convert), state machine, offer/object-URL lifecycle, failure/diagnostic fallback, no storage or network. |
| `styles.css` | Design tokens, layout, breakpoints, states and reduced-motion rules. |
| `pkg/` (generated, ignored) | `wasm-pack` output plus the base64 `wasm-inline.js`; never committed. |

### Deferred (roadmap)

`ffi/uniffi` and native UIs — driving adapters over the unchanged application; neither exists yet, and creating one is a later change. `ffi/wasm` is no longer on this list: the browser converter built it, and the Site publication now serves it.
