# Proposal: publish-public-release

## Release note

> Publish duty2cal from one public repository where every change passes the project's checks before it lands there and every version tag offers a downloadable, provenance-verifiable executable for Windows, Linux and macOS.

## Use-Case Context

- Use-case ID: `publish-public-release` — "Publish a public release". New: the permanent corpus holds only the conversion use case (`export-duty-plan-calendar`); no contract exists for publishing yet.
- Ideation topic: none. This change follows the operator's direct instruction to put the project on GitHub; it does not originate from an ideation seed, so there is no seed path and no seed question block to disposition.
- Seed path: none.
- Confirmed Personas:
  - `project-maintainer--release-owner` — "Maintainer responsible for a public release"; confirmed by the operator on 2026-10-10 and promoted to the project collection by the Handoff on 2026-10-12. They decide when a version is released and judge the recorded evidence.
  - `crew-member--personal-calendar-owner` — "Crew member exporting their own duty plan"; confirmed on 2026-10-10, promoted on 2026-10-12. They are the person who downloads an archive and runs it on their own machine, so every platform promise below is judged against their machine.
  - Neither Persona is a Gherkin Actor of this change.
- Parked ideation questions that touch publication (the source-derived publication-rights question Q5, the FZ-provenance question Q13, the descriptive-fragment question Q28): not reopened. This change publishes no source-derived material; see Out of Scope.

The public home is the address the operator supplied, `git@github.com:kressnerd/duty2cal.git` (D1).

## Problem / Opportunity

The converter is releasable but unpublished, and the project's own distribution promise is narrower than the people who need it.

- **No public home.** The working copy has no remote; the README's quick start still reads `git clone <repository-url>`; the coverage badges point at a file inside the working copy rather than at any published state; nothing in the project is reachable by anyone outside this machine.
- **Nothing has ever been published.** The project already builds the Windows executable and attaches it with its SHA256 when a version tag is pushed, but no tag has been pushed, so the whole release path is unexercised and a visitor has nothing to download.
- **One platform.** The distribution promise is a single self-contained Windows executable. Crew members do not all use Windows, so a Mac or Linux user has no way to obtain the tool except by building it.
- **The gates are trusted, not required.** Formatting, lint, tests, dependency licences, per-file SPDX headers, third-party notices drift, the private-data guard, coverage and the Windows cross-build all run on every push and pull request — but nothing prevents a change from landing on the default branch without them, and a reader cannot see whether the revision they are reading passed.
- **Dependency updates arrive nowhere.** The weekly update configuration exists, but with no repository there are no reviewable update proposals and no alerts when a dependency becomes known-vulnerable or malicious; the deliberately pinned extraction and architecture-test crates need the opposite — proposals that stay deliberate.
- **A public home raises the project's existing privacy risk.** Private duty plans must never reach public material. Today that rests on a local guard and review; a public repository adds a credential-and-secret surface that has to be blocked at the point of the push.
- **The project states its licence only in files.** The project page a reader lands on has to state MPL-2.0 and link its text, and the third-party Unicode data file's own licence has to stay attributed rather than be read as a choice between two project licences.

The opportunity is one public home: a gated history, a downloadable release for each platform a crew member actually uses, and a checkable link between the file someone downloads and the project's own build.

## Proposed Solution

### The project's public home

One public repository at the operator's address holds the complete history, so `git clone` from the README works for anyone. Its page states what the tool does, names the licence (MPL-2.0) and links the licence text, carries the topics that let it be found, and shows the state of the checks and of the coverage for the revision being read — not an in-repository file that only a visitor with the working copy can open.

### Every change is gated before it lands

A change reaches the default branch only after the checks that already exist have passed: formatting, lint, tests, dependency licences, per-file SPDX headers, third-party notices drift, the private-data guard, coverage and the Windows cross-build. The gate is enforced by the repository, not by habit: a change that fails them cannot land, and a reader sees the outcome on the revision itself. The maintainer keeps an explicit, recorded route to land an urgent correction.

### Downloadable releases for the machines crew members use

A version tag publishes a release; nothing is uploaded by hand, and the release path exercised here is the one the project already implements, extended to the platforms below.

- **One archive per platform**, named for its platform so a visitor picks the right one without guessing: Windows (x86_64), Linux (x86_64 and arm64), macOS (Apple Silicon and Intel). Each archive holds the executable and a checksum file for it.
- **The executable runs there with no runtime installer** — the same promise the Windows build already makes, extended to the other four, and each platform is built and started on a machine of its own kind before the promise is made.
- **A stable download address.** The latest release stays reachable at a fixed address across versions, so a README link or a colleague's link does not rot as versions change.
- **Each release says what changed**, quoting the release-note subjects the project already stamps on every revision, so a downloader reads what they are getting instead of a bare asset list.
- **Each asset is verifiable and pinned to its build.** Every archive carries build provenance and a software bill of materials listing what went into it, so a downloader can check with a documented command that the file in their hands was built by this project's automation from the tagged revision. The release is published as an immutable record: once it is public its assets cannot be replaced and its tag cannot be moved, so the provenance covers the file a downloader actually holds (D3).
- **No reader is asked to trust a bare asset list.** The release is prepared, verified, and only then published, because the immutable record cannot be edited afterwards.

