# Review: report-row-diagnostics

Completeness review of the committed candidate at `c3db77a` (29 delta scenarios,
22 non-browser implementation commits plus the browser series). The suite is not
re-run here; the recorded execution evidence and the change verification
receipt are used, and every other check is static inspection of the committed
tree.

## Fabrication audit

### Step 0 — `@e2e` tag integrity

- The delta `.feature` file contains **zero** `@e2e` scenarios: 2 `@givn.removed`,
  20 non-browser `@givn.added`, and 8 `@givn.added @browser` scenarios.
- The permanent feature keeps both of its `@e2e` scenarios: the CLI action at
  line 5 (`@e2e`) and the browser action at line 443 (`@e2e @browser`).
- `git diff 23ac216 HEAD -- givn/specs/export-duty-plan-calendar/export-duty-plan-calendar.feature | grep -E '^[-+].*(@e2e|Scenario:)'`
  produces no output: this change neither added, removed, nor moved an `@e2e`
  tag on the permanent spec. No tag was dropped to bypass the e2e gate.

### Step 1 — empty / trivial step bodies

```
$ grep -rnE '^\s*(async )?fn [a-z0-9_]+\([^)]*\)( -> [^{]+)? \{\s*\}\s*$' apps/cli/tests/steps
(no matches)
```

**0 empty step bodies found across 5 step-definition files**
(`steps/export_duty_plan_calendar.rs`, `steps/browser.rs`,
`steps/e2e/export_duty_plan_calendar.rs`, `steps/e2e/visual_baseline.rs`,
`steps/support.rs`; `steps/mod.rs` holds module declarations only). Every new
step asserts against the published calendar, the report text, the transcript,
the rendered DOM, the driver's observation or a fixture invariant; none is a
bare `pass`/`return` or a no-op.

### Step 2 — commit cross-check

Every checked `[x]` scenario task has a commit. The diagnostic model, the
three-code fallback, the report builders, the CLI printing and the file-name
kind all landed in the task-1 production commit `f90cea2`; the route-incomplete
catch-all landed in `1f95728`. The later per-scenario commits are the RED/GREEN
pin for their branch — a real fixture plus real assertions in
`apps/cli/tests/steps/export_duty_plan_calendar.rs` and the `@wip` flip — and
several tasks state "Production: none beyond …" because their branch was already
delivered. No task's commit is an empty or stub-only diff.

| Task | Commit | Diff touches | Production lineage |
|---|---|---|---|
| setup | (pre-`f90cea2` working state) | runner, strict-mode proof | existing `features_runner.rs` |
| 1 Timed fallback warning | `f90cea2` | `application/mod.rs` (452), `domain/mod.rs`, `fs.rs`, `main.rs`, steps | production in commit |
| 2 All-day fallback warning | `d23996b` | steps + feature | `f90cea2` |
| 3 One-endpoint error | `04792e3` | steps + feature | `f90cea2` |
| 4 Several unrecognized rows | `f4522a8` | steps + feature | `f90cea2` |
| 5 Several failing rows | `d8a12b9` | steps + feature | `f90cea2` |
| 6 Failure-report sections | `fef3107` | steps + feature | `f90cea2` |
| 7 Warning-report collision | `8da8473` | steps + feature | `f90cea2` (`Kind::Warning`), `fs.rs` |
| 8 Executable prints warnings | `8a96826` | steps + feature | `f90cea2` (`main.rs`) |
| 9 No warning report without warnings | `7afb005` | steps + feature | `f90cea2` |
| 10 Unsaveable warning report | `8f6bd27` | steps + feature | `f90cea2` (`publish_warnings`) |
| 11 Page warnings + both offers | `1e99782` | `ffi/wasm`, `web/app.js`, `web/index.html`, `web/styles.css`, `driver.mjs`, steps | production in commit |
| 12 Page error + warning lists | `c261491` (test change `d446780`) | `web/app.js`, `web/styles.css` | production in commit |
| 13 Page hides empty warning list | `786fa0e` | steps + feature | `1e99782` |
| 14 Route + incomplete interval | `1f95728` | `application/mod.rs`, steps | production in commit |
| 15 Unrecognized code + route + one endpoint | `6048f3b` | steps + feature | `1f95728` |
| 16 Known code + route stays timed | `1b4a1a2` | steps + feature | classifier order in `f90cea2` |
| 17 Flight leg no warning | `0091e91` | steps + feature | classifier order in `f90cea2` |
| 18 Silent layout row | `b1569cd` | steps + feature | silent skip in `f90cea2` |
| 19 Raw row numbering | `69ad15a` | steps + feature | `f90cea2` |
| 20 Plan-order diagnostics | `ef83378` | steps + feature | `f90cea2` (`main.rs` reason) |
| 21 Overnight fallback | `39d99a9` | steps + feature | `row_interval` in `f90cea2` |
| 22 Equal window error | `6deba99` | steps + feature | `row_interval` in `f90cea2` |
| 23 Mis-paired leg two errors | `af910a2` | steps + feature | `f90cea2` |
| 24 No description on fallback | `d0c0d6d` | steps + feature | `f90cea2` (`domain`) |
| 25 Warning-report header | `d9ba755` | steps + feature | `warning_report_text` in `f90cea2` |
| 26 Ready count sentence | `f217209` | steps + feature | `1e99782` / `c261491` |
| 27 Offered report content | `66da742` | steps + feature | `ffi/wasm` + `web/app.js` in `1e99782` |
| 28 Second attempt clears lists | `9b7f0fa` | steps + feature | `clearAttempt` in `1e99782`/`c261491` |
| 29 Warning-offer failure kept | `f880e20` | steps + feature | `web/app.js` + `driver.mjs` in `1e99782` |

