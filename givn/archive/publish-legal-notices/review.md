# Review: publish-legal-notices

- Inputs: `proposal.md`, `specs/**`, `design.md`, `design-review.md`, `arc42.md`, `tasks.md`, `docs/arc42/**`, `docs/design/**`, and the committed implementation (`2d752bc`, `747462b` plus the fixes recorded below).
- Method: completeness-based coverage review; every check below was run against the current tree, and the visual block used four fresh-context reviews, one per confirmed Persona, reading the committed screenshots.

## Receipts

| Scope | Command | Result |
|---|---|---|
| Regular | `./run-tests.sh` | 2 features, 94 scenarios, 348 steps, all passed |
| E2E | `./run-tests.sh --e2e` | 1 scenario, 5 steps, passed |
| Browser | `./run-tests.sh --browser` | 2 features, 18 scenarios, 127 steps, passed |
| Scripts | `bats scripts/tests` | 36 tests, exit 0 |
| Public text | `scripts/public-text-check.sh` | clean, 18 promises; the notice check clean |
| Output gate | `scripts/check-site-content.sh dist/site` | eleven members, notices complete, links resolve |
| Licence | `reuse lint` | REUSE 3.3 compliant, 610 of 610 files |
| Workflow | `actionlint .github/workflows/ci.yml` | clean |
| Delta shape | `givn lint --change publish-legal-notices` | 4 findings, all advisory (three long scenarios, one persona path) |

## Fabrication audit

| # | Check | Evidence | Result |
|---|---|---|---|
| 0 | `@e2e` tag integrity | `grep -c "@e2e" specs/state-the-public-legal-notices/state-the-legal-notices.feature` → 2; both notice scenarios still carry it | CLEAN, 2 tags, none removed |
| 1 | Empty or no-op step bodies | `grep -rn "unimplemented!\|todo!()" apps/cli/tests/steps/` → 0 | CLEAN, 0 empty step bodies found across 6 step files |
| 2 | Every `[x]` task has a commit touching production source | `2d752bc` touches `web/`, `scripts/`, `apps/cli/tests/steps/`, `web/e2e/driver.mjs`, `REUSE.toml`, `cliff.toml`, `.github/workflows/ci.yml`; `747462b` touches the documents | CLEAN, with the shared-revision deviation recorded in `tasks.md` |
| 3 | Every component `design.md` names exists | `ls` over the promised paths: `web/impressum.html`, `web/privacy.html`, `scripts/legal-notices-check.sh`, `scripts/legal-notices.env`, `scripts/tests/legal-notices.bats`, `apps/cli/tests/steps/legal_notices.rs`, `apps/cli/tests/steps/e2e/visual_baseline.rs`, `docs/design/reference/notice-reference.html` | CLEAN |
| 4 | Strict-mode proof | `tasks.md` setup: with the step module unregistered, `./run-tests.sh --tags "@state-the-legal-notices"` → `Step doesn't match any function`, `10 scenarios (10 failed)` | CLEAN |
| 5 | `@e2e` Then steps assert the real interface | both notice scenarios assert `read_text` output (rendered text, links, language, cookie/storage) and `read_state`'s request log; the screenshots are the final state | CLEAN |
| 6 | Browser steps drive the browser | the steps call the Playwright driver's `click` (activate the footer link by visible text), `navigate`, `read_text` and `screenshot`; no HTTP client and no in-page `fetch` | CLEAN |
| 7 | The file `verify.e2e_command` invokes | `./run-tests.sh --e2e` → `cargo test --test features_runner -- --tags "@e2e and not @browser"`; the notice browser steps are the only implementation for those scenarios | CLEAN |
| 8 | E2E scope | two inventory rows, two `@e2e` scenarios, both distinct navigations from the converter footer | CLEAN |
| 9 | `verify.e2e_command` is a strict subset | `./run-tests.sh` vs `./run-tests.sh --e2e`; regular 94 scenarios, e2e 1 scenario | CLEAN, strictly smaller |
| 10 | Implementation matches `design.md` | two deliberate deviations, both recorded here: `BrowserDriver` is a generic `send` client, so "methods on `BrowserDriver`" became "actions in its allow-list"; the evidence directories are named after the notice rather than the scenario slug, and `design.md` was amended | CLEAN, both recorded |
| 11 | Interaction coverage | table below | CLEAN |

