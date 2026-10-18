# Release executable size — session state

- Topic: `release-executable-size`.
- Topic identity: selected by the agent on 2026-10-18 from the user's draft
  `private/smaller-rust.md`; no user naming decision has been taken. Rename on
  request before any Handoff.
- Phase: Handoff complete — published 2026-10-18 with the user's confirmation
  (Q9). The readable document, two ready seeds, one Persona promotion and four
  glossary terms are published. Frame confirmed (Q2 and Q7); Personas confirmed
  (Q4); terms confirmed and promoted (Q5); Q8 parks the missing success example
  until the first measurement.
- Next separate workflow: the user invokes normal proposal planning per seed,
  starting with `measure-executable-size`. This completed ideation authorizes no
  change start, no build-configuration edit and no measurement run.
- Latest user direction: work the draft's idea (a smaller Rust executable) into
  change-ready use cases through ideation.
- Completed authority: topic-local ideation state only. No change artifacts, no
  build-profile edit, no CI edit, no measurement run, no Handoff.

## Research completed

- `givn status`: two active changes, both unrelated to build configuration —
  `include-report-time-in-travel-title` (ready to archive) and
  `publish-organization-profile` (in progress). No change owns the release
  profile or the executable's size.
- `givn spec tree`: four permanent use cases — `adopt-existing-configuration`,
  `apply-configuration-change`, `export-duty-plan-calendar`,
  `publish-public-release`. The release use case owns what a published archive
  guarantees; it states no size promise.
- Read `README.md`, `docs/arc42/01`, `02`, `04`, `07`, `10`, `12`,
  `docs/arc42/adr/ADR-0003`, the permanent `publish-public-release` use case,
  and `givn/commands.yaml`.
- Read both sibling topics' `session.md` and the conversion topic's `frame.md`
  as format material.
- Inventoried `givn/personas/`: four confirmed project Personas.
- Read the user's draft `private/smaller-rust.md` without editing it.
- Measured every published Windows archive through the GitHub API and by
  downloading each one. Executables: 2,097,664 B (v0.1.0), 2,098,176 B
  (v0.2.0), 2,097,152 B (v0.3.0); archives: 1,035,414 B, 1,035,664 B,
  1,035,358 B. `cargo-bloat` needs `strip = false` to read symbols, so a size
  attribution describes a symbol-bearing rebuild, not the stripped file.
- Checked the repository's release profile, release workflow, packaging script,
  and panic handling. No size assertion exists anywhere.
- Verified the test-profile claim empirically in a scratch crate (see
  `frame.md`, findings).
- Checked local build capability: no `rustup`, no installed
  `x86_64-pc-windows-gnu` target, no `x86_64-w64-mingw32-gcc`, no
  `cargo-bloat`. A Windows cross-build cannot run on this workstation as
  configured.

## Persona reuse inventory

Inventoried against the four confirmed project Personas on 2026-10-18.

| Confirmed stakeholder type | Project Persona matches | Disposition |
|---|---|---|
| Project maintainer / release owner | `project-maintainer--release-owner` | Reused. Same role, authority (accepts a release) and evaluation (release evidence, distribution checks); the topic file adds record entries only. |
| Crew member converting their duty plan | `crew-member--personal-calendar-owner` | Reused. Same role and authority; the topic interest is the download and the failure diagnosis, not a new segment. |
| Contributor reviewing a build-configuration change | `contributor--infrastructure-reviewer` shares the review-only authority but not the vocabulary | New segment `contributor--build-configuration-reviewer`. Distinguishing criterion: own vocabulary (measured delta, attribution, panic behavior) differs; the review-and-approve authority is the same, and the stable core says so. |

Three segments, below the cap of three per stakeholder type and ten overall.
All three are confirmed (Q4).

## Next questions

- Q6 — measurement cadence (open). Q3 remains parked.