### Step 3 — promised components exist

| Component (from `design.md`) | Exists | Path / evidence |
|---|---|---|
| `Severity`, `Diagnostic` | yes | `crates/duty2cal-core/src/application/mod.rs:64,73` |
| `diagnostic_line`, `failure_report_text`, `warning_report_text` | yes | same file, lines 108, 116, 129 |
| `ConversionOutcome.warnings` | yes | same file, used by `main`/wasm |
| `ConvertError::{Table, Rows}` | yes | same file, lines 164–194, 285, 356 |
| `DutyEntry::UnrecognizedActivity` + accessor arms | yes | `crates/duty2cal-core/src/domain/mod.rs:180–240` |
| `Kind::Warning` (`<stem>.parse-warnings[-N].txt`) | yes | `apps/cli/src/fs.rs:65,74` |
| CLI prints each line and the report path | yes | `apps/cli/src/main.rs:71–105,109–127` |
| WASM success/failure shape with `warnings`/`warningReport` | yes | `ffi/wasm/src/lib.rs:71–117` |
| Page list containers and state | yes | `web/index.html:57,68,72`; `web/app.js:33–35,179–200` |
| Driver `lists`, `ready-with-warnings`, `warnings` offer | yes | `web/e2e/driver.mjs` (`readState`, `save_offer`) |
| Diagnostic-list styling | yes | `web/styles.css` (`.diagnostics`) |

No promised component is missing.

### Step 4 — strict-mode proof

Present in the tasks.md setup task: `apps/cli/tests/features_runner.rs` calls
`DutyWorld::cucumber().fail_on_skipped().run_and_exit(root)`; the recorded RED
run selected one scenario and failed with `Step doesn't match any function`,
exit non-zero. `run-tests.sh` also refuses a run that selects no scenario and
requires a named run to select exactly one.

### Step 5 — every `@e2e` scenario's Then steps

| `@e2e` scenario | Primary assertion | Repository-only? |
|---|---|---|
| Crew member converts a supported duty plan and receives a calendar file (CLI) | Reads the published calendar text (`BEGIN:VCALENDAR`/`VEVENT`/`SUMMARY:LU742 ABC-DEF`/`DTSTART`) and the CLI transcript naming `Written: <path>` | no — CLI output and published file |
| Crew member converts a supported duty plan in the browser and saves the offered calendar (Web UI) | Reads the browser-captured download, checks CRLF/VCALENDAR structure and byte-equal `VEVENT` blocks against the reference calendar; asserts rendered states, request log, no-account baseline and visual baseline | no — browser-produced download and rendered page |

Neither is an integration test wearing `@e2e`; both assert on the named real
interface.

### Step 6 — browser driving mechanism

