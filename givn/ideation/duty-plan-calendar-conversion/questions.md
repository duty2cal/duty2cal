# Duty-plan calendar conversion — question log

## Q1 — Topic identity and persistence

- Status: closed 2026-10-10 (user)
- Blocks: selecting the topic identity and authorizing topic-local discussion state.
- Context: the user requested structured ideation based on `temp_first_idea.md`; there are no earlier topics, permanent specifications, or Personas. The Mission describes supplying a duty-plan PDF and receiving a calendar file. Topic naming is not Frame approval or authorization to implement.
- Question: accept Duty-plan calendar conversion (`duty-plan-calendar-conversion`) as the proposed topic identity, or replace it with another name?
- Options:
  1. **Accept this topic identity** — create topic-local discussion state under `givn/ideation/duty-plan-calendar-conversion/`, distinguishing draft material from confirmed decisions.
     In practice: the UTC-date uncertainty remains visible next session.
     Gain: the discussion can resume without losing its reasoning.
     Cost: persisted notes need maintenance.
     Authorization: topic state only, not a confirmed Frame, Personas, change seeds, or implementation.
  2. **Revise the identity** — supply a different topic name before any state is written.
     In practice: “Personal crew-calendar export” would use `personal-crew-calendar-export`.
     Gain: the name matches the intended domain.
     Cost: persistence waits for that choice.
- Resolution: **Accept this topic identity** because the user replied “Accepted.” Rejected: **Revise the identity** because the user accepted the proposed name and supplied no replacement. The same response confirmed that `private/` contains private examples that must not be committed and that fixture and golden-calendar data must be anonymized or generated.

## Q2 — Problem, scope, and stakeholder interests

- Status: closed 2026-10-10 (user)
- Blocks: starting Persona definition and Event Storming from an agreed Frame.
- Context: the Mission in `temp_first_idea.md` supplies a local converter, but the underlying problem and stakeholder interests are not yet confirmed. The proposed Frame in `frame.md` identifies accurate personal-calendar conversion without manual re-entry as the problem, bounds the product to conversion rather than importing or synchronization, and leaves dates and supported formats unresolved. The user's privacy direction is already confirmed and is not being reopened.
- Question: adopt the proposed Frame—accurate personal-calendar conversion of published duty plans; local PDF-to-calendar output or an explicit failure; no automatic import, synchronization, hosting, or schedule editing; and the displayed interests of crew members, people identified in the PDF, and the maintainer/release owner—or revise a named part before domain exploration?
- Options:
  1. **Adopt the proposed Frame** — use the displayed problem, scope, and stakeholder interests as the basis for Persona proposals and domain exploration.
     In practice: explore how an overnight flight becomes one correct calendar event; do not add a calendar-account connection or uploaded-PDF service.
     Gain: exploration has a bounded goal and explicit accuracy, privacy, and release interests.
     Cost: excluded goals need separate consideration; this does not settle date interpretation or compatibility breadth.
     Authorization: topic-local ideation only; no approved Persona, implementation, change artifacts, or Handoff.
  2. **Revise a named part first** — correct the inferred problem, a scope boundary, or a stakeholder interest before deriving Personas and events.
     In practice: if the problem is specifically flight-leg visibility rather than representation of all activities, state that before elaborating FREE, OFF, and standby outcomes.
     Gain: avoids deriving contracts from the wrong objective.
     Cost: the affected exploration waits for the corrected Frame.
- Later-change cost: changing the Frame after elaboration requires revisiting affected Persona reviews, event chains, and use-case contracts; accepting now buys a coherent basis, not an irreversible commitment.
- Resolution: **Adopt the proposed Frame** because the user explicitly adopted it. Rejected: **Revise a named part first** because the user requested no Frame revision. The same response expressly authorized reading the private PDFs to learn plan structure and derive the conversion algorithm, clarifying that committing private material—not reading it—is prohibited. This does not authorize implementation or publishing identifying source content.

## Q3 — Date-block association with UTC times

- Status: closed 2026-10-11 (source research and user-confirmed interpretation rules)
- Blocks: any unconditional rule for assembling calendar event instants from `Date (LT)` and UTC time columns.
- Context: PDF Parsing Algorithm steps 6–7 in `temp_first_idea.md` combine date-block dates directly with UTC times while the header explicitly names a local date. The export may encode a specific association convention; that convention has not been established. A generated example of 6 January 00:30 at UTC+02 is 5 January 22:30 UTC.
- Question: what date-association convention do the private exports actually use for `Start (UTC)` and `End (UTC)`, especially when the local day and UTC day differ, and what non-identifying evidence establishes it?
- Options: factual evidence request, not a preference choice. Inspect local examples without disclosing their identifying contents; if the convention cannot be established, keep the rule unresolved rather than choose a guessed interpretation.
- Evidence update 2026-10-10: authorized inspection shows explicit parenthesized day qualifiers in UTC time cells, including previous-day arrivals and next-day departures relative to the local-date block. Start and End qualifiers must remain separate evidence; direct block-date assembly is disproved by the corpus. Qualifier interpretation at month/year boundaries and genuinely ambiguous unqualified cases remain unverified.
- Resolution: source inspection establishes independent parenthesized UTC day qualifiers; the user selected the narrow unqualified-travel next-day rule through the overnight decision (Q21) and the unique adjacent Gregorian-date interpretation through the boundary-date decision (Q30). Use the source row's date context for an unqualified endpoint and the explicit day marker's uniquely matching previous/same/next date for a qualified one; carry month/year with calendar arithmetic. Genuinely missing/contradictory required date evidence still fails under the complete-or-explicit-failure rule, and unmatched fragments are not reconstructed from Blocktime. Empirical month/year-boundary validation remains an evidence obligation, not an unresolved interpretation preference.

## Q4 — First-release supported duty-plan exports

- Status: closed 2026-10-11 (user)
- Blocks: defining an honest, testable compatibility guarantee for the first release.
- Context: acceptance criterion 9 in `temp_first_idea.md` promises any valid Jeppesen duty-plan PDF. All five private PDFs have now been inspected and show one apparent Released Roster detail-table family, with repeated headers, page continuations, explicit UTC day qualifiers, and varied surrounding statistics. This establishes concrete first-family evidence, not the universe of Jeppesen exports. The choice determines which sources and generated/anonymized examples are required before claiming a release ready; every alternative retains the privacy and fail-without-silent-omission constraints.
- Question: should the first release promise the Released Roster detail-table family represented by the supplied examples, several explicitly evidenced export families, or retain the original any-valid-Jeppesen-PDF ambition pending adequate evidence?
- Options:
  1. **One evidenced export family** — promise the Released Roster detail-table family represented by the supplied examples and its explicitly validated variations; required activity meanings still need resolution before the claim is ready.
     In practice: page-continuation rows and qualified UTC times in that family are supported; a different unverified table family receives explicit unsupported-source failure rather than a guessed calendar.
     Gain: a concrete first-release target with directly available local evidence.
     Cost: other Jeppesen export families remain outside the support promise; this is narrower than the draft's universal ambition.
  2. **Several evidenced export families** — identify and validate each included export family before claiming support.
     In practice: two distinct table layouts each have generated or anonymized examples and verified expected calendars.
     Gain: broader crew coverage with an explicit compatibility boundary.
     Cost: obtain additional source families, establish their semantics, and build generated/anonymized evidence before release; currently only one family is evidenced.
  3. **Retain the universal-support ambition pending evidence** — keep the draft's any-valid-Jeppesen-PDF objective rather than approve a narrower promise; do not declare it achieved without establishing the relevant format universe and feasibility.
     In practice: an unseen third layout is not excluded merely because the first two were tested; investigation must determine how it fits the universal promise before release.
     Gain: preserves the original coverage objective for all intended crew members.
     Cost: substantially broader discovery and verification, with readiness blocked if the scope cannot be bounded or evidenced.
- Later-change cost: expanding a documented compatibility promise requires additional evidence and tests; narrowing it after release changes users' expectations.
- Evidence update 2026-10-10: all five private PDFs have now been inspected; their detailed activity tables show the same apparent format family, while some surrounding statistical tables vary. This does not establish the universe of Jeppesen export layouts or settle the compatibility promise.
- Resolution: **One evidenced export family** because the user selected option 1, the supplied Released Roster detail-table family with explicitly validated variations. Rejected for the first release: **Several evidenced export families**, which requires additional source families outside the selected target; **Retain the universal-support ambition pending evidence**, because the user selected the narrower first-release promise rather than the draft's universal objective. This does not establish that every variation or activity is already understood; semantics and conversion evidence remain prerequisites. No additional-family work or universal-support claim is authorized by this choice.

## Q5 — Rights to publish source-derived test material

- Status: open
- Blocks: publishing any anonymized derivative or adapted layout based on a private duty-plan source.
- Context: `temp_first_idea.md` requires public repository material to be releasable under MIT or Apache-2.0, and the user requires anonymized or generated test material. Private access and anonymization do not by themselves establish publication rights. No issuer permission or source-use terms have been reviewed.
- Question: what source-use terms or authorization, if any, permit publication of an anonymized derivative or adapted source layout, and what evidence establishes that permission?
- Options: factual evidence request, not a preference choice. Establish permission before publication; absent evidence, keep source-derived publication unresolved and use independently generated examples where suitable.
- Resolution: pending evidence; not a live user question in this turn.

