# Review: declare-public-home

Change: `declare-public-home`
Stage: `tasks-done` (`givn status` reports 56/56 tasks)
Review date: 2026-10-17
Verdict: **FAIL** (two findings; see “Findings” and “Required fixes”)

## Preconditions

- `givn status --change declare-public-home` reports 56/56 tasks complete.
- The change is at `tasks-done`; the review artifact is the only missing gate.
- The review was read-only except for writing this file. The suite was not
  re-run; the recorded counts in `tasks.md` are used as execution evidence.
- `givn instructions review`, `givn instructions specs`, and
  `givn instructions dialog` were loaded before this report was written.

## Commands and scope under review

Read literally from `givn/commands.yaml`:

- `verify.command: ./run-tests.sh`
- `verify.e2e_command: ./run-tests.sh --e2e`

The two values are different. `run-tests.sh` maps `--e2e` to
`--tags "@e2e"` and invokes
`cargo test -p duty2cal --test features_runner -- --tags "@e2e"` over the
feature root assembled by `scripts/assemble-features.sh`
(`scripts/assemble_features.py`, which copies `givn/specs/**` and
`givn/changes/*/specs/**`). `features_runner.rs` adds
`Cucumber::fail_on_skipped()`.

Recorded execution evidence (from `tasks.md`; not re-run):

- Full suite: **132 scenarios (132 passed), 517 steps (517 passed)**, exit 0.
- E2E scope: **4 scenarios (4 passed), 22 steps (22 passed)**, exit 0.
- The e2e scope is strictly smaller than the regular scope (4 < 132). No
  archive verification receipt exists yet for this change; the archive gate
  will write it. The per-scope counts above are the available execution
  evidence at review time.

## Fabrication audit (steps 0–12)

### 0. `@e2e` tag integrity — clean

`grep -rn '@e2e' givn/changes/declare-public-home/specs/` finds exactly one
scenario, the success scenario, tagged `@givn.added @e2e`. The file history
(`git log -p -- .../adopt-existing-configuration.feature`) shows the tag was
present from the first authored revision (`aa38647`) and was never removed;
the only tag change was dropping `@wip` when the scenario was implemented.
No planned `@e2e` coverage was removed.

### 1. Empty or trivial step bodies — clean

- `0 empty step bodies found across the 9 step-definition files scanned`
  (`apps/cli/tests/steps/*.rs`, `apps/cli/tests/steps/e2e/*.rs`). The regex
  scan for `{}`, `() {}`, bare `pass`, `todo!`, `unimplemented!` produced no
  step-body match; the hits were `format!("{}")` string templates in
  `support.rs` and existing release/e2e scenarios, not bodies.
- The two capability files have substantive bodies:
  - `apps/cli/tests/steps/adopt_existing_configuration.rs`: `prepare` copies
    the declaration and fixture tree, starts the fake and captures its port;
    `adopt` runs `scripts/adopt-public-home.sh` with `std::process::Command`
    and captures stdout/stderr/exit; every Then asserts on the command
    output, the fake journal, the applied-state bytes, or `tofu state list`.
  - `apps/cli/tests/steps/e2e/adopt_existing_configuration.rs`: the Given
    starts the fake through `prepare` and checks the committed fixture set;
    the Then asserts the command’s exact five `adopted` lines, the journal’s
    served-path condition, and the five `tofu state list` addresses, then
    commits the transcript.

### 2. Checked task → commit → source cross-check

Every checked scenario task has a recorded revision. The production command
and declaration are introduced in `2bd68e3`, `cd89ada`, `bc134b0` and the
post-review hardening `2e2992d`; the other scenario revisions add the real
step definitions that exercise already-committed command behavior. No
revision is a spec-only diff.

