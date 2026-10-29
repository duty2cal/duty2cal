# Tasks: developer-setup

Operational maintenance change; `specs` is skipped in `.givn-skip`, and no
product scenario is added, modified or removed. The task list records the
toolchain pin, the developer bootstrap, the guard, the Nix shell, the CI
wiring, the documentation and the verification.

Design constraints: the compiler is the stable release pinned in
`rust-toolchain.toml` with `rustfmt`, `clippy`, `llvm-tools-preview`, the
`wasm32-unknown-unknown` and `x86_64-pc-windows-gnu` targets; rustup and the
Nix shell read that file. The guard refuses a non-exact channel, a different
`rustc`, a sysroot without `llvm-cov`/`llvm-profdata`, or a set
`LLVM_COV`/`LLVM_PROFDATA`, and runs before `run-tests.sh` and
`measure-coverage.sh`. The bootstrap installs no operating-system package and
needs no root. No new ADR; chapter 2 owns the constraint. Supported bootstrap
platforms: Linux/macOS via rustup, `x86_64-linux` via the flake; Windows
native is out of scope.

## 1. Toolchain pin and guard

- [x] Pin the compiler in `rust-toolchain.toml` and add the versions file.
  - Evidence: `rustc -V` on the host is `rustc 1.99.1`; the file names
    `profile = "minimal"`, `rustfmt`, `clippy`, `llvm-tools-preview`,
    `wasm32-unknown-unknown` and `x86_64-pc-windows-gnu`;
    `scripts/dev-tool-versions.env` sources to
    `CARGO_LLVM_COV_VERSION=0.9.0` and `GIT_CLIFF_VERSION=2.14.2`.
  - Commit: `724e97d9b653dc14ef3bdcebb5eb0a14ee8d7587`

- [x] Add `scripts/check-toolchain.sh` and its bats test.
  - Evidence: `bats scripts/tests/check-toolchain.bats` → 5/5 passed
    (matching compiler, different compiler, missing `llvm-cov`, set
    `LLVM_PROFDATA`, moving channel); `./scripts/check-toolchain.sh` →
    `toolchain: rustc 1.99.1; LLVM tools in …/rust-minimal-1.99.1/lib/rustlib/x86_64-unknown-linux-gnu/bin`.
  - Commit: `724e97d9b653dc14ef3bdcebb5eb0a14ee8d7587`

- [x] Call the guard from `run-tests.sh` and `scripts/measure-coverage.sh`.
  - Evidence: `./run-tests.sh --name "Crew member converts a supported duty
    plan and receives a calendar file"` printed the guard line and then
    1 scenario / 5 steps passed; `./measure-coverage.sh` wrote both Cobertura
    reports after the guard.
  - Commit: `724e97d9b653dc14ef3bdcebb5eb0a14ee8d7587`

## 2. Non-Nix bootstrap

- [x] Add `scripts/setup-dev.sh` with a `--check` mode and its bats test.
  - Evidence: `bats scripts/tests/setup-dev.bats` → 3/3 passed (missing
    rustup, missing cargo, wrong Node major); the script sources
    `$CARGO_HOME/env`, sources `$NVM_DIR/nvm.sh`, reads
    `web/e2e/.nvmrc`, runs `npm ci --prefix web/e2e` before Chromium and
    never invokes a package manager.
  - Commit: `724e97d9b653dc14ef3bdcebb5eb0a14ee8d7587`

## 3. Nix development shell

- [x] Add `flake.nix`/`flake.lock` with the pin, the tools, the browser build
  and the Windows cross compiler.
  - Evidence: `nix develop -c bash -lc 'rustc -V; scripts/check-toolchain.sh'`
    → `rustc 1.99.1`, guard passes, `wasm-pack` 0.15.0,
    `cargo-llvm-cov` 0.9.0, Node 24.21.0;
    `NIX_LDFLAGS_x86_64_w64_mingw32` names the pthreads and mcfgthread
    libraries; `cargo build --release --target x86_64-pc-windows-gnu` inside
    the shell produced `duty2cal.exe` (841,728 B). `reuse` and OpenTofu are
    deliberately not in the shell (broken package; version drift) and the
    bootstrap reports them.
  - Commit: `724e97d9b653dc14ef3bdcebb5eb0a14ee8d7587`

## 4. CI

- [x] Add the rust-toolchain and install-dev-tools composite actions and use
  them in the four workflows; drop the per-job components/targets.
  - Evidence: `actionlint .github/workflows/*.yml` exit 0; the toolchain
    action runs `rustup show` (installs the file's components and targets)
    then the guard; the release smoke job gained the checkout the local
    composite action needs; the five `dtolnay/rust-toolchain@stable` blocks
    and the per-job `components`/`targets` are gone.
  - Commit: `724e97d9b653dc14ef3bdcebb5eb0a14ee8d7587`

## 5. Documentation

- [x] Add the developer-setup section to `CONTRIBUTING.md` and point the
  README prerequisites at the pin.
  - Evidence: `CONTRIBUTING.md` "Set up a development machine" names both
    installers and the pin table; `README.md` Prerequisites names the pin
    and the compiler's LLVM tools; `bash scripts/public-text-check.sh` →
    clean (18 promises); `bash scripts/license-headers.sh` → clean.
  - Commit: `724e97d9b653dc14ef3bdcebb5eb0a14ee8d7587`

## 6. Verification

- [x] Run the script tests, the suite, coverage, `nix develop` and the Windows
  cross build against the pinned toolchain.
  - Evidence: `nix develop -c ./run-tests.sh` → 140 scenarios / 592 steps
    passed; `nix develop -c ./measure-coverage.sh && ./merge-coverages.sh` →
    13 files, 2,155 lines, 1,880 covered; `cargo fmt --check` and
    `cargo clippy --workspace --all-targets -- -D warnings` clean;
    `scripts/pii-guard.sh` clean; `git diff --check` clean.
  - Commit: `724e97d9b653dc14ef3bdcebb5eb0a14ee8d7587`

## Commit log

| Revision | Purpose |
|---|---|
| `724e97d9b653dc14ef3bdcebb5eb0a14ee8d7587` | The pin, the guard, the bootstrap, the flake, the CI actions and workflows, the documentation and the arc42 chapters. |
| this revision | The completed task record and its evidence; archived with the change. |
