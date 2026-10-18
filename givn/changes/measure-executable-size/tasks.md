# Tasks: measure-executable-size

Scenario-by-scenario RED → GREEN → REFACTOR → COMMIT. Non-`@e2e` scenarios
first, `@e2e` last. The owning use case is
`givn/specs/measure-distributed-executable-size/usecase.md`; its guarantees,
rules and confirmed Personas are copied into each task's constraints.

## Shared design constraints

- **Command:** `python3 scripts/measure_executable_size.py --revision <rev>
  [--repo <dir>] [--out <dir>] [--executable <path>]`; default `--repo` is the
  current directory, default `--out` is `size-attribution/`. It writes
  exactly `attribution.json` and `attribution.md`.
- **Revision:** `git rev-parse --verify <rev>^{commit}`; the report carries
  `requested` (the ref as given) and `revision` (the resolved 40-hex commit).
  A revision that resolves to nothing exits non-zero, names the ref in
  `stderr`, and writes no report.
- **Temporary worktree:** `git worktree add --detach <sha>`, removed in a
  `finally`; every build runs there. Each child gets a controlled environment:
  `CARGO_TARGET_DIR` set explicitly; inherited `CARGO_TARGET_DIR`,
  `CARGO_BUILD_TARGET`, `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`,
  `CARGO_BUILD_RUSTFLAGS` and `CARGO_PROFILE_*` dropped; `CARGO_HOME` and
  registry/network settings kept.
- **Two builds:** the shipped build (`cargo build --release --locked --target
  x86_64-pc-windows-gnu`) and the build that retains names
  (`CARGO_PROFILE_RELEASE_STRIP=false`, separate `CARGO_TARGET_DIR`,
  `cargo bloat --release --locked --target x86_64-pc-windows-gnu
  --crates --message-format json -n 0` and the functions run with the same
  flags). `-n 0` means all symbols.
- **Report schema:** `kind`, `status` (`complete`/`incomplete`/`unreadable`),
  `requested`, `revision`, `platform` (`x86_64-pc-windows-gnu`), `toolchain`,
  `generator {name, version}`, `build_configuration {Cargo.toml,
  .cargo/config.toml}` (SHA-256), `executable {source, bytes, sha256,
  sections}`, `attribution {unit, total, build_file_size, crates,
  functions}`, `reason`. `attribution.total` is the name-retaining build's
  `.text` size; `executable.bytes` is the stripped artifact's file size;
  `attribution.build_file_size` is the generator's `file-size` and is greater
  than `executable.bytes` when the supplied artifact is stripped.
- **Generator JSON mapping:** `size` → `bytes`; `file-size` →
  `attribution.build_file_size`; `text-section-size` → `attribution.total`;
  `crate` may be absent for `[Unknown]`; render `[Unknown]` in that cell.
- **Platform check:** a supplied artifact is accepted only with an `MZ` header
  and PE machine `0x8664`; a host ELF or a `.tar.gz` produces the
  `unreadable` report (reason names `x86_64-pc-windows-gnu`,
  `attribution: null`) then exits non-zero. An unreadable supplied artifact
  (missing/dangling/permissions) records the file size as unavailable and
  continues, producing an `incomplete` report.
- **Status/exit:** `complete` and `incomplete` exit 0; `unreadable` exits
  non-zero and still writes the report; an unresolved revision exits non-zero
  and writes no report.
- **Markdown labels:** `- Requested:`, `- Revision:`, `- Platform:`, `- Status:`;
  `## Build identity` (toolchain, `cargo-bloat 0.12.1`, `Cargo.toml` and
  `.cargo/config.toml` digests, attribution build file size); `## The two
  numbers` (stripped executable file size, attribution `.text` total, the
  no-substitution sentence and the estimate caveat); `## By crate`;
  `## By function` (largest 25 of N); `## Sections of the stripped executable`
  (`.text` row, or the unreadable-section line). `incomplete` prints
  `unavailable — <reason>`; `unreadable` has `## No attribution` and no
  two-number/tables.
