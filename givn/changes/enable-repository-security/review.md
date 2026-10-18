# Review: enable-repository-security

Change: `enable-repository-security`
Stage: `ready-to-archive` (`givn status` reports 44/44 tasks)
Review date: 2026-10-18
Verdict: **FAIL** (F1–F7 are remediated and the change is mechanically green, but the
corrected design has not been re-reviewed by `design-review`, which the review
instruction requires after a `design.md` change)

## Preconditions

- `givn instructions review --change enable-repository-security`,
  `givn instructions specs --change enable-repository-security` and
  `givn instructions dialog` were loaded before this report was written.
- `givn status --change enable-repository-security` reports 44/44 tasks and
  stage `ready-to-archive`; only the review stage was pending when this review
  began.
- This review was read-only except for writing this file. No other artifact,
  fixture, step definition, declaration or state file was edited.
- The suite was not re-run, per the assignment. The counts below are the
  recorded execution evidence in `tasks.md`, not a fresh run.
- `givn lint --change enable-repository-security` was run:
  `givn lint: 1 file(s) checked — clean`, exit 0.
- `givn check review --change enable-repository-security` was run with a
  review file present: it passes integrity, the run declaration, capability
  routing (net delta **8 added, 0 modified, 0 removed**) and overlap
  dispositions, exit 0. The exact output is under “Gate checks”.

## Commands and scope under review

Read literally from `givn/commands.yaml`:

- `verify.command: "./run-tests.sh"`
- `verify.e2e_command: "./run-tests.sh --e2e"`

The strings differ. `run-tests.sh` maps `--e2e` to `--tags "@e2e"` and invokes
`cargo test -p duty2cal --test features_runner -- --tags "@e2e"` over the
feature root assembled by `scripts/assemble-features.sh` (the permanent corpus
plus the active change’s `specs/**`). `apps/cli/tests/features_runner.rs`
calls `Cucumber::fail_on_skipped()`.

Recorded execution evidence (from `tasks.md`; not re-run):

- Full suite: **140 scenarios (140 passed), 584 steps (584 passed)**, exit 0.
- E2E scope: **6 scenarios (6 passed), 42 steps (42 passed)**, exit 0.
- The e2e scope is strictly smaller than the full scope: **6 < 140**.
- `givn lint --change enable-repository-security`: **clean**, exit 0.
- The most recent archive receipt (`givn/archive/declare-public-home/verification.json`,
  candidate `47277e1`) records the full `verify.command` scope at **132**
  scenarios (including **4** `@e2e`) and the e2e scope at **4**; this delta adds
  8 scenarios (6 non-e2e, 2 `@e2e`), giving **140 full** and **6 e2e**. There is
  no `givn/archive/enable-repository-security/verification.json` yet; the archive
  gate writes the per-scope receipt after this review.

