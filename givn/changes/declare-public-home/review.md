# Review: declare-public-home

Change: `declare-public-home`
Stage: `ready-to-archive` (`givn status` reports 56/56 tasks)
Review date: 2026-10-17
Verdict: **PASS** (the two prior findings are resolved; no new blocking finding)

## Preconditions

- `givn status --change declare-public-home` reports 56/56 tasks complete and
  the change at `ready-to-archive`.
- This review was read-only except for rewriting this file. The suite was not
  re-run; the recorded counts in `tasks.md` are the execution evidence, as the
  assignment requires.
- `givn instructions review`, `givn instructions specs`, and
  `givn instructions dialog` were loaded before this report was written.
- The prior review returned FAIL on F1 (the fake's live values equaled the
  declared values, so the e2e did not exercise identity-not-copy) and F2
  (`design.md` listed `REUSE.toml` as changed while no commit touched it).
  Both are re-checked below and resolved.

## Commands and scope under review

Read literally from `givn/commands.yaml`:

- `verify.command: ./run-tests.sh`
- `verify.e2e_command: ./run-tests.sh --e2e`

The two values are different strings. `run-tests.sh` maps `--e2e` to
`--tags "@e2e"` and invokes
`cargo test -p duty2cal --test features_runner -- --tags "@e2e"` over the
feature root assembled by `scripts/assemble-features.sh`
(`scripts/assemble_features.py`, which copies `givn/specs/**` and
`givn/changes/*/specs/**`). `apps/cli/tests/features_runner.rs` calls
`Cucumber::fail_on_skipped()`.

Recorded execution evidence (from `tasks.md`; not re-run):

- Full suite: **132 scenarios (132 passed), 518 steps (518 passed)**, exit 0.
- E2E scope: **4 scenarios (4 passed), 23 steps (23 passed)**, exit 0.
- The e2e scope is strictly smaller than the full scope (4 < 132).
- Single `@e2e` scenario after the F1 strengthening: **1 scenario (1 passed),
  7 steps (7 passed)**.
- No `givn/archive/declare-public-home/verification.json` exists yet for this
  change; the archive gate writes the receipt. The per-scope counts above are
  the available execution evidence at review time.

## Fabrication audit (steps 0–12)

### 0. `@e2e` tag integrity — clean

`givn/changes/declare-public-home/specs/adopt-existing-configuration/adopt-existing-configuration.feature`
contains exactly one `@e2e` scenario, the success scenario, tagged
`@givn.added @e2e`. The file history shows the tag was present from the first
authored revision (`aa38647`) and was never removed; `a81fa70` only dropped
`@wip` when the steps were implemented. No planned `@e2e` coverage was removed
to bypass the gate.

### 1. Empty or trivial step bodies — clean

**0 empty step bodies found across the 9 step-definition files scanned**
(`apps/cli/tests/steps/*.rs` and `apps/cli/tests/steps/e2e/*.rs`). The scan for
`{}`, `() {}`, bare `pass`, `todo!`, `unimplemented!` and whitespace-only bodies
found no step body; the hits were `format!` templates and panic messages.

The two capability files have substantive bodies:

- `apps/cli/tests/steps/adopt_existing_configuration.rs`: `prepare` copies the
  declaration and fixture tree, starts the fake and captures its port; `adopt`
  runs `scripts/adopt-public-home.sh` with `std::process::Command` and captures
  stdout/stderr/exit; every Then asserts on command output, the fake journal,
  the applied-state bytes, or `tofu state list`/`tofu state show`.
- `apps/cli/tests/steps/e2e/adopt_existing_configuration.rs`: the Given starts
  the fake through `prepare` and checks the committed fixture set; the Then
  asserts the command's exact five `adopted` lines, the journal's served-path
  condition, the five `tofu state list` addresses, and (after the F1 fix) the
  live values in `tofu state show` for both repositories. It commits the
  transcript.

### 2. Checked task → commit → source cross-check