## Q6 — Crew member exporting their own duty plan review lens

- Status: closed 2026-10-10 (user)
- Blocks: using a confirmed crew-member Persona to review Event Storming and later use-case value and accuracy.
- Context: the confirmed Frame includes crew members and people identified in their PDFs; no project or sibling-topic Personas exist to reuse. A proposed segment can cover both the owner's accuracy and privacy interests without inventing a separate biography for the same person. This segment evaluates fidelity to its own duty plan, unlike the maintainer/release owner who evaluates release conformance; this evaluation capability is the distinguishing criterion.
- Proposed Persona: **Crew member exporting their own duty plan (crew-member--personal-calendar-owner)**.
  - Role: a crew member turning their published duty plan into their personal calendar.
  - Authority boundary: supplies their duty plan and decides whether its calendar representation is usable; does not edit the official schedule or decide release compliance.
  - Evaluation capability: compares activity dates, flight legs, layovers, and report information with their own plan; cannot certify PDF extraction APIs or RFC compliance.
  - Own vocabulary: duty plan, flight leg, report time, deadhead, layover, and activity codes as printed; unexplained codes remain unexplained.
  - Interests and goals: faithful activity representation, clear conversion failures, and no exposure of private source information in public project material.
  - Success criterion: the calendar faithfully represents the required activities and distinguishes operating flights, positioning travel, and non-flight activities without invented dates or omissions; details of those distinctions remain for elaboration.
- Question: accept this proposed crew-member Persona as a review lens, or revise its role, authority, evaluation capability, vocabulary, or success criterion before it is created or used?
- Options:
  1. **Accept this Persona** — create its topic-local Persona file and use it as an advisory accuracy and privacy review lens.
     In practice: a generated departure shown as `(13) 00:20` under local day 12 must not appear a day early in the personal calendar.
     Gain: future flows are checked against what the crew member can actually notice and needs to trust.
     Cost: this lens does not judge release licensing or technical standards; a maintainer lens needs separate confirmation.
     Authorization: one topic-local Persona and its advisory reviews only; no project-level promotion, Gherkin actor approval, calendar mapping rule, or implementation.
  2. **Revise this Persona first** — correct a named part before creating or using it.
     In practice: if the intended converter operator works on other crew members' schedules rather than their own, revise that authority boundary explicitly.
     Gain: avoids reviews based on the wrong user segment or authority.
     Cost: that review lens remains unavailable until corrected and confirmed.
- Resolution: **Accept this Persona** because the user replied “Accepted” to the displayed crew-member Persona proposal. Rejected: **Revise this Persona first** because the user requested no change to its characteristics. Created the topic-local Persona file only; permanent promotion and all other stated authorization limits remain unapproved.

## Q7 — Meanings and calendar treatment of additional activity codes

- Status: closed 2026-10-11 (user, through the focused code decisions below)
- Blocks: a complete activity-classification contract for the privately supplied export family.
- Context: authorized inspection shows `TK`, `U`, `FZ`, and `DTUSA` codes absent from the draft's activity kinds, plus `DH` in the Pos column. Some non-flight rows carry UTC intervals, while others do not. The code spelling and unrelated duty metrics are insufficient to establish their business meanings or intended calendar representation.
- Question: what do the additional observed codes mean in this duty-plan family, and which source facts establish whether each is timed, all-day, or otherwise represented in the personal calendar?
- Options: factual domain-information request, not a preference choice. Establish code meanings and source timing conventions before selecting calendar mappings; do not guess from abbreviations or treat an unrecognized code as a flight.
- Evidence update 2026-10-11: the user defined U as Urlaub, meaning vacation, and explicitly made FREE, OFF, and U all-day activities with source times ignored. Their timing treatment is resolved; FZ, TK, DTUSA, and DH details remain open.
- Further evidence 2026-10-11: the user defined FZ as additional vacation-related days off. Its formal acronym expansion and entitlement provenance are uncertain; its proposed calendar mapping is presented separately for confirmation.
- Mapping update 2026-10-11: the user explicitly accepted FZ as all-day on its duty-plan date with source times ignored, while retaining a distinct source code. FZ timing is no longer open; TK, DTUSA, and DH details remain unresolved.
- TK meaning update 2026-10-11: the user defined TK as office-attendance or professional-development/seminar days, including team information-sharing sessions. TK is a known non-flight activity; its calendar timing is a separate pending choice.
- TK timing update 2026-10-11: the user selected the printed UTC Start/End interval. TK meaning and timed representation are settled; DTUSA and DH details remain unresolved.
- DTUSA meaning update 2026-10-11: the user expanded DT as Duty Time and defined DTUSA as a consulate visit to obtain a USA visa. The meaning is settled; its timing rule and DH details remain open.
- DTUSA timing update 2026-10-11: the user selected its printed UTC Start/End interval. DTUSA meaning and timing are now settled; DH assignment details remain open.
- DH meaning update 2026-10-11: the user defined DH as Dead Head positioning to a departure destination, still duty time in uniform without actual operating work. The role meaning is settled; placement of the visible DH distinction is presented separately.
- Resolution: the user supplied the observed meanings and selected their representation rules: U/vacation and FZ/additional vacation-related days off are all-day with source times ignored, as are FREE/OFF; TK/office-seminar and DTUSA/consulate duties use printed UTC intervals; DH is positioning duty on the underlying timed travel and appears in its title. The focused decisions below preserve their full options and rejected alternatives. Generic date resolution, other code variants, metadata, and calendar-identity policy remain separate; no unknown activity is licensed to default to a flight or layover.

## Q8 — Maintainer responsible for a public release review lens

- Status: closed 2026-10-10 (user)
- Blocks: reviewing Event Storming and later use cases against the confirmed maintainer/release-owner interests.
- Context: the confirmed Frame includes a maintainer/release owner, and the Non-Negotiable Constraints in `temp_first_idea.md` require a single Windows executable, standards-compliant output, permissively licensed dependencies, and releasable public material without private source data. The confirmed crew-member Persona judges fidelity to their own schedule, not technical or legal release conformance. No project or sibling-topic Persona exists to reuse; the distinguishing authority boundary is deciding whether release evidence permits publication of a release.
- Proposed Persona: **Maintainer responsible for a public release (project-maintainer--release-owner)**.
  - Role: maintains the converter and decides whether it is ready for a public release.
  - Authority boundary: accepts or rejects a release against the agreed constraints; does not change a crew member's official duty plan, relax confirmed constraints, or invent meanings for unexplained activity codes.
  - Evaluation capability: evaluates documented support boundaries, test and calendar-validation evidence, dependency licenses, Windows distribution checks, and the absence of private source material from public artifacts. Domain experts remain necessary for unexplained roster semantics.
  - Own vocabulary: supported export, generated fixture, golden calendar, validation evidence, dependency license, release artifact, and private-data exposure.
  - Interests and goals: a maintainable, verifiable, legally distributable converter whose public claims match its evidence.
  - Success criterion: a release satisfies the agreed distribution, calendar-compliance, licensing, and privacy requirements, with claimed duty-plan support backed by evidence rather than one passing example.
- Question: accept this maintainer Persona as an advisory release-conformance review lens, or revise its role, authority, evaluation capability, vocabulary, or success criterion before it is created or used?
- Options:
  1. **Accept this Persona** — create its topic-local record and use it for advisory reviews of release-relevant domain decisions and guarantees.
     In practice: a golden-calendar match alone does not justify the claim “supports every valid Jeppesen duty-plan PDF”; documented evidence must support the agreed scope.
     Gain: release promises and inherited constraints receive a review lens distinct from personal-calendar usability.
     Cost: later elaboration must make release-relevant claims testable; this Persona does not replace domain evidence for unfamiliar codes.
     Authorization: one topic-local Persona and its advisory reviews only; no project-level promotion, technology substitution, implementation, or change start.
  2. **Revise this Persona first** — correct a named characteristic before creating or using it.
     In practice: if a separate release approver—not the maintainer—has publication authority, specify that boundary instead.
     Gain: reviews match the actual release authority and evaluation capability.
     Cost: the release review lens waits for the correction and confirmation.
- Resolution: **Accept this Persona** because the user replied “Accepted” to the displayed maintainer Persona proposal. Rejected: **Revise this Persona first** because the user requested no change to its characteristics. Created the topic-local record only; permanent promotion and all other stated authorization limits remain unapproved.

## Q9 — Whole-plan calendar consistency boundary

