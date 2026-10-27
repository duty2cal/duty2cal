# Design review: state-the-supported-duty-plan

Two phases, run in fresh contexts as the instruction requires: a grilling
subagent interrogated the full plan and produced a ranked question list with
recommended answers (report: `agent://GrillStatement`; its verdict was
**NOT READY**, five blockers), and this hardening subagent then presented the
operator decisions that changed the plan's contract, applied every remaining
recommendation, and re-checked the artifacts.

Plan reviewed: `proposal.md`, `specs/state-the-converter-support-statement/`
(feature and use case), `specs/publish-public-website/`,
`specs/apply-configuration-change/`, `specs/publish-public-release/usecase.md`,
`design.md` and `arc42.md`, against the repository's permanent specs, scripts,
workflows and arc42 chapters.

## 1. Grilling record

The grilling subagent re-tested the plan's load-bearing claims rather than
accepting them, and every one held: `givn lint` was clean; the delta features
are `@documentation` and skipped by the assembler; the delta `@e2e` scenarios
never enter the assembled `@e2e` corpus; the fixture is a fictional June 2099
plan printing `LU742`; the activity-code list is complete; a documentation
`@e2e` scenario has precedent for committing transcript/frame evidence; the
public-text job runs the README gate and the `scripts` job runs bats; the site
publication triggers on every default-branch push with no CI dependency; the
site's inputs include the page and `scripts/check-site-content.sh`; all 12 arc42
chapters exist with content and no ASCII art; `docs/arc42/adr/` and its
`README.md` are unchanged. The findings below are therefore about the plan's
mechanism and coverage, not about false statements of repository fact.

### Branch coverage

| Required branch | Grilling verdict | Where it is resolved |
|---|---|---|
| **Scope** | Two gaps: an unauthorized public-text rule in the release delta, and the PR-template/contributor-guidance edit with no coverage | `proposal.md` now lists `publish-public-release` in its Use-Case Context and the routing rationale names the public-text rule (Q13); the withdrawn-wording scan now covers `.github/pull_request_template.md` and `CONTRIBUTING.md` with a bats mutation each (Q10) |
| **Tech choices** | Bash + `python3` fits; two simplifications missed | Constants moved to one `scripts/support-statement.env` (Q9); the site gate now runs the cross-carrier check (Q5) |
| **Missing scenarios** | The change's own apply is an update, but the delta modified the creation scenario | Delta now modifies `A revised profile content updates only the profile file` and asserts the served revised content carries the three sentences; the creation scenario is not touched (Q4) |
| **Testability** | Strict mode, stub pattern and single-run commands named; the check could not realize two scenarios and the clean output was unspecified | Second-plan scenario re-expressed as a mutated evidence record the check can produce; future-support refusal backed by a phrase list with a bats mutation per phrase; exact clean line defined and asserted (Q3, Q18) |
| **E2E fidelity** | Issue form asserted YAML as an `@e2e`; no per-delta-`@e2e` evidence; site publication evidence unplanned | Inventory row and `@e2e` tag removed by operator decision (Q1); per-scenario evidence matrix added (Q2); browser cell corrected (Q12) |
| **Interaction Coverage** | Consumer not an actor; matrix missed the release entries | Issue-form row removed (Q1); the two unchanged `publish-public-release` inventory rows added as document-only (Q12) |
| **Risk** | The check implemented to a presence table cannot go GREEN; the visual block fails for missing evidence | Mechanism extended (Q3, Q6, Q9); evidence matrix added (Q2); residual narrowed to an unforeseen contradicting sentence (Q3) |
| **Use case and Persona context** | Goal/trigger/level/relationships sound; success guarantee contradicted the shrink extension | Guarantee made one outcome covering the positive and shrunk statements (Q7); issue-form consumer removed with its row (Q1); four Personas unchanged, none invented |
| **Decision ledger** | D1–D5 all resolved, no seed-sourced IDs | D2–D5 updated to the hardened mechanism; D5 records the operator decision |
| **Visual design contract** | Present, both viewports, tokens, a11y; the visibly changed form was missing | Issue form recorded as excluded — GitHub-rendered, YAML-reviewed, no viewport evidence (Q17) |
| **Deferrals** | Live profile apply deferred without an operator decision | Operator decision recorded (Q4/D5); the update scenario and the operator transcript path are the acceptance evidence |
| **Persona disposition** | Four confirmations with relevance and evidence; at least one backs the visual contract | Unchanged |
| **Interface classification** | Page is Web UI (correct); publication action misclassified; form not tested against the opposite | Matrix classifies the publication action as the CLI, with the page text asserted by the content check (Q12); the form's opposite classification is now recorded and does not hold (Q17) |
| **ADR qualification** | No ADR is the right outcome; the marker had one verdict for three candidates and contradicted chapter 09 | Three per-candidate verdicts aligned to chapter 09; candidate 2's single canonical artifact is chapter 2 (Q15) |
| **Architecture documentation (arc42)** | 12-row table matched an independent derivation; chapter-09 verdict was the only defect | Verdicts split (Q15); glossary terms added (Q16); chapter 8's release-note claim fixed (Q14); chapter 11 TD-027 shrunk to one source, R-072 narrowed |
| **Ubiquitous language** | **Carrier** and **Evidence record** used but undefined | Both added to `docs/arc42/12-glossary.md`; intro and closing counts updated (Q16) |

