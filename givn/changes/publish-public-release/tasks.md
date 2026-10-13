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

- [ ] RED — remove `@wip` from this scenario only; write the step definitions with `unimplemented!(...)` bodies where real work is missing; run the single-scenario command; paste the non-zero output:
  ```
  ```
- [ ] GREEN — real assertions; create `scripts/prepare-release.sh` (version, binaries directory, output directory, repository); run the single-scenario command; paste the passing output:
  ```
  ```
- [ ] REFACTOR — tidy the script and the steps without changing behaviour; run again; paste the output:
  ```
  ```
- [ ] COMMIT — `<hash>` — subject is the release note, body names this scenario:
  ```
  ```

### 2. A platform whose executable was not built stops the release

Use-case constraint: an archive set missing a platform publishes nothing.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — the completeness refusal, leaving the output directory untouched; paste the output:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT — `<hash>`:
  ```
  ```

### 3. An executable for an unrecognised platform is refused

Use-case constraint: no invented platform is ever published.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — the platform whitelist refusal; paste the output:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT — `<hash>`:
  ```
  ```

### 4. Each archive's checksum covers that archive

Use-case constraint: a downloader verifies what they received before extracting it, and the checksum names the file it covers.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — `sha256sum` output beside each archive, verified with `sha256sum --check`; paste the output:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT — `<hash>`:
  ```
  ```

### 5. Preparing the same release twice produces the same archives

Use-case constraint: byte-identical repeat preparation, so a third party can reproduce a release.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — fixed tar ownership, ordering and time, and `gzip -n`; paste the output:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT — `<hash>`:
  ```

### 6. An already prepared version is not replaced

Use-case constraint: a preparation never replaces what exists; the staged archives keep their earlier modification times.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — the occupied-set refusal; paste the output:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT — `<hash>`:
  ```
  ```

### 7. An unreadable executable stops the preparation

Use-case constraint: an unreadable input stages nothing and names the cause.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — the read failure path; paste the output:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT — `<hash>`:
  ```
  ```

### 8. The release text states the released revisions' release notes

Use-case constraint: release-note subjects in release order, housekeeping and unrecognised subjects omitted, repeated subjects collapsed, the version named, and no preparation failure caused by a commit message.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — `release-notes.md` written from the repository's history, with the fixture repository built by the steps; paste the output:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
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
