# Change seed: state-the-supported-duty-plan

Status: implemented in state-the-supported-duty-plan

- Use-case IDs: `state-the-converter-support-statement` (new),
  `publish-public-website` (the page's what-it-does and non-affiliation
  statements), `apply-configuration-change` (the organization profile apply)
- Topic: `generalized-duty-plan-input`
- Personas: `crew-member--personal-calendar-owner` (will my plan work?),
  `project-maintainer--release-owner` (does the claim rest on evidence?),
  `contributor--web-reviewer` (the page statement and its check),
  `contributor--publication-reader` (the issue-form route and the evidence
  record)
- Suggested start: 1; supersedes `document-the-publication-evidence-route`.
- Handoff: ready. On 2026-10-27 the operator chose clarity over a generic
  statement and withdrew the general/NetLine-style intake; the project names
  the one plan it actually converts.

## Accepted decisions

### The public statement names the supported plan

- Chosen: **name the operator, the crew role and the export**. The public
  sentence is "duty2cal converts a Lufthansa cabin-crew duty plan published as
  a Jeppesen PDF export into one calendar file"; on the website it reads "This
  tool converts a Lufthansa cabin-crew duty plan published as a Jeppesen PDF
  export into one calendar file."
- The boundary travels with the sentence: cabin crew, this Jeppesen PDF export,
  one roster month; not cockpit or other Lufthansa plans, not other Jeppesen
  exports, not another operator.
- The non-affiliation sentence names them: "Not affiliated with Lufthansa,
  Jeppesen, or any duty-plan provider or airline." No logo, no "official", no
  implied endorsement.
- Rejected: **keep a generic "supported export"** because it neither avoids the
  name (the architecture chapter and issue forms already say "Released Roster
  detail table") nor answers the crew member's question; **say nothing about
  support** for the same reason; **name only the internal table** because a
  crew member does not know that name.

### One wording across five carriers, checked

- The same sentence appears in the website's `what-it-does` statement, the
  README, the organization profile, architecture chapter 3, and the two issue
  forms. A check refuses a carrier that disagrees with the statement; the
  profile is published through the declared configuration, so its carrier is a
  configuration apply rather than a hand edit.
- Rejected: **README authoritative, others link** because the profile and issue
  forms cannot defer cleanly to a section; **per-carrier wording** because a
  statement that disagrees with itself is worse than none.

### The claim rests on the converting fixture

- The generated fixture for the supported plan must convert through the
  unchanged conversion; the private plans stay private; the evidence record
  states what the sample showed (dates, codes, fields) and the committed
  fixture is generated or stripped. If the fixture fails, the statement shrinks
  and the refusal names the missing fact or unknown code; it does not wait for
  a release.
- Rejected: **sample alone** because a reviewer would judge by reading;
  **landed reading** because code review does not prove that a plan converts.

### The issue forms ask by outcome

- The form introduction states the support sentence and the boundary. The
  reporter is asked "did the plan convert?" and "what did the refusal say?",
  attaches a stripped sample when needed, and is never asked to classify the
  plan into an export-family taxonomy. The family dropdown is removed from both
  forms.
- Rejected: **keep the family dropdown** because it teaches a unit the
  project no longer uses and asks the reporter to do the classification.

### A new activity meaning stays a separate decision

- A reading may use only confirmed codes; an unknown code is refused at
  conversion, and giving it a meaning needs its own change with its own
  evidence. No per-publication code table.
- Rejected: **the reading carries a code table** because two places would
  decide what a code means.

### Evidence custody

- Two artifacts, one rule: the contributor inspects a real plan locally and
  never supplies it; the evidence record states what the sample showed; the
  committed fixture is generated or stripped. No real material enters history.
- Rejected: **commit a stripped sample as the fixture** because stripping
  cannot be verified from the file itself.

## Open questions

None that block the change. The exact sentence placement and the check's
mechanics are design work.
