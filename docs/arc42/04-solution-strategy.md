# 4. Solution Strategy

## Key decisions

- **A pure core behind ports.** `duty2cal-core` holds the domain, the application ports and the dependency-free ICS adapter; `duty2cal-pdf` is the only crate that touches the extraction dependency; `apps/cli` wires them. The compiler, `cargo deny`+clippy, archunit tests and review conventions enforce the inward-only rule (ADR-0002).
- **Staged pipeline with a completeness gate.** Source recognition → classification → endpoint date resolution → description composition → calendar composition → publication. The `Roster` aggregate reaches "complete" only when every required activity and field resolves; publication happens after completeness, so a partial calendar can never be published.
- **Determinism scoped to the source snapshot.** Component identity derives from the source snapshot plus the represented activity, not from an output counter or a title (opaque discriminator; the readable date/flight/route recipe was rejected as non-unique).
- **Create-new publication.** New calendars and diagnostics are written only to an unused name (create-new semantics with retry); existing or concurrently claimed files are never replaced.
- **Explicit failure as a first-class outcome**, with an in-process, synchronous `DomainEvent` stream for host surfaces.

## Technology choices

| Concern | Choice | Rationale (see ADR) |
|---|---|---|
| Language / runtime | Rust edition 2021, `stable` toolchain | Supplied constraint; chapter 2. |
| PDF extraction | `pdfplumber = "=0.4.1"` in `duty2cal-pdf` | Supplied pin (brief); chapter 2. |
| Calendar output | Hand-written RFC 5545 in `duty2cal-core/src/adapters/ics` | Supplied constraint. |
| Module boundaries | Two-crate workspace + four enforcement levels | ADR-0002. |
| Licensing | MPL-2.0 core, GPL-3.0-or-later CLI | ADR-0001. |
| Executable specification | Gherkin `.feature` files run by cucumber-rs (strict) | Chapter 2. |
| Distribution | One self-contained `x86_64-pc-windows-gnu` executable, cross-compiled | Chapter 7. |

ADR-0001 and ADR-0002 are the change's only ADRs; the remaining choices are supplied constraints (chapter 2) or product behaviour owned by the capability specification and the glossary.

## Approach to quality goals

| Quality attribute | Approach |
|---|---|
| Correctness / completeness | Whole-plan completeness gate before publication; contradictions and unresolved activities reject the conversion. |
| Reliability (no data loss) | Create-new publication with enumerated retry; numbered preserved diagnostics; existing contents untouched. |
| Interoperability | Hand-written writer with CRLF, escaping and octet-aware folding, asserted on the published file. |
| Privacy | Generated/anonymised fixtures only; no private sources in the repository. |
| Maintainability / portability | Hexagonal layout with mechanical layer enforcement; one self-contained executable; a core free of I/O, clock and environment. |
