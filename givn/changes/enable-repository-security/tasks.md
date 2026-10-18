# Tasks: enable-repository-security

Release note (every scenario commit carries it as the message subject; the
scenario title goes in the body):
Secret scanning and push protection are enabled on the public repository and the
Dependabot security settings are brought under the reviewed configuration, so a
credential can no longer enter the public history unnoticed.

Commit scope: `apply-configuration-change` (the use-case id). `infra/` is not a
user-receiving path, so this change is intentionally absent from `CHANGELOG.md`
unless the path list is changed in a separate decision.

## Setup task — routine, fake writes, base-apply state, fixtures

Design constraints:
- Runner `./run-tests.sh`; e2e `./run-tests.sh --e2e`; strict
  `fail_on_skipped()`; single scenario `./run-tests.sh --name "<title>"`.
- Step files: `apps/cli/tests/steps/enable_repository_security.rs` and
  `apps/cli/tests/steps/e2e/enable_repository_security.rs`; declare both in the
  `mod.rs` files; add the world fields (declaration, saved plan, state, stale
  marker, fake write sandbox, transcript).
- Declaration: `infra/security.tf` with `security_and_analysis` on the monorepo
  (`secret_scanning`, `secret_scanning_push_protection`, both enabled), the
  dedicated `github_repository_vulnerability_alerts.mono` and
  `github_repository_dependabot_security_updates.mono` resources, and
  `infra/adoption-batch-security.txt`.
- Apply routine: the design's preflight (stale marker, unadopted-setting guard),
  saved-plan application, refresh-only preflight with the plan JSON as the
  verdict, and the stale marker on partial failure.
- Fake: `--writes` sandbox, the `security-applied` base tree, the read-back
  fault, the journal body/keys, and the one-import-per-object proof for both
  batches.
- `scripts/setup-infra-tests.sh` covers the second batch and the base apply.
- `.gitignore` covers the stale marker and plan files; `jq` is checked.
- CI runs the setup script before the suite.

Evidence:
- [ ] Strict-mode proof: run one `@wip` scenario; paste the non-zero output.
- [ ] Step files, modules, world fields; `cargo check -p duty2cal --tests`.
- [ ] Fixture proof: base apply + second batch + the security reads; paste.
- [ ] Routine and fake write sandbox compile/run; paste the smoke output.

## 1. An unreadable repository setting refuses the preview and names the repository

Design constraints:
- The fake returns 401/403 for the monorepo's security read; the preview fails,
  names `duty2cal/duty2cal`, sends no write, and leaves the state unchanged.
- The routine's preflight and plan are the real `tofu` subprocess.

- [ ] RED — remove `@wip` from this scenario only; stubbed steps; run the single
      scenario; paste the non-zero output.
- [ ] GREEN — real assertions and minimum production code; list production
      files; run the single scenario; paste the zero output.
- [ ] REFACTOR — cleanup; re-run; paste the zero output.
- [ ] COMMIT — `feat(apply-configuration-change): <release note>` with the
      scenario title in the body; record the revision.

## 2. An apply whose first change is refused changes nothing

Design constraints:
- The fake refuses the monorepo PATCH (401/403); the apply fails naming the
  repository; live and state unchanged; no stale marker; a later preview names
  the same change.
- The saved plan is applied only after the preflight.

- [ ] RED — remove `@wip`; run the single scenario; paste non-zero.
- [ ] GREEN — assertions and production code; list files; paste zero.
- [ ] REFACTOR — cleanup; re-run; paste zero.
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## 3. A live edit between the preview and the apply is refused and named

Design constraints:
- A manual `has_projects` toggle after the saved plan is detected by the
  refresh-only preflight; the apply refuses before any write and names the
  monorepo; the saved plan is not applied; a new preview names the edit.
- The preflight's plan JSON is the verdict (opentofu#3015).

- [ ] RED — remove `@wip`; run the single scenario; paste non-zero.
- [ ] GREEN — assertions and production code; list files; paste zero.
- [ ] REFACTOR — cleanup; re-run; paste zero.
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## 4. The preview refuses to add a Dependabot setting that already exists live and is unbound

Design constraints:
- The state binds the base batch but not the two Dependabot resources; live has
  both enabled; the preview refuses and names both resources; no write; state
  unchanged.
