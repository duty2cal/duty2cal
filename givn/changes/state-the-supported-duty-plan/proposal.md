# Proposal: state-the-supported-duty-plan

## Release note

> Required. One complete, user-facing sentence naming what this change does.
> Every revision this change records carries the release note as its message subject, with the scenario title in the message body.

Every public statement now names the duty plan duty2cal converts: a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export.

## Use-Case Context

- Use-case ID: `state-the-converter-support-statement` (new), `publish-public-website` (extended: the site's `what-it-does` and `non-affiliation` statements), `apply-configuration-change` (extended: the organization profile apply), `publish-public-release` (document-only: its contributor-route description and the public-text rule it owns)
- Ideation topic: `generalized-duty-plan-input`
- Seed path: `givn/ideation/generalized-duty-plan-input/changes/state-the-supported-duty-plan.md`
- Superseded seed: `givn/ideation/generalized-duty-plan-input/changes/document-the-publication-evidence-route.md` (`Status: superseded by state-the-supported-duty-plan`); its further-publication route is withdrawn and nothing from it is specified here
- Confirmed Personas: `crew-member--personal-calendar-owner`, `project-maintainer--release-owner`, `contributor--web-reviewer`, `contributor--publication-reader` — all four are named by the seed and confirmed in `givn/personas/`; no Persona is invented or promoted here

## Problem / Opportunity

The project's public copy no longer agrees with itself, and none of it answers the crew member's question.

- The website says "This tool converts a crew duty plan into a calendar file." The README opens with "turns a released duty plan into a calendar" and later says "a supported export". The organization profile says "Turns a crew duty plan (PDF) into an iCalendar file". Chapter 3 names "One supported Released Roster duty-plan PDF". The two issue forms ask the reporter to pick an "Export family", offering "Released Roster detail table (supported)", "another export family (unsupported)" and "not sure".
- "Released Roster detail-table family" is the code's internal name for a layout. A crew member holding a plan has never seen it. So the one question the copy exists to answer — "will my plan work?" — has no answer: the reader is asked to classify their own plan into a unit the project does not publish and cannot check.
- The forms teach the reporter a taxonomy the project is withdrawing. The general-duty-plan intake, the NetLine-style route and the further-publication evidence route are withdrawn as of 2026-10-27: the project converts one plan, not a family of them. The forms still ask a question that presumes the withdrawn model.
- The claim that the plan converts is not tied to the check that proves it. The generated fixture that reproduces the supported detail table is exercised by the conversion suite, but no public carrier says so and nothing compares the carriers with each other, so a one-file edit can leave the website, the README, the profile and the forms disagreeing.

Consequence: a crew member cannot tell whether the plan in hand is in scope; a reviewer cannot tell whether a support claim is backed; and each carrier can drift from the others without a red check.

## Proposed Solution

One support statement, naming the plan the project actually converts, carried identically everywhere support is stated:

- The sentence is: "duty2cal converts a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export into one calendar file." On the website it reads "This tool converts a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export into one calendar file."
- The boundary travels with it: one roster month of cabin crew; not cockpit duties, other Lufthansa duty plans, other Jeppesen exports or another operator.
- The non-affiliation sentence names them: "Not affiliated with Lufthansa, Jeppesen, or any duty-plan provider or airline." No logo, no "official", no implied endorsement.
- The same three sentences appear in five carriers: the website's `what-it-does` statement, the README, the organization profile, architecture chapter 3, and both issue forms. A check refuses a carrier that disagrees with the statement or that still carries the withdrawn wording, and names the carrier.
- The issue forms ask by outcome instead of by taxonomy: the introduction states the support sentence and the boundary, the reporter answers whether the plan converted and what the refusal said, and attaches a stripped sample when one is needed. The export-family dropdown disappears from both forms. The pull-request template and the contributor guidance stop asking for an export family.
- The organization profile's carrier is published through the declared configuration: the reviewed text under `org/` is the source, and the existing profile apply publishes it. The live apply is the operator's step.
- The claim rests on the generated fixture for this plan converting through the unchanged conversion. The private plans stay private; the committed fixture is generated; the evidence record states what the sample showed (the roster month, the column labels, the codes, the travel leg). If the generated fixture stops converting, the statement shrinks and the refusal names the missing fact or unknown code; it does not wait for a release.
- The site keeps its existing evidence: the page-content check refuses a page whose statement is absent or duplicated, the browser scenario captures the changed page at both viewports, and the site publication still gates the published page.

Observable difference for the crew member: the website, the README and the organization profile name the plan, so "is my plan in scope?" is answered before any file is chosen. Observable difference for the release owner: a carrier edited alone is refused by a named check. Observable difference for the reporter: the form asks what happened, not which taxonomy their plan belongs to.

## Capability Routing

`givn spec route --change state-the-supported-duty-plan` reported no signal ("the top candidates tie at 13.00") and printed no ranked list, so the recommendation column is omitted and each decision is recorded with its rationale. This table is the declaration `givn check review` compares against the delta.

| Proposed capability | Decision | Rationale |
|---|---|---|
| state-the-support-statement | NEW in state-the-converter-support-statement | The support statement is a standing claim with its own actor goal (the release owner keeps it true), its own evidence rule and its own refusal. It advances that goal, not the site's publication or the configuration apply, so it is a new goal-shaped capability under the new use case rather than a variant of a channel. |
| publish-public-website | EXTEND publish-public-website | The website's `what-it-does` and `non-affiliation` statements and the page-content check already belong to the site publication. This change edits those sentences through that capability and has the content check refuse a page that does not name the plan; the publication mechanism is untouched. |
| publish-organization-profile | EXTEND publish-organization-profile | The profile text is already a declared file published by an apply. The carrier change is a new revision of the reviewed source plus the apply that publishes it; the apply mechanism and the declaration are unchanged. |
| publish-public-release | EXTEND publish-public-release | Only the use-case document changes, but two of its rules do: its description of the contributor route still says the issue forms and pull-request template ask for the export family, and the public-text rule it owns is where the README carrier's release promise lives, so the one statement and its check are recorded there. The release capability's scenarios and the release text are unchanged. |

## Out of Scope

- Any conversion behaviour: the reader, the activity-code meanings, the timing rules, the calendar contract, the refusals and the diagnostic format are unchanged. A new activity meaning stays its own change with its own evidence.
- The withdrawn general-duty-plan intake, the NetLine-style route, the further-publication evidence route and the `make-a-further-published-duty-plan-convertible` use case: withdrawn on 2026-10-27, not built here.
- Any other operator, export or Lufthansa plan (cockpit, other Lufthansa duty plans, other Jeppesen exports): the boundary says they are not supported, and no reading for them is added.
- The site publication's mechanism, Built revision, hosting target and release coupling; the configuration apply's declaration, state, preview and apply semantics.
- The release text's content: the change alters no conversion behaviour, but it is announced as a public-copy change because the page is a user-receiving path (`web/`), scoped `feat(publish-public-website)` until the archive move.
- Marketing copy, roadmaps, a support promise for a future plan, and per-publication documentation of quirks.
- Rewriting Persona files or the ideation record: the Personas are used as review lenses, and the ideation record keeps its withdrawn seeds.

## Open Questions

The seed's naming and boundary decisions are settled (2026-10-27) and are not reopened. The remaining choices are wording mechanics and evidence placement; each is carried into `design.md`, whose Decisions ledger resolves it.

### D1 — What exact boundary sentence travels with the support sentence?

- Context: The settled decision fixes the support sentence and the non-affiliation sentence, and names the boundary (cabin crew; this Jeppesen PDF export; one roster month; not cockpit or other Lufthansa plans, not other Jeppesen exports, not another operator) without fixing the words that carry it; the check needs one exact text in five differently rendered carriers.
- Question: What sentence states the boundary in all five carriers?
- Options:
  1. **One boundary sentence after the support sentence** — "One roster month of cabin crew; not cockpit duties, other Lufthansa duty plans, other Jeppesen exports or another operator." Gain: the same settled words travel with the claim, the check can require it verbatim, and a reader meets one sentence, not a table. Cost: two long sentences in a row in each carrier. In practice: the website's `what-it-does` element reads the support sentence followed by the boundary sentence.
  2. **A bulleted boundary list** — three bullets: "Cabin crew only", "This Jeppesen PDF export only", "One roster month only". Gain: scannable on the page. Cost: five carriers render bullets differently (the issue-form introduction is Markdown, chapter 3 is prose), so "one wording" becomes three fragments and the check compares fragments. In practice: the page shows three list items where the other carriers show a sentence.
  3. **Rely on the support sentence alone** — the nouns "Lufthansa", "cabin-crew", "Jeppesen PDF export" and "one calendar file" imply the boundary. Gain: shortest copy. Cost: nothing says cockpit duties or another operator are excluded, and the seed's boundary would exist only in the decision record. In practice: a cockpit crew member reads the sentence as covering their plan.
- Disposition: carried — the architect decides it in `design.md` (Decisions ledger, D1), and the shared constants in `scripts/support-statement.env` realize it.

### D2 — Where does the cross-carrier check live and where does it run?

- Context: The decision says a check refuses a carrier that disagrees with the statement, but the repository already has a README gate (`scripts/public-text-check.sh`), an assembled-page gate (`scripts/check-site-content.sh`) and a configuration apply; the carrier set spans source files, an assembled page and an applied file.
- Question: Which command owns the cross-carrier rule, and which runs execute it?
- Options:
  1. **A new `scripts/support-statement-check.sh` called from `scripts/public-text-check.sh`** — the cross-carrier check is one command; the existing CI `public-text` job runs it on every push because it already runs the public-text check. Gain: one owner for one rule, one place to edit the expected sentences, and the site publication's own page-content check keeps its separate job (it reads the assembled output, not the source). Cost: one more script and one more invocation.
  2. **Extend `scripts/public-text-check.sh` directly** — no new file. Gain: fewer files. Cost: the script already checks README release promises with a `readme` argument; carrier parsing for HTML and YAML would turn one focused gate into two responsibilities.
  3. **Only `scripts/check-site-content.sh` and the profile's review** — the page's check enforces the page, and a reviewer compares the other carriers. Gain: no new command. Cost: the README, chapter 3 and the forms are unguarded, which is exactly the drift the decision forbids.
- Disposition: carried — the architect decides it in `design.md` (D2), including the call sites and the bats coverage.

### D3 — What does the statement say when the generated fixture stops converting?

- Context: The decision says a fixture failure shrinks the statement and the refusal names the missing fact or unknown code, but there is exactly one named plan, so a reduction cannot drop to "no plan named" without losing the answer to the crew member's question; the five carriers and the shared constants must change in one act.
- Question: Which public sentence replaces the positive claim when the fixture stops converting, and who revises the five carriers?
- Options:
  1. **Name the plan and state the refusal** — "duty2cal does not currently convert a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export; the refusal names the fact or code it could not read." Gain: the same boundary and non-affiliation sentences stay, the reader learns that this plan is the one in question and that it is currently refused, and the release owner revises the five carriers in one act. Cost: a negative public sentence that must be revised back when the fixture converts again. In practice: the page's `what-it-does` element states the refusal instead of the conversion.
  2. **Drop the plan name and keep only the boundary** — the statement names no plan until evidence returns. Gain: never states a negative claim about the named plan. Cost: the crew member again cannot tell whether their plan is the one in question, which is the problem this change exists to remove.
  3. **Leave the carriers untouched until the next release** — the release text announces the loss. Gain: no mid-cycle edit. Cost: between the failure and the release every carrier names a conversion that does not happen, which the seed rejected.
- Disposition: carried — the architect decides it in `design.md` (D3), together with the check's handling of the withdrawn wording.

### D4 — Where does the evidence record live, and what does it state?

- Context: The decision requires an evidence record that states what the sample showed, while the private plans stay private and the committed fixture is generated; the record serves a standing public claim, so it must outlive this change and be readable without opening a test file.
- Question: Which artifact is the evidence record, and what must it say?
- Options:
  1. **A durable record at `docs/evidence/supported-duty-plan.md`** — the plan's name, the generated fixture's builder and its conversion scenario, the roster month, the column labels, the codes and the travel leg the fixture prints, and the privacy rule that no real plan or identifying value is committed. Gain: the claim's evidence is readable in the repository and can be required by the statement check; it survives the change archive. Cost: a new durable document that must be revised with the plan's evidence.
  2. **The check script's own header** — the expected sentences and the evidence facts live in one file. Gain: no new document. Cost: the evidence record is then a code comment, and a reviewer reads a script to learn what the sample showed.
  3. **The change's `evidence/` directory** — the record lives with this change. Gain: nothing durable to maintain. Cost: it is not part of the permanent repository after archive, so a standing public claim rests on an archived artifact.
- Disposition: carried — the architect decides it in `design.md` (D4), including whether the check requires the record.

### D5 — Is the live organization-profile apply part of this change?

- Context: The profile's carrier is published by an apply that needs the state passphrase and an administer token, which ADR-0006 and chapter 2 keep out of CI; the reviewed source is in the repository, so the carrier can be correct before it is public.
- Question: Does this change run the live apply, and how is its success verified if it does not?
- Options:
  1. **The operator runs the apply after GREEN; the change verifies the source and the apply's local proof** — the reviewed `org/profile/README.md` carries the statement, the statement check reads it, the profile scenarios assert the served content through the fake-backed apply, and the release owner runs the real apply as a recorded operator step. Gain: no change depends on a credential or a live write, and the reviewed bytes are what the apply publishes. Cost: the live profile can lag the repository until the operator applies.
  2. **The change applies live as part of GREEN** — the apply is the evidence. Gain: the profile is public with the change. Cost: the apply needs the state passphrase and an administer token, which the gate forbids; a live apply cannot be a test step.
  3. **Drop the profile as a carrier** — four carriers only. Gain: no apply step. Cost: the organization's front page would disagree with the README, which the decision forbids.
- Disposition: carried — the architect decides it in `design.md` (D5); the decision report names the operator step and the evidence that stands without it. The operator decided on 2026-10-27: option 1 — source-level verification now, the live apply as the operator's step after GREEN, with the preview/apply/read-back transcript under the update scenario's `@e2e` slug as the live acceptance evidence.
