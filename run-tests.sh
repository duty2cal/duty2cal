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
#
# Runner result contract (givn's archive gate): when GIVN_RESULT_FILE is set,
# this script writes the scope's scenario counts there as
# {"scope": "...", "total": N, "passed": N, "failed": N, "skipped": N}, with the
# scope "regular" for the full run and "e2e" for the @e2e run. The file is
# written whenever the runner produced a summary, so the gate reads counts
# rather than trusting an exit code.
set -euo pipefail
cd "$(dirname "$0")"

# Both suite modes drive the rendered page, so the bundle is built before the
# runner. `DUTY2CAL_SKIP_WEB_BUILD=1` is for a run that only re-executes already
# built scenarios (design.md "Test runner, strict mode, and single-scenario
# command").
if [ "${DUTY2CAL_SKIP_WEB_BUILD:-0}" != "1" ]; then
  scripts/build-web.sh
fi

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
transcript="$(mktemp)"
trap 'rm -rf "$root" "$transcript"' EXIT

export GIVN_FEATURES="$root"

status=0
cargo test -p duty2cal --test features_runner -- \
  ${filter[@]+"${filter[@]}"} ${args[@]+"${args[@]}"} 2>&1 | tee "$transcript" || status=$?

if [ -n "${GIVN_RESULT_FILE:-}" ]; then
  # The scope the gate asked for: "e2e" when the run is the @e2e filter.
  scope="regular"
  [ "${#filter[@]}" -gt 0 ] && scope="e2e"

  summary="$(grep -E '^[0-9]+ scenarios?' "$transcript" | tail -1 || true)"
  total="${summary%% *}"
  counts="${summary#*\(}"
  counts="${counts%\)*}"
  passed=0
  failed=0
  skipped=0
  if [ -n "$summary" ]; then
    IFS=',' read -r -a parts <<< "$counts"
    for part in "${parts[@]}"; do
      value="${part%% *}"
      kind="${part##* }"
      case "$kind" in
        passed) passed="$value" ;;
        failed) failed="$value" ;;
        skipped) skipped="$value" ;;
      esac
    done
  fi
  mkdir -p "$(dirname "$GIVN_RESULT_FILE")"
  printf '{"scope":"%s","total":%s,"passed":%s,"failed":%s,"skipped":%s}\n' \
    "$scope" "${total:-0}" "$passed" "$failed" "$skipped" > "$GIVN_RESULT_FILE"
fi

exit "$status"
