# Tasks: generate-release-notes

Release note (commit subject): Record what each released version of the tool changed in a change log a reader can scan, group the released changes by kind, and refuse to publish a version whose notes say nothing a user can see.

Owning use case: [Publish a public release](specs/publish-public-release/usecase.md) (`publish-public-release`). Confirmed Personas: `crew-member--personal-calendar-owner` (reader lens: whether the notes let them tell, from the notes alone, whether a version contains something they want), `project-maintainer--release-owner` (author lens: whether a release can be prepared and published without hand-editing any text, and whether the page and the repository can ever disagree). Actors in scenarios stay distinct from Personas.

Single-scenario run command (used by every RED/GREEN/REFACTOR below): `./run-tests.sh --name "<scenario title>"`.
Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`.

Use-case constraints carried into every scenario task: the section is rendered from the repository's history by the project's own generator, whose version is recorded beside its configuration (`cliff.toml`), and `scripts/render-changelog.sh` refuses a generator whose version differs; the capability allow-list is the use-case ids under `givn/specs/`, repeated per type in `cliff.toml`, and a capability the repository holds and the configuration leaves out is refused; a section is dated with the released revision's own committer date and, where an earlier version exists, carries the literal comparison `https://github.com/kressnerd/duty2cal/compare/<previous>...<this>`, none for the first version; identical release-note subjects collapse to one entry; a version whose notes would name no visible change is refused; a render replaces its own version's section and prepends a version not yet present, keeping every earlier section verbatim; and, under `--check`, a committed change log missing the released version's section, or differing from a fresh render, is refused.

## Setup task — generator pin, configuration, fixtures

