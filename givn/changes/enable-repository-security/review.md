# Review: enable-repository-security

Change: `enable-repository-security`
Stage: `tasks-done` (`givn status` reports 44/44 tasks)
Review date: 2026-10-18
Verdict: **FAIL** (the change is substantially implemented and the recorded suite is green, but the review gate refuses and completeness defects remain)

## Preconditions

- `givn instructions review --change enable-repository-security`,
  `givn instructions specs --change enable-repository-security` and
  `givn instructions dialog` were loaded before this report was written.
- `givn status --change enable-repository-security` reports 44/44 tasks and
  stage `tasks-done`; the review artifact is the only missing stage.
- This review was read-only except for writing this file. No other artifact,
  fixture, step definition, declaration or state file was edited.
- The suite was not re-run, per the assignment. The counts below are the
  recorded execution evidence in `tasks.md`.
- `givn lint --change enable-repository-security` was run: `givn lint: 1
  file(s) checked — clean`, exit 0.
- `givn check review --change enable-repository-security` was run: it refuses
  at the integrity check (`no "- [x] COMMIT:" lines found in tasks.md`). The
  detailed gate findings are in “Gate checks and artifact conformance”.

## Commands and scope under review

Read literally from `givn/commands.yaml`:

- `verify.command: "./run-tests.sh"`
- `verify.e2e_command: "./run-tests.sh --e2e"`

The strings are different. `run-tests.sh` maps `--e2e` to `--tags "@e2e"` and
invokes `cargo test -p duty2cal --test features_runner -- --tags "@e2e"` over
the feature root assembled by `scripts/assemble-features.sh` (the permanent
corpus plus the active change’s `specs/**`). `apps/cli/tests/features_runner.rs`
calls `Cucumber::fail_on_skipped()`.

Recorded execution evidence (from `tasks.md`; not re-run):

- Full suite: **140 scenarios (140 passed), 584 steps (584 passed)**, exit 0.
- E2E scope: **6 scenarios (6 passed), 42 steps (42 passed)**, exit 0.
- The e2e scope is strictly smaller than the full scope: **6 < 140**.
- Arithmetic against the preceding archive receipt: `declare-public-home`
  recorded 132 regular and 4 e2e scenarios; this delta adds 8 scenarios (6
  regular, 2 `@e2e`), giving 140 full and 6 e2e. The delta has exactly the two
  `@e2e` scenarios its feature file declares.
- No `givn/archive/enable-repository-security/verification.json` exists yet;
  the archive gate writes the per-scope receipt after this review. The counts
  above are the available execution evidence at review time.

## Fabrication audit (steps 0–12)

### 0. `@e2e` tag integrity — clean

The delta feature contains exactly two `@e2e` scenarios, both
`@givn.added @e2e`: the preview and the apply scenarios. `git log -p` for
`givn/changes/enable-repository-security/specs/apply-configuration-change/enable-repository-security.feature`
shows both tags introduced in `cafab30` and only `@wip` removed in `52b8c26`
and `a8fbd40`. No planned `@e2e` coverage was removed to bypass the
verify-e2e gate.

### 1. Empty or trivial step bodies — clean

**0 empty step bodies found across the 11 step-definition files scanned**
(`apps/cli/tests/steps/*.rs` and `apps/cli/tests/steps/e2e/*.rs`). The scan
matched every Rust function body; no body was `{}`, whitespace-only, bare
`return;`, `pass`, `todo!`, or `unimplemented!`. The two capability files have
substantive bodies: the non-e2e file drives real `tofu` subprocesses, the
adoption command, the fake journal, the plan JSON and the state; the e2e file
drives the same binary and commits the transcript.

### 2. Checked task → commit → source cross-check

Every checked scenario task records a revision. Spot-checked with
`git show --stat`:

| Task | Revision | Commit content verified |
|---|---|---|
| Setup task + scenario 1 | `ec38b47` | `infra/security.tf`, `infra/repos.tf`, `infra/adoption-batch-security.txt`, `.gitignore`, fake server and both fixture trees, setup script, both step modules, `features_runner.rs`, tasks |
| 2 first change refused | `abd3bc2` | step failure path, runner/world, design/tasks correction, feature |
| 3 live edit | `e830590` | step preflight refusal, feature |
| 4 unadopted setting | `7287552` | step guard/live reads, feature |
| 5 read-back failure | `e19691d` | step fault and stale path, feature |
| 6 partial failure | `6a0f096` | off-routine apply helper, stale sidecar, feature |
| 7 preview `@e2e` | `52b8c26` | e2e steps, shared helpers, committed transcript, feature |
| 8 apply `@e2e` | `a8fbd40` | e2e steps, committed transcript, `security-applied` fixture, feature |

The product’s Rust crates are untouched by design; the change’s production
surface is the declaration plus the executable test seam. Every commit touches
the declaration, the fake/setup, the step definitions or the feature, not only
a spec/stub.

**Gate finding (F1):** the tasks record the revisions in prose after each
`- [x] COMMIT —` line, not in the gate’s required `- [x] COMMIT: <identifier>`
form. `givn check review` therefore reports `no "- [x] COMMIT:" lines found in
tasks.md`. The same command on the archived `declare-public-home` change
passes, so this is not a tool-wide false positive.

