# Design review: list-only-changes-a-user-receives

Run after `design.md`, before `tasks.md`. The grilling ran in a fresh context: a subagent that wrote none of these artifacts read the proposal, the delta, the design, the arc42 assessment, the whole renderer and its configuration, the permanent specification and use case, the step definitions and the release runbook, and ran the specs, design-review and lint instructions plus the pinned generator against scratch repositories. Its report found one blocking defect in the plan, six design gaps and three missing scenarios.

## Questions and dispositions

| # | Question | Disposition |
|---|---|---|
| R1 | Where does the filter run relative to the `awk` that takes the version's section and collapses identical entries? | **Answered by this review — the plan was wrong and is corrected.** The marker the filter needs would enter the de-duplication key (`if (seen[$0]++) next`), so two revisions with one subject would no longer collapse and the permanent scenario that pins it would fail. The design's "the filter drops, identical subjects collapse" contradicted its own "after taking the version's section". Recorded as a design decision: take the section without collapsing, read and strip each entry's marker while resolving its revision, drop, then collapse. |
| R2 | Does this change re-render and commit the already-committed `0.2.0` section? | **Answered by the proposal, and required by the gate.** The proposal promises the re-rendered section, and `v0.2.0` does not exist yet, so it is an ordinary pre-tag render. Leaving it stale would make the release preparation's own drift check refuse the first `0.2.0` release. Recorded as a design decision and added to the changed files. |
| R3 | How does the filter treat a rename, a merge commit and an empty commit? | **Answered by this review.** `git show --name-only` output depends on the checkout's `diff.renames`, so rename detection is pinned off: a file moved out of a user-receiving path then lists both paths and the revision stays announceable. A merge commit and an empty commit list nothing, count as changing nothing, and are dropped. A revision whose paths cannot be read refuses, and no fixture reaches that arm: it is a defensive refusal, recorded as such rather than claimed as covered. |
| R4 | How is the machinery phrase list kept a genuine second net? | **Answered by this review.** The fixture revision whose subject names a coverage report must keep writing under a user-receiving path, so the phrase list remains the only reason it is dropped; otherwise that permanent scenario would pass for the new reason and stop proving the old rule. |
| R5 | The fixture builder writes one file per revision; how does the delta's "changed both" revision exist? | **Answered by this review.** `build_history` takes each revision's path set, not one path, and creates every listed path in the fixture repository. |
| R6 | The use case's visibility rule still describes words and scope only. | **Answered by this review.** The rule gains the third condition, and the use case is added to the changed files. |
| R7 | The third marked list has no agreed marker name. | **Answered by this review.** `<!-- user-receiving-paths: … -->`, used in the design, in `cliff.toml` and in the glossary. |
| R8 | The grilling found three implied behaviours without a scenario (the all-dropped-by-path refusal, the packaging path staying announceable, an unreadable revision). | **Answered by this review.** The packaging boundary gets its own scenario; the all-dropped case is exercised by repointing the permanent refusal's fixture at machinery paths, so its text is unchanged and it now proves the new rule; the unreadable-paths refusal is a defensive arm no fixture can reach, and is recorded in `design.md` instead of being claimed as covered. |
| R9 | Is the release runbook the right home for the authorship rule? | **Answered by this review.** `README.md`'s "Releasing a version" section, named in the design; the rule is stated once there and points at the general subject-authoring expectation rather than inventing a second policy. |

## Findings

| # | Branch | Finding | Where it landed |
|---|---|---|---|
| F1 | Testability | The fixture builder writes `revision-{index}.txt`, under no user-receiving path, and the fixture repository holds none of the listed paths: twelve permanent scenarios would break, several with the wrong refusal message. | The design's fixture decision and the changed-files table: per-revision path sets, every listed path created, announced revisions writing under a listed path, the empty-refusal history writing machinery paths. |
| F2 | The renderer's filter | The filter must run before the empty-case count, the heading replacement, the `--check` comparison and the `--release-text` write, and the marker must never reach a written or compared byte. | The pipeline order in the design, with the five steps named. |
| F3 | The marker | `commit.id` renders the full 40-character SHA into the section and survives extraction; `<!-- [0-9]* -->` does not touch it; the marker stays on the entry's own line. | Confirmed in the design's marker rule; no change needed beyond the name fixed in R7. |
| F4 | Path semantics | A rename out of a listed path, a merge commit and an empty commit were undecided and configuration-dependent. | The resolution rule and its four cases in the design. |
| F5 | Scope | The empty case under the path filter, the packaging boundary and the unreadable revision had no scenario or task. | The packaging scenario added; the empty case repointed at the permanent scenario's fixture; the unreadable arm recorded as defensive. |
| F6 | Documentation half | The use case contradicted the new rule, and the design's changed-files list omitted the change log and the use case. | Both added; the use case rule and its examples edited. |
| F7 | arc42 | The twelve-row derivation matches, and every chapter marked Yes carries the design's rule rather than an intention. | No change; the assessment stands. |
| F8 | Ubiquitous language | The marker name was undefined and the use case described visibility as words only. | The marker named; the use case rule and example edited; the glossary already holds the term and the refined ones. |
| F9 | E2E fidelity | No issue. No consumer action is added, so no new inventory row or `@e2e` scenario is owed, and the existing `@e2e` render scenario drives the real command. | — |
| F10 | Deferrals and gates | No issue beyond R2: no mandatory check is deferred, strict mode is intact, and the drift check passes only once the committed change log is re-rendered, which the design now records. | The design's decisions and the changed-files table. |
| F11 | Risk | The renderer restructure and the fixture change turn a wide slice of the existing suite red at once, tempting a weakened de-duplication or phrase-list scenario. | The pipeline order is fixed in the design, the phrase-list fixture keeps a user-receiving path, and the tasks list names the affected scenarios so a failure is attributed rather than edited away. |

## Hardening applied

- `design.md`: the pipeline's five steps and the reason for each; the pinned `git -c diff.renames=false` invocation with the four path cases; the marked list's name; per-revision path sets in the fixture builder; the re-rendered change log and the edited use case in the changed files; three design decisions recorded beside D1.
- Delta spec: the packaging scenario added; the rule scenario already distinguishes a machinery-only revision from one that changed both.
- `givn/specs/publish-public-release/usecase.md`: the visibility rule gains the third condition, with an example.
- `givn lint --change list-only-changes-a-user-receives` → the delta is well formed; the two `@wip` findings are the expected pre-implementation state.

No `@e2e` tag was added or removed. No mandatory check was deferred.

DESIGN-REVIEW: PASS
