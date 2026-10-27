# Change seed: export-released-roster-calendar

Status: implemented in export-released-roster-calendar

- Use-case IDs: export-duty-plan-calendar
- Topic: duty-plan-calendar-conversion
- Personas: crew-member--personal-calendar-owner, project-maintainer--release-owner
- Suggested start: 1 — first/only whole-deliverable seed; no prerequisite change.
- Handoff: explicitly confirmed by the user on 2026-10-12; readable record, this seed, both named Persona promotions, and thirteen confirmed glossary terms published. No normal change or implementation started.

Source links: [topic/resume state](../session.md), [approved contract](../use-cases/export-duty-plan-calendar.md), [full question/decision history](../questions.md), [Handoff mapping/promotions](../handoff.md), [human record](../../../../docs/ideation/duty-plan-calendar-conversion.md), [crew-member Persona](../../../personas/crew-member--personal-calendar-owner.md), [release-owner Persona](../../../personas/project-maintainer--release-owner.md).

The planning deliverable is **Export complete Released Roster calendars**: one standalone Windows converter produces a complete RFC 5545 calendar at an unused path, or exposes failure and attempts a new numbered diagnostic, preserving source/prior outputs. It is not a parser-only, writer-only, or packaging-only intermediate shipment. `ready` means ready input to normal proposal planning, not a compiled, tested, or releasable program.

## Accepted decisions

The records below preserve decision-relevant context, meanings, examples, gains/costs, chosen outcomes, and rejected alternatives. Full original standalone questions and dated resolutions remain in the linked source log; none is replaced there by an outcome-only summary. Initial constraints were supplied as non-negotiable rather than invented preferences.

### Baseline technology, distribution, and privacy constraints

- Context: the user specifies a local drag-and-drop/path converter, a public releasable repository, no real PII in committed artifacts, and fixed implementation/distribution constraints.
- Options:
  1. **Honor the supplied constraints** — Rust stable with application edition 2021, exact pdfplumber 0.1.0 pin, Windows GNU target built on Linux/CI, one executable without companion DLL/runtime installation, no native C/C++ PDF extraction dependency, hand-written RFC 5545, MIT/Apache-2.0 dependencies/material, generated/anonymized public evidence.
     In practice: the user supplies one PDF locally and gets a calendar beside it; no hosted upload, GUI, installer, private committed fixture, or unapproved dependency substitution is added.
     Gain: the intended portable/private/publicly distributable product remains bounded.
     Cost: exact API, extraction, resolved licenses, byte conformance, privacy/provenance, and target runtime imports must actually be verified.
- Chosen: **Honor the supplied constraints**, supplied by the user and retained in the approved Frame/contract. Rejected: silently relaxing them or creating extra hosted/import/synchronization functionality, because it is outside the confirmed scope. No false alternative violating a non-negotiable was offered.

### First-release source compatibility

- Context: the first draft promised any valid Jeppesen PDF; inspection supplied one apparent Released Roster detail-table family, not the export universe. Source: the supported-export decision (Q4), which bounds the release promise.
- Options:
  1. **One evidenced export family** — target the supplied Released Roster family and explicitly validated variations.
     In practice: repeated headers, sparse continuation rows, and UTC qualifiers in that family are covered; an unverified other family gets explicit unsupported-source failure.
     Gain: a concrete first target with available local evidence.
     Cost: other families are excluded; required semantics and extraction still need validation.
  2. **Several evidenced families** — obtain and verify additional export families before release.
     In practice: two layouts each have independent generated/anonymized fixtures and correct calendars.
     Gain: broader documented coverage.
     Cost: more sources, semantics, fixtures, and failure cases; only one family is currently evidenced.
  3. **Retain universal first-release ambition** — do not narrow any-valid-Jeppesen support, but do not claim it achieved without evidence.
     In practice: an unseen third layout cannot simply be excluded because two passed.
     Gain: preserves the initial breadth.
     Cost: readiness is blocked if the source universe cannot be bounded/verified.
- Chosen: **One evidenced export family**, explicitly selected by the user. Rejected: several families require sources outside this chosen target; universal support is not the chosen first-release promise. Expanding later needs new evidence; narrowing a published promise affects expectations.

### Whole-plan consistency boundary and one Actor goal