- The guard reads live and the plan's `to add` set.

- [ ] RED — remove `@wip`; run the single scenario; paste non-zero.
- [ ] GREEN — assertions and production code; list files; paste zero.
- [ ] REFACTOR — cleanup; re-run; paste zero.
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## 5. A failed read-back after the PATCH is reported and the state does not claim the statuses

Design constraints:
- The fake applies the PATCH, then faults the read-back GET once; the apply fails
  naming the monorepo; live has the statuses enabled; the state does not record
  them; a later preview names them.
- The journal proves the write happened.

- [ ] RED — remove `@wip`; run the single scenario; paste non-zero.
- [ ] GREEN — assertions and production code; list files; paste zero.
- [ ] REFACTOR — cleanup; re-run; paste zero.
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## 6. A partial failure reports the applied settings and names the failed one

Design constraints:
- The off-routine skip state (no Dependabot bindings); the PATCH enables the
  statuses and the vulnerability alerts, the security-updates PATCH fails; the
  report names the applied and failed settings; the state records the applied
  resources; `infra/terraform.tfstate.stale` is written; the next preview
  refuses naming the stale state.
- The scenario is labelled as the off-routine probe, not the routine.

- [ ] RED — remove `@wip`; run the single scenario; paste non-zero.
- [ ] GREEN — assertions and production code; list files; paste zero.
- [ ] REFACTOR — cleanup; re-run; paste zero.
- [ ] COMMIT — release note subject, scenario title body; record the revision.

## E2E setup task — before the first @e2e scenario

Design constraints:
- `verify.e2e_command` is `./run-tests.sh --e2e`, strictly smaller than the full
  run; prove the counts.
- E2E steps in `apps/cli/tests/steps/e2e/enable_repository_security.rs`; the base
  apply and the fake write sandbox start/stop per scenario; transcripts are
  committed under the change's evidence.

Evidence:
- [ ] Full and e2e scenario counts pasted; e2e strictly smaller.
- [ ] Strict-mode proof for the e2e runner; paste non-zero.
- [ ] The fake write sandbox is reached on loopback before the steps are written.

## 7. Organization owner previews the security settings and only the declared differences are named (@e2e)

Design constraints:
- The state is primed by the base declaration's own apply; the security
  declaration adds only the two statuses; the preview names exactly
  `github_repository.mono` with exactly the two live attributes; no create or
  delete; no write reaches the fake.
- The plan JSON's changed set is asserted, with the state-only attribute
  classified.

- [ ] RED — remove `@wip` from this scenario only; run the single scenario;
      paste non-zero.
- [ ] GREEN — assertions and production code; list files; paste zero.
- [ ] REFACTOR — cleanup; re-run; paste zero.
- [ ] COMMIT — `test(e2e): <release note>` with the scenario title in the body;
      record the revision.

## 8. Organization owner applies the previewed revision and the security settings are enabled (@e2e)

Design constraints:
- The apply reports the monorepo changed and exits 0; the fake reports both
  statuses enabled; the state records them; the journal's repository request
  changed only `security_and_analysis` with both statuses; no other declared
  field changed; a later preview is empty.
- The routine's preflight runs first; the saved plan is applied unchanged.

- [ ] RED — remove `@wip` from this scenario only; run the single scenario;
      paste non-zero.
- [ ] GREEN — assertions and production code; list files; paste zero.
- [ ] REFACTOR — cleanup; re-run; paste zero.
- [ ] COMMIT — `test(e2e): <release note>` with the scenario title in the body;
      record the revision.

## Final verification and operator apply

- [ ] Every scenario box checked with pasted output and a revision.
- [ ] `./run-tests.sh` exits 0 over the full suite.
- [ ] `./run-tests.sh --e2e` exits 0 with a strictly smaller count.
- [ ] `givn lint --change enable-repository-security` has no errors.
- [ ] Post-GREEN operator step: the maintainer adopts the second batch, runs the
      security preview, applies the saved plan, and commits the state and backup;
      the live API reports secret scanning and push protection enabled and a
      later preview is empty. (Awaits the operator; the real apply is the proof
      for R-026.)
- [ ] Archive-time obligation: when the archive creates
      `givn/specs/apply-configuration-change/`, add the capability to
      `cliff.toml` and its parser rows in the same revision.
