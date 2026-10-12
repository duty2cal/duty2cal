# Duty-plan calendar conversion — confirmed Frame and research

Status: confirmed by the user on 2026-10-10. Later research observations are distinguished from confirmed requirements below.

## Problem

Crew members need an accurate personal-calendar representation of their published duty plan without manually re-entering its activities.

Manual re-entry as the underlying problem was initially inferred from the requested conversion outcome and confirmed when the user adopted this Frame.

## Vision

A crew member supplies a Jeppesen duty-plan PDF to a local converter and receives a complete RFC 5545 calendar file suitable for import into their calendar application. If the converter cannot interpret the duty plan safely, it reports the failure rather than silently omitting activities or producing a partial calendar.

Example: a flight whose departure and arrival occupy two date blocks becomes one timed calendar event, not two incomplete events. Whether those block dates directly establish UTC dates remains unresolved.

## Confirmed scope

- Input: the Released Roster detail-table family represented by the privately supplied examples, with explicitly validated variations. The user selected this first-release boundary on 2026-10-11 through the supported-export decision (Q4), which narrows the initial universal-support ambition. Additional export families are outside the first-release promise; individual code meanings and date/layout variation rules still require elaboration.
- Outcome: a calendar file beside the input PDF, or an explicit failure with a diagnostic when it can be persisted. Failure remains explicit if the diagnostic destination is unwritable; this clarification follows the reviewed event flow and does not select overwrite or recovery behavior.
- Preserve the draft's flight, FREE, OFF, standby, EM, and layover distinctions; their exact meanings and calendar behavior need domain elaboration.
- Preserve local Windows drag-and-drop or path invocation, no GUI, no installer, and one distributable executable.
- No automatic calendar import, ongoing synchronization, hosted upload service, duty-plan editing, or determination of the official duty schedule is proposed.
- Do not silently omit unrecognized activities or unmatched flight fragments.
- Overwrite behavior, recovery from output-write failures, repeated imports, and revised duty plans are not settled by this Frame.

## Confirmed stakeholders and interests

These are confirmed stakeholder types, not approved Persona biographies or Gherkin Actors.

| Stakeholder type | Interest | Concrete example |
|---|---|---|
| Crew member converting their duty plan | Accurate activity dates, durations, and distinctions with minimal manual work. | An overnight flight appears once with the correct departure and arrival instants. |
| People identified in a duty-plan PDF, including its owner | Their identifying information does not enter the public repository or public test material. | A crew name, staff identifier, and phone number from a private example do not appear in a golden calendar. |
| Project maintainer / release owner | A distributable, legally compatible, verifiable converter without personal-data exposure. | A release has one Windows executable and tests based on generated or anonymized examples. |

## Constraints inherited from the first draft

Source: `temp_first_idea.md`, Mission and Non-Negotiable Constraints.

- Rust stable, edition 2021.
- Windows GNU target, cross-compiled on Linux/CI, no companion DLL or runtime installer.
- Dependencies restricted to MIT or Apache-2.0; public repository material must be releasable on that basis.
- Exactly pinned Rust `pdfplumber` extraction dependency, no native C/C++ extraction dependency.
- Hand-written RFC 5545 output, UTF-8, CRLF, octet-aware line folding; no iCalendar writer library.
- Clean Architecture is a supplied technical constraint, not a proposed actor goal or use-case boundary.

The implementation snippets are draft hypotheses, not verified APIs or architecture. Feasibility conflicts must be reported to the user; no substitution is authorized.

## Confirmed privacy direction

Source: user's 2026-10-10 response accepting the topic identity and identifying `private/` as private evidence.

- Never commit `private/` or its source documents.
- Reading and analyzing private PDFs is expressly authorized by the user's subsequent Frame-confirmation response. The prohibition concerns committing private material, not inspecting it.
- All fixture and golden-calendar data must be anonymized or generated.
- Topic notes must not become a second copy of private data: record only non-identifying structural observations and generated examples.
- Privacy checks must consider identifying combinations of dates, itinerary, trip identifiers, and diagnostic row context; replacing a crew name alone is not sufficient evidence of anonymization.
- Do not submit real duty plans or identifying calendar output to external validators or documentation services. Any public validation example must use generated or anonymized material.
- Ignoring files is not proof that every public artifact is free of personal information.

