---
status: accepted
date: 2026-10-17
decision-makers: duty2cal maintainer (organization owner)
consulted: none
informed: none
---

# Keep the applied state local, passphrase-encrypted and committed, and check configuration drift locally

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

The compared options are the three the topic's question log records: an
encrypted remote backend, local state with an encrypted snapshot exported for
automation, and local state with a local drift check. The choice fixes a data
and security boundary — the applied state is a committed ciphertext opened by a
machine-local passphrase, and no automation may hold that passphrase — and it
removes a capability the Frame had promised (a scheduled and per-push drift
report). It constrains at least three future changes: the security features,
the publishing plumbing whose key will live in this state, and the drift report
that can no longer run in CI. Reversal needs a state migration and a new
credential or service. `design.md` owns the mechanics (the `encryption` block,
the passphrase variable, the staging workspace); the boundary and its
alternatives are not owned there. ADR-0005 records the declaration mechanism;
this record fixes a distinct boundary — where the applied state lives, how it is
protected and where drift is checked — and does not duplicate it. The register
holds no other record about state storage, so the route is a new record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort because: the state would have to be migrated to a backend and the passphrase, CI credential and drift path redesigned across several changes.
- [x] **Shapes quality attributes** - It materially affects secrecy and reproducibility because: the publish key's private half will sit in this state, so who can read the state decides who can publish to the site.
- [x] **Structural** - It defines a boundary, contract, or dependency direction because: it fixes that the state is an encrypted repository artifact, that only the maintainer's workstation can open it, and that no pipeline holds the means to read it.
- [x] **Must be shared** - Developers unfamiliar with it would systematically violate the architecture because: they would commit an unencrypted state, add the passphrase to a repository secret, or add a CI job that reads the state.

## Context and Problem Statement

Adoption and every later configuration change need a record of what is bound,
and the preview and drift check need to read it. That record also holds the
private half of the site's publishing key once that key exists. The Frame
originally asked for a scheduled and per-push configuration drift report in CI,
which cannot read the maintainer's local state, so where the state lives decides
whether that report is possible at all.

## Decision Drivers

- The local apply must be able to read and write the binding.
- The state will hold a publishing private key, so its contents must not be readable from the public repository.
- The drift check should be repeatable against the same state the maintainer applied.
- GitHub offers no managed state service with the required read/write/lock semantics.
- Another credential and another service are a cost for a one-maintainer project.

## Considered Options

- Encrypted remote backend
- Local state with an encrypted snapshot exported for automation
- Local state with the drift check run locally

## Decision Outcome

Chosen option: "Local state with the drift check run locally", because it is
the only option that keeps the publishing key's material off every automated
service while still leaving the state restorable, and the project accepts that
configuration drift is found when the maintainer looks rather than on a
schedule.

### Consequences

- Good, because the state is one artifact: committed, encrypted at rest, and read and written by the same local command.
- Good, because no automation credential can decrypt the state, so a compromised workflow cannot reach the future publishing key.
- Bad, because divergence from the declaration is discovered only when the maintainer runs the drift check; the Frame's CI drift report is withdrawn and recorded in chapter 11 (R-022).
- Bad, because losing the passphrase loses the state; the state can then only be rebuilt by re-adopting, and the backup is protected by the same key (R-023).
- Bad, because there is no lock: two simultaneous applies are not prevented, only recorded as out of scope (R-024).

### Confirmation

The adoption command refuses to run without a passphrase of at least 16
characters, and the declaration sets `enforced = true` for both state and plan,
so OpenTofu refuses to write plaintext. The delta's `@e2e` scenario reads the
state it produced through `tofu state list` and asserts the five bound
addresses; the committed file is ciphertext in the repository. Configuration
drift reporting is out of this change's scope (`report-configuration-drift`).
This change is confirmed instead by one operator-run real adoption after the
suite is GREEN: the maintainer runs `scripts/adopt-public-home.sh` against the
real organization, records the transcript under
`givn/changes/declare-public-home/evidence/real-adoption/`, commits the
resulting encrypted `infra/terraform.tfstate` and its backup, and review
verifies the five bound addresses with `tofu state list`.

## Pros and Cons of the Options

### Encrypted remote backend

- Good, because the local apply and a CI drift report can read one source of state.
- Good, because the state is encrypted at rest and access-controlled by the service.
- Bad, because it adds a service to operate or pay for.
- Bad, because it adds a credential that can read the material protecting the publishing key.

### Local state with an encrypted snapshot exported for automation

- Good, because no backend service is needed.
- Bad, because the snapshot is stale by definition, so drift since the export is reported as no drift.
- Bad, because the export is a manual step that is easy to forget.

### Local state with the drift check run locally

- Good, because no service and no extra credential exist.
- Good, because the passphrase never leaves the maintainer's machine, so the publishing key stays out of automation's reach.
- Bad, because drift is found only when someone runs the check.

## More Information

The topic's question log Q4 and its resolution of 2026-10-17; proposal D1;
`design.md` ("The applied state", "The adoption command", "Risks and follow-ups
recorded here"); chapter 7 (where the state and passphrase live) and chapter 8
(the encryption concept); ADR-0005 records the declaration mechanism this state
belongs to.
