#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Privacy gate: no phone-number, staff-identifier or real-calendar shaped
# pattern in the working tree or in the history that would be published.
#
# Calendar-syntax aware. Two things committed material may not do:
#
#   1. carry a token shaped like a staff identifier — except inside an
#      iCalendar timestamp, which is why timestamp parts are stripped before
#      the identifier pattern is applied. The strip tolerates a format
#      placeholder in any digit run, because a test may hold
#      `DTSTART:20990118T{}00Z`.
#   2. carry a real duty plan's timestamp. This project's committed calendars
#      are fictional by convention: their UTC timestamps use 2098 or 2099.
#      Any date-time property outside that convention is treated as a real
#      plan's output, wherever it appears.
#
# Both surfaces are scanned: the working tree (`grep -r`) and every revision
# reachable from the published tip (`git grep` over `git rev-list HEAD`),
# because a file removed from the tree is still published in history. Binary
# content is skipped by the same content test on both surfaces (`grep -I`).
set -euo pipefail
cd "$(dirname "$0")/.."

pattern='(\+[0-9]{1,3}/[0-9]{6,})|([0-9]{6}[A-Z])'
year_pattern='(DTSTART|DTEND|DTSTAMP|UNTIL|RECURRENCE-ID|CREATED)[:;][0-9]{4}'
fictional_years='(2098|2099)'

# Remove iCalendar timestamp parts: first a time (T + HHMMSS + Z, tolerating a
# format placeholder in place of any digit run), then a date with its T
# separator. Order matters: the time is consumed before the date.
strip_calendar() {
  sed -E 's/T[0-9{}]{4,6}Z//g; s/[0-9{}]{8}T//g'
}

excludes=(--exclude-dir=.git --exclude-dir=target --exclude-dir=private --exclude='*.sh')

tree_identifier="$(grep -rIEn "$pattern" . "${excludes[@]}" 2>/dev/null \
  | strip_calendar | grep -E "$pattern" || true)"
tree_year="$(grep -rIEo "$year_pattern" . "${excludes[@]}" 2>/dev/null \
  | grep -Ev "[:;]${fictional_years}" || true)"

# One `git grep` per chunk of revisions keeps the argument list bounded however
# long the history grows; `xargs` appends the chunk after the pattern.
scan_history() { # $1 = git-grep flags, $2 = pattern
  # shellcheck disable=SC2086
  git rev-list HEAD | xargs -r -n 200 git grep -E -I $1 "$2"
}

history_identifier="$(scan_history "-n" "$pattern" 2>/dev/null \
  | strip_calendar | grep -E "$pattern" | sort -u | head -5 || true)"
history_year="$(scan_history "-o" "$year_pattern" 2>/dev/null \
  | grep -Ev "[:;]${fictional_years}" \
  | awk -F: '{ print $1 " " $2 " " $3 }' | sort -u | head -5 || true)"

if [ -n "$tree_identifier" ] || [ -n "$tree_year" ] || [ -n "$history_identifier" ] || [ -n "$history_year" ]; then
  [ -n "$tree_identifier" ] && printf 'identifier-shaped token in the tree:\n%s\n' "$tree_identifier"
  [ -n "$tree_year" ] && printf 'non-fictional calendar date in the tree:\n%s\n' "$tree_year"
  [ -n "$history_identifier" ] && printf 'identifier-shaped token in the published history (revision path match):\n%s\n' "$history_identifier"
  [ -n "$history_year" ] && printf 'non-fictional calendar date in the published history (revision path property):\n%s\n' "$history_year"
  echo "pii-guard: FAILED — private-shaped content found"
  exit 1
fi
echo "pii-guard: clean"
