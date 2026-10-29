---
status: accepted
date: 2026-10-13
amended: 2026-10-17
decision-makers: operator (kressnerd)
consulted: none
informed: none
---

# Publish downloadable releases for Windows x86_64 from one public repository

> **Amended 2026-10-17** by the `windows-only-release` change: the supported
> platform set is narrowed from five platforms to Windows x86_64. The release
> channel, the one-repository boundary, and the provenance, bill-of-materials
> and immutability promises are unchanged. This record is amended in place, not
> superseded.

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: AMEND_ADR
canonical_artifact: null
target_adr: docs/arc42/adr/ADR-0003-release-channel-and-platforms.md
replacement_adr: null

This is a refinement of the existing release-channel and platform-set
decision, not an independent decision: the boundary is the same one ADR-0003
already owns, narrowed from five downloadable platforms to one. The concrete
preparation command, workflow matrix, runner mechanics and public-text gate are
owned by the change's `design.md`; the durable promise about which machines the
project serves belongs here. The existing-ADR check searched the active register
(ADR-0001 licence, ADR-0002 layer model, ADR-0003 release channel and platform
set, ADR-0004 immutable draft-first publication) and the archived register
(there is no `docs/arc42/adr/archive/` directory). ADR-0003 owns this boundary
and ADR-0004 owns the publication mechanics, so the route is `AMEND_ADR` with
ADR-0003 as the target; no new ADR is minted and nothing is superseded.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort
      because: the narrowed platform set is recorded in the README, the issue
      form, the security policy and the architecture documentation, and a
      platform returns only through a later change that re-adds its build and
      start evidence. Undoing the promise means rewriting the public text and
      restoring the release matrix together.
- [x] **Shapes quality attributes** - It materially affects portability and
      distributability because: it sets exactly one downloadable executable —
      Windows x86_64 — for the audience the project serves, while the source
      remains buildable for other platforms with a Rust toolchain.
- [x] **Structural** - It defines a boundary, contract, or dependency direction
      because: it fixes the single build-and-start path (cross-build the Windows
      executable on Linux, start it on Windows) and the one archive a version
      may attach, and refuses any executable for a platform outside that set.
- [x] **Must be shared** - Developers unfamiliar with it would systematically
      violate the architecture because: they would add a platform promise
      without a build-and-start evidence path, or attach an archive for a
      platform the project does not support.

## Context and Problem Statement

The earlier decision offered five downloadable platforms — Windows x86_64,
Linux x86_64 and arm64, and macOS arm64 and x86_64 — so every crew member could
obtain an archive named for their machine. In practice the project serves only
Windows users: the Linux and macOS archives were never downloaded, yet every
release built them, started them on their own runners, attested them, attached
them and kept them on the public release page. The operator's instruction of
2026-10-17 narrows future releases to the one platform the project actually
serves, while leaving already-published releases immutable and keeping Linux as
the CI, coverage, local test and Windows cross-build host. The concrete build,
packaging and publishing mechanics remain owned by `design.md`.

## Decision Drivers

- Windows x86_64 is the audience the release serves; the Linux and macOS
  archives were never downloaded.
- Every platform promise must be backed by a build and a start on a machine of
  that platform's kind, so an unused platform consumes a runner, a build, a
  start-evidence job and its own provenance for no user.
- A crew member on any other platform still needs a route: the source can be
  built with a Rust toolchain, and other platforms are stated to be
  unsupported, untested and undistributed.
- Linux remains the CI, coverage and local test host and the host that
  cross-builds the Windows executable; it is not a published platform.
- A downloader must be able to trace the file in hand to the tagged revision
  and this project's automation.
- A public repository is free to operate: standard runners, artifact
  attestations on the Free plan, and automatic secret scanning.
- The converter itself must stay local and offline, with no network I/O and no
  runtime dependency.

## Considered Options

- Windows x86_64 only, built for the audience the release serves
- The five-platform set from the earlier decision
- Source-build only, with no downloadable archive

## Decision Outcome

