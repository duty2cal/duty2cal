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
  verdict, and the stale marker on partial failure. The failure path keeps a
  pre-apply state copy, restores it after a non-zero apply, and runs the
  read-only refresh-only report: no drift keeps the restored state and writes
  no marker; drift reinstalls the failed apply's state and writes the marker;
  an incomplete report keeps the restored state, writes no marker and reports
  the applied state as unverified.
  This is because SDKv2 persists the planned values for a refused update and
  OpenTofu persists a refreshed envelope, so neither bytes nor values of the
  failed state are evidence of a write. Corrected during implementation; see
  `design.md` "The apply procedure" and the decision record.
- Fake: `--writes` sandbox, the `security-applied` base tree, the read-back
  fault, the journal body/keys, and the one-import-per-object proof for both
  batches.
- `scripts/setup-infra-tests.sh` covers the second batch and the base apply.
- `.gitignore` covers the stale marker and plan files; `jq` is checked.
- CI runs the setup script before the suite.

Evidence:
- [x] Strict-mode proof: run one `@wip` scenario; paste the non-zero output.
      `./run-tests.sh --name "A live edit between the preview and the apply is
      refused and named"` → exit 101; the third step is undefined
      (`Step doesn't match any function`) and the run ends `1 scenario
      (1 failed)`. The base-apply priming steps before it passed.
- [x] Step files, modules, world fields; `cargo check -p duty2cal --tests`.
      `apps/cli/tests/steps/enable_repository_security.rs`, the empty e2e file,
      both `mod.rs` files, and `DutyWorld::{plan_path, preview_transcript,
      apply_transcript, write_sandbox}`; `cargo check` exits 0.
- [x] Fixture proof: base apply + second batch + the security reads; paste.
      `./scripts/setup-infra-tests.sh` exits 0: both trees' five base imports,
      `refresh-only plan (security-applied-adopted): No changes.`, the base
      declaration's apply (`0 live changes`, follow-up plan `No changes.`), and
      `refresh-only plan (security-applied-security-adopted): No changes.`;
      unknown paths served: 0. `jq 1.8.2` is checked.
- [x] Routine and fake write sandbox compile/run; paste the smoke output.
      The preview/apply routine runs as real `tofu` subprocesses in scenario 1
      (below); the fake's `--writes` sandbox served the base apply's PATCH and
      scenario 2's refused PATCH.

## 1. An unreadable repository setting refuses the preview and names the repository

Design constraints:
- The fake returns 401/403 for the monorepo's security read; the preview fails,
  names `duty2cal/duty2cal`, sends no write, and leaves the state unchanged.
- The routine's preflight and plan are the real `tofu` subprocess.

- [x] RED — remove `@wip` from this scenario only; stubbed steps; run the single
      scenario; paste the non-zero output.
      The decisive Then is undefined (`Step doesn't match any function`);
      `1 scenario (1 failed)`, runner exit 101.
- [x] GREEN — real assertions and minimum production code; list production
      files; run the single scenario; paste the zero output.
      Production files: `infra/security.tf`, `infra/repos.tf`,
      `infra/adoption-batch-security.txt`, `.gitignore`,
      `scripts/fake-github/serve.py`, the two fixture trees,
      `scripts/setup-infra-tests.sh`,
      `apps/cli/tests/steps/enable_repository_security.rs`, both `mod.rs` files,
      `apps/cli/tests/features_runner.rs`. Single scenario exits 0:
      `1 scenario (1 passed)`, 5 steps passed.
- [x] REFACTOR — cleanup; re-run; paste the zero output.
      `cargo fmt`; `cargo check -p duty2cal --tests` exits 0; the single
      scenario still exits 0 with `1 scenario (1 passed)`.
- [x] COMMIT: `ec38b47` — `feat(apply-configuration-change): <release note>` with the
      scenario title in the body; record the revision.
      Revision: `ec38b47` — `feat(apply-configuration-change): enable secret
      scanning and push protection` (carries the setup task and this scenario).

## 2. An apply whose first change is refused changes nothing

