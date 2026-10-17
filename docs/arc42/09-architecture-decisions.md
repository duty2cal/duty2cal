# 9. Architecture Decisions

This file is the chapter-09 register/index for architecture decisions. One
architecturally significant decision gets exactly one ADR, stored as a separate
Markdown file under `docs/arc42/adr/ADR-NNNN-short-slug.md`.

## Register

| ADR | Title | Status | Date | File |
|---|---|---|---|---|
| ADR-0001 | Licence the project under MPL-2.0 (per-file weak copyleft) | accepted | 2026-10-12 | [ADR-0001-project-license.md](adr/ADR-0001-project-license.md) |
| ADR-0002 | Enforce the hexagonal layer model with crate boundaries and four policy levels | accepted | 2026-10-12 | [ADR-0002-layer-enforcement.md](adr/ADR-0002-layer-enforcement.md) |
| ADR-0003 | Publish downloadable releases for Windows x86_64 from one public repository | accepted | 2026-10-13 (amended 2026-10-17) | [ADR-0003-release-channel-and-platforms.md](adr/ADR-0003-release-channel-and-platforms.md) |
| ADR-0004 | Publish releases immutably from an inspected draft | accepted | 2026-10-13 | [ADR-0004-immutable-draft-first-publication.md](adr/ADR-0004-immutable-draft-first-publication.md) |
| ADR-0005 | Declare the public home's GitHub configuration with OpenTofu and the GitHub provider | accepted | 2026-10-17 | [ADR-0005-declare-the-public-home-with-opentofu.md](adr/ADR-0005-declare-the-public-home-with-opentofu.md) |
| ADR-0006 | Keep the applied state local, passphrase-encrypted and committed, and check configuration drift locally | accepted | 2026-10-17 | [ADR-0006-encrypted-local-applied-state.md](adr/ADR-0006-encrypted-local-applied-state.md) |

ADR-0001 and ADR-0002 were minted by the change to export released roster
calendars (export-released-roster-calendar); ADR-0003 and ADR-0004 were minted
by the change to publish a public release (publish-public-release). ADR-0003 was
**amended on 2026-10-17** by the windows-only-release change, which narrowed its
platform set to Windows x86_64; it was amended in place, not superseded, and
ADR-0004 is unaffected. ADR-0005 and ADR-0006 were minted by the change to
declare the public home (declare-public-home): the mechanism that declares the
configuration, and where the applied state lives and how it is protected. They
are separate records because they are separate boundaries — the tool that
declares the configuration does not decide where the state lives, and the state
decision does not depend on the tool choice beyond its encryption support.
Every other design choice in those changes is a supplied constraint (chapter 2),
a local mechanism (`design.md`), or product behaviour owned by the capability
specification and the glossary, and did not pass the qualification gate.

## Individual ADR contract

Written and tailored from `docs/arc42/adr/adr-template.md`; MADR sections,
frontmatter, and `ADR-QUALIFICATION` verdict required. A failed candidate is
routed exactly once to its canonical feature, design, Arc42, process,
project-documentation, or code location.

## Archive

Historical ADRs move to `docs/arc42/adr/archive/` preserving ID and filename;
both indexes are updated on any move. `givn/archive/<change>/` is not an ADR
location.
