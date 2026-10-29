# Contributing to duty2cal

Thanks for considering a contribution. This page holds what a change needs to
land; the [README](README.md) points here rather than repeating it.

## The route a change takes

1. **Behaviour first.** Every behaviour is specified in Gherkin before it is
   implemented. A change adds or amends a `.feature` under
   `givn/changes/<change>/specs/<capability>/`, and a scenario is RED before it
   is GREEN: write the step definitions, watch the scenario fail, then make it
   pass. The feature files are the executable specification, not a later
   translation of the code.
2. **One change at a time.** Work is organised as a change under
   `givn/changes/<change>/` (`givn instructions`, `givn status --change <change>`).
   One capability's scenario is one revision.
3. **Commit shape.** Commits use conventional subjects. The release note is the
   commit subject and the scenario title goes in the body, so the release text
   can quote the project's own history. `givn commit -m "<subject>"` stamps the
   revision with the project's attribution.
4. **Sign-off, no CLA.** Every commit carries the Developer Certificate of
   Origin: `git commit -s` adds the `Signed-off-by:` trailer, which states that
   you wrote the change or have the right to submit it. There is no CLA and no
   copyright assignment. A pull request whose commits carry no sign-off fails
   the `dco` check.
5. **Documentation.** Architecture decisions are recorded as ADRs under
   `docs/arc42/adr/`; the durable architecture chapters live under `docs/arc42/`.

## Set up a development machine

The compiler is pinned once, in `rust-toolchain.toml`: stable 1.99.1 with
`rustfmt`, `clippy`, `llvm-tools-preview` and the `wasm32-unknown-unknown` and
`x86_64-pc-windows-gnu` targets. Every installer reads that file; there is no
second channel to keep in step.

- **Linux or macOS.** `./scripts/setup-dev.sh` installs rustup when it is
  missing and runs `rustup show`, which installs the pinned toolchain, its
  components and its targets. It then installs the pinned cargo tools, Node
  from `web/e2e/.nvmrc`, the browser test packages and Chromium. It never uses
  root and never installs an operating-system package: a missing host library or
  package is named instead. `--check` reports without changing anything.
- **NixOS or `x86_64-linux` with Nix.** `nix develop` resolves the same file
  through rust-overlay and provides the toolchain, the pinned cargo tools,
  `wasm-pack`, Node, the Playwright browser build and the Windows cross
  compiler. The npm client still comes from `npm ci --prefix web/e2e`.
- **Windows native.** Not a development target of this repository; the
  published executable is cross-built on Linux.

| Pin | Where |
|---|---|
| Rust and its LLVM tools | `rust-toolchain.toml` (`llvm-tools-preview` is the compiler's own LLVM) |
| cargo tools | `scripts/dev-tool-versions.env` |
| `wasm-pack` | `web/.wasm-pack-version` |
| Node | `web/e2e/.nvmrc` |
| Playwright and axe | `web/e2e/package-lock.json` |
| OpenTofu | `infra/.opentofu-version` (infrastructure work, not the build shell) |

Coverage uses only the compiler's LLVM tools. `run-tests.sh` and
`measure-coverage.sh` start with `scripts/check-toolchain.sh`, which refuses a
different compiler, a sysroot without `llvm-cov` and `llvm-profdata`, a
non-exact channel, or a set `LLVM_COV` or `LLVM_PROFDATA`. Do not put another
LLVM on `PATH` and do not export those variables; the remedy is `rustup show`,
`rustup component add llvm-tools-preview`, or `nix develop`.

The Windows target links with `x86_64-w64-mingw32-gcc`; install the
`mingw-w64` package on Linux, or use `nix develop`, which supplies the cross
compiler and the libraries the link needs. The browser suite needs
`npm ci --prefix web/e2e` before `./run-tests.sh --browser`; the setup script
does it, and names any missing Chromium host library.

To move the compiler pin, run `./scripts/bump-toolchain.sh
<major.minor.patch>` on a machine with Nix. It edits `rust-toolchain.toml`,
updates the rust-overlay revision in `flake.lock`, and re-runs the guard; an
unresolvable release restores both files. The cargo tools in
`scripts/dev-tool-versions.env` are bumped deliberately, because their output
is byte-compared by the change-log and notices gates.

## Checks to run before you propose a change

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
./run-tests.sh
./run-tests.sh --e2e
bash scripts/license-headers.sh
bash scripts/pii-guard.sh
bash scripts/public-text-check.sh
nix-shell -p actionlint --run 'actionlint .github/workflows/*.yml'
```

Actionlint is also available as the image the CI job uses:
`docker run --rm -v "$PWD:/repo" -w /repo rhysd/actionlint:1.7.12 .github/workflows/*.yml`.

The full CI set additionally runs `cargo deny check`, `reuse lint`, the
third-party notices drift check (`cargo about generate about.hbs
--output-file THIRD-PARTY-NOTICES.md` committed with no diff) and the coverage
scripts (`./measure-coverage.sh`, `./merge-coverages.sh`). A change does not land
while any of them fails.

Every source file carries an SPDX licence header on line 1 (or on line 2 after a
shebang), naming MPL-2.0 in the standard SPDX licence tag.

## Privacy: never attach a real duty plan

`duty2cal` converts a crew member's own plan, and a plan is personal data.
Therefore:

- never commit, upload or attach a real duty plan, a calendar converted from
  one, or a personal itinerary — not in an issue, not in a pull request, not as a
  test fixture;
- the issue forms and the pull-request template ask whether the plan
  converted, what the refusal said, and for a **stripped sample**: the page
  structure with every real name, employee number, date, station and route
  removed or replaced;
- fixtures, examples and golden calendars in the repository are generated or
  anonymised and follow the project's fictional 2098/2099 date convention;
  `scripts/pii-guard.sh` checks both the tree and the published history for
  plan-shaped material;
- a proposed change names the destination station codes and the dates it
  exercises, so the behaviour is reviewable without anyone's real plan.

A defect that could expose private plan data does not belong in a public issue:
use the confidential route in [SECURITY.md](SECURITY.md).

## Issues and pull requests

- Bug reports and proposals use the forms under
  [`.github/ISSUE_TEMPLATE/`](.github/ISSUE_TEMPLATE/).
- A pull request states what changed, which scenario carries it, whether the
  plan converted and what the refusal said, the dates it exercises, and confirms
  that no real plan is attached.

## Code of conduct

Participation is governed by the
[Contributor Covenant 2.1 code of conduct](CODE_OF_CONDUCT.md). Conduct concerns
go to the maintainer by direct message, as the code of conduct states.

## License

Contributions are licensed under MPL-2.0, the project's licence
([LICENSES/MPL-2.0.txt](LICENSES/MPL-2.0.txt)).
