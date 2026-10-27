# arc42 Documentation Update: state-the-supported-duty-plan

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The requirements overview and the truthful-public-copy quality goal still said "no third-party provider name" and a generic supported plan; the operator's 2026-10-27 decision names the plan and its boundary. | The requirements overview names a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export; requirement 1 names the supported plan; quality goal 8 states the one support statement, its boundary, the non-affiliation sentence and the five-carrier check. |
| 2 | 02 architecture-constraints | Yes | The archived profile change's constraint that public copy names no provider is replaced, and a new cross-carrier constraint appears. | The provider-name constraint becomes the naming, boundary and non-affiliation rule; a new row records the five carriers and `scripts/support-statement-check.sh` in the `public-text` job and as the site publication's first evidence step. |
| 3 | 03 context-and-scope | Yes | The plan the converter reads, the visitor's expected output and the scope boundary all named the internal family or a generic supported plan. | Diagram edges, partner-table rows, the visitor row and the scope boundary name the plan; a new subsection carries the support, boundary and non-affiliation sentences verbatim. |
| 4 | 04 solution-strategy | No | The statement is public copy and a check inside existing publishing paths; no fundamental technical strategy changes. | — |
| 5 | 05 building-block-view | Yes | A new repository check and its bats suite appear; the web block, `index.html`, the site content check and the community-files block change. | `scripts/support-statement-check.sh` and `scripts/tests/support-statement.bats` are added; `check-site-content.sh`, the `web/` row, the `index.html` row and the community-files row name the statement. |
| 6 | 06 runtime-view | Yes | The statement's check, refusal and shrink is a new runtime flow with a page and profile publication path. | A new scenario, "The support statement is checked across its carriers", with a Mermaid flow and six steps. |
| 7 | 07 deployment-view | No | The profile and page deployment mechanisms, the declaration and the site publication topology are unchanged; only reviewed copy changes. | — |
| 8 | 08 crosscutting-concepts | Yes | The public-copy concept excluded the provider name and had no statement rule. | The public organization copy section names the plan and the new non-affiliation sentence; a new section records the five carriers, the check, the evidence record and the shrink rule. |
| 9 | 09 architecture-decisions | Yes | Three ADR candidates required routing, and none qualifies. | A paragraph records the three `CANONICAL_ARTIFACT` routings; the register, both ADR indexes and the ADR files are unchanged. |
| 10 | 10 quality-requirements | Yes | The statement's cross-carrier truth is a new quality scenario and tree node. | Quality tree node `rp7`; `QS-043` states the disagreement case, the named refusal and the check's limits. |
| 11 | 11 risks-and-technical-debt | Yes | The check cannot prove truth; the expected sentences have one source read by two checks; the profile lags until the operator applies; the disclaimer duplication changes shape. | `R-072`, `TD-027`; `R-032` and `TD-014` are revised to the named plan and the checked carriers. |
| 12 | 12 glossary | Yes | Three terms are added and five existing contract uses change. | **Supported duty plan**, **Carrier** and **Evidence record** are added; **Support statement**, **Evidence for a publication**, **Profile content**, **Organization profile** and **Released Roster detail-table family** are revised; the intro and closing record the change. |

## Notes

- `docs/arc42/README.md` needs no change: no chapter was added, removed or
  renamed, so its twelve rows still match the files.
- The statement's five carriers are the website's `what-it-does` and
  `non-affiliation` elements, the README, `org/profile/README.md`, chapter 3 and
  both issue forms. The check owns the expected sentences; no carrier is
  authoritative and none is generated from another.
- The profile's published carrier lags the reviewed source until the operator
  applies. That is an operator step, not a test step: only the operator holds
  the state passphrase and the administer token, and ADR-0006 and chapter 2
  forbid both in CI. The change's success is verified without the live apply by
  the statement check reading the reviewed source and by the modified profile
  scenarios; the operator's preview, apply and read-back are recorded as the
  live acceptance evidence (R-035).
- The claim rests on the generated fixture under `apps/cli/tests/` converting
  through the unchanged conversion. A fixture failure turns the conversion
  suite red, the refusal names the unknown code or missing fact, and the
  statement shrinks in one act across the five carriers. No real plan, sample
  or identifying value is committed.
- The converter's own unsupported-document refusal keeps its wording
  (`the source is not a supported Released Roster detail-table export`). It is
  conversion behaviour owned by `export-duty-plan-calendar`, and changing it
  would be a separate change with its own refusal scenarios.
- The site publication's own page-content check gains exact-text assertions for
  the two page elements; the cross-carrier check reads the source, not the
  assembled page, so the two checks have distinct jobs. `scripts/site-evidence.sh`
  runs the cross-carrier check as its first evidence step, so the publication
  refuses a revision whose carriers disagree even though the site workflow does
  not wait for CI.

ADR action: NONE
ADR target: none

Candidate 1 — the public statement's unit and boundary:

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/changes/state-the-supported-duty-plan/specs/state-the-converter-support-statement/state-the-support-statement.feature
target_adr: null
replacement_adr: null
```

Candidate 2 — one wording checked across five carriers:

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: docs/arc42/02-architecture-constraints.md
target_adr: null
replacement_adr: null
```

Candidate 3 — the evidence record and the shrink rule:

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/changes/state-the-supported-duty-plan/specs/state-the-converter-support-statement/usecase.md
target_adr: null
replacement_adr: null
```

The three candidates have real alternatives — name the plan, keep a generic
"supported export", or name only the internal family; one checked wording
versus an authoritative carrier or per-carrier wording; a durable evidence
record versus the check's header or a change-scoped one. They have a durable
consequence for future copy and evidence changes, except candidate 2, whose
five-carrier comparison is policy inside existing publishing paths and is
reversed by one change's scripts and copy. All three fail `architectural_impact`:
the statement and its check move no component boundary, dependency direction or
deployment boundary. Candidate 1 fails `lower_level_artifact` because the
statement, its wording and its shrink rule are observable public behaviour owned
by the new capability's Gherkin specification and use case; candidate 3 fails it
because the evidence record's rule is the use case's extension and guarantee.
Candidate 2's canonical artifact is chapter 2's constraint; its check mechanics
are `design.md`, which is not a second routing. The existing-ADR check found no
record covering the statement's wording or its carriers; the archived profile
change's provider-name rule was a constraint, not an ADR, and chapter 2 now
records its replacement. No ADR is added and none is amended.

## Status

STATUS: DONE
