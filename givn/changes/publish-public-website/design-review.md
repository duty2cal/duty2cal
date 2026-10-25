# Design review: publish-public-website

- Change: `publish-public-website` — publish the Public website from a named Built revision.
- Reviewed: 2026-10-25, after the plan (proposal, specs, design, arc42 docs) and before `tasks.md`.
- Grilling: a fresh read-only subagent (`PlanGrillA1`) walked every required branch against the artifacts and the harness source; the orchestrator verified each material claim against the repository before presenting a question.
- Outcome: **PASS**. Every grilling question is answered, every operator decision is recorded below, and the hardening edits are applied.
- Inputs: `givn/changes/publish-public-website/{proposal.md,specs/**,design.md,arc42.md}`, `givn/ideation/public-website/**`, `docs/arc42/**`, `docs/design/**`, the harness at `/home/dan/dev/PRIVATE/givn`.
- Verification at sign-off: `givn lint --change publish-public-website` → **exit 2** (28 findings: 23 `@wip`, 5 pre-existing `PERS` advisories); no `UCDOC`, overlap, long-scenario or interaction finding. Every `@e2e` tag is intact. The approved reference renders at 360×780 and 1920×1080 with the icon loaded (natural size 1044 px) and nine statement elements.

## Grilling findings and dispositions

