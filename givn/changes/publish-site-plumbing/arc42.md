# Arc42 assessment: publish-site-plumbing

Assessed against `proposal.md`, `specs/apply-configuration-change/` and
`design.md` on 2026-10-19; the organization deploy-key prerequisite delta was
reassessed on 2026-10-20.

| # | Chapter | Affected? | Reason / change |
|---|---|---|---|
| 1 | Introduction and goals, quality requirements | Yes | Goal 5 and quality goal 6 now name the pages publishing credential and the pages rule set; the organization-owner stakeholder expects the reviewed plumbing and the passphrase now also guarding the only recoverable copy of the publishing key. |
| 2 | Architecture constraints | Yes | Seven constraints added: the in-declaration Ed25519 key pair and its provider, the private key existing only in the encrypted state and its loss consequence, the pages rule set's actor-type bypass, the create guard before preview and apply, the live-read classification of a failed create, and the organization deploy-key policy manual prerequisite; the suite's mirror constraint now names both providers. |
| 3 | Context and scope | Yes | The public-home diagram and partner table name both rule sets, the deploy key and its secret, the future publishing workflow as the credential's consumer, and the creation guard's output. No product boundary changes. |
| 4 | Solution strategy | Yes | The infrastructure bullet and the technology table add the publishing plumbing, the second provider and the state's new role; the reproducibility approach names the creation guard. |
| 5 | Building block view | Yes | `infra/pages_publish.tf`, the second provider in the toolbox, the fake's new endpoints and overlays, the setup script's second mirror, and one new step module pair are added; the existing profile and security base priming is adjusted. |
| 6 | Runtime view | Yes | New scenario "The pages publishing plumbing is previewed and created" with its guard, targeted plan, create-time re-check, three creates, post-apply read-back, failure classification and state commit. |
| 7 | Deployment view | Yes | The declared configuration, its diagram and nodes, a new subsection for the credential and rule set, the manual organization deploy-key policy prerequisite, and the 422 partial-apply recovery; the passphrase-loss consequence is corrected. |
| 8 | Cross-cutting concepts | Yes | New section "The pages publishing credential and its rule set"; state encryption, apply semantics, the post-apply read-back and failure reporting now cover a live credential in the state and create-only revisions. |
| 9 | Architecture decisions | Yes | ADR-0006 is amended in place (AMEND_ADR) for the credential now held by the state; both indexes updated. The other candidates were routed to lower-level artifacts (verdicts below). |
| 10 | Quality requirements | Yes | New quality scenario QS-034 for the pages publishing plumbing (preview, guard, apply, stock site unchanged, refusal, partial outcome, read-back failure). |
| 11 | Risks and technical debt | Yes | R-023 corrected (the key cannot be re-adopted); R-051 to R-056 added (credential at rest, actor-type bypass, dormant credential, create-without-state-record, read-back failure and guard window, fake cannot prove acceptance, organization deploy-key policy); TD-021 records the changelog parser-row gap. |
| 12 | Glossary | Yes | **Pages publishing plumbing** is added; **Deploy key** is realized with its code symbols and its state-only private half; **Protection ruleset** is refined with the pages rule set and its actor-type bypass. No new aggregate. |

## ADR qualification verdicts

```json
{
  "qualification": "QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "AMEND_ADR",
  "canonical_artifact": null,
  "target_adr": "ADR-0006",
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Generate the pair with tls_private_key inside the declaration",
      "Generate the pair by hand and pass it as sensitive variables",
      "Generate the pair with an external script"
    ],
    "architectural_impact": [
      "The applied state becomes the only copy of a live write credential, so who may read the state decides who may publish to the site"
    ],
    "durable_consequence": [
      "GitHub never returns a deploy key's private material; losing the state or passphrase means replacing the live deploy key and secret",
      "Reversal requires replacing the live credential again"
    ],
    "lower_level_artifact": [
      "design.md owns the provider, algorithm, attribute wiring and update values",
      "ADR-0006 owns the state boundary and already anticipated the publishing key"
    ],
    "existing_adr_check": [
      "The active register and docs/arc42/adr/ hold no credential-material record; no archive directory exists",
      "ADR-0005 owns only the declaration mechanism"
    ]
  }
}
```

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/specs/apply-configuration-change/publish-site-plumbing.feature",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Bypass the deploy key with deletion, non-fast-forward and linear history (chosen, proposal D1)",
      "Protect only deletion and non-fast-forward, no bypass",
      "No rule set on the pages repository"
    ],
    "architectural_impact": [
      "It sets one repository's protection policy; it moves no component, authority or deployment boundary beyond the declared resource"
    ],
    "durable_consequence": [
      "Reversal is a one-change edit of one resource; no migration or cross-component coordination"
    ],
    "lower_level_artifact": [
      "The observable protection outcome is the capability specification's rule; the HCL values and actor shape are implementation detail under design.md, not a second canonical destination"
    ],
    "existing_adr_check": [
      "No existing ADR covers ruleset policy; ADR-0005 covers the declaration mechanism only"
    ]
  }
}
```

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/publish-site-plumbing/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Generate the pair with the non-ephemeral tls_private_key resource (chosen)",
      "Use the provider's ephemeral tls_private_key resource",
      "Generate the pair by hand and pass it as sensitive variables",
      "Generate the pair with an external script"
    ],
    "architectural_impact": [
      "It chooses the local mechanism that produces the key material; the state boundary is already fixed by ADR-0006"
    ],
    "durable_consequence": [
      "Reversal is a declaration edit and a re-apply; the state copy of the secret's plaintext remains in provider 6.13.0 either way"
    ],
    "lower_level_artifact": [
      "design.md owns the provider, the algorithm, the attribute wiring and the rejected ephemeral alternative; no ADR owns the key-generation mechanism"
    ],
    "existing_adr_check": [
      "ADR-0006 owns the state boundary and the credential-at-rest consequence; no ADR owns how the pair is generated"
    ]
  }
}
```

The ephemeral `tls_private_key` resource does not remove the state copy: the
GitHub provider's `github_actions_secret.value` is an SDKv2 optional/sensitive
input with no write-only attribute in `6.13.0`, so the secret's plaintext still
reaches `terraform.tfstate`. The choice between generating resources is
therefore implementation detail with one canonical home, `design.md` ("The key
pair and what the state now holds", "Decisions").

## Status

STATUS: DONE