## F1–F7 dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| F1 — task commits in the gate’s `COMMIT:` syntax | **Fixed.** Every scenario task records `- [x] COMMIT: \`<hash>\``. | `tasks.md` lines 90, 118, 148, 180, 210, 240, 305, 346; `givn check review` reports `✓ integrity passed`. |
| F2 — delta use case parses for capability routing | **Fixed.** The use-case header, sections and bullets are in the parser’s shape; `## Level` is `!`, the project convention used by the permanent use cases. | `givn check review` reports no routing error and prints `net delta: 8 added, 0 modified, 0 removed`; the decision `NEW in apply-configuration-change` matches the built delta. |
| F3 — the delta use case states only what the eight scenarios assert | **Fixed.** The four removed behaviours are in `## Out of scope`; every remaining contract statement maps to a scenario (table below). | `specs/apply-configuration-change/usecase.md`; the mapping table under “Fabrication audit, step 12”. |
| F4 — commit the required real-apply transcript | **Fixed.** `evidence/real-apply/transcript.txt` exists (72 lines) with the adoption refusal, both direct imports and ids, the targeted plan (`0 to add, 1 changed, 0 destroyed`), the targeted apply, the empty targeted follow-up, and the `gh api` read showing both scanning statuses enabled; the three post-apply JSON files exist. | `evidence/real-apply/transcript.txt`; `evidence/live-read/post-apply-security.json`, `post-apply-repo-settings.json`, `post-apply-org-settings.json`; `evidence/live-read/README.md` §“Post-apply read — 2026-10-18”. |
| F5 — design and arc42 state the targeted scope and the full-preview drift | **Content fixed; re-review missing (F8).** `design.md` Post-GREEN steps 1, 3, 4 and 5 and the correction note say the targeted security preview/apply is empty and the full preview still names the organization billing drift, with “the full configuration is not clean”. `docs/arc42/06` and QS-019/QS-020 say the same. But `design.md` changed after the last `DESIGN-REVIEW: PASS`, and `design-review.md` was not re-run (F8). | `design.md` §“Post-GREEN operator apply”, §“Live operator run”; `docs/arc42/06-runtime-view.md` lines 204–262; `docs/arc42/10-quality-requirements.md` QS-019/020. |
| F6 — remove fake-journal/HTTP-`PATCH` detail from the feature | **Fixed.** The feature no longer contains `journal`, `PATCH` or “write request”; the read-back scenario and Given name the security settings. Every step text matches its definition (including the renamed `the live configuration changed only the security settings`). | `greps` return no `journal|PATCH|write request` in the delta feature; the step-attribute check covers all 8 scenarios; the recorded e2e run is 6/6. |
| F7 — one success outcome and consistent saved-plan/declared-edit semantics | **Fixed.** `## Success guarantee` names one outcome; the rule “the applied revision is the previewed revision” and the out-of-scope bullet for a declaration edited after the preview state the same semantics; the live-edit extension is anchored to step 3 (apply). | `usecase.md` §“Success guarantee”, §“Rules”, §“Extensions”, §“Out of scope”. |

## Fabrication audit (steps 0–12)

### 0. `@e2e` tag integrity — clean

The delta feature contains exactly two `@e2e` scenarios, both
`@givn.added @e2e`: the preview and the apply scenarios. `git log -p` for the
feature shows both tags introduced in `cafab30`; `@wip` was removed in
`52b8c26` and `a8fbd40`. No planned `@e2e` coverage was removed to bypass the
verify-e2e gate.

### 1. Empty or trivial step bodies — clean

**0 empty step bodies found across the 11 step-definition files scanned**
(`apps/cli/tests/steps/*.rs` and `apps/cli/tests/steps/e2e/*.rs`). A scan for
`{}` bodies, whitespace-only bodies, `todo!`, `unimplemented!` and bare
`return`/`pass` found no step body. The two capability files have substantive
bodies: the non-e2e file drives real `tofu` subprocesses, the adoption command,
the plan JSON and the state; the e2e file drives the same binary and commits
the transcript. Every feature step text maps to a `#[given]`/`#[when]`/`#[then]`
definition in the two capability files (fixed strings and capture-group
regexes).

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

All executable components named by `design.md` exist: `infra/security.tf`, the
`security_and_analysis` block and the two dedicated Dependabot resources,
`infra/adoption-batch-security.txt`, the changed encrypted state and backup, the
`.gitignore` rules, the fake’s `--writes` sandbox, both fixture trees, the setup
script, both `mod.rs` files, both step files, the `DutyWorld` fields, the CI
setup step, the live-read evidence, the two visual transcripts and the
real-apply transcript. The design’s step-definition table omits the world’s
`write_sandbox` field (the implementation has it and the e2e Given asserts it);
this is a non-blocking documentation omission, not a missing component.

### 4. Strict-mode proof — present

`apps/cli/tests/features_runner.rs` calls `fail_on_skipped()`. `tasks.md`
records the setup and e2e strict proofs with non-zero exits and `Step doesn't
match any function`; the feature has no `@wip` tag. The fake’s unknown-path 599
marker and refused-write journal are the second strictness net.

### 5. E2E Then-step real-interface assertions — present

Both `@e2e` scenarios assert the real CLI interface first:

- The preview scenario’s first Then asserts the preview transcript’s plan
  summary (`0 to add, 1 to change, 0 to destroy`), the two changed status
  lines, the plan JSON’s only changed resource and live attributes, and no
  create or delete. The fake journal and the live read are secondary.
