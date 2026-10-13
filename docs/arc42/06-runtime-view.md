# 6. Runtime View

## Scenario: Successful conversion

**Trigger:** A crew member supplies a supported duty plan.

```mermaid
sequenceDiagram
  participant CLI
  participant App as ConvertRoster
  participant PDF as PDF extraction
  participant Dom as Domain
  participant FS as Filesystem publication
  CLI->>App: convert(source path)
  App->>PDF: read source, build snapshot
  PDF-->>App: positioned content + revision footer
  App->>Dom: classify, resolve dates, compose descriptions
  Dom-->>App: complete Roster (or rejection)
  App->>FS: allocate unused calendar path + write
  FS-->>App: published path
  App-->>CLI: Published { path }
  CLI-->>CLI: print the new path, exit 0
```

**Steps:**

1. The CLI reads the source path and calls the orchestrator.
2. The orchestrator recognizes the family, builds the source snapshot, and extracts positioned content and the revision footer.
3. The domain classifies activities, resolves each UTC endpoint's date, composes descriptions and events, and enforces completeness.
4. On completeness, the orchestrator asks the publication port for an unused calendar path and writes the calendar.
5. The CLI prints the actual new path and exits zero.

**Notes:** No side effect occurs before completeness; the source file is never modified.

## Scenario: Interpretation failure with a diagnostic

**Trigger:** A required activity, date, fragment, or source family cannot be resolved.

```mermaid
sequenceDiagram
  participant CLI
  participant App as ConvertRoster
  participant Dom as Domain
  participant Diag as Diagnostic writer
  CLI->>App: convert(source path)
  App->>Dom: resolve
  Dom-->>App: rejection reason
  App->>Diag: allocate unused numbered report + write
  Diag-->>App: report path (or failure)
  App-->>CLI: Rejected { reason, diagnostic }
  CLI-->>CLI: print failure, exit non-zero
```

**Steps:**

1. The orchestrator reaches a rule that cannot be resolved.
2. It returns the primary reason and attempts a numbered diagnostic naming the source and reason.
3. The CLI prints the failure; when a report could not be saved, it states that no new report was written and never presents an older report as current.

**Notes:** Existing calendars and reports are unchanged. The diagnostic path is reported only after a successful write.

## Scenario: Publication failure

**Trigger:** Activities resolve, but the complete calendar cannot be written.

```mermaid
sequenceDiagram
  participant CLI
  participant App as ConvertRoster
  participant FS as Filesystem publication
  participant Diag as Diagnostic writer
  CLI->>App: convert(source path)
  App->>FS: publish complete calendar
  FS-->>App: write failure
  App->>Diag: attempt numbered diagnostic
  Diag-->>App: report path or failure
  App-->>CLI: Rejected { reason, diagnostic }
  CLI-->>CLI: print failure, exit non-zero
```

**Steps:**

1. The calendar is composed but cannot be published.
2. The orchestrator exposes the failure and independently attempts a diagnostic.
3. The CLI reports the failure and the diagnostic outcome; existing files are untouched.

**Notes:** Completeness and publication are distinct: a completed composition that fails to publish is not a success.

## Scenario: A version tag becomes a verified published release

**Trigger:** The maintainer decides a version is ready and pushes its `v*` tag.

```mermaid
flowchart TB
  tag["Maintainer pushes a v* tag"] --> rel["release.yml"]
  rel --> build["build matrix: five platforms, each built and started on a machine of its own kind"]
  rel --> smoke["smoke-windows on windows-latest: start the cross-built executable"]
  build --> prep["prepare on ubuntu-latest: run scripts/prepare-release.sh over the five binaries"]
  smoke --> prep
  prep -->|"incomplete or unrecognised set"| nostage["nothing staged, no release"]
  prep --> pub["publish on ubuntu-latest: generate the SBOM, attest every archive, create the draft release, check its asset set, then publish"]
  pub --> ver["verify: download the assets back, compare checksums, run gh attestation verify, assert immutable"]
  ver --> done["verified immutable release, reachable at the stable latest address"]
```

**Steps:**

1. The tag starts `release.yml`; the build matrix builds and starts each platform's executable where it must run, and the Windows smoke job starts the cross-built executable on `windows-latest`.
2. The `prepare` job runs `scripts/prepare-release.sh` over the five binaries; it refuses and stages nothing when a supported platform is missing or unrecognised.
3. The `publish` job generates the SBOM, attests every archive, creates a draft release with the archives and the release text, checks the draft's asset set against the staged set, and publishes it.
4. The `verify` job reads the published release back: it compares every checksum, runs `gh attestation verify` on each archive, and asserts the release reports `immutable: true`.

**Notes:** Publication cannot be undone, so every check that can precede publication does, and the one that cannot — reading the public record — fails the run loudly.

## Scenario: A pushed change reaches the gated default branch

**Trigger:** A change is pushed, or a pull request is opened.

```mermaid
flowchart TB
  push["Maintainer pushes a change or opens a pull request"] --> ci["ci.yml: formatting, lint, tests, dependency licences, SPDX headers, third-party notices drift, private-data guard, coverage, Windows cross-build, actionlint; dco on pull requests"]
  ci -->|"any check fails"| blocked["the change cannot land"]
  ci -->|"all checks pass"| rules{"required ruleset on main"}
  rules -->|"checks green"| land["the change lands on the default branch"]
  rules -->|"maintainer bypass"| land
```

**Steps:**

1. Every push and pull request runs the project's checks; the pull-request-only `dco` job additionally checks sign-off over the commit range.
2. A change whose checks are not green cannot land: the default branch's ruleset requires the CI status checks and blocks force pushes and deletion.
3. The maintainer keeps an explicit bypass route for an urgent correction.

**Notes:** The gate is enforced by the repository, not by habit, and a reader sees the outcome on the revision they are looking at.
