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
| Windows x86_64 is the only supported platform and the only target offered for download: `x86_64-pc-windows-gnu` | One executable for the one supported platform; nothing outside this set is supported, tested or distributed. Other platforms may build the source with a Rust toolchain. |
| Linux (`ubuntu-latest`) remains the CI, coverage and local test host and the host that cross-builds the Windows executable; it is not a published platform | The suite runs on Linux and the Windows executable is cross-built there. This is engineering context for the project's own build and evidence path, not a platform promise. |
| Release packaging is a Linux-only POSIX-shell script (`scripts/prepare-release.sh`) outside the workspace | Packaging is build tooling, not product behaviour; it adds no workspace member and stays off the coverage denominator. |
| CI validates the workflows with `rhysd/actionlint:1.7.12` and enforces DCO sign-off on pull requests (`scripts/dco-check.sh`) | The release path only fails on the host, so a schema/expression check precedes the push; sign-off is required without granting a third-party app repository access. |
| Every release archive carries a build provenance attestation and a bill of materials (`actions/attest@v4`, `anchore/sbom-action@v0`) | A downloader can trace the file in hand to the tagged revision and this project's automation. |
| `git-cliff` 2.14.2 pinned beside `cliff.toml` in the repository, installed in CI with `taiki-e/install-action@v2` (`tool: git-cliff@2.14.2`) | The change-log gate compares generated text byte for byte, so the generator's output shape is part of the contract; an unpinned tool would make the comparison meaningless, as the `cargo-about` 0.9.0 pin already records. |
| A `changelog` CI job refuses a committed `CHANGELOG.md` that differs from a fresh render of the released revision, naming the version whose section differs, and the release preparation runs the same check before it stages anything | The released version's notes are that file's newest section, so a stale committed change log would publish a claim the repository does not hold; and the publish job now waits for the tagged revision's check runs and refuses a concluded non-success or the bounded wait, while the preparation's own drift check remains an independent pre-publication check. |
| The capability allow-list — which capability scopes are released changes — is the set of use-case ids present as directories under `givn/specs/`, materialised in `cliff.toml` and checked against the corpus by the renderer | `fix(ci)` is shaped exactly like `fix(publish-public-release)`, so the corpus, not the subject's shape, is the source of truth for what a capability is; a dependency update (`chore(deps): …`) is released without a capability scope through its own parser, below this allow-list. |
| A marked machinery skip list in `cliff.toml` — distinctive tooling phrases (`coverage report`, `ci workflow`, `e2e transcript`, …), realized as subject-anchored `skip` parsers and read by the renderer, which refuses a term the configuration does not realize | A `fix(<capability>):` revision that only reports on a coverage report, a CI or release workflow or an e2e transcript is not a change a reader can act on, and the type-plus-scope allow-list cannot see that; each term is a phrase rather than a bare word, and matching the subject only keeps a body that discusses the tooling from silencing a released change or a subject that merely borrows the word for a crew member's own work. |
| A marked list of the paths a user receives — the converter's crates and application, the build inputs that decide what the executable contains, and the packaging and release workflow that decide what an archive holds — read by the renderer, which refuses a listed path the tree does not hold | The allow-list and the phrase list both judge a revision by its words, and a subject written in the project's own vocabulary is invisible to a reader; a revision whose change touched none of these paths is left out of the notes whatever its subject says, and the list is a maintained contract: a change that does reach a downloader but not through these paths is announceable by adding the path. |
| The change log is rendered by a Linux-only POSIX-shell script (`scripts/render-changelog.sh`) outside the workspace | Rendering is build tooling, not product behaviour; it adds no workspace member and stays off the coverage denominator, like `scripts/prepare-release.sh`. |

## Organisational constraints

| Constraint | Motivation |
|---|---|
| Licence: MPL-2.0 project-wide (per-file weak copyleft, App-Store compatible) | ADR-0001. |
| `deny.toml` allowlist; any GPL/LGPL/AGPL/SSPL third-party crate is a hard failure | Keeps the project embeddable in any host app. |
| Per-file SPDX header; `REUSE.toml` for binary/data files; `cargo-about` notices; DCO sign-off, no CLA | ADR-0001 obligations. |
| Private duty-plan PDFs are read for analysis but never committed; fixtures and golden calendars are generated/anonymised | Privacy. |
| Changes follow the givn workflow; `.feature` files are the executable specification | Specification and tests are one artifact. |
| Public repository at `kressnerd/duty2cal` | The project's public home and release channel; standard GitHub-hosted runners, attestations and secret scanning are free there. |
| Published releases are immutable; a version tag is never reused | A downloader's file and the provenance covering it must not be replaceable after publication. |
| DCO sign-off enforced on pull requests (`scripts/dco-check.sh`), no CLA | The project's contribution terms; a change without sign-off does not land. |
| Contributor routes published: `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, and the issue and pull-request templates | A contributor finds the contribution route, the sign-off, the conduct policy and the confidential reporting route without asking. |
| No private material in public assets: push protection and secret scanning block credentials, the private-data guard blocks duty-plan data | A public repository adds a credential surface; private duty plans must never enter the public history or a public asset. |

## Conventions

| Convention | Motivation |
|---|---|
| Dependencies point inward; each adapter implements exactly one port with no business logic | ADR-0002. |
| Value objects are built only through validating `Result` constructors with no public fields; ports are traits in `application` and know no adapter type; every port is exercised through the real adapters rather than a test double | ADR-0002, level 4. |
| No path or file handle crosses the use-case boundary (input `&[u8]`, output `String`) | Keeps one use case callable from CLI, WASM and UniFFI. |
| Strict Gherkin execution (`.fail_on_skipped()`); one step-definition file per capability | Prevents false completion. |
| Diagrams in arc42/design are Mermaid | Rendered consistently. |
