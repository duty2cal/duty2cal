# Review: publish-site-plumbing

- **Change:** `publish-site-plumbing`
- **Capability:** `publish-site-plumbing`, routed to use case `apply-configuration-change`
- **Owning use case (delta):** `givn/changes/publish-site-plumbing/specs/apply-configuration-change/usecase.md`
- **Design:** `givn/changes/publish-site-plumbing/design.md`
- **Design review:** `design-review.md` — `DESIGN-REVIEW: PASS` (initial review and §7 re-review after the operator apply)
- **Reviewed commits:** `daad3f5` (operator apply), `6b201f2` (manual organization prerequisite), `51bb244` (design re-review and deferral record)
- **Review basis:** the review instruction was loaded with `givn instructions review --change publish-site-plumbing`; the E2E policy was loaded with `givn instructions specs --change publish-site-plumbing`. The full suite was **not** re-run: the review uses the recorded runs and static inspection.

## Preconditions

- `givn status --change publish-site-plumbing` reports **65/67 tasks**. `tasks.md` carries 65 `[x]` and 2 `[ ]` checkboxes. (The 65/66 figure supplied in the request predates the second archive-time box added by `51bb244`; the file and `givn status` both report 67 boxes.)
- The two open boxes are the **Archive-time obligation**, explicitly marked in `tasks.md` as “after review, not a review gate”:
  - verify `cliff.toml`'s marked capability allow-list still names `apply-configuration-change` (no edit owed; TD-021 remains for the change that owns the change log);
  - verify the merged `givn/specs/apply-configuration-change/usecase.md` carries the delta's corrected stale-marker contract and the post-apply read-back extension.
- The user's continuation is recorded: the design re-review (§7 of `design-review.md`) found no blocking findings, and `tasks.md` “Done criteria” records the operator's deferral of the pre-existing full-suite failures from the release 0.5.0 merge.
- Scope of this review: the change's ten delta scenarios, the declaration and the two `@e2e` transcripts. The full-suite failures and the website publication change are recorded below as out-of-delta work, not as a coverage bucket.
- Proposal routing: the table row at `proposal.md:124–127` names `publish-site-plumbing` / `NEW in apply-configuration-change`; the backticked-cell parser warning was removed by formatting the capability and decision cells, and `givn check review` reports no routing warning. The capability is new to the use case and the delta `usecase.md` declares it.

## Step 0 — Fabrication audit

### 0. `@e2e` tag integrity

`givn/changes/publish-site-plumbing/specs/apply-configuration-change/publish-site-plumbing.feature` contains **10 scenarios**: **2 `@e2e`** (titles below) and **8 non-`@e2e`**.

- `@givn.added @e2e` — Organization owner previews the pages plumbing and only the three live creations are named
- `@givn.added @e2e` — Organization owner applies the previewed revision and the pages publishing credential is created

The feature file was added once in `da878bd`; every later edit to it is a `@wip` tag removal (`-  @givn.added [@e2e] @wip` / `+  @givn.added [@e2e]`). `git log -p --follow` shows **no removed `@e2e` tag and no `@givn.removed` scenario**. No tag was removed to bypass the verify-e2e gate.

### 1. Empty or trivial step bodies

Scanned every step definition file under `apps/cli/tests/steps/` (**20 files**) with attribute-and-brace parsing: **471 step functions, 0 empty or trivial bodies, 0 `todo!()`/`unimplemented!()`/`pass` bodies**.

For this capability: **47 step attributes in `apps/cli/tests/steps/publish_site_plumbing.rs`** and **13 in `apps/cli/tests/steps/e2e/publish_site_plumbing.rs`**; all have substantive bodies that reference the real subprocesses, the fake's HTTP responses, the journal, the state or the transcript.

### 2. Checked tasks and their commits

All **13** recorded `commit:` hashes exist in history:

`da878bd`, `c48a381`, `400f22d`, `2ae7ce1`, `db20d86`, `653562e`, `224fca6`, `42ab71f`, `72667c9`, `fffa813`, `bf36589`, `5a228a1`, `daad3f5`.

Spot-checked diffs:

| Task commit | Touches |
|---|---|
| `da878bd` | two new step modules + `mod.rs` declarations, the declaration and lock, and the arc42 chapters |
| `72667c9` | `apps/cli/tests/steps/publish_site_plumbing.rs` (+133 lines), feature tag, tasks |
| `bf36589` | `apps/cli/tests/steps/e2e/publish_site_plumbing.rs` (+138), transcript, `run-tests.sh` |
| `5a228a1` | `apps/cli/tests/steps/e2e/publish_site_plumbing.rs` (+280), transcript |
| `daad3f5` | `infra/terraform.tfstate`, `.backup`, `evidence/real-apply/transcript.txt`, task record |

`fffa813` touches `apps/cli/tests/steps/e2e/publish_site_plumbing.rs` and the two priming modules (the E2E setup task). No checked task is spec-only.

### 3. Promised components

Every component named by `design.md` exists:

| Design promise | Present |
|---|---|
| `infra/pages_publish.tf` with `tls_private_key.pages_publish`, `github_repository_deploy_key.pages_publish`, `github_actions_secret.pages_publish`, `github_repository_ruleset.pages_publish` | yes (four resources; `read_only = false`; `PAGES_DEPLOY_KEY`; `DeployKey`/`always`, no id) |
| `infra/versions.tf` `tls ~> 4.0` | yes |
| `infra/.terraform.lock.hcl` `hashicorp/tls` 4.4.1 beside GitHub 6.13.0 | yes |
| `scripts/setup-infra-tests.sh` mirrors both providers, writes `include`/`exclude` for both, removes `pages_publish.tf` from the inherited-base workspaces | yes (lines 65–102, 174, 222, 256) |
| `scripts/fake-github/serve.py` deploy-key/secret/rule-set endpoints, overlays, `--writes`, `after_requests`, journal fields (`applied_write`, `ciphertext_bytes`, `contains_pem_header`) | yes |
| `scripts/fake-github/fixtures/security-applied/**` empty keys/secrets/rulesets and fixed public key | yes (`[]`, `[]`, `[]`, `key_id 568250167242205012`) |
| `apps/cli/tests/steps/publish_site_plumbing.rs` | yes (1,973 lines) |
| `apps/cli/tests/steps/e2e/publish_site_plumbing.rs` | yes (502 lines) |
| Both `mod.rs` declarations | yes |
| Evidence transcripts | yes (two under `evidence/visual/<slug>/transcript.txt`; operator apply under `evidence/real-apply/transcript.txt`) |
| arc42 chapters, ADR-0006 amendment, glossary | yes (chapters 1–12 substantive; ADR amendment at line 133; glossary rows) |

### 4. Strict-mode proof

`apps/cli/tests/features_runner.rs:143` calls `.fail_on_skipped()`. `tasks.md` records the strict probe: `./run-tests.sh --e2e` exited **101** with “Step doesn't match any function” for the then-undefined pages steps. The E2E setup task records the same proof before the step scaffold. The proof is present with output.

### 5. `@e2e` Then assertions

- **Preview:** asserts the saved plan's JSON (`tofu show -json`) names the deploy key, the `PAGES_DEPLOY_KEY` secret, the rule set and the generated key as creates, and that the create set is exactly those four addresses (`CHANGES`/`CREATES` `jq` filters over the real plan report).
- **Apply:** asserts the real apply transcript contains the three created addresses and `Apply complete! Resources: 4 added, 0 changed, 0 destroyed.`, then reads the created deploy key, the sealed secret's journal entry (`key_id`, ciphertext bytes > 48, no PEM header), the active rule set with `DeployKey`/`always`, the state's four addresses and the empty follow-up plan.

Neither scenario asserts only against a repository/database: both have a primary assertion on the CLI's output, and the state/live reads are secondary evidence.

### 6. E2E implementation driving mechanism

The e2e module calls the shared `preview`/`apply_previewed` routines; both invoke the real binary through `tofu()` → `Command::new("tofu")` (`apps/cli/tests/steps/enable_repository_security.rs:100`) with `init`, targeted `plan -out`, `show -json`, refresh-only and `apply`. No HTTP client or `fetch` replaces the CLI driver for the preview/apply interaction.

The guard and post-apply read-back in the **test steps** read the fake's REST paths directly (`fake_request`) with the design's `jq` filters. `design.md` (“Step definitions”) discloses this: `gh api` cannot be pointed at a loopback server; the transport is GitHub's client and the artifact under test is the response shape and the verdict. This is a disclosed test-harness boundary, not a downgraded interaction: the real `tofu` subprocess remains the driver for the preview and the apply.