- Status: closed 2026-10-10 (user)
- Blocks: drawing an Event Storming canvas with a declared aggregate rather than mislabeling the input PDF, output file, or conversion-request record as an aggregate.
- Context: the confirmed Frame already requires complete conversion or explicit failure, with no silently omitted activities. The permanent glossary has no declared aggregate, and the term/synonym inventory found no prior boundary definition. The proposed Duty-plan calendar is a calendar composition governing the complete activity representation of one supplied plan; the source PDF and output `.ics` remain records outside that root.
- Proposed term: a **Duty-plan calendar** is a calendar composition that represents the required activities of one supplied duty plan and governs whether their complete representation is ready to export.
- Contrast: **Duty-plan calendar, not calendar export** — the former governs consistency and completion; the latter is its resulting file record.
- Invariant: a successful export is available only after every required source activity is accounted for and its required calendar dates/fields are resolved. This restates the confirmed completeness requirement; it is not reopened for preference.
- Command and candidate events: the crew member issues **Export duty plan**; the boundary can emit **Duty plan supplied**, **Duty-plan calendar completed**, **Calendar export published**, or **Duty-plan conversion rejected**. Exact causal chains and policies are developed after boundary confirmation.
- Question: accept Duty-plan calendar as the whole-plan consistency boundary used for Event Storming, or revise its name or extent while preserving the already confirmed complete-export-or-explicit-failure requirement?
- Options:
  1. **Accept this boundary** — model all required activity representations from one supplied duty plan under the Duty-plan calendar root; treat source PDF, calendar file, and diagnostic as records, not aggregates.
     In practice: if one flight continues onto the next page and its arrival cannot be resolved, other parsed legs do not become an apparently complete successful calendar.
     Gain: the canvas has an explicit domain boundary matching the completeness guarantee, without invented server or request-record aggregates.
     Cost: the model must retain and validate the whole required activity representation before declaring export success; large-plan limits will need evidence rather than assumptions.
     Authorization: one topic-local domain term and conceptual boundary, then a draft Event Storming canvas; no database, persistent workflow, code structure, release claim, or implementation is approved.
  2. **Revise the boundary first** — name the intended root or correct its extent before drawing the canvas, retaining the complete-export-or-explicit-failure rule.
     In practice: if “Roster calendar” is the established noun for this same whole-plan composition, supply that noun and its distinction from the generated file rather than letting the canvas invent one.
     Gain: the model follows the intended domain vocabulary and ownership.
     Cost: the canvas waits for the revised boundary and a collision check.
- Resolution: **Accept this boundary** because the user replied “Accepted” to the displayed Duty-plan calendar boundary. Rejected: **Revise the boundary first** because the user requested no name or extent revision. Confirmed the term topic-locally and used it for a draft Event Storming canvas only; no implementation or persistence mechanism is approved.

## Q10 — Existing outputs and failure to publish a calendar

- Status: closed 2026-10-11 (user, with an additional naming alternative)
- Blocks: an unconditional filesystem-failure guarantee, including how users distinguish a prior calendar from a failed new conversion.
- Context: `temp_first_idea.md` writes a fixed output path and reports parse failures, but does not establish what happens to a pre-existing `.ics`, a stale diagnostic, or an interrupted/unwritable output. The confirmed complete-export-or-explicit-failure rule forbids success with unresolved required activities; it does not by itself specify replacement and recovery behavior. Event Storming therefore distinguishes calendar-completion success from publication success and qualifies output-failure recovery.
- Question: when a calendar already exists beside the supplied PDF, should conversion replace it only on complete successful publication, or refuse to overwrite it until the crew member removes or relocates it?
- Options:
  1. **Replace only on complete successful publication** — update the fixed output path only once the new complete calendar can be published; failure must not present a partial replacement as success.
     In practice: a prior generated calendar remains the prior version if a later conversion cannot finish; the failure outcome identifies that it was not updated.
     Gain: repeated conversion has the draft's simple same-path workflow without sacrificing a known-good output.
     Cost: a successful replacement discards the prior contents at that path, including manual edits; no automatic backup is included. Establish and verify preservation/replacement behavior on Windows, interruptions, and write errors; diagnostic availability still depends on writable storage. Recovery of an already successfully replaced version is not promised.
  2. **Refuse to overwrite existing output** — leave an existing calendar untouched and report the name collision before publication.
     In practice: the user removes or relocates the prior generated calendar before producing another file at the fixed path.
     Gain: a simpler no-replacement contract with no silent loss of an existing file.
     Cost: extra manual work on repeat conversions; an explicit failure still needs a visible outcome if its diagnostic cannot be written.
  3. **Preserve existing calendars and enumerate the new output** — user-supplied alternative: do not replace a calendar, but publish the new complete export under an unused numbered name when its normal name is taken.
     In practice: using a hyphenated naming convention for the draft examples, preserve `roster.ics` and publish `roster-1.ics`; if both exist, use `roster-2.ics`.
     Gain: no existing calendar is lost and repeat conversion does not require manual deletion or relocation.
     Cost: calendar versions accumulate; the reported output path must identify the newly created file. This does not promise how importing several versions behaves.
     Authorization: unused calendar-output naming only; not overwriting/deleting calendars, error-report replacement, automatic version cleanup, or reimport reconciliation.
- Later-change cost: changing this behavior after release changes users' expectations about which existing file is preserved or replaced; recovery from already overwritten output must not be assumed.
- Resolution: **Preserve existing calendars and enumerate the new output** because the user said to refuse overwriting and add an enumerated number behind the name. Rejected: **Replace only on complete successful publication**, because existing calendars must not be overwritten; **Refuse to overwrite existing output and stop at collision**, because the user added automatic numbered-name publication instead of manual removal. The user settled preservation and numbering; hyphen punctuation in the concrete examples is a draft notation, not a separately requested preference. Diagnostic naming and publication-failure reporting remain separate under the failure-report question (Q22) and publication-failure recovery question (Q24). No implementation, power-loss durability, or successful diagnostic persistence is claimed.

## Q11 — Meaning of the U activity code

- Status: closed 2026-10-11 (user)
- Blocks: establishing the activity vocabulary needed to represent U rows in the selected first-release duty-plan family.
- Context: the authorized private-PDF inspection found dated U activity rows with blank UTC Start and End cells; nearby duration values belong to accounting columns, not event timestamps. U is absent from the first draft's named activity kinds. The first-release supported-export decision (Q4), which selects the supplied Released Roster detail-table family, makes these rows relevant; spelling alone does not establish the business meaning or calendar mapping.
- Question: what does U mean in this duty-plan family, or should its meaning remain parked as unknown?
- Options:
  1. **Provide the established meaning** — give the code's meaning from reliable crew/domain knowledge so it can be recorded without guessing.
     In practice: a generated `2099-01-12 | U` row with no UTC times becomes a known activity whose calendar representation can be settled next; this answer alone does not invent event times or approve all-day behavior.
     Gain: removes the semantic uncertainty for this observed activity.
     Cost: needs a reliable definition rather than an interpretation inferred from its letter.
  2. **Park the meaning as unknown** — retain U as an unresolved activity code and do not infer its meaning or dates.
     In practice: a plan containing a required U activity cannot be presented as a successful complete conversion by guessing flight or layover; the unresolved activity receives explicit failure treatment until its required mapping is established.
     Gain: avoids incorrect activities or invented timing.
     Cost: the first-release guarantee for plans containing U remains incomplete until the meaning and mapping are resolved.
- Resolution: **Provide the established meaning** because the user stated that U stands for Urlaub and means vacation. The user also explicitly supplied the mapping: FREE, OFF, and U are all-day events; source times are ignored. Rejected: **Park the meaning as unknown** because the user provided the meaning and timing rule. No separate all-day confirmation is needed; no display-label, metadata, or other-code mapping is inferred from this answer.

## Q12 — Meaning of the FZ activity code

- Status: closed 2026-10-11 (user)
- Blocks: establishing FZ activity vocabulary before choosing its calendar representation in the selected first-release duty-plan family.
- Context: authorized PDF inspection found dated FZ activity rows with blank UTC Start and End cells; visible zeros belong to accounting columns. The user has confirmed all-day treatment for FREE, OFF, and U, but has not defined FZ or included it in that rule. Similar row shape is not evidence of the same meaning or timing behavior.
- Question: what does FZ mean in this duty-plan family, or should its meaning remain parked as unknown?
- Options:
  1. **Provide the established meaning** — define FZ from reliable crew/domain knowledge, without deriving its meaning from the abbreviation or from the U mapping.
     In practice: a generated `2099-01-13 | FZ` row becomes a named activity whose calendar mapping can be established; no event times are invented.
     Gain: removes the next observed activity's semantic uncertainty.
     Cost: requires a reliable definition; a meaning alone does not establish timing unless the user explicitly supplies that rule too.
  2. **Park the meaning as unknown** — retain FZ as an unresolved code rather than guess.
     In practice: a required FZ activity cannot be silently omitted or automatically classified like U in a successful complete conversion.
     Gain: avoids a false activity or calendar classification.
     Cost: the first-release conversion guarantee for plans containing FZ remains incomplete until the meaning and required mapping are resolved.
- Resolution: **Provide the established meaning** because the user defined FZ as additional days off that are part of vacation. Rejected: **Park the meaning as unknown** because the practical meaning was supplied. The suggested expansion “Frei zusätzlich” and any company-agreement basis remain unverified; the user described a four-day annual entitlement from age 50 as background, not a conversion rule. This answer establishes meaning, not a separately confirmed calendar mapping or eligibility validator.

