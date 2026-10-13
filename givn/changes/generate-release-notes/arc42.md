# arc42 impact assessment — generate-release-notes

Change: generate-release-notes. Assessed against `proposal.md`, `specs/` and `design.md` (the pinned generator and its configuration, the renderer and its refusals, the generated change log, the release text as its newest section, the capability allow-list checked against the corpus, the drift check, and the back-fill of the `0.1.0` section).

| # | Change introduces or modifies | Affected | Reason / what changed |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | Yes | `01-introduction-and-goals.md`: requirement 5 now states that a release's notes name the changes a user can see, drawn from the change log the repository keeps; the Release-integrity quality goal and the crew-member and release-owner stakeholder rows name the change log and the release that states what changed. |
| 2 | Constraints (legal, tech, org) | Yes | `02-architecture-constraints.md`: added the pinned `git-cliff` 2.14.2 installed with `taiki-e/install-action@v2`, the `changelog` CI job that refuses a committed change log differing from a fresh render, the capability allow-list checked against the use-case ids under `givn/specs/`, and the renderer as a Linux-only script outside the workspace. Every existing row kept. |
| 3 | External systems, interfaces, context, new user-facing surface | Yes | `03-context-and-scope.md`: the public-home subsection now states that the release page's text is the newest section of the repository's change log and that the change log is one of the files a release publishes. No new external system, interface or runtime dependency. |
| 4 | Major technical strategy or approach | Yes | `04-solution-strategy.md`: the release strategy now states that the release text is the newest section of a generated, drift-checked change log filtered to what a user can see, and the technology choices gain the change-log generator pinned at 2.14.2. |
| 5 | New building blocks, modules, components | Yes | `05-building-block-view.md`: the release toolbox gains `cliff.toml`, `scripts/render-changelog.sh` and the generated `CHANGELOG.md`, the `changelog` CI job, and the renderer among what the step files drive; the product block view is unchanged. |
| 6 | New runtime flows, sequences | Yes | `06-runtime-view.md`: a new Mermaid flow renders a version's section (with its version, tag, allow-list, drift and empty-render refusals), and the tag-release and pushed-change flows now include the render, the release text as the newest section, and the change-log drift check. |
| 7 | Deployment changes | Yes | `07-deployment-view.md`: the change log now joins the release assets — `CHANGELOG.md` sits beside the release text and the five archives and checksums on the published release. |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | `08-crosscutting-concepts.md`: added change-log generation determinism, change-log drift checking, and the visibility rule that decides what a reader sees. |
| 9 | Architecture decisions (tradeoffs, ADRs) | No | `09-architecture-decisions.md` unchanged: the change mints no ADR. The generator-choice candidate fails the qualification gate and is routed once to its cheaper home, `design.md` (chapter 2 records the pin as a constraint); the verdict is recorded below. |
| 10 | New quality scenarios | Yes | `10-quality-requirements.md`: added QS-011 (a release states the grouped changes a user can see; a version whose notes would name none is refused) and QS-012 (a committed change log differing from the released revision is refused), with two nodes under Release integrity in the quality tree. |
| 11 | New risks or technical debt | Yes | `11-risks-and-technical-debt.md`: added R-015 (the pinned generator's output shape drifting between versions), R-016 (the capability allow-list drifting from the corpus) and R-017 (the notes inheriting their quality from revision subjects), and extended R-012 with the published `0.1.0` archives' one-off execute bit. |
| 12 | New domain terms | Yes | `12-glossary.md`: added **Capability allow-list**, **Change log**, **Drift check** and **Released change**, refined **Release text** (now the newest section of the change log) and **Housekeeping revision**, added change-log drift to **Gate (on landing)**, and updated the promotion boundary. |

## ADR qualification

One candidate was judged by the instruction's five mandatory dimensions and the three falsification tests: the generator choice — `git-cliff` 2.14.2, pinned in the repository beside its configuration, with a bespoke renderer and an unpinned tool as the rejected alternatives.

| Field | Value |
|---|---|
| qualification | NOT_QUALIFIED |
| alternatives | PASS |
| architectural_impact | FAIL |
| durable_consequence | FAIL |
| lower_level_artifact | FAIL |
| existing_adr_check | PASS |
| must_be_shared | SUPPORTING |
| routing | CANONICAL_ARTIFACT |
| canonical_artifact | `design.md` |
| target_adr | null |
| replacement_adr | null |

- **alternatives** — PASS: the design records a real choice between `git-cliff` 2.14.2 and a bespoke renderer, and between a pinned and a run-time-resolved tool.
- **architectural_impact** — FAIL: the choice fixes no component, ownership, deployment or compatibility boundary; it selects a build tool for one renderer script, the shape of the "choose a logging library" case the instruction excludes.
- **durable_consequence** — FAIL: it is fully implementable and reversible within this one change (replace the tool and re-render the file), so the single-change-reversibility falsification test applies.
- **lower_level_artifact** — FAIL: the complete rationale already lives in `design.md` ("Technology decisions", "The renderer"), and chapter 2 states the pin as a constraint, so that home is not incomplete.
- **existing_adr_check** — PASS: the active register was searched; ADR-0003 owns the release channel and platform set and covers none of this boundary.
- **must_be_shared** — SUPPORTING: a contributor should know the tool is pinned because the drift check compares bytes, but sharing alone never qualifies a candidate.

Verdict: **NOT_QUALIFIED**, routed exactly once to `design.md`. No ADR file, chapter-09 register row, or ADR-index entry was created.

## Status

STATUS: DONE