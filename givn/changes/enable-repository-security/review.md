# Review: enable-repository-security

Change: `enable-repository-security`
Stage: `ready-to-archive` (`givn status` reports 44/44 tasks)
Review date: 2026-10-18
Verdict: **FAIL** (F1–F9 are addressed and the artifact gates are green, but the
F8 scoping correction introduces a red scenario: the targeted security preview
no longer feeds the unadopted-setting guard)

## Preconditions

- `givn instructions review --change enable-repository-security`,
  `givn instructions specs --change enable-repository-security` and
  `givn instructions dialog` were loaded before this report was written.
- `givn status --change enable-repository-security` reports 44/44 tasks and
  stage `ready-to-archive`; all artifacts are present.
- This review was read-only except for writing this file. No other artifact,
  fixture, step definition, declaration or state file was edited.
- The suite was not re-run as a whole, per the assignment. The recorded counts
  below are the evidence supplied for this review. A single-scenario check was
  run on the current tree to verify the F8 scoping change, because the scoping
  commit (`7ececa9`) changed the shared `preview` seam; that check found a
  failing scenario (F10).
- `givn lint --change enable-repository-security` was run:
  `givn lint: 1 file(s) checked — clean`, exit 0.
- `givn check review --change enable-repository-security` was run with a
  review file present: it passes integrity, the run declaration, capability
  routing (net delta **8 added, 0 modified, 0 removed**) and overlap
  dispositions, exit 0. The mechanical gate does not execute the scenarios.

## Commands and scope under review

Read literally from `givn/commands.yaml`:

- `verify.command: "./run-tests.sh"`
- `verify.e2e_command: "./run-tests.sh --e2e"`

The strings differ. `run-tests.sh` maps `--e2e` to `--tags "@e2e"` and invokes
`cargo test -p duty2cal --test features_runner -- --tags "@e2e"` over the
feature root assembled by `scripts/assemble-features.sh` (the permanent corpus
plus the active change’s `specs/**`). `apps/cli/tests/features_runner.rs`
calls `Cucumber::fail_on_skipped()`.

Recorded execution evidence (from `tasks.md`; not re-run as a suite):

- Full suite: **140 scenarios (140 passed), 584 steps (584 passed)**, exit 0.
- E2E scope: **6 scenarios (6 passed), 42 steps (42 passed)**, exit 0.
- The e2e scope is strictly smaller than the full scope: **6 < 140**.
- The most recent archive receipt (`givn/archive/declare-public-home/verification.json`,
  candidate `47277e1`) records the full `verify.command` scope at **132**
  scenarios (including **4** `@e2e`) and the e2e scope at **4**; this delta adds
  8 scenarios (6 non-e2e, 2 `@e2e`), giving **140 full** and **6 e2e**.

**Current-tree execution check (F10):** the recorded 140/140 predates the F8
scoping commit `7ececa9`, which changed the shared `preview()` seam to
`-target=github_repository.mono` while leaving `GUARD_FILTER` reading the
plan’s `resource_changes`. Running the unadopted-setting scenario alone on the
current tree:

`./run-tests.sh --name "The preview refuses to add a Dependabot setting that already exists live and is unbound"`

exits 101 with **1 scenario (1 failed), 6 steps (5 passed, 1 failed)**. The
preview prints the targeted plan and exits 0 instead of refusing; the failing
assertion is `left: Some(0) right: Some(0)`. The recorded full-suite count is
therefore not a green claim for the current tree, and the archive gate’s
re-run will include this scenario.

