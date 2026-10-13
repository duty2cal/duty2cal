# Design review: generate-release-notes

Fresh-context grilling of the complete plan, then hardening. The grilling subagent read the proposal, the design, the delta spec and the permanent use case being extended, the code it drives (`scripts/prepare-release.sh`, both workflows, the step files), the archived review of the first release, and seven chapters of the arc42 set — and, rather than accepting the design's description of the generator, it ran `git-cliff` 2.14.2 itself against the real history and against fixtures. That is where most of the findings came from.

## Grilling — branches walked

| Branch | Outcome |
|---|---|
| Scope | No issue: the delta's scenarios cover the proposal's four gaps, the refusal of a version with nothing visible, the drift check, the date and comparison link, the first version and the back-fill; the out-of-scope items (the release trigger, the action pins, rewriting 0.1.0, hand-written highlights, announcement channels) are absent from the spec. |
| Tech choices | The tool supports the design, with five corrections the probes forced: grouping by type as well as scope, a literal comparison URL, the first release's range and date, an idempotent write, and what the CI jobs need on PATH. All landed. |
| Missing scenarios | Four: the permanent scenario the new rule invalidates, the change log among the release's files, the allow-list refusal, and the missing-section refusal. All added or carried as a modification. |
| Testability | Sound: every scenario's steps are unimplemented, so each fails in RED on its own name without production code, and no Then asserts only that something ran. Two weaker assertions were tightened while hardening. |
| E2E fidelity | Sound: the `@e2e` scenario drives the renderer as a real subprocess and asserts the section it produced, which is this capability's own interface; obtaining a file from the host and running the published tool remain the host's and the conversion use case's business. |
| Visual design contract | Sound: terminal interface, no viewports, the new e2e transcript path recorded, and the baseline stated. |
| Interface classification | Tested the opposite: nothing here renders HTML, CSS or JavaScript in a browser — the change produces Markdown text — so the terminal classification holds and no browser driver is needed. |
| Persona disposition | Both confirmed Personas are recorded with what the change gives them: the reader judges scanability, the author judges whether the notes match the history. No Persona invented or promoted. |
| Deferrals | None: every promise in this change is either driven by the local suite or by the pipeline that already exists. |
| Interaction coverage | Two inventory rows, two matrix rows, two `@e2e` titles. One matrix row over-claimed that the prepare scenario asserts the staged change log; corrected, and the missing assertion added. |
| Risk | The most likely failure is the generator not delivering the specified behaviour — which the probes confirmed it does not, unmodified — and that is now settled in the plan rather than at implementation time. |
| Domain context | The delta extends the capability the proposal names; the typed relationships are still `none`; the Actors remain distinct from the Personas. |
| ADR qualification | Verdict `NOT_QUALIFIED`: the generator choice is reversible and its rationale has a cheaper home, `design.md`. Recorded, with no record minted and no register touched. |
| Architecture documentation | Twelve rows re-derived independently and diffed against the committed `arc42.md`: all twelve agree, every Yes row really changed, no identifier renumbered, Mermaid only. |
| Ubiquitous language | The glossary already carries the four terms the delta needs — change log, released change, drift check, capability allow-list — and Refines release text, housekeeping revision and gate in place. |

## Grilling — findings landed in the plan

