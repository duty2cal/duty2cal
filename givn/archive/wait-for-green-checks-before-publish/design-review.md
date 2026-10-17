# Design review: wait-for-green-checks-before-publish

Run after `design.md`, before `tasks.md`. The grilling ran in a fresh context: a subagent that wrote none of these artifacts read the proposal, the delta and permanent use cases/features, the design, `release.yml`, `ci.yml`, both release scripts, the step harness and all arc42 chapters. It reproduced the API shape against the live v0.3.0 runs and ran the gate's `jq`/`awk` classification against the recorded `bd5d741` check-run response in `/tmp`. No repository file was modified by the grilling.

The review found no proposal change and no user decision; one blocking design defect, several scenario and test gaps, and two arc42 omissions. All are resolved below.

## Questions and dispositions

| # | Question | Disposition |
|---|---|---|
| R1 | Does the gate publish while a check is still queued or running? | **Answered — the design's core is correct.** The loop classifies each poll, waits while any row is not `completed`, refuses immediately on a completed non-success, and refuses at the bound naming the running checks. The live v0.3.0 evidence confirms the outside-suite set exists before `publish` starts (3m12s margin). |
| R2 | Does excluding only the current release run's suite survive a retag of an unpublished version? | **Answered — the plan was wrong and is corrected.** The revision can carry an earlier release run's suite; its failed `publish` check would refuse the new run. The workflow now resolves every `release.yml` run for `head_sha` and passes one `--exclude-suite` per suite; the script accepts the flag repeatably. |
| R3 | Is an all-skipped/all-neutral set refused? | **Answered by this review.** The rule and the design arm existed; no scenario pinned it. A scenario, step and example are added. |
| R4 | Is the unreadable check-run source fail-closed? | **Answered by this review.** The arm existed without a test. A scenario and a stub failing mode are added. |
| R5 | Is the own-suite exclusion exercised with a running release check? | **Answered by this review.** The waiting and atomic fixtures now include an own-suite `publish` in-progress row and assert it never appears under `still running`. |
| R6 | Does `filter=latest` mean one row per name? | **Answered — the design wording was wrong.** It means the latest attempt per suite and name; duplicate names across suites remain. The design and the risks say this, and the atomic-push fixture uses two suites with the same check name. |
| R7 | Can the test helper find the script? | **Answered by this review.** The gate invocation now sets `.current_dir(repository_root())`, as every existing script invocation does. |
| R8 | Are unknown conclusions, API failures, pagination and queued checks exercised? | **Answered by this review.** The fixtures add `timed_out`, a failing stub, a two-page poll, `queued`, and an `args.log` assertion for `filter=latest`. |
| R9 | Do the arc42 chapters cover this change? | **Answered by this review.** Chapters 2 and 5 were omitted (the drift-check rationale becomes false; the release toolbox gains a block), and chapter 8 was recommended. The design's impact list now names 2, 5, 6, 7, 8, 10, 11 and 12. |
| R10 | Is an ADR required? | **Answered by this review.** `NOT_QUALIFIED`: extracted command and timing are local mechanism choices; the behavioural rules belong to the use case and Gherkin. No ADR is created. |
| R11 | Is the host-only proof a deferral without a recorded operator decision? | **Answered by this review.** The 2026-10-13 operator boundary for the publication interface (no credential in the development environment; the host owns the interface) is cited; it is unchanged. |
| R12 | Is the atomic-push scenario distinct from the waiting scenario? | **Answered by this review.** It now drives two suites with the same check name and asserts both are waited for, so a single-suite implementation fails it. |
| R13 | Are the timeout message and the local test toolchain pinned? | **Answered by this review.** The bounded-wait assertion uses `1 second`; the design records the script's SPDX header and mode, and that local `jaq` versus CI's GNU `jq` is covered by CI's real parser. |

## Findings

