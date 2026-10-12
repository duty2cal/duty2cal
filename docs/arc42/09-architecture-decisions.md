# 9. Architecture Decisions

This file is the chapter-09 register/index for architecture decisions. One
architecturally significant decision gets exactly one ADR, stored as a separate
Markdown file under `docs/arc42/adr/ADR-NNNN-short-slug.md`.

## Register

| ADR | Title | Status | Date | File |
|---|---|---|---|---|
| ADR-0001 | Split licensing by hexagon layer: MPL-2.0 core, GPL-3.0-or-later CLI | accepted | 2026-10-12 | [ADR-0001-license-split.md](adr/ADR-0001-license-split.md) |
| ADR-0002 | Enforce the hexagonal layer model with crate boundaries and four policy levels | accepted | 2026-10-12 | [ADR-0002-layer-enforcement.md](adr/ADR-0002-layer-enforcement.md) |

Both records were minted by the change to export released roster calendars
(export-released-roster-calendar). Every other design choice in that change is a
supplied constraint (chapter 2) or product behaviour owned by the capability
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
