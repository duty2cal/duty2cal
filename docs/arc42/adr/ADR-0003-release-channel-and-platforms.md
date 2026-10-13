---
status: accepted
date: 2026-10-13
decision-makers: operator (kressnerd)
consulted: none
informed: none
---

# Publish verifiable immutable releases for five platforms from one public repository

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

The release channel and the platform set are a durable deployment boundary, not a local implementation detail: they fix where the project publishes, which machines it promises to serve, and what a downloader may verify, and every future version and every future platform promise is bound by them. `design.md` owns the concrete mechanism (the preparation command, the workflow jobs, the settings); the channel and the platform set are the architectural decision. The register already holds ADR-0001 (licence) and ADR-0002 (layer model); neither covers this boundary, so the route is a new record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort because: a published immutable release cannot be withdrawn or repaired, its tag is never reusable, and the per-platform support promises are already recorded in the README and the documentation.
- [x] **Shapes quality attributes** - It materially affects verifiability and portability because: it sets the floor of one archive per supported platform with build provenance and a bill of materials, and the ceiling that a published asset cannot be swapped.
- [x] **Structural** - It defines a boundary, contract, or dependency direction because: it makes one public repository the only publication boundary, fixes the five platform targets and the runners that build them, and adds build tooling outside the workspace crates.
- [x] **Must be shared** - Developers unfamiliar with it would systematically violate the architecture because: they would add a platform promise without a build-and-start evidence path, hand-attach a release asset, or reuse a published version tag.

## Context and Problem Statement

The project's distribution promise was a single self-contained Windows executable attached to a version tag, which is narrower than the people who need the tool and left the release path unexercised with no public home. The operator supplied a public repository address and asked for downloadable, traceable releases for the machines crew members actually use. A public repository is free to operate and is where GitHub artifact attestations and secret scanning live. This decision fixes the release channel and the platform set; the concrete build, packaging and publishing mechanics are owned by `design.md`.

## Decision Drivers

- Crew members use Windows, Linux and macOS; a single Windows executable leaves the others building from source.
- A downloader must be able to trace the file in hand to the tagged revision and this project's automation.
- A public repository is free to operate: unlimited standard runners, artifact attestations on the Free plan, and automatic secret scanning.
- Every platform promise must be backed by a build and a start on a machine of that platform's kind.
- The converter itself must stay local and offline, with no network I/O and no runtime dependency.

## Considered Options

- Windows-only: keep one Windows executable attached to the version tag
- Verifiable immutable releases for five platforms from one public repository
- Mutable releases: publish replaceable assets in place
- A container image or a package-registry publication
- Buy an Apple Developer account now and sign and notarise the macOS archives

## Decision Outcome

Chosen option: "Verifiable immutable releases for five platforms from one public repository", because it is the only option that reaches every crew member's machine while keeping the file a downloader holds traceable to the released revision and impossible to swap after publication.

### Consequences

- Good, because one archive per supported platform is built and started on a machine of that platform's kind, so no platform promise outruns its evidence.
- Good, because every archive carries build provenance and a bill of materials and the immutable release carries a release attestation, so a downloader can verify the file in hand.
- Good, because the public repository brings the gate on landing, secret scanning with push protection, and dependency alerts with it at no cost.
- Bad, because each platform needs its own build and start evidence, so the release path is five builds wide and its correctness is provable only on the host; recorded in chapter 11 (R-009).
- Bad, because a botched release cannot be repaired in place and a used tag is never reusable, so a mistake costs the next version; recorded in chapter 11 (R-012).
- Bad, because the actions in the release path are pinned by mutable major tags, a residual supply-chain risk; recorded in chapter 11 (R-011, TD-005).
- Bad, because the macOS archives are unsigned, so a browser download needs a one-time approval; recorded in chapter 11 (R-010, TD-006).

### Confirmation

The `verify` job in `.github/workflows/release.yml` reads the published release back on every release: it compares each archive's checksum with the staged set, runs `gh attestation verify` on every archive, and asserts the release reports `immutable: true`. The preparation command's completeness and refusal rules are covered by the `publish-public-release` scenarios, and `rhysd/actionlint:1.7.12` checks the workflow's schema, expressions and known action inputs before the push.

## Pros and Cons of the Options

### Windows-only executable attached to the version tag

- Good, because the release path already existed and needs no new platform work.
- Bad, because a Mac or Linux crew member has no way to obtain the tool except by building it.
- Bad, because a single build leaves the release path largely unexercised and unverifiable.

### Verifiable immutable releases for five platforms from one public repository

- Good, because every crew member gets an archive named for their machine.
- Good, because provenance, a bill of materials and an immutable release make the downloaded file checkable and unswappable.
- Bad, because it needs one build per platform kind, host-only correctness evidence, and it accepts an unrepairable release record.

### Mutable releases, assets replaceable in place

- Good, because a mistake in the first public release costs no version number, which matters while the release path is new.
- Bad, because neither a downloader nor the provenance can tell whether the file they hold is the published one; a checksum could be re-uploaded with no trace.

### A container image or a package-registry publication

- Good, because a container or a registry is a familiar distribution channel.
- Bad, because a container turns a local file-in/file-out tool into a mounted-volume, Linux-only runtime that no crew member's problem asks for, and a registry ships no runnable executable.

### Buy an Apple Developer account now and sign and notarise

- Good, because the macOS archives would open without a warning on a fresh Mac.
- Bad, because it adds an annual fee, a signing identity and a notarisation step for a one-time approval the operator accepted when choosing the platform set.

## More Information

The proposal's open questions D1 (public address `kressnerd/duty2cal`), D3 (immutable, draft-first publication) and D4 (unsigned macOS archives); `design.md` ("Technology decisions", "The release pipeline", "The public home's owner-applied settings", risks R-011); chapters 2, 7, 11 and 12; ADR-0001 and ADR-0002 cover the licence and the layer model and are unaffected by this decision.