### 7. Exact files `verify.e2e_command` invokes, and the parallel-implementation check

`givn/commands.yaml` literally reads:

- `verify.command: "./run-tests.sh"`
- `verify.e2e_command: "./run-tests.sh --e2e"`

`run-tests.sh` converts `--e2e` to `--tags "@e2e"`, assembles the feature root with `scripts/assemble-features.sh`, and runs `cargo test -p duty2cal --test features_runner -- --tags "@e2e"`. The invoked test target is `apps/cli/tests/features_runner.rs`; the `@e2e` steps for this capability are `apps/cli/tests/steps/e2e/publish_site_plumbing.rs`.

The tree was searched for a second implementation of this capability (tracked and untracked): `git status --porcelain` is clean, and the only files are the non-e2e `apps/cli/tests/steps/publish_site_plumbing.rs` and the e2e `apps/cli/tests/steps/e2e/publish_site_plumbing.rs`. No parallel or weaker implementation exists. The strongest implementation is the one reviewed.

### 8. Normalized E2E scope

The delta use case has **6 inventory entries**; `design.md` has **6 matrix rows**; the delta feature has **2 `@e2e` scenarios**, one for each new action (“preview the declared configuration and read the named differences”, “apply the previewed revision and read the outcome”). The inherited four entries have their four existing `@e2e` scenarios in `givn/specs/apply-configuration-change/enable-repository-security.feature` and `publish-organization-profile.feature`. There is no excess `@e2e` scenario and no action with two.

### 9. `verify.e2e_command` scoping

The two values are not identical. The recorded runs in `tasks.md` show:

- full suite: **194 scenarios** (163 passed, 31 failed) at the E2E setup run;
- `--e2e`: **13 scenarios** (8 passed, 5 failed at that point; 2 were the then-undefined pages scenarios and 3 are pre-existing failures).

**13 < 194**, so the e2e scope is strictly smaller and the tag filter is effective. The `givn/archive/publish-site-plumbing/verification.json` receipt does not exist yet; the archive will record the per-scope counts. The review does not re-run the suite.

### 10. Implementation vs `design.md` commands, file layout, framework and driver

| Design-declared item | As built | Match |
|---|---|---|
| `verify.command` = `./run-tests.sh` | `givn/commands.yaml` | yes |
| `verify.e2e_command` = `./run-tests.sh --e2e` | `givn/commands.yaml` | yes |
| separate e2e step file `apps/cli/tests/steps/e2e/publish_site_plumbing.rs` | present, declared in `steps/e2e/mod.rs` | yes |
| non-e2e step file `apps/cli/tests/steps/publish_site_plumbing.rs` | present, declared in `steps/mod.rs` | yes |
| cucumber-rs, `std::process::Command`, real `tofu` | `features_runner.rs`, `tofu()` | yes |
| fake server on an ephemeral loopback port in `--writes` mode | `serve.py`, `start_fake_with` | yes |
| evidence path `evidence/visual/<scenario-slug>/transcript.txt` | both transcripts present at the exact slugs | yes |
| `infra/pages_publish.tf` layout and four resources | present | yes |
| `cliff.toml` unchanged | no diff | yes |

`design.md` was updated for the manual organization prerequisite in `6b201f2` and re-reviewed in `51bb244`; the re-review is `DESIGN-REVIEW: PASS`. No silent deviation remains.

### 11. Interaction coverage cross-reference

Inventory source: `givn/changes/publish-site-plumbing/specs/apply-configuration-change/usecase.md` `## Interactions` (6 rows). Matrix source: `design.md` “Interaction Coverage Matrix” (6 rows). Scenario source: the delta feature plus the two inherited permanent feature files. Driver source: the step modules.

