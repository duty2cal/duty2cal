# Change seed: document-the-publication-evidence-route

Status: superseded by state-the-supported-duty-plan

- Use-case IDs: `make-a-further-published-duty-plan-convertible`,
  `state-the-converter-support-statement`
- Topic: `generalized-duty-plan-input`
- Personas: `project-maintainer--release-owner` (does the claim rest on
  evidence?), `contributor--publication-reader` (is the route usable and
  bounded?), `crew-member--personal-calendar-owner` (does the statement answer
  "will my plan work?")
- Suggested start: 2; depends on `generalize-duty-plan-intake` — the facts the
  statement names come from the generalized model, and the statement cannot be
  written before the behaviour it describes exists.
- Handoff: ready, published 2026-10-25. The user directed that only decisions
  that are hard to change be brought back for confirmation; this seed and its
  records are reversible by editing the repository.

## Accepted decisions

### A claim rests on evidence for a publication

- Context: the project already asks contributors for "the export family" and a
  "stripped sample", but nothing states what makes a sample sufficient, and
  the previous model wanted a named family that the user rejected.
- Options:
  1. **Evidence is sample, facts and a converting fixture** — the claim exists
     only when a fixture built from the stripped sample converts through the
     unchanged conversion.
     In practice: a contributor supplies two stripped days of a tabular export,
     the dates and codes they state, and a fixture; the fixture converts, so
     the publication may be named.
     Gain: the claim is testable in the same suite as everything else.
     Cost: every named publication costs a fixture, and a sample that shows too
     little is returned.
  2. **Evidence is a stripped sample alone** — the reviewer judges by reading.
     Gain: cheaper for the contributor.
     Cost: the claim rests on a human impression; nothing prevents a
     publication being named that never converts end to end.
  3. **Evidence is the landed reading** — code review decides.
     Gain: matches ordinary engineering review.
     Cost: code that looks right proves nothing about a publication.
- Chosen: **Sample, facts and a converting fixture** because the project's whole
  posture is that a claim ships with its evidence. Rejected: **sample alone**
  and **landed reading** for the proof they lack.

### The statement reports evidence, never intent

- Context: the public documents currently describe support as a named format.
  The generalized model makes support a property of what a plan states, so a
  format list would be both untrue and unmaintainable.
- Options:
  1. **Required facts plus evidenced publications** — the statement says what a
     duty plan must provide and names exactly the publications whose fixtures
     convert.
     In practice: "a duty plan must state a date and an activity code, a route
     for travel, and a start and an end for timed duty", followed by the one
     publication whose fixture converts.
     Gain: a crew member can answer "will my plan work?" from the statement.
     Cost: the statement is longer than a format name and must be revised when
     evidence changes.
  2. **Keep a format list** — name the layouts the project supports.
     Gain: familiar shape.
     Cost: the user rejected the publication as the unit; the list would be a
     registry by another name and would rot with every layout change.
  3. **Say nothing about support** — document only what the tool does.
     Gain: nothing to maintain.
     Cost: the crew member cannot tell whether their plan is in scope, and the
     issue form still asks for an "export family".
- Chosen: **Required facts plus evidenced publications**. Rejected: **format
  list** because it reinstates the rejected unit; **say nothing** because it
  leaves the crew member without an answer.

### One statement across every document that carries it

- Context: the README, the organization profile, chapter 3 and the issue form
  each state the support boundary today, in different words. A statement that
  disagrees with itself is worse than none.
- Options:
  1. **One wording, checked**: the four carriers are edited together, and a
     change that leaves them disagreeing is refused.
     In practice: a pull request that updates the README alone is refused by
     the content check.
     Gain: a reader meets the same claim wherever they look.
     Cost: every statement change is a four-file change, and the profile is
     published through the declared configuration rather than edited by hand.
  2. **README is authoritative, others link to it**: one editable source.
     Gain: one place to edit.
     Cost: the profile and the issue form cannot link meaningfully to a section
     a contributor must find; GitHub renders the profile as the organization's
     front page.
- Chosen: **One wording, checked** because the profile is the first thing a
  colleague reads. Rejected: **README authoritative** because the other carriers
  cannot defer to it cleanly.

### The issue form asks what happened, not which family

- Context: the form's field offers "Released Roster detail table (supported)",
  "another export family (unsupported)" and "not sure". Under the generalized
  model those options name a unit the project no longer uses.