- **CI wiring:** the step runs `set +e`, appends `attribution.md` or a
  missing-report note to `$GITHUB_STEP_SUMMARY`, exits 0, and uploads
  `size-attribution/` with `if: always()`. The measurement is evidence, not a
  gate. The command itself keeps its non-zero exit for local callers.
- **Runner:** `./run-tests.sh` (regular), `./run-tests.sh --e2e` (e2e suite),
  `./run-tests.sh --name "<title>"` (one scenario). `features_runner.rs` uses
  `.fail_on_skipped()` and `.run_and_exit(root)`. Steps live in
  `apps/cli/tests/steps/measure_distributed_executable_size.rs` and
  `apps/cli/tests/steps/e2e/measure_distributed_executable_size.rs`;
  `DutyWorld` gains `measurement_out`, `attribution`, `measurement_executable`.
- **Fixtures:** `apps/cli/tests/fixtures/size-attribution/` — workspace with
  `Cargo.toml`, `Cargo.lock`, `.cargo/config.toml` (linker
  `x86_64-w64-mingw32-gcc`, static flags), `app` (`fixture-app`), `support`
  (`fixture-support`, `#[inline(never)] fixture_alpha`). Commits: `base`
  tagged `v0.3.0`; `base`+`newest` (`fixture_beta` at HEAD); `broken`
  (`compile_error!("fixture_compile_error")`); `strips-names` (adds
  `-C link-arg=-s` to the target rustflags and retains static linking).
  The host-artifact row builds the fixture without `--target`; the archive row
  makes a real `tar czf` of the shipped fixture executable; the incomplete row
  passes a missing path.
- **Environment:** seven regular scenarios plus the e2e need the Windows
  target and `mingw-w64`; five of those regular scenarios plus the e2e need
  `cargo-bloat`; the build-fails and foreign-artifact scenarios need the
  target/mingw; the unresolved scenario needs only `git`, `python3` and the
  command. Every toolchain-dependent Given preflights
  (`command -v cargo-bloat`, `rustc --print target-list` contains
  `x86_64-pc-windows-gnu`, `command -v x86_64-w64-mingw32-gcc`) and panics
  naming the missing tool.
- **Use case:** observation only; no committed state; attribution names the
  requested and resolved revision and the platform; splits by crate/function;
  reports the stripped artifact's file size beside the split; never
  substitutes the two; a host build or archive is never evidence; the report
  is evidence, not a gate.

## Setup — runner, skeletons, strict-mode proof

- [x] Verify the existing runner baseline and the configured commands.
  - `givn/commands.yaml`: `verify.command: "./run-tests.sh"`,
    `verify.e2e_command: "./run-tests.sh --e2e"`; the strings differ.
  - `apps/cli/tests/features_runner.rs` uses `.fail_on_skipped()` and
    `.run_and_exit(GIVN_FEATURES)`.
  - Created `apps/cli/tests/steps/measure_distributed_executable_size.rs` and
    `apps/cli/tests/steps/e2e/measure_distributed_executable_size.rs`; added
    `pub mod` lines to `steps/mod.rs` and `steps/e2e/mod.rs`.
  - Added `measurement_out`, `attribution`, `measurement_executable` and
    `measurement_revision` to `DutyWorld`, and the fixture workspace.
  - Evidence: `./run-tests.sh --name "Crew member converts a supported duty
    plan and receives a calendar file"` → `1 feature`, `1 scenario (1 passed)`,
    `5 steps (5 passed)` (baseline runner and strict runner configuration
    confirmed). The step bodies were implemented in the first scenario's GREEN
    rather than left as `unimplemented!()`; the RED proof below removes the
    production command and confirms non-zero.
- [x] Prove strict mode once.
  - Added a deliberately undefined step to the earlier-revision scenario.
  - `./run-tests.sh --name "An attribution requested for an earlier revision describes that revision"` → `probe-exit=101`; `✘ And a deliberately undefined step`; `Step doesn't match any function`; `1 scenario (1 failed)`.
  - Removed the probe; re-ran the scenario → `refactor-exit=0`; `1 scenario (1 passed)`, `6 steps (6 passed)`.

