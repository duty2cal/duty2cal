# Design review — add-nightly-ci

Final design review, held after the second review's three remaining items were
reported fixed. Read-only: no build, no workflow run, no network call. Each fix
was checked against the current file content, not accepted as a claim.

Reviewed: `proposal.md`, `design.md`, `arc42.md`,
`givn/changes/add-nightly-ci/.givn-skip`, `.github/workflows/nightly.yml`,
`.github/workflows/ci.yml`, `run-tests.sh`,
`givn/specs/export-duty-plan-calendar/export-duty-plan-calendar.feature`,
`givn/archive/reduce-test-runtime/results/{regular,e2e}.json`, and
`docs/arc42/01-12` (the affected chapters 02, 07, 08, 10 and 11 read in full;
`git diff HEAD -- docs` shows no other chapter changed; every chapter file
exists and none contains box-drawing characters or ASCII pseudo-diagrams).

## Status of the second review's three remaining items

| # | Item | Status |
|---|---|---|
| 1 | The design no longer claims a separate `--e2e` nightly run | **Resolved** |
| 2 | R-070 names the 60-day schedule limit and R-071 | **Resolved** |
| 3 | `design.md`'s changed-file table lists chapter 8 | **Resolved** |

### 1. No separate `--e2e` nightly run — resolved

