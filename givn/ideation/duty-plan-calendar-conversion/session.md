# Duty-plan calendar conversion — session state

- Topic: duty-plan-calendar-conversion
- Topic identity: confirmed by the user on 2026-10-10.
- Phase: Handoff complete — published 2026-10-12.
- Frame: confirmed by the user on 2026-10-10.
- Latest user direction: publish the proposed Handoff, confirmed on 2026-10-12. The readable record, one ready seed, both Persona promotions, and thirteen glossary terms have been published as approved.
- Completed authority: the named Handoff records/promotions only. No code, normal change artifact, permanent spec leaf, test/fixture/golden, private-data publication, or change start.
- Next separate workflow: the user invokes normal proposal planning with Export complete Released Roster calendars (export-released-roster-calendar), using changes/export-released-roster-calendar.md. No new clarification or further code authority is assumed by this completed ideation.

## Confirmed inputs

- Preserve the user's stated non-negotiable constraints in `temp_first_idea.md`; any discovered feasibility conflict is a finding, not permission to substitute a technology.
- Keep `private/` out of version control. Do not copy identifying source text, filenames, personal values, or raw extraction output into topic state or public artifacts.
- Private PDFs may be read and analyzed. Privacy restrictions must not be misrepresented as a ban on reading the evidence.
- Use generated or anonymized data in fixtures and golden calendars. Anonymization must address identifying content and metadata, not just a displayed crew name.
- First-release compatibility: only the supplied Released Roster detail-table family and explicitly validated variations; no additional-family or universal-support work is authorized by this choice.
- U means Urlaub/vacation. FREE, OFF, and U are all-day events; source times are ignored. A nonzero time or a source interval appearing to cross midnight does not change this mapping.
- FZ means additional vacation-related days off. The exact acronym, agreement source, age eligibility, and annual quota are not verified or enforced by this converter.
- FZ is all-day on its duty-plan date with source times ignored, just like the settled FREE/OFF/U timing behavior; its source code remains distinguishable.
- TK means required office attendance or professional-development/seminar participation, including team information sharing, and uses its printed UTC Start/End interval as a timed non-flight activity. Required endpoints must resolve; no all-day fallback is invented.
- DT means Duty Time in the observed activity designation. DTUSA means a consulate visit to obtain a USA visa and uses its printed UTC Start/End interval as timed non-flight duty. Required endpoints must resolve; no all-day fallback, visa-processing action, or other DT-suffix mapping is inferred.
- DH is positioning travel to a departure destination in uniform without the usual operating work, and remains duty time. It is a Pos marker on the underlying travel, not a second event, an absence code, or a reason to derive timing from zero Blocktime.
- DH appears in the event title before the printed designator and route. This preserves the assignment distinction directly in compact calendar views without selecting UID or reimport-update behavior.
- For travel whose UTC endpoints have no explicit day qualifiers, share one local-date block, have no separate arrival-date block, and End clock precedes Start, interpret End as the following UTC day. This rule does not override explicit markers, repair equal clocks, or derive endpoints from Blocktime.
- Existing calendars are never overwritten. New calendar exports choose an unused enumerated name when their ordinary basename is occupied; draft notation is `roster.ics`, `roster-1.ics`, `roster-2.ics`. Report the actual new path; no automatic deletion or reimport reconciliation is approved.
- Carry known Trip ID within one unambiguous roster trip for travel-event descriptions, including page/date continuations. Do not leak it into new trips or unrelated duties; this does not propagate report times or offsets.
- Existing diagnostics are preserved under numbered names. New reports identify the attempted source PDF filename and their actual newly written path; an older report must not be claimed as a current write. No old-report deletion or overwrite is authorized.
- Every conversion failure attempts a new numbered diagnostic, including failed calendar publication. If saving that report fails, retain the primary failure and state no new report was saved; do not substitute an old report or claim success.
- Component identities are separate by fixed source-PDF snapshot and distinct activity; repeated identical-snapshot conversion is deterministic even when the new output filename differs. Different snapshots can coexist after import; no automatic update/cleanup is approved.
- Prefer the source Created UTC footer as selected-family revision metadata for DTSTAMP; fall back to the constructed representation's actual UTC timestamp when source metadata is unusable. Never use either timestamp to determine activity dates, and do not substitute the arbitrary first-day-of-roster-month constant.
- The real-source clarification of ordinary split-leg merging is accepted: complementary available fields combine and matching repeats represent one value. Differing optional values remain an unobserved parked robustness policy, not an established first-release source defect.
- SB/SBX/RE are timed availability assignments with their own UTC start/end windows. FRA is the current example station only; consecutive windows/FREE interruptions remain separate, and activation notice is not calculated.
- Expert-provided standby/reserve semantics are authorized for the draft now and explicitly require validation against a later supplied duty plan. No actual row, extraction test, code, or production evidence is invented by accepting the rule.
- Explicit UTC day qualifiers resolve to the unique previous/same/next Gregorian date around their own source local date, carrying month/year; no unique/consistent resolution means failure. Unqualified endpoint dates use their own source context; the selected overnight repair keeps its narrow travel-only scope.

