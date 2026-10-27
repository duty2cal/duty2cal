# Infrastructure operator runbook

The organization, its repositories, the Pages target, the rule sets and the
public profile are declared in this directory and applied with OpenTofu from the
maintainer's workstation. The credential constraint and the guarantees are in
`docs/arc42/02`; the apply mechanism and the manual prerequisites are in
`docs/arc42/05`, `06` and `07`.

No apply runs in GitHub Actions and no credential is a repository secret (ADR-0006).
The passphrase, the billing address and the administer token exist only in the
maintainer's environment.

## Prerequisites

- `tofu` (the version in `.opentofu-version`), with the provider already
  initialized or reachable through the local mirror.
- The state passphrase, at least 16 characters, from the password manager. It
  decrypts `terraform.tfstate` and the plan file; losing it means re-adopting.
- The organization's billing address, supplied through `TF_VAR_billing_email`.
- A GitHub token with owner/admin rights. A full apply needs `repo` and
  `admin:org`; a targeted profile-file apply needs write access to
  `duty2cal/.github`. `gh auth token` works after `gh auth login`, but that
  OAuth token may not carry `admin:org`, so the owner PAT remains the safe
  choice for a full apply.

## Prompt for the sensitive values

The prompts keep the passphrase and the token out of the shell history. They
are *not* command-line arguments: the provider reads `GITHUB_TOKEN`, and the
variables are `TF_VAR_*`.

### fish

`-P` is fish's literal prompt; `-s` masks the input.

```fish
cd ~/dev/PRIVATE/duty2cal/infra

read -s -P 'State passphrase: ' TF_VAR_state_passphrase
set -gx TF_VAR_state_passphrase $TF_VAR_state_passphrase

read -P 'Billing email: ' TF_VAR_billing_email
set -gx TF_VAR_billing_email $TF_VAR_billing_email

read -s -P 'GitHub token: ' GITHUB_TOKEN
set -gx GITHUB_TOKEN $GITHUB_TOKEN
```

If `gh` is authenticated and its scopes are sufficient:

```fish
set -gx GITHUB_TOKEN (gh auth token)
```

### bash

Bash's prompt flag is lowercase `-p`; `-r` keeps backslashes literal and `-s`
masks the input. `IFS=` preserves leading and trailing spaces in the value.

```bash
cd ~/dev/PRIVATE/duty2cal/infra

IFS= read -rsp 'State passphrase: ' TF_VAR_state_passphrase
export TF_VAR_state_passphrase

IFS= read -rp 'Billing email: ' TF_VAR_billing_email
export TF_VAR_billing_email

IFS= read -rsp 'GitHub token: ' GITHUB_TOKEN
export GITHUB_TOKEN
```

If `gh` is authenticated and its scopes are sufficient:

```bash
export GITHUB_TOKEN="$(gh auth token)"
```

## Apply

The declaration resolves `org/` beside `infra/` by default; set
`TF_VAR_org_source_dir` only when applying a staged copy.

### Targeted file change

The profile README is one declared file; target it so the plan names no
unrelated object:

```sh
tofu init -input=false
tofu plan -input=false -no-color \
  -target=github_repository_file.org_profile_readme \
  -out=plan.bin
tofu apply -input=false plan.bin
```

The plan file is encrypted (`enforced = true` for both state and plan), is
git-ignored, and is deleted after the apply. Do not paste a plan or
`tofu show -json` output: it carries the sensitive variables.

### The full declaration

```sh
tofu init -input=false
tofu plan -input=false -no-color -out=plan.bin
tofu apply -input=false plan.bin
```

A full apply can touch the organization settings, repositories, rule sets,
Pages and the profile. The creation guard and the refresh-only preflight
described in chapter 6 are the full procedure; run them before a broad apply.

### After a green apply

- The state is committed only after the apply is green:
  `infra/terraform.tfstate` and `infra/terraform.tfstate.backup`.
- A partial apply leaves `infra/terraform.tfstate.stale`; read it, complete the
  revision, and do not commit the stale state.
- Read back the published object. For the profile:

```sh
curl -sS https://raw.githubusercontent.com/duty2cal/.github/main/profile/README.md \
  | grep -F 'Lufthansa cabin-crew duty plan'
```

Commit the state:

```sh
cd ~/dev/PRIVATE/duty2cal
givn commit -m "chore(apply-configuration-change): record the applied revision" \
  infra/terraform.tfstate infra/terraform.tfstate.backup
```

### Drift

The local, read-only drift check needs the same environment:

```sh
scripts/check-drift.sh
```

It reads the live configuration and the applied state and refuses when they
differ; it never writes GitHub and never writes a plan.

## Clear the environment

```fish
set -e TF_VAR_state_passphrase TF_VAR_billing_email GITHUB_TOKEN
```

```bash
unset TF_VAR_state_passphrase TF_VAR_billing_email GITHUB_TOKEN
```

The read prompts keep the values out of history; they do not hide them from the
same user's process environment, so clear them when the apply is done.
