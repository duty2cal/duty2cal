# Design review — reduce-test-runtime

Fourth design review, held after the third review failed on its items 1, 3 and 5.
Every conclusion below was re-derived from the current files; the third review's
statements were treated as claims to check, not as evidence. Read-only: no build,
no `cargo` run, no `bats` run and no network call. Evidence is the file content,
the CI step order and the script bodies.

Reviewed: `proposal.md`, `design.md`, `arc42.md`,
`givn/changes/reduce-test-runtime/.givn-skip`, `docs/arc42/02,05,06,07,08,10,11`,
`.github/workflows/{ci,site,release}.yml`, `run-tests.sh`, `measure-coverage.sh`,
`scripts/setup-infra-tests.sh`, `scripts/site-evidence.sh`,
`scripts/assemble_features.py`, `scripts/tests/publish-site.bats`,
`apps/cli/tests/features_runner.rs`, `apps/cli/tests/golden.rs`,
`apps/cli/tests/steps/{mod,support}.rs`, the deleted step files at `HEAD`,
`givn/specs/*/*.feature`, `givn/specs/export-duty-plan-calendar/usecase.md`,
`README.md`, `.github/pull_request_template.md`.

## Status of the third review's items

| # | Item | Status |
|---|---|---|
| 1 | arc42 honesty: stale operational-suite claims corrected or covered by a precise amendment; row 6 honest; `STATUS: DONE` honest | **Resolved** |
| 2 | `setup-infra-tests.sh` documentation matches its actual fake-API proof | **Resolved** (unchanged) |
| 3 | Size-budget prose consistent (2,097,152 ceiling; 2,013,696 local cross-build; 2,014,208 landed build) | **Resolved** |
| 4 | Attribution produced and uploaded before a budget failure can end the job | **Resolved** (unchanged) |
| 5 | Chapter 5 and 7 diagrams show the CI attribution and the artifact | **Resolved** |

## Verification of the three corrections

### Item 1 — the arc42 notes name only owners that perform the proofs

Every affected chapter now carries the same test-scope amendment at its head:
`02-architecture-constraints.md:3-10`, `06-runtime-view.md:3-9`,
`07-deployment-view.md:3-9`, `08-crosscutting-concepts.md:3-9`,
`10-quality-requirements.md:3-9`, `11-risks-and-technical-debt.md:3-9`. Each says
the operational files remain `@documentation` with their step definitions deleted,
that the executable evidence is the owning workflow, the `bats` tests, or the
maintainer's preview/apply, that `scripts/setup-infra-tests.sh` covers only the
provider/import/base setup, and that the product suite is the conversion
capability. Chapter 5 has no amendment block but was corrected directly
(`05-building-block-view.md:59-85` test harness, `:132-136` publication,
`:222-223` setup and configuration step definitions), and no stale suite claim
remains there.

The script was read end to end. It checks `tofu`, `python3`, `jq` and the
platform, mirrors both locked providers (`integrations/github`,
`hashicorp/tls`), and runs two proof trees:

- `adopt-all` (`:105-236`): one fresh-workspace `tofu import` per batch entry, one
  shared-workspace import, a refresh-only no-op and `assert_no_unknown_paths`.
- `security-applied` (`:232-311`): the same imports, the *base* declaration
  applied for real with `org_profile.tf`, `pages_publish.tf` and the
  `security_and_analysis` block removed (`:252-270`), a follow-up plan that must
  be a no-op, then the Dependabot batch and the unknown-path check (`:298-311`).

It calls neither `scripts/adopt-public-home.sh`, `scripts/publish-site.sh` nor
`scripts/check-drift.sh`; it never reads `org/`, never drives a preview, an
apply, the site publication or the release, and its only journal assertion is
the unknown-path scan. The script is unmodified by this change (`git status`
shows no entry), so the documentation now describes what exists rather than a
tree the plan would have added.

Every reference matches that scope or explicitly denies the stronger claim:

- `02:80`: the local planning proof "exercises the real `tofu` binary and the
  provider's import/refresh/base-apply shape … The adoption command and each
  configuration change are exercised by the maintainer's real preview/apply".
- `05:222`: "It still runs the fake-API import proof for local planning; the
  cucumber suite no longer drives it."
- `06:490`: exercises "the provider's import and refresh-only shape against the
  fake, not the adoption command itself".