## Research completed

- `givn instructions`: no changes exist.
- `givn status`: no changes exist.
- `givn spec tree`: no permanent specifications.
- Read `README.md`, the introduction/goals, constraints, context/scope, quality requirements, risks, and glossary chapters under `docs/arc42/`; these contain placeholders, not established domain contracts.
- Read `temp_first_idea.md` without editing it; it was untracked user work at the time (it was later committed, then deleted on 2026-10-12 once the change was planned — see below).
- Checked `private/` without displaying filenames or contents: five PDFs, zero tracked files, directory ignored by Git.
- No `pdftotext`, `pdfinfo`, or `mutool` command is available; the PDF-capable read tool nevertheless returned parsed text and rendered pages for all five private PDFs after the user authorized inspection.
- Inspected all five PDFs. Structural findings and generated counterexamples are recorded in `frame.md`; no identifying source values are copied into topic state.
- Reinspected a supplied private PDF after the user requested a checkable source reference. Its split leg has complementary endpoint cells, matching repeated optional values, and some arrival-only metadata. It does not instantiate the generated two-Report-values contradiction; the exact private reference stays in the user-facing answer only.
- Read authoritative documentation/registry metadata for Rust pdfplumber 0.1.0 after Context7 did not resolve that Rust library. The pin exists and documents positioned extraction/table APIs; the draft's method snippets need later adaptation. No dependency upgrade, private-source Rust extraction, compile, resolved-license check, or cross-build was performed.

## Persona reuse inventory

No project Personas or sibling topics existed at the initial and pre-Handoff inventories. Handoff added the two explicitly confirmed project Persona files on 2026-10-12; stable cores and all existing dated records were retained exactly. The topic originals remain historical. Earlier inventory counts below describe the pre-promotion state, not missing current files.

| Confirmed stakeholder type | Project Persona matches | Sibling-topic Persona matches | Disposition |
|---|---|---|---|
| Crew member converting their duty plan | none before Handoff; promoted 2026-10-12 | none | crew-member--personal-calendar-owner promoted with unchanged core/records; topic original historical. Distinguishing criterion: judges fidelity to their own duty plan, unlike the maintainer who judges release conformance. |
| People identified in a duty-plan PDF | none | none | The confirmed crew-member Persona covers the identified owner's privacy interest; no duplicate owner biography. Non-owner segments require separate confirmation if later evidence establishes a distinct authority, evaluation capability, or vocabulary. |
| Project maintainer / release owner | none before Handoff; promoted 2026-10-12 | none | project-maintainer--release-owner promoted with unchanged core/records; topic original historical. Distinguishing criterion: authorizes a public release and judges release evidence, not fidelity to the operator's own schedule. |

