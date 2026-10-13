# 4. Solution Strategy

## Key decisions

- **A pure core behind ports.** `duty2cal-core` holds the domain, the application ports and the dependency-free ICS adapter; `duty2cal-pdf` is the only crate that touches the extraction dependency; `apps/cli` wires them. The compiler, `cargo deny`+clippy, archunit tests and review conventions enforce the inward-only rule (ADR-0002).
- **Staged pipeline with a completeness gate.** Source recognition → classification → endpoint date resolution → description composition → calendar composition → publication. The `Roster` aggregate reaches "complete" only when every required activity and field resolves; publication happens after completeness, so a partial calendar can never be published.
- **Determinism scoped to the source snapshot.** Component identity derives from the source snapshot plus the represented activity, not from an output counter or a title (opaque discriminator; the readable date/flight/route recipe was rejected as non-unique).
- **Create-new publication.** New calendars and diagnostics are written only to an unused name (create-new semantics with retry); existing or concurrently claimed files are never replaced.
- **Explicit failure as a first-class outcome**: the CLI names the source and the reason in a numbered diagnostic written beside it and reports the failure through its exit status; the use case returns a typed error rather than a partial calendar.
- **Release strategy.** Each supported platform is built on a machine of its own kind, so the archive is built and started where it must run. Packaging happens once on Linux: `scripts/prepare-release.sh` turns the five already-built executables into one deterministic `.tar.gz` archive and checksum per platform plus the release text, and refuses an incomplete set. The release is published draft-first and immutably from the public repository, with build provenance and a bill of materials attached to every archive, and the published release is then read back and its checksums and attestations verified (ADR-0003; the draft-first immutable publication is ADR-0004).

## Technology choices

| Concern | Choice | Rationale (see ADR) |
|---|---|---|
| Language / runtime | Rust edition 2021, `stable` toolchain | Supplied constraint; chapter 2. |
| PDF extraction | `pdfplumber = "=0.4.1"` in `duty2cal-pdf` | Supplied pin (brief); chapter 2. |
| Calendar output | Hand-written RFC 5545 in `duty2cal-core/src/adapters/ics` | Supplied constraint. |
| Module boundaries | Two-crate workspace + four enforcement levels | ADR-0002. |
| Licence | MPL-2.0 for the whole project | ADR-0001. |
| Executable specification | Gherkin `.feature` files run by cucumber-rs (strict) | Chapter 2. |
| Distribution | One self-contained executable per supported platform (Windows x86_64, Linux x86_64/arm64 musl-static, macOS arm64/x86_64), published as an immutable release from the public repository | ADR-0003, ADR-0004; chapters 2 and 7. |

ADR-0001, ADR-0002, ADR-0003 and ADR-0004 are the change's ADRs; the remaining choices are supplied constraints (chapter 2) or product behaviour owned by the capability specification and the glossary.

## Approach to quality goals

| Quality attribute | Approach |
|---|---|
| Correctness / completeness | Whole-plan completeness gate before publication; contradictions and unresolved activities reject the conversion. |
| Reliability (no data loss) | Create-new publication with enumerated retry; numbered preserved diagnostics; existing contents untouched. |
| Interoperability | Hand-written writer with CRLF, escaping and octet-aware folding, asserted on the published file. |
| Privacy | Generated/anonymised fixtures only; no private sources in the repository. |
| Maintainability / portability | Hexagonal layout with mechanical layer enforcement; one self-contained executable per supported platform; a core free of I/O, clock and environment. |
| Release integrity / verifiability | One build per platform on a machine of that kind; a single deterministic packaging step with checksums and release text; draft-first immutable publication with provenance and a bill of materials; a read-back verification job. |