| Task | Revision | Commit content verified |
|---|---|---|
| Setup | `2bd68e3` (same revision as scenario 1) | `scripts/adopt-public-home.sh`, `infra/*`, `scripts/fake-github/serve.py`, `scripts/setup-infra-tests.sh`, runner/world, both step modules, CI, `.gitignore` |
| 1 invalid declaration | `2bd68e3` | production command + declaration + steps; scenario passes |
| 2 malformed batch | `4a654b3` | step definitions + feature; the batch preflight is in the command committed at `2bd68e3`; no spec-only revision |
| 3 missing object | `cd89ada` | command refusal mapping, fixture tree, fake `/api/v3` normalization, setup proof, steps |
| 4 already bound | `7519c27` | priming adoption, state-list assertion, runner/world; import refusal already in the command |
| 5 unreadable object | `53f3267` | fault injection and refusal assertions; command 401/403 mapping already present |
| 6 undeclared entry | `b558e4d` | undeclared batch and refusal assertions; `tofu import` refusal already present |
| 7 changed after read | `bc134b0` | deterministic mutation, refresh-plan drift parser, steps |
| 8 wrong passphrase | `bb895e0` | state snapshot and priming passphrase, steps; decrypt refusal already present |
| 9 state cannot be written | `64129e6` | read-only state directory, steps; write refusal already present |
| 10 selected batch only | `9964f24` | two-entry batch and state-list assertions; batch selection already present |
| 11 `@e2e` success | `a81fa70` | e2e steps, shared helper visibility, committed transcript |
| Post-review fix | `2e2992d` | verification hardening: status 0/1/2 only, `change_summary` required |

### 3. Promised components — present

All components named by `design.md` exist: `infra/versions.tf`,
`infra/main.tf`, `infra/repos.tf`, `infra/rulesets.tf`, `infra/pages.tf`,
`infra/adoption-batch.txt`, `infra/terraform.tfstate` and `.backup`,
`infra/.terraform.lock.hcl`, `infra/.opentofu-version`, the `.gitignore`
rules, `scripts/adopt-public-home.sh`, `scripts/setup-infra-tests.sh`,
`scripts/fake-github/serve.py` plus the five `fixtures/adopt-all` responses,
the two step files, both `mod.rs` files, `features_runner.rs`, the CI test
job, `REUSE.toml` (existing), the live-read/real-adoption/visual evidence,
the arc42 chapters and ADR-0005/ADR-0006. `cliff.toml` is correctly not
edited before the archive.

### 4. Strict-mode proof — present

`apps/cli/tests/features_runner.rs` calls `Cucumber::fail_on_skipped()`.
`tasks.md` records two undefined-step proofs (setup and E2E setup) with
non-zero exits and “Step doesn’t match any function”. `@wip` is absent from
the delta specs. The fake’s unknown-path 599 marker plus journal is a second
strictness net: the e2e asserts no journal entry has `unknown_path: true`,
and refusal steps use the same assertion.

### 5. E2E Then-step real-interface assertions — present, with one coverage gap

The success scenario’s Then asserts:

- the exact five `adopted <address> <import-id>` lines on the command’s
  **stdout** (the CLI is the real interface);
- the fake journal has no unknown-path entry (secondary);
- the five addresses in the applied state read with `tofu state list`
  (secondary);
- `the live configuration is unchanged` calls the shared
  `assert_no_live_write`, which rejects a refused write and a GraphQL
  mutation (secondary).

It is not a repository-only assertion. The gap is that the fake’s “live”
fixture values equal the declared values for the fields the change says
differ (see Finding F1), so the scenario does not exercise the
identity-not-a-copy rule even though `design.md` claims it does.

### 6. Driver fidelity — valid for a CLI

This capability is a terminal/CLI interface, not a browser UI. The e2e uses
`std::process::Command` in the shared `adopt` helper to run
`scripts/adopt-public-home.sh` as a real subprocess, with a real `tofu`
binary and the local fake GitHub API behind the provider. It does not send
HTTP from a page, does not call `fetch()`/`XMLHttpRequest`, and does not use
an in-process adoption routine. A browser driver is not applicable; the
opposite classification is addressed in “E2E policy” below.

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
  (the When step, `prepare`, `adopt`, journal and state-list helpers).

The tree was searched for a second/parallel adopter:
`grep -rln 'adopt-public-home\|adopt_existing'` returns only those two step
files, the two `mod.rs` files, the command script, and documentation.
`git status --porcelain` is clean; ignored directories are the OpenTofu
cache and `__pycache__`, with no second implementation. No weaker parallel
implementation was found.

### 8. E2E scope and normalized inventory

The owning use case has exactly one normalised interaction:
“adopt the selected batch of existing objects”. The feature has exactly one
`@e2e` scenario for it. The other ten scenarios are variants/error cases of
the same action (invalid declaration, malformed batch, missing object,
already bound, unreadable object, undeclared entry, changed object, wrong
passphrase, state-write failure, narrower batch). There is no second `@e2e`
for the same action. This is the canonical scope.

### 9. E2E command isolation and counts

