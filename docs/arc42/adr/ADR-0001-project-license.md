---
status: accepted
date: 2026-10-12
decision-makers: operator (kressnerd)
consulted: none
informed: none
---

# Licence the project under MPL-2.0 (per-file weak copyleft)

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
routing: AMEND_ADR
canonical_artifact: null
target_adr: ADR-0001
replacement_adr: null

Amended record: the earlier accepted outcome split licensing by layer (MPL-2.0 crates, GPL-3.0-or-later apps). The operator's stated goal — publishing an iOS UI in the App Store — is incompatible with the GPL application layer, so the same decision boundary (project licensing) is narrowed to a single licence rather than given a parallel ADR. The project's arc42 gate generally routes licensing policy to its owning project document; `lower_level_artifact` passes through the gate's exception because the choice fixes a durable per-file obligation and a distribution-channel boundary across every crate and app.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort because: relicensing requires contributor consent — there is no CLA, only DCO sign-off, so once external contributions exist the copyright holder cannot relicense unilaterally.
- [x] **Shapes quality attributes** - It materially affects distributability/maintainability because: it fixes which channels may ship the products and what a redistributor of a modified app must publish.
- [x] **Structural** - It defines a boundary, contract, or dependency direction because: it sets a per-file licence obligation that every current and future file in the workspace must satisfy.
- [x] **Must be shared** - Developers unfamiliar with it would systematically violate the architecture because: they would add a copyleft-incompatible third-party crate or a contradictory SPDX header.

## Context and Problem Statement

The core is intended to be embedded later (WASM, UniFFI) and the operator intends to publish an iOS UI in the App Store. The supplied MIT/Apache-2.0 constraint was reversed by the operator's brief, which weighed permissive, LGPL and file-level-copyleft options for their effect on static linking and app-store distribution. An earlier version of this record split the licence by layer; the operator has since decided that every `apps/**` product, including the future iOS UI, must be App-Store distributable, which removes the GPL application layer.

## Decision Drivers

- The operator intends to publish an iOS UI in the App Store in the future, so every product and the core must be licensable on terms Apple permits.
- Static linking into a proprietary or app-store-distributed app must be permitted.
- The project's own code should carry copyleft protection that a redistributor cannot escape by including it in a larger proprietary work.
- No copyleft third-party dependency may enter the tree.

## Considered Options

- MIT OR Apache-2.0 everywhere
- LGPL-3.0 for the core
- MPL-2.0 core with GPL-3.0-or-later applications
- MPL-2.0 for the whole project

## Decision Outcome

Chosen option: "MPL-2.0 for the whole project", because MPL-2.0 is file-level copyleft that permits static linking into a proprietary or app-store-distributed app, so it satisfies the App Store goal for every app while still protecting the project's own source files.

### Consequences

- Good, because the App Store path is open for the future iOS UI and for any other app-store product.
- Good, because a redistributor of a modified product must publish the modified MPL files.
- Bad, because there is no strong copyleft: a redistributor may combine the MPL code into a larger proprietary or store-distributed application and need only publish the MPL files they modified; recorded in chapter 11 (R-007).

### Confirmation

`cargo deny check licenses` (allowlist includes MPL-2.0; copyleft third-party crates forbidden), `reuse lint`, the CI `license-headers` job (every source file carries exactly one SPDX header), and a `cargo about generate` notices diff check on release.

## Pros and Cons of the Options

### MIT OR Apache-2.0 everywhere

- Good, because simplest and maximally embeddable.
- Bad, because no copyleft protection anywhere.

### LGPL-3.0 for the core

- Good, because it is copyleft with a linking exception.
- Bad, because the combined-work clause requires the recipient to relink against a modified library, which is not satisfiable for statically linked iOS App Store binaries.

### MPL-2.0 core with GPL-3.0-or-later applications

- Good, because the core stays embeddable while the applications are strong copyleft.
- Bad, because a GPL application cannot ship in the App Store without a separate copyright-holder grant, and a single external contribution closes that route permanently under DCO-only contribution terms.

### MPL-2.0 for the whole project

- Good, because every product is App-Store distributable and the project's files stay copyleft.
- Bad, because the protection stops at file boundaries.

## More Information

The external brief's ADR-0001 (originally a licence split); `deny.toml`, `REUSE.toml`, `LICENSES/MPL-2.0.txt`. Because no GPL code remains, the MPL §3.3 "Larger Work" combination clause and its Exhibit B notice are no longer relevant to this repository.
