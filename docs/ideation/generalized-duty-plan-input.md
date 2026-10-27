# Generalized duty-plan input

An ideation record. It answers one question: how does the converter stay open
for further duty-plan formats without turning into a registry of formats?

Status: Handoff published 2026-10-25. Two change seeds are ready; no change has
been started. Topic state, the full question log and the collision records live
under `givn/ideation/generalized-duty-plan-input/`.

## The problem

The converter reads exactly one arrangement of a duty plan. Its intake is the
shape of that publication — a PDF reduced to column labels and rows of cell
strings — and the interpretation demands the columns `Date`, `From` and `To`.
Support is decided by three header labels, and every other document ends in one
sentence: "the source is not a supported Released Roster detail-table export".

That message is uninformative in both directions. A crew member cannot tell
whether their document was nearly readable or was something else entirely, and
a contributor cannot see what would have to be provided. The domain's ability
to accept a publication also depends on the publication being a table: a
calendar feed that states the same activities has nowhere to land after
reading.

## What was rejected first

The topic began by modelling each publication as a named *family*, with a
catalogue, per-family evidence and a refusal that names the family. That was
withdrawn by the user: "forget about the family. This was just the first step
towards a generalization."

Two further candidates were rejected on the way:

- **A declared mapping language per publication** — one generic reader plus a
  mapping file and code table per publication. Cheapest to add, but a mapping
  cannot express the timing rules, so the boundary between mapping and code
  becomes the new hard problem, and a wrong declaration looks like support.
- **Generalizing only the refusal** — state what was recognized and what was
  missing, and change nothing else. Truthful, but a second publication still
  needs a reader.

## The direction

The converter states what it must learn from a duty plan — its dated entries,
the codes, routes and clocks it states, and its own revision metadata — as one
model that is independent of how the plan arrives. Every publication is read
into that model before anything is interpreted. Interpretation, the activity
vocabulary, the timing rules, the calendar contract and the facades stay as they
are.

Three consequences follow:

- A publication that states those facts can be read; whether the plan then
  converts still depends on the unchanged rules resolving every required
  activity, date, fragment and interval. **No non-PDF reading exists today**,
  and the Frame does not claim one converts.
- A publication that cannot be read is refused naming what could not be
  established: the unknown code with the date of its entry, the absent route,
  the absent clock, the unmatched fragment, the contradiction.
- Adding a publication is a translation into the model. It changes one reading,
  its fixture and the composition that wires it. It does not change the domain,
  the calendar writer, the facades' behaviour or the release path. The project
  ships no format list and maintains none as configuration; the only place a
  publication is named is the support statement, and it names only what the
  check suite exercises.

This direction replaces confirmed statements — the supported-family rule, the
conversion's step 2, its unsupported-family extension and diagnostic scenario,
the glossary's family entry as a support boundary, chapter 3's input sentence,
and the diagnostic's `Source PDF:` label. The Frame names each with its path;
the changes rewrite them deliberately.

## What the public converter corpus shows

The user pointed at the converters published at
<https://crew2crew.de/pages/downloads.html>. Two archives and the page's
changelogs were inspected on 2026-10-25; the Java archives were not
decompiled, and no code or data table from them is reused.

| Observation | What it means for the design |
|---|---|
| Seven operator builds are published — CityLine, Passage, Germanwings, Eurowings, AeroLogic, AUA/Tyrolean, Sun Express — as separate programs. | One program per operator is the common shape, not one program with a publication switch — though a single build does handle several publications: the Passage build's own note says its format detection had to treat Passage and Cargo plans differently, and the CityLine build added LHX rosters. |
| Every changelog is dominated by layout adaptations, including preview-versus-released differences and two repair releases for shipping months. | A publication is not stable; what a reading produces has to be the stable thing. |
| The duty-code vocabulary is a data file (`Kürzel;Kurzbeschreibung;Langbeschreibung;Busy Status`), extended release by release. | The code vocabulary is the least stable and most operator-specific part of the input. Our confirmed rule — an unknown code is refused, never guessed — stays. |
| One release added a routine for duty codes the program did not know. | The corpus chose tolerance; this project chooses explicit refusal. |
| The distributions also read contact lists, a crew database and hotel transport data. | That software is a suite. This project reads one duty plan. |
| The site states that the converter writes UTC times and the operating system's timezone produces local times. | An independent implementation reached the same UTC-endpoint choice. |

The changelog quotes come from each archive's own `changelog.txt`. The archives
are not retained in the repository; a later phase re-downloads them to re-check
a quote. No code or data table from them is reused.


## The two new use cases

| Use case | Actor | Goal | Outcome |
|---|---|---|---|
| Make one further published duty plan convertible | Contributor bringing a publication in | Make one further published duty plan convertible | The publication's duty plans convert, its fixture is exercised by the checks, and no domain rule moved. |
| Keep the public support statement true to the exercised evidence | Release owner | Keep the public support statement true to the exercised evidence | The public documents state what a duty plan must provide and name exactly the publications the evidence covers. |

The existing use case, *Export one published duty plan as an importable
personal calendar file*, is extended, not replaced: its second step becomes
"establish what the plan states" instead of "check the format", and its
refusal names the code or fact instead of a format.

## The evidence route

A claim that the converter handles a publication rests on **evidence for a
publication**: a stripped sample of that publication, the facts that sample
states, and a fixture built from it that converts. The landed reading is not
evidence. A sample is stripped of identifying data — including combinations of
base, dates and trip identifiers, not only names.

A reading that would need a new activity meaning or a new timing convention is
not landed as part of bringing a publication in; that is a separate domain
decision.

## Open design work

The Frame deliberately leaves these to design, and no seed may settle them by
accident:

- The intake model's exact shape: how it carries fragments, date blocks, day
  qualifiers and absent facts, without weakening the confirmed timing rules.
- How a reader for a non-tabular publication (a calendar feed) reaches the
  interpretation, if the current interpretation cannot consume it.
- The wording of each refusal and of the statement.

## Seeds

1. **Generalize the input model and name what is missing** — the model, the
   reading, the fact-granular refusal, and the public copy that describes the
   input, so nothing the project publishes is untrue. No dependency.
2. **Document the evidence route** — the evidence standard, the contributor
   route, the issue form, and the support statement as a standing claim.
   Depends on the first: the facts the statement names come from the model.

## Vocabulary

Three terms were confirmed: **Duty plan** (the published schedule for one crew
member, not the calendar derived from it), **Evidence for a publication** (the
stripped sample, the facts it states and the converting fixture), and **Support
statement** (the public statement of what a duty plan must provide and which
publications the evidence covers). The intake record deliberately has no term:
it is the anti-corruption target every reading fills, not a domain noun.
