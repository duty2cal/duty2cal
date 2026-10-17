# Public-home infrastructure — Handoff

Status: **published on 2026-10-17** with the user's confirmation (Q12). The
readable topic document, four seed files, two Persona promotions and six
glossary terms are published. No change was started.

## Use-case-to-permanent mapping

| Use case | Stable ID | Future permanent leaf |
|---|---|---|
| Adopt the existing public-home configuration | `adopt-existing-configuration` | `givn/specs/adopt-existing-configuration/usecase.md` |
| Apply a configuration change | `apply-configuration-change` | `givn/specs/apply-configuration-change/usecase.md` |
| Check the configuration drift | `check-configuration-drift` | `givn/specs/check-configuration-drift/usecase.md` |

Each `!` use case maps to its own leaf; no capability is placed inside another
goal. The proposal flow creates each leaf; ideation creates none.

## Durable-term proposals

All six are `confirmed` topic-locally. The user decides each for the durable
glossary.

| Term | Proposed durable definition | Contrast / anti-terms | Recommendation |
|---|---|---|---|
| Declared configuration | The organization's and its repositories' configuration as the repository's `infra/` states it should be. | Not the live configuration; not the product's build configuration. | Adopt |
| Configuration drift | A difference between the declared configuration and what the organization currently holds. | Not the glossary's **Drift check** (change-log comparison). | Adopt; keep the qualifier mandatory |
| Protection ruleset | The rules that constrain changes to a protected branch, with named bypass actors. | Not the legacy branch-protection setting as a separate object; not the gate itself. | Adopt |
| Deploy key | The write-enabled public key on the pages repository whose private half lets automation publish the site. | Not a personal SSH key; not `GITHUB_TOKEN`. | Adopt, marked as GitHub vocabulary |
| Adoption | The one-time act of placing an already-existing GitHub object under the declared configuration instead of creating it. | Not an apply; not creation; not the prior topic's contract adoption. | Adopt |
| Configuration change | A reviewed revision of the declared configuration that reaches GitHub through an apply. | Not a product change; not a release. | Adopt |

## Persona promotion

| Persona | Disposition | Consequence |
|---|---|---|
| `project-maintainer--organization-owner` | Propose promotion as a new project segment. | The project collection gains an administration review lens; the topic file stays as historical record. |
| `contributor--infrastructure-reviewer` | Propose promotion as a new project segment. | The project collection gains a review-without-authority lens. |
| `crew-member--personal-calendar-owner` | No promotion; already in the project collection. | The topic file stays as a historical topic stance. |

## Change seeds

Cut by deliverable guarantee; each can ship alone. Suggested start order is the
listed order.

### 1. `declare-public-home` — ready

- Use-case IDs: `adopt-existing-configuration`
- Topic: `public-home-infrastructure`
- Personas: `project-maintainer--organization-owner`, `contributor--infrastructure-reviewer`, `crew-member--personal-calendar-owner`
- Suggested start: 1; no seed dependency.
- Handoff: `Status: ready`
- Guarantee: the declared configuration names the organization, both
  repositories, the existing protections and the Pages source; the existing
  objects are bound in the applied state; the live configuration is unchanged.
- Accepted decisions: Frame (Q1), Personas (Q7), terms (Q8).
- Open questions: Q4 (where the applied state lives), Q5 (which values are
  intended), Q9 (classic protection versus ruleset). If Q9 consolidates, this
  seed binds the ruleset only and a later seed retires the classic protection.

### 2. `enable-repository-security` — ready

- Use-case IDs: `apply-configuration-change`
- Topic: `public-home-infrastructure`
- Personas: `project-maintainer--organization-owner`, `contributor--infrastructure-reviewer`
- Suggested start: 2; depends on seed 1.
- Handoff: `Status: ready`
- Guarantee: secret scanning, push protection and Dependabot security updates
  are on for `duty2cal/duty2cal`, declared and applied through a previewed,
  reviewed change; a refused apply changes nothing and a partial failure is
  reported with a stale state.
- Accepted decisions: the apply contract's guarantees.
- Open questions: Q3 (ruleset pull requests), Q10 (review gating), Q11
  (deletion). The security features themselves are not open.

### 3. `publish-site-plumbing` — ready

- Use-case IDs: `apply-configuration-change`
- Topic: `public-home-infrastructure`
- Personas: `project-maintainer--organization-owner`, `contributor--infrastructure-reviewer`, `crew-member--personal-calendar-owner`
- Suggested start: 3; depends on seed 1.
- Handoff: `Status: ready`
- Guarantee: the pages repository carries a write-enabled deploy key and the
  monorepo holds the matching `PAGES_DEPLOY_KEY` secret, declared and applied
  without touching the stock site; the credential has no consumer until the
  website topic.
- Accepted decisions: Q1's inclusion of the publishing plumbing.
- Open questions: Q2 (deploy-key bypass on the pages ruleset), Q10, Q11.

### 4. `report-configuration-drift` — ready

- Use-case IDs: `check-configuration-drift`
- Topic: `public-home-infrastructure`
- Personas: `project-maintainer--organization-owner`, `contributor--infrastructure-reviewer`
- Suggested start: 4; depends on seed 1; CI execution depends on Q4.
- Handoff: `Status: ready`
- Guarantee: a report names every difference it can compare, marks unreadable
  fields unchecked, and never concludes "no configuration drift" on an
  unavailable applied state; the live configuration is unchanged.
- Accepted decisions: the drift contract's guarantees.
- Open questions: Q4 (state access determines whether CI can run it).

## Publication on confirmation

1. Write `docs/ideation/public-home-infrastructure.md` — readable start to
   finish.
2. Write the four `changes/<seed>.md` files using the seed schema.
3. Promote the two new Personas by appending their records to
   `givn/personas/`; leave the crew-member topic file historical.
4. Add the approved terms to `docs/arc42/12-glossary.md`.
5. Report the published records. Start no change; the user runs the normal
   proposal flow per seed.