- The apply scenario’s first Then asserts the apply transcript’s
  `github_repository.mono` change and `Apply complete! Resources: 0 added, 1
  changed, 0 destroyed.`, then the fake’s live enabled statuses, the state
  read with the real binary, the single applied write, the unchanged declared
  fields, and the empty follow-up preview. The first assertion is on CLI
  output; the rest is secondary evidence.

Neither scenario is a repository-only assertion.

### 6. Driver fidelity — valid for a CLI

The capability’s interface is a terminal CLI (`tofu` plus the adoption
command), not a browser UI. The e2e steps use `std::process::Command` through
the shared `tofu` helper; they do not call `fetch()`/`XMLHttpRequest`, send an
HTTP request from page JavaScript, or call an in-process routine. A browser
driver is not applicable. The opposite classification is addressed under “E2E
policy and real-interface assertion”.

### 7. Exact files `verify.e2e_command` invokes, and parallel-implementation search

`./run-tests.sh --e2e` invokes:

- `./run-tests.sh` itself (`--e2e` → `--tags "@e2e"`);
- `scripts/assemble-features.sh` (feature-root assembly);
- `cargo test -p duty2cal --test features_runner -- --tags "@e2e"`, i.e.
  `apps/cli/tests/features_runner.rs`;
- for the capability,
  `apps/cli/tests/steps/e2e/enable_repository_security.rs` (Given/Then,
  transcript) and `apps/cli/tests/steps/enable_repository_security.rs` (shared
  `preview`, `apply_previewed`, `ensure_base_state`, `tofu`, `jq`, live-read
  and state helpers).

The tree was searched for a second/parallel implementation of the same
capability (`grep -rln "Organization owner previews the security settings|
enable_repository_security"`, excluding `.git` and `target`): only the feature,
the two step files, the two `mod.rs` files, the design/review/tasks documents
and the arc42 building-block view match. `git status --porcelain` is clean;
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
scope is strictly smaller. The predecessor receipt records 132 full (including
4 e2e) and 4 e2e; the delta’s 8 scenarios (6 non-e2e, 2 e2e) give 140 and 6.
The archive will record the merged per-scope receipt; review did not re-run the
suite.

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
| CI runs the setup script before the suite | `.github/workflows/ci.yml` line 70 | yes |
| `DutyWorld` gains `plan_path`, `preview_transcript`, `apply_transcript` | `features_runner.rs`; plus `write_sandbox` | yes |
| Visual transcripts per `@e2e` scenario | two committed transcripts under `evidence/visual/` | yes |
| Real-apply transcript after GREEN | present (F4 fixed) | yes |
| Post-GREEN targeted security scope and full-preview drift | `design.md` steps 1, 3, 4, 5 and the correction note | yes |
| Delta use case parses for capability routing | `givn check review` passes routing | yes |
| No implementation-layer detail in the spec | no journal/PATCH/write in the feature (F6 fixed) | yes |
| **The corrected design was re-reviewed** | `design-review.md` was not updated after the `design.md` change | **no (F8)** |

The live operator note and the targeted-scope correction were added in
`5d0710a`/`0730cc5`, after the last `design-review.md` `DESIGN-REVIEW: PASS`
(its last modification is `3165c04`). The commands, file layout, framework and
driver match the corrected design; the reviewed-design artifact is stale (F8).

### 11. Interaction coverage cross-reference

| Inventory entry (`specs/apply-configuration-change/usecase.md`) | Design `Interaction Coverage Matrix` row | Existing `@e2e` scenario | Driving mechanism | Match? |
|---|---|---|---|---|
| preview the declared configuration and read the named differences | preview the declared configuration and read the named differences | “Organization owner previews the security settings and only the declared differences are named” | `std::process::Command` runs `tofu init`, `tofu plan -out`, `tofu show -json` against the fake; asserts the plan text and JSON | yes |
| apply the previewed revision and read the outcome | apply the previewed revision and read the outcome | “Organization owner applies the previewed revision and the security settings are enabled” | `std::process::Command` runs the preflight, `tofu apply <plan>`, then a second `tofu plan`; asserts the transcript, journal, live read and state | yes |

