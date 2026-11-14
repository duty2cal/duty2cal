# Design: recognize-cockpit-duty-codes

Owning capability: [export-duty-plan-calendar](../../specs/export-duty-plan-calendar/usecase.md).

## Context

The classifier in `crates/duty2cal-core/src/application/mod.rs` recognizes three exact all-day codes, six exact availability codes and three exact non-flight codes. Every other code falls back to `UnrecognizedActivity`, which preserves the printed evidence but titles the event `unknown duty: <code>` and emits a warning. The operator confirmed six cockpit code families, so those rows can enter the existing kinds without a new event shape.

## Architecture Impact

No port, adapter, data model or facade changes.

| Component | Change |
|---|---|
| `crates/duty2cal-core/src/application/mod.rs` | `ALL_DAY_CODES` gains `U1` and `OFF_2`; `NON_FLIGHT_CODES` gains `MED` and `EMCRM`; a prefix predicate recognizes `WBT_` and `EBTF3`; `classify` uses the predicate. |
| `crates/duty2cal-pdf` | None. The table reader and raw cells are unchanged. |
| ICS writer, UIDs, `DTSTAMP`, descriptions, reports | None. |
| Public support statement | None; no carrier changes. |

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| `RosterSource` (unchanged) | export a duty-plan calendar | `ConvertRoster` | `PdfRosterSource` | A second publication representation would add an adapter, not split this one. |

The classification is an internal rule of the application layer; it adds no contract.

## Classification design

- **All-day:** `ALL_DAY_CODES` becomes `FREE`, `OFF`, `U`, `FZ`, `U1`, `OFF_2`. Title is the printed code; printed clocks are ignored because the kind is decided before interval resolution.
- **Non-flight exact:** `NON_FLIGHT_CODES` becomes `TK`, `DTUSA`, `EM`, `MED`, `EMCRM`. Each uses its own printed UTC start/end and the printed station when supplied; title is the printed code.
- **Non-flight families:** `NON_FLIGHT_PREFIXES` is `WBT_` and `EBTF3`. A printed code that starts either prefix is a non-flight duty under the same rules; the suffix is preserved in the title and is not interpreted.
- **Boundary:** a code that does not equal an exact member and does not start a confirmed prefix keeps the existing `UnrecognizedActivity` fallback and warning. This keeps `AMV`, `REP`, `SBAO`, `S5`, `WBTX` and `EBTF4A` outside the confirmed set.
- **Order:** all-day and availability checks stay before the non-flight check, so the existing precedence is unchanged. The prefix predicate runs only in the non-flight branch.

## Test design

### Test runner and strict mode

- Full suite: `./run-tests.sh`.
- E2E-only: `./run-tests.sh --e2e`.
- Single scenario: `./run-tests.sh --name "<scenario title>"`.
- Strict mode: `apps/cli/tests/features_runner.rs` calls `.fail_on_skipped()`; undefined or skipped steps fail. No runner change.

### Step definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| export-duty-plan-calendar | `apps/cli/tests/steps/export_duty_plan_calendar.rs` | same file |

One new Then is added:
`Then the calendar contains one timed activity titled "<title>" from HH:MM to HH:MM UTC on YYYY-MM-DD`.
It asserts exactly one `VEVENT`, the `SUMMARY`, and the matching `DTSTART`/`DTEND`. The all-day outline and the fallback boundary reuse existing steps.

### Test technique

- The modified all-day outline adds `U1` and `OFF_2` with printed clocks, proving the kind wins over clocks.
- The new non-flight outline converts each exact code and each prefix family through the real executable, asserts the printed code as the title, the printed UTC window and **no conversion warning**. Under the old classifier every row would title `unknown duty: <code>` and warn, so the scenario fails before the change.
- The boundary outline uses `WBTX` and `EBTF4A`, which do not match the confirmed prefixes, and asserts the `unknown duty:` title plus one warning. This protects the prefix rule from becoming a loose substring rule.
- The generated fixture prints the code in the `Activity` cell and the window/station; no real plan is committed.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| `export-duty-plan-calendar`: supply a duty plan for conversion through the executable | Crew member converts a supported duty plan and receives a calendar file | CLI (terminal) | `std::process::Command` runs `env!("CARGO_BIN_EXE_duty2cal")` and reads the published `roster.ics` |
| `export-duty-plan-calendar`: supply a duty plan through the converter page and save the offered calendar | Crew member converts a supported duty plan in the browser and saves the offered calendar | Web UI | Playwright driver (`web/e2e/driver.mjs`) |
| `state-the-support-statement`: run the support-statement check | Release owner runs the support-statement check and every carrier carries the statement | CLI (shell check) | `scripts/support-statement-check.sh` |