### 3. Promised components — one missing

All executable/components named by `design.md` exist: `infra/security.tf`,
the `security_and_analysis` block and the dedicated Dependabot resources,
`infra/adoption-batch-security.txt`, the changed encrypted state and backup,
the `.gitignore` rules, the fake’s `--writes` sandbox, both fixture trees, the
setup script, both `mod.rs` files, both step files, the `DutyWorld` fields, the
CI setup step, the live-read evidence and the two visual transcripts.

**Finding (F4):** `design.md`’s file table promises
`givn/changes/enable-repository-security/evidence/real-apply/transcript.txt`
(“new after GREEN — the scrubbed transcript of the required operator apply
against the real organization”), and “Post-GREEN operator apply” step 5
requires the scrubbed preflight/plan/apply/follow-up transcripts and the two
adopted lines there. No such file or directory exists in the tree or in the
git history. The live operator note records the outcome in prose, and the
encrypted state serial moved from 6 to 9 (consistent with two direct imports
plus one targeted apply), but the design’s required real-apply evidence is not
committed.

### 4. Strict-mode proof — present

`apps/cli/tests/features_runner.rs` calls `fail_on_skipped()`. `tasks.md`
records the setup and e2e strict proofs with non-zero exits and
`Step doesn't match any function`; the feature has no `@wip` tag. The fake’s
unknown-path 599 marker and refused-write journal are the second strictness
net.

### 5. E2E Then-step real-interface assertions — present

Both `@e2e` scenarios assert the real CLI interface, not only a repository or
database:

- The preview scenario’s Then asserts the preview transcript’s plan summary
  (`0 to add, 1 to change, 0 to destroy`), the two changed status lines, the
  saved plan JSON’s only changed resource and live attributes, and no create
  or delete. The fake journal and the live read are secondary.
- The apply scenario’s Then asserts the apply transcript’s
  `github_repository.mono` change and `Apply complete! Resources: 0 added, 1
  changed, 0 destroyed.`, then the fake’s live enabled statuses, the state
  read with the real binary, the journal’s single write, the unchanged
  declared fields, and the empty follow-up preview. The first assertion is on
  the CLI output; the rest is secondary evidence.

Neither scenario is a repository-only assertion.

### 6. Driver fidelity — valid for a CLI

This capability’s interface is a terminal CLI (`tofu` plus the adoption
command), not a browser UI. The e2e steps use `std::process::Command` through
the shared `tofu` helper; they do not call `fetch()`/`XMLHttpRequest`, send an
HTTP request from page JavaScript, or call an in-process routine. A browser
driver is not applicable. The opposite classification (a rendered page would
be a Web UI and would require a real browser driver) is addressed in “E2E
policy and real-interface assertion” below.

### 7. Exact files `verify.e2e_command` invokes, and parallel-implementation search

`./run-tests.sh --e2e` invokes:

- `./run-tests.sh` itself (`--e2e` → `--tags "@e2e"`);
- `scripts/assemble-features.sh` (feature root assembly);
- `cargo test -p duty2cal --test features_runner -- --tags "@e2e"`, i.e.
  `apps/cli/tests/features_runner.rs`;
- for the capability,
  `apps/cli/tests/steps/e2e/enable_repository_security.rs` (Given/Then,
  transcript) and `apps/cli/tests/steps/enable_repository_security.rs`
  (shared `preview`, `apply_previewed`, `ensure_base_state`, `tofu`, `jq`,
  live-read and state helpers).

The tree was searched for a second/parallel implementation of the same
capability: `grep -rln "Organization owner previews the security settings|
enable_repository_security"` (excluding `.git` and `target`) returns only the
feature, the two step files, the two `mod.rs` files, the design/review/tasks
documents and the arc42 building-block view. `git status --porcelain` is clean;
there is no untracked second e2e implementation. No weaker parallel
implementation was found.

### 8. E2E scope and normalized inventory

The owning use case has exactly two normalized interactions:

1. preview the declared configuration and read the named differences;
2. apply the previewed revision and read the outcome.

The feature has exactly one `@e2e` scenario for each. The other six scenarios
are variants/error cases of those actions (unreadable setting, refused first
change, live edit, unadopted Dependabot setting, read-back failure, partial
failure). There is no second `@e2e` for the same action and no missing
inventory action.

### 9. E2E command isolation and counts

`verify.e2e_command` is not a copy of `verify.command`; it is the same runner
with the `@e2e` filter. The recorded counts are 140 full and 6 e2e; the e2e
scope is strictly smaller. No archive receipt exists before archive; the
archive gate will record the per-scope receipt. The `tasks.md` counts are the
available evidence, and the archive will measure the merged state.

### 10. Implementation vs `design.md` — commands, files, framework and data

