# Design review: publish-site-plumbing

- **Change:** `publish-site-plumbing` (stage `arc42-docs-done`).
- **Reviewer context:** fresh; this reviewer did not author the proposal,
  specification, design or arc42 chapters. The grilling report was produced
  independently at `/tmp/publish-site-plumbing-grilling.md`; this record applies
  its findings and the operator's answers.
- **Inputs read:** `proposal.md`, `specs/apply-configuration-change/{publish-site-plumbing.feature,usecase.md}`,
  `design.md`, `arc42.md`; the permanent `apply-configuration-change` use case
  and the two inherited capabilities; all 12 `docs/arc42/` chapters, the ADR
  index and ADR-0006; the pinned provider and repository claims named in the
  grilling report; the current worktree after the release 0.5.0 merge.
- **Operator decisions:** Q1 = option 1 (add the explicit post-apply read-back);
  Q2 = option 1 (the operator removes the stale marker deliberately; the next
  green apply completes the revision).

## 1. Branch walk

| Branch | Verdict | Evidence |
|---|---|---|
| Scope — spec equals proposal, nothing extra or missing | **PASS** | The preview now names the generated key material as well as the three live creations; the apply records all four state addresses; the second creation guard, the applied/failed report lines and the read-back failure have scenarios. No publish, Pages configuration, rotation or review gate was added. |
| Scope — "done" unambiguous | **PASS** | `design.md` "Post-GREEN operator apply (required)": the suite is not done at GREEN; the real apply, the read-back, the transcript and the commit are required. |
| Tech choices — stack fit | **PASS** | OpenTofu 1.13.1, GitHub provider 6.13.0, TLS provider 4.4.1; the ephemeral `tls_private_key` alternative is now recorded with the reason it does not remove the state copy. |
| Tech choices — simpler alternatives | **PASS** | Hand-generated variables and external scripts remain rejected; the fake extension is the minimum seam; the routine reuses `gh`/`jq`/`tofu`. |
| Missing scenarios — proposal-implied behaviour | **PASS** | The generated key is asserted in the preview and the apply state; the second guard has its own scenario; failure reports assert applied and failed objects. |
| Missing scenarios — error paths | **PASS** | Partial failure is deterministic (both the secret `PUT` and the rule-set `POST` faulted); clean refusal blocks every declared create; read-back failure targets the routine's own post-apply read; recovery priming is documented. |
| Testability — strict mode | **PASS** | `features_runner.rs` `.fail_on_skipped()`; ten undefined `@wip` scenarios are red; the rename-a-step proof remains an implementation task. |
| Testability — not-implemented stub pattern | **PASS** | All ten scenarios are `@givn.added @wip`; no step is defined. |
| Testability — single-scenario command | **PASS** | `./run-tests.sh --name "…"` forwards `--name` to cucumber-rs; the chosen title has no regex metacharacters. |
| Testability — RED with no production code | **PASS** | Undefined steps fail; the routine's failure paths now name the applied and failed objects and the reason. |
| Testability — concrete Then values | **PASS** | The secret Then asserts the fetched-key ciphertext and the absence of a plaintext key; the stock-site Then asserts the journal's applied-write set; failure Thens assert object names and a 403. |
| E2E fidelity — action scope | **PASS** | Two normalized actions (preview, apply); exactly two `@e2e` scenarios; the eight other scenarios are variants and error paths. |
| E2E fidelity — real driver | **PASS WITH NOTE** | Real `tofu` subprocesses. The guard and read-back test reads target the fake directly because `gh` cannot reach loopback; the production routine still uses `gh`, and the substitution is disclosed in `design.md` "Step definitions". |
| E2E fidelity — primary assertion | **PASS** | Preview asserts the named differences; apply asserts the apply outcome plus live reads and the state. |
| E2E fidelity — evidence | **PASS** | One scrubbed transcript per `@e2e` scenario; the terminal baseline adaptation is now stated. |
| E2E fidelity — non-e2e does not duplicate e2e | **PASS** | Recovery, refusal, guard and read-back scenarios assert outcomes the two e2e scenarios do not. |
| Interaction Coverage | **PASS** | The matrix has all six inventory entries: the four inherited security/profile rows with their existing `@e2e` titles and driving mechanisms, plus the two pages rows. |
| Risk — most likely implementation failure | **PASS** | The secret create/read seam is resolved: the fault targets the routine's read-back, and the provider fact is corrected. The partial rule set is blocked by a second fault, and the reinstalled state is cleaned of failed planned values (F17). |
| Use-case scope rejection criteria | **PASS** | Goal, Actor, trigger, placement, extensions, flow size and relationships pass; the stale-marker precondition/extension/guarantee contradiction is resolved per Q2. |
| Persona coverage and dispositions | **PASS** | Three confirmed Personas; dispositions unchanged; no invented Persona; Actors remain interaction roles. |
| Decision ledger | **PASS** | D1 resolved; D2/D3 carried with owner "the user" and seed references; no unresolved proposal question. |
| Visual design contract | **PASS** | Interface terminal, viewports n/a, no screen; the terminal baseline adaptation names the expected Terraform addresses, the forbidden secrets/paths and the error-state rule. |
| Deferrals | **PASS** | No mandatory check deferred. The post-GREEN real apply is part of done; TD-021 is recorded debt with a named future owner. |
| Interface classification — opposite test | **PASS** | A Web UI classification fails: nothing renders HTML/CSS/JS, the consumer is the real `tofu` CLI, and GitHub is an external REST system. |
| ADR qualification | **PASS** | AMEND_ADR to ADR-0006 is unchanged. The rule-set verdict now names exactly one canonical artifact (the capability specification). The key-generation decision has a structured NOT_QUALIFIED/CANONICAL_ARTIFACT verdict routed to `design.md`. |
| Architecture documentation | **PASS** | All 12 rows independently assessed Yes; all chapter files exist with substantive content; no box-drawing or pseudo-diagram; ADR-0006 is in chapter 09 and the index; risk consequences are in chapter 11. |
| Ubiquitous language | **PASS** | **Deploy key**, **Protection ruleset** and the new **Pages publishing plumbing** term are in the glossary; **Configuration change** now carries the corrected stale-marker contract. |
| `givn lint` | **PASS (exit 2 allowed)** | Ten `@wip` findings, one shape advisory pairing the unreadable-repository scenarios, one long-scenario advisory on the partial failure scenario. No `@e2e` mapping error. |
| Merge reconciliation | **PASS** | The release 0.5.0 merge left duplicate risk IDs and stale QS/TD references. The plumbing risks were renumbered to R-051..R-055, the plumbing quality scenario to QS-034 and the changelog gap to TD-021; release 0.5.0 content was preserved. |

