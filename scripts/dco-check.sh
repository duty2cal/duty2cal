#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Sign-off gate: every revision in a commit range carries a
# `Signed-off-by:` trailer (DCO, no CLA — see CONTRIBUTING.md).
#
# Usage: scripts/dco-check.sh <base> <head>
#
#   <base>  the revision the range starts after (the pull request's base tip)
#   <head>  the range's tip (the pull request's head revision)
#
# The two revisions are exactly what a pull-request event carries, so the CI
# job passes `github.event.pull_request.base.sha` and
# `github.event.pull_request.head.sha`. Runs against the checkout in the
# current directory; no network, no credentials.
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: scripts/dco-check.sh <base> <head>" >&2
  exit 2
fi

base="$1"
head="$2"

for rev in "$base" "$head"; do
  if ! git rev-parse --quiet --verify "${rev}^{commit}" >/dev/null; then
    echo "dco-check: not a revision in this checkout: $rev" >&2
    exit 2
  fi
done

count=0
unsigned=0
while IFS= read -r revision; do
  [ -n "$revision" ] || continue
  count=$((count + 1))
  trailer="$(git log -1 --format='%(trailers:key=Signed-off-by)' "$revision")"
  if [ -z "$trailer" ]; then
    subject="$(git log -1 --format='%s' "$revision")"
    printf 'no Signed-off-by: %s %s\n' "${revision:0:12}" "$subject"
    unsigned=$((unsigned + 1))
  fi
done <<< "$(git rev-list "${base}..${head}")"

if [ "$unsigned" -ne 0 ]; then
  printf 'dco-check: FAILED - %d of %d revision(s) carry no Signed-off-by trailer; sign off with `git commit -s`\n' \
    "$unsigned" "$count"
  exit 1
fi

printf 'dco-check: clean - every one of the %d revision(s) in %s..%s carries a Signed-off-by trailer\n' \
  "$count" "$base" "$head"
