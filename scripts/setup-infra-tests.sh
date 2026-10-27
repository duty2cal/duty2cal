#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
#
# Prepare the infrastructure suite's local prerequisites (design.md "Local
# runnability & digital twins", "Provider installation for tests"):
#
#   * the pinned OpenTofu exactly as recorded in infra/.opentofu-version;
#   * python3, which runs the fake GitHub API;
#   * a filesystem mirror of the locked integrations/github provider outside
#     the repository;
#   * a terraformrc that makes `tofu init` resolve the provider from that
#     mirror, so the suite runs offline.
#
# Nothing is installed silently and nothing is written inside the repository.
# Run this once per workstation as the local planning proof; CI does not run it.
set -euo pipefail
cd "$(dirname "$0")/.."

fail() {
  printf 'setup-infra-tests: %s\n' "$1" >&2
  exit 1
}

required_version="$(tr -d '[:space:]' < infra/.opentofu-version)"
[ -n "$required_version" ] || fail "infra/.opentofu-version is empty"

command -v tofu >/dev/null 2>&1 || fail "tofu is not on PATH"
tofu_bin="$(command -v tofu)"
installed_version="$(tofu version | sed -n 's/^OpenTofu v\([0-9.]*\).*/\1/p' | head -1)"
[ "$installed_version" = "$required_version" ] ||
  fail "tofu $required_version is required, found ${installed_version:-unknown} ($tofu_bin)"

command -v python3 >/dev/null 2>&1 || fail "python3 is not on PATH"
python_version="$(python3 --version 2>&1)"

# The apply routine's preview guard and live-change preflight read the plan
# JSON with jq (design.md "The apply procedure"), so a missing jq would only
# surface inside a scenario.
command -v jq >/dev/null 2>&1 || fail "jq is not on PATH"
jq_version="$(jq --version 2>&1)"

os="$(uname -s | tr '[:upper:]' '[:lower:]')"
case "$os" in
  linux|darwin) ;;
  *) fail "unsupported operating system: $os" ;;
esac
case "$(uname -m)" in
  x86_64|amd64) arch="amd64" ;;
  aarch64|arm64) arch="arm64" ;;
  *) fail "unsupported machine architecture: $(uname -m)" ;;
esac
platform="${os}_${arch}"

cache="${XDG_CACHE_HOME:-$HOME/.cache}/duty2cal"
mirror="$cache/terraform-provider-mirror"
terraformrc="$cache/terraformrc"
mkdir -p "$cache"

github_version="$(
  awk '/provider "registry.opentofu.org\/integrations\/github"/ { found = 1; next }
       found && /version[[:space:]]*=/ { gsub(/[^0-9.]/, ""); print; exit }' \
    infra/.terraform.lock.hcl
)"
tls_version="$(
  awk '/provider "registry.opentofu.org\/hashicorp\/tls"/ { found = 1; next }
       found && /version[[:space:]]*=/ { gsub(/[^0-9.]/, ""); print; exit }' \
    infra/.terraform.lock.hcl
)"
[ -n "$github_version" ] || fail "infra/.terraform.lock.hcl names no integrations/github version"
[ -n "$tls_version" ] || fail "infra/.terraform.lock.hcl names no hashicorp/tls version"

github_marker="$mirror/registry.opentofu.org/integrations/github/${github_version}.json"
tls_marker="$mirror/registry.opentofu.org/hashicorp/tls/${tls_version}.json"
if [ ! -f "$github_marker" ] || [ ! -f "$tls_marker" ]; then
  (
    cd infra
    tofu providers mirror -platform="$platform" "$mirror"
  )
fi
[ -f "$github_marker" ] || fail "the provider mirror does not hold integrations/github $github_version"
[ -f "$tls_marker" ] || fail "the provider mirror does not hold hashicorp/tls $tls_version"

cat > "$terraformrc" <<EOF
# Written by scripts/setup-infra-tests.sh. The suite resolves the locked
# providers from the local mirror and never from the registry.
provider_installation {
  filesystem_mirror {
    path    = "$mirror"
    include = ["registry.opentofu.org/integrations/github", "registry.opentofu.org/hashicorp/tls"]
  }
  direct {
    exclude = ["registry.opentofu.org/integrations/github", "registry.opentofu.org/hashicorp/tls"]
  }
}
EOF

printf 'tofu %s (%s)\n' "$installed_version" "$tofu_bin"
printf '%s\n' "$python_version"
printf '%s\n' "$jq_version"
printf 'provider integrations/github %s for %s\n' "$github_version" "$platform"
printf 'provider hashicorp/tls %s for %s\n' "$tls_version" "$platform"
printf 'mirror %s\n' "$mirror"
printf 'terraformrc %s\n' "$terraformrc"
printf 'export TF_CLI_CONFIG_FILE=%s\n' "$terraformrc"

