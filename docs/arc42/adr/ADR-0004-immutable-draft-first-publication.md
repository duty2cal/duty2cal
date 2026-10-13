---
status: accepted
date: 2026-10-13
decision-makers: operator (kressnerd)
consulted: none
informed: none
---

# Publish releases immutably from an inspected draft

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

The amendability of a published release is a durable publication boundary, not a local implementation detail: it fixes what may change after publication and makes the release record append-only, so every future release and every provenance statement is bound by it. It is independent of ADR-0003's channel and platform-set boundary (which repository publishes, for which machines) and must not be bundled with it, so it gets its own record. `design.md` owns the concrete mechanism (the `gh release` draft-then-publish flow, the operator's immutability setting, the `verify` job). The register already holds ADR-0001 (licence), ADR-0002 (layer model) and ADR-0003 (channel and platform set); none covers this boundary, so the route is a new record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort because: a published immutable release cannot be withdrawn or repaired, its tag is never reusable even after the release is deleted, and a correction must be issued as the next version rather than as a repair in place.
- [x] **Shapes quality attributes** - It materially affects verifiability because: it makes the published asset set unchangeable, so the build provenance and the bill of materials cover the file a downloader actually holds.
- [x] **Structural** - It defines a boundary, contract, or dependency direction because: it fixes what may change after publication — the release record is append-only — and requires the draft-then-inspect-then-publish order before anything becomes public.
- [x] **Must be shared** - Developers unfamiliar with it would systematically violate the architecture because: they would hand-attach a release asset, re-upload a replacement under an existing tag, or publish from an uninspected draft.

## Context and Problem Statement

Attaching build provenance and a bill of materials to each archive only reassures a downloader if the provenance covers the file they actually hold. If a release's assets stay replaceable, the provenance can be re-uploaded beside a substituted file with no trace, and neither a downloader nor the attestation can tell which file was the published one. This decision fixes whether a published release can change after publication; the concrete publishing mechanics are owned by `design.md`.

## Decision Drivers

- The provenance statement must cover the file a downloader actually holds, or it is worthless.
- Neither a downloader nor the provenance can detect a swapped asset after the fact.
- A release should be inspected as a complete set before anything becomes public.
- A new release path benefits from being able to correct a mistake, which cuts against immutability.

## Considered Options

- Immutable releases, prepared as a draft and published only after verification
- Mutable releases, assets replaceable in place

## Decision Outcome

Chosen option: "Immutable releases, prepared as a draft and published only after verification", because verifiability is the point of attaching provenance and a bill of materials to each asset, and only an immutable record makes those statements about the file a downloader holds.

### Consequences

- Good, because an asset or a tag cannot be replaced once public, so the build provenance and the bill of materials cover the file a downloader actually holds, and the repository issues a release attestation for the release as a whole.
- Good, because the draft-first order attaches the complete, inspected set before the release becomes public, so an incomplete or wrong set is never published.
- Bad, because a botched release cannot be repaired in place and a used tag is never reusable, so a mistake costs the next version; recorded in chapter 11 (R-012).

### Confirmation

The `verify` job in `.github/workflows/release.yml` reads the published release back on every release: it runs `gh attestation verify` on every archive and asserts the release reports `immutable: true` through the releases API, which fails loudly if the record ever became changeable. The operator applies the "Enable release immutability" setting under Settings → Releases and confirms it as this change's evidence.

## Pros and Cons of the Options

### Immutable releases, prepared as a draft and published only after verification

- Good, because the provenance statement and the bill of materials cover the file a downloader actually holds.
- Good, because the complete asset set is attached and inspected before the release is public.
- Bad, because a mistake cannot be repaired in place and a used tag is never reusable; it costs the next version.

### Mutable releases, assets replaceable in place

- Good, because a mistake in the first public release costs no version number, which matters while the release path is new.
- Bad, because neither a downloader nor the provenance can tell whether the file they hold is the published one; a checksum could be re-uploaded with no trace.

## More Information

The proposal's open question D3 (immutable, draft-first publication); `design.md` ("Technology decisions", "The release pipeline", "The public home's owner-applied settings", risk R-012); chapter 11; ADR-0003 records the release channel and the platform set; ADR-0001 and ADR-0002 are unaffected by this decision.
