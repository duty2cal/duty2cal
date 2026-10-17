# Design review: announce-dependency-updates

Run after `design.md`, before `tasks.md`. The grilling ran in a fresh context:
a subagent that wrote none of the artifacts read the proposal, the delta and
permanent specs, the design, the arc42 assessment and chapters, `cliff.toml`,
both release scripts, the step harness and runner, and reproduced the parser,
renderer and preparation behaviour with `git-cliff` 2.14.2 in throwaway
repositories. No repository file was modified by the grilling.

The review found one blocking defect and eight lesser findings. Every finding
was settleable from the repository; no question required the user.

## Questions and dispositions

| # | Question | Disposition |
|---|---|---|
| R1 | When the path filter removes a dependency group's only bullet, does an empty `### Dependencies` heading survive into the change log and the release text? | **Answered by this review — the plan was wrong and is corrected.** `git-cliff` emits the heading because a revision matched the parser; the path filter drops the bullet but keeps every non-bullet line. A parser-only implementation stages an empty group. The design now adds one generic post-filter step that prunes a `### ` heading whose next non-blank line is another heading or the section end, before the visible-count check; the delta's checks-only scenario now asserts that no Dependencies heading is present. |
| R2 | Is the “identical dependency subjects collapse” promise evidenced by a scenario? | **Answered by this review.** It was not: the ordered and capability-scoped fixtures use distinct subjects. A dedicated scenario and `DEPENDENCY_COLLAPSE_HISTORY` now pin one occurrence of a repeated `chore(deps)` subject. |
| R3 | Does the delta use case contradict itself about the refusal? | **Answered by this review.** The extension repeated the old “nothing a user can see” rule beside the new dependency-only releasability rule. The extension and the grouped-notes rule now say “a version whose notes name nothing at all”, and “changes a user receives” replaces “can see” in the rule and main flow. The renderer's legacy refusal message is unchanged. |
| R4 | Can the checks-only and modified-refusal scenarios fail before the parser exists? | **Answered by this review.** No: they are preservation guards whose RED is stub-induced. The checks-only scenario becomes a genuine failing case against a parser-only implementation through the new no-heading assertion. This is recorded in the design rather than claimed as RED-on-production. |
| R5 | Does the numeric group marker order the groups or collide? | **Answered by this review.** `git-cliff` 2.14.2 orders groups by parser position; the marker is cosmetic and stripped. The full group string must be unique because identical strings merge entries. The design's rationale and risk are corrected. |
| R6 | Does the dependency-only scenario prove “those updates and no other change”? | **Answered by this review.** It only asserted the updates were named. It now also asserts that the `chore(ci)` and `docs(readme)` subjects are absent. |
| R7 | Does the glossary's new term carry every condition? | **Answered by this review.** The `Dependency update` row and the `Housekeeping revision` exception now require that the subject's words not name the project's own tooling, matching the delta and the machinery skip list. |
| R8 | Does the README change leave a now-false blanket claim? | **Answered by this review.** Appending one sentence would leave “so it is never released at all” standing. The design now requires amending the existing paragraph to name the `chore(deps):` exception. |
| R9 | What happens to the replaced refusal scenario's fixture and step after archive? | **Answered by this review.** The old `INVISIBLE_HISTORY` and its `Given` remain used while the permanent and delta features run together, then become dead code. The design records their archive-time removal for the tasks artifact. |

## Findings

| # | Branch | Finding | Where it landed |
|---|---|---|---|
| F1 | Tech choices / D4 / testability | A parser-only renderer leaves a dangling `### Dependencies` heading after the path filter removes its only bullet; `prepare-release.sh` stages it into the release text, and the checks-only scenario could not detect it. | The design's renderer decision and “The renderer change: prune an empty group heading”; the changed-files table; D4; the delta's checks-only scenario; arc42 chapters 5, 6 and 8. |
| F2 | Scope / missing scenarios | The identical-subject collapse promise had no scenario or fixture. | New scenario “Identical dependency update subjects collapse to one entry”, `DEPENDENCY_COLLAPSE_HISTORY`, `dependency_update_named_once`, and a traceability row. |
| F3 | Use case / language | The delta use case's extension contradicted the dependency-only releasability rule. | The extension, the grouped-notes rule and main flow now use “nothing at all” and “a user receives”. |
| F4 | Testability | Two scenarios are preservation guards, not RED-on-production. | The design records which scenarios fail pre-change and why, and that the checks-only scenario fails against a parser-only implementation. |
| F5 | Tech choices / risk | The marker-number rationale was mechanically wrong. | The parser-entry rationale and the risk bullet now say ordering is parser position and the full group string must be unique. |
| F6 | Scope / testability | Scenario 1 did not prove the absence of housekeeping bullets. | The scenario gains “the notes name no housekeeping revision” and the design defines the assertion. |
| F7 | Ubiquitous language | The glossary's dependency term omitted the machinery-word condition. | The `Dependency update` row and the `Housekeeping revision` exception now carry it. |
| F8 | Scope / documentation | The README design task appended a sentence beside a now-false blanket claim. | The changed-files entry now requires amending the existing paragraph. |
| F9 | Scope / cleanup | The replaced refusal scenario leaves an unused fixture and step after archive. | The design records the archive-time removal for the tasks artifact. |

