# Tasks: publish-site-plumbing

- **Change:** `publish-site-plumbing`
- **Capability:** `publish-site-plumbing`, routed to the use case `apply-configuration-change`
- **Owning use case:** `givn/changes/publish-site-plumbing/specs/apply-configuration-change/usecase.md`
- **Design:** `givn/changes/publish-site-plumbing/design.md` (design-review `DESIGN-REVIEW: PASS`)
- **Interaction inventory:** 6 entries in the delta use case; 6 rows in the design's Interaction Coverage Matrix; the two new entries map to the two `@e2e` scenarios below.
- **Runner:** `./run-tests.sh` → `cargo test -p duty2cal --test features_runner` over the assembled feature root.
- **E2E runner:** `./run-tests.sh --e2e` → the same runner with `--tags @e2e`; distinct from `verify.command`.
- **Single scenario:** `./run-tests.sh --name "<scenario title>"`; E2E: `./run-tests.sh --name "<scenario title>"`.
- **Commit scope:** `apply-configuration-change` for the feature commits; E2E commits use `test(e2e):`.
- **Strict mode:** `apps/cli/tests/features_runner.rs` calls `.fail_on_skipped()`. Undefined, pending or skipped steps fail. Empty step bodies are banned; use `todo!()` while RED.
- **Task order:** setup, 8 non-`@e2e` scenarios in feature order, E2E setup, 2 `@e2e` scenarios, operator apply, archive-time obligation.

## Pre-flight evidence

- `givn status --change publish-site-plumbing`: design-review complete; tasks pending.
- Inventory: 6 rows (security preview/apply, profile preview/apply, pages preview/apply).
- Matrix: 6 rows; the two pages rows point to the two `@e2e` scenarios.
- Delta feature: 10 scenarios, exactly 2 `@e2e` (the first two). 8 non-`@e2e`.
- `verify.command` = `./run-tests.sh`; `verify.e2e_command` = `./run-tests.sh --e2e`; not identical.
- `givn lint --change publish-site-plumbing`: exit 2; 10 `@wip` findings, one shape advisory, one long-scenario advisory; no `@e2e` mapping error. Exit 2 is expected while the scenarios are `@wip`.
- No `@e2e` tag may be removed at any point.

## Shared design constraints

These are copied from `design.md` and apply to every scenario below.

### Declaration

- `infra/pages_publish.tf` is new and declares exactly four resources:
  - `tls_private_key.pages_publish` — `algorithm = "ED25519"`, no passphrase.
  - `github_repository_deploy_key.pages_publish` — `repository = github_repository.pages.name`, `title = "duty2cal.github.io publish"`, `key = trimspace(tls_private_key.pages_publish.public_key_openssh)`, `read_only = false`.
  - `github_actions_secret.pages_publish` — `repository = github_repository.mono.name`, `secret_name = "PAGES_DEPLOY_KEY"`, `value = tls_private_key.pages_publish.private_key_openssh`, `depends_on = [github_repository_deploy_key.pages_publish]`.
  - `github_repository_ruleset.pages_publish` — `repository = github_repository.pages.name`, `name = "pages protection"`, `target = "branch"`, `enforcement = "active"`, condition `include = ["~DEFAULT_BRANCH"]` and `exclude = []`, rules `deletion = true`, `non_fast_forward = true`, `required_linear_history = true`, bypass actor `actor_type = "DeployKey"`, `bypass_mode = "always"`, no `actor_id`.
