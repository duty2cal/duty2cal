# Change seed: enable-repository-security

Status: proposed by enable-repository-security

- Use-case IDs: `apply-configuration-change`
- Topic: `public-home-infrastructure`
- Personas: `project-maintainer--organization-owner`, `contributor--infrastructure-reviewer`
- Suggested start: 2; depends on `declare-public-home`.
- Handoff: confirmed by the user on 2026-10-17 (Q12).

## Accepted decisions

### Security features are declared configuration, applied through the change flow

- Context: secret scanning, push protection and secret-scanning validity checks
  are off on the public repository; Dependabot security updates are on. The
  Frame puts the provider-exposed security features inside the declaration.
- Options:
  1. **Declare and apply them through the reviewed change flow** — a preview
     names the fields, the apply enables them, the state records the revision.
     In practice: the preview names `secret_scanning` and
     `secret_scanning_push_protection`; the apply enables both.
     Gain: the change is reviewable and drift-checked like any other.
     Cost: the apply mechanism must exist before the features can land.
  2. **Turn them on by hand now** — one settings page.
     Gain: immediate.
     Cost: an unrecorded change, exactly the problem the topic addresses.
- Chosen: **Declare and apply through the flow** because the Frame's scope and
  the adoption/apply contracts require one declaration. Rejected: **Turn them
  on by hand** because it creates undeclared state.

### Refusal and partial failure keep their meaning

- Context: enabling several fields is the first real apply; a permission can
  fail after an earlier field succeeded.
- Options:
  1. **Refuse before the first write; report partial failure after it** — a
     refusal leaves live and state unchanged; a later failure marks the state
     stale and names the object.
     In practice: the organization field applies, the ruleset update fails, and
     the report names both.
     Gain: truthful reporting; no rollback claim.
     Cost: the operator must re-preview after a partial failure.
  2. **Attempt rollback on failure** — revert applied fields automatically.
     Gain: all-or-nothing appearance.
     Cost: rollback can itself fail; the contract would promise more than the
     provider guarantees.
- Chosen: **Refuse or report partial** because the apply contract says so.
  Rejected: **Automatic rollback** because no transactional apply is assumed.

## Open questions

### Does review gate the apply? (Q10)

A gating rule stops an unreviewed change; an advisory rule records the review
but lets the owner apply anyway. The security fields themselves are not open.

### What may an apply delete? (Q11)

The revision may remove a field or an object. Whether a removal deletes live
configuration or only stops managing it changes the apply's success guarantee.

### Does the ruleset require pull requests? (Q3)

If the first apply lands through a direct push, review is convention; if the
ruleset requires pull requests, review is structural. That decision shapes the
landing routine, not the scanning fields.