The browser `@e2e` steps send driver commands through `BrowserDriver`
(`steps/export_duty_plan_calendar.rs:3716`) to `web/e2e/driver.mjs`, which uses
Playwright: `page.setInputFiles('#plan', params.path)` (`driver.mjs:650`),
`save_offer`/`capture_offer` for the download, `read_state` for the DOM and
request log, `axe`, and `screenshot`. No project step definition sends an HTTP
request or calls `fetch()`/`XMLHttpRequest`; the only `fetch`/XHR matches are
inside `web/e2e/node_modules/playwright`. This is a real browser driver.

### Step 7 — the exact files `verify.e2e_command` invokes, and the parallel-implementation search

`givn/commands.yaml` has `verify.e2e_command: "./run-tests.sh --e2e"`. Read
literally, `run-tests.sh` translates that to
`--tags "@e2e and not @browser"` and runs
`cargo test -p duty2cal --test features_runner` over the features assembled by
`scripts/assemble-features.sh` (permanent `givn/specs/**` plus active change
`specs/**`). The bindings are
`apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` (e2e-only steps) and
`apps/cli/tests/steps/export_duty_plan_calendar.rs` (shared Given/When/Then).

Parallel-implementation search:
`grep -rln 'cucumber' --include='*.rs' .` returns only
`apps/cli/tests/features_runner.rs`, `apps/cli/tests/golden.rs` (cucumber-free
oracle), `steps/export_duty_plan_calendar.rs`, `steps/e2e/export_duty_plan_calendar.rs`
and `steps/legal_notices.rs` (another capability). `git status --porcelain` is
clean, so there is no untracked second e2e implementation. There is no weaker
duplicate and no stronger implementation being ignored.

### Step 8 — e2e scope

The inventory (`usecase.md` `## Interactions`) has two normalized actions:
supply a plan to the executable, and supply a plan through the page and save
the calendar. Those are the only two `@e2e` scenarios in the permanent feature
(one CLI, one browser), one per action. The delta adds no action and therefore
no `@e2e` scenario; the eight `@browser` scenarios are regular coverage of the
second action (list rendering, second offer, stale-list clearing, offer
failure). No over-production.

### Step 9 — e2e isolation

| `verify.command` | `verify.e2e_command` | Identical? | Regular scenarios | E2E scenarios | Strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | no | 114 | 1 | yes (1 < 114) |

The e2e command carries its own `@e2e and not @browser` filter; it is not a
copy of the full command. The 114 count matches `givn check tasks`' change
verification receipt (`result.json`: `{"scope":"regular","total":114,"passed":114}`)
and the recorded full run in the task evidence.

### Step 10 — design conformance (commands, layout, framework/driver)

| Decision (from `design.md`) | `design.md` says | Built | Matches? |
|---|---|---|---|
| E2E command | `./run-tests.sh --e2e` (`@e2e and not @browser`) | same | yes |
| Browser suite | `./run-tests.sh --browser` builds `web/pkg` + `dist/site` | same in `run-tests.sh` | yes |
| E2E step file | `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`; regular browser steps in `apps/cli/tests/steps/export_duty_plan_calendar.rs` | same two files, one per layer | yes |
| Browser driver | Playwright over `web/e2e/driver.mjs` | same; `set_file`/`save_offer`/`read_state` | yes |
| No new `@e2e` | D10: the delta adds no `@e2e` scenario and retags none | delta has zero `@e2e`; permanent tags unchanged | yes |
| Report builders in application | `failure_report_text`, `warning_report_text`, `diagnostic_line` | same functions, both facades call them | yes |
| `Kind::Warning` spelling | `(".parse-warnings", "txt")` in `spelling()` | same at `fs.rs:74` | yes |
| File layout | crates/`apps/cli`/`ffi/wasm`/`web` unchanged | no crate/module/dependency added | yes |

One documentation-format defect was found in this section during the first
review and has been remediated in `c3db77a`: `design.md`'s `## Step Definitions`
table now reads `| Capability | Step definition file | Module | Permanent merge
target | New or changed phrase groups |`, so the review gate's promised-files
check resolves the real path from column 2 and every named file exists. The
design re-review records `DESIGN-REVIEW: PASS`. No implementation deviation was
found.

### Step 11 — interaction coverage cross-reference

