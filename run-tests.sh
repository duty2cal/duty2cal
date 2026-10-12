#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Test runner for duty2cal (see design.md "Test runner, strict mode, and
# single-scenario command").
#
# cucumber-rs takes one feature root, so scripts/assemble-features.sh builds a
# temporary root from the permanent corpus and the active change's specs and
# this script points the runner at it.
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

root="$(scripts/assemble-features.sh)"
trap 'rm -rf "$root"' EXIT

export GIVN_FEATURES="$root"
status=0
cargo test -p duty2cal --test features_runner -- ${filter[@]+"${filter[@]}"} ${args[@]+"${args[@]}"} || status=$?
exit "$status"