## F1–F9 dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| F1 — task commits in the gate’s `COMMIT:` syntax | **Fixed.** Every scenario task records `- [x] COMMIT: \`<hash>\``. | `tasks.md` lines 90, 118, 148, 180, 210, 240, 305, 346; `givn check review` reports `✓ integrity passed`. |
| F2 — delta use case parses for capability routing | **Fixed.** The use-case header, sections and bullets are in the parser’s shape; `## Level` is `!`, the project convention. | `givn check review` reports no routing error and prints `net delta: 8 added, 0 modified, 0 removed`. |
| F3 — the delta use case states only what the eight scenarios assert | **Fixed as a scope decision.** The four removed behaviours are in `## Out of scope` and use “in this use case”; every remaining contract statement maps to a scenario. | `usecase.md` §“Out of scope”; the mapping under “Fabrication audit, step 12”. |
| F4 — commit the required real-apply transcript | **Fixed.** The transcript exists with the adoption refusal, both direct imports (the vulnerability-alerts import corrected to `duty2cal`; the refreshed id is `1416075332`), the targeted plan, the apply, the targeted follow-up, and the statuses; the preflight omission is recorded; the three post-apply JSON files exist. | `evidence/real-apply/transcript.txt`; `evidence/live-read/post-apply-*.json`; `evidence/live-read/README.md`. |
| F5 — design and arc42 state the targeted scope and the full-preview drift | **Content fixed.** `design.md` Post-GREEN steps 1, 3, 4 and 5 and the correction note say the targeted preview/preflight/failure report is scoped, the apply uses the saved plan without `-target`, and the full preview still names the organization billing drift; `docs/arc42/06` and QS-019/QS-020 say the same. | `design.md` §“Post-GREEN operator apply”, §“Live operator run”; `docs/arc42/06-runtime-view.md`; `docs/arc42/10-quality-requirements.md`. |
| F6 — remove fake-journal/HTTP-`PATCH` detail from the feature | **Fixed.** The feature has no `journal`, `PATCH` or “write request”; the read-back scenario and Given name the security settings; the last apply Then now says “a later targeted preview of the security settings reports no differences” and its step definition matches. | Feature and `apps/cli/tests/steps/e2e/enable_repository_security.rs`. |
| F7 — one success outcome and consistent saved-plan/declared-edit semantics | **Fixed.** One success outcome; the rule and the out-of-scope bullet agree; the live-edit extension is anchored to step 3. | `usecase.md` §§ Success guarantee, Rules, Extensions, Out of scope. |
| F8 — re-review the corrected design | **Process artifact fixed.** `design-review.md` has a “Final design review — 2026-10-18” with `DESIGN-REVIEW: PASS` appended in `7ececa9`, naming the targeted preview/preflight/failure report, the direct-import recovery, the preflight omission, the scoped use-case rule and the targeted feature Then. | `design-review.md` final section; `7ececa9`. The review’s claim that “the refusal scenarios pass” is contradicted by the unadopted-setting refusal (F10). |
| F9 — normalize the out-of-scope wording | **Fixed.** The four bullets say “in this use case”, not “in this delta”. | `usecase.md` §“Out of scope”. |

## Fabrication audit (steps 0–12)

### 0. `@e2e` tag integrity — clean

The delta feature contains exactly two `@e2e` scenarios, both
`@givn.added @e2e`: the preview and the apply scenarios. Git history shows both
tags introduced in `cafab30`; `@wip` was removed in `52b8c26` and `a8fbd40`.
No planned `@e2e` coverage was removed.

### 1. Empty or trivial step bodies — clean

**0 empty step bodies found across the 11 step-definition files scanned.** A
scan for `{}` bodies, whitespace-only bodies, `todo!`, `unimplemented!` and bare
`return`/`pass` found no step body. The two capability files have substantive
bodies. Every feature step text maps to a `#[given]`/`#[when]`/`#[then]`
definition, including the renamed targeted-preview Then.

### 2. Checked task → commit → source cross-check

Every checked scenario task records a revision, and `git show --stat` confirms
the revision touches the implementation surface, not only a spec/stub:

| Task | Revision | Commit content verified |
|---|---|---|
| Setup task + scenario 1 | `ec38b47` | `infra/security.tf`, `infra/repos.tf`, `infra/adoption-batch-security.txt`, `.gitignore`, the fake server and both fixture trees, the setup script, both step modules, `features_runner.rs`, tasks |
| 2 first change refused | `abd3bc2` | step failure path, runner/world, design/tasks correction, feature |
| 3 live edit | `e830590` | step preflight refusal, feature |
| 4 unadopted setting | `7287552` | step guard/live reads, feature |
| 5 read-back failure | `e19691d` | step fault and stale path, feature |
| 6 partial failure | `6a0f096` | off-routine apply helper, stale sidecar, feature |
| 7 preview `@e2e` | `52b8c26` | e2e steps, shared helpers, committed transcript, feature |
| 8 apply `@e2e` | `a8fbd40` | e2e steps, committed transcript, `security-applied` fixture, feature |

The product’s Rust crates are untouched by design; the change’s production
surface is the declaration plus the executable test seam. No commit is
spec-only.

### 3. Promised components — present

All components named by `design.md` exist: `infra/security.tf`, the
`security_and_analysis` block and the two dedicated Dependabot resources,
`infra/adoption-batch-security.txt`, the encrypted state and backup, the
`.gitignore` rules, the fake’s `--writes` sandbox, both fixture trees, the setup
script, both `mod.rs` files, both step files, the `DutyWorld` fields, the CI
setup step, the live-read evidence, the two visual transcripts and the
real-apply transcript. The design’s step-definition table omits the world’s
`write_sandbox` field (the implementation has it); this is a non-blocking
documentation omission.

### 4. Strict-mode proof — present

