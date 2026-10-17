#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Gate for the public text a reader lands on (README.md): the promises the
# publish-public-release use case makes about it must all be present before it
# is published, and no withdrawn download may reappear. Each failure is named
# separately, so it says which promise was lost rather than merely that the
# file changed.
#
# The promises checked:
#   - the repository's own clone address
#   - the one supported platform's archive name
#   - the command that verifies an archive's checksum, on Windows and in a
#     POSIX shell
#   - the command that verifies an archive's build provenance
#   - the licence statement with a link to its text
#   - the third-party Unicode data licence, kept attributed
#   - the unsupported-source-build and platform-request statements
#   - the immutability of already-published releases
# Forbidden:
#   - a Linux or macOS archive name
#   - a Linux or macOS download row
#
# Usage: scripts/public-text-check.sh [readme]
#   readme  the file to check (default: the repository's README.md)
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
readme="${1:-$(cd "$here/.." && pwd)/README.md}"

if [ ! -f "$readme" ]; then
  echo "public-text-check: no such file: $readme" >&2
  exit 2
fi

promises=0
missing=0

require() { # $1 = the named promise, $2 = extended regular expression
  promises=$((promises + 1))
  if ! grep -Eq -- "$2" "$readme"; then
    printf 'missing promise: %s\n' "$1"
    missing=$((missing + 1))
  fi
}

require_fixed() { # $1 = the named promise, $2 = the exact line fragment
  promises=$((promises + 1))
  if ! grep -Fq -- "$2" "$readme"; then
    printf 'missing promise: %s\n' "$1"
    missing=$((missing + 1))
  fi
}

forbid() { # $1 = the forbidden text's name, $2 = extended regular expression
  promises=$((promises + 1))
  if grep -Eq -- "$2" "$readme"; then
    printf 'forbidden text: %s\n' "$1"
    missing=$((missing + 1))
  fi
}

require 'clone-address' 'git clone https://github\.com/duty2cal/duty2cal\.git'
require 'windows-archive' 'duty2cal-[^[:space:]]*-windows-x86_64\.tar\.gz'
require 'checksum-command' 'sha256sum --check'
require 'checksum-command-powershell' 'Get-FileHash'
require 'provenance-command' 'gh attestation verify'
require 'licence-statement' 'MPL-2\.0'
require 'licence-text-link' 'LICENSES/MPL-2\.0\.txt'
require 'unicode-data-licence' 'Unicode-3\.0'
require 'unicode-data-licence-link' 'LICENSES/Unicode-3\.0\.txt'
require_fixed 'source-route-statement' 'Other platforms are not supported or distributed; the source can be built with a Rust toolchain.'
require_fixed 'platform-request-statement' 'A platform may be added back when users request it.'
require_fixed 'published-history-statement' 'Already-published releases keep their original assets and links; only future releases are Windows-only.'
forbid 'non-windows-archive' 'duty2cal-[^[:space:]]*-(linux-x86_64|linux-arm64|macos-arm64|macos-x86_64)\.tar\.gz'
forbid 'non-windows-download-row' '^\|[[:space:]]*(Linux x86_64|Linux arm64|macOS arm64( \([^)]*\))?|macOS x86_64( \([^)]*\))?)[[:space:]]*\|'

if [ "$missing" -ne 0 ]; then
  printf 'public-text-check: FAILED - %d of %d promise(s) violated in %s\n' \
    "$missing" "$promises" "$readme"
  exit 1
fi

printf 'public-text-check: clean - all %d public-text promises present in %s\n' \
  "$promises" "$readme"