- Context: complete conversion or explicit failure is the approved outcome; components have no useful independently shipping stopping point. Source: whole-plan boundary decision (Q9) and complete-contract adoption (Q31).
- Options:
  1. **Duty-plan calendar as whole-plan boundary** — all required activity representations from one input must resolve before success; PDF/file/report/snapshot/trip group are records or context.
     In practice: one unresolved arrival fragment prevents the understood legs from becoming an apparently complete successful calendar.
     Gain: a domain boundary matching completeness without request-record/server aggregates.
     Cost: validate the complete required representation before publishing; capacity limits need evidence.
  2. **Revise the boundary first** — correct its domain name/extent while preserving complete-or-failure.
     In practice: establish an actual alternative domain noun rather than let code invent a component-shaped root.
     Gain: intended vocabulary/ownership.
     Cost: the canvas/contracts wait for corrected declaration and collision checks.
- Chosen: **Duty-plan calendar as whole-plan boundary**. Rejected: boundary revision was not requested. The approved `!` goal Export a duty-plan calendar belongs to Crew member; release verification remains an advisory/evidence obligation, not another product goal. No persistent database/workflow or particular code class is required.

### Vacation and additional vacation days

- Context: actual U/FZ rows lacked UTC clocks and accounting zeros are not event times. U meaning/all-day behavior and FZ practical meaning/mapping were supplied by the user. Source: vacation decision (Q11), FZ definition (Q12), and FZ mapping (Q14).
- Options:
  1. **Use established code meanings and code-driven all-day treatment** — U is Urlaub/vacation; FZ is an additional vacation-related day off; FREE/OFF/U/FZ are all-day and ignore source clocks.
     In practice: OFF with 23:00 and 01:00 remains an all-day activity; FZ with 09:00 and 17:00 does too.
     Gain: no omitted absences or invented midnight intervals.
     Cost: printed clocks cannot affect these event times; entitlement/acronym provenance is not established.
  2. **Park unknown meanings or revise FZ mapping** — do not invent semantics/representation when knowledge is absent or a boundary correction is needed.
     In practice: unresolved U/FZ could not be guessed to be a flight or layover; a genuinely different absence-block extent would need specification.
     Gain: avoids a wrong definition or boundary.
     Cost: affected conversion rules remain incomplete until clarified.
- Chosen: **Established meanings and all-day treatment**, directly defined/accepted. Rejected: parking U/FZ is unnecessary after expert definitions; FZ representation revision was not requested. The initial Start==00:00 prerequisite is superseded for these codes. No annual/age/quota or labor-agreement enforcement is added; suggested formal FZ expansion remains unverified.

### Office/seminar and consulate-duty timing

- Context: TK and DTUSA are real non-flight codes omitted by the initial model. The user defined TK as office/development/team-information attendance and DTUSA as a consulate visit for a USA visa, with DT expanding to Duty Time. Sources: definitions (Q15, Q17) and printed-window choices (Q16, Q18).
- Options:
  1. **Use each printed UTC interval** — timed non-flight attendance/duty with resolved positive endpoints, not ordinary flight-route validation.
     In practice: TK 08:00–12:00 is four hours; DTUSA 09:00–15:00 is six hours, even without a flight destination.
     Gain: preserves the supplied hours and leaves the rest of the day outside the event.
     Cost: missing/unresolved endpoints cause failure rather than invented all-day fallback.
  2. **Use all-day markers** — date-level duty labels with source clocks ignored.
     In practice: those same printed windows become whole-day activities without times.
     Gain: coarse day-level reminders.
     Cost: discards scheduling precision and represents more than the supplied interval.
- Chosen: **Printed UTC intervals** for both, explicitly selected. Rejected: all-day markers discard their chosen scheduling precision. No unprovided TK/EM expansion, generic DT suffix mapping, visa application, or duty-accounting calculation is inferred.

### Deadhead meaning and visibility

- Context: DH is in Pos, not Activity, and can accompany missing A/C/zero Blocktime despite positive travel time. The user defined it as positioning in uniform without usual operating work, still duty time. Sources: assignment meaning (Q19), visibility choice (Q20).
- Options:
  1. **DH in the title** — prefix the printed designator/route while keeping one underlying travel interval.
     In practice: `DH 007 ABC-DEF` is distinguishable in compact calendar views; a non-DH title remains `007 ABC-DEF`.
     Gain: role distinction without opening details.
     Cost: longer title than the initial convention.
  2. **DH only in description** — ordinary designator/route title, role in details.
     In practice: title `007 ABC-DEF`, description `Pos DH (deadhead duty)`.
     Gain: uniform concise titles while retaining the role.
     Cost: compact views cannot distinguish the assignment.
