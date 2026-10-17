# Proposal: wait-for-green-checks-before-publish

## Release note

A release is published only after every check on the tagged revision has concluded successfully, so the branch and tag can be pushed together without racing the gate.

## Use-Case Context

- Use-case ID: `publish-public-release`
- Ideation topic: none — this change comes from a recorded technical debt (`docs/arc42/11-risks-and-technical-debt.md`, TD-009) and an observed failed release, not from an ideation seed.
- Seed path: none
- Confirmed Personas: `project-maintainer--release-owner` (the Actor's review lens: whether a version can be published without racing its checks) and `crew-member--personal-calendar-owner` (the reader lens: whether a published version is one the checks actually passed). Confirmed by the change brief; Actors in scenarios remain domain roles, not Personas.

## Problem / Opportunity

The publication step reads the checks on the tagged revision once, at the moment publication begins. It refuses a check that has already concluded non-success, and it requires at least one successful check, but it ignores a check that is still queued or running. A release therefore does not wait for the very evidence it claims to require.

Two consequences are observable:

- The documented release route — push the default branch and the version tag together — races the gate. The branch push starts the checks, and the tag push starts publication before those checks conclude.
- A check that would have failed can conclude after the release is already public. Because a published release is immutable, the project then has a version published without a green check set and no way to repair it in place.

This was observed on 2026-10-17 with version 0.3.0: the first push of `main` and the tag together produced a failed `ci` run and publication refused. The safe workaround was to push `main`, wait for the checks to be green, then push the tag. The atomic push documented in `README.md` — which exists so a rejected branch push cannot leave a tag published against a revision that never landed — remains racy. The release gate should make that documented atomic push safe instead of forcing the maintainer to serialize two pushes by hand.

## Proposed Solution

- Publication waits until every check run on the tagged revision, other than the release workflow's own check suite, has concluded. A queued or running check is waited for, not ignored.
- Once the checks conclude, publication refuses when any concluded check is not a success. A skipped check and a neutral check are not refusals — they are the outcomes an intentionally absent or not-applicable check reports, and refusing them would make a green landing set unpublishable.
- At least one concluded success is required. A tagged revision that reports no check evidence outside the release workflow's own suite is refused rather than published on faith.
- The wait is bounded. A check that never starts, or never concludes within the bound, causes publication to refuse and names every check still running, so nothing becomes public while a check is outstanding.
- The refusals and the wait are stated in the publication output, naming the checks involved, so the maintainer can act on the outcome without reading the workflow definition.
- Everything about publication after the gate is unchanged: the release is still prepared as a draft, inspected while still a draft, and only then made public; the published release is still immutable; no asset is attached by hand.
- The documented atomic push of the default branch and the tag stays as it is and becomes safe, because publication waits for the checks the branch push started instead of refusing them as not yet concluded.

## Capability Routing

`givn spec route --change wait-for-green-checks-before-publish` reported signal and recommended `EXTEND export-duty-plan-calendar` (score 13.00) over `EXTEND publish-public-release` (score 12.00).

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `publish-public-release` — the gate that decides whether a version may become public | `EXTEND export-duty-plan-calendar` (score 13.00, confidence: signal) | `EXTEND publish-public-release` | Route's top candidate matched the word “route” in the change text and generic goal words. This change has no conversion behaviour and no crew-member Actor: it is the release gate owned by `publish-public-release`, whose Actor is the release maintainer and whose trigger is publishing a version. `export-duty-plan-calendar` has no publication step to extend. |

## Out of Scope

- Changing what the project's checks are, what each one runs, or which checks the landing ruleset requires.
- Requiring publication to name a fixed subset of checks instead of reading the check runs that report on the tagged revision.
- Changing the draft-first, inspect-then-publish order, the attestations, the bill of materials, the published asset set, or release immutability.
- Changing the documented release order: the atomic branch-and-tag push stays. It becomes safe because publication waits; the runbook is not rewritten into a serialized order.
- Handling check runs from apps other than the release workflow's own suite differently from today — every other check run on the revision is part of the gate.
- Acting on the release automatically after the gate refuses: a refused publication stays refused until the maintainer acts.
- The repository-owner settings and host-side behaviour covered by the existing use case.

## Open Questions

### D1 — Where the bounded wait lives

- Question: should the wait be a repository command the maintainer can also run, or a few shell lines embedded directly in the release automation?
- Options:
  1. **A repository command** — one executable command under the project's release tooling, called by the release automation; gain: the suite can drive it as a real subprocess with a stubbed check-run source, and a maintainer can run it locally; cost: one more file and its command-line surface.
  2. **Inline in the release automation** — the wait is written in the publication job's shell; gain: no new file; cost: the behaviour can only be exercised on GitHub, so its waiting, timeout and refusal rules are proven solely by static workflow checking and the first tagged run.
- Disposition: carried — `design.md` decides, with the test location and the exact command surface; owner: the design author.

### D2 — What counts as a green check set

- Question: does “green” mean every check run that reports on the tagged revision, or a fixed list of named required checks?
- Options:
  1. **Every check run outside the release workflow's own suite** — gain: a check added later is covered without editing the gate, and the gate agrees with what the landing ruleset reports on the revision; cost: publication waits for every reporting check, including one the landing ruleset does not require.
  2. **A fixed list of named checks** — gain: publication waits only for the names the project chooses; cost: a new check silently escapes the gate until someone edits the list, which is the defect this change closes.
- Disposition: answered — option 1, from the change's required outcome and TD-009, which name every check the branch requires as reporting on a pushed revision. Source: this change brief and `docs/arc42/11-risks-and-technical-debt.md` TD-009.

### D3 — How long publication waits and how often it looks

- Question: what polling interval and total wait bound should the gate use so a normal release never hits the bound while a stuck check cannot hold publication forever?
- Options:
  1. **Short** — poll every 10 seconds for 5 minutes; gain: a refusal is quick; cost: a normal full check set can exceed the bound and a healthy release is refused.
  2. **Generous** — poll every 15 seconds for 30 minutes; gain: a full check set, including coverage and the Windows cross-build, concludes well inside it, and the request count stays far below the API limit; cost: a genuinely stuck check holds the publish job for up to the bound before refusing.
  3. **Very long** — poll every 30 seconds for 60 minutes; gain: maximum tolerance for a slow runner; cost: a stuck gate occupies the release job for an hour and delays the maintainer's diagnosis.
- Disposition: carried — `design.md` decides the interval, the bound, the API call count and the exact refusal text; owner: the design author.

### D4 — Whether the runbook changes or the atomic push becomes safe

- Question: should `README.md` stop documenting the atomic push of the branch and tag and document a serialized order, or should the gate make the atomic push safe?
- Options:
  1. **Keep the atomic push and make it safe** — the gate waits for the checks the branch push started; gain: one documented route that is both resistant to a rejected branch push and green-gated; cost: publication takes longer because it waits.
  2. **Document a serialized order** — push the branch, wait for the checks, then push the tag; gain: publication starts on an already-green revision; cost: the atomicity guarantee is lost — a correction pushed between the two pushes leaves the tag pointing at a revision that never landed — and the maintainer must watch a clock.
- Disposition: answered — option 1. The change brief prefers it, and the gate now closes the race that made the atomic push unsafe. Source: this change brief.

### D5 — A queued check, and a skipped or neutral conclusion

- Question: how does the gate treat a check that is queued and never starts, and how does it treat a check that concluded skipped or neutral?
- Options:
  1. **A queued check is still running; skipped and neutral are allowed** — the queued check is waited for under the same bound, and the refusal on the bound names it; gain: the gate never publishes while any check is outstanding, and an intentionally absent check does not block a release; cost: a check stuck in the queue holds publication until the bound, then refuses.
  2. **A queued check refuses immediately** — gain: fast failure; cost: the normal queue delay of a just-started check refuses healthy releases, which is the spurious-refusal problem this change removes.
  3. **Skipped or neutral refuses** — gain: a stricter reading of “green”; cost: checks that are intentionally skipped or informational make a healthy release unpublishable.
- Disposition: answered — option 1, from the change's required outcome: publication waits for every check to conclude, and skipped and neutral remain allowed as the current gate already treats them. Source: this change brief.

### D6 — How a gate that runs only on the host is proven

- Question: the gate executes inside the publication job on GitHub, which the development environment cannot invoke; how is its behaviour proven before a tagged release depends on it?
- Options:
  1. **A repository command driven by the suite with a stubbed check-run source, plus the workflow linter** — gain: the waiting, refusal, timeout and green-set rules are exercised locally through the real command, and the workflow definition is statically checked; cost: a test twin of the check-run API must be maintained, and the last mile — the real API and job ordering — is still proven by a tagged run.
  2. **Workflow linter only** — gain: nothing extra; cost: none of the gate's decisions are exercised before a real release, and the first proof is an irreversible publication.
  3. **No automated proof** — gain: least effort; cost: the change is unverified until it runs against a real tag.
- Disposition: carried — `design.md` decides the mechanism and records the host-only limit honestly; owner: the design author.
