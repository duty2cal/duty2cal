# Review: state-site-footer

- Inputs: `proposal.md`, the delta `specs/publish-public-website/publish-public-website.feature`, `arc42.md`, `tasks.md`, the permanent use case and the implementation.
- Method: one documentation scenario modified; no executable step changes; the visual block is one fresh-context review of the captured page.

## Receipts

| Check | Command | Result |
|---|---|---|
| Regular | `./run-tests.sh` | 2 features, 94 scenarios, 348 steps, passed |
| Browser | `./run-tests.sh --browser` | 2 features, 18 scenarios, 127 steps, passed |
| Scripts | `bats scripts/tests` | exit 0 |
| Support statement | `scripts/support-statement-check.sh` | clean, five carriers agree |
| Page content | `scripts/check-site-content.sh` | clean on the assembled output |
| Public text | `scripts/public-text-check.sh` | clean, 18 promises |
| Delta shape | `givn lint --change state-site-footer` | one advisory (long scenario) |

## Fabrication audit

| Check | Evidence | Result |
|---|---|---|
| `@e2e` integrity | the delta modifies the existing `@e2e` documentation scenario and keeps its tag | CLEAN |
| Empty steps / stubs | no step file is touched; the scenario is `@documentation` and runs nothing | NOT APPLICABLE |
| Task-to-commit | one revision carries the page, the two checks, the rule and the design system | CLEAN |
| Runner and strict mode | unchanged | NOT APPLICABLE |

## Coverage

No production code. Two checks lose an assertion that required the duplicated
sentence; the statement element remains required and compared, and the support
statement check still reads it. Nothing becomes unverified by the removal: the
page carries the sentence once, and that occurrence is the checked element.

## Visual review

- Evidence: one final-state, full-page screenshot per viewport under
  `evidence/visual/release-maintainer-publishes-the-site-and-the-hosting-target-holds-the-built-page/`
  (`360x1595` and `1920x1150`).
- One fresh-context review (`visitor--project-reader`) read both frames: the
  non-affiliation and as-is sentences appear once, the footer names and links
  the Built revision, states the local-processing note and links both notices,
  and the statements panel, converter and "Source and licence" panel hold their
  contract order. The first capture was a viewport crop and the review refused
  it because the notice links were outside the frame; the full-page recapture
  closed that, and the re-review returned `VISUAL-REVIEW: PASS`.

VISUAL-REVIEW: PASS

PERSONAS: visitor--project-reader
PERSONAS: project-maintainer--release-owner
PERSONAS: contributor--web-reviewer

## Split-or-keep

| Scenario | Decision | Reason |
|---|---|---|
| Release maintainer publishes the site and the hosting target holds the built page | keep | the publication's one end-to-end record; the change modifies one step, and splitting the scenario would multiply publications |

## README impact decision

README-IMPACT: none

## Use-case and Persona conformance

The change preserves `publish-public-website`'s goal, its statement placement
and its reachability rule, and removes the contradiction between "the footer
repeats the terms" and "no statement is stated twice". TD-014, which recorded
the duplication, is paid and removed.

REVIEW: PASS