## Draft findings

The findings in `frame.md` are research observations and candidate reconstruction implications, not silently approved behavior changes to the first draft.
Date, timing, identity, reporting, preservation, and field-selection rules are approved. The optional-description disagreement, conditional source-derived rights, and formal FZ provenance remain parked in `questions.md`; real-source/build/license/privacy/calendar evidence remains later work.

## Decision history

- The topic-identity decision (Q1), which authorizes persisted topic discussion state, is closed with the user's acceptance; its full question and alternatives remain in `questions.md`.
- The Frame decision (Q2), which establishes the problem, scope, and stakeholder interests, is closed with the user's adoption. The same turn explicitly authorized reading the private PDFs for structural and algorithm analysis.
- The crew-member review-lens decision (Q6), which authorizes one topic-local accuracy/privacy Persona, is closed with the user's acceptance. Its Persona file is created without permanent promotion.
- The maintainer review-lens decision (Q8), which authorizes one topic-local release-conformance Persona, is closed with the user's acceptance. Its Persona file is created without permanent promotion.
- The whole-plan calendar boundary decision (Q9), which establishes the root and invariant for the Event Storming canvas, is closed with the user's acceptance. The definition is confirmed topic-locally only; the permanent glossary is unchanged.
- The first-release supported-export decision (Q4), which governs compatibility breadth, is closed on 2026-10-11: the user selected the supplied Released Roster detail-table family and explicitly validated variations. Multiple-family and universal-support alternatives are rejected for this first-release target.
- The vacation/all-day decision (Q11) is closed on 2026-10-11: the user defined U and also explicitly settled FREE/OFF/U as all-day with source times ignored. Its complete question/options remain in the log; no duplicate mapping confirmation is needed.
- The FZ-meaning decision (Q12) is closed: the user supplied the practical meaning. The FZ acronym/agreement question (Q13) is parked as non-blocking background. The FZ representation decision (Q14) is closed with the user's acceptance; the broader additional-activity question (Q7) still covers TK, DTUSA, and DH details.
- The TK-meaning decision (Q15) and office/seminar timing decision (Q16) are closed with the user's explicit definition and printed-UTC-interval selection. No formal acronym expansion is invented.
- The DTUSA-meaning decision (Q17) and consulate-duty timing decision (Q18) are closed with the user's explicit definition and printed-UTC-interval selection.
- The DH-assignment decision (Q19) and deadhead-visibility decision (Q20) are closed with the user's definition and title-prefix choice.
- The observed additional-activity question (Q7) is closed through the focused user decisions. Unrecognized future variants still fail explicitly; the closed code work does not imply a universal mapper.
- The unqualified-overnight decision (Q21) is closed with the user's next-day interpretation for the narrow travel case. It does not authorize overriding explicit qualifiers or treating the chosen convention as already empirically validated.
- The calendar-preservation decision (Q10) is closed with the user's no-overwrite/enumerated-name variant. Rejecting name collisions without creating a new numbered export is not the selected behavior.
- The trip-description decision (Q23) is closed with the user's within-trip inheritance choice. Roster trip is confirmed as descriptive source grouping, not a new aggregate.
- The failure-report decision (Q22) is closed with the user's numbered-history choice and approved source-filename content.
- The publication-failure decision (Q24) is closed with the user's all-failure report choice. The failure canvas is updated accordingly; the diagram's recovery edge is no longer qualified as open.
- The calendar-identity decision (Q25), timestamp-provenance decision (Q27), standby/reserve definition decision (Q26), single-day-window decision (Q29), and boundary-date decision (Q30) are closed. The original date-association record (Q3) is resolved by source research and those explicit rules. Optional conflicts (Q28) stay unobserved/parked; actual standby/boundary validation remains later evidence work.
- The complete-contract decision (Q31) and Handoff decision (Q32) are closed. Publication/promotions completed on 2026-10-12 without creating a normal change, permanent leaf, or implementation.

