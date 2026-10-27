# Generalized duty-plan input — Use Case Definitions

Status: cut recorded for the confirmed Frame (Q6) and the Event Storming. The
phase term check is closed (Q8): `Evidence for a publication` and `Support
statement` are confirmed terms. The cut is published with the Handoff; the
Personas reviewed it and their findings are advisory.

## The cut

| Use-case id | Level | Primary actor | Goal (one active-verb phrase) | Value | Mutation |
|---|---|---|---|---|---|
| `export-duty-plan-calendar` | `!` | Crew member | Export one published duty plan as an importable personal calendar file. | A calendar of the plan, or an explicit refusal naming what could not be established. | Changing: publishes or offers one calendar. |
| `make-a-further-published-duty-plan-convertible` | `!` | Contributor bringing a publication in | Make one further published duty plan convertible. | A publication that was refused now converts, with the domain rules unchanged and its fixture exercised by the checks. | Changing: lands one reading and one fixture in the repository. |
| `state-the-converter-support-statement` | `!` | Release owner | Keep the public support statement true to the exercised evidence. | A reader learns what a duty plan must state and which publications the evidence covers. | Changing: publishes the statement in the public documents. |

`export-duty-plan-calendar` already owns a permanent leaf; this topic extends
that leaf and creates no second conversion goal. The other two are new goals
with no permanent leaf yet.

## Sizing checks

1. **One goal, one primary actor.**
   - `export-duty-plan-calendar`: one outcome, one actor; the refusal is not a
     second outcome but the same goal's minimal guarantee.
   - `make-a-further-published-duty-plan-convertible`: one outcome — that
     publication converts. Landing code is not a second outcome; it is how the
     outcome is reached.
   - `state-the-converter-support-statement`: one outcome — the statement
     matches the exercised evidence. "Required facts" and "evidenced
     publications" are the two halves of one claim, not two goals: a statement
     naming facts without evidence, or evidence without facts, would be false.
2. **Goal-shaped, never component-shaped.** No goal names a reader, an adapter,
   a port, a page or a file format. "Publication" is the crew member's word for
   what their system hands them; it is not a component.
3. **Level discipline.** All three are `!` and own their capabilities. No `+`
   exists in this cut: no goal's flow names three distinct actor goals, and no
   success guarantee is a conjunction of separate outcomes.
4. **Mutation containment.** Each case states whether it changes. The
   observation-only variant of the conversion (a browser page that offers but
   writes nothing) stays inside `export-duty-plan-calendar` — it is the same
   goal through another channel, not a second use case.
5. **Extension discipline.** The conversion's existing extensions stay; this
   topic adds none. The refusal reason changes the message content, not the
   branch.
6. **Precondition and flow consistency.** `make-a-further-published-duty-plan-convertible`
   requires a publication the converter refuses and a sample the contributor
   may strip; neither is established by its own flow. Its success guarantee —
   the plan converts and the fixture is exercised — is established by the flow,
   not assumed.
7. **Implementation independence.** Goal, trigger, flow and guarantees stay true
   if the converter were rebuilt: nothing in them names Rust, PDF extraction,
   WASM or a table. "Printed" becomes "stated" throughout, which is exactly the
   change that makes this independence true.
8. **Relationship semantics.** No `Includes` and no `Extends` between the three;
   the graph stays acyclic. `state-the-converter-support-statement` consumes the
   outcome of the other two but does not include them.

## Boundaries and vocabulary

- The **publication** is deliberately not a term. "Publication" already means
  the act of publishing in this project (**Site publication**, **Publication
  gate**, the published site, the published release), so coining it as a noun
  for a document form would collide with confirmed vocabulary. The artifacts say
  "the publication" in prose, meaning the representation a crew member's system
  hands them, and never as a domain noun.
- **Duty plan** is confirmed (Q7). The conversion's subject is the duty plan,
  not the publication.
- Term collisions for the two candidates are recorded in `domain-terms.md`.

## Personas per use case

Personas are review lenses, not Gherkin Actors.

| Use case | Confirmed lenses |
|---|---|
| `export-duty-plan-calendar` | `crew-member--personal-calendar-owner` (accuracy, privacy, and whether a refusal is actionable); `contributor--web-reviewer` only for the browser page's refusal text and offer behaviour, which this topic does not change beyond wording. |
| `make-a-further-published-duty-plan-convertible` | `contributor--publication-reader` (does the reading establish exactly the required facts?); `project-maintainer--release-owner` (is the evidence sufficient for the claim?). |
| `state-the-converter-support-statement` | `project-maintainer--release-owner`; `crew-member--personal-calendar-owner` (does the statement answer "will my plan work?"). |

## What is deliberately not a use case

- **Read a publication into the facts a duty plan states.** It is a
  subfunction of the conversion: no actor stops there, and its success is not
  observable on its own. It remains a step of `export-duty-plan-calendar`.
- **Review the evidence.** A step of
  `make-a-further-published-duty-plan-convertible`, performed by the release
  owner as a supporting actor. Its independent value is the statement, which is
  the third use case.
- **Withdraw a publication from the statement.** No actor goal was evidenced: a
  publication that stops converting is refused by the conversion with a named
  reason, and the statement is corrected under the third use case. A separate
  goal would only restate those two.
- **Choose the intake model's shape.** Design work, not an actor goal.

## Open questions

None. The phase term check closed (Q8); the cut was reviewed at the boundary
and published with the Handoff; the remaining unknowns are design work named in
the Frame.
