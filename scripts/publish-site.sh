#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Publish the Site output built from one Built revision into a git hosting
# target (design.md "scripts/publish-site.sh").
#
# Usage:
#   scripts/publish-site.sh --repo DIR [--revision REV] [--since COMMIT]
#                           --target URL --out DIR --verify-url URL
#                           [--key FILE] [--branch main] [--lock PATH]
#
# A default-branch publication names the revision it compares against; when
# the diff since that base touches no site input, the command reports
# `no site input changed` and exits 0 without building, gating or writing. A
# tag publication omits `--since` and therefore always builds.
set -euo pipefail

repo=""
revision="HEAD"
since=""
target=""
out=""
verify_url=""
key=""
branch="main"
lock=""

# The site-input paths this command owns. It is deliberately not the change
# log's user-receiving list: that answers what a released archive contains.
site_inputs="crates/* ffi/wasm/* web/* docs/assets/*"
site_inputs="$site_inputs Cargo.toml Cargo.lock rust-toolchain.toml LICENSES/MPL-2.0.txt"
site_inputs="$site_inputs .github/workflows/web.yml .github/workflows/site.yml"

site_input() {
  local path="$1" pattern
  for pattern in $site_inputs; do
    # shellcheck disable=SC2254 # the pattern is the glob being matched
    case "$path" in $pattern) return 0 ;; esac
  done
  return 1
}

usage() {
  cat >&2 <<'EOF'
usage: scripts/publish-site.sh --repo DIR [--revision REV] [--since COMMIT]
                               --target URL --out DIR --verify-url URL
                               [--key FILE] [--branch main] [--lock PATH]
EOF
}

refuse() {
  echo "publish-site: $*" >&2
  exit 1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --repo) [ "$#" -ge 2 ] || { usage; exit 2; }; repo="$2"; shift 2 ;;
    --revision) [ "$#" -ge 2 ] || { usage; exit 2; }; revision="$2"; shift 2 ;;
    --since) [ "$#" -ge 2 ] || { usage; exit 2; }; since="$2"; shift 2 ;;
    --target) [ "$#" -ge 2 ] || { usage; exit 2; }; target="$2"; shift 2 ;;
    --out) [ "$#" -ge 2 ] || { usage; exit 2; }; out="$2"; shift 2 ;;
    --verify-url) [ "$#" -ge 2 ] || { usage; exit 2; }; verify_url="$2"; shift 2 ;;
    --key) [ "$#" -ge 2 ] || { usage; exit 2; }; key="$2"; shift 2 ;;
    --branch) [ "$#" -ge 2 ] || { usage; exit 2; }; branch="$2"; shift 2 ;;
    --lock) [ "$#" -ge 2 ] || { usage; exit 2; }; lock="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) usage; refuse "unknown argument: $1" ;;
  esac
done

[ -n "$repo" ] || { usage; refuse "--repo is required"; }
[ -n "$target" ] || { usage; refuse "--target is required"; }
[ -n "$out" ] || { usage; refuse "--out is required"; }
[ -n "$verify_url" ] || { usage; refuse "--verify-url is required"; }
[ -d "$repo" ] || refuse "the repository does not exist: $repo"

# Decide whether the revision changes a site input. Only --since asks the
# question; a tag publication omits it and always builds.
if [ -n "$since" ]; then
  base="$(git -C "$repo" rev-parse --verify "$since^{commit}" 2>/dev/null)"
  changed="$(git -C "$repo" diff --name-only "$base..$revision" 2>/dev/null)"
  changed_site=0
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    if site_input "$path"; then
      changed_site=1
      break
    fi
  done <<< "$changed"
  if [ "$changed_site" -eq 0 ]; then
    echo "no site input changed"
    exit 0
  fi
fi

refuse "the publication is not implemented for revision '$revision'"
