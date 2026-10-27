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
#   - the one published archive platform's archive name
#   - the statements that Windows x86_64 is the only published archive
#     platform, that no archive is published for another platform, and that
#     builds for other operating systems are available on request from a Rust
#     toolchain
#   - the browser conversion route on the Public website at its canonical
#     address, so a reader does not read "Windows x86_64 only" as "no route
#     without Windows"
#   - the command that verifies an archive's checksum, on Windows and in a
#     POSIX shell
#   - the command that verifies an archive's build provenance
#   - the licence statement with a link to its text
#   - the third-party Unicode data licence, kept attributed
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
require_fixed 'windows-only-archive-statement' 'Windows x86_64 is the only published archive platform.'
require_fixed 'no-other-archive-statement' 'No archive is published for any other platform.'
require_fixed 'platform-request-statement' 'Builds for other operating systems are available on request; their source can be built with a Rust toolchain.'
require_fixed 'platform-readdition-statement' 'A platform may be added back when users request it.'
require_fixed 'browser-route-statement' 'A current browser converts a plan on the [Public website](https://duty2cal.github.io/)'
require_fixed 'public-website-address' 'https://duty2cal.github.io/'
require_fixed 'published-history-statement' 'Already-published releases keep their original assets and links; only future releases are Windows-only.'
forbid 'non-windows-archive' 'duty2cal-[^[:space:]]*-(linux-x86_64|linux-arm64|macos-arm64|macos-x86_64)\.tar\.gz'
forbid 'non-windows-download-row' '^\|[[:space:]]*(Linux x86_64|Linux arm64|macOS arm64( \([^)]*\))?|macOS x86_64( \([^)]*\))?)[[:space:]]*\|'

# The support statement is one claim across five carriers. The composed check
# reads the repository tree, so a caller-provided readme is refused rather than
# checking a different README against the repository's carriers.
if [ "$(cd "$(dirname "$readme")" && pwd)/$(basename "$readme")" != "$(cd "$here/.." && pwd)/README.md" ]; then
  echo "public-text-check: the readme argument is not the repository README; the support statement check reads the repository carriers" >&2
  exit 2
fi
"$here/support-statement-check.sh"
# The legal notices are public text too: the site's two notice pages and the
# links the profile and this README carry are checked against the same tree.
"$here/legal-notices-check.sh" --root "$(cd "$here/.." && pwd)"

if [ "$missing" -ne 0 ]; then
  printf 'public-text-check: FAILED - %d of %d promise(s) violated in %s\n' \
    "$missing" "$promises" "$readme"
  exit 1
fi

printf 'public-text-check: clean - all %d public-text promises present in %s\n' \
  "$promises" "$readme"