Design constraints:
- The fake refuses the monorepo PATCH (401/403); the apply fails naming the
  repository; live and the pre-apply state's values/bytes unchanged (the
  routine restores the state); no stale marker; a later preview names the same
  change.
- The saved plan is applied only after the preflight.

- [x] RED — remove `@wip`; run the single scenario; paste non-zero.
      The decisive Then ("the applied state is not marked stale") is
      undefined (`Step doesn't match any function`); `8 steps (7 passed,
      1 failed)`, runner exit 101.
- [x] GREEN — assertions and production code; list files; paste zero.
      Production files: `apps/cli/tests/steps/enable_repository_security.rs`
      (refused-PATCH fault, restore-and-verify failure path, later preview),
      `apps/cli/tests/steps/adopt_existing_configuration.rs` (writes-aware fake
      launcher), `apps/cli/tests/features_runner.rs`; `design.md` and this file
      for the corrected failure path. Single scenario exits 0: `9 steps
      (9 passed)`, 1 scenario passed.
- [x] REFACTOR — cleanup; re-run; paste zero.
      `cargo fmt`; `cargo check -p duty2cal --tests` exits 0; the single
      scenario still exits 0 with `1 scenario (1 passed)`.
- [x] COMMIT: `abd3bc2` — release note subject, scenario title body; record the revision.
      Revision: `abd3bc2` — `feat(apply-configuration-change): enable secret
      scanning and push protection`.
      The failure path was corrected during this scenario: a refused PATCH
      still makes SDKv2 persist the planned values, so the routine restores
      the pre-apply state and keeps it only when the read-only refresh-only
      report shows no drift (design.md "The apply procedure").

## 3. A live edit between the preview and the apply is refused and named

Design constraints:
- A manual `has_projects` toggle after the saved plan is detected by the
  refresh-only preflight; the apply refuses before any write and names the
  monorepo; the saved plan is not applied; a new preview names the edit.
- The preflight's plan JSON is the verdict (opentofu#3015).

- [x] RED — remove `@wip`; run the single scenario; paste non-zero.
      Before the steps existed: `3 steps (2 passed, 1 failed)`; the manual-edit
      Given was `Step doesn't match any function`; `1 scenario (1 failed)`,
      runner exit 101.
- [x] GREEN — assertions and production code; list files; paste zero.
      Production files: `apps/cli/tests/steps/enable_repository_security.rs`
      (loopback `PATCH` with `X-Fake-Manual-Edit`, the preflight refusal naming
      the drifted address with its live id, the single-manual-edit journal
      assertion, the unapplied saved plan, the re-preview naming
      `has_projects`), the feature file (`@wip` removed). Single scenario exits
      0: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR — cleanup; re-run; paste zero.
      `cargo fmt`; `cargo check -p duty2cal --tests` exits 0; the single
      scenario still exits 0 with `8 steps (8 passed)`.
- [x] COMMIT: `e830590` — release note subject, scenario title body; record the revision.
      Revision: `e830590` — `feat(apply-configuration-change): enable secret
      scanning and push protection`.

## 4. The preview refuses to add a Dependabot setting that already exists live and is unbound

Design constraints:
- The state binds the base batch but not the two Dependabot resources; live has
  both enabled; the preview refuses and names both resources; no write; state
  unchanged.
- The guard reads live and the plan's `to add` set.

- [x] RED — remove `@wip`; run the single scenario; paste non-zero.
      Before the steps existed: the first Given was `Step doesn't match any
      function`; `1 scenario (1 failed)`, `1 step (1 failed)`, runner exit 101.
- [x] GREEN — assertions and production code; list files; paste zero.
      Production files: `apps/cli/tests/steps/enable_repository_security.rs`
      (base-batch-only priming split from the second batch, the shared preview
      `When` no longer re-primes the state, live Dependabot reads, the
      declaration assertion, the refusal assertions), the feature file (`@wip`
      removed). Single scenario exits 0: `1 scenario (1 passed)`,
      `9 steps (9 passed)`.
      Corrected during implementation: the plan's refreshed prior state does
      not carry `security_and_analysis.dependabot_security_updates` — provider
      6.13.0's repository schema has no such member and flattens only three
      settings (measured 2026-10-18). Both Dependabot creates are therefore
      judged by the live `vulnerability_alerts` flag, which proves the
      automated-security-fixes object exists live. See `design.md` "The apply
      procedure", the decision record and the `design-review.md` correction.