`verify.e2e_command` is not a copy of `verify.command`; it is the same
runner with the `@e2e` filter. The recorded counts are 132 full and 4 e2e;
the e2e scope is strictly smaller. No archive receipt exists yet; the
archive will record the per-scope receipt. This is not an excuse: the
recorded `tasks.md` evidence is used, and the archive gate will measure the
final merged state.

### 10. Implementation vs `design.md` — commands/files/framework match; two deviations

| Design item | As built | Match? |
|---|---|---|
| `verify.command` = `./run-tests.sh` | `givn/commands.yaml` and `run-tests.sh` | yes |
| `verify.e2e_command` = `./run-tests.sh --e2e` | `givn/commands.yaml` and `run-tests.sh`; `--tags @e2e` | yes |
| Non-e2e step file `apps/cli/tests/steps/adopt_existing_configuration.rs` | exists, real subprocess helpers and assertions | yes |
| E2E step file `apps/cli/tests/steps/e2e/adopt_existing_configuration.rs` | exists, separate; transcript committed | yes |
| Framework: cucumber-rs with `std::process::Command` | `features_runner.rs` and step files | yes |
| `infra/` files, batch, encrypted state, lock, `.opentofu-version` | all present; state and backup are ciphertext (`encrypted_data`, `encryption_version: v0`) | yes |
| Fake API, `/api/v3` normalization, journal, faults, mutations, unknown-path 599 | `scripts/fake-github/serve.py` implements them | yes |
| Setup script: pin, mirror, terraformrc, fixture proof | `scripts/setup-infra-tests.sh` implements them; recorded proof output has 5 imports and a no-op refresh | yes |
| Exit-2 verification hardening | `adopt-public-home.sh` parses `change_summary`/`resource_drift`; `2e2992d` demonstrates the fix | yes |
| Live fixture values represent the “hand-made” live configuration | Fixture uses declared values for the divergent fields | **no — F1** |
| `REUSE.toml` | listed as “changed”, but no commit touches it; the existing `path = ["**"]` aggregate annotation covers the fixtures | **no — F2 (documentation)** |

### 11. Interaction coverage cross-reference

| Inventory entry (`specs/adopt-existing-configuration/usecase.md`) | Design `Interaction Coverage Matrix` row | `@e2e` scenario in the feature | Driving mechanism in the step definitions | Match? |
|---|---|---|---|---|
| adopt the selected batch of existing objects | adopt the selected batch of existing objects / “Maintainer adopts the existing public home and every object is bound without changing it” | “Maintainer adopts the existing public home and every object is bound without changing it” | `std::process::Command` runs `scripts/adopt-public-home.sh` with `--declaration`, `--batch`, `--state`, `--api-url`, `--token`, `TF_VAR_state_passphrase`; shared `prepare` starts the fake; the e2e asserts stdout, journal and state | yes |

Every inventory entry has a matrix row, an existing `@e2e` title, and the
promised CLI driver. No missing or extra matrix row was found.

### 12. Fabrication-audit conclusion

The change is real, not a stub: the command, declaration, fake, setup proof,
runner and step definitions exist and the recorded suite is green. Two
audit findings remain: F1 (live-fixture/test-fidelity gap in the e2e) and F2
(stale `REUSE.toml` design row). Per the instruction, they are unfinished
work, not coverage excuses.

## Arc42 implementation conformance

