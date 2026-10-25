#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Refuse a Site output whose page does not carry the public statements, the
# Built revision link, the licence file or the project icon on the page's own
# origin (design.md "Where the public statements live").
#
# The check reads the assembled output, never the page source, so a page that
# is correct in the repository but lost a statement while it was assembled is
# still refused. Each required statement is an element carrying
# `data-site-statement="<name>"`; exactly one element per name is accepted,
# because two copies of one statement can disagree.
#
# Usage: scripts/check-site-content.sh OUT
set -euo pipefail

out="${1:-}"
fail() {
  echo "check-site-content: page content check failed: $*" >&2
  exit 1
}

[ -n "$out" ] || { echo "usage: scripts/check-site-content.sh OUT" >&2; exit 2; }
[ -d "$out" ] || fail "the output directory does not exist: $out"

index="$out/index.html"
[ -f "$index" ] || fail "the output holds no index.html"

statement_names=(
  what-it-does
  browser-route
  archive-route
  privacy
  non-affiliation
  as-is
  source
  license
  built-revision
)
for name in "${statement_names[@]}"; do
  count="$(grep -o "data-site-statement=\"$name\"" "$index" | wc -l | tr -d '[:space:]')"
  if [ "$count" -eq 0 ]; then
    fail "the page carries no $name statement"
  fi
  if [ "$count" -gt 1 ]; then
    fail "the page carries the $name statement $count times; exactly one element per statement name is required"
  fi
done

# The Built revision anchor names the commit and wraps the revision text.
grep -Eq 'href="https://github\.com/duty2cal/duty2cal/commit/[0-9a-f]{7,40}"' "$index" \
  || fail "the page links no commit for its Built revision"
grep -Eq '<span data-site-statement="built-revision">[^<]+</span>' "$index" \
  || fail "the page names no Built revision text"

# The licence file is a separate built output.
[ -s "$out/LICENSE.txt" ] || fail "the output holds no LICENSE.txt"
[ -s "$out/icon.png" ] || fail "the output holds no icon.png"

# The tab icon and the header mark come from the page's own origin.
grep -Eq '<link[^>]+rel="icon"[^>]+href="\./icon\.png"' "$index" \
  || fail "the page's tab icon does not point at ./icon.png"
grep -Eq '<img[^>]+src="\./icon\.png"[^>]*>' "$index" \
  || fail "the page's header icon does not point at ./icon.png"
grep -Eq '<img[^>]+src="\./icon\.png"[^>]+alt="duty2cal"' "$index" \
  || fail "the page's header icon carries no alt text naming the project"
grep -Eq '<img[^>]+width="48"[^>]+height="48"' "$index" \
  || fail "the page's header icon is not declared at 48 by 48"

# No asset may be loaded from another origin; the CSP forbids it and the
# content check refuses the markup that would ask.
foreign="$(grep -Eo '(src|srcset|poster)="(https?:)?//[^"]*"' "$index" || true)"
[ -z "$foreign" ] || fail "the page loads an asset from another origin: $foreign"
foreign="$(grep -Eo '<link[^>]+href="(https?:)?//[^"]*"' "$index" || true)"
[ -z "$foreign" ] || fail "the page loads an asset from another origin: $foreign"

echo "check-site-content: the page carries the required statements and assets"
