# Generalized duty-plan input — question log

Every parked unknown, with the alternatives and what each one costs. A closed
question keeps its full text.

## Q1 — Does "further formats" mean further PDF layouts or any published export representation?

- Status: closed 2026-10-25 (user)
- Blocks: the Frame's scope and stakeholder list, and therefore every later
  phase of this topic. It decides whether a second family must be a PDF at all.
- Context: the converter reads one published PDF layout — the Released Roster
  detail-table family — and refuses every other document with one opaque
  message (`SourceError::UnsupportedFamily`). The user asked to keep the
  converter open for further "Dienstplanformate" and pointed at a corpus of
  per-operator PDF converters. The word admits two readings, and they differ
  in what a family is: the anti-corruption intermediate between the file and
  the domain (`RawRosterTable`: column labels plus rows of cell strings) is
  already free of PDF words and positions, so a CSV or HTML family could fill
  it; what a non-PDF family cannot do is arrive as something other than a
  monthly, dated detail table with printed activity codes, because the
  domain's interpretation and vocabulary are unchanged either way. The choice
  also fixes which evidence a contributor must bring (a stripped PDF, or any
  export form) and what the public copy promises.
- Question: Should a source family be a PDF publication layout only, or any
  published representation of a duty plan?
- Options:
  1. **PDF publication layouts only.** Meaning: a family is a layout that a
     crew scheduling system publishes as a PDF; openness means recognizing and
     adding such layouts. In practice: a SunExpress PDF is refused by name as a
     recognized but unsupported publication; a crew app's CSV export is refused
     as not a supported duty-plan publication, whatever its columns say.
     Gain: matches the confirmed constraints (one pinned PDF extraction
     dependency, no native dependency, one self-contained executable), and the
     evidence we can actually obtain from the corpus and from a crew member is
     PDF-shaped. Cost: a crew member whose system publishes CSV, HTML or an
     `.ics` feed gains nothing, even where that data would be trivial to read;
     adding them later reopens the scope.
  2. **Any published export representation.** Meaning: a family may be a PDF,
     CSV, HTML or calendar publication, each with its own adapter filling the
     same intermediate shape. In practice: an operator that emails an `.ics`
     or an HTML roster is supported by an adapter that yields the same dated
     rows; a PDF family and a CSV family coexist and are listed together.
     Gain: reaches crew members the PDF route cannot, and some formats are far
     cheaper to read than positioned PDF text. Cost: the support promise and
     the evidence standard must cover non-tabular publications (an `.ics` feed
     is a list of events, not a dated detail table, so it does not fit the
     intermediate and needs a different domain reading); the refusal text must
     separate "not a duty-plan publication" from "a supported kind, unknown
     family", and the release story gains a second input surface to test.
- Resolution: **Any published export representation** because the user asked
  for openness "möglichst" and does not want a second scope decision later; the
  reach of a crew member to the tool matters more than keeping the input
  surface to one file kind. Rejected: **PDF publication layouts only** because
  it would close the door on a crew member whose system publishes CSV, HTML or
  a calendar feed, and reopening it later would change the Frame, the public
  copy and the evidence standard again.

Consequences the later phases must handle (not decisions):

- The port (`RosterSource`: bytes in, `SourceDocument` out) already admits any
  family, but `SourceDocument` carries exactly one shape — a `RawRosterTable`
  of column labels and cell strings. A tabular non-PDF publication fits it; an
  `.ics` or other event-list publication does not, so the intermediate becomes
  a design decision rather than a given.
- The refusal must distinguish an unrecognizable document from a recognized
  publication kind whose family is unsupported.
- The support promise, the evidence standard and the release checks now cover
  more than PDF extraction; no non-PDF family is evidenced today.

## Q2 — Adopt the Frame for additional export families?

- Status: closed 2026-10-25 (user); the adopted Frame was later withdrawn by
  the user, and its stakeholder types were carried into the rewritten Frame
  (Q6).
- Blocks: Persona creation, the event chain, and every use case in this topic.
  Nothing can be derived until the goal, the boundary and the stakeholders
  stand.
- Context: the Frame in [frame.md](frame.md) states the problem, the vision,
  the scope, the stakeholders and the evidence. Q1 already widened it: a
  family may be any published representation of a duty plan. What is still
  unconfirmed is the whole package, and in particular three things that shape
  everything after it — that no second family is implemented in this topic,
  that adding a family is a declared identity plus one adapter plus an
  evidence set, and that the refusal names a recognized publication instead of
  refusing everything with one sentence.
- Question: Should the recorded Frame — goal, vision, in-scope and out-of-scope
  list, and stakeholder types — be adopted as the basis for this topic's use
  cases?