### A public repository stays safe to work in

- **Secrets and private data are blocked at the push**, not discovered later: credential leaks are refused before they enter the history, and any secret that still lands raises an alert. This is the new surface the existing private-data guard does not cover.
- **Dependencies are watched.** A dependency that becomes known-vulnerable or malicious raises an alert, and updates — including the project's own automation — arrive as reviewable proposals rather than silent changes. The deliberately pinned extraction and architecture-test crates are excluded from automatic minor and major bumps; nothing is merged automatically.
- **Security reports are private until a fix exists.** Someone who finds a defect that could expose private plan data has a confidential route to the maintainer, and the project states that route publicly instead of inviting a public issue that would carry the details.
- **The code and the project's own automation are scanned** on the way in, so findings appear as reviewable alerts rather than as an afterthought.

### Contributing to a public project

A contributor arriving at the repository finds, without asking: how to contribute and that behaviour is specified before it is implemented, the sign-off requirement (DCO, no CLA), a code of conduct, an issue form that asks for the export family and a stripped sample — never a real plan — a pull-request template that asks for the same, and a security policy naming the confidential route (D2). A contribution that does not carry the sign-off, or that fails the checks, does not land.

### Release evidence

None of the promises above is made until it is evidenced, in the same spirit as the previous change's release gates:

- the repository is reachable at the recorded address and the README's clone command works from a clean machine;
- the checks are required and green on the revision the first release is cut from;
- each platform's archive was built **and started on that platform's own machine** in the project's automation, and the Windows archive still imports no non-system library;
- the documented provenance and bill-of-materials verification was run against a real published release and succeeded;
- secret scanning, push protection, the dependency alerts and the required checks are actually on — this is the part only the repository owner can switch on, so the change records the exact list of settings and states that the operator applies it and confirms the outcome;
- the project page states MPL-2.0 and the third-party Unicode data file's licence stays attributed;
- the published history contains no private source material, checked against the existing guard and review.

Two kinds of check stand behind these promises, and the difference matters for what follows. The promises that shape a shippable artefact — one archive per platform, each holding the executable and a checksum for it, and release text that quotes the stamped release-note subjects — are produced and checked by the project's own automation, with no network and no account. The promises that exist only on a public host — the gate on landing, the secret and dependency alerts, the published release, its provenance and its immutability — cannot be reproduced in the project's own test run; they are exercised by the automation on the host and recorded as this change's evidence, with the owner-applied settings confirmed by the operator.

The first release is cut at the version the workspace already carries, `v0.1.0`, so the latest-release address works from the first day. That release publishes the tool as it stands; it does not widen the converter's documented support boundaries.

## Capability Routing

`givn spec route --change publish-public-release` reported `EXTEND export-duty-plan-calendar` (score 13.00, confidence: signal), from word overlap alone — its own tokens plus three generic goal/interaction words shared with the conversion use case. No conversion behaviour changes here, so that recommendation is not followed.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `publish-public-release` — Publish a public release: the project's public home, the gate every change passes before it lands there, and each version tag's downloadable, verifiable executables | `EXTEND export-duty-plan-calendar` | `NEW in publish-public-release` | The signal is lexical: three generic goal/interaction words matched, and the existing capability is about converting a duty plan into a calendar, none of which changes. This change's goal belongs to the release owner — it is their Persona goal of publishing a release with evidence — who is not an Actor of the conversion use case. The public home, the gate on landing, the per-platform archives, the provenance and the dependency-and-secret safety are all contributions to that one outcome, not shipping capabilities of the converter. |

## Out of Scope