## Q13 — Formal FZ expansion and entitlement provenance

- Status: open
- Blocks: no calendar-conversion work; only a verified claim about the acronym expansion or source of the entitlement.
- Context: the user knows FZ as additional vacation-related days off, describes four days per year from age 50, and suggests “Frei zusätzlich” as a possible expansion. The formal expansion and source agreement are unknown. The converter reads assigned activities; it does not determine whether a person is legally entitled to them.
- Question: what authoritative source, if any, establishes the formal expansion of FZ and its entitlement provenance?
- Options: parked factual evidence request, not a live preference question. Keep the expansion and provenance unverified rather than researching labor rules outside this conversion scope or encoding an age/annual-limit check.
- Resolution: parked as non-blocking; no research or eligibility-enforcement work is authorized.

## Q14 — Calendar representation of additional vacation days

- Status: closed 2026-10-11 (user)
- Blocks: confirming the FZ representation rule before including it as unconditional in the use-case contract.
- Context: the user defined FZ as additional vacation-related days off through the FZ-meaning decision (Q12). The vacation/all-day decision (Q11) explicitly makes FREE, OFF, and U all-day with source times ignored. Applying that same representation to FZ is a proposed extension; the uncertain acronym or agreement source does not affect this output rule.
- Proposed rule: FZ uses all-day calendar representation on its duty-plan date, with source times ignored, while its source code remains distinguishable from U, FREE, and OFF. Exact SUMMARY wording and metadata are not decided here.
- Question: apply the confirmed all-day/source-times-ignored treatment to FZ as well, or revise its calendar representation before recording the rule?
- Options:
  1. **Apply the same all-day treatment** — include FZ in the code-driven all-day group and ignore source times for event timing.
     In practice: a generated FZ row dated 2099-01-13 appears as an all-day additional-vacation activity even if the source row contains 09:00 and 17:00.
     Gain: a clear, consistent representation of the additional day off without invented clock times.
     Cost: any printed source times cannot change the event's all-day timing; display-label and metadata choices remain separate.
     Authorization: this FZ calendar rule only, not an age threshold, annual quota, acronym expansion, or labor-agreement interpretation.
  2. **Revise the representation first** — supply the intended observable FZ representation before recording the proposed rule.
     In practice: if an FZ row is intended to represent a whole absence block rather than its dated day, specify the block boundary so the calendar does not invent its extent.
     Gain: avoids committing the wrong event boundary or interpretation.
     Cost: FZ mapping remains conditional until the corrected rule is confirmed.
- Resolution: **Apply the same all-day treatment** because the user said “ok, apply this rule.” Rejected: **Revise the representation first** because the user accepted the displayed mapping without a boundary correction. FZ is all-day on its duty-plan date with source times ignored, and remains distinguishable from U/FREE/OFF. Display wording, metadata, formal expansion, and entitlement provenance remain separate; no repeated mapping question is needed.

## Q15 — Meaning of the TK activity code

- Status: closed 2026-10-11 (user)
- Blocks: establishing the meaning of a timed non-flight activity in the selected duty-plan family before fixing its calendar classification.
- Context: private-PDF inspection shows a TK activity with a station and UTC Start/End values, but no ordinary flight route. TK is absent from the initial draft's named activity kinds. The code's meaning cannot be established merely from its time interval, and the confirmed FREE/OFF/U/FZ all-day rule does not cover it.
- Question: what does TK mean in this duty-plan family, or should its meaning remain parked as unknown?
- Options:
  1. **Provide the established meaning** — define TK from reliable crew/domain knowledge so its activity class can be named without guessing.
     In practice: a generated `2099-01-15 | TK | ABC | 08:00–12:00 UTC` row becomes a known activity whose calendar classification can be established rather than being guessed to be a flight or layover.
     Gain: removes the next observed activity's semantic uncertainty.
     Cost: needs a reliable definition; timing and display rules are not silently approved unless the user explicitly supplies them too.
  2. **Park the meaning as unknown** — retain TK as unresolved and do not infer its kind from the abbreviation or station.
     In practice: a required TK row causes explicit unresolved-activity handling instead of disappearing from a successful complete export.
     Gain: avoids a false flight, layover, or absence classification.
     Cost: conversion rules for plans containing TK remain incomplete until its meaning and required mapping are resolved.
- Resolution: **Provide the established meaning** because the user defined TK as days requiring office attendance or participation in a professional-development seminar, for example a team information-sharing session called by the team leader. Rejected: **Park the meaning as unknown** because the practical meaning was supplied. No formal acronym expansion is invented, and timed versus all-day calendar treatment is not silently selected.

## Q16 — Calendar timing of office or seminar duty

- Status: closed 2026-10-11 (user)
- Blocks: a confirmed TK timing rule for the use-case contract.
- Context: the TK-meaning decision (Q15) establishes office/seminar duty as a known non-flight activity. The inspected source prints a UTC Start/End interval for TK. The user has explicitly ignored source times for FREE/OFF/U/FZ, but has not applied that rule to TK. This choice determines whether a calendar preserves TK's printed attendance hours or presents a whole-day marker.
- Question: should TK preserve its printed UTC Start/End interval as a timed event, or appear as an all-day activity with source times ignored?
- Options:
  1. **Use the printed UTC interval** — represent TK as timed office/seminar duty, using its resolved Start (UTC) and End (UTC) endpoints rather than flight-route assumptions.
     In practice: a generated TK row with 08:00–12:00 UTC appears as a four-hour appointment, not an all-day absence or a flight.
     Gain: preserves the supplied attendance hours and leaves the rest of the day outside that event.
     Cost: valid endpoints are needed; unresolved dates/times prevent successful conversion rather than being replaced by an invented all-day fallback.
     Authorization: TK timing only; no SUMMARY label, metadata inheritance, or unresolved UTC month/year policy is approved.
  2. **Use an all-day marker** — represent TK on its duty-plan date as all-day and ignore source times for its event timing.
     In practice: the same generated 08:00–12:00 UTC row appears as a whole-day TK activity without clock times.
     Gain: a coarse day-level indication of office/seminar attendance.
     Cost: discards printed scheduling precision and represents the event beyond the supplied attendance interval.
     Authorization: TK timing only, not a general rule for every ground duty or the other unresolved codes.
- Resolution: **Use the printed UTC interval** because the user explicitly selected it. Rejected: **Use an all-day marker** because the selected rule preserves the printed attendance interval rather than discarding its scheduling precision. TK is a timed non-flight office/seminar activity; unresolved endpoints cause explicit failure, not an all-day fallback. Display labels, metadata, and unresolved date-boundary policies remain separate, and the timing choice is not reopened.

## Q17 — Meaning of the DTUSA activity code

- Status: closed 2026-10-11 (user)
- Blocks: establishing the next observed non-flight activity's meaning before its calendar classification is fixed.
- Context: private-PDF inspection shows DTUSA with a station and UTC Start/End values but no ordinary flight route. Its meaning is absent from the initial draft and has not been supplied by the user. Neither its spelling nor the settled TK timing rule establishes what it represents.
- Question: what does DTUSA mean in this duty-plan family, or should its meaning remain parked as unknown?
- Options:
  1. **Provide the established meaning** — define DTUSA from reliable crew/domain knowledge rather than inferring it from the letters.
     In practice: a generated `DTUSA | ABC | 09:00–15:00 UTC` row becomes a known activity whose calendar mapping can be settled, instead of being guessed to be a flight or layover.
     Gain: removes the next observed activity's semantic uncertainty.
     Cost: requires a reliable definition; timing or display rules are not silently inferred unless the user also supplies them.
  2. **Park the meaning as unknown** — retain DTUSA as unresolved rather than guess its activity class.
     In practice: a required DTUSA row cannot be silently omitted from an apparently successful complete export.
     Gain: avoids false classification or invented activity semantics.
     Cost: conversion rules for plans containing DTUSA remain incomplete until its meaning and required representation are established.
- Resolution: **Provide the established meaning** because the user stated that DT means Duty Time and DTUSA denotes a consulate visit to obtain a visa for the USA. Rejected: **Park the meaning as unknown** because the definition was supplied. This establishes a known non-flight visa appointment, not automatic handling of every other DT-prefixed code or a confirmed timing/display rule.

## Q18 — Calendar timing of consulate visa duty