- Chosen: **DH in title**. Rejected: description-only hides the distinction in the chosen view. Zero accounting time does not erase travel or make it all-day; no second event, pay calculation, UID-from-title, or client update behavior is authorized.

### Narrow unqualified travel overnight interpretation

- Context: initial algorithm repaired an earlier End clock generally; explicit UTC day markers cannot be overwritten. Source: the unqualified-overnight decision (Q21).
- Options:
  1. **Reject unqualified dates** — require more date evidence instead of automatically repairing the clock pair.
     In practice: same local block 12, unqualified 23:10 to 01:05, no separate arrival block, yields a diagnostic.
     Gain: does not conceal a bad/missing time behind a plausible night flight.
     Cost: legitimate unqualified overnight forms remain unsupported pending evidence.
  2. **End is the next UTC day in this narrow case** — both endpoints unqualified, same block, no separate arrival block, End clock strictly earlier.
     In practice: day 12 23:10 to day 13 01:05, 1:55, independent of Blocktime.
     Gain: handles an unmarked overnight pair.
     Cost: assumes a convention requiring real-family validation; never repairs equal clocks, explicit marker contradictions, or unrelated ground-duty cases.
- Chosen: **Narrow next-day End**, explicitly selected. Rejected: blanket rejection is not the selected interpretation. Do not generalize this repair beyond its stated travel case.

### Explicit UTC qualifiers at calendar boundaries

- Context: a numeric UTC day can belong to an adjacent month/year; assigning the roster month blindly is wrong. Source: adjacent-date decision (Q30), completing the researched date-association record (Q3).
- Options:
  1. **Resolve the unique adjacent Gregorian date** — match the marker to previous/same/next date around its own source local date, carrying month/year; no match/ambiguity fails.
     In practice: local 1 January 2099 with UTC (31) means 31 December 2098; local January 31 with (1) means February 1.
     Gain: correct boundary carry without timezone-offset arithmetic.
     Cost: later boundary-source evidence is still needed; no arbitrary distant date repair.
  2. **Defer boundary support** — reject currently unvalidated boundary rows.
     In practice: the same generated rows yield unresolved-date reports while within-month cases work.
     Gain: no claim before real-source validation.
     Cost: otherwise valid boundary rosters cannot convert yet.
- Chosen: **Adjacent Gregorian dates**, explicitly selected with later validation. Rejected: deferral is not the chosen rule. Start/end resolve independently; unqualified endpoints use their own source context, and Report/offset/accounting/revision fields never date activities.

### Calendar preservation and enumerated outputs

- Context: the initial fixed output path could destroy an earlier calendar. Source: calendar-preservation decision (Q10), including the user's additional alternative.
- Options:
  1. **Replace after complete publication** — update the fixed path only after full new output is ready.
     In practice: a failed conversion preserves the prior version; a success replaces it.
     Gain: simple same-path reruns.
     Cost: successful replacement loses old/manual contents without backup and requires verified preservation/replacement behavior.
  2. **Refuse and stop at collision** — preserve old output and report its occupied name.
     In practice: the user relocates/removes old roster.ics before rerunning.
     Gain: no implicit replacement.
     Cost: manual work on repeat conversion.
  3. **Preserve and enumerate** — choose another unused numbered path automatically.
     In practice: retain roster.ics/roster-1.ics and publish another complete file such as roster-2.ics, reporting its actual path.
     Gain: no loss and no mandatory manual cleanup.
     Cost: versions accumulate; import/version selection is the user's action, not automatic reconciliation.
- Chosen: **Preserve and enumerate**, user-supplied. Rejected: replacement violates no-overwrite; stopping at collision omits the requested automatic new name. No occupied candidate may be clobbered under a race. Hyphen punctuation is the concrete convention illustrated in the approved record; no automatic deletion/backups or power-loss durability claim is added.

### Within-trip descriptive identifiers

- Context: real Trip ID cells are sparse at grouping boundaries, unlike repeated synthetic values. Source: trip-description decision (Q23).
- Options:
  1. **Carry known ID in the same unambiguous travel trip** — describe each associated travel leg even when its cell is blank or crosses page/date context.
     In practice: legs 007/008 of T42 both say `Trip T42`; the later T43 does not inherit T42.
     Gain: available grouping context on every travel event.
     Cost: prove boundaries and prevent stale context; ambiguous association is not permission to guess.
  2. **Only own-row/fragment IDs** — use a value only when printed on that leg's contributing rows.
     In practice: the first leg says T42, the blank-cell next leg has no Trip field.
     Gain: direct row traceability without group inheritance.
     Cost: many events omit their available trip context.
