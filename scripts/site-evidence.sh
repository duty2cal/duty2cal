#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# The publication's evidence gate (design.md "scripts/site-evidence.sh").
#
# Run inside the revision's detached worktree so every check is the revision's
# own: the page content, the licensing, the browser scenario and the conversion
# suite. Fail fast, in that order, and name the check that refused. Checks 3
# and 4 run under GIVN_RESULT_FILE, so a filter that selected nothing is a
# refusal instead of an empty green; the browser check additionally requires
# exactly one selected scenario.
#
# Usage:
#   scripts/site-evidence.sh --output DIR --repo DIR
set -euo pipefail

repo="$PWD"
out=""

usage() {
  echo "usage: scripts/site-evidence.sh --output DIR --repo DIR" >&2
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --output) [ "$#" -ge 2 ] || { usage; exit 2; }; out="$2"; shift 2 ;;
    --repo) [ "$#" -ge 2 ] || { usage; exit 2; }; repo="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) usage; echo "site-evidence: unknown argument: $1" >&2; exit 2 ;;
  esac
done

[ -n "$out" ] || { usage; exit 2; }
[ -d "$repo" ] || { echo "site-evidence: the worktree does not exist: $repo" >&2; exit 2; }

results="$(mktemp -d)"
trap 'rm -rf "$results"' EXIT

# 1. Page content: cheap and purely local.
if ! ( cd "$repo" && scripts/check-site-content.sh "$out" ); then
  echo "site-evidence: the page content check failed" >&2
  exit 1
fi

# 2. Licensing: MPL-2.0 for code, CC0-1.0 for non-code assets.
if ! lint="$( cd "$repo" && reuse lint 2>&1 )"; then
  printf '%s\n' "$lint" >&2
  echo "site-evidence: the licensing check failed" >&2
  exit 1
fi

# The result file names what a filtered run selected.
selected_total() {
  python3 - "$1" <<'PY'
import json
import sys
with open(sys.argv[1], encoding="utf-8") as handle:
    print(int(json.load(handle).get("total", 0)))
PY
}

# 3. The browser check: the project's existing browser scenario, driven
#    against the assembled output, and exactly one selected scenario.
browser_result="$results/browser.json"
if ! ( cd "$repo" && \
    GIVN_RESULT_FILE="$browser_result" \
    DUTY2CAL_SITE_ROOT="$out" \
    DUTY2CAL_SKIP_WEB_BUILD=1 \
    ./run-tests.sh --name "Crew member converts a supported duty plan in the browser and saves the offered calendar" ); then
  echo "site-evidence: the browser check failed" >&2
  exit 1
fi
browser_total="$(selected_total "$browser_result")"
if [ "$browser_total" -ne 1 ]; then
  echo "site-evidence: the browser check selected $browser_total scenarios; exactly one is required" >&2
  exit 1
fi

# 4. The conversion suite: every non-browser scenario the revision holds.
#    The @site-publication tag keeps a publication's own scenarios out of the
#    suite it runs, so the gate can never re-enter itself.
conversion_result="$results/conversion.json"
if ! ( cd "$repo" && \
    GIVN_RESULT_FILE="$conversion_result" \
    DUTY2CAL_SITE_ROOT="$out" \
    DUTY2CAL_SKIP_WEB_BUILD=1 \
    ./run-tests.sh --tags "not @e2e and not @site-publication" ); then
  echo "site-evidence: the conversion suite failed" >&2
  exit 1
fi
conversion_total="$(selected_total "$conversion_result")"
if [ "$conversion_total" -le 0 ]; then
  echo "site-evidence: the conversion suite selected no scenario" >&2
  exit 1
fi

echo "site-evidence: page content, licensing, browser check and conversion suite passed"
