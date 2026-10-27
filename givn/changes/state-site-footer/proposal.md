# Proposal: state-site-footer

## Release note

> Required. One complete, user-facing sentence naming what this change does.
> Every revision this change records carries the release note as its message subject, with the scenario title in the message body.

The converter page's footer names the Built revision, the local-processing note and the two notice links, and no longer repeats the statements panel's sentences.

## Use-Case Context

- Use-Case ID: `publish-public-website` (the page's copy is that capability's; this change corrects one sentence rule and its check)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `visitor--project-reader` (reads the page), `project-maintainer--release-owner` (answers for what the page says), `contributor--web-reviewer` (judges the page content and its check)

## Problem / Opportunity

The converter page states the non-affiliation sentence twice: once as the panel's `non-affiliation` statement — "Not affiliated with Lufthansa, Jeppesen, or any duty-plan provider or airline." — and once again in the footer, followed by the as-is sentence. A reader meets the same sentence twice on one page, and the second copy is prose no check compares against the statement it repeats; the project recorded that drift risk as technical debt TD-014.

The page's rule said both things at once: the footer "repeats the non-affiliation, as-is and privacy terms as plain prose" and "no statement is stated twice". The two cannot both hold.

## Proposed Solution

The statements live in the two panels; the footer carries what only it carries:

- the Built revision, naming and linking the commit;
- the page's local-processing note ("No account, email or upload is needed … no request after it loads.");
- the links to the Impressum and the Datenschutzerklärung.

The non-affiliation and as-is sentences appear once, in the statements panel. The content check and the support-statement check read the statement element, not a footer copy; both lose the assertion that required the duplicate. The notice pages already carry link-only footers, so all three pages now follow one rule.

Observable difference for the reader: the sentence appears once. Observable difference for the reviewer: no footer prose can drift from the statement, because no footer prose states it.

## Capability Routing

`givn spec route --change state-site-footer` reports no signal; the delta names one existing capability.

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| publish-public-website | no signal | EXTEND publish-public-website | The footer's sentences and the page-content check already belong to the site publication; the change corrects one rule through it and adds no capability. |

## Out of Scope

- The statements panel's wording, the notice pages and their content.
- The page's visual structure: the footer keeps its place and its links.
- Any new statement or carrier.