## 2. Findings and resolution

### F1 (H) — preflight aborted under `set -e`

**Status:** resolved.

The routine now captures the `-detailed-exitcode` status:

```sh
preflight_status=0
tofu plan -refresh-only -detailed-exitcode -json ... >preflight.json ||
  preflight_status=$?
```

`preflight_verdict` is computed from the JSON and defaults to `incomplete` if
`jq` fails; the refusal names the verdict and the status. A comment and the
bullet "The preflight's verdict is the JSON report, not the exit status" state
that the JSON, not the exit status, decides, including OpenTofu 1.13.1's
spurious no-op exit 2.

**Evidence:** `design.md` "The apply procedure" and its bullet list.

### F2 / Q1 (H) — read-back scenario was not producible; provider fact was wrong

**Status:** resolved per Q1 option 1.

- The routine reads the applied state and the three live objects back after a
  green `tofu apply`; a failed read prints `failed read-back: <object>
  <reason>`, keeps the applied state, writes `terraform.tfstate.stale` with the
  three addresses and exits non-zero.
- The feature fault targets the routine's post-apply read: the secret create's
  timestamp read is request one and succeeds; the routine's read is request two
  and fails.
- The provider fact now says the create makes one timestamp read and ignores
  that read's failure; SDKv2 performs no second read after create.
- The failure names `github_actions_secret.pages_publish (PAGES_DEPLOY_KEY)`.

**Evidence:** `design.md` provider-facts table and "The apply procedure";
`feature` scenario "A failed read-back after the publishing secret is stored is
reported and the state is marked stale"; `docs/arc42/06-runtime-view.md` step 6
and notes; `docs/arc42/02-architecture-constraints.md` read-back constraint;
`docs/arc42/08-crosscutting-concepts.md` apply semantics; `docs/arc42/11-risks-and-technical-debt.md` R-054.

### F3 (H) — second creation guard untested

**Status:** resolved.