| # | Finding | Landing place |
|---|---|---|
| F1 | The pinned generator exists and offers the surface the design uses: its config keys are real, it strips `type(scope):` and exposes the scope, and the pinned version is what nixpkgs supplies. | No change needed; recorded as the probe result behind the technology table. |
| F2 | One parser matching the scope cannot group by type — it filed `chore(publish-public-release)` under New capabilities. | The configuration row, the allow-list row and the decisions ledger: the allow-list is repeated inside each type's parser, with the rejected shared-scope alternative recorded. |
| F3 | Building the comparison link through the tool's remote integration makes live API calls and panics without a token; offline it renders a placeholder. | The comparison-links row, the renderer's determinism bullet and the ledger: the URL is literal and the remote integration is not enabled. |
| F4 | The first release's range is not directly expressible: the obvious forms fail or render an `[unreleased]` heading, and passing `--tag` stamps today's date into a historic section. | The range row, the renderer's time bullet and the ledger: the renderer never passes `--tag`, takes the heading date from the released revision, and refuses rather than falling back to the clock. |
| F5 | A literal prepend duplicates the released section, because the precondition already says the committed file carries it — which would make the drift check fail forever. | The renderer's output and `--check` bullets, its refusal list, the ledger and the traceability table: a render replaces its own version's section and prepends only one not yet present. |
| F6 | The section date and comparison link survive a lightweight tag and a first release; only an untagged revision fails, which the design already refuses. | No change; recorded as the probe result behind F4's resolution. |
| F7 | The draft-versus-staged asset comparison still matches once the change log joins the staged set, provided the workflow attaches it. | The file table's release-workflow row states the single edit. |
| F8 | The files the design names exist, or are correctly new. | No change. |
| F9 | Six of the seven scenarios drive the renderer as a subprocess, so the job running the suite needs the pinned generator on PATH, and the drift job needs the full history and tags. | The CI-gate row and the file table: `fetch-depth: 0` for the drift job and the pinned generator installed for the suite. |
| F10 | The matrix claimed the prepare scenario asserts the staged change log; it does not. | The matrix row corrected, and a new scenario asserts it instead. |
| M1 | The permanent scenario `The release text states the released revisions' release notes` is invalidated silently: its fixture scopes are not capabilities, and it is also the scenario that asserts the collapse the design had dropped. | Carried in the delta as `@givn.modified` under its exact title, restated against the new rule, and named in the traceability table as the collapse's evidence. |
| M2 | Nothing asserted that the change log joins the release. | A new scenario: the prepared release carries the change log its release text is drawn from. |
| M3 | The allow-list refusal and the missing-section refusal were claimed as covered but no scenario drove either. | Two new scenarios, one per refusal. |
| M4 | No scenario covered duplicate collapsing, which the permanent specification requires. | Covered by the modified permanent scenario above. |

## Operator questions

### Question 1 — the collapse of identical subjects

- Raised as an operator question; settled as a finding instead, because the artifacts left no choice: the permanent scenario already asserts that a repeated subject appears once and the glossary says so, and the design had dropped the rule — the back-filled 0.1.0 section would have been thirty-seven bullets, thirty-four of them duplicates.
- Disposition: **fixed in the plan.** The renderer collapses identical subjects — the generator does not — and the design records the consequence as the evidence that the rule works: 0.1.0's section falls from thirty-seven bullets to about five.

### Question 2 — the already-published v0.1.0

- Question: its page carries the older rule's flat fifteen-bullet list while the repository's new 0.1.0 section is about five grouped bullets; should the page be brought in line, or should the agreement promise start from the next version?
- Disposition: **answered 2026-10-13 — bring the page in line.** Its body becomes the repository's 0.1.0 section together with the one-off execute-bit note, so the promise holds for every version rather than from the next one. Rejected: leaving the page as generated and qualifying the promise, because the body is being edited anyway and the page is the account a downloader reads. The paste-ready text is produced once the section exists.

### Question 3 — the release procedure

- Question: must the released version's section be committed before the tag, or may the repository catch up afterwards?
- Disposition: **answered 2026-10-13 — committed before the tag.** Releasing a version gains one step: render the section, commit the change log, then push the tag, so the tagged revision already carries its own section and the drift check passes by construction. Rejected: exempting the version being released from the check, which would make the repository lag the page in exactly the window where agreement is claimed.

## Hardening applied

