# Review: state-the-supported-duty-plan

## Scope

The change names the one duty plan the converter reads — a Lufthansa cabin-crew
duty plan published as a Jeppesen PDF export — in the five collectors that
carry the support statement: the website's `what-it-does` and `non-affiliation`
statements, the README, the organization profile, architecture chapter 3 and the
two issue forms. It adds the cross-carrier check, the evidence record, the
outcome-based issue forms and the site/profile carriers; it changes no conversion
behaviour.

## Fabrication audit

### @e2e tag integrity

The support-statement check is the new capability's one `@e2e` scenario; the
site publication and profile-update scenarios retain `@e2e` on their modified
documentation scenarios. The issue-form scenario was deliberately made a
regular `@givn.added` scenario at design time (design-review Q1): the form is
GitHub-rendered, not a project-hosted interface, and the check's YAML assertions
are its test. No tag was removed to bypass a gate; the capability retains its
`@e2e`.

### Empty step bodies

No step definition is added or changed. The executable evidence is
`scripts/tests/support-statement.bats` and `scripts/check-site-content.sh`; the
bats file has real assertions on exit status and named lines. Scan of the changed
tests: no empty or bare bodies.

### Task and commit check

| Task | Commit | Production files touched |
|---|---|---|
| The support statement check, carriers, forms, evidence record and site page | `327b6f84e8bf89405caa662563dfcb725edaf445` | `scripts/support-statement-check.sh`, `scripts/support-statement.env`, `scripts/public-text-check.sh`, `scripts/check-site-content.sh`, `scripts/site-evidence.sh`, `web/index.html`, `README.md`, `org/profile/README.md`, `.github/ISSUE_TEMPLATE/*`, `.github/pull_request_template.md`, `CONTRIBUTING.md`, `docs/evidence/supported-duty-plan.md` |
| The site publication evidence | `327b6f84e8bf89405caa662563dfcb725edaf445` (page) and `9756347d336e48ee023e8d96238a3ff72138ca1e` (frames) | the page copy and the committed frames |
| The profile carrier applied | `38fd9ecff775927b752c85ec5ec6cb7574cdaa2c` | `infra/terraform.tfstate`, the transcript |

Every checked task has a reachable commit touching production or executable
evidence. The planning revision is `9756347d`.

### Promised components

| Design promise | Present |
|---|---|
| `scripts/support-statement-check.sh` + `support-statement.env` | yes |
| Five carriers with the exact statement, boundary and non-affiliation | yes |
| `scripts/tests/support-statement.bats` | yes, 7 tests |
| `docs/evidence/supported-duty-plan.md` | yes, the fictional month and no calendar date |
| Site exact-text assertions and the site gate's first check | yes |
| Issue forms outcome questions, no export-family dropdown | yes |
| Profile source applied; live profile serves the statement | yes |

### Strict mode and execution evidence

`.fail_on_skipped()` remains. The full suite records 84 scenarios / 303 steps;
the e2e scope 1 scenario / 5 steps. The bats suite passes 7/7. The browser smoke
passes 12/12 steps and captured four frames.

### E2E assertions and driver

The support-statement check is driven as a real subprocess by bats
(`bash scripts/support-statement-check.sh --root <temp>`) and asserts its exit
status and named lines. The page is driven by Playwright; the profile apply is
the operator's `tofu` run, recorded by the state change and the live read-back.
The issue form is YAML reviewed and asserted by the check; it is not an `@e2e`
scenario.

### E2E scope

`verify.command` is `./run-tests.sh`; `verify.e2e_command` is
`./run-tests.sh --e2e`. The counts are 84 regular and 1 e2e; the e2e scope is
strictly smaller.

### Interaction coverage

| Inventory entry (`usecase.md`) | Design matrix row | @e2e scenario | Driving mechanism |
|---|---|---|---|
| state-the-support-statement: run the check before publication and read the named refusal | present | Release owner runs the support-statement check and every carrier carries the statement | bats runs the check as a real subprocess; the public-text and site gates invoke it |
| publish-public-website: publish the site | present | Release maintainer publishes the site and the hosting target holds the built page | the page text is asserted by `check-site-content.sh`; Playwright supplies the states and frames |
| apply-configuration-change: apply the previewed revision | present | A revised profile content updates only the profile file | the operator's `tofu` apply; the state and live read-back |

The removed issue-form inventory row (Q1) remains a regular documentation
scenario; it adds no interaction.

### Design conformance