- Options:
  1. **Adopt the Frame as recorded.** Meaning: the problem, the vision, the
     scope list and the five stakeholder types become the basis; the phases
     proceed to Personas and Event Storming. In practice: the next artifacts
     describe how a plan is recognized as a family, how an unsupported but
     recognized publication is refused, and how a family is declared with its
     evidence — and they do not implement a second family. Gain: a bounded,
     evidence-backed basis; the openness work stays independent of a format
     nobody supplied. Cost: no crew member outside the supported family gains
     a conversion from this topic; the promise is openness, not reach.
  2. **Revise a named element first.** Meaning: one element of the Frame
     (goal, an in-scope item, an out-of-scope item, or a stakeholder type) is
     changed before elaboration. In practice: if the supported set must grow
     in this topic, the user names the publication and supplies a stripped
     sample; the Frame then carries a second family and the evidence standard
     is exercised against it. Gain: the work is aimed at a real second
     publication rather than at a seam. Cost: it waits for that sample; without
     one the use case cannot be specified and the topic stalls.
- Resolution: **Adopt the Frame as recorded** because its every claim rests on
  inspected code and public evidence, and because the user's request was
  openness rather than immediate reach. Rejected: **Revise a named element
  first** because the only element that would change the outcome — a second
  implementation — has no sample behind it, so the revision would stall the
  topic instead of sharpening it.

The Frame in [frame.md](frame.md) is confirmed by this resolution. Persona
creation, Event Storming, Use Case Definition, Use Case Elaboration and
Handoff remain unconfirmed steps of their own.

## Q3 — Which Persona segments review this topic?

- Status: closed 2026-10-25 (user)
- Blocks: the Persona review of the event chain and of every use case, and the
  Handoff's Persona promotion choice. Personas are review lenses; they do not
  become Gherkin Actors.
- Context: the Frame names five stakeholder types. The project collection and
  the sibling topics already hold seven Personas, and reuse is preferred: a new
  segment needs a distinguishing criterion, not a new title. Two of the five
  types map cleanly onto existing segments, and one contributor segment covers
  three existing files whose difference is the artifact they review. The open
  part is the privacy lens: the earlier conversion topic named "people
  identified in a duty-plan PDF" as a stakeholder type but created no Persona
  for it, and this topic is exactly where that lens would earn its place,
  because a new family is admitted on a stripped sample that a person must
  recognize as their own data.
- Question: Which Persona segments should review this topic?
- Options:
  1. **Reuse two, add two.** Meaning: `crew-member--personal-calendar-owner`
     covers both crew-member types (same authority, evaluation and vocabulary
     — the publication kind changes nothing they judge) and
     `project-maintainer--release-owner` covers the support promise; new are
     `contributor--source-family-contributor` (a reviews-only contributor whose
     evaluation is a publication's family identity, printed codes and dates,
     and whether a fixture is anonymized) and
     `data-subject--identified-plan-owner` (the person in a plan, whose only
     evaluation is recognizing their own identifiers in a sample and whose only
     authority is over their own data). In practice: the contributor rejects a
     fixture that still shows a base, a trip ID and a date range; the
     identified person says "that combination is me" even though the name is
     replaced. Gain: the privacy obligation acquires a lens that can actually
     spot identifying combinations, which is the one risk a new family brings
     in. Cost: one more persona file, and the identified person cannot decide
     anything — their finding must be arbitrated by the maintainer.
  2. **Reuse two, add the contributor only.** Meaning: the identified person
     stays a stakeholder type with an obligation, as in the earlier conversion
     topic, and the privacy check remains a maintainer duty. In practice: the
     evidence standard states that a new family's fixture must be stripped, and
     the maintainer judges stripping by the rules; no Persona voices the
     "combination identifies me" objection. Gain: smaller record, no lens
     without decision authority. Cost: the objection that the project has
     already accepted as a real risk in an earlier Frame review has no reviewer
     standing behind it.
- Resolution: **Reuse two, add the contributor only** because the identified
  person cannot decide anything and the stripping rules are already written and
  arbitrated by the maintainer; a Persona without authority would add a lens
  nobody must answer. Rejected: **Reuse two, add two** because it purchases a
  privacy lens at the cost of a segment whose findings the maintainer already
  owns — the identifying-combination risk stays recorded as a stakeholder
  interest and as a maintainer obligation in the evidence standard.

Confirmed Personas: `crew-member--personal-calendar-owner` (reused),
`project-maintainer--release-owner` (reused), and the new
`contributor--source-family-contributor`. The stakeholder type "people
identified in a duty-plan document" keeps no Persona; its interest is a
constraint enforced through the evidence standard.

## Q4 — Which word names one published publication form of duty plans?

- Status: **withdrawn 2026-10-25 (user)**, before it was answered.
- Withdrawal: the user rejected the premise — "forget about the family. This was
  just the first step towards a generalization." A per-publication "family"
  presumes the publication is the unit of the model, and Q5 reopens exactly
  that. The candidates stay in the record because both words are still in use
  in the repository and the retirement decision returns with whatever unit Q5
  selects.
