# Design review: publish-public-release

Fresh-context grilling of the complete plan (proposal, spec delta, design, arc42 assessment), then hardening. The grilling subagent read every planning artifact, the corpus and archive for house style, the code the scenarios will drive (`apps/cli/tests/features_runner.rs`, `apps/cli/tests/steps/**`, `scripts/*`, `.github/workflows/ci.yml`, `.cargo/config.toml`, `Cargo.toml`), the Gherkin runner's own source, and the three instructions (`specs`, `design-review`, `arc42-docs`), and independently re-derived the twelve arc42 rows before diffing `arc42.md`.

## Grilling — branches walked

| Branch | Outcome |
|---|---|
| Scope | One gap, blocking in spirit: the proposal's promises about the public text (licence statement and link, the Unicode attribution, the clone address, how to obtain and verify an archive) appeared in no rule, no scenario and no check, while the README still said `git clone <repository-url>`. Fixed (hardening 5). |
| Tech choices | No issue. Each choice carries its rationale and its rejected alternative; the two that could remove behaviour were checked (no single-archive-per-platform invariant without one place that sees all five binaries; the script seam keeps packaging out of the workspace and out of the coverage denominator). |
| Missing scenarios | Two: a first release has no previous tag, so the release-text range was undefined on the critical path of the only `@e2e` scenario; and one real history revision carries no release-note type, so that case was undecided. Both are now rules, and the non-conforming subject is covered by the release-text scenario's history. |
| Testability | No issue, proven by running the runner: `./run-tests.sh --name "An executable for an unrecognised platform is refused"` fails at the scenario (`Step doesn't match any function`, exit 101), so the delta is assembled and each scenario fails as RED rather than as a harness error. Two assertions were too weak to pin their behaviour and were replaced (hardening 4). |
| E2E fidelity | One defect: the `@e2e` scenario's only assertion was on the extracted executable, while the named interface is the preparation command, and the design claimed an assertion on the command's output the feature did not state. Fixed: the scenario now asserts the reported archive and checksum and the version in the release text, and names the exact usage line the binary prints. |
| Visual design contract | No issue: `Interface: terminal`, viewports n/a, the transcript evidence path matches the existing precedent, and the baseline sentence now matches what the scenario asserts. |
| Interface classification | No issue: the opposite classification was tested. Nothing this project builds renders HTML, CSS or JS in a browser; the release and repository pages are rendered by GitHub, and the text this change authors for them is covered as a document contract (hardening 5), not as a UI. |
| Persona disposition | No issue: both Personas are confirmed and promoted with dates and a Handoff record; the design records the relevance and evidence per Persona, the scenarios name a domain role (the maintainer) rather than a biography, and no Persona was invented or promoted. |
| Deferrals | One, and it needed the operator: every host-side promise is checked by GitHub's automation plus the operator's settings confirmation, not by the project's suite. Recorded as `D5` after the operator's decision (question 2 below). |
| Interaction coverage | No issue: the use case carries exactly one inventory entry, the matrix has exactly one row, and the titles match. The boundary note explains why obtaining a file from the host and running the published tool are not this capability's interactions. |
| Risk | The plan named the likeliest failure honestly (the release workflow is provable only on the host). The grilling added the undefined first-release text, the unenforced "checks green on the released revision" and the missing link between the two gate paths, all now closed. |
| Domain context | No issue: one capability, no typed relationships (Includes/Extends both `none`), the conversion use case untouched, Actors distinct from Personas. |
| ADR qualification | No issue on qualification; one on shape: one record bundled four decisions. Split (hardening 3). |
| Architecture documentation | All twelve rows re-derived independently and diffed against `arc42.md`: all twelve agree, no row omitted, no chapter placeholder-only, every diagram Mermaid, chapter 11 mapping each of the ADRs' consequences. Three content findings were fixed (hardening 3). |
| Ubiquitous language | One finding: the release-text rule's own vocabulary (`release-note subject`, `housekeeping revision`) plus `version tag` and `stable latest-release address` were used normatively without a glossary entry; two older rows also sat in the wrong table. Fixed (hardening 3). |

## Grilling — findings resolved from the repository, without operator input

