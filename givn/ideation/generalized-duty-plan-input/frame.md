# Generalized duty-plan input — Frame

Status: **confirmed by the user on 2026-10-25 (Q6)**, in the direction Q5 set
and Q1 widened. Research observations below stay findings, not requirements.

## Problem

The converter's intake is the shape of one publication, and its support rule
names that publication. A PDF is reduced to column labels and rows of cell
strings (`RawRosterTable`), and the interpretation refuses any table that lacks
the labels `Date`, `From` and `To` (`interpret`,
`crates/duty2cal-core/src/application/mod.rs`). Support is decided by three
header labels — `Date`, `(LT)`, `(UTC)` (`recognises_family`,
`crates/duty2cal-pdf/src/lib.rs`).

What a refused crew member actually sees is inconsistent:

- Bytes that are not a readable PDF get their own message
  (`SourceError::Unreadable`).
- A readable table of another publication is refused with "the source is not a
  supported Released Roster detail-table export" — one sentence naming a format
  the crew member has never heard of.
- A table that is close enough to be read but lacks a column is refused later
  with "the detail table carries no From column" — a fact, but stated without
  the date or entry it concerns.
- An unknown activity code is refused with the code token
  (`ConvertError::UnresolvedRequiredActivity`), without the date of its entry.
- The diagnostic file always begins `Source PDF:`, whatever was supplied.

Three consequences follow, and they are about the model, not about a missing
list of formats:

- Taking on a further publication means writing a second reader around a second
  shape. Nothing in the converter states what a duty plan must provide; it
  states what one table looks like, and `interpret` enforces that shape by
  demanding three column labels.
- A refusal does not let a crew member tell whether their document was nearly
  readable, was a different publication, or was something else entirely.
- Support can only be stated by naming a publication, which is a promise the
  project has to maintain, and the statement cannot say what a plan must
  provide.

## Vision

The converter states what it must learn from a duty plan — the facts the
confirmed rules require, independent of how the plan is published — as one
model. Every publication is read into that model before anything is
interpreted. Interpretation, the activity vocabulary, the timing rules, the
calendar contract and the facades' behaviour stay as they are.

- A publication that states the required facts **can be read**; whether the
  plan then converts still depends on the unchanged rules resolving every
  required activity, date, fragment and interval.
- A publication that cannot be read is refused naming what could not be
  established — the field, the code with the date of its entry, the absent fact
  — rather than a format name.
- Adding a publication is a translation into the model: one reading, one
  fixture, and the composition that wires it. It changes no domain rule, no
  calendar output, no facade behaviour and no release path.
- The public statement says what a duty plan must provide and which
  publications the exercised evidence covers. It derives that list from the
  check suite; it maintains no list of its own.

Example, and the facts are generated: a plan whose activity column states `S5`
on 12 January 2099 is refused with "the duty plan states the code S5 on
12 January 2099, which the converter does not understand". A tabular export
would enter the same model if a reading for it existed; **no such reading
exists today**, and nothing here claims one converts.

## Confirmed inputs

- **Q1** — any published representation of a duty plan may be supported, not
  only a PDF layout.
- **Q5** — the openness is located in the input model. Rejected: a declared
  mapping language per publication, and a refusal-only generalization.