`apps/cli/tests/features_runner.rs` calls `fail_on_skipped()`. `tasks.md`
records the setup and e2e strict proofs with non-zero exits and `Step doesn't
match any function`; the feature has no `@wip` tag. The fake’s unknown-path 599
marker and refused-write journal are the second strictness net. The
single-scenario check above confirms strictness: the failed unadopted assertion
surfaced instead of being skipped.

### 5. E2E Then-step real-interface assertions — present

Both `@e2e` scenarios assert the real CLI interface first:

- The preview scenario’s first Then asserts the preview transcript’s plan
  summary, the two changed status lines, the plan JSON’s only changed resource
  and live attributes, and no create or delete. The fake journal and the live
  read are secondary.
- The apply scenario’s first Then asserts the apply transcript’s
  `github_repository.mono` change and `Apply complete! Resources: 0 added, 1
  changed, 0 destroyed.`, then the fake’s live enabled statuses, the state
  read with the real binary, the single applied write, the unchanged declared
  fields, and the empty targeted follow-up. The first assertion is on CLI
  output; the rest is secondary.

Neither scenario is a repository-only assertion. The main agent reports the e2e
preview and apply scenarios pass individually with the scoped seam.

### 6. Driver fidelity — valid for a CLI

The capability’s interface is a terminal CLI (`tofu` plus the adoption
command), not a browser UI. The e2e steps use `std::process::Command`; they do
not call `fetch()`/`XMLHttpRequest`, send an HTTP request from page JavaScript,
or call an in-process routine. A browser driver is not applicable. The opposite
classification is addressed under “E2E policy and real-interface assertion”.

### 7. Exact files `verify.e2e_command` invokes, and parallel-implementation search

`./run-tests.sh --e2e` invokes `run-tests.sh`, `assemble-features.sh`,
`features_runner.rs`, and for the capability
`apps/cli/tests/steps/e2e/enable_repository_security.rs` and
`apps/cli/tests/steps/enable_repository_security.rs`. The tree was searched for
a second/parallel implementation of the same capability: only the feature, the
two step files, the two `mod.rs` files, the design/review/tasks documents and
the arc42 building-block view match. `git status --porcelain` is clean; no
parallel implementation was found.

### 8. E2E scope and normalized inventory

The owning use case has exactly two normalized interactions: preview the
declared configuration and read the named differences; apply the previewed
revision and read the outcome. The feature has exactly one `@e2e` scenario for
each. The other six scenarios are variants/error cases. No second `@e2e` for
the same action and no missing inventory action.

### 9. E2E command isolation and counts

`verify.e2e_command` is not a copy of `verify.command`; it is the same runner
with the `@e2e` filter. The recorded counts are 140 full and 6 e2e; the e2e
scope is strictly smaller. The predecessor receipt records 132 full (including
4 e2e) and 4 e2e; the delta’s 8 scenarios give 140 and 6. The **full-scope**
count is not green on the current tree because of the unadopted-setting failure
(F10); the e2e scope itself is reported green.

### 10. Implementation vs `design.md` — commands, files, framework and data

| Design item | As built | Match? |
|---|---|---|
| `verify.command` = `./run-tests.sh` | `givn/commands.yaml` and `run-tests.sh` | yes |
| `verify.e2e_command` = `./run-tests.sh --e2e` | `givn/commands.yaml` and `run-tests.sh`; `--tags @e2e` | yes |
| Non-e2e step file | exists, real subprocess/state assertions | yes |
| E2E step file, separate | exists, transcript committed | yes |
| Framework: cucumber-rs with `std::process::Command` | runner and step files | yes |
| `infra/security.tf`, `security_and_analysis`, second batch | present and wired | yes |
| Fake `--writes`, journal body keys, `after_applied_writes`/`once`, both trees | present | yes |
| Setup script: pin, mirror, terraformrc, second batch, base apply proof | present; tasks record the proof | yes |
| CI runs the setup script before the suite | `.github/workflows/ci.yml` | yes |
| `DutyWorld` fields | `plan_path`, `preview_transcript`, `apply_transcript`, `write_sandbox` | yes |
| Visual transcripts per `@e2e` scenario | two committed transcripts | yes |
| Real-apply transcript after GREEN | present; preflight omission recorded | yes |
| Targeted preview/preflight/failure report; apply the saved plan without `-target` | `preview()` and `apply_previewed()` use the target for the plan/preflight/failure report; the apply uses the saved plan | yes for the scoping; **no for the guard interaction (F10)** |
| The preview guard refuses an unadopted live create | `GUARD_FILTER` reads the targeted plan’s `resource_changes`; the Dependabot creates are outside that plan, so the guard returns `ok` and the unadopted scenario fails | **no (F10)** |
| No implementation-layer detail in the spec | no journal/PATCH/write in the feature | yes |
| Corrected design re-reviewed | final `DESIGN-REVIEW: PASS` appended in `7ececa9` | yes as an artifact; its refusal-scenario claim is incomplete (F10) |