## Research protocol and current evidence

1. Read existing state before asking about established behavior: `givn status` and `givn spec tree` show no changes or permanent specifications; no earlier topic exists.
2. Read durable project context: the README and relevant arc42 chapters are templates; the glossary declares no terms or aggregates.
3. Preserve user work: `temp_first_idea.md` is the supplied first draft and remains unedited.
4. Inventory evidence without copying private data into public artifacts: `private/` contains five PDFs, is ignored by Git, and contains zero tracked files.
5. Examine the private PDFs using the available PDF-capable read tool, as explicitly authorized, and retain only non-identifying observations needed to answer domain questions. Do not install tooling or publish private content implicitly.
6. Match observable requirements to generated or anonymized examples rather than assuming a synthetic layout proves real-export support.
7. Verify technical-library feasibility separately before a change becomes ready; do not let guessed API snippets decide domain contracts.
8. Establish source-use rights before publishing anonymized derivatives or adapted layouts. Availability of a private example does not establish permission to publish a derivative; generated material remains an option.

## Findings from the first draft

- **Date association is unproven.** The parser combines `Date (LT)` blocks directly with UTC times. A generated example of 6 January 00:30 at UTC+02 corresponds to 5 January 22:30 UTC; a local block date cannot be assumed to equal the UTC date. Parked in the date-association question (Q3), which asks for evidence about the export's date convention.
- **The first draft's compatibility promise was unbounded.** Acceptance criterion 9 promises any valid Jeppesen duty-plan PDF, but only one synthetic layout is specified. Resolved by the supported-export decision (Q4): on 2026-10-11 the user selected the privately supplied Released Roster detail-table family and its explicitly validated variations for the first release, not universal support.
- **Standby is not covered by the proposed table.** No `SB` or `SBX` rows occur despite the fixture claiming every entry kind.
- **The blocktime-independence example is ineffective.** The three overnight flights last 4:30, 12:00, and 12:00, matching their stated blocktimes rather than differing from them.
- **The golden calendar contains untraceable values.** Timezone differences and `5_MFL` tags appear in expected output but have no matching source cells in the supplied table; the `3_OFF` value appears under Block rather than a Tags column.
- **Safe failure still needs elaboration.** The draft reports parse errors, but does not establish protection of pre-existing output files or handling of write failures. No behavior has been chosen for these cases.
- **Source-use rights remain unknown.** Permission to publish adapted layouts or anonymized derivatives has not been established. Parked in the publication-rights question (Q5), which requests evidence before any derivative is published.

## Advisory Frame review

A fresh-context review on 2026-10-10 found no blocking Frame defect. It advised preserving the original universal-support ambition as an explicit unresolved alternative, avoiding vocabulary that could license partial exports, checking identifying combinations as well as names, and recording publication rights as unknown. Those corrections preserve existing constraints and do not settle the pending Frame or compatibility decision. No Persona was used; the user remains the decision-maker.

## Confirmation boundary

The user accepted this Frame and authorized reading the private PDFs for structural and conversion-algorithm analysis. That authorizes domain exploration of the displayed problem, scope, and stakeholder interests. It does not approve Personas, date rules, compatibility breadth, overwrite behavior, use-case contracts, a release claim, technology substitutions, implementation, or Handoff.

Changing the Frame later requires revisiting any affected Persona reviews, event chains, and use-case contracts.

## Structural observations from authorized private-PDF inspection

Source: the five privately supplied monthly PDFs, inspected through parsed text and rendered pages. The descriptions below retain public column labels and non-identifying format characteristics, not source itineraries, personal identifiers, filenames, or copied date/time sequences. All examples in this section are independently generated.

### Document and table boundaries

- Each inspected PDF has two pages. The first includes a month/year heading, identifying crew header, overview calendar, statistical tables, and the start of a detailed activity table. Later pages repeat the month heading and detail-table headers.
- Statistical tables vary within the corpus while the detailed activity columns have the same apparent meaning. This supports recognizing the detail table independently of the overview and statistics; it does not prove universal layout support.
- A new page can begin with a continuation activity that has neither a date nor a trip ID. Page boundaries do not terminate the preceding date/trip context or an incomplete flight leg.
- The detail table has more columns than the draft's shortened example, including `LSW`, `BZW`, `Tags`, `Blocktime`, duty metrics, and rest metrics. Several headings span multiple lines. Flat parsed text is insufficient to assign adjacent numeric values safely; the rendered column structure matters.
- Date-only rows exist without an activity. Their absence of activity does not establish FREE, OFF, or layover.