New non-`@e2e` scenario: "A live object created after the preview refuses the
apply and names it". It seeds a deploy key into the fake's overlay after the
saved preview and asserts the apply refuses, names the deploy key, creates no
declared object, leaves the applied state unchanged and writes no stale marker.
The fixture section documents the seeding point, and the traceability preflight
row cites the scenario.

**Evidence:** `feature`; `design.md` "Fixtures", "Step definitions" and
"Use-case traceability".

### F4 (M) — partial failure was nondeterministic

**Status:** resolved.

The partial scenario now also faults `POST /repos/duty2cal/duty2cal.github.io/rulesets`.
Only the deploy key lands; the scenario asserts the rule set is absent, the
marker names only the deploy key, the report names the deploy key as created
and the secret and rule set as not created, and the report states a 403. The
fault table records both writes. The failure path removes the failed addresses
from the reinstalled state (F17), so the state assertions are true.

**Evidence:** `feature` partial-failure scenario; `design.md` "Fault injection".

### F5 (M) — clean-refusal Given understated the fault

**Status:** resolved.

The Given now says the authority cannot create any live object the revision
declares; the Then asserts the report names the deploy key and the rule set as
not created. The design's two-write fault is retained.

**Evidence:** `feature` clean-refusal scenario; `design.md` "Fault injection".

### F6 (M) — generated key material was not asserted

**Status:** resolved.

The preview scenario adds "And the preview names the generated pages
publishing key material as an object to create"; the apply scenario adds "And
the applied state records the generated pages publishing key material". The
design's four-address claims (traceability, matrix, chapters 06 and 10) now
match the feature.

**Evidence:** `feature`; `design.md` "Use-case traceability" and "Interaction
Coverage Matrix"; `docs/arc42/06-runtime-view.md` notes;
`docs/arc42/10-quality-requirements.md` QS-034.

### F7 (M) — failure scenarios did not assert applied/failed objects or reason

**Status:** resolved.

The partial scenario asserts the report names the deploy key as created, the
secret and rule set as not created, and a 403. The read-back scenario asserts
the report names the deploy key, secret and rule set as applied and a 403. The
routine prints `applied:`, `partial:`, `failed:` and `reason:` lines.

**Evidence:** `feature` partial and read-back scenarios; `design.md` failure
path.

### F8 (M) — recovery state had no production mechanism

**Status:** resolved.

The step-definition section now states that the recovery scenario's partial
state is produced by running the partial scenario's faulted apply through the
real routine, reading the marker, removing it as the operator would, then
running the preview and apply without the faults. The routine's state cleanup
(F17) leaves the state binding the deploy key and the generated key only, so
the recovery Given and the expected set hold. No state is fabricated.

**Evidence:** `design.md` "Step definitions".

### F9 / Q2 (M) — stale-marker contract contradicted itself

**Status:** resolved per Q2 option 1.

Every occurrence now says the preview and the apply refuse while the marker
exists, the operator removes it after reading the report, and the next green
apply completes the revision. The delta use case's precondition, stale
extension, rule, example and minimal guarantee; the glossary; chapters 02, 06,
08 and 11; and the design traceability row were edited. The routine no longer
removes the marker on a green apply; recovery is the operator's removal.

**Evidence:** `specs/apply-configuration-change/usecase.md`;
`docs/arc42/12-glossary.md`; `docs/arc42/02-architecture-constraints.md`;
`docs/arc42/06-runtime-view.md`; `docs/arc42/08-crosscutting-concepts.md`;
`docs/arc42/11-risks-and-technical-debt.md`; `design.md`.

### F10 (L) — interaction matrix omitted four inherited entries

**Status:** resolved.

The matrix now has six rows: security preview/apply, profile preview/apply, and
pages preview/apply, each with its existing `@e2e` scenario title and driving
mechanism. The inherited rows are marked unchanged.

**Evidence:** `design.md` "Interaction Coverage Matrix".

### F11 (L) — terminal baseline adaptation missing

**Status:** resolved.

The Visual Design Contract now states: no controls/headings/page; Terraform
addresses are the operator's domain names and are expected; no secret/token/
passphrase/temp path/plaintext key; no duplicated line; an error names the
failing object and the next action.

**Evidence:** `design.md` "Visual Design Contract".

### F12 (L) — rule-set canonical artifact named two homes

**Status:** resolved.

