# Proposal: developer-setup

## Release note

A developer on any machine can reproduce the build environment from one pinned toolchain declaration, so the compiler, its LLVM tools and the pinned build tools match CI and the release.

## Use-Case Context

- Use-case ID: none — this is an operational maintenance change to the build
  environment; no product capability changes.
- Ideation topic: none.
- Seed path: none.
- Confirmed Personas: none — the change adds no product surface (no browser
  interface, terminal interface or CLI text). The operator requested the setup
  on 2026-10-29.

## Problem / Opportunity

The repository never names the compiler it builds with, so every machine
supplies its own and the pairing with LLVM is accidental.

- `rust-toolchain.toml` declares `channel = "stable"`, `rustfmt` and `clippy`,
  and no targets. A clone on one machine and a clone on another resolve
  different compilers, and `rustup update` moves the project without a
  revision.
- CI repeats the choice in every job: `dtolnay/rust-toolchain@stable` in
  `ci.yml`, `nightly.yml`, `release.yml` and `site.yml`, each adding its own
  components or targets. The effective toolchain is per job, not per
  repository.
- The compiler's own LLVM tools are the only LLVM used by coverage.
  `cargo llvm-cov` resolves `llvm-cov` and `llvm-profdata` from
  `$sysroot/lib/rustlib/<host>/bin` and never from `PATH`. When the selected
  toolchain lacks `llvm-tools-preview`, or a foreign `llvm-profdata` is put on
  `PATH` to compensate, the raw profile format does not match the tool: the
  run reports zero counters and the archive gate refuses. This failure class
  is recorded in `private/llmv-fix.md` (profile format 11 against Nix LLVM
  21.1.8, 22.1.8 and 23.1.3).
- This workstation works only because nixpkgs' `rustc` happens to carry
  matching LLVM tools in its sysroot. A rustup machine without the component
  does not, and the NixOS host ignores `rust-toolchain.toml` entirely because
  it has no rustup shim.
- There is no non-Nix setup path. `CONTRIBUTING.md` lists the checks but not
  how to obtain the toolchain or the pinned cargo tools. `git-cliff@2.14.2`,
  `cargo-about@0.9.0` and `cargo-bloat@0.12.1` are installed only in CI;
  `cargo-llvm-cov` is 0.8.7 locally, 0.9.0 in nixpkgs and latest in CI.
- The earlier release-size ideation already recorded the local gap: no rustup,
  no `x86_64-pc-windows-gnu` target, no `x86_64-w64-mingw32-gcc`, no
  `cargo-bloat` on this workstation (`givn/ideation/release-executable-size/session.md`).

Consequence: the compiler, the LLVM tools used for coverage and the tools
that render byte-compared files (change log, third-party notices) can differ
between a developer machine, CI and the release. "Works on my machine" is the
normal state, and the source-build route the public text promises has no
defined environment.

## Proposed Solution

The repository declares one environment and every installer consumes the same
declaration:

- **One compiler pin.** `rust-toolchain.toml` names the exact stable release,
  the components the build needs — `rustfmt`, `clippy`, `llvm-tools-preview` —
  and the targets the build paths need: `wasm32-unknown-unknown` for the
  browser bundle and `x86_64-pc-windows-gnu` for the release executable. A
  machine with rustup resolves exactly that; the compiler owns its LLVM tools,
  so coverage has no second LLVM to mix.
- **One developer bootstrap on any machine.** `scripts/setup-dev.sh` brings a
  fresh Linux or macOS machine to the same state: it ensures rustup, lets
  `rustup show` install the pinned toolchain and its components and targets,
  installs the pinned cargo tools, installs the Node version named by
  `.nvmrc`, installs the Playwright driver packages and Chromium, and reports
  any missing system tool with the package that provides it. It is idempotent
  and installs no operating-system package.
- **One guard.** `scripts/check-toolchain.sh` refuses when the active `rustc`
  differs from the declared release or when the sysroot lacks `llvm-cov` and
  `llvm-profdata`, naming both remedies (`rustup component add
  llvm-tools-preview`, or `nix develop`). `measure-coverage.sh` runs it before
  measuring, so a mismatch is a named refusal instead of a zero report.
