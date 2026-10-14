# Proposal: list-only-changes-a-user-receives

## Release note

> A version's notes list only changes a user can receive: a released revision that touched none of the paths a user's copy is made of — the converter itself and the packaging that decides what an archive holds — is left out, so a version's notes never present the project's own engineering work as something a reader can act on.

## Use-Case Context

- Use-case ID: publish-public-release ("Publish a public release")
- Ideation topic: none — this change is not from ideation; it closes a debt recorded by the change to generate release notes
- Seed path: none
- Confirmed Personas: `project-maintainer--release-owner` ("Maintainer responsible for a public release") is the actor who prepares a release and reads what it will state; `crew-member--personal-calendar-owner` ("Crew member exporting their own duty plan") is the reader the notes promise serves. Neither is a Gherkin Actor of this change; the Actor in its scenarios stays the maintainer.

## Problem / Opportunity

A revision is announced in a version's notes when its conventional-commit type carries a capability scope and its subject's words avoid a maintained list of tooling phrases. The words are the wrong instrument: the list is reactive, so it removes an entry only after someone has added the phrase that names the tooling.

Version 0.2.0's section shows the result. Of its three entries, two are the project's own engineering work — "Give the suite job the repository history its preparation check reads" and "Render an untagged released revision, so a version can be rendered before its tag" — and a crew member can act on neither. The third, the travel title that prints the crew position and the flight number as two words, is the one the reader can use. A reader who trusts the notes cannot tell which is which, and the notes' own promise is that they name what a version gives them.

Chapter 11 records this as TD-008: the rules can remove an entry no reader can act on, but they cannot write one a reader can. That half — the authorship — stays the maintainer's, and this change does not pretend to automate it. What this change fixes is the removal: it stops being a guess about words and becomes a fact about the change.

## Proposed Solution

A revision is announced only if it changed something a user receives.

- A **user-receiving path** is a path whose contents reach a downloader: the converter's crates and application, and the packaging that decides what an archive holds. The list is maintained in the change-log configuration beside the capability allow-list and the machinery phrases, and the renderer checks it against the tree, so a path that no longer exists stops the render rather than silently narrowing what can be announced.
- For each announced entry the renderer resolves the revision it came from and the paths that revision changed. An entry whose revision changed no user-receiving path is left out of the section.
- The existing refusal stays: a version whose notes would then hold nothing states that the version changes nothing a user can see, and nothing is written.
- The prose rule is recorded where the maintainer reads it: a released subject states what a crew member can act on, and engineering work takes a housekeeping type so it is never released. The machinery phrase list stays as a second net for a subject that names tooling while changing a user-receiving path.

Observable consequences a reader sees:

- 0.2.0's section, rendered again after this change, holds the travel-title fix and neither of the two engineering entries.
- A revision that changed only `.github/workflows/`, `scripts/render-changelog.sh`, `cliff.toml`, `docs/` or test code is never announced, whatever its subject says and whatever capability its scope names.
- A revision that changed `crates/duty2cal-core/src/…` is announced even when its subject is terse, because what it changed is what a user receives.
- A version whose released revisions all changed only engineering paths fails as it does today, naming that nothing a user can see changed.

## Capability Routing

`givn spec route --change list-only-changes-a-user-receives` recommends `EXTEND export-duty-plan-calendar` (score 13.00), which is the advisory lexical ranking: the proposal's words about "change" and "release" overlap a capability that converts a duty plan. The decision differs, and the routing table is the declaration:

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| publish-public-release | `EXTEND export-duty-plan-calendar` | `EXTEND publish-public-release` | The release text, the change log, their visibility rule and the configuration that decides what is announced belong to the use case to publish a public release ("Publish a public release"). The conversion capability owns what a calendar contains, not what a release states about it. Route's lexical match is not a signal about ownership. |

## Out of Scope

- Writing a note a reader can act on. The authorship is the maintainer's, and TD-008 records that no rule can do it.
- Version numbering, the release channel, immutability, per-platform archives and their provenance.
- The capability allow-list, the pinned generator, the drift check and the machinery phrase list, which stay as they are.
- Rewriting published history. The 0.2.0 section is not yet tagged, so re-rendering it before the tag is a normal render, not a repair.
- Any judgement about a subject's wording beyond the rule this change records.

## Open Questions

### D1 — How the process should notice a note that names work no reader can act on

- Status: **answered 2026-10-14 by the operator**, in this proposal step, before the spec was written.
- Blocks: whether the release notes change by construction, by a gate, or not at all.
- Context: chapter 11's TD-008 records that the notes are only as clear as the released revisions' own subjects, and that the machinery phrase list can remove an entry no reader can act on but cannot know that a subject written in the project's vocabulary is invisible to a crew member. 0.2.0's section carries two such entries.
- Options:
  1. **Report at render time.** Meaning: the renderer prints each entry's revision and the paths it changed, naming the entries whose revision changed no user-receiving path; the notes and the release text are unchanged.
     Gain: notices by construction, from the diff rather than the words; cannot block a release; leaves authorship with the maintainer.
     Cost: a notice can be ignored, and the user-receiving path list is a maintained judgement that cries wolf when a new path qualifies.
     In practice: 0.2.0's render would print the two engineering entries under a notice while leaving them in the section.
  2. **Filter them out of the notes.** Meaning: an entry whose revision changed no user-receiving path is left out of the section.
     Gain: no engineering work can reach a published release text, mechanically, without depending on anyone's phrasing.
     Cost: the drop is silent and can remove a change a reader needed — the fix that forced the executable mode into the archives changed only the packaging, and a downloader who no longer sees it cannot learn why their download differs — and a missing note is the failure nobody reports.
     In practice: 0.2.0's section keeps the travel-title fix and drops the two engineering entries.
  3. **Refuse the release.**
     Gain: loud, and consistent with the pinned generator and the two marked lists.
     Cost: it is a trap — the offending revision is already committed when the refusal fires, so the version cannot be released without rewriting history (the project rewrites it only for a private-data leak) or reverting the check.
     In practice: today's 0.2.0 render would refuse, and the only repairs would be a history rewrite or a check change.
  4. **Keep it to the written rule and the phrase list.**
     Gain: nothing new to maintain.
     Cost: it is the state TD-008 records — one phrase added after each leak, and no way to know that a project-vocabulary subject is invisible to a reader.
     In practice: the next pipeline fix whose words are new is released as a change a reader can act on, and someone notices once it is public.
- Resolution: **Option 2, filter them out of the notes**, chosen by the operator. The Packaging paths are part of the user-receiving list precisely because of the cost Option 2 names: forcing the executable mode into an archive is a change a downloader benefits from, and it must stay announceable. Rejected: Option 1, because a notice that can be ignored leaves the same entry in a published release text; Option 3, because a gate whose failing state cannot be cleared by doing the right thing next is disabled by the next maintainer; Option 4, because it is the debt this change exists to close.