`design.md:36` now reads
`product job: ./run-tests.sh (includes the CLI e2e)`, and the traceability row
for "Export through the executable" (`design.md:108`) now names `Product job`.
That agrees with `design.md:16` ("Product job | `./run-tests.sh` ... runs the
84 product scenarios (which include the CLI e2e scenario)") and with the
workflow: `nightly.yml` has exactly three jobs (`product`, `browser`, `deny`)
and one product-suite command, `nightly.yml:37` `./run-tests.sh`. No `--e2e`
argument occurs anywhere in `nightly.yml`, and there is no e2e job.

The selection arithmetic still holds. The feature has 91 `Scenario:` headers,
two of them outlines with 4 + 7 example rows, so 100 scenario instances; 16
carry `@browser` (lines 443, 458, 466, 473, 483, 493, 503, 514, 523, 532, 539,
546, 554, 561, 569, 576), leaving 84 for the default run. `run-tests.sh:106`
selects `not @browser`, which includes the CLI `@e2e` scenario at `:6`;
`run-tests.sh:99-100` selects `@browser` for the browser job, which includes
the `@e2e @browser` smoke at `:443`. The archived
`results/regular.json` records `{"total":84}` and `results/e2e.json` records
`{"total":1}`; the archived `tasks.md:26` records 84 scenarios / 303 steps,
matching `design.md:69` and the proposal's counts.

`design.md:71` and `:87` still list `./run-tests.sh --e2e` as a runner/local
command. That is the runner's documented mode (`run-tests.sh:12-17`) and
`design.md:85-88` attributes `--e2e` to local runnability, while the nightly's
command list is the workflow's. It is a wording overlap, not a claim of a
nightly step; see the non-blocking observations.

### 2. R-070 names the 60-day limit and R-071 — resolved

`docs/arc42/11-risks-and-technical-debt.md:79` (R-070) now says the nightly
"runs all 16 browser variants while the schedule is enabled", then: "GitHub
disables the schedule after 60 days without repository activity; a manual
dispatch or any push re-enables it (R-071)." R-071 (`:80`) states the limit
and the re-enable path, and its sentence "The workflow header, chapter 2,
QS-042 and R-070 name the limit and the re-enable path" is now true.

The limit is also present at `nightly.yml:9-11` (workflow header),
`docs/arc42/02-architecture-constraints.md:50`,
`docs/arc42/08-crosscutting-concepts.md:94-95` and
`docs/arc42/10-quality-requirements.md:103` (QS-042). Design decision D4
(`design.md:100-102`) names R-071, the workflow header and QS-042, all of
which carry it.

### 3. Changed-file table lists chapter 8 — resolved

`design.md:47-55` now lists `.github/workflows/nightly.yml`,
`.github/workflows/ci.yml`, and chapters 02, 07, **08**, 10 and 11. The
chapter-8 row's purpose ("The test-scope concept names the per-push smoke and
the nightly all-variant run") matches `08:89-95`. `git status` shows the new
workflow plus modifications to exactly `ci.yml` and those five chapters
(`git diff --stat HEAD -- docs` lists only 02, 07, 08, 10, 11), so the table is
complete and nothing changed is omitted.

## Workflow correctness (re-walked)

- **Trigger and boundary.** `nightly.yml:12-15` has `schedule` at
  `17 3 * * *` plus `workflow_dispatch`; `:17-18` is workflow-level
  `contents: read`, repeated on every job (`:28-29`, `:72-73`, `:129-130`).
  A grep for `secrets|tofu|passphrase|administer|deploy|publish|site.yml|
  release.yml|contents: write|pull_request|environment` finds only the header
  comment at `:7` that forbids the passphrase, administer token and deploy key.
  No configuration apply, no site or release publication, and no write
  permission is present; ADR-0006's boundary is intact.
- **Concurrency.** `:21-22` uses `group: nightly` with
  `cancel-in-progress: false`, so a slow browser run is not killed by the next
  night and overlapping manual runs serialize.
- **Product job.** `:25-67` checks out, installs the stable toolchain,
  restores the Rust cache and runs `./run-tests.sh` once (`:37`) with a
  15-minute timeout (`:27`). The default run executes the core/PDF unit and
  property tests plus the architecture test (the latter lives at
  `crates/duty2cal-core/tests/architecture.rs`, so `run-tests.sh:145` covers
  it), the golden test (`:150`), and the 84 cucumber scenarios including the
  CLI e2e (`:159`). The transcript goes to `DUTY2CAL_TRANSCRIPT_PATH` (`:36`),
  `phase=` lines go to the step summary (`:55-61`), failing scenarios are
  annotated (`:38-54`) and the log is uploaded with `if: always()`
  (`:62-67`). The 15-minute timeout is the proposal's and design's stated
  choice, above the 10-minute CI target QS-040 owns.
- **Browser job.** `:69-124` checks out with `fetch-depth: 0` (`:78`; the Site
  build needs `git describe`), adds `wasm32-unknown-unknown`, installs Node
  from `web/e2e/.nvmrc` with the lockfile cache, runs `npm ci` and
  Playwright/Chromium (`:88-90`), then `./run-tests.sh --browser` (`:94`) with
  a 30-minute timeout (`:71`). `run-tests.sh:99-100` selects all 16 `@browser`
  scenarios, including the smoke; `run-tests.sh:186-197` refuses a run that
  selects no scenario and requires exactly one for `--name`, so an empty
  selection is red, not green. The transcript is uploaded with
  `if: always()` (`:119-124`).
- **Deny job.** `:126-147` installs `cargo-deny` through
  `taiki-e/install-action@v2` (`:135-137`) and runs
  `cargo deny check 2>&1 | tee "$RUNNER_TEMP/nightly-deny.log"` under
  `set -euo pipefail` (`:139-141`); `pipefail` makes a cargo-deny failure fail
  the step after the log is written, and the log is uploaded with
  `if: always()` as `nightly-deny-log` (`:142-147`).
- **Actionlint.** `ci.yml:234` adds `.github/workflows/nightly.yml` to the
  actionlint file list; the other names are unchanged.

## Branch re-walk

### Scope

Proposal, design and workflow agree on a nightly/manual-dispatch, read-only run
of the 84 product scenarios (including CLI e2e), the 16 browser variants and
`cargo deny check`. `.givn-skip` contains `specs`, and no `specs/` file changed,
consistent with "no product behaviour changes". No configuration apply, site or
release publication, second platform matrix, issue creation or notification is
added. No scope defect remains.

### Technology choices

The schedule minute, runner class, Rust/Node caches, pinned `.nvmrc` and
lockfile, `wasm32` target, Playwright install, `taiki-e/install-action` and the
reuse of `run-tests.sh` are the mechanisms the per-push workflows already use.
`fetch-depth: 0` is present where the Site build needs it. The design's
"Version freshness" claim matches the files: no version is written from memory
(`dtolnay/rust-toolchain@stable`, `.nvmrc`, `package-lock.json`,
`taiki-e/install-action@v2`).

### Missing scenarios

No spec delta and no new observable product behaviour. The 16 browser scenarios
and the 84 product scenarios already exist and are run.

### Testability

This change adds no product behaviour, so RED/GREEN testability is not the
applicable gate. The runner contracts are concrete: an empty selection is a
failure (`run-tests.sh:186-197`), a `--name` run must select exactly one
scenario, failures accumulate across the Rust, golden and cucumber classes, and
the browser filter is a real tag selection, not a comment. A missing
Playwright/Chromium install fails loudly.

### E2E fidelity

Unchanged. The CLI `@e2e` scenario drives the real subprocess; the
`@e2e @browser` scenario drives Playwright against the built Site output. The
nightly runs the existing selectors through `run-tests.sh` and redefines
neither.

### Interaction coverage

The matrix in `design.md:76-81` has one row per interaction inventory entry:
executable conversion (CLI, real subprocess) and converter-page save (Web UI,
Playwright). "The nightly runs both" holds through the product job's default
filter and the browser job. No inventory entry is without a row.

### Risk

The main failure mode is the 60-day schedule disable. R-071 records it and the
re-enable path; the workflow header, chapter 2, chapter 8 and QS-042 repeat it,
and R-070 now cross-references it correctly. A quiet repository can still lose
the signal without a red run; that residual risk is stated, not hidden. The
nightly's red state is visible in the run and its artifacts.

### Use-case and Persona context

No use-case is owned; this is operational maintenance, and the proposal records
"Confirmed Personas: none — no product surface changes". No persona is invented
and no biography is used as a Gherkin actor.

### Decision ledger

D1 (whole runnable suite nightly and on dispatch), D2 (read-only and
secret-free), D3 (advisory refresh through `cargo deny`) and D4 (60-day
disable and re-enable path) each resolve in the design and are realized in the
workflow and the docs. No proposal question is left unowned.

### Visual design contract and interface classification

No screen is introduced or visibly changed; the nightly runs the existing page
against the existing browser scenarios. No new Visual Design Contract is due,
and the interface classification is unchanged: the converter page remains
Web UI driven by Playwright, and the executable remains CLI.

### ADR qualification

No new ADR is due. The read-only/no-secret boundary is a constraint recorded in
chapter 2 and enforced by ADR-0006's existing exclusion; the schedule and
timeout choices are operational design, not a new architecture decision.

### Architecture documentation (independent re-derivation)

Walking the 12 rows from proposal/design before opening `arc42.md` gives:
1 No, 2 Yes, 3 No, 4 No, 5 No, 6 No, 7 Yes, 8 Yes, 9 No, 10 Yes, 11 Yes,
12 No. `arc42.md` matches every row; no "Yes" is omitted and no row is marked
"No" that the change contradicts.

For every Yes row the content was checked: chapter 2's nightly constraint
(`:48` and the new `:50`), chapter 7's runner description (`:60`, which now
names the scheduled `nightly` workflow), chapter 8's test-scope section
(`:89-95`), QS-042 (`10:103`) and R-070/R-071 (`11:79-80`). Chapter 9 has no
new ADR and chapter 12 has no new domain term. The design's own file table now
agrees with the changed set.

