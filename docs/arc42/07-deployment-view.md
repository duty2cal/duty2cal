# 7. Deployment View

## Infrastructure level 1

The project publishes one self-contained Windows x86_64 executable, cross-built by its own automation on a GitHub-hosted Linux runner, started on a Windows runner and published as an immutable release from the public repository. A crew member on Windows downloads the archive from the stable latest-release address and runs the executable on their own machine; it touches only the local filesystem. Other platforms receive no archive and build the source with a Rust toolchain.

The browser conversion page is a second deployment realization of the same
conversion: a static Site output served over loopback for the test and manual
run. The page needs no application server, database or runtime service; its
only runtime is the browser. Publishing that built output as the Public website
is the separate Site publication, not this deployment.

```mermaid
flowchart TB
  subgraph host["GitHub-hosted runners"]
    b1["ubuntu-latest — Windows x86_64 cross-build"]
    sm["windows-latest — Windows smoke"]
    prep["ubuntu-latest — prepare-release.sh over the one Windows binary"]
    pub["ubuntu-latest — publish (wait for the tagged revision's checks, SBOM, attestations, draft release)"]
    ver["ubuntu-latest — verify the published release"]
  end
  b1 --> prep
  sm --> prep
  prep --> pub --> ver
  ver --> rel["Immutable release at the stable latest address"]
  rel --> dl["Crew member on Windows x86_64 downloads the archive"]
  dl --> run["Runs it locally; touches only the local filesystem"]
  build["scripts/build-web.sh — wasm-pack + base64 inline"] --> site["web/ Site output"]
  site --> loop["loopback static server (manual run and E2E)"]
  loop --> browser["crew member's current browser; no backend"]
  site -.->|"later Site publication"| pages["Pages repository / Public website"]
```

| Node / Environment | Description |
|---|---|
| `ubuntu-latest` | Cross-builds the Windows (`x86_64-pc-windows-gnu`) executable and also runs the single packaging, publish and verify jobs; the publish job waits for the tagged revision's checks before anything becomes public. It remains the CI, coverage and local test host but publishes nothing of its own. |
| `windows-latest` | Starts the cross-built Windows executable: the "started on a machine of that kind" evidence. |
| GitHub Releases | Serves the published Windows archive, its checksum, its provenance and bill of materials, the release text, and the change log (`CHANGELOG.md`) at a stable latest-release address. |
| Crew member's machine | Runs the downloaded executable; supplies the plan and receives the calendar or a diagnostic. |
| Local filesystem | Holds the input plan and the published calendar/report files. |
| Loopback static server | Serves the built `web/` output for the manual run and each browser scenario; no application server, database or queue. |
| Crew member's browser | Runs the page and the inline WASM; offers the calendar/diagnostic for a browser-owned save. Supported by capability (WebAssembly and ES modules), not by a version matrix. |
| GitHub Pages / Public website | The built Site output's eventual public deployment, owned by the separate Site publication; not published by this change. |

## Infrastructure level 2

### The browser conversion output

| Property | Value |
|---|---|
| Artefact | Static Site output: `index.html`, `capability.js`, `app.js`, `styles.css`, the generated `pkg/` modules and the inline WASM bytes, plus the licence file |
| Build | `scripts/build-web.sh` installs the pinned `wasm-pack`, builds the bundle and writes the base64 inline module; Node, Playwright and Chromium are test-only and pinned |
| Runtime | The crew member's current browser; no application server, database, queue or container |
| Serving | Loopback static server for the manual run and each E2E scenario; the GitHub Pages deployment is the separate Site publication |
| Network | No request to another origin after load; no third-party asset; the WASM is inline, so no `.wasm` request occurs |
| Evidence | A Playwright/Chromium `@e2e` scenario plus regular browser variants; screenshots at 360x780 and 1920x1080; the capability and startup branches are forced with init scripts |

### The Windows artefact and the runners that build it

| Artefact | Target triple | Built by |
|---|---|---|
| `duty2cal-<version>-windows-x86_64.tar.gz` | `x86_64-pc-windows-gnu` | Cross-built on `ubuntu-latest`, then started on `windows-latest`. |

The archive holds `duty2cal.exe` and nothing else; a checksum file covering the archive sits beside it on the release. Packaging into the archive happens once, on `ubuntu-latest`; the release is published and then verified on `ubuntu-latest` as well. No other platform archive is built or attached.

### The change log among the release assets