- [ ] Record the pinned generator where the design says: `cliff.toml` at the repository root carries the pinned version on the line `# git-cliff version: 2.14.2` and the capability allow-list on the line marked `<!-- capability-allow-list: … -->`, which `scripts/render-changelog.sh` reads. Both are the design's own mechanism, so no second pin file is invented.
- [ ] Confirm the environment: `git-cliff` 2.14.2 through `nix-shell -p git-cliff` (crates.io `max_version` and nixpkgs agree), Rust through `nix-shell -p rustc cargo`, and that the suite runs the renderer as a real subprocess with no network, no credentials and no GitHub. Record the pinned version and the confirmed suite counts here.
- [ ] Add the `DutyWorld` fields the new steps need (fixture repository, change log path, command transcript as the render scenario's evidence) in `apps/cli/tests/features_runner.rs`; the step files already declared in `apps/cli/tests/steps/mod.rs` and `apps/cli/tests/steps/e2e/mod.rs` are extended, never replaced (one step-definition file per capability).
- [ ] Build the fixture repositories inside the steps: `git init`, commits with fixed `GIT_COMMITTER_DATE`/`--date` values and conventional subjects, `v*` tags, and a copy of the committed `cliff.toml` plus the capability directories under `givn/specs/` that the allow-list check reads. No real repository and no clock are involved.

## Interaction coverage (checked before writing scenarios)

| Inventory entry (usecase.md) | Matrix row (design.md) | `@e2e` scenario in the delta | Driver |
|---|---|---|---|
| generate the change log for a version | present, one row | `Maintainer generates the change log for a version and reads its newest section` | real subprocess: `scripts/render-changelog.sh --version 0.1.0 --repo <fixture repo> --out <path>` |
| prepare the release for a version | present, one row | `Maintainer prepares a release and runs the archived executable` (permanent) | real subprocess: `scripts/prepare-release.sh`, then the executable extracted from the produced archive |

Every inventory entry has a matrix row, the row names a real subprocess driver, and the `@e2e` title matches the feature file exactly. No `@e2e` scenario exists that the inventory does not name.

## Scenarios (non-@e2e first, in feature order)

Each scenario's production code is `scripts/render-changelog.sh` (new) and, for the two preparation scenarios, `scripts/prepare-release.sh` (changed); `cliff.toml` (new) supplies the grouping, the skip rules and the allow-list. No Rust production code changes in this change — the only Rust added is step definitions.

### 1. A version's notes group the changes a reader can see

Use-case constraint: a new capability groups above a fix; work no user can see is listed nowhere.

- [ ] RED — remove `@wip` from this scenario only; write the step definitions with `unimplemented!(...)` bodies; run the single-scenario command; paste the non-zero output:
  ```
  ```
- [ ] GREEN — the renderer, the configuration and the fixture; run the single-scenario command; paste the passing output. List every production file touched:
  ```
  ```
- [ ] REFACTOR — tidy without changing behaviour; run again; paste the output:
  ```
  ```
- [ ] COMMIT: `<hash>` — subject is the release note, body names this scenario.

### 2. A version that changes nothing a user can see is refused

Use-case constraint: a version whose notes would name no change a user can see is refused rather than published.

- [ ] RED — remove `@wip` from this scenario only; write the steps with `unimplemented!(...)` bodies; paste the non-zero output:
  ```
  ```
- [ ] GREEN — the empty-section refusal; paste the passing output. List every production file touched:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT: `<hash>` — subject is the release note, body names this scenario.

### 3. The change log keeps the earlier versions beside the new one

Use-case constraint: a render replaces its own version's section and places a new version's section above the earlier ones, which stay verbatim.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — the replace-or-prepend write; paste the passing output. List every production file touched:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT: `<hash>` — subject is the release note, body names this scenario.

### 4. A change log that does not match the released revision is refused

Use-case constraint: a committed change log differing from a fresh render of the released revision is refused, naming the version whose section differs.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — `--check`'s render-and-compare; paste the passing output. List every production file touched:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT: `<hash>` — subject is the release note, body names this scenario.

### 5. Each version's section is dated and compared with the one before it

Use-case constraint: a section states its own date (the released revision's own committer date) and the comparison against the immediately preceding version.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — the date stamp and the literal comparison link; paste the passing output. List every production file touched:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT: `<hash>` — subject is the release note, body names this scenario.

### 6. The first released version is recorded from the history

Use-case constraint: the first version of all renders the whole history it contains, one section, no comparison against an earlier version.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — the first-release range and the heading date; paste the passing output. List every production file touched:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT: `<hash>` — subject is the release note, body names this scenario.

### 7. Maintainer generates the change log for a version and reads its newest section (`@e2e`)

Use-case constraint: the assertion is on the change log the real command wrote and the newest section it reported; the transcript is the committed evidence.

- [ ] Confirm `./run-tests.sh --e2e` counts strictly fewer scenarios than the full run and that strict mode holds; bring up the environment named in the design (the pinned generator on `PATH`, `git`, `tar`, `gzip`, `sha256sum`).
- [ ] RED — remove `@wip`; write the e2e steps with `unimplemented!(...)` bodies; run the single-scenario command; paste the non-zero output:
  ```
  ```
- [ ] GREEN — drive the real renderer as a subprocess; commit the transcript to `givn/changes/generate-release-notes/evidence/visual/maintainer-generates-the-change-log-for-a-version-and-reads-its-newest-section/transcript.txt` with the run-specific directory replaced by a placeholder; paste the passing output. List every production file touched:
  ```
  ```
- [ ] REFACTOR — paste the output, and the `./run-tests.sh --e2e` run:
  ```
  ```
- [ ] COMMIT: `<hash>` — subject is the release note, body names this scenario.

### 8. The release text states the released revisions' release notes (`@givn.modified`)

Use-case constraint: the permanent scenario is restated against the new rule — the release text is the newest section of the change log, grouped by kind, one entry per released change however many revisions carry it, and nothing a user cannot see.

- [ ] RED — remove `@wip`; run the single-scenario command; paste the non-zero output (the two scenarios share the title, so the full-suite count is pasted alongside it):
  ```
  ```
- [ ] GREEN — switch `scripts/prepare-release.sh` from the flat subject list to the rendered newest section and stage the change log; paste the passing output. List every production file touched:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT: `<hash>` — subject is the release note, body names this scenario.

### 9. The prepared release carries the change log its section came from

Use-case constraint: the preparation stages `CHANGELOG.md`, whose newest section is the release text, so the page and the repository cannot disagree.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — staging the change log beside the release text; paste the passing output. List every production file touched:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT: `<hash>` — subject is the release note, body names this scenario.

### 10. A capability missing from the configured allow-list is refused

Use-case constraint: the configuration names every capability the repository holds; one present under `givn/specs/` and absent from it is refused rather than left out of every release.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — the allow-list-versus-corpus refusal; paste the passing output. List every production file touched:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT: `<hash>` — subject is the release note, body names this scenario.

### 11. A committed change log missing the released version's section is refused

Use-case constraint: the change log is committed before the version's tag; a committed change log that lacks the released version's section is refused and the version is named.

- [ ] RED — paste the non-zero output:
  ```
  ```
- [ ] GREEN — the `--check` missing-section refusal; paste the passing output. List every production file touched:
  ```
  ```
- [ ] REFACTOR — paste the output:
  ```
  ```
- [ ] COMMIT: `<hash>` — subject is the release note, body names this scenario.

## Non-scenario work (the change log, the preparation, the pipeline and the public surface)

Not observable through a Gherkin scenario; each is verified by running it or by the host's own run, and recorded as evidence.

- [ ] `cliff.toml` at the repository root: the capability allow-list repeated inside each type's parser (`feat`, `fix`, `perf`, `revert` and each breaking-change case), the group order (Breaking changes, New capabilities, Fixes, Performance, Reversals), the skip rules for housekeeping (`chore`, `docs`, `style`), `test`, `build`, `ci` and any scope outside the allow-list, the literal comparison URL built in the template from the two versions, and no `[remote.github]` integration. Evidence: the render scenarios and the drift check.
- [ ] `CHANGELOG.md` at the repository root, generated with the pinned tool — never hand-written — with the `0.1.0` section back-filled under the new rules: it holds about five bullets rather than the published page's thirty-seven, because identical subjects collapse. Evidence: `nix-shell -p git-cliff --run 'scripts/render-changelog.sh --version 0.1.0 --repo . --out CHANGELOG.md'` and `--check` clean; the bullet count pasted.
- [ ] `.github/workflows/ci.yml`: a `changelog` job (pushes and pull requests) that checks out with `fetch-depth: 0`, installs `git-cliff@2.14.2` with `taiki-e/install-action@v2` and runs `scripts/render-changelog.sh --version <newest v* tag> --check`; the `test` job installs the pinned generator on `PATH` because the suite's steps drive the renderer as a real subprocess. Every existing job keeps its name and steps. Evidence: `nix-shell -p actionlint --run 'actionlint .github/workflows/*.yml'` clean.
- [ ] `.github/workflows/release.yml`: `staged/CHANGELOG.md` joins the `gh release create` asset list so the draft carries the change log among its assets. Evidence: the same actionlint run; the staged-versus-draft comparison still matches, since `release-notes.md` stays excluded from it.
- [ ] `README.md`: the Development section records the one extra release step — render the version's section, commit the change log, then push the tag — without disturbing the givn-managed blocks. Evidence: `givn readme check` and `bash scripts/public-text-check.sh` clean.

## Final verification (the delta's eleven scenarios green)

- [ ] Full suite — `nix-shell -p rustc cargo --run './run-tests.sh'`:
  ```
  ```
- [ ] E2E suite — `nix-shell -p rustc cargo --run './run-tests.sh --e2e'`:
  ```
  ```
- [ ] Corpus lint — `givn lint --change generate-release-notes` (exit 0 or 2):
  ```
  ```
- [ ] Workflow definitions — `nix-shell -p actionlint --run 'actionlint .github/workflows/*.yml'`:
  ```
  ```
- [ ] Public text and private data — `bash scripts/public-text-check.sh` and `bash scripts/pii-guard.sh`:
  ```
  ```
- [ ] Licence compliance — `nix-shell -p reuse --run 'reuse lint'`:
  ```
  ```
- [ ] The drift check on the committed change log — `nix-shell -p git-cliff --run 'scripts/render-changelog.sh --version 0.1.0 --repo . --out CHANGELOG.md --check'`:
  ```
  ```
- [ ] The pinned generator's version, recorded where the design says (`cliff.toml`, `# git-cliff version: 2.14.2`), and the version the render actually ran:
  ```
  ```

## Evidence pointers

- Transcript for the `@e2e` scenario: `evidence/visual/maintainer-generates-the-change-log-for-a-version-and-reads-its-newest-section/transcript.txt`.
- Host-side evidence (the `changelog` job's first run, the released version's draft carrying `CHANGELOG.md`) is recorded in `review.md` during the review phase, as the archived change recorded its own host-side evidence.
