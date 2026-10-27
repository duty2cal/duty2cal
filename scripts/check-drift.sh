#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# The local configuration drift check (ADR-0006).
#
# The applied state is passphrase-encrypted and committed, and no workflow may
# hold the passphrase. Run this on the maintainer's workstation with
# TF_VAR_state_passphrase, TF_VAR_billing_email and GITHUB_TOKEN (or GH_TOKEN)
# in the environment. It reads the live configuration and the applied state and
# refuses when they differ; it never writes GitHub and never writes a plan.
# The prompt/export sequence for bash and fish is in infra/README.md.
#
# Usage: scripts/check-drift.sh [--repo DIR]
set -euo pipefail

repo="$(cd "$(dirname "$0")/.." && pwd)"
while [ "$#" -gt 0 ]; do
  case "$1" in
    --repo) [ "$#" -ge 2 ] || { echo "check-drift: --repo needs a value" >&2; exit 2; }; repo="$2"; shift 2 ;;
    -h|--help) sed -n '2,12p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
    *) echo "check-drift: unknown argument: $1" >&2; exit 2 ;;
  esac
done

refuse() {
  echo "check-drift: $*" >&2
  exit 1
}

[ -d "$repo/infra" ] || refuse "the declaration directory is missing: $repo/infra"
command -v tofu >/dev/null 2>&1 || refuse "'tofu' is not installed"
[ -n "${TF_VAR_state_passphrase:-}" ] || refuse "TF_VAR_state_passphrase is not set"
[ "${#TF_VAR_state_passphrase}" -ge 16 ] || refuse "TF_VAR_state_passphrase must be at least 16 characters"
[ -n "${TF_VAR_billing_email:-}" ] || refuse "TF_VAR_billing_email is not set"
[ -n "${GITHUB_TOKEN:-${GH_TOKEN:-}}" ] || refuse "GITHUB_TOKEN or GH_TOKEN is not set"

state="$repo/infra/terraform.tfstate"
[ -f "$state" ] || refuse "the applied state is missing: $state"
if [ -f "$state.stale" ]; then
  refuse "the applied state is marked stale; read $state.stale and complete the revision before checking drift"
fi

# Read-only: the state is opened with the passphrase, the live configuration is
# read, and the plan is compared. No -out, so no plan file is written; no
# apply.
set +e
(
  cd "$repo/infra"
  tofu init -input=false -no-color >/dev/null
  tofu plan -refresh-only -detailed-exitcode -input=false -no-color
)
status=$?
set -e

case "$status" in
  0)
    echo "check-drift: no configuration drift"
    ;;
  2)
    refuse "the live configuration differs from the applied state; the plan above names every changed object"
    ;;
  *)
    refuse "the drift plan could not be read (exit status $status); the declaration or the credential is unreadable"
    ;;
esac
