#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Gate for the public text a reader lands on (README.md): the promises the
# publish-public-release use case makes about it must all be present before it
# is published. Each missing promise is named separately, so a failure says
# which promise was lost rather than merely that the file changed.
#
# The promises checked:
#   - the repository's own clone address
#   - the archive name for each of the five supported platforms
#   - the command that verifies an archive's checksum
#   - the command that verifies an archive's build provenance
#   - the licence statement with a link to its text
#   - the third-party Unicode data licence, kept attributed
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

require 'clone-address' 'git clone https://github\.com/kressnerd/duty2cal\.git'
for platform in windows-x86_64 linux-x86_64 linux-arm64 macos-arm64 macos-x86_64; do
  require "archive-${platform}" "duty2cal-[^[:space:]]*-${platform}\.tar\.gz"
done
require 'checksum-command' 'sha256sum --check'
require 'provenance-command' 'gh attestation verify'
require 'licence-statement' 'MPL-2\.0'
require 'licence-text-link' 'LICENSES/MPL-2\.0\.txt'
require 'unicode-data-licence' 'Unicode-3\.0'
require 'unicode-data-licence-link' 'LICENSES/Unicode-3\.0\.txt'

if [ "$missing" -ne 0 ]; then
  printf 'public-text-check: FAILED - %d of %d promise(s) missing from %s\n' \
    "$missing" "$promises" "$readme"
  exit 1
fi

printf 'public-text-check: clean - all %d public-text promises present in %s\n' \
  "$promises" "$readme"
