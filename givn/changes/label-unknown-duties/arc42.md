# arc42 impact — label-unknown-duties

Source read in full: [proposal](proposal.md), [design](design.md),
[delta feature](specs/export-duty-plan-calendar/export-duty-plan-calendar.feature)
and [usecase](../../specs/export-duty-plan-calendar/usecase.md).

The change titles every activity represented from an unrecognized activity code
`unknown duty: <printed code>`, and redraws the converter page's two ready states
(`Calendar ICS ready`, the count sentence, the `Unknown duties` list, the offer
copy, the two save labels, the way-back label). Conversion outcomes, the failure
state, the warning report, the executable and the publication rules are
unchanged.

## Chapter assessment

| # | Row | Affected | Reason and change |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | Yes | Chapter 10's acceptance criteria name the calendar title and the ready-state copy, so the two rewritten observable facts must be recorded there. Chapter 01 unchanged: the goal, the actors and the three Personas stay the same. Changed: QS-001's title criterion and QS-046's ready-state criterion now state `unknown duty: <code>`, the `Unknown duties` list, the count sentence and the two save labels. |
| 2 | Constraints (legal, tech, org) | No | No dependency, platform, licence, CSP, privacy or delivery constraint moves; the page still writes nothing, loads no third-party asset and makes no request after load. |
| 3 | External systems, interfaces, context, new user-facing surface | Yes | The browser route is a user-facing surface whose described output changes. Changed: the context table's browser row now names the ready heading, the count sentence, the `Unknown duties` list with date and optional UTC start, and the warning report labelled `Save unknown duty warning for publisher`; the browser context flow gains one edge for the list and the publisher offer. |
| 4 | Major technical strategy or approach | Yes | The strategy is unchanged — the staged pipeline, the completeness gate, the browser-local offer, the no-clobber publication and the severity model stay — but its summary asserted that the page "lists the conversion warnings beside the calendar", which the change makes false: the ready state lists unknown duties and only the failure state lists warning sentences. Changed: chapter 04, that one browser-channel clause, corrected rather than restyled. Marked Yes because the chapter file changes and this table is cross-checked against an independent read. |
| 5 | New building blocks, modules, components | Yes | No crate, port, adapter, module or test target is introduced, but two listed blocks change shape. Changed: the `ffi/wasm` row now states that the success result returns the calendar with its warnings and the represented unknown duties in plan order; the `index.html` row now names the ready heading, status region, `Unknown duties` list, offer copy, both save labels and the way back; the `app.js` row now renders the structured unknown-duty list and labels the publisher offer. |
| 6 | New runtime flows, sequences | Yes | The browser success flow now returns and renders unknown-duty facts. Changed: the sequence's facade answer carries `unknownDuties` and the page renders the heading, status and list; the steps state the two ready states, the list line forms and the two offer labels. |
| 7 | Deployment changes | No | No build input, bundle member, hosting target, workflow or release artifact changes; `web/styles.css`, the driver and the Site output's member set are untouched. |
| 8 | Cross-cutting concepts (error handling, security, config) | Yes | The page-presentation concept sentence is part of the durable cross-cutting contract. Changed: the diagnostic/report concept now says the browser lists the unknown duties in plan order with their date and optional UTC start and offers the publisher's warning report. |
| 9 | Architecture decisions (tradeoffs, ADRs) | Yes | The qualification gate ran for the change's one candidate decision and returned `NOT_QUALIFIED` → `CANONICAL_ARTIFACT`; no ADR is minted and the register, `adr/README.md` and the ADR files are unchanged. Changed: chapter 09 records that verdict and its routing. |
| 10 | New quality scenarios | No | No distinct quality requirement is missing: QS-001 and QS-046 already carry the correctness and browser-visibility scenarios, and both are amended under row 1. No scenario is added. |
| 11 | New risks or technical debt | Yes | R-074 and TD-029 describe the fallback-representation assumption, whose visibility changes. Changed: both mitigations now say the imported calendar's own title carries `unknown duty: <code>`, beside the warning report and the page's `Unknown duties` list; no new risk is invented. |
| 12 | New domain terms | Yes | The presented name of the fallback activity becomes durable vocabulary. Changed: chapter 12 adds **Unknown duty** (genus: a calendar activity; differentia: represented from an unrecognized code and titled `unknown duty: <printed code>`), refines **Conversion warning** to name both presentations, and keeps **Standby shift** correct — those three codes are recognized and produce no unknown duty. |

## ADR qualification

Candidate: the unknown-duty presentation contract (the calendar title for every
activity represented from an unrecognized code, the page's structured
`unknownDuties` projection and `Unknown duties` ready list, and the
publisher-specific warning-report offer label).

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/specs/export-duty-plan-calendar/usecase.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Keep the bare printed code and explain the fallback only in the warning report (proposal, Problem / Opportunity).",
      "Parse the diagnostic sentence in the page instead of projecting a structured fact (proposal D1, option 2).",
      "Apply the prefix only to codes without a standby shape (proposal D2, option 2).",
      "Reword the failure state, the warning report and the executable lines too (proposal D3, option 2)."
    ],
    "architectural_impact": [
      "No crate, port, adapter, facade, deployment or authority boundary moves; `ffi/wasm` keeps wrapping the same application boundary (design.md, Architecture impact).",
      "The change adds one derived field to the existing `ConversionOutcome` and one arm to the existing `summary()` accessor (design.md, Data model).",
      "Publication, no-clobber naming, report filenames and the failure state are untouched (design.md, Decisions, D3)."
    ],
    "durable_consequence": [
      "Reversal is one change's title arm, one projection and one page state, all inside the same boundary (design.md, Data model / Derivation).",
      "The calendar and report file names, the report text and the CLI contract stay as they are, so no migration or caller change is required (design.md, Browser result shape)."
    ],
    "lower_level_artifact": [
      "The title rule, the two ready-state copies, the list order and the offer labels are observable behaviour owned by the `export-duty-plan-calendar` use case and its delta Gherkin.",
      "The projection shape, the facade array shape and the page markup are design detail owned by `design.md`."
    ],
    "existing_adr_check": [
      "ADR-0001 … ADR-0008 searched in docs/arc42/09-architecture-decisions.md and docs/arc42/adr/.",
      "ADR-0007 fixes the browser-local conversion facade boundary, which this change does not move.",
      "No record covers calendar titles, ready-state copy or offer labels."
    ]
  }
}
```

Falsification tests: single-change reversibility holds (one component family,
one change); no multi-change constraint is introduced; the product-behaviour
exclusion applies — the candidate is presentation of an existing outcome, and it
moves no component, authority or deployment boundary. Chapter 09 therefore
needs no new ADR, and chapter 11 gets no new "Bad, because…" line.

## Status

STATUS: DONE