- Context (unchanged, for the record): two candidate words exist — "source
  family" in the code (`RosterSource`, `SourceError`, `SourceDocument`),
  chapter 6 and four archived proposals, and "Export family" in the public
  issue form and the documents quoting it. The glossary defines neither. The
  chosen word would have become the parent term and the refusal wording would
  have used it.
- Options (recorded, not decided): **Source family** — one vocabulary from code
  through architecture to the issue form, matching "source snapshot" and
  "source revision timestamp", renaming the public form field. **Export
  family** — the crew member's own word, keeping the public field name, at the
  price of two vocabularies for one idea.
- Resolution: withdrawn; the unit the word names is in question (Q5).

## Q5 — Which unit does the generalization use, if not the publication?

- Status: closed 2026-10-25 (user)
- Blocks: the entire Event Storming. The event chain, the use cases and the
  change seeds are organized around whatever unit is chosen, so nothing past
  the Frame can be written until this is settled.
- Context: the confirmed Frame organizes support per published source family:
  a named publication is recognized, declared or unsupported, and refusal names
  it. The user has rejected that premise — "forget about the family. This was
  just the first step towards a generalization" — so the topic needs a
  different organizing idea. Three candidates are available, and they differ in
  what a contributor edits to add a publication, what the refusal can say, and
  how much of the domain stays code. The evidence stands either way: one
  intermediate shape (`RawRosterTable`), one activity vocabulary fixed in the
  application, and the corpus showing per-operator and per-era layout change.
- Question: Should the converter be opened by generalizing the input model, by
  generalizing the reader through a declared mapping, or by generalizing only
  the refusal?
- Options:
  1. **Generalize the input model.** Meaning: define one neutral duty-plan
     document — dated activities, their printed fields, their provenance —
     that every reader fills, and make "support" a property of the model rather
     than of a named publication. In practice: a monthly table is read into the
     same neutral document whatever its header; a reader that cannot map a
     field or a code says which field or code, as in "no mapping for column
     `Duty`; codes S5, S6 and RO are unknown", rather than naming a family. A
     CSV whose headers are declared maps without a new reader. Gain: openness
     lives in the shape every source is translated into; a new publication is a
     translation, not an identity the project must name and maintain; nothing
     is declared before it is understood. Cost: "recognized" becomes
     field-granular, so the completeness rule needs restating (what counts as
     mapped enough); the evidence standard becomes per field and per code
     rather than per document; the refusal gets longer.
  2. **Generalize the reader through a declared mapping.** Meaning: one generic
     reader plus a mapping descriptor per publication — column labels, code
     table, date convention — so adding a publication is declaration, not Rust.
     In practice: a new operator is a mapping file and a code table reviewed in
     a pull request, in the spirit of the corpus's `Dienste.txt`, while the
     timing rules stay code. Gain: the cheapest possible addition, and the
     vocabulary is visibly operator data. Cost: a second language to review and
     test; the mapping cannot express the domain's timing rules, so the split
     between mapping and code becomes the new hard boundary and a wrong file
     can look like a supported publication.
  3. **Generalize only the refusal.** Meaning: keep one reader and one published
     layout; stop claiming a format and state what was recognized and what was
     missing. In practice: "the detail table carries no From column" instead of
     "not a supported Released Roster export", and a dropped plan names the
     part that could not be read. Gain: truthful, small, no new abstraction.
     Cost: a second publication still needs code; this is honesty, not
     openness.
- Resolution: open.


- Resolution: **Generalize the input model** because it is the only candidate
  that makes a further publication a translation instead of an identity the
  project must name and maintain, and because it keeps the confirmed domain
  rules as the fixed point. Rejected: **Generalize the reader by declared
  mapping** because the mapping cannot express the timing rules, so the split
  between mapping and code becomes a new hard boundary and a wrong declaration
  can look like support; **Generalize only the refusal** because a second
  publication would still need a reader and the openness would be a wording
  change.

Consequences the later phases must handle (not decisions): the intake model
becomes the anti-corruption target and must admit a publication without a
table; refusal becomes fact-granular while `SourceError`'s two variants reach
both facades; the public statement moves from a format name to required facts
plus evidence; the contributor segment is renamed with the unit.

## Q6 — Adopt the Frame rewritten for the generalized input model?

- Status: closed 2026-10-25 (user)
- Amendment 2026-10-25, after a fresh-context Frame review: two of the adopted
  option's sentences needed correction before the Frame could stand.
  1. "No publication is ever declared or named by the project" overstated the
     decision. The project ships no publication list, taxonomy or configuration,
     but the **Support statement** does name the publications whose fixtures
     convert, and that set is derived from the check suite rather than
     maintained as a list.
  2. The Frame now names the confirmed rules the direction replaces (the
     supported-family rule, main-flow step 2, extension 2, the scenario that
     asserts the unsupported-family diagnostic, the glossary's family entry as
     a support boundary, chapter 3's input sentence, and the diagnostic's
     `Source PDF:` label), and it no longer calls the change facade-free or the
     model's storage shape the reason a non-tabular publication is refused.
