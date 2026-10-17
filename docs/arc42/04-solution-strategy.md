# 4. Solution Strategy

## Key decisions

- **A pure core behind ports.** `duty2cal-core` holds the domain, the application ports and the dependency-free ICS adapter; `duty2cal-pdf` is the only crate that touches the extraction dependency; `apps/cli` wires them. The compiler, `cargo deny`+clippy, archunit tests and review conventions enforce the inward-only rule (ADR-0002).
- **Staged pipeline with a completeness gate.** Source recognition → classification → endpoint date resolution → description composition → calendar composition → publication. The `Roster` aggregate reaches "complete" only when every required activity and field resolves; publication happens after completeness, so a partial calendar can never be published.
- **Determinism scoped to the source snapshot.** Component identity derives from the source snapshot plus the represented activity, not from an output counter or a title (opaque discriminator; the readable date/flight/route recipe was rejected as non-unique).
- **Create-new publication.** New calendars and diagnostics are written only to an unused name (create-new semantics with retry); existing or concurrently claimed files are never replaced.
- **Explicit failure as a first-class outcome**: the CLI names the source and the reason in a numbered diagnostic written beside it and reports the failure through its exit status; the use case returns a typed error rather than a partial calendar.
- **Infrastructure as code for the public home.** The organization, both repositories, the default branch's existing rule set and the Pages source are declared under `infra/` with OpenTofu and the GitHub provider (ADR-0005). The objects that already exist are bound once by a read-only adoption into an encrypted local state that is committed (ADR-0006), so a later preview can compare the declaration with what GitHub holds; no apply runs in Actions, because the administer token must not live in the repository it administers.
- **Release strategy.** The one supported platform, Windows x86_64, is cross-built on Linux and its executable is started on Windows before publication, so the archive is built where the project's toolchain lives and started where it must run. Packaging happens once on Linux: `scripts/prepare-release.sh` turns the one already-built Windows executable into one deterministic `.tar.gz` archive and checksum plus the release text, and refuses a set that is missing that executable or contains one for a platform outside the supported set. A crew member on any other platform builds the source with a Rust toolchain; no archive is offered for it. The release text is the newest section of the change log the repository keeps — rendered from the same history, grouped by kind and filtered to the changes a user receives (a revision whose change touched none of the paths a user's copy is made of is left out, and a dependency update that changed the dependency graph is announced under Dependencies), and drift-checked against the released revision — so the page and the repository cannot disagree. The release is published draft-first and immutably from the public repository, with build provenance and a bill of materials attached to every archive, and the published release is then read back and its checksums and attestations verified (ADR-0003; the draft-first immutable publication is ADR-0004).

## Technology choices

| Concern | Choice | Rationale (see ADR) |
|---|---|---|
| Language / runtime | Rust edition 2021, `stable` toolchain | Supplied constraint; chapter 2. |
| PDF extraction | `pdfplumber = "=0.4.1"` in `duty2cal-pdf` | Supplied pin (brief); chapter 2. |
| Calendar output | Hand-written RFC 5545 in `duty2cal-core/src/adapters/ics` | Supplied constraint. |
| Module boundaries | Two-crate workspace + four enforcement levels | ADR-0002. |
| Licence | MPL-2.0 for the whole project | ADR-0001. |
| Executable specification | Gherkin `.feature` files run by cucumber-rs (strict) | Chapter 2. |
| Distribution | One self-contained Windows x86_64 executable (cross-built on Linux, started on Windows), published as an immutable release from the public repository; other platforms are unsupported and build from source | ADR-0003, ADR-0004; chapters 2 and 7. |
| Change-log generation | `git-cliff` 2.14.2, pinned beside `cliff.toml` and rendered by `scripts/render-changelog.sh` | Chapter 2; `design.md`. |
| Infrastructure declaration | OpenTofu `>= 1.7` with `integrations/github` `~> 6.0`; HCL under `infra/` | ADR-0005; chapter 2. |
| Applied state and drift | Local, passphrase-encrypted state committed under `infra/`; the drift check runs on the maintainer's machine | ADR-0006; chapter 2. |

ADR-0001, ADR-0002, ADR-0003, ADR-0004, ADR-0005 and ADR-0006 are the project's ADRs; the remaining choices are supplied constraints (chapter 2), a local mechanism owned by `design.md`, or product behaviour owned by the capability specification and the glossary.

## Approach to quality goals

| Quality attribute | Approach |
|---|---|
| Correctness / completeness | Whole-plan completeness gate before publication; contradictions and unresolved activities reject the conversion. |
| Reliability (no data loss) | Create-new publication with enumerated retry; numbered preserved diagnostics; existing contents untouched. |
| Interoperability | Hand-written writer with CRLF, escaping and octet-aware folding, asserted on the published file. |
| Privacy | Generated/anonymised fixtures only; no private sources in the repository. |
| Maintainability / portability | Hexagonal layout with mechanical layer enforcement; one self-contained Windows x86_64 executable; a core free of I/O, clock and environment. |
| Reproducibility of the public home | One declaration under `infra/` reviewed with the code; a read-only adoption that binds the existing objects and changes nothing live; an encrypted committed state; a local drift check. |
| Release integrity / verifiability | One Windows build on Linux plus a start on Windows; a single deterministic packaging step with one archive, its checksum and the release text; draft-first immutable publication with provenance and a bill of materials; a read-back verification job. |