# ---------------------------------------------------------------------------
# Fixture proof (design.md "Test seam: the fake GitHub API", setup task).
#
# A fixture set is complete only when the real provider's reads succeed for
# every adopted object. One real `tofu import` runs per batch entry against the
# fake, then the same workspace proves that a `-refresh-only` plan on the
# unchanged fake is a no-op. The fake journals every request, so an unserved
# path shows up as a 599 and fails the proof instead of defaulting silently.
# ---------------------------------------------------------------------------

proof="$(mktemp -d)"
fake_pid=""
port=""
cleanup_proof() {
  [ -n "$fake_pid" ] && kill "$fake_pid" 2>/dev/null || true
  rm -rf "$proof"
}
trap cleanup_proof EXIT INT TERM

export GITHUB_TOKEN=test-token
export TF_VAR_state_passphrase=test-passphrase-0123456789
export TF_VAR_billing_email=billing@example.test
export TF_CLI_CONFIG_FILE="$terraformrc"

start_fake() { # <fixtures> <journal> [extra fake flags...]
  local fixtures="$1" journal="$2"
  shift 2
  rm -f "$proof/port" "$proof/fake.err"
  python3 scripts/fake-github/serve.py \
    --fixtures "$fixtures" \
    --port 0 --journal "$journal" "$@" >"$proof/port" 2>"$proof/fake.err" &
  fake_pid=$!
  for _ in $(seq 1 50); do
    [ -s "$proof/port" ] && break
    sleep 0.1
  done
  [ -s "$proof/port" ] || { cat "$proof/fake.err" >&2; fail "the fake GitHub API did not print a port"; }
  port="$(cat "$proof/port")"
  export GITHUB_BASE_URL="http://127.0.0.1:$port"
}

stop_fake() {
  [ -n "$fake_pid" ] && kill "$fake_pid" 2>/dev/null || true
  wait "$fake_pid" 2>/dev/null || true
  fake_pid=""
}

assert_no_unknown_paths() { # <journal>
  if grep -q '"unknown_path": true' "$1"; then
    cat "$1" >&2
    fail "the fixture proof requested a path the fake does not know"
  fi
}

