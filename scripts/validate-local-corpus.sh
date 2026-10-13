#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Validate the converter against the real duty plans the operator keeps locally.
#
# The plans are read from `private/` (never committed, never copied into the
# repository, never sent anywhere). Outputs and diagnostics are written into a
# temporary directory, so nothing is created inside `private/`.
#
# Usage: ./scripts/validate-local-corpus.sh [--keep]
set -euo pipefail
cd "$(dirname "$0")/.."

corpus="${DUTY2CAL_LOCAL_CORPUS:-private}"
if ! compgen -G "$corpus/*.pdf" >/dev/null; then
  echo "no local duty plans found in $corpus/ — nothing to validate"
  exit 0
fi

binary="${DUTY2CAL_BINARY:-target/release/duty2cal}"
if [ ! -x "$binary" ]; then
  echo "no converter at $binary — run: cargo build --release -p duty2cal" >&2
  exit 1
fi

work="$(mktemp -d)"
trap 'if [ "${KEEP:-0}" = "1" ]; then echo "kept: $work"; else rm -rf "$work"; fi' EXIT

passed=0
failed=0
for plan in "$corpus"/*.pdf; do
  cp "$plan" "$work/"
  name="$(basename "$plan")"
  if output="$("$binary" "$work/$name" 2>&1)"; then
    if compgen -G "$work/${name%.pdf}.ics" >/dev/null; then
      printf 'PASS  %s\n' "$name"
      passed=$((passed + 1))
      continue
    fi
  fi
  printf 'FAIL  %s — %s\n' "$name" "$(printf '%s' "$output" | head -1)"
  failed=$((failed + 1))
done

printf 'local corpus: %d passed, %d failed\n' "$passed" "$failed"
[ "$failed" -eq 0 ]
