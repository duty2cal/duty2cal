# Review: enable-repository-security

Change: `enable-repository-security`
Stage: `ready-to-archive` (`givn status` reports 44/44 tasks)
Review date: 2026-10-18
Verdict: **PASS** (F1–F10 are remediated; the artifact gates are green and the
guard split is verified on the current tree)

## Preconditions

- `givn instructions review --change enable-repository-security`,
  `givn instructions specs --change enable-repository-security` and
  `givn instructions dialog` were loaded before this report was written.
- `givn status --change enable-repository-security` reports 44/44 tasks and
  stage `ready-to-archive`; all artifacts are present.
- This review was read-only except for writing this file. No other artifact,
  fixture, step definition, declaration or state file was edited.
- The suite was not re-run as a whole, per the assignment. The recorded counts
  below are the evidence; the archive gate re-runs both suites. A
  single-scenario check was run on the current tree to verify the F10 guard
  split: the unadopted-setting scenario now passes 9/9.
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

Recorded execution evidence (from `tasks.md`; the archive gate re-runs):

- Full suite: **140 scenarios (140 passed), 584 steps (584 passed)**, exit 0.
- E2E scope: **6 scenarios (6 passed), 42 steps (42 passed)**, exit 0.
- The e2e scope is strictly smaller than the full scope: **6 < 140**.
- `givn lint --change enable-repository-security`: **clean**, exit 0.
- The most recent archive receipt (`givn/archive/declare-public-home/verification.json`,
  candidate `47277e1`) records the full `verify.command` scope at **132**
  scenarios (including **4** `@e2e`) and the e2e scope at **4**; this delta adds
  8 scenarios (6 non-e2e, 2 `@e2e`), giving **140 full** and **6 e2e**.

Current-tree check for F10: the unadopted-setting scenario alone runs
**1 scenario (1 passed), 9 steps (9 passed)**, exit 0, over the full
`guard.bin` plan and the targeted saved plan. This confirms the F10 fix
without re-running the suite.

## F1–F10 dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| F1 — task commits in the gate’s `COMMIT:` syntax | **Fixed.** Every scenario task records `- [x] COMMIT: \`<hash>\``. | `tasks.md` lines 90, 118, 148, 180, 210, 240, 305, 346; `givn check review` reports `✓ integrity passed`. |
| F2 — delta use case parses for capability routing | **Fixed.** The header, sections and bullets are in the parser’s shape; `## Level` is `!`, the project convention. | `givn check review` reports no routing error and prints `net delta: 8 added, 0 modified, 0 removed`. |
| F3 — the delta use case states only what the eight scenarios assert | **Fixed.** The four removed behaviours are in `## Out of scope` and use “in this use case”; every remaining contract statement maps to a scenario. | `usecase.md` §“Out of scope”; mapping below. |
| F4 — commit the required real-apply transcript | **Fixed.** The transcript has the adoption refusal, both direct imports (vulnerability alerts imported by `duty2cal`, refreshed id `1416075332`; updates imported by `duty2cal`), the targeted plan, the apply of the saved plan, the targeted follow-up, and the statuses; the preflight omission is recorded; the three post-apply JSON files exist. | `evidence/real-apply/transcript.txt`; `evidence/live-read/post-apply-*.json`; `evidence/live-read/README.md`. |
| F5 — design and arc42 state the targeted scope and the full-preview drift | **Fixed.** `design.md` Post-GREEN steps 1, 3, 4 and 5 and the correction note record the targeted preview/preflight/failure report, the apply of the saved plan without `-target`, and the full-preview drift; `docs/arc42/06` and QS-019/QS-020 say the same. | `design.md` §“Post-GREEN operator apply”, §“Live operator run”; `docs/arc42/06-runtime-view.md`; `docs/arc42/10-quality-requirements.md`. |
| F6 — remove fake-journal/HTTP-`PATCH` detail from the feature | **Fixed.** The feature has no `journal`, `PATCH` or “write request”; the read-back scenario and Given name the security settings; the last apply Then says “a later targeted preview of the security settings reports no differences” and its step definition matches. | Feature and `apps/cli/tests/steps/e2e/enable_repository_security.rs`. |
| F7 — one success outcome and consistent saved-plan/declared-edit semantics | **Fixed.** One success outcome; the rule and the out-of-scope bullet agree; the live-edit extension is anchored to step 3. | `usecase.md` §§ Success guarantee, Rules, Extensions, Out of scope. |
| F8 — re-review the corrected design | **Fixed.** `design-review.md` appends the final design review for the targeted scope (`7ececa9`) and the guard-split review (`651fc93`), each with `DESIGN-REVIEW: PASS`. | `design-review.md` final sections. |
| F9 — normalize the out-of-scope wording | **Fixed.** The four bullets say “in this use case”, not “in this delta”. | `usecase.md` §“Out of scope” lines 103–106. |
| F10 — restore the unadopted-setting guard under the targeted preview | **Fixed.** `preview()` runs a full-declaration `tofu plan -out=guard.bin` and `show -json`, evaluates `GUARD_FILTER` over `guard.json`, refuses before the targeted plan if an unadopted live Dependabot create exists, and only then runs the targeted `plan.bin` that is saved and applied. The `.gitignore`, design routine, test-seam table, matrix and arc42/06/08 reflect the split. The unadopted-setting scenario passes 9/9. | `apps/cli/tests/steps/enable_repository_security.rs`; `design.md`; `docs/arc42/06`/`08`; `.gitignore`; `651fc93`. |

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
`.gitignore` rules (now including `infra/guard.bin`), the fake’s `--writes`
sandbox, both fixture trees, the setup script, both `mod.rs` files, both step
files, the `DutyWorld` fields, the CI setup step, the live-read evidence, the
two visual transcripts and the real-apply transcript. The design’s
step-definition table omits the world’s `write_sandbox` field (the
implementation has it); this is a non-blocking documentation omission, not a
missing component.

