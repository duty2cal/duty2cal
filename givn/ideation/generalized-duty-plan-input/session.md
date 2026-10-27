# Generalized duty-plan input — session state

- Topic: `generalized-duty-plan-input`. Started 2026-10-25 as
  `additional-export-families`; renamed after the input-model decision (Q5)
  dropped the publication-as-family unit.
- Phase: **Handoff published 2026-10-25.** Frame, Personas, Event Storming,
  Use Case Definition and Use Case Elaboration are complete; the human record,
  the vocabulary, the Persona promotion and two ready seeds are published.
- The user directed that only decisions hard to change be brought back for
  confirmation; the cut, the promotions and the seeds were recorded rather than
  asked.
- Nothing here authorizes code, a reader for a second publication, an edit to a
  permanent spec, or a change start.

## Settled input

- **Q1** — any published representation of a duty plan may be supported.
- **Q2 / Q6** — the Frame: first confirmed in a family-centred form, then
  rewritten and re-confirmed after the user withdrew that premise.
- **Q3** — Personas: `crew-member--personal-calendar-owner` and
  `project-maintainer--release-owner` reused, `contributor--publication-reader`
  new.
- **Q4** — withdrawn by the user; the family-naming candidates remain recorded.
- **Q5** — the openness lives in the input model. Rejected: a declared mapping
  language, and a refusal-only generalization.
- **Q7** — `Duty plan` confirmed.
- **Q8** — `Evidence for a publication` and `Support statement` confirmed.

## Published artifacts

- Human record: [docs/ideation/generalized-duty-plan-input.md](../../../docs/ideation/generalized-duty-plan-input.md)
- Frame: [frame.md](frame.md); question log: [questions.md](questions.md)
- Event chain and canvas: [eventstorming.md](eventstorming.md)
- Use cases: [use-case-definitions.md](use-case-definitions.md),
  [use-cases/export-duty-plan-calendar.md](use-cases/export-duty-plan-calendar.md),
  [use-cases/make-a-further-published-duty-plan-convertible.md](use-cases/make-a-further-published-duty-plan-convertible.md),
  [use-cases/state-the-converter-support-statement.md](use-cases/state-the-converter-support-statement.md)
- Vocabulary: [domain-terms.md](domain-terms.md) → three terms in
  [the glossary](../../../docs/arc42/12-glossary.md)
- Personas: [personas/](personas/) → `contributor--publication-reader` promoted
  to [the project collection](../../personas/contributor--publication-reader.md);
  records appended to the two reused project files.
- Seeds: [changes/generalize-duty-plan-intake.md](changes/generalize-duty-plan-intake.md),
  [changes/document-the-publication-evidence-route.md](changes/document-the-publication-evidence-route.md)
- Handoff: [handoff.md](handoff.md)

## Boundary reviews

Two fresh-context reviews ran: one on the Frame, one on the Event Storming.
Both reported blocking findings; all were applied before the Handoff — the
replaced confirmed statements are named, the composition change is
acknowledged, the non-PDF capability is no longer asserted, the refusal outcomes
are split into three, the revision is optional, the evidence route has its own
events, the statement has a shrink path, and copy ownership is fixed against the
in-progress public-website change. Their advisory findings are either applied or
recorded in the documents they concern.

## Next separate workflow

The user runs the normal proposal flow per seed, in the suggested order. The
first seed changes behaviour and copy together; the second adds the evidence
route and the standing statement.

## Research completed (2026-10-25)

- `givn status`: one active change, `publish-public-website`, in progress
  (11 of 109 tasks). It owns the browser route and the Site publication; this
  topic does not touch its tasks.
- `givn spec tree`: six permanent use cases; `export-duty-plan-calendar`
  (91 scenarios) owns the conversion contract, `publish-public-release` the
  platform promise.
- Read `README.md`; arc42 chapters 2, 3, 4, 5, 6, 8, 11, 12; the conversion use
  case; `crates/duty2cal-core/src/application/mod.rs`;
  `crates/duty2cal-pdf/src/lib.rs`; `apps/cli/src/main.rs`;
  `.github/ISSUE_TEMPLATE/bug.yml`; `CONTRIBUTING.md`.
- Read the public corpus at <https://crew2crew.de/pages/downloads.html> and two
  archives (`CLH-DPC_1.34.zip`, `4U-DPC_0.56.zip`): changelogs, `Dienste.txt`,
  `Transpo.txt`, `IATA.csv`, archive listings. No decompilation, no reuse of
  code or data tables.
- One public search surfaced KDE's KItinerary (Lufthansa event extractor with
  autotests) and an LLM-based `crewcal`; leads, not evidence.