| # | Finding | Severity | Disposition |
|---|---|---|---|
| F1 | The private-data guard failed on the clean tree: its calendar-timestamp strip only removed complete `YYYYMMDDTHHMMSSZ` tokens, so a test holding `DTSTART:20990118T{}00Z` still matched the staff-identifier pattern (10 false positives). | blocking | Fixed and committed: the strip now consumes a time with a placeholder and a date with its separator; the tree is clean, and a planted line with a real timestamp and a staff id still fails. |
| F2 | The guard read only the working tree and its identifier pattern cannot describe a roster, so it could not see the file this change was about to publish. | blocking | Fixed: the guard scans every revision of the published tip and fails on any calendar date-time outside the project's fictional 2098/2099 convention; it named the leaked fixture in five revisions, and the history was rewritten (question 1). |
| F3 | The spec and proposal required the Windows archive to be built on a Windows machine, which the design (and the confirmed cross-build constraint) does not do. | high | Fixed: the rule now says built *for* that platform and started on a machine of that platform's kind, naming the Windows cross-build as the deliberate exception; ADR-0003's confirmation says the same. |
| F4 | The design claimed major-tag pins "matching the existing workflows" while `ci.yml` stays on `v4`, and simultaneously claimed `ci.yml` is untouched — two conventions, and a contradiction inside the plan. | high | Fixed: one convention — `ci.yml`'s existing action uses move to `checkout@v7`, `upload-artifact@v7`, `download-artifact@v8` in the same revision, the release path uses the current majors, and the design says so once. |
| F5 | The release-text rule was undefined for a version with no earlier tag — the case of this change's own first release. | high | Fixed: rule, design row and examples now define it as every revision reachable from the tagged revision. |
| F6 | Release-text extraction was undefined for subjects matching no release-note type, and the real history contains one. | medium | Fixed: such a subject is omitted and never fails a preparation; the release-text scenario's history includes one. |
| F7 | Scenario 6's second assertion ("still holds exactly its five archives") could pass while the staged set was silently rewritten, because repeated preparation is byte-identical by design. | medium | Fixed: it now asserts the staged archives keep their earlier modification times, which differs between runs. |
| F8 | Scenario 1's "holds exactly these archives" collided with the design's own staged output (checksum files and the release text). | medium | Fixed: the scenario says "exactly these archives, each with its checksum file", and the design states that the reported set is exactly five archives, five checksums and the text, with no other archive. |
| F9 | The `@e2e` assertion was not on the interface the inventory names, and the design overstated what it asserted. | medium | Fixed: the scenario asserts the preparation's own report, and the design's matrix row and baseline match it. |
| F10 | Nothing verified that the tagged revision passed the gate, although the release evidence requires it. | medium | Fixed: the `publish` job asserts every check run on the tagged revision concluded successfully and fails otherwise. |
| F11 | The required status-check set would include the pull-request-only sign-off job, making every direct push depend on the bypass. | medium | Fixed: the settings table names the required checks that report on a push, excludes `dco` deliberately, and states the owner-bypass reality and the follow-up. |
| F12 | The pinning risk misstated the third-party surface and claimed immutability makes a substitution detectable. | medium | Fixed in `design.md`, chapter 11, TD-005 and ADR-0003: the enumeration is corrected and the residual risk is stated as accepted, with no detection claim. |
| F13 | The proposal's public-text promises had no rule, scenario or check. | medium | Fixed: use-case rules for the public text, plus `scripts/public-text-check.sh` as a CI job, recorded as a design decision with its alternatives. |
| F14 | One ADR bundled four decisions. | low-medium | Fixed: ADR-0003 is the channel and platform set; ADR-0004 is the immutable draft-first publication; the unsigned-macOS cost choice stays a risk and a design decision, with the cheaper-home reason recorded. |
| F15, F16, F17 | Glossary terms missing; two five-column rows stranded in the lifecycle table; chapter 01 pointing at a change directory that no longer exists. | low-medium | Fixed in the chapter set, with the promotion count corrected. |
| F18 | The file table omitted the two `mod.rs` files without which the new step modules never compile. | low | Fixed. |
| F19 | "No `-sys` crate in `Cargo.lock`" was literally false, though the musl conclusion held. | low | Fixed: the claim now names the release build graph and the two bindings-only crates that are outside it. |
| F20 | The gate enumeration omitted the licence-compliance job and did not name the new checks. | low | Fixed in the use-case rule. |
| F21 | The design said the platform set lives "in the script and nowhere else" and then described the hand-written matrix as a second place. | low | Fixed: the script owns the canonical set, and drift can only fail loudly. |

