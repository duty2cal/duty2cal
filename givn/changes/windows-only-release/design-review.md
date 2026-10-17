# Design review: windows-only-release

Fresh-context grilling of the planning layer, followed by hardening. The
questions below are ranked by severity and cite the tree as it stood during
grilling; the resolutions at the end record the edits applied to `design.md`,
the delta feature and the delta use case. `givn lint --change
windows-only-release` exits 2 after hardening (six `@wip` findings and shape
advisories only), so the delta parses; `givn status` reports proposal, specs
and design done.

Ranking: Critical → High → Medium → Low. Every finding cites the current tree.

---

## Ranked questions

### Q1 — Critical — The e2e coverage report becomes runner-only, and the design's "non-empty" claim is unsupported

**Evidence**

- `givn/changes/windows-only-release/design.md:248` fills the mandatory
  "Non-zero production probe" column with **none** for the e2e row.
- `design.md:250` claims "The e2e report remains non-empty because
  `cargo llvm-cov --workspace` instruments the workspace and the test harness".
- `measure-coverage.sh:27-28` runs `cargo llvm-cov --workspace --test
  features_runner ... --tags "@e2e"`.
- The only production execution in the current e2e scenario is
  `runs_the_extracted_executable` and
  `prints_usage_and_reports_failure`
  (`apps/cli/tests/steps/e2e/publish_public_release.rs:151-179`);
  `design.md:203-206` deletes both.