Every inventory entry has a matrix row, an existing `@e2e` title and the
promised CLI driver. No missing or extra matrix row was found.

### 12. Fabrication-audit conclusion

The change is real, not a stub: the declaration, fake, setup proof, runner and
step definitions exist, and the recorded suite is green. Steps 0–9 and 11 are
clean. The audit records one unfinished item: **F8**, the missing design-review
re-run for the corrected design (step 10 / arc42 step 5). No `@e2e` tag was
removed and no empty step body was found.

Contract-statement-to-scenario mapping (F3):

| Use-case statement | Scenario(s) |
|---|---|
| Main flow 1 — record the revision and keep it as the candidate | the declaration/previewed-revision Givens of all 8 scenarios |
| Main flow 2 — preview names every difference; unreadable or unadopted object refused and named | preview `@e2e`; unreadable setting; unadopted Dependabot setting |
| Main flow 3 — apply checks the saved plan and the preflight, then applies | apply `@e2e`; live-edit refusal |
| Main flow 4 — report changed objects and record the applied revision | apply `@e2e` |
| Main flow 5 — the live configuration matches the revision | apply `@e2e` |
| Extension 2 — unreadable live configuration | unreadable setting |
| Extension 2 — unadopted live object | unadopted Dependabot setting |
| Extension 2 — stale applied state | read-back failure; partial failure |
| Extension 3 — live edit since the preview | live edit |
| Extension 3 — authority does not cover the first object | first change refused |
| Extension 3 — read-back failure after a write | read-back failure |
| Extension 3 — later object fails after an earlier write | partial failure |
| Rules — applied revision, readability, adoption, preflight, refusal/partial, recorded revision/stale, unreadable authority, declared values | apply `@e2e`, unreadable setting, unadopted setting, live edit, first change refused, read-back/partial, preview `@e2e` |
| Minimal guarantee | first change refused; partial failure; the stale-preview Thens |
| Success guarantee | apply `@e2e` |
| Out of scope — declaration edit after preview, invalid revision, incomplete preflight, unreadable applied state | no scenario owed; the use case declares them out of scope |

No remaining contract statement lacks a scenario.

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
| 5 — public-home declaration, apply block and fake write sandbox | `docs/arc42/05-building-block-view.md` lines 70, 92–96 | affected: building blocks | Architecture impact, normal mechanism, step definitions | setup, all | `infra/`, scripts, fake, step files | yes (the block describes the normal mechanism; the live targeted recovery is recorded in 06/design) |
| 6 — security-change runtime: adoption, preview, guard, preflight, targeted apply, partial, read-back, full-preview drift | `docs/arc42/06-runtime-view.md` lines 204–262 | affected: targeted security scope and the full-preview drift | Apply procedure, Post-GREEN, live note | 1–8, operator apply | `apply_previewed`, fake, transcripts, real-apply | yes (F5 content fixed) |
| 7 — deployment: workstation, environment-only secrets, ciphertext state, no CI apply | `docs/arc42/07-deployment-view.md` line 99 | affected: deployment | Applied state, live note | 5, 6, operator apply | state serial 6→9, command, CI | yes |
| 8 — declared-vs-applied state, saved plan, preflight, stale marker | `docs/arc42/08-crosscutting-concepts.md` lines 100–145 | affected: crosscutting | Decisions, apply procedure | 3–6 | step code, fake journal, state | yes |
| 9 — no new ADR | `docs/arc42/09-architecture-decisions.md` | affected: no new ADR | Four NOT_QUALIFIED verdicts | change docs | ADR-0005/0006 unchanged | yes |
| 10 — QS-019/QS-020 | `docs/arc42/10-quality-requirements.md` line 59 | affected: targeted security scope and full-preview drift | Traceability, coverage | 1–8 | e2e, refusals, state, real-apply | yes (F5 content fixed) |
| 11 — R-026…R-031, TD-013 | `docs/arc42/11-risks-and-technical-debt.md` lines 32–37, 55 | affected: risks | Risks, live note | 1–8, operator apply | declaration, fake, state | yes |
| 12 — **Security settings**, refined **Configuration change** | `docs/arc42/12-glossary.md` rows Security settings / Configuration change | affected: glossary | Decisions, file layout | all | `infra/security.tf`, `security_and_analysis`, state/stale marker | yes |