| # | Branch | Finding | Where it landed |
|---|---|---|---|
| F1 | Missing scenarios | No all-skipped/all-neutral scenario. | Scenario, step, use-case example and traceability row added. |
| F2 | Risk / retags | Only the current release suite was excluded; a retagged unpublished version inherited an earlier failed `publish` check. | The workflow resolves every release-workflow suite for `head_sha`; the script flag is repeatable; the rules and examples say “suites”. |
| F3 | Arc42 | Chapter 5 (release toolbox) omitted the new block. | Design impact and changed-files list updated; arc42 assessment will add it. |
| F4 | Arc42 | Chapter 2's drift-check rationale quotes a now-false premise. | Design impact updated; the rationale will be rewritten. |
| F5 | Tech choices | `filter=latest` was described as one row per name; duplicate suites were unrecorded. | Wording corrected; a risk records the atomic push's two CI suites. |
| F6 | Testability | The gate helper omitted the working directory. | `.current_dir(repository_root())` added to the design. |
| F7 | Testability | Several fail-closed arms had no fixture. | Fixtures extended: own-suite in-progress, `timed_out`, failing stub, two pages, `queued`, `args.log`. |
| F8 | Missing scenarios | The no-evidence refusal claimed a suite “will never” appear; the claim was overstated. | Softened to the observed margin; the refusal stays immediate and fail-closed. |
| F9 | Ubiquitous language | Glossary lacked **Publication gate**, **check run**, **check suite** and the bounded wait. | Design's glossary impact names them; the arc42 assessment will add them. |
| F10 | ADR | No qualification verdict was recorded. | `NOT_QUALIFIED`, canonical `design.md`, with the existing-ADR check. |
| F11 | Deferrals | The host-only proof did not cite the operator boundary. | The 2026-10-13 boundary is cited; no fresh decision is needed. |
| F12 | Scope / overlap | The atomic-push scenario duplicated the waiting scenario. | Rewritten to drive two same-named checks in two suites; the gate must wait for both. |
| F13 | Implementation risk | Timeout message grammar, script metadata and local `jaq` were unstated. | Message pinned to `1 second`; SPDX/mode added; CI's real `jq` recorded. |

## Hardening applied

- `design.md`: repeatable `--exclude-suite`; the pre-loop `head_sha` runs query and exact path match; plural suite wording throughout; the corrected `filter=latest` semantics; the retag and duplicate-suite risks; the all-skipped, unreadable and two-suite scenarios and steps; `.current_dir(repository_root())`; the expanded fixtures (two pages, own-suite in-progress, `timed_out`, `queued`, failing stub, `args.log`); the eight-scenario counts; the ADR verdict; the operator-boundary citation; arc42 chapters 2, 5 and 8; the script metadata and local-parser note.
- Delta spec: all-skipped and unreadable-source scenarios; the atomic-push scenario now covers both suites; the use case says the release workflow's own **suites**, adds the unreadable-source rule, the no-success extension and three examples.
- `givn lint --change wait-for-green-checks-before-publish` parses cleanly; the findings are the expected `@wip` notices.

No `@e2e` tag was added or removed. No mandatory check is deferred beyond the recorded host boundary.

## Branches with no issue found

- **Scope** — the delta's rules, extensions and examples map to the proposal; nothing outside it.
- **Tech mechanism** — endpoint, `check_suite.id`, `--paginate --slurp`, `jq` flattening, `SECONDS`, `--interval 0`, `--timeout 0`, the 40-minute backstop and the permissions were verified against the live API and the v0.3.0 data.
- **E2E fidelity / interaction coverage** — no new consumer action; the two inventory rows and their `@e2e` scenarios are unchanged; the gate is automation, not a consumer action.
- **Use case / Personas** — `publish-public-release` owns the behaviour; the two confirmed Personas are review lenses; no scope-rejection criterion triggers.
- **Decision ledger** — D1–D6 all resolve; D2 is corrected for plural suites.
- **Visual contract** — terminal; no new evidence owed; the gate output names checks and the bound, never a path or token.

## Independent arc42 derivation

| # | Verdict | Chapter checked |
|---|---|---|
| 1 | No | Goals unchanged; the new requirement belongs to chapter 10 |
| 2 | Yes | Drift-check rationale quotes the old publish behaviour |
| 3 | No | GitHub is already the external system |
| 4 | No | Local mechanism, not a strategy change |
| 5 | Yes | New release-toolbox block and `release.yml` responsibility |
| 6 | Yes | Gate step and node in the release scenario |
| 7 | Yes | Publish job description and unchanged permissions |
| 8 | Yes (recommended) | Publication gate concept beside the landing gate |
| 9 | No ADR | Qualification verdict recorded; no MADR |
| 10 | Yes | Release-gate requirement and quality-tree node |
| 11 | Yes | TD-009 closed with residual |
| 12 | Yes | **Publication gate**, **check run**, **check suite**, refine **Gate (on landing)** |

All 12 chapter files exist with real content; no ASCII-art diagrams found.

DESIGN-REVIEW: PASS
