# Design review — reduce-executable-size

- **Change:** `reduce-executable-size`
- **Date:** 2026-10-19
- **Method:** a fresh-context grilling subagent read the proposal, the delta,
  the design, the arc42 assessment, the permanent use case, the implementation,
  the measured report and the affected arc42 chapters; it walked every
  required branch and ran the design-review/specs/arc42/review instructions.
  The orchestrator then hardened the plan and the implementation.

## Operator decision (recorded)

The grilling's host/Windows question: the Windows executable's size is measured
on the supported platform, but the Linux suite cannot run the Windows PE; the
failure-path evidence is a host release build with the same landed profile,
while the release workflow's Windows smoke job starts the Windows executable.
The operator's "make it so" authorizes this boundary; it is recorded as R-040
and in the design's failure-outcome section. The alternative — a Windows-side
unsupported-source run in the smoke job — is a later CI addition, not a local
test substitute.

## Questions and resolutions

| Q | Branch | Resolution |
|---|---|---|
| Q1 | Measurement unit — candidate configuration vs landed revision | The command now builds the **landed** commit's `Cargo.toml` with no profile override, records `landed.bytes`/`landed.status`, and writes `applied` only when a lever was accepted and the landed build is smaller. The e2e Given asserts `git show HEAD:Cargo.toml`; the evidence is generated after the landed commit, so the report's revision carries the lever. |
| Q2 | Extension 7 and extension 5 | Extension 7 is implemented: `not_landed` when accepted but the landed build is not smaller; the command records it and never writes `applied`. Extension 5 (review rejection) remains a human review control owned by the contributor reviewer; a command behaviour would be a second mechanism. |
| Q3 | Host/Windows failure path | Recorded above; the host lever build proves the platform-independent `Result` diagnostic; the Windows smoke starts the Windows executable; R-040 records the boundary. |
| Q4 | Must a panic be triggered? | No test-only panic hook. The report's `## Failure outcome` lines are asserted; the panic message/abort is the toolchain's guarantee, and the conversion diagnostic is exercised on the host lever build. |
| Q5 | Transcript contents | The transcript now holds the measurement command, the report (appended before substitution), the diagnostic command/output and both exit statuses; the build log is not recorded; the scenario directory and canonical project path are substituted after the report is appended. |
| Q6 | `opt-level=s` standalone vs cumulative | The landed precondition is the panic build; the rejection scenario asserts against the **row's cumulative baseline** (2,084,864 vs 2,014,208). The proposal keeps the standalone pre-lever measurement as evidence; the use case example is the cumulative comparison. |

## Findings and resolutions

| ID | Finding | Resolution |
|---|---|---|
| F1 | The landed evidence measured a revision without the lever. | The landed build mode, the `git show HEAD:Cargo.toml` Given, and the landed commit `6b54806` close it; the e2e is regenerated after the commit. |
| F2 | The larger-lever scenario compared the row against the unchanged baseline and would go RED after the commit. | The step asserts `lever > row[2]` and `saved_bytes < 0`; the scenario/use case use the cumulative comparison. |
| F3 | Extension 7 / landed rebuild missing. | Implemented as `landed.bytes`, `landed.status`, and `status: not_landed`; the success guarantee is the landed build smaller than the baseline. |
| F4 | Missing error/refusal scenarios. | Added the unknown-lever refusal and the build-failure report. The diagnosis-loss refusal is a control: the lever does not lose the typed diagnostic; the host lever build proves it. The review rejection is a human control (Q2). The zero-saving case uses the same comparison as the larger lever; no no-op lever exists; recorded in the design's decisions. |
| F5 | Transcript leaked host paths, PID and duration; wrong exit status. | The transcript appends the report before substitution, canonicalizes the project path, records the measurement exit and the diagnostic exit, and drops the build log. |
| F6 | Use-case rule copied the measurement's name-retaining build. | The rule now says the saving is measured from the stripped executable; the file size is the quantity. |
| F7 | Weak report assertions. | The steps assert the supplied published size, `baseline_sha256`, `saved_bytes`, the landed section, `## Failure outcome`, the JSON keys, and a new numbered diagnostic with a seeded prior report unchanged. |
| F8 | Duplicate published scenario and an uninventoried diagnostic action. | The duplicate regular scenario was removed; the e2e carries the distinct saved-baseline assertion. The diagnostic is the failure-outcome half of the same inventory action; the use case and matrix name it. |
| F9 | Decision ledger/seed provenance. | The design has a `## Decisions` ledger with D1, D2 (seed Q3) and D3 (seed Q8), D4, and the build/review controls. The seed status is `proposed by reduce-executable-size`; its Q6/Q8 are annotated with the resolutions. |
| F10 | Capability-routing recommendation was stale. | The proposal records `EXTEND publish-public-release` (score 17; lexical) and the placement-test override to `NEW in reduce-distributed-executable-size`. |
| F11 | Ubiquitous-language terms and sign. | The glossary adds Baseline, Published binary and Saved bytes; refused proposals are no longer called levers; `saved_bytes` is positive, and the proposal/use case say "saves". |
| F12 | Testability omitted the stub/proof. | The design and tasks record the RED stub (`panic` with the step name) and the strict-mode proof. |
| F13 | Interface classification implicit. | The design states not-a-Web-UI and why. |
| F14 | Local-runnability shim claim. | The design names the Nix-only `CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS=-L <dir>` environment; no shim is committed. |
| F15 | `create_worktree` failure escaped. | The command catches `measurement.MeasurementError` and writes the `failed` report. |
| F16 | Arc42 overstated the landed verification. | The landed build is implemented; chapters 2/6/10 and `arc42.md` state the landed measurement; chapter 2 clarifies "unchanged by the measurement"; the glossary terms are added. |
| F17 | `@wip` remains. | Expected at design-review; the tasks stage is next and removes the tags after the RED/GREEN loops. |
| F18 | Release-note commit shape. | The landed `feat(reduce-distributed-executable-size)` revision includes `Cargo.toml`, a user-receiving path; the design records the shape. |

