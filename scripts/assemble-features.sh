#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Assemble every `.feature` from the permanent corpus (givn/specs/**) and the
# active change's specs (givn/changes/*/specs/**) into a temporary root and
# print that root on stdout. cucumber-rs takes a single feature root, so
# ./run-tests.sh and ./measure-coverage.sh both use this assembly.
#
# The caller owns cleanup of the printed directory.
set -euo pipefail
cd "$(dirname "$0")/.."

root="$(mktemp -d)"
i=0
while IFS= read -r -d '' f; do
  i=$((i + 1))
  cp "$f" "$root/$(printf 'f%04d' "$i").feature"
done < <(find givn/specs givn/changes -name '*.feature' -print0 2>/dev/null)

printf '%s\n' "$root"