## 2. Operator decisions

The operator's instruction was "Make it so" on the statement plan, whose
carriers include the profile apply. Two questions were put to the operator
because they change the plan's contract or defer a mandatory check; both are
recorded here with the decision taken.

### Q1 — the issue-form `@e2e` (planning correction)

The plan had invented an `@e2e` scenario for a GitHub-rendered issue form and
listed its consumer (`Reporter`) in the new capability's interaction inventory,
although that actor is not an actor of the use case and the Then steps assert
repository YAML, which the review classifies as a downgraded e2e.

**Operator decision: remove the inventory row and the `@e2e` tag; keep a regular
`@givn.added` documentation scenario for the form, and let the support check's
YAML assertions (bats) be its honest test. The capability keeps one `@e2e` — the
check run.** Recorded as a planning correction, not a gate bypass: the row and
the tag were introduced by this plan, the capability still has its real-interface
`@e2e` scenario (the CLI check run — bat-driven, exit status and named lines),
and the form's observable assertions are exercised by the bats suite per rule.
The residual is stated in the visual contract: the form is GitHub-rendered, the
project owns only its YAML, and no viewport evidence can be produced.

Applied: `specs/state-the-converter-support-statement/state-the-support-statement.feature`
(the form scenario is `@givn.added` only);
`specs/state-the-converter-support-statement/usecase.md` (inventory row removed);
`design.md` (matrix row removed, Visual Design Contract exclusion added, evidence
matrix row marked removed).

### Q4 / D5 — the live profile apply

The plan deferred the live profile apply as an architect decision, which the
review's Deferrals branch forbids without an operator question. Separately, this
change's apply is an **update** of the profile the archived
`publish-organization-profile` change created, while the delta had modified the
creation-premised scenario.

**Operator decision: the live profile apply is the operator's step after GREEN;
the operator accepts source-level verification now. Modify the profile's UPDATE
scenario ("A revised profile content updates only the profile file") to assert
the served revised content carries the three sentences; the creation scenario
stays unchanged. Record the operator's preview/apply/read-back transcript path
under the profile `@e2e` slug as the live acceptance evidence, and state that
the live profile lags until applied.**

Applied: `specs/apply-configuration-change/publish-organization-profile.feature`
now carries the update scenario as `@givn.modified @e2e` with the statement
assertion; `design.md` records the decision in D5, names
`evidence/visual/a-revised-profile-content-updates-only-the-profile-file/transcript.txt`
as the live acceptance evidence, and states the lag; `proposal.md`'s D5
disposition records the decision.

