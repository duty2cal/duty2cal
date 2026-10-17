# Public-home infrastructure — use-case definition

Status: revised 2026-10-17 after the Use Case Definition boundary review; six
blocking and six advisory findings applied. The identifiers below must survive
the elaborated use cases, the change seeds and the permanent leaf documents.

Vocabulary note: **Declared configuration** and **Configuration change** are
the confirmed terms; "public-home configuration" names the boundary, and "a
revision of the declared configuration", "applied state record", "live
configuration", "preview" and "configuration drift report" are artifact labels,
not new domain terms.

## Actor-goal cut

| Name and stable use-case ID | Level | Primary Actor | Goal | Boundary | Mutation |
|---|---|---|---|---|---|
| Adopt the existing public-home configuration (`adopt-existing-configuration`) | ! | Organization owner | Place the already-existing objects of the public home under the declared configuration without changing them. | Public-home configuration; the live configuration is read only. | Changing: writes the declared configuration and the applied state record; live configuration unchanged. |
| Apply a configuration change (`apply-configuration-change`) | ! | Organization owner | Make the live configuration match the declared configuration revision the apply accepts. | Public-home configuration. | Changing: changes the live organization and repositories, and the applied state record. |
| Check the configuration drift (`check-configuration-drift`) | ! | Organization owner | Obtain a report of every difference between the declared configuration and the live configuration. | Public-home configuration; the live configuration is read only. | Observing: reads; writes no configuration. |

All three are `!` leaves. None names a component, mechanism or channel. No `+`
summary case is needed: no leaf's flow names three independent actor goals,
and each success guarantee is one outcome.

## Boundary

The boundary is **public-home configuration**: the declared configuration in
the repository, the applied state record of what was last applied, and the
live organization and repositories the declared configuration describes. A
term collision marks the boundary — **Configuration drift** is not the
glossary's change-log **Drift check**, and **Adoption** is not the prior
topic's contract adoption.

## Bounded outcomes

### Adopt the existing public-home configuration

- **Trigger:** the owner begins declaring the objects that already exist in
  the organization and its repositories.
- **Success guarantee:** every already-existing object in the selected batch
  is represented in the declared configuration and bound in the applied state,
  with the live configuration unchanged. Adoption batches are selected; an
  object Q9 leaves for a later change is not in the batch. (open: Q9)
- **Minimal guarantee:** no live object is created, deleted or modified; a
  failed adoption records no binding and names the object; a declared
  configuration revision recorded before a later failure may remain, but the
  applied state records no binding for it.
- **Closure test:** after the adoption the owner can stop; the live
  configuration serves the public home exactly as before.
- **Scope:** the batch of existing objects — organization settings, both
  repositories, the Pages source, the existing rulesets and the classic
  branch protection. The deploy key and `PAGES_DEPLOY_KEY` secret do not exist
  and are therefore created by the first configuration change, never adopted.

### Apply a configuration change

- **Trigger:** the owner begins a configuration change by recording a revision
  of the declared configuration.
- **Success guarantee:** the live configuration matches the declared
  configuration revision the apply accepted for every object the revision
  retains; removals follow Q11. (open: Q11)
- **Minimal guarantee:** a refusal before the first write leaves the live
  configuration and the applied state unchanged; a failure after the first
  write is reported as partial with the applied state marked stale or
  unavailable; no already-existing object is recreated; the failure names the
  object and the reason.
- **Closure test:** after a successful apply the owner can stop; the applied
  state record and the live configuration agree.
- **Open:** whether the reviewer's decision gates the apply or is advisory
  (Q10).

### Check the configuration drift

- **Trigger:** the owner — directly or through the automation acting for the
  owner — checks the live configuration against the declared configuration.
- **Success guarantee:** the owner receives a report that names every
  difference between the declared configuration and the live configuration, or
  states that there is none, and states the applied state's availability and
  age.
- **Minimal guarantee:** the live configuration is not changed; a missing,
  unreadable or stale applied state is reported as unavailable, never as no
  configuration drift.
- **Closure test:** after reading the report the owner can stop; the system is
  undisturbed.
- **Open:** whether the check can run in CI at all depends on the state access
  (Q4).

## Stakeholder tags

| Use case | Confirmed Persona lenses |
|---|---|
| Adopt the existing public-home configuration | `project-maintainer--organization-owner` (primary); `contributor--infrastructure-reviewer`. |
| Apply a configuration change | `project-maintainer--organization-owner` (primary); `contributor--infrastructure-reviewer`; `crew-member--personal-calendar-owner` (the monorepo ruleset and the release channel, open: Q3; the Pages publish path, open: Q2; also the future site visitor's lens). |
| Check the configuration drift | `project-maintainer--organization-owner` (primary); `contributor--infrastructure-reviewer`. |

"Future site visitor" has no separate Persona: it is the crew-member lens on
the published artifact.

## Inside the goals, not separate use cases

- Recording a revision of the declared configuration: the first step of a
  configuration change, not a separate actor goal.
- Preparing a preview: a precondition of applying.
- Reviewing the revision: a supporting actor's step in the apply flow; whether
  it gates is Q10, not a new goal.
- Recording the applied revision in the applied state: part of applying.
- Enabling secret scanning, push protection and Dependabot security updates:
  content of a configuration change.
- Declaring the Pages source: content of the adoption on the first pass;
  declaring the deploy key and the secret: content of the first configuration
  change, because they do not exist yet.

## Excluded actor goals

- Publishing the website: the website topic owns the build and publish chain.
- Creating the organization or transferring a repository into it: manual
  preconditions with no GitHub endpoint or provider resource.
- Setting release immutability or private vulnerability reporting: no provider
  resource; manual, outside the declared configuration.
- Managing team access and membership: outside the confirmed Frame; Q6 remains
  open and would revise this exclusion if answered to include teams.
- Changing the product executable, its release workflow, or the conversion
  contract.
- Repairing a published release: releases are immutable; a correction is the
  next version.

## Sizing checks

| Rule | Result |
|---|---|
| One goal, one primary actor | Each goal is one active-verb phrase for the Organization owner; no goal joins two verb phrases. |
| Goal-shaped, never component-shaped | No use-case name or goal names a tool, provider, workflow or channel. |
| Level discipline | All three are `!`; none is promoted to `+`. |
| Mutation containment | Adopt and apply are changing; check is observing and owns no mutating interaction. |
| Extension discipline | The apply flow's branches (missing or stale preview, unreviewed revision under Q10, create-before-adopt, partial failure) protect its guarantees; none introduces a new actor goal. |
| Preconditions and flow | The declared configuration exists before apply and check; adoption may create the declared configuration; apply requires a current preview; check accepts a missing applied state and reports it unavailable. The trigger is the owner's action. |
| Implementation independence | The goals stay true if OpenTofu, the provider or the CI system were replaced. |
| Relationship semantics | No Includes/Extends: the three goals stand alone. |
