# arc42 Documentation Update: publish-organization-profile

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The public home's requirement and the organization-owner/reader stakeholder expectations now include the profile, its default security policy, the manual private-reporting prerequisite and the manual pin step; a truthful-public-copy quality goal is added. | Requirement 5 names the profile and the created-when-absent rule; quality goal 8 adds truthful public copy; the crew-member, organization-owner and reviewer rows name the profile. |
| 2 | 02 architecture-constraints | Yes | The change adds content-publishing, licensing, third-party-resource, naming and manual-step constraints. | `org/**` is CC0-1.0 and the third-party Unicode data is attributed to Unicode-3.0; the profile is published by the declaration; no third-party embeds and absolute links; no provider name; pinned repositories and private vulnerability reporting are manual. |
| 3 | 03 context-and-scope | Yes | The organization profile page and the default security policy are a new public-facing surface; the pinned and private-reporting steps are new manual interfaces. | The public-home section, partner table, technical-context table and scope boundary name the profile repository, the profile reader and the manual pin. |
| 4 | 04 solution-strategy | No | The declaration mechanism and the apply strategy are unchanged; this is another object and two files under an existing strategy. | — |
| 5 | 05 building-block-view | Yes | `org/`, `infra/org_profile.tf`, the fake's create/contents overlays, the adoption/setup declaration exclusions and the new step modules are new or changed blocks. | The public-home toolbox diagram and table gain `org/`, the profile resources, the extended fake, the profile steps and the workspace-shape rule. |
| 6 | 06 runtime-view | Yes | The preview, apply and publication of the profile is a new runtime flow with a concrete provider call sequence and the two manual operator steps. | A new scenario, "The organization profile is previewed, applied and published", with a Mermaid flow and six steps; its notes record the inherited pre-security fixture and the journal offset. |
| 7 | 07 deployment-view | Yes | A third repository and two files are deployed; pinned repositories and private vulnerability reporting are new manual deployment steps. | The declared-configuration text names the profile and its `org_source_dir` source; a profile-repository table, the manual private-reporting and pinned-repository steps and deployment consequences are added. |
| 8 | 08 crosscutting-concepts | Yes | Public copy, default community health files and declared-versus-manual objects are new cross-cutting concepts. | Three concept sections; the declared-configuration concept now names creation of an absent declared object. |
| 9 | 09 architecture-decisions | No | No ADR qualifies: the declaration boundary is ADR-0005's, the state is ADR-0006's and the platform is ADR-0003's; the candidate's single canonical home is chapter 7. | The register and `adr/README.md` are unchanged; the `org/**` licence is not an ADR candidate and its canonical home is `REUSE.toml`. |
| 10 | 10 quality-requirements | Yes | Two new quality scenarios state the profile's truthfulness and the default security policy. | `QS-022` (truthful public copy) and `QS-023` (default security policy) plus the quality-tree node `rp6`. |
| 11 | 11 risks-and-technical-debt | Yes | Profile-copy drift, the manual private-reporting prerequisite, the untestable GitHub fallback, manual pinning and the fake's limited proof are new risks; disclaimer duplication and the unpublished remaining health files are debt. | `R-030`, `R-032`–`R-035`, `TD-014`–`TD-015`. |
| 12 | 12 glossary | Yes | Seven public-presentation terms are introduced. | `Default security policy`, `Default community health files`, `Organization profile`, `Organization profile repository`, `Pinned repositories`, `Profile content`, `Reviewed public copy`. |

## Notes

- `docs/arc42/README.md` needs no change: no chapter was added, removed or
  renamed, so the index's twelve rows still match the files.
- The profile page is rendered by GitHub, not by this system. Its content is
  verified through the GitHub REST API response, not through a browser
  screenshot; there is no Visual Design Contract screen owned by the project.
- The public copy's source of truth is `org/`; the profile repository is
  output. The content-invariant scenario compares the published bytes with the
  reviewed `org/` source and asserts the positive claims (local conversion,
  Windows x86_64 only, no browser claim, the crew-duty-plan input, the
  non-affiliation sentence, the stable release and repository links, the
  Unicode-3.0 attribution, absolute links and no third-party embeds), so a
  future edit cannot silently reintroduce the draft's browser or three-platform
  claim. The absence of the excluded provider name is a review control, not a
  stored deny-list.
- The operator's real apply after GREEN remains the acceptance evidence for
  GitHub's own acceptance and rendering (R-035); the fake proves the provider's
  calls, the declaration and the published content.

ADR action: NONE
ADR target: none

```text
qualification: NOT_QUALIFIED
alternatives: PASS
architectural_impact: FAIL
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: CANONICAL_ARTIFACT
canonical_artifact: docs/arc42/07-deployment-view.md
target_adr: null
replacement_adr: null
```

The candidate — source the public profile and default security policy from
`org/` and publish them through the existing declaration — has real
alternatives (hand-edit `.github`; a separate publishing workflow), and a
durable consequence for future text changes. It fails `architectural_impact`
because the declaration boundary, the single-writer rule and the applied state
are already fixed by ADR-0005 and ADR-0006; adding a repository and two file
resources to that boundary is an application, not a new boundary. It fails
`lower_level_artifact` because chapter 7 owns the deployed object inventory
(the third repository, the two files, and the manual pin and private-reporting
steps); that chapter is the single canonical home, and the public-copy concept
and the design's implementation sections are realization rather than additional
homes. The
existing-ADR check found ADR-0005, ADR-0006 and ADR-0003 covering the
declaration, the state and the platform; no record covers the profile text or
the manual steps, and neither is an architecture decision. The `org/**` CC0-1.0
choice is not an ADR candidate: it is a content-licensing configuration choice,
and its canonical home is `REUSE.toml` (the design records the implementation).
No ADR is added.

The three contract facts that shaped the copy are current truth, not
preferences: the repository has no web UI or WASM and `apps/cli` is the only
application; the current release support is Windows x86_64 only; and the owner
excludes the third-party provider name from new public content. The profile
therefore describes the local converter, names Windows x86_64 only, omits the
provider name and states non-affiliation.

## Status

STATUS: DONE