- [x] REFACTOR — cleanup; re-run; paste zero.
      `cargo fmt`; `cargo check -p duty2cal --tests` exits 0; the single
      scenario still exits 0 with `9 steps (9 passed)`.
- [x] COMMIT: `7287552` — release note subject, scenario title body; record the revision.
      Revision: `7287552` — `feat(apply-configuration-change): enable secret
      scanning and push protection`.

## 5. A failed read-back after the PATCH is reported and the state is marked stale

Design constraints:
- The fake applies the PATCH, then faults the read-back GET once; the apply fails
  naming the monorepo; live has the statuses enabled; the state may carry the
  planned statuses and is marked stale; a new preview refuses and names the stale
  state.
- The journal proves the write happened.

- [x] RED — remove `@wip`; run the single scenario; paste non-zero.
      Before the steps existed: the read-back Given was `Step doesn't match any
      function`; `1 scenario (1 failed)`, `3 steps (2 passed, 1 failed)`,
      runner exit 101.
- [x] GREEN — assertions and production code; list files; paste zero.
      Production files: `apps/cli/tests/steps/enable_repository_security.rs`
      (the `after_applied_writes: 1` + `once` read-back fault, the live
      scanning read, the stale-marker assertion, the refusing re-preview), the
      feature file (`@wip` removed). Single scenario exits 0:
      `1 scenario (1 passed)`, `8 steps (8 passed)`.
      Corrected during implementation: a `once` status-500 fault is consumed by
      the provider's 5xx retry and the apply succeeds (measured 2026-10-18), so
      the fault uses a non-retried status (403) that fails the read-back
      exactly once. See `design.md` "Fault injection".
- [x] REFACTOR — cleanup; re-run; paste zero.
      `cargo fmt`; `cargo check -p duty2cal --tests` exits 0; the single
      scenario still exits 0 with `8 steps (8 passed)`.
- [x] COMMIT: `e19691d` — release note subject, scenario title body; record the revision.
      Revision: `e19691d` — `feat(apply-configuration-change): enable secret
      scanning and push protection`.

## 6. A partial failure reports the applied settings and names the failed one

Design constraints:
- The off-routine skip state (no Dependabot bindings); the PATCH enables the
  statuses and the vulnerability alerts, the security-updates PATCH fails; the
  report names the applied and failed settings; the state records the applied
  resources; `infra/terraform.tfstate.stale` is written; the next preview
  refuses naming the stale state.
- The scenario is labelled as the off-routine probe, not the routine.

- [x] RED — remove `@wip`; run the single scenario; paste non-zero.
      Before the steps existed: the skipped-batch Given was `Step doesn't match
      any function`; `1 scenario (1 failed)`, `1 step (1 failed)`, runner exit
      101.
- [x] GREEN — assertions and production code; list files; paste zero.
      Production files: `apps/cli/tests/steps/enable_repository_security.rs`
      (the off-routine `init`/`plan -out`/`apply -parallelism=1` helper and its
      failure-path stale sidecar, the refused automated-security-fixes fault,
      the applied/failed-setting report assertions, the state-address
      assertions, the refusing next preview), the feature file (`@wip`
      removed). Single scenario exits 0: `1 scenario (1 passed)`,
      `8 steps (8 passed)`.
- [x] REFACTOR — cleanup; re-run; paste zero.
      `cargo fmt`; `cargo check -p duty2cal --tests` exits 0; the shared
      stale-preview assertion was factored into one helper; the single scenario
      still exits 0 with `8 steps (8 passed)`.
- [x] COMMIT: `6a0f096` — release note subject, scenario title body; record the revision.
      Revision: `6a0f096` — `feat(apply-configuration-change): enable secret
      scanning and push protection`.