# One real import per batch entry, each in a fresh workspace: every object's
# live read is proven independently.
prove_each_import() { # <fixtures> <batch> <label>
  local fixtures="$1" batch="$2" label="$3" address import_id entry=0
  while IFS=$'\t' read -r address import_id; do
    case "$address" in ''|'#'*) continue ;; esac
    entry=$((entry + 1))
    local workspace="$proof/entry-$label-$entry"
    mkdir -p "$workspace"
    cp infra/*.tf infra/.terraform.lock.hcl "$workspace/"
    # The profile repository is never adopted; this proof covers only the
    # existing objects (design.md "Workspace shapes").
    rm -f "$workspace/org_profile.tf"
    rm -f "$workspace/pages_publish.tf"
    (
      cd "$workspace"
      tofu init -backend=false -input=false -no-color >/dev/null 2>&1 ||
        fail "tofu init failed for $address"
      tofu import -input=false -no-color "$address" "$import_id" >"$proof/import.log" 2>&1 ||
        { cat "$proof/import.log" >&2; fail "tofu import failed for $address"; }
    )
    printf 'import ok %s %s\n' "$address" "$import_id"
  done < "$batch"
}

import_batch() { # <workspace> <batch>
  local workspace="$1" batch="$2" address import_id
  while IFS=$'\t' read -r address import_id; do
    case "$address" in ''|'#'*) continue ;; esac
    (
      cd "$workspace"
      tofu import -input=false -no-color "$address" "$import_id" >"$proof/import.log" 2>&1 ||
        { cat "$proof/import.log" >&2; fail "tofu import failed for $address"; }
    )
    printf 'import ok %s %s\n' "$address" "$import_id"
  done < "$batch"
}

assert_refresh_noop() { # <workspace> <label>
  (
    cd "$1"
    tofu plan -refresh-only -input=false -no-color >"$proof/refresh.log" 2>&1 ||
      { cat "$proof/refresh.log" >&2; fail "the refresh-only plan failed for $2"; }
    grep -q 'No changes\.' "$proof/refresh.log" ||
      { cat "$proof/refresh.log" >&2; fail "the refresh-only plan for $2 is not a no-op"; }
  )
  printf 'refresh-only plan (%s) on the unchanged fake: No changes.\n' "$2"
}

# ---------------------------------------------------------------------------
# Tree 1: adopt-all, the predecessor's adoption scenarios.
# ---------------------------------------------------------------------------

journal="$proof/adopt-all.jsonl"
start_fake scripts/fake-github/fixtures/adopt-all "$journal"
prove_each_import scripts/fake-github/fixtures/adopt-all infra/adoption-batch.txt adopt-all

workspace="$proof/adopt-all-workspace"
mkdir -p "$workspace"
cp infra/*.tf infra/.terraform.lock.hcl "$workspace/"
rm -f "$workspace/org_profile.tf"
rm -f "$workspace/pages_publish.tf"
(
  cd "$workspace"
  tofu init -backend=false -input=false -no-color >/dev/null 2>&1 ||
    fail "tofu init failed in the adopt-all workspace"
)
import_batch "$workspace" infra/adoption-batch.txt
assert_refresh_noop "$workspace" adopt-all
assert_no_unknown_paths "$journal"
printf 'unknown paths served: 0\n'
printf 'fixtures %s proven for %s objects\n' \
  scripts/fake-github/fixtures/adopt-all "$(grep -vc '^#' infra/adoption-batch.txt)"
stop_fake

# ---------------------------------------------------------------------------
# Tree 2: security-applied, the security scenarios' base tree. The base batch
# is bound and the base declaration (this change's declaration minus
# security.tf and the scanning block) is applied for real, so the inherited
# applied state exists; then the second batch binds the two Dependabot
# settings.
# ---------------------------------------------------------------------------

journal="$proof/security-applied.jsonl"
# The base declaration's real apply records the state-only
# `ignore_vulnerability_alerts_during_read` field with an update, so the fake
# accepts the write sandbox for this tree; the plan check below proves the
# update carries no live attribute change.
start_fake scripts/fake-github/fixtures/security-applied "$journal" --writes
prove_each_import scripts/fake-github/fixtures/security-applied infra/adoption-batch.txt security-applied

workspace="$proof/security-applied-workspace"
mkdir -p "$workspace"
cp infra/*.tf infra/.terraform.lock.hcl "$workspace/"
rm -f "$workspace/org_profile.tf"
rm -f "$workspace/pages_publish.tf"
rm "$workspace/security.tf"
# Strip the scanning block, leaving the base declaration this change inherits.
awk '
  /^[[:space:]]*security_and_analysis \{/ { depth = 1; next }
  depth > 0 {
    if ($0 ~ /\{/) depth++
    if ($0 ~ /\}/) depth--
    next
  }
  { print }
' "$workspace/repos.tf" >"$workspace/repos.tf.base"
mv "$workspace/repos.tf.base" "$workspace/repos.tf"
(
  cd "$workspace"
  tofu init -backend=false -input=false -no-color >/dev/null 2>&1 ||
    fail "tofu init failed in the security-applied workspace"
)
import_batch "$workspace" infra/adoption-batch.txt
assert_refresh_noop "$workspace" security-applied-adopted

# The base declaration's own apply records the inherited live values and the
# state-only fields; the declaration then matches live.
(
  cd "$workspace"
  tofu init -input=false -no-color >/dev/null 2>&1 ||
    fail "the base declaration's tofu init failed"
  tofu plan -input=false -no-color -out=base-plan.bin >"$proof/base-plan.log" 2>&1 ||
    { cat "$proof/base-plan.log" >&2; fail "the base declaration's plan failed"; }
  # The inherited applied state differs from the declaration only in the
  # provider's state-only read flag; any other changed line is a live change
  # and fails the proof before the security plan is made.
  if grep -E '^ +[~+-] [a-z_]+ +=' "$proof/base-plan.log" |
      grep -v 'ignore_vulnerability_alerts_during_read'; then
    fail "the base declaration's plan changes a live attribute"
  fi
  tofu apply -input=false -no-color base-plan.bin >"$proof/base-apply.log" 2>&1 ||
    { cat "$proof/base-apply.log" >&2; fail "the base declaration's apply failed"; }
  tofu plan -input=false -no-color >"$proof/base-after.log" 2>&1 ||
    { cat "$proof/base-after.log" >&2; fail "the base declaration's follow-up plan failed"; }
  grep -q 'No changes\.' "$proof/base-after.log" ||
    { cat "$proof/base-after.log" >&2; fail "the base apply left a live change"; }
)
printf 'base declaration apply: 0 live changes (follow-up plan: No changes.)\n'

# Restore this change's declaration and bind the two Dependabot settings.
cp infra/security.tf infra/repos.tf "$workspace/"
import_batch "$workspace" infra/adoption-batch-security.txt
assert_refresh_noop "$workspace" security-applied-security-adopted
assert_no_unknown_paths "$journal"
printf 'unknown paths served: 0\n'
printf 'fixtures %s proven for %s base + %s security objects\n' \
  scripts/fake-github/fixtures/security-applied \
  "$(grep -vc '^#' infra/adoption-batch.txt)" \
  "$(grep -vc '^#' infra/adoption-batch-security.txt)"
stop_fake