- **Q6** — this Frame. An earlier, family-centred Frame was confirmed and then
  withdrawn by the user ("forget about the family. This was just the first step
  towards a generalization").
- **Q7** — the term `Duty plan`.
- **Q8** — the terms `Evidence for a publication` and `Support statement`.
- **Q3** — the Personas: `crew-member--personal-calendar-owner` and
  `project-maintainer--release-owner` reused; `contributor--publication-reader`
  new. **Q2** confirmed the earlier Frame's stakeholders; its stakeholder types
  carry over, and this Frame revises them below.

## Confirmed rules this direction replaces

The direction contradicts confirmed statements. Each is named here so the
seeding change rewrites it deliberately instead of appearing to break it:

| Confirmed statement | Where | What replaces it |
|---|---|---|
| The first release targets the Released Roster detail-table family, not every PDF producer layout. | `givn/specs/export-duty-plan-calendar/usecase.md`, source-and-activity rules | The converter reads any publication that states the required facts; support is no longer a named layout. |
| Main-flow step 2 checks the source against the supported family. | same use case, main flow | Step 2 establishes what the plan states. |
| Extension 2 refuses "not a supported duty-plan export". | same use case, extensions | The refusal names what could not be read or established. |
| A scenario asserts the unsupported-family diagnostic. | `givn/specs/export-duty-plan-calendar/export-duty-plan-calendar.feature:13,485` | The scenario asserts the named reason instead. |
| **Released Roster detail-table family** is the supported source layout. | `docs/arc42/12-glossary.md:95` | The term stays as the description of that layout and stops being the support boundary. |
| The context table names a supported Released Roster PDF as the input. | `docs/arc42/03-context-and-scope.md:16-17,175` | The input is a duty plan as published, with the facts a plan must state. |
| The diagnostic format begins `Source PDF:`. | `crates/duty2cal-core/src/application/mod.rs` (`format!`) | The label names the supplied document, not a format. |
| The rules describe facts as "printed" (crew position, flight number, clocks, qualifiers, tags). | `givn/specs/export-duty-plan-calendar/usecase.md`, rules and examples | "Stated by the duty plan", because a publication need not print anything; no rule's meaning changes. |

## Scope

Inside:

- One intake model that states what a duty plan must provide as the confirmed
  rules require it: for every entry a date and an activity code; for travel the
  route and the crew position with the flight number as the plan states them;
  the clocks and their explicit UTC day qualifiers for timed duty; the
  descriptive fields the rules select; the source's own revision where it
  states one. The model records what the plan states; it does not decide what
  is resolvable, and a plan without revision metadata is not refused for it.
- Reading into that model: today's PDF reading becomes one reading among a
  possible set; its recognition moves from "is this the supported format" to
  "what can be established from this document".
- Interpretation unchanged: every confirmed activity meaning, timing rule,
  completeness rule and calendar guarantee stays as specified, apart from the
  replaced statements listed above.
- Refusal granularity: a refusal names what was recognized and what could not
  be established — an unknown code with the date of its entry, an absent fact
  with its activity, an unmatched fragment, a contradiction — in the
  executable's diagnostic and in the browser page. The diagnostic's source
  label stops claiming a PDF.
- The public statement and the documents that describe support: what a duty
  plan must provide and which publications the evidence covers.

Outside:

- Implementing a second reading. No sample of another publication exists in the
  repository and none was supplied for this topic. Whether a non-PDF
  publication can reach the existing interpretation, or needs a further domain
  reading, is design work and is not claimed here.
- Any named format, family or operator registry; a mapping language;
  per-publication configuration shipped with the tool.
- The site's publication, its Built revision and its page copy, which the
  in-progress change `publish-public-website` owns. This topic changes the
  support sentence about the input, not the site.
- The browser page's file-type filter, as long as no non-PDF reading exists: the
  page keeps accepting PDFs and the statement says so. Widening the filter
  belongs to the change that lands the first non-PDF reading.
- Changing the activity vocabulary, timing rules, completeness rule, RFC 5545
  output, diagnostic shape or the browser route's guarantees.
- Reading contacts, hotel lists, crew databases or any other publication that
  is not a duty plan.
- Calendar-client import, synchronization, a hosted service, or a second
  channel.

## Stakeholders

Stakeholder types, not Persona biographies or Gherkin Actors.

| Stakeholder type | Interest | Concrete example |
|---|---|---|
| Crew member converting their own plan | Their plan converts, or the refusal names the fact or code that stood in the way, so they can tell whether another file from the same system would help. | A plan is refused with "the duty plan states the code S5 on 12 January 2099"; not with a format name. |
| Crew member whose plan already converts | No regression in conversion, calendar content or diagnostic wording they rely on. | Their June plan still converts to the same calendar. |
| Contributor bringing a publication in | A bounded change: read the publication into the existing model, add its fixture, wire the composition. | A contributor adds a reading for a tabular export and does not touch a timing rule. |
| Reviewer of the browser route | The page's refusal text and its file-type filter stay true to what the converter reads. | The page still accepts PDFs only while no non-PDF reading exists, and says so. |
| Release owner | The support statement rests on evidence, not names; the refusal is actionable. | The statement says what a plan must provide and names only publications whose fixtures convert. |
| Reader of the statement without a plan in hand | Can decide whether the tool is worth trying before they have a file. | A colleague reads the statement, sees the required facts, and asks their crew system for the export. |
| Producer of a publication | Their material is not misrepresented, and no code or data table is reused without a licence. | The project describes facts a plan must state; it does not copy a distributor's code tables. |
| People identified in a duty-plan document | A new reading's samples do not put identifying data into the repository. | A contributor inspects a stripped sample locally; the committed fixture is generated. |

## Constraints inherited from confirmed project decisions

- The use-case boundary takes bytes and returns text; no path or handle crosses
  it (chapter 2, and the `application` module doc). The model sits inside that
  boundary.
- One pinned PDF extraction dependency, no native C/C++ extraction dependency,
  one self-contained executable, no network at run time.
- Private PDFs and real identifying values are never committed; public examples
  are generated or anonymized (`export-duty-plan-calendar`). Evidence custody
  therefore has two parts: a real plan is inspected locally and never
  committed; the committed fixture is generated or stripped, and the evidence
  record states what the sample showed without reproducing it.
- Completeness and explicit failure stand: an unknown code, a missing required
  fact, an unresolved fragment or a non-positive interval rejects the
  conversion; the numbered diagnostic and the exit status keep their shape
  apart from the source label named above.
- `SourceError` has two variants today (`Unreadable`, `UnsupportedFamily`) and
  both facades render them. Fact-granular refusals already live in
  `ConvertError`, not in `SourceError`; whether a richer refusal needs a new
  failure class is design work.
- The facades compose the reading: `ConvertRoster::new(PdfRosterSource, …)` in
  `apps/cli` and `ffi/wasm`. Adding a reading therefore touches the
  composition, either in both facades or by moving it inward — that is design
  work, and the Frame does not call the change facade-free.
- The browser route's no-network, no-storage and offer-not-save guarantees are
  untouched.

## Research protocol and evidence

1. Read project state before asking what it answers: `givn status`,
   `givn spec tree`, the permanent conversion use case, the README and the
   architecture chapters on scope, building blocks, runtime and glossary.
2. Read the code that decides intake: `RawRosterTable`, `interpret`, the
   column-label demands, `recognises_family`, both error enums, the diagnostic
   format, and how `apps/cli` and `ffi/wasm` compose and render them.
3. Inventory the public corpus the user pointed at
   (<https://crew2crew.de/pages/downloads.html>): which publications exist and
   what their changelogs say about change. Record observations; do not
   decompile and do not reuse code or data tables.
4. Keep the private corpus out of scope: this topic reads no private PDF and
   records no itinerary.
5. Treat claims about other publications as unverified until a stripped sample
   exists; the corpus shows that variants exist, not what each contains.

## Findings — the public converter corpus

Source: <https://crew2crew.de/pages/downloads.html> and two archives downloaded
from it on 2026-10-25 (`CLH-DPC_1.34.zip`, `4U-DPC_0.56.zip`). The changelog
entries below are from each archive's own `changelog.txt`; the archives are not
retained in the repository, so a later phase must re-download them to re-check
the quotes. Neither archive's own code or data tables are reused.

| Observation | What it shows | What it does not establish |
|---|---|---|
| Seven operator builds are published — CityLine, Passage, Germanwings, Eurowings, AeroLogic, AUA/Tyrolean, Sun Express — as separate programs. | One program per operator is the common shape in this input domain, not one program with a publication switch. | That a single program cannot read several publications: the Passage build's own note says its format detection had to handle Passage and Cargo plans differently, and the CityLine build added LHX rosters. |
| The changelogs are dominated by layout adaptations, including preview-versus-released differences and two repair releases for shipping months. | A publication changes under the same system; a reading is maintained, not written once. | Which specific layout changes will reach this project, or when. |
| The duty-code vocabulary is a data file (`Kürzel;Kurzbeschreibung;Langbeschreibung;Busy Status`), extended release by release. | The code vocabulary is the least stable and most operator-specific part of the input. Our confirmed rule — an unknown code is refused, never guessed — stays. | Any need to move our confirmed codes into data; doing so would contradict the explicit-failure rule. |
| One release added a routine for duty codes the program did not know. | The corpus chose tolerance for unknown codes. | Which reading serves a crew member better; this project refuses. |
| The distributions also read contact lists, a crew database and hotel transport data. | That software is a suite. | Any need for this project to read more than one plan. |
| The site states that the converter writes UTC times and the operating system's timezone produces local times. | An independent implementation reached the same UTC-endpoint choice. | Anything about date association or the overnight rule. |
| The 1.34 archive ships third-party licence texts (Apache-2.0 for PDFBox, font and glyph licences); no licence text covering the distributor's own code or data tables was found in either archive. | The code and the tables are not offered under a reusable licence. | That the files are unlicensed in every jurisdiction. |
| The site advises: "kontrolliert die importierten Daten genau, da sich immer wieder Pläne mit Besonderheiten finden, auf die mein Programm nicht vorbereitet ist". | Unanticipated variants are the normal state of this input domain. | That best-effort partial results are acceptable; this project refuses instead. |

Derivable for the model, reported as inputs rather than decisions:

1. What a reading produces must be the stable thing, not the publication it
   came from — the same system republishes with a changed layout.
2. Preview and released publications are distinct arrangements of the same
   plan, so the model has to record the facts it holds rather than assume them.
3. The printed code vocabulary is the least stable and most operator-specific
   part of the input; the model carries codes as stated, and an unknown code
   stays an explicit refusal.
4. A refusal that names a missing fact is possible only if the model knows
   which facts are required — the model's job, not the reading's.
5. KDE's KItinerary carries a Lufthansa event extractor with autotests; it is a
   public, licensed implementation to consult for format knowledge. Not
   inspected; no code reuse is proposed.

## Findings — current repository state

- Support is decided by three header labels; a document that lacks them is
  refused after the whole document was read.
- `RawRosterTable` holds label strings and cell strings, so a reading can put
  other content into it; what refuses a non-tabular publication today is
  `interpret`'s demand for the labels `Date`, `From` and `To`, not the storage.
- The activity vocabulary is fixed in `ALL_DAY_CODES`, `AVAILABILITY_CODES`,
  `NON_FLIGHT_CODES`; the interpretation refuses an unknown code with the code
  token but no date.
- The diagnostic format always begins `Source PDF:`.
- Both facades construct the reading themselves.
- The README, the organization profile, chapter 3 and the issue form describe
  support as a named format; the issue form offers "Released Roster detail
  table (supported)" and "another export family (unsupported)".
- What a duty plan must provide is stated in the permanent conversion use case,
  not in the public copy, so a refused crew member cannot read it.

## What this Frame does not settle

- The model's exact shape: how it carries fragments, date blocks, day
  qualifiers and absent facts.
- Whether the existing interpretation can consume a non-tabular publication, or
  whether that needs a further domain reading. No non-PDF publication converts
  today.
- Where the composition lives when a second reading exists.
- The wording of each refusal and of the statement.
- Any activity meaning; the vocabulary stays confirmed and unchanged.

## Open questions

None in the Frame. Q7 and Q8 closed the vocabulary; the remaining unknowns are
design work listed above, not disagreements.