The inventory is the owning use case's `## Interactions` table; the design's
`Interaction Coverage Matrix` has both rows and no extra row.

| Inventory entry | Matrix row | `@e2e` scenario | Promised mechanism | Actual mechanism (step defs) | Match |
|---|---|---|---|---|---|
| supply a duty plan for conversion through the executable | yes | Crew member converts a supported duty plan and receives a calendar file | `std::process::Command` runs the built `duty2cal` binary; transcript + published calendar | shared `the crew member converts it` runs the binary; e2e steps read the transcript and the published `.ics` | yes |
| supply a duty plan through the converter page and save the offered calendar | yes | Crew member converts a supported duty plan in the browser and saves the offered calendar | Playwright via `web/e2e/driver.mjs`: navigate, file input, save offer, request log, rendered state | `set_file`, `save_offer`, `read_state` through the driver; no HTTP client | yes |

The new warning/error visibility is regular coverage of the second action and
asserts invariants distinct from the save scenario (the lists and the second
offer), exactly as the matrix declares.

## Use-case and Persona conformance

| Contract | Evidence | Pass |
|---|---|---|
| Goal, trigger and primary actor preserved | `## Goal` is one verb phrase; `## Trigger` names the Crew member; `## Actors` has the Crew member as primary | yes |
| Guarantees, rules, typed relationships preserved | D8's rule replacement is recorded in place ("This rule replaces the earlier rule…"); `## Extends`/`## Includes` remain `none`; one capability, goal-shaped | yes |
| Capability ownership preserved | Only `export-duty-plan-calendar`; proposal routing table says `EXTEND`; page is a realization, not a second goal | yes |
| Confirmed Persona outcomes preserved | See the visual block; accuracy lens and web-review lens both PASS | yes |
| Actors remain interaction roles; Personas review lenses | Gherkin names the Crew member, never a Persona id | yes |
| Migration ledger | not applicable — no migrated capability | n/a |

Scope rejection criteria re-checked on the changed use case: Goal is a single
verb phrase; Success guarantee names one export outcome with two artifacts and
explicitly justifies why that is one goal; Actors/Trigger identify the primary
actor; the capability passes the placement test; the use case is a changing
goal, so no observing/mutation conflict; every Extension names its main-flow
step; preconditions are checked in the flow rather than assumed; capability and
use-case names are goal-shaped, not component- or channel-shaped; the main flow
names actor goals rather than three implementation goals; no `Extends` is used.

PERSONAS: crew-member--personal-calendar-owner
PERSONAS: contributor--web-reviewer

`project-maintainer--release-owner` is the pre-existing use-case lens; it has
no release-artifact, dependency, deployment or golden-fixture impact in this
change and is not a confirmed Persona of the new surface.

## Deferred mandatory checks

None. No mandatory check is deferred, so no `DEFERRAL:`/`OPERATOR-DECISION:`
pair is recorded.

## Visual review

**Contract:** web interface from `design.md` `## Visual Design Contract`:
viewports mobile 360×780 and desktop 1920×1080; states `ready`,
`ready-with-warnings`, `failure` (plus unchanged `empty`); reference as drawn
(operator approval 2026-10-27).

**Evidence completeness.** Every delta `@browser` scenario commits final-state
screenshots for both viewports; the permanent `@e2e` browser scenario commits
its `ready` and `empty` states. 18 committed PNGs under
`givn/changes/report-row-diagnostics/evidence/visual/`:

| State directory | Files |
|---|---|
| `the-page-shows-the-warnings-and-offers-both-files-when-a-plan-converts-with-warnings` | `mobile--ready-with-warnings.png`, `desktop--ready-with-warnings.png` |
| `the-page-lists-the-errors-and-the-warnings-when-a-plan-cannot-be-converted` | `mobile--failure.png`, `desktop--failure.png` |
| `the-page-shows-no-warning-list-when-the-plan-converts-without-warnings` | `mobile--ready.png`, `desktop--ready.png` |
| `the-page-states-how-many-rows-were-converted-with-warnings` | `mobile--ready-with-warnings.png`, `desktop--ready-with-warnings.png` |
| `the-page-offers-a-warning-report-that-names-the-source-and-carries-the-warnings` | `mobile--ready-with-warnings.png`, `desktop--ready-with-warnings.png` |
| `a-second-attempt-clears-the-previous-attempt-s-warning-list` | `mobile--ready.png`, `desktop--ready.png` |
| `the-page-keeps-the-calendar-and-the-warnings-when-it-cannot-offer-the-warning-report` | `mobile--ready-with-warnings.png`, `desktop--ready-with-warnings.png` |
| `crew-member-converts-a-supported-duty-plan-in-the-browser-and-saves-the-offered-calendar` | `mobile.png`, `desktop.png`, `mobile--empty.png`, `desktop--empty.png` |