- The current `coverage/e2e-cobertura.xml` contains exactly the seven
  production source files, not the test harness: that report exists because
  the spawned CLI was instrumented and run. `apps/cli/Cargo.toml:18-20`
  declares the `features_runner` test target; the package has no library
  target, so the test binary links cucumber/tokio, not the converter.
  [INFERENCE from the test-target shape and the current report's file list.]
- `givn/commands.yaml:45-46` states the project rule: "Reject runner-only or
  zero-production reports; verify one known production path is non-zero."
  The implement instruction's step 5a turns that into
  `COVERAGE MEASUREMENT: INVALID` when a scenario's production coverage is
  runner-only or test-only. The givn binary also carries the rejection
  string "Cobertura XML shows zero lines measured (lines-valid=0)".

**Recommended answer**

Do not claim the report will be non-empty. D1 (operator's 2026-10-17
decision) moves *execution evidence* to the Windows smoke job; it does not
authorise an invalid coverage measurement. Before tasks are written, either:

1. give the e2e scope a genuine production probe (not possible without
   changing the interaction inventory, so probably not), or
2. record an operator decision that the e2e coverage report is deliberately
   harness-only, say exactly how `measure-coverage.sh` and the archive gate
   treat it (including a non-zero `lines-valid` outcome), or
3. change the coverage design so the change is measured from the non-e2e
   scope only and the archive gate receives a valid, distinct e2e report by a
   mechanism the design names.

Run `./measure-coverage.sh` during the tasks RED check and record the actual
`lines-valid` for `coverage/e2e-cobertura.xml`. If it is zero, the current
design is invalid, not merely optimistic.

Severity: **Critical** (sign-off blocker).

---

### Q2 — High — The `@e2e` title asserts an execution the scenario does not perform

**Evidence**

- `givn/changes/windows-only-release/specs/publish-public-release/publish-public-release.feature:37-46`
  is titled "Maintainer prepares a release and runs the archived executable";
  its steps prepare, report, extract, and assert the extracted member and
  bytes. There is no run step.
- `design.md:131` says retitling "would require `@givn.removed` plus
  `@givn.added`, which would delete the permanent scenario instead of
  modifying it". The specs instruction's merge semantics define
  `@givn.removed` (delete by title) and `@givn.added` (append); the pair is
  exactly how a rename is expressed. It does not "delete instead of modify";
  it removes the old-title scenario and appends the new-title one.
- `design.md:283` explicitly defers the honesty fix to "a later change",
  without an operator decision.
- The permanent copy at
  `givn/specs/publish-public-release/publish-public-release.feature:59-67`
  really does run the executable; that is why the title used to be true.
- The delta usecase records the same title in the interaction inventory
  (`givn/changes/windows-only-release/specs/publish-public-release/usecase.md:179`).

**Recommended answer**

Retitle the scenario. Use:

```gherkin
@givn.removed @wip
Scenario: Maintainer prepares a release and runs the archived executable
  Given the scenario is removed
```

plus a `@givn.added @e2e @wip` scenario titled
"Maintainer prepares a release and extracts the Windows archive" (or
"…inspects the archived Windows executable"). Update the usecase
`## Interactions` row, the design's Interaction Coverage Matrix, the evidence
directory slug, and every traceability reference. Keeping the title is not
defensible: the scenario's own assertions now contradict it, and the
"delete instead of modify" rationale is wrong. See Q3 for the one real
mechanical consequence of this change: the assembly must not run the
`@givn.removed` placeholder.

Severity: **High**.

---

### Q3 — High — The `assemble-features.sh` shell filter is fragile and under-specified; it can remove the wrong scenario or run removed placeholders

**Evidence**

- `design.md:24, 55, 130, 216-220` places Gherkin-aware merge logic in a
  shell script and says it parses "the active change" per capability.
- The current assembly copies **every** feature under both roots:
  `scripts/assemble-features.sh:16-17` (`find givn/specs givn/changes`), with
  no active-change selection. There is no `--change` argument in
  `run-tests.sh:35` or `measure-coverage.sh:18`.
- The delta file mixes tag lines and scenario bodies
  (`publish-public-release.feature:8-60`); the permanent file has a leading
  `@e2e` tag line (`givn/specs/publish-public-release/publish-public-release.feature:59`).

**Concrete failure cases**

1. **Multiple active changes.** `find givn/specs givn/changes` includes every
   open change, not "the active change". Two open deltas that modify the same
   permanent scenario produce two delta copies and a union removal set; the
   suite runs both modified versions. The assembly has no concept of the
   change being tested.
2. **Title-only removal across capabilities.** Two capabilities may hold the
   same exact scenario title (`givn spec duplicates` exists for a reason).
   Removing by title from every permanent file deletes the wrong scenario;
   the design says "per capability" but never says how the delta's capability
   path maps to the permanent capability path.
3. **Scenario Outline / Examples.** A `Scenario Outline:` line and its
   `Examples:` table are not matched by a `Scenario:`-line shell matcher.
   The removed block leaves the `Examples:` table behind, or the permanent
   outline keeps running beside the delta.
4. **Indentation / tag lines / doc strings.** Gherkin allows varying
   indentation; tags may be on one or more lines above the scenario; a
   `Scenario:` string inside a doc string or step text can be mistaken for a
   block boundary. A leftover `@e2e` tag line re-tags the next scenario and
   silently adds e2e coverage.
5. **Malformed or unmatched title.** The design says "A malformed delta is
   left to `givn lint`", but `run-tests.sh` never invokes `givn lint`; the
   gate only sees the mis-assembled corpus. A title typo leaves both the
   permanent (five-platform) scenario and the delta running, producing a
   confusing red or, for a scenario whose old values happen to pass, a false
   green.
6. **`@givn.removed` placeholders.** If Q2's retitle is adopted, the delta
   contains a `@givn.removed` scenario with one placeholder step. The design
   says "copy every delta feature unchanged"; cucumber-rs with
   `.fail_on_skipped()` then runs the placeholder, which has no step
   definition, and the suite fails. The filter must omit removed scenarios
   from the copied delta too, or `run-tests.sh` must add
   `--tags "not @givn.removed"`.
7. **Archive interaction.** After archive the delta lives under
   `givn/archive/`, which the current `find` does not scan, so no filtering is
   needed then. During the archive gate the delta is still under
   `givn/changes/`; verify the gate runs the suite before the merge/move, or
   the removal set disappears at the wrong moment.

**Recommended answer**

Replace the shell parse with a parser-aware helper (a small Python state
machine with a fixture test, or a Rust helper using the `gherkin` crate the
runner already depends on). It must:

- take an explicit change id, or fail when more than one `givn/changes/*`
  directory exists;
- map a delta path to the same usecase/capability path under `givn/specs/`
  and remove titles only there;
- parse `Scenario:` and `Scenario Outline:` blocks including tag lines,
  doc strings, data tables, and `Examples:` tables;
- omit `@givn.removed` placeholder scenarios from every copied feature;
- fail loudly when a `@givn.modified`/`@givn.removed` title is not found in
  the corresponding permanent capability (never "leave it to lint");
- copy delta features unchanged apart from removed placeholders; and
- have fixture tests for an outline, a duplicate title, multi-line tags,
  indentation, a doc string containing `Scenario:`, and two active changes.

The safer alternative is not "no filter": without it the pre-archive suite is
red against the old five-platform assertions. The right place is the
assembly, but the implementation must be a parser, not a title grep.

Severity: **High**.

---

### Q4 — High — The added scenario duplicates the modified table scenario, and its two new steps restate what the table already asserts

**Evidence**

- `publish-public-release.feature:55-60` adds "A prepared release offers the
  Windows archive and no other platform archive": `Given an executable built
  for every supported platform` (which is now Windows only), prepare, `Then
  holds exactly the Windows archive` and `holds no archive for a platform
  outside Windows x86_64`.
- The modified scenario at `publish-public-release.feature:8-15` already
  asserts `holds exactly these archives, each with its checksum file` with a
  one-row table, and its step
  (`apps/cli/tests/steps/publish_public_release.rs:585-633`) asserts the
  **exact staged set** (`entries(&out)` equals the archives, checksums,
  `release-notes.md`, and `CHANGELOG.md`). That exact-set assertion already
  implies "no archive for a platform outside Windows x86_64".
- `design.md:140-142` cites both scenarios as evidence for the same rule;
  `design.md:198-200` adds two new steps for assertions the table step
  already makes. `givn lint` reports shape overlap between the added scenario
  and its neighbours.

**Recommended answer**

Delete the added scenario and the two new steps
(`then the prepared release holds exactly the Windows archive for version
<version>` and `then the prepared release holds no archive for a platform
outside Windows x86_64`). Keep the modified table scenario; if the negative
wording is considered valuable, add one assertion to that scenario's existing
step (or a sentence in the feature description), not a second scenario. One
scenario must assert one observable behaviour; this one is the same behaviour
as the table.

Severity: **High** (spec quality; the duplicate also weakens the "one
scenario = one behaviour" and non-e2e difference rules).

---

### Q5 — High — The public-text gate does not enforce "must not promise Linux or macOS", and the issue form/security policy are outside every check

**Evidence**

- `design.md:23` states the rule: "The README must not promise Linux or macOS
  downloads", and claims the existing gate checks the archive names against
  the platform set.
- `scripts/public-text-check.sh:39-48` only **requires presence**. After the
  design's change it would loop over `windows-x86_64` only; a README that
  kept the four old rows would still pass.
- `README.md:69-72` currently holds the four old rows; `README.md:76` says
  "all five platforms"; `README.md:100-101` still names a macOS archive; the
  `chmod` note at `README.md:80-82` and the macOS section at
  `README.md:109-118` are also stale and are not caught by any check.
- `.github/ISSUE_TEMPLATE/bug.yml:26-34` still offers Linux and macOS as
  platform choices; `SECURITY.md:29` still describes the Gatekeeper/macOS
  out-of-scope case. `design.md:116` says these are changed, but no check
  reads either file.
- `design.md:54,114` says the gate adds "a required source-build statement"
  and "a platform may be added back on request" without naming the exact
  strings or regexes. Any prose containing the words would pass; any rewording
  of a true promise would fail.
- `scripts/public-text-check.sh:43` currently requires
  `sha256sum --check`; the design's gate list at `design.md:114` omits it
  while the README keeps it (`design.md:112`). Decide whether it is a
  required promise.
- The platform list is hardcoded in three places:
  `scripts/prepare-release.sh:42`, `apps/cli/tests/steps/publish_public_release.rs:24`,
  and `scripts/public-text-check.sh:40`. `design.md:20` calls this "one
  canonical value"; it is a mirror, not a single source, and nothing checks
  the three agree.

**Recommended answer**

- Add negative assertions: the README must contain none of
  `duty2cal-<version>-linux-x86_64`, `-linux-arm64`, `-macos-arm64`, or
  `-macos-x86_64`, and must contain no `Linux x86_64` / `macOS` download row.
- Extend the public-text gate (or add a second gate invoked by CI) over
  `.github/ISSUE_TEMPLATE/bug.yml` and `SECURITY.md`: the only allowed
  platform labels are `Windows x86_64` and `built from source`; the
  Gatekeeper text must not remain.
- Name the exact regexes for the source-build and platform-request
  statements in the design, and add them to the README verbatim.
- Decide `sha256sum --check`: keep it required (the README retains it) or
  remove it from the README; do not leave the gate and the text disagreeing.
- Either derive the public-text loop from `PLATFORMS` in
  `scripts/prepare-release.sh` or add a lint that compares the three lists.

Severity: **High** (the proposal's main public-text claim is not enforced).

---

### Q6 — High — Removing the four release builds also removes all compile evidence for the "other platforms build from source" promise

**Evidence**

- `.github/workflows/release.yml:14-44` currently builds five targets,
  including macOS arm64/x86_64 and Linux musl/arm64. The design deletes four
  of them (`design.md:88-96`).
- `.github/workflows/ci.yml:38-56` compiles only the Linux host;
  `ci.yml:195-235` builds and smokes only `x86_64-pc-windows-gnu`.
- The delta usecase now promises: "a crew member on any other platform obtains
  the same tool by building it from source"
  (`givn/changes/windows-only-release/specs/publish-public-release/usecase.md:162`),
  and the README is required to state the same
  (`givn/changes/windows-only-release/specs/publish-public-release/usecase.md:105`,
  rule "The public text a reader lands on").
- `design.md:278-282` records only that users "must build from source, which
  needs a Rust toolchain"; it does not record that no target in CI compiles
  those platforms any more, and chapter 11's plan
  (`design.md:66-70`) does not add a risk for it.
- `docs/arc42/11-risks-and-technical-debt.md:19` (R-013) currently claims the
  build matrix proves musl, Linux arm64 and both macOS targets; that evidence
  disappears with the matrix.

**Recommended answer**

Either (a) keep a cheap, non-publishing compile/check job for the platforms
whose source build the public text promises (at least
`cargo check --target x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`,
and the macOS targets on macOS runners), or (b) narrow the promise: say the
source is available and builds are not a supported platform, and record the
unverified source-build route as a risk in chapter 11 and in the usecase
success guarantee. Do not leave a public guarantee with zero evidence after
deleting the only compile coverage. If (a), `ci.yml` must change, contradicting
`design.md:77`.

Severity: **High**.

---

### Q7 — High — The ADR-0003 amendment plan is incomplete: the qualification verdict, More Information, frontmatter, and chapter-11 consequence are missing

**Evidence**

- `docs/arc42/adr/ADR-0003-release-channel-and-platforms.md:24` still says
  `routing: NEW_ADR`; the design's verdict is `AMEND_ADR` with target
  ADR-0003 (`design.md:267-274`).
- The ADR's `## Qualification` section is not in the design's amendment list
  (`design.md:71`, `design.md:276` lists title, context, drivers, options,
  outcome, consequences, confirmation, pros/cons only).
- `ADR-0003:91-93` (`## More Information`) still references D4 (unsigned
  macOS archives) and the old design; it is not listed.
- The frontmatter date is `2026-10-13` (`ADR-0003:3`); the design says the
  amendment "records the narrowing date" but does not say whether the
  frontmatter, the register date, or only an inline note changes.
- `ADR-0003:65` carries the "Bad, because ... five builds" consequence; it
  must be replaced. The new "Bad" consequence (other platforms lose the
  download route and their source build is not evidenced) is not added to
  chapter 11 by the plan (`design.md:66-70` only removes R-010/TD-006 and
  rewrites R-009/R-013).
- The existing-ADR check should record that the archived register was searched
  (there is no `docs/arc42/adr/archive/`); the design says only "ADR-0003 owns
  the release channel ... ADR-0004 owns immutability".

**Recommended answer**

Amend the whole record: Qualification verdict (`qualification: QUALIFIED`,
`routing: AMEND_ADR`, `target_adr: ADR-0003`, archived-register check), the
frontmatter date or an explicit "Amended 2026-10-17" section, `More
Information`, and the `Bad` consequence. Update chapter 09 and
`docs/arc42/adr/README.md` with the amended title **and** the new date or an
amendment note. Add the new "Bad" consequence to chapter 11 as a risk row.
`AMEND_ADR` is the defensible route (same channel/platform boundary); a new
ADR would split one boundary, and a supersession would discard the channel
decision.

Severity: **High** (the durable record would otherwise contradict the design).

---

### Q8 — High — `arc42.md` is missing and chapter 03 is omitted from the chapter plan

**Evidence**

- `givn/changes/windows-only-release/` contains only `design.md`,
  `proposal.md`, and `specs/` (directory listing, 2026-10-17). There is no
  `arc42.md`, so the mandatory 12-row assessment and `STATUS` marker do not
  exist. The design-review instruction cross-checks `arc42.md`; a missing
  marker is a sign-off blocker.
- The design's "New or changed files" table lists chapters 01, 02, 04, 05, 06,
  07, 08, 09, 10, 11, and 12 (`design.md:60-72`) but **not**
  `docs/arc42/03-context-and-scope.md`.
- Chapter 03 describes the release channel's outward interface and says
  GitHub serves "per-platform archive, checksum, provenance, bill of
  materials" (`docs/arc42/03-context-and-scope.md:27-30`); that text becomes
  inaccurate under the one-archive decision.

**Recommended answer**

Create `givn/changes/windows-only-release/arc42.md` with the full 12-row
assessment (see the independent table below) and `STATUS: DONE`, and add
`docs/arc42/03-context-and-scope.md` to the changed-files table with the
narrowed archive wording. The other eleven chapters are correctly identified.
Do not write `STATUS: DONE` until the chapters are actually updated.

Severity: **High** (sign-off blocker; omission, not just a contradiction).

---

### Q9 — Medium — The design names three coverage/test scripts that do not exist at those paths

**Evidence**

- `design.md:77` says `scripts/run-tests.sh`, `scripts/measure-coverage.sh`,
  and `scripts/merge-coverages.sh` need no change.
- The actual files are `run-tests.sh`, `measure-coverage.sh`, and
  `merge-coverages.sh` at the repository root; `measure-coverage.sh:18` and
  `run-tests.sh:35` call `scripts/assemble-features.sh`. `scripts/` contains
  `assemble-features.sh`, `prepare-release.sh`, and `public-text-check.sh`
  but not the three named scripts.
- The design's test-runner command and single-scenario command correctly use
  `./run-tests.sh` (`design.md:178,180`), so the error is localized but
  actionable: a task that follows `design.md:77` will edit nonexistent files.

**Recommended answer**

Correct the three paths to the repository root and state that the root
`measure-coverage.sh`/`merge-coverages.sh` already delegate assembly to
`scripts/assemble-features.sh` (true). Keep Q1's coverage decision separate:
"need no change" may be false if the e2e report is empty.

Severity: **Medium**.

---

### Q10 — Medium — Not every modified scenario can fail in RED against the old implementation; the design does not classify the keep-true scenarios

**Evidence**

- The delta's modified "A platform whose executable was not built stops the
  release" (`publish-public-release.feature:17-23`) now leaves
  `windows-x86_64` out. With the new step definitions and the **old**
  five-platform script, the preparation still refuses a missing Windows
  executable and names it (`scripts/prepare-release.sh:150-160`). The
  scenario therefore passes before the production change. It is a
  re-baselining scenario, not a RED driver.
- The modified checksum/unreadable scenarios can be RED only because the test
  helper's `SUPPORTED` changes first; the table and e2e scenarios are the ones
  that distinguish the old five-archive output from the new one.
- `design.md:190-215` lists the step changes but does not classify which
  scenarios are legitimate immediate-green (the implement instruction permits
  "all steps reused" immediate green, but requires recording it).
- No executable scenario or gate covers the issue form, `SECURITY.md`, or the
  README's **absence** of the old platforms (Q5), so the public-text half of
  the change can be unfinished while the suite is green.

**Recommended answer**

In `design.md`, mark the modified scenarios that are keep-true/re-baselining
and the two or three that are the change's RED drivers. Add at least one
change-specific assertion that fails against the old five-platform script
(the table scenario already does; make that explicit). Add the public-text
negative/issue-form gate (Q5) so the non-scenario half is measurable.

Severity: **Medium**.

---

### Q11 — Medium — The delta usecase joins two outcomes in its goal/success guarantee and introduces vocabulary the glossary does not hold

**Evidence**

- `givn/changes/windows-only-release/specs/publish-public-release/usecase.md:15`
  (Goal) says "…a crew member on Windows can obtain …; a crew member on any
  other platform builds the tool from source."
- `usecase.md:160-162` (Success guarantee) names Windows download, other
  platform source build, maintainer statement, and evidence consistency.
  The review instruction's use-case rejection criteria include "Goal
  conjunction — `## Goal` joins two verb phrases" and "`## Success
  guarantee` names two outcomes".
- `usecase.md:58` says "Linux remains the project's build and test host",
  which is an internal engineering constraint, not a crew-member-observable
  rule; chapter 2 is the canonical home.
- `docs/arc42/12-glossary.md:62` still defines "Supported platform" as five
  combinations, and the glossary has no "build and test host" term
  (`grep host` finds only a prose "distribution channel" line at 119).
  `design.md:195` introduces `distribution_platform()` as a code name; the
  design does not say whether "distribution platform" is a synonym for
  "Supported platform" or a new term.

**Recommended answer**

Keep one success guarantee: the release is obtainable and traceable for the
platform it serves. Move the source-build route to a public-text rule and the
build-host fact to chapter 2 (and the design), not the usecase success
guarantee. Add "Build and test host" to the glossary or use an existing
chapter-2 term; use "Supported platform" consistently and record
"distribution platform" as a code-only helper name (or rename the helper).
The glossary's `Supported platform` and `Release archive` updates are
correctly planned (`design.md:70`), but this additional term is not.

Severity: **Medium**.

---

### Q12 — Medium — The Visual Design Contract's transcript baseline is not produced by the planned steps

**Evidence**

- `design.md:173` requires the transcript to show "the release text's
  version, the extraction listing with exactly `duty2cal.exe`, and the exit
  statuses".
- The planned e2e steps (`design.md:203-213`) keep
  `the release text names version 0.1.0`, which the current step reads from
  the file without printing it
  (`apps/cli/tests/steps/publish_public_release.rs:862-868`: the
  `the_release_text_names_version` body uses `release_text(w)` at line 496 and
  an assertion, not `run_command`). The new
  `then the extracted archive holds exactly the executable "duty2cal.exe"`
  is described as "asserts the `tar` member list" (`design.md:207`); the
  existing `archive_members()` helper runs `tar` and captures output
  internally, so the transcript may not show the listing.
- `design.md:226` says the matrix "commits the transcript", but no step is
  named as the call site. The current call site is inside the deleted
  `prints_usage_and_reports_failure`
  (`apps/cli/tests/steps/e2e/publish_public_release.rs:178`).

**Recommended answer**

Do one of:

- have the final Then print the release text (e.g. `cat release-notes.md`)
  and run `tar -tzf` through `run_command` so both appear in
  `scenario_transcript`, then call `commit_transcript(w)` there; or
- correct the baseline to what the steps actually show (preparation command,
  reported archive/digest, extraction command, one appended exit status).

Name the exact step that commits the transcript. If Q2's retitle is adopted,
update the evidence path and the baseline slug.

Severity: **Medium**.

---

### Q13 — Medium — The stand-in makes the extracted-bytes assertion a fixture round-trip, and the "deterministic transcript" claim fails on a Windows host

**Evidence**

- `design.md:195` says `executable_bytes()` returns the real built binary
  when the host is Windows x86_64, otherwise the fixed stand-in.
- `design.md:212` says "the stand-in digest is deterministic and is not
  scrubbed". On a Windows host the e2e uses the real built binary; its
  digest is toolchain-dependent and the current code scrubs it
  (`apps/cli/tests/steps/e2e/publish_public_release.rs:57-72`). The planned
  "not scrubbed" rule would make the committed evidence drift on Windows.
- The new Then "the extracted executable is the stand-in the preparation
  staged" compares the extracted bytes with the input fixture. On Linux both
  are the same deterministic stand-in, so it proves a tar round-trip, not
  that the archive holds the production Windows executable. The design itself
  says the stand-in is not executable evidence (`design.md:132`).

**Recommended answer**

Always use the stand-in for the e2e (even on Windows) or keep the digest
scrub for any host. Rename the step to "is the executable the preparation
staged" if it can be real. Strengthen the assertion by comparing the
extracted digest with the digest the preparation reported in its terminal
output and the `.sha256` file (three-way), so the assertion ties the
consumer-visible output to the bytes, not only to the fixture. Keep the
execution claim in the Windows smoke job.

Severity: **Medium**.

---

### Q14 — Low — The one-entry release matrix leaves dead musl linker configuration behind

**Evidence**

- `.github/workflows/release.yml:47-50` sets
  `CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER` and
  `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER`; every matrix entry also
  carries `linker` (`:22-45`). With only the Windows GNU entry these are
  dead values.
- `design.md:88-96` describes deleting the four entries and the start step
  but does not remove the linker fields/env.

**Recommended answer**

Remove the `linker` field and the two musl `CARGO_TARGET_*` env lines, or
document them as intentionally retained. `actionlint` does not care, but the
design's own "delete dead weight" rule does.

Severity: **Low**.

---

### Q15 — Low — The README rewrite does not address historical assets or every stale macOS/Linux sentence

**Evidence**

- `README.md:62` says "every release holds exactly one archive per supported
  platform"; the `0.1.0` release still holds five (proposal D2,
  `proposal.md:88`). The sentence is ambiguous for historical releases.