### 4. Strict-mode proof — present

`apps/cli/tests/features_runner.rs` calls `fail_on_skipped()`. `tasks.md`
records the setup and e2e strict proofs with non-zero exits and `Step doesn't
match any function`; the feature has no `@wip` tag. The fake’s unknown-path 599
marker and refused-write journal are the second strictness net. The
single-scenario check confirms strictness: the unadopted assertion runs and
passes rather than being skipped.

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

Neither scenario is a repository-only assertion. The preview, apply and
refusal scenarios are reported passing with the split guard plan.

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
4 e2e) and 4 e2e; the delta’s 8 scenarios give 140 and 6. The archive gate
re-runs both suites.

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
| Full-declaration guard plan plus targeted saved plan | `preview()` runs `guard.bin`/`show -json` for `GUARD_FILTER`, then the targeted `plan.bin`/`show -json`; `apply_previewed()` applies the saved plan without `-target` | yes |
| Unadopted-setting guard refuses an unadopted live create | `GUARD_FILTER` reads `guard.json`; the unadopted-setting scenario passes 9/9 | yes |
| Scoped preflight/failure report | `-target=github_repository.mono` in both | yes |
| No implementation-layer detail in the spec | no journal/PATCH/write in the feature | yes |
| Corrected design re-reviewed | final `DESIGN-REVIEW: PASS` for the targeted scope and the guard split | yes |

### 11. Interaction coverage cross-reference

| Inventory entry (`usecase.md`) | Design matrix row | Existing `@e2e` scenario | Driving mechanism | Match? |
|---|---|---|---|---|
| preview the declared configuration and read the named differences | same | “Organization owner previews the security settings and only the declared differences are named” | `std::process::Command` runs `tofu init`, the full `guard.bin` plan and `show -json`, then the targeted `plan.bin` and `show -json`; asserts the plan text/JSON | yes |
| apply the previewed revision and read the outcome | same | “Organization owner applies the previewed revision and the security settings are enabled” | `std::process::Command` runs the targeted preflight, `tofu apply <plan>`, then a targeted `tofu plan`; asserts the transcript, journal, live read and state | yes |

