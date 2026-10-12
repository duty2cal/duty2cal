# 2. Architecture Constraints

## Technical constraints

| Constraint | Motivation |
|---|---|
| Rust, application edition 2021 | Supplied non-negotiable constraint inherited from the first draft and the approved contract. |
| Latest stable Rust toolchain satisfying the extraction dependency's minimum (the pinned crate declares Rust 1.85) | The dependency cannot be built on an older toolchain; the exact version resolves at setup. |
| Windows GNU target, cross-compiled on Linux/CI | Supplied distribution constraint. |
| One executable, no companion DLL or runtime installer | Supplied single-file distribution constraint. |
| PDF extraction from the Rust `pdfplumber` crate, exact pin `=0.1.0` | Supplied exact pin; no substitution without a user decision. |
| No native C/C++ PDF extraction dependency | Supplied; keeps the build and the executable self-contained. |
| Hand-written RFC 5545 output; no iCalendar writer library | Supplied; makes the byte-level contract directly provable. |
| Dependencies restricted to MIT or Apache-2.0; public material releasable on that basis | Supplied licensing constraint for a public repository. |
| No network I/O at runtime | The converter is local and offline; nothing may call out. |

Clean Architecture is a supplied technical constraint, not a proposed actor goal or a use-case boundary.

## Organisational constraints

| Constraint | Motivation |
|---|---|
| Private duty-plan PDFs are read for analysis but never committed | Privacy: plans contain identifying personal data. |
| Fixtures and golden calendars are generated or anonymized | Public material must carry no real itineraries, names, or identifiers, and privacy checks consider identifying combinations, not just names. |
| Changes follow the givn workflow; `.feature` files are the executable specification | The specification and its tests are the same artifact. |

## Conventions

| Convention | Motivation |
|---|---|
| Clean Architecture dependency rule: dependencies point inward at a pure domain; PDF/filesystem reached through ports | Keeps domain logic testable without the outside world and independent of the extraction dependency. |
| One Gherkin step-definition file per capability | Keeps the executable-spec bindings scoped and replaceable. |
| Strict Gherkin execution (`.fail_on_skipped()`) | Prevents a change from being marked complete while steps are unimplemented. |
| Diagrams in arc42 and design documents are Mermaid | Rendered consistently and diffable. |