- `README.md:76` says "one extraction route serves all five platforms";
  `README.md:80-82` has the Linux/macOS `chmod` note; `README.md:100-101`
  has the macOS `shasum` example; `README.md:109-118` is the macOS approval
  section.
- `design.md:100-114` lists the new download section and says the macOS
  approval section is deleted, but does not mention the historical-release
  distinction, the `chmod` note, or the `shasum` example.

**Recommended answer**

State that only future releases are Windows-only and that already-published
releases (including `0.1.0`) keep their original assets and links. Remove the
`chmod`, `shasum`, "all five platforms", and macOS approval text, or mark
them explicitly as historical and not a current download route. Add the
negative assertions from Q5 so a regression is caught.

Severity: **Low** (public-text quality; no gate covers it yet).

---

### Q16 — Low — D2's "README/ADR text stating immutability" is not true of the README, and D1's host evidence has no recorded artifact

**Evidence**

- `design.md:122` says D2 is realized "by the README/ADR text stating that
  published releases are immutable". `grep -n immutab README.md` returns
  nothing; ADR-0004 and the usecase state it, the README does not.
- `design.md:121` says D1 is "evidenced by ... the host-side smoke run", but
  the "New or changed files" table lists only the e2e transcript; no CI run
  URL or log is recorded under the change.