Both fresh-context Persona reviews saw every file above, including both
failure viewports.

**Baseline.** Each `@browser` scenario asserts the baseline via the driver's
`read_state` (`assert_visual_baseline`), axe (`assert_axe_clean`) and the
screenshot capture: labelled controls, no internal identifier or state name in
visible text, one `h1` with a real heading, no duplicated visible content, a
failure heading with a way back (`Choose another plan`), `lang="en"`, and the
computed reduced-motion duration. No blocker was raised against the baseline.

**Persona findings and dispositions.**

| Persona | Finding | Disposition |
|---|---|---|
| `contributor--web-reviewer` (review B) | Blocker: the failure-state report offer overpainted the last Warnings entry and the reset button overlapped the offer at both viewports | Fixed in `c261491`; re-read of both regenerated failure PNGs confirmed no overlap and no new blocker |
| `contributor--web-reviewer` | Improvement: a non-reference `No calendar was created.` line | Fixed in `c261491` (copy now follows the reference) |
| `contributor--web-reviewer` | Improvement: missing reference note between the offer and the reset | Fixed in `c261491` |
| `crew-member--personal-calendar-owner` (review A) | Blocker: the offer overpainted the last Warnings entry and the label was not contained at 360 px | Fixed in `c261491`; re-read confirmed containment, no clipping, no horizontal scroll |
| `crew-member--personal-calendar-owner` | Improvement: offer and reset flush without separation | Fixed in `c261491` |
| `crew-member--personal-calendar-owner` | Improvement (fixed as drawn): the two offers are distinguished by filename plus the adjacent Warnings heading | Accepted as drawn in the approved reference |

Every finding names a contract rule (layout containment, the failure panel's
copy and way back) or a failed Persona goal (reading the warnings and reaching
the report), not a pixel measurement.

VISION-REVIEW B: PASS
VISION-REVIEW A: PASS

VISUAL-REVIEW: PASS

## Retirements

No capability is retired. Two delta scenarios are retired under the replaced
rule (see the removed+added pairs below); that is a scenario retirement, not a
capability retirement.

RETIRE: none — no capability is retired by this change.

## Arc42 implementation conformance

The durable chapters and the change-level `arc42.md` were read against the
completed implementation, `design.md` and `tasks.md`. Rows 2 and 7 were checked
and are genuinely unaffected (no constraint/infrastructure change; no new
dependency, target, port or package).

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Goals and quality goals distinguish conversion error from warning | `01-introduction-and-goals.md:11` | Yes — requirement 2 / quality goal 1 | D1, D2 | 1–10 | `Severity`; warning/failure scenarios | yes |
| Constraints | `02-architecture-constraints.md` | No | "Technology decisions" — no version/constraint change | setup | no `Cargo.toml` dependency diff | yes |
| Context/scope gains warning report and page lists | `03-context-and-scope.md:11,17,198,208` | Yes | D5, D6 | 1, 7, 8, 11 | `Kind::Warning`, page offer/list | yes |
| Solution strategy: one-pass diagnostics, second report kind | `04-solution-strategy.md:6,9,10` | Yes | Aggregation algorithm | 1, 5, 11 | `interpret` collects `Diagnostic`s; `Rows` | yes |
| Building blocks: `UnrecognizedActivity`, `Diagnostic`/`Severity`, report builders, `Kind::Warning`, page lists | `05-building-block-view.md:242–257` | Yes | Data model, Architecture impact | 1, 7, 11 | files/line references in Step 3 above | yes |
| Runtime flow: conversion with warnings publishes a warning report; failure processes every row | `06-runtime-view.md:39,43–72` | Yes | Aggregation/Report text | 1, 5, 8, 10 | `main.rs` print/publish; wasm/page | yes |
| Deployment | `07-deployment-view.md` | No | unchanged | — | binaries/Site unchanged | yes |
| Cross-cutting: severity, complete-or-explicit failure, no-clobber | `08-crosscutting-concepts.md:19,27` | Yes | Report text, File naming | 1, 7, 10 | builders + `fs::publish` | yes |
| Architecture decisions: one candidate `NOT_QUALIFIED`, routed to the use case; no new ADR | `09-architecture-decisions.md:99,164–168` | Yes | Decisions ledger | — | ADR register/index unchanged | yes |
| Quality scenarios QS-045/QS-046 | `10-quality-requirements.md:109–110` | Yes | Persona/Visual contract | 5, 11, 12, 28, 29 | aggregation and browser scenarios | yes |
| Risks R-070/R-074/R-075, TD-029 | `11-risks-and-technical-debt.md:79,90,91,124` | Yes | Visual verification posture, Decisions | browser tasks, closeout | nightly `@browser` posture unchanged; no ADR consequence | yes |
| Glossary: Detail row, Conversion error/warning, Failure report, Warning report, Silent layout row | `12-glossary.md:47–198` | Yes | Data model, Report text | 1 | code symbols match the glossary's mapping | yes |