- **One declaration for the Nix machines.** `flake.nix` builds its development
  shell from the same `rust-toolchain.toml` through the pinned rust-overlay,
  and adds the pinned cargo tools, Node, Playwright browsers, Python, jq, bats,
  actionlint, reuse, OpenTofu and the Windows cross compiler. `nix develop`
  therefore produces the same compiler and LLVM as the rustup path.
- **One source in CI.** Every job installs the channel read from
  `rust-toolchain.toml` and stops passing its own components and targets; the
  pinned cargo tools are installed from `scripts/dev-tool-versions.env`. The
  guard runs in the test and coverage jobs.
- **One documented route.** `CONTRIBUTING.md` gains a "Set up a development
  machine" section with the two installers (rustup script on any OS, `nix
  develop` on Nix), the pinned-versions table and the rule that coverage uses
  only the compiler's LLVM tools. `README.md`'s Prerequisites points at the
  same pin.
- **Consistent wording.** The release-size capability's baseline is the
  repository's pinned toolchain, not an unqualified "current stable".

Observable difference for a developer: a fresh clone on Ubuntu or macOS runs
`./scripts/setup-dev.sh`, then `./run-tests.sh` and `./measure-coverage.sh`,
and the compiler and LLVM tools are the same release CI used. A clone that
still has another compiler is refused by name. A Nix machine runs `nix
develop` and gets the same. No machine needs Nix, and no Nix machine needs
rustup.

## Capability Routing

`givn spec route --change developer-setup` reported no signal ("the top
candidates tie at 13.00"). The change adds no product behaviour, so `specs` is
skipped in `.givn-skip`.

| Proposed capability | Decision | Rationale |
|---|---|---|
| none | No capability delta | The change defines the build environment, not observable product behaviour. It edits build configuration, developer scripts, CI and developer documentation; no `.feature` file is added, amended or removed. |

## Out of Scope

- **Product code, product scenarios and coverage policy.** The conversion,
  the browser page, the CLI output and the test suite are unchanged.
- **The CI runner image and the release workflow's semantics.** The workflow
  keeps its jobs, permissions, timeouts, measurements and publication steps;
  only the toolchain source and the tool installs change.
- **A container image or a vendored toolchain.** No Dockerfile, no
  `nix-portable`, no checksum-pinned tarball installer and no air-gapped
  mirror. The pin is a version consumed by rustup or rust-overlay.
- **Adding platforms.** The Windows cross build stays a Linux task; macOS,
  Windows-native and ARM hosts are not added to CI.
- **Updating the Playwright or OpenTofu versions.** The change pins what the
  repository already uses and records the drift; it does not upgrade them.
- **A Nix CI job.** CI continues to use rustup, which is the non-Nix path; a
  flake check on a runner is a possible later change.
- **Changing the archive or release gates.** The gates keep their commands;
  only their environment becomes reproducible.

## Open Questions

### D1 — How the compiler is pinned across Nix and non-Nix machines

- Question: where does the compiler version live, given that one machine has
  Nix and the project's public text promises a source build with a Rust
  toolchain on any platform?
- Options:
  1. **`rust-toolchain.toml` as the single pin, rustup on any OS and
     rust-overlay on Nix** — gain: one file read by both installers, the
     standard rustup mechanism on Linux, macOS and Windows, and no Nix
     requirement for contributors; cost: a non-Nix machine needs rustup and
     network access once, and the NixOS host needs the flake to honour the
     file because its nixpkgs `rustc` ignores it; in practice: a fresh Ubuntu
     clone runs `rustup show` and gets 1.99.1 with `llvm-tools-preview`, while
     the NixOS host runs `nix develop` and gets the same release.
  2. **The flake as the only pin** — gain: one command on every machine that
     has Nix; cost: a machine without Nix has no declaration to follow, so the
     public source-build route stays undefined and this workstation's success
     stays accidental; in practice: an Ubuntu contributor reads a flake they
     cannot run.
  3. **A container image** — gain: the host contributes nothing; cost: an image
     to build and store, Docker or Podman on every machine, and the browser
     driver inside the image; in practice: `docker run` replaces `cargo` for
     every small task.
  4. **Vendored toolchain tarballs** — gain: no rustup and no network at build
     time; cost: a maintained downloader that resolves components and per-target
     standard libraries and verifies every hash, duplicating rustup; in
     practice: a new script that must be repaired on every Rust release.
- Disposition: answered — option 1. Rejected: option 2 leaves non-Nix
  machines undefined; option 3 adds a runtime the project does not need for
  its own CI; option 4 reimplements a tool whose artifact hashes rustup already
  verifies. The flake is in scope because this workstation needs it to bind
  the same pin (D4).

### D2 — Which targets the pin installs

- Question: should `rust-toolchain.toml` install only the host standard
  library, the browser target, or both cross targets for every developer?
- Options:
  1. **Host only** — gain: the smallest install; cost: `scripts/build-web.sh`
     and a local Windows cross build refuse with a missing-target error, and
     the developer adds targets by hand, which is the state this change
     removes; in practice: `cargo build --target wasm32-unknown-unknown` fails
     on a fresh clone.
  2. **Browser and Windows targets** — gain: every build path the repository
     automates works on a fresh clone, including the local size measurement
     and the Windows executable; cost: roughly 30 MB of extra standard
     libraries per toolchain; in practice: `scripts/build-web.sh` and `cargo
     build --release --target x86_64-pc-windows-gnu` both run after setup.
  3. **Browser target only** — gain: the browser bundle works and the install
     stays smaller; cost: the Windows build remains a CI-only path and the
     local release-size baseline cannot be reproduced; in practice: a
     maintainer cannot rebuild the released target locally.
- Disposition: answered — option 2. The repository automates both paths, the
  earlier ideation recorded the missing Windows target as a local gap, and the
  added download is bounded.

### D3 — How the pinned non-Rust tools are provisioned

- Question: how do the cargo tools and the Node/Playwright/browser tooling
  reach a developer machine and CI from one set of versions?
- Options:
  1. **One versions file, read by the developer script and a CI composite
     action** — gain: one edit moves local, Nix and CI together, and the
     byte-compared changelog and notices are rendered by the same versions;
     cost: a composite action and one small shell file to maintain; in
     practice: `git-cliff` is 2.14.2 on the laptop, in `nix develop` and in CI
     from one line.
  2. **Versions inline in CI and repeated in documentation** — gain: no new
     file; cost: three copies drift silently; the local coverage tool is
     already 0.8.7 while CI takes latest; in practice: a locally rendered
     change log differs from the committed one for a reason nobody sees.
  3. **A general version manager (mise/asdf)** — gain: one command for Node,
     Python and the cargo tools; cost: another tool on every machine, plugin
     sources to trust, and the Rust pin still belongs to rustup; in practice:
     `mise install` becomes a second bootstrap beside `setup-dev.sh`.
- Disposition: answered — option 1. The repository already uses file-based
  pins (`.nvmrc`, `web/.wasm-pack-version`, `infra/.opentofu-version`); one
  more file follows the existing convention.

### D4 — Whether the Nix development shell belongs to this change

- Question: does this change also add the flake that makes the NixOS host
  honour the pin, or does it defer that to a later change?
- Options:
  1. **Include it** — gain: the host, CI and other machines all resolve the
     same toolchain from one file, and the earlier local cross-build gap
     closes; cost: `flake.lock` must carry a rust-overlay revision that knows
     the pinned release, so a Rust bump also updates the lock; in practice:
     after a toolchain bump, `nix develop` fails by name until `nix flake
     update rust-overlay` runs in the same revision.
  2. **Defer it** — gain: a smaller change; cost: this workstation's compiler
     is still whatever nixpkgs ships, so the guard passes by coincidence until
     the next Rust release and the host cannot be reproduced from the
     repository; in practice: the pin says 1.99.1 while `nixpkgs#rustc`
     evaluates to 1.99.0, and nothing enforces the difference.
- Disposition: answered — option 1. The flake is the consumer that binds this
  machine, and deferring it leaves the stated problem on the machine the
  operator uses.
