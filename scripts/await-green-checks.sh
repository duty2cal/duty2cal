#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Wait for the tagged revision's check runs to conclude green, and refuse
# publication when they cannot.
#
# Usage:
#   scripts/await-green-checks.sh --repo <owner/name> --sha <sha> \
#     --exclude-suite <id> [--exclude-suite <id> ...] \
#     [--interval <seconds>] [--timeout <seconds>]
#
# The command reads the check runs on <sha> from GitHub, excludes every suite
# named by --exclude-suite (the release workflow's own suites, so its checks
# cannot gate themselves), and polls until:
#   - no check remains queued or running: it requires at least one success and
#     exits 0; a skipped or neutral conclusion is allowed;
#   - a completed check is not a success: it refuses immediately, naming it;
#   - no check run remains after the exclusion: it refuses as no evidence;
#   - the bounded wait elapses: it refuses, naming every still-running check.
# A check-run query or classification failure refuses rather than passing.
#
# Linux only, like the rest of the release tooling. It touches only the API and
# a temporary directory; it writes nothing into the checkout.
set -euo pipefail

usage() {
  printf 'usage: await-green-checks.sh --repo <owner/name> --sha <sha> --exclude-suite <id> [--exclude-suite <id> ...] [--interval <seconds>] [--timeout <seconds>]\n' >&2
}

refuse() {
  printf 'await-green-checks: %s\n' "$1" >&2
  exit 1
}

repo=
sha=
interval=15
timeout=1800
exclude_suites=()
while [ "$#" -gt 0 ]; do
  case "$1" in
    --repo|--sha|--exclude-suite|--interval|--timeout)
      [ "$#" -ge 2 ] || { usage; refuse "missing value for $1"; }
      value=$2
      case "$1" in
        --repo) repo=$value ;;
        --sha) sha=$value ;;
        --exclude-suite) exclude_suites+=("$value") ;;
        --interval) interval=$value ;;
        --timeout) timeout=$value ;;
      esac
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage
      refuse "unknown argument: $1"
      ;;
  esac
done

[ -n "$repo" ] || { usage; refuse "--repo is required"; }
[ -n "$sha" ] || { usage; refuse "--sha is required"; }
[ "${#exclude_suites[@]}" -gt 0 ] || { usage; refuse "at least one --exclude-suite is required"; }
case "$interval" in
  *[!0-9]*|'') refuse "the interval is not a non-negative integer: $interval" ;;
esac
case "$timeout" in
  *[!0-9]*|'') refuse "the timeout is not a non-negative integer: $timeout" ;;
esac
[ -n "${GH_TOKEN:-}" ] || refuse "GH_TOKEN is not set"
command -v gh >/dev/null 2>&1 || refuse "gh is not on PATH"
command -v jq >/dev/null 2>&1 || refuse "jq is not on PATH"

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

# The excluded suite ids as a JSON array for the jq filter.
exclude_json=$(printf '%s\n' "${exclude_suites[@]}" | jq -R . | jq -s .)

deadline=$((SECONDS + timeout))
poll=0
while :; do
  poll=$((poll + 1))

  if ! gh api --paginate --slurp \
      "repos/${repo}/commits/${sha}/check-runs?filter=latest&per_page=100" \
      > "$work/check-runs.json"; then
    refuse "could not read the check runs of ${sha}"
  fi

  if ! jq -r --argjson suites "$exclude_json" '
      [ .[] | .check_runs[]
        | select(((.check_suite.id // 0) | tostring) as $id
                 | ($suites | index($id) | not)) ]
      | .[] | [ .name, .status, (.conclusion // "pending") ] | @tsv
    ' "$work/check-runs.json" > "$work/checks.tsv"; then
    refuse "could not classify the check runs of ${sha}"
  fi

  [ -s "$work/checks.tsv" ] || \
    refuse "no check evidence: no check run outside this workflow's own suites reports on ${sha}"

  not_green=$(awk -F'\t' \
    '$2 == "completed" && $3 != "success" && $3 != "skipped" && $3 != "neutral" { printf "%s (%s), ", $1, $3 }' \
    "$work/checks.tsv")
  if [ -n "$not_green" ]; then
    printf 'not green:\n'
    awk -F'\t' \
      '$2 == "completed" && $3 != "success" && $3 != "skipped" && $3 != "neutral" { print }' \
      "$work/checks.tsv"
    refuse "the tagged revision does not carry a green check set: ${not_green%, }"
  fi

  running=$(awk -F'\t' '$2 != "completed" { printf "%s (%s)\n", $1, $2 }' "$work/checks.tsv")
  if [ -z "$running" ]; then
    green=$(awk -F'\t' '$2 == "completed" && $3 == "success"' "$work/checks.tsv" | wc -l)
    if [ "$green" -eq 0 ]; then
      refuse "no check on the tagged revision concluded successfully"
    fi
    printf 'every check run on %s outside this workflow'"'"'s own suites concluded: %s successful\n' \
      "$sha" "$green"
    exit 0
  fi

  running_names=$(awk -F'\t' '$2 != "completed" { printf "%s (%s), ", $1, $2 }' "$work/checks.tsv")
  printf 'poll %s: %s check run(s) outside this workflow'"'"'s own suites; %s still running; %s successful\n' \
    "$poll" "$(wc -l < "$work/checks.tsv")" \
    "$(printf '%s\n' "$running" | wc -l)" \
    "$(awk -F'\t' '$2 == "completed" && $3 == "success"' "$work/checks.tsv" | wc -l)"
  printf 'still running:\n%s\n' "$running"

  if [ "$SECONDS" -ge "$deadline" ]; then
    unit=seconds
    [ "$timeout" -eq 1 ] && unit=second
    refuse "the tagged revision's checks did not conclude within ${timeout} ${unit}: ${running_names%, }"
  fi
  sleep "$interval"
done
