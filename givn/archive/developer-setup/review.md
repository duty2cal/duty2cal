# Review: developer-setup

Operational maintenance change; `specs` is skipped in the change's
`.givn-skip`. No `.feature` file, step definition or product behaviour is
added, amended or removed.

## Fabrication audit

- **Delta specs and `@e2e` tags:** no delta `.feature` file exists, so no tag
  was added, removed or downgraded.
- **Empty step bodies:** 0 step-definition files changed; the existing
  `apps/cli/tests/features_runner.rs` and step files are untouched. 0 empty
  bodies introduced across 0 changed files.
- **Checked tasks:** 8/8 tasks carry a commit whose diff touches the
  implementation, not only the change record:
  `724e97d9b653dc14ef3bdcebb5eb0a14ee8d7587` adds the pin, guard, bootstrap,
  flake, CI actions/workflows and documentation;
  `4fbe39e20e94d43eab55204b8e66f66a8119f169` adds the Playwright refusal,
  the dry-run package list and the CI component assertions.
- **Promised components:** `scripts/check-toolchain.sh`,
  `scripts/setup-dev.sh`, `scripts/dev-tool-versions.env`,
  `scripts/tests/{check-toolchain,setup-dev}.bats`, `flake.nix`/`flake.lock`,
  `.github/actions/{rust-toolchain,install-dev-tools}/action.yml` and the four
  rewired workflows all exist.
- **Strict mode:** not applicable — no scenario is added. The existing
  `apps/cli/tests/features_runner.rs:80` `.fail_on_skipped()` is unchanged and
  the full suite ran 140/140.
- **Design conformance:** the implementation matches the design's named
  commands, files and mechanisms: the composite action runs `rustup show` then
  `scripts/check-toolchain.sh`; the guard refuses the exact channel, a
  different `rustc`, the sysroot LLVM tools and the two environment overrides;
  the bootstrap uses no package manager; the flake names
  `pkgsCross.mingwW64.windows.pthreads`/`.mcfgthreads` and exports
  `NIX_LDFLAGS_x86_64_w64_mingw32`; the Playwright mismatch refusal is in both
  the script and the shell.
- **E2E scope:** `verify.command` = `./run-tests.sh`, `verify.e2e_command` =
  `./run-tests.sh --e2e`; they are different strings and no `@e2e` scenario
  changed. The opposite classification (Web UI) does not hold: the change
  adds terminal scripts, not a rendered page.
- **Interaction coverage:** no capability and no `@e2e` inventory delta; no
  matrix row is claimed.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Pinned compiler, components, targets, guard | ch2 | Yes | "Compiler pin", "Coverage guard" | Task 1, 2, 3 | `rust-toolchain.toml`; `scripts/check-toolchain.sh` | Yes |
| Language/runtime row names the pin | ch4 | Yes | "Compiler pin" | Task 1 | `docs/arc42/04` row; `rust-toolchain.toml` | Yes |
| Build/test block view lists the new files | ch5 | Yes | file table | Tasks 1-5 | all listed files exist | Yes |
| Bootstrap and refusal runtime flow | ch6 | Yes | "Local runnability", "Coverage guard" | Tasks 3-5 | `run-tests.sh`, `measure-coverage.sh`, scripts | Yes |
| Developer-machine platforms and installers | ch7 | Yes | "Non-Nix installer", "Nix installer" | Tasks 4, 5 | `setup-dev.sh`, `flake.nix` | Yes |
| Compiler-owned LLVM, env-override refusal | ch8 | Yes | "LLVM for coverage", "Coverage guard" | Tasks 2, 3 | `check-toolchain.sh`; bats test | Yes |
| No new ADR; routing to ch2 | ch9 | Yes | "Documentation" | — | no `adr/ADR-0009`; ch9 record | Yes |
| Toolchain reproducibility quality scenario | ch10 | Yes | "Coverage guard" | Tasks 3, 6 | 140/140 suite; coverage run | Yes |
| Lock coupling, Playwright coupling | ch11 | Yes | "Risks and limits" | Tasks 4, 5 | `flake.nix`, `setup-dev.sh`, R-080/R-081 | Yes |
| Pinned toolchain and toolchain guard terms | ch12 | Yes | "Documentation", "Coverage guard" | — | glossary rows | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Ubiquitous language conformance

**Pinned toolchain** and **Toolchain guard** are the glossary terms
(`docs/arc42/12-glossary.md`); the proposal, design, tasks, chapters and
scripts use "the pin", "the guard script `scripts/check-toolchain.sh`" and the
two terms consistently. No term is used inconsistently; no new term is
unrecorded.

## Coverage

Most recent measured evidence: `./measure-coverage.sh` under the pinned
toolchain wrote both Cobertura reports; `./merge-coverages.sh` merged 13
files, 2,155 lines, 1,880 covered. The change adds no instrumented Rust line
(shell scripts, YAML, Nix and Markdown only), so no new uncovered region is
introduced. The pre-existing uncovered lines are owned by earlier
classifications (for example TD-007) and are not reclassified here.

`./run-tests.sh` (in `nix develop`) → 140 scenarios / 592 steps passed;
`cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
`scripts/pii-guard.sh`, `scripts/license-headers.sh`,
`scripts/public-text-check.sh` and `actionlint .github/workflows/*.yml` all
exit 0; `bats scripts/tests` → 9/9 passed; the Windows cross build inside the
shell produced a 841,728-byte PE executable.

## Overlap dispositions

No delta scenario exists, so there is no shape-match pair to disposition.

## Split-or-keep

No scenario is longer than the gate threshold; no row.

## README impact

`README-IMPACT: updated - Prerequisites`

## Use-case and Persona conformance

Operational maintenance change: no use case, no capability, no product
surface. No Persona is invented or promoted; no visual design contract is
declared (the change renders no screen), so the confirmed-Persona requirement
does not apply. The operator requested the developer setup on 2026-10-29.

## Deferrals

None. The proposal's consistent-wording promise is delivered in
`givn/specs/reduce-distributed-executable-size/usecase.md` without a behaviour
delta.

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching production files.
- [x] Every promised component exists.
- [x] Strict-mode proof: not applicable, no scenario added; the existing
  `.fail_on_skipped()` is unchanged.
- [x] Scenario execution evidence recorded (140/140; coverage reports).
- [x] Coverage classification uses measured evidence; no new gap.
- [x] Dead code deleted where the change made it dead (none); missing tests
  added as the two bats files; no hard-to-test gap claimed.
- [x] No redundant unit tests added; no `@wip` tags.
- [x] No implementation-layer detail in a spec; no spec delta.
- [x] E2E policy unchanged and re-read; no inventory delta.
- [x] `verify.e2e_command` differs from `verify.command`; no `@e2e` tag changed.
- [x] The local run command (`./run-tests.sh` after setup/`nix develop`) starts
  the full local suite; there are no external services or digital twins.
- [x] The design's named commands, files and mechanisms match the
  implementation; the one post-review design correction was re-reviewed
  (`design-review.md`, delta section) and passed.
- [x] Interaction coverage: no inventory delta.
- [x] No finding is excused outside the three coverage buckets.

REVIEW: PASS