### Dates, times, and flight fragments

- UTC time cells sometimes carry an explicit parenthesized day number. That day can precede or follow the row's local date. Such qualifiers occur on both Start and End values.
- A complete flight row may contain explicit day qualifiers on both timestamps. A split flight may also have a qualified start or end. These are independent features, not one edge case.
- A split flight can occupy consecutive local-date blocks yet depart and arrive on the same UTC day. A local-date change therefore does not imply a UTC midnight crossing.
- Generated example: a local-date block 12 contains Start `(13) 00:20`, and an arrival fragment in block 13 contains End `08:10`. The indicated UTC interval is day 13 00:20–08:10, lasting 7:50; using block 12 for the departure would add a false 24 hours.
- Generated example: block 12 contains Start `19:15`, and block 13 contains End `(12) 21:05`. The indicated UTC interval is day 12 19:15–21:05, lasting 1:50 despite the two local-date blocks.
- The meaning of an explicit within-month day qualifier is directly evidenced. Mapping qualifiers across month/year boundaries and handling genuinely inconsistent or absent dates remain unverified.
- Numeric flight designators can include leading zeroes. The Pos column distinguishes operating assignments and `DH` assignments. Missing aircraft and a zero Blocktime can coexist with a positive timed travel interval; they are not evidence that the activity should disappear.
- The Timezone Difference column appears alongside arrival information; these samples do not justify treating it as an unambiguous departure offset. No timezone arithmetic or report-time-derived UTC calculation is adopted.

### Activity and field placement

- FREE and OFF rows have blank Start/End cells in the inspected rendered tables. Their visible `00:00` values belong to other metric columns such as LSW and BZW, not necessarily the UTC time columns. The draft's `Start == 00:00` recognition prerequisite does not fit these examples.
- Layovers appear as station-bearing text spanning part of the row (`Layover: <station>`), rather than a conventional Activity cell followed by zero UTC times.
- The corpus contains additional codes `TK`, `U`, `FZ`, and `DTUSA` beyond the draft's named kinds. On 2026-10-11 the user defined U as Urlaub/vacation and explicitly confirmed FREE, OFF, and U as all-day events with source times ignored. The meanings and calendar treatment of the other additional codes remain unresolved; some have UTC intervals and others do not.
- `SB` and `SBX` occur in overview/statistical material; that is not evidence of a dated standby activity in the detail table. The inspected corpus does not supply a standby duty-row example.
- Tags occupy their own column and can contain comma-separated text or punctuation. Duty/report/trip information is sometimes present only at a grouping boundary; the intended inheritance of those values into calendar descriptions remains unresolved.

## Candidate reconstruction implications — not implementation decisions

1. Recognize the detailed activity-table region and its multi-line semantic headers on every page; exclude the overview and statistical tables as activity sources.
2. Reconstruct cells with their column identity intact, including explicit day qualifiers; do not treat a header word as an entire column or flatten away qualifiers.
3. Carry dated activity context and incomplete-flight context across pages, while distinguishing a blank continuation from a date-only row.
4. Treat an explicit UTC-cell day qualifier as evidence separate from the local row date. Determine each endpoint date independently before checking duration; do not repair away conflicting explicit dates by automatically adding a day.
5. Resolve flight fragments in sequence and context, preserving position and designator text. Matching solely by flight number is not sufficient when a designator repeats.
6. Classify explicit activity codes and station-bearing layover rows before validating their required fields. Do not derive all-day status from zeros in unrelated metric columns, or classify every unknown code as a flight.
7. Preserve descriptive fields from the actual column and fragment where they occur. Determine inheritance and merge-conflict rules through later examples rather than silently filling values.
8. Produce an export only after the required activities and dates are resolved. Unknown codes, ambiguous dates, and unmatched fragments remain candidates for explicit failure under the confirmed no-silent-omission rule.

