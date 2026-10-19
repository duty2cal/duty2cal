# Change seed: reduce-executable-size

Status: proposed by reduce-executable-size

- Use-case IDs: `reduce-distributed-executable-size`
- Topic: `release-executable-size`
- Personas: `project-maintainer--release-owner`, `contributor--build-configuration-reviewer`, `crew-member--personal-calendar-owner`
- Suggested start: second, after `measure-executable-size`; the reduction consults the attribution that seed produces.
- Handoff: published by the user's confirmed Handoff on 2026-10-18; not proposed or implemented.

## Accepted decisions

### Each lever is measured cumulatively

- Context: Independently measured per-lever deltas need not add under LTO and
  section alignment, so a sum could disagree with the landed difference.
- Options:
  1. **Independent deltas, summed in the guarantee** — measure each lever
     against the unchanged revision.
     In practice: lever A shows −20 KB and lever B −30 KB, while both together
     remove 42 KB; the guarantee's 50 KB is false.
     Gain: each lever is measured in isolation.
     Cost: the stated sum can be unattainable.
  2. **Cumulative deltas** — measure each lever against the configuration
     carrying the previously accepted levers.
     In practice: lever A shows −20 KB, then lever B shows −22 KB against A's
     build; the landed difference is 42 KB by construction.
     Gain: the sum equals the landed difference.
     Cost: the order of levers affects each individual delta.
- Chosen: **Cumulative deltas** because the success guarantee must be
  established by the flow. Rejected: **Independent deltas, summed** because the
  arithmetic is not preserved by LTO and alignment.

### `panic = "abort"` is conditional

- Context: The draft treats abort as the one unspent stable lever, with an
  unmeasured 5–15% estimate; the release build would lose unwinding.
- Options:
  1. **Apply it with the other stable levers** — set `panic = "abort"` now.
     In practice: the next release ends on panic without unwinding; the delta is
     never attributed.
     Gain: one fewer experiment.
     Cost: an unmeasured change with a user-visible failure consequence.
  2. **Accept it only with a measured delta and failure evidence** — measure,
     assess the panic diagnosis, then decide.
     In practice: the decision record names the bytes removed and what a panic
     leaves the user — the panic message or a numbered report.
     Gain: the change is justified and its consequence recorded.
     Cost: one extra build and assessment.
- Chosen: **Accept only with a measured delta and failure evidence** because the
  Frame's problem is estimates in place of measurement. Rejected: **Apply it
  now** because it is the exact behavior the topic exists to stop.

### The baseline is a rebuild, not the published binary

- Context: The release build uses the rolling stable toolchain of its date, and
  v0.3.0 was the last five-platform release; its published binary is not a
  like-for-like baseline.
- Options:
  1. **Published binary as baseline** — compare against the released
     executable's bytes.
     In practice: a difference partly caused by a newer toolchain is attributed
     to the lever.
     Gain: no baseline build.
     Cost: a false delta.
  2. **Rebuilt baseline** — rebuild the released revision with the current
     stable toolchain and the unchanged configuration; report the published
     binary separately.
     In practice: the baseline build and the lever build share everything but
     the lever.
     Gain: a comparable measurement.
     Cost: one more build.
- Chosen: **Rebuilt baseline** because the delta must isolate the lever.
  Rejected: **Published binary** because toolchain drift would be counted as a
  saving.

### Feature selection is measured-empty; dead-code pruning stays out

- Context: `pdfplumber` enables only `default`/`std`, `time` unifies to
  `default`/`std`, and no `rayon` is in the lock file; no attribution exists for
  `core::fmt`.
- Options:
  1. **Audit features and prune dead code anyway** — apply the draft's items 4
     and 5.
     In practice: edits with no measured target; the 5–20% estimate has nothing
     to drop.
     Gain: follows the draft's checklist.
     Cost: unmeasured churn.
  2. **Revisit only where the attribution points** — no feature or dead-code
     change without a measured target.
     In practice: the attribution is produced first; only a named weight leads
     to a feature change.
     Gain: every edit has a target.
     Cost: some levers wait for data.
- Chosen: **Revisit only where the attribution points** because the guarantees
  require measured justification. Rejected: **Audit anyway** because it repeats
  the estimate-driven behavior the topic rejects.

### The diagnosis a panic leaves is recorded before landing

- Context: With abort, a panic ends the process; the permanent conversion use
  case promises a numbered diagnostic for `Result` failures, not for panics.
- Options:
  1. **Treat panics as unreachable** — rely on the no-`unwrap`/`expect`
     guardrail.
     In practice: a bug-induced panic aborts with only the default message.
     Gain: no extra code.
     Cost: no recorded promise about what a panic leaves the user.
  2. **Record the diagnosis form** — state before landing whether a panic leaves
     the message on stderr or a numbered report.
     In practice: the decision record names the form; the reviewer checks it.
     Gain: the failure outcome is explicit.
     Cost: a design choice the change must settle.
- Chosen: **Record the diagnosis form** because the Frame requires it before
  landing. Rejected: **Treat panics as unreachable** because it leaves a
  user-visible outcome unstated.

## Open questions

### Is a size ceiling needed after the reduction?

Parked in Q3. The options are a ceiling on the executable, a ceiling on the
archive, or no ceiling with the attribution report as the only guard. The choice
depends on the floor the reduction establishes and on whether a release should
refuse rather than report.

### Which concrete example carries the success guarantee?

Resolved by the change's D3: the first measured lever, `panic = "abort"`, saves
83,968 B (2,098,176 B → 2,014,208 B); that measured saving is the success
example. No number is invented.

### When is the attribution produced?

Resolved by `measure-executable-size`: the attribution is produced for every
pushed revision and requestable for any named revision, so the reduction's first
step can consult one. The release-only cadence that would block it is no longer
the chosen cadence.
