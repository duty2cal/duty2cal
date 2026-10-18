# Proposal: measure-executable-size

## Release note

> A size attribution of the Windows executable is produced for every pushed
> revision, naming what its bytes are spent on beside the stripped
> executable's own file size.

## Use-Case Context

- Use-case ID: `measure-distributed-executable-size` — "Measure the
  distributed executable's size". This change creates that permanent leaf from
  the ideation contract; it owns no other use case.
- Ideation topic: `release-executable-size`.
- Seed path:
  `givn/ideation/release-executable-size/changes/measure-executable-size.md`
  — its status line now reads `Status: proposed by measure-executable-size`;
  the rest of the seed is unchanged.
- Confirmed Personas, confirmed for this topic through Q4 on 2026-10-18 and
  reused from the project collection:
  - `project-maintainer--release-owner` — reads the attribution as evidence.
  - `contributor--build-configuration-reviewer` — judges whether the
    attribution is reproducible and names the right artifact.
  - `crew-member--personal-calendar-owner` — the privacy lens: no
    plan-derived content appears in a published attribution or sample.
- Personas are review lenses, never Gherkin Actors. The Actor of this change's
  scenarios is the Release maintainer; the project's automation supports it.
- The change adds no new Persona and records no Persona biography.

## Problem / Opportunity

The published Windows executable is 2,097,152 B (v0.3.0) and the project
cannot say what occupies it. The only number cited as evidence today is the
compressed archive — 1,035,358 B for v0.3.0 — which is a different quantity
from the executable that decides the archive's size.

- The committed build configuration already applies the stable levers that
  need no measurement: `opt-level = "z"`, link-time optimisation, stripping
  and one code-generation unit. Everything left — panic behaviour, `"s"`
  versus `"z"`, and dependency feature selection — rests on an unmeasured
  5–20% estimate.
- The gate on landing already cross-builds the Windows x86_64 executable on
  every pushed revision and starts it, so the artifact a measurement would
  describe already exists for every change; nothing measures it.
- A contributor revising the build configuration therefore cannot show what
  the revision does to the executable, and the release maintainer cannot
  explain the two megabytes to a reviewer or to the next decision.

The opportunity is to make the missing fact exist: for every pushed revision,
a measurement of where the executable's bytes go, taken from the supported
platform's build, with the shipped executable's own size reported beside it
and never substituted for it. That is the precondition the topic's remaining
work already states.

## Proposed Solution

For a named revision and the supported platform (Windows x86_64), the project
produces a **size attribution**:

- **It names what it describes.** The attribution names the revision it
  describes and the supported platform. It is produced for every pushed
  revision by the project's own automation, and the release maintainer can
  request the same measurement for any named revision, such as `v0.3.0`.
- **It explains the code's size.** It divides the size by crate and by
  function, and it is taken from a build of that revision that keeps the names
  the attribution needs. Stripping is disabled for that attribution build
  only; the distributed executable keeps the shipped configuration.
- **The executable's own size is reported beside it, never as it.** The
  stripped executable built from the same revision and build configuration is
  measured separately, and its file size is stated beside the split. The two
  numbers are different quantities and are never substituted for each other;
  the attribution's own total is not presented as the executable's file size.
- **A reviewer sees it before the lever lands.** The attribution for a
  revision is available where that revision's run is reviewed, so a change to
  the build configuration is judged against the measured evidence it produces
  rather than after the fact.
- **Failure is visible, never invented.** A build that fails produces no
  attribution, and the report states that the build could not be read. A build
  that keeps no names produces no attribution, and the report names the
  missing capability. A stripped executable whose file size cannot be read
  leaves the attribution explicitly incomplete: it states that the file size
  is unavailable and still never presents the split's total as the file size.
- **No substitute evidence.** A build for the host machine is not accepted as
  evidence about the supported platform, and the release archive's compressed
  size is never used as the executable's size.

The published executable, the release archive, the packaging and publication
of a release, and the product's own behaviour are unchanged.

## Capability Routing

`givn spec route --change measure-executable-size` reports
`EXTEND export-duty-plan-calendar` (score 13.00, confidence: signal): the
proposal's words overlap the conversion use case's goal and interaction text.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `measure-distributed-executable-size` | `EXTEND export-duty-plan-calendar` | `NEW in measure-distributed-executable-size` | The ideation confirmed this permanent leaf for the measurement, so the capability belongs to it, not to the conversion. The placement test asks which actor goal the capability advances: measuring the distributed executable advances the release maintainer's goal of accounting for the executable's size; exporting a calendar advances the crew member's goal of obtaining a calendar file. Route's overlap is lexical (`route`, goal words) and names no ownership; no conversion rule changes here. |

## Out of Scope

- **Applying any size lever.** `panic = "abort"`, `opt-level = "s"` versus
  `"z"`, and dependency feature selection are the sibling change
  `reduce-executable-size`, which consults this measurement.
- **A size ceiling.** This change measures; it does not gate a release or a
  revision on a bound (D2, seed Q3).
- **Nightly-only levers, UPX and dynamic linking.** They are excluded by the
  stable-toolchain and distribution constraints the Frame records.
- **Replacing the PDF extraction backend.** A product decision with its own
  ADR; the measurement may point at it, this change does not decide it.
- **The WASM facade's load time.** A later phase with its own payload
  question.