| # | Inventory entry | Matrix row | `@e2e` scenario | Driving mechanism | Match |
|---|---|---|---|---|---|
| 1 | `enable-repository-security` — preview the declared configuration and read the named differences | same | Organization owner previews the security settings and only the declared differences are named | `std::process::Command` → real `tofu` (`steps/enable_repository_security.rs`) | yes |
| 2 | `enable-repository-security` — apply the previewed revision and read the outcome | same | Organization owner applies the previewed revision and the security settings are enabled | `std::process::Command` → real `tofu` | yes |
| 3 | `publish-organization-profile` — preview the declared configuration and read the named differences | same | Organization owner previews the profile declaration and only the three creations are named | `std::process::Command` → real `tofu` (`steps/publish_organization_profile.rs`) | yes |
| 4 | `publish-organization-profile` — apply the previewed revision and read the outcome | same | Organization owner applies the previewed revision and the organization profile is published | `std::process::Command` → real `tofu` | yes |
| 5 | `publish-site-plumbing` — preview the declared configuration and read the named differences | same | Organization owner previews the pages plumbing and only the three live creations are named | `std::process::Command` → real `tofu`; guard reads the fake's REST lists (disclosed) | yes |
| 6 | `publish-site-plumbing` — apply the previewed revision and read the outcome | same | Organization owner applies the previewed revision and the pages publishing credential is created | `std::process::Command` → real `tofu`; read-back reads the fake's REST lists and journal (disclosed) | yes |

No inventory entry is unmapped; no matrix row lacks a scenario; no promised driving mechanism is a different driver.

**Fabrication audit result: clean.** 0 empty step bodies across 20 files; 0 missing components; no `@e2e` removal; no parallel e2e implementation; no silent design deviation.

## Arc42 implementation conformance

The affected-chapter set was re-derived from the implementation, `design.md` and `tasks.md` before accepting `arc42.md`: goals/quality goal, constraints, context partner, strategy, building blocks, runtime scenario, deployment prerequisite, cross-cutting credential, ADR-0006, QS-034, R-051–R-056/TD-021 and the glossary. All 12 rows match.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| 1 — goal 5, quality goal 6, organization-owner stakeholder | `docs/arc42/01-introduction-and-goals.md:13,27,38` | Yes: pages publishing credential and rule set named; passphrase guards the only recoverable key | Scope | header/shared constraints | `infra/pages_publish.tf`; transcripts | yes |
| 2 — constraints: in-declaration Ed25519 pair, encrypted-state-only private key, actor-type bypass, create guard, live-read classification, org deploy-key policy, two-provider mirror | `docs/arc42/02-architecture-constraints.md` | Yes | “The declaration”, “Post-GREEN operator apply” | shared constraints; operator task | `pages_publish.tf`, `versions.tf`, `setup-infra-tests.sh`, real-apply transcript | yes |
| 3 — context partner and both rule sets, deploy key, secret, future publishing workflow, guard output | `docs/arc42/03-context-and-scope.md` | Yes | architecture impact / scope | shared constraints | fixture keys/secrets/rulesets; transcript | yes |
| 4 — infrastructure strategy, second provider, state's new role, creation guard | `docs/arc42/04-solution-strategy.md` | Yes | technology decisions | shared constraints | `versions.tf`; declaration; transcript | yes |
| 5 — building blocks: `infra/pages_publish.tf`, provider toolbox, fake endpoints/overlays, second mirror, step-module pair | `docs/arc42/05-building-block-view.md:127,136,161–170` | Yes | design file table | file lists | all files present | yes |
| 6 — “The pages publishing plumbing is previewed and created” | `docs/arc42/06-runtime-view.md:408` | Yes | “The apply procedure” | scenarios 9–10 | real `tofu` routines; two transcripts | yes |
| 7 — declared configuration, manual organization deploy-key policy, 422 recovery | `docs/arc42/07-deployment-view.md:160,196–219,253` | Yes | post-GREEN operator apply | operator task | `daad3f5`; real-apply transcript | yes |
| 8 — section “The pages publishing credential and its rule set”; state encryption, read-back, failure reporting | `docs/arc42/08-crosscutting-concepts.md:174` | Yes | declaration/apply procedure | shared constraints | transcripts; state | yes |
| 9 — ADR-0006 amendment in place; index updated | `docs/arc42/09-architecture-decisions.md:16,40`; `docs/arc42/adr/ADR-0006-encrypted-local-applied-state.md:133` | Yes (AMEND_ADR) | Decisions | operator/archive records | ADR amendment present | yes |
| 10 — QS-034 | `docs/arc42/10-quality-requirements.md:81` | Yes | scenarios | scenarios 1–10 | ten delta scenarios; two transcripts | yes |
| 11 — R-023 corrected; R-051–R-056; TD-021 | `docs/arc42/11-risks-and-technical-debt.md:29,52–57,89` | Yes | risks/follow-ups | operator task; archive-time box | real-apply transcript; guard | yes |
| 12 — **Pages publishing plumbing**, **Deploy key**, **Protection ruleset**; stale-marker contract | `docs/arc42/12-glossary.md:53,63,73,74` | Yes | Visual Contract / Decisions | shared constraints | code symbols match glossary rows | yes |

