#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# CI gate: every source file carries exactly one SPDX licence header.
# The header is on line 1, or on line 2 when line 1 is a shebang (see ADR-0001).
set -euo pipefail
cd "$(dirname "$0")/.."

extensions='\.(rs|toml|yml|yaml|sh|kt|swift|cs|dart)$'
missing=0

while IFS= read -r f; do
  [ -f "$f" ] || continue
  case "$f" in
    # givn-managed scaffold and generated agent rules are not project source.
    givn/*|.omp/*|.claude/*|.agents/*|.opencode/*) continue ;;
    LICENSES/*) continue ;;
  esac
  # REUSE-IgnoreStart
  tag='SPDX-License-Identifier:'
  # REUSE-IgnoreEnd
  if ! head -n 2 "$f" | grep -q "$tag"; then
    echo "missing SPDX header on line 1 (or 2 after a shebang): $f"
    missing=1
  fi
done < <( { git ls-files; git ls-files --others --exclude-standard; } | sort -u | grep -E "$extensions" || true )

if [ "$missing" -ne 0 ]; then
  echo "license-headers: FAILED"
  exit 1
fi
echo "license-headers: clean"
