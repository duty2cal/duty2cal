#!/usr/bin/env bash
# Test runner for duty2cal (see design.md "Test runner, strict mode, and
# single-scenario command").
#
# cucumber-rs takes one feature root, so this script assembles every .feature
# from the permanent corpus (givn/specs/**) and the active change's specs
# (givn/changes/*/specs/**) into a temporary root and points the runner at it.
#
# Usage:
#   ./run-tests.sh                       # full suite (non-@e2e and @e2e)
#   ./run-tests.sh --e2e                 # only @e2e scenarios (may appear anywhere)
#   ./run-tests.sh --name "<title>"      # one scenario
#   ./run-tests.sh --tags "@wip"         # tag filter
set -euo pipefail
cd "$(dirname "$0")"

filter=()
args=()
for a in "$@"; do
  if [ "$a" = "--e2e" ]; then
    filter+=(--tags "@e2e")
  else
    args+=("$a")
  fi
done

root="$(mktemp -d)"
trap 'rm -rf "$root"' EXIT
mkdir -p "$root"

i=0
while IFS= read -r -d '' f; do
  i=$((i + 1))
  cp "$f" "$root/$(printf 'f%04d' "$i").feature"
done < <(find givn/specs givn/changes -name '*.feature' -print0 2>/dev/null)

export GIVN_FEATURES="$root"
status=0
cargo test -p duty2cal --test features_runner -- ${filter[@]+"${filter[@]}"} ${args[@]+"${args[@]}"} || status=$?
exit "$status"