- `06:555`: "prepares a security-applied base state against the fake, but it does
  not drive the targeted preview or apply."
- `06:625`: "only prepares the inherited base state".
- `06:689`: "prepares only the imports and the base apply and does not inspect
  `org/` or drive the profile preview".
- `07:241`: "no automated scenario drives the profile publication".
- `07:317`: starts the fake "for the import/base proof only"; the maintainer's
  real preview/apply is the evidence.
- `08:364`: the script "does not inspect `org/`".
- `08:455-458`: the *deleted scenario*, not the script, performed the security
  change's scanning write; the script "prepares the imports and base apply only".
- `08:468-472`: "exercises the provider's import and refresh-only shape, not the
  adoption command".
- R-028 (`11:42`), R-032 (`11:46`), R-033 (`11:47`), R-055 (`11:64`) and R-061
  (`11:70`) now describe the deleted scenarios in the past tense and name the
  current owner instead.

No note now assigns a preview/apply, publication or release proof to
`scripts/setup-infra-tests.sh`, and R-067 records the deliberate gap. `design.md`
does not list the script among changed files, which is correct because the plan
no longer adds any proof tree to it.

### Item 3 — chapter 7's budget prose is internally consistent

`07-deployment-view.md:127` reads: the job "fails when the file size exceeds
2,097,152 bytes. The ceiling is the last published executable's file size; a
local cross-build of this revision measured 2,013,696 B, and the landed
panic=abort build measured 2,014,208 B." Each number now has its own role.

The same assignment appears in `02:50` (ceiling and 2,013,696 B local
cross-build), `10:102` (QS-041, the 2,097,152-byte gate), `11:77` (R-068, the
ceiling, the local build and the headroom), `design.md:22` and `proposal.md:78`.
`ci.yml:314-324` enforces `budget=2097152` with `stat -c '%s'` and fails only on
`-gt`. The 2,014,208 B / 2,098,176 B pair at `07:119` is the separate panic-lever
measurement (also `02:54`, `04:33`, `06:460`, `10:94`, `12:127`), not the budget;
no sentence presents it as the ceiling.

### Item 5 — chapter 7's deployment diagram is restored

`07:26` `b1["ubuntu-latest — Windows x86_64 cross-build + byte budget"]`, `07:27`
`attr["ubuntu-latest — per-push size attribution"]`, `07:35` `b1 --> attr`, and
`07:36` `attr --> art["workflow-run artifact: size-attribution + step summary"]`.
The prose at `07:60` and `07:129` keeps the attribution in
`windows-gnu-build`, matching `ci.yml:289-313` (attribution step then
`upload-artifact` with `if: always()`). Chapter 5's release diagram retains
`CI --> MEASURE`, `MEASURE --> REPORT`, `REPORT --> ART` at `05:167-169`. Both
diagrams now match the workflow.

### `arc42.md` row 6 and `STATUS: DONE`

Row 6 (`arc42.md:10`) is `Yes`; its reason records the evidence redistribution,
points to chapter 6's amendment, and no longer implies the local proof drives a
preview/apply. The amendment it points to names the maintainer's preview/apply
for the flows that have no automated proof.

`STATUS: DONE` (`arc42.md:20-24`) is honest: chapter 6's amendment is at its head,
each of the four changed operational notes (`06:490`, `:555`, `:625`, `:689`)
names its new owner, and the status sentence lists all four owner classes,
including the maintainer's preview/apply.

## Branch re-walk

### Scope

No scope issue. Proposal, design D1–D6 and the amended chapters agree on a
product-only `cargo test`, one retained browser smoke, one owner per operational
guarantee, a local drift check, a byte budget and the per-push attribution.
`.givn-skip` contains `specs` (no capability delta). Eight operational feature
files remain under a feature-level `@documentation` tag: adopt, security,
profile, plumbing, measure size, release, website and reduce size. The design no
longer needs a `setup-infra-tests.sh` entry because it no longer claims to add a
proof tree.

### Tech choices