**Recommended answer**

Add the immutability sentence to the README (or change the D2 ledger to name
ADR-0004/verification only). Record the Windows smoke run's URL/result (or
the exact CI job name and revision) under the change's evidence, or amend
the ledger to say the host evidence is the release workflow itself, not a
change artifact. Do not claim a README statement that does not exist.

Severity: **Low**.

---

### Q17 — Low — Stale "five archives" wording and a single-scenario example that uses the duplicate scenario

**Evidence**

- `apps/cli/tests/steps/publish_public_release.rs:633` still says "the staged
  set is not the five archives, their checksums, the release text and the
  change log". `design.md:55` says comments stop saying five but does not
  list the assertion message.
- `design.md:181` gives the single-scenario command with the title of the
  added duplicate scenario; if Q4 deletes it, the example is invalid.

**Recommended answer**

Update the assertion message to the one-archive set and point the
single-scenario example at the surviving table scenario (or the retitled e2e
scenario). Grep the step files for "five" as part of the task.

Severity: **Low**.

---

### Q18 — Low — The "except windows-x86_64" Given is the empty set, and the step's assertion is vacuous

**Evidence**

- `publish-public-release.feature:17-23` uses "an executable built for every
  supported platform except `windows-x86_64`".
- `apps/cli/tests/steps/publish_public_release.rs:651-661` filters the
  one-entry `SUPPORTED` to zero and asserts `platforms.len() ==
  SUPPORTED.len() - 1` (0 == 0), then `build_binaries(w, &[])` creates an
  empty binaries directory. The scenario still tests the intended refusal
  (the preparation refuses the missing Windows executable), but it no longer
  tests a partial set; it is indistinguishable from "no binaries at all".