| Design item | As built | Match? |
|---|---|---|
| `verify.command` = `./run-tests.sh` | `givn/commands.yaml` and `run-tests.sh` | yes |
| `verify.e2e_command` = `./run-tests.sh --e2e` | `givn/commands.yaml` and `run-tests.sh`; `--tags @e2e` | yes |
| Non-e2e step file `apps/cli/tests/steps/enable_repository_security.rs` | exists, real subprocess/state assertions | yes |
| E2E step file `apps/cli/tests/steps/e2e/enable_repository_security.rs` | exists, separate; transcript committed | yes |
| Framework: cucumber-rs with `std::process::Command` | runner and step files | yes |
| `infra/security.tf`, the `security_and_analysis` block, `infra/adoption-batch-security.txt` | all present and wired into the scenarios | yes |
| Fake `--writes`, journal body keys, `after_applied_writes`/`once`, both fixture trees | `scripts/fake-github/serve.py` and fixtures implement them | yes |
| Setup script: pin, mirror, terraformrc, second batch and base apply proof | `scripts/setup-infra-tests.sh` implements them; tasks record the proof | yes |
| CI runs the setup script before the suite | `.github/workflows/ci.yml` | yes |
| `DutyWorld` gains `plan_path`, `preview_transcript`, `apply_transcript` | `features_runner.rs` | yes |
| Visual transcripts per `@e2e` scenario | two committed transcripts under `evidence/visual/` | yes |
| Real-apply transcript after GREEN | absent | **no (F4)** |
| Post-GREEN step 4: “on success, `tofu plan` again confirms no differences” | live note says the full preview still names the organization billing drift; only the targeted plan exits 0 | **no (F5)** |
| Delta use case parses for capability routing | `givn check review` reports parse errors; a scratch copy with formatting fixes parses and routes `NEW in apply-configuration-change` | **no (F2)** |
| No implementation-layer detail in the spec | the feature names the fake’s journal and the HTTP `PATCH` | **no (F6)** |

The live operator note was added in `5d0710a`, after the last
`design-review.md` `DESIGN-REVIEW: PASS` (`4b6c1cd`). The note changes the
design’s Post-GREEN procedure, but `design-review.md` has not been re-run for
it (F5).

### 11. Interaction coverage cross-reference

| Inventory entry (`specs/apply-configuration-change/usecase.md`) | Design `Interaction Coverage Matrix` row | Existing `@e2e` scenario | Driving mechanism | Match? |
|---|---|---|---|---|
| preview the declared configuration and read the named differences | preview the declared configuration and read the named differences | “Organization owner previews the security settings and only the declared differences are named” | `std::process::Command` runs `tofu init`, `tofu plan -out`, `tofu show -json` against the fake; asserts the plan text and JSON | yes |
| apply the previewed revision and read the outcome | apply the previewed revision and read the outcome | “Organization owner applies the previewed revision and the security settings are enabled” | `std::process::Command` runs the preflight, `tofu apply <plan>`, then a second `tofu plan`; asserts the transcript, journal, live read and state | yes |

Every inventory entry has a matrix row, an existing `@e2e` title and the
promised CLI driver. No missing or extra matrix row was found.

### 12. Fabrication-audit conclusion

The change is real, not a stub: the declaration, fake, setup proof, runner and
step definitions exist, and the recorded suite is green. The audit nevertheless
records four blockers outside the empty-step checks: the COMMIT-line gate
format (F1), the use-case parser format (F2), missing use-case scenarios (F3)
and the missing real-apply evidence (F4). No `@e2e` tag was removed and no
empty step body was found.

## Gate checks and artifact conformance

`givn check review --change enable-repository-security` output:

```
Mandatory checks for 'review':
[integrity] no `- [x] COMMIT:` lines found in tasks.md — revision identifiers must be recorded
error: integrity check failed: hash presence
```

A scratch copy of the change (not the repository; no artifact in the working
tree was edited) with the F1/F2 artifact corrections and a review file present
was run through the same command. It then passed integrity and overlap, and the capability
routing check reported:

```
error: capability routing decisions do not match the built delta:
  decision NEW in apply-configuration-change: usecase.md failed to parse: line 3: content is outside a recognised section
```

After removing the metadata lines, the parser moved to the next defect:
`unknown heading '## Scenario skeletons'`; then `section '## Capabilities'
must contain one capability per bullet`; `section '## Actors' must contain one
actor per bullet`; `section '## Preconditions' must contain one precondition
per bullet`; `section '## Extensions' must contain one extension per bullet`;
`section '## Examples' must contain one example per bullet`; and `section
'## Personas' must contain one persona per bullet`.

With those sections conformed in the scratch copy — title then `## Level`, no
`Name:`/`Status:` prose before `Level`, no `## Scenario skeletons`, one
physical line per bullet in Actors/Preconditions/Main flow/Extensions/Rules/
Personas/Capabilities/Out of scope, a bare `- enable-repository-security`
capability bullet, and Examples as bullets — the same scratch copy passed
`givn check review` with:

```
net delta: 8 added, 0 modified, 0 removed
```

This proves the capability routing decision itself matches the built delta; the
blocker is the delta use case’s format, not a missing capability. It also
confirms the scratch-only changes are the required fixes (F1 and F2); the
working tree remains unmodified.

`givn lint --change enable-repository-security` is clean; the lint does not
exercise the `givn check review` integrity/routing parser.

## Arc42 implementation conformance