- **Source-level dead-code pruning.** Revisit after the attribution exists.
- **Changing the distributed executable or the release channel.** Its bytes,
  its archive, its packaging, its provenance and its publication are
  untouched; the measurement describes what is already built.
- **Measuring the release archive.** The archive's compressed size is a
  different quantity and is not evidence here.
- **A committed, hand-maintained size history.** No number is copied into a
  document by hand; the measurement is produced for the revision that carries
  it (D4 decides where the report is carried).

## Open Questions

### D1 — When is the attribution produced? (seed Q6)

- Question: Is the attribution produced for every change, for every release,
  or only when someone asks for it?
- Options:
  1. **Every change** — the project's automation that already builds the
     Windows executable produces and publishes the attribution for every
     pushed revision. Gain: the decision consults evidence that already exists
     when a build-configuration change is reviewed, and a reviewer sees the
     same number as the release maintainer. Cost: automation time on every
     push and a report shape to maintain; a report nobody reads is pure cost.
     In practice: a pull request that changes the profile shows its measured
     delta before it lands.
  2. **Every release only** — the attribution is produced while a release is
     prepared. Gain: no change to the pull-request path. Cost: the lever is
     reviewed without its delta and can only be reverted, not rejected before
     landing. In practice: the release maintainer sees the attribution after
     the lever has already landed.
  3. **On demand** — the release maintainer runs the measurement locally when a
     decision needs it. Gain: no automation to maintain. Cost: no reproducible
     evidence in the review, and the measurement depends on a workstation that
     can cross-build. In practice: a reviewer cannot check the number the
     maintainer quotes.
- Disposition: **answered** — option 1, **every change**. The user closed Q6 on
  2026-10-18: the reduction's review must consult a delta that exists before a
  lever lands, and on a release-only cadence the evidence arrives after the
  lever has landed. The measurement is therefore produced for every pushed
  revision, and the release maintainer can additionally request it for any
  named revision.

### D2 — Is a size ceiling added by this change? (seed Q3)

- Question: Should the measurement also refuse a revision or a release whose
  executable exceeds a bound?
- Options:
  1. **A ceiling now** — preparation or the gate fails above a bound. Gain:
     growth is caught. Cost: the bound has no measured headroom; a tight
     number blocks a legitimate update and a loose one catches nothing. In
     practice: a dependency bump that adds 60 KB refuses a release nobody
     measured a budget for.
  2. **Measurement only** — the attribution is produced and read; nobody is
     refused. Gain: no threshold without evidence. Cost: a regression is
     visible only to a reader who compares reports. In practice: each revision
     has a report and the decision stays with the release maintainer.
  3. **A ceiling on the archive** — the compressed download is bounded
     instead. Gain: it gates the number a downloader experiences. Cost: it
     moves for reasons other than executable size and is not the artifact the
     levers change. In practice: a packaging change trips a size check that
     was meant for the profile.
- Disposition: **out of scope** — the seed's accepted decision is measurement
  only (its "No gate in this seed"), Q1 selected measured reduction rather
  than a growth gate, and Q3 stays parked until the reduction establishes a
  floor. This change introduces no threshold and refuses no revision on size.

### D3 — Which change carries the reduction's success example? (seed Q8)

- Question: Which measured lever and delta becomes the reduction's success
  example?
- Options:
  1. **The first measured lever** — the delta from the first lever the
     measurement justifies. Gain: a real, reproducible example. Cost: written
     after the measurement, not during ideation. In practice: the baseline and
     landed sizes measured on the supported platform's build.
  2. **A generated comparison** — a synthetic build pair in the tests. Gain:
     independent of the first release. Cost: a fixture the size work otherwise
     does not need. In practice: two builds differing only by the lever.
  3. **No numeric example** — the guarantee stands without one. Gain: nothing
     invented. Cost: a reader cannot see the success path concretely. In
     practice: only the rejection example remains.
- Disposition: **out of scope** — the example belongs to the use case that
  applies a lever (`reduce-executable-size`), and this change applies none.
  Seeded by the topic's question log; it is not this change's decision.

### D4 — How is the attribution requested, and where does a reviewer read it?

- Question: The use case says the release maintainer can request a
  measurement and that the automation produces one for a change. Through what
  interface is it requested, and where is the finished attribution carried so
  a reviewer can compare revisions?
- Options:
  1. **A repository command, with the report carried beside the run** — the
     measurement is a command the automation and the release maintainer both
     run, and its report is published with the revision's run. Gain: one
     interface for both callers, a report per revision, and no repository
     growth. Cost: a reviewer must open the run rather than read a file in the
     tree; the report is not part of the history. In practice: the Windows
     build's run for revision `abc1234` carries the attribution for `abc1234`.
  2. **A committed report in the repository** — the attribution is written to
     a tracked file for each revision. Gain: it is readable in the tree and
     survives with the history. Cost: the file changes on every revision even
     when nothing about the size changed, the history grows, and a drift check
     or refresh step must maintain it. In practice: every commit touches a
     generated size file, and a stale one fails a check.
  3. **A release-preparation output only** — the attribution is produced while
     a release is prepared. Gain: no per-revision work. Cost: it contradicts
     D1: no evidence exists when a lever is reviewed. In practice: the report
     arrives with the release it cannot influence.
- Disposition: **carried** — owner: `design.md`, which decides the interface,
  the report's shape, and where it is carried; the `specs` artifact records the
  behaviour, and the design's Decisions ledger records the resolution as
  `Decides D4`.