## Frame review

- A fresh-context reviewer found no blocking Frame defects on 2026-10-10.
- Advisory corrections were applied without selecting a new direction: retain universal support as a pending alternative; remove a partial-export implication from candidate vocabulary; describe identifying combinations in the privacy check; record publication rights as unresolved.
- The Frame, Personas, boundary, rules, and complete contract are user-approved; Event Storming/contract were rendered and reviewed. Separate Handoff authority was granted and completed on 2026-10-12. No normal change, permanent spec, or implementation was authorized/created.

## Phase reports

- Event Storming: seven past-tense events, four commands, one aggregate root, one primary Actor plus its automation proxy, four policies, three records, and one read model are drawn across two Mermaid panels. Both rendered PNGs are generated domain diagrams with no private source values.
- Event Storming review: initial publication-failure visibility defect corrected by applying the confirmed explicit-failure requirement. The same reviewer rechecked the current source and rendered failure image and found no blocking defects. No recovery or overwrite choice was made.
- Use Case Definition originally proposed one changing `!` leaf, Export a duty-plan calendar (export-duty-plan-calendar). Later contract approval and Handoff preserve that stable ID; internal parsing/date/publication/failure responsibilities are not independent actor goals. The optional conflicting-description policy remains parked, not silently resolved.
- Use Case Definition review: fresh-context reviewer found no blocking defects. Applied explicit failure visibility in the minimal guarantee and qualified diagnostic persistence in the older Frame wording; neither correction selects overwrite or recovery behavior.
- Elaboration completed with user approval and the unobserved optional conflict parked. The subsequent confirmed Handoff published the durable record/promotions and one complete-deliverable ready seed; it did not create code or a normal change scaffold.

## Read-only standards/oracle findings

- Read authoritative RFC UID/DTSTAMP sections; the user's later snapshot-identity and source/fallback provenance choices supply the required policies. The first draft's date/code/route recipe and month-first stamp alone were not proof of compliance.
- Inspected only the supplied synthetic golden text: 19 events, five physical lines over 75 octets, incorrect first weekday for January 2099, an unanchored PII-regex false positive on a generated timestamp, and normalization that erases CRLF distinctions.
- These are recorded findings, not permission to edit the first draft, write fixtures/goldens/tests/guards, or alter an exact dependency pin during ideation.

## Full-contract draft review

- One fresh-context reviewer inspected the stable-schema contract and current rendered failure/actor-goal diagrams. It confirmed one Actor/goal, level, flow/extension structure, guarantees, implementation independence, and all-failure diagnostic attempts.
- It identified one document defect: non-trip optional metadata association/conflict handling was implied without a policy. The draft now enumerates the inherited field list, qualifies association/merge behavior under the descriptive-fragment question (Q28), distinguishes required contradictions from optional disagreements, and includes a generated two-Report-values example.
- The same reviewer verified the correction and final consolidated draft with no blocking defects. The user approved the complete contract and separately confirmed Handoff on 2026-10-12. Optional conflicting descriptions remain unobserved/parked; no substantive choice was made through review.

## Handoff research and reuse checks

- Pre-publication inventory: no project Personas or sibling biography to merge; Handoff subsequently added only the two approved project files.
- Pre-publication glossary: empty template; Handoff subsequently added only the thirteen confirmed introductions/required boundary records, without replacing user definitions.
- `givn instructions specs` required an active change and was not used to create one. The generated local spec skill explicitly names `givn/specs/<usecase-id>/usecase.md`; the proposed future leaf follows that path.
- Handoff published one end-to-end ready seed, not independently shipping parser/writer fragments. It remains normal-proposal input, not an implemented or validated release; no normal change artifact was created.

All search-before-coin blocks below preserve the dated pre-introduction/pre-Handoff
checks and hit counts. Their then-unwritten glossary/code statements are historical;
the current publication state and actual global outputs are listed above.

