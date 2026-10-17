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
# Run this once per workstation; CI runs it before ./run-tests.sh.
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

provider_version="$(
  sed -n 's/^[[:space:]]*version[[:space:]]*=[[:space:]]*"\([0-9][0-9.]*\)"/\1/p' \
    infra/.terraform.lock.hcl | head -1
)"
[ -n "$provider_version" ] || fail "infra/.terraform.lock.hcl names no provider version"

marker="$mirror/registry.opentofu.org/integrations/github/${provider_version}.json"
if [ ! -f "$marker" ]; then
  (
    cd infra
    tofu providers mirror -platform="$platform" "$mirror"
  )
fi
[ -f "$marker" ] || fail "the provider mirror does not hold integrations/github $provider_version"

cat > "$terraformrc" <<EOF
# Written by scripts/setup-infra-tests.sh. The suite resolves the locked
# GitHub provider from the local mirror and never from the registry.
provider_installation {
  filesystem_mirror {
    path    = "$mirror"
    include = ["registry.opentofu.org/integrations/github"]
  }
  direct {
    exclude = ["registry.opentofu.org/integrations/github"]
  }
}
EOF

printf 'tofu %s (%s)\n' "$installed_version" "$tofu_bin"
printf '%s\n' "$python_version"
printf 'provider integrations/github %s for %s\n' "$provider_version" "$platform"
printf 'mirror %s\n' "$mirror"
printf 'terraformrc %s\n' "$terraformrc"
printf 'export TF_CLI_CONFIG_FILE=%s\n' "$terraformrc"
