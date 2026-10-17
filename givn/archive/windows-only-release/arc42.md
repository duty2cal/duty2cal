# Arc42 documentation assessment: windows-only-release

The change narrows the published platform set from five platforms to Windows
x86_64 only. It changes deployment, distribution, context, runtime and quality
documentation, and amends ADR-0003 in place. Every chapter row was assessed
against `proposal.md`, the delta `publish-public-release` specification and
`design.md`.

| # | Chapter | Affected? | Reason | What changed |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Requirement 5, quality goals 5–6 and the crew-member/maintainer stakeholder rows promise the platform and archive set. | Requirement 5, quality goals 5 and 6, and both stakeholder rows now state one Windows x86_64 archive and the source-build route for other platforms. |
| 2 | 02 architecture-constraints | Yes | The target/platform constraint lists five targets and the musl/macOS build rule; the Linux host role must be recorded as engineering context. | Replaced the five-target and musl/macOS rows with a Windows-x86_64-only constraint and a Linux CI/test/cross-build-host constraint; the Windows cross-build, smoke and static-linking constraints remain. |
| 3 | 03 context-and-scope | Yes | The GitHub release-channel edge and the crew member's download route describe a per-platform archive set. | The channel now serves one Windows x86_64 archive with checksum, provenance and bill of materials; the text states there is no per-platform set and other platforms build from source. |
| 4 | 04 solution-strategy | Yes | The release strategy and the distribution technology row name five builds and five executables. | Release strategy and distribution row now describe one Windows cross-build plus Windows smoke, one deterministic archive, and the source-build route; quality-approach rows follow. |
| 5 | 05 building-block-view | Yes | The release toolbox describes one archive per supported platform, a five-platform matrix, and an e2e step that runs the archived executable. | `prepare-release.sh`, `release.yml` and the step-definition responsibilities now describe one Windows archive, one cross-build, the Windows smoke job, and local archive-inspection rather than local execution. |
| 6 | 06 runtime-view | Yes | The release flow shows a five-platform build matrix and preparation over five binaries. | The flow shows one Windows x86_64 cross-build, the `windows-latest` smoke start, preparation over the one binary, and attestation/verification of the one archive. |
| 7 | 07 deployment-view | Yes | Infrastructure diagram, runner table, artefact table, crew-member OS row and the download section describe five platforms and macOS signing. | Reduced to one Windows artefact and the `ubuntu-latest` build/prepare/publish/verify pair plus `windows-latest` smoke; crew member is Windows x86_64; download/verify text is Windows-only; macOS signing text removed. |
| 8 | 08 crosscutting-concepts | Yes | Package determinism is stated as "the same five executables"; the preparation refusal names a supported-platform set. | Release determinism now covers the one Windows executable with the same fixed `tar` inputs; the completeness refusal names Windows x86_64 and rejects previously offered platforms. |
| 9 | 09 architecture-decisions | Yes | ADR-0003's platform-set decision is amended and both ADR indexes must reflect the amendment. | Register row carries the amended title and 2026-10-17 amendment note; `adr/README.md` matches; ADR-0003 is amended in place with the AMEND_ADR verdict, revised content and consequences. |
| 10 | 10 quality-requirements | Yes | The quality tree and QS-007/QS-008 name the archive set and five executables. | The tree node is "One Windows x86_64 archive per version"; QS-007 names `windows-x86_64`; QS-008 prepares the one Windows executable twice. |
| 11 | 11 risks-and-technical-debt | Yes | R-010/TD-006 (unsigned macOS archives) no longer apply, R-009/R-013 describe five-build evidence, and the unverified source-build route needs a risk row. | Closed R-010 and TD-006; rewrote R-009 for the one-build-plus-Windows-smoke path and R-013 for the narrowed evidence; added R-019 for unverified source builds on other platforms. |
| 12 | 12 glossary | Yes | "Supported platform" and "Release archive" define the five-platform set. | Supported platform is Windows x86_64 only; Release archive is exactly one per version; unrelated terms are untouched. |

Overall: the change is a narrowed deployment boundary, so it touches all twelve
chapters and amends ADR-0003 rather than minting a new ADR; ADR-0004 is
unaffected.

## ADR qualification verdict

qualification: QUALIFIED
alternatives: PASS — Windows x86_64 only versus the five-platform set versus source-build only (proposal D3).
architectural_impact: PASS — it fixes which machines the project promises to serve and the single build-and-start evidence path every release needs.
durable_consequence: PASS — every future version, the public text and the release evidence are bound by the narrowed set.
lower_level_artifact: PASS — the preparation, matrix and runner mechanics live in `design.md`, but the platform-set promise itself is the durable boundary owned here.
existing_adr_check: PASS — the active register and the absent `docs/arc42/adr/archive/` were searched; ADR-0003 owns the release channel and platform set, ADR-0004 owns immutability, and no competing record exists.
must_be_shared: SUPPORTING — a contributor who adds a platform promise without build-and-start evidence would violate the architecture.
routing: AMEND_ADR
canonical_artifact: null
target_adr: docs/arc42/adr/ADR-0003-release-channel-and-platforms.md
replacement_adr: null

## Edited chapter files

- `docs/arc42/01-introduction-and-goals.md`
- `docs/arc42/02-architecture-constraints.md`
- `docs/arc42/03-context-and-scope.md`
- `docs/arc42/04-solution-strategy.md`
- `docs/arc42/05-building-block-view.md`
- `docs/arc42/06-runtime-view.md`
- `docs/arc42/07-deployment-view.md`
- `docs/arc42/08-crosscutting-concepts.md`
- `docs/arc42/09-architecture-decisions.md`
- `docs/arc42/10-quality-requirements.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md`
- `docs/arc42/adr/ADR-0003-release-channel-and-platforms.md`
- `docs/arc42/adr/README.md`

## Status

STATUS: DONE