The durable `docs/arc42/` chapters and the change-level `arc42.md` were read.
The affected chapters were independently derived from the completed
implementation, `design.md` and `tasks.md`.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 1 — reproducibility goal and Persona roster | `docs/arc42/01-introduction-and-goals.md` lines 13, 27 | affected: security settings and preview/apply | Scope, Persona impact | setup, scenarios 1–8 | declaration, step files, transcripts | yes |
| 2 — OpenTofu/provider pin, encrypted committed state, environment-only secrets, no CI apply, read-only adoption, saved-plan/preflight/stale constraint | `docs/arc42/02-architecture-constraints.md` lines 33–35 | affected: constraints | Technology decisions, apply procedure, state | setup, 1–8 | `infra/*`, `.opentofu-version`, lock, CI, step helpers | yes |
| 3 — declared configuration/adoption as project-side interface | `docs/arc42/03-context-and-scope.md` line 47 | affected: context | Architecture impact, test seam | all | declaration, command, fake | yes |
| 4 — infrastructure-as-code strategy and saved-plan apply | `docs/arc42/04-solution-strategy.md` line 10 | affected: strategy | Technology decisions | setup, 1–8 | declaration and steps | yes |
| 5 — public-home declaration and fake write sandbox | `docs/arc42/05-building-block-view.md` lines 70, 92–96 | affected: building blocks | Architecture impact, step definitions | setup, all | `infra/`, scripts, fake, step files | yes |
| 6 — security-change runtime: adoption, preview, guard, preflight, apply, partial, read-back | `docs/arc42/06-runtime-view.md` lines 204–259 | affected: new scenario and post-apply empty plan | Apply procedure, live note | 1–8 | `apply_previewed`, fake, transcripts | **no (F5)**: the durable diagram/steps say `tofu plan: no differences` (line 226) and “a second plan reports no differences” (line 251), but the live note says the full preview still names the organization billing drift; only the targeted plan exits 0 |
| 7 — deployment: workstation, environment-only secrets, ciphertext state, no CI apply | `docs/arc42/07-deployment-view.md` line 99 | affected: deployment | Applied state, live note | 5, 6, operator apply | state serial 6→9, command, CI | yes |
| 8 — declared-vs-applied state, saved plan, preflight, stale marker | `docs/arc42/08-crosscutting-concepts.md` lines 100–145 | affected: crosscutting | Decisions, apply procedure | 3–6 | step code, fake journal, state | yes |
| 9 — no new ADR | `docs/arc42/09-architecture-decisions.md` | affected: no new ADR | Four NOT_QUALIFIED verdicts | change docs | ADR-0005/0006 unchanged | yes |
| 10 — QS-019/QS-020 | `docs/arc42/10-quality-requirements.md` line 59 | affected: quality | Traceability, coverage | 1–8 | e2e, refusals, state | **no (F5)**: QS-019’s “a later preview reports no differences” is unqualified; the live note says the full preview still names the pre-existing organization billing drift |
| 11 — R-026…R-031, TD-013 | `docs/arc42/11-risks-and-technical-debt.md` lines 32–37, 55 | affected: risks | Risks, live note | 1–8, operator apply | declaration, fake, state | yes |
| 12 — **Security settings**, refined **Configuration change** | `docs/arc42/12-glossary.md` rows Security settings / Configuration change | affected: glossary | Decisions, file layout | all | `infra/security.tf`, `security_and_analysis`, state/stale marker | yes |

The five arc42 sign-off checks do not all hold: the affected chapters were
independently derived and are current for the fake-driven security change, but
chapters 6 and 10 contain an unqualified “no differences” claim that the live
operator note contradicts. The clean marker is not recorded.

**ARC42 CONFORMANCE: NOT CLEAN** — fix F5, then repeat this block.

## Ubiquitous language conformance

The durable glossary’s confirmed terms are reused with their exact senses:
**Security settings** (secret scanning, push protection, Dependabot alerts and
Dependabot security updates), **Configuration change** (the saved plan,
refresh-only preflight, apply, state record and stale marker), **Declared
configuration**, **Adoption**, **Configuration drift**, **Public home** and
**Protection ruleset**. The spec, design and implementation use “security
settings”, “declared configuration”, “applied state”, “preview”, “apply”,
“saved plan” and “stale applied state” consistently with the glossary. The
design’s “settings toggles” is descriptive of the two Dependabot resources, not
a new domain term; the glossary’s **Security settings** row already names them.
No conflicting sense or silently promoted term was found.

**UBIQUITOUS LANGUAGE: CONFORMANT**

## Coverage classification

Coverage is measured by the archive against the committed candidate and
published in the verification receipt. No archive receipt exists for this
change yet. The most recent measured evidence is
`givn/archive/declare-public-home/verification.json`: **98% line coverage
(871/886)** at revision `47277e1`. This change adds no production Rust: the
only Rust added is step definitions and `DutyWorld` fields, which the runner
instruments as test code. The HCL, the fake server and the shell scripts are
outside the Rust coverage denominator by the design’s declared process
boundary; their observable contract is exercised by the eight scenarios.

Classification from inspection, using only the three buckets:

1. **Dead code (YAGNI/KISS) → delete.** No dead production code was found. The
   product crates are untouched. The fake’s `DELETE` handlers are test-twin
   plumbing for the provider’s delete path and are not exercised by these
   scenarios; they are test support, not product code, and are outside the
   production denominator. No product deletion is required.
