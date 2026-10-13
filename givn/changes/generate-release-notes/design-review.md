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

DESIGN-REVIEW: PASS
