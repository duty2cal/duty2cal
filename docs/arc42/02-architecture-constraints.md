# 2. Architecture Constraints

## Technical constraints

| Constraint | Motivation |
|---|---|
| Rust, edition 2021, `stable` toolchain (`rust-toolchain.toml`) | Supplied constraint. |
| Cargo workspace with a crate boundary only where dependencies, target platform, or `crate-type` differ | ADR-0002 (reduced two-crate core + PDF split). |
| `pdfplumber = "=0.4.1"` (Apache-2.0), depended on only by `duty2cal-pdf` | Supplied exact pin, updated by the operator's brief; re-proved to build. |
| Hand-written RFC 5545 serializer with no calendar library dependency, in either the product or the tests; the golden scenario compares the published file | Output must be deterministic for the golden scenario, and the serializer carries no third-party licence obligation. |
| No native C/C++ dependency; core is `wasm32`-safe and does no I/O, clock or environment access | `crates/duty2cal-core/clippy.toml` `disallowed-types`/`disallowed-methods`, enforced by `cargo clippy --workspace -- -D warnings`. |
| Windows GNU target (`x86_64-pc-windows-gnu`), cross-compiled on Linux/CI, statically linked | Supplied distribution constraint (kept). |
| One self-contained executable, no companion DLL or runtime installer | Supplied distribution constraint. |
| `archunit = "=0.0.2"` architecture tests for the intra-crate layer direction | ADR-0002, level 3. |
| Typed errors (`thiserror`); no `unwrap()`/`expect()` outside test code | Operator brief conventions. |
| No network I/O at runtime | The converter is local and offline. |

## Organisational constraints

| Constraint | Motivation |
|---|---|
| Licence: MPL-2.0 project-wide (per-file weak copyleft, App-Store compatible) | ADR-0001. |
| `deny.toml` allowlist; any GPL/LGPL/AGPL/SSPL third-party crate is a hard failure | Keeps the project embeddable in any host app. |
| Per-file SPDX header; `REUSE.toml` for binary/data files; `cargo-about` notices; DCO sign-off, no CLA | ADR-0001 obligations. |
| Private duty-plan PDFs are read for analysis but never committed; fixtures and golden calendars are generated/anonymised | Privacy. |
| Changes follow the givn workflow; `.feature` files are the executable specification | Specification and tests are one artifact. |

## Conventions

| Convention | Motivation |
|---|---|
| Dependencies point inward; each adapter implements exactly one port with no business logic | ADR-0002. |
| Value objects are built only through validating `Result` constructors with no public fields; ports are traits in `application` and know no adapter type; every port is exercised through the real adapters rather than a test double | ADR-0002, level 4. |
| No path or file handle crosses the use-case boundary (input `&[u8]`, output `String`) | Keeps one use case callable from CLI, WASM and UniFFI. |
| Strict Gherkin execution (`.fail_on_skipped()`); one step-definition file per capability | Prevents false completion. |
| Diagrams in arc42/design are Mermaid | Rendered consistently. |