These implications are research input to Event Storming and use-case elaboration, not a change-side technical design. No extraction-library capability has been tested and no algorithm has been implemented.

## Confirmed activity-rule update — 2026-10-11

Source: the user's answer to the focused U-meaning question (Q11), which defined vacation and explicitly supplied the all-day mapping.

- U means Urlaub, or vacation.
- FREE, OFF, and U always use all-day calendar representation; source time values are ignored for these activities.
- Nonzero or missing time values cannot turn one of these codes into a timed event. The draft's `Start == 00:00` classification prerequisite is superseded for these codes.
- No midnight-crossing repair or timed-interval validation is applied to these activities on the basis of ignored source times.
- FZ, TK, DTUSA, DH, and metadata/display rules are not resolved by this answer.

### Subsequent FZ meaning clarification

- The user defined FZ as additional days off associated with vacation. This practical meaning is confirmed through the FZ-meaning decision (Q12).
- “Frei zusätzlich” is only a possible expansion, not an established fact. The user-described age/annual entitlement background and agreement provenance are not conversion rules.
- Entitlement validation is not added to scope. The user separately accepted all-day FZ treatment on its duty-plan date with source times ignored through the FZ representation decision (Q14). The source code remains distinguishable; output-label and metadata rules are not decided by this acceptance.

### Subsequent TK meaning clarification

- The user defined TK as office-attendance or professional-development/seminar duty, including team information-sharing sessions.
- This practical meaning is confirmed through the TK-meaning decision (Q15); a formal acronym expansion is not asserted.
- The user selected the printed UTC Start/End interval for TK through the office/seminar timing decision (Q16). It is a timed non-flight activity; the FREE/OFF/U/FZ time-ignore rule does not apply. Unresolved endpoint dates/times cause explicit failure rather than an invented all-day fallback.

### Subsequent DTUSA meaning clarification

- The user stated DT means Duty Time and DTUSA denotes a consulate visit to obtain a USA visa through the DTUSA-meaning decision (Q17).
- This is a known non-flight duty. The user selected its printed UTC Start/End interval through the consulate-duty timing decision (Q18); unresolved required endpoints cause explicit failure rather than an invented all-day marker.
- Do not confuse an activity-code expansion with numeric duty-time accounting columns, infer other DT-suffix meanings, or add visa-processing/institutional integrations to scope.

### Subsequent DH assignment clarification

- The user defined DH as Dead Head positioning to a departure destination, still duty time in uniform without the usual operating work, through the DH-assignment decision (Q19).
- DH belongs to the Pos column and describes the assignment on an underlying travel activity. Zero accounting Blocktime does not erase that travel or turn it into time off; the converter does not calculate pay or operating-work totals.
- The user selected DH in the title through the deadhead-visibility decision (Q20), for example `DH 007 ABC-DEF`. It remains one timed travel activity; the display choice does not approve new identifiers or reimport-update behavior.

### Date-rule elaboration now in progress

- Explicit UTC day qualifiers observed in the source must not be ignored or overridden by a generic midnight-repair step. Endpoint dates are resolved independently; accounting Blocktime is not a substitute for date evidence.
- The user selected next-UTC-day rollover through the unqualified-overnight decision (Q21), only for travel with unqualified endpoints in one local-date block, no separate arrival-date block, and End clock earlier than Start. Equal clocks and explicit qualifier contradictions are not repaired; no ground-duty or Blocktime-based fallback is introduced. Local source evidence for this particular variation remains unverified even though the desired rule is confirmed.
- Month/year boundaries, source fragments missing at roster boundaries, and unqualified date ambiguity remain part of the broader date-association question (Q3). The confirmed activity/title rules do not settle them.

## Read-only extraction-dependency verification — 2026-10-11