2. **Missing test coverage → add scenario, run RED→GREEN→REFACTOR→COMMIT.**
   Four use-case behaviours are not covered by a delta scenario:
   - a declaration edited after the preview: the saved plan applies the
     previewed revision, the edit is not carried, and a new preview names it
     (use case main flow 4, rule P2, extension line 72, example line 146,
     scenario skeleton lines 163–165);
   - a preflight report that does not complete: the apply refuses, the saved
     plan is not applied, and the incomplete report is not treated as a no-op
     (use case extension line 82; `design.md` “The apply procedure” implements
     an `incomplete` branch and `apply_previewed` handles it, but no scenario
     drives it);
   - a declared revision that is invalid: the preview refuses and names the
     problem (use case extension line 64);
   - the applied state cannot be read: the preview refuses and names it (use
     case extension lines 62–63; scenario 1 covers only the live read).

   The first is explicitly in the use case’s examples, rules and scenario
   skeletons; the others are in its extensions. None has a recorded operator
   deferral, so none is excused. All four are testable through the existing
   real-`tofu` seams; they are bucket 2, not bucket 3.
3. **Legitimately hard to test (rare) → concrete technical justification.**
   None. The shell/Python/HCL are outside the Rust denominator by the design’s
   declared process boundary; their observable contract is exercised by the
   scenarios, and the operator’s real apply is the live evidence (F4).

No fourth bucket, and no “acceptable for now”, “future scope”, “sufficient for
current scope” or similar exception is used. The missing scenarios are bucket
2 and must be added. No redundant unit tests were added; the `.feature` files
remain the source of truth.

## E2E policy and real-interface assertion

The canonical policy was read from
`givn instructions specs --change enable-repository-security`.

- **Opposite classification:** a rendered page is a Web UI, and a Web UI would
  require a real browser driver such as Playwright; an HTTP client would not be
  a valid substitute. That classification **does not hold here**: the
  interface is the terminal `tofu` CLI plus the adoption command; its output is
  plain text on stdout/stderr. There is no HTML, CSS, JavaScript, DOM, page
  request or browser-rendered state. The design correctly declares
  `Interface: terminal`.
- **Real interface:** real `tofu` subprocesses (`init`, `plan -out`,
  `show -json`, the refresh-only preflight, `apply <plan>`) against the local
  fake GitHub API, plus the real adoption command.
- **Primary assertion:** the CLI output — the preview’s plan summary and
  changed statuses, and the apply’s changed resource and `Apply complete!`
  line. The journal, live fake reads and `tofu state` are secondary evidence;
  neither e2e Then is repository-only.
- **Driver:** `std::process::Command`, not an in-process call and not HTTP from
  page JavaScript.
- **Scope:** two normalized actions, two `@e2e` scenarios; variants are regular
  scenarios.
- **Canonical one-per-action:** no capability has more than one `@e2e` for the
  same action.

## Use-case and Persona conformance

The owning use case is
`givn/changes/enable-repository-security/specs/apply-configuration-change/usecase.md`;
the ideation contract is
`givn/ideation/public-home-infrastructure/use-cases/apply-configuration-change.md`.
The implementation preserves the goal, the minimal/success guarantees’ intent,
the rules, capability ownership and the confirmed Persona references. The
implementation drives the real declaration and state through the real `tofu`
binary. The live operator deviation is assessed separately below.

Use-case scope rejection criteria checked:

| Criterion | Result |
|---|---|
| Goal conjunction | Not triggered: one goal phrase (“Make the live configuration match the declared configuration revision the apply accepts”) |
| Success guarantee names two outcomes | **Triggered (F7)**: “The live configuration matches … and the applied state records that revision” names two outcomes; the applied-state record belongs in the rules/main flow, not the success guarantee |
| Missing primary actor | Not triggered: Organization owner is primary and the trigger names the owner |
| Placement failure | Not triggered: `enable-repository-security` advances the organization owner’s administration goal |
| Mutation under an observing goal | Not triggered: the use case explicitly declares itself changing |
| Extension without a step reference | **Triggered (F7)**: extension line 72 is anchored to step `3` (the review step), but the declared-edit-after-preview behaviour belongs to step `4` (apply) or step `2` (preview) |
| Inconsistent preconditions | Not triggered: the stale-state precondition is a state, the trigger is an action, the success guarantee is not assumed, and the Q11 guarantee is qualified |
| Mechanism vocabulary in goal/trigger/guarantees | Not triggered: “apply” is the domain action, not a component/channel name |
| Component- or channel-shaped name | Not triggered |
| Oversized leaf | Not triggered: the main flow is one actor goal with supporting steps |
| Relationship misuse | Not triggered: no Extends/Includes |

Confirmed Personas and dispositions:

| Persona | Evidence and disposition |
|---|---|
| `project-maintainer--organization-owner` | Runs the preview and apply, reads the named differences, sees the enabled statuses/apply outcome and answers for refusals and partial outcomes; the two committed transcripts show the preview and apply reach the goal. |
| `contributor--infrastructure-reviewer` | Reads `infra/security.tf`, the second batch and the encrypted state in one history, holds no token or passphrase and cannot apply; the declaration/batch are text and the state is ciphertext. Goal reached as a review lens. |
| `crew-member--personal-calendar-owner` | The parent use case’s lens, not a Persona confirmed for this change and not a Gherkin Actor; no release object, rule or archive is touched, so no impact and no invented Persona. |