- [x] Every affected chapter was opened and checked for stale/placeholder content.
- [x] Every relevant fact is represented in `design.md` and mapped to tasks.
- [x] The implementation follows those facts (evidence names built files/results).
- [x] No architecture change discovered outside the recorded chapters.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The durable glossary holds **Detail row**, **Conversion error**, **Conversion
warning**, **Failure report**, **Warning report** and **Silent layout row**.
The specs, design and code reuse them consistently: the feature says
"conversion warning"/"conversion error"/"warning report"/"failure report"/
"detail row"; `design.md` uses the same terms and maps them to
`Severity::Error`/`Severity::Warning`, `Diagnostic`, `warning_report_text`,
`failure_report_text`; the code symbols the glossary names exist at the stated
paths. No spec or design term is missing from the glossary and no inconsistent
reuse was found. `docs/arc42/12-glossary.md` needs no correction.

UBIQUITOUS-LANGUAGE: CLEAN

## Coverage

**Coverage result:** no archive receipt exists yet for this change (it is an
unarchived candidate). The most recent measured evidence is the merged
Cobertura report at `coverage/cobertura-coverage.xml`: **line rate 0.984340
(880/894)**, branch rate n/a (the report carries no branch data). The archive
will publish the final merged receipt. The 14 uncovered lines were classified
from static inspection:

| Location | Lines | Classification | Action |
|---|---|---|---|
| `apps/cli/src/fs.rs` | 47 | 3 — hard to test: concurrent claimant between the name check and `create_new` | none; exercise requires an external race in another process |
| `apps/cli/src/fs.rs` | 54 | 3 — hard to test: the `Ok(None)` exhaustion after every enumerated name is taken | none; requires all 1000 candidates occupied |
| `apps/cli/src/main.rs` | 27 | 3 — hard to test: the environment-override branch; only a malformed `DUTY2CAL_CONSTRUCTION_CLOCK` reaches the visible fallback | none; no consumer behaviour depends on it |
| `crates/duty2cal-core/src/adapters/ics/mod.rs` | 88–89 | 3 — hard to test: the `\\` and `;` escape arms; the supported family prints neither character in the escaped text | none; no generated fixture can carry them |
| `crates/duty2cal-core/src/domain/mod.rs` | 184–186 | 3 — instrumentation attribution of the new variant's field declarations; the constructors are exercised by the timed/all-day/overnight/description scenarios | none; no executable region is unexercised |
| `crates/duty2cal-pdf/src/lib.rs` | 288, 365, 383, 389, 407, 408 | 3 — hard to test: parser guards and `?` fallbacks for source shapes outside the supported Released Roster family, which the family check rejects before these branches matter | none; unreachable through the supported consumer interface |

No bucket-1 dead code and no bucket-2 missing coverage was found: every branch
this change introduces (both fallbacks, the one-endpoint error, both route
errors, the silent-row skip and numbering, the two report shapes, the two page
list states, the count sentence, the stale-list clearing and the offer failure)
is exercised by a delta scenario through the real interface. No redundant unit
test re-asserts a feature-observed invariant (the change adds none).