- **Changing the converter.** No scenario of `export-duty-plan-calendar` changes; conversion behaviour, its support boundaries, its diagnostics and its naming rules stay as they are. This change makes the tool obtainable and verifiable.
- **Packages.** GitHub Packages has no Cargo registry, so the tool cannot be published there; and two package-shaped alternatives are excluded deliberately: a container image (it turns a local file-in, file-out tool into a mounted-volume, Linux-only runtime, which no crew member's problem asks for) and a publication to crates.io (not a GitHub feature, ships no executable, and its publication is effectively irreversible). Releases with downloads are the channel the users of this tool need.
- **Signing and notarising the macOS archives.** An unsigned archive is quarantined by a browser download and needs a one-time approval from the crew member, which the operator accepted when choosing the platform set; removing that step needs a paid Apple Developer account and is a separate decision.
- **Further platforms.** Windows on ARM, 32-bit Windows, BSDs, and arm64 Linux variants beyond the five archives above are not supported by download; their users build from source.
- **Hosted documentation, discussions, a wiki, a development container, funding metadata and AI code review.** Excluded with their reasons: the project's documentation lives in the repository and the project page; there is no community to host; the workspace needs no container definition; no funding is sought; the AI reviewer needs a separate subscription.
- **Repairing a published release in place.** A published release is immutable (D3), so a bad asset or a mistyped tag is corrected by the next version. That constraint is accepted deliberately, and it is the price of a verifiable release.
- **Automatic dependency merging, and the two deliberately pinned crates' minor and major bumps.** Updates stay reviewable proposals.
- **The parked ideation questions.** Source-derived publication rights (Q5), formal FZ provenance (Q13) and descriptive-fragment conflicts (Q28) stay parked: this change adds no source-derived artefact, makes no formal acronym claim, and changes no conversion rule. Nothing here is a precedent for them.

## Open Questions

### D1 — The repository's public address and the working copy's directory name

- Question: which public address does the project take, given that the operator's instruction names `kressnerd/duty2cal` while the working copy sits in a directory named `duty2ical` and the product, its crate and its documentation all call themselves `duty2cal`?
- Options:
  1. **Take the operator's address, `kressnerd/duty2cal`** — the repository is named after the product; gain: the clone address, the release links and every document already use that name; cost: the local directory keeps its different name until someone renames it; in practice: a reader clones `https://github.com/kressnerd/duty2cal`, and nothing in the repository mentions `duty2ical`.
  2. **Name it `kressnerd/duty2ical` after the directory** — gain: the local path and the remote agree; cost: the product, the crate, the binary and the documentation would carry one name while the public address carries another, and changing it later breaks clones and every published release link; in practice: the README would advertise `duty2ical` while its usage output prints `duty2cal`.
- Disposition: **answered** — option 1: the operator supplied `git@github.com:kressnerd/duty2cal.git` as the repository to add on 2026-10-13, and the product, crate and documentation agree with it. Rejected: option 2, because renaming the public address after the first release breaks the links this change's stable download address exists to preserve.

### D2 — The contact route published in the security policy and the code of conduct

- Question: which address or route do the public security policy and code of conduct name for reports that must not be public?
- Options:
  1. **The repository's confidential reporting route, plus a conduct address the operator names** — gain: security reports arrive as private advisories attached to the repository, and conduct reports have a stated destination; cost: a public address has to be chosen and will be harvested; in practice: a plan-data exposure report arrives as a draft advisory, and a conduct report arrives by mail.
  2. **Confidential reporting only, with no mail address anywhere** — gain: no address to harvest and one route to watch; cost: the code of conduct cannot name a reporting method, which is precisely what it is for; in practice: a contributor with a conduct concern finds no route and opens a public issue instead.
- Disposition: **answered** — the operator supplied the conduct route on 2026-10-13, answering the design review's third question: a direct message to the maintainer's Mastodon account `@kressnerd@chaos.social` (checked to exist at `https://chaos.social/@kressnerd`), alongside the repository's private vulnerability reporting for plan-exposure reports. Rejected: publishing no route at all, which is the opposite of what a code of conduct is for. A mailbox was not adopted because the maintainer publishes none, and a GitHub `noreply` address cannot receive mail.

### D3 — Whether a published release can be changed after publication

- Question: should the project publish immutable releases, so a download can never be swapped after publication?
- Options:
  1. **Immutable releases, prepared as a draft and published only after verification** — gain: an asset or a tag cannot be replaced once public, so the provenance statement covers the file a downloader actually holds, and the repository issues a release attestation; cost: a botched release cannot be repaired in place — a wrong checksum or a broken archive is fixed by the next version — and a tag name used by an immutable release is never reusable, even after deleting the repository; in practice: a macOS archive that crashes in v0.1.0 is fixed in v0.1.1 and v0.1.0 stays on the page.
  2. **Mutable releases, assets replaceable in place** — gain: a mistake in the first public release costs no version number, which matters most while the release path itself is new; cost: neither a downloader nor the provenance can tell whether the file they hold is the published one; in practice: a checksum file is re-uploaded minutes later with no trace — and equally so by anyone who takes over a maintainer account.
- Disposition: **answered** — the operator selected immutable releases with the draft-then-publish order on 2026-10-13 in this proposal step. Rejected: option 2, because verifiability is the point of attaching provenance and a bill of materials to each asset.

### D4 — macOS signing and notarisation

- Question: should the published macOS archives be signed and notarised so no crew member has to approve the download?
- Options:
  1. **Publish them unsigned, with the approval step documented** — gain: no recurring cost and no signing identity to maintain; cost: a Mac download from a browser is quarantined and needs a one-time approval, and a reader may mistake the warning for a defect; in practice: the README tells the Mac user what the dialog means before they meet it.
  2. **Buy an Apple Developer account and notarise** — gain: the archives open without a warning, which matters for a non-technical crew member; cost: an annual fee, a signing identity and a notarisation step in the release path; in practice: the same archive opens on a fresh Mac with no dialog.
- Disposition: **out of scope for this change** — the operator accepted option 1's cost when choosing the platform set on 2026-10-13; option 2 needs a paid Apple Developer account, which is a separate decision with its own cost. The README documents the approval step instead of leaving the warning unexplained.