## E2E setup task — before the first @e2e scenario

Design constraints:
- `verify.e2e_command` is `./run-tests.sh --e2e`, strictly smaller than the full
  run; prove the counts.
- E2E steps in `apps/cli/tests/steps/e2e/enable_repository_security.rs`; the base
  apply and the fake write sandbox start/stop per scenario; transcripts are
  committed under the change's evidence.

Evidence:
- [x] Full and e2e scenario counts pasted; e2e strictly smaller.
      `./run-tests.sh` over the assembled corpus runs 140 scenarios (138
      passed, 2 failed — the two undefined `@e2e` scenarios); `./run-tests.sh
      --e2e` runs 6 scenarios (4 passed, 2 failed). 6 < 140. Final counts are
      under "Final verification".
- [x] Strict-mode proof for the e2e runner; paste non-zero.
      `./run-tests.sh --e2e` exits 101 with `Step doesn't match any function`
      for both `@e2e` scenarios and ends `6 scenarios (4 passed, 2 failed)`;
      the single-scenario run of scenario 7 exits 101 with `1 scenario
      (1 failed)`, `1 step (1 failed)`.
- [x] The fake write sandbox is reached on loopback before the steps are written.
      The baseline e2e run's scenario 8 passed its priming Given, its previewed
      revision Given and its apply When against the ephemeral loopback fake in
      `--writes` mode (the `github_repository.mono` update landed); only its
      undefined Then failed. Scenario 7's priming and preview served every read
      from the same fake (no unknown-path 599). The steps live in
      `apps/cli/tests/steps/e2e/enable_repository_security.rs`; the base apply
      and the fake start/stop per scenario are the shared priming helper, the
      fake being killed with the scenario's world. Transcripts are committed
      under `evidence/visual/<slug>/transcript.txt`.

## 7. Organization owner previews the security settings and only the declared differences are named (@e2e)

Design constraints:
- The state is primed by the base declaration's own apply; the security
  declaration adds only the two statuses; the preview names exactly
  `github_repository.mono` with exactly the two live attributes; no create or
  delete; no write reaches the fake.
- The plan JSON's changed set is asserted, with the state-only attribute
  classified.

- [x] RED — remove `@wip` from this scenario only; run the single scenario;
      paste non-zero.
      The first Given is undefined (`Step doesn't match any function`);
      `1 scenario (1 failed)`, `1 step (1 failed)`, runner exit 101.
- [x] GREEN — assertions and production code; list files; paste zero.
      Production files: `apps/cli/tests/steps/e2e/enable_repository_security.rs`
      (the primed Given, the base apply's serial proof, the live disabled read,
      the exact plan-text and plan-JSON assertions, the transcript),
      `apps/cli/tests/steps/enable_repository_security.rs` (`pub(crate)`
      helpers and the base apply in the scenario transcript), the feature file
      (`@wip` removed). Single scenario exits 0: `1 scenario (1 passed)`,
      `10 steps (10 passed)`.
      Corrected during implementation: the base apply's state-only
      `ignore_vulnerability_alerts_during_read` is not shown by `tofu state
      show` and its state JSON value is null, so provenance is asserted from
      the raw state's serial — seven imports plus the base apply make 8, one
      write beyond the seven bound objects (`tofu state pull`).
- [x] REFACTOR — cleanup; re-run; paste zero.
      `cargo fmt`; `cargo check -p duty2cal --tests` exits 0; the single
      scenario still exits 0 with `10 steps (10 passed)`.
- [x] COMMIT: `52b8c26` — `test(e2e): <release note>` with the scenario title in the body;
      record the revision.
      Revision: `52b8c26` — `test(e2e): enable secret scanning and push
      protection`; transcript committed under
      `evidence/visual/organization-owner-previews-the-security-settings-and-only-the-declared-differences-are-named/transcript.txt`.

## 8. Organization owner applies the previewed revision and the security settings are enabled (@e2e)

Design constraints:
- The apply reports the monorepo changed and exits 0; the fake reports both
  statuses enabled; the state records them; the journal's repository request
  changed only `security_and_analysis` with both statuses; no other declared
  field changed; a later preview is empty.