- Status: closed 2026-10-11 (user)
- Blocks: confirming DTUSA's event timing before its use-case rule becomes unconditional.
- Context: the DTUSA-meaning decision (Q17) establishes consulate visa duty as a known non-flight activity, and the source prints UTC Start/End values. TK already preserves its printed UTC interval, whereas FREE/OFF/U/FZ explicitly ignore source times. Neither rule has yet been extended to DTUSA; “Duty Time” is a code expansion, not a duration calculation from accounting columns.
- Question: should DTUSA use its printed UTC Start/End interval as a timed event, or appear as an all-day activity with source times ignored?
- Options:
  1. **Use the printed UTC interval** — represent DTUSA as timed consulate visa duty with independently resolved UTC endpoints.
     In practice: a generated 09:00–15:00 UTC row becomes a six-hour visa-appointment duty, not a flight or whole-day absence.
     Gain: preserves the scheduled duty interval and leaves the rest of the day outside that event.
     Cost: valid endpoints are required; unresolved dates/times cause explicit failure rather than an invented all-day fallback.
     Authorization: DTUSA timing only; no other DT-prefixed mapping, display label, accounting-duration rule, or unresolved date-boundary policy is approved.
  2. **Use an all-day marker** — represent DTUSA on its duty-plan date as all-day and ignore printed times for event timing.
     In practice: the same generated 09:00–15:00 UTC row appears as a whole-day DTUSA activity without clock times.
     Gain: a coarse day-level reminder of the consulate duty.
     Cost: discards the printed attendance interval and represents the event beyond those hours.
     Authorization: DTUSA timing only, not an all-day rule for every Duty Time activity.
- Resolution: **Use the printed UTC interval** because the user explicitly selected it. Rejected: **Use an all-day marker** because the selected rule preserves the scheduled consulate-duty interval instead of discarding those hours. DTUSA is timed non-flight duty with required resolvable endpoints and no invented all-day fallback. Other DT-prefixed codes, labels, accounting-duration treatment, and unresolved date-boundary policies remain separate.

## Q19 — Meaning of the DH position marker

- Status: closed 2026-10-11 (user)
- Blocks: confirming how the crew assignment marked DH differs from the other positions before representing that distinction in the calendar.
- Context: private-PDF inspection shows DH in the Pos column, not the Activity column, alongside travel rows with a designator, departure/arrival stations, and a positive UTC interval. Some such rows lack aircraft information and show zero Blocktime, so neither field can establish that the timed activity disappears. The precise assignment meaning has not been supplied by the user.
- Question: what does DH in the position column mean for the crew member in this duty-plan family, or should its meaning remain parked as unknown?
- Options:
  1. **Provide the established meaning** — define the DH crew assignment from reliable domain knowledge before choosing its calendar distinction.
     In practice: a generated `Pos DH | Activity 007 | ABC–DEF | 06:00–07:00 UTC` row can be represented with the correct assignment distinction even if aircraft information is blank and Blocktime is zero.
     Gain: establishes the meaning of an observed role marker instead of confusing it with an activity code or duration.
     Cost: requires a reliable definition; label/description placement and any additional transport rules remain separate unless explicitly supplied.
  2. **Park the meaning as unknown** — retain the unresolved role marker and do not treat it as interchangeable with another crew assignment.
     In practice: the generated row's required assignment distinction is not silently discarded or inferred from zero Blocktime.
     Gain: avoids a false role interpretation.
     Cost: the first-release assignment-distinction contract remains incomplete for rows marked DH.
- Resolution: **Provide the established meaning** because the user defined DH as Dead Head to a departure destination, still duty time in uniform without the usual operating work. Rejected: **Park the meaning as unknown** because the definition was supplied. DH is a crew-assignment marker on positioning travel, not a separate absence code; it does not erase the underlying travel interval or make zero Blocktime evidence of no event. Calendar-title versus description placement remains a separate choice.

## Q20 — Visibility of the deadhead assignment

- Status: closed 2026-10-11 (user)
- Blocks: deciding where the required deadhead-versus-operating distinction appears in the exported calendar.
- Context: the DH-meaning decision (Q19) establishes positioning travel that remains duty time. The confirmed crew-member Persona requires operating travel and positioning travel to remain distinguishable. The draft's flight title contains only the designator and route; the Pos column is absent from its proposed model/output fields. Both alternatives below preserve the travel's printed UTC endpoints and the DH distinction; neither derives duration from Blocktime or adds a second event for the same leg.
- Question: should DH appear in the event title, or only in the event description while retaining the ordinary designator-and-route title?
- Options:
  1. **Put DH in the title** — prefix a deadhead travel title with the printed assignment marker, retaining the designator and route.
     In practice: a generated travel event is titled `DH 007 ABC-DEF`; a non-DH event remains `007 ABC-DEF`.
     Gain: the crew member can distinguish positioning travel directly in a calendar's compact day/week view without opening the event.
     Cost: adds a marker to the draft's shorter flight-title convention.
     Authorization: DH display placement only; no new event, UID policy, import-update guarantee, or duty-pay calculation is approved.
  2. **Put DH only in the description** — preserve the ordinary title and include the position meaning in event details.
     In practice: the generated title remains `007 ABC-DEF`, and the description contains `Pos DH (deadhead duty)`.
     Gain: uniform concise designator-and-route titles, with the role still retained in details.
     Cost: compact calendar views do not distinguish operating and deadhead assignments; the user must open the event details.
     Authorization: DH display placement only, not permission to discard the marker or treat the activity as off duty.
- Resolution: **Put DH in the title** because the user explicitly selected it. Rejected: **Put DH only in the description** because the selected rule makes the assignment visible in compact calendar views. A generated title is `DH 007 ABC-DEF`; the underlying travel remains one timed event. No UID, import-update, or duty-pay rule is changed by this title choice.

## Q21 — Unqualified travel times that appear to cross midnight

- Status: closed 2026-10-11 (user)
- Blocks: confirming whether the draft's automatic next-day repair is a supported interpretation or an unsafe guess when source date evidence is incomplete.
- Context: PDF Parsing Algorithm step 6 in `temp_first_idea.md` proposes adding one day when an end clock is earlier than its start. The private examples show explicit parenthesized UTC day qualifiers, which must not be overridden; the Frame left date rules open. The unresolved case here is narrower: both travel endpoints are unqualified UTC times in the same local-date block, with End earlier than Start and no separate arrival-date block. No local evidence has established a general rollover convention for that case.
- Question: for an unqualified same-block travel interval whose End clock precedes Start, reject the unresolved dates or interpret End as the following UTC day?
- Options:
  1. **Reject the unresolved dates** — require source date evidence to establish a positive timed interval rather than repair the clock pair automatically.
     In practice: a generated local-day-12 row with Start 23:10 and End 01:05, neither carrying a UTC day qualifier, produces explicit date-resolution failure and no successful calendar.
     Gain: prevents a plausible-looking overnight event from concealing missing or contradictory date information.
     Cost: a legitimate unqualified overnight layout is unsupported until its convention is evidenced and deliberately added.
     Authorization: this narrow travel-date ambiguity rule only; explicit qualifier interpretation and month/year mapping remain separate.
  2. **Interpret End as the following UTC day** — document an automatic one-day rollover for this unqualified same-block travel case only.
     In practice: the same generated row becomes day 12 23:10 to day 13 01:05 UTC, lasting 1:55; it is not derived from Blocktime.
     Gain: converts overnight clock pairs without requiring an explicit arrival-day marker.
     Cost: assumes an overnight convention and can conceal a bad source time; that convention needs real family evidence before being claimed as validated support. This does not permit overriding explicit UTC day qualifiers or repairing zero-length intervals.
     Authorization: this narrow rollover interpretation only, not arbitrary duration-based reconstruction or a fallback for the already specified ground duties.
- Resolution: **Interpret End as the following UTC day** because the user explicitly selected it. Rejected: **Reject the unresolved dates** because the user selected automatic one-day rollover for this narrow unqualified same-block travel case. Apply only when both UTC endpoints lack day qualifiers, share one local-date block, have no separate arrival-date block, and End clock is earlier than Start. Explicit day markers remain authoritative; zero-length intervals, ground-duty fallbacks, Blocktime-derived dates, and arbitrary repairs are not authorized. The desired interpretation is confirmed; real-family evidence for this particular layout variation is not manufactured by the choice.

## Q22 — Preservation and names of failure reports

- Status: closed 2026-10-11 (user)
- Blocks: an unconditional naming/preservation rule for repeated persisted diagnostics, separate from calendar naming.
- Context: the calendar-output decision (Q10) preserves existing calendars and selects unused numbered names for new exports. The first draft uses a fixed `<stem>.parse-error.txt` path for failures. The user's calendar choice did not explicitly authorize replacing error reports or extend numbering to them; old reports could otherwise be mistaken for the current outcome.
- Question: should repeated failure reports replace the fixed diagnostic path with the latest failure, or preserve older reports under numbered diagnostic names?
- Options:
  1. **Keep only the latest report at the fixed path** — replace the prior diagnostic when another failure occurs.
     In practice: a new failure changes `roster.parse-error.txt` to the new reason while all existing calendars remain untouched.
     Gain: one predictable place to find the latest failure.
     Cost: prior report contents, including manual annotations, are lost; stale-report handling after a successful run still needs an explicit rule.
  2. **Preserve reports with numbered names** — allocate an unused diagnostic name rather than replace a prior report.
     In practice: preserve `roster.parse-error.txt` and create a separately numbered report for the next failure.
     Gain: diagnostic history is retained without overwriting files.
     Cost: reports accumulate and must identify their run/source so the user can find the current outcome.
