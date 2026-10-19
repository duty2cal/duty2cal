# Design: reduce-executable-size

## Scope

The HOW for [the saving](specs/reduce-distributed-executable-size/reduce-distributed-executable-size.feature):
apply `panic = "abort"` to the release profile after measuring it against a
rebuilt unchanged baseline, record the failure outcome, and record
`opt-level = "s"` as rejected because it measures larger. The distributed
executable changes size; its platform, packaging and publication do not.

## Measured evidence

The release-branch baseline and candidates, built for
`x86_64-pc-windows-gnu` with the committed configuration:

| Build | File size | Saved | Decision |
|---|---:|---:|---|
| baseline, unchanged configuration | 2,098,176 B | — | — |
| `panic = "abort"` | 2,014,208 B | 83,968 B (4.00%) | apply |
| `opt-level = "s"` | 2,172,416 B | larger by 74,240 B | reject |
| `panic = "abort"` + `opt-level = "s"` | 2,084,864 B | larger by 70,656 B against the panic build | reject the second lever |

The feature audit is measured-empty (`pdfplumber` enables only `default`/`std`,
`time` unifies to `default`/`std`, no `rayon`), so no feature or dead-code
change is proposed.

## Technology decisions

| Area | Decision | Rationale / source |
|---|---|---|
| Lever | `panic = "abort"` in `[profile.release]`. | The one stable lever with a measured target; the ideation seed accepts it only with a measured saving and failure evidence. |
| Measurement interface | `python3 scripts/measure_size_lever.py --revision <rev> --lever <name> [--repo <dir>] [--out <dir>] [--published-size <bytes>]`, used by the project's automation and a maintainer. | One command builds the rebuilt baseline and the cumulative lever builds, and records every decision. |
| Language | Python 3, reusing `scripts/measure_executable_size.py`'s worktree/build helpers. | The measurement tooling already uses Python 3; no new dependency. |
| Baseline | A rebuild of the same revision with `panic = "unwind"` and `opt-level = "z"` (the unchanged configuration), not the published binary. | The published binary comes from a different toolchain date; the rebuilt baseline isolates the lever. |
| Published size | Reported separately when `--published-size` is supplied. | The use case requires the two quantities to stay distinct. |
| Report | `size-lever.json` and `size-lever.md`, with baseline, cumulative rows, status and the failure outcome. | The tests and a reviewer read one dataset; the report names every lever decision. |
| Failure outcome | A panic prints the standard Rust panic message on stderr and aborts; a typed conversion failure still attempts a numbered diagnostic. | The ideation seed requires the diagnosis recorded before landing; the host lever build proves the `Result` path. |
| Test runner | Unchanged: `./run-tests.sh`, `./run-tests.sh --e2e`, `./run-tests.sh --name "<title>"`. | The project's executable-spec runner. |

## Decisions

- **D1 — apply `panic = "abort"`.** The measured lever saves 83,968 B
  (2,098,176 → 2,014,208) and the failure outcome is recorded. Rejected:
  `opt-level = "s"` (larger by 74,240 B, and by 70,656 B against the panic
  build) and a feature audit (no measured target). Realized in `Cargo.toml`
  and the reduction scenario.
- **D2 (seed Q3) — no size ceiling.** Out of scope; the parked question in
  `givn/ideation/release-executable-size/questions.md` (Q3) owns it. The
  measurement report is the guard. Realized negatively: no threshold input
  and no refusal on size.
- **D3 (seed Q8) — the success example is the first measured lever.** The
  recorded saving, 83,968 B, is the example the use case carries. Realized in
  the use case's Examples and the e2e report.
- **D4 — the panic diagnosis.** A panic prints the standard Rust panic
  message on stderr and aborts; a typed conversion failure still attempts a
  numbered diagnostic naming the source and reason. The host lever build is
  run on an unsupported source; the Windows executable is started by the
  Windows smoke job. Rejected: treating panics as unreachable.
- **Build failure and review controls.** A failed baseline/lever build records
  `failed` with no saving; a review rejection is a human control owned by the
  contributor reviewer and recorded in the change/release review, not a second
  command. The zero-saving case is rejected by the same comparison as the
  larger lever; no no-op lever exists to exercise it. The landed build is
  measured from the worktree's committed profile and must be smaller; an
  accepted lever whose landed build is not smaller is `not_landed`, never
  `applied`.

## The size-lever command