### 11. Interaction coverage cross-reference

| Inventory entry (`usecase.md`) | Design matrix row | Existing `@e2e` scenario | Driving mechanism | Match? |
|---|---|---|---|---|
| preview the declared configuration and read the named differences | same | “Organization owner previews the security settings and only the declared differences are named” | `std::process::Command` runs `tofu init`, targeted `plan -out`, `show -json`; asserts the plan text/JSON | yes |
| apply the previewed revision and read the outcome | same | “Organization owner applies the previewed revision and the security settings are enabled” | `std::process::Command` runs the targeted preflight, `tofu apply <plan>`, then a targeted `tofu plan`; asserts the transcript, journal, live read and state | yes |

Every inventory entry has a matrix row, an existing `@e2e` title and the
promised CLI driver. No missing or extra row.

### 12. Fabrication-audit conclusion

The change is real, not a stub: the declaration, fake, setup proof, runner and
step definitions exist, and the recorded suite was green before the F8 scoping
change. Steps 0–9 and 11 are clean. The F8 process artifact is present. The
audit records one unfinished implementation item: **F10**, the targeted
preview/guard interaction, which leaves the unadopted-setting refusal red.

Contract-statement-to-scenario mapping (F3):

| Use-case statement | Scenario(s) |
|---|---|
| Main flow 1 — record the revision and keep it as the candidate | the declaration/previewed-revision Givens |
| Main flow 2 — preview names every difference; unreadable or unadopted object refused and named | preview `@e2e`; unreadable setting; **unadopted Dependabot setting (red, F10)** |
| Main flow 3 — apply checks the saved plan and the scoped preflight, then applies | apply `@e2e`; live-edit refusal |
| Main flow 4 — report changed objects and record the applied revision | apply `@e2e` |
| Main flow 5 — the live configuration matches the revision | apply `@e2e` |
| Extension 2 — unreadable live configuration | unreadable setting |
| Extension 2 — unadopted live object | **unadopted Dependabot setting (red, F10)** |
| Extension 2 — stale applied state | read-back failure; partial failure |
| Extension 3 — live edit since the preview | live edit |
| Extension 3 — authority does not cover the first object | first change refused |
| Extension 3 — read-back failure after a write | read-back failure |
| Extension 3 — later object fails after an earlier write | partial failure |
| Rules and guarantees | the apply, unreadable, unadopted, live-edit, first-change-refused, read-back/partial and preview scenarios |

The mapping is complete, but the unadopted scenario does not pass (F10).

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 1 — reproducibility goal and Persona roster | `docs/arc42/01` lines 13, 27 | affected | Scope, Persona impact | setup, 1–8 | declaration, step files, transcripts | yes |
| 2 — constraints: pin, encrypted state, secrets, no CI apply, read-only adoption, saved plan/preflight | `docs/arc42/02` lines 33–35 | affected | Technology decisions | setup, 1–8 | `infra/*`, CI, step helpers | yes |
| 3 — declared configuration/adoption context | `docs/arc42/03` line 47 | affected | Architecture impact | all | declaration, command, fake | yes |
| 4 — IaC strategy and saved-plan apply | `docs/arc42/04` line 10 | affected | Technology decisions | setup, 1–8 | declaration and steps | yes |
| 5 — declaration, apply block, fake sandbox | `docs/arc42/05` lines 70, 92–96 | affected | normal mechanism | setup, all | `infra/`, scripts, fake, steps | yes |
| 6 — runtime: adoption, preview, guard, preflight, apply, refusal, partial | `docs/arc42/06` lines 204–270 | affected; targeted scope and full-preview drift | apply procedure | 1–8, operator apply | step files, transcripts, real-apply | **no (F10)**: the runtime view/diagram says the preview guard refuses an unadopted create, but the targeted plan excludes the Dependabot creates and the guard returns `ok` |
| 7 — deployment: workstation, secrets, state | `docs/arc42/07` line 99 | affected | applied state | 5, 6, operator | state, command, CI | yes |
| 8 — apply semantics: saved plan, preflight, guard, stale marker | `docs/arc42/08` lines 100–145 | affected | decisions, apply procedure | 3–6 | step code, fake journal, state | **no (F10)**: “the preview itself refuses a plan that would create a setting … unbound” is not true with the targeted plan |
| 9 — no new ADR | `docs/arc42/09` | no | four NOT_QUALIFIED verdicts | change docs | ADR-0005/0006 unchanged | yes |
| 10 — QS-019/QS-020 | `docs/arc42/10` line 59 | affected; targeted scope | traceability | 1–8 | e2e, refusals, state | yes (QS-019/020 are scoped; the unadopted guard is the F10 defect) |
| 11 — R-026…R-031, TD-013 | `docs/arc42/11` lines 32–37, 55 | affected | risks | 1–8, operator | declaration, fake, state | yes |
| 12 — Security settings / Configuration change | `docs/arc42/12` rows | affected | decisions, file layout | all | declaration, state/stale marker | yes |

