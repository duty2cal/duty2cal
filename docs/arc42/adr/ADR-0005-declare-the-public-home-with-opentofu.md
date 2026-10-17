---
status: accepted
date: 2026-10-17
decision-makers: duty2cal maintainer (organization owner)
consulted: none
informed: none
---

# Declare the public home's GitHub configuration with OpenTofu and the GitHub provider

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

The alternatives are real and were compared in the topic's Frame: OpenTofu with
`integrations/github`, `safe-settings` (a deployable GitHub App with no Pages
management), and raw API scripts (no preview, no drift detection). The choice
fixes a component and a dependency boundary — a declaration under `infra/`
owned by the repository, a provider that is the only writer of GitHub
configuration, and an apply that never runs in Actions — and it constrains at
least three future changes (the security features, the publishing plumbing and
the drift report), so reversal would mean rewriting the declaration, adopting
the objects again under different tooling and carrying the decision across
those changes. `design.md` owns the concrete mechanism (file layout, the
adoption command, the state); it does not own the choice of mechanism or its
alternatives. The register holds ADR-0001 (licence), ADR-0002 (layer model),
ADR-0003 (release channel) and ADR-0004 (immutable publication); none covers
infrastructure declaration, so the route is a new record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort because: the declaration, the applied state and the adoption batch are tied to the chosen tool, and a replacement tool would have to adopt every object again and re-record the state.
- [x] **Shapes quality attributes** - It materially affects reviewability and reproducibility because: the configuration becomes diffable text in the same history as the code and can be rebuilt from that history, which is the point of the change.
- [x] **Structural** - It defines a boundary, contract, or dependency direction because: `infra/` owns the declaration, the provider is the only writer of live configuration, and the administer token and apply stay on the maintainer's machine rather than in Actions.
- [x] **Must be shared** - Developers unfamiliar with it would systematically violate the architecture because: they would change a setting by hand in the web interface, hand-edit the state, or add an apply to a workflow, all of which contradict the declaration.

## Context and Problem Statement

The organization and its repositories are configured by hand, so a change such
as the 2026-10-17 repository transfer happens as unrecorded clicks. The project
needs one mechanism that makes the intended configuration reviewable text,
previews a difference before it changes anything, and can rebuild what GitHub
allows to be rebuilt — without giving an automated pipeline the token that
administers the repository.

## Decision Drivers

- The intended configuration must be reviewable and diffable beside the code it supports.
- A change must be previewable before it changes anything.
- The token that administers the repository must not be stored in the repository it administers.
- The mechanism must cover every object the topic needs, including the Pages source and the deploy key.
- Two repositories and one organization do not justify operating a service.

## Considered Options

- OpenTofu with the `integrations/github` provider
- `safe-settings` (a deployable GitHub App)
- Raw API scripts

## Decision Outcome

Chosen option: "OpenTofu with the `integrations/github` provider", because it
is the only compared mechanism that covers every in-scope object — including the
Pages source and the deploy key — while offering a declarative preview and a
recorded state, and it runs locally so the administer token never enters the
repository.

### Consequences

- Good, because the declaration is text in the repository: an infrastructure change is reviewed in the same history as the workflow or document it supports.
- Good, because a preview names every difference before anything changes, and the recorded state lets the same comparison be repeated later.
- Good, because a run happens on one workstation; the administer token and the state passphrase stay there.
- Bad, because the tooling brings a state file whose integrity and secrecy become the project's responsibility; the passphrase decision is recorded in ADR-0006 and its consequences in chapter 11 (R-020).
- Bad, because the provider's object coverage and import identifiers are version-specific, so a provider upgrade is a reviewed change of its own (R-021).

### Confirmation

`tofu validate` runs in the adoption command's preflight and in the setup task;
the delta's `@e2e` scenario drives a real `tofu import` of every adopted object
against the local fake API and asserts the resulting state. A hand-made change
to a declared setting is not prevented by the mechanism — it is surfaced by the
local drift check, which is confirmed by running it against the real
organization and recording the result as the change's evidence.

## Pros and Cons of the Options

### OpenTofu with the `integrations/github` provider

- Good, because it covers the organization, both repositories, rule sets, the Pages source, the deploy key and the secret in one declaration.
- Good, because a preview and a state make a difference visible before it is applied and comparable afterwards.
- Good, because it runs locally, so the administer credential is never stored in the repository.
- Bad, because the state must be kept secret and restorable by the project itself.

### `safe-settings`

- Good, because it is declarative YAML and needs no local tooling.
- Bad, because it is a GitHub App that must be deployed and installed in the organization.
- Bad, because it cannot manage the Pages configuration the topic needs.

### Raw API scripts

- Good, because it needs no new tool and no state.
- Bad, because it offers no preview and no difference detection, so it recreates the problem the topic exists to solve.

## More Information

The topic's Frame and research (`givn/ideation/public-home-infrastructure/frame.md`,
`private/opentofu.md`); proposal D1, D2 and D3; `design.md` ("Technology
decisions", "The declaration", "The adoption command"); chapter 2 (the no-apply
constraint), chapter 4 (the strategy) and chapter 8 (the state-encryption
concept); ADR-0006 records where the state lives and how it is protected.
