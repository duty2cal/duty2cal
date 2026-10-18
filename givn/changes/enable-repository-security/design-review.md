# Design review — `enable-repository-security`

Review of the full planning layer (proposal, delta use case and feature, design,
arc42 assessment) before the task list is written. The grilling report
(`SecurityGrill`, 15 ranked questions G1–G15, 2026-10-18) was dispositioned by
the user on 2026-10-18; every disposition is applied below. No `tasks.md` was
written and nothing was implemented.

- **Review basis:** `proposal.md`, `specs/apply-configuration-change/{usecase.md,enable-repository-security.feature}`,
  `design.md`, `arc42.md`; the live evidence under `evidence/live-read/`
  (captured for this review, 2026-10-18); the topic question log; `docs/arc42/**`;
  the provider `v6.13.0` source and the adoption verifier's JSON lesson.
- **Grilling verdict at intake:** NOT READY (G1–G4 blocking/high).

## Question log and dispositions

Severities and branch references are the grilling report's. Each disposition was
decided by the user on 2026-10-18; rejected alternatives are recorded.

### G1 — Saved-plan semantics versus the use case's live-change and revision rules

- **Severity:** Blocker (P1); branches 3, 7, 8.
- **Question:** should the apply enforce the live-change and revision-mismatch
  refusals, or should the use case be amended to the saved plan?