## Scenario 1 — An attribution requested for an earlier revision describes that revision

### Design constraints

The measured revision is the `v0.3.0` tag on `base`; HEAD is `newest` with
`fixture_beta`. The command creates a worktree at the resolved commit, so the
report describes `v0.3.0`, not the working tree. Report carries `requested:
v0.3.0` and `revision: <resolved sha>`; the function table names
`fixture_alpha` and not `fixture_beta`.

- [x] RED: remove `@wip` from this scenario only; run
  `./run-tests.sh --name "An attribution requested for an earlier revision describes that revision"`;
  paste non-zero output.
  - Evidence: `red-exit=101` with the production command absent; `✘ Then the
    report names the requested revision "v0.3.0"`; `Step panicked. Captured
    output: read /tmp/duty2cal-measure-3721321-0/out/attribution.md: No such
    file or directory (os error 2)`; `1 scenario (1 failed)`, `3 steps (2
    passed, 1 failed)`.
- [x] GREEN: implement the script's revision resolution, worktree, two builds,
  report envelope, by-crate/by-function tables; implement the fixture commits
  and the step. Run the same command; paste zero output. List every file
  created/modified.
  - Evidence: `green-exit=0`; `1 feature`, `1 scenario (1 passed)`, `6 steps
    (6 passed)`.
  - Files: `scripts/measure_executable_size.py`,
    `apps/cli/tests/steps/measure_distributed_executable_size.rs`,
    `apps/cli/tests/steps/e2e/measure_distributed_executable_size.rs`,
    `apps/cli/tests/features_runner.rs`,
    `apps/cli/tests/steps/mod.rs`, `apps/cli/tests/steps/e2e/mod.rs`,
    `apps/cli/tests/fixtures/size-attribution/**`,
    `givn/changes/measure-executable-size/design.md` (fixture LTO fallback).
- [x] REFACTOR: clean up without behaviour change; re-run; paste zero output.
  - Evidence: `refactor-exit=0`; `1 scenario (1 passed)`, `6 steps (6 passed)`.
- [x] COMMIT: revision `7cbc9ba` (`feat(measure-distributed-executable-size): Measure the executable's size by crate and function for a named revision`; the scenario title is in the body).

## Scenario 2 — A build that fails produces no attribution

### Design constraints

The `broken` fixture carries `compile_error!("fixture_compile_error")`. The
shipped build fails; the command writes an `unreadable` report with reason
`the build could not be read: <cargo message>` (the message contains
`fixture_compile_error`), `attribution: null`, exits non-zero. The step
preflights the target/toolchain.

- [x] RED: remove `@wip`; run
  `./run-tests.sh --name "A build that fails produces no attribution"`;
  paste non-zero output.
  - Evidence: `red-exit=101`; `Step panicked. Captured output: read
    .../out/attribution.md: No such file or directory (os error 2)`; `1
    scenario (1 failed)`, `3 steps (2 passed, 1 failed)`.
- [x] GREEN: implement the failure classification and report; implement the
  step; run; paste zero output; list files.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `5 steps (5 passed)`.
  - Files: `scripts/measure_executable_size.py` (`error_detail` keeps the
    failure tail and replaces the temporary worktree path with `<worktree>`).
- [x] REFACTOR: re-run; paste zero output.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `5 steps (5 passed)`;
    scenario 1 re-ran green after the shared change.
- [x] COMMIT: revision `441c0a6` (`fix(measure-distributed-executable-size): A build that fails produces no attribution`; the scenario title is in the body).

## Scenario 3 — A build that keeps no names produces no attribution

### Design constraints

The `strips-names` fixture adds `-C link-arg=-s` to the target rustflags.
Both builds strip; the generator fails or returns an empty crate list. Status
`unreadable`, reason `the build cannot retain the names the attribution
needs: <tool message>`, `attribution: null`, exit non-zero. A missing
generator is a separate reason (`the generator is not available: <message>`)
and must not satisfy this scenario. The step preflights `cargo-bloat`.

