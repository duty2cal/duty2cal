#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Privacy gate: no phone-number or staff-identifier shaped pattern in the tree.
#
# Calendar-syntax aware: an iCalendar UTC timestamp (YYYYMMDDTHHMMSSZ) is not a
# staff identifier, so those tokens are stripped before the pattern is applied.
# This replaces the first draft's guard, which false-positived on DTSTAMP.
set -euo pipefail
cd "$(dirname "$0")/.."

pattern='(\+[0-9]{1,3}/[0-9]{6,})|([0-9]{6}[A-Z])'

report="$(grep -rIEn "$pattern" . \
    --exclude-dir=.git \
    --exclude-dir=target \
    --exclude-dir=private \
    --exclude='*.sh' \
    2>/dev/null \
  | sed -E 's/[0-9]{8}T[0-9]{6}Z//g' \
  | grep -E "$pattern" || true)"

if [ -n "$report" ]; then
  echo "$report"
  echo "pii-guard: FAILED — PII-shaped pattern found"
  exit 1
fi
echo "pii-guard: clean"