```
scripts/measure_size_lever.py \
  --revision <rev>            # default: HEAD; resolved with git rev-parse
  --lever <name>              # required; comma-separated, cumulative
  [--repo <dir>]              # default: current directory
  [--out <dir>]               # default: size-lever/
  [--published-size <bytes>]  # reported separately
```

Order of operations:

1. **Refuse before a build.** `nightly`, `upx` and `dynamic-linking` are not
   size levers: the command names the violated constraint, writes no report and
   exits non-zero. An unknown lever is refused the same way.
2. **Resolve and create a temporary worktree** at the revision. The profile is
   read from the worktree's `Cargo.toml`.
3. **Build the unchanged baseline** with `CARGO_PROFILE_RELEASE_PANIC=unwind`
   and `CARGO_PROFILE_RELEASE_OPT_LEVEL=z`, in its own target directory.
4. **Build each lever cumulatively.** For each lever, the baseline is the
   previously accepted build with the lever's key at its old value; the lever
   build sets the new value. A build failure records the failure and no saving.
5. **Measure the stripped executable's file size.** `saved_bytes = baseline -
   lever`; a positive saving is `applied` and becomes the new current build; a
   zero or negative saving is `rejected` and the current build stays.
6. **Build the landed configuration** — the worktree's committed `Cargo.toml`
   with no profile override — and record `landed.bytes` and `landed.status`
   (`smaller` or `not_smaller`). The report is `applied` only when a lever was
   applied and the landed build is smaller; `not_landed` when a lever was
   applied but the landed build is not smaller; `rejected` when no lever was
   applied.
7. **Preserve the landed executable** in `--out/duty2cal-lever.exe` for the
   failure-path test, then remove the temporary worktree.
8. **Write the report** and exit 0 for an applied, rejected or not-landed
   report; non-zero for a refusal or a build failure.

### Report schema

```json
{
  "kind": "size-lever",
  "status": "applied",
  "requested": "HEAD",
  "revision": "<40-hex sha>",
  "platform": "x86_64-pc-windows-gnu",
  "levers": ["panic=abort"],
  "published_binary_bytes": 2097152,
  "baseline_bytes": 2098176,
  "baseline_sha256": "<sha256>",
  "builds": [
    { "lever": "panic=abort", "baseline_bytes": 2098176, "lever_bytes": 2014208,
      "saved_bytes": 83968, "percent": 4.0, "status": "applied" }
  ],
  "landed": { "bytes": 2014208, "sha256": "<sha256>", "status": "smaller" },
  "lever_executable": "<out>/duty2cal-lever.exe",
  "failure_outcome": {
    "panic": "standard Rust panic message on stderr; process aborts",
    "conversion": "typed failure still attempts a numbered diagnostic"
  },
  "reason": null
}
```

`status` is `applied` when at least one lever was applied, `rejected` when
none was, and `failed` for a build failure or refusal. The Markdown labels are
the assertion surface:

- `- Requested:`, `- Revision:`, `- Platform:`, `- Levers:`, `- Status:`
- `- Published binary (file): <N> B (reported separately)` or `not supplied`
- `## Baseline` / `- Baseline (rebuild, unchanged): <N> B`
- `## Cumulative` with `| lever | baseline bytes | lever bytes | saved bytes | percent | status |`
- `## Landed` with `- Landed (committed configuration): <N> B` and `- Landed status: smaller|not_smaller`
- `## Failure outcome` with the panic and conversion statements
- `- Lever executable: <path>` when an applied build was preserved

## Architecture impact

No product component changes. The release profile gains one setting; one
build-tooling command, one permanent use case, the step definitions, and the
release-note allow-list entry are added.

```mermaid
flowchart LR
  REV["Named revision"] --> WT["temporary git worktree"]
  WT --> BASE["baseline build<br/>panic = unwind, opt-level = z"]
  BASE --> LEV["lever build<br/>panic = abort"]
  LEV --> MEAS["stripped file size<br/>saving and status"]
  MEAS --> REPORT["size-lever.json + size-lever.md"]
  REPORT --> PRODUCT["Cargo.toml: panic = abort"]
  PRODUCT --> RELEASE["release build on the Windows job"]
```

Changed files:

| Path | Kind | Purpose |
|---|---|---|
| `Cargo.toml` | changed | `[profile.release]` gains `panic = "abort"` with the measured reason. |
| `scripts/measure_size_lever.py` | new | Rebases the unchanged baseline, builds cumulative levers, measures, records and preserves the accepted executable. |
| `apps/cli/tests/steps/reduce_distributed_executable_size.rs` | new | Regular steps for the lever report, rejection, cumulative and refusal scenarios, plus the host failure-path proof. |
| `apps/cli/tests/steps/e2e/reduce_distributed_executable_size.rs` | new | The `@e2e` steps and the committed transcript. |
| `apps/cli/tests/features_runner.rs`, `steps/mod.rs`, `steps/e2e/mod.rs` | changed | The lever world fields and module lines. |
| `cliff.toml` | changed | The capability allow-list and the matching parser rows, so the reduction is released. |
| `givn/specs/reduce-distributed-executable-size/usecase.md` | new | The permanent use case written by the specs artifact. |
| `docs/arc42/*` | changed by the arc42-docs artifact | The affected chapters record the profile lever and its consequence. |

## Test runner, strict mode, single-scenario command

- Regular suite: `./run-tests.sh`; e2e: `./run-tests.sh --e2e`; single
  scenario: `./run-tests.sh --name "<title>"`.
- Strict mode: `features_runner.rs` uses `.fail_on_skipped()` and
  `.run_and_exit(root)`; undefined or skipped steps fail.
- The lever scenarios build the product; the `test` and `coverage` jobs
  already install the Windows target and `mingw-w64` for the measurement
  change.
- The `@e2e` scenario's transcript is committed at
  `givn/changes/reduce-executable-size/evidence/visual/maintainer-measures-the-panic-abort-lever-and-the-landed-executable-is-smaller/transcript.txt`
  and at the archive path after archive, with `<scenario-dir>` and `<project>`
  substitutions. The transcript holds the command line, the report, the
  diagnostic run and both exit statuses; the build log is not recorded.
- During RED every new step body panics with its own name. The setup task adds
  one deliberately undefined step, records the non-zero exit, removes it and
  re-runs the scenario; the strict-mode proof is recorded in `tasks.md`.
- Not a Web UI: the command writes text and files; it renders no HTML/CSS/JS
  in a browser, so no browser driver applies.
- The released `feat(reduce-distributed-executable-size)` revision includes
  `Cargo.toml` (a user-receiving path) so the reduction is not filtered from
  the release notes; a commit touching only the scripts and tests would be.

## Step definitions and fixtures

- `apps/cli/tests/steps/reduce_distributed_executable_size.rs` drives
  `scripts/measure_size_lever.py` with `std::process::Command`, parses the
  Markdown report, and builds/runs a **host** release executable with the
  landed profile on an unsupported source to prove the failure diagnostic. The
  Windows executable is measured, and the release workflow's Windows smoke job
  starts it; the local host build proves the platform-independent `Result`
  path.
- `apps/cli/tests/steps/e2e/reduce_distributed_executable_size.rs` runs the
  same command, commits the transcript and appends the report.
- The scenarios use the real project revision (`HEAD`), not a fixture
  workspace; the product build and the report are the observable boundary.

## Use-case traceability

| Use-case element | Technical decision | Evidence |
|---|---|---|
| Goal: a smaller executable by a measured lever | `panic = "abort"` saves 83,968 B | e2e; "A lever that measures larger is rejected" |
| Main flow 1: select one justified lever; state its constraint | The command refuses forbidden levers before a build | "A nightly-only lever is refused before a build"; the outline |
| Main flow 2: rebuilt unchanged baseline; published size separate | `CARGO_PROFILE_RELEASE_PANIC=unwind` baseline; `--published-size` separate | "The published binary's size is reported separately" |
| Main flow 3: lever build and cumulative saving | Sequential builds; per-lever row | e2e; "A second lever is measured against the configuration carrying the first" |
| Main flow 4: failure outcome | Standard panic message; numbered conversion diagnostic | e2e's unsupported-source step |
| Main flow 6: accept/reject and record | Report status applied/rejected with the comparison | "A lever that measures larger is rejected" |
| Main flow 7: landed rebuild smaller | The accepted build's size and the `Cargo.toml` lever | e2e |
| Rule: forbidden lever refused before a build | `nightly`/`upx`/`dynamic-linking` checked first | "A nightly-only lever is refused before a build"; outline |
| Rule: cumulative savings | The script's sequential baseline/lever builds | "A second lever is measured against the configuration carrying the first" |
| Rule: published size separate | `published_binary_bytes`, never the saving baseline | "The published binary's size is reported separately" |

