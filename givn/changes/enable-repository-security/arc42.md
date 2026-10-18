# arc42 impact — enable-repository-security

Assessment walked over all twelve rows against the change's `proposal.md`,
`specs/apply-configuration-change/` and `design.md` on 2026-10-18.

| # | Chapter | Affected | Reason / what changed |
|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Requirement 5 and quality goal 6 now name the repository's security settings and the preview-and-apply of a configuration change. |
| 2 | 02 architecture-constraints | Yes | Added: the monorepo's security settings are declared (`security_and_analysis` for the two scanning fields, dedicated resources for the two Dependabot settings, no advanced security on a public repository); a configuration change is a saved plan applied unchanged, and the state is committed only after a green apply. |
| 3 | 03 context-and-scope | Yes | The project-side interface now includes the apply's write (adoption, preview and the drift check remain reads); the declared-configuration diagram and partner table name the security settings and the one previewed write. |
| 4 | 04 solution-strategy | Yes | The infrastructure-as-code decision now includes the security settings and the saved-plan apply; the reproducibility approach row names the preview, the apply and the post-success state commit. |
| 5 | 05 building-block-view | Yes | `infra/` gains the security resources and a second adoption batch; the apply is named as a block; the fake gains an opt-in write sandbox; two step files are added. |
| 6 | 06 runtime-view | Yes | New scenario "The declared security settings are previewed and applied" covers adoption, preview, apply, refusal, partial failure and the post-apply empty plan. |
| 7 | 07 deployment-view | Yes | The maintainer's workstation now writes one previewed change to GitHub behind a read-only refresh-only preflight; the plan file and the two JSON reports are transient and git-ignored; the state is committed only after a green apply, and a partial state is marked stale and is not committed. |
| 8 | 08 crosscutting-concepts | Yes | Added "Declared security settings on the public repository" and "Apply semantics: the saved plan, the stale marker and partial reporting"; the read-only adoption concept is preserved. |
| 9 | 09 architecture-decisions | No | No new ADR qualifies. Four candidate decisions are refinements owned by `design.md`, the use case's applied-revision rule and chapter 2; see the qualification verdicts below, and the register is unchanged. |
| 10 | 10 quality-requirements | Yes | Added QS-019 (declared security settings are applied and recorded) and QS-020 (a refusal changes nothing; a partial failure names the applied and failed settings), plus the matching quality-tree nodes. |
| 11 | 11 risks-and-technical-debt | Yes | Added R-026 (push protection may be refused on a free public repository; the public-repo documentation and the read-only preflight are recorded), R-027 (the security block is provider-version-specific and 6.13.0 cannot write the remaining checks), R-028 (the fake's write sandbox is not GitHub's policy), R-029 (a partial state is marked stale and must not be committed), R-030 (private vulnerability reporting stays manual) and R-031 (the saved-plan window is now the preflight window), plus TD-013 (the second adoption batch is an operator step). |
| 12 | 12 glossary | Yes | Added **Security settings** and refined **Configuration change**, which now names the saved plan, the apply and the state it records. |

## ADR qualification verdicts

Four candidates were tested; none passes all five mandatory dimensions, so no
ADR file is created and no register entry changes.

### Candidate: adopt the already-on Dependabot settings through dedicated resources and a second batch

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/enable-repository-security/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Adopt both settings (proposal D1 option 1)",
      "Declare both without binding them (option 2)",
      "Leave both unmanaged (option 3)"
    ],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [
      "The decision is a declaration and batch detail: it names two resources and two import identifiers inside the boundary ADR-0005 already fixed"
    ],
    "existing_adr_check": [
      "ADR-0005 records the declaration mechanism; ADR-0006 records the state; neither is refined or replaced"
    ]
  }
}
```

Reversal is one change: remove two resources and two batch lines, and either
adopt or re-create the settings. It fixes no component, contract or deployment
boundary. The complete rationale is owned by `design.md` ("The declaration",
"The adoption batch") and the proposal's D1.

### Candidate: apply the declaration as a saved plan behind a read-only preflight, with no wrapper command

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/enable-repository-security/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Saved plan plus tofu apply (chosen)",
      "A wrapper command that re-plans and compares before applying",
      "Plain tofu apply without a saved plan"
    ],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [
      "The local apply mechanism is a design technique; ADR-0005 already fixes that the apply runs locally with the declaration's own tooling"
    ],
    "existing_adr_check": [
      "ADR-0005 fixes the declaration mechanism and the local apply; no boundary moves"
    ]
  }
}
```

**FAIL reason (durable consequence):** the residual live-change window (R-031)
is accepted for this change and is recorded in the risk register and the use
case, not carried as a durable architectural promise; a wrapper would need its
own decision. **FAIL reason (architectural impact):** the choice is
implementable and reversible within one change and does not constrain a second
component; the deploy/credential boundary it touches was already fixed by
ADR-0005 and chapter 2.

### Candidate: declare security settings on the public repository

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "docs/arc42/02-architecture-constraints.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Turn the settings on by hand",
      "Declare and apply them through the reviewed change flow"
    ],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [
      "The requirement that the public repository's credential surface is closed by declared configuration is a quality constraint"
    ],
    "existing_adr_check": [
      "ADR-0005 already establishes that the security features belong to the declared configuration"
    ]
  }
}
```

It is a quality constraint on the declared configuration, recorded in chapter 2
and the delta's scenarios, not a new architectural boundary.

### Candidate: the applied revision is the committed state plus the declaration revision, not a value carried in the declaration

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/changes/enable-repository-security/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "The commit pairs the declaration revision with the state it applied (chosen)",
      "A terraform_data/null_resource carrying var.declared_revision in the state",
      "A separate revision record outside the declaration"
    ],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [
      "The pairing is recorded by the use case's applied-revision rule and the design's apply procedure; the commit is the record, so no new durable data boundary is introduced"
    ],
    "existing_adr_check": [
      "ADR-0006 records that the state is committed with the declaration; the applied-revision reading refines it without changing the boundary"
    ]
  }
}
```

**FAIL reason (durable consequence):** the alternative would add a plan diff on
every commit and duplicate what the commit already proves; no durable recording
format or second component depends on it. **FAIL reason (architectural impact):**
the choice is a reading of ADR-0006's commit pairing, not a new boundary. A
later change that introduces a durable revision record qualifies on its own.

## Status

STATUS: DONE