The durable chapters and the change-level `arc42.md` were read. The affected
chapters were independently derived from the implementation, `design.md`
and `tasks.md` before accepting the change-level claims. All twelve chapters
marked affected by `arc42.md` contain current, decision-specific content.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 1 — reproducibility goal and Persona roster | `docs/arc42/01-introduction-and-goals.md` (goal 6, Persona rows) | affected: reproducible public home; owner/reviewer | Scope, Persona impact | Setup, scenarios 1–11 | `infra/`, adoption command, transcript | yes |
| 2 — OpenTofu/provider floor and pin, encrypted committed state, environment-only passphrase/token, no apply in Actions, no lock, read-only adoption, offline fake | `docs/arc42/02-architecture-constraints.md` lines 28–34 | affected: constraints | Technology decisions, encryption block, test seam | Setup, 7–11 | `infra/versions.tf`, `.opentofu-version`, `.terraform.lock.hcl`, CI, `adopt-public-home.sh`, `serve.py` | yes |
| 3 — declared configuration/adoption as project-side interface and partner table | `docs/arc42/03-context-and-scope.md` lines 33–78 | affected: context | Architecture impact, test seam | all scenarios | `infra/`, command, fake | yes |
| 4 — infrastructure-as-code strategy and technology table | `docs/arc42/04-solution-strategy.md` lines 10, 25–26 | affected: strategy | Technology decisions | Setup, 1–11 | declaration and command | yes |
| 5 — public-home declaration toolbox blocks | `docs/arc42/05-building-block-view.md` lines 68–90 | affected: building blocks | Architecture impact, step definitions | Setup, all | `infra/`, scripts, fake, step files | yes |
| 6 — adoption runtime: preflight, staging/validate, decrypt, import, refresh, atomic bind, refusals | `docs/arc42/06-runtime-view.md` lines 174–205 | affected: runtime | The adoption command | 1–11 | `adopt-public-home.sh` | yes |
| 7 — deployment: workstation, environment-only secrets, ciphertext state, no CI apply/drift | `docs/arc42/07-deployment-view.md` lines 63–100 | affected: deployment | Applied state, local runnability | 8, 9, real adoption | state files, command, CI | yes |
| 8 — declared-vs-applied state, all-or-none, encryption, read-only adoption | `docs/arc42/08-crosscutting-concepts.md` lines 60–110 | affected: crosscutting | Decisions, test seam | 3–10 | command, fake journal, state | yes |
| 9 — ADR-0005/0006 and indexes | `docs/arc42/09-architecture-decisions.md` lines 15–23; `docs/arc42/adr/README.md` | affected: ADRs | Decisions | Change docs | ADR files, indexes | yes |
| 10 — QS-016/017/018 | `docs/arc42/10-quality-requirements.md` lines 54–57 | affected: quality | Traceability, coverage | 11, setup, refusals | e2e, state ciphertext, refusals | yes |
| 11 — R-020…R-025 | `docs/arc42/11-risks-and-technical-debt.md` lines 26–30 | affected: risks | Risks | 8, 9, real adoption | declaration, command, state | yes |
| 12 — no new terms; Adoption/Declared configuration/Protection ruleset code symbols | `docs/arc42/12-glossary.md` lines 26, 39, 49, 153 | affected: glossary | Decisions, file layout | all | `scripts/adopt-public-home.sh`, `infra/*.tf`, `infra/rulesets.tf` | yes |

**ARC42 CONFORMANCE: CLEAN**

The five arc42 sign-off checks hold: the affected chapters were independently
derived; every affected chapter is current; each fact is cross-referenced to
design/tasks/implementation; no omission or stale contradiction was found;
the clean marker is recorded only after the table was completed.

## Ubiquitous language conformance

The durable glossary’s confirmed terms are reused with their exact senses:
**Adoption**, **Declared configuration**, **Protection ruleset**,
**Configuration change**, **Configuration drift**, **Deploy key**, **Public
home**, **Default branch**, **Gate (on landing)**, **Immutable release**.
The spec and design use “rule set” as the object the glossary names
**Protection ruleset**; the glossary’s anti-term (the legacy branch
protection as a separate object) is correctly excluded. The change does not
rename or silently extend a confirmed term.

The labels **applied state**, **selected batch** and **import identifier**
are the record/input vocabulary the topic’s event storming records as
artifact labels/mechanism for design, not new domain terms; the glossary’s
promotion boundary states that the declared configuration is a specification
and the applied state is a record. No conflicting sense was found. No
ubiquitous-language finding.

**UBIQUITOUS LANGUAGE: CONFORMANT**

## Coverage classification

Coverage is measured by the archive. The available XML files
(`coverage/non-e2e-cobertura.xml`, `coverage/e2e-cobertura.xml`,
`coverage/cobertura-coverage.xml`) predate the final step commit (timestamp
18:15) and there is no archive receipt for this change yet; the archive will
measure the merged revision. Classification from inspection:

1. **Dead code (YAGNI/KISS) → delete.** No dead production code was found.
   The command’s helpers, refusal branches and fake endpoints used by the
   scenarios are reachable; the change adds no unused Rust product code. The
   existing release/e2e scenarios are untouched. No deletion required.
2. **Missing test coverage → add scenario.** **F1**: the fake’s live
   repository values are the declared values for the fields the change says
   differ, so the e2e does not test the declared-vs-live divergence or the
   “identity, not a copy” rule. This is missing test coverage (bucket 2).