- Chosen: **Within-trip carry**, explicitly selected. Rejected: own-row-only omits the selected grouping information. This does not copy Report/TZ/Tags into every event or add inherited metadata to unrelated ground/absence/layover rows.

### Diagnostic preservation and source identification

- Context: a fixed error path could overwrite earlier diagnostics. Source: failure-report decision (Q22), with source filename content approved by the user.
- Options:
  1. **Keep latest at fixed path** — replace the old error report.
     In practice: roster.parse-error.txt changes to the latest reason while calendars stay protected.
     Gain: one predictable report location.
     Cost: old contents/manual notes are lost and stale-success handling needs a rule.
  2. **Preserve numbered reports** — allocate an unused report path and identify the source/current reason.
     In practice: keep roster.parse-error.txt and write roster.parse-error-1.txt containing `Source PDF: roster.pdf`.
     Gain: history without file loss.
     Cost: accumulated reports must distinguish the current run/path from old ones.
- Chosen: **Preserve numbered reports**, explicitly selected, with source PDF filename in content. Rejected: fixed-path replacement loses the requested history. Old files are neither deleted nor called a new successful write; real local filenames/reports are not authorized public test data.

### Diagnostic attempts after every failure

- Context: the first draft reported interpretation errors but panicked on output failure. Source: all-failure reporting decision (Q24).
- Options:
  1. **Attempt a diagnostic for every failure** — include publication errors without relying on report saving for visibility.
     In practice: failed publication exposes failure and, if possible, creates a numbered report identifying the source/reason.
     Gain: explanation can survive drag-and-drop process exit.
     Cost: storage can also reject the report; preserve the primary error and state no new report was saved.
  2. **Only interpretation reports** — publication errors use message/non-success outcome only.
     In practice: a denied output write leaves no attempted sidecar.
     Gain: fewer writes during storage failure.
     Cost: transient console can disappear without retained explanation.
- Chosen: **Every-failure attempt**, explicitly selected. Rejected: interpretation-only excludes the requested publication-failure explanation. No writable-storage, successful report, or rollback guarantee is fabricated.

### Separate source-snapshot component identities

- Context: RFC UID is persistent/globally unique; date/code/route alone can collide across owners or repeated components. No import/synchronization is in scope. Source: calendar-identity decision (Q25).
- Options:
  1. **Corresponding duty IDs across revisions** — retain IDs for stable unambiguous duty correspondence.
     In practice: flight 007 changing 09:00 to 10:00 UTC retains event-A.
     Gain: correlation for clients using UID matching.
     Cost: ownership/stable keys and ambiguous-repeat rules; client updates/deduplication are not guaranteed.
  2. **Separate source-snapshot IDs** — different input snapshots have separate identities; the identical snapshot repeats deterministically with distinct component IDs.
     In practice: a revised PDF gives that flight event-B, while another output filename for the identical original retains event-A.
     Gain: clear immutable snapshot boundary without cross-revision matching promises.
     Cost: unchanged activities can get new IDs and may accumulate if different snapshots are imported.
- Chosen: **Separate source snapshots**, explicitly selected. Rejected: cross-revision correspondence is not the chosen entity model. Output counters/titles are not identity. The fingerprint/discriminator technique is later design work; changing IDs after import may require manual cleanup/reimport, not a filename rename or demonstrated migration.

### Preferred source revision time with construction fallback

- Context: without METHOD, RFC DTSTAMP means associated component information's revision time, UTC; arbitrary month-first is not evidence. Source: timestamp-provenance decision (Q27), with the user's added combined rule.
- Options:
  1. **Construction/revision moment as primary** — current generated-information timestamp, controlled in tests.
     In practice: same snapshot IDs can accompany different DTSTAMP values on separate runs.
     Gain: always available from the representation clock.
     Cost: metadata varies; tests must isolate it without erasing CRLF/folding evidence.
  2. **Evidenced source revision only** — require verified source semantics/time, with no established fallback.
     In practice: unchanged source retains its verified stamp; missing evidence blocks that policy.
     Gain: source revision provenance.
     Cost: source Created label is not a universal Last-Modified rule and missing metadata needs a valid fallback.
  3. **Created UTC source footer, construction fallback** — selected-family expert convention prefers its Created footer, otherwise the actual generated-information timestamp.
     In practice: generated Created 10Jan2099 12:34 (UTC) supplies DTSTAMP; unusable source metadata uses controlled/current construction time.
     Gain: source-stated snapshot time without blocking solely on absent metadata.
     Cost: validate footer association/parsing and fallback clock; not a universal equation of any PDF's creation and revision.
