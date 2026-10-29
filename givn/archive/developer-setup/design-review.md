# Design review: developer-setup

Two fresh-context subagents ran this review: a grilling pass over the full
planning layer and a hardening pass that re-walked every branch and applied
residual fixes. This file records the questions, the dispositions and the
result.

## Grilling questions

### Q1 — How the Linux bootstrap provides Playwright's host libraries

The proposal says the bootstrap installs no operating-system package, while the
first design draft used `npx playwright install --with-deps`, which runs the
distribution package manager with root.

- Options: (A) no root, install Chromium and report the missing libraries and
  packages by name; (B) keep `--with-deps` and amend the promise to allow
  root; (C) default to no root with an opt-in `--install-system-deps` flag;
  (D) drop Chromium from the bootstrap and leave it to the browser-suite
  command.
- Disposition: **A**. The proposal's "installs no operating-system package" is
  the constraint, so the bootstrap never runs apt/dnf and never needs root; it
  names the missing libraries after downloading Chromium. B contradicts the
  proposal, C adds a flag the change does not need, D weakens the one-command
  setup. Realized in `design.md` "Playwright" and the script task.

### Q2 — Whether the release-size wording lands now or is deferred

The proposal promises the size baseline uses the pinned toolchain, while
`givn/specs/reduce-distributed-executable-size/usecase.md` still says "the
current stable toolchain".

- Options: (A) deliver the wording now as a factual correction in the use
  case (no scenario or step changes); (B) defer it with an owner, condition and
  date recorded here.
- Disposition: **A**. The proposal promised it, the correction changes no
  behaviour (a pinned stable release is still stable), and `specs` stays
  skipped. Realized at `usecase.md:29,51`.

No other question required operator authority; the repository resolved the
rest.

## Findings and resolutions

The grilling produced 21 findings (2 blockers, 9 major, 10 minor). All are
resolved in the current tree:

| # | Severity | Finding | Resolution |
|---|---|---|---|
| 1 | blocker | The CI composite action claimed the proxy path installs the file's components; jobs would lose `rustfmt`, `clippy`, the wasm target and `llvm-tools-preview`. | The action ensures rustup, runs `rustup show` (the command that installs the file's components and targets), then runs the guard; jobs stop passing components/targets. |
| 2 | blocker | `windows.pthreads` / `windows.mcfgthreads` unqualified are Windows-host packages and fail evaluation on Linux. | The flake names `pkgsCross.mingwW64.windows.pthreads` and `.mcfgthreads`; their setup hooks set `NIX_LDFLAGS_x86_64_w64_mingw32`. |
| 3 | major | No-ADR conclusion had the wrong canonical destination and an incomplete existing-ADR check. | One canonical artifact: chapter 2, with the D1 alternatives; `CONTRIBUTING.md` owns the procedure; ADR-0003 and TD-005 checked, neither decision amended, their factual text corrected. |
| 4 | major | The guard did not refuse `LLVM_COV`/`LLVM_PROFDATA`, which cargo-llvm-cov prefers over the sysroot. | The guard refuses either variable, requires an exact channel, derives the tool directory from `rustc --print target-libdir`, and names the remedy. |
| 5 | major | The bootstrap used `--with-deps` (root), did not source the rustup env or `nvm.sh`, and named a root `.nvmrc` that does not exist. | No root; sources `$CARGO_HOME/env` and `$NVM_DIR/nvm.sh`; reads `web/e2e/.nvmrc`; reports missing host libraries. |
| 6 | major | The browser suite could not run: no `npm ci`, and `wasm-pack` was missing from the flake. | `npm ci --prefix web/e2e` precedes every Playwright command; `wasm-pack` 0.15.0 is in the shell. |
| 7 | major | The guard was not wired into the test path, so "refused by name" was half delivered. | `run-tests.sh` and `measure-coverage.sh` call it before doing work. |
| 8 | major | The proposal's consistent-wording promise was silently deferred. | The two baseline sentences in the size use case now name the pinned toolchain. |
| 9 | major | The two new scripts had no executable evidence. | `scripts/tests/check-toolchain.bats` and `scripts/tests/setup-dev.bats` cover the refusals; the `scripts` job runs them. |
| 10 | major | Chapter 6 was marked No although the chapter documents operational flows. | Chapter 6 gains the fresh-machine bootstrap and refusal flow; the assessment says Yes. |
| 11 | major | Adding the Dependabot `rust-toolchain` ecosystem would move the pin without `flake.lock`, which CI cannot detect. | The ecosystem is deliberately not added; a toolchain bump is a deliberate change that updates the lock. |
| 12 | minor | TD-005 and ADR-0003 still described the rolling toolchain action. | Both corrected factually; R-011 likewise. |
| 13 | minor | "rustup on Windows" and "any Nix-capable Linux" overstated the supported bootstrap. | Linux/macOS via the script, `x86_64-linux` via the shell, Windows native out of scope. |
| 14 | minor | Chapter 9 had no routing record for the non-qualifying candidate. | Added with `CANONICAL_ARTIFACT` → chapter 2. |
| 15 | minor | The flake's OpenTofu (1.13.2) disagreed with `infra/.opentofu-version` (1.13.1). | OpenTofu is not in the build shell; the infra pin stays its own. |
| 16 | minor | Chapter 11 lacked the Playwright browser/npm lock risk. | R-081 added; the design says browsers, not driver. |
| 17 | minor | "Changes no CLI output" denied the scripts' terminal surface. | The section says no product CLI change; the scripts' output is bats-covered. |
| 18 | minor | "Guard" was used but undefined. | **Toolchain guard** added to the glossary. |
| 19 | minor | The D4 example contradicted the host, and the LLVM observation was unrecorded. | D4 now contrasts 1.99.1 with `nixpkgs#rustc` 1.99.0; the observation commands are recorded. |
| 20 | minor | `rustup show` installs every declared target in every job. | Accepted and recorded; rustup artifacts are outside the Rust cache and the timeouts hold. |
| 21 | minor | The architecture diagram omitted the CI and guard readers. | Both edges added; four readers, one guard. |