**Recommended answer**

Keep the scenario (the refusal is the point), but either rename the Given to
"no executable was built for `windows-x86_64`" (clearer for a one-platform
world) or add a comment in the step that with one supported platform the
filter is necessarily empty. Assert the refusal names the supported set, not
just the platform. This is a wording/feedback improvement, not a functional
defect.

Severity: **Low**.

---

## Branch coverage

| Required branch | Finding |
|---|---|
| **Scope** | Finding: the delta contains a duplicate added scenario (Q4); the design's scope omits `docs/arc42/03` (Q8) and the issue-form/security policy checks (Q5). The core scope (Windows-only future releases, immutable history, Linux test host, source-build route) matches the proposal's D1–D3. |
| **Tech choices** | Finding: `scripts/assemble-features.sh` shell parsing is under-designed (Q3); the design names nonexistent `scripts/*` coverage paths (Q9); the coverage process choice leaves the e2e report invalid (Q1). The one-entry matrix, `PLATFORMS` narrowing, and `public-text-check` extension are otherwise appropriate. |
| **Missing scenarios** | Finding: no executable check or scenario covers the issue form, `SECURITY.md`, or the README's **absence** of Linux/macOS rows (Q5); no evidence path for the source-build promise on other platforms (Q6). The proposal's preparation refusals are covered. |
| **Testability** | Finding: the e2e cannot be measured as production coverage (Q1); the modified missing-platform scenario can pass before the production change (Q10); the design does not classify keep-true/immediate-green scenarios. Strict mode, the stub pattern, and the single-scenario command are named. |
| **E2E fidelity** | Finding: the title claims execution the scenario does not perform (Q2); the transcript baseline is not produced by the planned steps (Q12); the stand-in comparison is a fixture round-trip and the transcript determinism fails on Windows (Q13). The real interface (`scripts/prepare-release.sh` as a real subprocess) and primary assertion on its output/files are otherwise correct. |
| **Interaction Coverage** | Finding: both inventory entries have matrix rows and the inventory size is unchanged, but the prepare row's title overclaims execution (Q2). No missing or extra inventory entry. |
| **Risk** | Finding: the design's risk list omits the two most likely implementation failures — the runner-only/empty e2e coverage gate (Q1) and the shell Gherkin parser (Q3) — and the source-build evidence gap (Q6). It records the host-only workflow risk and the historical-assets divergence. |
| **Use-case and Persona context** | Finding: the delta Goal/Success guarantee now names two outcomes (Q11); the usecase rule mixes the internal "build and test host" constraint with product rules. Persona coverage is correct: the two confirmed Personas appear in the proposal and design, no new Persona is invented, and the Persona impact is recorded. |
| **Decision ledger** | No unresolved D<n>: D1, D2, and D3 are explicitly decided in `design.md:119-127`; the proposal has no seed. Finding (low): D1's "host-side smoke run" evidence has no recorded artifact and D2's README-immutability claim is false (Q16). |
| **Visual design contract** | Finding: `## Visual Design Contract` exists with `Interface: terminal` and viewports n/a, which is the right classification for a CLI; the evidence-kind baseline does not match the planned transcript (Q12). |
| **Deferrals** | Finding: D1's execution deferral is operator-approved (proposal D1, 2026-10-17); the e2e coverage-validity gap is a separate mandatory check and has no operator decision (Q1). The Q2 title honesty fix is deferred to "a later change" without a decision (Q2). |
| **Persona disposition** | No issue found. `project-maintainer--release-owner` and `crew-member--personal-calendar-owner` are confirmed, their relevance and evidence are recorded in `design.md:158-165`, and neither is used as a Gherkin Actor. |
| **Interface classification** | No issue found. The project's interface here is the terminal preparation command; the README and release page are GitHub-rendered text, not a project-rendered Web UI, so the opposite classification (Web UI) does not hold and no browser driver is required. The E2E uses a real subprocess, not an HTTP client. |
| **ADR qualification** | Finding: `AMEND_ADR` with target ADR-0003 is the defensible route (same channel/platform boundary; ADR-0004 owns immutability), and no `NEW_ADR` is proposed. The amendment itself is incomplete: the ADR's qualification verdict still says `NEW_ADR`, and `More Information`, the date, and the new Bad consequence are missing (Q7). |
| **Architecture documentation (arc42)** | Finding: all 12 rows independently assess as **Yes**; the design plans 11 of them and omits chapter 03 (Q8); `arc42.md` and its `STATUS` marker are absent. All 12 chapter files exist, contain real content, and contain no ASCII-art box diagrams or multi-line `+`/`|` pseudo-diagrams. |
| **Ubiquitous language** | Finding: the glossary still says "Supported platform" is five combinations and has no "Build and test host" term; the delta usecase introduces that concept and the design's helper uses "distribution platform" (Q11). The terms "Release archive" and "Supported platform" are otherwise used consistently. |