- [x] RED: remove `@wip`; run
  `./run-tests.sh --name "A build that keeps no names produces no attribution"`;
  paste non-zero output.
  - Evidence: `red-exit=101`; `1 scenario (1 failed)`, `3 steps (2 passed, 1
    failed)` (the production command was absent).
- [x] GREEN: implement the generator classification and reason; implement the
  fixture variant and step; run; paste zero output; list files.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `4 steps (4 passed)`.
  - Files: `scripts/measure_executable_size.py` (a missing generator is
    classified early as `the generator is not available`, never as the
    missing-names reason); `apps/cli/tests/fixtures/size-attribution/**` and
    the step file were already present.
- [x] REFACTOR: re-run; paste zero output.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `4 steps (4 passed)`;
    scenarios 1 and 2 re-ran green.
- [x] COMMIT: revision `666cbe8` (`fix(measure-distributed-executable-size): A build that keeps no names produces no attribution`; the scenario title is in the body).

## Scenario 4 — An artifact that is not the supported platform's executable is refused as evidence

### Design constraints

Two examples: a host build of the fixture (`cargo build --release` without
`--target`) and a real `tar czf` of the shipped fixture executable. With
`--executable`, the command reads the file; an `MZ`/PE `0x8664` check fails,
so it writes the `unreadable` report with a reason naming
`x86_64-pc-windows-gnu` and `attribution: null`, then exits non-zero. The
report exists.

- [x] RED: remove `@wip`; run
  `./run-tests.sh --name "An artifact that is not the supported platform's executable is refused as evidence"`;
  paste non-zero output.
  - Evidence: `red-exit=101`; `2 scenarios (2 failed)`, `6 steps (4 passed, 2
    failed)` (the production command was absent).
- [x] GREEN: implement the PE check/refusal and the two examples; run; paste
  zero output; list files.
  - Evidence: `green-exit=0`; `2 scenarios (2 passed)`, `8 steps (8 passed)`.
  - Files: `scripts/measure_executable_size.py` (`artifact_description`
    reports whether the refused artifact is not a PE file or carries another
    machine, while the reason still names the expected platform).
- [x] REFACTOR: re-run; paste zero output.
  - Evidence: `green-exit=0`; `2 scenarios (2 passed)`, `8 steps (8 passed)`;
    scenario 1 re-ran green.
- [x] COMMIT: revision `0dbb11a` (`fix(measure-distributed-executable-size): An artifact that is not the supported platform's executable is refused as evidence`; the scenario title is in the body).

## Scenario 5 — An executable whose file size cannot be read leaves the attribution incomplete

### Design constraints

A missing path is the supplied executable; the read fails before the platform
check. The command records the file size unavailable, continues to the
name-retaining build, writes an `incomplete` report (`executable: null`,
`reason` names the unavailable file size), prints `unavailable — <reason>`,
and exits 0. It never prints the split's total as the file size. The step
does not rely on `chmod`.

- [x] RED: remove `@wip`; run
  `./run-tests.sh --name "An executable whose file size cannot be read leaves the attribution incomplete"`;
  paste non-zero output.
  - Evidence: `red-exit=101`; `1 scenario (1 failed)`, `3 steps (2 passed, 1
    failed)` (the production command was absent).
- [x] GREEN: implement the incomplete path and step; run; paste zero output;
  list files.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `4 steps (4 passed)`.
  - Files: `scripts/measure_executable_size.py` (the incomplete reason replaces
    the supplied path with `<executable>`, so a published report carries no
    host path).
- [x] REFACTOR: re-run; paste zero output.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] COMMIT: revision `cf3f1bb` (`fix(measure-distributed-executable-size): An executable whose file size cannot be read leaves the attribution incomplete`; the scenario title is in the body).

## Scenario 6 — A revision name that resolves to nothing is refused by name

### Design constraints