- The design absorbed all ten findings above: the configuration and allow-list rows, the comparison-links row, the range row, the renderer's output, check and refusal bullets, the CI-gate row and the file table, plus ledger entries for each with its rejected alternative, and the collapse rule restored.
- The delta grew from seven scenarios to eleven: three added for the change log's staging, the allow-list refusal and the missing-section refusal, and one carried as a modification of the permanent release-text scenario.
- The use-case document gained the two new refusals and the idempotent write in its own voice.
- The traceability table names every new or modified scenario against the rule it evidences, including the collapse's.

## Deterministic overlap findings

None. `givn lint --change generate-release-notes` reports eleven findings, every one of them the `@wip` tag of a scenario awaiting its step definitions; no shape match and no long scenario arose, so no disposition table is owed. The lint exit code is 2, which the design-review gate accepts.

## Per-branch coverage after hardening

Scope, Tech choices, Missing scenarios, Testability, E2E fidelity, Visual design contract, Interface classification, Persona disposition, Deferrals, Interaction coverage, Risk, Domain context, ADR qualification, Architecture documentation, Ubiquitous language: no issue outstanding, and every correction the grilling forced is visible in the artifacts rather than promised for later.

## Corrections after implementation (2026-10-13)

Question 2's answer above is left as it was written; these notes correct the record it rests on, added after the review had signed off and the implementation had run.

- The published v0.1.0 page's real count is **16** entries, not fifteen (`curl -sS https://api.github.com/repos/kressnerd/duty2cal/releases/tags/v0.1.0`, `body`); the repository's back-filled 0.1.0 section lists 3 under the new rules.
- The release procedure Question 2 assumes did not work: the renderer refused any version whose `v*` tag did not exist yet, so "render the section, commit the change log, then push the tag" exited non-zero on its first command. The renderer now takes the section's date from the tag when it exists and otherwise from `HEAD` — the revision the tag will point at — never the clock, and the commit that records the section must carry that same committer date.
- The reader lens found a realization gap in the permanent rule it had already stated: a `fix(<capability>):` revision whose words name the project's own tooling passed the type-plus-scope allow-list and reached the page (the two coverage entries in the 0.1.0 section). A marked machinery skip list now keeps those words out of every release, whatever a revision's scope says.
- The `publish` job refuses only a check run that has already concluded non-green, so the release preparation now runs the drift check on the committed change log itself, before it stages anything.

## Design-review re-run (2026-10-14)

A fresh context re-ran the review over the corrected design and the finished implementation. Everything below is the re-run's own record; the first pass and its corrections above are left intact.

### What the re-run read

`design.md` (corrected), `proposal.md`, the delta `specs/publish-public-release/publish-public-release.feature` and its `usecase.md`, the permanent `givn/specs/publish-public-release/publish-public-release.feature` and `usecase.md`, `tasks.md` (the review-fix items F1–F9), `arc42.md`, every chapter under `docs/arc42/` (all twelve read; unicode box-drawing scan clean), and the code the design drives: `cliff.toml`, `scripts/render-changelog.sh`, `scripts/prepare-release.sh`, `.github/workflows/ci.yml`, `.github/workflows/release.yml`, the README `### Releasing a version` section, `apps/cli/tests/features_runner.rs`, both `publish_public_release.rs` step files, and the committed `CHANGELOG.md`. The pinned generator (`git-cliff 2.14.2` via `nix-shell -p git-cliff`) was run against throwaway fixtures rather than trusted from the design's description.

### Branches walked, with a verdict each

