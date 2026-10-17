# Proposal: announce-dependency-updates

## Release note

> A version whose only changes are dependency updates can now be published, with those updates named in its release notes.

## Use-Case Context

- Use-case ID: `publish-public-release` — "Publish a public release". This change extends the use case that already owns what a version states, because a dependency update is announced by the same release text and the same change log as every other released change.
- Ideation topic: none — the change follows the operator's direction after Dependabot began proposing dependency updates; it has no ideation seed.
- Seed path: none.
- Confirmed Personas:
  - `project-maintainer--release-owner` — the person who decides a version is ready, prepares it, and answers for what its notes state; confirmed on 2026-10-10 and promoted on 2026-10-12.
  - `crew-member--personal-calendar-owner` — the person who reads a version's notes before deciding whether to download it; same confirmation and promotion.
  - Neither Persona is a Gherkin Actor of this change; the Actor in its scenarios stays the release maintainer.

## Problem / Opportunity

A dependency update is a change a crew member receives: the executable is built from the dependency graph, so a bumped crate changes the bytes a downloader runs. Today such an update cannot reach a version's notes, and a version made only of them cannot be released at all.

- The change-log configuration admits a revision only when its subject carries a capability scope and a release-note type — a new capability, a fix, a performance improvement, or a reversal. A dependency update arrives as a housekeeping revision (`chore(deps): …`), so it is skipped before anything else looks at it.
- The renderer then refuses a version whose section would name no change a user can see. A version whose only revisions are dependency updates therefore fails preparation, and nothing is staged or published.
- The same gate applies to the release text: even when a dependency update is bundled with a capability change, the notes omit the updates.

The practical result is that the maintainer cannot publish a security or maintenance update on its own. They must either wait for an unrelated visible change to carry the release, or publish nothing, even though the version's executable and its dependency graph did change.

The opportunity is narrow and mechanical: treat a dependency update as a released change when it changed something a user receives, name it under its own group, and let a version made only of such updates be published — without widening what else is announced.

## Proposed Solution

A dependency update that changed something a user receives becomes a released change.

- **Announced.** A dependency update — a revision whose subject is a dependency bump, such as `chore(deps): …` — is announced when its change touched a path a user receives. The dependency graph is such a path, so a Cargo dependency bump is announced.
- **Grouped.** Announced dependency updates appear under **Dependencies**, after Reversals, in the order the revisions occur in history, each stating the update's own words.
- **Publishable on its own.** A version whose only announced changes are dependency updates is releasable: its notes name those updates and nothing else, and preparation stages the version as usual.
- **Still filtered.** A dependency update that changed only the project's own checks or automation is not announced; one that changed the release workflow is, because that workflow decides what an archive holds.
- **Announced once.** A dependency update whose subject also names a capability stays in that capability's group and is not repeated under Dependencies. Identical update subjects collapse to one entry, as every released change already does.
- **Still refused when empty.** A version whose notes would name nothing at all — no capability change and no dependency update — is refused exactly as today.

Everything else about a release is unchanged: capability-scoped entries, the machinery skip rules, the user-receiving path filter, version/date/comparison rendering, the drift check, immutable publication, and the five-platform archive set.

## Capability Routing

`givn spec route --change announce-dependency-updates` recommends `EXTEND export-duty-plan-calendar` (score 13.00, `publish-public-release` second at 12.00). The recommendation is lexical: the proposal's words about releases and updates overlap the conversion use case, while the behaviour itself is what a published version states and what preparation stages.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `publish-public-release` | `EXTEND export-duty-plan-calendar` | `EXTEND publish-public-release` | The behaviour is the release text and the change log — what a published version states and what preparation stages. The conversion capability owns what a calendar contains, and no conversion rule changes here. Route's lexical overlap is not ownership. |

## Out of Scope

- **Other housekeeping revisions.** Only dependency updates (`chore(deps): …`) gain admission; documentation, test, build, CI, and other housekeeping revisions stay unannounced no matter which paths they touch.
- **Widening the path filter.** The existing user-receiving path list decides which dependency updates are announced; this change adds no path and removes none, so a checks-only update stays invisible and a release-workflow update stays visible.
- **Merging dependency updates automatically.** Updates remain reviewable proposals; nothing merges itself.
- **The deliberately pinned crates.** The extraction and architecture-test crates stay excluded from automatic bumps; if a pinned crate is updated by hand and touches the dependency graph, it is announced like any other dependency update.
- **Dependabot configuration.** The update cadence, grouping, labels, and commit-message prefix are unchanged; the change relies on the conventional `chore(deps):` subject Dependabot already produces.
- **Release mechanics.** The tag trigger, draft-then-publish flow, immutability, archive set, provenance, bill of materials, and drift check are untouched.
- **Rewriting published releases.** A published section or release page is not edited; the next dependency-only version is the first to show the new group.
- **A dependency update's own vulnerability status.** The notes state that a dependency changed; they do not assert that the new version is free of advisories.

