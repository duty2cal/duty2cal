# Proposal: windows-only-release

## Release note

> Offer one downloadable archive for Windows, the only platform the project distributes.

## Use-Case Context

- Use-case ID: `publish-public-release` — "Publish a public release". This change narrows the platform set that capability owns; it does not open a second release goal.
- Ideation topic: none. The change follows the operator's direct instruction of 2026-10-17; it has no ideation seed.
- Seed path: none.
- Confirmed Personas:
  - `project-maintainer--release-owner` — the person who prepares and publishes a release and answers for the platform promise it makes; confirmed on 2026-10-10 and promoted on 2026-10-12. They gain a release path with one build, one start evidence job and one archive to inspect.
  - `crew-member--personal-calendar-owner` — the person who downloads and runs the tool on their own machine; same confirmation and promotion. The Windows crew member keeps a downloadable, verifiable archive; a crew member on any other platform builds from source.
  - Neither Persona is a Gherkin Actor of this change, and no new Persona is introduced.

## Problem / Opportunity

The project currently promises five downloadable platforms — Windows x86_64, Linux x86_64, Linux arm64, macOS arm64 and macOS x86_64 — in its release workflow, its preparation command, its README, its issue form, its security policy and its architecture documentation. In practice the project has only Windows users. The Linux and macOS archives are never downloaded, yet every release builds them, starts them on their own runners, attests them, attaches them and keeps them on the public release page.

The cost is visible in four places:

- **Release page and storage.** Every version carries five archives and five checksums that no user asked for, filling the release assets and release storage with files nobody downloads.
- **Build and evidence burden.** Each additional platform needs its own runner, its own build, its own start evidence and its own provenance, so the release path is wider and slower than the audience requires.
- **Support promises.** The public text, the issue form and the documentation invite defects for platforms the project does not actually serve, and answering them competes with the Windows users the project does have.
- **Misleading evidence.** A five-platform matrix makes the release evidence look broader than the user base, while the only platform anyone uses is the one the project already builds and smokes on Windows.

The opportunity is a release path that matches reality: one Windows x86_64 archive per version, with the same checksum, provenance, bill of materials and immutability promises, and a public statement that anyone on another platform can build from source. A platform can be added back by a later change when users ask for it.

## Proposed Solution

### One downloadable platform

A future release offers exactly one platform archive: `duty2cal-<version>-windows-x86_64.tar.gz`, holding `duty2cal.exe` and nothing else, with its `.sha256` checksum file beside it. The release still carries its release text and its change log, and it keeps the build provenance, bill-of-materials and immutable-release promises already made.

The preparation refuses a set that does not contain exactly the Windows executable. A missing Windows build fails the preparation and stages nothing. An executable for any other platform — including Linux x86_64, Linux arm64, macOS arm64 or macOS x86_64, which earlier releases offered — is outside the supported set and is refused, with nothing staged. No archive is ever published for a platform the release does not support.

### Linux is a build and test host, not a published platform

The project keeps building and testing on Linux, and the Windows executable stays cross-built on Linux and started on a Windows machine before its archive is published. That work is the project's own development and evidence path; it does not make Linux a download platform.

### Other platforms build from source

A crew member on Linux, macOS or any other platform obtains the tool by building it from the repository. The README and the project's public text say so, and they state that a platform may be added back to the release set when users request it. The issue form and the security policy stop offering Linux and macOS as download choices; "built from source" becomes the route for everyone else.

### Existing releases stay as they are

Already-published releases are immutable. Their Linux and macOS archives are not deleted, replaced or re-published; this change narrows future releases only.

### The documentation and automation stop promising five platforms

The release workflow loses its Linux and macOS matrix entries and the start step that only applied to non-Windows builds; the Windows smoke job remains. The preparation command and the public-text check carry the Windows-only set. The README, the issue form, the security policy, the architecture documentation and ADR-0003 are amended so no public text promises a Linux or macOS download.

## Capability Routing

`givn spec route --change windows-only-release` reported `EXTEND export-duty-plan-calendar` (score 13.00, confidence: signal), followed by `publish-public-release` (score 12.00). The recommendation is lexical: neither candidate shares real behaviour with this change, and three generic goal/interaction words matched the conversion use case.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `publish-public-release` | `EXTEND export-duty-plan-calendar` | `EXTEND publish-public-release` | No conversion rule, support boundary or diagnostic changes. What changes is which archive a release offers, which is the release owner's goal and the capability that already owns the platform set, the release preparation and the public text. Extending the conversion use case would attach a distribution decision to a use case that never made one. |

## Out of Scope