The verdict's `canonical_artifact` is exactly
`givn/specs/apply-configuration-change/publish-site-plumbing.feature`. The
evidence says the HCL values and actor shape are implementation detail under
`design.md`, not a second destination. `lower_level_artifact` is `PASS`, because
the observable protection rule is owned by the capability specification.

**Evidence:** `arc42.md` second verdict.

### F13 (L) — key generation lacked a structured verdict and the ephemeral alternative

**Status:** resolved.

A third structured verdict records `NOT_QUALIFIED`, routing
`CANONICAL_ARTIFACT` to `givn/changes/publish-site-plumbing/design.md`, with the
ephemeral `tls_private_key` alternative. The reason it does not remove the
state copy is recorded in the verdict and in the design: the secret's `value`
is an SDKv2 optional/sensitive input with no write-only attribute in provider
6.13.0, so the plaintext still reaches state.

**Evidence:** `arc42.md` third verdict; `design.md` "Technology decisions" and
"Decisions".

### F14 (L) — two Thens asserted conclusions the fake cannot establish

**Status:** resolved.

The secret Then now asserts a sealed ciphertext for the fetched repository
public key and no plaintext key (the journal's `key_id`, ciphertext length and
no PEM header). The stock-site Then now asserts the journal contains no applied
write other than the deploy key, the secret and the rule set.

**Evidence:** `feature` apply and stock-site scenarios; `design.md` journal
fields.

### F15 (L) — `gh` was not a stated prerequisite

**Status:** resolved.

The procedure names `gh`, `jq` and `tofu` on `PATH` and checks all three with
`command -v` before any read or write.

**Evidence:** `design.md` "The apply procedure".

### F16 (L) — no glossary term for the declared unit

**Status:** resolved.

The glossary adds **Pages publishing plumbing** — the deploy key, its
`PAGES_DEPLOY_KEY` secret and the `pages protection` rule set as one declared
unit — with code symbols and the anti-terms **Site publication**, **Public
website** and the future consuming workflow. The introduction names the new
term. The 12-row assessment row 12 records it.

**Evidence:** `docs/arc42/12-glossary.md`; `arc42.md` row 12.

### F17 (H) — the reinstalled partial state could record failed planned values

**Status:** resolved.

The failure path reinstalled the failed state wholesale and wrote the marker
without removing planned values for creates that never landed. SDKv2 persists
planned values for a failed create, so the state could claim the failed secret
or rule set; the partial scenario's "records the deploy key and not the
secret" and the recovery Given could then be false, and the recovery preview
would exclude the failed objects from `expected`.

The failure path now copies the failed state, then removes every address in
`$failed` that the state records with `tofu state rm <address>` (the objects are
not live, so there is no live effect), and only then writes the marker. If a
removal fails, the routine keeps the state, writes the failed address as
`unverified` in the marker, reports that it could not be removed from the
applied state and exits non-zero. The "Why the failing state is not trusted"
and "The partial verdict" bullets state the cleanup and the unverified failure.
The partial scenario asserts the rule set is not recorded; the recovery Given
asserts the same; the step-definition priming says the routine leaves the state
binding the deploy key and the generated key only.

**Evidence:** `design.md` failure path and bullets; `feature` partial and
recovery scenarios; `design.md` "Step definitions".

## 3. Operator question records

### Q1 — read-back guarantee

- **Status:** closed 2026-10-19 (operator chose option 1).
- **Branch:** Testability / missing scenarios / scope.
- **Blocks:** the read-back scenario, the use-case read-back extension, the
  routine's failure path and the provider-fact correction.
- **Context:** the capability creates three live objects after a saved plan and
  promises truthful reporting when a read after a successful write fails. The
  GitHub provider's secret create makes one timestamp read and ignores its
  failure; SDKv2 does not read after create. The design's original fault could
  not produce the promised outcome, and the routine had no read-back at all.
- **Question:** should the routine add an explicit post-apply read-back of the
  applied state and the three live objects before declaring the revision
  applied, or should the read-back extension and scenario be withdrawn?