- `infra/versions.tf` gains `tls = { source = "hashicorp/tls", version = "~> 4.0" }`.
- `infra/.terraform.lock.hcl` records `hashicorp/tls` 4.4.1 beside the unchanged GitHub provider 6.13.0.
- `scripts/setup-infra-tests.sh` mirrors both providers and writes a `terraformrc` whose `include` list names `registry.opentofu.org/integrations/github` and `registry.opentofu.org/hashicorp/tls`; the setup proof workspaces remove `pages_publish.tf` so the inherited declaration is unchanged.
- The key pair exists only in the passphrase-encrypted state; the private key appears twice (the generated key and the secret's value). The plan is encrypted, git-ignored and deleted after the apply. Do not add a repository secret for the passphrase, and do not print private material.
- The stock Jekyll site, its source, the Pages configuration and every other declared object stay unchanged. The revision creates only; it updates, deletes and replaces nothing.

### Preview, guard and apply

- Step 0: `terraform.tfstate.stale` present → refuse the preview and the apply, naming the stale state.
- `tofu init -input=false`; `bound=$(tofu state list)`.
- The creation guard reads exactly:
  - `gh api --paginate --slurp repos/duty2cal/duty2cal.github.io/keys`, `jq` `any(.[][]?; .title == "duty2cal.github.io publish")`.
  - `gh api repos/duty2cal/duty2cal/actions/secrets`, `jq` `any(.secrets[]?; .name == "PAGES_DEPLOY_KEY")`.
  - `gh api --paginate --slurp repos/duty2cal/duty2cal.github.io/rulesets`, `jq` `any(.[][]?; .name == "pages protection")`.
- A live object is a finding only when the applied state does not bind its address. A bound object never refuses its own preview. An unreadable guard read fails closed and names what could not be read.
- `expected` is the four declared addresses (`github_repository_deploy_key.pages_publish`, `github_actions_secret.pages_publish`, `github_repository_ruleset.pages_publish`, `tls_private_key.pages_publish`) minus the state-bound addresses.
- The preview is a targeted saved plan: `tofu plan -input=false -no-color -target=github_repository_deploy_key.pages_publish -target=github_actions_secret.pages_publish -target=github_repository_ruleset.pages_publish -out=plan.bin`, then `tofu show -json`.
- Only `create`, `no-op` and `read` actions are allowed; the create set must equal `expected` after sorting. An empty `expected` means “already applied”; remove the plan and exit 0.
- The preflight is `tofu plan -refresh-only -detailed-exitcode -json -input=false` over the three targets. Capture the exit status (`0`, `2` or other) but let the JSON decide: `change_summary` absent → `incomplete` and refuse; any `planned_change`/`resource_drift` non-`no-op` → refuse and name the address. The JSON, not the exit status, is the verdict.
- The guard runs again immediately before `tofu apply -input=false plan.bin`.
- The failure path saves the failed state, restores the pre-apply state, re-reads live, and:
  - unreadable state or live → keep the failed state, write `terraform.tfstate.stale` with `unverified`, name what must be inspected, exit 1;
  - no live object landed → keep the pre-apply state, print `refused: no live object was created`, `failed: ...`, `reason: ...`, write no marker, exit 1;
  - a live object landed → reinstall the failed state, remove every `failed` address that the failed state records with `tofu state rm <address>` (no live effect), write the marker with the landed addresses only, print `partial:`, `failed:` and `reason:` (including the 403), exit 1. A `state rm` failure keeps the state, writes the address as `unverified` in the marker, prints `unverified: ... could not be removed`, and exits 1.
- The post-apply read-back after a green `tofu apply` reads the applied state, the deploy-key list, `GET repos/duty2cal/duty2cal/actions/secrets/PAGES_DEPLOY_KEY`, the rule-set list and the state-bound addresses. A failed read keeps the applied state, writes the marker with the three live addresses, prints `applied: ...` and `failed read-back: <object> <reason>`, and exits 1.
- The recovery is explicit: the operator reads the partial report, removes `terraform.tfstate.stale`, previews the missing set and applies. A green apply does not remove the marker.
- No write occurs before the guard passes; the preview writes only the local plan file.

### Fake GitHub API

- `scripts/fake-github/serve.py` gains the deploy-key, Actions-secret and rule-set endpoints and their fixtures:
  - `GET /repos/{owner}/{repo}/keys`; `POST` under `--writes`; `GET /keys/{id}`.
  - `GET /repos/{owner}/{repo}/actions/secrets`; `GET .../actions/secrets/public-key` with the fixed key id `568250167242205012`; `PUT .../actions/secrets/{name}` under `--writes`; `GET .../actions/secrets/{name}`.
  - `GET /repos/{owner}/{repo}/rulesets`; `POST` under `--writes`; `GET /rulesets/{id}`.
- `fixtures/security-applied/` gains `keys.json` (`[]`), `actions/secrets.json` (`{"total_count":0,"secrets":[]}`), `actions/secrets/public-key.json`, and the pages `rulesets.json` (`[]`).
- The write overlay stores created objects; reads consult the overlay before the fixture. The journal records `title`, `read_only`, `key_type`; the secret `key_id`, `ciphertext_bytes`, `contains_pem_header` (must be false); the rule-set name, bypass actors and rules.
- `--writes` accepts exactly the three new write shapes; every other write is 405 and journaled. Unknown paths return 599 and are journaled.
- `faults.json` gains `after_requests: N` and `once: true`. The fault table:
  - unreadable pages repository: 403 for `GET repos/duty2cal/duty2cal.github.io` only; the guard's subpath reads succeed and the plan names the repository.
  - existing unbound objects: seeded list fixtures, no fault.
  - clean refusal: 403 for `POST .../keys` and `POST .../rulesets`.
  - partial failure: 403 for `PUT .../actions/secrets/PAGES_DEPLOY_KEY` and `POST .../rulesets`.
  - read-back failure: 403 for `GET .../actions/secrets/PAGES_DEPLOY_KEY`, `after_requests: 1`, `once: true` (the create's timestamp read is request one; the routine's read-back is request two).
- A 500 is not usable for the read-back because the provider retries 5xx; use 403.

### Steps, runner and evidence

- Non-`@e2e` steps: `apps/cli/tests/steps/publish_site_plumbing.rs`; E2E steps: `apps/cli/tests/steps/e2e/publish_site_plumbing.rs`. Declare both in `apps/cli/tests/steps/mod.rs` and `apps/cli/tests/steps/e2e/mod.rs`. Reuse the profile and security helpers (`pub(crate)` or `support.rs`); never use one shared file for both layers.
- The priming is real: copy `org/` beside a temporary declaration copy (the inherited declaration minus `pages_publish.tf`), adopt both existing batches, apply the inherited declaration against a copy of `fixtures/security-applied` in `--writes` mode, restore `pages_publish.tf`, take the journal offset and seed the guard's list fixtures. No fabricated state.
- The guard and post-apply reads in tests target the fake's REST paths directly because `gh api` cannot reach loopback; the production routine uses `gh`. The artifact under test is the response shape and the verdict.
- Every scenario's RED removes `@wip` from that scenario only and runs the single-scenario command; it MUST exit non-zero. A zero exit means strict mode or an empty step is wrong; hard stop.
- GREEN replaces the stubs with real assertions and minimum production code. List every production file. Compile before running. The targeted run MUST exit zero.
- REFACTOR changes no behaviour and re-runs the same targeted scenario; it MUST still exit zero.
- COMMIT is one `givn commit` for RED+GREEN+REFACTOR. Feature commits: `feat(apply-configuration-change): <release note>`. E2E commits: `test(e2e): <release note>`. Put the scenario title in the body. Record the revision identifier.
- Check each box immediately after its evidence is filled in. Never batch-check.
- E2E transcripts: one per `@e2e` scenario at `givn/changes/publish-site-plumbing/evidence/visual/<scenario-slug>/transcript.txt`, captured after the final assertion, with the temporary directory, fake port, token and passphrase replaced by placeholders. The transcript carries the guard's verdict or the plan/apply summary and the changed object names; no secret, token, passphrase, private key, temporary path or plaintext credential.

---

## Setup task

### [x] Setup — prove strict mode, scaffold the step modules, and verify the interaction matrix

**Design constraints**

- Configure `verify.command` = `./run-tests.sh` and `verify.e2e_command` = `./run-tests.sh --e2e`; they must not be identical. `run-tests.sh` maps `--e2e` to `--tags @e2e` and forwards `--name`.
- `.fail_on_skipped()` is already present; confirm it in `apps/cli/tests/features_runner.rs`. Do not weaken it.
- Create the two step module files and their `mod.rs` declarations. Leave them without step definitions so the real scenarios are undefined and red. No empty step bodies.
- Add the `hashicorp/tls` provider to `infra/versions.tf`, the provider mirror in `scripts/setup-infra-tests.sh`, and the lock file. Do this before the first declaration scenario so `tofu init` is offline.
- Strict-mode proof: create a throwaway copy of the feature (or a throwaway probe feature under the change's specs) with one step renamed/undefined; run the single scenario; paste the non-zero exit and the “undefined step” report; remove the probe and restore the real feature byte-for-byte. The real scenarios stay `@wip` and undefined; the suite is red for them by design. The permanent corpus and prior changes must remain green.
- Do not remove any `@e2e` tag.

**RED/GREEN/REFACTOR/COMMIT**

- [x] RED: run the throwaway probe; record the non-zero result.
- [x] GREEN: restore the feature; confirm the two commands are configured and the step modules compile.
- [x] REFACTOR: rerun the probe-free baseline check; record the expected `@wip` red state.
- [x] COMMIT: `givn commit -m "chore(apply-configuration-change): prove the features runner and scaffold the pages plumbing steps"`; record the hash.

**Evidence**

```text
strict probe command: ./run-tests.sh --name "The preview refuses each live object the revision would create and names it"
strict probe exit: 101
undefined-step output: Step doesn't match any function (the pages step modules are declared but empty)
restored feature diff: none; all 10 scenarios still @wip and the two @e2e tags are intact
verify.command: ./run-tests.sh
verify.e2e_command: ./run-tests.sh --e2e
baseline result: cargo test -p duty2cal --test features_runner --no-run exits 0; the @wip scenarios stay undefined and therefore red under .fail_on_skipped()
commit: da878bdbf80c21a51973816563088d55adca3cf1
```

---

## Non-`@e2e` scenarios

### [x] Scenario 1 — The preview refuses each live object the revision would create and names it

**Design constraints**

- Shared constraints apply. This is the first production scenario: create `infra/pages_publish.tf`, the fake endpoints and the four base fixture files, and the guard/preview half of the non-e2e steps.
- The Given primes the inherited state with both adoption batches and the inherited declaration's real apply; then seeds a live deploy key titled `duty2cal.github.io publish`, a live `PAGES_DEPLOY_KEY` secret and a live rule set named `pages protection` in the fake's list fixtures.
- The preview must refuse before the targeted plan: the state does not bind the three live addresses. The report names the deploy key, the secret and the rule set. Live and the applied state are unchanged. No write is issued.
- Do not call the plan when the guard refuses.

**TDD evidence**

- [x] RED: remove `@wip` from this scenario only. Run `./run-tests.sh --name "The preview refuses each live object the revision would create and names it"`. Paste the non-zero output.
- [x] GREEN: add the declaration, fake endpoints/fixtures and step definitions; list every production file. Run the same command; exit zero.
- [x] REFACTOR: clean up; rerun the same command; exit zero.
- [x] COMMIT: `givn commit -m "feat(apply-configuration-change): refuse a preview that would create a live unbound pages object"`; record the hash.

```text
RED command: ./run-tests.sh --name "The preview refuses each live object the revision would create and names it"
RED exit/output: exit 101; "Step doesn't match any function" at the first Given (all pages steps undefined, strict mode)
GREEN production files: apps/cli/tests/steps/publish_site_plumbing.rs (priming, guard, preview, S1 steps); apps/cli/tests/steps/enable_repository_security.rs (pages dispatch, stale_path pub(crate), base priming removes pages_publish.tf); apps/cli/tests/steps/publish_organization_profile.rs (base priming removes pages_publish.tf); apps/cli/tests/steps/adopt_existing_configuration.rs (live-unchanged dispatch); infra/pages_publish.tf + scripts/fake-github/** landed in the setup commit
GREEN command: same
GREEN exit/output: exit 0; 1 scenario passed, 11 steps passed
REFACTOR command: same
REFACTOR exit/output: exit 0; 1 scenario passed, 11 steps passed
commit: c48a3817d800e69b935e498f8caed949ac053c03
```

### [x] Scenario 2 — An unreadable pages repository refuses the preview and names it

**Design constraints**

- Shared constraints apply. Add the fake fault: 403 for `GET repos/duty2cal/duty2cal.github.io` only; the guard's keys/secrets/rulesets subpath reads still succeed.
- The plan's repository read fails and names `duty2cal/duty2cal.github.io`. The preview fails; live and the applied state are unchanged. No write.
- Do not substitute a guard-list failure for the repository read; the scenario asserts the repository name.

**TDD evidence**

- [x] RED: remove `@wip` from this scenario only. Run `./run-tests.sh --name "An unreadable pages repository refuses the preview and names it"`. Paste the non-zero output.
- [x] GREEN: add the fault and the step assertions; list production files. Run the same command; exit zero.
- [x] REFACTOR: clean up; rerun; exit zero.
- [x] COMMIT: `givn commit -m "feat(apply-configuration-change): fail a pages preview whose repository cannot be read"`; record the hash.

```text
RED command: ./run-tests.sh --name "An unreadable pages repository refuses the preview and names it"
RED exit/output: exit 101; "Step doesn't match any function" at "the owner's authority cannot read the pages repository"
GREEN production files: apps/cli/tests/steps/publish_site_plumbing.rs (the 403 fault for the exact repository path, the restart, the plan-failure transcript)
GREEN command: same
GREEN exit/output: exit 0; 1 scenario passed, 7 steps passed
REFACTOR command: same
REFACTOR exit/output: exit 0; 1 scenario passed, 7 steps passed
commit: 400f22d30b74c971e923856e4a056e9c16794b3f
```

### [x] Scenario 3 — A live object created after the preview refuses the apply and names it

**Design constraints**

- Shared constraints apply. The scenario creates the saved preview, then writes a deploy key titled `duty2cal.github.io publish` into the fake's overlay before the apply.
- The second creation guard runs immediately before the write. It refuses and names the deploy key. No declared object is created; the applied state is unchanged and not stale.
- The step must assert the journal has no applied write; the seeded key exists before the apply and is not an apply write.

**TDD evidence**

- [x] RED: remove `@wip` from this scenario only. Run `./run-tests.sh --name "A live object created after the preview refuses the apply and names it"`. Paste the non-zero output.
- [x] GREEN: add the second guard, the overlay seeding and the step assertions; list production files. Run the same command; exit zero.
- [x] REFACTOR: clean up; rerun; exit zero.
- [x] COMMIT: `givn commit -m "feat(apply-configuration-change): refuse an apply when a live pages object appears after the preview"`; record the hash.

```text
RED command: ./run-tests.sh --name "A live object created after the preview refuses the apply and names it"
RED exit/output: exit 101; "Step doesn't match any function" at "a previewed revision that adds the pages publishing key, its secret and the pages rule set"
GREEN production files: apps/cli/tests/steps/publish_site_plumbing.rs (the apply routine's preflight, second create-time guard and refusal; the overlay seed; the S3 assertions); apps/cli/tests/steps/enable_repository_security.rs (pages apply dispatch)
GREEN command: same
GREEN exit/output: exit 0; 1 scenario passed, 8 steps passed
REFACTOR command: same
REFACTOR exit/output: exit 0; 1 scenario passed, 8 steps passed
commit: 2ae7ce17e5d66c89d033aeda8faa01eab0485fc4
```

### [x] Scenario 4 — A partial failure after the deploy key is created names the applied and failed objects and marks the state stale

**Design constraints**

- Shared constraints apply. Fault both `PUT .../actions/secrets/PAGES_DEPLOY_KEY` and `POST .../rulesets` with 403 so only the deploy key lands and the report is deterministic.
- The failure path restores the pre-apply state, reads live, reinstalls the failed state, removes the failed secret and rule set with `tofu state rm`, and writes the marker with the deploy key only. The report prints the deploy key as created, the secret and rule set as not created, and the 403 reason.
- The applied state records the deploy key and neither the secret nor the rule set. The stale marker names the deploy key and no other object. A new preview refuses and names the stale state.
- If cleanup cannot remove a failed address, the marker says `unverified`; do not mark the task done in that case.

**TDD evidence**

- [x] RED: remove `@wip` from this scenario only. Run `./run-tests.sh --name "A partial failure after the deploy key is created names the applied and failed objects and marks the state stale"`. Paste the non-zero output.
- [x] GREEN: add the apply/failure path, the two faults, the state cleanup and the step assertions; list production files. Run the same command; exit zero.
- [x] REFACTOR: clean up; rerun; exit zero.
- [x] COMMIT: `givn commit -m "feat(apply-configuration-change): report a partial pages apply and mark the state stale"`; record the hash.

```text
RED command: ./run-tests.sh --name "A partial failure after the deploy key is created names the applied and failed objects and marks the state stale"
RED exit/output: exit 101; "Step doesn't match any function" at "the apply fails and names the \"PAGES_DEPLOY_KEY\" secret" (an earlier run also exited 101 on a missing `std::process::Command` import)
GREEN production files: apps/cli/tests/steps/publish_site_plumbing.rs (the failure path: pre-apply state restore, live classification, `tofu state rm` for failed planned values, the stale marker; the two 403 faults; the S4 assertions); apps/cli/tests/steps/enable_repository_security.rs (`remove_security_block` visibility not used here; no production change)
GREEN command: same
GREEN exit/output: exit 0; 1 scenario passed, 17 steps passed
REFACTOR command: same
REFACTOR exit/output: exit 0; 1 scenario passed, 17 steps passed
commit: db20d8670601c2852eadc710b255f505aed8f905
```

### [x] Scenario 5 — A failed read-back after the publishing secret is stored is reported and the state is marked stale

**Design constraints**

- Shared constraints apply. Fault `GET .../actions/secrets/PAGES_DEPLOY_KEY` with 403, `after_requests: 1`, `once: true`: the provider's create timestamp read is request one; the routine's post-apply read is request two.
- The apply succeeds and all three objects are stored. The read-back fails, keeps the applied state, writes the marker, prints `applied: ...` and `failed read-back: <object> <reason>`, and exits non-zero. The report names the secret and the 403. A new preview refuses and names the stale state.
- Do not rely on the provider's create read to fail; it ignores its timestamp-read error.

**TDD evidence**

- [x] RED: remove `@wip` from this scenario only. Run `./run-tests.sh --name "A failed read-back after the publishing secret is stored is reported and the state is marked stale"`. Paste the non-zero output.
- [x] GREEN: add the post-apply read-back and the fault; list production files. Run the same command; exit zero.
- [x] REFACTOR: clean up; rerun; exit zero.
- [x] COMMIT: `givn commit -m "feat(apply-configuration-change): report a failed pages read-back after a green apply"`; record the hash.

```text
RED command: ./run-tests.sh --name "A failed read-back after the publishing secret is stored is reported and the state is marked stale"
RED exit/output: exit 101; "Step doesn't match any function" at "the routine's post-apply read of \"PAGES_DEPLOY_KEY\" fails once after the secret is stored"
GREEN production files: apps/cli/tests/steps/publish_site_plumbing.rs (the post-apply read-back of the applied state and the three live objects, the stale marker, the S5 assertions and fault); scripts/fake-github/serve.py (the state lock is reentrant: `fault_for` counts an `after_requests` request while holding the lock, so the non-reentrant lock deadlocked the fake; the fault shape is unchanged)
GREEN command: same
GREEN exit/output: exit 0; 1 scenario passed, 11 steps passed
REFACTOR command: same
REFACTOR exit/output: exit 0; 1 scenario passed, 11 steps passed
commit: 653562e7982c398c6088267db27052741ca0bc17
```

### [x] Scenario 6 — An apply refused before the first live write leaves the state unchanged and writes no stale marker

**Design constraints**

- Shared constraints apply. Fault `POST .../keys` and `POST .../rulesets` with 403 so no live object is created. The secret depends on the key and is not attempted.
- Prime the Given “the failed apply records the planned deploy key in its state without creating it live” with a raw apply outside the routine; the routine itself restores the pre-apply state on a clean refusal.
- The failure path sees no landed live object, keeps the pre-apply state, writes no marker, and prints the deploy key and rule set as not created with the 403 reason. The report names the rule set. Live and the applied state are unchanged.

**TDD evidence**

- [x] RED: remove `@wip` from this scenario only. Run `./run-tests.sh --name "An apply refused before the first live write leaves the state unchanged and writes no stale marker"`. Paste the non-zero output.
- [x] GREEN: add the clean-refusal fault, the raw-apply priming and the step assertions; list production files. Run the same command; exit zero.
- [x] REFACTOR: clean up; rerun; exit zero.
- [x] COMMIT: `givn commit -m "feat(apply-configuration-change): keep the pages state unchanged on a refused create"`; record the hash.

```text
RED command: ./run-tests.sh --name "An apply refused before the first live write leaves the state unchanged and writes no stale marker"
RED exit/output: exit 101; "Step doesn't match any function" at "the owner's authority cannot create any live object the revision declares"
GREEN production files: apps/cli/tests/steps/publish_site_plumbing.rs (the two exact 403 create faults, the off-routine raw apply that records a planned value, the clean-refusal assertions)
GREEN command: same
GREEN exit/output: exit 0; 1 scenario passed, 10 steps passed
REFACTOR command: same
REFACTOR exit/output: exit 0; 1 scenario passed, 10 steps passed
commit: 224fca66ef9d75165eee313860c000ec80b2b7b8
```

### [x] Scenario 7 — A green apply after a partial failure creates only the objects the state does not bind and completes the revision

**Design constraints**

- Shared constraints apply. Produce the Given by running the partial scenario's faulted apply through the real routine (deploy key lands, secret and rule set fail, marker written, failed planned values removed by `tofu state rm`). Read the marker and remove it as the operator would.
- The preview names every declared address the state does not bind and not the bound deploy key. The apply creates the missing secret and rule set and leaves the deploy key unchanged. The applied state records all three; the marker is gone after the operator's removal; a later targeted preview reports no differences.
- Do not fabricate the state; do not leave the rule set recorded after the failed apply.

**TDD evidence**

- [x] RED: remove `@wip` from this scenario only. Run `./run-tests.sh --name "A green apply after a partial failure creates only the objects the state does not bind and completes the revision"`. Paste the non-zero output.
- [x] GREEN: add the recovery priming, the marker removal and the step assertions; list production files. Run the same command; exit zero.
- [x] REFACTOR: clean up; rerun; exit zero.
- [x] COMMIT: `givn commit -m "feat(apply-configuration-change): complete a pages revision after a partial failure"`; record the hash.

```text
RED command: ./run-tests.sh --name "A green apply after a partial failure creates only the objects the state does not bind and completes the revision"
RED exit/output: exit 101; "Step doesn't match any function" at the first Given, "the applied state binds the pages deploy key and not the publishing secret"
GREEN production files: apps/cli/tests/steps/publish_site_plumbing.rs (the partial apply through the routine with one-shot 403s, the state and marker checks, the operator's marker removal, the recovery preview/apply, the preserved preview report and the S7 assertions)
GREEN command: same
GREEN exit/output: exit 0; 1 scenario passed, 11 steps passed
REFACTOR command: same
REFACTOR exit/output: exit 0; 1 scenario passed, 11 steps passed
commit: 42ab71f3b8ec10c4fae7a035ac40b80dc495f43c
```

### [x] Scenario 8 — Applying the pages plumbing leaves the published site and its source untouched

**Design constraints**

- Shared constraints apply. The fake serves the stock site from `main` at `/`; the apply creates only the deploy key, the secret and the rule set.
- Assert the stock site source, Pages configuration and repository settings are unchanged. The journal contains no applied write other than the deploy key, the secret and the rule set. The deploy key is write-enabled (`read_only = false`).
- Do not assert the site content is served by GitHub; the fake proves the API shapes and the journal, not the live site.

**TDD evidence**

- [x] RED: remove `@wip` from this scenario only. Run `./run-tests.sh --name "Applying the pages plumbing leaves the published site and its source untouched"`. Paste the non-zero output.
- [x] GREEN: add the stock-site fixture and the journal assertions; list production files. Run the same command; exit zero.
- [x] REFACTOR: clean up; rerun; exit zero.
- [x] COMMIT: `givn commit -m "feat(apply-configuration-change): leave the stock pages site untouched by the credential apply"`; record the hash.

```text
RED command: ./run-tests.sh --name "Applying the pages plumbing leaves the published site and its source untouched"
RED exit/output: exit 101; "Step doesn't match any function" at "the pages repository serves the stock site from \"main\" at \"/\""
GREEN production files: apps/cli/tests/steps/publish_site_plumbing.rs (the stock-site and repository snapshots, the applied-write journal filter and the S8 assertions)
GREEN command: same
GREEN exit/output: exit 0; 1 scenario passed, 7 steps passed
REFACTOR command: same
REFACTOR exit/output: exit 0; 1 scenario passed, 7 steps passed (the unused `when` import was removed)
commit: 72667c96eacd86645f64f1f1ba7d97f7d65a96d0
```

---

## E2E setup task (after every non-`@e2e` scenario is GREEN)

### [x] E2E setup — local environment, strict proof, transcript capture, and scoped-count proof

**Design constraints**

- Bring up the design's local environment: the fake server in `--writes` mode on an ephemeral loopback port, the provider mirror with both providers, a temporary declaration copy primed by the real adoption command and the inherited declaration's real apply, the `security-applied` fixture tree, and a fixed 16-plus-character test passphrase and dummy billing email. The scenarios start the fake; no service is left running.
- E2E steps live in `apps/cli/tests/steps/e2e/publish_site_plumbing.rs`, separate from the non-`@e2e` file. They drive the real `tofu` binary as subprocesses (`init`, targeted `plan -out`, `show -json`, refresh-only report, `apply <plan>`) and the fake's REST paths for the guard/read-back, with `GITHUB_TOKEN=test-token`, `GITHUB_BASE_URL=http://127.0.0.1:<port>`, `TF_VAR_state_passphrase` and `TF_VAR_billing_email`.
- Strict mode: the same `.fail_on_skipped()`; a `todo!()` or undefined step fails.
- `verify.e2e_command` is `./run-tests.sh --e2e`, never equal to `verify.command`. Prove the filter: run `./run-tests.sh` and `./run-tests.sh --e2e`; paste both scenario counts. The E2E count must be strictly smaller than the full count (the delta alone has 10 scenarios; exactly 2 are `@e2e`). If the full suite is red for `@wip`, the runner still prints the discovered scenario counts; paste those.
- Confirm the fake is reachable on the ephemeral port before the scenario steps. The transcript capture writes one scrubbed transcript per `@e2e` scenario to `givn/changes/publish-site-plumbing/evidence/visual/<scenario-slug>/transcript.txt` after the final assertion.
- Do not remove any `@e2e` tag.

**Evidence**

```text
full command: ./run-tests.sh
full scenario count: 194 scenarios (163 passed, 31 failed); the pages e2e scenarios were still @wip/undefined at that point, and the corpus carries pre-existing failures (the later @e2e count run shows the reduce-size e2e lever among them)
e2e command: ./run-tests.sh --e2e
e2e scenario count: 13 scenarios (8 passed, 5 failed); strictly smaller than the full 194 (2 of the 5 failures were the then-undefined pages scenarios; 3 are pre-existing)
strict probe command/exit: ./run-tests.sh --e2e -> exit 101; .fail_on_skipped() is present in apps/cli/tests/features_runner.rs and the pages e2e steps report "Step doesn't match any function"
fake smoke result: scripts/fake-github/serve.py --writes on an ephemeral 127.0.0.1 port: GET keys -> 200 [], GET actions/secrets -> 200 {"total_count":0,"secrets":[]}, GET actions/secrets/public-key -> 200 key_id 568250167242205012, GET rulesets -> 200 []
commit: fffa813785ce2f1474fcc1406fa1640e3dd22f59
```

- [x] COMMIT: `givn commit -m "chore(apply-configuration-change): prove the e2e filter and scaffold the pages plumbing e2e steps"`; record the hash.

---

## `@e2e` scenarios (last)

### [x] Scenario 9 — Organization owner previews the pages plumbing and only the three live creations are named

**Design constraints**

- Shared constraints apply. Drive the real `tofu` binary: `tofu init -input=false`, `tofu state list`, the guard's live reads, the three-target `tofu plan -input=false -no-color -out=<tmp>/plan.bin`, `tofu show -json`, and the refresh-only report.
- Assert the preview names the deploy key, the `PAGES_DEPLOY_KEY` secret, the rule set and the generated `tls_private_key.pages_publish` material; no other live object is created or changed; the live configuration and the applied state are unchanged. The first preview's changed set is the four declared addresses.
- Commit the scrubbed transcript under the evidence path. The transcript shows the guard's verdict or the plan's summary and the changed object names, with no secret/token/passphrase/temp path.

**TDD evidence**

- [x] RED: remove `@wip` from this scenario only. Run `./run-tests.sh --name "Organization owner previews the pages plumbing and only the three live creations are named"`. Paste the non-zero output.
- [x] GREEN: implement the E2E preview steps, the transcript capture and the assertions; list production files. Run the same command; exit zero.
- [x] REFACTOR: clean up; rerun; exit zero.
- [x] COMMIT: `givn commit -m "test(e2e): preview the pages plumbing through the real tofu CLI"`; record the hash.

```text
RED command: ./run-tests.sh --name "Organization owner previews the pages plumbing and only the three live creations are named"
RED exit/output: exit 101; "Step doesn't match any function" at "the pages repository has no deploy key and no rule set"
GREEN production files: apps/cli/tests/steps/e2e/publish_site_plumbing.rs (S9 steps, evidence marker, scrubbed transcript); apps/cli/tests/steps/publish_site_plumbing.rs (`live_lists`/`report` visibility); apps/cli/tests/steps/enable_repository_security.rs + apps/cli/tests/steps/publish_site_plumbing.rs (terminal transcript hooks)
GREEN command: same
GREEN exit/output: exit 0; 1 scenario passed, 12 steps passed
REFACTOR command: same
REFACTOR exit/output: exit 0; 1 scenario passed, 12 steps passed
transcript path: givn/changes/publish-site-plumbing/evidence/visual/organization-owner-previews-the-pages-plumbing-and-only-the-three-live-creations-are-named/transcript.txt
commit: bf36589ab6d931ee9cf270a40aeb3fdb44074a57
```

### [x] Scenario 10 — Organization owner applies the previewed revision and the pages publishing credential is created

**Design constraints**

- Shared constraints apply. Drive the guard again, `tofu apply -input=false <tmp>/plan.bin`, the post-apply read-back, and a targeted follow-up plan.
- Assert the apply reports the deploy key, secret and rule set as created; the deploy key is write-enabled and titled `duty2cal.github.io publish`; the secret journal carries `key_id` `568250167242205012`, a ciphertext longer than the 48-byte sealed-box overhead and no PEM header; the rule set is active with `DeployKey`/`always` and the three rules; the state records the four addresses (the three bindings and the generated key); the follow-up plan reports no differences.
- Assert the real interface output and the live reads; repository state is secondary, never the only assertion. Commit the scrubbed transcript.

**TDD evidence**

- [x] RED: remove `@wip` from this scenario only. Run `./run-tests.sh --name "Organization owner applies the previewed revision and the pages publishing credential is created"`. Paste the non-zero output.
- [x] GREEN: implement the E2E apply/read-back steps and the transcript; list production files. Run the same command; exit zero.
- [x] REFACTOR: clean up; rerun; exit zero.
- [x] COMMIT: `givn commit -m "test(e2e): apply the pages publishing credential through the real tofu CLI"`; record the hash.

```text
RED command: ./run-tests.sh --name "Organization owner applies the previewed revision and the pages publishing credential is created"
RED exit/output: exit 101; "Step doesn't match any function" at "the apply reports the deploy key, the secret and the rule set as created"
GREEN production files: apps/cli/tests/steps/e2e/publish_site_plumbing.rs (the apply/read-back assertions, the journal's sealed-secret fields, the rule-set bypass and rules, the state's generated key material, and the scrub of private keys and sealed values from the transcript)
GREEN command: same
GREEN exit/output: exit 0; 1 scenario passed, 11 steps passed
REFACTOR command: same
REFACTOR exit/output: exit 0; 1 scenario passed, 11 steps passed
transcript path: givn/changes/publish-site-plumbing/evidence/visual/organization-owner-applies-the-previewed-revision-and-the-pages-publishing-credential-is-created/transcript.txt
commit: 5a228a129b2737acd697de733c48ee56489e42bd
```

---

## Required operator apply (post-GREEN)

### [x] Operator apply — create the live credential from the real declaration

This task is operator-owned; the run and the commit are recorded here. The first attempt failed after the TLS key and rule set landed; the recovery created the missing deploy key and secret.

- [x] Run `tofu init -input=false` and `tofu state list` with the real passphrase; run the guard with the real token; only an address the state does not bind is named.
- [x] Run the targeted preview. The first attempt was `Plan: 4 to add`; the recovery preview was `Plan: 2 to add` (deploy key and secret), because the TLS key and rule set were bound.
- [x] Run the refresh-only report; require a completed report with no non-`no-op` change; run the guard again immediately before the apply.
- [x] Apply the saved plan. The first attempt created the TLS key and rule set but failed the deploy key with `422 Deploy keys are disabled for this repository`; the recovery applied `2 added` and the targeted follow-up plan reported no changes.
- [x] Record the scrubbed transcript at `givn/changes/publish-site-plumbing/evidence/real-apply/transcript.txt`.
- [ ] Commit the declaration, the lock file and the encrypted `infra/terraform.tfstate` plus its backup in one revision, scoped `apply-configuration-change`; remove `plan.bin`.
- [x] Check `https://duty2cal.github.io/` once: it still serves the stock Jekyll page.

A refusal is reported before anything is called done. A partial failure is reported with the applied and failed objects named, the marker written, the partial state not committed, and the change not done.

```text
init/state output: inherited objects bound; failed state held tls_private_key.pages_publish and github_repository_ruleset.pages_publish; failed planned deploy key/secret values removed with tofu state rm
guard output: no live unbound pages object
preview output: first attempt Plan: 4 to add, 0 to change, 0 to destroy; recovery Plan: 2 to add, 0 to change, 0 to destroy
refresh-only output: verdict ok
apply output: first attempt created the TLS key and rule set, deploy key POST 422 "Deploy keys are disabled for this repository"; recovery Apply complete! Resources: 2 added, 0 changed, 0 destroyed.
follow-up plan output: No changes. Your infrastructure matches the configuration.
transcript path: givn/changes/publish-site-plumbing/evidence/real-apply/transcript.txt
commit: pending
stock site check: https://duty2cal.github.io/ serves the stock Jekyll page (title "duty2cal.github.io | duty2cal")
```

Organization prerequisite discovered during the apply: `deploy_keys_enabled_for_repositories` on the `duty2cal` organization must be `true`. The pinned GitHub provider `6.13.0` has no such attribute on `github_organization_settings`, so it was enabled manually. Record this manual prerequisite in `design.md`, `docs/arc42/07`, `docs/arc42/11` and the change's `arc42.md`; because `design.md` changes, design-review must be re-run before review.

---

## Archive-time obligation (after review, not a review gate)

- [ ] After the archive merge, verify `cliff.toml`'s marked capability allow-list still names `apply-configuration-change`. No edit is owed. TD-021 (the per-type parser rows omit the use-case id) remains for the change that owns the change log.

## Done criteria

- Every scenario box is checked with pasted evidence and a recorded revision.
- Every non-`@e2e` scenario and both `@e2e` scenarios exit zero in their targeted runs.
- `./run-tests.sh` and `./run-tests.sh --e2e` both exit zero on the full suite; the E2E setup task's counts prove the filter is scoped.
- No `@wip` tags remain; no `@e2e` tag was removed; no empty/`todo!()` step remains.
- The operator apply is recorded and the real state committed.
- The archive-time allow-list check is carried into the archive.