- Resolution: **Preserve reports with numbered names** because the user explicitly selected it. Rejected: **Keep only the latest report at the fixed path** because prior report contents must not be replaced. The user also approved the source PDF filename as report content; the draft includes it to identify the attempted input. Use unused report names, report the actual newly written path, and do not remove old reports or confuse an old file with a new successful diagnostic write. Full source paths or private filenames in public topic/fixture artifacts are not authorized by this local-report choice.

## Q23 — Trip identifiers in travel-event descriptions

- Status: closed 2026-10-11 (user)
- Blocks: deciding how the source's sparsely printed Trip ID becomes event metadata.
- Context: the inspected detail tables print a trip identifier at a grouping boundary, while subsequent travel legs in that same trip can have blank Trip ID cells. The first draft's synthetic table repeats identifiers on several rows and therefore does not establish real-source inheritance. Trip metadata is descriptive, not a basis for UTC time calculation; the rule must not propagate report times or station offsets simply because it propagates a trip identifier.
- Question: should a travel event inherit the identifier of its unambiguously associated roster trip, or include Trip ID only when printed on that leg or one of its merged fragments?
- Options:
  1. **Carry the identifier within its roster trip** — include the known trip identifier on each travel leg unambiguously belonging to that trip, even when its row cell is blank.
     In practice: a generated first leg prints Trip T42, and the next leg in the same trip has a blank cell; both event descriptions include `Trip T42`.
     Gain: every travel event retains its source trip context.
     Cost: establish trip boundaries and prevent an identifier leaking into unrelated duties or a later trip; ambiguous association is not permission to guess. This does not propagate Report (LT) or Timezone Difference.
  2. **Use only identifiers printed on the leg** — preserve a Trip ID from the leg's own source row or merged fragments, without broader inheritance.
     In practice: the generated first event includes `Trip T42`, while the next leg with a blank cell has no Trip field in its description.
     Gain: direct row/fragment traceability without a trip-inheritance convention.
     Cost: many real-source travel events lack their available grouping context in the calendar.
- Resolution: **Carry the identifier within its roster trip** because the user explicitly selected it. Rejected: **Use only identifiers printed on the leg** because the selected rule retains trip context on subsequent travel legs with blank cells. Carry only a known identifier within an unambiguous source grouping, including across page/date continuations; do not leak it into unrelated duties or a later trip. Report-time/offset propagation and layover/non-flight inheritance were not authorized by this choice.

## Q24 — Persisted diagnostics for publication failures

- Status: closed 2026-10-11 (user)
- Blocks: confirming the recovery path after calendar publication fails, independent of the already required visible failure outcome.
- Context: the first draft saves parse diagnostics but panics on calendar-write failure. The reviewed Event Storming makes publication failure visible regardless of diagnostic persistence. Whether a publication failure also attempts a sidecar diagnostic remains open; a filesystem that rejects calendar output may also reject the diagnostic.
- Question: should a publication failure attempt a persisted diagnostic when possible, or report only through the failure status/message while reserving sidecars for interpretation failures?
- Options:
  1. **Attempt a diagnostic for every conversion failure** — try to persist publication-failure context too, without depending on that attempt for failure visibility.
     In practice: a denied calendar publication yields failure status/message and, if writable, a diagnostic explaining why no new calendar was produced.
     Gain: failure information can survive the drag-and-drop process exiting.
     Cost: handle diagnostic failure explicitly; no sidecar can be guaranteed when storage is unwritable. Use the already confirmed no-overwrite numbered-report rule, identifying the source PDF filename without replacing old reports.
  2. **Persist only interpretation-failure diagnostics** — leave publication failures to the explicit failure status/message.
     In practice: the same failed publication emits an error and non-success result but does not attempt an additional report file.
     Gain: fewer writes when publication/storage is already failing.
     Cost: a transient drag-and-drop console may disappear without a retained publication-failure explanation.
- Resolution: **Attempt a diagnostic for every conversion failure** because the user explicitly selected it. Rejected: **Persist only interpretation-failure diagnostics** because publication failures also need an attempted retained explanation. Each failed conversion attempts a new numbered report identifying the source PDF filename; if that attempt fails, retain the primary failure in the visible message/non-success outcome and state that no new report was saved. Existing calendars and reports remain untouched. This is a reporting requirement, not a guarantee that unwritable storage can persist a report.

## Q25 — Calendar activity identity across revised duty plans

- Status: closed 2026-10-11 (user)
- Blocks: specifying persistent calendar identifiers beyond the first draft's insufficient date/code/route recipe.
- Context: RFC 5545 section 3.8.4.7 requires persistent globally unique UIDs. The first draft's date/code/route recipe can collide across crew members and repeated same-code activities; numbering output files does not resolve component identity. No automatic import or synchronization is in scope, and no calendar client's update/deduplication behavior has been established. The choice defines which source activities the exporter regards as corresponding components across revised PDFs, not whether the tool updates an imported calendar.
- Question: should corresponding roster activities retain their identifiers across revised PDFs where their correspondence is unambiguous, or should different source-PDF snapshots have separate component identities?
- Options:
  1. **Retain corresponding activity identities** — preserve an activity's identifier across a revision when a stable source identity can establish it is the same duty; still scope identities globally and distinguish repeated activities.
     In practice: generated flight 007 on ABC–DEF for 12 January changes departure from 09:00 to 10:00 UTC, but its identifier remains the same if its correspondence is unambiguous.
     Gain: preserves revision correlation and supports clients that use UID to match changed events.
     Cost: define stable ownership/activity keys and ambiguous-repeat handling; identical identifiers alone do not guarantee any client's import updates or duplicate suppression.
  2. **Separate source-snapshot identities** — scope component identifiers to the source PDF snapshot, while keeping repeated conversion of the identical snapshot deterministic and its components distinct.
     In practice: that revised PDF gives the generated flight a different identifier; both immutable output files remain valid snapshots, and importing both may retain both events.
     Gain: a clear snapshot boundary without promising correspondence between revisions.
     Cost: unchanged activities in revised snapshots can receive new identifiers and may accumulate in an imported calendar; it departs from the initial recipe's apparent cross-revision stability.
- Later-change cost: UIDs are stored by calendar clients. Changing the identity policy after files have been imported may require user-controlled cleanup/reimport; rewriting filenames or reinstalling the converter is not a demonstrated migration or rollback.
- Resolution: **Separate source-snapshot identities** because the user explicitly selected it. Rejected: **Retain corresponding activity identities** because revision correspondence is not the selected identity model. Different source PDF snapshots have separate component identities, including unchanged activities in a revised snapshot; repeated conversion of an identical snapshot remains deterministic and distinct activities must still have distinct globally unique identifiers. Output filename counters and title changes are not identity inputs. No cross-revision matching, import-update, duplicate-suppression, or automatic cleanup guarantee is approved.

## Q26 — Standby activity rows not represented in the private corpus

- Status: closed 2026-10-11 (user domain information; SBX window clarification remains separate)
- Blocks: an evidenced unconditional SB/SBX timing rule; the inspected corpus has only overview/statistical occurrences, not a dated standby duty row.
- Context: the first draft proposes all-day SB/SBX when Start is 00:00, but its synthetic table has no standby row. Inspection established that apparent zeros on other all-day activities can be accounting values rather than UTC times. The selected first-release family has no supplied standby-row evidence, so neither the presence nor absence of a real UTC window is established for standby.
- Question: what is the established calendar timing rule for dated SB/SBX activities in this family, or should their timing remain unverified until a real standby example is available?
- Options:
  1. **Provide the established standby timing rule** — supply authoritative crew/domain knowledge or a dated source example establishing how an assigned standby activity is represented.
     In practice: a generated dated SB row can receive its known all-day or availability-window representation once that rule is supplied; the annual standby counter still creates no event.
     Gain: enables a defined standby rule rather than relying on the first draft's unverified zero-time assumption.
     Cost: requires a reliable rule or example; accounting zeros and overview legends do not establish availability times.
  2. **Keep standby timing unverified** — leave this source variation unsupported until its actual dated-row semantics are established.
     In practice: an input containing an unresolved required SB/SBX activity receives an explicit diagnostic instead of a guessed event or silent omission; the supplied corpus's statistics alone still create no standby events.
     Gain: avoids claiming support from nonexistent evidence.
     Cost: first-release support for plans containing dated standby activities remains incomplete.
- Resolution: **Provide the established standby timing rule** because the user defined SB as standby/Stby with start and end times, often assigned on consecutive days interrupted by FREE, and added RE as reserve with start/end times. Rejected: **Keep all standby timing unverified**, because authoritative domain information is now supplied for SB and RE. The user defined SBX as single-day standby or an assignment replacing a cancelled flight; whether SBX always supplies a window is clarified separately under its window question (Q29), not inferred from the words single-day. The SB/RE activation-to-reporting delay distinction is background, not a converter workflow or timing calculation.

## Q27 — Provenance of the calendar revision timestamp