### Interaction coverage cross-reference

| Inventory entry (`usecase.md` §Interactions) | Design matrix row | `@e2e` scenario title | Driving mechanism in the step file |
|---|---|---|---|
| follow the footer's "Impressum" link on the site | yes | Reader opens the Impressum from the site footer and reads who publishes the site | Playwright: `read_text` on the site root, `click` the footer link by its visible text `Impressum`, `read_text` the rendered notice; `screenshot` at both viewports |
| follow the footer's "Datenschutz" link on the site | yes | Reader opens the Datenschutzerklärung from the footer and reads what a page request processes | Playwright: same actions with the visible text `Datenschutz` |

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Requirements overview names the two notices and their reachability | `01-introduction-and-goals.md` | Yes | §The two pages | Carriers row | `web/impressum.html`, `web/privacy.html`, `web/index.html` footer | yes |
| Quality goal: every public carrier states the operator and the processing | `01`, `10` quality goal / `QS-044` | Yes | §Visual/§Decisions | Carriers row | the two notices; the check's required list | yes |
| Constraint: two notices, footer links, no script, cross-carrier check | `02-architecture-constraints.md` | Yes | §Technology decisions, §The checks | Setup + scenarios | `NOTICE_CSP`, both checks | yes |
| Context: the reader, the host's logging, the notices as a project record | `03-context-and-scope.md` | Yes | §The two pages | Carriers row | `web/privacy.html` §2–3 | yes |
| Published-site row enumerates the assembled members | `04-solution-strategy.md` | Yes | §Scope | Carriers row | `scripts/build-site.sh` eleven members | yes |
| Building blocks: sources, checks, steps, driver | `05-building-block-view.md` | Yes | §Scope diagram, §Step Definitions | Setup + carriers | all named files exist | yes |
| Runtime: the notices are checked and published | `06-runtime-view.md` | Yes | §The checks | scenarios 1–10 | `scripts/legal-notices-check.sh`, `check-site-content.sh` | yes |
| Deployment: eleven members, two notice screens, assembler copies both | `07-deployment-view.md` | Yes | §Scope | Carrier row | `dist/site` listing, bats fixture | yes |
| Crosscutting: static/no-script/same-origin, the second checked claim | `08-crosscutting-concepts.md` | Yes | §Technology decisions | Carrier row | CSP assertion in both checks; browser request log | yes |
| ADR register: no ADR, three `CANONICAL_ARTIFACT` routings | `09-architecture-decisions.md` | Yes | §Decisions | — | register and ADR files unchanged | yes |
| Quality requirement: one-step reachability and the named refusal | `10-quality-requirements.md` | Yes | §The checks | scenarios 1–2, 11–12 | check refusals; browser scenarios | yes |
| Risks: address drift, c/o consent outside the repository, two checks | `11-risks-and-technical-debt.md` | Yes | §Decisions D1 | Precondition task | `R-073`, `TD-028` | yes |
| Glossary: Impressum, Datenschutzerklärung, Legal notice, Legal-notice carrier | `12-glossary.md` | Yes | §The two pages | — | terms used in specs, design and code | yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The change uses **Impressum**, **Datenschutzerklärung**, **Legal notice** and
**Legal-notice carrier** as `docs/arc42/12-glossary.md` defines them; the specs,
the design, the checks and the commit messages use those terms and no synonym
for a notice. **Carrier** keeps its support-statement meaning and is not reused
for the notices. CLEAN.

## Coverage

No archive receipt exists yet for this change, so the gaps are classified by
inspection; the archive gate measures the committed candidate.

- The change adds no line of instrumented production Rust: the pages are HTML
  and CSS, the checks are shell, and the only Rust is test-step code
  (`apps/cli/tests/steps/legal_notices.rs`, the shared baseline helper and the
  `DutyWorld` fields).
- Every new behaviour has executable evidence at the level it lives: the output
  gate and the source check through 19 bats tests and 10 Gherkin scenarios, the
  rendered pages through 2 browser scenarios with committed screenshots.
- Bucket 1 (dead code): none added; `read_state`'s `site` observation and the
  notice steps share the existing driver rather than a second harness.
- Bucket 2 (missing coverage): none identified — the one previously uncovered
  family, the foreign-asset rule for a notice, has its own scenario and bats
  test.