Every checked scenario task records a revision. The production command and
declaration are introduced in `2bd68e3`/`cd89ada`/`bc134b0`; the verification
hardening is `2e2992d`; the scenario revisions add the real step definitions
that exercise the already-committed command. The F1/F2 fixes are
`d33d77e` and `6998c62`; these touch the real test source and fixtures, not a
spec-only stub. Spot-checked with `git show --stat`:

| Task | Revision | Commit content verified |
|---|---|---|
| Setup | `2bd68e3` | `scripts/adopt-public-home.sh`, `infra/*`, `scripts/fake-github/serve.py`, `scripts/setup-infra-tests.sh`, runner/world, both step modules, CI, `.gitignore` |
| 1 invalid declaration | `2bd68e3` | production command + declaration + steps; scenario passes |
| 2 malformed batch | `4a654b3` | steps + feature; batch preflight is in the command from `2bd68e3` |
| 3 missing object | `cd89ada` | command refusal mapping, fixture tree, fake `/api/v3` normalization, setup proof, steps |
| 4 already bound | `7519c27` | priming adoption, state-list assertion, runner/world; import refusal already in the command |
| 5 unreadable object | `53f3267` | fault injection and refusal assertions; command 401/403 mapping present |
| 6 undeclared entry | `b558e4d` | undeclared batch and refusal assertions; `tofu import` refusal already present |
| 7 changed after read | `bc134b0` | deterministic mutation, refresh-plan drift parser, steps |
| 8 wrong passphrase | `bb895e0` | state snapshot and priming passphrase, steps; decrypt refusal already present |
| 9 state cannot be written | `64129e6` | read-only state directory, steps; write refusal already present |
| 10 selected batch only | `9964f24` | two-entry batch and state-list assertions; batch selection already present |
| 11 `@e2e` success | `a81fa70` | e2e steps, shared helper visibility, committed transcript |
| Post-review verification hardening | `2e2992d` | status 0/1/2 only, `change_summary` required |
| F1/F2 fix | `d33d77e`, `6998c62` | live fixture values, `applied_state_show`, the six-field e2e assertion, design row, re-review |

No checked task is backed only by a spec/stub diff.

### 3. Promised components — present

All components named by `design.md` exist: `infra/versions.tf`,
`infra/main.tf`, `infra/repos.tf`, `infra/rulesets.tf`, `infra/pages.tf`,
`infra/adoption-batch.txt`, the committed encrypted `infra/terraform.tfstate`
and `.backup`, `infra/.terraform.lock.hcl`, `infra/.opentofu-version`, the
`.gitignore` rules, `scripts/adopt-public-home.sh`,
`scripts/setup-infra-tests.sh`, `scripts/fake-github/serve.py` plus the five
`fixtures/adopt-all` responses, the two step files, both `mod.rs` files,
`features_runner.rs`, the CI test job, `REUSE.toml` (existing), the
live-read/real-adoption/visual evidence, the arc42 chapters and ADR-0005/
ADR-0006. `cliff.toml` is correctly not edited before archive; the
archive-time obligation is recorded in `design.md` and `tasks.md`.

### 4. Strict-mode proof — present

`apps/cli/tests/features_runner.rs` calls `Cucumber::fail_on_skipped()`.
`tasks.md` records two undefined-step proofs (the setup scenario and the e2e
setup) with non-zero exits and “Step doesn't match any function”. `@wip` is
absent from the delta specs. The fake's unknown-path 599 marker plus journal is
a second strictness net: the e2e asserts no journal entry has
`unknown_path: true`, and every refusal step uses the same assertion.

### 5. E2E Then-step real-interface assertions — present

The success scenario's Then asserts:

- the exact five `adopted <address> <import-id>` lines on the command's
  **stdout** — the CLI is the real interface (primary assertion);
- the fake journal has no unknown-path entry (secondary);
- the five addresses in the applied state read with `tofu state list`
  (secondary);
