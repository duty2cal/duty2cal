# Review — publish-organization-profile

Completeness review of the `publish-organization-profile` change under the
owning use case `apply-configuration-change`. Read for this review: the
proposal, the delta feature, `givn/specs/apply-configuration-change/usecase.md`,
`design.md`, `design-review.md` (including `## Post-implementation amendment
review`), `arc42.md`, the durable chapters `docs/arc42/01`–`12`,
`tasks.md`, the two committed transcripts, the two profile step modules,
`infra/org_profile.tf`, `org/`, `scripts/fake-github/serve.py`,
`scripts/adopt-public-home.sh`, `scripts/setup-infra-tests.sh`,
`apps/cli/tests/steps/enable_repository_security.rs`, `run-tests.sh`, and
`givn/commands.yaml`.

No production code, test, design, task, or spec was changed by this review.
The suite was not re-run: the tasks evidence supplies the per-scope counts and
the archive receipt is not yet produced. The review ran the read-only
`givn check review` gate under the Nix Rust toolchain and `givn lint`. F1
(tasks revision format) and F2 (use-case preconditions) were both remediated;
the gate now exits 0.

## Fabrication audit

### 0. `@e2e` tag integrity

All six delta scenarios were checked in the feature and across the seven
commits that touched the feature (`ef18cd8`, `f90447c`, `17336d2`, `3b4cf5b`,
`ebe14ae`, `6c46ce0`, `3322b38`).

- The two planned `@e2e` scenarios were added as `@givn.added @e2e @wip` in
  `ef18cd8`.
- Each `@e2e` tag survived; the later commits removed only `@wip`:
  `ebe14ae` and `6c46ce0` show `-  @givn.added @e2e @wip` / `+  @givn.added @e2e`.
- No scenario's tags were changed from including `@e2e` to excluding it, and no
  `@wip` remains (current feature tags: two `@e2e`, four regular `@givn.added`).

Result: **no tag-integrity finding.**

### 1. Empty/trivial step bodies

Grep of `apps/cli/tests/steps/publish_organization_profile.rs` and
`apps/cli/tests/steps/e2e/publish_organization_profile.rs` for
`unimplemented!`, `todo!`, empty function bodies, bare `pass`/`return`:

- 0 `unimplemented!`/`todo!` hits.
- 0 empty step bodies.
- The `return;` hits at lines 99, 193, 209, 220, 248, 307, 332 of the non-e2e
  module are early returns inside preview/apply/helper functions, not empty
  step bodies; `404 => {}` is a match arm.
- The two files contain 30 and 11 step bindings respectively (41 total), and
  every body calls the real `tofu` process, the fake HTTP boundary, `jq`, the
  applied state, or an `assert` (69 and 27 assertion sites). The shared
  `When` bindings in `enable_repository_security.rs` dispatch to the profile
  module when `DutyWorld.source_tree` is set; they are not stubs.

Result: **0 empty step bodies found across 2 files** (41 step bindings).

### 2. Checked task ↔ commit

| Task | Revision | Diff touches | Verdict |
|---|---|---|---|
| 1. Revised profile content | `ef18cd8` | production: `infra/org_profile.tf`, `org/profile/README.md`, `org/SECURITY.md`, `LICENSES/CC0-1.0.txt`, `REUSE.toml`, fake, adoption/setup/base-prime integration; tests | verified |
| 2. Content claims | `f90447c` | step-definition implementation + delta feature; production copy already in `ef18cd8` | verified (assertion task; not spec-only) |
| 3. Default security policy | `17336d2` | step-definition implementation + delta feature; production copy already in `ef18cd8` | verified (assertion task; not spec-only) |
| 4. Unbound profile repository refuses | `3b4cf5b` | step-definition implementation + delta feature; preview guard in the profile harness | verified (not spec-only) |
| 5. `@e2e` apply | `ebe14ae`; strengthened by `3322b38` | e2e step implementation, transcript, feature, tasks | verified |
| 6. `@e2e` preview | `6c46ce0` | e2e step implementation, transcript, feature, tasks | verified |
| Setup (runner/strict mode/stubs) | no commit, as declared | step modules, `DutyWorld`, runner wiring | verified in working tree |
| E2E setup | no commit, as declared | setup proof, stub module, count proof | verified in working tree |
| Final verification | no commit, as declared | evidence only | verified from evidence |