- Chosen: **Preferred Created UTC footer with fallback**, explicitly provided by the user as good evidence for this family. Rejected: construction-only is not primary; source-only is incomplete without requested fallback. Neither stamp calculates scheduled instants or uses arbitrary forecast-month midnight.

### Timed standby, single-day standby, and reserve

- Context: private inputs had statistics but no dated standby row. The expert defined SB/Stby, SBX, and RE, and explicitly permits adding semantics before later real-duty-plan validation. Sources: standby/reserve information decision (Q26) and SBX-window decision (Q29).
- Options:
  1. **Add the established timed-window rule** — SB/RE have own start/end windows; SBX also supplies its own UTC window, single-day or replacing a cancelled flight.
     In practice: SBX 08:00–16:00 at example FRA is eight hours; SB, SB, FREE, SB preserves each window/day off, no continuous absence or old-flight copy.
     Gain: expert semantics rather than guessing from accounting zeros or annual counters.
     Cost: missing required endpoints fail; source extraction/station association must be validated later. FRA is example context, not a global constant.
  2. **Keep timing unverified / accept missing SBX window** — leave the cases unresolved or allow missing-window data needing another mapping.
     In practice: a dated required standby without a known window cannot be guessed from the cancelled flight or silently omitted.
     Gain: no false timing inference before authoritative information.
     Cost: no complete support until its representation is established.
- Chosen: **Expert timed own-window rule**, explicitly supplied, including SBX UTC units and validation obligation. Rejected: parking all timing is unnecessary after expert definition; a missing SBX window is not an accepted case. Activation-to-report notice differentiates SB/RE but is outside conversion, not a dispatcher/notification use case. No real dated row was manufactured by this answer.

### Complete contract, inherited mappings, and one Handoff seed

- Context: a reviewed end-to-end contract integrates all choices rather than creating component-shaped use cases. Source: complete-contract adoption (Q31) and Handoff publication (Q32).
- Options:
  1. **Adopt/publish the complete record** — approve EM as timed printed-UTC non-flight duty, explicit dated station-bearing Layover as all-day, and the original DESCRIPTION selection/order: Trip, Report LT, A/C, Block, TZ, Layover duration, Tags. Preserve ordinary complementary/equal fragment handling and parked differing-value questions.
     In practice: EM 06:00–10:00 is timed; Layover ABC is all-day; failed publication attempts a fresh report while old files stay intact. One seed covers the complete Windows converter guarantee.
     Gain: coherent contract, stable leaf ID, durable review lenses/vocabulary, traceable planning input.
     Cost: exclusions, parked questions, and actual validation/release gates remain binding; approval is not implemented readiness.
  2. **Revise a named contract/Handoff clause first** — correct a mapping, field, promotion, or deliverable cut before publication.
     In practice: defer a named Persona promotion or alter a genuinely different Layover representation with reviewed examples.
     Gain: durable records reflect intended authority/scope.
     Cost: affected review/publication waits.
- Chosen: **Adopt then publish**, explicitly confirmed on 2026-10-11 and 2026-10-12 respectively. Rejected: no clause/promotion/term/seed revision was requested. Both named advisory Personas and thirteen confirmed terms were promoted; candidates were not. This seed starts no change, writes no permanent specs/code, and chooses no parked optional-conflict alternative.

## Open questions

### Differing optional descriptive fields after compatible fragment assembly

This blocks an unconditional non-trip metadata retention/merge rule. The DESCRIPTION selection is approved, but source association/differing optional values are not resolved by Trip ID inheritance. A generated departure/arrival pair with Report (LT) 07:30 and 08:00 has no counterpart contradiction found in the supplied corpus. Required classification/date/route/assignment contradictions still fail.

Question: should otherwise compatible fragments with different selected optional values cause rejection, or retain both values with their source-fragment roles?

1. **Reject conflicting optional descriptions** — do not emit a single apparently authoritative value by choosing one.
   In practice: the generated Report disagreement yields a diagnostic, not arbitrary 07:30/08:00 selection.
   Gain: exposes inconsistent data or wrong association.
   Cost: legitimate multi-valued annotations need another evidenced representation before support.