The mechanisms hold: the assembler skips `@documentation`
(`scripts/assemble_features.py:104-113,194-196`); `run-tests.sh` normalises
`--tags=<expr>` into `--tags "<expr>"` and appends `and not @browser` to a
caller's expression; `.fail_on_skipped()` stays; `--no-fail-fast` reports every
class; the `browser` job checks out with `fetch-depth: 0`, builds the bundle and
Site output and runs the smoke by name; `coverage` is `main`-only and uses the
`not @e2e and not @browser and not @nested-build` / `@e2e and not @browser and
not @nested-build` pair; `site.yml` runs the golden test before the build. The
`--browser`/`--e2e` plus caller `--tags` combination still passes two `--tags`
arguments; cucumber-rs combines them, so it narrows or fails closed and cannot
bypass the browser exclusion. Non-blocking.

### Missing scenarios

No product scenario is lost: `export-duty-plan-calendar.feature` keeps 89
scenarios, 16 of them `@browser` (1 smoke + 15 variants, matching R-070). The
operational scenarios stay as documentation and the gap is recorded (R-067,
TD-025); the chapters no longer claim they execute.

### Testability

`run-tests.sh` refuses an empty selection, requires exactly one scenario for
`--name`, accumulates `status` across the Rust, golden and cucumber classes, and
sets `failed=1` when an earlier class failed before writing `GIVN_RESULT_FILE`.
`scripts/site-evidence.sh` requires `exactly-one` for the browser check and
`at-least-one` for the conversion suite, and the conversion run is scoped to the
capability with the `@e2e` and `@nested-build` exclusions.

### E2E fidelity

One CLI `@e2e` scenario and one `@e2e @browser` scenario. The browser job and the
publication run the browser scenario by name through the real Playwright driver;
the CLI scenario spawns the real subprocess. The publication's golden test is the
real binary conversion.

### Interaction coverage

The design's matrix matches `usecase.md` §Interactions — executable conversion and
converter-page save — one `@e2e` scenario each.

### Risk

R-067 and TD-025 now say the same thing as the chapters, and R-068 matches the
budget prose. The remaining risk statements distinguish deleted scenarios from
today's owners.

### arc42

Items 1–3 resolved; row 6 and the status are honest. Row 4's `No` remains
defensible (attribution and Windows release strategy unchanged; the budget is a
CI gate recorded in chapters 2 and 7).

### Ubiquitous language

No new domain term; `@documentation`, `@browser` and "browser smoke" are
engineering vocabulary described in chapter 8. No domain term is reused
inconsistently.

### Decision ledger, visual design, deferrals, interfaces, ADRs

Unchanged and still sound: D1–D6 are answered or recorded as design decisions;
the Visual Design Contract still classifies the interface as `web` with both
viewports, no new or visibly changed screen, the evidence path, the motion
assertion and the persona lens; the deferrals have operator decisions; no new ADR
is due.

## Non-blocking observations

1. `08:263` now reads "the suite excludes @documentation features tagged
   `@site-publication`"; the tag is on the scenarios inside the
   `@documentation` feature, and the exclusion is the assembler's feature-level
   skip plus the tag query. Wording only.
2. `07:309` says the `bats` tests cover "only the early refusals"; the file also
   asserts the documentation-only `--since` decision
   (`scripts/tests/publish-site.bats:93-114`), as QS-038 correctly states. The
   chapter understates its own coverage.
3. "~83 KiB" of headroom in proposal D3, `design.md:22` and R-068 is 83,456 B
   (81.5 KiB); a rounded byte count labelled KiB. Cosmetic and consistent across
   the three files.
4. `scripts/setup-infra-tests.sh:16` still says "CI runs it before
   ./run-tests.sh"; the trimmed `test` job no longer invokes it. The file is
   unmodified by this change and the arc42 text does not repeat the claim.

## Verdict

The third review's three failing items are resolved. The arc42 chapters now
certify only the proofs `scripts/setup-infra-tests.sh` and the workflows
actually produce, and the flows with no automated evidence are recorded as
maintainer evidence with R-067 as the gap. Chapter 7's budget sentence assigns
the ceiling, the local cross-build and the landed build to the right numbers, and
its deployment diagram again shows the per-push attribution and its artifact.
`arc42.md` row 6 and `STATUS: DONE` are honest.

The mechanical surface re-walked cleanly: the disclosure guards, the tag
handling, the browser smoke, the coverage filters, the site golden test, the
publication's bats cases and the budgets all match the design. The four
observations above are wording or comment nits, not evidence claims, and none
blocks sign-off.

DESIGN-REVIEW: PASS