**ARC42 CONFORMANCE: CLEAN**

## Ubiquitous-language conformance

- `Pages publishing plumbing` is in `docs/arc42/12-glossary.md:74` with its code symbols (`infra/pages_publish.tf`, `tls_private_key.pages_publish`, `github_repository_deploy_key.pages_publish`, `github_actions_secret.pages_publish`, `github_repository_ruleset.pages_publish`) and its anti-terms. The design, feature and transcripts use the term consistently.
- `Deploy key` (line 63) and `Protection ruleset` (line 73) carry the implementation's exact facts: the write-enabled key titled `duty2cal.github.io publish`, the state-only private half, the `DeployKey`/`always` bypass with no actor id, and the three rules.
- `Configuration change` (line 53) carries the corrected stale-marker contract used by the delta use case and scenarios.
- No spec, design or code term is absent from the glossary; no term is used with a competing meaning. No glossary edit is owed.

## Coverage classification

No coverage evidence exists yet for this change: `givn/archive/publish-site-plumbing/verification.json` and its directory are absent. The classification below is from static inspection of the committed candidate; the archive receipt will carry the measured result. Review does not run an instrumented pass.

1. **Dead code** — none. The change adds `infra/pages_publish.tf`, two declared step modules and fake endpoints; both modules are declared and used; no unused production path was introduced. No redundant unit test was added (the `.feature` files are the executable specification).
2. **Missing test coverage** — none inside the delta. Each of the ten scenarios has bound steps and recorded green targeted runs; the two `@e2e` scenarios have real-interface primary assertions and committed transcripts. The inherited four `@e2e` scenarios remain in the permanent corpus.
3. **Legitimately hard to test** — live GitHub acceptance of the generated Ed25519 deploy key, the sealed secret and the `DeployKey` bypass cannot be exercised against the fake; the fake proves the provider's request shapes, not GitHub's acceptance. The concrete evidence is the required operator apply (`daad3f5`, `evidence/real-apply/transcript.txt`), which succeeded (recovery `2 added`, follow-up plan “No changes”) and left the stock site serving. The organization `deploy_keys_enabled_for_repositories` policy is not expressible in pinned provider 6.13.0 and is verified/enabled manually; the transcript records the policy, the 422 and the recovery.

**Out-of-delta, operator-deferred (not a coverage bucket):**

- the pre-existing full-suite failures from the release 0.5.0 merge: 194 scenarios with 31 failures at the recorded run, and 3 of the 13 `@e2e` scenarios failing pre-existing;
- the website publication change.

The operator decision is recorded in `tasks.md` “Done criteria” and §7 of `design-review.md`. The archive gate still enforces the full suite: the archive must not sign off until the suite is green or the failures are classified. This record does not convert either item into a fourth bucket.

## E2E coverage

- **Classification:** CLI / terminal. The consumer runs the real `tofu` binary and reads its terminal output. The opposite classification — a rendered page is a Web UI — does not hold: the capability renders no HTML, CSS or JavaScript, no browser consumes the surface, and GitHub is an external REST system rather than a UI. The browser-driver rule therefore does not apply.
- **Real driver:** `Command::new("tofu")` at `apps/cli/tests/steps/enable_repository_security.rs:100`; the e2e module calls the shared preview/apply routines.
- **Primary assertions:** the preview asserts the saved plan's JSON; the apply asserts the apply transcript, then the live key, sealed secret, rule set, state and empty follow-up plan.
- **Guard/read-back:** the test steps read the fake's REST paths directly because `gh api` cannot target a loopback server; `design.md` discloses this and the real `tofu` subprocess remains the driver.
- **Scope:** 13 `@e2e` scenarios of 194 in the recorded full run; the delta contributes 2.
- **Evidence:** `evidence/visual/organization-owner-previews-the-pages-plumbing-and-only-the-three-live-creations-are-named/transcript.txt` and `evidence/visual/organization-owner-applies-the-previewed-revision-and-the-pages-publishing-credential-is-created/transcript.txt`.

