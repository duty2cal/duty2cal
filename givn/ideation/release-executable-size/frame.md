# Release executable size — Frame

Status: confirmed by the user on 2026-10-18 through Q1 and Q2. The
Frame-boundary review findings are advisory and the mechanical corrections are
applied below; the stakeholder-list correction is open in Q7.

## Problem

The published Windows executable is 2,097,152 B, and the project cannot say
what occupies it. The remaining stable profile levers rest on estimates: the
draft's 5–20% range is explicitly unmeasured, and the only number cited as
evidence is the compressed archive — 1,035,358 B for v0.3.0 — which is a
different quantity from the executable that decides the archive's size.

## Vision

The release owner can read a measured attribution of the code the distributed
executable contains — with the stripped artifact's bytes measured separately —
and show that every remaining stable lever was either applied with a measured
gain or rejected with a recorded reason. The next release ships a smaller
executable with every applied lever measured, and no failure outcome becomes
less visible on the way.

Example: the attribution names the top crates by size; `panic = "abort"` is
kept only if its measured delta justifies losing unwinding, the shipped path
has no `unwrap`/`expect`, and the diagnostic consequence is recorded;
`opt-level = "s"` is rejected because it measured larger than `"z"`; a
dependency feature is dropped only where the attribution points and the
comparison still holds.

## Confirmed scope

- Measure the code the distributed executable contains: a symbol-bearing
  `x86_64-pc-windows-gnu` build attributed by crate and by function, plus the
  stripped artifact's file bytes and section sizes. The attribution's totals
  are not the stripped file's size.
- Apply the stable levers the measurement justifies: `panic = "abort"` only
  when its measured delta justifies losing unwinding and the diagnostic
  consequence is recorded; `opt-level = "s"` versus `"z"` decided by
  measurement; dependency feature selection across the workspace manifests
  where the attribution points.
- Record the measured before and after for each lever as the change's evidence.
- Leave the test suite unwinding. Cargo's test profile always unwinds; no
  `[profile.test]` entry is needed, and adding one produces a warning.
- State, for every applied lever, what it costs a crew member: a panic that
  ends the process instead of unwinding must still leave the user a diagnosis —
  whether that is the panic message on stderr or a numbered report is a design
  decision, recorded before landing.
- Define the baseline as a rebuild of the released revision on `ubuntu-latest`
  with the current `stable` toolchain and the unchanged profile; the published
  binary is reported separately, because the release used the rolling stable
  toolchain of its own date.

## Out of scope

- Nightly-only levers (`build-std`, `-Zlocation-detail=none`,
  `panic=immediate-abort`): they violate the stable-toolchain constraint, and
  the last removes the panic text a failed conversion should leave behind.
- UPX and dynamic CRT linking: they contradict the AV constraint the draft
  records for the crew members' machines and the single-file constraint.
- Replacing the PDF backend: a product decision with its own ADR. The
  measurement may recommend it; this topic does not decide it.
- Source-level dead-code pruning: no attribution yet points at `core::fmt`;
  revisit after the attribution exists.
- The WASM facade's load time: a later phase with its own payload question.
- A hard size ceiling checked in CI: Q1 selected the measured-reduction
  problem, and no headroom number exists yet. Parked in Q3.
- Any change to archive contents, packaging, publication or the supported
  platform set.

## Constraints inherited

- Rust stable, edition 2021 (`rust-toolchain.toml`; `Cargo.toml:14`; chapter 2).
- One self-contained Windows x86_64 executable, statically linked, no
  companion DLL or runtime installer (chapter 2, ADR-0003).
- `pdfplumber = "=0.4.1"` pinned; no native C/C++ extraction dependency.
- The release archive holds the executable and nothing else; publishing and
  packaging are unchanged by a profile edit.
- A failed conversion attempts a new numbered diagnostic naming the source and
  the reason; when no new report can be saved, the primary failure remains
  visible and no older report is presented as current
  (`givn/specs/export-duty-plan-calendar/usecase.md:94-95,135`). Nothing in
  this work may weaken that outcome.
- Typed errors; no `unwrap()`/`expect()` outside test code — verified absent
  from the shipped crates.

## Stakeholders and interests

| Stakeholder type | Interest | Concrete example |
|---|---|---|
| Project maintainer / release owner | An executable whose size is measured and intentional; levers justified by evidence; a stable-toolchain configuration. | A profile change carries the measured delta and the attribution that justifies it. |
| Crew member converting their duty plan | A download of about 1 MiB and an executable that runs without installation. Whether 0.8 versus 1.0 MiB is perceptible is an unmeasured assumption, so this interest does not by itself justify the work. | The Windows archive downloads and the executable runs without a runtime installer. |
| Contributor reviewer (`contributor--infrastructure-reviewer`), segment: build-configuration reviewer | A configuration change justified by measurement, with its failure-behavior consequence stated. This is a segment of the confirmed type, not a new stakeholder type. | Reviewing `panic = "abort"` sees the bytes saved and what a panic now leaves the user. |
| Person identified in a duty plan | No plan-derived content enters a published attribution, a panic message or other evidence. | A panic message quoted in public evidence names no crew member, itinerary, date or identifier from a real plan. |
| Crew member building from source on an unsupported platform | The source route keeps the conversion's failure guarantee after a build-configuration change. | A source build with a changed panic behavior still writes the numbered diagnostic on a failed conversion. |