- **Disposition:** **Option 2.** Keep the saved plan as the applied revision; the
  routine runs a read-only `tofu plan -refresh-only -detailed-exitcode -json`
  immediately before `tofu apply plan.bin` and refuses a non-no-op change by
  address. The revision-mismatch rule is reworded to “the saved plan is the
  previewed revision”; a declaration edit after the preview is not carried.
  **The plan JSON is authoritative, not the exit status:** OpenTofu 1.13.1
  returns exit 2 for a no-op (opentofu/opentofu#3015), so only a completed
  `change_summary` with no non-no-op `planned_change`/`resource_drift`
  continues — the same rule `scripts/adopt-public-home.sh` already applies.
  No wrapper command; the wrapper that re-plans and applies stays rejected.
- **Sources:** use case main flow 4, extensions, rules, examples, skeletons;
  `design.md` “The apply procedure”, “Decisions”, traceability, risks, obstacles;
  `docs/arc42/02`, `06`, `08`, `11` (R-031); feature scenario “A live edit
  between the preview and the apply is refused and named”.
- **Rejected:** Option 1 (withdraw the no-silent-overwrite rule), Option 3 (an
  apply wrapper command).
- **Residual:** a live edit made *during* the preflight; recorded as R-031 and
  bounded to seconds, not a human pause.

### G2 — Mark the partial applied state stale

- **Severity:** Blocker (P1); branches 3, 8, 9.
- **Question:** how is the partial state marked stale, or is the guarantee
  withdrawn?
- **Disposition:** **Option 1.** The failure path writes
  `infra/terraform.tfstate.stale` beside the state, but only when the state
  bytes changed (a refusal before the first write leaves them identical, so no
  marker); the preview and the apply refuse while it exists; a green apply
  removes it. The sidecar is git-ignored and never committed. The use case,
  minimal guarantee, glossary, R-029 and TD-013 now state this; the partial
  feature scenario asserts the marker and the refusing next preview.
- **Sources:** use case extensions/rules/guarantee/skeleton; feature partial
  scenario; `design.md` file table, apply routine, risks; `docs/arc42/02`,
  `06`, `07`, `08`, `11`, `12`; proposal guarantee.
- **Rejected:** Option 2 (withdraw the seed guarantee; on-disk state stays
  indistinguishable), Option 3 (transactional apply; impossible per-resource).

### G3 — Prime the security scenarios with a real apply of the base declaration

- **Severity:** Blocker (P1); branches 4, 5, 7.
- **Question:** should the scenarios reproduce the inherited applied state,
  including provider state-only fields, before asserting the single change?
- **Disposition:** **Option 1.** After adopting the base batch against
  `security-applied`, the step removes `security.tf` and the two-line
  `security_and_analysis` block (the base declaration), runs that declaration's
  real `init`/`plan`/`apply`, then restores the security declaration and adopts
  the second batch. The base apply records the one-time state-only
  `ignore_vulnerability_alerts_during_read = false` per repository. The preview
  step then asserts the exact changed live set; the setup proof is extended to
  both batches plus the base-declaration apply.
- **Sources:** `design.md` “Read fixtures”, “Test seam”, step-definition table,
  interaction matrix; feature preview Given “that applied state was produced by
  the base declaration's own apply”.
- **Rejected:** Option 2 (test-only state edit), Option 3 (weaken the “only”
  assertion).

### G4 — Enforce “adopted before updated”

- **Severity:** High (P1); branches 2, 3, 8.
- **Question:** should the preview/apply refuse a plan that creates a setting
  that already exists live and is unbound?
- **Disposition:** **Option 1.** A preview guard reads the preview's own plan
  JSON (`tofu show -json plan.bin`): its refreshed prior state for
  `github_repository.mono` carries both live Dependabot values, so a `create`
  for `github_repository_vulnerability_alerts.mono` or
  `github_repository_dependabot_security_updates.mono` while the setting is
  already live is refused by address before any write. A new scenario proves
  the skip refusal and names both settings.
- **Partial-failure reachability (explicit):** with the guard in place the
  skipped-adoption shape cannot pass the preview, so the partial scenario is
  labelled and documented as the **off-routine skip path**: it produces the
  plan without the routine's guard, applies it, and writes the failure-path
  sidecar. The route still adopts first; TD-013 records the guard.
- **Sources:** use case main flow 2, extensions, rules, example; feature skip
  and partial scenarios; `design.md` guard bullet, routine, matrix, TD-013,
  `docs/arc42/06` and chapter 11.
- **Rejected:** Option 2 (procedural only), Option 3 (unrecorded off-routine
  probe).

### G5 — Reconcile the earlier apply, the live-read base and the “first” claim

- **Severity:** High (P1/P2); branches 1, 5, 7, 16.
- **Question:** should the plan record the earlier apply and derive
  `security-applied` from a post-apply live read, and which change is “first”?
- **Disposition:** **Option 1.** Captured an authenticated post-apply read on
  2026-10-18 under `evidence/live-read/` (org, both repositories, topics, pages,
  rulesets, both vulnerability-alerts reads, both automated-security-fixes
  reads, classic protection): projects off, rebase-only, delete-branch on,
  downloads off; pages issues/projects/wiki off; org display name set; scanning
  disabled; Dependabot updates on for the monorepo. `security-applied` is
  derived from and checked against it; the proposal now says “the security
  change after the earlier configuration change”; the topic Q9 resolution
  records the classic protection's removal by hand on 2026-10-18 (404 on the
  protection read). No write was issued.
- **Sources:** `evidence/live-read/**` (including its README), proposal scope and
  opportunity, `design.md` file table and live-read section, `docs/arc42/07`,
  `givn/ideation/public-home-infrastructure/questions.md` Q9.
- **Rejected:** Option 2 (state-derived fixture without a live read), Option 3
  (`adopt-all` base, which would widen the visible change).

### G6 — Assert the preview's “only changes” exactly

- **Severity:** Medium (P2); branches 4, 5, 6.
- **Question:** should the preview step assert the plan's changed resource and
  attribute set, or should the Then be relaxed?
- **Disposition:** **Option 1.** The step parses the saved plan's JSON and
  asserts the only changed resource is `github_repository.mono` and the only
  changed live attributes are `secret_scanning` and
  `secret_scanning_push_protection`; the one-time state-only
  `ignore_vulnerability_alerts_during_read` is classified as bookkeeping and
  allowed, any other live attribute fails.
- **Sources:** feature preview Thens; `design.md` interaction matrix, fixtures,
  traceability.
- **Rejected:** Option 2 (relaxed Then), Option 3 (human review only).

### G7 — Export the fake environment for the manual walkthrough

- **Severity:** Medium (P2); branches 4, 16.
- **Question:** should the walkthrough export the fake environment for
  `tofu init/plan/apply`, or be replaced?