All content rows match; the affected chapters contain decision-specific text,
and chapters 6 and 10 now record the targeted security scope and the
pre-existing organization billing drift without claiming the full configuration
is clean. The arc42 block is nevertheless **not clean for sign-off**: step 5
requires a design-review re-run when `design.md` changes, and that re-run is
missing (F8).

ARC42 CONFORMANCE: NOT CLEAN

Content rows match; the design-review re-run required after the `design.md` change is outstanding (F8).

## Ubiquitous language conformance

The durable glossary’s confirmed terms are reused with their exact senses:
**Security settings** (secret scanning, push protection, Dependabot alerts and
Dependabot security updates), **Configuration change** (the saved plan,
refresh-only preflight, apply, state record, stale marker and partial
reporting), **Declared configuration**, **Adoption**, **Configuration drift**,
**Public home** and **Protection ruleset**. The spec, design and implementation
use “security settings”, “declared configuration”, “applied state”, “preview”,
“apply”, “saved plan” and “stale applied state” consistently with the glossary;
“applied state”, “saved plan”, “preview” and “stale marker” are the
**Configuration change** row’s vocabulary, not new domain terms. The design’s
“settings toggles” is descriptive of the two Dependabot resources, not a new
domain term; the glossary’s **Security settings** row already names them. No
conflicting sense or silently promoted term was found.

UBIQUITOUS LANGUAGE: CONFORMANT

Non-blocking observation: the use case’s four new `## Out of scope` bullets
phrase their boundary as “in this delta” / “not a use-case promise in this
delta”. They are change-process wording, not a domain term; when the use case
is merged into `givn/specs/apply-configuration-change/`, reword them to the
capability boundary (for example, “A declaration edited after the saved preview
is not applied; the saved plan is the applied revision.”). This does not change
the domain vocabulary or the coverage classification.

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
   gap remains. The four behaviours the previous review classified as bucket 2
   (declaration edited after the preview, invalid revision, incomplete
   preflight, unreadable applied state) are now outside the capability’s
   contract by the use case’s explicit `## Out of scope` boundary; they are not
   a coverage gap and no scenario is owed. Every remaining contract statement
   maps to one of the eight scenarios (mapping above).
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
- **Scope:** two normalized actions, two `@e2e` scenarios; the six variants are
  regular scenarios.
- **Canonical one-per-action:** no capability has more than one `@e2e` for the
  same action; no inventory action is missing.

## Use-case and Persona conformance

The owning use case is
`givn/changes/enable-repository-security/specs/apply-configuration-change/usecase.md`;
the ideation contract is
`givn/ideation/public-home-infrastructure/use-cases/apply-configuration-change.md`.
The implementation preserves the goal, the one success outcome, the minimal
guarantee, the rules, capability ownership and the confirmed Persona-relevant
outcomes. The implementation drives the real declaration and state through the
real `tofu` binary. The live operator deviation is assessed separately below.

Use-case scope rejection criteria checked:

| Criterion | Result |
|---|---|
| Goal conjunction | Not triggered: one goal phrase (“Make the live configuration match the declared configuration revision the apply accepts”) |
| Success guarantee names two outcomes | Not triggered: one outcome (F7 fixed) |
| Missing primary actor | Not triggered: Organization owner is primary and the trigger names the owner |
| Placement failure | Not triggered: `enable-repository-security` advances the organization owner’s administration goal |
| Mutation under an observing goal | Not triggered: the use case explicitly declares itself changing |
| Extension without a step reference | Not triggered: every extension is anchored to step 2 or 3; the declaration-edit extension was removed to `## Out of scope` |
| Inconsistent preconditions | Not triggered: the stale-state precondition is a state, the trigger is an action, the success guarantee is not assumed, and it is qualified `(open: Q11)` |
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
migration; no migration-ledger evidence applies. The live operator run’s
direct-import/targeted-apply deviation is recorded and consistent with the
use case’s success guarantee for the retained objects; it is assessed below.

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
  statuses, the base apply’s follow-up `No changes.`, and `exit status: 0`.
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
  parent lens is unaffected and is not a Persona confirmed for this change. No
  finding.