3. **Legitimately hard to test (rare).** None. The shell/Python/HCL are
   outside the Rust coverage denominator by the design’s declared process
   boundary; their observable contract is exercised by the eleven scenarios,
   and the real adoption is the operator-run evidence. That is not an excuse
   for F1, which is testable with the fake.

No classification outside these three buckets is used. F1 is not “future
scope”, “acceptable for now”, or any other fourth-bucket language.

## E2E policy and real-interface assertion

The canonical policy was read from
`givn instructions specs --change declare-public-home`.

- **Opposite classification:** a rendered page is a Web UI; a Web UI would
  require a real browser driver such as Playwright, and an HTTP client would
  not be a valid substitute. That classification **does not hold here**:
  `scripts/adopt-public-home.sh` is a terminal command; its interface is
  plain text on stdout/stderr. There is no HTML, CSS, JavaScript, DOM, page
  request, or browser-rendered state. The design correctly declares
  `Interface: terminal`.
- **Real interface:** the real subprocess invocation of the adoption
  command, with the real `tofu` binary and the local fake GitHub API behind
  the provider.
- **Primary assertion:** the command’s stdout — exactly five
  `adopted <address> <import-id>` lines. The journal and `tofu state list`
  are secondary evidence; the Then is not repository-only.
- **Driver:** `std::process::Command`, not an in-process call and not HTTP
  from page JavaScript.
- **Scope:** one normalized action, one `@e2e`; variants are regular
  scenarios.
- **Gap:** F1 means the e2e’s success path does not model the live values
  the declaration deliberately differs from; it does not change the
  primary-assertion rule, but it leaves the identity-not-copy rule unproven.

## Use-case and Persona conformance

The owning use case is
`givn/changes/declare-public-home/specs/adopt-existing-configuration/usecase.md`;
the ideation contract is
`givn/ideation/public-home-infrastructure/use-cases/adopt-existing-configuration.md`.
The implementation preserves the goal (“place the already-existing objects
of the public home under the declared configuration without changing them”),
the minimal/success guarantees, the rules (no live change, identity not a
copy, all-or-none selected batch, excluded deploy key/secret, one protection
object, encrypted local state), the extension step references, the
capability ownership, and the confirmed Persona references. The declaration
revision is a repository commit; the command reads and validates it, which
is the use case’s recorded division.

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
advisories and otherwise “clean”.

## Split-or-keep

No scenario exceeds the long-scenario gate threshold; the change’s longest
scenario is the eight-step missing-object scenario, and `givn lint` reported
no `[LONG]` finding. No split-or-keep decision is required. There are no
removed+added pairs.

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

## Findings

### F1 — Align the fake live repository values with the recorded live configuration

- **Priority:** P2
- **Confidence:** 0.9
- **File:** `scripts/fake-github/fixtures/adopt-all/repos/duty2cal/duty2cal.json`
- **Lines:** 39–41 (merge fields; `has_projects` is line 21)

**Bug.** The fake’s “live” monorepo uses the declared values for the four
fields the change says differ live: `has_projects = false` (line 21),
`allow_merge_commit = false` (line 39), `allow_rebase_merge = false` (line
40), `delete_branch_on_merge = true` (line 41). The committed live read
`evidence/live-read/repo-mono.json` records `has_projects = true`,
`allow_merge_commit = true`, `allow_rebase_merge = true`,
`delete_branch_on_merge = false`. The pages fixture has the same problem:
`has_issues = false` and `has_wiki = false` while
`evidence/live-read/repo-pages.json` records both `true`.

**Trigger and impact.** `design.md`’s traceability row for “the state binds
an identity, not a copy of the values” cites “`@e2e` (declared projects-off
vs live projects-on stays live)”. The success scenario’s Then only asserts
the five stdout lines, the journal flags and the state addresses; it never
asserts a value difference. Because the fake’s live values equal the
declared values, a regression that wrote declared values into the applied
state (or otherwise reconciled declaration to live) would not be caught by
the e2e. The scenario therefore does not exercise the identity-not-copy
guarantee it claims to cover, and the fake is not the “hand-made
configuration” its Given names. The real adoption state is unaffected
(it imported real live values), but the suite’s e2e fidelity is reduced.

**Fix.**

1. Set the mono fixture to the recorded live values:
   `"has_projects": true`, `"allow_merge_commit": true`,
   `"allow_rebase_merge": true`, `"delete_branch_on_merge": false` (keep
   `has_issues: true`, `has_wiki: false`, `has_discussions: false`,
   `allow_squash_merge: true`, `allow_auto_merge: false`,
   `allow_forking: true`).
