# Architecture Decision Records

Index of the project's Architecture Decision Records. Each record is one MADR
file at `docs/arc42/adr/ADR-NNNN-short-slug.md`; the chapter-09 register
(`docs/arc42/09-architecture-decisions.md`) links to the same files.

## Register

| ADR | Title | Status | Date | File |
|---|---|---|---|---|
| ADR-0001 | Licence the project under MPL-2.0 (per-file weak copyleft) | accepted | 2026-10-12 | [ADR-0001-project-license.md](ADR-0001-project-license.md) |
| ADR-0002 | Enforce the hexagonal layer model with crate boundaries and four policy levels | accepted | 2026-10-12 | [ADR-0002-layer-enforcement.md](ADR-0002-layer-enforcement.md) |
| ADR-0003 | Publish downloadable releases for Windows x86_64 from one public repository | accepted | 2026-10-13 (amended 2026-10-17) | [ADR-0003-release-channel-and-platforms.md](ADR-0003-release-channel-and-platforms.md) |
| ADR-0004 | Publish releases immutably from an inspected draft | accepted | 2026-10-13 | [ADR-0004-immutable-draft-first-publication.md](ADR-0004-immutable-draft-first-publication.md) |
| ADR-0005 | Declare the public home's GitHub configuration with OpenTofu and the GitHub provider | accepted | 2026-10-17 | [ADR-0005-declare-the-public-home-with-opentofu.md](ADR-0005-declare-the-public-home-with-opentofu.md) |
| ADR-0006 | Keep the applied state local, passphrase-encrypted and committed, and check configuration drift locally | accepted | 2026-10-17 | [ADR-0006-encrypted-local-applied-state.md](ADR-0006-encrypted-local-applied-state.md) |
| ADR-0007 | Run the conversion in the browser through a WASM facade over the unchanged application | accepted | 2026-10-19 | [ADR-0007-browser-local-conversion-facade.md](ADR-0007-browser-local-conversion-facade.md) |

## Notes

- The change to export released roster calendars (export-released-roster-calendar)
  minted ADR-0001 and ADR-0002; the change to publish a public release
  (publish-public-release) minted ADR-0003 (the release channel and platform set)
  and ADR-0004 (immutable draft-first publication). Every other choice is a
  supplied constraint (chapter 2), local mechanism (design.md), or product
  behaviour owned by the capability specification and the glossary.
- ADR-0003 was **amended on 2026-10-17** by the windows-only-release change,
  narrowing its supported platform set to Windows x86_64. It was amended in
  place, not replaced; ADR-0004 is unaffected.
- The change to declare the public home (declare-public-home) minted ADR-0005
  (the declaration mechanism) and ADR-0006 (the encrypted local applied state
  and the local drift check). Each records one boundary; neither supersedes an
  earlier record.
- The change to build the browser converter (build-browser-converter) minted
  ADR-0007 (the browser-local conversion facade). It applies ADR-0002's layer
  model rather than amending it, and it is not ADR-0003 because the browser is a
  conversion route, not a downloadable archive platform. The browser-owned save
  and the browser test driver were routed to the capability specification and
  `design.md` respectively, not to an ADR.
- Archived historical ADRs live under `docs/arc42/adr/archive/` with their IDs
  preserved; both indexes are updated on any move.
