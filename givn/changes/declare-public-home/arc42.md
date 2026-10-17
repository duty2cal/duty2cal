# Arc42 impact assessment — declare-public-home

Assessed against the change's `proposal.md`, `specs/` and `design.md` on
2026-10-17, walking all twelve chapters in order.

| # | Chapter | Affected? | Reason / what changed |
|---|---|---|---|
| 1 | introduction-and-goals | Yes | Added the reproducible-public-home requirement and a quality goal: the configuration is declared, reviewable and rebuildable, and adoption changes nothing live. |
| 2 | architecture-constraints | Yes | Added the OpenTofu/provider floor and pin, the encrypted committed state, the passphrase/token boundary, no apply in Actions, no locking, read-only adoption, and the offline fake-GitHub test constraint. |
| 3 | context-and-scope | Yes | Added the declared configuration, its read-only adoption and the applied state as a project-side interface with GitHub, with its own context diagram and partner table. |
| 4 | solution-strategy | Yes | Added infrastructure as code for the public home plus the technology choices and the reproducibility quality approach, citing ADR-0005 and ADR-0006. |
| 5 | building-block-view | Yes | Added the public-home declaration toolbox: `infra/`, the adoption command, the test setup, the fake GitHub API and the two step-definition files. |
| 6 | runtime-view | Yes | Added the adoption runtime scenario: preflight, staged imports, refresh verification, atomic binding, refusals. |
| 7 | deployment-view | Yes | Added where the declaration, the encrypted state, the passphrase and the token live, and the deployment consequences (local-only drift, re-adoption recovery, no CI service). |
| 8 | crosscutting-concepts | Yes | Added declared-vs-applied state, all-or-none adoption, state encryption at rest, and read-only adoption against the live organization. |
| 9 | architecture-decisions | Yes | Added ADR-0005 (declaration mechanism) and ADR-0006 (encrypted local state and local drift), plus both indexes. |
| 10 | quality-requirements | Yes | Added the reproducibility branch of the quality tree and quality scenarios QS-016 (adoption binds and leaves live unchanged), QS-017 (state is ciphertext) and QS-018 (batch atomicity). |
| 11 | risks-and-technical-debt | Yes | Added R-020 to R-025: state responsibility, provider version and import identifiers, withdrawn CI drift, passphrase loss, no locking, tamperable public ciphertext. |
| 12 | glossary | Yes | No new terms; updated the code symbols for **Adoption**, **Declared configuration** and **Protection ruleset**, and recorded that the remaining infrastructure terms stay planned for later seeds. |

## ADR qualification verdicts

### ADR-0005 — Declare the public home's GitHub configuration with OpenTofu and the GitHub provider

```json
{
  "qualification": "QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "NEW_ADR",
  "canonical_artifact": null,
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "OpenTofu with integrations/github",
      "safe-settings (a deployable GitHub App with no Pages management)",
      "raw API scripts (no preview, no drift detection)"
    ],
    "architectural_impact": [
      "infra/ becomes a declaration owned by the repository and reviewed with the code",
      "the provider is the only writer of GitHub configuration",
      "the administer token and the apply stay on the maintainer's workstation, never in Actions"
    ],
    "durable_consequence": [
      "reversal means rewriting the declaration, re-adopting every object under other tooling and re-recording the state",
      "the choice constrains enable-repository-security, publish-site-plumbing and report-configuration-drift"
    ],
    "lower_level_artifact": [
      "design.md owns the file layout, the adoption command and the state mechanics, not the choice of mechanism or its alternatives",
      "chapter 4 records the chosen strategy but not the decision record"
    ],
    "existing_adr_check": [
      "searched ADR-0001 to ADR-0004 and the ADR archive: none covers infrastructure declaration or its mechanism",
      "result: NEW_ADR"
    ]
  }
}
```

### ADR-0006 — Keep the applied state local, passphrase-encrypted and committed, and check configuration drift locally

```json
{
  "qualification": "QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "NEW_ADR",
  "canonical_artifact": null,
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "encrypted remote backend",
      "local state with an encrypted snapshot exported for automation",
      "local state with the drift check run locally (chosen)"
    ],
    "architectural_impact": [
      "the applied state is a committed ciphertext opened only by a machine-local passphrase",
      "no automation may hold the passphrase, so the future publishing key stays out of a workflow",
      "the Frame's scheduled and per-push drift report is withdrawn; the drift capability becomes local"
    ],
    "durable_consequence": [
      "moving to a backend later requires a state migration and a new credential or service",
      "the decision constrains enable-repository-security, publish-site-plumbing and report-configuration-drift"
    ],
    "lower_level_artifact": [
      "design.md owns the encryption block, the passphrase variable and the staging workspace, not the boundary or its alternatives",
      "the passphrase boundary and the withdrawn CI capability are cross-change constraints, not local mechanism"
    ],
    "existing_adr_check": [
      "searched ADR-0001 to ADR-0004 and the ADR archive: none covers state storage, secrecy or drift execution",
      "result: NEW_ADR"
    ]
  }
}
```

## Status

STATUS: DONE