## Branches walked

- **Scope:** the proposal's outcomes map to the design's deliverables; no
  extra capability and no missing outcome.
- **Tech choices:** the pin plus two installers is the simplest route that
  keeps non-Nix machines defined; the container and vendored alternatives are
  recorded in chapter 2.
- **Missing scenarios:** `specs` is skipped and no product behaviour changes;
  the scripts' refusals are covered by bats tests instead.
- **Testability:** the guard, the bootstrap's `--check` mode and the bats
  tests are concrete; strict mode and the single-scenario command are named
  although no scenario is added.
- **E2E fidelity / interaction coverage:** no capability or `@e2e` delta; the
  existing drivers and inventory are untouched.
- **Risk:** the most likely failure is a CI job losing a component; the
  `rustup show` mechanism plus the guard's sysroot assertions catch it by
  name.
- **Use-case and Persona context:** operational change, no use case, no
  Persona, no invented actor.
- **Decision ledger:** D1-D4 resolved with realizations; no seed.
- **Visual design contract / interface classification:** no rendered screen;
  the terminal scripts are covered by bats; the opposite (Web UI)
  classification does not hold.
- **ADR qualification:** no `NEW_ADR`; one canonical artifact, existing-ADR
  check completed.
- **Arc42 documentation:** 12 chapters re-derived; chapter 6 corrected to Yes;
  every Yes chapter checked; all files exist with real content and Mermaid
  diagrams only.
- **Ubiquitous language:** **Pinned toolchain** and **Toolchain guard** are in
  the glossary and used consistently.

## Hardening

Applied in `design.md`, `arc42.md`, chapters 5, 6, 7 and 11: the CI mechanism,
the cross-package attribute names, the guard's env refusal, the bootstrap's
environment sourcing and npm order, chapter 6's flow, R-081, the R-011/TD-005
corrections and the wording precision. No proposal scope change.

## Post-implementation design correction

Implementation exposed two gaps the first review could not see: the design
promised a Playwright browser-build/npm-client mismatch refusal that the
script and shell did not implement, and the CI composite asserted the LLVM
tools but not `cargo fmt --version`/`cargo clippy --version`. The design's
Playwright and CI rows were corrected, both mechanisms were implemented, and a
second fresh-context review (`DeltaReview`) re-ran the affected branches. It
verified the script refusal, the shell refusal, the bats proof, the composite
assertions, the corrected design text, the unchanged ADR conclusion and the
unchanged chapter set, and returned:

DESIGN-REVIEW: PASS