A user-visible surface (the terminal preview/apply output) has two confirmed
Personas. No invented or silently promoted Persona was found. This is not a
migration; no migration-ledger evidence applies. The live operator note’s
direct-import/targeted-apply deviation is recorded but is not fully reconciled
with the use case’s full success guarantee (F5).

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
  (91 lines): the base apply, both adoptions, the plan with
  `0 to add, 1 to change, 0 to destroy`, the two `disabled -> enabled`
  statuses, the base apply's follow-up `No changes.`, and `exit status: 0`.
- apply:
  `evidence/visual/organization-owner-applies-the-previewed-revision-and-the-security-settings-are-enabled/transcript.txt`
  (122 lines): the preview, the preflight, the `github_repository.mono`
  update, `Apply complete! Resources: 0 added, 1 changed, 0 destroyed.`, the
  follow-up `No changes.` preview, and `exit status: 0`.

The transcripts carry the final state for both declared terminal scenarios;
no screenshots or viewports apply. The run-specific directory, fake port,
token, passphrase and billing email are placeholders; no token, passphrase,
temporary path or live credential appears. The real plan’s provider state-only
`ignore_vulnerability_alerts_during_read` field appears; `design-review.md` G6
classifies it as bookkeeping and the design’s baseline does not prohibit it.

Fresh-context Persona lenses against the committed transcripts:

- **Organization owner:** sees the preview’s two named statuses and the
  apply’s changed resource and success line; can reach the preview/apply goal
  with what is shown. No finding.
- **Contributor reviewer:** can read the declaration, batch and ciphertext
  state and is not given an apply path; can reach the review-without-authority
  goal. No finding.
- **Crew member:** the transcripts show no site or release operation; the
  parent lens is unaffected. No finding.

Every finding would have to name a contract rule or a failed Persona goal; none
was found. No pixel-level or coordinate finding applies.

**VISUAL-REVIEW: PASS**

## Live operator deviation assessment

The live run is assessed against the seed and the change’s own contract:

- **Seed guarantee:** the live API is recorded as reporting secret scanning and
  push protection `enabled`; the two Dependabot settings are bound. The
  encrypted state serial moved from 6 (the predecessor’s state) to 9, which is
  three writes after the predecessor: two direct imports plus one targeted
  apply. The targeted plan exits 0. The seed’s guarantee is met as recorded.
- **Direct imports and targeted apply:** the second adoption command refused
  the batch because its whole-state refresh saw a pre-existing organization
  billing drift. Both Dependabot resources were imported directly by their
  repository-name ids (`1416075332` for the alerts resource and `duty2cal` for
  the updates resource), and the security plan was applied with
  `-target=github_repository.mono`. The design’s live note (lines 964–980) and
  the tasks final operator box (lines 364–377) record the refusal, the direct
  imports, the targeted apply and the reason; the direct imports satisfy
  adopt-before-create, and the target keeps the unrelated drift out of the
  apply.
- **Adoption whole-state defect:** the note records it as a defect for a
  follow-up: the adoption verifier must refresh only the batch addresses so a
  pre-existing drift elsewhere does not refuse an unrelated adoption. It is
  not silently accepted. The follow-up is recorded in `design.md` and
  `tasks.md`; it is not yet an actionable successor change/issue.
- **What is not reconciled:** the design’s Post-GREEN step 4 says a follow-up
  `tofu plan` confirms no differences; the live note says the full preview
  still names the organization billing drift. Arc42 chapters 6 and 10 repeat
  the unqualified “no differences” claim. The live note was added after the
  last `design-review.md` PASS, and the design was not re-reviewed. The
  targeted security plan is empty and the security guarantee is met, but the
  design/arc42 full-declaration claim is stale. Fix F5; do not treat the note
  as a silent exception.

## Findings and required fixes

### F1 — Record task commits in the gate’s `COMMIT:` syntax (blocker)

`givn check review --change enable-repository-security` refuses at
`[integrity] no "- [x] COMMIT:" lines found in tasks.md`. The revisions are
present in prose after each `- [x] COMMIT —` line, but the gate parses only
`- [x] COMMIT: <identifier>`. The archived `declare-public-home` change passes
the same check, so the failure is specific to this tasks.md format.

Fix: at `tasks.md` lines 90, 118, 148, 180, 210, 240, 305 and 346, change the
line to the gate’s form with the recorded revision, preserving the rest of the
task text and the “Revision:” evidence, for example:

`- [x] COMMIT: \`ec38b47\` — ...` for the setup task/scenario 1,
`- [x] COMMIT: \`abd3bc2\` — ...` for scenario 2,
`- [x] COMMIT: \`e830590\` — ...`, `- [x] COMMIT: \`7287552\` — ...`,
`- [x] COMMIT: \`e19691d\` — ...`, `- [x] COMMIT: \`6a0f096\` — ...`,
`- [x] COMMIT: \`52b8c26\` — ...` and `- [x] COMMIT: \`a8fbd40\` — ...`.
Then re-run `givn check review`.

### F2 — Conform the delta use case to the capability-routing parser (blocker)