The content is current for the targeted scope, but chapters 6 and 8 now claim a
guard behavior the scoped implementation does not provide. The arc42 block is
not clean.

ARC42 CONFORMANCE: NOT CLEAN

Chapters 6 and 8 contradict the failing unadopted-setting refusal (F10); the
other rows match.

## Ubiquitous language conformance

The glossary’s confirmed terms are reused with their exact senses: **Security
settings**, **Configuration change**, **Declared configuration**, **Adoption**,
**Configuration drift**, **Public home** and **Protection ruleset**. The spec,
design and implementation use “security settings”, “declared configuration”,
“applied state”, “preview”, “apply”, “saved plan” and “stale applied state”
consistently; the last four are the **Configuration change** row’s vocabulary,
not new terms. The F9 wording fix removed the change-process “in this delta”
phrasing from the out-of-scope bullets. No conflicting sense or silently
promoted term was found.

UBIQUITOUS LANGUAGE: CONFORMANT

## Coverage classification

Coverage is measured by the archive against the committed candidate and
published in the verification receipt. No archive receipt exists for this
change yet. The most recent measured evidence is
`givn/archive/declare-public-home/verification.json`: **98% line coverage
(871/886)** at revision `47277e1`. This change adds no production Rust: the only
Rust added is step definitions and `DutyWorld` fields, which the runner
instruments as test code. The HCL, the fake server and the shell scripts are
outside the Rust coverage denominator by the design’s declared process
boundary.

Classification from inspection, using only the three buckets:

1. **Dead code (YAGNI/KISS) → delete.** No dead production code was found. The
   product crates are untouched. The fake’s write-sandbox handlers and fault
   paths are test-twin plumbing and are outside the production denominator.
2. **Missing test coverage → add scenario, run RED→GREEN→REFACTOR→COMMIT.** No
   missing scenario. The four behaviours the previous review classified as
   bucket 2 are outside the capability’s contract by the use case’s explicit
   `## Out of scope` boundary; they are not a coverage gap. The unadopted
   scenario exists and is red — this is a broken implementation (F10), not a
   missing scenario, and it cannot be excused by a fourth bucket.
3. **Legitimately hard to test (rare) → concrete technical justification.**
   None. The shell/Python/HCL are outside the Rust denominator by the design’s
   declared process boundary; their observable contract is exercised by the
   scenarios, and the operator’s real apply is the live evidence.

No redundant unit tests were added; the `.feature` files remain the source of
truth. The coverage classification uses the most recent measured evidence; the
archive receipt will carry the merged result. The red scenario does not change
the coverage classification; it fails the GREEN condition.

## E2E policy and real-interface assertion

The canonical policy was read from
`givn instructions specs --change enable-repository-security`.

- **Opposite classification:** a rendered page is a Web UI and would require a
  real browser driver; an HTTP client would not be valid. That classification
  **does not hold here**: the interface is the terminal `tofu` CLI plus the
  adoption command; its output is plain text. There is no HTML, CSS,
  JavaScript, DOM or browser-rendered state. The design declares
  `Interface: terminal`.
- **Real interface:** real `tofu` subprocesses (targeted `plan -out`,
  `show -json`, the targeted refresh-only preflight, `apply <plan>`) against
  the local fake GitHub API, plus the real adoption command.
- **Primary assertion:** the CLI output — the preview’s plan summary/statuses
  and the apply’s changed resource and `Apply complete!` line. The journal,
  live fake reads and `tofu state` are secondary.
- **Driver:** `std::process::Command`, not an in-process call or HTTP from page
  JavaScript.
- **Scope:** two normalized actions, two `@e2e` scenarios; the six variants are
  regular scenarios. The e2e preview and apply are reported green individually;
  the regular unadopted-setting scenario is red (F10), not an e2e defect.

## Use-case and Persona conformance

The owning use case is
`givn/changes/enable-repository-security/specs/apply-configuration-change/usecase.md`;
the ideation contract is
`givn/ideation/public-home-infrastructure/use-cases/apply-configuration-change.md`.
The implementation preserves the goal, the one success outcome, the minimal
guarantee, the rules, capability ownership and the confirmed Persona-relevant
outcomes, subject to F10.

