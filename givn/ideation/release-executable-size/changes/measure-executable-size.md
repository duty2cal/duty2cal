# Change seed: measure-executable-size

Status: implemented in measure-executable-size

- Use-case IDs: `measure-distributed-executable-size`
- Topic: `release-executable-size`
- Personas: `project-maintainer--release-owner`, `contributor--build-configuration-reviewer`, `crew-member--personal-calendar-owner`
- Suggested start: first; no dependency. Run `/givn-propose` before `/givn-propose reduce-executable-size`.
- Handoff: published by the user's confirmed Handoff on 2026-10-18; not proposed or implemented.

## Accepted decisions

### Measure the supported platform's build, not the host build

- Context: The only artifact a crew member runs is the Windows x86_64
  executable; a Linux host build would describe a different artifact.
- Options:
  1. **Host build** — measure the local Linux build for speed.
     In practice: a report on `target/release/duty2cal`; its crate weights do not
     describe the size-aligned PE file.
     Gain: no cross-build needed.
     Cost: evidence that cannot justify a Windows-only change.
  2. **Supported platform's build** — measure `x86_64-pc-windows-gnu`.
     In practice: the attribution is produced on the project's automation, not
     this workstation.
     Gain: the numbers describe the artifact being decided about.
     Cost: the measurement depends on the automation's cross-build capability.
- Chosen: **Supported platform's build** because the use case's guarantee names
  the supported platform. Rejected: **Host build** because it would substitute a
  different artifact.

### Two numbers, never substituted

- Context: `cargo-bloat` disables `strip` to read symbols, while the shipped
  profile strips the executable; attribution totals are not file size.
- Options:
  1. **Attribution only** — report the crate/function split.
     In practice: a table of crate weights with no comparable file size.
     Gain: one number to maintain.
     Cost: no way to relate the split to the 2,097,152-byte artifact.
  2. **Attribution plus stripped file size** — report both, side by side.
     In practice: the split's total and 2,097,152 B are stated together and
     never conflated.
     Gain: the decision sees the artifact and its explanation.
     Cost: a second measurement step.
- Chosen: **Attribution plus stripped file size** because the Frame's problem is
  precisely the conflation of the archive, the stripped file and the attribution.
  Rejected: **Attribution only** because it leaves the file size unexplained.

### No gate in this seed

- Context: Q1 selected measured reduction, not unnoticed growth; growth is not
  observed (net −56 B across three releases).
- Options:
  1. **Add a size ceiling now** — fail a release above a bound.
     In practice: a dependency bump that grows the executable refuses the
     release; the bound has no measured headroom.
     Gain: growth is caught.
     Cost: a threshold that can block a legitimate update and has no evidence
     behind it.
  2. **Measurement only** — produce and report the attribution.
     In practice: each measured revision has a report; nobody is refused.
     Gain: no unmeasured threshold.
     Cost: a regression is visible only to a reader who compares reports.
- Chosen: **Measurement only** because no headroom has been measured. Rejected:
  **Add a size ceiling now** because it is parked in Q3 until the reduction
  establishes a floor.

## Open questions

### When is the attribution produced?

Resolved by D1 (2026-10-18): the attribution is produced for **every pushed
revision** by the gate's Windows cross-build, and it can be requested for any
named revision. Every change means the cross-build step also publishes an
attribution, so a pull request that changes the profile shows its delta before
landing; a named revision (for example `v0.3.0`) is measured on request by the
same command. Rejected: release only (the evidence would arrive after a lever
lands) and on demand only (no automated evidence in the review).