- **Disposition:** **Option 1.** The walkthrough exports `GITHUB_BASE_URL`,
  `GITHUB_TOKEN`, `TF_CLI_CONFIG_FILE` (the path the setup script prints),
  `TF_VAR_state_passphrase` and `TF_VAR_billing_email`, then runs the same
  routine; it never relies on an ambient token or the public registry.
- **Sources:** `design.md` “Local runnability”, “Interface obstacles”.
- **Rejected:** Option 2 (remove the walkthrough), Option 3 (a new script).

### G8 — Add the read-back-failure scenario

- **Severity:** Medium (P2); branches 3, 6.
- **Question:** should a scenario drive the PATCH-succeeds/read-back-fails path?
- **Disposition:** **Option 1.** New scenario: the fake applies the PATCH, the
  provider's read-back then returns 500 once (`after_applied_writes: 1`,
  `once: true`); the apply exits non-zero and names the repository, live
  carries both statuses, the state does not claim them, and the next preview
  reads successfully and names the two statuses.
- **Sources:** feature read-back scenario; `design.md` fault table, apply
  routine, traceability, obstacles, risks; `docs/arc42/06`.
- **Rejected:** Option 2 (remove the branch), Option 3 (fold into partial).

### G9 — Prove “wrote nothing else”

- **Severity:** Low/Medium (P3); branches 4, 5.
- **Question:** should the journal record the full PATCH body or changed keys,
  or should the claim be weakened?
- **Disposition:** **Option 1 (+ the overlay assertion).** The journal entry
  carries the scrubbed request body's sorted top-level keys plus the full
  `security_and_analysis` object; the step asserts the key set equals the
  fixture's and only the nested member differs, and the follow-up read asserts
  the overlay equals the fixture plus the two enabled statuses. Nothing is
  weakened.
- **Sources:** feature apply Thens; `design.md` write sandbox, matrix, obstacles.
- **Rejected:** Option 3 (weaken the claim); Option 2 alone (overlay only).

### G10 — Close Q5 and record the classic-protection removal

- **Severity:** Medium (P2); branches 9, 15.
- **Question:** should the delta use case mark Q5 resolved and record the manual
  classic-protection removal?
- **Disposition:** **Option 1.** The use case's Q5 line and the examples caption
  now state the settled values (Q5 closed 2026-10-17) with their source; the
  classic protection is recorded as removed by hand on 2026-10-18 and verified
  by this change (use case out-of-scope, D5, chapter 11 read context), and the
  topic Q9 resolution carries the superseding update.
- **Sources:** use case rules/examples/out-of-scope; proposal D5; `design.md`
  D5; `questions.md` Q9; `docs/arc42/11`.
- **Rejected:** Option 2 (link-only), Option 3 (leave contradictory).

### G11 — Applied-revision ADR verdict and the second verdict's evidence

- **Severity:** Low (P3); branches 13, 14.
- **Question:** should the applied-revision decision get a qualification verdict,
  and should the second verdict's evidence match its FAIL status?
- **Disposition:** **Option 1.** Added a fourth NOT_QUALIFIED verdict for “the
  applied revision is the committed state plus the declaration revision”, routed
  to `design.md` with the use case's rule in evidence (register unchanged). The
  saved-plan verdict's `durable_consequence` evidence array is now empty (FAIL
  is not a pass fact); the residual-window fact and the architectural-impact
  reason are labelled as FAIL reasons in prose.
- **Sources:** `arc42.md` verdicts and chapter 9 row; no chapter 09 edit.
- **Rejected:** Option 2 (fold in), Option 3 (new ADR).

### G12 — R-026 and the push-protection fallback

- **Severity:** Medium (P2); branches 7, 16.
- **Question:** is “a refusal aborts the change” the accepted outcome, or should
  secret scanning and push protection be split?