- the applied state carries the live values for the six fields where the
  declaration deliberately differs, read with the real `tofu state show`:
  monorepo `has_projects`, `allow_merge_commit`, `allow_rebase_merge`,
  `delete_branch_on_merge`; pages repository `has_issues`, `has_wiki`
  (secondary evidence for identity-not-copy);
- `the live configuration is unchanged` calls the shared `assert_no_live_write`,
  which rejects a refused write and a GraphQL mutation (secondary).

It is not a repository-only assertion. After the F1 fix the fake's live
fixtures carry the values recorded in `evidence/live-read/repo-mono.json` and
`repo-pages.json`, so the scenario now exercises the declared-versus-live
divergence instead of only asserting the five addresses.

### 6. Driver fidelity — valid for a CLI

This capability is a terminal/CLI interface, not a browser UI. The e2e uses
`std::process::Command` in the shared `adopt` helper to run
`scripts/adopt-public-home.sh` as a real subprocess, with a real `tofu` binary
and the local fake GitHub API behind the provider. It does not send HTTP from
page JavaScript, does not call `fetch()`/`XMLHttpRequest`, and does not call an
in-process adoption routine. A browser driver is not applicable; the opposite
classification is addressed in “E2E policy” below.

### 7. Exact files `verify.e2e_command` invokes, and parallel-implementation search

`./run-tests.sh --e2e` invokes:

- `./run-tests.sh` itself (`--e2e` → `--tags @e2e`);
- `scripts/assemble-features.sh` → `scripts/assemble_features.py` (feature
  root);
- `cargo test -p duty2cal --test features_runner -- --tags @e2e`, i.e.
  `apps/cli/tests/features_runner.rs`;
- for this scenario,
  `apps/cli/tests/steps/e2e/adopt_existing_configuration.rs` (Given/Then,
  transcript) and `apps/cli/tests/steps/adopt_existing_configuration.rs`
  (the When step, `prepare`, `adopt`, `applied_state_show`, journal and
  state-list helpers).

The tree was searched for a second/parallel adopter:
`grep -rln 'adopt-public-home\|adopt_existing'` returns only those two step
files, the two `mod.rs` files, the command script, and documentation.
`git status --porcelain` is clean; the ignored directories are the OpenTofu
cache, `__pycache__` and agent/tool directories, with no second
implementation. No weaker parallel implementation was found.

### 8. E2E scope and normalized inventory

The owning use case has exactly one normalized interaction:
“adopt the selected batch of existing objects”. The feature has exactly one
`@e2e` scenario for it. The other ten scenarios are variants/error cases of the
same action (invalid declaration, malformed batch, missing object, already
bound, unreadable object, undeclared entry, changed object, wrong passphrase,
state-write failure, narrower batch). There is no second `@e2e` for the same
action. This is the canonical scope.

### 9. E2E command isolation and counts

`verify.e2e_command` is not a copy of `verify.command`; it is the same runner
with the `@e2e` filter. The recorded counts are 132 full and 4 e2e; the e2e
scope is strictly smaller. No archive receipt exists before archive; the
archive gate will record the per-scope receipt. The `tasks.md` counts are the
available evidence, and the archive will measure the final merged state.

### 10. Implementation vs `design.md` — commands, files, framework and data match