- **Options:**
  1. **Add the post-apply read-back (chosen).**
     Meaning: after a green apply the routine reads the state and the three
     live objects. A read error prints `failed read-back: <object> <reason>`,
     keeps the applied state, writes `terraform.tfstate.stale` and exits
     non-zero.
     In practice: a 403 on the secret read after the secret is stored makes the
     apply report the three applied objects, name
     `github_actions_secret.pages_publish (PAGES_DEPLOY_KEY)` and the 403, and
     mark the state stale; the next preview refuses until the operator removes
     the marker.
     Gain: the promised truthful-reporting outcome is real, detected and
     testable; the seam is a read pass, not a new component.
     Cost: one extra read pass; a failure after every write still needs operator
     recovery.
  2. **Withdraw the extension and the scenario.**
     Meaning: only the provider's own reads count; the secret's ignored
     timestamp read means no scenario is written.
     In practice: a read failure after the secret write stays green and
     unreported.
     Gain: no extra routine behavior; one fewer scenario.
     Cost: the use-case extension and the P6 truthful-reporting consequence are
     weakened; an unreported post-write failure is accepted silently.
- **Resolution:** option 1 adopted. The routine reads the state and three
  objects back; the feature faults that read; the failure keeps the applied
  state and writes the marker.
- **Rejected alternatives:** option 2 withdraws a promised outcome and hides a
  failure; keeping the original `after_requests` fault is unimplementable
  because the provider ignores its only post-write read; faulting only the next
  preview leaves the apply green and the state unmarked, so the extension is
  not met.

### Q2 — who clears a stale applied state

- **Status:** closed 2026-10-19 (operator chose option 1).
- **Branch:** use-case contract / testability (inconsistent preconditions).
- **Blocks:** the stale-marker extension, guarantee, glossary, chapters 02, 06,
  08 and 11, and the recovery scenario.
- **Context:** the routine refuses both preview and apply while
  `terraform.tfstate.stale` exists, and its recovery is an explicit operator
  removal. The use case's precondition said "not marked stale" while its
  extension and guarantee said a green apply removes the marker. Both could not
  hold, and a green apply can never run while the marker exists.
- **Question:** should the contract say the operator removes the marker after
  reading the partial report and the next green apply completes the revision,
  or should the apply run while the marker exists and remove it on success?
- **Options:**
  1. **Operator removes, then a green apply completes (chosen).**
     Meaning: the preview and the apply refuse while the marker exists; the
     operator reads the partial report and the marker's addresses, confirms
     them and removes `infra/terraform.tfstate.stale`; the next preview sees the
     landed objects bound and plans only the missing creates; the apply
     completes.
     In practice: after the secret's 403 the operator reads "partial:
     github_repository_deploy_key.pages_publish", removes the marker, previews
     the missing secret and rule set, applies, and commits.
     Gain: no blind retry; the stale state is never applied over; the contract
     matches the design's risk decision (R-054).
     Cost: one deliberate manual step that the old wording attributed to the
     apply; the use case, glossary and chapters must change.
  2. **The apply clears it on success.**
     Meaning: only the preview refuses while the marker exists; the apply runs
     and removes the marker when every object succeeds.
     In practice: the operator reruns the apply without reading the report; a
     success removes the marker.
     Gain: matches the old wording; one fewer manual step.
     Cost: allows the blind retry the marker exists to prevent; contradicts the
     precondition; reopens the recorded safety decision.
- **Resolution:** option 1 adopted. All delta-contract occurrences now say the
  preview and the apply refuse while the marker exists, the operator removes it
  after reading the report, and the next green apply completes the revision.
  The recovery scenario is retitled "...completes the revision" because the
  green apply does not clear the marker. The durable
  `givn/specs/apply-configuration-change/usecase.md` still carries the
  pre-change wording until the archive merges the delta copy; the archive must
  carry this corrected precondition, extension and minimal guarantee.
- **Rejected alternatives:** option 2 allows a blind retry and reopens R-054;
  leaving the contradiction fails the use-case "inconsistent preconditions"
  criterion and leaves design, glossary and use case disagreeing.

## 4. Hardening edits applied