2. Set the pages fixture to `"has_issues": true`, `"has_wiki": true` (keep
   `has_projects: true` and the live merge settings; the current pages
   merge fields already match live).
3. Add a Then assertion after adoption that the bound state carries the
   live values, not the declared ones: run
   `tofu state show github_repository.mono` with the test passphrase and
   assert `has_projects = true`, `allow_merge_commit = true`,
   `allow_rebase_merge = true`, `delete_branch_on_merge = false`; or read
   the fake’s repository endpoints after adoption and compare with the
   original live fixture. The journal’s no-write assertion remains the
   secondary proof that live did not change.
4. Re-run the single scenario, `./run-tests.sh`, and `./run-tests.sh --e2e`;
   update the `tasks.md` evidence and the committed transcript. The real
   adoption transcript/state need no re-run; they already record the real
   live values.

### F2 — Correct the `REUSE.toml` row in the reviewed design

- **Priority:** P3
- **Confidence:** 0.85
- **File:** `givn/changes/declare-public-home/design.md`
- **Line:** 102

**Bug.** The design’s file table says `REUSE.toml` is “changed” for SPDX
attribution of the fake’s JSON fixtures. No commit in this change touches
`REUSE.toml` (`git log -- REUSE.toml` shows only the earlier
`642f50d`/`f9dc7be`). The existing aggregate annotation
(`path = ["**"]`, MPL-2.0) already covers the fixtures, so the outcome is
met, but the reviewed design row is stale and the implementation silently
deviates from it.

**Fix.** Update the row to “unchanged — the existing `path = ["**"]`
annotation covers the fake fixtures”, or add an explicit annotation for the
fixtures and record that change. Because `design.md` changes, re-run the
design-review sign-off and record the corrected row in `arc42.md` if the
change is structural. (No implementation gap remains if the row is
corrected.)

## Sign-off checklist

- [ ] Fabrication audit: clean — **FAIL (F1, F2)**
- [x] Every checked task has a verified commit; no spec-only revision
      (production command/declaration in `2bd68e3`, `cd89ada`, `bc134b0`,
      `2e2992d`; scenario revisions add the real step definitions)
- [x] Every promised component exists (F2 is a stale design row, not a
      missing component)
- [x] Strict-mode proof present and passing (`fail_on_skipped`; two
      undefined-step proofs in `tasks.md`)
- [x] Scenario execution evidence recorded per scope in `tasks.md`
      (132 full / 4 e2e); no archive receipt yet — the archive gate will
      carry the merged receipt
- [ ] Coverage classification: one bucket-2 gap (F1); no dead code; no
      hard-to-test excuse
- [x] Every coverage gap classified and resolved or fixed by the listed F1
      work; no fourth-bucket language used
- [x] No `@wip` tags; no implementation-layer detail beyond the batch’s
      observable input syntax
- [x] Canonical E2E policy read; one normalized inventory action has one
      `@e2e`; variants are regular scenarios
- [x] Every e2e scenario has a real-interface primary assertion and uses
      the reviewed CLI driver; no browser/HTTP-page substitution
- [x] The local run path starts the fake/digital twin per scenario; the
      recorded setup proof and 132/4 runs are the evidence (not re-run)
- [x] The exact files `verify.e2e_command` invokes were identified and
      read; no parallel adopter exists in the tree (including ignored dirs)
- [x] `verify.e2e_command` is not identical to `verify.command`; 4 < 132
- [ ] Implementation (commands, file layout, framework/driver) matches
      `design.md` — commands/files/framework match; **F1 test data and F2
      stale row remain**
- [x] Interaction coverage: inventory → matrix → `@e2e` → driver, no
      mismatch
- [x] No finding was excused with a classification outside the three
      buckets; no “future scope”, “acceptable for now”, or similar language

## Required fixes before the review can pass

1. **F1:** align the fake live values to `evidence/live-read/repo-mono.json`
   and `repo-pages.json`, add the state/live-value assertion to the `@e2e`
   Then, re-run the scenario and both suite scopes, and update `tasks.md`
   and the committed transcript.
2. **F2:** correct `design.md`’s `REUSE.toml` row (or add the explicit
   annotation) and re-run the design-review sign-off.
3. Re-run `givn lint --change declare-public-home` and the review after the
   fixes; the archive receipt will then carry the final per-scope counts.

REVIEW: FAIL
