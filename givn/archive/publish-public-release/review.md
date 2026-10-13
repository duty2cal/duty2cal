# Review: publish-public-release

Stage: `tasks-done` → `review`. This pass covers the release preparation, the host automation and the public surface; the design layer's own record is in `design-review.md`.

## Step 0 — fabrication audit

| # | Check | Method | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | `git log -S"@e2e" -- <delta feature>`; `grep -n "@e2e" <delta feature>` | CLEAN — the tag has been touched by exactly one revision (the specs commit) and the scenario still carries it. Nine `@givn.added` scenarios, one `@e2e`. |
| 1 | Empty or no-op step bodies | `grep -rn 'unimplemented!\|todo!()' apps/cli/tests/steps/` | CLEAN — no match in any step file. |
| 2 | Every `[x]` task has a committed revision touching real work | `git log` for the nine scenario revisions and their evidence revisions; the non-scenario work in `80cefb4` and `6d170d8` | CLEAN, with one process note: a path-less `givn commit` at 08:21:55 swept the host-automation slice's then-final files and one of my own uncommitted edits into a sibling's scenario-1 revision, so `80cefb4` contains more than its own scenario. The content is exactly what the tasks record and each file is named in its tasks evidence; no work is uncommitted. |
| 3 | Every component `design.md` promised exists | `ls` over the promised paths | CLEAN — `scripts/prepare-release.sh`, `scripts/dco-check.sh`, `scripts/public-text-check.sh`, `.github/workflows/release.yml`, the community files, `docs/arc42/adr/ADR-0003`/`ADR-0004`, the two step files, both `mod.rs` declarations, the transcript. |
| 4 | Proof-of-strictness present with a non-zero exit | The runner's `.fail_on_skipped()` (proven in the previous change) plus this change's nine RED runs, each `Step doesn't match any function`, exit 101 | CLEAN — no scenario was ever GREEN by a stub. |
| 5 | Every `@e2e` Then asserts through the real interface | `apps/cli/tests/steps/e2e/publish_public_release.rs` + the committed transcript | CLEAN — the assertions are on what `scripts/prepare-release.sh` printed (staged archive and checksum, the release text's version) and on what the executable extracted from the produced archive printed, with its exit status. |
| 6 | Browser-UI driver fidelity | `design.md`'s `## Visual Design Contract` declares `Interface: terminal`; no HTML/CSS/JS is produced or served by this project | NOT APPLICABLE, and the opposite classification was tested: the release and repository pages are rendered by GitHub, not by this codebase, so the text this change authors for them is checked as a document contract by `scripts/public-text-check.sh`, not driven by a browser. |
| 7 | The file `verify.e2e_command` actually invokes | `givn/commands.yaml` → `./run-tests.sh --e2e` → `cargo test --test features_runner -- --tags @e2e` | CLEAN — one `@e2e` step implementation for this capability; no second, weaker implementation anywhere in the tree, committed or not. |
| 8 | Canonical action normalization and E2E scope | `givn instructions specs` re-read | CLEAN — one inventory entry, one `@e2e` scenario, no variant promoted to `@e2e`. |
| 9 | Local run command starts the whole stack | `design.md`'s Local Runnability: the preparation command plus the product CLI; nothing to orchestrate | CLEAN — nothing to start; no server, database or twin exists (and none is required, because the only thing that talks to the host is the host's own automation). |
| 10 | `verify.command` ≠ `verify.e2e_command`, and the e2e run is a strict subset | `./run-tests.sh` → 2 features, 84 scenarios; `./run-tests.sh --e2e` → 2 features, 2 scenarios | CLEAN — different commands, strictly fewer scenarios. |
| 11 | Implementation vs the reviewed design | Read `release.yml`, `ci.yml`, the scripts and the step files against `design.md`'s file table, pipeline table and step-definition locations | CLEAN — one step file per capability, one preparation command driven by both the suite and the pipeline, the job set and its order as reviewed. Two deliberate details recorded below (the bill of materials travels in the attestation rather than as a separate asset, and the packaging job is the only place that runs the script). |
| 12 | Findings reopened as unfinished implementation | — | The two gate failures found during implementation were repaired as revisions (`fix(toolchain)`, the guard's precision fix), not excused. |
| 13 | Interaction coverage cross-reference | Below | MATCH |
| 14 | Coverage measurement validity | Below | VALID |

### Interaction coverage cross-reference

| Inventory entry (`usecase.md`) | Matrix row (`design.md`) | `@e2e` scenario in the delta | Driving mechanism in the step file |
|---|---|---|---|
| prepare the release for a version | present, exactly one row | `Maintainer prepares a release and runs the archived executable` | real subprocess: `std::process::Command` runs the real `scripts/prepare-release.sh`, then extracts the produced archive with `tar` and runs the executable taken out of it |

The delta `.feature` carries no `# User Interaction Inventory:` comment block: this project keeps the inventory in the use-case document, which is what the canonical specs instruction requires and what `givn lint` compares. No missing, extra or over-produced entry.

### Coverage measurement (check 14)

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| features_runner (non-e2e) | `./measure-coverage.sh` | `cargo llvm-cov --workspace --test features_runner --tags "not @e2e"` → 82 scenarios | `coverage/non-e2e-cobertura.xml` (fresh run written to a temporary path to keep the tracked reports clean) | `./merge-coverages.sh` | `crates/duty2cal-core/src/application/mod.rs` 288/289 |
| E2E runner + the spawned CLI | `./measure-coverage.sh` | the same runner under instrumentation, spawning the built executable → 2 scenarios | `coverage/e2e-cobertura.xml` | `./merge-coverages.sh` | `apps/cli/src/main.rs`, whose `main` and `parse_stamp` appear only in this report |

VALID: both reports are freshly produced by the configured commands from distinct instrumentation runs (different Cobertura timestamps 1791873974 / 1791873997, different mtimes, different per-file rates — e.g. `application/mod.rs` 288/289 non-e2e against 172/289 e2e), the merged report is freshly written by `merge-coverages.sh` (7 files, 889 lines, 874 covered → line-rate 98.31%), and production coverage is present, not runner-only: `crates/duty2cal-core/src/domain/timing.rs` 60/60 and `apps/cli/src/main.rs` 49/50 among others. No branch entries are emitted by this toolchain (the merged report's `branches-valid` is 0), which is the same shape as the tracked report the README's badge links; the line data is what carries the reading.

## Coverage classification (15 uncovered production lines)

All fifteen are inherited: this change adds no production line in any crate. The evidence is arithmetic from the two measurements — the tracked report from the archive counts 880 instrumented lines with 865 covered, the fresh run counts 889 with 874 — so the change's own nine instrumented lines (its step-definition code) are all covered, and the fifteen uncovered lines are the same code the previous change shipped.

| Range | Bucket | Disposition |
|---|---|---|
| `crates/duty2cal-pdf/src/lib.rs:288` | 3 — hard to test | The early return for a table with no `Date` band. The reader establishes the family from the header anchors and rejects a source without them before this code is reachable, so the line guards a shape the supported family cannot present. |
| `crates/duty2cal-pdf/src/lib.rs:365` | 3 — hard to test | The closing brace of the `if let Some(index)` in `assign_cells`, counted when `areas` is empty. `areas` is non-empty by construction once the header is established. |
| `crates/duty2cal-pdf/src/lib.rs:383` | 3 — hard to test | `Date::from_calendar_date(…).ok()?` in the footer's compact-date fallback: it needs a footer whose day is out of range for a validly spelled month, e.g. `31Feb2099`. The behaviour is already specified and covered ("A revision footer with an unusable stamp supplies no revision"); this finer arm is not reachable through the producer's own spelling. |
| `crates/duty2cal-pdf/src/lib.rs:389` | 3 — hard to test | The non-UTC zone return. The family prints `(UTC)` in this footer; the existing scenario "A revision footer naming no UTC zone supplies no revision" covers the behaviour through the missing-token path. |
| `crates/duty2cal-pdf/src/lib.rs:407, 408` | 3 — hard to test | The month-index and day conversions inside `compact_date`, reachable only by a footer naming an out-of-range month or day, which the producer's spelling excludes; the behaviour they guard is covered by the same unusable-stamp scenario. |
| `crates/duty2cal-core/src/domain/mod.rs:179–181` | 3 — hard to test | The `date()` match arms for the travel, availability and non-flight variants. Every one of those variants is exercised; the accessor itself is only asked for a date by the all-day and layover paths, so the arms are an exhaustive-match artifact rather than a behaviour of their own. |
| `apps/cli/src/fs.rs:47` | 3 — hard to test | The retry arm for a candidate claimed between the existence check and the create. Reaching it needs a second process to interleave in that window; the design names it as the mitigation for a lost race. |
| `apps/cli/src/fs.rs:54` | 3 — hard to test | The exhaustion return for the bounded enumeration (10 000 occupied candidates). Reachable in principle, not worth 10 000 fixtures; the occupied-name behaviour itself is covered. |
| `crates/duty2cal-core/src/adapters/ics/mod.rs:88, 89` | 3 — hard to test | Escaping a raw newline and dropping a carriage return. The supported family's cells cannot carry either, so no fixture can produce them without hand-building the intermediate representation. |
| `apps/cli/src/main.rs:27` | 3 — hard to test | The fall-through when the construction-clock seam is set but unparsable. The seam is a test affordance; both of its real outcomes (a usable instant, and the system clock) are covered. |
| `crates/duty2cal-core/src/application/mod.rs:402` | 2 — missing coverage | The arm where two compatible fragments supply the *same* selected optional value, so it is represented once. Unlike the rows above, an existing scenario builds compatible fragments and could be extended with a repeated value; it is a conversion-capability gap, and closing it means a scenario in `export-duty-plan-calendar`, not in this change. |

Fifteen lines, fourteen judged legitimately hard to test with the reasons above and one recorded as missing coverage in another capability. The conversion capability's reader guards deserve the same treatment the previous review gave them, so the gap is recorded as technical debt (TD-007 in chapter 11) with the follow-up named rather than left implicit: a change that hunts the reader's uncovered defensive arms, extends the fixtures that can reach them, and deletes the ones that cannot be reached at all.

## Visual review

Interface: terminal, so the evidence is the committed transcript rather than screenshots. One fresh-context reviewer per confirmed Persona read the transcript, the use case, the contract and the public download route.

PERSONAS: project-maintainer--release-owner
The capability's second confirmed Persona, `crew-member--personal-calendar-owner`, is dispositioned the same way and reviewed separately in the table below.

| Persona | Verdict | Findings (all advisory) |
|---|---|---|
| `project-maintainer--release-owner` | `PERSONA-REVIEW: PASS` | A1: four of the five staged archives wrap stand-in payloads locally, so their checksums attest stand-ins; the transcript claims nothing about them, and the real per-platform evidence is the host pipeline's build matrix and smoke job. A2: the version is shown in two of its three loci (archive names, release text); the tag is host-side by design. A3: the host half of the release promises has no recorded run yet, which the change names as the pipeline's own evidence rather than asserting it. |
| `crew-member--personal-calendar-owner` (as a downloader) | `PERSONA-REVIEW: PASS` | The archive-selection, extraction, checksum, provenance, stable-address, macOS-approval and privacy routes all conform. Advisory: the verification block named no Windows command (fixed in this phase — `Get-FileHash -Algorithm SHA256` is now given); no download command is given because the release page is the host's interface; the transcript shows the preparation's digest report rather than the checksum file's contents; and the transcript demonstrates that the shipped executable runs, not that a duty plan converts, which the conversion use case owns and already proves. |

`VISUAL-REVIEW: PASS` — the evidence kind is complete for a terminal interface, the contract's baseline holds (every staged archive and its checksum and the release text are named, the run-specific directory is a placeholder, and no internal identifier such as a temporary path, runner name or local user appears), and every Persona finding is dispositioned.

## Arc42 implementation conformance

Independently derived from the finished implementation and compared with the chapters and `arc42.md`.

| Chapter | Claim | Implementation evidence | Verdict |
|---|---|---|---|
| 01 | Requirement 5 is one self-contained executable per supported platform, published as a verifiable immutable release; the behaviour specification points at the merged corpus | the five-platform set in `scripts/prepare-release.sh` and the workflow matrix; the corpus pointer now names `givn/specs/export-duty-plan-calendar/` | MATCH |
| 02 | The five targets, musl-static Linux, native macOS builds, the Linux-only preparation script, actionlint and the DCO check, attestations, the public repository and the immutable-release constraints | `release.yml`'s matrix and per-platform linkers; `ci.yml`'s `actionlint` and `dco` jobs; `.cargo/config.toml` unchanged | MATCH |
| 03 | GitHub is the first external system, one-way, and the product keeps no network I/O | nothing in the product's crates references the network; only the workflows do | MATCH |
| 04 | The release strategy: build per platform, package once deterministically, draft-first immutable publication with provenance, read-back verification | `release.yml`'s five jobs; `prepare-release.sh`'s determinism flags; `gh release create --draft` then `edit --draft=false` | MATCH |
| 05 | The release toolbox outside the product crates | `scripts/prepare-release.sh`, `scripts/dco-check.sh`, `scripts/public-text-check.sh`, the two workflows, the community files, the two step files | MATCH |
| 06 | The tag-to-verified-release flow and the pushed-change-to-gated-branch flow | `release.yml`'s job graph; `ci.yml`'s job set that the ruleset requires | MATCH |
| 07 | Five artefacts on five runners, the download and verification route, the stable address, the unsigned-macOS approval | the matrix's runner labels and targets; the README's download and verification sections | MATCH |
| 08 | Determinism, completeness refusal, provenance and immutability, secret handling, version/tag/archive-name consistency | the `tar`/`gzip -n` flags and the refusal paths; the attestation steps; `prepare-release.sh`'s bare-version refusal | MATCH |
| 09 | ADR-0003 and ADR-0004 registered beside ADR-0001 and ADR-0002 | both records exist, both registers list all four | MATCH |
| 10 | QS-007 to QS-010 | the four quality scenarios exist and trace to the delta's rules and scenarios | MATCH |
| 11 | R-009 to R-014, TD-005 and TD-006 | the rows exist; R-011's enumeration is the corrected one; R-014 records the guard's limit | MATCH |
| 12 | Sixteen release-channel terms in the language table, the two mis-tabled rows moved | the language table holds them, the lifecycle table holds event rows only, the promotion count matches | MATCH |

ARC42 CONFORMANCE: CLEAN.

## Ubiquitous language conformance

Every term the delta and the design use normatively is in `docs/arc42/12-glossary.md`: supported platform, release archive, release preparation, release text, release-note subject, housekeeping revision, checksum, build provenance, software bill of materials, immutable release, release attestation, gate (on landing), default branch, public home, version tag, stable latest-release address. `scripts/prepare-release.sh` and the workflows use the same words as the use case ("prepared release", "staged", "supported platform", "release text"); no term appears with a competing meaning. No finding.

## Overlap dispositions

| Scenario A | Scenario B | Decision |
|---|---|---|
| A platform whose executable was not built stops the release | An executable for an unrecognised platform is refused | variant |
| A platform whose executable was not built stops the release | An unreadable executable stops the preparation | variant |
| An executable for an unrecognised platform is refused | An unreadable executable stops the preparation | variant |

Each pair is a **variant**, not a duplicate. The three refusals share a shape but pin different invariants — the completeness refusal (no platform promise without an archive), the whitelist refusal (no invented platform) and atomicity (an unreadable input stages nothing) — and each failure stays separately attributable in the message. Merging them would let a regression in one hide behind a pass in another.

No removed+added pairs: the delta adds nine scenarios and removes none. No scenario in the delta is long enough to warrant a split-or-keep row; each asserts one behaviour in three to five steps.

README-IMPACT: updated - Description, Quick start, Prerequisites, Usage, Contributing, License
(the clone address, the download-and-verify route with the macOS approval, the badges, the sign-off enforcement and the licence statement; the three givn-managed blocks are untouched and `givn readme check` reports OK).

- Second refusal, same phase: the merged coverage report could not be parsed by the gate, because `scripts/merge-cobertura.py` omitted the root `branches-covered` and `branches-valid` attributes the Cobertura parser requires (the two source reports carry them; the merge built a fresh root without them). Both attributes are now written on the root and the package element, and the same run's measurement and merge pass unchanged: two distinct instrumented runs merged into 7 files, 889 lines, 874 covered.

## Sign-off checklist

- [x] `./run-tests.sh` GREEN — 2 features, 84 scenarios (84 passed), 312 steps.
- [x] `./run-tests.sh --e2e` GREEN — 2 features, 2 scenarios (2 passed), 12 steps.
- [x] Coverage measured and merged by the configured commands, from two distinct instrumentation runs, with production coverage present.
- [x] Every uncovered line classified; the one missing-coverage line belongs to another capability and is recorded as TD-007 with its follow-up named.
- [x] Fabrication audit clean.
- [x] Interaction coverage MATCH.
- [x] Arc42 conformance CLEAN; ubiquitous language clean.
- [x] Visual review complete with both Persona verdicts and every finding dispositioned.
- [x] Deterministic gates: `cargo fmt --all --check` clean, `cargo clippy --workspace --all-targets -- -D warnings` no warnings, `cargo deny check` advisories/bans/licenses/sources ok, `reuse lint` compliant with REUSE 3.3, `bash scripts/license-headers.sh` clean, `bash scripts/pii-guard.sh` clean over the tree and every revision, third-party notices with no drift, `actionlint` 1.7.12 clean over both workflows, `scripts/public-text-check.sh` clean, `givn lint` clean (advisories only).

## Host-side evidence

- **The repository holds the project.** 2026-10-13: the history was pushed to `kressnerd/duty2cal` (`+ 4de3b32...bbbf7f main -> main (forced update)`), replacing the bootstrap commit; `refs/heads/main` is `bbbf7f2`. Nothing is published publicly: the repository is still private, so no promise has left the operator's account yet.
- **The first CI run on `bbbf7f2` was red in three jobs, all latent in a workflow that had never executed** (the project had no remote until now): `test` (a bare `cargo test --workspace` leaves the Gherkin runner looking for `tests/features`, which `./run-tests.sh` assembles), `notices` (`cargo install cargo-about --locked` did not provide the command), and `windows-gnu-build` (the non-system-DLL allowlist matched `api-ms-win-` only when immediately followed by `.dll`, and lacked `bcryptprimitives`, a Windows system DLL the GNU-linked executable imports). Nine jobs were green (`fmt`, `clippy`, `deny`, `reuse`, `license-headers`, `pii-guard`, `coverage`, `actionlint`, `public-text`), and `dco` was skipped as designed. All three are fixed in `1ff1f65`, verified by actionlint and by running the corrected allowlist over exactly the imports the runner reported.
- **What the operator applied** (2026-10-13): release immutability; classic branch protection on `main` (linear history, no force pushes) — a ruleset is inert on a user-owned private repository, so the required-check version waits for the public flip; dependency graph, automated dependency submission, Dependabot alerts, malware alerts, security updates and grouped security updates; read-only workflow permissions with pull-request creation off; the About description and the suggested topics. Code scanning and secret scanning are not offered on a private repository on the free plan; both are enabled after the flip.
## The first release, verified from outside

- **Cut.** The checks on `8102571` completed successfully and `v0.1.0` was pushed there; the tag was re-pointed from `e1eb3ab`, whose release run had failed (nothing was published, so no immutable record was replaced).
- **Published.** `https://github.com/kressnerd/duty2cal/releases/tag/v0.1.0` — not a draft, **immutable: true**, published 2026-10-13T18:31:22Z, ten assets: the five platform archives and their five checksums, about 1 MB each.
- **Verified from outside the pipeline.** Every published `.sha256` verifies against its archive (`sha256sum -c`, five of five); the public API reports the same digests; three attestations exist for the live archive digest — SLSA build provenance (`https://slsa.dev/provenance/v1`, builder `.github/workflows/release.yml@refs/tags/v0.1.0`, subjects the five archives), the SPDX bill of materials (`https://spdx.dev/Document/v2.3`) and the release attestation that immutability issues (`https://in-toto.io/attestation/release/v0.2`). The pipeline's own `verify` job — checksum comparison, `gh attestation verify` per archive, `immutable: true` — concluded successfully.
- **Two defects the pipeline's first real run exposed, both fixed.** First, the `publish` job's SBOM step failed before anything was published: `anchore/sbom-action` writes its file but does not create missing parents, and it was given an empty `artifact-name`; fixed in `8102571`, after which the run went green through `publish` and `verify`. Second, the published archives store the executable as `-rw-r--r--`, so a Linux or macOS downloader must `chmod +x` before the tool runs: CI's artifact transport does not carry file permissions, so `tar` stored the mode its input arrived with. Fixed in `fdb5764` with `--mode=755`, proved by packaging a 0644 input and running the executable extracted from the result. `v0.1.0` keeps the flaw, because an immutable release's assets cannot be replaced; the fix reaches users with the next version, or through the documented `chmod`.
- **One quality finding the release exposed.** The generated notes list fifteen subjects of which two matter to a downloader; the rest are internal CI, coverage, guard and tooling work that I typed `fix(...)` when the project's convention is that such work is `chore(...)`. The rule is sound but unguarded: nothing fails a release whose notes carry no user-visible change. The neighbouring `watn` checkout answers that precisely — `git-cliff` pinned at 2.13.1 with a `cliff.toml`, a maintained `CHANGELOG.md`, a guard that refuses a release whose generated section has no user-visible change, every action pinned by commit SHA with a version comment, and a scheduled security workflow — and a follow-up change should adopt the changelog generation with a scope-aware configuration, that guard, and the SHA pins (which closes R-011 and TD-005). Its release is `workflow_dispatch`-driven, which this change decided against (D5), so whether a release can be re-run is a decision that change must take for itself.
- **The evidence the review owed is complete**: the tag, the green run it was cut from, the release, its assets, the checksums, the attestations and the operator-applied settings. One settings row remains outstanding: the ruleset on `main` enforces deletions, force pushes and linear history but does not yet require the status checks that report on a push.

REVIEW: PASS