## 3. Findings applied (Q2–Q19, blockers B2, B3, B5)

| # | Finding | Applied change |
|---|---|---|
| Q2 | No committed evidence per delta `@e2e` scenario | `design.md` gains an **E2E evidence matrix**: the check's `transcript.txt` under its slug, the site publication's `mobile.png`/`desktop.png` copied from the browser run, the profile update scenario's operator `transcript.txt`; the browser smoke's four frames are marked supplementary; the removed form row is recorded |
| Q3 | The check cannot realize the future-support and second-plan refusals | Added a future-support phrase list (`will support`, `will convert`, `coming soon`, `future support`, `plans to support`, `roadmap`) to every carrier with a bats mutation per phrase; the evidence record became the single evidence authority (literal sentence, fixture builder, conversion scenario, month `2099-06`, labels/codes/leg, privacy rule, no `DTSTART`/`CREATED`); the second-plan scenario is now "A statement whose evidence record names no fixture is refused", realizable by mutating the record |
| Q4 | See §2 | See §2 |
| Q5 | The site publication was not gated by the cross-carrier check | `scripts/support-statement-check.sh` is now the **first** evidence step of `scripts/site-evidence.sh`, run inside the publication's detached worktree; `scripts/site-evidence.sh` is in the changed-files table; the design requires the check, `scripts/support-statement.env`, the five carriers, the record and `web/index.html` to land in **one revision** before any push |
| Q6 | The shrink state had no page constant and an ambiguous acceptance rule | `SUPPORT_SITE_SHRUNK` added; the page rule is "equals exactly one of `SUPPORT_SITE`/`SUPPORT_SITE_SHRUNK` + `BOUNDARY`"; the same pair is in `check-site-content.sh`; a bats test covers the shrunk tree |
| Q7 | The success guarantee contradicted the shrink extension | One outcome: "Every carrier carries the same statement; it names the one plan and states whether its fixture converts, and no carrier states more than the exercised evidence" |
| Q8 | `public-text-check.sh`'s `readme` argument was silently ignored by the composed check | Threaded as a refusal: when the argument is not the repository's `README.md`, the script refuses with one named line instead of checking another tree whose other four carriers would be the repository's |
| Q9 | Duplicated constants contradicted D2's "one place to edit" and TD-027 recorded the drift | Five constants (one positive and one shrunk form per subject) live in `scripts/support-statement.env`, sourced by both checks; TD-027 shrinks to "two call sites, one source" |
| Q10 | Withdrawn-wording coverage omitted two edited files | `.github/pull_request_template.md` and `CONTRIBUTING.md` added to the withdrawn-wording scan (forbidding `export family`/`export-family`) with a bats mutation each |
| Q11 | The evidence record's plan name and month were under-specified | Record must state the literal support sentence, the fixture builder and functions, the conversion scenario, the fictional month `2099-06`, the printed labels/codes/leg and the privacy rule; the check requires them and refuses a `DTSTART`/`CREATED` property or a complete calendar date |
| Q12 | The matrix's browser cell claimed a statement assertion the smoke does not have | Cell now says the publication action is the CLI, the page text is asserted by `check-site-content.sh` over the assembled output, and the Playwright smoke supplies the states, baseline and frames; the two unchanged `publish-public-release` inventory rows are added as document-only |
| Q13 | The release delta added an unauthorized public-text rule | Chosen fix: `publish-public-release` added to the proposal's Use-Case Context and the routing rationale broadened to the public-text rule; the delta keeps the rule |
| Q14 | The release-note claim was false for page-copy revisions | Design and chapter 8 now say the change alters no conversion behaviour but is announced as a public-copy change because `web/` is user-receiving; the commit scope is `feat(publish-public-website)`; the use case's release-text rule and the proposal's out-of-scope line are reworded |
| Q15 | One ADR verdict for three candidates, contradicting chapter 09 | `arc42.md` records three verdict blocks aligned to chapter 09 (candidate 1: `architectural_impact`/`lower_level_artifact` FAIL; candidate 2: `architectural_impact`/`durable_consequence` FAIL; candidate 3: `architectural_impact`/`lower_level_artifact` FAIL); candidate 2's single canonical artifact is `docs/arc42/02-architecture-constraints.md`, with the check mechanics called implementation detail, not a second routing. Outcome remains no ADR |
| Q16 | The glossary lacked **Carrier** and **Evidence record** | Both added to `docs/arc42/12-glossary.md`; the intro and closing record three added terms |
| Q17 | The visual contract omitted the visibly changed form | Added the exclusion paragraph: GitHub-rendered, YAML-reviewed by the statement check, no viewport evidence, not a project-owned Web UI |
| Q18 | The clean-run output contract was unspecified | One clean line defined (`support-statement: clean — 5 carriers agree; the evidence record names apps/cli/tests/steps/support.rs and Crew member converts a supported duty plan and receives a calendar file`), asserted in the bats clean test and in the scenario's `Then` |
| Q19 | Residual taxonomy in the bug form and the converter refusal | The form's `dates` placeholder becomes "the month the plan covers"; the converter's unsupported-document refusal is recorded as the separate change `export-duty-plan-calendar` already owns |