Beside the one Windows archive and its checksum (and the archive's provenance and bill of materials), a release carries the change log and its newest section: `CHANGELOG.md` is published as an asset, so a downloader reads what changed without visiting the repository, and the release page's text is that same file's newest section. Both come from one render of the released revision, so the page and the published file cannot state different things about what a version contains.

### The crew member's machine

| Property | Value |
|---|---|
| Hardware / VM | Crew member's ordinary machine |
| OS | Windows x86_64 for this executable. No archive is offered for Linux, macOS or any other platform, and their users may build the source with a Rust toolchain. The browser conversion route is separate: it needs a current browser, not this machine class. |
| Middleware / Runtime | None beyond the executable (Windows GNU statically linked; no companion DLL or runtime installer) |
| Key software deployed | One self-contained `duty2cal.exe` executable |

### Downloading and verifying

- A downloader on Windows x86_64 takes `duty2cal-<version>-windows-x86_64.tar.gz` from the stable latest-release address `https://github.com/duty2cal/duty2cal/releases/latest`, which stays reachable as later versions are published. No archive is offered for any other platform.
- They recompute the archive's digest and compare it with the published `.sha256` (`sha256sum --check` or `Get-FileHash`), so they verify what they received before extracting it.
- They then confirm the file's build provenance and bill of materials with `gh attestation verify`, run against the archive, which ties the file in hand to the tagged revision and this project's automation.
- Already-published releases keep their original assets and links; only future releases are Windows-only.

## The declared configuration and the applied state

The public home's configuration is deployed from the maintainer's workstation,
never from CI. The declaration lives in the repository; the binding between the
declaration and GitHub's objects is an encrypted state file that is committed
with it; the passphrase that opens it, and the token that authorizes the
operation, exist only in the maintainer's environment. The declaration covers
the organization settings, the monorepo, the Pages repository, the protection
rule, the repository security settings, and the public organization profile
repository with its two reviewed files. The last three of those are created by
an apply when live lacks them; the others were adopted before their first
change.

```mermaid
flowchart TB
  ws["Maintainer's workstation"]
  decl["infra/ — declaration, adoption batch, terraform.tfstate (ciphertext), .terraform.lock.hcl"]
  env["Environment only: TF_VAR_state_passphrase, GITHUB_TOKEN, TF_VAR_billing_email"]
  tool["tofu + integrations/github provider (offline mirror in tests)"]
  gh{{"GitHub REST API"}}
  ws --> env --> tool
  ws --> decl --> tool
  tool -->|"read-only import / refresh; one previewed apply"| gh
  tool -->|"writes the encrypted state"| decl
```

| Node / Environment | Description |
|---|---|
| Maintainer's workstation | Runs the adoption and later the preview, apply and local drift check; holds the passphrase and the token. |
| `infra/` | The declaration, the batch, the provider lock file and the encrypted state; committed, reviewed and public — but the state is ciphertext. |
| GitHub | The only owner of the live settings; receives a write only from a later, separately reviewed apply. |
| CI | Runs the same suite against a local fake API; holds no GitHub token and no passphrase, so it can neither apply nor report drift. |

### The organization profile repository

| Property | Value |
|---|---|
| Repository | `duty2cal/.github`, public, created by a previewed apply (it does not exist live before this change) |
| Profile page | `profile/README.md` in that repository; GitHub renders it on the organization's overview page |
| Default security policy | `SECURITY.md` at that repository's root; GitHub uses it for an organization repository that has no policy of its own; its private-reporting route is enabled manually before publication (see below) |
| Source | `org/profile/README.md` and `org/SECURITY.md` in the monorepo, reviewed and covered by REUSE as CC0-1.0; the declaration resolves them through `variable "org_source_dir"` (default `../org` beside `infra/`) |
| Licence | The page's own prose is CC0-1.0 and the project is MPL-2.0; third-party Unicode data is licensed under Unicode-3.0 and attributed with an absolute link to `LICENSES/Unicode-3.0.txt` |
| Repository features | issue tracker, projects, wiki and discussions disabled; no Pages site, no declared scanning block; a public repository's scanning statuses are GitHub's computed defaults |
| Publication mechanism | `github_repository_file` resources in `infra/org_profile.tf`, applied from the maintainer's workstation like every other declared change |
| Adoption | none; the repository is absent live, so the apply creates it and the state records the new binding |

The profile repository is output. A text change is a change to `org/`, reviewed
in the same history as the declaration, previewed and applied like any other
configuration change. The repository is never edited by hand; the declaration's
file resources replace an existing file when the reviewed content differs.

The profile page is a public presentation surface, not a system UI: GitHub
renders it, and the project neither serves nor scripts that rendering. The
page's truthfulness is a property of the reviewed `org/` text; its links are
absolute and it embeds no third-party image or widget.

### Manual operator prerequisite: private vulnerability reporting

The default security policy names the private vulnerability reporting route
`https://github.com/duty2cal/duty2cal/security/advisories/new`. That route
exists only when the repository owner has enabled private vulnerability
reporting on `duty2cal/duty2cal` (Settings → Code security and analysis); the
pinned `integrations/github` provider (6.13.0) has no resource for it. The
operator therefore enables it by hand **before** the profile is published:

1. Open `https://github.com/duty2cal/duty2cal/settings/security_analysis` as
   the organization owner.
2. Enable private vulnerability reporting.
3. Confirm the repository's "Security" tab offers "Report a vulnerability".

The setting remains outside the declaration, the applied state and the drift
check; chapter 11's R-030 records the gap. The default-security scenario asserts
only that the published policy names the route; neither the fake nor the suite
can prove the live setting, and the operator's confirmation is the acceptance
evidence that the route exists.

### Manual operator step: pinned repositories

GitHub shows up to six pinned repositories on the organization overview, and
the organization owner sets them in the organization's web settings. The pinned
`integrations/github` provider (6.13.0) exposes no resource for them, and this
change introduces no API script or other second write path. The step is
therefore durable operator documentation, not a declared object:

1. After the apply that creates the profile repository, open
   `https://github.com/organizations/duty2cal/settings/profile` as the
   organization owner.
2. Pin `duty2cal` and `duty2cal.github.io`. Up to six repositories may be
   pinned; the `.github` repository itself does not need to be pinned.
3. Save, then open the organization overview once and confirm both appear.

The step is one-time. It cannot be previewed, it is not recorded in the applied
state, and the local configuration drift check cannot report a change to it; a
later organization-settings change that clears the pins is repaired by
repeating the step. Chapter 11 records the limitation.

## Deployment consequences

- No package manager, service, or configuration file is required on the supported platform.
- The executable may be launched by path or by dragging a PDF onto it.
- One Windows archive is attached per version: the executable runs there with no runtime installer and no companion library beyond the platform's own. Other platforms receive no archive and build the source; that route is not exercised by CI.
- Windows GNU was kept over the brief's MSVC target: the Linux cross-build stays available and static linking preserves the single-executable requirement; the GNU runtime imports, the cross-build, and the Windows smoke start are release-evidence gates, not assumptions.
- Linux is the cross-build, CI and local test host, not a published platform; it does not appear on the release page.
- Already-published Linux and macOS archives stay as they were: releases are immutable, so this narrowing applies to future releases only.
- The applied state is deployed as a committed ciphertext: `infra/terraform.tfstate` and its backup are in the repository, protected by the passphrase and by OpenTofu's `enforced = true` encryption. Losing the passphrase loses the state; re-adopting rebuilds it without changing anything live.
- The security change from this workstation, after the earlier configuration change, enables secret scanning and push protection on the monorepo. The two Dependabot settings are adopted read-only first, so the apply carries one live change; a read-only refresh-only preflight refuses a live edit made since the preview before `tofu apply` runs. The state that records the change is committed only after the apply is green; a partial apply's state is marked stale beside itself and is not committed.
- The apply that publishes the profile creates `duty2cal/.github` and writes its two reviewed files. The repository is not adopted: it does not exist live, so the preview names the three creations and the apply records them. The profile and security-policy text stays in `org/` under CC0-1.0; the profile repository is output and is never edited by hand.
- Before that apply, the owner enables private vulnerability reporting on `duty2cal/duty2cal` by hand, because the pinned provider cannot declare it and the published policy names its route. The setting is not in the declaration, the applied state or the drift check.
- The declaration resolves its reviewed source through `variable "org_source_dir"`; the default is `../org` beside `infra/`. The adoption command stages the declaration without `org_profile.tf`, and the setup proof and security base workspaces omit it, so no non-profile workspace evaluates the profile files.
- The organization profile page is GitHub's rendering of that output. This change builds and locally/CI-serves the browser converter but does not publish it: the live Pages site still serves a placeholder until the Site publication runs, and the profile keeps its truthful current-release wording (the browser route becomes a published claim with that publication). No three-platform archive is released. The profile page embeds no third-party resource, so opening it issues no request beyond GitHub's own.
- Pinned repositories are a one-time manual step in the organization's web settings. They have no declared object, no state binding and no drift report; the step is recorded above and the limitation is a chapter-11 risk.
- The test environment starts the fake in `--writes` mode only for the scenarios that apply a change; it now serves repository creation, repository contents, branches and commits from an in-memory overlay. The adoption scenarios keep the read-only server and still prove that adoption issues no write.
- The administer token, the passphrase, the billing email and the transient encrypted plan file never leave the workstation; the plan file is git-ignored and deleted after the apply.
- The administer token and the passphrase are not deployed to any service: there is no apply in Actions and no configuration drift job in CI, so divergence from the declaration is found by the maintainer's local check.
- The test environment deploys no service: the fake GitHub API runs on loopback from the suite's step definitions, and the provider is installed from a local mirror.
