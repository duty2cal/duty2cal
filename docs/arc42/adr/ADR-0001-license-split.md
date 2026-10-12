---
status: accepted
date: 2026-10-12
decision-makers: operator (kressnerd)
consulted: none
informed: none
---

# Split licensing by hexagon layer: MPL-2.0 core, GPL-3.0-or-later CLI

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

The project's arc42 qualification gate generally routes a licensing policy to its owning project document; this candidate passes `lower_level_artifact` through the gate's explicit exception because it introduces a durable per-path licence boundary across every crate and a distribution-channel boundary (a statically linked GPL CLI cannot ship in the iOS App Store without a separate grant). The register was empty before this record (`existing_adr_check`).

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort because: relicensing requires contributor consent — there is no CLA, only DCO sign-off, so once external GPL contributions to `apps/**` exist the copyright holder cannot relicense unilaterally.
- [x] **Shapes quality attributes** - It materially affects distributability/maintainability because: it fixes which host applications may embed the core and which channels may ship the CLI.
- [x] **Structural** - It defines a boundary, contract, or dependency direction because: it sets a licence boundary at the `crates/**` vs `apps/**` split that every future crate must respect.
- [x] **Must be shared** - Developers unfamiliar with it would systematically violate the architecture because: they would add a copyleft third-party crate or an SPDX header that breaks the combined-work rule.

## Context and Problem Statement

The core is intended to be embedded later (WASM, UniFFI, app-store apps) while the CLI is an end-user product. The supplied MIT/Apache-2.0 constraint was reversed by the operator's brief, which weighed permissive, LGPL and file-level-copyleft options for their effect on static linking and app-store distribution. The decision fixes the licence of every current and future file in the workspace.

## Decision Drivers

- Static linking of the core into proprietary or app-store-distributed apps must be permitted.
- End-user products (`apps/**`) should carry strong copyleft.
- No copyleft third-party dependency may enter the tree.

## Considered Options

- MIT OR Apache-2.0 everywhere
- LGPL-3.0 for the core
- MPL-2.0 core with GPL-3.0-or-later CLI
- MPL-2.0 everywhere

## Decision Outcome

Chosen option: "MPL-2.0 core with GPL-3.0-or-later CLI", because MPL-2.0 is file-level copyleft that permits static linking into a proprietary or app-store app while the CLI keeps strong copyleft.

### Consequences

- Good, because the core stays embeddable in closed host apps without a relinking obligation.
- Bad, because `apps/**` under GPL-3.0-or-later cannot ship in the iOS App Store without a separate licence grant by the copyright holder; recorded as a Phase-3 constraint in chapter 11.
- Bad, because the licence split couples the repository layout to the licence (a new crate must choose a side).

### Confirmation

`cargo deny check licenses` (allowlist, copyleft third-party crates forbidden), `reuse lint`, the CI `license-headers` job (every source file carries exactly one SPDX header), and a `cargo about generate` notices diff check on release.

## Pros and Cons of the Options

### MIT OR Apache-2.0 everywhere

- Good, because simplest and maximally embeddable.
- Bad, because no copyleft protection anywhere.

### LGPL-3.0 for the core

- Good, because it is copyleft with a linking exception.
- Bad, because the combined-work clause requires the recipient to relink against a modified library, which is not satisfiable for statically linked iOS App Store binaries.

### MPL-2.0 core with GPL-3.0-or-later CLI

- Good, because the core stays embeddable and the CLI stays strong copyleft.
- Bad, because of the App-Store constraint on the CLI.

### MPL-2.0 everywhere

- Good, because one licence.
- Bad, because the CLI would lose strong copyleft.

## More Information

The external brief's ADR-0001; `deny.toml`, `REUSE.toml`, `LICENSES/{MPL-2.0,GPL-3.0-or-later}.txt`. MPL §3.3 permits combining MPL code with GPL into a Larger Work as long as no file carries the Exhibit B "Incompatible With Secondary Licenses" notice — no file in this repository may carry it.