No interaction is added. The new code rows are variants of the executable conversion action; no new `@e2e` scenario is created.

## Visual Design Contract

```
- Interface: web
- Viewports: mobile (360x780), desktop (1920x1080)
- Design system: docs/design/design-system.md
```

- **Screens covered:** none. The change removes some entries from the existing `Unknown duties` list when a cockpit plan has no remaining unknown codes; the list's structure, styles, states and controls are unchanged.
- **Design tokens, primitives, breakpoints, accessibility and motion:** unchanged.
- **Rendered reference:** no change.
- **Capture mechanism and evidence path:** no new or changed `@e2e` browser scenario; the existing browser evidence remains in force.
- Persona impact: the crew member reads a calendar without misleading `unknown duty:` titles for confirmed codes; the release owner sees the evidence record name the confirmed codes; the web reviewer sees no new page state.

## External boundary cases

| Boundary | Adversarial case | Expected behaviour |
|---|---|---|
| exact code | `U1` or `OFF_2` prints clocks | all-day; clocks ignored |
| exact code | `MED` or `EMCRM` prints both UTC endpoints | timed non-flight at the printed station |
| prefix | `WBT_EM`, `WBT_GR`, `WBT_XY` | recognized non-flight; printed code preserved |
| prefix | `EBTF3A`, `EBTF3B`, `EBTF3C` | recognized non-flight; printed code preserved |
| prefix near miss | `WBTX`, `EBTF4A` | `unknown duty: <code>` plus warning |
| printed window | a recognized code with exactly one endpoint | existing conversion error; no all-day invention |
| unconfirmed code | `AMV`, `REP`, `SBAO`, `S5` | existing fallback and warning |

## Local runnability

- Local run command: `./run-tests.sh`; no server, database, container or third-party service.
- Digital twins: none.

## Coverage process boundaries

Coverage is enabled. This change adds no production process.

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| `features_runner` | `cargo llvm-cov --test features_runner` from `./measure-coverage.sh` | the instrumented test binary | cargo-llvm-cov's per-process profile | `./measure-coverage.sh` writes `coverage/non-e2e-cobertura.xml`; `./merge-coverages.sh` merges | `crates/duty2cal-core/src/application/mod.rs` (the non-flight family predicate) |
| `duty2cal` CLI child | the conversion steps through `env!("CARGO_BIN_EXE_duty2cal")` | the cargo-built, instrumented CLI binary | `LLVM_PROFILE_FILE` inherited from the runner; flushed on exit | the same `measure-coverage.sh` run folds it into the matching scope | `crates/duty2cal-core/src/application/mod.rs` (all-day membership) |

Measured population and exclusions are unchanged from the archive's coverage declaration: production source under `crates/` and `apps/cli/src`; tests, fixtures, generated code and the browser bundle excluded.

## Use-case traceability

| Use-case guarantee, rule or event | Technical decision | Evidence |
|---|---|---|
| Confirmed code meanings are represented by their printed evidence | exact all-day and non-flight membership | `All-day codes ignore printed clocks`; `Medical and training duties use their printed UTC window` |
| An unconfirmed code warns rather than silently acquiring a meaning | prefix/exact boundary | `A code outside the confirmed training families still warns`; existing unknown-code scenarios |
| A recognized timed code keeps its own printed window and station | non-flight branch unchanged | the new outline asserts `DTSTART`/`DTEND` and the emitted title |
| An unrecognized code never becomes all-day merely because a prefix resembles one | exact/prefix predicate | `WBTX`, `EBTF4A` examples |

## Decisions

- **Decides D1:** use the existing timed non-flight representation; title the printed code; no translated title. Realized by adding members to `NON_FLIGHT_CODES` and the prefix predicate.
- **Decides D2:** `WBT_*` and `EBTF3*` match their confirmed prefixes; the suffix is preserved and not interpreted. Realized by `NON_FLIGHT_PREFIXES`.

## Black-Box-First policy

Every scenario drives the built executable and reads the published calendar and transcript. No internal unit test is added; the classifier is observed through its calendar title, interval and warning.

## Risks

- A future code that starts `WBT_` or `EBTF3` but means something else would be recognized. The operator supplied the prefixes; a new meaning needs its own change. The near-miss scenarios bound the rule to the confirmed spellings.
- `U1` and `OFF_2` are exact; a future `U2` or `OFF_3` remains a warning until confirmed.
