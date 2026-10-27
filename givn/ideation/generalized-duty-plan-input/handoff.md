# Generalized duty-plan input — Handoff

Published 2026-10-25. The user directed that only decisions that are hard to
change be brought back for confirmation, so this Handoff records the cut, the
promotions and the seeds rather than asking for them. Everything here is
reversible by editing the repository.

## What this topic concluded

The converter is opened by generalizing **what it learns from a duty plan**, not
by naming publications. One intake model states what a duty plan must provide;
every publication is read into it before interpretation; the confirmed rules,
the calendar contract and the facades are unchanged. A refusal names the code or
the fact that could not be established. Adding a publication is a translation
plus a fixture, and the project maintains no format registry.

Rejected on the way: the named publication family (withdrawn by the user), a
declared mapping language, and a refusal-only generalization.

## Use-case mapping

| Use-case id | Level | Primary actor | Permanent leaf |
|---|---|---|---|
| `export-duty-plan-calendar` | `!` | Crew member | Exists at `givn/specs/export-duty-plan-calendar/`; this topic **extends** it and creates no second leaf. |
| `make-a-further-published-duty-plan-convertible` | `!` | Contributor bringing a publication in | New leaf, created by the second seed. |
| `state-the-converter-support-statement` | `!` | Release owner | New leaf, created by the second seed. |

No `+` summary use case exists: no goal's flow names three distinct actor goals
and no success guarantee is a conjunction of separate outcomes.

## Vocabulary promoted

Three confirmed terms, now in the durable glossary
(`docs/arc42/12-glossary.md`):

- **Duty plan** — the published schedule for one crew member, not the calendar
  derived from it.
- **Evidence for a publication** — the stripped sample, the facts it states and
  the converting fixture; the landed reading is not evidence.
- **Support statement** — the public statement of what a duty plan must provide
  and which publications the evidence covers.

The intake record has no term on purpose: it is the anti-corruption target every
reading fills, not a domain noun.

## Personas

- `crew-member--personal-calendar-owner` — reused; stance and review entry
  appended to the project file.
- `project-maintainer--release-owner` — reused; stance and review entry
  appended.
- `contributor--publication-reader` — new segment (confirmed 2026-10-25, Q3 as
  `contributor--source-family-contributor`, renamed with the unit); promoted to
  `givn/personas/`.
- The stakeholder type "people identified in a duty-plan document" keeps no
  Persona; its interest is enforced by the evidence standard.

## Seeds

| Seed | Use cases | Start | Depends on |
|---|---|---|---|
| `generalize-duty-plan-intake` (withdrawn 2026-10-27) | `export-duty-plan-calendar` (extended) | — | — |
| `document-the-publication-evidence-route` (superseded by `state-the-supported-duty-plan`) | `make-a-further-published-duty-plan-convertible`, `state-the-converter-support-statement` | — | withdrawn with the intake |
| `state-the-supported-duty-plan` | `state-the-converter-support-statement`, `publish-public-website` (extended), `apply-configuration-change` (extended) | 1 | — |

The general intake and its further-publication route were withdrawn by the
operator on 2026-10-27: too many converters already exist, and a broader
converter would need duty-plan examples beyond the Lufthansa cabin plans. The
project now names the one plan it converts; the support statement and the
evidence route for that plan live in `state-the-supported-duty-plan`.

## Open design work, not questions

- The intake model's shape: fragments, date blocks, day qualifiers, absent
  facts. It must not weaken a confirmed timing, classification or completeness
  rule.
- Whether the current interpretation can consume an event-list publication, or
  whether that needs a further reading.
- The exact refusal sentences and the statement's wording.

## Review corrections applied

A fresh-context Frame review ran at the Frame boundary and its blocking
findings were applied before this Handoff:

- The Frame now names the confirmed statements the direction replaces, with
  their paths, instead of claiming that every confirmed rule stands.
- It no longer claims the change is facade-free: both facades compose the
  reading today, so the composition changes.
- It no longer asserts that a non-PDF publication converts; no such reading
  exists, and whether the current interpretation can consume one is design work.
- The Problem states what the refusals actually say today, including the
  already fact-shaped ones and the diagnostic's `Source PDF:` label.
- The support statement no longer reads as a registry: the project ships no
  list, and the statement derives its publication names from the check suite.
- Copy ownership is fixed: the site's page copy belongs to the in-progress
  public-website change, and the second seed waits for it.
- The corpus table attributes its quotes to each archive's `changelog.txt`,
  records that the archives are not retained, and drops the claim that a build
  reads exactly one publication.

A second fresh-context review covered the Event Storming, and its blocking
findings were applied too:

- The intake now has three outcomes — bytes unreadable, not established as a
  duty plan, facts established — so a readable-but-unmapped document cannot
  reach interpretation with facts missing.
- The source revision is optional in the intake; a plan without it converts
  with the construction-time fallback.
- The statement's shrink path has an event (`Publication evidence failed`), and
  landing the reading has its own event between acceptance and exercise.
- The evidence panel states why it holds no aggregate, and its command targets
  are named as records.
- The diagnostic's document identity and the replaced `Source PDF:` label are
  modelled, and the support statement is a record carried by four named
  documents, read by the crew member as well as the contributor.
- The intake record names the crew position, the flight number and the explicit
  UTC day qualifiers, so a reading cannot claim conformance while dropping
  them.
- The "printed → stated" wording change is now listed in the Frame's replaced
  statements instead of being presented as a mere consequence.

## What this Handoff does not authorize

No change is started, no code or fixture is written, no permanent spec leaf is
edited. The proposal flow per seed is the user's next separate action.
