# Tasks: state-organization-fields

A value change to already-declared objects. `specs`, `design`, `arc42-docs` and
`design-review` are skipped: no observable behaviour is added, the apply
mechanism is unchanged, and no architecture chapter's decision changes. The
declaration is the whole product here.

Use-case context: `apply-configuration-change` owns the declared settings and
the preview/apply procedure; the organization owner runs it. Personas:
`project-maintainer--organization-owner` (operates the declaration) and
`visitor--project-reader` (reads the organization page).

## Setup task — values and their source

- [x] The four values are the ones the notices and the support statement already
      publish: the description is the support sentence without the project's
      name, the URL is the Public website's canonical address, the mail address
      is the Impressum's alias, and the location is the operator's city from the
      Impressum.

## Implementation

- [x] `infra/main.tf` declares `description`, `blog`, `email` and `location` on
      `github_organization_settings.org`; `company` and `twitter_username` stay
      empty, and the comment states why no social account is declared.
- [x] `infra/README.md` gains "Declared organization profile fields" with the
      values and the rule that a social account needs its own notice decision.
- [x] The two fake-GitHub fixtures under `scripts/fake-github/fixtures/*/orgs/`
      carry the same profile values, so the test twin describes the live
      organization after the apply.
- [x] `tofu fmt -check` and `tofu validate` pass in `infra/` (OpenTofu 1.13.1,
      provider integrations/github 6.13.0).

## Operator step — preview and apply

- [ ] Preview with the maintainer's token and passphrase; the plan names exactly
      the four profile fields of `github_organization_settings.org` and no other
      object.
- [ ] Apply, then read the live organization back: the description, the URL, the
      mail address and the location are those declared here, `company` and the
      Twitter/X field are empty, and no social account exists.
- [ ] Commit `infra/terraform.tfstate` and `infra/terraform.tfstate.backup` after
      a green apply; a partial apply leaves `terraform.tfstate.stale`, which is
      read and completed, never committed.
- [ ] `scripts/check-drift.sh` reports no difference after the apply.

## Verification

- [x] `tofu fmt -check` clean; `tofu validate` reports "Success! The
      configuration is valid."
- [x] Both fixtures parse as JSON and carry the four values.
- [x] No `github_organization_social_account` resource exists in `infra/`, and
      `twitter_username` is empty.
- [ ] The live apply and the read-back are the operator's evidence for the
      published values; they cannot run in this environment because the token,
      the passphrase and the billing address exist only there.