| Criterion | Result |
|---|---|
| Goal conjunction | Not triggered: one goal phrase |
| Success guarantee names two outcomes | Not triggered: one outcome (F7 fixed) |
| Missing primary actor | Not triggered: Organization owner is primary and the trigger names the owner |
| Placement failure | Not triggered: `enable-repository-security` advances the administration goal |
| Mutation under an observing goal | Not triggered: the use case declares itself changing |
| Extension without a step reference | Not triggered: extensions are anchored to step 2 or 3; the declaration-edit extension is out of scope |
| Inconsistent preconditions | Not triggered: the stale-state precondition is a state, the trigger is an action, the guarantee is qualified `(open: Q11)` |
| Mechanism vocabulary in goal/trigger/guarantees | Not triggered |
| Component- or channel-shaped name | Not triggered |
| Oversized leaf | Not triggered |
| Relationship misuse | Not triggered: no Extends/Includes |
| Extension 2 — unadopted live object is refused and named | **Not satisfied by the current implementation (F10)**: the scenario exists but fails because the targeted plan excludes the Dependabot creates |

Confirmed Personas and dispositions:

| Persona | Evidence and disposition |
|---|---|
| `project-maintainer--organization-owner` | Runs the preview/apply and answers for refusals/partial outcomes; the two committed transcripts show the preview/apply reach the goal. The unadopted refusal is red (F10), but the Persona’s interface is present. |
| `contributor--infrastructure-reviewer` | Reads the declaration, batch and ciphertext state; holds no token/passphrase and cannot apply; goal reached as a review lens. |
| `crew-member--personal-calendar-owner` | The parent use case’s lens, not a Persona confirmed for this change and not an Actor; no release object, rule or archive is touched. No invented Persona. |

A user-visible surface (terminal preview/apply) has two confirmed Personas. No
invented or silently promoted Persona was found. The live operator deviation is
assessed below.

## Visual review

PERSONAS: project-maintainer--organization-owner
PERSONAS: contributor--infrastructure-reviewer

The design’s `## Visual Design Contract` declares:

- Interface: **terminal**
- Viewports: n/a (no rendered screen)
- Screens: none
- Evidence: one transcript per `@e2e` scenario under
  `givn/changes/enable-repository-security/evidence/visual/<scenario-slug>/transcript.txt`
- Baseline: the plan’s summary and changed setting names, or the apply’s
  outcome; no token, passphrase, temporary path or live credential.

Committed evidence reviewed:

- preview:
  `evidence/visual/organization-owner-previews-the-security-settings-and-only-the-declared-differences-are-named/transcript.txt`
  (the base apply, both adoptions, the targeted plan with
  `0 to add, 1 to change, 0 to destroy`, the two `disabled -> enabled`
  statuses, the base apply’s follow-up, and `exit status: 0`).
- apply:
  `evidence/visual/organization-owner-applies-the-previewed-revision-and-the-security-settings-are-enabled/transcript.txt`
  (the preview, the targeted preflight, the `github_repository.mono` update,
  `Apply complete! Resources: 0 added, 1 changed, 0 destroyed.`, the targeted
  follow-up `No changes.`, and `exit status: 0`).

The transcripts carry the final state for both declared terminal scenarios; no
screenshots or viewports apply. The run-specific directory, fake port, token,
passphrase and billing email are placeholders; no token, passphrase, temporary
path or live credential appears. The provider state-only
`ignore_vulnerability_alerts_during_read` field appears; `design-review.md` G6
classifies it as bookkeeping and the baseline does not prohibit it.

Fresh-context Persona lenses against the committed transcripts:

- **Organization owner:** sees the preview’s two named statuses and the apply’s
  changed resource and success line; can reach the preview/apply goal. No
  finding.
- **Contributor reviewer:** can read the declaration, batch and ciphertext
  state and is not given an apply path; goal reached. No finding.
- **Crew member:** no site or release operation appears; the parent lens is
  unaffected and is not a confirmed Persona for this change. No finding.

Every finding would have to name a contract rule or a failed Persona goal; none
was found. No pixel-level or coordinate finding applies.

VISUAL-REVIEW: PASS

## Live operator deviation assessment

- **Seed guarantee:** the post-apply authenticated read reports secret scanning
  and push protection `enabled` and Dependabot security updates `enabled`; both
  Dependabot resources were imported and bound
  (`github_repository_vulnerability_alerts.mono`, refreshed id `1416075332`,
  imported by `duty2cal`; `github_repository_dependabot_security_updates.mono`,
  id `duty2cal`). The encrypted state serial moved from 6 to 9 (two imports
  plus one apply), and the state/backup are in `877753f`. The seed guarantee is
  met as recorded.