Every finding would have to name a contract rule or a failed Persona goal; none
was found. No pixel-level or coordinate finding applies.

VISUAL-REVIEW: PASS

## Live operator deviation assessment

The live run is assessed against the seed and the change’s own contract:

- **Seed guarantee:** the post-apply authenticated read reports secret scanning
  and push protection `enabled`, and Dependabot security updates `enabled`; both
  Dependabot resources were imported and bound (`github_repository_vulnerability_alerts.mono`
  id `1416075332`, `github_repository_dependabot_security_updates.mono` id
  `duty2cal`). The encrypted state serial moved from 6 to 9 (two direct imports
  plus one targeted apply), and the state and backup are in `877753f`. The
  seed’s guarantee is met as recorded.
- **Direct imports and targeted apply:** the second adoption command refused
  the batch because its whole-state refresh saw the pre-existing organization
  billing drift. Both Dependabot resources were imported directly by their
  repository-name ids, and the security plan was applied with
  `-target=github_repository.mono`. The targeted plan named only
  `github_repository.mono` (the two statuses enabled), applied
  `0 added, 1 changed, 0 destroyed`, and the targeted follow-up exited 0 with
  `No changes.` The direct imports satisfy adopt-before-create; the target keeps
  the unrelated drift out of the apply.
- **Adoption whole-state defect:** the design’s “Live operator run” and
  `tasks.md`’s final operator box record the defect for a follow-up: the
  adoption verifier must refresh only the batch addresses so a pre-existing
  drift elsewhere does not refuse an unrelated adoption. It is not silently
  accepted.
- **Full-preview drift:** the full preview still names the pre-existing
  organization billing drift; it is outside this change and was not applied by
  the targeted run. `design.md` Post-GREEN steps 1, 3, 4 and 5, `docs/arc42/06`
  and QS-019/QS-020 record the targeted scope and the drift, and no claim
  remains that the full configuration is clean.
- **What is not reconciled:** the corrected design has not been re-reviewed by
  `design-review` (F8). The live deviation itself is recorded and justified;
  the design-review artifact is stale.

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

The change adds maintainer infrastructure (`infra/security.tf`,
`infra/adoption-batch-security.txt`, the fake/setup/seam). It does not change a
product command, flag, configuration file, output or any section of the README
(Description, Quick start, Prerequisites, Usage, Configuration, Contributing,
License). The declaration and the operator routine are documented in
`design.md` and `tasks.md`, not in the public README. `README.md` contains no
open placeholder.

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

Exit 0. `givn status --change enable-repository-security` reports
`ready-to-archive`, 44/44 tasks, all artifacts complete. The mechanical gate
does not verify design-review freshness; that is the review-level F8.

## Sign-off checklist

