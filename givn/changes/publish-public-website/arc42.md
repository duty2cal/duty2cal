# Arc42 assessment — publish-public-website

Independent assessment against all twelve rows of the arc42-docs chapter
selection table, read from this change's `proposal.md`, `specs/` and
`design.md` on 2026-10-25. The change publishes a public web surface that did
not exist and couples the release channel to it, so most chapters are affected.

| # | Chapter | Affected? | Reason / change |
|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The requirements overview names the Public website and the Site publication; quality goal 8 no longer describes the page as unpublished; the release owner and the crew member expectations name the published site, and the web reviewer joins the contributor stakeholder type. |
| 2 | 02 architecture-constraints | Yes | Added: publication into the Pages repository through the deploy key; the assembled `dist/site/` artifact with the header icon; whole-content replacement as one fast-forward commit under a lock; the reusable `site.yml` workflow whose two callers pass `PAGES_DEPLOY_KEY`, with `web.yml` as a trigger workflow without a `v*` trigger and `fetch-depth: 0`; the in-command fail-fast gate run inside a detached worktree, with the `@site-publication` exclusion and a lockfile-checked driver tree; build-time `git describe` stamping from that worktree with refusal of an unresolvable revision and of an unclean tree (`git status --porcelain --untracked-files=no`); the idempotent republish; the command-level `--since` trigger decision with a single owner for the site-input list; the required `--verify-url` serving check; the release gated on the site job and on the target carrying the tagged Built revision; the local git twin; `web/` and `ffi/wasm/` as user-receiving paths, and the `docs/assets/**` CC0-1.0 override for the icon. |
| 3 | 03 context-and-scope | Yes | The Public website becomes a live external surface: the publication flow and its partners (release maintainer, visitor, GitHub Pages, GitHub Releases), the git transport, the serving read (`--verify-url`) and hosting interfaces, and the corrected scope sentence naming the `publish-public-website` leaf. |
| 4 | 04 solution-strategy | Yes | New key decision "published page from one gated command", now including the idempotent republish, the required serving check and the reusable wrapper; technology rows for the published site and the release coupling; a quality-approach row; a statement that no ADR was minted. |
| 5 | 05 building-block-view | Yes | The page block gains the header icon/favicon left of the heading, the statements panel above the converter, the "Source and licence" panel below it and the prose footer; the driver serves the assembled output; a new "Site publication toolbox" section with its diagram (including the detached worktree, the serving read and the reusable workflow), its five commands, the reusable `site.yml` with its two callers and the two step files; the output member set is enumerated as eight members including `icon.png`. |
| 6 | 06 runtime-view | Yes | Two new scenarios — the Site publication (unclean-tree refusal with `--untracked-files=no`, `--since` decision, worktree checkout, build, gate, idempotent no-op, replace, read back, serving check) and the release with a `site` job and a `publish` job that verifies the target's tagged revision — and the release scenario updated. |
| 7 | 07 deployment-view | Yes | The deployment diagram, the node table, the browser output table (artefact plus page layout), the credential's consumer and the deployment consequences all describe the live page; a new "Public website and the site publication" subsection records the address, target, credential, worktree, gate, `--since` trigger, serving check, release coupling, member set and twin; the placeholder paragraph is corrected. |
| 8 | 08 crosscutting-concepts | Yes | New section "The Site publication: gated output, whole-content replacement, named revision, verified serving", including the trigger-as-argument rule, the revision's own tree, the idempotent republish, the split statement panels with the on-request download copy, the local icon and `img-src 'self'`, and the serving check; the credential's consumer now names `site.yml` and both callers, and the public-copy paragraph is corrected. |
| 9 | 09 architecture-decisions | Yes | Register note recording that no ADR was minted or amended, and the routing of the four candidates (publishing topology → chapter 7, release coupling → the release capability spec, publication mechanism → `design.md`, serving check → the publication capability spec). |
| 10 | 10 quality-requirements | Yes | Quality tree gains the published-site branch; QS-022 revised to the live page, its panel split and the on-request download copy; QS-031 extended with the lockfile guard and its scenario; QS-035 (gated publication, including a worktree that cannot be created), QS-036 (published revision traceability, including the newer default branch), QS-037 (release gated on the site deploy and on the target's tagged revision) and QS-038 (site-input trigger decision, with its three scenarios) added, and QS-039 added for the serving check. |
| 11 | 11 risks-and-technical-debt | Yes | R-043 and R-053 closed; R-055's stale clause corrected; R-045's exclusions name `dist/`; R-057 to R-065 added (write credential in CI, whole-content deletion, push is not served, gate flakiness, stand-in fidelity, copy/page landing window, the per-push runner start the trigger decision costs, stale worktrees or unusable comparison bases, and a driver tree borrowed from a different lockfile); R-057's consumers and R-059's mitigation revised for `site.yml` and the serving check; R-066 added for the serving check's propagation, cache and post-replacement failure; TD-022 to TD-024 added. |
| 12 | 12 glossary | Yes | No new term. **Public website**, **Site publication**, **Site output** and **Built revision** are realized with their code symbols (`site.yml`, `icon.png`), the site's lifecycle observations are recorded including the serving refusal, the serving check stays descriptive rather than a term, and the promotion-boundary section states the change introduces no term and no aggregate. |

## Chapter decisions

- Chapters 1–3 describe the published surface and its boundaries without
  implementation detail in the specification layer.
- Chapter 4 records the delivery strategy; chapter 5 the blocks; chapter 6 the
  flows; chapter 7 the deployment shape and its local twin.
- Chapter 8 records the gate, the whole-content replacement, the named revision,
  the idempotent republish, the serving check and the closed image policy as
  cross-cutting rules.
- Chapter 10 uses the command-driven scenarios as the evidence for the site
  quality scenarios; chapter 11 records every accepted residual risk and the
  three debt items.
- Chapter 12 is the durable vocabulary: the five public-website terms were
  promoted by the ideation Handoff and are now realized; the conversion
  aggregate remains the only aggregate.

## ADR qualification verdicts

Four candidates were assessed; none qualified. The serving check was assessed
in the design-review hardening of 2026-10-25; the other three verdicts are
unchanged in outcome, with their evidence updated for the reusable wrapper and
the target verification.

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "docs/arc42/07-deployment-view.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "An external Pages repository written from the monorepo through a write-enabled deploy key (chosen)",
      "A project site under duty2cal.github.io/duty2cal",
      "A hand-edited Pages repository"
    ],
    "architectural_impact": [
      "It fixes the public page's write target, its source of truth and the direction of the publishing credential"
    ],
    "durable_consequence": [
      "Reversal means a new canonical address and publish path, a credential change and re-pointing every public link"
    ],
    "lower_level_artifact": [
      "The choice and its rejected alternatives were already decided and recorded in the ideation seed's accepted decision; this change applies it",
      "The durable position is a deployment fact, recorded with the target, credential and rule set in chapter 7; design.md owns the mechanism"
    ],
    "existing_adr_check": [
      "The active register holds no record for the site's publication target; ADR-0003 covers downloadable archive platforms, ADR-0005 the declaration mechanism, ADR-0006 the state"
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
  "canonical_artifact": "givn/specs/publish-public-release/publish-public-release.feature",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "A release `site` job that calls the reusable site workflow, then a `publish` job (`needs: site`) whose command verifies the target carries the tagged Built revision before any `gh` call (chosen)",
      "The release's publication command publishing the site itself before the archive",
      "The release stays independent of the site"
    ],
    "architectural_impact": [
      "It orders two existing capabilities and adds a read of the target chapter 7 already fixes; it moves no component, authority or deployment boundary"
    ],
    "durable_consequence": [
      "Reversal is one job and one command's verification removed within one change; no migration or cross-component rework"
    ],
    "lower_level_artifact": [
      "The observable ordering and the refusal are a rule and scenario of the release capability; the mechanism is design.md's"
    ],
    "existing_adr_check": [
      "ADR-0004 covers draft-first immutable publication of a release, not its relation to a second publication"
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
  "canonical_artifact": "givn/changes/publish-public-website/design.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "One project-owned publication command with an extracted gate, a local git target, and a reusable workflow wrapper that both callers invoke (chosen)",
      "A reusable GitHub workflow whose jobs gate each other and an inline `gh release` step",
      "A third-party deploy action configured in the workflow",
      "An assembled dist/site artifact, the page source tree, or a tarball as the published artifact",
      "Build-time git describe stamping, a runtime fetch, or a hand-written revision"
    ],
    "architectural_impact": [
      "It chooses build-and-publish tooling inside the deployment boundary chapter 7 fixes; the reusable wrapper moves the toolchain and the deploy key into one host-side place without opening a new component or authority boundary"
    ],
    "durable_consequence": [
      "Reversal is a script and workflow edit inside one change's scope"
    ],
    "lower_level_artifact": [
      "design.md owns the commands, the wrapper's inputs, the artifact shape including icon.png, the stamping rule, the refusal paths, the serving check's retry bound and the local twin; the behaviour is the two capability specifications'"
    ],
    "existing_adr_check": [
      "No ADR owns build-and-publish tooling; prepare-release.sh and render-changelog.sh are design.md-owned by the same rule"
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
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/specs/publish-public-website/publish-public-website.feature",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "After the replacement, fetch the required --verify-url address with bounded retries and refuse unless the served page names the Built revision (chosen)",
      "Report only the pushed tree and leave serving to the host (rejected: a push the host never serves would be reported as a publication)",
      "Trust the host's reported deployment status or a separate post-deploy job (rejected: the failure would live only on the host and no scenario could observe it)"
    ],
    "architectural_impact": [
      "It adds one read of an address the use case already names; the write target, the credential, the deployment topology and the ownership boundaries are unchanged"
    ],
    "durable_consequence": [
      "Reversal is deleting one required argument and its step within one change; no migration, no interface other than the command's argument list"
    ],
    "lower_level_artifact": [
      "The refusal, what it names and when it happens are observable workflow behaviour owned by the capability specification's scenario and the use case's extension",
      "The fetch, the retry bound and the revision comparison are design.md's mechanism",
      "The product-behaviour falsification test applies: it is a gate in the publication workflow, not a component, authority or deployment boundary"
    ],
    "existing_adr_check": [
      "The active register holds no record for the publication's serving verification; ADR-0003 and ADR-0004 cover the release channel and its immutable publication, not the site's serving read"
    ]
  }
}
```

The dependency-free result: no ADR file, no register entry, no amendment. The
four candidates are routed once each, and their rationale is not copied into
the ADR archive.

## Status

STATUS: DONE