- **Direct imports and targeted plan:** the second adoption command refused
  the batch because its whole-state refresh saw the pre-existing organization
  billing drift. Both Dependabot resources were imported directly by their
  repository-name ids; the targeted plan named only `github_repository.mono`
  (two statuses enabled) and the saved plan was applied without `-target`; the
  targeted follow-up exited 0 with `No changes.` The direct imports satisfy
  adopt-before-create; the plan scope keeps the unrelated drift out of the
  apply.
- **Preflight omission:** the 2026-10-18 live run omitted the read-only
  preflight; `design.md` Post-GREEN step 5 and the transcript record the
  omission, and the suite proves the preflight rule. This is a recorded
  operator deviation, not a silent one.
- **Adoption whole-state defect:** the design’s “Live operator run” and
  `tasks.md` record the defect for a follow-up: the adoption verifier must
  refresh only the batch addresses so a pre-existing drift elsewhere does not
  refuse an unrelated adoption. It is not silently accepted.
- **Full-preview drift:** the full preview still names the pre-existing
  organization billing drift; it is outside this change and was not applied.
  `design.md` Post-GREEN and `docs/arc42/06`/QS-019/QS-020 record the targeted
  scope and the drift; no claim remains that the full configuration is clean.

The live deviation itself is recorded and justified. The outstanding problem is
the executable seam’s guard interaction (F10), not the live run.

## Deterministic gate dispositions

`givn lint --change enable-repository-security` reports no `[SHAPE]`, `[LONG]`
or other overlap finding; `givn check review` reports
`✓ overlap dispositions passed`. The delta is 8 added, 0 modified, 0 removed;
there are no removed+added pairs.

### Overlap dispositions

`givn lint --change enable-repository-security` reports no `[SHAPE]` finding
and `givn check review` reports `✓ overlap dispositions passed`. There are no
finding pairs to disposition and no removed+added pair in the delta, so the
section carries no rows.

### Split-or-keep

No scenario exceeds the long-scenario gate threshold; `givn lint` reports no
`[LONG]` finding. There are no removed+added pairs, so no split-or-keep
decision is required and the section carries no rows.

## README impact decision

README-IMPACT: none

The change adds maintainer infrastructure. It does not change a product
command, flag, configuration file, output or any README section (Description,
Quick start, Prerequisites, Usage, Configuration, Contributing, License). The
declaration and the operator routine are documented in `design.md` and
`tasks.md`, not in the public README. `README.md` contains no open placeholder.

## Gate checks

`givn check review --change enable-repository-security`:

```
Mandatory checks for 'review':
  ✓ integrity passed
  ✓ run declaration passed
net delta: 8 added, 0 modified, 0 removed
net delta json: {"added":8,"modified":0,"removed":0}
  ✓ overlap dispositions passed
```

Exit 0. `givn status` reports `ready-to-archive`, 44/44 tasks, all artifacts
complete. The mechanical gate does not execute the scenarios; the current-tree
unadopted-setting failure (F10) is the review-level blocker.

## Sign-off checklist

| Check | Result |
|---|---|
| Fabrication audit clean (or all findings remediated) | **No** — F10 (targeted preview/guard regression); F8/F9 artifacts are fixed; no empty steps or removed `@e2e` |
| Every checked task has a verified commit touching production code | Yes — the eight revisions touch the declaration, fake, setup, step definitions or runner |
| Every promised component exists | Yes — including the real-apply transcript and post-apply reads |
| Strict-mode proof present and passing | Yes — `fail_on_skipped()`; the single-scenario failure surfaced rather than being skipped |
| Scenario execution evidence recorded per scope | **No for the current tree** — the recorded 140/140, 584/6/42 predates `7ececa9`; the unadopted-setting scenario fails on the current tree (1 failed, 5/6 steps, exit 101) |
| Coverage classification uses the most recent measured evidence | Yes — last archive receipt 98% (871/886); no current receipt; static inspection used |
| Every coverage gap classified and resolved | Yes — no gap; the red scenario is a correctness regression, not a coverage gap |
| Dead code deleted; missing tests added; hard-to-test gaps justified | Yes — no product dead code; no missing scenario; no hard-to-test gap |
| Redundant unit tests removed | Yes — no unit tests added; `.feature` is the source |
| No `@wip` tags; no implementation-layer detail in the spec | Yes — F6 fixed; the feature has no journal/PATCH/write wording |
| Canonical E2E policy read; one normalized action per `@e2e`; variants regular | Yes — two inventory actions, two `@e2e`; six variants regular |
| Every E2E scenario has a real-interface primary assertion and the reviewed driver; no HTTP/fetch substitution | Yes — CLI output first; `std::process::Command`; terminal |
| Local run command starts the full stack (including digital twins) cleanly | Recorded; the scenarios start the fake per scenario; `scripts/setup-infra-tests.sh` prepares the mirror and is in CI |
| Exact files `verify.e2e_command` invokes identified and read; parallel implementation searched | Yes — `run-tests.sh`, `assemble-features.sh`, `features_runner.rs`, the two step files; tree search and `git status` found no parallel implementation |
| `verify.e2e_command` distinct; counts prove a strictly smaller e2e scope | Yes — `./run-tests.sh --e2e` vs `./run-tests.sh`; 6 < 140; predecessor receipt 4 < 132 |
| Implementation matches design’s commands, files, framework/driver | **No** — the design’s guard claim is not met by the targeted plan (F10); the commands/files/framework themselves match |
| Interaction coverage verified | Yes — two inventory entries → two matrix rows → two existing `@e2e` → promised CLI driver |
| No finding excused outside the three coverage buckets | Yes — the out-of-scope behaviours are a use-case boundary; F10 is a correctness/implementation defect, not a fourth-bucket exception |
| Arc42 conformance | **No** — chapters 6 and 8 claim a guard behavior the scoped implementation does not provide (F10) |
| `givn check review` gate | Passes mechanically — integrity, run declaration, routing and overlap; it does not execute the suite |

