---
status: accepted
date: 2026-10-13
decision-makers: operator (kressnerd)
consulted: none
informed: none
---

# Publish downloadable releases for five platforms from one public repository

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
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

The release channel and the platform set are a durable deployment boundary, not a local implementation detail: they fix where the project publishes, which machines it promises to serve, and what a downloader may obtain, and every future version and every future platform promise is bound by them. `design.md` owns the concrete mechanism (the preparation command, the workflow jobs, the runners, the settings); the channel and the platform set are the architectural decision. The publication mechanics that go with the channel — draft-first, immutable releases — are the separate decision recorded as ADR-0004, so this record does not bundle them. The register already holds ADR-0001 (licence) and ADR-0002 (layer model); neither covers this boundary, so the route is a new record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort because: the per-platform support promises are recorded in the README and the documentation, and moving the public address would break the clone command and every published release link the stable latest-release address exists to preserve.
- [x] **Shapes quality attributes** - It materially affects portability and distributability because: it sets the floor of one downloadable archive per supported platform and makes each platform's executable run with no runtime installation.
- [x] **Structural** - It defines a boundary, contract, or dependency direction because: it makes one public repository the only publication boundary, fixes the five platform targets and the runners that build them, and adds build tooling outside the workspace crates.
- [x] **Must be shared** - Developers unfamiliar with it would systematically violate the architecture because: they would add a platform promise without a build-and-start evidence path, or publish from a channel other than the one public repository.

## Context and Problem Statement

The project's distribution promise was a single self-contained Windows executable attached to a version tag, which is narrower than the people who need the tool and left the release path unexercised with no public home. The operator supplied a public repository address and asked for downloadable releases for the machines crew members actually use. A public repository is free to operate and is where GitHub artifact attestations and secret scanning live. This decision fixes the release channel and the platform set; the concrete build, packaging and publishing mechanics are owned by `design.md`. The macOS archives are published unsigned — a cost choice the operator made when choosing the platform set — but that choice is not an ADR: it reverses cheaply (a paid Apple Developer account can sign and notarise later without touching this boundary), so its complete rationale lives in chapter 11 as R-010/TD-006 and in `design.md`, and it is left out of this record.

## Decision Drivers

- Crew members use Windows, Linux and macOS; a single Windows executable leaves the others building from source.
- A downloader must be able to trace the file in hand to the tagged revision and this project's automation.
- A public repository is free to operate: unlimited standard runners, artifact attestations on the Free plan, and automatic secret scanning.
- Every platform promise must be backed by a build and a start on a machine of that platform's kind.
- The converter itself must stay local and offline, with no network I/O and no runtime dependency.

## Considered Options

- Windows-only: keep one Windows executable attached to the version tag
- Downloadable releases for five platforms from one public repository
- A container image or a package-registry publication

## Decision Outcome

Chosen option: "Downloadable releases for five platforms from one public repository", because it is the only option that reaches every crew member's machine while keeping the file a downloader holds traceable to the released revision.

### Consequences

- Good, because one archive per supported platform is built and started on a machine of that platform's kind, so no platform promise outruns its evidence.
- Good, because every archive carries build provenance and a bill of materials, so a downloader can trace the file in hand to the released revision.
- Good, because the public repository brings the gate on landing, secret scanning with push protection, and dependency alerts with it at no cost.
- Bad, because each platform needs its own build and start evidence, so the release path is five builds wide and its correctness is provable only on the host; recorded in chapter 11 (R-009).
- Bad, because the release path's steps and tooling are pinned by mutable tags and branches — the `actions/*` steps are GitHub's own, but `dtolnay/rust-toolchain@stable` (a rolling branch, used by every build job) and `anchore/sbom-action@v0` are third-party, and `rhysd/actionlint` is a third-party container image pinned by tag on the CI side — a residual supply-chain risk with no detection control, because an attestation produced after a substituted step ran attests the substituted artefact; recorded in chapter 11 (R-011, TD-005).

### Confirmation

Every platform's archive is published only after its executable was built for that platform and started on a machine of that platform's kind (the build matrix, where the Windows executable is cross-built on Linux, plus the Windows smoke job). The preparation command's completeness and refusal rules are covered by the `publish-public-release` scenarios, and `rhysd/actionlint:1.7.12` checks the workflow's schema, expressions and known action inputs before the push. Immutability and attestation are confirmed by ADR-0004's own fitness function.

## Pros and Cons of the Options

### Windows-only executable attached to the version tag

- Good, because the release path already existed and needs no new platform work.
- Bad, because a Mac or Linux crew member has no way to obtain the tool except by building it.
- Bad, because a single build leaves the release path largely unexercised and unverifiable.

### Downloadable releases for five platforms from one public repository

- Good, because every crew member gets an archive named for their machine.
- Good, because build provenance and a bill of materials make the downloaded file checkable.
- Bad, because it needs one build per platform kind and host-only correctness evidence.

### A container image or a package-registry publication

- Good, because a container or a registry is a familiar distribution channel.
- Bad, because a container turns a local file-in/file-out tool into a mounted-volume, Linux-only runtime that no crew member's problem asks for, and a registry ships no runnable executable.

## More Information

The proposal's open questions D1 (public address `kressnerd/duty2cal`) and D4 (unsigned macOS archives, recorded as R-010/TD-006 and in `design.md`, not here); `design.md` ("Technology decisions", "The release pipeline", "The public home's owner-applied settings", risk R-009); chapters 2, 3, 7, 11 and 12; ADR-0004 records the draft-first immutable publication decision; ADR-0001 and ADR-0002 cover the licence and the layer model and are unaffected by this decision.