Additional consistency edits: the traceability rows, risk list, decision ledger,
arc42 notes, chapter 2 constraint row and the architecture diagram were updated
to the hardened mechanism, and the release use case's rule now states the
public-copy announcement.

## 4. Blockers

| Blocker | Status |
|---|---|
| **B1** — issue-form `@e2e`/inventory entry: invented interface, wrong actor, no possible evidence kind | **Resolved** by the operator's Q1 decision (§2) |
| **B2** — missing committed evidence for the site publication, check and profile delta `@e2e` scenarios | **Resolved**: E2E evidence matrix, including the operator transcript path |
| **B3** — the check could not realize the future-support and second-plan refusals | **Resolved**: phrase list, evidence record as authority, re-expressed scenario, bats mutation per rule |
| **B4** — profile apply deferred without an operator decision, and the delta covered the creation path | **Resolved** by the operator's Q4/D5 decision (§2); the update scenario now asserts the statement |
| **B5** — the site publication was not gated by the cross-carrier check | **Resolved**: first `site-evidence.sh` step plus the one-revision landing rule |

## 5. Checks run

- `givn lint --change state-the-supported-duty-plan` — exit 0, 3 files checked,
  two `LONG` advisories (the long site-publication and profile-content
  scenarios). No error or warning.
- `givn status --change state-the-supported-duty-plan` — exit 0; stage
  `arc42-docs-done`; proposal, specs, design and arc42-docs present;
  `design-review` is the next required artifact.
- The seed `givn/ideation/generalized-duty-plan-input/changes/state-the-supported-duty-plan.md`
  keeps `Status: proposed`; it was not stamped.

## 6. Remaining findings

None blocking. The recorded residuals are the plan's own:

- The issue form has no viewport evidence and no `@e2e` scenario; its YAML is
  covered by the statement check's bats suite (operator decision Q1).
- The live profile lags the reviewed source until the operator applies; the
  update scenario and the operator's transcript are the acceptance evidence
  (operator decision Q4/D5).
- A carrier can still add an *unforeseen* contradicting sentence that neither the
  withdrawn-wording list nor the future-support phrase list names (R-072, as
  narrowed); the clause's truth rests on the generated fixture and the review
  lenses, and the plan says so rather than claiming more.
- The converter's own unsupported-document refusal keeps its wording; it is a
  separate change owned by `export-duty-plan-calendar`.

DESIGN-REVIEW: PASS
