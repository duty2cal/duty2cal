# Tasks: publish-public-release

Release note (commit subject): Publish duty2cal from one public repository where every change passes the project's checks before it lands there and every version tag offers a downloadable, provenance-verifiable executable for Windows, Linux and macOS.

Owning use case: [Publish a public release](specs/publish-public-release/usecase.md) (`publish-public-release`). Confirmed Personas: `project-maintainer--release-owner` (release-evidence lens, the Actor's review lens), `crew-member--personal-calendar-owner` (accuracy/privacy lens for what a downloader receives). Actors in scenarios stay distinct from Personas.

Single-scenario run command (used by every RED/GREEN/REFACTOR below): `./run-tests.sh --name "<scenario title>"`.
Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`.

Use-case constraints carried into every scenario task: exactly one archive per supported platform (Windows x86_64, Linux x86_64, Linux arm64, macOS arm64, macOS x86_64) and never one promised without an archive; an incomplete, unrecognised, unreadable or already-prepared set stages nothing; a published release is immutable and published draft-first, so a preparation never replaces what exists; the release text carries the released revisions' release-note subjects, omits housekeeping, and never fails on a commit message; private duty plans and real identifying values never enter the public history or a public asset.

## Setup task — runner, strict mode, step files for this capability

- [x] The Gherkin runner is already configured from the previous change: `apps/cli/tests/features_runner.rs` is a `harness = false` cucumber-rs target with `.fail_on_skipped()`, reading its feature root from `GIVN_FEATURES`, which `scripts/assemble-features.sh` builds from `givn/specs/**` and `givn/changes/*/specs/**`. Strict mode was proven then (an undefined step exits non-zero); it is re-proven by this change's first RED run.
- [x] Two new step files, one per capability as `design.md` requires, plus their module declarations: `apps/cli/tests/steps/publish_public_release.rs` and `apps/cli/tests/steps/e2e/publish_public_release.rs`, declared in `apps/cli/tests/steps/mod.rs` and `apps/cli/tests/steps/e2e/mod.rs`.
- [x] `verify.command` (`./run-tests.sh`) and `verify.e2e_command` (`./run-tests.sh --e2e`) stay as configured; they are distinct commands and the e2e run is a strict subset of the full run.

## Interaction coverage (checked before writing scenarios)

| Inventory entry (usecase.md) | Matrix row (design.md) | `@e2e` scenario in the delta | Driver |
|---|---|---|---|
| prepare the release for a version | present, one row | `Maintainer prepares a release and runs the archived executable` | real subprocess: `scripts/prepare-release.sh`, then the executable extracted from the produced archive |

Every inventory entry has a matrix row, the row names a real subprocess driver, and the `@e2e` title matches the feature file exactly. No `@e2e` scenario exists that the inventory does not name.

## Scenarios (non-@e2e first, in feature order)

Each scenario's production code is `scripts/prepare-release.sh` (extended as needed); no Rust production code changes in this change.

### 1. A prepared release offers one archive for every supported platform

Use-case constraint: one archive per supported platform, named for the platform, holding only that platform's executable, plus a checksum covering the archive.

- [x] RED — remove `@wip` from this scenario only; write the step definitions with `unimplemented!(...)` bodies where real work is missing; run the single-scenario command; paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: A prepared release offers one archive for every supported platform
     ✘  Given an executable built for every supported platform
        Step panicked. Captured output: not implemented: build an executable for every supported platform
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
  Step file created in RED: `apps/cli/tests/steps/publish_public_release.rs` (helpers plus the four stubs), declared in `apps/cli/tests/steps/mod.rs`; `apps/cli/tests/steps/e2e/publish_public_release.rs` and its declaration in `apps/cli/tests/steps/e2e/mod.rs`; `DutyWorld` gained the fields the capability's steps need (`binaries`, `staged`, `fixture_repo`, `command_transcript`, `scenario_transcript`, `staged_snapshot`).
- [x] GREEN — real assertions; create `scripts/prepare-release.sh` (version, binaries directory, output directory, repository); run the single-scenario command; paste the passing output:
  ```
  Feature: Publish a public release
    Scenario: A prepared release offers one archive for every supported platform
     ✔  Given an executable built for every supported platform
     ✔  When the maintainer prepares the release for version 0.1.0
     ✔  Then the prepared release holds exactly these archives, each with its checksum file
         | archive                              | executable   |
         | duty2cal-0.1.0-windows-x86_64.tar.gz | duty2cal.exe |
         | duty2cal-0.1.0-linux-x86_64.tar.gz   | duty2cal     |
         | duty2cal-0.1.0-linux-arm64.tar.gz    | duty2cal     |
         | duty2cal-0.1.0-macos-arm64.tar.gz    | duty2cal     |
         | duty2cal-0.1.0-macos-x86_64.tar.gz   | duty2cal     |
     ✔  And every archive holds only its own executable
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
  Production code: `scripts/prepare-release.sh` — the canonical platform set, the argument contract, the completeness/whitelist/readability/occupied refusals (all before any write), the staging directory moved into `<out>` only when complete, the deterministic `tar … | gzip -n` payload, `sha256sum` beside each archive, and the release text. The steps assert the exact staged set (five archives, five checksum files, the release text) and that each archive lists exactly its own executable via `tar -tzf`.
- [x] REFACTOR — tidy the script and the steps without changing behaviour; run again; paste the output:
  ```
  rustfmt --edition 2021 --check apps/cli/tests/steps/publish_public_release.rs -> clean
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
  Two format-neutral rewrites in `fresh_dir` and `run_prepare` so the file is stable under rustfmt 1.98 (the toolchain in this environment), and the not-yet-used `machine_executable` helper removed. Honest tooling note: `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` do not run clean here, but every finding is in files this change does not touch (`crates/duty2cal-pdf/src/lib.rs`, `apps/cli/tests/steps/support.rs`, `apps/cli/tests/steps/export_duty_plan_calendar.rs`); rustfmt 1.98 and clippy 1.98 are newer than whatever formatted them, and the repository pins `channel = "stable"`.
- [x] COMMIT — `80cefb4` — subject is the release note, body names this scenario:
  ```
  feat(publish-public-release): Publish duty2cal from one public repository where every change passes the project's checks before it lands there and every version tag offers a downloadable, provenance-verifiable executable for Windows, Linux and macOS.

  Scenario: A prepared release offers one archive for every supported platform

  RED: the four steps existed as unimplemented!() stubs; the single-scenario run failed with exit 101 under strict mode.
  GREEN: scripts/prepare-release.sh stages one deterministic archive and one checksum per supported platform plus the release text, refusing an incomplete set, a platform outside the set, an unreadable executable and an occupied output set before any write; the steps assert the exact staged set and each archive's payload.
  REFACTOR: format-neutral rewrites so the step file is stable under rustfmt 1.98 and the unused helper removed; the single scenario still passes 4/4 steps with exit 0.
  ```

### 2. A platform whose executable was not built stops the release

Use-case constraint: an archive set missing a platform publishes nothing.

- [x] RED — paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: A platform whose executable was not built stops the release
     ✘  Given an executable built for every supported platform except "linux-arm64"
        Step panicked. Captured output: not implemented: build every supported platform except linux-arm64
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — the completeness refusal, leaving the output directory untouched; paste the output:
  ```
  Feature: Publish a public release
    Scenario: A platform whose executable was not built stops the release
     ✔  Given an executable built for every supported platform except "linux-arm64"
     ✔  When the maintainer prepares the release for version 0.1.0
     ✔  Then the preparation fails and names "linux-arm64"
     ✔  And the prepared release holds no archive
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
  Production code: none new — `scripts/prepare-release.sh`'s completeness refusal already names the platform whose executable is missing and exits before any write (`prepare-release: no executable was built for platform linux-arm64: <path>`), which this scenario pins; the `holds_no_archive` step asserts the staged directory is still empty, so "nothing is staged" is checked rather than assumed.
- [x] REFACTOR — paste the output:
  ```
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
  The `except` Given now asserts that the name it leaves out is a supported platform (a typo would otherwise quietly build five), and `the_preparation_fails_and_names`/`holds_no_archive` needed no change.
- [x] COMMIT — `9378bea`:
  ```
  feat(publish-public-release): Publish duty2cal from one public repository where every change passes the project's checks before it lands there and every version tag offers a downloadable, provenance-verifiable executable for Windows, Linux and macOS.

  Scenario: A platform whose executable was not built stops the release

  RED: the three steps existed as unimplemented!() stubs; the single-scenario run failed with exit 101.
  GREEN: the completeness refusal in scripts/prepare-release.sh names the platform whose executable is missing and exits before any write; the steps assert the failure names linux-arm64 and that the staged directory is still empty.
  REFACTOR: the except-Given asserts the name it leaves out is a supported platform; the single scenario passes 4/4 steps with exit 0.
  ```

### 3. An executable for an unrecognised platform is refused

Use-case constraint: no invented platform is ever published.

- [x] RED — paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: An executable for an unrecognised platform is refused
     ✔  Given an executable built for every supported platform
     ✘  And an executable presented as the unrecognised platform "freebsd-x86_64"
        Step panicked. Captured output: not implemented: present an executable as freebsd-x86_64
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — the platform whitelist refusal; paste the output:
  ```
  Feature: Publish a public release
    Scenario: An executable for an unrecognised platform is refused
     ✔  Given an executable built for every supported platform
     ✔  And an executable presented as the unrecognised platform "freebsd-x86_64"
     ✔  When the maintainer prepares the release for version 0.1.0
     ✔  Then the preparation fails and names "freebsd-x86_64"
     ✔  And the prepared release holds no archive
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  GREEN_EXIT=0
  ```
  Production code: none new — the whitelist refusal in `scripts/prepare-release.sh` walks the binaries directory's subdirectories before anything else and refuses any name outside the canonical set (`prepare-release: unsupported platform: freebsd-x86_64 (supported: …)`); the Given presents the executable under a name the script does not know, and the existing `names`/`no archive` steps assert the refusal and the untouched output.
- [x] REFACTOR — paste the output:
  ```
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 5 steps (5 passed)   REFACTOR_EXIT=0
  ```
  The Given now asserts the name it presents really is outside the supported set, so the step cannot pass by presenting a supported platform twice.
- [x] COMMIT — `3016612`:
  ```
  feat(publish-public-release): Publish duty2cal from one public repository where every change passes the project's checks before it lands there and every version tag offers a downloadable, provenance-verifiable executable for Windows, Linux and macOS.

  Scenario: An executable for an unrecognised platform is refused

  RED: the new Given existed as an unimplemented!() stub; the single-scenario run failed with exit 101.
  GREEN: the platform whitelist refusal in scripts/prepare-release.sh refuses a binaries-directory subdirectory outside the canonical set before any write; the Given presents an executable as freebsd-x86_64 and the scenario asserts the failure names it and stages nothing.
  REFACTOR: the Given asserts the name it presents is outside the supported set; the single scenario passes 5/5 steps with exit 0.
  ```

### 4. Each archive's checksum covers that archive

Use-case constraint: a downloader verifies what they received before extracting it, and the checksum names the file it covers.

- [x] RED — paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: Each archive's checksum covers that archive
     ✘  Given a prepared release for version 0.1.0
        Step panicked. Captured output: not implemented: prepare a release for version 0.1.0
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — `sha256sum` output beside each archive, verified with `sha256sum --check`; paste the output:
  ```
  Feature: Publish a public release
    Scenario: Each archive's checksum covers that archive
     ✔  Given a prepared release for version 0.1.0
     ✔  When anyone verifies the checksum against the archive for "linux-x86_64"
     ✔  Then the checksum matches the archive it names
  [Summary]
  1 feature
  1 scenario (1 passed)
  3 steps (3 passed)
  GREEN_EXIT=0
  ```
  Production code: none new — the script writes `sha256sum`'s own output beside each archive with the archive's own name, from inside the staged directory, so the recorded name is the file a downloader holds. The When runs `sha256sum --check` with the staged directory as its working directory; the Then asserts exactly one `<archive>: OK` line, that the named file is staged, that the checksum file names that archive, and that a freshly computed digest equals the recorded one.
- [x] REFACTOR — paste the output:
  ```
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 3 steps (3 passed)   REFACTOR_EXIT=0
  ```
  No tidy was needed: the steps read as one command per step, `sha256_of` is the only new helper, and the file is rustfmt-clean; the checks above are the whole REFACTOR.
- [x] COMMIT — `84c1648`:
  ```
  feat(publish-public-release): Publish duty2cal from one public repository where every change passes the project's checks before it lands there and every version tag offers a downloadable, provenance-verifiable executable for Windows, Linux and macOS.

  Scenario: Each archive's checksum covers that archive

  RED: the three steps existed as unimplemented!() stubs; the single-scenario run failed with exit 101.
  GREEN: the script writes sha256sum's own output beside each archive from inside the staged directory; the When verifies it with sha256sum --check and the Then asserts the checksum file names that archive and that a freshly computed digest equals the recorded one.
  REFACTOR: no tidy needed; the step file is rustfmt-clean and the single scenario passes 3/3 steps with exit 0.
  ```

### 5. Preparing the same release twice produces the same archives

Use-case constraint: byte-identical repeat preparation, so a third party can reproduce a release.

- [x] RED — paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: Preparing the same release twice produces the same archives
     ✔  Given a prepared release for version 0.1.0
     ✘  When the maintainer prepares version 0.1.0 again from the same executables
        Step panicked. Captured output: not implemented: prepare version 0.1.0 a second time
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — fixed tar ownership, ordering and time, and `gzip -n`; paste the output:
  ```
  Feature: Publish a public release
    Scenario: Preparing the same release twice produces the same archives
     ✔  Given a prepared release for version 0.1.0
     ✔  When the maintainer prepares version 0.1.0 again from the same executables
     ✔  Then the archives are identical to the first preparation, byte for byte
  [Summary]
  1 feature
  1 scenario (1 passed)
  3 steps (3 passed)
  GREEN_EXIT=0
  ```
  Production code: none new — the script already builds every archive as `tar --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner -cf - <exe> | gzip -n`, so ownership, ordering, time and the gzip header are fixed. The When prepares the same version a second time from the same executables into a second directory; the Then compares every archive byte for byte.
  Independent proof of the same property, from the real built executable (`target/debug/duty2cal`) as this machine's platform and stand-ins for the others, with two runs into two directories:
  ```
  $ ( cd A && sha256sum *.tar.gz ) | sort
  5ff653f32223b3c844e6c351caccf0b67ced728f0a8629a940149beb2321c859  duty2cal-0.1.0-linux-arm64.tar.gz
  5ff653f32223b3c844e6c351caccf0b67ced728f0a8629a940149beb2321c859  duty2cal-0.1.0-linux-x86_64.tar.gz
  5ff653f32223b3c844e6c351caccf0b67ced728f0a8629a940149beb2321c859  duty2cal-0.1.0-macos-arm64.tar.gz
  5ff653f32223b3c844e6c351caccf0b67ced728f0a8629a940149beb2321c859  duty2cal-0.1.0-macos-x86_64.tar.gz
  9ade484bb7139806d7f31821929b49b1319b29188927b8fa64ecb795e2e6c0a9  duty2cal-0.1.0-windows-x86_64.tar.gz
  $ ( cd B && sha256sum *.tar.gz ) | sort
  5ff653f32223b3c844e6c351caccf0b67ced728f0a8629a940149beb2321c859  duty2cal-0.1.0-linux-arm64.tar.gz
  5ff653f32223b3c844e6c351caccf0b67ced728f0a8629a940149beb2321c859  duty2cal-0.1.0-linux-x86_64.tar.gz
  5ff653f32223b3c844e6c351caccf0b67ced728f0a8629a940149beb2321c859  duty2cal-0.1.0-macos-arm64.tar.gz
  5ff653f32223b3c844e6c351caccf0b67ced728f0a8629a940149beb2321c859  duty2cal-0.1.0-macos-x86_64.tar.gz
  9ade484bb7139806d7f31821929b49b1319b29188927b8fa64ecb795e2e6c0a9  duty2cal-0.1.0-windows-x86_64.tar.gz
  TWO RUNS: IDENTICAL DIGESTS
  BYTE COMPARISON DONE
  ```
  (The four non-Windows archives share one digest because the proof fed the same executable under each of their names; the Windows archive differs because its member is `duty2cal.exe`.)
- [x] REFACTOR — paste the output:
  ```
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 3 steps (3 passed)   REFACTOR_EXIT=0
  ```
  No behaviour change: the second preparation's read is a `match` rather than a closure around `panic!`, which keeps the file rustfmt-clean under both the older and the current formatter.
- [x] COMMIT — `1aba314`:
  ```
  feat(publish-public-release): Publish duty2cal from one public repository where every change passes the project's checks before it lands there and every version tag offers a downloadable, provenance-verifiable executable for Windows, Linux and macOS.

  Scenario: Preparing the same release twice produces the same archives

  RED: the two steps existed as unimplemented!() stubs; the single-scenario run failed with exit 101.
  GREEN: the fixed tar ownership, ordering and time plus gzip -n make a repeated preparation byte-identical; the When prepares the version a second time into a second directory and the Then compares every archive byte for byte. A two-run shell proof from the real built executable is pasted in tasks.md with both digest lists.
  REFACTOR: the second read is a match instead of a closure around panic!, keeping the step file rustfmt-clean; the single scenario passes 3/3 steps with exit 0.
  ```

### 6. An already prepared version is not replaced

Use-case constraint: a preparation never replaces what exists; the staged archives keep their earlier modification times.

- [x] RED — paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: An already prepared version is not replaced
     ✔  Given a prepared release for version 0.1.0
     ✘  When the maintainer prepares version 0.1.0 in the same place again
        Step panicked. Captured output: not implemented: prepare version 0.1.0 again in the same place
  [Summary]
  1 feature
  1 scenario (1 failed)
  2 steps (1 passed, 1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — the occupied-set refusal; paste the output:
  ```
  Feature: Publish a public release
    Scenario: An already prepared version is not replaced
     ✔  Given a prepared release for version 0.1.0
     ✔  When the maintainer prepares version 0.1.0 in the same place again
     ✔  Then the preparation fails and names version 0.1.0
     ✔  And the staged archives keep their earlier modification times
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
  Production code: none new — the occupied-set refusal in `scripts/prepare-release.sh` runs before any write and names the version (`prepare-release: the release for version 0.1.0 is already prepared in <out>: duty2cal-0.1.0-windows-x86_64.tar.gz exists`). The When records every staged archive's modification time before the repeated preparation; the Then asserts each is unchanged, which is the assertion that distinguishes "refused" from "silently rewritten" — byte-identical archives would not (the design review's F7).
- [x] REFACTOR — paste the output:
  ```
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
  No behaviour change: the two modification-time helpers are used by this scenario only and the file is rustfmt-clean.
- [x] COMMIT — `cb1af7a`:
  ```
  feat(publish-public-release): Publish duty2cal from one public repository where every change passes the project's checks before it lands there and every version tag offers a downloadable, provenance-verifiable executable for Windows, Linux and macOS.

  Scenario: An already prepared version is not replaced

  RED: the three steps existed as unimplemented!() stubs; the single-scenario run failed with exit 101.
  GREEN: the occupied-set refusal in scripts/prepare-release.sh runs before any write and names the version; the When records every staged archive's modification time and the Then asserts each is unchanged, which distinguishes a refusal from a silent rewrite.
  REFACTOR: no tidy needed; the step file is rustfmt-clean and the single scenario passes 4/4 steps with exit 0.
  ```

### 7. An unreadable executable stops the preparation

Use-case constraint: an unreadable input stages nothing and names the cause.

- [x] RED — paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: An unreadable executable stops the preparation
     ✘  Given an executable built for the platform "linux-x86_64" that cannot be read
        Step panicked. Captured output: not implemented: build an unreadable executable for linux-x86_64
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — the read failure path; paste the output:
  ```
  Feature: Publish a public release
    Scenario: An unreadable executable stops the preparation
     ✔  Given an executable built for the platform "linux-x86_64" that cannot be read
     ✔  When the maintainer prepares the release for version 0.1.0
     ✔  Then the preparation fails and names "linux-x86_64"
     ✔  And the prepared release holds no archive
  [Summary]
  1 feature
  1 scenario (1 passed)
  4 steps (4 passed)
  GREEN_EXIT=0
  ```
  Production code: none new — the script checks each platform's executable for readability after completeness and before any write (`prepare-release: the executable for platform linux-x86_64 cannot be read: <path>`). The Given builds all five platforms and removes every permission bit from the named one, so the readability check — not the completeness check — is the one that must fire.
- [x] REFACTOR — paste the output:
  ```
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 4 steps (4 passed)   REFACTOR_EXIT=0
  ```
  No behaviour change: the permission change is one `#[cfg(unix)]` block with an explicit `#[cfg(not(unix))]` refusal, since the preparation itself is Linux-only.
- [x] COMMIT — `30681d5`:
  ```
  feat(publish-public-release): Publish duty2cal from one public repository where every change passes the project's checks before it lands there and every version tag offers a downloadable, provenance-verifiable executable for Windows, Linux and macOS.

  Scenario: An unreadable executable stops the preparation

  RED: the new Given existed as an unimplemented!() stub; the single-scenario run failed with exit 101.
  GREEN: the script's readability check names the platform whose executable cannot be read, after completeness and before any write; the Given builds all five platforms and removes every permission bit from linux-x86_64, so the readability check is the one that fires.
  REFACTOR: the permission change is one #[cfg(unix)] block with an explicit #[cfg(not(unix))] refusal; the single scenario passes 4/4 steps with exit 0.
  ```

### 8. The release text states the released revisions' release notes

Use-case constraint: release-note subjects in release order, housekeeping and unrecognised subjects omitted, repeated subjects collapsed, the version named, and no preparation failure caused by a commit message.

- [x] RED — paste the non-zero output:
  ```
  Feature: Publish a public release
    Scenario: The release text states the released revisions' release notes
     ✘  Given a history whose released revisions carry release-note subjects, including one subject repeated by several revisions and one subject that carries no release-note type, and whose housekeeping revisions do not
        Step panicked. Captured output: not implemented: build a history of release-note, housekeeping and untyped subjects
  [Summary]
  1 feature
  1 scenario (1 failed)
  1 step (1 failed)
  RED_EXIT=101
  ```
- [x] GREEN — `release-notes.md` written from the repository's history, with the fixture repository built by the steps; paste the output:
  ```
  Feature: Publish a public release
    Scenario: The release text states the released revisions' release notes
     ✔  Given a history whose released revisions carry release-note subjects, including one subject repeated by several revisions and one subject that carries no release-note type, and whose housekeeping revisions do not
     ✔  When the maintainer prepares the release for version 0.2.0
     ✔  Then the release text names version 0.2.0
     ✔  And the release text lists these release notes in release order
         | release note                                  |
         | Offer an archive for every supported platform |
         | Publish only an archive the evidence supports |
     ✔  And the release text omits the housekeeping revisions and the subject that carries no release-note type
  [Summary]
  1 feature
  1 scenario (1 passed)
  5 steps (5 passed)
  GREEN_EXIT=0
  ```
  Production code: `scripts/prepare-release.sh`'s release-text collection — the range is the previous `v*` tag reachable from `HEAD^` or every revision reachable from `HEAD` when there is none, the type is read from `<type>(<scope>): `, release-note types are quoted, housekeeping types and untyped subjects are dropped, identical notes collapse to one occurrence, and the file names the version. The Given builds the fixture history with `git init` plus eight revisions (two release-note subjects, one of them repeated three times, three housekeeping subjects, one subject with no type); `--repo` points the preparation at it. What the script actually wrote for the same history, reproduced outside the steps:
  ```
  $ git -C history log --format='%s'
  feat(release): Offer an archive for every supported platform
  test(release): cover the refusal paths
  Publish the notes by hand
  fix(release): Publish only an archive the evidence supports
  feat(release): Offer an archive for every supported platform
  docs(release): describe the download routes for readers
  feat(release): Offer an archive for every supported platform
  chore(release): bootstrap the release channel
  $ cat out/release-notes.md
  # duty2cal 0.2.0

  Release notes for version 0.2.0.

  - Offer an archive for every supported platform
  - Publish only an archive the evidence supports
  ```
  Interpretation recorded: "in release order" is read as the order the revisions entered the release, so the range is walked oldest first (`git log --reverse`) — the table in the scenario is written in that order and the assertion fails if the order is reversed. Collapsing is by the release note the reader sees, which is what makes a repeated subject appear once.
- [x] REFACTOR — paste the output:
  ```
  apps/cli/tests/steps/publish_public_release.rs FMT OK
  1 scenario (1 passed), 5 steps (5 passed)   REFACTOR_EXIT=0
  ```
  No behaviour change: the fixture's `git` helper no longer returns a transcript no caller reads, and the file is rustfmt-clean.
- [ ] COMMIT — `<hash>`:
  ```
  ```

### 9. Maintainer prepares a release and runs the archived executable (`@e2e`)

Use-case constraint: the assertion is on what the preparation command reported and on what the executable taken out of the archive printed; the transcript is the committed evidence.

- [ ] Bring up the local environment named in `design.md` and confirm `./run-tests.sh --e2e` counts strictly fewer scenarios than the full run:
  ```
  ```
- [ ] RED — remove `@wip`; write the e2e steps with `unimplemented!(...)` bodies; run `./run-tests.sh --e2e --name "<title>"`; paste the non-zero output:
  ```
  ```
- [ ] GREEN — drive the real command and the extracted executable; commit the transcript to `givn/changes/publish-public-release/evidence/visual/maintainer-prepares-a-release-and-runs-the-archived-executable/transcript.txt` with the run-specific directory replaced by a placeholder; paste the passing output:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT — `<hash>`:
  ```
  ```

## Non-scenario work (host automation and the public surface)

Not observable through a Gherkin scenario; each is verified by running it or by the host's first run, and recorded as evidence.

- [ ] `.github/workflows/release.yml`: the five jobs `design.md` names — build matrix (five platforms), `smoke-windows`, `prepare`, `publish` (check-run assertion, SBOM, attestations, draft, publish), `verify` (read-back, checksums, `gh attestation verify`, `immutable: true`), with least-privilege permissions per job.
- [ ] `.github/workflows/ci.yml`: the `actionlint` job (`rhysd/actionlint:1.7.12`), the `dco` job (pull requests only), and the existing action uses bumped to `checkout@v7`, `upload-artifact@v7`, `download-artifact@v8`.
- [ ] `scripts/dco-check.sh`: the sign-off check over a commit range, proven on a range with and without a `Signed-off-by` trailer.
- [ ] `scripts/public-text-check.sh`: the README promises (clone address, the five archive names, the checksum and provenance commands, the licence statement and link, the Unicode attribution), wired into `ci.yml`.
- [ ] Public files: `README.md` (clone address, download and verification section including the unsigned-macOS approval, badges, sign-off enforcement), `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, `.github/ISSUE_TEMPLATE/{bug,proposal,config}.yml`, `.github/pull_request_template.md`, `.github/SECURITY.md`.
  - Operator question from the proposal's D2 and the design's ledger — **answered 2026-10-13**: the conduct route is a direct message to `@kressnerd@chaos.social`, and `SECURITY.md` points at the repository's private vulnerability reporting for exposure reports. Write both files accordingly.
- [ ] Repository hygiene: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check`, `reuse lint`, `bash scripts/license-headers.sh`, `bash scripts/pii-guard.sh`, `cargo about generate about.hbs --output-file THIRD-PARTY-NOTICES.md` with no drift, `./measure-coverage.sh && ./merge-coverages.sh`.
- [ ] `givn readme` completeness check after the README edit.
- [ ] Record the operator-applied settings checklist from `design.md` as the pending evidence for the review phase (the settings themselves are applied in the publish phase, after the review).

## Evidence pointers

- Transcript for the `@e2e` scenario: `evidence/visual/maintainer-prepares-a-release-and-runs-the-archived-executable/transcript.txt`.
- The history rewrite and its mapping: `evidence/history-rewrite.md`.
- Host-side evidence (the first CI run, the first tagged release and its `verify` job, the operator's settings confirmation) is recorded in `review.md` during the review phase.
