# arc42 Documentation Update: state-site-footer

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The goals and quality goals speak of the page's statements, not of where a duplicate sat. | — |
| 2 | 02 architecture-constraints | No | No constraint names the footer's prose; the constraint is the one-element-per-statement rule, which the page now meets. | — |
| 3 | 03 context-and-scope | No | The context describes the page's panel and its reader, not the footer's sentences. | — |
| 4 | 04 solution-strategy | No | The publishing strategy is unchanged. | — |
| 5 | 05 building-block-view | No | The `web/` row lists the panels and the footer; it does not claim prose in the footer. | — |
| 6 | 06 runtime-view | No | No flow changes. | — |
| 7 | 07 deployment-view | Yes | The page-layout row said the footer carries "plain prose". | The row now names the Built revision, the local-processing note and the two notice links. |
| 8 | 08 crosscutting-concepts | No | The public-copy section lists the statements and their carriers; none is the footer. | — |
| 9 | 09 architecture-decisions | No | No decision qualifies; the rule lives in the use case. | — |
| 10 | 10 quality-requirements | No | No quality scenario asserts the footer's prose. | — |
| 11 | 11 risks-and-technical-debt | Yes | TD-014 recorded exactly this duplication and is now paid. | The row is removed; the risk it described no longer exists. |
| 12 | 12 glossary | No | "Site footer" is not a glossary term; the footer's role is in the design system. | — |

## Notes

- **ADR routing.** The choice "the footer carries no repeated statement" is a
  page-content rule with a lower-level home: the use case rule and the design
  system's footer primitive. Verdict: `CANONICAL_ARTIFACT`; the register, the
  ADR indexes and the ADR files are unchanged.
- The change's delta modifies the site-publication scenario's footer step; the
  permanent feature is unchanged until the archive merges the delta.

## Status

STATUS: DONE