Every inventory entry has a matrix row, an existing `@e2e` title and the
promised CLI driver. No missing or extra row.

### 12. Fabrication-audit conclusion

The change is real, not a stub: the declaration, fake, setup proof, runner and
step definitions exist, and the recorded suite is green. The F10 guard split is
implemented and verified: the unadopted-setting scenario passes 9/9 over the
full `guard.bin` plan. Steps 0–12 are clean; no `@e2e` tag was removed and no
empty step body was found.

Contract-statement-to-scenario mapping (F3):

| Use-case statement | Scenario(s) |
|---|---|
| Main flow 1 — record the revision and keep it as the candidate | the declaration/previewed-revision Givens |
| Main flow 2 — preview names every difference; unreadable or unadopted object refused and named | preview `@e2e`; unreadable setting; unadopted Dependabot setting (9/9) |
| Main flow 3 — apply checks the saved plan and the scoped preflight, then applies | apply `@e2e`; live-edit refusal |
| Main flow 4 — report changed objects and record the applied revision | apply `@e2e` |
| Main flow 5 — the live configuration matches the revision | apply `@e2e` |
| Extension 2 — unreadable live configuration | unreadable setting |
| Extension 2 — unadopted live object | unadopted Dependabot setting |
| Extension 2 — stale applied state | read-back failure; partial failure |
| Extension 3 — live edit since the preview | live edit |
| Extension 3 — authority does not cover the first object | first change refused |
| Extension 3 — read-back failure after a write | read-back failure |
| Extension 3 — later object fails after an earlier write | partial failure |
| Rules and guarantees | the apply, unreadable, unadopted, live-edit, first-change-refused, read-back/partial and preview scenarios |

No remaining contract statement lacks a scenario.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 1 — reproducibility goal and Persona roster | `docs/arc42/01` lines 13, 27 | affected | Scope, Persona impact | setup, 1–8 | declaration, step files, transcripts | yes |
| 2 — constraints: pin, encrypted state, secrets, no CI apply, read-only adoption, saved plan/preflight | `docs/arc42/02` lines 33–35 | affected | Technology decisions | setup, 1–8 | `infra/*`, CI, step helpers | yes |
| 3 — declared configuration/adoption context | `docs/arc42/03` line 47 | affected | Architecture impact | all | declaration, command, fake | yes |
| 4 — IaC strategy and saved-plan apply | `docs/arc42/04` line 10 | affected | Technology decisions | setup, 1–8 | declaration and steps | yes |
| 5 — declaration, apply block, fake sandbox | `docs/arc42/05` lines 70, 92–96 | affected | normal mechanism | setup, all | `infra/`, scripts, fake, steps | yes |
| 6 — runtime: adoption, full guard plan, targeted preview, preflight, apply, refusal, partial | `docs/arc42/06` lines 204–270 | affected; guard plan and targeted scope | apply procedure | 1–8, operator apply | step files, transcripts, real-apply | yes |
| 7 — deployment: workstation, secrets, state | `docs/arc42/07` line 99 | affected | applied state | 5, 6, operator | state, command, CI | yes |
| 8 — apply semantics: saved plan, preflight, full guard plan, stale marker | `docs/arc42/08` lines 100–145 | affected | decisions, apply procedure | 3–6 | step code, fake journal, state | yes |
| 9 — no new ADR | `docs/arc42/09` | no | four NOT_QUALIFIED verdicts | change docs | ADR-0005/0006 unchanged | yes |
| 10 — QS-019/QS-020 | `docs/arc42/10` line 59 | affected; targeted scope | traceability | 1–8 | e2e, refusals, state | yes |
| 11 — R-026…R-031, TD-013 | `docs/arc42/11` lines 32–37, 55 | affected | risks | 1–8, operator | declaration, fake, state | yes |
| 12 — Security settings / Configuration change | `docs/arc42/12` rows | affected | decisions, file layout | all | declaration, state/stale marker | yes |

