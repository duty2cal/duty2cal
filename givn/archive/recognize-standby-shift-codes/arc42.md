# arc42 Documentation Update: recognize-standby-shift-codes

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The goal "faithful conversion" and the quality attributes are unchanged; recognizing three confirmed codes narrows the existing fallback rather than adding a goal, stakeholder or quality attribute. | — |
| 2 | 02 architecture-constraints | No | No legal, technical or organizational constraint changes: no dependency, platform, tooling or licensing decision moves. | — |
| 3 | 03 context-and-scope | No | No external system, interface or user-facing surface changes; the executable and the browser page keep the same inputs and outputs, and the supported duty plan is the same one. | — |
| 4 | 04 solution-strategy | No | The staged pipeline and the whole-plan severity gate are untouched; the three codes are recognized inside the existing classification step and the fallback strategy remains for every other code. | — |
| 5 | 05 building-block-view | No | No new module, crate, port or responsibility; chapter 5 describes the `application` classification responsibility without enumerating activity codes, so its text stays accurate. | — |
| 6 | 06 runtime-view | No | The conversion flow and its error/warning branches are unchanged; the chapter's description of the unrecognized-code fallback remains true for codes outside the widened set. | — |
| 7 | 07 deployment-view | No | Nothing is deployed differently: same executable, same Site output, same platform and release pipeline. | — |
| 8 | 08 crosscutting-concepts | No | Error handling, severity, purity of classification and privacy are unchanged; the recognized set is data within the existing literal-set concept, and the code's letters are still never read as a kind. | — |
| 9 | 09 architecture-decisions | No | No ADR: the recognition decision is product behavior owned by the use case and Gherkin, is reversible within one change, and touches one component (see the qualification verdict below). The register and `adr/README.md` are unchanged. | — |
| 10 | 10 quality-requirements | No | No new or changed quality scenario; the existing faithfulness/completeness scenarios already describe representable rows publishing with the correct kind. | — |
| 11 | 11 risks-and-technical-debt | Yes | The change fires TD-029's own revisit trigger and narrows the debt; R-004 must distinguish the shift meaning the operator validated on real dated rows from the semantics still awaiting their own rows. | TD-029 rewritten: the fallback debt remains for every other unrecognized code, and the three shift codes are recorded as known availability that reuses the shared printed-window rule (`row_interval`/`station`, unchanged) and produces no warning. R-004 split: the shift meaning is validated by the operator's real November 2026 dated rows and only the converter's recognition is exercised by generated fixtures, while the expert-provided SB/SBX/RE semantics and the qualifier boundary rule still await their own dated-row validation. |
| 12 | 12 glossary | Yes | Three new domain codes enter the ubiquitous language, and the availability vocabulary must stay exact. | Availability duty's code list gains the three shift codes; a new **Standby shift** term (genus, contrast pair, code symbols, anti-terms) records Standby Früh/Mittel/Spät and the no-prefix/no-translation contrasts; the timing invariant row lists the shift codes among the own-window codes; the provenance paragraph records the new term. |

## Notes

- The only production-code edit is the literal availability-code set plus one doc comment; the ICS serializer, interval resolution, report builders and page are reused unchanged, so no architectural boundary moves.
- ADR action: NONE
- ADR target: none

Structured qualification verdict recorded before any MADR file is created or edited:

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: FAIL
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: givn/specs/export-duty-plan-calendar/usecase.md
target_adr: none
replacement_adr: none
```

Evidence for the failed dimensions: the decision changes no component, authority, deployment or durability boundary (`architectural_impact`); reversing it is one array edit in one crate (`durable_consequence`); and the complete rationale is correctly owned by the use-case rule plus its scenarios (`lower_level_artifact`), so the product-behavior exclusion applies. `existing_adr_check`: the active register (ADR-0001…ADR-0007) and `docs/arc42/adr/README.md` hold no activity-code or representation-vocabulary decision, so no amendment or supersession route exists. The destination is exactly one artifact, `givn/specs/export-duty-plan-calendar/usecase.md`; the `export-duty-plan-calendar` Gherkin scenarios are execution evidence for that rule, not a second canonical home.

---

## Status

STATUS: DONE
