# Generalized duty-plan input — domain vocabulary

Status is `confirmed`, `candidate` or `rejected`. A confirmed entry carries its
genus-and-differentia definition, a contrast pair, one concrete domain sentence
and a recorded collision check. `Duty plan` was confirmed on 2026-10-25 in the
Event Storming term check (Q7); `Evidence for a publication` and `Support
statement` were confirmed the same day in the Use Case Definition term check
(Q8).

| Term | Status | Definition | Contrast | First use | Collision check |
|---|---|---|---|---|---|
| Duty plan | confirmed | The schedule a crew member's crew scheduling system publishes for one period, assigning dated activities to that crew member together with the facts each activity states. | Duty plan, not the personal calendar: the published schedule versus the calendar derived from it. Not a file format, and not the crew scheduling system that produces it. | frame.md, Problem; the earlier topic held it as `candidate`. | 2026-10-25, before confirmation: `grep(pattern="(?i)duty plan", path="docs/arc42/12-glossary.md")` → 9 lines, all uses inside other definitions (Browser conversion, Report time, Duty-plan calendar, Profile content, Default security policy, Built revision) and the event `Duty plan supplied`, no entry of its own; `grep(pattern="(?i)duty plan", path="README.md CONTRIBUTING.md docs")` → 37 lines, all denoting the published schedule; the earlier topic's `domain-terms.md` holds `Duty plan | candidate | … | Pending before confirmation` with no competing definition. |
| Evidence for a publication | confirmed | The set of artifacts on which a claim about one publication rests: a stripped sample of it, the facts that sample states, and a fixture built from it that converts. | Evidence for a publication, not the landed reading: code proves nothing about a publication unless a fixture converts. Not the fixture alone: the sample decides whether the fixture proves anything. Not release evidence, which backs a version's build. | eventstorming.md, event `Publication evidence supplied`. | 2026-10-25, before confirmation: `grep(pattern="(?i)evidence", path="docs/arc42/12-glossary.md")` → 13 occurrences, every one generic and none defining a term (release evidence, site evidence, size-lever evidence); `grep(pattern="(?i)evidence for a publication", path="docs/arc42/12-glossary.md")` → 0 lines; no sibling topic defines an evidence noun. |
| Support statement | confirmed | The public statement of what a duty plan must state for conversion and which publications the exercised evidence covers. | Support statement, not the Release text: a standing claim about capability versus one version's change notes. Not the Profile content, which renders it among other statements, and not the issue form, which asks for a sample. | eventstorming.md, event `Support statement published`. | 2026-10-25, before confirmation: `grep(pattern="(?i)statement", path="docs/arc42/12-glossary.md")` → the footer's `data-site-statement` element in Built revision and the Public website's "public statements", neither defining a term; `grep(pattern="(?i)support", path="docs/arc42/12-glossary.md")` → the confirmed **Supported platform** plus "supported release"/"supported family" prose, which qualify a platform or a source rather than a statement; no entry for a support statement anywhere. |

## Confirmed duty plan

- Genus and differentia: a schedule (genus) that a crew scheduling system
  publishes for one period and that assigns dated activities to one crew
  member, each activity carrying the facts the producing system states (code,
  route, clocks, descriptive fields).
- Contrast: the personal calendar is derived from it; the crew scheduling
  system produces it; the representation is how it arrives, not what it is.
- Concrete domain sentence: "The duty plan for June states a standby on
  12 June from 08:00 to 16:00; the calendar represents it as one timed event,
  and the plan is not the calendar."
- Verified name: confirmed by the user on 2026-10-25 (Q7).

## Confirmed evidence for a publication

- Genus and differentia: a set of artifacts (genus) on which a claim about one
  publication rests, consisting of a stripped sample of that publication, the
  facts the sample states, and a fixture built from it.
- Contrast: the landed reading is not evidence; the fixture alone is not
  evidence, because the sample decides whether it proves anything; release
  evidence backs a version's build, not a publication's readability.
- Concrete domain sentence: "The evidence for the July publication is a
  stripped sample of its detail page, the dates and codes that page states, and
  the fixture that converts it; the landed reading on its own would prove
  nothing."
- Verified name: confirmed by the user on 2026-10-25 (Q8).

## Confirmed support statement

- Genus and differentia: a public statement (genus) that names what a duty plan
  must state for conversion and which publications the exercised evidence
  covers, and that changes only when that evidence changes.
- Contrast: the Release text reports one version's changes; the Profile content
  renders the statement among other statements; the issue form asks for a
  sample; none of them is the statement itself.
- Concrete domain sentence: "The support statement says a duty plan must state
  a date, a code and, for timed duty, a start and an end; it names the June
  publication because its fixture converts, and it names no other."
- Verified name: confirmed by the user on 2026-10-25 (Q8).

## Candidate not used as confirmed

None. The intake record (the facts a duty plan states) deliberately has no
term: it is the anti-corruption target every reading fills, not a domain noun.