## Open Questions

### D1 — Should every dependency update be announced, or only those a user receives?

- Question: A dependency update can touch the dependency graph, the project's checks, the release automation, or a document. Which of them belong in a version's notes?
- Options:
  1. **Every `chore(deps)` revision** — announce each update whatever it changed; gain: no dependency update is ever missed; cost: a checks-only or documentation-only bump would be presented as something a crew member receives, which the notes' own promise forbids; in practice: bumping a CI action would appear beside a fix a user can act on.
  2. **Only updates that changed a user-receiving path** — reuse the existing path rule; gain: the notes keep meaning "what you receive", and the release-workflow boundary is already defined; cost: a dependency update that changes only a check is silent; in practice: a `Cargo.lock` bump is announced, a `.github/workflows/ci.yml` bump is not, and a `.github/workflows/release.yml` bump is.
  3. **Only Cargo updates** — name the dependency graph and ignore every other dependency update; gain: the simplest boundary; cost: a release-workflow action bump that changes what an archive holds would be omitted; in practice: bumping the archive action would be silent although the packaging changed.
- Disposition: **answered** — option 2, by the use case's existing rule that a released revision must have changed a path a user receives. `Cargo.lock` is on that list, so a Cargo bump qualifies; the checks path is not; the release workflow is.

### D2 — Where do dependency updates appear in a version's notes?

- Question: Dependency updates could lead the notes, follow the capability groups, or be folded into an existing group. Where should a reader find them?
- Options:
  1. **Before new capabilities** — gain: updates are the first thing a returning reader sees; cost: a crew member looking for a new capability must pass maintenance noise; in practice: a dependency bump would outrank a new export capability.
  2. **A Dependencies group after Reversals** — gain: capability changes keep the top of the notes, and maintenance has a predictable home at the end; cost: a major dependency upgrade is not prominent; in practice: the notes read Breaking changes, New capabilities, Fixes, Performance, Reversals, Dependencies.
  3. **Folded into Fixes** — gain: no new group; cost: a reader cannot tell a defect repair from maintenance, and the group's promise ("Fixes") becomes inaccurate; in practice: a serde bump would read as a fix.
- Disposition: **answered** — option 2, confirmed by the operator for this change. The group is named **Dependencies** and sits after Reversals, in history order.

### D3 — What happens when a dependency update also names a capability?

- Question: A revision can update the dependency graph while its subject carries a capability scope, for example `fix(export-duty-plan-calendar): Upgrade the PDF parser`. Should it appear in both the capability group and Dependencies, or once?
- Options:
  1. **Both groups** — gain: the dependency change is visible under Dependencies; cost: one revision appears twice in one version's notes, and identical words appear twice if another revision carries them; in practice: a parser upgrade would be listed under Fixes and again under Dependencies.
  2. **Only under the capability group** — gain: one revision is one announced change, matching the existing collapse rule; cost: a reader scanning only Dependencies may miss it; in practice: the parser upgrade appears once, under Fixes, and Dependencies names only updates without a capability scope.
- Disposition: **answered** — option 2. A revision is matched by one kind at a time, and each released change is stated once; the existing identical-subject collapse must not be bypassed.

### D4 — Do the renderer and the preparation command need code changes?

- Question: Once dependency updates are admitted to the generated notes, must the renderer or the preparation command change to accept them?
- Options:
  1. **Configuration only** — add the dependency-update rule to the change-log configuration and leave both scripts alone; gain: the smallest change, and the existing visible-count, path filter, collapse, and staging rules keep one implementation each; cost: the design must prove every generic rule already admits a dependency-only section; in practice: one configuration entry changes.
  2. **Configuration plus a renderer filter** — teach the renderer that dependency entries are visible even when the path filter would drop them; gain: explicit handling; cost: a second visibility rule that can disagree with the path filter, and a dependency bump of a checks-only path could leak; in practice: the renderer would grow an exception beside the existing filter.
  3. **Configuration plus a preparation change** — teach the preparation command to accept a dependency-only version; gain: the refusal reads explicitly; cost: duplicates the renderer's visible-count decision and lets the two drift; in practice: a version could pass one gate and fail the other.
- Disposition: **carried** — owner: `design.md`, which must decide between the options and prove, from the renderer's actual order of operations, whether the generic path filter, identical-subject collapse, and visible-count check already admit `chore(deps)` entries once the configuration names them. The proposal does not presume a script change.