---

## No-issue checks (searched, not assumed)

- **`machine_platform` / `machine_executable` references.** A tree-wide
  search finds them only in
  `apps/cli/tests/steps/publish_public_release.rs:110-123` (definitions),
  `:147-148` (`executable_bytes`), and the e2e file's imports and call sites
  (`apps/cli/tests/steps/e2e/publish_public_release.rs:15,67,107,133,153`).
  The design replaces or deletes every one (`design.md:191-212`); no other
  step file references them. No other step file breaks.
- **Permanent scenarios other than the five `@givn.modified` titles.** I
  scanned every scenario in
  `givn/specs/publish-public-release/publish-public-release.feature`. The
  only hard-coded Linux/macOS/Windows values are at lines 8-12, 16, 18, 30,
  45, 47, and 61-65, all inside the five modified scenarios. The remaining
  scenarios use "every supported platform" or "this machine" steps that
  remain true when `SUPPORTED` has one entry. No other permanent scenario
  becomes false (Q18 is a wording issue, not a false assertion).
- **Merged coverage denominator.** `coverage/non-e2e-cobertura.xml` already
  covers all 886 lines across the seven production files; `merge-cobertura.py`
  unions the file sets, and `coverage/cobertura-coverage.xml` already has
  886 lines. The denominator therefore does not change. The problem is the
  e2e report's validity, not the denominator (Q1).
- **Architecture chapter file integrity.** All 12 files under `docs/arc42/`
  exist, contain real content, and contain no Unicode box-drawing characters
  or non-table `+`/`|` pseudo-diagrams. The blocker is the missing
  change-level `arc42.md`, not a scaffolded chapter.
- **ADR existing-record check.** There are four ADRs, no
  `docs/arc42/adr/archive/` directory, and no competing release-channel
  record. ADR-0003 is the only target for the platform-set decision;
  ADR-0004 is the separate immutability record. The `AMEND_ADR` route is
  correct; the amendment's completeness is Q7.
- **`cargo llvm-cov --tags "@e2e"` distinctness.** The reports are separate
  runs with separate output paths (`measure-coverage.sh:23-28`); they remain
  distinct as files. The unresolved question is whether the e2e one has any
  lines (Q1), not whether it collides with the non-e2e report.

