# Review: state-organization-fields

A value change to already-declared objects: the organization's description, its
URL, its public mail address and its location. `specs`, `design`, `arc42-docs`
and `design-review` are skipped (recorded in `.givn-skip`): the apply mechanism
is unchanged, no capability is added, and no architecture chapter's decision
moves.

## Receipts

| Check | Command | Result |
|---|---|---|
| Declaration format | `tofu fmt -check` | clean |
| Declaration validity | `tofu validate` | "Success! The configuration is valid." (OpenTofu 1.13.1, integrations/github 6.13.0) |
| Fixtures | `python3 -c json.load …` | both parse, both carry the four values |
| Live read-back | `https://api.github.com/orgs/duty2cal` | description, `blog`, `email` and `location` are the declared values; `company` and `twitter_username` are null |
| Social accounts | `https://api.github.com/orgs/duty2cal/social_accounts` | no account |
| Applied state | `infra/terraform.tfstate`, `.backup` | committed in `1603a36`; no `.stale` file |

## Fabrication audit

| Check | Evidence | Result |
|---|---|---|
| Empty steps / stubs | no `.feature` delta and no step file: the change is a declaration value change | NOT APPLICABLE |
| `@e2e` integrity | no `@e2e` scenario in scope | NOT APPLICABLE |
| Task-to-commit | `6c7992e` (declaration, runbook, fixtures) and `1603a36` (applied state) | CLEAN |
| Promised components exist | `github_organization_settings.org` carries the four fields; `infra/README.md` documents them; no `github_organization_social_account` resource exists | CLEAN |
| Strict mode / runner | the runner is unchanged; the change adds no scenario | NOT APPLICABLE |

## Coverage

No production code and no test code is added. The declared values are their own
evidence; the live read-back is the acceptance. The archive gate's coverage step
cannot run in this environment (Nix rustc emits raw profile format 11 while the
Nix `llvm-profdata` accepts 10), which is a pre-existing environment limitation
recorded with the previous change and not a finding of this one.

## Visual review

Interface: none — no screen is added or changed. The organization page is
GitHub's rendering of the applied values, and the read-back above is its
evidence.

VISUAL-REVIEW: NOT APPLICABLE

PERSONAS: project-maintainer--organization-owner
PERSONAS: visitor--project-reader

## README impact decision

README-IMPACT: none

The repository README is unchanged; the runbook under `infra/README.md` gained
the declared-fields table.

## Use-case and Persona conformance

`apply-configuration-change` owns the declaration and the preview/apply
procedure; this change changed four declared values and nothing about the
procedure. The organization owner's goal (the declaration states what the
organization publishes) and the visitor's goal (the organization page says what
the project does and where it lives) are both met by the live read-back.

REVIEW: PASS