## Search-before-coin for source-snapshot vocabulary

- `grep(pattern="(?i)source[- ]snapshot|source[- ]pdf snapshot|document version|snapshot identit", path="docs/arc42/12-glossary.md")`: 0 lines.
- `glob(pattern="givn/ideation/**/domain-terms.md")`: 1 current file, 0 siblings.
- The same repository Markdown grep returned 4 lines, all pending identity-choice references in this topic, with no competing definition.
- Source snapshot is confirmed as the fixed input-document version, with genus, contrast, generated sentence, and exact checks in domain-terms.md. It is a record, not a new aggregate; technical fingerprinting and real-data anonymization are not inferred from the identity policy.

## Search-before-coin for source revision metadata

- `grep(pattern="(?i)source revision|revision timestamp|Created footer|DTSTAMP", path="docs/arc42/12-glossary.md")`: 0 lines.
- `glob(pattern="givn/ideation/**/domain-terms.md")`: 1 current file, 0 siblings.
- The same repository Markdown grep returned 38 lines, including first-draft placeholder timestamps and pending-policy references, with no competing domain definition.
- Source revision timestamp is confirmed from the user's Created-footer interpretation and construction-time fallback, with genus, contrast, generated sentence, and exact checks in domain-terms.md. It is metadata, not an event-time input; no permanent glossary, fixture, or code is written.

## Search-before-coin for standby/reserve vocabulary

- Final bounded check: `grep(pattern="(?i:\\b(?:standby|stby|reserve)\\b)|\\bSBX\\b|\\bRE\\b", path="docs/arc42/12-glossary.md")`: 0 lines.
- `glob(pattern="givn/ideation/**/domain-terms.md")`: 1 current file, 0 siblings.
- The same repository Markdown grep returned 27 lines, old standby proposals and evidence questions, with no existing reserve definition. Earlier broader checks matched irrelevant preserve/re-run text and were narrowed; their hit counts are not treated as domain collisions.
- Standby duty, Reserve duty, and Single-day standby meanings are recorded with genus, contrasts, and generated sentences. The later user answer also confirms SBX's own UTC window, with real-duty-plan validation expressly deferred. No new aggregate, dispatcher Actor, activation workflow, or code is created.

## Search-before-coin for the proposed calendar boundary

- Read the permanent glossary; it contains no domain definitions or declared aggregate boundaries.
- `grep(pattern="(?i)duty[- ]plan calendar|crew calendar|roster calendar|conversion aggregate|consistency boundary", path="docs/arc42/12-glossary.md")`: zero matching lines.
- `glob(pattern="givn/ideation/**/domain-terms.md")`: one current-topic file, zero sibling-topic files to search.
- `grep(pattern="(?i)duty[- ]plan calendar|crew calendar|roster calendar|conversion aggregate", include="*.md")`: six matching lines, all topic titles or references to the topic identity; no prior definition or competing aggregate noun.
- These are recorded tool invocations, not claims that equivalent shell commands were run. The user subsequently confirmed the term; the table in domain-terms.md retains the exact checks and their pre-introduction hit counts.

## Current evidence limits

- The examples show the same apparent detail-table family, not the entire Jeppesen export universe.
- Explicit parenthesized day qualifiers disprove direct use of every local date as a UTC date. The adjacent Gregorian-date interpretation is now confirmed; actual source-boundary validation remains unperformed.
- The observed code/assignment semantics, calendar/report preservation, snapshot identity, revision metadata, date rules, inherited EM/dated-Layover mappings, and field selection are user-approved. Optional descriptive conflicts remain parked; actual source/extraction/build/license/privacy/calendar evidence remains unperformed.

## Handoff preview review

