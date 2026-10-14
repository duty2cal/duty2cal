# Tasks: separate-position-marker-and-flight-number

Release note (commit subject): A travel activity's calendar title prints the crew position and the flight number as two words separated by a space — an operating leg reads `FB 123 ABC-DEF` instead of `FB123 ABC-DEF`, while positioning travel keeps reading `DH 007 ABC-DEF`.

Owning use case: [Export a duty-plan calendar](specs/export-duty-plan-calendar/usecase.md) (`export-duty-plan-calendar`). Confirmed Personas: `crew-member--personal-calendar-owner` (accuracy lens — the title and the failure text a crew member reads), `project-maintainer--release-owner` (release-evidence lens — untouched by this change). Actors in scenarios stay distinct from Personas.

Single-scenario run command (used by every RED/GREEN/REFACTOR below): `./run-tests.sh --name "<scenario title>"`.
Full suite: `./run-tests.sh`. E2E-only suite: `./run-tests.sh --e2e`.

Use-case constraints carried into every scenario task (from the owning use case's Rules):

- The converted plan must keep representing every required activity; this change alters only the text a travel leg's title carries, the text a failure names, and which two printed rows pair.
- Printed values are preserved (leading zeroes included) and are never converted into a numeric quantity merely for formatting.
- Deadhead duty stays one travel event whose title carries the printed position (`DH 007 ABC-DEF`); it is not a second event and not time off.
- Every conversion failure names the source and the reason, with a numbered diagnostic attempted; this change never turns a failure into success or the reverse except for the pairing rule the design records.
- Private plans are never committed: every fixture uses generated values, and the local corpus is read only through `scripts/validate-local-corpus.sh`.

Interaction coverage (checked before writing tasks): the permanent inventory holds one entry — "supply a duty plan for conversion" — matched by the `@e2e` scenario "Crew member converts a supported duty plan and receives a calendar file", and `design.md`'s Interaction Coverage Matrix holds that row with the CLI subprocess as its driving mechanism. This change adds no consumer action, so no new `@e2e` scenario and no new matrix row; the delta's five scenarios are regular scenarios of that one action.

## Setup task — environment, runner, strict mode

- [x] Confirm the toolchain and the runner the design names, and record the proof that strict mode is on.
  - The toolchain is the nix-store Rust 1.95.0 with a gcc 15.3.0 linker (this machine has no cargo/cc on `PATH`).
  - The change-log scenarios of the release capability drive `scripts/render-changelog.sh`, which refuses any `git-cliff` other than the pinned 2.14.2; the nix-store `git-cliff 2.14.2` is placed on `PATH` for every run below. Without it, five scenarios of `publish-public-release` fail for a reason unrelated to this change.
  - Strict mode is `.fail_on_skipped()` in `apps/cli/tests/features_runner.rs`; the step definition files already exist, one per capability (`apps/cli/tests/steps/export_duty_plan_calendar.rs`, `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`), and `givn/commands.yaml` already names `verify.command: ./run-tests.sh` and `verify.e2e_command: ./run-tests.sh --e2e`.

  Proof of strictness and of the baseline (this is the RED state of all five delta scenarios before any step definition exists):

  ```
  ./run-tests.sh
  3 features
  99 scenarios (78 passed, 21 failed)
  336 steps (315 passed, 21 failed)
  ...
  thread 'main' panicked at .../cucumber-0.23.0/src/cucumber.rs:1243:13:
  21 steps failed
  error: test failed
  exit=101
  ```

  The failing scenarios are exactly the delta's five (`Step doesn't match any function`); the 95 permanent scenarios pass. Re-run below with `git-cliff` on `PATH` narrows the failures to the delta's five alone:

  ```
  ./run-tests.sh        ->  4 steps failed   (exit 101)
  failing: A crew position and its flight number are printed as separate words
           A crew position printed beside a non-flight activity stays out of its title
           An unmatched travel fragment is named with its position and flight number separate
           A travel leg that prints no crew position keeps its flight number alone
  ```

## Scenario tasks

RED/GREEN evidence is captured per scenario. Scenarios marked *(guard)* assert behaviour the current production code already produces; their RED is the undefined step, and their GREEN requires no production code — they exist so the new rule cannot over-reach (see `design-review.md` R3).

### 1. A positioning departure fragment has no partner when its arrival fragment prints no crew position

- [x] RED: remove `@wip` from this scenario only; add its `Given` step (a departure fragment printing `Pos=DH` and flight number `124`, an arrival fragment printing flight number `124` and no `Pos`); run `./run-tests.sh --name "A positioning departure fragment has no partner when its arrival fragment prints no crew position"`; paste the output; the scenario must fail because a calendar is published where the delta expects a refusal.

  ```
  Feature: Export a duty-plan calendar
    Scenario: A positioning departure fragment has no partner when its arrival fragment prints no crew position
     ✔  Given a supported duty plan whose departure fragment prints crew position DH and flight number 124 and whose arrival fragment prints flight number 124 without a crew position
     ✔  When the crew member converts it
     ✘  Then no calendar is published
        Step failed:
        Matched: apps/cli/tests/steps/export_duty_plan_calendar.rs:109:1
        Step panicked. Captured output: a calendar was published: ["roster.ics"]
  1 scenario (1 failed) / 3 steps (2 passed, 1 failed)
  error: test failed
  ```

  The first attempt at this scenario printed `FB` on the departure fragment and passed unchanged: an operating fragment already had to match the partner's printed position, so the only behaviour this change moves is the positioning leg, whose marker the old code suppressed. The scenario was reworded to `DH` and is RED as pasted.
- [x] GREEN: make the pairing key the printed pair — a `Row` carries the printed position cell beside the joined code, and a departure pairs with an arrival only when both the code and the printed position are equal. Production files: `crates/duty2cal-core/src/application/mod.rs`. Run the same single-scenario command; paste the output.

  ```
  1 scenario (1 passed) / 4 steps (4 passed)
  ```
- [x] REFACTOR: no behaviour change; keep the doc comments truthful. Run the same single-scenario command; paste the output.

  ```
  ./run-tests.sh  (whole suite, after the edit)
  3 features
  100 scenarios (96 passed, 4 failed)
  370 steps (366 passed, 4 failed)
  failing: the four delta scenarios that are still @wip — no permanent scenario regressed.
  ```
- [x] COMMIT: `fix(export-duty-plan-calendar): <release note>` with this scenario's title in the body; record the revision identifier.

### 2. A crew position and its flight number are printed as separate words

- [x] RED: remove `@wip` from this scenario only; add its `Given` step (a travel leg printing crew position `FB` and flight number `123` from `ABC` to `DEF`); run `./run-tests.sh --name "A crew position and its flight number are printed as separate words"`; paste the output; the scenario must fail with the spliced title `FB123 ABC-DEF`.

  ```
  SUMMARY:FB123 ABC-DEF
  LOCATION:ABC-DEF
  1 scenario (1 failed) / 3 steps (2 passed, 1 failed)
  error: test failed
  ```
- [x] GREEN: the two-field model — the travel value carries the plan's printed position and activity, one `printed_identification()` rule reads them together, the travel title uses it, and the deadhead special case plus the retired `Position` are deleted. Regenerate the reference calendar surgically: only the twelve travel `SUMMARY` lines change, each gaining exactly one space, with every `UID`, instant, `LOCATION` and non-travel title byte-identical. Reword the "preserve printed designators" rule in the owning use case. Production files: `crates/duty2cal-core/src/domain/mod.rs`, `crates/duty2cal-core/src/application/mod.rs`. Run the single-scenario command and the golden-oracle scenario; paste both.

  ```
  Scenario: A crew position and its flight number are printed as separate words
  ✔ Then that activity's title is "FB 123 ABC-DEF"      1 scenario (1 passed) / 3 steps (3 passed)

  Scenario: The reference roster converts to the expected calendar
  ✔ Then the published calendar equals the expected calendar after CRLF-to-LF and folding normalisation
                                                        1 scenario (1 passed) / 3 steps (3 passed)

  git diff --stat apps/cli/tests/fixtures/expected/2098-06.ics
   1 file changed, 12 insertions(+), 12 deletions(-)
  ```
  The fixture diff is surgical: twelve `SUMMARY` lines, each `LU742 …` → `LU 742 …`, and the diff shows no `UID`, `DTSTART`, `DTEND`, `DTSTAMP`, `LOCATION` or non-travel line changed.
- [x] REFACTOR: no behaviour change — the fixture builder's split helper and its comments name the two printed fields. Run the single-scenario command; paste the output.

  ```
  ./run-tests.sh (whole suite)
  3 features
  100 scenarios (97 passed, 3 failed)
  372 steps (369 passed, 3 failed)
  failing: the three delta scenarios still @wip — no permanent scenario regressed, including the deadhead scenario and the golden oracle.
  ```
- [x] COMMIT: `fix(export-duty-plan-calendar): <release note>` with this scenario's title in the body; record the revision identifier.

### 3. An unmatched travel fragment is named with its position and flight number separate

- [x] RED: remove `@wip` from this scenario only; add its `Given` step (an unmatched travel fragment printing crew position `FB` and flight number `123`); run `./run-tests.sh --name "An unmatched travel fragment is named with its position and flight number separate"`; paste the output; the scenario must fail because the failure names `123` rather than the printed pair.

  ```
  Step panicked. Captured output: the failure does not name "FB 123": conversion failed for
  /tmp/duty2cal-tests-1189205-0/roster.pdf: a required activity could not be resolved:
  no unique compatible partner for travel fragment "123"
  1 scenario (1 failed) / 4 steps (3 passed, 1 failed)
  error: test failed
  ```
- [x] GREEN: the failure message names the fragment through the same `printed_identification()` rule. Production files: `crates/duty2cal-core/src/application/mod.rs`. Run the single-scenario command; paste the output.

  ```
  Scenario: An unmatched travel fragment is named with its position and flight number separate
  ✔ And the conversion failure names "FB 123"            1 scenario (1 passed) / 4 steps (4 passed)
  ```
- [x] REFACTOR: no behaviour change. Run the single-scenario command; paste the output.

  ```
  ./run-tests.sh (whole suite)
  3 features
  100 scenarios (98 passed, 2 failed)
  375 steps (373 passed, 2 failed)
  failing: the two guard scenarios still @wip — the permanent "LU900" and compatibility failures, whose rows print no position, still name exactly what they did.
  ```
- [x] COMMIT: `fix(export-duty-plan-calendar): <release note>` with this scenario's title in the body; record the revision identifier.

### 4. A travel leg that prints no crew position keeps its flight number alone *(guard)*

- [x] RED: remove `@wip` from this scenario only; add its `Given` step (a travel leg printing flight number `123` and no crew position); run `./run-tests.sh --name "A travel leg that prints no crew position keeps its flight number alone"`; paste the output.

  ```
  Step failed:
  Defined: tmp/tmp.60sGZbhtAx/f0003.feature:11:5
  Step doesn't match any function
  1 scenario (1 failed) / 1 step (1 failed)
  error: test failed
  ```

  The RED here is the undefined step, not a behavioural failure: this scenario asserts what the code already did, so it exists to keep the join rule from reaching a leg that prints no position. The step registration was stashed for this capture and restored immediately after.
- [x] GREEN: no production change is required — the join rule leaves an unprinted position out. Record that finding and run the single-scenario command; paste the output.

  ```
  Scenario: A travel leg that prints no crew position keeps its flight number alone
  ✔ Then that activity's title is "123 ABC-DEF"           1 scenario (1 passed) / 3 steps (3 passed)
  ```

  Production files: none. The behaviour was already produced before this change (the join only ever fired with a printed position), and `design.md` records that no digit guard replaces it: a travel title prints whatever the two fields hold.
- [x] REFACTOR: none; run the single-scenario command; paste the output.

  ```
  1 scenario (1 passed) / 3 steps (3 passed)
  ```
- [x] COMMIT: `test(export-duty-plan-calendar): <release note>` with this scenario's title in the body; record the revision identifier.

### 5. A crew position printed beside a non-flight activity stays out of its title *(guard)*

- [ ] RED: remove `@wip` from this scenario only; add its `Given` step (a dated row printing crew position `AK` beside activity code `EM`); run `./run-tests.sh --name "A crew position printed beside a non-flight activity stays out of its title"`; paste the output.
- [ ] GREEN: no production change is required — a non-flight activity carries no position into its title. Record that finding and run the single-scenario command; paste the output.
- [ ] REFACTOR: none; run the single-scenario command; paste the output.
- [ ] COMMIT: `test(export-duty-plan-calendar): <release note>` with this scenario's title in the body; record the revision identifier.

## Documentation task

- [ ] Update the durable architecture documentation for this change and commit it: `docs/arc42/05-building-block-view.md` (the row interpretation's responsibility), `08-crosscutting-concepts.md` (the printed identification of a travel leg and the pairing rule), `10-quality-requirements.md` (QS-013), `11-risks-and-technical-debt.md` (TD-010), `12-glossary.md` (**Crew position**, **Flight number**, **Travel leg**, **Deadhead duty**). Record the revision identifier.

## Verification task

- [ ] `git-cliff` on `PATH`: `./run-tests.sh` → every scenario passes; paste the summary line.
- [ ] `./run-tests.sh --e2e` → the `@e2e` scenario passes; paste the summary line.
- [ ] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` → clean; paste.
- [ ] `scripts/validate-local-corpus.sh` with a release build → the same five plans convert, and a diff of the published titles against the pre-change baseline shows only travel titles gaining one space; paste.
- [ ] `scripts/pii-guard.sh`, `givn lint --change separate-position-marker-and-flight-number` → clean; paste.