- Context7 resolution returned Python pdfplumber and unrelated ports, not the Rust crate; those results were not treated as evidence for Rust APIs.
- Authoritative Rust sources read: https://docs.rs/pdfplumber/0.1.0/pdfplumber/ , https://docs.rs/pdfplumber/0.1.0/pdfplumber/struct.Page.html , and https://crates.io/api/v1/crates/pdfplumber . No private source content was submitted to these services.
- The pinned Rust version 0.1.0 exists, is not yanked in the inspected registry metadata, and declares MIT OR Apache-2.0. This verifies the direct crate's declaration, not the resolved dependency graph's license compliance.
- The version documents positioned word extraction and table detection. The draft's path-opening and word/page access snippets do not match the documented 0.1.0 API; adaptation belongs in later technical design, not new code during ideation.
- Registry metadata declares Rust 1.85 and edition 2024 for the dependency. This does not require changing the application's requested edition 2021; the exact stable-toolchain/build requirements still need implementation evidence.
- Available docs list Windows MSVC targets, not proof of the requested Windows GNU executable or absence of companion DLLs. No extraction against private PDFs with this Rust crate, compile, license gate, or cross-target dependency/runtime test has been run.
- The latest registry version is not authorization to upgrade the exact pin. The user-specified version, no-native-dependency constraint, and distribution requirements remain unchanged; implementation feasibility is still an evidence obligation.

## Confirmed calendar-output preservation — 2026-10-11

- The user rejects overwriting existing calendars and requests enumerated names for new exports instead of failing solely because the ordinary name exists.
- Use the ordinary basename when free; illustrate collision handling as `roster.ics`, then `roster-1.ics`, then `roster-2.ics`. Exact hyphen punctuation is a draft notation, not a separately requested preference.
- Never replace an occupied calendar candidate, including a candidate claimed during concurrent conversion. Report the actual newly published path; old versions are not automatically deleted.
- This calendar decision initially left reports separate. The user subsequently selected no-overwrite numbered diagnostic names through the failure-report decision (Q22) and report attempts after every conversion failure through the publication-failure decision (Q24).
- This is required no-clobber behavior to verify, not evidence of implemented file atomicity, rollback, power-loss durability, or import-update compatibility.

## Confirmed travel-trip metadata — 2026-10-11

- The user selected carrying a known Trip ID across travel legs within the same unambiguous roster trip through the trip-description decision (Q23).
- Blank continuation cells, page breaks, and date changes do not erase that context. A new trip or unrelated duty must not receive an old identifier simply because it was the last one read.
- Trip metadata is descriptive only. This does not authorize propagating report times, station offsets, or unrelated ground/absence/layover metadata and does not affect UTC assembly.

## Confirmed diagnostic preservation — 2026-10-11

- The user selected numbered diagnostic names that preserve existing reports through the failure-report decision (Q22) and approved identifying the source PDF filename inside the report.
- Draft example: preserve `roster.parse-error.txt`, write `roster.parse-error-1.txt`, and include `Source PDF: roster.pdf`. Report the actual newly written path; do not delete old reports or mistake one for a current write.
- This permits local output identifying the attempted source, not copying actual private filenames/data into public topic notes, fixtures, or golden calendars.
- The user selected report attempts after every conversion failure, including calendar publication failure, through the publication-failure decision (Q24). Failure visibility remains independent of report-write success; an unwritable destination still cannot be promised a saved report.

## Read-only calendar-standard and first-draft checks — 2026-10-11

- Authoritative source read: RFC 5545, sections 3.8.4.7 (UID) and 3.8.7.2 (DTSTAMP), at https://www.rfc-editor.org/rfc/rfc5545.txt . No private data was submitted.
- UID must be persistent and globally unique. A date/code/route recipe alone does not scope different crew members or distinguish all repeated components. The calendar-identity question (Q25) now records the corresponding-activity versus source-snapshot choice without promising a client's reimport behavior.
- DTSTAMP is UTC. Without METHOD it records when associated component information was last revised in the calendar store, equivalent to LAST-MODIFIED; with METHOD it records instance creation. The first roster-month midnight is not evidence of either event. Timestamp provenance remains explicit under its question (Q27); controlled test time does not require arbitrary production values.
- Read-only inspection of the first draft's synthetic golden block found 19 events and five unfurled content lines above the user's 75-octet limit: lengths 77, 77, 77, 90, and 90. The literal oracle must be reconciled with required folding in later test work, not committed verbatim as contradictory evidence.
- Gregorian calculation shows 1 January 2099 is Thursday, not the draft table's Saturday. Generated fixture labels must be internally consistent; no first-draft edits were made.
- The draft's unanchored six-digits-plus-letter PII regex flags generated `DTSTAMP:20990101T000000Z`. This is a demonstrated false positive, not real personal data; later guard design must distinguish valid calendar syntax from private identifiers and must not mistake binary-skipping grep for PDF privacy proof.
- The draft's normalization removes line-ending distinctions, so its comparison cannot establish the separately required CRLF byte property. Later evidence must preserve or directly assert that property. No calendar, fixture, test, or guard code was written during these checks.