---

## Independent arc42 assessment (derived before reading `arc42.md`)

`arc42.md` does not exist in `givn/changes/windows-only-release/`, so there is
no claim table to cross-check. The table below is my own read of the design,
proposal, and specs against the 12 rows.

| # | Chapter | My Yes/No and reason | design.md plan | Current chapter evidence | Finding |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | **Yes** — requirement 5 and the stakeholder/quality rows promise the platform set. | Listed (`design.md:60`). | `docs/arc42/01-introduction-and-goals.md:13` still says five platforms. | Planned update is necessary; no omission. |
| 2 | 02 architecture-constraints | **Yes** — the target/platform and musl/macOS constraints change. | Listed (`design.md:61`). | `docs/arc42/02-architecture-constraints.md:12,18,19`. | Planned update is necessary; no omission. |
| 3 | 03 context-and-scope | **Yes** — the GitHub release-channel interface and the crew member's download route change. | **Not listed.** | `docs/arc42/03-context-and-scope.md:27-30` says "per-platform archive" and describes the archives served to a downloader. | **Omission** — add chapter 03 and rewrite the archive wording. |
| 4 | 04 solution-strategy | **Yes** — the distribution strategy row and release strategy change. | Listed (`design.md:62`). | `docs/arc42/04-solution-strategy.md:10,22` says five platforms/five executables. | Planned update is necessary. |
| 5 | 05 building-block-view | **Yes** — the release toolbox/workflow building block's description changes. | Listed (`design.md:63`). | `docs/arc42/05-building-block-view.md:58` says "build the five platforms". | Planned update is necessary. |
| 6 | 06 runtime-view | **Yes** — the release runtime flow changes (one build, one smoke, one archive). | Listed (`design.md:64`). | `docs/arc42/06-runtime-view.md:96-111` still shows the five-platform flow. | Planned update is necessary. |
| 7 | 07 deployment-view | **Yes** — deployment topology, artefacts, runners, and crew-member OS change. | Listed (`design.md:65`). | `docs/arc42/07-deployment-view.md:5,12-13,34-50,64,73,81-82`. | Planned update is necessary. |
| 8 | 08 crosscutting-concepts | **Yes** — packaging determinism text names five executables. | Listed (`design.md:66`). | `docs/arc42/08-crosscutting-concepts.md:33`. | Planned update is necessary. |
| 9 | 09 architecture-decisions | **Yes** — ADR-0003 is amended and the register/index change. | Listed (`design.md:67`). | `docs/arc42/09-architecture-decisions.md:13` still titles five platforms. | Planned update is necessary; the ADR amendment is incomplete (Q7). |
| 10 | 10 quality-requirements | **Yes** — QS-007/QS-008/QS-009 name the supported archive set. | Listed (`design.md:68`). | `docs/arc42/10-quality-requirements.md:42` (QS-008) says "same five executables". | Planned update is necessary. |
| 11 | 11 risks-and-technical-debt | **Yes** — R-010/TD-006 are removed and R-009/R-013 change; a new source-build risk is needed. | Listed (`design.md:69`). | `docs/arc42/11-risks-and-technical-debt.md:16,19,35`. | Planned update is necessary; the new source-build risk is missing (Q6/Q7). |
| 12 | 12 glossary | **Yes** — "Supported platform" and "Release archive" change; "Build and test host" is introduced. | Listed (`design.md:70`). | `docs/arc42/12-glossary.md:47,62`. | Planned update is necessary; the new term is missing (Q11). |

**Chapter-file integrity:** all 12 files under `docs/arc42/` exist; each has
real content; no Unicode box-drawing characters and no non-table `+`/`|`
pseudo-diagrams were found. The only integrity blocker is the missing
change-level `arc42.md` marker.

**ADR check:** ADR-0003 is the right target for an amendment (the channel and
platform set are one boundary; ADR-0004 owns immutability). The record's own
`ADR-QUALIFICATION` still says `NEW_ADR`, and the design's amendment list does
not update it. The `Bad, because ... five builds` consequence must be
replaced, and the new source-build consequence must appear in chapter 11.

---

## Resolutions

Every question is resolved. The orchestrator's decisions are applied in
`design.md`, the delta feature and the delta use case; no other artifact was
written.