2. **Preserve both with source roles** — retain optional evidence with departure/arrival labels, not inferred broader semantics.
   In practice: description distinguishes departure-fragment 07:30 LT and arrival-fragment 08:00 LT; neither calculates UTC instants.
   Gain: no supplied descriptive value is lost.
   Cost: longer descriptions/provenance labels; cannot justify merging conflicting required values.

Status: parked/unselected. Source: [descriptive-fragment question](../questions.md#q28--association-and-conflicts-of-descriptive-fragment-fields). Ordinary complementary/equal fields combine as approved; Handoff does not silently choose this robustness policy or claim its support.

### Rights to publish source-derived test material, if considered

This affects future anonymized/adapted source material, not permission to read private files or create independent generated tests. The user requires public MIT/Apache-releasable material, generated/anonymized data, and no committed private PDFs. Access/anonymization alone is not a publication license.

Question: what authoritative source-use terms permit a proposed source-derived public example, or should planning use independently generated examples only?

1. **Establish permission before any derivative** — obtain the source-use evidence rather than infer rights from privacy removal.
   In practice: a proposed adapted/anonymized document is not published until its rights/provenance are established; real private PDFs stay uncommitted.
   Gain: legal/source provenance supports the public-material claim.
   Cost: additional evidence; a permissive project license does not manufacture permission.
2. **Use independent generated material** — keep source-derived publication unverified and do not rely on it.
   In practice: Typst generates a fictional future roster with independent data, while real plans remain local validation inputs.
   Gain: avoids needing a derivative-publication assumption for the initial evidence set.
   Cost: generated layout/fonts still need actual extraction fidelity and public-material license review; they are not identical to real producer evidence.

Status: parked factual/source-scope clarification, no derivative permission claimed. Source: [publication-rights question](../questions.md#q5--rights-to-publish-source-derived-test-material). The first draft's independent synthetic route remains available; do not invent a legal answer.

### Formal FZ expansion and entitlement provenance, if documented

This blocks only a verified acronym/agreement claim, not conversion. FZ's practical additional-vacation meaning/all-day representation is approved; the proposed German expansion and described age/annual entitlement source remain unverified. The exporter neither grants leave nor checks labor-law eligibility.

Question: what authoritative source establishes the formal expansion/provenance if a verified statement is needed, or should documentation retain only the known practical meaning?

1. **Establish evidence before formal claims** — obtain the authoritative wording/provenance rather than turn a suggestion into fact.
   In practice: a glossary names an agreement only after evidence; no age/quota validation enters conversion.
   Gain: accurate background explanation if needed.
   Cost: research unrelated to the initial calendar outcome and not a first-release blocker.
2. **Retain practical meaning without expansion/provenance claim** — keep the optional background unknown.
   In practice: FZ still appears as the assigned all-day activity without asserting an agreement or enforcing four-day/age conditions.
   Gain: bounded accurate converter behavior.
   Cost: no formal entitlement explanation is provided.

Status: parked/non-blocking; no acronym/agreement fact invented. Source: [FZ provenance question](../questions.md#q13--formal-fz-expansion-and-entitlement-provenance).

## Validation obligations before release claims

- Validate expert SB/SBX/RE dated rows, UTC columns, and station associations against the later supplied duty plan; FRA is not a parser constant.
- Validate qualifier month/year edges and the selected unqualified travel variation with source evidence; generated stories specify rules but do not prove producer behavior.
- Demonstrate required-activity coverage, compatible fragment/trip boundaries, and dynamic column/header/body extraction for the selected profile; word/table counts alone are insufficient.
- Verify CRLF bytes, text escaping, 75-octet UTF-8 folding, positive endpoints, distinct globally scoped/deterministic snapshot IDs, source/fallback timestamps, and no-clobber naming/failure/race behavior.
- Reconcile the original oracle/data inconsistencies, five overlong lines, incorrect future weekday, missing standby case, ineffective Blocktime-independence example, normalization weakness, and timestamp privacy-regex false positive before test acceptance.
- Verify actual pinned Rust APIs, complete dependency licenses/native status, Linux tests, Windows GNU cross-build, executable runtime imports/no companion installer, and public-data provenance/privacy gates.
- Keep real-source validation local/uncommitted; external validators/issues receive generated/anonymized material only.

These are future design/spec/implementation/review work, not tasks silently executed by Handoff. The user invokes the normal proposal flow separately with this ready seed; no normal change exists yet.