| Branch | Verdict |
|---|---|
| Scope | No issue: the delta's eleven scenarios cover the proposal's four gaps plus the refusals, the date, the comparison and the back-fill; the out-of-scope items stay out. Unchanged by the re-run. |
| Tech choices | Correct after hardening: the generator, the extraction-from-whole-history render, the literal comparison URL, the date rule and the per-type allow-list all behave as the design states. The machinery skip list did **not** — see H1. |
| Missing scenarios | Hardened: the skip list's narrowness was unobservable, so the grouping scenario now drives a revision that merely borrows a tooling word for a crew member's own work and asserts it stays in the notes (H1). |
| Testability | Sound: every scenario fails RED on its own name without production code, and the two new steps assert concrete subjects, not "it ran". |
| E2E fidelity | Sound: the `@e2e` scenario drives the renderer as a real subprocess and asserts the section it wrote; the release host stays the host's business. |
| Visual design contract | Sound: terminal interface, no viewports, transcript path recorded, baseline stated. |
| Interface classification | Tested the opposite: nothing renders HTML/CSS/JS in a browser — the change produces Markdown text — so the terminal classification holds. |
| Persona disposition | Both confirmed Personas recorded with what the change gives them; none invented or promoted. |
| Deferrals | None: every promise is driven locally or by the existing pipeline. |
| Risk | Sound, and now complete: the most likely failure (the generator not delivering the specified behaviour) is settled; the publish gate's remaining window is recorded as TD-009 (H4). |
| Use-case and Persona context | Confirmed: `publish-public-release` extended, typed relationships still `none`, Actors distinct from Personas; the new delta scenario is this change's own. |
| Decision ledger | No issue: the proposal carries no `D<n>` (its Open Questions state none remain and records two unnumbered decisions), and both are carried in `design.md` with their rejected alternatives. |
| ADR qualification | `NOT_QUALIFIED`, routed once to `design.md`; the register and chapter 09 untouched. Re-confirmed against the corrected design. |
| Architecture documentation | Hardened: twelve rows re-derived independently and diffed against `arc42.md`. Two under-declarations fixed in `arc42.md` (H2, H3) and one missing residual row added to chapter 11 (H4); every Yes row's chapter content matches the design; all twelve chapter files exist with real content and no ASCII art. |
| Ubiquitous language | The glossary already carries change log, released change, drift check and capability allow-list; no term changed with the hardening. No issue. |

### Findings, and where each landed