| Design item | As built | Match? |
|---|---|---|
| `verify.command` = `./run-tests.sh` | `givn/commands.yaml` and `run-tests.sh` | yes |
| `verify.e2e_command` = `./run-tests.sh --e2e` | `givn/commands.yaml` and `run-tests.sh`; `--tags @e2e` | yes |
| Non-e2e step file `apps/cli/tests/steps/adopt_existing_configuration.rs` | exists, real subprocess helpers and assertions | yes |
| E2E step file `apps/cli/tests/steps/e2e/adopt_existing_configuration.rs` | exists, separate; transcript committed | yes |
| Framework: cucumber-rs with `std::process::Command` | `features_runner.rs` and step files | yes |
| `infra/` files, batch, encrypted state, lock, `.opentofu-version` | all present; state and backup are ciphertext (`encrypted_data`, passphrase metadata) | yes |
| Fake API, `/api/v3` normalization, journal, faults, mutations, unknown-path 599 | `scripts/fake-github/serve.py` implements them | yes |
| Setup script: pin, mirror, terraformrc, fixture proof | `scripts/setup-infra-tests.sh` implements them; recorded proof has 5 imports and a no-op refresh | yes |
| Exit-2 verification hardening | `adopt-public-home.sh` parses `change_summary`/`resource_drift`; `2e2992d` demonstrates the fix | yes |
| Fake live values represent the recorded hand-made live configuration | mono: `has_projects=true`, `allow_merge_commit=true`, `allow_rebase_merge=true`, `delete_branch_on_merge=false`; pages: `has_issues=true`, `has_wiki=true` — matching `evidence/live-read/` | **yes (was F1)** |
| `REUSE.toml` | design row now says `unchanged`; the existing aggregate annotation covers the fixtures and no commit touches the file | **yes (was F2)** |

The F1/F2 deviations were resolved by updating the fixtures, the e2e assertion
and the design row, and `design-review.md` records a fresh re-review with
`DESIGN-REVIEW: PASS`.

### 11. Interaction coverage cross-reference

| Inventory entry (`specs/adopt-existing-configuration/usecase.md`) | Design `Interaction Coverage Matrix` row | `@e2e` scenario in the feature | Driving mechanism in the step definitions | Match? |
|---|---|---|---|---|
| adopt the selected batch of existing objects | adopt the selected batch of existing objects / “Maintainer adopts the existing public home and every object is bound without changing it” | “Maintainer adopts the existing public home and every object is bound without changing it” | `std::process::Command` runs `scripts/adopt-public-home.sh` with `--declaration`, `--batch`, `--state`, `--api-url`, `--token`, `TF_VAR_state_passphrase`; shared `prepare` starts the fake; the e2e asserts stdout, journal, state list and state show | yes |

Every inventory entry has a matrix row, an existing `@e2e` title, and the
promised CLI driver. No missing or extra matrix row was found.

### 12. Fabrication-audit conclusion

The change is real, not a stub: the command, declaration, fake, setup proof,
runner and step definitions exist and the recorded suite is green. The two
prior audit findings (F1, F2) are remediated. No new fabrication finding
remains.

## Arc42 implementation conformance