- Bucket 3 (hard to test): none claimed.

## E2e coverage

The capability renders HTML, so it is a Web UI; the opposite classification
("static text, therefore an HTTP response is the interface") does not hold
because the evidence a reader depends on — the footer link being activatable and
the rendered document being readable at both viewports — is produced only by a
browser. The driver is Playwright, the command is `./run-tests.sh --browser`,
and each of the two inventory rows has exactly one `@e2e` scenario.

## Deferrals

None. `reuse lint` was run rather than deferred (installed locally because the
Nix derivation failed on this machine).

## Use-case and Persona conformance

The implementation preserves the use case's goal, its two inventory rows, its
rules (two pages on the operator's origin; the Impressum names name, postal and
mail address; nothing withdrawn; the Datenschutzerklärung describes the actual
processing; one reachability rule; German binding with an English summary; no
consent banner) and its guarantees. No Persona was invented or promoted.

| Persona | Disposition | Evidence |
|---|---|---|
| visitor--project-reader | PASS | fresh-context review of the four screenshots: operator, address and processing legible at 360 and 1920 |
| project-maintainer--release-owner | PASS | the four screenshots: the claims the project answers for, with the capture showing the loaded page |
| contributor--web-reviewer | PASS | shell continuity, accent links, measure, one `h1`, no third-party content |
| project-maintainer--organization-owner | PASS | the contact block and the employer's role; the profile's link leads to an answer |

### Use-case scope rejection criteria

Walked: one goal, one primary Actor (Operator) named by the trigger, the
capability passes the placement test (a reader's goal, not a channel), no
observing goal with a mutation, every extension names its step, preconditions
are not established by the flow, no mechanism vocabulary in the goal or trigger,
no component-shaped name, the main flow names one actor goal, and no `Extends`
relationship.

PERSONAS: visitor--project-reader
PERSONAS: project-maintainer--release-owner
PERSONAS: contributor--web-reviewer
PERSONAS: project-maintainer--organization-owner

## Visual review

- Evidence: one final-state screenshot per viewport per `@e2e` scenario, under
  `evidence/visual/impressum/{mobile,desktop}.png` and
  `evidence/visual/datenschutzerklaerung/{mobile,desktop}.png`.
- Four fresh-context reviews, one per confirmed Persona, each reading the four
  screenshots: all four returned `VISUAL-REVIEW: PASS`.
- The reviews named the states they saw: the Impressum in its default state and
  the Datenschutzerklärung in its default state, both from the document top at
  360x780 and 1920x1080.
- Findings from the first pass and their dispositions: the mobile heading clipped
  at 360 (`h1` now hyphenates), the desktop captures scrolled (the driver scrolls
  to the document top before capture), the mail address leaving its panel (it
  breaks at `@`), the service-address sentence contradicting itself (rewritten
  and asserted through `IMP_EMPLOYER_ROLE`), the reference and the primitive
  showing a single panel (amended to the built form: header carries the `h1`,
  each section is a panel), the evidence directory naming (the contract now
  names the notice). All closed and re-reviewed.

VISUAL-REVIEW: PASS

## README impact decision

README-IMPACT: updated - Legal

The README gains one Legal section: two labelled links to the notices, the
statement that the notices are the single source, and the rule that no postal or
mail address is copied. The three givn-managed blocks are untouched and
`scripts/public-text-check.sh` passes with 18 promises.

## Overlap dispositions

`givn lint --change publish-legal-notices` reports no shape match involving this
delta, so the table is empty.

| Finding pair | Decision |
|---|---|

## Split-or-keep

| Scenario | Decision | Reason |
|---|---|---|
| Release maintainer publishes the site and the hosting target holds the built page | keep | one publication action; splitting would duplicate the action or create fragments that cannot run, because the feature is documentation |
| The published profile content states only supported claims and requests no third-party resource | keep | a documentation scenario over one reviewed file; each step names one claim |
| Reader opens the Datenschutzerklärung from the footer and reads what a page request processes | keep | one reader action through the real interface; each step asserts one required element of the same document |

## Deterministic gate dispositions

`givn lint` reports no overlap finding; the three long scenarios are dispositioned
above; no `@e2e` tag was removed.

REVIEW: PASS
