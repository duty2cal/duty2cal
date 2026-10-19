---
status: accepted
date: 2026-10-17
amended: 2026-10-19
decision-makers: duty2cal maintainer (organization owner)
consulted: none
informed: none
---

# Keep the applied state local, passphrase-encrypted and committed, and check configuration drift locally

> **Amendment 2026-10-19.** The state now holds the only copy of a live
> credential — the pages deploy key's private half, generated inside the
> declaration — so the consequences below are extended, not replaced. This
> record is amended in place, not superseded; see "Amendment — 2026-10-19".

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

## Amendment — 2026-10-19: the publishing key is in this state

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: AMEND_ADR
canonical_artifact: null
target_adr: ADR-0006
replacement_adr: null

- **Alternatives.** The key pair can be generated inside the declaration
  (`tls_private_key` from `hashicorp/tls`), generated by hand and supplied as
  sensitive variables, or generated by an external script. The chosen option is
  the first; the other two leave an unrecorded step outside the reviewed plan.
- **Architectural impact.** The state stops being only a record of bindings and
  becomes the only copy of a live credential: who may read the state now decides
  who may publish to the site, which is a security and deployment boundary.
- **Durable consequence.** GitHub never returns a deploy key's private material,
  so losing the state or the passphrase is unrecoverable for the key; recovery
  means generating a new pair and replacing the live deploy key and secret. A
  reversal would replace the live credential again.
- **Lower-level artifact.** `design.md` owns the generation mechanics (the
  provider, the algorithm, the attribute wiring, the update values). The
  boundary — that the state holds this material and what follows from it — is
  this record's, and no other document owns it.
- **Existing-ADR check.** The active register and the ADR directory were
  searched: no record covers credential material in the state, and ADR-0005
  owns only the declaration mechanism. The register names no archived directory.
  The route is `AMEND_ADR` with ADR-0006 as the target; no new ADR is minted
  and nothing is superseded.

### What changed

The change to create the pages publishing plumbing (publish-site-plumbing)
generates an Ed25519 key pair inside the declaration, registers the public half
as a write-enabled deploy key on `duty2cal/duty2cal.github.io`, and stores the
private half as the monorepo's `PAGES_DEPLOY_KEY` secret, which the GitHub
provider seals with the repository's public key before sending it. The state
therefore now holds the private key twice — as `tls_private_key`'s
`private_key_openssh` and as the secret's `value` — both inside the same
passphrase-encrypted file, and the plan file that carries the secret value is
encrypted and deleted after the apply. The rule set that admits write-enabled
deploy keys is protection policy, routed to the capability specification and
`design.md`; it crosses no boundary this record owns.

### Amendment consequences

- Good, because the publishing credential is created by a reviewed plan instead
  of a hand-made key, and its material never exists in plaintext outside the
  maintainer's environment.
- Bad, because losing the passphrase loses the private key: re-adoption can
  rebuild the bindings, but the deploy key's material cannot be read back from
  GitHub, so the credential is replaced — a rotation that Q11 leaves to a
  separate decision (chapter 11, R-051).
- Bad, because the state's secrecy now protects a live write credential, not
  only a record of identity; a reader who also knows the passphrase can push to
  the pages repository as the deploy key (chapter 11, R-052).
- Bad, because the private key appears twice in one state, so a review that
  treats the state as ciphertext is the only protection; no printed or exported
  copy is made.

### Amendment confirmation

The delta's `@e2e` apply scenario reads the state's three new addresses with
`tofu state list` and asserts the repository state file is ciphertext, the
apply's secret write carries a sealed-box ciphertext for the fetched public key
and no PEM header, and the rule set's read carries the `DeployKey` bypass. The
one-time operator apply after GREEN records the scrubbed transcript and commits
the resulting encrypted state; review verifies the three addresses and that
the committed file remains ciphertext.

## More Information

The topic's question log Q4 and its resolution of 2026-10-17; proposal D1;
`design.md` ("The applied state", "The adoption command", "Risks and follow-ups
recorded here"); chapter 7 (where the state and passphrase live) and chapter 8
(the encryption concept); ADR-0005 records the declaration mechanism this state
belongs to. The amendment of 2026-10-19 is recorded by the change to create the
pages publishing plumbing (publish-site-plumbing) and its `design.md` ("The key
pair and what the state now holds", "Decisions").