## Branch verdicts

| Branch | Verdict |
|---|---|
| Scope | Resolved: the delta covers the measured lever, the rejected larger lever, the refusals, the landed rebuild, the failure outcome and the published-size separation. |
| Tech choices | Resolved: the command measures the landed configuration; the report/JSON are asserted; the dead `config_target` helper was removed. |
| Missing scenarios | Resolved: unknown lever and build failure added; the diagnosis-loss/review-rejection/zero-saving controls are recorded. |
| Testability | Resolved: strict mode, stub pattern, single-scenario command; concrete assertions; the cumulative comparison. |
| E2E fidelity | Resolved: one inventory action → one `@e2e`; the real command; the landed `git show` precondition; the transcript scrubbed. |
| Interaction Coverage | No issue: one inventory entry, one matrix row, one `@e2e`; the failure outcome is part of the action. |
| Risk | Resolved: the landed gap and the larger-lever false green are closed; R-039/R-040 record the panic and host/Windows boundaries. |
| Use-case and Persona | Resolved: the rule no longer requires a name-retaining build; the Actor and Personas are correct; the review-rejection control is recorded. |
| Decision ledger | Resolved: D1/D2 (seed Q3)/D3 (seed Q8)/D4 and the controls; the seed annotations. |
| Visual design contract | Resolved: terminal contract; the committed transcript is scrubbed and carries both exit statuses. |
| Deferrals | No mandatory check deferred; the host/Windows boundary is the operator-recorded R-040 and the Windows smoke still starts the executable. |
| Persona disposition | No issue: three confirmed Personas with relevance; no invented Persona. |
| Interface classification | Resolved: terminal; not a Web UI; no browser driver. |
| ADR qualification | No issue: the profile lever is NOT_QUALIFIED (architectural impact/durable consequence/lower-level artifact fail) and routed once to `design.md`; no ADR. |
| Arc42 | Resolved: the independent 12-row walk matches (1 No, 2 Yes, 3 No, 4 Yes, 5 Yes, 6 Yes, 7 Yes, 8 Yes, 9 No, 10 Yes, 11 Yes, 12 Yes); all chapters exist, are non-placeholder and Mermaid-only. |
| Ubiquitous language | Resolved: Baseline/Published binary/Saved bytes added; the proposal/use case/design use the positive saved convention. |

## Use-case scope rejection criteria

| Criterion | Evidence | Result |
|---|---|---|
| Goal conjunction | One verb phrase: make the executable smaller by a measured lever. | PASS |
| Missing primary actor | Release maintainer is primary and named in the Trigger. | PASS |
| Placement failure | The capability advances the reduction goal, not the measurement or release. | PASS |
| Mutation under an observing goal | The use case changes the build configuration and lands it; it is not observing. | PASS |
| Extension without a step reference | Extensions reference steps 3, 3, 4, 5, 7. | PASS |
| Inconsistent preconditions | The attribution exists; the stable/single-file/static constraints hold. | PASS |
| Mechanism vocabulary | Goal/Trigger/guarantees avoid mechanisms; the command is in Rules. | PASS |
| Component/channel-shaped name | The use case and capability are goal-shaped. | PASS |

## ADR qualification

Candidate: the `panic = "abort"` profile lever. Alternatives: unwind, abort,
`opt-level = "s"`, feature audit. `alternatives` PASS; `architectural_impact`,
`durable_consequence` and `lower_level_artifact` FAIL (build-configuration
choice owned by `design.md`/the capability; one-line reversal, no migration or
boundary); `existing_adr_check` PASS (ADR-0001…0006; no archived ADR
directory). Routing: `CANONICAL_ARTIFACT` — `design.md`. No ADR file or
register row.

## Visual design contract

Terminal; no rendered screen. The committed transcript path and scrubbing rule
are in `design.md`; the e2e scenario commits it. The review's visual pass is
recorded in `review.md`.

DESIGN-REVIEW: PASS