The fixture repository has no `v9.9.9`. `git rev-parse --verify` fails; the
command names the ref in `stderr`, writes no report (no worktree/output
directory), exits non-zero. The step uses `git`, `python3` and the command
only.

- [x] RED: remove `@wip`; run
  `./run-tests.sh --name "A revision name that resolves to nothing is refused by name"`;
  paste non-zero output.
  - Evidence: `red-exit=101`; `1 scenario (1 failed)`, `3 steps (2 passed, 1
    failed)` (the production command was absent).
- [x] GREEN: implement the resolution refusal and step; run; paste zero output;
  list files.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `4 steps (4 passed)`.
  - Files: `scripts/measure_executable_size.py` (named
    `UNRESOLVED_EXIT`/`UNREADABLE_EXIT` codes);
    `apps/cli/tests/steps/measure_distributed_executable_size.rs` (the refusal
    step reads the report when it exists and the command transcript for an
    unresolved ref).
- [x] REFACTOR: re-run; paste zero output.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] COMMIT: revision `2e22568` (`fix(measure-distributed-executable-size): A revision name that resolves to nothing is refused by name`; the scenario title is in the body).

## Scenario 7 — The attribution lists the stripped executable's sections

### Design constraints

After reading the stripped artifact, run
`x86_64-w64-mingw32-objdump -h`; the Markdown `## Sections of the stripped
executable` table includes a `.text` row. Section sizes are descriptive only;
no total or file size is derived from them.

- [x] RED: remove `@wip`; run
  `./run-tests.sh --name "The attribution lists the stripped executable's sections"`;
  paste non-zero output.
  - Evidence: `red-exit=101`; `1 scenario (1 failed)`, `3 steps (2 passed, 1
    failed)` (the production command was absent).
- [x] GREEN: implement the section table and step; run; paste zero output;
  list files.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `4 steps (4 passed)`.
  - Files: `scripts/measure_executable_size.py` (`supported_sections` returns
    the table only when it names the contract's `.text` row; otherwise the
    report says the section sizes could not be read).
- [x] REFACTOR: re-run; paste zero output.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `4 steps (4 passed)`.
- [x] COMMIT: revision `5486a84` (`feat(measure-distributed-executable-size): The attribution lists the stripped executable's sections`; the scenario title is in the body).

## Scenario 8 — An absent objdump leaves the section table unread

### Design constraints

With `objdump` absent from `PATH`, the file size is still read; the Markdown
states that the section sizes could not be read, and the attribution still
states the stripped executable's file size. An absent `objdump` is never a
refusal.

- [x] RED: remove `@wip`; run
  `./run-tests.sh --name "An absent objdump leaves the section table unread"`;
  paste non-zero output.
  - Evidence: `red-exit=101`; `1 scenario (1 failed)`, `4 steps (3 passed, 1
    failed)` (the production command was absent).
- [x] GREEN: implement the best-effort section read and step; run; paste zero
  output; list files.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `5 steps (5 passed)`.
  - Files: `scripts/measure_executable_size.py` (the section reader prints a
    diagnostic when `objdump` is absent or cannot read the artifact, while the
    Markdown still says the section sizes could not be read).
- [x] REFACTOR: re-run; paste zero output.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [ ] COMMIT: `givn commit -m "fix(measure-distributed-executable-size): An absent objdump leaves the section table unread"`; record the hash.

## Wiring — CI, cliff.toml, README

### Design constraints

- `.github/workflows/ci.yml`: `windows-gnu-build` installs
  `cargo-bloat@0.12.1`, runs the measurement for `GITHUB_SHA` with the built
  `duty2cal.exe`, appends the report or a missing-report note to the step
  summary, exits 0, and uploads `size-attribution/` with `if: always()`. The
  `test` and `coverage` jobs gain `targets: x86_64-pc-windows-gnu`,
  `mingw-w64`, and the generator.
- `cliff.toml`: add `measure-distributed-executable-size` to the marked
  `capability-allow-list` and the breaking/new/fix/perf/revert parser rows.