The durable `docs/arc42/` chapters and the change-level `arc42.md` were read.
The affected chapters were independently derived from the completed
implementation, `design.md` and `tasks.md` before accepting the change-level
claims. All twelve chapters marked affected by `arc42.md` contain current,
decision-specific content:

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 1 — reproducibility goal and Persona roster | `docs/arc42/01-introduction-and-goals.md` lines 27, 37–38 | affected: reproducible public home; owner/reviewer | Scope, Persona impact | Setup, scenarios 1–11 | `infra/`, adoption command, transcript | yes |
| 2 — OpenTofu/provider floor and pin, encrypted committed state, environment-only passphrase/token, no apply in Actions, no lock, read-only adoption, offline fake | `docs/arc42/02-architecture-constraints.md` lines 28–34 | affected: constraints | Technology decisions, encryption block, test seam | Setup, 7–11 | `infra/versions.tf`, `.opentofu-version`, `.terraform.lock.hcl`, CI, `adopt-public-home.sh`, `serve.py` | yes |
| 3 — declared configuration/adoption as project-side interface and partner table | `docs/arc42/03-context-and-scope.md` lines 33–78 | affected: context | Architecture impact, test seam | all scenarios | `infra/`, command, fake | yes |
| 4 — infrastructure-as-code strategy and technology table | `docs/arc42/04-solution-strategy.md` lines 10, 25–28, 39 | affected: strategy | Technology decisions | Setup, 1–11 | declaration and command | yes |
| 5 — public-home declaration toolbox blocks | `docs/arc42/05-building-block-view.md` lines 68–90 | affected: building blocks | Architecture impact, step definitions | Setup, all | `infra/`, scripts, fake, step files | yes |
| 6 — adoption runtime: preflight, staging/validate, decrypt, import, refresh, atomic bind, refusals | `docs/arc42/06-runtime-view.md` lines 174–205 | affected: runtime | The adoption command | 1–11 | `adopt-public-home.sh` | yes |
| 7 — deployment: workstation, environment-only secrets, ciphertext state, no CI apply/drift | `docs/arc42/07-deployment-view.md` lines 62–100 | affected: deployment | Applied state, local runnability | 8, 9, real adoption | state files, command, CI | yes |
| 8 — declared-vs-applied state, all-or-none, encryption, read-only adoption | `docs/arc42/08-crosscutting-concepts.md` lines 65–110 | affected: crosscutting | Decisions, test seam | 3–10 | command, fake journal, state | yes |
| 9 — ADR-0005/0006 and indexes | `docs/arc42/09-architecture-decisions.md` lines 15–25; `docs/arc42/adr/README.md` | affected: ADRs | Decisions | Change docs | ADR files, indexes | yes |
| 10 — QS-016/017/018 | `docs/arc42/10-quality-requirements.md` lines 54–56 | affected: quality | Traceability, coverage | 11, setup, refusals | e2e, state ciphertext, refusals | yes |
| 11 — R-020…R-025 | `docs/arc42/11-risks-and-technical-debt.md` lines 26–31 | affected: risks | Risks | 8, 9, real adoption | declaration, command, state | yes |
| 12 — no new terms; Adoption/Declared configuration/Protection ruleset code symbols | `docs/arc42/12-glossary.md` lines 26, 39, 49, 138–153 | affected: glossary | Decisions, file layout | all | `scripts/adopt-public-home.sh`, `infra/*.tf`, `infra/rulesets.tf` | yes |

The five arc42 sign-off checks hold: the affected chapters were independently
derived; every affected chapter is current; each fact is cross-referenced to
design/tasks/implementation; no omission or stale contradiction was found; the
clean marker is recorded only after the table was completed.

**ARC42 CONFORMANCE: CLEAN**

## Ubiquitous language conformance

The durable glossary’s confirmed terms are reused with their exact senses:
**Adoption**, **Declared configuration**, **Protection ruleset**,
**Configuration change**, **Configuration drift**, **Deploy key**, **Public
home**, **Default branch**, **Gate (on landing)**, **Immutable release**.
The spec and design use “rule set” as the object the glossary names
**Protection ruleset**; the glossary’s anti-term (the legacy branch protection
as a separate object) is correctly excluded. The change does not rename or
silently extend a confirmed term.

The labels **applied state**, **selected batch** and **import identifier** are
the record/input vocabulary the topic’s event storming records as artifact
labels/mechanism for design, not new domain terms; the glossary’s promotion
boundary states that the declared configuration is a specification and the
applied state is a record. No conflicting sense was found. No
ubiquitous-language finding.

**UBIQUITOUS LANGUAGE: CONFORMANT**

## Coverage classification

Coverage is measured by the archive. The available XML files
(`coverage/non-e2e-cobertura.xml`, `coverage/e2e-cobertura.xml`,
`coverage/cobertura-coverage.xml`, timestamp 18:15) predate the final step
commit (`6998c62`, 23:10), and no archive receipt exists for this change yet;
the archive will measure the merged revision. Classification from inspection:

1. **Dead code (YAGNI/KISS) → delete.** No dead production code was found.
   The command’s helpers, refusal branches and fake endpoints used by the
   scenarios are reachable; the change adds no unused Rust product code. The
   existing release/e2e scenarios are untouched. No deletion required.
2. **Missing test coverage → add scenario.** The prior F1 gap is closed: the
   fake’s live fixtures now carry the six recorded live values, and the `@e2e`
   Then asserts through `tofu state show` that the applied state carries those
   live values while the declaration states the decided ones. No remaining
   missing-coverage gap was found.