| # | Finding | Disposition |
|---|---|---|
| H1 | **The machinery skip list was not narrow.** `cliff.toml` realized the terms as bare subject-anchored words (`coverage`, `cobertura`, `clippy`, `rustfmt`, `notices`, `workflow`, `transcript`), so a genuine crew-facing subject that merely borrowed one was silently dropped. Demonstrated: a fixture whose released subjects were `Support the crew's ramp workflow`, `Handle notices from the roster`, `Record the coverage of a plan` and `Fix the transcript of the planning notes` rendered a 0.1.0 section naming **only** `Support night duties` — the four others vanished, against the use-case rule that a revision naming a capability is a released change. | **Fixed.** Each term is now a distinctive tooling phrase (`coverage report`, `coverage merge`, `cobertura`, `clippy`, `rustfmt`, `third-party notices`, `ci workflow`, `release workflow`, `e2e transcript`); the marked list is comma-separated so a term may contain spaces, and the renderer splits on commas and refuses a phrase the configuration does not realize (`scripts/render-changelog.sh`). The two real 0.1.0 machinery entries (`…coverage merge`, `…merged coverage report`) are still dropped, so `CHANGELOG.md` is unchanged (drift check still exits 0), while the four borrowed-word subjects now survive. The grouping scenario gained `And a revision whose subject borrows a tooling word for a crew member's own work` and its Then; `design.md`, the delta use case and `docs/arc42/02`/`08` carry the narrowed wording. |
| H2 | `arc42.md` row 2 under-declared what `docs/arc42/02-architecture-constraints.md` holds: it did not name the marked machinery skip list, nor the drift check the release preparation itself runs. | **Fixed.** Both added to `arc42.md` row 2. |
| H3 | `arc42.md` row 11 did not name TD-008, which the review added to chapter 11 and `design.md`'s file table names. | **Fixed.** TD-008 named in `arc42.md` row 11. |
| H4 | The publish gate's remaining window was unrecorded: the `publish` job refuses only a check run that has already concluded non-green, so an in-progress run is neither green nor refused and publication does not wait for it (the change log alone is covered by the preparation's own check). | **Fixed.** Added TD-009 to `docs/arc42/11-risks-and-technical-debt.md`, naming the residual and the follow-up (make the publish job wait for the tagged revision's check suite to conclude, or require the named checks); `arc42.md` row 11 names it. |
| H5 | `design.md`'s "New or changed files" table was stale for `docs/arc42/02-architecture-constraints.md`, which the review phase changed alongside 05, 06, 08, 11 and 12; and it omitted the assessment-phase chapters 01, 03, 04, 07 and 10, which this change genuinely modified (visible in `git log`, and claimed by `arc42.md`'s Yes rows). | **Fixed.** The row now names every durable chapter this change changed — the review-re-derived `02`/`05`/`06`/`08`/`11`/`12` and the assessment-updated `01`/`03`/`04`/`07`/`10` — so the table and `arc42.md` agree. |
| H6 | The documented push was not atomic: `git push origin <branch> v<version>` can publish the tag even when the branch push is rejected, starting the release workflow for a revision that never reached the default branch. | **Fixed.** README and `design.md` now use `git push --atomic origin <branch> v<version>` and say why in one clause. |
| H7 | The README procedure named `nix-shell -p git-cliff`, which resolves to whatever the channel carries, while the renderer refuses anything but the pinned version. | **Fixed.** The README now states the pin (`git-cliff` 2.14.2, recorded in `cliff.toml`) before the commands. |
| H8 | The first pass's findings F1–F9 were re-checked against the implementation. | **F1 (machinery words reached the page)** fixed, and further hardened by H1; **F2 (procedure could not run)** fixed — the renderer dates an untagged version from `HEAD`; **F3 (stale release could ship)** fixed by the preparation's own check, whose residual window is now TD-009; **F4 (check scope overstated)** fixed in the renderer section, the `--check` bullet and the glossary; **F5 (comparison link built by the renderer, not the template)** fixed in every place that said otherwise; **F6 (file table incomplete)** fixed for the code files, and the arc42 row further corrected by H5; **F7 (durable chapters stale)** fixed, plus chapter 11's TD-009 (H4); **F8 (bullet counts disagreed)** fixed; **F9 (the item set)** closed. |
| H9 | **TD-008** (a version's notes are only as clear as the released revisions' own subjects) — is it still true after H1? | **Holds.** The narrowed skip list removes only the phrases that name the project's own tooling, not a subject written in the project's vocabulary; TD-008 is unchanged and now named in `arc42.md` row 11 (H3). |

### Operator questions

None. H1 was settled from the artifacts rather than by asking: the delta use case already states that a revision whose subject names a capability — a crew member's own work — is a released change, and that a revision naming the project's automation is not, so the bare-word skip list contradicted a rule the operator had already fixed. The first pass's Questions 1–3 and their dispositions stand unchanged.

### Verification after hardening

- Full suite — `nix-shell -p rustc cargo git-cliff --run './run-tests.sh'`: `3 features`, `96 scenarios (96 passed)`, `367 steps (367 passed)`, exit 0.
- `givn lint --change generate-release-notes`: `1 file(s) checked — clean`, exit 0.
- The drift check on the committed change log — `nix-shell -p git-cliff --run 'scripts/render-changelog.sh --version 0.1.0 --repo . --out CHANGELOG.md --check'`: `change log is current for version 0.1.0`, exit 0 (the narrowed list leaves the committed file byte-identical).
- Negative controls run during the re-run: a fixture whose released subjects borrow the tooling words now keeps them; a copy of the configuration with one realized phrase removed still refuses (`the machinery skip list names a term the configuration does not skip: cobertura`).

All findings dispositioned, no question outstanding, and no item carried to a later phase:

DESIGN-REVIEW: PASS
