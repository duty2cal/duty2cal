# arc42 impact assessment — measure-executable-size

Change: measure-executable-size. Assessed against `proposal.md`, the delta
under `specs/measure-distributed-executable-size/`, the permanent use case,
and `design.md` (the measurement command, the two builds, the report, the
workflow cadence, and the test fixtures).

| # | Change introduces or modifies | Affected | Reason / what changed |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | Yes | `01-introduction-and-goals.md`: the contributor stakeholder row records the promoted build-configuration reviewer as a segment of the contributor type — they judge whether a size attribution is reproducible and names the right artifact, and what a build-configuration change costs a crew member. No product requirement or quality goal changes; the measurement serves the existing release-integrity goal. No new stakeholder type or Persona was minted. |
| 2 | Constraints (legal, tech, org) | Yes | `02-architecture-constraints.md`: added the rule that a size attribution is produced for every pushed revision by the gate's Windows cross-build, from a build that retains the names the measurement needs with `strip` disabled for that build only, with the stripped executable's file size reported separately and never substituted; the report names the requested and resolved revision, the platform and the build's identity; added the pinned-generator row for `cargo-bloat` 0.12.1 and the build-tooling script outside the workspace. |
| 3 | External systems, interfaces, context, new user-facing surface | No | `03-context-and-scope.md` unchanged: GitHub remains the only external system and the measurement consumes no external interface. The command and its report are project-owned build tooling between the maintainer, the automation and the run artifact, not a product or business interface. |
| 4 | Major technical strategy or approach | Yes | `04-solution-strategy.md`: the technology-choices table gains the size-attribution approach (the shipped artifact and a second build that retains the names the measurement needs, measured separately, `cargo-bloat` pinned), and the release-integrity approach line names the measured attribution produced for every pushed revision. |
| 5 | New building blocks, modules, components | Yes | `05-building-block-view.md`: the release toolbox diagram and table gain `scripts/measure_executable_size.py` (the two builds, the report) and the two report files; the `ci.yml` row names the measurement step and the Windows-toolchain setup on the `test` and `coverage` jobs; a row names the new step-definition files and the fixture workspace. |
| 6 | New runtime flows, sequences | Yes | `06-runtime-view.md`: a new scenario, "A pushed revision gains a size attribution", records the revision resolution (requested and resolved), the shipped build, the build retaining the names, the two numbers, the refusals and the run artifact; a failed measurement publishes its report and the job continues — evidence, not a gate. |
| 7 | Deployment changes | Yes | `07-deployment-view.md`: the `ubuntu-latest` node and its diagram gain the attribution produced for the revision it built; a new subsection records that the measurement step is evidence, not a gate (it exits zero and always publishes the report), that the report travels beside the run, not among the release assets, and that the suite's jobs install the Windows target, `mingw-w64` and the pinned generator. |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | `08-crosscutting-concepts.md`: a new section, "Measured size attribution", records the two-build rule, the two numbers that are never substituted, the report's statuses, the evidence-not-a-gate rule, the estimate caveat (the generator's numbers are estimates of `.text`, not a byte-exact file size), and the no-archive/no-host inference rule. |
| 9 | Architecture decisions (tradeoffs, ADRs) | No | `09-architecture-decisions.md` unchanged: the measurement command, the generator pin, the report shape and the cadence all fail the qualification gate and are routed once to `design.md` (below). No register or index entry was created. |
| 10 | New quality scenarios | Yes | `10-quality-requirements.md`: the quality tree gains "Measured size attribution" under release integrity, and QS-022 states the per-revision attribution criterion, including the refusals, the incomplete case, the build identity, and the rule that the measurement does not refuse a revision or release. |
| 11 | New risks or technical debt | Yes | `11-risks-and-technical-debt.md`: R-032 now leads with the unverified GNU-PE symbol premise (with the first-task spike and its fallback), R-033 (the supplied artifact's origin is a claim; pull-request runs measure the merge revision), R-034 (evidence, not a gate: a broken measurement is green and can be ignored, so the published report is the control) and TD-014 (a second workspace binary would need an explicit target choice; the human report lists the largest 25 functions of N). |
| 12 | New domain terms | Yes | `12-glossary.md`: refined **Size attribution** with the cadence, the requested and resolved revision, the build identity, the command, the pinned generator (both the `--crates` and the functions run) and the corrected contrast (the numbers are estimates of `.text`, not a byte-exact file size), and added **Size attribution report** for the report's two files and its complete/incomplete/unreadable statuses. |

## ADR qualification

Four candidates were assessed against the five mandatory dimensions. Existing
ADRs searched: ADR-0001 (licence), ADR-0002 (layer enforcement), ADR-0003
(release channel and platforms), ADR-0004 (immutable draft-first publication),
ADR-0005 (public-home declaration), ADR-0006 (encrypted local applied state);
no archived ADRs exist. None covers build measurement, so no candidate is an
amendment or a supersession.

| Candidate | alternatives | architectural_impact | durable_consequence | lower_level_artifact | existing_adr_check | must_be_shared | routing | canonical_artifact |
|---|---|---|---|---|---|---|---|---|
| Measurement command (one command, two builds, temporary worktree, artifact input) | PASS — one repository command vs a CI-only step; two builds vs one; the gate's artifact vs a rebuild | FAIL — a build-tooling script beside the workspace; no component, ownership, dependency, deployment or compatibility boundary moves | FAIL — reversible within one change by deleting the script and reverting its workflow step; no migration or cross-component coordination | FAIL — `design.md` owns the mechanism, and the use case with the delta's scenarios owns the observable behaviour | PASS — no existing ADR covers build measurement | SUPPORTING — a contributor running the measurement should know the two-build rule; chapter 8 records it | CANONICAL_ARTIFACT | `design.md` |
| `cargo-bloat` 0.12.1 as the attribution generator | PASS — `cargo-bloat` vs hand-rolled symbol accounting vs no split | FAIL — an interchangeable local tool choice with no system boundary | FAIL — a pin bump is a one-change edit; no interface or data migration | FAIL — `design.md` and the report/use case own the choice and its consequences | PASS — none of ADR-0001…0006 names a measurement tool | FAIL — a contributor never encounters the tool outside the command | CANONICAL_ARTIFACT | `design.md` |
| Report shape (JSON + Markdown, complete/incomplete/unreadable, separate fields) | PASS — two renderings vs one; statuses vs a single error channel | FAIL — an internal build-tooling format; the product never reads it, and no public interface or compatibility promise attaches to it | FAIL — the format can change with its scenarios and the report is regenerated per revision; no consumer outside the project | FAIL — the use case and its scenarios own the observable contract; `design.md` owns the fields | PASS — no existing ADR covers a size report | FAIL — the labels live in the delta's scenarios and the report itself | CANONICAL_ARTIFACT | `design.md` |
| CI cadence (every pushed revision, artifact beside the run) | PASS — every change vs every release vs on demand | FAIL — a step in the existing Windows job; no runner, asset, permission or deployment topology changes | FAIL — reversible within one change by removing the step; no boundary others build against | FAIL — repository process policy; chapter 2 records the constraint and chapter 7 the deployment fact, while `design.md` owns the rationale | PASS — ADR-0003 decides the channel and platforms, ADR-0004 publication immutability; neither decides measurement cadence | SUPPORTING — the release maintainer and the reviewer rely on the artifact existing before a lever lands | CANONICAL_ARTIFACT | `design.md` |

Verdict: **NOT_QUALIFIED** for all four candidates; each is routed exactly once
to `design.md`. No ADR file, chapter-09 register row or ADR-index entry was
created, and the MADR template was not copied. The durable facts that do not
belong to the decision rationale are recorded where they live: chapter 2 as a
constraint, chapter 7 as a deployment fact, chapter 8 as a cross-cutting rule,
chapter 10 as a quality scenario and chapter 11 as risks and debt.

## Term checks

The change added one term the ideation Handoff did not confirm. It was put to
the user and confirmed on 2026-10-18.

| Term | Definition | Contrast | Collision check | Confirmed |
|---|---|---|---|---|
| Size attribution report | The pair of files a measurement writes — a machine-readable report and a human rendering of the same data — naming the revision, the platform and the build's identity, with the crate and function split and the stripped executable's file size as separate fields. Its status is `complete`, `incomplete` or `unreadable`. | Not the attribution itself; not the release archive; not the change log. | `git grep -ic 'size attribution report' -- ':!target'` → 1 file (the glossary row itself); `grep -ric 'size attribution report' givn/specs` → 0; `grep -ric 'size attribution report' givn/ideation` → 0. | 2026-10-18 |

The four Handoff terms — build configuration, distributed executable, size
attribution and size lever — keep their confirmed definitions; this change
refines only **Size attribution**'s contract use with the cadence, build
identity, command and pinned generator.

## Status

STATUS: DONE
