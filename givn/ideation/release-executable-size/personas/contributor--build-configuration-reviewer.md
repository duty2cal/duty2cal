# Persona: contributor--build-configuration-reviewer

Name: Contributor reviewing a build-configuration change

Status: confirmed by the user on 2026-10-18 through Q4. Distinguishing
criterion: own vocabulary differs from `contributor--infrastructure-reviewer`,
whose vocabulary is the declared configuration, protection rulesets and
previews. This segment judges size attribution and compiler-profile deltas and
shares that reviewer's authority boundary and review-only capability.

## Stable core

- Role: A contributor who reviews a change to the build configuration
  (`Cargo.toml` profiles, feature selection, build workflow) without releasing
  it.
- Authority boundary: May review and approve a change. May not apply a
  profile change to a release, publish, or relax the project's distribution
  constraints.
- Evaluation capability: Judges whether a size claim is measured against the
  distributed artifact, whether the attribution supports the chosen lever, and
  whether the stated consequence (panic behavior, diagnostics, build time) is
  accurate. Cannot measure the Windows executable unless the project's
  automation produces the report.
- Own vocabulary: Measured delta, size attribution, crate weight, panic
  behavior, feature selection, stable lever.
- Success criterion: A build-configuration change names the quantity it
  measured, shows the before and after, and states what the change costs a
  crew member.

## Running record

- 2026-10-18 — Created as a topic-local segment for release-executable-size.
  Justified by distinct vocabulary, not by a new authority: the review and
  approval boundary is the one `contributor--infrastructure-reviewer` already
  holds.

## Topic stances

- 2026-10-18 — release-executable-size: a profile change that cites the
  archive's size as evidence for the executable's size is a review finding, not
  an acceptance.