- Status: closed 2026-10-11 (user, with a preferred-source/fallback rule)
- Blocks: documenting the DTSTAMP policy consistently with RFC 5545 rather than using an arbitrary month-start timestamp.
- Context: RFC 5545 section 3.8.7.2 requires UTC DTSTAMP and, without METHOD, defines it as when the component information was last revised in the calendar store; it is equivalent to LAST-MODIFIED. The first draft fixes it to the roster month's first midnight for test determinism, without evidence that this is an actual revision time. The converter constructs new calendar information from a released source; its timestamp provenance needs a documented rule, and controlled test timestamps need not force arbitrary production values.
- Question: should the new exported representation's revision timestamp come from its actual construction time, or from an evidenced source-roster revision timestamp?
- Options:
  1. **Timestamp the constructed representation** — use the actual UTC revision/construction moment of the generated component information, with a controlled clock in tests.
     In practice: two exports of the same generated PDF may have different DTSTAMP values while their activity identity follows the separately selected UID policy.
     Gain: timestamp provenance is available even when the source omits reliable revision metadata.
     Cost: metadata varies between runs; tests must control or isolate that timestamp without erasing CRLF/folding checks.
  2. **Use evidenced source revision metadata** — use a documented source timestamp only if its semantics establish the represented information's revision time; do not equate a footer's creation time with last revision without evidence.
     In practice: repeated exports of an unchanged source retain its verified revision timestamp; a changed released source uses its new verified time.
     Gain: metadata follows the source information's evidenced revision rather than each materialization.
     Cost: source timestamp semantics and availability must be established; a footer labelled Created is not proof by itself, and missing metadata needs a separately valid fallback.
  3. **Use the source Created footer, falling back to construction time** — user-supplied rule for the selected family: treat its Created UTC footer as evidence of source-snapshot revision metadata; use the constructed representation's UTC timestamp when usable source revision metadata is unavailable.
     In practice: a generated `Created 10Jan2099 12:34 (UTC)` footer supplies the component timestamp; without usable source metadata, the current construction/revision moment supplies it.
     Gain: uses the source's stated snapshot time where available without preventing export solely because revision metadata is unavailable.
     Cost: footer parsing/association must be validated for this source family; fallback metadata varies between runs and requires controlled test time. This is not a generic rule that every PDF's creation time equals its last revision.
- Resolution: **Use the source Created footer, falling back to construction time** because the user explicitly accepted that footer as good evidence and specified the fallback. Rejected as the primary rule: **Timestamp the constructed representation**, because source metadata is preferred. Rejected as an incomplete rule: **Use source metadata without an established fallback**, because the user requires construction time when source revision metadata is unavailable. The interpretation is a user-confirmed selected-family convention; no arbitrary month-start stamp or universal Created/Last-Modified equivalence is asserted.

## Q28 — Association and conflicts of descriptive fragment fields

- Status: open
- Blocks: an unconditional retention/merge rule for non-trip optional descriptions; the confirmed Trip ID rule alone does not settle it.
- Context: the initial DESCRIPTION proposal selects Report (LT), A/C, Blocktime, Timezone Difference, Layover duration, and Tags alongside Trip ID. Some values occur on departure/arrival fragments or grouping rows rather than every completed activity. Required classification, endpoint, route, and explicit assignment contradictions cannot be silently repaired; the remaining question concerns differing selected optional values after an otherwise unambiguous activity has been assembled.
- Question: when compatible fragments supply different values for the same selected optional descriptive field, reject the inconsistent description or preserve both values with their source-fragment roles identified?
- Options:
  1. **Reject conflicting descriptive values** — reject an activity whose selected optional field cannot be represented as one consistent value without choosing between source fragments.
     In practice: generated departure and arrival fragments print Report (LT) 07:30 and 08:00; conversion fails rather than silently pick one report time.
     Gain: no apparently authoritative description hides a disagreement or mistaken association.
     Cost: legitimate multi-valued source annotations require a separately evidenced representation before support.
  2. **Preserve both with source roles** — retain differing optional values labelled by the source fragment, without guessing their broader business meaning or using them as time-calculation inputs.
     In practice: the generated description distinguishes a departure-fragment report value of 07:30 LT from an arrival-fragment value of 08:00 LT; neither becomes an inferred event start.
     Gain: retains supplied descriptive evidence rather than losing one value.
     Cost: longer descriptions and explicit provenance labels; this does not allow merging contradictory required dates, routes, or crew assignments.
- Evidence clarification 2026-10-11: the user requested a real PDF example and its filename. The differing Report values in this entry are a generated stress-test case, not an observed contradiction. Reinspection of a supplied split-leg example shows complementary endpoint data, equal repeated optional values, and additional values present only on the arrival fragment—not a conflict. The specific private filename/itinerary is supplied only in the user-facing answer for local checking, not copied into public topic state.
- User follow-up 2026-10-11: the user accepted that clarification as reasonable and asked to continue. Ordinary non-conflicting fragment handling is retained: available complementary fields combine, and equal repeated values do not become competing alternatives. The user did not select a policy for differing optional values or authorize implementation/Handoff.
- Resolution: parked pending actual contradictory-source evidence or a later deliberate robustness policy. The user has not selected either optional-conflict alternative. Subsequent complete-contract approval (Q31), which accepted the displayed DESCRIPTION selection, resolves field selection but not the differing-optional-value policy. Required date/route/assignment contradictions still fail, and descriptive values never calculate UTC instants.

## Q29 — Source availability window for single-day standby

- Status: closed 2026-10-11 (user/domain expert)
- Blocks: an unconditional SBX time-field requirement without interpreting single-day as all-day or copying a cancelled flight's times.
- Context: the standby/reserve definition decision (Q26) establishes SB and RE as availability assignments with start/end times. The user defined SBX as single-day standby or standby replacing a cancelled flight, but expressly stated the time-field requirement for SB and RE rather than SBX. No dated SBX row is present in the supplied PDFs. The replacement meaning does not authorize retaining a cancelled flight as another current activity or reusing its window.
- Question: does SBX also supply a start/end availability window in this family, or can it be printed without that window?
- Options:
  1. **SBX also supplies a window** — establish that SBX uses its own printed start/end values just like the timed SB/RE availability assignments.
     In practice: a generated SBX row with 08:00–16:00 in UTC columns produces one eight-hour single-day standby event, not an all-day marker or an additional cancelled-flight event.
     Gain: a defined, precise SBX interval without inventing times.
     Cost: required missing/unresolved endpoints cause explicit failure rather than an all-day or former-flight fallback.
  2. **SBX can lack the window** — establish that a dated SBX row may omit the time fields; its representation then needs an explicit rule rather than inheriting an assumption.
     In practice: a generated SBX row without UTC endpoints remains unresolved until its intended day-level or other representation is established; the old flight's times are not silently reused.
     Gain: accurately exposes the difference from SB/RE if that is the source convention.
     Cost: a separate missing-window mapping is required before claiming support for that case.
- Resolution: **SBX also supplies a window** because the user explicitly confirmed its own start/end times and UTC units. Rejected: **SBX can lack the window as an accepted case**, because the supplied rule requires its own window; missing/unresolved endpoints cause explicit failure rather than an all-day or cancelled-flight fallback. FRA is the stated location for the current examples, not a universal hard-coded station. The user identifies these facts as domain-expert information authorized for the contract now and requiring validation against a later provided duty plan. Semantics are confirmed; actual source-row/extraction validation remains pending.

## Q30 — UTC day qualifiers at month and year boundaries

- Status: closed 2026-10-11 (user)
- Blocks: a complete source-date interpretation rather than accidentally applying the roster month to an adjacent UTC date.
- Context: the date-association question (Q3) established within-month parenthesized UTC day qualifiers, and the narrow unqualified overnight rule is confirmed. The remaining boundary case has a local-date block at month start/end and an explicit UTC day number from an adjacent date. The supplied corpus does not include a checked month/year boundary row; a desired rule can be added with its source-validation obligation explicit, but it must not invent a distant month or use a revision/footer timestamp as an activity date.
- Question: resolve an explicit UTC day qualifier as the uniquely matching previous/same/next Gregorian date relative to its local-date block, or leave boundary-qualified cases unsupported until a real example is validated?
- Options:
  1. **Resolve the adjacent calendar date** — match the explicit day number to the previous, same, or next date around the source local date, carrying month/year correctly; reject if no unique match exists.
     In practice: local 1 January 2099 with UTC `(31) 23:10` means 31 December 2098, while local 31 January 2099 with UTC `(1) 00:20` means 1 February 2099, not 1 January.
     Gain: preserves the qualifier's meaning across ordinary calendar boundaries without offset arithmetic or arbitrary month assignment.
     Cost: the selected-family interpretation needs later boundary-source validation; a marker inconsistent with the adjacent dates fails instead of being repaired.
     Authorization: this explicit-day calendar rule only, not extending unqualified rollover or deriving dates from Report (LT), Timezone Difference, Blocktime, or Created metadata.
  2. **Defer boundary-qualified support** — fail these currently unvalidated source cases rather than choose their missing month/year interpretation.
     In practice: either generated boundary row above receives a clear unresolved-date diagnostic, while evidenced within-month qualifier cases remain supported.
     Gain: no boundary-support claim precedes its real-source validation.
     Cost: valid rosters with those boundary rows cannot convert successfully until support is deliberately added.