| Check | Result |
|---|---|
| Fabrication audit clean (or all findings remediated) | **No** — F8 (design-review re-run missing); no empty steps, no removed `@e2e`, no fabricated task |
| Every checked task has a verified commit touching production code | Yes — the eight revisions touch the declaration, fake, setup, step definitions or runner |
| Every promised component exists | Yes — including the real-apply transcript and the post-apply reads |
| Strict-mode proof present and passing | Yes — `fail_on_skipped()`; tasks record the undefined-step proofs; no `@wip` |
| Scenario execution evidence recorded per scope | Yes — 140/140 and 584 steps; 6/6 e2e and 42 steps; review did not re-run |
| Coverage classification uses the most recent measured evidence | Yes — last archive receipt 98% (871/886); no current receipt; static inspection used |
| Every coverage gap classified and resolved | Yes — the four narrowed behaviours are out of scope; no remaining gap |
| Dead code deleted; missing tests added; hard-to-test gaps justified | Yes — no product dead code; no missing scenario; no hard-to-test gap |
| Redundant unit tests removed | Yes — no unit tests added; `.feature` is the source |
| No `@wip` tags; no implementation-layer detail in the spec | Yes — F6 fixed; the feature has no journal/PATCH/write wording |
| Canonical E2E policy read; one normalized action per `@e2e`; variants regular | Yes — two inventory actions, two `@e2e`; six variants regular |
| Every E2E scenario has a real-interface primary assertion and the reviewed driver; no HTTP/fetch substitution | Yes — CLI output first; `std::process::Command`; terminal, not a browser |
| Local run command starts the full stack (including digital twins) cleanly | Recorded, not re-run per the assignment: the scenarios start the fake per scenario; `scripts/setup-infra-tests.sh` prepares the mirror and is in CI |
| Exact files `verify.e2e_command` invokes identified and read; parallel implementation searched | Yes — `run-tests.sh`, `assemble-features.sh`, `features_runner.rs`, the two step files; tree search and `git status` found no parallel implementation |
| `verify.e2e_command` distinct; counts prove a strictly smaller e2e scope | Yes — `./run-tests.sh --e2e` vs `./run-tests.sh`; 6 < 140; predecessor receipt 4 < 132 |
| Implementation matches design’s commands, files, framework/driver | **No** — F8 (the corrected design was not re-reviewed); the commands/files/framework themselves match |
| Interaction coverage verified | Yes — two inventory entries → two matrix rows → two existing `@e2e` → promised CLI driver |
| No finding excused outside the three coverage buckets | Yes — the out-of-scope behaviours are a use-case boundary, not a coverage exception; no “acceptable for now”/“future scope” wording |
| Arc42 conformance | **No** — content rows match, but the design-review re-run required after the `design.md` change is missing (F8) |
| `givn check review` gate | Passes mechanically — integrity, run declaration, routing and overlap |

## Findings and required fixes

### F8 — Re-review the corrected design before archive (blocker)

`design.md` changed after the last `DESIGN-REVIEW: PASS`: `5d0710a` added the
live operator run and `0730cc5` changed the Post-GREEN operator apply to the
targeted security scope and the direct-import recovery (`design.md` lines
897–935 and the “Live operator run” correction note). `design-review.md` was
last modified in `3165c04`; neither commit updated it, and its last
`DESIGN-REVIEW: PASS` does not cover the targeted-scope procedure. The review
instruction’s fabrication-audit step 10 and arc42 step 5 require a design-review
re-run when `design.md` changes; the previous review’s F5 also explicitly
required it. The content of the correction is right, but the reviewed-design
artifact is stale.

Required fix: run a fresh-context `design-review` on the corrected design — the
targeted Post-GREEN steps 1, 3, 4 and 5, the direct-import recovery, and the
arc42 chapter 6/QS-019/QS-020 qualification — then append the question log and
`DESIGN-REVIEW: PASS` to
`givn/changes/enable-repository-security/design-review.md` and commit. Re-run
`givn check review --change enable-repository-security` and this review
afterwards.

### F9 — Normalize the out-of-scope wording at merge (minor)

The use case’s four `## Out of scope` bullets use “in this delta” and “not a
use-case promise in this delta”. At archive the delta use case becomes
`givn/specs/apply-configuration-change/usecase.md`; those phrases would be
change-process language in a durable document. Reword them to the capability
boundary, preserving the scope decision (no scenario is owed). This is an
editorial fix, not a coverage gap; it can ride with F8.

## Verdict

F1–F7 are remediated: the commit lines and use-case parser are gate-clean, the
delta use case is narrowed to the eight scenarios, the real-apply evidence and
post-apply reads are committed, the design/arc42 targeted-scope correction is
present, the feature is free of fake-journal/PATCH detail, and the success
guarantee has one outcome. The recorded suite is green (140/140, 584 steps;
6/6 e2e, 42 steps) and `givn check review` passes its mechanical checks.

The earned verdict is **FAIL** because the corrected design was never
re-reviewed: `design-review.md` is stale for the targeted-scope Post-GREEN
change (F8), so the arc42/fabrication sign-off cannot be completed. Fix F8
(with F9) and re-run this review.

REVIEW: FAIL
