# Proposal: reduce-executable-size

## Release note

> The Windows executable is smaller: a panic aborts instead of unwinding, so
> the unwinding tables leave the release build.

## Use-Case Context

- Use-case ID: `reduce-distributed-executable-size` — "Make the executable
  built for the supported platform smaller by applying only the size levers a
  measurement justifies." This change creates that permanent leaf from the
  ideation contract.
- Ideation topic: `release-executable-size`.
- Seed path:
  `givn/ideation/release-executable-size/changes/reduce-executable-size.md` —
  its status line now reads `Status: proposed by reduce-executable-size`; the
  rest of the seed is unchanged.
- Confirmed Personas, reused from the project collection:
  - `project-maintainer--release-owner` — decides and answers for the result.
  - `contributor--build-configuration-reviewer` — judges the measured saving
    and the stated consequence.
  - `crew-member--personal-calendar-owner` — the download and failure-diagnosis
    lens; also the privacy lens for published panic evidence.
- Personas are review lenses, never Gherkin Actors. The Actor is the Release
  maintainer; the project's automation supports it.
- The change adds no new Persona.

## Problem / Opportunity

The distributed executable is 2,098,176 B at the release-branch baseline. The
committed profile already carries `opt-level = "z"`, `lto = true`,
`strip = true` and `codegen-units = 1`; the remaining stable lever with a
measured target is `panic = "abort"`.

Measured on the release branch with the supported platform's build:

| Candidate | Lever build | Saved vs baseline | Decision |
|---|---:|---:|---|
| baseline (unchanged configuration) | 2,098,176 B | — | — |
| `panic = "abort"` | 2,014,208 B | 83,968 B (4.00%) | apply |
| `opt-level = "s"` | 2,172,416 B | larger by 74,240 B | reject |
| `panic = "abort"` + `opt-level = "s"` | 2,084,864 B | larger by 70,656 B against the panic build | reject the second lever |

The feature audit is measured-empty (`pdfplumber` enables only `default`/`std`,
`time` unifies to `default`/`std`, no `rayon`); no feature or dead-code change
is proposed. No nightly-only flag, UPX or dynamic linking is a candidate.

## Proposed Solution

- **Apply `panic = "abort"` to the release profile.** The landed Windows
  executable is rebuilt and measured against a rebuild of the same revision
  with `panic = "unwind"`; the recorded cumulative saving is 83,968 B on the
  release branch, and the landed build must measure smaller.
- **Record the failure outcome.** A panic that ends the process prints the
  standard Rust panic message on stderr and aborts; a failed conversion still
  returns the typed failure and attempts a new numbered diagnostic naming the
  source and reason, as `export-duty-plan-calendar` promises. The existing
  conversion failure scenario remains the evidence; the lever build is also
  run on an unsupported source.
- **Report the published binary separately.** The baseline is a rebuild of the
  same revision with the unchanged configuration; the published binary's own
  size is reported separately and is never the baseline.
- **Measure cumulatively.** Each candidate lever is measured against the build
  carrying the previously accepted levers, so the landed difference equals the
  sum of the recorded savings. `opt-level = "s"` is recorded as rejected with
  its measured comparison.
- **Refuse before a build.** A nightly-only flag, UPX or dynamic linking is
  refused as a proposal with the violated constraint named; no measurement is
  recorded.
- The distributed executable's platform set, packaging, publication, and the
  product's normal behaviour are unchanged.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `reduce-distributed-executable-size` | `EXTEND publish-public-release` (score 17; lexical) | `NEW in reduce-distributed-executable-size` | The measurement change created the permanent measurement leaf; this change owns the reduction leaf from the same ideation contract. The capability advances the release maintainer's goal of making the distributed executable smaller; it is not the measurement or the release. The route's overlap is lexical (release, size); the placement test asks which actor goal the capability advances, and the reduction is a new leaf. |

## Out of Scope

- Publishing a public release, its archive, packaging or platform set
  (`publish-public-release`).
- A size ceiling or a refusal of a release on size (Q3 remains parked; the
  measurement report is the guard).
- `off`-list nightly-only flags, UPX and dynamic linking.
- Dependency feature selection and source-level dead-code pruning: no
  measured target.
- The WASM facade's load time and replacing the PDF extraction backend.
- The measurement command's own cadence and report shape
  (`measure-executable-size`).

## Open Questions

### D1 — Which levers are applied?

- Question: which stable lever(s) does the measurement justify?
- Options:
  1. **`panic = "abort"` only** — apply the measured lever that saves 83,968 B;
     reject `opt-level = "s"` because it measures larger by 74,240 B and by
     70,656 B against the panic build; leave features unchanged.
  2. **`opt-level = "s"`** — apply a lever that measures larger.
  3. **A feature audit** — edit features with no measured target.
- Disposition: **answered** — option 1. The dialogs: `panic = "abort"` is
  conditional on a measured saving and failure evidence; the measurement gives
  both. `opt-level = "s"` is rejected by its measured comparison. Feature
  selection is measured-empty and stays out. Rejected: option 2 and option 3.

### D2 (seed Q3) — Is a size ceiling added?

- Question: should a release be refused above a bound?
- Disposition: **out of scope** — Q3 is parked until the reduction establishes
  a floor; the measurement report is the guard. No threshold input exists.
  Owner: `givn/ideation/release-executable-size/questions.md` Q3.

### D3 (seed Q8) — Which example carries the success guarantee?

- Question: which concrete saving becomes the success example?
- Disposition: **answered** — `panic = "abort"` saves 83,968 B, from
  2,098,176 B to 2,014,208 B on the release branch. This is the first measured
  lever and the example the reduction uses.

### D4 — What diagnosis does a panic leave?

- Question: with abort, what does a user see when a panic ends the process?
- Disposition: **answered** — the standard Rust panic message on stderr, with
  no unwinding or backtrace; a `Result` conversion failure is unchanged and
  still writes/attempts a numbered diagnostic. The design records it; the lever
  build is run on an unsupported source to prove the diagnostic path.