- A fresh-context reviewer found no blocking Handoff-preview defects. It verified the stable leaf mapping, one complete-deliverable seed preview/start order, full record retention, explicit Persona copy/merge transformations, confirmed-only terms, privacy, and no premature publication/change start.
- Applied its state cleanup: inherited mappings/field selection are now approved while optional conflicting descriptions remain parked. The published document/seed must contain navigable source-topic links, and promotion records use the actual confirmed Handoff date.
- The user confirmed the preview through the Handoff decision (Q32) on 2026-10-12; the named records/promotions are published and verified below, not treated as code or release readiness.

## Published outputs and verification

- Human record: [Duty-plan calendar conversion](../../../docs/ideation/duty-plan-calendar-conversion.md).
- Ready seed: [Export complete Released Roster calendars](changes/export-released-roster-calendar.md), Status: ready; first/only start, no prerequisite change; stable use-case ID export-duty-plan-calendar retained.
- Project Personas: [Crew member exporting their own duty plan](../../personas/crew-member--personal-calendar-owner.md) and [Maintainer responsible for a public release](../../personas/project-maintainer--release-owner.md). Read-only comparison verified exact stable core and all pre-promotion dated records; promotion records use 2026-10-12.
- Glossary: [Thirteen confirmed introductions and one declared boundary](../../../docs/arc42/12-glossary.md); Duty plan, Flight leg, Calendar export remain candidates in topic state. No rejected entry was deleted.
- Read-only checks: seed title/ready marker/required sections/stable ID valid; local Markdown link targets exist; no actual private PDF filename appears in those records; bounded identifying-token scan found no match.
- Private files remain ignored with zero tracked files under private/. `givn status` confirms no normal change; no permanent usecase.md or implementation was created. `git diff --check` passed for tracked changes.
- The token scan is not a universal PII/anonymization proof. Actual data provenance and full source/build/calendar/Windows evidence are required later. The user draft and private sources were not edited, staged, or committed.
- The same Handoff reviewer checked the published document, seed, glossary, and actual promotion metadata and found no blocking publication defects. Its non-blocking stale resume wording was corrected without rewriting historical Persona records or approved preview diffs.
- This is structural analysis, not a performance or feasibility test of the proposed Rust extraction dependency.

## Search-before-coin for vacation vocabulary

- `grep(pattern="(?i)vacation|urlaub|vacation day", path="docs/arc42/12-glossary.md")`: 0 lines; the same repository Markdown search returned 0 lines.
- Alias-inclusive check: `grep(pattern="(?i)vacation|urlaub|\\bU\\b", path="docs/arc42/12-glossary.md")`: 0 lines; `grep(pattern="(?i)vacation|urlaub|\\bU\\b", include="*.md")`: 13 lines, all existing unknown-U references in this topic, no competing definition.
- `glob(pattern="givn/ideation/**/domain-terms.md")`: 1 current file, 0 siblings.
- Vacation day is confirmed from the user's explicit definition/mapping, with genus, contrast, generated example, and exact checks recorded in domain-terms.md. The durable glossary remains unchanged during ideation.

## Search-before-coin for additional vacation vocabulary

- `grep(pattern="(?i)additional (vacation|day|time)|frei zus|\\bFZ\\b", path="docs/arc42/12-glossary.md")`: 0 lines.
- `glob(pattern="givn/ideation/**/domain-terms.md")`: 1 current file, 0 siblings.
- The same repository Markdown grep returned 17 lines, all existing unknown-FZ references in this topic; no competing definition.
- Additional vacation day is confirmed as a practical meaning, with genus, contrast, generated sentence, and exact checks in domain-terms.md. Its timing was subsequently confirmed through a separate user decision; no permanent glossary update is written.

## Search-before-coin for office/seminar vocabulary

- `grep(pattern="(?i)office|seminar|professional development|\\bTK\\b", path="docs/arc42/12-glossary.md")`: 0 lines.
- `glob(pattern="givn/ideation/**/domain-terms.md")`: 1 current file, 0 siblings.
- The same repository Markdown grep returned 18 lines, all unresolved-TK references in this topic; no competing definition.
- Office or seminar duty is confirmed as the user's practical meaning, with genus, contrast, generated sentence, and exact checks in domain-terms.md. The user subsequently confirmed printed-UTC-interval timing through a separate decision; no permanent glossary or code change is written.

