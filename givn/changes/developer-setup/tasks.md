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

- [ ] Pin the compiler in `rust-toolchain.toml` and add the versions file.
  - Evidence:
  - Commit:

- [ ] Add `scripts/check-toolchain.sh` and its bats test.
  - Evidence:
  - Commit:

- [ ] Call the guard from `run-tests.sh` and `scripts/measure-coverage.sh`.
  - Evidence:
  - Commit:

## 2. Non-Nix bootstrap

- [ ] Add `scripts/setup-dev.sh` with a `--check` mode and its bats test.
  - Evidence:
  - Commit:

## 3. Nix development shell

- [ ] Add `flake.nix`/`flake.lock` with the pin, the tools, the browser build
  and the Windows cross compiler.
  - Evidence:
  - Commit:

## 4. CI

- [ ] Add the rust-toolchain and install-dev-tools composite actions and use
  them in the four workflows; drop the per-job components/targets.
  - Evidence:
  - Commit:

## 5. Documentation

- [ ] Add the developer-setup section to `CONTRIBUTING.md` and point the
  README prerequisites at the pin.
  - Evidence:
  - Commit:

## 6. Verification

- [ ] Run the script tests, the suite, coverage, `nix develop` and the Windows
  cross build against the pinned toolchain.
  - Evidence:
  - Commit:

## Commit log

| Revision | Purpose |
|---|---|
| | |