## Findings and required fixes

### F10 — Restore the unadopted-setting guard under the targeted preview (blocker)

Commit `7ececa9` changed `preview()` to run
`tofu plan -target=github_repository.mono -out=plan.bin` and left
`GUARD_FILTER` reading `resource_changes` for `create` actions on
`github_repository_vulnerability_alerts.mono` and
`github_repository_dependabot_security_updates.mono`. A plan targeted to the
monorepo excludes both Dependabot resources, so the guard sees no create,
returns `ok`, and the preview exits 0. The scenario “The preview refuses to add
a Dependabot setting that already exists live and is unbound” therefore fails:
on the current tree it runs **1 scenario (1 failed), 6 steps (5 passed, 1
failed)**, exit 101, and the preview transcript shows the targeted plan and
`exit status: 0`. The design and `docs/arc42/06`/`08` still claim the preview
refuses an unadopted live create; the design-review’s final PASS claims “the
preview, apply and refusal scenarios pass”, which is false for this refusal.
The recorded 140/140 predates `7ececa9` and is not a green claim for the
current tree; the archive gate’s re-run will include the scenario.

Required fix:

1. In `apps/cli/tests/steps/enable_repository_security.rs`, keep the targeted
   security plan for `plan.bin` (the previewed apply), but evaluate
   `GUARD_FILTER` over a second plan that covers the declaration’s Dependabot
   resources — either a full `tofu plan -out=guard.bin` or
   `tofu plan -target=github_repository_vulnerability_alerts.mono
   -target=github_repository_dependabot_security_updates.mono
   -out=guard.bin` — and read that report’s `resource_changes`. The guard’s
   live-alerts/prior-state check stays.
2. Update `design.md` “The apply procedure”/“The preview” and the corresponding
   `docs/arc42/06`/`08` text to state that the unadopted-setting guard is
   evaluated on the full declaration (or the Dependabot-targeted plan) while
   the preview/apply is scoped to the previewed object.
3. Because `design.md` changes, append a fresh design-review PASS for the
   guard-scope correction to `design-review.md` (the F8 rule) and correct its
   “refusal scenarios pass” claim.
4. Re-run the unadopted-setting scenario and the full suite; update the
   recorded counts in `tasks.md` (and this review). Re-run
   `givn check review` afterwards.

### F8/F9 — fixed

F8’s process artifact is present (`design-review.md`’s final section with
`DESIGN-REVIEW: PASS` in `7ececa9`); F9’s wording is fixed (“in this use
case”). F10 is the implementation regression those fixes did not catch.

## Verdict

F1–F9 are addressed as artifacts: the commit lines and parser are gate-clean,
the use case is narrowed and reworded, the real-apply evidence and post-apply
reads are committed, the design/arc42 targeted-scope correction is present and
re-reviewed, the feature is free of fake-journal/PATCH detail, the success
guarantee has one outcome, and the e2e preview/apply pass individually.

The earned verdict is **FAIL** because the F8 scoping change breaks the
unadopted-setting guard: the targeted preview plan excludes the Dependabot
creates, so the use case’s unadopted-object refusal scenario fails on the
current tree (1 failed, 5/6 steps). Fix F10, re-run the full suite, and re-run
this review.

REVIEW: FAIL