### Ubiquitous language

No new domain term. "Nightly", "transcript", "browser smoke" and "the product
suite" are engineering vocabulary already used in chapter 8; no glossary term
is reused inconsistently.

### Deferrals

None.

## Non-blocking observations

- `docs/arc42/02:48` says "the remaining browser variants run per push only
  with `./run-tests.sh --browser`". The per-push CI job does not invoke
  `--browser`; the intended meaning is on demand. Chapter 8 (`:92-94`) and
  QS-042 state the split correctly, so this is wording only.
- `docs/arc42/07:60` parenthetical "(product suite, CLI e2e, all browser
  variants, `cargo deny`)" lists the CLI e2e beside the product suite that
  already contains it. It is a redundant enumeration, not a separate step; the
  workflow and `design.md:36` are unambiguous.
- `proposal.md:28` says the product and browser jobs write their transcript
  "(and phase lines) to the job summary and an artifact". The step summary
  receives only the `phase=` lines (`nightly.yml:61`, `:118`); the transcript
  is the artifact. The design's transcript row and QS-042 are accurate.

## Verdict

All three remaining items are resolved as content, not as claims. The workflow
runs one product job (`./run-tests.sh`, 84 scenarios including the CLI e2e,
plus the Rust and golden tests), one browser job selecting all 16 `@browser`
scenarios and failing on an empty selection, and one deny job whose log is
always uploaded. It is read-only and secret-free, adds no publication or
apply path, uses the 15/30/10 timeouts the proposal and design state, and is
covered by actionlint. The 60-day limit is recorded in the workflow header,
chapter 2, chapter 8, QS-042, R-070 and R-071, and the design's changed-file
table matches the actual changed set. No blocking defect remains.

DESIGN-REVIEW: PASS