Chosen option: "Windows x86_64 only, built for the audience the release
serves", because it reaches the platform the project actually serves while
keeping every remaining platform promise backed by build and start evidence.

### Consequences

- Good, because one archive per version is cross-built and started before
  publication, so the published platform promise does not outrun its evidence.
- Good, because every archive carries build provenance and a bill of materials,
  so a downloader can trace the file in hand to the released revision.
- Good, because the public repository brings the gate on landing, secret
  scanning with push protection, and dependency alerts with it at no cost.
- Bad, because the release path is now a single build plus a single
  `windows-latest` smoke start, so its correctness is still provable only on
  the host; recorded in chapter 11 (R-009).
- Bad, because users on Linux, macOS and any other platform lose the download
  route, are unsupported and untested, and their source build with a Rust
  toolchain is not verified by CI; recorded in chapter 11 (R-019, R-013).
- Bad, because the release path's steps and tooling are pinned by mutable tags
  and branches — the `actions/*` steps are GitHub's own, but
  `anchore/sbom-action@v0` is third-party, and `rhysd/actionlint` is a
  third-party container image pinned by tag on the CI side — a residual
  supply-chain risk with no detection control, because an attestation produced
  after a substituted step ran attests the substituted artefact; recorded in
  chapter 11 (R-011, TD-005). *Corrected 2026-10-29 by `developer-setup`: the
  compiler is pinned in `rust-toolchain.toml`, and no job uses the rolling
  `dtolnay/rust-toolchain@stable` step; the decision above is unchanged.*

### Confirmation

Every release archive is published only after the Windows x86_64 executable was
cross-built for `x86_64-pc-windows-gnu` and started by the `smoke-windows` job
on `windows-latest`. The preparation command's completeness, readability and
unsupported-platform refusals — including the previously offered
`linux-x86_64` — are covered by the `publish-public-release` scenarios. The
README's Windows archive name, checksum and provenance commands, source-build
and platform-request statements, and the absence of non-Windows archive names
and download rows are enforced by `scripts/public-text-check.sh`, and
`rhysd/actionlint:1.7.12` checks the workflow's schema, expressions and known
action inputs before the push. Immutability and attestation are confirmed by
ADR-0004's own fitness function.

## Pros and Cons of the Options

### Windows x86_64 only, built for the audience the release serves

- Good, because the one archive is named for the machine the project's users
  actually have, so the release page matches the audience.
- Good, because one build and one start-evidence path are easier to inspect and
  answer for than five.
- Bad, because Linux and macOS users must build from source, and that route is
  not compiled or tested by CI.
- Bad, because a platform can return only through a later change that adds its
  build and start evidence.

### The five-platform set from the earlier decision

- Good, because every crew member could download an archive named for their
  machine.
- Good, because build provenance and a bill of materials make each downloaded
  file checkable.
- Bad, because four of the five archives were never downloaded, yet consumed
  their runners, builds, start evidence, attestations and release storage.
- Bad, because the release page and the support text promised platforms the
  project did not actually serve, and the five-platform evidence looked broader
  than the user base.

### Source-build only, with no downloadable archive

- Good, because there is no per-platform archive to build, attest or maintain.
- Bad, because the Windows audience the project serves would have to install a
  Rust toolchain instead of downloading a runnable executable.
- Bad, because it discards the checksummed, provenance-bearing, immutable
  download route the release channel exists to provide.

## More Information

The `windows-only-release` change's `design.md`, proposal and delta
`publish-public-release` specification; chapters 2 (the Windows-only constraint
and the Linux build/test-host context), 3 (the one-archive release-channel
edge), 7 (the one artefact and its runner pair), 11 (R-009, R-013 and R-019)
and 12 (Supported platform, Release archive); ADR-0004 records the draft-first
immutable publication decision. Already-published releases keep their original
assets and links, including their Linux and macOS archives; this narrowing
applies to future releases only. ADR-0001 and ADR-0002 cover the licence and the
layer model and are unaffected by this amendment.