## Hardening applied

- `design.md`: renderer changed with the empty-heading prune and its exact
  step; D4 rewritten; parser-position ordering; group-string uniqueness;
  changed-files table includes the renderer and the README amendment;
  `DEPENDENCY_COLLAPSE_HISTORY`; the new Given/Then signatures; the guard
  scenarios; the archive-time cleanup; traceability rows for collapse and the
  empty group; the flow diagram now includes the prune.
- Delta spec: new “Identical dependency update subjects collapse to one entry”
  scenario; scenario 1 asserts no housekeeping revision; scenario 4 asserts no
  Dependencies heading; the ordering scenario's Given names the actual fixture
  shape.
- Delta use case: the refusal extension no longer contradicts the
  dependency-only rule; “a user receives” replaces “a user can see” where the
  grouped notes are described; examples for collapse and the empty group.
- `docs/arc42/05`, `06`, `08` and `12-glossary.md`: the empty-group prune, the
  machinery condition, and the group-absence rule.
- `givn lint --change announce-dependency-updates` → 1 file, 9 findings, all
  the expected `@wip` pre-implementation state.

No `@e2e` tag was added or removed. No mandatory check was deferred.

## Branches with no issue found

- **Tech mechanism** — the exact `^chore\(deps\):` parser, first-match order,
  `chore(deps)!:` protection, `revert` preprocessing, history order, path
  filter, collapse and preparation were reproduced with 2.14.2; only the empty
  heading was wrong.
- **Preparation** — dependency-only staging of five archives, five checksums,
  the change log and the release text exits zero; the refusal fixtures still
  refuse.
- **E2E fidelity** — the inventory has two entries, both driven through the
  real scripts; the delta's scenarios are variants of those actions, so no new
  `@e2e` scenario is owed and none was removed.
- **Interaction coverage** — the design matrix has both inventory rows with
  drivers.
- **Interface classification / visual contract** — terminal interface; no new
  rendered screen, no new transcript.
- **Deferrals** — strict-mode proof is in the setup task; coverage boundaries
  are unchanged; no mandatory check is skipped.
- **ADR qualification** — `NOT_QUALIFIED`, canonical `design.md`; chapter 9 and
  the ADR indexes are correctly unchanged.
- **Use-case identity / Personas** — level `!`, one capability, no
  includes/extends, both confirmed Personas, the maintainer as the only Gherkin
  actor.
- **Decision ledger** — D1–D4 all resolve in `design.md`; D4's proof is now
  correct after F1.
- **Risk** — the parser-prefix, dev-dependency-noise, missing-`Cargo.lock` and
  breaking-`chore` risks are accurate.
- **Architecture documentation** — the independent 12-row derivation matches
  `arc42.md`; rows 1–6 and 8–12 are Yes with content checked, rows 7 and 9 are
  No; all chapter files exist, contain real content, and use Mermaid only.
- **Ubiquitous language** — the terms are in the glossary after the F7 edit.

## Independent arc42 derivation

| # | Verdict | Chapter checked |
|---|---|---|
| 1 | Yes | ch. 1 release goal and quality goal name dependency updates |
| 2 | Yes | ch. 2 allow-list decides capability scopes; the dependency parser sits below it |
| 3 | Yes | ch. 3 release-page text names capability changes and dependency updates |
| 4 | Yes | ch. 4 release strategy names Dependencies and the path filter |
| 5 | Yes | ch. 5 `cliff.toml` parser/group and the renderer's empty-group prune |
| 6 | Yes | ch. 6 group list, housekeeping exception and prune |
| 7 | No | no deployment target, archive, asset or provenance change |
| 8 | Yes | ch. 8 released-change definition, group order and hidden emptied group |
| 9 | No | no ADR qualifies; chapter 9 and both indexes unchanged |
| 10 | Yes | ch. 10 quality-tree node and QS-014 |
| 11 | Yes | ch. 11 R-018 and TD-011 |
| 12 | Yes | ch. 12 **Dependency update** and the refined terms |

DESIGN-REVIEW: PASS
