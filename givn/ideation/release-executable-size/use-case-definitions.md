# Release executable size — use-case definition

Status: definition boundary review applied on 2026-10-18. The identifiers below
must survive the elaborated use cases, the change seeds and the permanent leaf
documents.

Vocabulary note: **distributed executable**, **build configuration**, **size
attribution** and **size lever** are the four confirmed terms. "Release
archive" keeps the durable glossary meaning and is never used for the
executable.

## Actor-goal cut

| Name and stable use-case ID | Level | Primary Actor | Goal | Boundary | Mutation |
|---|---|---|---|---|---|
| Measure the code's size for the distributed executable (`measure-distributed-executable-size`) | ! | Release maintainer | Obtain a size attribution of the code the distributed executable is built from, for a named revision and the supported platform. | The code and build configuration that produce the supported platform's executable; the release archive and its packaging are out. | Observing: reads a build of the revision and returns a size attribution; changes no committed file. |
| Reduce the distributed executable's size (`reduce-distributed-executable-size`) | ! | Release maintainer | Make the executable built for the supported platform smaller by applying only the size levers a measurement justifies. | The code, the build configuration, the size-lever decision record and the executable it produces; the release archive and its packaging are out. | Changing: changes the build configuration, the size-lever decision record and the built executable. |

Both are `!` leaves. No `+` summary case is needed: each success guarantee is
one outcome, and the per-lever decision is a rule of the reduction, not a
second outcome.

## Boundary

The boundary is the **code and build configuration that produce the supported
platform's executable**, plus the decision record and the built executable.
Two term collisions mark it:

- **Build configuration**, not **Declared configuration**: the latter states how
  the organization and its repositories should be configured, declared under
  `infra/` and applied to the live public home (glossary line 45); the former
  decides what the executable contains.
- **Distributed executable**, not **Release archive**, and not the symbol-bearing
  build used for the attribution: the archive is the `.tar.gz` a downloader
  receives and is owned by `publish-public-release`; the distributed executable
  is what the archive carries; the attribution's build is a separate artifact
  whose totals are not the executable's file size.

## Relationships

No `Includes` and no `Extends`. `measure-distributed-executable-size` is an
independent `!` leaf; its size attribution is a precondition and input of
`reduce-distributed-executable-size`, not a fragment, child or extension. The
graph stays acyclic. The measurement stands alone because it answers the
Frame's problem even when no lever is applied.

## Bounded outcomes

### Measure the code's size for the distributed executable

- **Trigger:** the release maintainer requests a measurement of a named revision,
  or the project's automation produces one for a change (open: Q6 — cadence).
- **Preconditions:** the named revision exists; the project's automation can
  build that revision for the supported platform.
- **Success guarantee:** the owner holds a size attribution that names the
  revision and the supported platform, divides the code's size by crate and by
  function, is obtained from a build of that revision that retains the names
  the attribution needs, and reports the stripped artifact's file size beside
  it; when that size cannot be read, the attribution states it is unavailable,
  and the two numbers are never substituted for each other.
- **Minimal guarantee:** a measurement that cannot be produced yields no
  attribution and names what could not be read; an attribution that cannot
  state the file size is reported as incomplete and never presents the split's
  total as the file size; no number is inferred from the release archive or
  from another project's measurement.
- **Closure test:** the owner can stop after reading the attribution; no build
  configuration and no repository file has changed.
- **Scope:** the code and build configuration that produce the supported
  platform's executable. The release archive, the packaging script and the
  publication flow are out.
- **Actors:** Release maintainer (primary); Project automation for the release maintainer
  (supporting — builds the revision and produces the measurement).
- **Stakeholders:** release maintainer; contributor reviewing a build-configuration
  change (reads the attribution as evidence); person identified in a duty plan
  (no plan-derived content in a published attribution or panic sample).

### Reduce the distributed executable's size

- **Trigger:** the release maintainer selects one size lever from the size
  attribution for the attributed revision.
- **Preconditions:** a size attribution for the revision is available (open:
  Q6 — whether an attribution exists for the current revision before a lever is
  selected); the stable toolchain, single-file and static-linking constraints
  hold.
- **Success guarantee:** the executable built for the supported platform from
  the landed build configuration is smaller than a rebuild of the same revision
  with the unchanged build configuration, by the recorded cumulative deltas.
- **Minimal guarantee:** no failure outcome becomes less visible than before: a
  panic that ends the process leaves a diagnosis, and a failed conversion still
  attempts a new numbered diagnostic naming the source PDF and the failure
  reason, and when no report can be saved the primary failure remains visible
  (`export-duty-plan-calendar`, `usecase.md:94-95,135`). Every refusal, build
  failure, rejection or revert is recorded with its reason. An accepted lever
  whose revision does not land is recorded as not landed, never as applied.
  When no lever yields a reduction, the build configuration is unchanged.
- **Closure test:** after the revision lands and the landed revision is rebuilt
  and measured, the maintainer can stop; the release flow proceeds exactly as
  before with a smaller executable.