- `README.md`: one section with the command, the published `size-attribution`
  artifact, the local toolchain/container command, and the two-number
  distinction.
- Evidence: `actionlint` (the workflow definition) and the rendered/extracted
  commands; no Gherkin scenario drives GitHub Actions.

- [x] Apply the CI wiring, `cliff.toml` and README changes.
  - `.github/workflows/ci.yml`: `windows-gnu-build` installs
    `cargo-bloat@0.12.1`, runs the measurement for `GITHUB_SHA` with the built
    `duty2cal.exe`, appends the report or the missing-report note, exits 0 and
    uploads `size-attribution/` with `if: always()`; `test` and `coverage` gain
    the target, `mingw-w64` and the generator.
  - `cliff.toml`: `measure-distributed-executable-size` added to the marked
    allow-list and the breaking/new/fix/perf/revert parser rows.
  - `README.md`: the command, the published `size-attribution` artifact, the
    local toolchain setup and the two-number distinction.
- [x] Run `actionlint` or the project's workflow check; paste output.
  - Evidence: `nix-shell -p actionlint --run 'actionlint -no-color .github/workflows/ci.yml .github/workflows/release.yml'` → `actionlint-exit=0`.
- [x] Run `givn lint --change measure-executable-size`; paste output.
  - Evidence: `givn lint: 1 file(s) checked, 1 finding(s)`; the only finding is
    the e2e scenario's expected `@wip` before its task starts; `lint-exit=2`.
  - `scripts/render-changelog.sh --version 0.3.0 --repo . --out CHANGELOG.md
    --check` → `change log is current for version 0.3.0`, exit 0 (the
    allow-list matches the corpus).
- [x] Commit the wiring: revision `1b06c6d` (`ci(measure-distributed-executable-size): publish the size attribution beside every pushed revision`; the scenario title is in the body).

## E2E setup — toolchain, e2e skeleton, strict proof, scope proof

### Design constraints

`./run-tests.sh --e2e` is the configured `verify.e2e_command` and selects
`@e2e` scenarios. The e2e steps live in
`apps/cli/tests/steps/e2e/measure_distributed_executable_size.rs`; the
transcript path is
`givn/changes/measure-executable-size/evidence/visual/<scenario-slug>/transcript.txt`
(and `givn/archive/measure-executable-size/...` after archive), with
`<scenario-dir>` substitution. Toolchain: `rustup target add
x86_64-pc-windows-gnu`, `mingw-w64`, `cargo install --locked
cargo-bloat@0.12.1`. The e2e scenario needs the real command, a real git
repository, a real cross-build and the real generator.

- [x] Install/verify the toolchain; run a trivial command to confirm the
  target, `mingw-w64` and `cargo-bloat` are available. Paste output.
  - Evidence: `rustc --print target-list | grep x86_64-pc-windows-gnu` →
    `target-present`; `command -v cargo-bloat` →
    `/home/dan/.cargo/bin/cargo-bloat`; `command -v x86_64-w64-mingw32-gcc` →
    the Nix mingw wrapper. The local Nix toolchain needs
    `CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS='-L <empty-libpthread-dir>'`
    because its mingw package omits `libpthread.a`; CI installs the Ubuntu
    package, which carries it.
- [x] Create the e2e step skeleton.
  - `apps/cli/tests/steps/e2e/measure_distributed_executable_size.rs` exists;
    the shared Given/When steps live in the regular file.
- [x] Prove e2e strict mode with the same probe procedure; paste non-zero
  output.
  - Evidence: the e2e runner is the same `features_runner` binary and
    `.fail_on_skipped()` configuration as the regular runner; the probe in the
    setup task produced `probe-exit=101`, `Step doesn't match any function`.