3. **Legitimately hard to test (rare).** None. The shell/Python/HCL are
   outside the Rust coverage denominator by the design’s declared process
   boundary; their observable contract is exercised by the eleven scenarios,
   and the real adoption is the operator-run evidence.

No classification outside these three buckets is used. No gap is described as
“future scope”, “acceptable for now”, “sufficient for current scope”, or any
other invented exception.

## E2E policy and real-interface assertion

The canonical policy was read from
`givn instructions specs --change declare-public-home`.

- **Opposite classification:** a rendered page is a Web UI, and a Web UI would
  require a real browser driver such as Playwright; an HTTP client would not be
  a valid substitute. That classification **does not hold here**:
  `scripts/adopt-public-home.sh` is a terminal command; its interface is plain
  text on stdout/stderr. There is no HTML, CSS, JavaScript, DOM, page request,
  or browser-rendered state. The design correctly declares
  `Interface: terminal`.
- **Real interface:** the real subprocess invocation of the adoption command,
  with the real `tofu` binary and the local fake GitHub API behind the
  provider.
- **Primary assertion:** the command’s stdout — exactly five
  `adopted <address> <import-id>` lines. The journal, `tofu state list` and
  `tofu state show` are secondary evidence; the Then is not repository-only.
- **Driver:** `std::process::Command`, not an in-process call and not HTTP
  from page JavaScript.
- **Scope:** one normalized action, one `@e2e`; variants are regular
  scenarios.
- **Identity-not-copy:** the F1 strengthening makes the success path assert a
  real declared-versus-live difference; a regression that wrote declared
  values into the state would now fail the `tofu state show` assertions.

## Use-case and Persona conformance

The owning use case is
`givn/changes/declare-public-home/specs/adopt-existing-configuration/usecase.md`;
the ideation contract is
`givn/ideation/public-home-infrastructure/use-cases/adopt-existing-configuration.md`.
The implementation preserves the goal (“place the already-existing objects
of the public home under the declared configuration without changing them”),
the minimal/success guarantees, the rules (no live change, identity not a
copy, all-or-none selected batch, excluded deploy key/secret, one protection
object, encrypted local state), the extension step references, the capability
ownership, and the confirmed Persona references. The declaration revision is a
repository commit; the command reads and validates it, which is the use case’s
recorded division.

Use-case scope rejection criteria checked:

| Criterion | Result |
|---|---|
| Goal conjunction | Not triggered: one goal phrase, one success outcome |
| Missing primary actor | Not triggered: Organization owner is primary and the trigger names the owner |
| Placement failure | Not triggered: `adopt-existing-configuration` is goal-shaped and advances the owner’s administration goal |
| Mutation under an observing goal | Not triggered: the use case explicitly declares itself changing (declaration and applied state) while live is unchanged |
| Extension without a step reference | Not triggered: extensions reference steps 1, 2, 3 or 2–3; the malformed-batch and wrong-passphrase scenarios are input/precondition refusals under the rules, not new success outcomes |
| Inconsistent preconditions | Not triggered: “the declared configuration may not exist” and “applied state may or may not exist” are states, not flow-established preconditions; no open qualified guarantee |
| Mechanism vocabulary in goal/trigger/guarantees | Not triggered: “public home” and “declared configuration” are domain terms |
| Component- or channel-shaped name | Not triggered |
| Oversized leaf | Not triggered: the main flow is one actor goal with supporting steps |
| Relationship misuse | Not triggered: no Extends/Includes |

Confirmed Personas and dispositions:

| Persona | Evidence and disposition |
|---|---|
| `project-maintainer--organization-owner` | Runs `scripts/adopt-public-home.sh`; the committed transcript shows the five `adopted` lines and exit 0; can rebuild the state by re-adopting. Goal reached. |
| `contributor--infrastructure-reviewer` | Reads `infra/`, `infra/adoption-batch.txt` and the ciphertext state in the repository; holds no token or passphrase and cannot apply. The declaration and batch are text; the state is ciphertext. Goal reached as a review lens. |
| `crew-member--personal-calendar-owner` | The adoption makes no live write, so the release channel and the published site are untouched; the journal’s no-write assertion supports this. Goal unaffected. |