## Scenario execution evidence

Recorded run at `b08abd6`; the change verification receipt was generated at
`c3db77a` (the archive is the single full-suite authority; this review did not
re-run the suites):

- `./run-tests.sh` → 3 features, **114 scenarios passed**, 461 steps, exit 0.
- `./run-tests.sh --e2e` → 1 feature, **1 scenario passed**, 5 steps, exit 0.
- `./run-tests.sh --browser` → 3 features, **25 scenarios passed**, 179 steps, exit 0.
- `cargo fmt --all --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean;
  `givn lint --change report-row-diagnostics` exit 0 (advisories only).
- `givn check tasks --change report-row-diagnostics` wrote the change
  verification receipt (`{"scope":"regular","total":114,"passed":114,"failed":0,"skipped":0}`,
  `status: passed`, head `c3db77a`).
- Manual real-plan check: the November plan produced 6 warnings, 21 events,
  exit 0, with the calendar and the warning report.

**Receipt status:** unproven candidate — no archive receipt yet; the change
verification receipt passes for the regular scope, and the recorded e2e and
browser runs are above.

## E2E coverage and the opposite classification

The delta adds no consumer action, so it adds no `@e2e` scenario (D10): the two
inventory actions already have exactly one `@e2e` scenario each, and the new
page states are regular coverage of the page action. The CLI action's e2e
asserts the real binary's transcript and published calendar; the browser
action's e2e drives Playwright and asserts the saved download and rendered page.

Opposite classification test: **a rendered page is a Web UI**, so the page
action would normally require a browser driver and could not be satisfied by an
HTTP client. That classification does not change the delta's coverage decision,
because the page action already *has* its permanent `@e2e` browser scenario
(`Crew member converts a supported duty plan in the browser and saves the
offered calendar`), which drives the real browser; the delta's eight `@browser`
scenarios add list/offer invariants to that same action and are not a second
interaction. No HTTP/fetch substitute exists or is used. The non-browser e2e
scenario that remains is the CLI action, whose real interface is the process.

## Overlap dispositions

`givn lint --change report-row-diagnostics` reports one shape-match finding for
this delta: the two route-error scenarios share a shape. It is not a duplicate —
one exercises a known timed code (`TK`) and the other an unrecognized code on
the same printed-route shape, so merging them would hide which classifier arm
fired. The gate reports `✓ overlap dispositions passed` for this pair; the
lint's additional `[SUBST]` notices are its shared-step heuristic over unrelated
scenarios and are not disposition pairs.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A timed activity code with a route and an incomplete printed interval rejects the conversion | An unrecognized activity code with a route and a single printed endpoint rejects the conversion | variant |

### Removed and added pairs

| Removed scenario | Added counterpart | Relationship |
|---|---|---|
| An unknown required activity code rejects the conversion | An unrecognized activity code with a printed window becomes a timed activity with a warning; … without printed times becomes an all-day activity …; … with a single printed endpoint rejects the conversion | supersedes — the removed rule (any unknown code fails) is replaced by the representation-plus-warning rule, and the one-endpoint case keeps the failure branch |
| An unknown activity code with no route rejects the conversion | An unrecognized activity code without printed times becomes an all-day activity with a warning | supersedes — the no-route unknown code no longer fails without an interval; the all-day fallback replaces it |

No removed+added pair is unrelated.

## Split-or-keep

`givn lint --change report-row-diagnostics` reports no `[LONG]` scenario (the
gate threshold is more than 14 steps). The longest delta scenario is 9 steps
("The page shows the warnings and offers both files when a plan converts with
warnings", "The page lists the errors and the warnings when a plan cannot be
converted", "A second attempt clears the previous attempt's warning list"); each
asserts one interaction's invariant and is not split. There is no long-scenario
row to record.

## Design.md conformance (remaining checks)

| Decision | `design.md` says | Built | Matches? | If not |
|---|---|---|---|---|
| Internal primitives section | `## Internal Primitives` listing the new structures | Section present in `c3db77a`: `Severity`, `Diagnostic`, `ConversionOutcome`, `ConvertError::{Table,Rows}`, `DutyEntry::UnrecognizedActivity`, `diagnostic_line`, `failure_report_text`, `warning_report_text`, `Kind::Warning`; signatures match the code | yes | n/a — F2 resolved |
| Step-definition table shape | The review gate resolves the promised file from the table's second column | Table now reads `Capability \| Step definition file \| Module \| Permanent merge target \| New or changed phrase groups`; every named path exists | yes | n/a — F1 resolved |
| `failure_report_text` signature | Two parameters (`source`, `diagnostics`) | Code declares `(source_name, diagnostics)`; the design's stale three-parameter text was corrected in the re-review | yes | n/a |
| Escaped table pipes | The gate parses table cells literally | The step-definition phrase cell escapes its `\|` characters | yes | n/a |