- Resolution: **Resolve the adjacent calendar date** because the user explicitly selected it. Rejected: **Defer boundary-qualified support**, because the chosen rule includes boundary interpretation now with later source validation. Match the marker to the uniquely corresponding previous/same/next Gregorian date around its own local-date block, carrying month/year correctly; otherwise fail rather than select a distant date or override the marker. This does not expand unqualified rollover, change its travel-only scope, or use revision/Report/TZ/Blocktime metadata as activity-date inputs.

## Q31 — Complete conversion contract before Handoff

- Status: closed 2026-10-11 (user)
- Blocks: human approval of the consolidated actor-goal contract, including inherited mappings/field selection, before preparing confirmed Handoff records.
- Context: the end-to-end draft in `use-cases/export-duty-plan-calendar.md` has passed a fresh-context structural review and incorporates the user-confirmed source, activity, timing, output-preservation, reporting, identity, and timestamp choices. This approval does not re-confirm the Frame or reopen those choices; it reviews the complete flow and explicitly displayed inherited EM/layover/description proposals. The unobserved optional-description disagreement remains parked rather than assigned a policy; domain/extraction, boundary, license, privacy, and Windows evidence still must be produced later.
- Approval subject:
  - Goal/Actor: the Crew member supplies one supported Released Roster PDF and obtains one complete RFC 5545 calendar beside it, not an imported/synchronized calendar.
  - Timing: flights and DH retain source UTC endpoints; FREE/OFF/U/FZ are all-day with source times ignored; TK/DTUSA/SB/SBX/RE use their own UTC windows. Explicit day qualifiers resolve independently to the unique adjacent Gregorian date; the selected narrow unqualified-travel overnight rule remains limited to its stated case.
  - Inherited mapping for approval: EM remains timed non-flight duty from printed UTC values; an explicit station-bearing dated Layover row is all-day. No unprovided EM expansion, inferred empty-row layover, or invented duration interval is adopted.
  - Context: preserve printed activity/designator text and leading zeroes; DH prefixes travel titles; known Trip ID carries only within the same unambiguous travel trip. The proposed description fields are Trip ID, Report (LT), A/C, Blocktime, Timezone Difference, Layover duration, Tags, including only supplied/associated values and coalescing identical compatible-fragment repeats.
  - Publication/failure: source/prior output contents are unchanged. New calendars and diagnostics select unused enumerated names; every failure attempts a new report identifying the source PDF, without hiding the primary failure if report creation fails.
  - Identity/metadata: distinct source-snapshot component identities are deterministic for the identical snapshot and independent of output counters; prefer the Created UTC footer as selected-family revision evidence, with construction-time fallback.
  - Evidence/privacy: expert standby/reserve and boundary rules require later real-duty-plan validation; public examples/fixtures/goldens are generated or anonymized and private sources are never committed. Inherited Rust/pin/Windows GNU/single-executable/license/hand-written-format constraints remain unchanged.
  - Parked unknowns: optional differing descriptions, source-derived publication rights if such derivatives are considered, and formal FZ provenance. Approval does not silently select an optional-conflict alternative or prove release readiness.
- Question: adopt this complete contract for Handoff preparation, or revise a named clause first?
- Options:
  1. **Adopt the complete contract** — confirm the displayed flow, guarantees, inherited mappings, and field selection while preserving the explicitly parked questions and validation obligations.
     In practice: an EM row with resolved 06:00–10:00 UTC is a timed non-flight event, a dated Layover ABC row is all-day, and a failed conversion produces no successful partial calendar but attempts a new diagnostic without replacing older files.
     Gain: one coherent, reviewed contract can be mapped to a permanent leaf and a complete-deliverable change seed during the separate confirmed Handoff.
     Cost: the preserved exclusions and evidence gates remain binding; unsupported/unresolved cases are not made production-ready by approval.
     Authorization: topic contract approval and Handoff preparation only; no code, change artifacts, seed publication, Persona/glossary promotion, or change start.
  2. **Revise a named clause first** — correct the displayed inherited mapping, field selection, flow, guarantee, or exclusion before Handoff preparation.
     In practice: if the dated Layover should have different representation, specify its observable outcome before the contract is signed off rather than letting implementation invent one.
     Gain: avoids planning a change against a wrong consolidated contract.
     Cost: affected rules/examples and review must be revisited before Handoff.
- Resolution: **Adopt the complete contract** because the user explicitly selected it. Rejected: **Revise a named clause first** because no clause revision was requested. The complete flow, guarantees, inherited EM/dated-Layover mappings, and DESCRIPTION selection are approved with the displayed parked questions and later validation obligations retained. This authorizes Handoff preparation only; seed/document publication, global Persona/glossary promotion, code, and starting a change still require the separate Handoff confirmation.

## Q32 — Publish the ideation Handoff records

- Status: closed 2026-10-12 (user)
- Blocks: publishing the readable ideation document and ready seed, and performing the proposed project-level Persona/glossary promotions.
- Context: the complete conversion contract was approved through the complete-contract decision (Q31). The Handoff preview in `handoff.md` displays the one permanent leaf mapping, one whole-deliverable seed, two Persona copy/merge previews, thirteen confirmed glossary terms, source topic, parked questions, and pending validation. Project Persona and sibling-topic inventory is empty apart from this topic, and the durable glossary is still a template, so the proposed promotions add rather than overwrite user biographies or existing definitions.
- Approval subject:
  - Source topic: Duty-plan calendar conversion (`duty-plan-calendar-conversion`).
  - Use-case mapping: Export a duty-plan calendar (`export-duty-plan-calendar`) maps to its own future leaf `givn/specs/export-duty-plan-calendar/usecase.md`, written only through normal proposal/spec/archive work.
  - Deliverable seed: Export complete Released Roster calendars (`export-released-roster-calendar`), suggested first/only start with no change dependency; one end-to-end Windows converter guarantee, not parser/writer/packaging-only intermediate seeds. The published seed will say Status: ready and retain accepted alternatives and full parked questions.
  - Persona promotions: Crew member exporting their own duty plan (`crew-member--personal-calendar-owner`) and Maintainer responsible for a public release (`project-maintainer--release-owner`) are copied into project collection with stable cores unchanged, append-only validated topic/review records retained, origin context, and a promotion record. The original topic files stay historical; both remain advisory, not Gherkin Actors or approval gates.
  - Glossary promotions: only the thirteen currently confirmed terms displayed in `handoff.md`; Duty plan, Flight leg, and Calendar export remain candidate introductions and are not silently promoted. No rejected term entry exists; unverified FZ expansion/provenance stays unverified.
  - Parked questions: conflicting optional descriptive values, source-derived publication rights if derivatives are considered, and formal FZ provenance. Standby/boundary/extraction and build/license/privacy/calendar-byte evidence remain later validation obligations, not an unverified release claim.
  - Files authorized: human-readable `docs/ideation/duty-plan-calendar-conversion.md`, the one topic-local ready seed, the two named project Persona files, durable glossary additions, and topic state recording this Handoff. No change scaffold, permanent leaf/spec delta, code, tests, fixture, golden calendar, existing-output deletion, or real private source publication is authorized.
- Question: publish this Handoff with the proposed seed and the displayed Persona/glossary promotions, or revise a named mapping, promotion, or seed first?
- Options:
  1. **Publish the proposed Handoff** — publish the displayed human record and ready planning seed, and perform each named promotion exactly as previewed.
     In practice: future proposal work can read the permanent advisory crew-member/release-owner profiles and the confirmed Duty-plan calendar boundary; the converter itself is still not implemented, and no change has been started.
     Gain: one durable, traceable planning input with reusable review lenses and vocabulary.
     Cost: promoted definitions/records become project-level references for future work and need controlled append/updates; all stated unknowns and validation gates remain visible rather than disappear.
     Authorization: only the listed Handoff files and promotions; the user separately invokes the normal proposal flow when ready.
  2. **Revise a named Handoff record first** — identify the mapping, Persona promotion, term promotion, or deliverable seed to change before publication.
     In practice: keep the maintainer Persona topic-local while promoting the crew-member lens, or defer a named glossary term; no unspecified promotion occurs.
     Gain: the durable record matches the intended reuse/authority and delivery cut.
     Cost: publication waits for the corrected preview and confirmation; affected records remain topic-local.
- Resolution: **Publish the proposed Handoff** because the user explicitly said “Publish this Handoff.” Rejected: **Revise a named Handoff record first** because no mapping, Persona, term, or seed deferral/revision was requested. Publication includes the readable ideation document, one topic-local ready seed, both named project Persona promotions with unchanged stable cores/retained records, and the thirteen confirmed glossary terms. Actual Handoff/promotion date is 2026-10-12, replacing the preview's planned date where publication metadata is recorded. No normal change, permanent spec leaf, code, test, fixture/golden, private-data publication, or existing-output cleanup is authorized or performed.