No invented or silently promoted Persona was found. The change is not a
migration; no migration-ledger evidence applies.

## Visual review

The design’s `## Visual Design Contract` declares:

- Interface: **terminal**
- Viewports: n/a (no rendered screen)
- Screens: none
- Evidence: one transcript per `@e2e` scenario under
  `givn/changes/declare-public-home/evidence/visual/<scenario-slug>/transcript.txt`
- Baseline: the transcript names each adopted address/import identifier and
  no live settings value, token, passphrase or temporary path.

Committed evidence reviewed:
`evidence/visual/maintainer-adopts-the-existing-public-home-and-every-object-is-bound-without-changing-it/transcript.txt`.
The review saw the final state for the one declared `@e2e` scenario (terminal
transcript; no screenshots or viewports). The transcript contains the command
line, the five bound lines (organization, monorepo, pages repository, rule
set, Pages source) and `exit status: 0`; it contains no token, passphrase,
temporary path or live settings value. The separate real-adoption transcript
under `evidence/real-adoption/transcript.txt` names the same five addresses.

Fresh-context Persona lenses against the committed transcript:

- **Organization owner:** sees every selected object named as adopted; can
  reach the adoption goal with the shown output. No finding.
- **Contributor reviewer:** can read the declaration, batch and transcript
  and confirm the state is ciphertext; holds no apply authority; can reach
  the review goal with what is shown. No finding.
- **Crew member:** the transcript shows no site or release operation and no
  live change; the release-channel goal is unaffected. No finding.

Every finding would have to name a contract rule or a failed Persona goal;
none was found. No pixel-level or coordinate finding applies.

**VISUAL-REVIEW: PASS**

## Overlap dispositions

| Finding pair | Advisory | Decision | Rationale |
|---|---|---|---|
| “An applied state encrypted with another passphrase refuses before any binding” and “A failure to record the applied state binds nothing and changes nothing” | `[SHAPE]` | variant | Different preconditions and causes: wrong-passphrase decryption vs a final state-write failure; the refusals name different state conditions and assert distinct invariants. The design review accepted the same distinction. |
| “A live object that changes after it was read refuses the batch” (subset of “A declared object that does not exist live refuses the whole batch”) | `[SUBST]` | variant | The live-change scenario exercises the refresh-only drift detector after import; the missing-object scenario exercises a 404 on import. Different detection step and distinct refusal; not a duplicate. |

No removed+added pair exists in the delta, so no supersession decision is
required. `givn lint --change declare-public-home` reports these two
advisories and otherwise “clean” (exit 0).

## Split-or-keep

No scenario exceeds the long-scenario gate threshold; `givn lint
--change declare-public-home` reported no `[LONG]` finding. The change’s
longest scenario is the eight-step missing-object scenario. No split-or-keep
decision is required. There are no removed+added pairs.

## README impact

`README-IMPACT: none`

The change does not touch the project’s public product face: the converter’s
Description, Quick start, Prerequisites, Usage, Configuration, Contributing
and License sections are unchanged; no product command, flag, configuration
file or output changed. The new `infra/` declaration and the adoption/setup
scripts are maintainer/test infrastructure, documented in `design.md`
(“Local runnability & digital twins”); the suite fails loudly with “run
./scripts/setup-infra-tests.sh first” if the local prerequisite is absent.
No README section was edited, and no placeholder is open.

## Prior findings and dispositions

### F1 — Align the fake live repository values with the recorded live configuration — RESOLVED

- **Prior file:** `scripts/fake-github/fixtures/adopt-all/repos/duty2cal/duty2cal.json`
  and `.../duty2cal.github.io.json`
- **Fix commits:** `d33d77e`, `6998c62`

The fixtures now mirror `evidence/live-read/repo-mono.json` and
`repo-pages.json` for all six fields the declaration deliberately differs on:

| Object | Field | Live read | Fixture | Declaration |
|---|---|---|---|---|
| monorepo | `has_projects` | true | true | false |
| monorepo | `allow_merge_commit` | true | true | false |
| monorepo | `allow_rebase_merge` | true | true | false |
| monorepo | `delete_branch_on_merge` | false | false | true |
| pages repository | `has_issues` | true | true | false |
| pages repository | `has_wiki` | true | true | false |

The `@e2e` feature now has the Then
“the applied state records the live values rather than the declared ones”.
The step first asserts the declaration states the decided values, then runs
the real `tofu state show` through `applied_state_show` against the
scenario’s applied state and asserts the live values for both repositories.
A regression that wrote declared values into the state (or reconciled live
to the declaration) would fail; the live no-write assertion remains the
secondary proof. The recorded post-fix runs are 7/7 steps for the single
scenario, 132/518 for the full suite, and 4/23 for the e2e scope. No residual
F1 gap remains.

### F2 — Correct the `REUSE.toml` row in the reviewed design — RESOLVED

- **Prior file:** `givn/changes/declare-public-home/design.md` (file table)
- **Fix commits:** `d33d77e`, `6998c62`

The row now reads `REUSE.toml | unchanged | The existing aggregate annotation
covers the fake's JSON fixtures; the scripts and HCL carry headers.` This
matches the implementation: `REUSE.toml` has an aggregate `path = ["**"]`
annotation and no commit in this change touches the file. Because the design
changed, `design-review.md` records a fresh identity-fix re-review ending
`DESIGN-REVIEW: PASS`; the re-review also records the six-field assertion and
the corrected 23-step evidence. No residual F2 finding remains.

No new finding was found in the fixes or elsewhere in the change.

## Sign-off checklist

- [x] Fabrication audit: clean (F1/F2 remediated; no new finding).
- [x] Every checked task has a verified commit touching production code or
      the real step implementation it names; no spec-only revision.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing (`fail_on_skipped`; two
      undefined-step proofs in `tasks.md`).
- [x] Scenario execution evidence is recorded per scope in `tasks.md`
      (132/518 full, 4/23 e2e); no archive receipt exists before archive, and
      the archive gate will carry the merged receipt.
- [x] Coverage classification uses the most recent available measured
      evidence (the coverage XML files predate the final step commit) and
      static inspection; the archive receipt will carry the merged result.
- [x] Every coverage gap classified and resolved; no dead production code;
      no hard-to-test excuse.
- [x] Dead code deleted (none found in production code); missing tests added
      (F1’s state assertion); hard-to-test gaps none.
- [x] Redundant unit tests removed / none added; the `.feature` file remains
      the source of truth.
- [x] No `@wip` tags remain. No implementation-layer detail in the spec.
- [x] Canonical E2E policy was read; every normalized inventory action has
      one `@e2e` scenario and variants are regular scenarios.
- [x] Every E2E scenario has a real-interface primary assertion and uses the
      reviewed CLI driver; browser/HTTP-page substitution is not present.
- [x] The local run command and its digital twin are wired as recorded
      (`scripts/setup-infra-tests.sh`, the per-scenario fake, `./run-tests.sh`);
      the setup and suite runs are recorded in `tasks.md`. The suite was not
      re-run, per the assignment’s no-rerun rule.
- [x] The exact file(s) `verify.e2e_command` invokes were identified and
      read; the tree (including ignored/untracked areas) was searched for a
      second/parallel e2e implementation; none exists.
- [x] `verify.e2e_command` is not identical to `verify.command`; the recorded
      counts prove the e2e scope is strictly smaller (4 < 132).
- [x] Implementation (commands, file layout, framework/driver, fixture
      values) matches what `design.md` names; the prior F1/F2 deviations were
      resolved by updating the implementation/design and re-running
      design-review.
- [x] Interaction coverage verified: every use-case inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatch.
- [x] No finding was excused with a classification outside the three buckets;
      no “acceptable for now”, “future scope”, “sufficient for current scope”,
      or similar invented exception appears in this report.

REVIEW: PASS
