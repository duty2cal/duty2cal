# 4. Solution Strategy

## Key decisions

- **A pure domain behind ports.** Classification, timing, description composition, trip carry, and calendar composition are pure and dependency-free; the PDF extractor, the RFC 5545 writer, and the filesystem are adapters behind ports. This keeps the domain testable without a real PDF and independent of the extraction crate.
- **Staged pipeline with a completeness gate.** Source recognition → activity classification → endpoint date resolution → description composition → calendar composition → publication. The `DutyPlanCalendar` aggregate reaches "complete" only when every required activity and field resolves; publication happens after completeness, so a partial calendar can never be published.
- **Determinism scoped to the source snapshot.** Component identity is derived from the source snapshot plus the represented activity, not from an output counter or a title, so identical input is reproducible and a revised input gets new identities.
- **Create-new publication.** New calendars and diagnostics are written only to an unused name (create-new semantics with retry), so an existing or concurrently claimed file is never replaced.
- **Explicit failure as a first-class outcome.** A conversion failure returns a distinct outcome, attempts a numbered diagnostic, and reports the primary error even when the diagnostic cannot be saved.

## Technology choices

| Concern | Choice | Rationale (see ADR) |
|---|---|---|
| Language / runtime | Rust (edition 2021), latest stable toolchain meeting the dependency minimum | Supplied constraint; chapter 2. |
| PDF extraction | Rust `pdfplumber` crate, exact pin `=0.1.0` | Supplied exact pin; chapter 2. |
| Calendar output | Hand-written RFC 5545 writer | Supplied constraint; chapter 2. |
| Executable specification | Gherkin `.feature` files run by cucumber-rs (strict) | The spec and its tests are one artifact; chapter 2. |
| Distribution | One self-contained Windows GNU executable, cross-compiled | Supplied constraint; chapter 7. |

No new ADR is minted for these choices: each is either a supplied constraint (owned by chapter 2) or a product-behaviour decision owned by the capability specification and the glossary. See the qualification assessment in the change's `arc42.md`.

## Approach to quality goals

| Quality attribute | Approach |
|---|---|
| Correctness / completeness | Whole-plan completeness gate before publication; required-value contradictions and unresolved activities reject the conversion rather than being omitted. |
| Reliability (no data loss) | Create-new publication with enumerated retry; diagnostics preserved and numbered; existing contents untouched. |
| Interoperability | Hand-written writer with CRLF, escaping, and octet-aware folding, asserted by a scenario on the published file. |
| Privacy | Generated/anonymized fixtures only; no private sources or identifying values in the repository. |
| Maintainability / portability | Clean Architecture layering; one self-contained executable with no native dependency. |
