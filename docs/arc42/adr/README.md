# Architecture Decision Records

Index of the project's Architecture Decision Records. Each record is one MADR
file at `docs/arc42/adr/ADR-NNNN-short-slug.md`; the chapter-09 register
(`docs/arc42/09-architecture-decisions.md`) links to the same files.

## Register

| ADR | Title | Status | Date | File |
|---|---|---|---|---|
| ADR-0001 | Licence the project under MPL-2.0 (per-file weak copyleft) | accepted | 2026-10-12 | [ADR-0001-project-license.md](ADR-0001-project-license.md) |
| ADR-0002 | Enforce the hexagonal layer model with crate boundaries and four policy levels | accepted | 2026-10-12 | [ADR-0002-layer-enforcement.md](ADR-0002-layer-enforcement.md) |

## Notes

- The change to export released roster calendars (export-released-roster-calendar)
  minted both records; every other choice is a supplied constraint (chapter 2),
  local mechanism (design.md), or product behaviour owned by the capability
  specification and the glossary.
- Archived historical ADRs live under `docs/arc42/adr/archive/` with their IDs
  preserved; both indexes are updated on any move.