`givn check review` cannot parse
`specs/apply-configuration-change/usecase.md`: `line 3: content is outside a
recognised section` (`Name:`/`Status:` before `## Level`), then
`unknown heading '## Scenario skeletons'`, then the section-bullet errors for
Capabilities, Actors, Preconditions, Extensions, Examples and Personas. A
scratch copy with only formatting changes parses and passes the routing check
with `NEW in apply-configuration-change` and `net delta: 8 added, 0 modified,
0 removed`, so the capability is correctly routed; the delta document is not
in the parser’s shape.

Fix: rewrite the use case header and list sections to the shape the permanent
use cases use:

- title, then `## Level`; remove the `Name:`/`Status:` prose before `Level`
  (the title already names the use case; keep the provenance in the proposal
  or design);
- remove the unrecognized `## Scenario skeletons` heading (move its content to
  the feature, `design.md`, or an existing allowed section);
- one physical line per bullet in `## Actors`, `## Preconditions`,
  `## Main flow`, `## Extensions`, `## Rules`, `## Personas`,
  `## Capabilities` and `## Out of scope` (do not wrap a bullet across lines);
- make `## Capabilities` a bare bullet: `- enable-repository-security`;
- convert the `## Examples` table to one bullet per example, matching the
  permanent use cases’ examples style.

Then re-run `givn check review`.

### F3 — Add the missing use-case scenarios (blocker; bucket 2)

The delta has eight scenarios; four observable use-case behaviours have no
delta scenario:

1. **Declaration edited after the preview** — the saved plan applies the
   previewed revision, the edit is not carried, and a new preview names it.
   Sources: use case main flow 4, rule at line 98, extension line 72, example
   line 146, skeleton lines 163–165. This is the most important gap: it is the
   P2 rule and an explicit example/skeleton.
2. **Preflight report does not complete** — the apply refuses, does not treat
   an incomplete report as a no-op, and does not apply the saved plan.
   Source: use case extension line 82; `design.md` “The apply procedure” and
   the `apply_previewed` incomplete branch implement the behaviour, but no
   scenario drives it.
3. **Declared revision is invalid** — the preview refuses and names the
   problem. Source: use case extension line 64.
4. **Applied state cannot be read** — the preview refuses and names it.
   Source: use case extension lines 62–63; scenario 1 covers only the live
   read.

Fix: add regular (non-`@e2e`) scenarios under
`specs/apply-configuration-change/enable-repository-security.feature` and the
corresponding step definitions in
`apps/cli/tests/steps/enable_repository_security.rs`; drive the existing real
`tofu`/fake seams; follow RED→GREEN→REFACTOR→COMMIT for each and record the
outputs and revisions in `tasks.md` in the new `COMMIT:` form.

### F4 — Commit the required real-apply transcript (blocker)

`design.md` line 97 promises
`givn/changes/enable-repository-security/evidence/real-apply/transcript.txt`;
“Post-GREEN operator apply” step 5 (lines 919–921) requires the scrubbed
preflight/plan/apply/follow-up transcripts and the two adopted lines there.
The file does not exist, and no commit added it. The live-read evidence is
pre-change and reports both scanning statuses disabled; the e2e transcripts
are fake-only; the operator note and the state serial 6→9 are corroboration,
not the required scrubbed real-run output.

Fix: commit a scrubbed `evidence/real-apply/transcript.txt` containing the
adoption refusal and reason, the two direct imports and their ids, the targeted
plan (`0 added, 1 changed, 0 destroyed`; the two statuses), the targeted apply,
the follow-up targeted plan (`No changes.`), the full preview’s organization
billing drift (outside this change), and the live API read showing both
statuses enabled; replace token, passphrase, billing email and temporary paths
with placeholders. If the operator note is intended to be the evidence instead,
update `design.md`’s file table and Post-GREEN step 5 to say so and re-run
design-review; the note alone does not carry the required command output.

### F5 — Reconcile the design/arc42 full-preview claim and re-run design-review (blocker)

`design.md` Post-GREEN step 4 (line 916) says “on success, `tofu plan` again
confirms no differences.” The live note (line 980) says the full preview still
names the organization billing drift; only the targeted plan exits 0. Arc42
chapter 6 (lines 226, 251) and QS-019 (`docs/arc42/10-quality-requirements.md`
line 59) repeat the unqualified “no differences” claim. The live note was
added in `5d0710a`, after the last `design-review.md` PASS. The targeted apply
and direct imports are recorded and justified, but the design’s full-declaration
claim and its Post-GREEN procedure are stale, and the design change was not
re-reviewed.

Fix: update `design.md` “Post-GREEN operator apply” steps 1, 4 and 5 to record
the adoption command’s refusal and the direct-import recovery, the targeted
apply, the targeted plan’s empty result, and the full preview’s pre-existing
organization billing drift (outside this change); qualify arc42 chapter 6 and
QS-019 to the security-scoped/targeted preview (or record the full-preview
drift and its follow-up); add an actionable follow-up item for the adoption
command’s whole-state verification (scope the refresh to the batch addresses);
then run design-review again and record `DESIGN-REVIEW: PASS` for the corrected
design.

### F6 — Remove test-harness detail from the feature (blocker for the spec sign-off)

The feature names the fake server’s journal and the HTTP `PATCH`:

- line 26: `And the journal's repository request changed only
  "security_and_analysis" with both statuses enabled`;
- line 74: `Scenario: A failed read-back after the PATCH is reported ...`;
- line 77: `And the read of the monorepo fails once after the PATCH is
  applied`.

