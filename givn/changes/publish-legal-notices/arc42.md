# arc42 Documentation Update: publish-legal-notices

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The requirements overview and the quality goals did not name a legal notice, and a public page without one is a compliance defect rather than a missing feature. | The requirements overview gains the two notice pages and their reachability; a quality goal records that every public carrier states who publishes it and what a page request processes. |
| 2 | 02 architecture-constraints | Yes | A new binding constraint appears: the Site output carries two notices, the three page footers link them, and no carrier copies the postal address. | A constraint row records the two pages, the footer links, the no-script policy for them, the cross-carrier check in the `public-text` job and the output gate's refusal. |
| 3 | 03 context-and-scope | Yes | The reader is a new counterpart of the site: the notices describe the operator and the host, and the hosting service is now described in the page set rather than only in the architecture. | A new subsection and a partner-table row record the reader, the notices, the host's IP logging, and the scope boundary that the notices are a project record, not a converter behaviour. |
| 4 | 04 solution-strategy | Yes | The published-site row enumerates what the assembler copies, so the two new members appear there; the strategy itself is unchanged. | The published-site row names the two legal notice pages among the assembled members. |
| 5 | 05 building-block-view | Yes | Two new page members, one new source check, and changed rows for the assembler, the content check and the public-text check. | Rows for `web/impressum.html`, `web/privacy.html` and `scripts/legal-notices-check.sh`; the `build-site.sh`, `check-site-content.sh` and `public-text-check.sh` rows name the notices; the member count becomes eleven. |
| 6 | 06 runtime-view | Yes | A new runtime flow: the notices are checked, assembled, published, and read by a visitor through the footer. | A new scenario, "The legal notices are checked and published", with a Mermaid flow and its steps. |
| 7 | 07 deployment-view | Yes | The artefact row enumerates the output members and changes from nine to eleven; the page-layout row gains the notice pages' shell. | The artefact row names `impressum.html` and `privacy.html`; the page-layout row records the two notice screens; the assembler row copies both. |
| 8 | 08 crosscutting-concepts | Yes | A new crosscutting rule: the notices are static, script-free, same-origin pages; and the public-copy concept gains a second checked claim. | A section records the notice pages' policy, their no-script rule and the two checks that enforce them; the public-copy section names the notices as the second checked claim. |
| 9 | 09 architecture-decisions | Yes | Three candidates required routing, and none qualifies. | A paragraph records the `CANONICAL_ARTIFACT` routings; the register, both ADR indexes and the ADR files are unchanged. |
| 10 | 10 quality-requirements | Yes | Reachability in one step and the refusal of a placeholder are new measurable requirements. | Quality tree node and a quality scenario stating the reachability rule and the named refusal. |
| 11 | 11 risks-and-technical-debt | Yes | Two new risks: the address drifts and the check cannot prove serviceability; the c/o address depends on a consent the repository cannot record. | `R-073` and `TD-028`; the public-copy risk row is revised to name the second checked claim. |
| 12 | 12 glossary | Yes | Four terms are added and the Site output's member list changes. | **Impressum**, **Datenschutzerklärung**, **Legal notice** and **Legal-notice carrier** are added; **Site output** and **Public website** are revised to eleven members and the two notices. |

## Notes

- `docs/arc42/README.md` needs no change: no chapter is added, removed or reordered.
- **ADR qualification.** Three candidates, each walked through the gate:
  1. *Static, script-free notice pages under a stricter content security policy.*
     Alternatives exist (reuse the converter's policy, or render the notices with
     a script), architectural impact is limited to two page members, and the
     complete rationale fits the use case's rule and `design.md`. Verdict:
     `CANONICAL_ARTIFACT` — `design.md` "Technology decisions" plus the notice
     rule in `givn/specs/state-the-public-legal-notices/usecase.md`.
  2. *One output gate plus one source-level cross-carrier check, instead of one
     combined script.* Alternatives exist (one script reading both trees), and
     the consequence is contained in the two scripts' contracts. Verdict:
     `CANONICAL_ARTIFACT` — `design.md` "Design decision: the output gate and the
     source check stay separate".
  3. *No build stamping on the notice pages.* Alternatives exist (stamp both
     pages like the converter page), and the reason is a page-content rule. Verdict:
     `CANONICAL_ARTIFACT` — `design.md` "Technology decisions" and the notice
     rule.
  The register, `docs/arc42/adr/README.md` and the ADR files are unchanged; no
  existing ADR is amended or superseded, because none covers page-level content
  policy or a second output gate.
- **The notices describe processing the project does not perform.** The host logs
  the visitor's IP address; the operator cannot read those logs and cannot
  delete them. Chapter 3's host row and the Datenschutzerklärung say the same
  thing, and neither promises a control the project does not have.
- **The operator's contact block is not a repository fact.** The employer, the
  postal address, the mail alias and the employer's consent to accept service
  are supplied by the operator; the repository holds the block once supplied and
  the check refuses a placeholder in its place. No chapter records the literal
  address.
- **Reachability is a page fact, not a legal opinion.** The check proves that
  the converter page's footer links both notices, that each notice's footer
  links the other notice and the converter page, and that no other carrier
  copies the address; it cannot prove that the notices are legally sufficient,
  and no chapter claims it does.

## Status

STATUS: DONE