- **Changing the converter.** No conversion rule, support boundary, diagnostic, fixture or golden calendar changes.
- **Publishing packages.** No container image, registry publication or other channel beyond the release archive.
- **Signing or notarising macOS archives.** No macOS archive is published after this change, so the question no longer arises.
- **Publishing Linux or macOS archives.** Those users build from source. A platform is added back only when users request it and a later change records the need.
- **Removing already-published archives.** Published releases are immutable; the existing Linux and macOS archives stay reachable on their release pages.
- **Changing release semantics.** Draft-first publication, immutability, provenance, bill of materials, the publication gate, the change log and the release-note rules are unchanged.
- **Removing Linux from CI or local development.** Linux remains the build and test host.
- **Hosted documentation, discussions, funding metadata and automatic dependency merging.** Unchanged exclusions.

## Open Questions

### D1 — How the local end-to-end preparation proves the Windows archive when the test host cannot execute a Windows binary

- Question: the release-preparation `@e2e` scenario runs on Linux, but the only distributed executable is a Windows `.exe`; how does the local scenario prove the staged Windows archive without pretending to execute it?
- Options:
  1. **Drive the real preparation with a stand-in `duty2cal.exe` and assert the staged archive, its checksum and its extracted contents; keep the real execution evidence in the Windows smoke job.** Gain: the local scenario still exercises the real preparation command, archive format and extraction route, and the execution claim stays with the job that runs on Windows. Cost: the local transcript is not execution evidence for the Windows binary. In practice: the `@e2e` scenario reports the Windows archive and checksum, extracts it, asserts the only member is `duty2cal.exe` with the staged bytes, and the `smoke-windows` job starts the real cross-built executable on `windows-latest`.
  2. **Run the Windows executable under Wine on Linux.** Gain: the scenario literally executes the archive. Cost: a new host dependency and no evidence about the real Windows runtime, so it is no better than the smoke job while making the suite heavier. In practice: a Linux runner installs Wine and executes a binary whose actual platform is never exercised.
  3. **Drop the `@e2e` tag and keep only the preparation scenarios.** Gain: no platform mismatch. Cost: the interaction inventory loses its end-to-end evidence and the archive/checksum/extraction path is no longer driven through the real command. In practice: a broken archive layout would be caught only after publication.
- Disposition: **answered** — option 1, by the operator's instruction of 2026-10-17: the local `@e2e` must still drive the real preparation command and assert the staged Windows archive, checksum and extracted contents, while the real execution evidence remains the `windows-latest` smoke job in CI and the release workflow. Rejected: options 2 and 3 for the reasons above.

### D2 — Whether already-published Linux and macOS artifacts are removed or re-published

- Question: should this change clean up the Linux and macOS archives already attached to published releases?
- Options:
  1. **Leave every published release untouched.** Gain: the immutability promise and every existing download link stay true. Cost: the old archives remain visible on historical release pages. In practice: the `0.1.0` page keeps its five archives and the next version offers one.
  2. **Delete the Linux and macOS assets from published releases.** Gain: the release pages show only Windows. Cost: a published release becomes mutable, directly contradicting the release attestation and the immutability setting. In practice: a checksum or provenance statement no longer covers the asset set a downloader sees.
- Disposition: **answered** — option 1, by the operator's instruction of 2026-10-17: already-published releases are immutable and this changes future releases only. Rejected: option 2 because it breaks the promise the release channel is built on.

### D3 — Whether Linux remains a build/test host and how a platform returns

- Question: does dropping Linux from the download set also remove it from the project's build and test path, and how would a Linux or macOS archive ever come back?
- Options:
  1. **Keep Linux as the build and test host; add a platform back only through a later change when users request it.** Gain: the project keeps testing and cross-building on the cheapest runners, and the public platform set changes only with recorded evidence. Cost: the platform set needs a future change and its own build/start evidence to widen again. In practice: CI still runs on `ubuntu-latest`, the Windows executable is cross-built there, and a Linux download returns only after a user request and a recorded decision.
  2. **Remove Linux from CI and development as well.** Gain: one less environment to maintain. Cost: the Windows cross-build and the test host disappear, and the project loses the environment its suite runs on. In practice: the release build would need a different host and the test suite would have nowhere to run.
  3. **Keep the five-platform matrix in the workflow but attach only Windows.** Gain: future platforms are pre-built. Cost: the unused builds and runners remain the cost this change exists to remove. In practice: four runners keep producing archives no release attaches.
- Disposition: **answered** — option 1, by the operator's instruction of 2026-10-17: Linux remains the CI and local test host, and a platform may be added back when users request it. Rejected: option 2, which removes the environment the tests and cross-build depend on, and the workflow-only option, which keeps the cost without the download.