Every row matches; the affected chapters are current, the guard split is
recorded in the runtime view and apply semantics, and the design-review re-runs
are present.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The glossary’s confirmed terms are reused with their exact senses: **Security
settings**, **Configuration change**, **Declared configuration**, **Adoption**,
**Configuration drift**, **Public home** and **Protection ruleset**. The spec,
design and implementation use “security settings”, “declared configuration”,
“applied state”, “preview”, “apply”, “saved plan”, “guard plan” and “stale
applied state” consistently; the latter terms are the **Configuration change**
row’s vocabulary, not new domain terms. The F9 wording fix removed the
change-process “in this delta” phrasing from the out-of-scope bullets. No
conflicting sense or silently promoted term was found.

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
boundary; their observable contract is exercised by the eight scenarios.

Classification from inspection, using only the three buckets:

1. **Dead code (YAGNI/KISS) → delete.** No dead production code was found. The
   product crates are untouched. The fake’s write-sandbox handlers and fault
   paths are test-twin plumbing and are outside the production denominator. No
   product deletion is required.
2. **Missing test coverage → add scenario, run RED→GREEN→REFACTOR→COMMIT.** No
   gap. The four behaviours the previous review classified as bucket 2 are
   outside the capability’s contract by the use case’s explicit `## Out of
   scope` boundary; they are not a coverage gap and no scenario is owed. Every
   remaining contract statement maps to one of the eight scenarios, and the
   unadopted-setting scenario passes.
3. **Legitimately hard to test (rare) → concrete technical justification.**
   None. The shell/Python/HCL are outside the Rust denominator by the design’s
   declared process boundary; their observable contract is exercised by the
   scenarios, and the operator’s real apply is the live evidence.

No fourth bucket, and no “acceptable for now”, “future scope”, “sufficient for
current scope” or similar exception is used. No redundant unit tests were
added; the `.feature` files remain the source of truth. The coverage
classification uses the most recent available measured evidence; the archive
receipt will carry the merged result.

## E2E policy and real-interface assertion

The canonical policy was read from
`givn instructions specs --change enable-repository-security`.

- **Opposite classification:** a rendered page is a Web UI and would require a
  real browser driver; an HTTP client would not be valid. That classification
  **does not hold here**: the interface is the terminal `tofu` CLI plus the
  adoption command; its output is plain text. There is no HTML, CSS,
  JavaScript, DOM or browser-rendered state. The design declares
  `Interface: terminal`.
- **Real interface:** real `tofu` subprocesses (the full guard `plan -out`,
  the targeted `plan -out`, `show -json`, the targeted refresh-only preflight,
  `apply <plan>`) against the local fake GitHub API, plus the real adoption
  command.
- **Primary assertion:** the CLI output — the preview’s plan summary/statuses
  and the apply’s changed resource and `Apply complete!` line. The journal,
  live fake reads and `tofu state` are secondary.
- **Driver:** `std::process::Command`, not an in-process call or HTTP from page
  JavaScript.
- **Scope:** two normalized actions, two `@e2e` scenarios; the six variants are
  regular scenarios. The preview, apply and refusal scenarios pass.

## Use-case and Persona conformance

The owning use case is
`givn/changes/enable-repository-security/specs/apply-configuration-change/usecase.md`;
the ideation contract is
`givn/ideation/public-home-infrastructure/use-cases/apply-configuration-change.md`.
The implementation preserves the goal, the one success outcome, the minimal
guarantee, the rules, capability ownership and the confirmed Persona-relevant
outcomes.

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
| Extension 2 — unadopted live object is refused and named | Satisfied: the full guard plan sees the Dependabot creates; the scenario passes 9/9 |

Confirmed Personas and dispositions:

| Persona | Evidence and disposition |
|---|---|
| `project-maintainer--organization-owner` | Runs the preview/apply and answers for refusals/partial outcomes; the two committed transcripts show the preview/apply reach the goal; the unadopted refusal passes 9/9. |
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

The live deviation is recorded and justified. The F10 guard split is verified
on the current tree.

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
complete. `givn lint` is clean. The mechanical gate does not execute the
scenarios; the recorded suites and the archive gate’s re-run cover execution.

