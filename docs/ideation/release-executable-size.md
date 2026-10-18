# Release executable size

Status: published by the user's confirmed Handoff on 2026-10-18. Two change
seeds are ready; no change has been started.

## The problem

The published Windows executable is 2,097,152 bytes. Nobody could say what
occupies it. The remaining stable build-profile levers were being chosen from
estimates, and the only number cited as evidence was the compressed archive —
1,035,358 bytes for v0.3.0 — a different quantity from the executable that
decides it.

The question was not "is one megabyte too much"; it was "what are these two
megabytes, and which of the remaining stable levers actually pays?".

## What the research measured

Every published Windows archive was downloaded and opened.

| Release | Archive (`.tar.gz`) | Executable inside |
|---|---|---|
| v0.1.0 | 1,035,414 B | 2,097,664 B |
| v0.2.0 | 1,035,664 B | 2,098,176 B |
| v0.3.0 | 1,035,358 B | 2,097,152 B |

The archive moved +250 B from v0.1.0 to v0.2.0 (a fix release) and −306 B from
v0.2.0 to v0.3.0 (the dependency updates): net −56 B. The executables moved
+512 B, then −1,024 B. Growth is not the observed problem.

The v0.3.0 executable's PE section table ends exactly at its last byte, so the
two megabytes are payload and section alignment, not appended padding.

The committed build configuration already carries `opt-level = "z"`,
`lto = true`, `strip = true` and `codegen-units = 1`; `panic` is unset. The
stable toolchain, the single self-contained Windows executable and the static
link are constraints; nightly-only levers, UPX and dynamic CRT linking are out.

Two findings changed the draft's method:

- **The attribution cannot be of the stripped file.** `cargo-bloat` disables
  `strip` to read symbols. A size attribution therefore describes a rebuild of
  the same code with names retained; the stripped artifact's file size is a
  separate number.
- **Feature selection is currently measured-empty.** `pdfplumber` enables only
  `default`/`std`, `time` unifies to `default`/`std` across all three crates,
  and no `rayon` exists in the lock file. The draft's 5–20% feature-audit
  estimate has no enabled optional feature to drop today.

The draft's proposed `[profile.test] panic = "unwind"` is a no-op: Cargo warns
that `panic` is ignored for the test profile, and `cargo test --release` passes
under `panic = "abort"`.

## Who cares

- The **release maintainer** decides and answers for the executable the project
  ships.
- The **crew member** downloads it; the difference between 0.8 and 1.0 MiB is
  not a perceptible interest, so size does not outrank conversion fidelity.
- The **contributor reviewing a build-configuration change** judges whether a
  delta is measured and its consequence stated.
- The **person identified in a duty plan** must not see plan-derived content in
  a published attribution or panic sample.
- The **crew member building from source** on an unsupported platform inherits
  the changed build configuration and must keep the conversion's failure
  diagnosis.

## The two use cases

| Use case | Level | Goal | Mutation |
|---|---|---|---|
| Measure the code's size for the distributed executable (`measure-distributed-executable-size`) | ! | Obtain a size attribution of the code the distributed executable is built from, for a named revision and the supported platform. | Observing |
| Reduce the distributed executable's size (`reduce-distributed-executable-size`) | ! | Make the executable built for the supported platform smaller by applying only the size levers a measurement justifies. | Changing |

The measurement stands alone; the reduction consumes its attribution. The
reduction's outcome is one measured difference, not a pile of profile edits.

## Decisions

- **Measure the distributed target, not the host.** A Linux host build is not
  evidence about the Windows executable.
- **Two numbers, never substituted.** The attribution's total and the stripped
  artifact's file size are reported side by side.
- **Cumulative deltas.** Each lever is measured against a build carrying the
  previously accepted levers, so the landed difference equals their sum.
- **`panic = "abort"` is conditional.** It is accepted only with a measured
  delta, a shipped path free of `unwrap`/`expect`, and a recorded diagnosis
  consequence — the panic message or a numbered report.
- **The baseline is a rebuild, not the published binary.** The released
  revision is rebuilt with the current stable toolchain and the unchanged
  configuration; the published binary is reported separately.
- **No ceiling yet.** A hard size gate was considered and parked: growth is not
  observed, and no headroom number has been measured.

## Out of scope

Nightly-only levers (`build-std`, `-Zlocation-detail=none`,
`panic=immediate-abort`); UPX and dynamic CRT linking; replacing the PDF
extraction backend; source-level dead-code pruning until an attribution points
at `core::fmt`; the WASM facade's payload; and any change to the archive,
packaging, publication or the supported platform set.

## Change seeds

| Seed | Guarantee | Depends on |
|---|---|---|
| `measure-executable-size` | A named revision's attribution: split by crate and function, with the stripped artifact's size beside it; incomplete when that size is unreadable. | none |
| `reduce-executable-size` | The landed rebuild is smaller than the same revision's unchanged-configuration rebuild, by the recorded cumulative deltas; every lever decision is recorded; panic and conversion diagnoses survive. | `measure-executable-size` |

Start with `measure-executable-size`: the reduction's decisions consult the
attribution it produces.

## Open questions

- **Measurement cadence** — every change, every release, or on request. A
  per-change attribution informs review before landing; a release-only one
  arrives after the lever has landed. Recorded as Q6 in the topic state.
- **Size ceiling** — whether a gate guards the executable or the archive after
  the reduction. Parked as Q3.
- **The reduction's success example** — the first measured lever's delta does
  not exist yet, so no numeric example is invented. Parked as Q8.

## Source records

- `givn/ideation/release-executable-size/frame.md`
- `givn/ideation/release-executable-size/eventstorming.md`
- `givn/ideation/release-executable-size/use-case-definitions.md`
- `givn/ideation/release-executable-size/use-cases/`
- `givn/ideation/release-executable-size/domain-terms.md`
- `givn/ideation/release-executable-size/questions.md`
- The user's draft, `private/smaller-rust.md`, remains unedited and outside
  version control.
