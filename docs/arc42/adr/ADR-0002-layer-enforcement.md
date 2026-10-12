---
status: accepted
date: 2026-10-12
decision-makers: operator (kressnerd)
consulted: none
informed: none
---

# Enforce the hexagonal layer model with crate boundaries and four policy levels

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

The layering model is a durable structural contract, not a local implementation detail: it fixes module boundaries and the dependency direction every future crate must obey. `design.md` owns the concrete mechanism; the model itself is the architectural decision. The register was empty before this record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort because: it re-shapes the workspace, every crate manifest, and the CI jobs that enforce it.
- [x] **Shapes quality attributes** - It materially affects testability/maintainability because: it makes the core I/O-free, clock-free and environment-free by construction.
- [x] **Structural** - It defines a boundary, contract, or dependency direction because: crate boundaries encode the inward-only rule and the core clippy policy encodes the purity rule.
- [x] **Must be shared** - Developers unfamiliar with it would systematically violate the architecture because: a convenience import would put I/O or `SystemTime` into the domain and the build would fail without an explanation.

## Context and Problem Statement

Clean Architecture was a supplied constraint but was previously expressed only as module convention inside one crate, which the compiler could not enforce. The operator's brief replaced that with a reduced two-crate workspace plus explicit policy levels so the inward-only rule and the purity of the core are mechanically checked rather than reviewed.

## Decision Drivers

- The domain must not perform I/O, read a clock, or touch the environment.
- The dependency direction must be mechanically enforced, not convention only.
- Only one crate may depend on the extraction dependency.

## Considered Options

- Single crate with module layering, enforced by review only
- Per-adapter crate split (one crate per adapter)
- Reduced two-crate workspace (`duty2cal-core` + `duty2cal-pdf`) with four enforcement levels

## Decision Outcome

Chosen option: "Reduced two-crate workspace with four enforcement levels", because a boundary should exist only where dependencies, target platform, or `crate-type` differ, and the four levels (compiler, `cargo deny` + clippy, archunit tests, review) cover what each can enforce.

### Consequences

- Good, because the domain physically cannot perform I/O, read a clock, or read the environment.
- Bad, because `archunit` is pre-1.0 and syntax-based (file granularity, no macro expansion), so some intra-crate violations are not detectable; recorded as technical debt in chapter 11.
- Bad, because the workspace carries more manifests and internal dependencies than a single crate.

### Confirmation

`cargo test -p duty2cal-core --test architecture`, `cargo clippy --workspace -- -D warnings`, `cargo deny check`, plus the review conventions (one port per adapter, validating constructors, port fakes, no `unwrap`/`expect` outside tests).

## Pros and Cons of the Options

### Single crate with module layering

- Good, because one manifest and the simplest build.
- Bad, because the compiler cannot stop `domain` importing an adapter or `std::fs`.

### Per-adapter crate split

- Good, because maximum isolation.
- Bad, because it creates boundaries that do not correspond to a dependency, platform or `crate-type` difference.

### Reduced two-crate workspace with four enforcement levels

- Good, because each boundary is justified and each rule is enforced at its lowest possible level.
- Bad, because of the extra manifests and the pre-1.0 architecture-test tool.

## More Information

The external brief §4 (replacement); `design.md` "Layer rules (four levels)"; `crates/duty2cal-core/clippy.toml`; `deny.toml` `[bans]`.