Scenario 1 carries the production declaration and reviewed copy; scenarios
2–4 add the assertions for behaviour that declaration already supplies (the
copy is the design's reviewed block). No checked task is spec-only or
stub-only. The setup and final-verification tasks explicitly record
"No commit" and are verification tasks, not implementation tasks.

### 3. Promised components

| Design promise | Present |
|---|---|
| `infra/org_profile.tf` with `github_repository.dot_github`, `github_repository_file.org_profile_readme`, `github_repository_file.org_security`, `variable "org_source_dir"` | yes |
| `org/profile/README.md`, `org/SECURITY.md`, `LICENSES/CC0-1.0.txt`, `REUSE.toml` `org/**` override | yes |
| Separate `apps/cli/tests/steps/publish_organization_profile.rs` and `apps/cli/tests/steps/e2e/publish_organization_profile.rs` | yes, declared in both `mod.rs` files |
| Fake create/contents/branch/commit/vulnerability-alert overlays and journal | yes (`scripts/fake-github/serve.py`) |
| Adoption staging exclusion; setup proof/base-workspace exclusion; security base-prime removal/restore | yes (`scripts/adopt-public-home.sh:211`, `scripts/setup-infra-tests.sh:163,210,243`, `enable_repository_security.rs:359,435`) |
| Four 404 descriptors, including `repos/duty2cal/.github.json` | yes (`write_profile_descriptors`) |
| Strict mode `fail_on_skipped()` | yes (`features_runner.rs`), proof in tasks |
| Two committed transcripts | yes, one per `@e2e` scenario |

No promised component is missing.

### 4. Strict-mode proof

Present in `tasks.md` §0: the apply scenario with the stub panics with
`not implemented: ...` and exits `EXIT:101`. The preview setup task repeats the
proof at the second step (`EXIT:101`). Strict mode is
`DutyWorld::cucumber().fail_on_skipped()`; an undefined/pending step fails.
This proof is present and consistent with the implementation.

### 5. `@e2e` primary assertions

Both `@e2e` scenarios assert through the real interface, not repository-only
state:

- Apply: the saved plan JSON creates, the CLI apply transcript
  (`Apply complete! Resources: 3 added...`), the fake's `GET
  /repos/duty2cal/.github` body (public, no issues/projects/wiki/discussions),
  the contents API bytes compared byte-for-byte with the reviewed `org/`
  source, the applied-state addresses, the journal's exact write set (one
  `POST /orgs/duty2cal/repos`, exact `PATCH` set `/repos/duty2cal/.github`,
  exactly two `PUT`s), no refused/unknown path, and an empty targeted
  follow-up plan.
- Preview: the real targeted plan JSON's three creates and no other change,
  the live `GET` 404 for the profile repository, zero applied writes, no
  refused/unknown path, and an unchanged applied-state snapshot.

Neither scenario is repository-only or CLI-less. The journal assertion was
strengthened after design-review: the current implementation uses
`unique | join(" ")` and compares the complete `PATCH` path set, closing the
design-review's non-blocking `. [0]` finding.

### 6. Browser classification

The change's declared interface is terminal/CLI + GitHub REST API. The
opposite classification — the organization profile page is a rendered Web UI
— does not hold for this system: the project neither serves nor scripts that
rendering; GitHub renders the profile from the published bytes. There is no
project-owned browser screen, DOM, click, or navigation to drive. The
observable system output is the REST API response the apply produces, which is
what the scenarios assert. A browser test against GitHub's renderer would test
a third-party service and require a live credential. The classification is
tested and rejected in `design.md` ("Visual Design Contract", "Black-Box-First
policy") and is consistent with the step implementations (real `tofu`
subprocesses and raw loopback HTTP, no browser driver). No HTTP-in-page or
`fetch()` substitution exists.

### 7. `verify.e2e_command` target and parallel implementations

`givn/commands.yaml`:

- `verify.command: "./run-tests.sh"`
- `verify.e2e_command: "./run-tests.sh --e2e"`

`run-tests.sh` rewrites `--e2e` to `--tags "@e2e"` and invokes
`cargo test -p duty2cal --test features_runner` over the assembled feature
root. The file invoked is `apps/cli/tests/features_runner.rs`; the profile e2e
implementation is
`apps/cli/tests/steps/e2e/publish_organization_profile.rs`.

Tree search (tracked and working tree) found exactly one profile e2e
implementation. `git status` shows only `docs/arc42/06-runtime-view.md` and an
unrelated ideation file modified; no untracked parallel profile step file
exists. The non-e2e profile module is a separate file for the regular
scenarios, matching the design's two-file layout. There is no weaker parallel
implementation to displace.

### 8. E2E normalized-action scope

The owning use case's `## Interactions` has four entries: two for
`enable-repository-security` and two for `publish-organization-profile`. The
normalized profile actions are exactly "preview the declared configuration and
read the named differences" and "apply the previewed revision and read the
outcome"; each has one `@e2e` scenario. The revised content, content-claim,
policy and unbound-refusal scenarios are variants/error cases of the same
actions and are regular scenarios. No profile action has two `@e2e` scenarios,
and no `@e2e` scenario covers a different action.

### 9. Command scoping and counts

`verify.command` and `verify.e2e_command` are distinct strings (not a copy).
The e2e command is the full runner restricted by the `@e2e` tag. Tasks evidence
records:

- full suite: 150 scenarios / 648 steps;
- `@e2e`: 8 scenarios / 59 steps;
- 8 < 150, so the e2e scope is strictly smaller.

The archive receipt for this change does not exist yet
(`givn/archive/publish-organization-profile/verification.json` absent), so
there is no receipt count to read at review time; the tasks evidence is the
execution record, and the archive will write the receipt. The counts were not
re-derived by re-running the suite, per the review constraints.

### 10. Design-versus-implementation conformance

| Design statement | Implementation | Match |
|---|---|---|
| `verify.command` / `verify.e2e_command` exactly as above | `givn/commands.yaml` | yes |
| Separate non-e2e and e2e step files per capability | both files exist, declared | yes |
| cucumber-rs 0.23.0, `std::process::Command`, `std::net::TcpStream`, no browser driver | `tofu` helper + `fake_request`; no browser crate | yes |
| Targeted plan/preflight/follow-up at the three resources | `TARGETS` used by preview, preflight and follow-up | yes |
| Shared `When` bindings dispatch on `source_tree` | `enable_repository_security.rs:789,800` dispatch to profile module | yes |
| Four 404 descriptors (post-review correction) | `write_profile_descriptors` writes four | yes |
| Apply scenario primes `prime_base_state` (unchanged fixture, no billing rewrite); preview scenario reuses `ensure_base_state` (billing drift excluded by targeting) | `prime_base_state` does not rewrite the organization body; `ensure_base_state` rewrites billing; both plans target the three profile resources | yes |
| Transcript path and scrubbing | both transcripts committed, scrubbed | yes |
| `--name` is the single-scenario form; `--tags` + `--name` conflict | evidence uses `--name`; counts proven separately | yes |
| Journal assertion is an exact set, two `PUT`s in either order | implementation asserts exact sets and counts | yes |

No silent deviation from a reviewed design decision was found. The three
post-implementation factual corrections in `design.md` (four descriptors,
workspace `org_profile.tf` restore, apply-side priming) match the code.

### 11. Interaction coverage cross-reference

| Use-case inventory entry | Design matrix row | `@e2e` scenario | Step driver |
|---|---|---|---|
| `enable-repository-security` — preview | archived sibling design matrix | `Organization owner previews the security settings and only the declared differences are named` (permanent feature, `@e2e`) | `std::process::Command` runs `tofu` in `steps/e2e/enable_repository_security.rs` |
| `enable-repository-security` — apply | archived sibling design matrix | `Organization owner applies the previewed revision and the security settings are enabled` (permanent feature, `@e2e`) | `std::process::Command` runs `tofu` in `steps/e2e/enable_repository_security.rs` |
| `publish-organization-profile` — preview | this change's matrix row 1 | `Organization owner previews the profile declaration and only the three creations are named` | shared `When` dispatches to `steps/publish_organization_profile.rs::preview`; real `tofu` subprocess + raw HTTP |
| `publish-organization-profile` — apply | this change's matrix row 2 | `Organization owner applies the previewed revision and the organization profile is published` | shared `When` dispatches to `apply_previewed`; real `tofu` subprocess + raw HTTP |

Every use-case inventory entry is mapped. This change's matrix deliberately
maps its own two entries; the two sibling entries are mapped in the archived
`sibling` design and feature. No inventory entry is unmapped and no extra
matrix row exists. The promised driving mechanism is `std::process::Command`
`tofu` plus raw loopback HTTP; the step files use exactly that.

### 12. Fabrication-audit result

No fabricated step, task, component, tag removal, downgraded e2e, parallel
implementation, or silent design deviation was found. The audit is clean. The
separate `givn check review` integrity failure is a tasks-format defect, not a
fabrication finding; see F1.

## Arc42 implementation conformance

Affected chapters were derived independently from the implementation and
compared with `arc42.md`, `design.md`, `tasks.md` and the durable chapters.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Requirement 5, quality goal 8, stakeholder rows | `01-introduction-and-goals.md:13,29,35,38,39` | Affected; profile, default policy, manual prerequisites, truthful copy | D1–D4, profile copy | scenarios 1–6, operator step | `infra/org_profile.tf`, `org/` | yes |
| Content/licensing/third-party/naming/manual constraints | `02-architecture-constraints.md:29–34,48,57` | Affected; CC0-1.0 `org/**`, Unicode-3.0, absolute links, no third-party embeds, manual reporting/pinning | D5–D9 | tasks §1, §2, §3, operator step | `org/`, `REUSE.toml`, `LICENSES/CC0-1.0.txt`, declaration | yes |
| Public-home context and profile reader | `03-context-and-scope.md:20–29,60,84` | Affected; profile repository and reader are a new public surface | architecture impact | — | `org/` declaration and scenarios | yes |
| Building blocks: `org/`, `infra/org_profile.tf`, fake, step modules, workspace exclusions | `05-building-block-view.md:71–108` | Affected | declarations/step sections | tasks §1, §5, §6 | files exist with the documented responsibilities | yes |
| Runtime scenario: preview, preflight, apply, publish, manual steps; apply/preview base distinction | `06-runtime-view.md:274–360` | Affected; new scenario and notes | "Why the preview and follow-up are targeted" | tasks §5, §6 | `prime_base_state` vs `ensure_base_state`; targeted plans; transcripts | yes |
| Deployment: third repository, two files, manual reporting/pinning, consequences | `07-deployment-view.md:95–171` | Affected; profile repository table and manual steps | "Manual operator steps" | operator step | declaration and chapter 7 | yes |
| Crosscutting: public copy, default health files, declared vs manual | `08-crosscutting-concepts.md:76–82,126–186,213–217` | Affected | profile/security sections | tasks §1–§4 | implementation and tests | yes |
| Quality scenarios `QS-022`, `QS-023`, tree node `rp6` | `10-quality-requirements.md:35,63,64` | Affected; two scenarios | profile/security assertions | scenarios 2, 3 | content and policy scenarios | yes |
| Risks `R-030`, `R-032`–`R-035`; debt `TD-014`, `TD-015` | `11-risks-and-technical-debt.md:36–38` | Affected | risks/manual steps | operator step | chapter entries present | yes |
| Glossary: seven public-presentation terms | `12-glossary.md:50,60,69,82` and profile terms | Affected; seven terms | reviewed-public-copy section | — | glossary rows present and reused | yes |
| Chapters 04 and 09 unaffected | `04-solution-strategy.md`, `09-architecture-decisions.md`, `adr/README.md` | Not affected; no ADR qualifies | ADR qualification | — | no new ADR or solution-strategy change | yes |

The runtime chapter was corrected during this review to distinguish the apply
e2e base (`prime_base_state`, unchanged `security-applied` fixture, no billing
rewrite) from the preview e2e base (sibling `ensure_base_state`, known billing
drift excluded by the targeted plan). The corrected text matches
`steps/publish_organization_profile.rs` and
`steps/enable_repository_security.rs:471–485`. No stale or contradictory
chapter remains.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The seven terms introduced by this change are present in
`docs/arc42/12-glossary.md` and are reused with the same code symbols:

| Term | Spec/design use | Code symbol | Recorded? |
|---|---|---|---|
| Organization profile | use case and prose | GitHub rendering of `profile/README.md` | yes |
| Organization profile repository | use case/spec | `github_repository.dot_github` | yes |
| Profile content | spec/design | `org/profile/README.md`; `github_repository_file.org_profile_readme` | yes |
| Default security policy | spec/design | `org/SECURITY.md`; `github_repository_file.org_security` | yes |
| Default community health files | design/chapter 8 | profile repository files | yes |
| Pinned repositories | proposal/chapter 7 | manual web setting | yes |
| Reviewed public copy | design/chapter 8 | umbrella for `org/` | yes |

Existing terms used by the change (`Declared configuration`, `Configuration
change`, `Adoption`, `Applied state` as the state a configuration change
records, `Security settings`) are the glossary's established vocabulary; the
glossary's `Configuration change` entry names the applied state as the recorded
state, and the spec uses it consistently. No new unrecorded domain term is
introduced. The profile copy's "crew duty plan" is the reviewed public wording
for the tool's input, recorded inside the `Profile content` definition, not a
new aggregate name. Code symbols (`dot_github`, `org_profile_readme`,
`org_security`, `org/`) match the glossary's `Code symbol` column.

No ubiquitous-language finding.

## Coverage classification

No coverage receipt exists yet for this change; the coverage addon is enabled
(`givn/config.yaml`), and the archive receipt will carry the measured merged
result. Per the review constraint the suite was not re-run. Classification from
the recorded evidence and static inspection:

1. **Dead code:** none. `publish_profile` is used by the content and policy
   scenarios; `prime_base_state`, `preview`, `apply_previewed`,
   `published_file`, `write_profile_descriptors`, `ensure_source_tree` and
   `source_tree` are used by the scenario bindings; clippy was clean in the
   tasks evidence.
2. **Missing test coverage:** none. All six scenarios are implemented; both
   normalized actions have one `@e2e` scenario; the revised-content, claim,
   policy and unbound-refusal variants have regular scenarios. The shared
   configuration-change refusal/partial/stale branches are exercised by the
   sibling `enable-repository-security` scenarios through the same use-case
   procedure, as the use-case traceability records; the profile scenarios
   assert the profile-specific create/update/content outcomes. No redundant
   unit test was added; the `.feature` scenarios are the source of truth.
3. **Legitimately hard to test:** the live GitHub acceptance and rendering of
   the profile page, the organization's actual private-vulnerability-reporting
   setting, and the manual pin state. Each has a concrete technical
   justification: they live behind a third-party service and a credential the
   suite must not hold (R-030, R-033, R-035); the fake proves the provider
   calls and the published bytes, and the operator's real apply/confirmation is
   the recorded acceptance evidence. This is bucket 3, not an invented
   exception.

No classification outside the three buckets is used.

## E2e coverage

- Normalized inventory: two profile actions (preview; apply), each with one
  `@e2e` scenario. No excess `@e2e` scenario.
- Primary assertions are real-interface: the CLI apply/preview JSON and
  transcript, the fake's REST responses, the published bytes, the applied
  state, and the journal. Repository-only assertions are supplementary, not
  primary.
- Driver: real `tofu` subprocesses and raw loopback HTTP. No browser driver is
  used, and none is required for this non-browser capability.
- Opposite classification considered: the organization profile page *is* a
  rendered Web UI in GitHub's renderer. It does not make this capability a
  browser-UI capability: the project owns the declaration and the published
  bytes, not the renderer or a project-owned screen. The system's observable
  interface is the CLI/API. The classification stands.
- Browser-UI test replacement: none (no HTTP-in-page or `fetch()` step).

## Deferrals

The proposal, use case, design and chapter 7 record three manual operator
obligations: enable private vulnerability reporting on `duty2cal/duty2cal`
before publication, apply the reviewed revision with the passphrase, and pin
`duty2cal` and `duty2cal.github.io` once in the organization settings. Each is
outside the pinned provider's resources, has a recorded owner decision and a
canonical deployment-chapter home (R-030 and chapter 7); none is an
unrecorded deferral of an automated check. The live apply has not yet updated
`infra/terraform.tfstate` (the committed state predates the declaration and
contains no profile addresses); the tasks record it as an archive-time
operator obligation, not a ticked task. This is consistent with the proposal
and design, but the archive must confirm the operator steps before the change
is considered published.

## Use-case and Persona conformance

Owning use case: `apply-configuration-change`.

| Check | Result |
|---|---|
| Goal is one verb phrase / one success outcome | yes: "Make the live configuration match the declared configuration revision the apply accepts" |
| Primary actor and trigger | Organization owner is primary; the trigger names the owner recording a revision |
| Capability placement | `publish-organization-profile` is a new capability under the administration use case; the proposal's routing table records why the lexical `EXTEND enable-repository-security` route was not followed |
| Guarantees | creation of an absent declared object, preview scope, refusal/partial rules, stale state, recorded revision preserved |
| Extensions reference main-flow steps | all extensions name step 2 or 3; no new success outcome |
| Preconditions | yes — F2 remediated: each bullet holds one precondition; the absent-object creation stays in the main flow/rules/success guarantee |
| Mechanism vocabulary / component-shaped names | goal and trigger use the use case's vocabulary; capability name is domain-shaped |
| Leaf main-flow size | one goal decomposed into record/preview/apply/report/stop, not three actor goals |
| Relationship misuse | no `Includes`/`Extends` relations |
| Interactions | four entries; both profile entries map to the matrix and `@e2e` scenarios |

Persona dispositions (the use case's three confirmed Personas; no new Persona
or migration ledger entry is introduced):

| Persona | Disposition |
|---|---|
| `project-maintainer--organization-owner` | Actor lens. Records the revision, previews three creates, applies and owns the published claims and the manual pin/reporting steps. Both `@e2e` scenarios and chapter 7. |
| `contributor--infrastructure-reviewer` | Review lens. Reads the declaration, `org/` copy and default policy without a token or passphrase; the content and policy scenarios assert the bytes. |
| `crew-member--personal-calendar-owner` | Prospective-reader lens. The published copy states local conversion, Windows x86_64 only, no browser claim, the download/source links and the non-affiliation sentence; the content scenario asserts them. |

The user-visible profile surface has confirmed Personas; no invented Persona
is promoted.

## Visual review

The design declares `Interface: terminal` (CLI + GitHub REST API) and no
project-owned rendered screen. Evidence: one committed transcript for each of
the two `@e2e` scenarios, under
`evidence/visual/<scenario-slug>/transcript.txt`.

- The apply transcript shows the real `tofu init`, targeted plan,
  `Apply complete! Resources: 3 added, 0 changed, 0 destroyed.`, the three
  created resource IDs (`profile/README.md`, `SECURITY.md`, `.github`), and a
  follow-up targeted plan with `No changes.`
- The preview transcript shows the base state `No changes.`, the targeted
  `Plan: 3 to add, 0 to change, 0 to destroy.`, the `tofu show -json` create
  set, and exit status 0.
- Scrub check: no scenario directory, loopback port, token, passphrase or
  billing address remains as a live value; they appear only as
  `<scenario-dir>`, `127.0.0.1:<port>`, `<token>`, `<passphrase>` and
  `<billing-email>` placeholders.
- Persona dispositions: the organization owner can see the preview and the
  apply outcome and reach the publication goal. The reviewer and crew-member
  goals are served by the reviewed `org/` copy and the content/policy
  scenarios; the page rendering is GitHub's and is not a project-owned screen.
  The documented limitation is R-035: the live GitHub rendering/acceptance is
  the operator's apply evidence, not a screenshot the project can own.
- No pixel-level or coordinate findings. No Persona finding is open.

VISUAL-REVIEW: PASS

## Overlap dispositions

`givn check review` reports `✓ overlap dispositions passed` with the table
below. The table records the shape relationships found by static inspection of
the six delta scenarios and the sibling security feature; no exact duplicate
was found and no unrecognised or empty decision remains.

| Finding pair | Decision |
|---|---|
| `Organization owner applies the previewed revision and the organization profile is published` ↔ `A revised profile content updates only the profile file` | variant (create vs update of the same profile objects) |
| `Organization owner previews the profile declaration and only the three creations are named` ↔ `A profile repository that already exists live and is unbound refuses the preview` | variant (create preview vs refusal branch) |
| `The published profile content states only supported claims and requests no third-party resource` ↔ `The profile repository carries the organization's default security policy` | boundary (profile content vs default security policy; different artifacts) |
| `Organization owner previews the profile declaration ...` ↔ `Organization owner applies the previewed revision ...` | boundary (preview plan vs apply outcome) |
| Profile preview/apply scenarios ↔ sibling `Organization owner previews the security settings ...` / `... applies the previewed revision ...` | boundary (different capability and objects; shared use-case procedure) |

## Split-or-keep

No delta scenario in this change exceeds the review gate's long-scenario
threshold, so the gate reports no long scenario and no split-or-keep row is
required. The longest delta scenario is the content-invariant scenario at 11
steps; it remains one coherent invariant set, not a candidate to split.

## README impact

The project README was not edited; no commit in this change touches
`README.md`, and `git log -- README.md` shows no change from this work. The
change's public face is the organization profile content in `org/`, published
to `duty2cal/.github`; it is not a project README section. No converter
command, flag, configuration, or output consumed through the README changed.
`scripts/public-text-check.sh` still reports the README's 14 public-text
promises present, so no README section needs refreshing.

README-IMPACT: none

## Gate and execution evidence

- `givn lint --change publish-organization-profile`: `1 file(s) checked — clean`.
- `nix-shell -p rustc cargo --run 'givn check review --change
  publish-organization-profile'` exits 0:
  `✓ integrity passed`, `✓ run declaration passed`,
  `net delta: 6 added, 0 modified, 0 removed`, `✓ overlap dispositions
  passed`.
- `givn spec coverage` exits 0: `135 behavior entries, 6 @e2e` (the permanent
  corpus; the delta's six scenarios are separate).
- Scenario counts from the tasks evidence: full scope 150 scenarios / 648
  steps; e2e scope 8 scenarios / 59 steps; 8 < 150, so the e2e scope is
  strictly smaller.
- The two working-tree transcript diffs are ordering/timestamp churn from the
  gate run and are not the reviewed evidence; the committed transcripts are
  the evidence and are unchanged in content and scrub.
- Archive verification receipt: not produced yet
  (`givn/archive/publish-organization-profile/verification.json` is absent);
  the archive writes the receipt and the measured coverage result. The review
  did not re-run the suite.
- Local setup: `./scripts/setup-infra-tests.sh` evidence is recorded in
  `tasks.md`; the review did not re-run it.

## Findings

### F1 (resolved) — tasks revision identifiers

The six scenario tasks now use the gate form
`- [x] COMMIT: \`<hash>\` — ...` (`ef18cd8`, `f90447c`, `17336d2`, `3b4cf5b`,
`ebe14ae`, `6c46ce0`; the strengthening commit `3322b38` is in the note).
Re-run under the Nix Rust toolchain:

```text
$ nix-shell -p rustc cargo --run 'givn check review --change publish-organization-profile'
Mandatory checks for 'review':
  ✓ integrity passed
  ✓ run declaration passed
net delta: 6 added, 0 modified, 0 removed
net delta json: {"added":6,"modified":0,"removed":0}
  ✓ overlap dispositions passed
gate_exit:0
```

Integrity is clean. The six hashes were previously verified with
`git show --stat`; each touches the expected files. F1 is resolved.

### F2 (resolved) — `usecase.md` `## Preconditions`

The first precondition is now the single bullet
`- The organization and the repositories the revision retains exist.`; the
absent-object creation remains in the main flow, rules and success guarantee.
`givn spec coverage` exits 0 (`135 behavior entries, 6 @e2e`), and the gate's
capability-routing check now passes (`gate_exit:0`). F2 is resolved.

### Non-blocking observations

- The live apply, private-reporting setting and pins remain operator
  obligations; the committed `infra/terraform.tfstate` predates this
  declaration and does not yet record the profile addresses. Recorded in the
  design, chapter 7, R-030/R-035 and the tasks operator note; not an
  unrecorded deferral.
- `tasks.md` now records `./run-tests.sh --name "<title>"` as the correct
  single-scenario command, matching the pinned runner's `--tags`/`--name`
  conflict; the earlier stale wording is gone.

## Missing evidence

- No `givn/archive/publish-organization-profile/verification.json` receipt
  exists yet; the per-scope counts in this review come from `tasks.md`
  (150 full / 648 steps; 8 e2e / 59 steps). The archive will produce the
  receipt and the measured coverage result.
- No live GitHub screenshot/receipt exists; the rendering/acceptance is the
  operator's real apply (R-035), as designed. The committed transcripts are the
  complete terminal evidence.

## Sign-off checklist

- Fabrication audit: clean (0 empty step bodies; tags intact; components
  present; strict-mode proof present; no downgraded e2e; no parallel
  implementation; design conformance matches).
- Every checked implementation task has a verified revision touching the
  expected files (scenario 1 carries production; later scenarios carry
  assertions; setup/final tasks are commitless verification tasks as
  declared).
- Every promised component exists.
- Strict-mode proof present and passing (`EXIT:101` proof).
- Scenario execution evidence: tasks.md records 150 full / 8 e2e; the archive
  receipt is pending.
- Coverage classification: no coverage receipt yet; static classification uses
  buckets 1–3; no gap unresolved; no redundant unit test added.
- No `@wip` tags remain; the spec uses domain vocabulary, not
  implementation-layer detail.
- Canonical E2E policy: two normalized actions, two `@e2e` scenarios, variants
  regular.
- Real-interface primary assertions and reviewed drivers: yes; no HTTP/fetch
  browser substitution.
- Local run command / setup proof: recorded setup proof (setup-infra-tests)
  passes; the review did not re-run it.
- `verify.e2e_command` target identified (`run-tests.sh` → features_runner;
  `@e2e` filter); tree searched for a parallel implementation; none found.
- `verify.e2e_command` differs from `verify.command`; 8 < 150.
- Implementation matches design.md's commands, file layout and driver; no
  silent deviation.
- Interaction coverage: all four inventory entries mapped; profile entries
  have matrix rows, feature scenarios and the promised `tofu`/HTTP drivers.
- No classification outside the three coverage buckets.
- **Review gate: PASSES.** `givn check review` exits 0 with integrity, run
  declaration and overlap dispositions passed; `givn spec coverage` exits 0.
  F1 and F2 are both resolved; all sign-off items pass.

REVIEW: PASS