## Persona impact

User-visible surface: the release executable's size and failure diagnosis, and
the terminal report.

- `project-maintainer--release-owner` — reads the applied saving and the
  failure consequence before the release.
- `contributor--build-configuration-reviewer` — sees the cumulative
  comparison, the rejected lever and the stated panic consequence.
- `crew-member--personal-calendar-owner` — receives a smaller download; a
  failed conversion still writes the numbered diagnostic naming the source
  and reason.

## Visual Design Contract

- Interface: terminal
- Viewports: n/a — no rendered screen
- Design system: none (the terminal output is plain text)
- Screens covered: none; this change introduces no screen
- Evidence kind: a transcript per `@e2e` scenario, committed at
  `givn/changes/reduce-executable-size/evidence/visual/maintainer-measures-the-panic-abort-lever-and-the-landed-executable-is-smaller/transcript.txt`
  (and under `givn/archive/reduce-executable-size/` after archive). It holds
  the command line, `size-lever.md`, the host failure-path run and the exit
  status, with `<scenario-dir>` and `<project>` substitution; no host path,
  user name or clock value survives.
- Motion: none; `prefers-reduced-motion` is not applicable to a terminal
  report.

## E2E smoke test infrastructure

- E2E runner: `./run-tests.sh --e2e` (`verify.e2e_command`).
- E2E step location:
  `apps/cli/tests/steps/e2e/reduce_distributed_executable_size.rs`.
- Real interface: CLI (terminal). The scenario runs the real
  `python3 scripts/measure_size_lever.py` against the project revision, reads
  the report, then builds and runs the host lever executable on an unsupported
  source. The Windows executable is measured and is started by the release
  workflow's Windows smoke job.
- Local test infrastructure: the project toolchain, the Windows target,
  `mingw-w64`, `cargo` and `python3`; no server, port, database, container,
  network or credential.
- Framework: cucumber-rs over `DutyWorld`; strict mode is
  `.fail_on_skipped()`.
- Single-scenario run: `./run-tests.sh --name "<title>"`.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| measure one size lever and read the landed saving | Maintainer measures the panic-abort lever and the landed executable is smaller | CLI (terminal) | `std::process::Command` runs `python3 scripts/measure_size_lever.py --repo <project> --revision HEAD --lever panic=abort --out <dir>`, reads `size-lever.md` and the exit status |

## Local runnability & digital twins

- One command: on a Linux host with the Windows target and `mingw-w64`, the
  whole suite is `./run-tests.sh`; the e2e scenario alone is
  `./run-tests.sh --name "<title>"`.
- This workstation's Nix mingw lacks `libpthread.a`; the lever and measurement
  builds use a local empty-archive directory exposed through
  `CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS=-L <dir>` for the Nix-only run.
  The `.cargo/config.toml` and the repository carry no shim; CI's Ubuntu
  `mingw-w64` carries the library.
- The lever builds are CPU-only and isolated; no network or service is used.
- Digital twins: none.

## Coverage process boundaries

Unchanged; the change adds no production Rust process. The new step
definitions are part of `features_runner` and are instrumented with it; the
Python command and the product builds are build tooling outside the Rust
workspace, like the measurement command.

## Black-Box-First policy

Every scenario drives the real command and the real product build; no internal
unit test is added. The refusal scenarios are observed through the command's
exit status and message; the applied/rejected scenarios through the report;
the failure path through the host lever executable's numbered diagnostic.

## Risks and follow-ups

- **Abort loses unwinding.** A panic ends the process with the standard Rust
  panic message and no backtrace. The product uses no `unwrap`/`expect` outside
  tests, and a `Result` conversion failure still writes the numbered
  diagnostic; the release note states the consequence.
- **The host and Windows builds prove different things.** The size is measured
  on the Windows executable; the failure path is run on the host lever build.
  The release workflow's Windows smoke job starts the Windows executable; a
  Windows-side diagnostic run remains a host-runner boundary.
- **CI cost.** Each lever scenario builds the product; the `test` and
  `coverage` jobs run the reduction scenarios. The saving is measured once per
  revision, not per release.
- **The cumulative order is fixed.** The change applies one lever; a later
  lever must be measured against the configuration carrying `panic = "abort"`.
- **`opt-level = "s"` is rejected by measurement.** It is recorded here and
  is not landed; a later toolchain may change the comparison and would be a new
  decision.
