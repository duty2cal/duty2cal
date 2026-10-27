# Arc42 assessment — add-vector-brand-assets

Independent assessment against all twelve rows of the arc42-docs chapter
selection table, read from this change's `proposal.md`, `specs/` and
`design.md` on 2026-10-27. The change swaps the rendered form of the existing
mark and adds one member to the Site output; it moves no component, capability
or deployment boundary, so the affected chapters are the ones that enumerate the
output, the page's assets and the mark's vocabulary.

| # | Chapter | Affected? | Reason / change |
|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The requirements overview, the quality goals and the stakeholder list are unchanged; the mark's rendered form is not a goal, a quality attribute or a new stakeholder, and the published page's goals already cover it. |
| 2 | 02 architecture-constraints | Yes | The assembled-output constraint now names the header's vector mark and its raster tab alternate, and the licence constraint names `docs/assets/duty2cal.svg` beside the raster, recording that the existing `docs/assets/**` CC0-1.0 override covers the new file with no licensing decision. |
| 3 | 03 context-and-scope | No | No new external system, interface, partner or user-facing surface: the page, the Pages host and the content check already exist, and the asset moves inside the boundary chapter 3 records. |
| 4 | 04 solution-strategy | Yes | The published-site technology row names both icon members, and the "published page from one gated command" key decision states that the page carries one mark in two forms, with the content check refusing a swap or a missing alternate. |
| 5 | 05 building-block-view | Yes | `web/` gains the vector header mark and the two typed tab declarations; `scripts/build-site.sh` copies both marks into a nine-member output; `scripts/check-site-content.sh` gains the mark, ordered-and-typed fallback, member and plain-vector refusals; the `index.html` element row records `src="./icon.svg"` and both declarations; the new asset `docs/assets/duty2cal.svg` is the mark's source. |
| 6 | 06 runtime-view | No | The publication flow is unchanged in order, participants and outcome; the new assertions run inside the existing page-content check step, so no scenario, sequence or state transition is added. |
| 7 | 07 deployment-view | Yes | The artefact row enumerates nine members with `icon.svg` and `icon.png`, the page-layout row names the vector mark, the build row copies both marks, the deployment diagram's assembler label names the mark pair, and the Site-publication paragraph adds a missing mark to the list of refusals. |
| 8 | 08 crosscutting-concepts | Yes | The page's local-image rule now covers both icons, states that the header mark is the vector while the raster stays the tab alternate, and records the content check's plain-vector and palette refusals; the profile cross-reference names both local images. |
| 9 | 09 architecture-decisions | Yes | The register's publication-mechanism summary now says nine members including the vector mark and its raster alternate, and a new routing note records the one candidate's `NOT_QUALIFIED` verdict. No ADR is minted and none is amended. |
| 10 | 10 quality-requirements | No | No quality scenario is added or changed, and no quality-tree branch moves; the change serves the page's existing rendering quality without new measurable scenarios, whose evidence is the content check and the browser baseline. |
| 11 | 11 risks-and-technical-debt | Yes | TD-026 added: one mark now lives in two files, nothing compares them on each revision, the fidelity measurement is a per-change manual step, and a render-based fidelity gate was rejected as a permanent renderer dependency in the publication's gate. |
| 12 | 12 glossary | Yes | New term **Brand mark**; **Public website** and **Site output** are realized with both icon members (`icon.svg` as the mark and the tab's primary icon, `icon.png` as the raster alternate) and with the nine-member output. |

## Chapter decisions

- Chapters 2, 4, 5, 7 and 8 describe the same artifact at different levels:
  the constraint, the strategy, the blocks, the deployment shape and the
  cross-cutting asset rule. Each was edited only where it enumerated the icon
  or the member set, so no chapter restates another.
- Chapters 3 and chapter 6 stay untouched because the change adds no boundary and
  no flow; chapter 1 and chapter 10 stay untouched because no goal, stakeholder
  or quality scenario changes.
- `scripts/assemble_features.py` gains the delta half of a rule chapter 5 already
  records — a feature carrying `@documentation` is not executed. The chapter's
  statement was already correct, so it needed no edit; the defect and its
  mechanical proof live in `design.md`.
- Chapter 9 is a register, not an ADR container: it received the routing note
  and the corrected member count, and no ADR file was created.
- Chapter 11 records the one durable consequence of the choice — two files that
  must stay one mark — as technical debt rather than as a new risk, because the
  page-content gate already refuses the failure modes that are mechanically
  detectable.
- Chapter 12 keeps the promotion boundary: **Brand mark** is a realized project
  term, not a new aggregate, and the conversion aggregate remains the only one.

## ADR qualification verdicts

One candidate was assessed and did not qualify. ADR-0001 to ADR-0007 were
searched first; none records a brand asset.

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
  "canonical_artifact": "givn/changes/add-vector-brand-assets/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "A vector master beside the frozen raster, with raster as the tab fallback (chosen)",
      "Keep the raster as the only mark",
      "Serve the vector only and drop the raster"
    ],
    "architectural_impact": [
      "No component, contract, dependency direction or deployment boundary moves; the asset and its two declarations sit inside the page-and-asset shape chapter 7 already fixes"
    ],
    "durable_consequence": [
      "Reversal is one asset file and three markup lines inside one change; no migration, no interface change, no cross-command coordination"
    ],
    "lower_level_artifact": [
      "The mark, its palette and the tab fallback are design facts owned by design.md, with docs/design/design-system.md as supporting context rather than a second destination",
      "The observable publication requirement (header mark, tab icon, output members) is owned by the Site publication's use case and capability specification"
    ],
    "existing_adr_check": [
      "ADR-0001 records the licences, not an asset",
      "ADR-0007 records the browser conversion facade, not the page's artwork",
      "No active or archived record covers a brand asset or its tab declaration"
    ]
  }
}
```

Routing: `CANONICAL_ARTIFACT` → exactly one canonical artifact,
`givn/changes/add-vector-brand-assets/design.md`. The durable
`docs/design/design-system.md` is supporting context for that rationale, not a
second destination. No ADR file, chapter-09 register row or
`docs/arc42/adr/README.md` entry is created, and no archived record is amended.

## Status

STATUS: DONE