- Blocks: Event Storming, the use cases and every later phase of this topic.
- Context: after Q5 the Frame was rewritten. What it now claims: the converter
  states what a duty plan must provide (dated activities, printed codes,
  routes, clocks, the source's own revision) as one model; every publication
  is read into it before interpretation; interpretation, the activity
  vocabulary, the timing rules and the calendar contract stay unchanged; a
  refusal names the fact, field or code that could not be established instead
  of a format; adding a publication changes one reader and its fixture and
  nothing else; the public statement speaks of required facts and evidence
  rather than of formats. Outside: implementing a second reader, any format or
  operator registry, a mapping language, changes to the calendar contract, and
  reading anything that is not a duty plan.
- Question: Should this rewritten Frame be adopted as the basis for the topic's
  event chain and use cases?
- Options:
  1. **Adopt the rewritten Frame.** Meaning: the model-centred scope, the four
     stakeholder types, the three Personas and the constraints become the
     basis; the phases proceed to Event Storming. In practice: the event chain
     starts at a supplied duty plan, passes through what could be established
     from it, and ends either in a calendar or in a refusal naming the missing
     fact; adding a publication is a contributor's translation with a fixture,
     and no publication is ever declared or named by the project. Gain: the
     unit matches the user's direction and the openness is testable — the model
     either admits a publication or says what it lacks. Cost: the refusal and
     the public statement must be rewritten, and "is my format supported" has
     no yes/no answer any more — the crew member has to read what was missing.
  2. **Revise a named element first.** Meaning: one element — the scope list, a
     stakeholder, the treatment of the intake model's fragment/date handling,
     or the public-statement claim — is changed before elaboration. In
     practice: if the intake model must stay a table, the Frame says so and the
     calendar-feed case leaves the scope; if the public statement must stay a
     format list, that is written down as the promise. Gain: the disagreement is
     settled where it belongs. Cost: the phases wait for it.
- Resolution: open.

- Resolution: **Adopt the rewritten Frame** because every element follows from
  the input-model decision (Q5) and the evidence already inspected, and because
  the model-centred scope is what makes a further publication a translation
  rather than a registry entry. Rejected: **Revise a named element first**
  because no element is in dispute — the only remaining unknowns (the model's
  exact shape, refusal wording, evidence form) are design and use-case work,
  not Frame disagreements.

The Frame in [frame.md](frame.md) is confirmed for the reopened direction.
Personas stay as confirmed (Q3) with the contributor renamed to
`contributor--publication-reader`.

## Q7 — Term check, Event Storming: confirm `Duty plan`

- Status: closed 2026-10-25 (user)
- Blocks: the Event Storming vocabulary and every use case that names its
  subject.
- Context: the topic's central noun was a candidate from the earlier conversion
  topic, never confirmed. The glossary uses the phrase inside nine other
  definitions but defines it nowhere. Confirming it fixes the subject of the
  Frame, the refusal text and the intake rules.
- Options: **Confirm Duty plan** — a crew member's schedule as their crew
  scheduling system publishes it for one period, assigning dated activities and
  their stated facts; contrast: not the derived calendar, not the system, not a
  format. **Revisit the term first** — keep it a candidate and paraphrase.
- Resolution: **Confirm Duty plan** because the artifacts already refer to it
  everywhere and the collision check found one meaning and no competitor.
  Rejected: **Revisit the term first** because leaving the central noun
  unpinned while the input model is generalized around it would force every
  later reader to re-derive the subject.

## Q8 — Term check, Use Case Definition: confirm `Evidence for a publication` and `Support statement`

- Status: closed 2026-10-25 (user)
- Blocks: the two new use cases, which name their input artifact and their
  outcome.
- Context: both nouns were introduced by the Event Storming. `evidence` and
  `support` are heavily used in the project, both unqualified, and `statement`
  is used for the site's public claims. Neither candidate had a definition, and
  neither collided with an established meaning once qualified.
- Options: **Confirm both**; **Revisit the statement term** — choose between
  the document ("Support statement") and the limit it states ("Support
  boundary", matching the release owner's existing "documented support
  boundaries"); **Revisit the evidence term** — choose between the qualified
  compound and a sample-based name.
- Resolution: **Confirm both** because each fills its genus slot with a
  qualifier that no confirmed term already owns, and because renaming later
  would touch the event chain, both use cases and the public wording.
  Rejected: **Revisit the statement term** and **Revisit the evidence term**
  because no competing meaning was found that the qualifier does not already
  separate.