- **Main flow:**
  1. Release maintainer selects one lever the size attribution justifies;
     System states the lever's constraint status before any build.
  2. Project automation rebuilds the released revision as the baseline with the
     current stable toolchain and the unchanged build configuration; System
     records the baseline's file size and reports the published binary's size
     separately.
  3. Project automation builds the revision with the lever; System reports the
     measured delta against a rebuild of the same revision carrying the
     previously accepted levers but not this one, and the artifact's file size.
  4. Release maintainer assesses the failure outcome; System reports whether a
     panic that ends the process still leaves a diagnosis, and whether a failed
     conversion still attempts a numbered diagnostic naming the source PDF and
     the failure reason.
  5. Release maintainer prepares the build-configuration revision; Contributor
     reviewer reviews it; System reports the review outcome.
  6. Release maintainer accepts or rejects the lever; System records the
     decision with its delta, the failure-outcome evidence, the review outcome
     and the reason.
  7. Release maintainer lands the accepted revision; System rebuilds the landed
     revision and reports its size against the same-revision rebuild with the
     unchanged build configuration.

  Steps 1–6 repeat for each candidate lever; one landing may carry several
  accepted levers, each with its cumulative delta.
- **Extensions:**
  - 3: the build fails → no delta is recorded; the rejection is recorded with
    the build failure as its reason; protects the minimal guarantee.
  - 3: the lever measures larger or equal → rejected with the measured
    comparison; protects the success guarantee from an unfounded change.
  - 4: the diagnosis is no longer preserved → the lever is refused and the
    refusal recorded with its reason; protects the minimal guarantee.
  - 5: the review rejects the revision → it is not accepted, does not land, and
    the rejection is recorded; protects the minimal guarantee.
  - 7: the landed rebuild is not smaller than the same-revision rebuild with
    the unchanged build configuration → the revision is corrected or reverted,
    and the discrepancy and the revert are recorded; protects the success
    guarantee.
- **Rules:**
  - Every proposed lever has one recorded decision: a refusal before a build
    names the violated constraint and records no measurement; otherwise the
    record states the measured delta, the failure-outcome evidence where the
    lever changes failure behavior, the review outcome where a revision was
    reviewed, and the reason. A lever is recorded as applied only after its
    landed revision was rebuilt and measured as smaller.
  - A nightly-only flag, UPX or dynamic linking is not a size lever: it is
    refused before a build and recorded as a rejected proposal naming the
    violated constraint.
  - Every delta is cumulative against the build configuration as it stood
    before that lever, so the landed difference equals the sum of the recorded
    deltas.
  - The baseline is the released revision rebuilt by the project's automation
    with the current stable toolchain and the unchanged build configuration;
    the published binary's size is reported separately.
  - Every delta is measured from a build that retains the names the split
    needs, and the stripped artifact's file size is reported separately.
  - The diagnosis a panic leaves — the panic message or a numbered diagnostic —
    is recorded before the lever lands.
  - A source build from the landed revision retains the conversion's failure
    diagnosis.
- **Scope:** the code, the build configuration, the decision record and the
  built executable; publishing, the archive and the supported platform set are
  excluded goals (`publish-public-release`).
- **Actors:** Release maintainer (primary); Contributor reviewer (supporting);
  Project automation for the release maintainer (supporting).
- **Stakeholders:** release maintainer; contributor reviewing a build-configuration
  change; person identified in a duty plan (published panic evidence);
  crew member building from source on an unsupported platform (inherits the
  changed panic behavior and the diagnosis guarantee).

## What is not a use case

- **A size ceiling** — parked in Q3. A gate is a policy over the measurement,
  not an actor goal. If the user later wants it, it is a rule of
  `publish-public-release`'s release preparation, which refuses a release that
  exceeds the bound — never a capability of the observing measurement use case.
- **The release** — `publish-public-release` owns packaging, publication and
  what an archive guarantees. This topic changes what the archive carries, not
  how it is published.
- **The conversion** — `export-duty-plan-calendar` owns the failure diagnosis;
  the reduction constrains the lever so the diagnosis survives; it does not own
  or specify it.
- **A different PDF extraction backend** — a product decision with its own ADR;
  the measurement may recommend it, and this topic does not decide it.
- **The WASM facade's payload** — a later phase with its own payload question.
- **UPX or dynamic linking** — a distribution-form change contradicting the
  single-file and static-linking constraints; a refused lever, not a goal.
- **The review and landing of the revision** — the gate on landing is a project
  rule enforced by the repository, not an actor goal (glossary line 56).
- **The attribution's cadence and CI wiring** — a policy under Q6, not a goal.
- **The archive's compressed size** — a different quantity; the release archive
  is out of the boundary.
- **A nightly experiment** — excluded by the stable-toolchain constraint; not a
  goal, a rejected alternative.
- **Source-level dead-code pruning** — out of scope until an attribution points
  at `core::fmt`; revisit after the attribution.

## Personas and stakeholders

| Use case | Stakeholder types | Review lens |
|---|---|---|
| Measure the code's size for the distributed executable | Release maintainer; contributor reviewing a build-configuration change; person identified in a duty plan | `project-maintainer--release-owner`, `contributor--build-configuration-reviewer`, `crew-member--personal-calendar-owner` |
| Reduce the distributed executable's size | Release maintainer; contributor; person identified in a duty plan; crew member building from source | all three confirmed Personas |

## Open questions

- Q3 — the size ceiling (parked; not blocking).
- Q6 — measurement cadence: every change, every release, or on request.