The check, constants, carriers, forms, site and profile match the design. Two
documented adjustments: the issue-form `@e2e` was removed at design time (Q1),
and the profile transcript records the state change and live read-back; the
operator's preview/apply output was not captured, but the applied state and the
served page are the acceptance evidence.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Named statement and five carriers | `docs/arc42/02`, `03` | affected | the constants and carriers | task 1 | scripts + carriers | yes |
| Cross-carrier check and site gate | `docs/arc42/05`, `06`, `08` | affected | the check and first gate step | task 1 | scripts + site evidence | yes |
| Profile application | `docs/arc42/07` | affected | operator apply | task 3 | state + live read | yes |
| ADR routing | `docs/arc42/09` | affected | three non-qualifying candidates | — | `arc42.md` verdicts | yes |
| Quality and risk | `docs/arc42/10`, `11` | affected | QS-043, R-072, TD-027 | — | chapters | yes |
| Glossary terms | `docs/arc42/12` | affected | Carrier, Evidence record, Supported duty plan | — | glossary | yes |

`ARC42 CONFORMANCE: CLEAN`

## Ubiquitous language conformance

**Carrier**, **Evidence record** and **Supported duty plan** are recorded in
`docs/arc42/12-glossary.md`; the design and use case use those terms. No term
outside the glossary remains.

## Coverage

No instrumented run exists for this change yet; the archive receipt will carry
the merged result. Inspection: the change adds shell gates, documentation and
an encrypted state; no production Rust path. Dead code: none. Missing test
coverage: the check's eight refusal families and the clean path are bats tests;
the page is the browser smoke and the content check. The profile live apply is
the operator's evidence, not a test. The fidelity of the statement is the
check's, not a percentage.

## E2e coverage

The interface of the new capability is the CLI check; bats drives it. The page
is a Web UI, driven by Playwright for its states and screenshots; its statement
text is asserted by the assembled-page check. The issue form is not a
project-hosted interface and is YAML-reviewed. No HTTP/fetch substitute, no
over-produced e2e.

## Use-case and Persona conformance

PERSONAS: project-maintainer--release-owner
PERSONAS: crew-member--personal-calendar-owner
PERSONAS: contributor--web-reviewer
PERSONAS: contributor--publication-reader

The use case has one goal, the Release owner as the primary Actor, the evidence
precondition, the extensions for a differing carrier, withdrawn wording, an
unexercised plan, a future-support promise and a shrink, and one success
outcome covering the positive and shrunk statements. The issue-form consumer is
not an Actor; it was removed from the inventory at design time (Q1). No Persona
is invented or promoted.

Persona dispositions: the release owner judges the claim and the evidence; the
crew member gets a direct answer to "will my plan work?"; the web reviewer
judges the page statement and the same-origin closure; the publication reader
finds the evidence route and the outcome questions.

## Visual review

The four committed frames show the changed page at 360×780 and 1920×1080 in the
empty and ready states:

- `evidence/visual/crew-member-converts-a-supported-duty-plan-in-the-browser-and-saves-the-offered-calendar/{mobile,desktop,mobile--empty,desktop--empty}.png`
- `evidence/visual/release-maintainer-publishes-the-site-and-the-hosting-target-holds-the-built-page/{mobile,desktop}.png`

The page states the plan, the boundary and the named non-affiliation sentence;
the layout, header mark, statements panel, source/licence panel and footer keep
their order and tokens. No visible claim disagrees with the check. The issue
form is GitHub-rendered and not a project screen; it is YAML-reviewed, as the
design's Q1 records. The check and profile are CLI scenarios with transcripts.

| Persona | Goal reachable from the page evidence | Findings |
|---|---|---|
| `project-maintainer--release-owner` | yes — the page names the plan and the evidence record backs it | none |
| `crew-member--personal-calendar-owner` | yes — the statement says which plan is supported and its boundary | none |
| `contributor--web-reviewer` | yes — one image, same-origin, statements and layout intact | none |
| `contributor--publication-reader` | the issue form is YAML-reviewed; no page finding | none |

VISUAL-REVIEW: PASS

## Overlap dispositions

No shape match involving this delta is reported; the changed scenarios are the
support statement, the site publication sentence and the profile update, with
different invariants. No removed+added pair needs a supersession decision.

## Split-or-keep

| Long scenario | Decision | Reason |
|---|---|---|
| Release maintainer publishes the site and the hosting target holds the built page | keep | one publication action; splitting duplicates the action or creates non-runnable fragments |
| The published profile content states only supported claims and requests no third-party resource | keep | one published-content invariant; the profile apply is one action |

## README impact

README-IMPACT: updated - Description

The Description now names the supported plan, its boundary and the non-affiliation
sentence; the contributor route describes the outcome questions.

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a reachable commit touching production or executable evidence.
- [x] Every promised component exists.
- [x] Strict-mode proof present.
- [x] Scenario execution evidence recorded per scope (84 regular, 1 e2e).
- [x] Coverage classified; the archive receipt carries the merged result.
- [x] No dead code; missing tests added; the operator apply is the live evidence.
- [x] No `@wip` tags; no implementation-layer detail in the spec.
- [x] One `@e2e` per inventory entry; the driver is real.
- [x] The issue form is not a project-hosted interface; YAML-reviewed.
- [x] The e2e command differs from verify; counts prove the subset.
- [x] Interaction coverage verified end to end.
- [x] No finding excused outside the three buckets.

REVIEW: PASS