## Operator questions

### Question 1 — the history this change was about to publish

- Question: the history that the first push would publish contains the operator's own real June-2026 duty plan as a converted calendar; rewrite the history, publish a history-free repository, publish the exposure, or keep the repository private and publish assets only?
- Disposition: **answered 2026-10-13 — rewrite the history.** `git filter-repo --path apps/cli/tests/fixtures/expected/2026-06.ics --invert-paths` removed the blob from the five revisions that carried it; 43 of 98 revisions changed id, the tree of the tip is unchanged, the blob is pruned locally, and the guard is green over the tree and over every revision. Rejected: a history-free repository (it loses the complete history the change promises and leaves the first release's text with nothing to quote), publishing as-is (a real schedule, mirrored permanently), and a private repository (it would drop the public home, the visible gate and the release channel). Evidence: `evidence/history-rewrite.md`, including the mapping for the three archived citations whose ids changed.

### Question 2 — how the host-side promises are verified

- Question: every host-side promise (the gate on landing, secret scanning with push protection, dependency alerts, code scanning, provenance and bill of materials, immutability, the published release) is checked by GitHub's automation plus the operator's settings confirmation rather than by the project's suite — is that boundary accepted, and where is it recorded?
- Disposition: **answered 2026-10-13 — record the boundary as it stands.** Recorded as `D5` in `design.md`, realized in the pipeline's `verify` job, the owner-applied settings table and the use case's "Where this interface ends" rules. Rejected: adding a `workflow_dispatch` rehearsal entry point (it could not exercise the immutability assertion or the published download, and would add a second path to keep honest), and a local fake of the release API (it would prove the shim rather than the host and contradicts the design's own finding that nothing local talks to the release API).

### Question 3 — the contact route in the security policy and the code of conduct

- Question: which address or route should the public security policy and code of conduct name for reports that must not be public?
- Disposition: **carried — owner: the operator; decided in this change's `tasks.md` before those files are written** (the proposal's D2 already records this, and the design's ledger repeats it). The confidentiality route itself is settled: the repository's private vulnerability reporting, which the security policy points at. Not raised as a fresh question because the proposal had already carried it to the same place with the same owner, and no artifact written so far depends on the answer.

## Hardening applied

1. **`scripts/pii-guard.sh`** — placeholder-tolerant calendar strip, the fictional-year rule, and the history scan (F1, F2). Committed on its own, with a positive control.
2. **History rewrite** (question 1), with `evidence/history-rewrite.md`.
3. **ADR split and chapter set** (F12, F14, F15, F16, F17): ADR-0003 and ADR-0004 with their own verdicts, the corrected pinning risk, four added glossary terms, two rows moved into the language table, and the stale chapter-01 pointer.
4. **Spec delta** (F3, F5, F6, F7, F8, F9, F20): the corrected build-and-start rule, the first-release and unrecognised-subject rules, the staged-set wording, the strengthened scenario-6 and `@e2e` assertions, the unrecognised subject in the release-text history, and the gate enumeration.
5. **Design** (F4, F5, F6, F8, F9, F10, F11, F13, F18, F19, F21, `D5`): one pinning convention, the first-release text, the public-text check and its decision, the check-run assertion, the required-check list with the bypass reality, the corrected file table, the corrected dependency claim, and the platform-set ownership sentence.
6. **Use case** (F13, F20): the public-text rule group and the gate enumeration.

## Per-branch coverage after hardening

Scope, Tech choices, Missing scenarios, Testability, E2E fidelity, Visual design contract, Interface classification, Persona disposition, Deferrals (`D5`, recorded), Interaction coverage, Risk, Domain context, ADR qualification, Architecture documentation (twelve rows re-derived, content checked, no placeholder text, Mermaid only), Ubiquitous language: no issue outstanding.

The delta keeps nine scenarios, one of them `@e2e`, all `@wip` until the implementation phase, and the use case's inventory still matches the single matrix row.

`givn lint --change publish-public-release` exits 2 (advisories only: the `@wip` tags and three shape suggestions among the refusal scenarios). `bash scripts/pii-guard.sh` is clean over the tree and over every revision of the published tip. `./run-tests.sh` is RED for the new scenarios by design; the RED state was proven by running one of them (`Step doesn't match any function`, exit 101).

DESIGN-REVIEW: PASS