- **Disposition:** **Option 1.** Keep R-026, record the public-repository
  documentation evidence (secret scanning runs automatically for free on public
  repositories; push protection is disabled by default and enableable by a
  repository administrator, organization owner, security manager or enterprise
  owner — checked 2026-10-18) and that a refusal **stops the change** and
  returns the question; the post-GREEN apply now has a read-only preflight
  (live field, admin permission, documentation). No split.
- **Sources:** `design.md` risks and post-GREEN step 2; `docs/arc42/11` R-026.
- **Rejected:** Option 2 (split the change), Option 3 (GHAS; not applicable to a
  public repository).

### G13 — Correct the future path for non-provider patterns and validity checks

- **Severity:** Low (P3); branches 2, 16.
- **Question:** should the plan record the provider write/read gap?
- **Disposition:** **Option 1.** Proposal D2 and design D2 now state that
  6.13.0's `calculateSecurityAndAnalysis` sends only advanced security, secret
  scanning and push protection, and `secret_scanning_validity_checks` is absent
  from the schema; a later change needs a provider upgrade or another mechanism.
  R-027 records the same.
- **Sources:** proposal D2, design D2, `docs/arc42/11` R-027.
- **Rejected:** Option 2 (upgrade now), Option 3 (leave misleading wording).

### G14 — Crew-member Persona

- **Severity:** Low (P3); branches 8, 12.
- **Question:** is the crew member a non-impact parent lens, or should the
  delta list only the confirmed Personas?
- **Disposition:** **Option 1.** The use case and design state that the crew
  member is a stakeholder lens of the parent use case, not a Persona confirmed
  for this change and not a Gherkin Actor, and records no impact. No Persona is
  invented or promoted.
- **Sources:** use case Personas; `design.md` Persona impact and traceability.
- **Rejected:** Option 2 (remove without explanation), Option 3 (confirm a new
  Persona).

### G15 — Implementation commit scope and changelog intent

- **Severity:** Low (P3); branches 16.
- **Question:** should the plan name the commit scope and whether the change
  appears in the changelog?
- **Disposition:** **Option 1.** Implementation and archive commits use
  `apply-configuration-change` (the use-case id), e.g.
  `feat(apply-configuration-change): enable secret scanning and push
  protection`. `infra/` is outside the marked user-receiving paths, so the
  change is intentionally absent from `CHANGELOG.md` unless that list is widened
  in a separate decision. The archive-time allow-list obligation is unchanged.
- **Sources:** `design.md` “Post-GREEN operator apply”, “Archive-time
  obligation”, decisions.
- **Rejected:** Option 2 (rely on the archive, commit scope filtered), Option 3
  (widen the paths here).

## Edits applied