- The routine's preflight runs first; the saved plan is applied unchanged.

- [x] RED — remove `@wip` from this scenario only; run the single scenario;
      paste non-zero.
      The shared priming Given, previewed revision Given and apply When pass;
      the first e2e Then is undefined (`Step doesn't match any function`);
      `1 scenario (1 failed)`, `4 steps (3 passed, 1 failed)`, runner exit 101.
- [x] GREEN — assertions and production code; list files; paste zero.
      Production files: `apps/cli/tests/steps/e2e/enable_repository_security.rs`
      (the apply outcome and preflight order, the state read, the journal's
      single applied write, the declared-field overlay, the later preview, the
      transcript), the feature file (`@wip` removed), and
      `scripts/fake-github/fixtures/security-applied/repos/duty2cal/duty2cal.json`
      (`allow_update_branch: false`, the live read's value the fixture omitted).
      Single scenario exits 0: `1 scenario (1 passed)`, `9 steps (9 passed)`.
      Corrected during implementation: the provider's `PATCH` carries its
      update schema's fields, not the whole API body — 21 top-level keys
      including the create-only `auto_init`, `gitignore_template` and
      `license_template`, and it reorders `topics`. The journal assertion
      therefore accepts those three provider-only keys, and the live assertion
      compares the declared fields with `topics` as a set; the journaled
      `security_and_analysis` and the two live status differences carry the
      "only" claim. See `design.md` "Write sandbox" and the `design-review.md`
      correction.
- [x] REFACTOR — cleanup; re-run; paste zero.
      `cargo fmt`; the fixture path was factored into one helper; `cargo check
      -p duty2cal --tests` exits 0; the single scenario still exits 0 with
      `9 steps (9 passed)`.
- [x] COMMIT: `a8fbd40` — `test(e2e): <release note>` with the scenario title in the body;
      record the revision.
      Revision: `a8fbd40` — `test(e2e): enable secret scanning and push
      protection`; transcript committed under
      `evidence/visual/organization-owner-applies-the-previewed-revision-and-the-security-settings-are-enabled/transcript.txt`.

## Final verification and operator apply

- [x] Every scenario box checked with pasted output and a revision.
      Scenarios 1-8 carry their RED/GREEN/REFACTOR outputs and the revisions
      `ec38b47` (setup + 1), `abd3bc2`, `e830590`, `7287552`, `e19691d`,
      `6a0f096`, `52b8c26` and `a8fbd40`.
- [x] `./run-tests.sh` exits 0 over the full suite.
      140 scenarios (140 passed), 584 steps (584 passed), exit 0.
- [x] `./run-tests.sh --e2e` exits 0 with a strictly smaller count.
      6 scenarios (6 passed), 42 steps (42 passed), exit 0; 6 < 140.
- [x] `givn lint --change enable-repository-security` has no errors.
      `givn lint: 1 file(s) checked — clean`, exit 0.
- [x] Post-GREEN operator step: the maintainer adopts the second batch, runs the
      security preview, applies the saved plan, and commits the state and backup;
      the live API reports secret scanning and push protection enabled and the
      targeted security preview is empty. Operator run 2026-10-18:
      the adoption command refused the second batch because its whole-state
      refresh saw a pre-existing organization billing drift; both Dependabot
      resources were imported directly (`tofu import … duty2cal`, both
      successful, ids 1416075332 and duty2cal). The targeted plan named only
      `github_repository.mono` (two statuses enabled) and applied
      (`0 added, 1 changed, 0 destroyed`); the targeted plan then exited 0; the
      live API reports both statuses enabled; the state and backup are in commit
      `877753f`. The full preview still names the organization billing drift
      (a separate, pre-existing issue); the adoption command's whole-state
      verification is a recorded defect: it must scope the refresh to the batch
      addresses.
- **Archive-time obligation (not a task box):** when the archive creates
      `givn/specs/apply-configuration-change/`, add the capability to
      `cliff.toml` and its parser rows in the same revision.