| File | Edits |
|---|---|
| `givn/changes/publish-site-plumbing/design.md` | F1 preflight status capture and JSON verdict; F2 post-apply read-back, failure output, provider fact; F3 second-guard traceability and seeding; F4/F5 fault table; F6 four-address alignment; F7 report lines; F8 recovery priming; F9 recovery wording and no green-apply marker removal; F10 six interaction rows; F11 terminal baseline; F13 ephemeral alternative; F15 `gh`/`jq` checks; F17 failed planned values removed from the reinstalled state; risk references renumbered; TD-021; post-GREEN read-back. |
| `givn/changes/publish-site-plumbing/specs/apply-configuration-change/publish-site-plumbing.feature` | F3 new second-guard scenario; F4 partial-failure fault/assertions; F5 clean-refusal Given; F6 generated-key preview and state Thens; F7 applied/failed/reason Thens; F14 observable secret and journal Thens; F17 state-cleanup assertions in the partial and recovery scenarios; Q2 recovery title. Two `@e2e` tags preserved. |
| `givn/changes/publish-site-plumbing/specs/apply-configuration-change/usecase.md` | Q1 read-back extension names applied objects and the unreadable object; F3 second-guard extension; Q2 stale precondition companion, rule, example and minimal guarantee. |
| `givn/changes/publish-site-plumbing/arc42.md` | Rows 6, 8, 10, 11 and 12 updated; QS-034, R-051..R-055 and TD-021; F12 one canonical artifact; F13 structured key-generation verdict and ephemeral reason. |
| `docs/arc42/02-architecture-constraints.md` | Q2 stale wording; F2 post-apply read-back constraint. |
| `docs/arc42/06-runtime-view.md` | Q1 read-back step and diagram; F2 provider read correction; Q2 stale wording; four-address notes retained and now asserted. |
| `docs/arc42/08-crosscutting-concepts.md` | Q2 stale wording; Q1 read-back semantics; R-054 reference. |
| `docs/arc42/10-quality-requirements.md` | QS-034 records the generated key material. |
| `docs/arc42/11-risks-and-technical-debt.md` | R-029 Q2 wording; plumbing risks renumbered R-051..R-055; R-054 covers the read-back failure. |
| `docs/arc42/12-glossary.md` | **Configuration change** Q2 contract; new **Pages publishing plumbing** term (F16); introduction names it. |
| `docs/arc42/adr/ADR-0006-encrypted-local-applied-state.md` | Risk references R-051/R-052. |

No `tasks.md` was written. `proposal.md` was not modified. No `@e2e` tag was
removed. `.github/workflows/ci.yml`, `CHANGELOG.md`, `Cargo.toml`, release
files and unrelated archives were not edited.

## 5. Required explicit statements

### Strict-mode proof

`apps/cli/tests/features_runner.rs` calls `.fail_on_skipped()`; an undefined,
pending or skipped step fails the run. All ten scenarios are `@givn.added @wip`
and have no step definitions, so the suite is RED for the right reason. The
concrete proof remains an implementation task: temporarily rename one step in a
throwaway copy, observe the non-zero exit and the "undefined step" report, then
restore. The fake's unknown-path 599 marker and refused-write journal are the
second net. **PASS.**

### Single-scenario command

`./run-tests.sh --name "Organization owner previews the pages plumbing and only the three live creations are named"`
forwards `--name` to cucumber-rs unchanged. The title contains no regex
metacharacters. **PASS.**

### E2E tag integrity

The delta contains exactly two `@e2e` scenarios (preview and apply), matching
the two normalized inventory actions, and eight non-e2e scenarios. No tag was
removed; the new second-guard and rewritten scenarios are non-e2e variants of
the apply action. `verify.e2e_command` remains `./run-tests.sh --e2e`, distinct
from `verify.command`. **PASS.**

### Interface classification — opposite test

The opposite classification, Web UI, does not hold: the capability renders no
HTML, CSS or JavaScript, no browser consumes the surface, and the consumer runs
the real `tofu` CLI and reads terminal output. GitHub is an external REST
system, not a UI. The terminal classification is correct and the browser-driver
rule does not apply. **PASS.**

### Deferrals

No mandatory check is deferred. The post-GREEN real apply is part of the
change's definition of done, not a deferral. TD-021 is recorded pre-existing
debt with a named future owner. No operator-approved deferral exists. **PASS.**

### ADR verdicts

- **Credential material in the state:** `AMEND_ADR` to ADR-0006; all five
  mandatory gates pass; alternatives, architectural impact, durable consequence
  and the existing-ADR check are decision-specific; the amendment is in place
  in the ADR file, chapter 09 and the ADR index.
- **Pages rule set:** `NOT_QUALIFIED`, `CANONICAL_ARTIFACT` to the capability
  specification. Exactly one destination; HCL values and actor shape are
  implementation detail under `design.md`.
- **Key generation:** `NOT_QUALIFIED`, `CANONICAL_ARTIFACT` to `design.md`.
  The ephemeral alternative is recorded with the reason it does not remove the
  state copy.
