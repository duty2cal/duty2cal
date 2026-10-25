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

# Every refused check prints one `site-evidence: the <check> …` line, so the
# publication's refusal names the check that failed and never reports an
# unperformed check as passed.
refuse() {
  echo "site-evidence: $*" >&2
  exit 1
}

[ -n "$out" ] || { usage; exit 2; }
[ -d "$repo" ] || { echo "site-evidence: the worktree does not exist: $repo" >&2; exit 2; }
[ -d "$out" ] || { echo "site-evidence: the Site output does not exist: $out" >&2; exit 2; }
# The browser check hands the output to another process, so the path must not
# depend on this process's working directory.
out="$( cd "$out" && pwd )"
repo="$( cd "$repo" && pwd )"

results="$(mktemp -d)"
trap 'rm -rf "$results"' EXIT

# 1. Page content: cheap and purely local.
if ! ( cd "$repo" && scripts/check-site-content.sh "$out" ); then
  refuse "the page content check failed"
fi

# 2. Licensing: MPL-2.0 for code, CC0-1.0 for non-code assets. A missing tool
#    refuses the check by name; it never turns into a skipped check.
if ! command -v reuse >/dev/null 2>&1; then
  refuse "the licensing check cannot run: 'reuse' is not installed"
fi
if ! lint="$( cd "$repo" && reuse lint 2>&1 )"; then
  printf '%s\n' "$lint" >&2
  refuse "the licensing check failed; the offending file is named above"
fi

# The result file names what a filtered run selected. A run that wrote no
# result file performed no check the gate can read, so it refuses too.
selected_total() {
  local file="$1" total
  total="$(python3 - "$file" <<'PY' 2>/dev/null
import json
import sys
with open(sys.argv[1], encoding="utf-8") as handle:
    print(int(json.load(handle).get("total", 0)))
PY
)"
  printf '%s' "${total:-0}"
}

# The result-file contract: a check that selected fewer scenarios than it must
# have performed less than it claims, so it is a refusal, not a pass. "exactly"
# is the browser check (a renamed scenario must not become a silent green);
# "at least one" is the conversion suite (an empty selection proves nothing).
require_selection() {
  local check="$1" file="$2" rule="$3" total
  total="$(selected_total "$file")"
  case "$rule" in
    exactly-one)
      if [ "$total" -ne 1 ]; then
        refuse "the $check selected $total scenarios; exactly one is required"
      fi
      ;;
    at-least-one)
      if [ "$total" -le 0 ]; then
        refuse "the $check selected no scenario; a run that selects nothing is a refusal"
      fi
      ;;
    *) echo "site-evidence: unknown selection rule: $rule" >&2; exit 2 ;;
  esac
}

# One run of the revision's own suite, against the assembled output and with
# the result file the gate reads back.
run_revision_suite() {
  local result="$1"
  shift
  ( cd "$repo" && \
    GIVN_RESULT_FILE="$result" \
    DUTY2CAL_SITE_ROOT="$out" \
    DUTY2CAL_SKIP_WEB_BUILD=1 \
    ./run-tests.sh "$@" )
}

# 3. The browser check: the project's existing browser scenario, driven
#    against the assembled output, and exactly one selected scenario. The driver
#    tree and Chromium are prerequisites of *this* check, so their absence
#    refuses it by name instead of skipping the browser it was meant to drive.
if [ ! -d "$repo/web/e2e/node_modules" ]; then
  refuse "the browser check has no driver tree at '$repo/web/e2e/node_modules'; run 'npm ci --prefix web/e2e' at this revision"
fi
if ! command -v node >/dev/null 2>&1; then
  refuse "the browser check cannot run: 'node' is not installed"
fi
chromium="${CHROMIUM_PATH:-}"
if [ -z "$chromium" ]; then
  chromium="$( ( cd "$repo/web/e2e" && node -e 'const { chromium } = require("playwright"); process.stdout.write(chromium.executablePath());' ) 2>/dev/null || true )"
fi
if [ -z "$chromium" ] || [ ! -x "$chromium" ]; then
  refuse "the browser check cannot run: Chromium is not installed; set CHROMIUM_PATH or run 'npx --prefix web/e2e playwright install chromium'"
fi
browser_result="$results/browser.json"
if ! run_revision_suite "$browser_result" \
    --name "Crew member converts a supported duty plan in the browser and saves the offered calendar"; then
  refuse "the browser check failed"
fi
require_selection "browser check" "$browser_result" exactly-one

# 4. The conversion suite: every non-browser scenario the revision holds. The
#    @site-publication tag keeps a publication's own scenarios out of the suite
#    it runs, so the gate can never re-enter itself.
conversion_result="$results/conversion.json"
if ! run_revision_suite "$conversion_result" \
    --tags "not @e2e and not @site-publication"; then
  refuse "the conversion suite failed"
fi
require_selection "conversion suite" "$conversion_result" at-least-one

echo "site-evidence: page content, licensing, browser check and conversion suite passed"