## Sign-off checklist

| Check | Result |
|---|---|
| Fabrication audit clean (or all findings remediated) | Yes — F1–F10 remediated; no empty steps or removed `@e2e` |
| Every checked task has a verified commit touching production code | Yes — the eight revisions touch the declaration, fake, setup, step definitions or runner |
| Every promised component exists | Yes — including the real-apply transcript and post-apply reads |
| Strict-mode proof present and passing | Yes — `fail_on_skipped()`; the single-scenario check and the recorded proofs |
| Scenario execution evidence recorded per scope | Yes — 140/140 and 584 steps; 6/6 e2e and 42 steps; the unadopted scenario verified 9/9 on the current tree; the archive gate re-runs both suites |
| Coverage classification uses the most recent measured evidence | Yes — last archive receipt 98% (871/886); no current receipt; static inspection used |
| Every coverage gap classified and resolved | Yes — no gap; the three buckets are exhaustive |
| Dead code deleted; missing tests added; hard-to-test gaps justified | Yes — no product dead code; no missing scenario; no hard-to-test gap |
| Redundant unit tests removed | Yes — no unit tests added; `.feature` is the source |
| No `@wip` tags; no implementation-layer detail in the spec | Yes — the feature has no journal/PATCH/write wording |
| Canonical E2E policy read; one normalized action per `@e2e`; variants regular | Yes — two inventory actions, two `@e2e`; six variants regular |
| Every E2E scenario has a real-interface primary assertion and the reviewed driver; no HTTP/fetch substitution | Yes — CLI output first; `std::process::Command`; terminal |
| Local run command starts the full stack (including digital twins) cleanly | Recorded; the scenarios start the fake per scenario; `scripts/setup-infra-tests.sh` prepares the mirror and is in CI |
| Exact files `verify.e2e_command` invokes identified and read; parallel implementation searched | Yes — `run-tests.sh`, `assemble-features.sh`, `features_runner.rs`, the two step files; tree search and `git status` found no parallel implementation |
| `verify.e2e_command` distinct; counts prove a strictly smaller e2e scope | Yes — `./run-tests.sh --e2e` vs `./run-tests.sh`; 6 < 140; predecessor receipt 4 < 132 |
| Implementation matches design’s commands, files, framework/driver | Yes — the full guard plan and targeted saved plan match; the scoped preflight/failure report match; the apply uses the saved plan without `-target` |
| Interaction coverage verified | Yes — two inventory entries → two matrix rows → two existing `@e2e` → promised CLI driver |
| No finding excused outside the three coverage buckets | Yes — the out-of-scope behaviours are a use-case boundary; no “acceptable for now”/“future scope” wording |
| Arc42 conformance | Yes — all rows match; the guard split is in chapters 6 and 8 |
| `givn check review` gate | Passes — integrity, run declaration, routing and overlap |

## Findings and required fixes

No remaining blocker. F1–F10 are remediated:

- F1–F7: the commit syntax, use-case parser, narrowed contract, real-apply
  evidence, targeted design/arc42 text, spec cleanup and one-outcome guarantee
  are in place.
- F8–F9: the final design reviews are appended with `DESIGN-REVIEW: PASS`, and
  the out-of-scope wording says “in this use case”.
- F10: the full-declaration `guard.bin` plan now feeds the unadopted-setting
  guard, the targeted saved plan remains the applied revision, the design,
  test-seam table, matrix, `.gitignore` and arc42 06/08 reflect the split, and
  the unadopted-setting scenario passes 9/9.

## Verdict

F1–F10 are remediated. The artifact gates are green (`givn check review`
passes integrity, run declaration, routing and overlap; `givn lint` is clean;
`givn status` is `ready-to-archive` with 44/44 tasks). The recorded execution
is 140/140 scenarios and 584 steps, with 6/6 e2e and 42 steps; the
unadopted-setting scenario was verified 9/9 on the current tree. The archive
gate re-runs both suites.

The earned verdict is **PASS**.

REVIEW: PASS