- No `NEW_ADR` is proposed, so the cheaper-home requirement is not triggered.
  **PASS.**

### Arc42 omission check

Re-derived independently before opening `arc42.md`: all 12 rows are affected
(goal/quality goal, constraints, context partner, strategy/provider, building
block, runtime flow, deployment, cross-cutting state/credential/apply, ADR,
QS-034, R-051..R-055/TD-021, glossary terms). `arc42.md` also marks all 12 Yes,
so no row is missing or contradicted. All 12 chapter files exist with
substantive content; no box-drawing characters or non-table pseudo-diagrams
were found. Chapter 09 links ADR-0006 and the index carries the amendment;
chapter 11 carries the plumbing risks and TD-021. **PASS.**

### Ubiquitous language

**Deploy key** and **Protection ruleset** remain in the glossary with code
symbols and contrasts; **Pages publishing plumbing** is added with its code
symbols and anti-terms; **Configuration change** now carries the corrected
stale-marker contract. The design, feature, use case and chapters use the terms
consistently. **PASS.**

### Lint and status

`givn lint --change publish-site-plumbing` exited **2**: ten `@wip` findings (all
scenarios), one shape advisory pairing the unreadable-pages-repository scenario
with the inherited unreadable-repository-setting scenario, and one long-scenario
advisory on the partial-failure scenario. No `@e2e` mapping error. Exit 2 is
allowed while the steps are unimplemented.

`givn status --change publish-site-plumbing` reports stage `design-review-done`,
with proposal, specs, design, arc42-docs and design-review complete and tasks
as the next required artifact.

Conflict-marker scan over the change directory and `docs/arc42/`: none. `@e2e`
count in the delta: 2.

## 6. Residual notes

- F17 extends the archived security precedent (a failed write persists planned values) to creates: the routine removes them before the marker. The partial and recovery scenarios pin the cleaned state.
- The durable `givn/specs/apply-configuration-change/usecase.md` still carries the pre-change stale-marker wording (`until a green apply removes it`) because the delta copy is the reviewed contract. The archive merge must carry the delta's corrected precondition, stale extension and minimal guarantee, plus the post-apply read-back extension; `tasks.md` must include that archive-time check rather than assume the merge.
- The suite cannot prove GitHub's live acceptance of the Ed25519 key, the
  sealed secret or the `DeployKey` bypass; the post-GREEN operator apply is the
  required evidence (R-055), not a design-review deferral.
- The lint shape advisory is intentional: the pages scenario exercises a
  different repository and a different unreadable object. It belongs in
  `review.md`'s overlap dispositions.
- The release 0.5.0 merge introduced duplicate risk IDs and stale QS/TD
  numbers; the plumbing side was renumbered and the merged release rows were
  preserved.

## 7. Re-review after the operator apply (2026-10-20)

The operator apply exposed one missing prerequisite: the organization
deploy-key policy. The delta `6b201f2` records it in `design.md`,
`docs/arc42/02`, `docs/arc42/07`, `docs/arc42/11` and the change `arc42.md`.
The operator commit `daad3f5b2eeadcb7b2dcb13188e7c41abc46d2e9` records the
encrypted state, backup, transcript and task record; live verification shows
the policy true, the write-enabled deploy key, the `PAGES_DEPLOY_KEY` secret,
the active `pages protection` rule set and the unchanged stock Jekyll site.

A fresh-context re-review of the delta found **no blocking findings**. It
confirmed:

- the declared revision still creates exactly three live objects and no
  update/delete; the manual organization policy stays outside the declaration;
- the pinned provider 6.13.0 cannot declare the policy; the documented
  `gh api` verification/enablement and 422 recovery are correct;
- the affected arc42 rows are 2, 7 and 11; the chapter content matches
  `design.md`; no new ADR is required; the glossary needs no new term;
- the ten delta scenarios, the two `@e2e` transcripts, the operator transcript
  and the task record are consistent;
- the full-suite pre-existing failures and the website change are
  operator-deferred out-of-delta work, not design-review blockers.

Non-blocking precision notes applied with this record: “exactly three live
creates”; the operator-apply commit's tree-level scope; the arc42 reassessment
date; the full-suite deferral note; the archive-time use-case merge check. The
optional chapter-8 manual-step-register and transcript-preflight notes are not
required for sign-off.

DESIGN-REVIEW: PASS