- Options:
  1. **Ask by outcome** — "did the plan convert?", "what did the refusal say?",
     plus the stripped sample.
     In practice: the reporter pastes the named code or fact the refusal
     reported, and attaches a stripped sample.
     Gain: the report carries what the evidence route needs, and a reporter is
     never asked to classify their own plan into a taxonomy.
     Cost: an issue no longer says at a glance whether the plan was in scope.
  2. **Keep the family dropdown** — familiar to existing reporters.
     Gain: a quick classification.
     Cost: the form teaches a unit the domain does not use and the project no
     longer names.
- Chosen: **Ask by outcome**. Rejected: **keep the dropdown**.

### A publication's support is withdrawn by evidence, not by announcement

- Context: the corpus shows publications changing under the same system, so a
  named publication can stop converting.
- Options:
  1. **The statement shrinks when evidence fails** — a publication whose
     fixture no longer converts, or whose plan is refused, leaves the statement;
     the conversion's refusal names what could not be established.
     In practice: a layout change makes a plan refuse with the unknown code or
     absent fact; the statement loses that publication in the next revision.
     Gain: the claim never outlives its evidence; the crew member learns what
     changed.
     Cost: a publication can leave the statement between releases, which the
     release text does not announce.
  2. **Withdraw only at a release** — the statement changes with the version.
     Gain: every change is announced.
     Cost: between releases the statement names a publication that no longer
     works.
- Chosen: **Shrinks when evidence fails**. Rejected: **release-only withdrawal**
  for the untruthful interval.

### A new activity meaning stays a separate decision

- Context: bringing a publication in may expose codes the confirmed vocabulary
  does not know. The corpus solved this by extending a data table per release;
  this project refuses unknown codes.
- Options:
  1. **Separate domain decision** — a reading may only use confirmed codes; an
     unknown code is refused at conversion, and giving it a meaning needs its
     own change with its own evidence.
     In practice: a publication whose plans use `S5` converts only after `S5`
     has been given a meaning and a fixture that exercises it.
     Gain: no code meaning enters through a reader.
     Cost: taking on such a publication takes two changes.
  2. **The reading carries a code table** — per-publication code meanings in
     the reading.
     Gain: one change instead of two.
     Cost: two places decide what a code means, and the calendar representation
     would depend on which publication the plan came from.
- Chosen: **Separate domain decision**. Rejected: **per-reading code table**.

### Copy ownership and ordering

- Context: the site's publication, its Built revision and its page copy belong
  to the public-website use case, and the change that builds that site is in
  progress (11 of 109 tasks). The support statement lives in the README, the
  organization profile, chapter 3 and the issue form — carriers the site work
  also touches.
- Options:
  1. **Sequence after the site change** — this seed changes only the support
     sentence about the input, waits for `publish-public-website` to land, and
     never edits page copy.
     In practice: the README and the issue form are edited here; the profile is
     published through the declared configuration; the page's own statements
     stay with the site change.
     Gain: one owner per sentence, no merge collision on public copy.
     Cost: this seed starts later.
  2. **Edit the sentences now** — the statement is correct sooner.
     Gain: a shorter untruthful interval.
     Cost: two changes edit the same files concurrently, and the site change
     may revert or duplicate the wording.
- Chosen: **Sequence after the site change** because the site change already
  owns the page's copy and pinning two owners on one sentence is how statements
  drift apart. Rejected: **edit now** for the collision.

### Evidence custody

- Context: privacy forbids committing a real duty plan or identifying values,
  while the evidence route needs a sample to judge. The two must not be
  confused in the contributor's instructions.
- Options:
  1. **Two artifacts, one rule** — the contributor inspects a real plan locally
     and never supplies it; the evidence record states what the sample showed
     (dates, codes, fields) without reproducing it; the committed fixture is
     generated or stripped.
     In practice: the pull request carries a generated fixture and a sentence
     such as "a 31-day detail table with codes SB, OFF and TK", not the plan.
     Gain: the claim is reviewable and nothing identifying is committed.
     Cost: a reviewer must trust a description of a document they cannot see.
  2. **Commit a stripped sample as the fixture** — one artifact.
     Gain: the reviewer sees exactly what was judged.
     Cost: stripped plans have repeatedly been shown to keep identifying
     combinations; the gate forbids real material.
- Chosen: **Two artifacts, one rule**. Rejected: **commit the stripped sample**
  because stripping is exactly what cannot be verified from the file itself.

## Open questions

None that block the change. Which documents count as carriers beyond the four
named is a design detail of this change; the statement's exact sentences are
drafted there and reviewed against the evidence.