## Sign-off checklist

- [x] Fabrication audit: clean — 0 empty step bodies across 5 files, every task
      commit is real (production or its scenario's test bindings; lineage in
      the table), all promised components exist, no downgraded e2e scenario, no
      browser e2e driven by HTTP/fetch, no over-produced e2e scope.
- [x] Every checked task has a verified commit; the production branches are
      traced to `f90cea2` and `1f95728` where the later commit is the
      fixture/assertion pin.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope (regular 114, e2e 1,
      browser 25); review did not re-run the suite.
- [x] Coverage evidence: the merged report is the most recent measured
      evidence (98.43% lines); the archive receipt will carry the merged
      result. All 14 uncovered lines classified under the three buckets with no
      invented fourth bucket.
- [x] Dead code deleted; missing tests added; hard-to-test gaps justified under
      bucket 3.
- [x] No redundant unit test re-asserts a feature-observed invariant.
- [x] No `@wip` tags remain; no implementation-layer detail in the delta spec.
- [x] Canonical E2E policy read from the served specs instruction; the two
      normalized inventory actions each have one `@e2e` scenario; variants are
      regular scenarios.
- [x] Every e2e scenario has a real-interface primary assertion and uses the
      reviewed driver; browser scenarios are driven by Playwright, never
      HTTP/fetch.
- [x] Local run command starts the product cleanly: the product is a CLI,
      `cargo run --release -p duty2cal -- <duty-plan.pdf>`; the recorded manual
      run on the real November plan produced 21 events and 6 warnings, exit 0.
      No digital twins exist (no database, queue or service).
- [x] The exact files `verify.e2e_command` invokes were identified; the tree
      and `git status` were searched for a parallel e2e implementation — none.
- [x] `verify.e2e_command` is not identical to `verify.command`; counts prove
      isolation (1 vs 114).
- [x] Implementation matches design.md as reviewed. **F1 resolved in
      `c3db77a`:** the `## Step Definitions` table now carries the real path in
      column 2 (`Capability | Step definition file | Module | Permanent merge
      target | New or changed phrase groups`), the escaped pipes parse, and
      `givn check review --change report-row-diagnostics` exits **0** with
      `✓ integrity passed`; design-review re-ran to `DESIGN-REVIEW: PASS`.
- [x] **F2 resolved in `c3db77a`:** `design.md` now has `## Internal
      Primitives` naming `Severity`, `Diagnostic`, `ConversionOutcome`,
      `ConvertError::{Table, Rows}`, `DutyEntry::UnrecognizedActivity`,
      `diagnostic_line`, `failure_report_text`, `warning_report_text` and
      `Kind::Warning`, with the corrected two-parameter `failure_report_text`
      signature; the design re-review passed.
- [x] Interaction coverage verified: both inventory entries map to a matrix row,
      an existing `@e2e` scenario, and step definitions using the promised
      mechanism.
- [x] arc42 slug mapping and chapter conformance: CLEAN.
- [x] No finding is excused with invented "acceptable/future scope" language;
      the two findings are fixed by the named `design.md` corrections.

README-IMPACT: updated - Description, Usage

Findings F1 and F2 were documentation defects in `design.md`; both are fixed in
`c3db77a` and the design re-review ends `DESIGN-REVIEW: PASS`. The gate now
reports `✓ integrity passed`, `✓ run declaration passed`, `✓ overlap
dispositions passed` and `✓ change verification receipt passed` (114 scenarios
at `c3db77a`) and exits 0. No production, test, spec or design defect remains.

REVIEW: PASS