| Q | Resolution applied |
|---|---|
| Q1 | **No issue.** The grilling missed the permanent conversion scenario `Crew member converts a supported duty plan and receives a calendar file` (`givn/specs/export-duty-plan-calendar/export-duty-plan-calendar.feature:4`), which spawns the production CLI through `CARGO_BIN_EXE_duty2cal` (`apps/cli/tests/steps/export_duty_plan_calendar.rs:83`) and runs `apps/cli/src/main.rs` and the converter on Linux. The coverage section now names that production path as the e2e row's non-zero probe, removes the "none — the e2e scenario launches a shell script" cell, and states that the publish `@e2e` contributes the archive/extraction assertions while the conversion e2e carries the production probe. `measure-coverage.sh` is unchanged and the e2e report remains valid and distinct. |
| Q2 | **Adopted retitle.** The delta replaces the `@givn.modified` e2e scenario with an `@givn.removed` placeholder under the exact old title `Maintainer prepares a release and runs the archived executable` (one step: `Given this scenario is retired`) and a new `@givn.added @e2e` scenario `Maintainer prepares a Windows release and inspects the archived executable`. The use-case Interactions row, the Interaction Coverage Matrix, the traceability table and the evidence slug/directory (`maintainer-prepares-a-windows-release-and-inspects-the-archived-executable`) all use the new title. The proposal names no scenario title and is untouched. |
| Q3 | **Replaced the title grep with a Gherkin-aware helper.** `scripts/assemble_features.py` is invoked by the unchanged `scripts/assemble-features.sh` entry point used by `run-tests.sh` and `measure-coverage.sh`. `design.md` specifies: permanent `givn/specs/**` plus active `givn/changes/*/specs/**`; path mapping to `givn/specs/<usecase>/<capability>.feature`; removal of only the matched permanent file's `@givn.modified`/`@givn.removed` blocks; block parsing from `Scenario:`/`Scenario Outline:` with preceding tags, `Examples:` retention and doc-string awareness; omission of every `@givn.removed` block from copied deltas; loud non-zero failure naming title and file for an unmatched or duplicate title; no on-disk rewrite of permanent files; no macro expansion. Verification is a throwaway fixture smoke plus a real-corpus smoke (one copy per modified title, no placeholder, added scenarios present, scenario count recorded); no permanent script unit tests are added because the repo has no script-test harness. |
| Q4 | **Duplicated scenario deleted.** `A prepared release offers the Windows archive and no other platform archive` and its two dedicated steps were removed from the plan. The modified exact-set table scenario remains the sole evidence for the one-archive rule. |
| Q5 | **Gate strengthened with exact patterns.** `design.md` lists the presence checks (clone address, Windows archive name, `sha256sum --check`, `Get-FileHash`, `gh attestation verify`, MPL-2.0 and licence link, Unicode licence and link, and the three verbatim statements) and the negative checks (no `linux-x86_64`/`linux-arm64`/`macos-arm64`/`macos-x86_64` archive name; no Linux/macOS download-row label). The README's exact source-route sentences are `Other platforms are not supported or distributed; the source can be built with a Rust toolchain.` and `A platform may be added back when users request it.`. `.github/ISSUE_TEMPLATE/bug.yml` and `SECURITY.md` are changed and reviewed by hand; the design records that they are not gated. |
| Q6 | **Promise narrowed, no compile jobs.** The delta use case's goal, rules, examples, out-of-scope and success guarantee say other platforms are not supported, tested or distributed and that their source can be built with a Rust toolchain. The Linux CI/test-host fact moved out of the use-case rules into chapter 2 (and is engineering context in the design). Chapter 11 gains a risk row for unverified source builds on other platforms. No release-matrix compile jobs are added. |
| Q7 | **Complete ADR-0003 amendment planned.** The design enumerates the frontmatter `amended: 2026-10-17` date and opening note, the Qualification verdict (`routing: AMEND_ADR`, target ADR-0003, register/archive-dir check), checklist evidence, Context, Drivers, Considered Options, new title and Decision Outcome, Consequences (five-build cost replaced; source-build/unsupported-route cost added), Confirmation, Pros/Cons and More Information. Chapter 09's register and `adr/README.md` carry the amended title with the 2026-10-17 amendment note; chapter 11 carries the new risk. |
| Q8 | **Chapter 03 added.** `docs/arc42/03-context-and-scope.md` is in the changed-files table, narrowing the GitHub release-channel edge to one Windows x86_64 archive. The design records that the later arc42-docs artifact creates `arc42.md` with the 12-row **Yes** assessment; no `arc42.md` is written here. |
| Q9 | **Root paths corrected.** The three scripts that need no change are `run-tests.sh`, `measure-coverage.sh` and `merge-coverages.sh` at the repository root, and the design states they already delegate assembly to `scripts/assemble-features.sh`. |
| Q10 | **RED drivers classified.** The one-row exact-set table scenario and the added Linux-refusal scenario are the production-change RED drivers (old script stages five archives; old `PLATFORMS` accepts `linux-x86_64`), and the strengthened public-text gate is the shell RED driver. The missing-executable, checksum, unreadable and retitled e2e scenarios are recorded as re-baselined/keep-true evidence, with the exact reason each fails or passes against the old implementation. |
| Q11 | **One outcome and one vocabulary.** The use-case Goal and Success guarantee name one outcome (the release is obtainable and traceable for Windows x86_64). The plan uses "Supported platform" consistently and renames the host helpers to `supported_platform()` / `supported_executable()`; "distribution platform" is gone. No "Build and test host" glossary term is added, because the term does not survive in the use case; the host fact stays in chapter 2 and the design. |
| Q12 | **Transcript flow specified and the committing step named.** Preparation (archive, digest and release-text line) → `tar -tzf` through `run_command` so the member listing is in the transcript → exact-member assertion → extracted-bytes assertion → commit. The final Then (`the extracted executable is the executable the preparation staged`) calls `commit_transcript(w)`. The Visual Design Contract baseline lists exactly what is captured. |
| Q13 | **Always the deterministic stand-in.** The e2e never uses the host binary; the Given writes fixed stand-in bytes. The digest assertion is three-way (reported digest == `.sha256` file == computed archive digest), and the extracted bytes are compared with the staged stand-in. The step is renamed `the extracted executable is the executable the preparation staged`; no digest scrub is needed. |
| Q14 | **Dead linker configuration removed.** The release workflow plan deletes the `linker` matrix field, both `CARGO_TARGET_*_UNKNOWN_LINUX_MUSL_LINKER` env lines and their comment; the `apt`/mingw-w64 install stays. |
| Q15 | **README plan completed.** Only future releases are Windows-only; the text states that already-published releases keep their original assets and links. The `chmod` note, macOS `shasum` example, "all five platforms" and macOS approval section are removed and replaced with the Windows route and the unsupported-source-build statement. |
| Q16 | **Ledger corrected.** D2 names the README sentence the change adds (`Already-published releases keep their original assets and links; only future releases are Windows-only.`) and ADR-0004; D1 records the `smoke-windows` job on `windows-latest` in CI and the release workflow as host evidence, not a change artifact. |
| Q17 | **Stale wording and example fixed.** The exact-set failure message becomes `the staged set is not the one Windows archive, its checksum, the release text and the change log`; the single-scenario example uses the surviving one-row table scenario. The task greps the step files for remaining "five" wording. |
| Q18 | **Given clarified.** The scenario keeps its title and uses `Given no executable was built for the supported platform`; a dedicated step builds no binaries, and the Then asserts the refusal names `windows-x86_64`. |

All hardening edits are applied. The plan is internally consistent: the delta's
six executable scenarios carry `@wip`, the retired placeholder is never
assembled, the coverage claim is grounded in the permanent conversion e2e, and
the ADR, chapter and gate plans name the exact files and patterns the tasks
will implement.

DESIGN-REVIEW: PASS
