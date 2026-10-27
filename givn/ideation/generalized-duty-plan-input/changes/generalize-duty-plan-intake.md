# Change seed: generalize-duty-plan-intake

Status: ready

- Use-case IDs: `export-duty-plan-calendar` (extended; its permanent leaf
  exists and is not duplicated)
- Topic: `generalized-duty-plan-input`
- Personas: `crew-member--personal-calendar-owner`,
  `contributor--web-reviewer` (the browser page's refusal wording),
  `contributor--publication-reader` (what counts as establishing a fact)
- Suggested start: 1; no dependency.
- Handoff: ready, published 2026-10-25. The user directed that only decisions
  that are hard to change be brought back for confirmation; this seed and its
  records are reversible by editing the repository.

## Accepted decisions

### The openness lives in the intake model

- Context: support is decided by three header labels and the intake is one
  publication's column-and-row shape; a second publication means a second
  reader around a second shape, and a non-tabular publication has nowhere to
  land. The user rejected modelling publications as named families and directed
  that the first step was towards a generalization.
- Options:
  1. **Generalize the input model** — one model states what a duty plan must
     provide; every publication is read into it before interpretation.
     In practice: an unmappable plan is refused with "the duty plan states the
     code S5 on 12 January 2099, which the converter does not understand", and
     a tabular export with declared headers converts without a domain change.
     Gain: a new publication is a translation, not an identity to maintain.
     Cost: the completeness rule becomes field-granular, and the evidence
     standard moves to per-fact and per-code.
  2. **Generalize the reader by declared mapping** — a mapping file per
     publication.
     Gain: the cheapest addition.
     Cost: a second language whose boundary with the timing rules is the new
     hard problem; a wrong declaration looks like support.
  3. **Generalize only the refusal** — keep one reader, say what was missing.
     Gain: small and truthful.
     Cost: a second publication still needs code.
- Chosen: **Generalize the input model** because it is the only option that
  makes a further publication a translation while the confirmed domain stays
  the fixed point. Rejected: **declared mapping** because the rule boundary
  moves into data; **refusal only** because it is honesty without openness.

### Any published representation may be supported

- Context: the converter reads PDFs; the user's request named "formats"
  generally, and an intermediate already independent of PDF positions exists.
- Options:
  1. **Any published representation** — PDF, tabular export or calendar feed.
     In practice: a CSV with `Date;Code;From;To;Start;End` converts; an
     event-list feed may, once the model admits it.
     Gain: reach beyond PDF.
     Cost: the support statement and the evidence standard must cover more than
     PDF extraction.
  2. **PDF layouts only** — narrow, matching the evidence in hand.
     Gain: the release story stays one input surface.
     Cost: a crew member whose system exports CSV gets nothing, and reopening
     the scope later changes the Frame again.
- Chosen: **Any published representation** because the user asked for maximum
  openness and a second scope decision later would be more expensive.
  Rejected: **PDF only** because it closes a door the model can keep open.

### Reading establishes; interpretation refuses

- Context: the permanent contract already refuses an unknown code, a missing
  required endpoint and an unmatched fragment during interpretation, with a
  numbered diagnostic. The generalization must not relocate those rules.
- Options:
  1. **Two stages** — reading records what the plan states; interpretation
     refuses what it cannot resolve, naming the code or the absent fact.
     In practice: an unknown code is rejected at interpretation with its date;
     unreadable bytes are rejected at reading.
     Gain: the confirmed contract and its scenarios keep their shape; only the
     reason becomes specific.
     Cost: two refusal paths to keep distinct in code and text.
  2. **One stage** — reading validates everything and refuses directly.
     Gain: one path.
     Cost: the rule set moves into the reader, and a second reader would
     duplicate the domain.
- Chosen: **Two stages** because the domain must stay the only place that
  decides what is resolvable. Rejected: **One stage** because it would put
  domain rules into every reader.

### "Printed" becomes "stated"

- Context: the confirmed rules speak of printed cells because the only
  publication was a PDF. The generalized model admits representations that
  state the same facts without printing them.
- Options:
  1. **Re-word the rules** — "as the duty plan states it" replaces "as printed";
     no meaning, timing or completeness rule changes.
     In practice: the travel title still carries the crew position and flight
     number as the plan states them, leading zeroes kept.
     Gain: the contract stops assuming one representation.
     Cost: a wording change across the permanent leaf and its scenarios.
  2. **Keep "printed"** — fewer edits.
     Gain: no wording churn.
     Cost: the contract contradicts the model it now rests on.
- Chosen: **Re-word** because the wording would otherwise be false for a
  non-PDF publication. Rejected: **Keep "printed"** for that contradiction.

### No aggregate, no registry, no term for the intake

- Context: the project has exactly one aggregate, the Duty-plan calendar, and
  disallows records, specifications and services drawn as aggregates. The user
  rejected the publication as the organizing unit.
- Options:
  1. **Record inside the existing boundary** — the intake is a record; the
     refusal is a reason on the existing rejection; no format list and no
     domain term for the intermediate.
     Gain: nothing new to maintain; the aggregate keeps its invariant.
     Cost: the intermediate's name stays a design decision.
  2. **A new aggregate or a declared catalogue** — e.g. a supported-source
     catalogue with its own lifecycle.
     Gain: a place for per-publication state.
     Cost: a second consistency boundary for data that is evidence, not domain
     state; and it reintroduces the registry the user rejected.
- Chosen: **Record inside the existing boundary**. Rejected: **new aggregate or
  catalogue**.

### The public copy is corrected in this change

- Context: the README, chapter 3, chapter 6 and the issue form describe support
  as a named format; the refusal text is quoted in the release notes. After the
  model lands, those sentences would describe a converter that no longer
  exists.
- Options:
  1. **Correct the input wording here** — the input is described in
     required-facts terms, the refusal names what could not be established, the
     diagnostic's source label stops claiming a PDF, and the statement names the
     publication whose evidence exists. The site's page copy stays with the
     change that publishes the site.
     Gain: no interval in which the project publishes an untrue claim about its
     input.
     Cost: prose work in the same change as the code.
  2. **Defer the copy to the second seed** — ship the behaviour first.
     Gain: a smaller code change.
     Cost: the public claim would be false until the second change, which the
     truthful-copy goal forbids.
- Chosen: **Correct it here**. Rejected: **Defer** for the truthfulness
  interval it creates.

### The composition changes; the facades' behaviour does not

- Context: both driving adapters construct the reading themselves
  (`ConvertRoster::new(PdfRosterSource, …)` in `apps/cli` and `ffi/wasm`). A
  second reading must be reachable from both, and the Frame no longer claims the
  change is facade-free.
- Options:
  1. **Compose internally** — the application selects the reading for the
     supplied document; the facades keep passing bytes and receiving text.
     In practice: neither facade names a reading, and the browser and the
     executable accept what the converter can read.
     Gain: one place decides the reading; adding one touches no adapter.
     Cost: the application gains a selection responsibility it did not have.
  2. **Compose in both facades** — each adapter names the readings it wires.
     Gain: no new application responsibility.
     Cost: every new reading edits both adapters, and the browser can fall
     behind the executable without a test noticing.
- Chosen: **Compose internally** because the boundary takes bytes and returns
  text and a caller cannot know which reading applies before the bytes are
  read. Rejected: **compose in both facades** because it duplicates a decision
  and lets the routes diverge.

### The diagnostic names the supplied document, not a format

- Context: the diagnostic format is `Source PDF: <name>` today, and a non-PDF
  publication is admissible in principle. The format is quoted in the README
  and asserted by scenarios.
- Options:
  1. **Relabel to the supplied document** — the line names the document and its
     filename, without claiming a format.
     In practice: `Source: roster.csv` where a CSV was supplied, and the same
     line for a PDF.
     Gain: the diagnostic stops making a false statement about the source.
     Cost: every golden transcript and the README's example change with it.
  2. **Keep `Source PDF:`** — fewer edits.
     Gain: no transcript churn.
     Cost: a permanent diagnostic that lies about a non-PDF source, in the one
     artifact a crew member keeps after a failure.
- Chosen: **Relabel**. Rejected: **keep** for the false statement it persists.

### The browser page keeps accepting PDFs for now

- Context: the page's file input accepts `application/pdf,.pdf`. No non-PDF
  reading exists, and the page's publication and copy belong to the in-progress
  change `publish-public-website`.
- Options:
  1. **Keep the filter, and say why** — the page accepts what the converter can
     read today; the filter widens with the first non-PDF reading.
     In practice: a crew member with a CSV is told by the page that it takes
     PDFs, which is true.
     Gain: no untrue claim about the browser route, no collision with the
     active change.
     Cost: a second change touches the page when the first non-PDF reading
     lands.
  2. **Widen the filter now** — accept any file.
     Gain: the route looks ready for anything.
     Cost: the page would advertise a capability no reading backs, and it edits
     copy the active change owns.
- Chosen: **Keep the filter and say why**. Rejected: **widen now**.

### The confirmed statements this change replaces

Named so the change rewrites them deliberately; the Frame's table lists them
with their paths: the supported-family rule, main-flow step 2, extension 2's
wording, the scenario asserting the unsupported-family diagnostic, the
glossary's family entry as a support boundary, chapter 3's input sentence, and
the diagnostic's `Source PDF:` label.

## Open questions

None that block the change. The intake model's shape, the reader's internal
boundary and the exact refusal sentences are design work for this change; they
must not weaken a confirmed timing, classification or completeness rule. The
evidence route and the standing statement belong to the second seed.