## Search-before-coin for consulate-duty vocabulary

- `grep(pattern="(?i)visa|consulate|duty[ -]?time|\\bDTUSA\\b|\\bDT\\b", path="docs/arc42/12-glossary.md")`: 0 lines.
- `glob(pattern="givn/ideation/**/domain-terms.md")`: 1 current file, 0 siblings.
- The same repository Markdown grep returned 19 lines, all unresolved-DTUSA references in this topic; no prior expansion or competing definition.
- DT prefix and Consulate visa duty are confirmed from the user's definition, with genus, contrast, generated sentences, and exact checks in domain-terms.md. Numeric duty-accounting values and unknown DT suffixes remain distinct; no permanent glossary or implementation change is written.

## Search-before-coin for deadhead vocabulary

- `grep(pattern="(?i)dead[ -]?head|positioning|\\bDH\\b", path="docs/arc42/12-glossary.md")`: 0 lines.
- `glob(pattern="givn/ideation/**/domain-terms.md")`: 1 current file, 0 siblings.
- The same repository Markdown grep returned 25 lines: unknown-DH references plus Persona vocabulary/value statements, with no competing definition.
- Deadhead duty is confirmed with genus, contrast, generated sentence, and exact checks in domain-terms.md. The role is distinct from a flight designator and from numeric work/accounting durations. No permanent glossary or code change is written.

## Search-before-confirmation for roster-trip grouping

- `grep(pattern="(?i)roster trip|\\bpairing\\b|trip identifier|trip id", path="docs/arc42/12-glossary.md")`: 0 lines.
- `glob(pattern="givn/ideation/**/domain-terms.md")`: 1 current file, 0 siblings.
- The same bounded repository Markdown grep returned 12 lines: the candidate term and source-field/topic references, without a competing grouping definition. An initial unbounded pairing search returned 13 because it also matched “repairing”; the bounded check excludes that irrelevant hit.
- Roster trip is confirmed from the user's within-trip inheritance choice, with genus, contrast, generated sentence, and exact checks in domain-terms.md. Duty-plan calendar remains the only aggregate; no permanent glossary or code change is written.

## 2026-10-12 — first draft superseded and deleted

- The user confirmed that `temp_first_idea.md` is superseded and may be deleted.
- Superseded by: `frame.md` (Mission, non-negotiable constraints, structural findings, and the recorded draft defects), the approved contract in `use-cases/export-duty-plan-calendar.md`, the ready seed `changes/export-released-roster-calendar.md`, and the normal change's `proposal.md` / `specs/` / `design.md`. Its concrete artifacts were explicitly replaced rather than reused: the January-2099 synthetic oracle and its L1/L2/L3 test shape (replaced by the Gherkin suite plus the operator's June-2026 oracle scenario), the fixed output/error paths (replaced by enumerated no-overwrite names and numbered diagnostics), the MIT/Apache-only licence rule and `=0.1.0` pin (replaced by ADR-0001's MPL/GPL split and `pdfplumber = "=0.4.1"`), and the Clean-Architecture file layout (replaced by ADR-0002's two-crate workspace).
- The maintainer release-evidence lens recorded against it still binds: its synthetic golden had five overlong lines, its normalization erased the CRLF property, and its PII regex false-positived on a generated `DTSTAMP` — so none of its oracle, guard, or test text may be reused verbatim.
- Preserved before deletion: the real-PDF `.gitignore` patterns (now in `.gitignore`), the `x86_64-w64-mingw32-gcc` linker note, the pattern-based PII guard, Typst fixture generation with metadata stripping, and the small-binary release profile (now in `design.md`/`tasks.md`).
- The file was tracked in Git, so its full text remains retrievable from history; references in this topic are historical and resolve through that history.
