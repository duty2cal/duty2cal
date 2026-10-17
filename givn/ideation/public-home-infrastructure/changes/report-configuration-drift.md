# Change seed: report-configuration-drift

Status: ready

- Use-case IDs: `check-configuration-drift`
- Topic: `public-home-infrastructure`
- Personas: `project-maintainer--organization-owner`, `contributor--infrastructure-reviewer`
- Suggested start: 4; depends on `declare-public-home`; CI execution depends on Q4.
- Handoff: confirmed by the user on 2026-10-17 (Q12).

## Accepted decisions

### Drift is declared versus live; the applied state is evidence, not a drift source

- Context: the draft proposed a `tofu plan -detailed-exitcode` report, which
  compares the declaration against the applied state as well as live. A stale
  or missing state could then be misread as drift.
- Options:
  1. **Compare declared configuration with live configuration; report state
     availability separately** — the report names differences and states
     whether the applied state was available and its age.
     In practice: with no state, the report still names differences and says
     the state is unavailable, never "no drift".
     Gain: drift has one meaning; state problems are visible but distinct.
     Cost: the report carries a second dimension readers must not ignore.
  2. **Treat state staleness as drift** — one combined verdict.
     Gain: one line to read.
     Cost: a missing state would read as a configuration difference, which it
     is not.
- Chosen: **Compare declared versus live** because the confirmed term
  **Configuration drift** defines it that way. Rejected: **Treat staleness as
  drift** because it redefines the term.

### Unreadable fields are unchecked, never equal

- Context: the owner's authority can read the repositories but not necessarily
  every organization setting; a comparison cannot assume what it could not
  read.
- Options:
  1. **Report unchecked fields explicitly** — the report lists them and never
     counts them as equal.
     In practice: organization settings unreadable → the field appears under
     "unchecked"; the report does not say "no differences".
     Gain: an honest report.
     Cost: a report can be green except for unchecked fields, which needs a
     human read.
  2. **Treat unreadable as equal** — silence.
     Gain: a clean verdict.
     Cost: false assurance.
- Chosen: **Report unchecked fields** because the success guarantee counts only
  compared fields. Rejected: **Treat unreadable as equal** because it defeats
  the report's purpose.

## Open questions

### Which state access supports the CI report? (Q4)

A CI job cannot read the maintainer's local state. An encrypted remote backend
lets the scheduled report compare against the applied state; a local-only state
means the report either does not run in CI or reads a stale export. The answer
also decides whether this seed includes a CI job at all.