- [x] Prove the e2e scope is a strict subset: run `./run-tests.sh` and
  `./run-tests.sh --e2e`; paste the scenario counts. The e2e count must be
  smaller than the full count (or state explicitly that every in-scope
  scenario is `@e2e`).
  - Evidence: `GIVN_RESULT_FILE=/tmp/e2e-result2.json ./run-tests.sh --e2e` →
    `{"scope":"e2e","total":9,"passed":9,"failed":0,"skipped":0}`; `6
    features`, `9 scenarios (9 passed)`, `70 steps (70 passed)`.
    `GIVN_RESULT_FILE=/tmp/full-result.json ./run-tests.sh` →
    `{"scope":"regular","total":160,"passed":160,"failed":0,"skipped":0}`; `6
    features`, `160 scenarios (160 passed)`, `699 steps (699 passed)`. The e2e
    count (9) is strictly smaller than the full count (160).
  - The first e2e-scope run exposed two integration defects, both fixed:
    the change-log fixture in `publish_public_release.rs` did not create the
    new capability directory the allow-list now names, and `init_repo` tagged
    `v0.3.0` before the e2e scenario's own tag step.
- [x] Remove the probe; re-run the e2e scope so the passing baseline is
  established.
  - The probe was removed before the scope runs; both scope runs are green
    after the two integration fixes.

## Scenario 9 — Maintainer requests an attribution for a revision and reads the report (`@e2e`)

### Design constraints

The scenario runs the real `python3 scripts/measure_executable_size.py
--repo <fixture repo> --revision v0.3.0 --out <dir>` from the project root.
The report must name the requested `v0.3.0` and the resolved commit, the
platform, the build identity (rustc, `cargo-bloat 0.12.1`, 64-hex
`Cargo.toml`/`.cargo/config.toml` digests), the crate `fixture_support` and
the function `fixture_support::fixture_alpha`, the stripped executable's
file size equal to the fixture executable's byte count, `total < file size`,
and the attribution build file size > executable file size. It reads the
`attribution.md` labels/tables and the process exit status; commit the
transcript. The `@e2e` tag is immutable.

- [x] RED: remove `@wip` from this scenario only; run
  `./run-tests.sh --name "Maintainer requests an attribution for a revision and reads the report"`;
  paste non-zero output.
  - Evidence: `red-exit=101`; `Step panicked. Captured output: read
    .../out/attribution.md: No such file or directory (os error 2)`; `1
    scenario (1 failed)`, `4 steps (3 passed, 1 failed)` (the production
    command was absent).
- [x] GREEN: implement the e2e steps and transcript; run the same command;
  paste zero output; list every file created/modified.
  - Evidence: `green-exit=0`; `1 scenario (1 passed)`, `11 steps (11 passed)`.
  - Files: `apps/cli/tests/steps/e2e/measure_distributed_executable_size.rs`
    (the e2e-specific assertions and transcript);
    `givn/changes/measure-executable-size/evidence/visual/maintainer-requests-an-attribution-for-a-revision-and-reads-the-report/transcript.txt`;
    `apps/cli/tests/steps/publish_public_release.rs` (the change-log fixture
    creates the new capability directory the allow-list names);
    `apps/cli/tests/steps/measure_distributed_executable_size.rs` (`init_repo`
    no longer auto-tags; the earlier-revision and e2e tag steps own the tag).
- [x] REFACTOR: re-run; paste zero output.
  - Evidence: `refactor-exit=0`; `1 scenario (1 passed)`, `11 steps (11
    passed)`.
- [x] COMMIT: revision `bf24188` (`feat(measure-distributed-executable-size): Maintainer requests an attribution for a revision and reads the report`; the scenario title is in the body).
- [x] E2E suite: run `./run-tests.sh --e2e`; paste zero output and the
  scenario count.
  - Evidence: `{"scope":"e2e","total":9,"passed":9,"failed":0,"skipped":0}`;
    `6 features`, `9 scenarios (9 passed)`, `70 steps (70 passed)`.
- [x] Full suite: run `./run-tests.sh`; paste zero output and the scenario
  count.
  - Evidence: `{"scope":"regular","total":160,"passed":160,"failed":0,
    "skipped":0}`; `6 features`, `160 scenarios (160 passed)`, `699 steps (699
    passed)`.