## Use-case and Persona conformance

- **Goal, trigger, actors:** unchanged. Goal is the single “Make the live configuration match the declared configuration revision the apply accepts”; the primary Actor is the organization owner; the trigger names the owner recording a revision.
- **Guarantees and rules:** the delta preserves the minimal guarantee, the success guarantee, the preview scope, no-removal, readability, adoption-before-update, preflight, refusal/partial outcomes and the corrected stale-marker contract. The new post-apply read-back extension is carried; the archive-time box verifies the merge into the durable use case.
- **Extensions:** each references a main-flow step and protects a named guarantee; no new success outcome was added.
- **Capabilities:** `publish-site-plumbing` is goal-shaped and advances the apply goal; placement is correct (channel is not the capability).
- **Personas:** all three confirmed Personas are dispositioned:
  - `project-maintainer--organization-owner` — runs the guard, previews and applies; the transcripts show all three live creations, the generated key and the green follow-up.
  - `contributor--infrastructure-reviewer` — reads the declaration and the scrubbed terminal evidence without a token or passphrase.
  - `crew-member--personal-calendar-owner` — no change: the stock site keeps serving; the non-e2e scenario asserts the site, source and repository settings are untouched.
- No Persona was invented or silently promoted; Actors remain interaction roles.
- Scope rejection criteria: none holds. Goal is not a conjunction; the primary actor is identifiable; no mutation under an observing goal; extensions carry step references; preconditions are not established by the flow; guarantees are not assumed; no mechanism vocabulary or component-shaped name; the main flow does not name three actor goals; no `Extends` misuse.

## Visual review

**Interface:** terminal (`design.md` “Visual Design Contract”: Interface: terminal; viewports n/a; screens covered: none). The evidence is one committed transcript per `@e2e` scenario.

**States seen by this review:**

- preview final state — `evidence/visual/organization-owner-previews-the-pages-plumbing-and-only-the-three-live-creations-are-named/transcript.txt` (132 lines; init, state list, targeted plan “4 to add”, `show -json`, refresh-only, exit 0);
- apply final state — `evidence/visual/organization-owner-applies-the-previewed-revision-and-the-pages-publishing-credential-is-created/transcript.txt` (215 lines; preview, apply “4 added”, follow-up preview “No changes”, exit 0).

**Baseline:** the transcripts carry placeholders only (`<token>`, `<passphrase>`, `<billing-email>`, `<scenario-dir>`, `<private-key>`); no token, passphrase, private key material, temporary path or live credential appears. Object addresses are the operator's domain names and are expected. No content block is duplicated: the repeated `$` command headers are distinct invocations in a sequential transcript, not duplicate rendered content. The traversed states are success states; the contract's error-state rule applies to error states, which these two scenarios do not traverse, and the non-e2e failure scenarios carry their own assertions.

**Persona findings:**

PERSONAS: project-maintainer--organization-owner
PERSONAS: contributor--infrastructure-reviewer
PERSONAS: crew-member--personal-calendar-owner

| Persona | Judgment | Disposition |
|---|---|---|
| `project-maintainer--organization-owner` | The transcript shows the guard's expected set, the four named creations, the apply's three created objects plus the generated key, and the empty follow-up plan; the owner can reach the goal. | no finding |
| `contributor--infrastructure-reviewer` | The transcript is scrubbed and names the declaration's addresses and the plan/apply outcome without a token or passphrase; the reviewer can read the evidence without authority. | no finding |
| `crew-member--personal-calendar-owner` | Nothing in the terminal surface changes the crew member's goal; the stock-site scenario asserts the site, source and settings are untouched. | no finding |

No pixel/coordinate finding is raised (not applicable); no finding contradicts a contract rule or a Persona goal.

**VISUAL-REVIEW: PASS**

## Scenario execution evidence

Review does not re-run the suite. The change has not been archived yet: `givn/archive/publish-site-plumbing/verification.json` does not exist. The recorded per-scope runs in `tasks.md` are the full suite **194 scenarios** (163 passed, 31 failed) and `--e2e` **13 scenarios** (8 passed, 5 failed at that point; 2 then-undefined pages scenarios, 3 pre-existing). The ten delta scenarios are green in their targeted runs.