## Confirmed calendar-component identity — 2026-10-11

- The user selected separate source-snapshot identities through the calendar-identity decision (Q25), not matching activities across revised PDFs.
- Repeated conversion of the identical fixed source version retains deterministic component identities while output names can differ. A revised source snapshot has separate identities for its components even if some activity data is unchanged.
- Distinct components require globally unique identifiers; output enumeration and human-facing titles do not supply that identity. The concrete fingerprint/discriminator method belongs in later technical design.
- No calendar-client reconciliation, update, duplicate suppression, or cleanup is promised. The identity policy is not permission to publish actual source snapshots or private calendar data.

## Confirmed revision-timestamp provenance — 2026-10-11

- The user accepts the selected family's Created UTC footer as good evidence of source-snapshot revision metadata and prefers it for DTSTAMP.
- If no usable source revision metadata is available, use the constructed representation's actual UTC revision/construction time. The user explicitly supplied this fallback through the timestamp-provenance decision (Q27).
- The first-draft roster-month-midnight constant is superseded. A controlled test clock can make fallback examples deterministic without erasing CRLF/folding evidence.
- This is a source-profile convention to validate during extraction/design, not a universal claim that any PDF creation label means last revision. It does not alter activity timing, source snapshot identity, or old files.

## Clarification of descriptive-fragment evidence — 2026-10-11

- The two conflicting Report (LT) values discussed during elaboration are a generated edge case, not an observed contradiction in the supplied private source.
- A rechecked split-leg case demonstrates ordinary complementary endpoints, identical repeated optional values, and values present on only one fragment. That is distinct from the optional-conflict policy parked under the descriptive-fragment question (Q28).
- The user requested the actual filename and location for local inspection. Those private references and itinerary details are supplied in the dialog only, not persisted in public topic artifacts.

## Confirmed standby/reserve domain information — 2026-10-11

- The user defined SB as standby/Stby with start/end times, often assigned on multiple consecutive dates interrupted by FREE days. Preserve those individual windows rather than create an all-day or continuous block.
- The user added RE as reserve with start/end times. SB and RE differ by the activation-call-to-reporting lead time; that difference does not require activation, notice-time calculation, or check-in behavior in this converter.
- SBX is single-day/replacement standby with its own UTC start/end window, explicitly confirmed through the single-day-window decision (Q29). No all-day inference, old-flight window reuse, or extra cancelled-flight event is introduced.
- Domain information closes the broad standby-evidence question (Q26) but does not manufacture a dated row in the supplied private corpus or prove its extraction. Later generated/source validation must include the new codes without copying real data into public fixtures.

### Explicit expert-rule validation obligation

- The user states SB/SBX/RE information comes from the domain expert, permits including it now, and requires checking it against a later provided duty plan.
- UTC units are confirmed; FRA is the station for the current examples, not a universal mapping for the export family.
- This separates accepted business semantics from still-pending source-layout/extraction evidence. It does not authorize fixture/code creation during ideation or an unsupported production-readiness claim.

## Confirmed UTC qualifier boundary rule — 2026-10-11

- The user selected adjacent Gregorian-date resolution through the boundary-date decision (Q30), completing the source-date interpretation captured in the date-association record (Q3).
- Resolve an explicit UTC day marker independently to the unique previous/same/next date around its own local-date block; calendar arithmetic carries month/year. No unique match or contradictory required evidence causes failure rather than arbitrary month selection.
- Generated examples: local 1 January 2099 with (31) 23:10 means 31 December 2098; local 31 January 2099 with (1) 00:20 means 1 February 2099.
- This does not extend the narrow unqualified-travel rollover or use source revision metadata, report times, offsets, or Blocktime as activity dates. Real-source boundary validation remains pending despite the desired rule being accepted.
