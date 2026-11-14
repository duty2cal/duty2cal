# arc42 impact — recognize-cockpit-duty-codes

## Assessment

| # | Chapter | Affected? | Reason |
|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The goal remains faithful conversion and explicit warnings for unconfirmed codes; no quality goal, stakeholder or public claim changes. |
| 2 | 02 architecture-constraints | No | No constraint, dependency or public-copy rule changes. |
| 3 | 03 context-and-scope | No | The input, actors, external systems and browser states are unchanged; the `Unknown duties` list can be shorter, but its contract does not move. |
| 4 | 04 solution-strategy | No | The same owned extraction, application interpretation and ICS writer remain. |
| 5 | 05 building-block-view | No | The `application` block already owns activity classification; no new module, port or responsibility. |
| 6 | 06 runtime-view | No | The conversion flow and its branches are unchanged; only the set of recognized codes grows. |
| 7 | 07 deployment-view | No | No platform, packaging, hosting or credential change. |
| 8 | 08 crosscutting-concepts | No | No cross-cutting policy changes; the classification rule stays inside the application layer. |
| 9 | 09 architecture-decisions | No | No ADR is qualified: the new code members are data in an existing classifier, not a boundary or contract decision. |
| 10 | 10 quality-requirements | No | QS-001's generated unknown-code plan is unchanged; its assertion is about unconfirmed codes, still true. |
| 11 | 11 risks-and-technical-debt | Yes | R-074 no longer says the converter never reads code letters; it now names the confirmed prefixes. TD-029 records the 2026-11-14 confirmed meanings and the codes that still take the fallback. |
| 12 | 12 glossary | Yes | **Vacation day** and **Day off** now name U1 and OFF_2; **Medical appointment duty** and **Training duty** are new terms; the code invariant adds the new exact members and prefixes. |

## ADR qualification

- **Architectural commitment:** none. The change edits two code-list constants and one prefix predicate inside the existing application classifier; no port, adapter, storage or deployment boundary moves.
- **Real alternatives:** exact-only recognition versus confirmed prefixes was decided by the operator's `WBT_*`/`EBTF3*` wording and is recorded in the proposal; the existing representation versus a translated title is also recorded.
- **Durable consequence:** reversing a recognition changes the constants and its scenarios; no data, interface or compatibility boundary moves.
- **Existing decisions:** ADR-0008 owns the extraction boundary; the label-unknown-duties decision owns the fallback presentation. Neither is amended or superseded. No `NEW_ADR`, `AMEND_ADR` or `SUPERSEDE_ADR`.

## Status

STATUS: DONE