**Receipt status:** not archived yet

## Deterministic gate dispositions

`givn lint --change publish-site-plumbing` exits **0** with two advisories: one shape match and one long scenario. No `@e2e` mapping error.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| An unreadable repository setting refuses the preview and names the repository | An unreadable pages repository refuses the preview and names it | variant |

The shape match is a **variant**, not a duplicate: the first scenario exercises the monorepo's unreadable repository setting, the second the pages repository's unreadable deploy-key/rule-set surface; the refusal path and the named object differ, and neither supersedes the other.

## Split-or-keep

| Scenario | Decision |
|---|---|
| A partial failure after the deploy key is created names the applied and failed objects and marks the state stale | keep |

The scenario is one coherent failure path: the deploy key lands, the secret and rule set fail with 403, the failure report names applied and failed objects, the state records only the landed key and the marker names the deploy key. Splitting would separate the partial report from the state and marker it explains.

**Removed+added pairs:** none. The delta feature was added whole in `da878bd`; later commits only remove `@wip`. There is no `@givn.removed` scenario and no added scenario that supersedes a removed one.

## README impact

`README.md` is unchanged by this change and the change introduces no consumer-facing command, flag, configuration key or output. The operator prerequisite is documented in `docs/arc42/07-deployment-view.md` and chapter 11 (R-056), not in the README.

README-IMPACT: none

## Sign-off checklist

- [x] Fabrication audit: clean (0 empty step bodies across 20 files; 0 missing components; no `@e2e` removal; no parallel e2e implementation; no silent design deviation).
- [x] Every checked task has a verified commit touching implementation source: 13 hashes exist; spot-checked `da878bd`, `72667c9`, `bf36589`, `5a228a1`, `daad3f5`.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing: `.fail_on_skipped()` at `features_runner.rs:143`; recorded exit 101 probe in `tasks.md`.
- [x] Scenario execution evidence is recorded per scope in `tasks.md` (194 full / 13 e2e); the archive verification receipt is not yet produced and will carry the measured result; the review does not re-run the suite.
- [x] Coverage classification uses the recorded evidence and static inspection; the archive receipt will carry the merged result.
- [x] Every coverage gap classified and resolved: dead code none; missing coverage none in the delta; hard-to-test items justified under bucket 3.
- [x] Dead code deleted (none introduced); missing tests added (ten scenarios, two `@e2e`); hard-to-test gaps justified.
- [x] Redundant unit tests removed (none added; the `.feature` files are the executable specification).
- [x] No `@wip` tags remain; no implementation-layer detail in the spec.
- [x] Canonical E2E policy read from `givn instructions specs --change publish-site-plumbing`; every normalized inventory action has one `@e2e` scenario; variants are regular scenarios.
- [x] Every E2E scenario has a real-interface primary assertion and uses the reviewed driver; no browser capability was replaced by HTTP/fetch.
- [x] The local run command and digital twins: the product `run` command is CLI; the E2E setup evidence records the fake on an ephemeral loopback port and the step code starts it per scenario and leaves no service running.
- [x] The exact files `verify.e2e_command` invokes were identified and read (`run-tests.sh` → `features_runner.rs` → `steps/e2e/publish_site_plumbing.rs`); the tree was searched for a second implementation (none; working tree clean).
- [x] `verify.e2e_command` is not identical to `verify.command`; the recorded per-scope counts prove the e2e scope is strictly smaller (13 < 194).
- [x] Implementation (commands, file layout, framework/driver) matches `design.md`; deviations were resolved by updating `design.md` and re-running design-review (`6b201f2`, `51bb244`).
- [x] Interaction coverage verified: 6 inventory entries → 6 matrix rows → 6 `@e2e` scenarios → promised driving mechanisms; no mismatches.
- [x] Internal Primitives: the change introduces no production-code struct/fn/enum; the new Rust is harness glue in the two step modules, and the declaration is HCL. No production primitive is owed.
- [x] arc42 chapter mapping: every chapter number in `arc42.md` maps to an existing `docs/arc42/01`–`12` file; no chapter is missing or stale.
- [x] No finding was excused with a classification outside the three buckets; the out-of-delta operator deferral is recorded separately, and the archive gate still enforces the full suite.

REVIEW: PASS