| # | Finding | Disposition |
|---|---|---|
| F1 | `@site-publication` sat only on the Feature line; the archive merge writes a new permanent feature header and drops it, so the publication gate would select its own scenarios and recurse after archive. | **Applied.** Every non-`@e2e` site scenario now carries `@site-publication` at scenario level; the gate's filter is documented as relying on that. |
| F2 | A release tag started two site publications (`web.yml` on `v*` and the release job's own call), racing on two runners. | **Applied.** `web.yml` has no `v*` trigger; the release's `site` job calls the reusable `site.yml`; one tag publishes once. |
| F3 | The release job's own untracked `staged/` and `sbom/` trees made the cleanliness check refuse every release; the design also dropped the `web/LICENSE.txt` ignore entry while the file remains. | **Applied.** Cleanliness is `git status --porcelain --untracked-files=no`; the stale `web/LICENSE.txt` is deleted; `publish-release.sh` takes `--site-repo`. |
| F4 | The gate could pass with zero selected scenarios; `run-tests.sh` writes counts only when `GIVN_RESULT_FILE` is set. | **Applied.** Both gate checks set `GIVN_RESULT_FILE`; `total == 0` refuses, and the browser check requires exactly one scenario. |
| F5 | CI checkouts are shallow, so `--since` and `git describe` could not resolve. | **Applied.** `fetch-depth: 0` in the site caller and the release jobs. |
| F6 | The `@e2e` asserted the literal SHA `abc1234` with a fixture of two commits after the tag. | **Applied.** Fixture creates three commits; the expected footer text is derived from the fixture's own `git describe`. |
| F7 | The other-origin refusal named the browser check although the content check runs first and the CSP blocks runtime fetches. | **Applied.** Static foreign asset → page-content check (own scenario); runtime request attempt → browser check, with the driver's `requestfailed` log as the observation. |
| F8 | No scenario for a failing conversion check, an unresolvable `--since`, or the lockfile guard; two quality scenarios asserted unexercised behaviour. | **Applied.** Three scenarios added; QS-031, QS-032 (implicit) and QS-038 name them. |
| F9 | “Converter bundle was not built” was unreachable: the publication always runs the browser build first. | **Applied.** Scenario retargeted to “A browser build that fails refuses the publication and names the build step”; the assembler's missing-bundle refusal stays a defensive check. |
| F10 | The release use case said the site is published before the checks; the design publishes after. | **Applied.** Use case main flow is checks → site → archive. |
| F11 | `publish-release.sh` compared the draft assets with a set that still contained `release-notes.md`, and had no directory for the site publication. | **Applied.** The comparison excludes `release-notes.md`; `--site-repo` added. |
| F12 | Adding the capability to the change-log allow-list before the use case is permanent makes the renderer refuse. | **Applied.** The allow-list entry lands with the archive merge; the release-announcement fixture's capability list gains the capability in the same pass. |
| F13 | The visible change had no approved rendered reference; the committed one showed only the old footer. | **Applied.** Three rendered placements were produced; the operator chose the own-panel placement and then specified the icon and panel split; the reference is extended and re-rendered at both viewports. |
| F14 | The use case promised a named serving failure the design could not produce. | **Applied with an operator decision (Q2).** The publication now requires `--verify-url` and names a served/revision mismatch. |
| F15 | The design deviated from the seed's user-confirmed “one reusable workflow” without an operator record. | **Applied with an operator decision (Q3).** A reusable `site.yml` serves both callers; the amendment is recorded in the design and here. |
| F16 | The `@e2e` proved only a static render; the specs instruction requires a rendered-state change between two observations. | **Applied.** The `@e2e` observes the empty state and then the ready state after a supported plan; the baseline claim names only the states traversed. |
| F17 | The Interaction Coverage Matrix omitted the extended capabilities' inventory rows. | **Applied.** Four inherited rows added (release ×2, profile ×2), marked unchanged with their existing `@e2e` titles. |
| F18 | Republishing an unchanged revision would fail on an empty commit; the serialization scenario asserted too little. | **Applied.** The command reports `already current` and exits 0 without a commit; the scenario asserts both runs exit 0. |
| F19 | The use-case Trigger named no actor. | **Applied.** The Trigger names the Release maintainer. |
| F20 | Chapter 08 said `web.yml` was the only workflow holding the deploy-key secret, while the release job also holds it. | **Applied.** Chapter 08 names `site.yml` and its two callers accurately. |
| F21 | The proposal undercounted the release delta and the design's file table omitted the permanent export-use-case correction. | **Partly applied.** Traceability rows and the `givn/specs/export-duty-plan-calendar/usecase.md` row are added. The proposal is **not** edited: the review instruction forbids proposal edits outside a fundamental scope mismatch, and the count is prose, not a plan decision. Recorded as a residual note below. |
| F22 | Seed references were missing from the decision ledger. | **Applied.** `D1 (seed Q8, amended 2026-10-19; operator amendment 2026-10-25)`, `D2 (seed Q8, amended 2026-10-19; operator amendment 2026-10-25)`, `D3 (seed Q5)`, `D6 (seed Q7)`. |

## Operator decisions

### Q1 — Visual direction for the statements block and footer

- Status: closed 2026-10-25 (operator).
- Blocks: whether implementation may start against an approved rendered reference.
- Context: the page gains a public-statements block and a footer. The committed reference (`docs/design/reference/converter-reference.html`) was approved on 2026-10-19 and showed only the old privacy footer. The design claimed the reference would be extended during implementation, which is not an approved rendered direction.
- Question: how should the extended reference be approved?
- Options:
  1. **Extend the committed reference and approve it before tasks.** Gain: an approved rendered direction exists before code; cost: one reference-and-screenshot round trip.
  2. **Produce two or three rendered placements, operator picks one.** Gain: a real visual choice; cost: extra rendering and review.
  3. **Treat the 2026-10-19 textual direction as covering the addition.** Gain: no round trip; cost: the visible change has no approved rendered reference, so the visual branch fails.
- Resolution: **option 2** — three placements were rendered at both viewports (statements inside the converter panel; own panel above; own panel above with a condensed footer). The operator then specified the final direction instead of picking one of the three: statements panel above, a separate “Source and licence” panel below, full footer, icon left of the heading and subheadline, and the download wording from Q6. The reference was rebuilt to that direction and re-rendered.
- Rejected: option 1 (the operator wanted a real placement choice, not one extension); option 3 (no rendered approval, which the visual gate forbids).

### Q2 — Serving failure after the content replacement

- Status: closed 2026-10-25 (operator).
- Blocks: what the success guarantee may promise after the target's content is replaced.
- Context: the use case promised that a serving failure “is named” (extension 5, example, minimal guarantee). The design and deployment chapter said the publication never claims the page is served, because serving is the hosting service's interface. Both cannot hold.
- Options:
  1. **Host-reported framing.** The contract says the target holds the complete content, the host reports its own failure, and the project never reports a mixed page. Gain: implementable without a live service; cost: no project-visible serving failure.
  2. **Post-push HTTP probe.** The publication fetches the canonical address and names a mismatch. Gain: a project-visible failure; cost: a live service in the check and a failure after the replacement that cannot restore the previous page.
  3. **Leave both texts.** Cost: contradiction; sign-off fails.
- Resolution: **option 2.** `--verify-url` is required; after the read-back the command reads the address with bounded retries and refuses unless the served page carries the same Built revision text, naming both revisions. The check runs after the replacement, so it never claims restoration. Tests use a loopback server over the target's tree (passing) and over the earlier tree (refusing).
- Rejected: option 1 (the operator wanted a project-visible check); option 3 (contradiction); making `--verify-url` optional (the default path would be the unverified one).

### Q3 — Shared build-and-deploy implementation

- Status: closed 2026-10-25 (operator).
- Blocks: the mechanism recorded against the seed's 2026-10-19 Q8 decision.
- Context: the seed says “one reusable build-and-deploy workflow serves both callers”. The design shared one command instead, because workflow YAML cannot be tested, and it left the `v*` trigger in `web.yml` while the release job published the site again.
- Options:
  1. **Accept the command as the shared implementation.** Gain: every promise is a subprocess a scenario drives; cost: the seed's literal wording changes and the tag trigger must move.
  2. **Reusable workflow wrapper around the command.** Gain: the seed's wording survives; cost: one indirection whose calling contract no scenario proves, and the tag trigger still moves.
  3. **Keep the tag deploy in `web.yml`; the release waits for that run.** Gain: closest literal reading; cost: the release gate depends on another workflow's timing, untestable and racy.
- Resolution: **option 2.** `.github/workflows/site.yml` declares `workflow_call` and runs `scripts/publish-site.sh`; `web.yml` triggers on default-branch pushes and manual dispatch and calls it; `release.yml`'s `site` job calls it for the tag, and `publish` needs `site`. `web.yml` has no `v*` trigger. The design records the operator amendment to Q8.
- Rejected: option 1 (the operator wanted the reusable workflow in the plan); option 3 (untestable gate).

### Q4 — Keeping the release gate testable under the reusable workflow

- Status: closed 2026-10-25 (operator).
- Blocks: how “a failed site publication publishes no release” is observable when the site deploy is a workflow job.
- Context: GitHub allows a reusable workflow only at job level, so the site-before-archive order stops being one command's sequence.
- Options:
  1. **Verify the target's Built revision in the release command.** Gain: the gate is subprocess behaviour against a local bare target; cost: the release checks the target, not the CDN.
  2. **Rely on the job dependency alone.** Gain: no new code; cost: the gate scenario becomes unwritable, an unenforced gap.
  3. **Keep the site publication inside the release command.** Gain: one command proves the order; cost: contradicts the wrapper choice.
- Resolution: **option 1.** `publish-release.sh` reads the target's branch tip and refuses before any `gh` call unless it carries the tagged Built revision; the release scenario drives that against a local target.
- Rejected: option 2 (unobservable); option 3 (contradicts Q3).

### Q5 — Icon licence

- Status: closed 2026-10-25 (operator).
- Blocks: whether the page's licence statement can be true for the icon it publishes.
- Context: the project icon (`docs/assets/duty2cal.png`) was covered by the aggregate MPL-2.0 annotation while the page says non-code assets are CC0-1.0.
- Options:
  1. **Keep the icon MPL-2.0 and name it in the statement.** Gain: no relicensing; cost: a longer statement that splits the non-code category.
  2. **Re-license `docs/assets/**` as CC0-1.0 via `REUSE.toml`.** Gain: the planned statement stays true; cost: an existing published asset changes its declared licence.
- Resolution: **option 2.** `REUSE.toml` gains the `docs/assets/**` override; the page statement is unchanged.
- Rejected: option 1 (splits the licence statement the use case fixes).

### Q6 — Download wording across public copy

- Status: closed 2026-10-25 (operator).
- Blocks: whether the page may say “downloads for various operating systems are available on request” while the README and release notes say other platforms are “not supported or distributed”.
- Context: the operator removed the Windows imperative from the page. The durable public text still carried the Windows-only distribution claim.
- Options:
  1. **Published archive stays Windows-only; say “on request” everywhere.** Gain: one coherent claim and Q1's published-archive rule intact; cost: profile and release wording change.
  2. **Align all public copy to “downloads for various OSes on request”.** Gain: no page qualifier; cost: reverses the 2026-10-19 Q1 public-copy commitment and promises builds the project does not automate.
  3. **Leave the contradiction.** Cost: sign-off fails.
- Resolution: **option 1.** The page says “Downloads for various operating systems are available on request; the release page carries the published builds”, linking `https://github.com/duty2cal/duty2cal/releases`. The profile, README, release text, quality scenario QS-022, risks and glossary say Windows x86_64 is the **only published archive**, with builds for other operating systems available on request; no other archive is published or tested by the automation.
- Rejected: option 2 (unautomated commitment and reversal of Q1); option 3 (contradiction).

### Q7 — Approval of the revised rendered reference

- Status: closed 2026-10-25 (operator direction).
- Context: after Q1, the operator directed the icon left of the heading and subheadline, Source and Licence in a separate lower panel, and the Q6 wording. The reference was rebuilt to that direction.
- Resolution: the operator's direction is the approved direction; the committed reference and the renders under `givn/changes/publish-public-website/evidence/visual/reference/` implement it, and `design.md` and `docs/design/design-system.md` record the 2026-10-25 approval. Rejected: the earlier three-way placement sheet (superseded by the explicit direction); keeping Source and Licence in the footer (the operator moved them).

## Branch walk log

| Branch | Result | Evidence |
|---|---|---|
| Scope | clean after F2/F8/F21 | Every proposal bullet traced to a scenario; the site's archive statement now matches the operator's wording; the release delta's three obligations are traced. The proposal's undercount remains recorded as prose only. |
| Tech choices | clean after F2/F3/F5/F11/F12 | Pins verified from the files that record them (Playwright 1.64.1 and axe 4.14.0 in `web/e2e/package-lock.json`, Node 24 in `.nvmrc`, git-cliff 2.14.2 in `cliff.toml`). |
| Missing scenarios | clean | Conversion failure, unresolvable `--since`, lockfile guard, idempotent republish, serving mismatch and the build failure now each have a scenario. |
| Testability | clean after F4/F6/F9/F18 | Strict mode `.fail_on_skipped()`; stub pattern = the commands fail closed and the recording `gh` stand-in; single-scenario command `./run-tests.sh --name "<title>"`; result-file counts close the zero-selection hole. |
| E2E fidelity | clean after F6/F7/F16 | One normalized inventory entry, one `@e2e`, real subprocess + real Chromium, primary assertions on the target tree and the rendered page; static-asset and runtime-request cases split. |
| Interaction Coverage | clean after F17 | Five rows: the new leaf plus four inherited entries. |
| Risk | clean | Recorded risks cover the deploy key in CI, whole-content deletion, serving propagation after the replacement, gate flakiness, stand-in fidelity, copy/page ordering, per-push runner starts, stale worktrees and the borrowed driver tree. |
| Use case / Persona | clean after F19 | Goal is one verb phrase; the Trigger names the Release maintainer; Actors are roles, Personas are lenses; every extension references a step. |
| Decision ledger | clean after F22 | `D1`–`D7` resolve in `design.md` with rejected alternatives; the seed references are kept. |
| Visual design contract | clean after Q1/Q7 | `Interface: web`, viewports 360×780 and 1920×1080, tokens reused, primitives and states, capture mechanism, approval 2026-10-25, committed reference and two renders. |
| Deferrals | clean | No mandatory check deferred; the `gh` host is an explicit recording twin; no `@e2e` removed. |
| Persona disposition | clean | Release owner (claims, Built revision, coupling), web reviewer (CSP, no-network, artifact, gate), visitor (copy before the picker), crew member (route and file), organization owner (credential and profile apply, supporting). All recorded with relevance and evidence; at least one confirmed Persona backs the visual contract. |
| Interface classification | clean | Web UI for the published page and CLI for the publication command; the opposite classification was tested — treating the page as a non-browser surface fails because the `@e2e` renders it in Chromium and asserts rendered state. |
| ADR qualification | clean | Four candidates, all `NOT_QUALIFIED`: publishing topology → chapter 7; release coupling → the release feature; publication mechanism → `design.md`; serving check → the publication feature. `existing_adr_check` ran against ADR-0001–0007 and chapter 09; no MADR is owed and no `NEW_ADR` is proposed. |
| arc42 | clean after F20 | Independent 12-row derivation below; chapter 08 corrected; chapter 09 carries the four verdicts; chapter 11 carries the new serving risk; all 12 files exist with substantive content; no box-drawing characters. |
| Glossary | clean | **Public website**, **Site publication**, **Site output** and **Built revision** were already present; the change adds no term. **Supported platform** and the profile rows now say *published* archive with builds on request; use is consistent with the specs and design. |
| Lint | clean (warnings only) | `givn lint --change publish-public-website` → exit 2: 23 `@wip` (every new scenario) and 5 `PERS` advisories that pre-date the change. The persona links are written for the permanent merge target (`givn/specs/...` → `../../personas`), where they resolve; the change-dir copy triggers the advisory. |

### Independent arc42 chapter derivation

The reviewer derived its own Yes/No per chapter from proposal + specs + design **before** opening `arc42.md`; the two sets match on every row.

| # | Chapter | Reviewer | `arc42.md` | Match |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Yes | ✓ |
| 2 | 02 architecture-constraints | Yes | Yes | ✓ |
| 3 | 03 context-and-scope | Yes | Yes | ✓ |
| 4 | 04 solution-strategy | Yes | Yes | ✓ |
| 5 | 05 building-block-view | Yes | Yes | ✓ |
| 6 | 06 runtime-view | Yes | Yes | ✓ |
| 7 | 07 deployment-view | Yes | Yes | ✓ |
| 8 | 08 crosscutting-concepts | Yes | Yes | ✓ |
| 9 | 09 architecture-decisions | Yes | Yes | ✓ |
| 10 | 10 quality-requirements | Yes | Yes | ✓ |
| 11 | 11 risks-and-technical-debt | Yes | Yes | ✓ |
| 12 | 12 glossary | Yes | Yes | ✓ |

Content checks: chapter 08's credential sentence now names `site.yml` and its two callers; chapter 07 records the reusable workflow, the target verification, the required serving check and the eight-member output; chapter 10 carries QS-031/QS-038 scenarios and QS-039; chapter 11 revises R-059 and adds the serving-propagation risk R-066; chapter 12 keeps “no new term” true.

## Hardening applied

- `givn/changes/publish-public-website/design.md`: reusable `site.yml`, target verification, required `--verify-url` with D7, untracked-files cleanliness, idempotent republish, eight-member output with `icon.png`, both statement panels and the prose-only footer, icon left of the heading, REUSE override, `--site-repo`, asset-comparison exclusion, `fetch-depth: 0`, result-file contract, scenario-level `@site-publication`, three-commit fixture and derived footer, second rendered-state observation, four inherited matrix rows, traceability and seed-event rows, risks, local twins, seed references.
- `givn/changes/publish-public-website/specs/publish-public-website/publish-public-website.feature`: 19 scenarios, all `@site-publication`; the `@e2e` asserts the icon, both panels, the prose footer, the derived revision text, no foreign request and the empty→ready rendered change; added scenarios for the browser-build failure, content-check refusal of a static foreign asset, conversion-check failure, unresolvable comparison base, differing driver lockfile, unchanged revision and serving mismatch.
- `givn/changes/publish-public-website/specs/publish-public-release/publish-public-release.feature`: the public-text wording and the release-site gate now assert the only-published-archive plus on-request rule and the target verification.
- `givn/changes/publish-public-website/specs/apply-configuration-change/publish-organization-profile.feature`: the profile names the only published archive and the on-request builds.
- `givn/changes/publish-public-website/specs/publish-public-website/usecase.md`: actor-named Trigger; serving extension, examples and minimal guarantee reworded for the post-replacement check; icon and on-request route rules.
- `givn/changes/publish-public-website/specs/publish-public-release/usecase.md`: main flow ordered checks → site → archive; target verification; on-request builds wording.
- `givn/changes/publish-public-website/arc42.md`: re-derived rows 5/7/8/10/11; four ADR verdicts; member set.
- `docs/arc42/01,02,03,04,05,06,07,08,09,10,11,12`: reusable workflow, serving check, target verification, icon, statement panels, on-request builds, output members, credential consumers, QS-031/038/039, R-066, glossary rows.
- `docs/design/design-system.md`: statements panel above, Source-and-licence panel below, prose footer, header icon/favicon primitive, `img-src 'self'`, 2026-10-25 approval.
- `docs/design/reference/converter-reference.html`: implements the approved reference; renders at both viewports; approval evidence at `givn/changes/publish-public-website/evidence/visual/reference/{360x780,1920x1080}.png`.

## Residual notes (not blocking)

- The proposal's Q1 disposition still says “only downloadable archive platform”. It remains true in the sense of published archives, and the review instruction forbids proposal edits outside a fundamental scope mismatch; the design, specs and durable docs carry the precise wording.
- The five `PERS` lint advisories come from persona links written for the permanent merge target; they resolve after archive at `givn/specs/publish-public-website/usecase.md`.
- The visual-placement scratch files live under the ignored `reports/visual/` tree; only the approved reference and the two renders are kept.
- Chapter 12's duplicate rows for **Distributed executable** and **Size attribution** come from the concurrent size work, not from this change.

## Sign-off

Every grilling question is resolved or recorded, the hardening edits are applied, the approved rendered direction exists before implementation, and no mandatory check was deferred.

DESIGN-REVIEW: PASS