| File | Changes |
|---|---|
| `proposal.md` | “first configuration change” replaced by “the security change after the earlier configuration change” and the post-apply read cited; partial-failure bullet names the stale marker; D2 records the 6.13.0 write gap. |
| `specs/apply-configuration-change/usecase.md` | Main flow 2/4: preview guard and saved-plan/preflight checks; preconditions gain the stale marker; extensions rewritten for the saved-plan revision rule, preflight refusal, stale refusal, incomplete report, unadopted preview refusal; rules rewritten (saved plan, preflight and JSON authority, stale marker, Q5 settled, classic protection); examples corrected (edited declaration, unadopted setting, live edit, read-back failure, stale partial); skeletons updated; minimal guarantee names the sidecar; Personas reconcile the crew-member lens; out-of-scope records the classic removal. |
| `specs/apply-configuration-change/enable-repository-security.feature` | Eight scenarios: preview and apply strengthened (base-apply primed Given, exact changed resource/live attributes, journal body keys, no other declared field); new live-change, unadopted-setting skip, and read-back-failure scenarios; first-change-refused asserts no stale marker; partial scenario labelled as the off-routine skip path and asserts the stale marker plus the refusing next preview. No `@e2e` removed. |
| `design.md` | Apply mechanism/state rows; file table (`terraform.tfstate.stale`, `.gitignore`, live-read evidence, fixture derivation); live-read section with the 2026-10-18 evidence and classic 404; executable apply routine with stale check, plan-JSON guard (`jq`), refresh-only preflight (JSON authoritative) and state-byte failure path; partial reachability rewritten; fixtures and base-apply priming; setup proof; write sandbox journal body/overlay and manual-edit seam; faults (`once`, read-back, live change); wip count; step-definition table; decisions (preflight, guard, sidecar, commit scope; D2/D5 corrected); traceability rows; Persona impact; matrix; black-box count; manual walkthrough environment; obstacles; risks; post-GREEN apply; archive obligation and changelog intent. |
| `arc42.md` | Chapter 7/8/9/11/12 rows updated; four ADR candidates (applied-revision verdict added, canonical home `design.md`); saved-plan verdict's FAIL evidence moved to labelled prose reasons; register unchanged. |
| `docs/arc42/02-architecture-constraints.md` | Saved-plan constraint now names the read-only preflight and the stale marker. |
| `docs/arc42/06-runtime-view.md` | Runtime diagram and steps gain the guard, preflight, read-back refusal and stale marker; notes updated. |
| `docs/arc42/07-deployment-view.md` | Security-change bullet names the earlier change, the preflight and the stale marker. |
| `docs/arc42/08-crosscutting-concepts.md` | “Apply semantics” now covers the saved-plan revision rule, the JSON-authoritative preflight, the preview guard and the stale marker. |
| `docs/arc42/11-risks-and-technical-debt.md` | R-026 (docs evidence, read-only preflight, refusal stops the change), R-027 (write gap), R-029 (sidecar), R-031 (preflight window) and TD-013 (guard) updated. |
| `docs/arc42/12-glossary.md` | **Configuration change** behaviour and source columns name the preflight and `infra/terraform.tfstate.stale`; promotion prose updated. |
| `givn/ideation/public-home-infrastructure/questions.md` | Q9 resolution carries the 2026-10-18 update: classic protection removed by hand; this change verifies it. |
| `givn/changes/enable-repository-security/evidence/live-read/**` | New authenticated read of 2026-10-18 (org, both repositories, topics, pages, rulesets, vulnerability alerts, automated security fixes, classic protection 404) plus `README.md` with commands, values, fixture mapping and the no-write statement. |

## Scenario changes

| Scenario | Tag | Change |
|---|---|---|
| Organization owner previews the security settings and only the declared differences are named | `@givn.added @e2e @wip` | Base-apply primed; exact changed resource and live-attribute assertions added |
| Organization owner applies the previewed revision and the security settings are enabled | `@givn.added @e2e @wip` | Journal body key set and “no other declared field” assertions added |
| An unreadable repository setting refuses the preview and names the repository | `@givn.added @wip` | unchanged |
| An apply whose first change is refused changes nothing | `@givn.added @wip` | asserts the state is not marked stale |
| A live edit between the preview and the apply is refused and named | `@givn.added @wip` | **new** |
| The preview refuses to add a Dependabot setting that already exists live and is unbound | `@givn.added @wip` | **new** |
| A failed read-back after the PATCH is reported and the state does not claim the statuses | `@givn.added @wip` | **new** |
| A partial failure reports the applied settings and names the failed one | `@givn.added @wip` | labelled off-routine skip path; stale marker and refusing next preview asserted |

The interaction inventory (two entries) and the matrix (two rows) still agree;
the matrix's driving mechanisms now carry the exact-set, body-key and preflight
assertions. No `@e2e` tag was removed.

## Verification

- `givn lint --change enable-repository-security`: exit **2**, eight `@wip`
  findings, **no errors**. Exit 2 is the documented `@wip` advisory state.
- `givn status`: stage `arc42-docs-done`; `design-review` is the next required
  artifact and is completed by this file.
- No GitHub write was issued; the live read used read-only `gh api` commands.
- No `tofu apply`, no state write, no commit, no `tasks.md`.
- The open questions that remain are the topic-level ones the change carries
  (Q2, Q3, Q10, Q11), each with an owner and out-of-scope disposition; none
  blocks the task list.

## Verdict

Every grilling question G1–G15 is resolved and every hardening edit is applied;
the lint has no errors. The plan is ready for the task list.

DESIGN-REVIEW: PASS