The two added types were confirmed through Q7. Their interests are covered by
the reused crew-member Persona's topic record; no new Persona was minted.

Persona reuse is inventoried in `session.md`; the three Personas were confirmed
through Q4.

## Research protocol and current evidence

1. Read the project's own state before asking: `givn status` (two unrelated
   active changes) and `givn spec tree` (four permanent use cases; none owns
   the executable's size).
2. Read the durable context that could already answer the question:
   `README.md`; chapters 1, 2, 3, 4, 7, 8, 10, 11 and 12 under `docs/arc42/`;
   ADR-0003; the permanent `publish-public-release` use case;
   `givn/commands.yaml`; `CHANGELOG.md`.
3. Read the sibling topics' session and frame files as format material, and
   inventoried `givn/personas/`.
4. Verified the draft's claims against the repository instead of repeating
   them (findings below).
5. Measured every published Windows archive through the GitHub API and by
   downloading each one; read the v0.3.0 PE section table to distinguish real
   payload from trailing padding.
6. Checked whether the proposed controls already exist in the release path.
7. Checked whether a local Windows cross-build and `cargo bloat` are possible
   on this workstation before promising one.

## Findings

### Published sizes

| Release | Windows archive (`.tar.gz`) | Executable inside |
|---|---|---|
| v0.1.0 | 1,035,414 B | 2,097,664 B |
| v0.2.0 | 1,035,664 B | 2,098,176 B |
| v0.3.0 | 1,035,358 B | 2,097,152 B |

The archive moved +250 B from v0.1.0 to v0.2.0 (a fix release) and −306 B from
v0.2.0 to v0.3.0 (the dependency updates): net −56 B. The executables moved
+512 B, then −1,024 B. v0.3.0 is the last release that published five
platforms; the current rule is one Windows archive per version, and the
published assets stay as they are (`docs/arc42/07-deployment-view.md:60`), so
the Windows asset is the only comparable baseline and any comparison holds the
platform set constant.

The v0.3.0 executable's PE section table ends exactly at its last byte
(`.reloc` ends at 0x200000), so the 2 MiB is payload and section alignment, not
appended padding. A size control must state which quantity it gates.

### Attribution method

`cargo-bloat` forces `CARGO_PROFILE_RELEASE_STRIP=false` to read symbols, while
the shipped profile sets `strip = true`. The attribution therefore describes a
symbol-bearing rebuild of the same code, not the stripped file; its totals are
not the distributed file's bytes, and file size is section-aligned. The vision
and the use cases state both numbers.

### Profile and controls as committed

- `Cargo.toml` `[profile.release]`: `opt-level = "z"`, `lto = true`,
  `strip = true`, `codegen-units = 1`; `panic` unset.
- `.cargo/config.toml:4-6`: `linker = "x86_64-w64-mingw32-gcc"`,
  `rustflags = ["-C", "target-feature=+crt-static", "-C", "link-arg=-static"]`.
  Part of the confirmed build configuration; already applied.
- `.github/workflows/release.yml` and `scripts/prepare-release.sh` contain no
  size check.
- `docs/arc42/10-quality-requirements.md` states no size scenario.
- The permanent `publish-public-release` use case makes no size promise.

### Feature graph as it stands

- `cargo tree -e features -i pdfplumber` shows only `default`/`std`; no
  `serde`, no `parallel`.
- `time` is a direct dependency of `apps/cli`, `duty2cal-core` and
  `duty2cal-pdf`; its features unify to `default`/`std`.
- `Cargo.lock` contains no `rayon`.

Feature selection is therefore measured-empty until the attribution shows
otherwise; the draft's 5–20% estimate has no enabled optional feature to drop
today.

### Test-profile claim

Cargo ignores `panic` in the `test` profile: a scratch crate carrying the
draft's `[profile.test] panic = "unwind"` emits
`warning: Cargo.toml: 'panic' setting is ignored for 'test' profile`, and
`cargo test --release` passes under a release profile with `panic = "abort"`.

### Toolchain and measurement availability

- Stable Rust 1.98.1 from a source tarball; `rustup` absent.
- `x86_64-pc-windows-gnu` target absent, `x86_64-w64-mingw32-gcc` absent,
  `cargo-bloat` absent.
- A Windows cross-build and `cargo bloat --target x86_64-pc-windows-gnu`
  cannot run on this workstation as configured; the release workflow on
  `ubuntu-latest` can. A Linux-only measurement would not describe the
  distributed artifact.

## Open questions

- Q3 — whether a measured size ceiling is added after the reduction, parked.
- Q6 — whether the attribution is produced for every change, every release, or
  on demand.
- Q7 — whether the two remaining durable stakeholder types join this Frame.