The journal is a test-fake artifact, not a user-observable interface, and the
spec instruction forbids step/framework detail in the executable spec. The
primary e2e assertions on the CLI output are correct; the secondary journal
check belongs in the step definition.

Fix: reword the Then to an observable domain result (for example, “the apply
changed only the repository’s security settings and no other declared field”);
reword the read-back scenario and Given to the repository update rather than
the `PATCH`; keep the journal assertions inside
`apps/cli/tests/steps/e2e/enable_repository_security.rs`. Re-run the affected
scenarios and update the scenario text/revisions.

### F7 — Correct the use-case success guarantee and extension anchor (blocker for use-case conformance)

- The success guarantee (lines 189–191) names two outcomes: the live
  configuration matches, and the applied state records the revision. The
  review criterion requires one success outcome. The applied-state record is
  already main flow 5 and a rule; keep it there.
- Extension line 72 is anchored to step `3` (the review step), but a
  declared-configuration edit after the preview is an apply/preview
  behaviour. It belongs at step `4` (or step `2` for the preview).

Fix: rewrite the success guarantee to the single outcome (“The live
configuration matches the declared configuration revision the apply accepted
for every object the revision retains”) and renumber the extension to `4:`
(or `2:` if the failure is detected at the preview), preserving its text and
the Q11 qualifier. Re-run the artifact checks after the edit.

## Overlap dispositions

`givn lint --change enable-repository-security` reports no `[SHAPE]` or other
overlap finding; a scratch copy with the F1/F2 formatting fixes passes
`givn check review`’s overlap check. There are no finding pairs to disposition
and no removed+added pair in the delta.

## Split-or-keep

No scenario exceeds the long-scenario gate threshold; `givn lint` reports no
`[LONG]` finding. No split-or-keep decision is required. There are no
removed+added pairs.

## README impact

`README-IMPACT: none`

The change adds maintainer infrastructure (`infra/security.tf`,
`infra/adoption-batch-security.txt`, the fake/setup/seam). It does not change a
product command, flag, configuration file, output or any section of the README
(Description, Quick start, Prerequisites, Usage, Configuration, Contributing,
License). The declaration and the operator routine are documented in
`design.md` and `tasks.md`, not in the public README.

## Sign-off checklist

| Check | Result |
|---|---|
| Fabrication audit clean (or all findings remediated) | **No** — F1–F7; no empty steps or removed `@e2e`, but the gate and completeness findings remain |
| Every checked task has a verified commit touching production code | Revisions verified for all eight tasks; the gate’s `COMMIT:` syntax is F1 |
| Every promised component exists | **No** — F4 (`evidence/real-apply/transcript.txt`) |
| Strict-mode proof present and passing | Yes — `fail_on_skipped()`; tasks record two undefined-step proofs; no `@wip` |
| Scenario execution evidence recorded per scope | Yes, recorded: 140/140 and 584 steps; 6/6 e2e and 42 steps; no archive receipt exists yet; review did not re-run |
| Coverage classification uses the most recent measured evidence | Yes — last archive receipt 98% (871/886); no current receipt; static inspection used |
| Every coverage gap classified and resolved | **No** — F3 is bucket 2 (missing scenarios); no bucket 4 |
| Dead code deleted; missing tests added; hard-to-test gaps justified | **No** — F3 missing scenarios; no hard-to-test gap; no product dead code |
| Redundant unit tests removed | Yes — no unit tests added; `.feature` is the source |
| No `@wip` tags; no implementation-layer detail in the spec | `@wip`: 0; spec detail: **F6** |
| Canonical E2E policy read; one normalized action per `@e2e`; variants regular | Yes — two inventory actions, two `@e2e`; six variants regular |
| Every E2E scenario has a real-interface primary assertion and the reviewed driver; no HTTP/fetch substitution | Yes — CLI output first; `std::process::Command`; terminal, not a browser |
| Local run command starts the full stack (digital twins) | Recorded, not re-run: the scenarios start the fake themselves; `scripts/setup-infra-tests.sh` prepares the mirror and is in CI |
| Exact files `verify.e2e_command` invokes identified and read; parallel implementation searched | Yes — `run-tests.sh`, `assemble-features.sh`, `features_runner.rs`, the two step files; tree search found no parallel implementation |
| `verify.e2e_command` distinct; counts prove a strictly smaller e2e scope | Yes — `./run-tests.sh --e2e` vs `./run-tests.sh`; 6 < 140 |
| Implementation matches design’s commands, files, framework/driver | **No** — F4 (real-apply evidence), F5 (full-preview claim), F6 (spec detail); commands/files/framework otherwise match |
| Interaction coverage verified | Yes — two inventory entries → two matrix rows → two existing `@e2e` → promised CLI driver |
| No finding excused outside the three coverage buckets | Yes — F3 is bucket 2; no “acceptable for now”, “future scope” or similar exception |
| `givn check review` gate | **Fails** — F1 (integrity) and F2 (capability-routing parse) |

The earned verdict is FAIL: the change is substantially real and the recorded
suite is green, but the review gate refuses, four use-case behaviours lack
scenarios, the required real-apply transcript is absent, and the design/arc42
full-preview claim is stale after the live operator deviation.

REVIEW: FAIL
