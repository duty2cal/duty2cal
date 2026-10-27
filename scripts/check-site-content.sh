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
# The text inside the element carrying one statement name, with the element's
# own markup removed, so an empty element is refused as well as an absent one.
statement_text() {
  local name="$1"
  tr '\n' ' ' < "$index" \
    | sed -n "s/.*data-site-statement=\"$name\"[^>]*>\([^<]*\)<.*/\1/p" \
    | head -n 1
}

# How many elements carry one statement name. `grep` exits non-zero when a name
# is absent, which `pipefail` would turn into an unannounced exit, so the empty
# result is the count.
statement_count() {
  local matches
  matches="$(grep -o "data-site-statement=\"$1\"" "$index" || true)"
  if [ -z "$matches" ]; then
    echo 0
    return 0
  fi
  printf '%s\n' "$matches" | wc -l | tr -d '[:space:]'
}

for name in "${statement_names[@]}"; do
  count="$(statement_count "$name")"
  if [ "$count" -eq 0 ]; then
    fail "the page carries no $name statement"
  fi
  if [ "$count" -gt 1 ]; then
    fail "the page carries the $name statement $count times; exactly one element per statement name is required"
  fi
  if ! statement_text "$name" | grep -q '[^[:space:]]'; then
    fail "the page's $name statement carries no text"
  fi
done

# The support claim is exact, not merely present: the constants are shared with
# scripts/support-statement-check.sh so a wording revision edits one file.
# shellcheck source=support-statement.env
. "$(dirname "$0")/support-statement.env"
what="$(statement_text what-it-does)"
if [ "$what" != "$SUPPORT_SITE $BOUNDARY" ] && [ "$what" != "$SUPPORT_SITE_SHRUNK $BOUNDARY" ]; then
  fail "the page's what-it-does statement does not name the supported plan and its boundary"
fi
nonaff="$(statement_text non-affiliation)"
[ "$nonaff" = "$NON_AFFILIATION" ] \
  || fail "the page's non-affiliation statement does not name Lufthansa and Jeppesen"
grep -Fq -- "$NON_AFFILIATION" "$index" \
  || fail "the page's footer prose does not carry the non-affiliation sentence"
grep -Fq 'Lufthansa cabin-crew duty plan published as a Jeppesen PDF export' "$index" \
  || fail "the page's lede does not name the supported plan"

# The Built revision anchor names the commit and wraps the revision text.
grep -Eq 'href="https://github\.com/duty2cal/duty2cal/commit/[0-9a-f]{7,40}"' "$index" \
  || fail "the page links no commit for its Built revision"
grep -Eq '<span data-site-statement="built-revision">[^<]+</span>' "$index" \
  || fail "the page names no Built revision text"

# The licence file is a separate built output.
[ -s "$out/LICENSE.txt" ] || fail "the output holds no LICENSE.txt"
[ -s "$out/icon.svg" ] || fail "the output holds no icon.svg"
[ -s "$out/icon.png" ] || fail "the output holds no icon.png"

# The tab icon and the header mark come from the page's own origin; the vector
# is the icon and the mark. The raster alternate is asserted below.
svg_line="$(grep -nE '<link[^>]+rel="icon"[^>]+type="image/svg[+]xml"[^>]+href="\./icon\.svg"' "$index" | head -n 1 | cut -d: -f1 || true)"
[ -n "$svg_line" ] \
  || fail "the page's tab declares no vector icon at ./icon.svg with type=\"image/svg+xml\""
# The header mark is the vector; the raster there is a refusal.
if grep -Eq '<img[^>]+src="\./icon\.png"[^>]*>' "$index"; then
  fail "the page's header mark is the raster; the vector mark is required"
fi
png_line="$(grep -nE '<link[^>]+rel="icon"[^>]+type="image/png"[^>]+href="\./icon\.png"' "$index" | head -n 1 | cut -d: -f1 || true)"
[ -n "$png_line" ] \
  || fail "the page's tab declares no raster alternate at ./icon.png with type=\"image/png\""
[ "$svg_line" -lt "$png_line" ] \
  || fail "the page declares the raster tab icon (line $png_line) before the vector (line $svg_line); the vector must be the tab's first declaration"
grep -Eq '<img[^>]+src="\./icon\.svg"[^>]*>' "$index" \
  || fail "the page's header mark does not point at ./icon.svg"
grep -Eq '<img[^>]+src="\./icon\.svg"[^>]+alt="duty2cal"' "$index" \
  || fail "the page's header icon carries no alt text naming the project"
grep -Eq '<img[^>]+width="48"[^>]+height="48"' "$index" \
  || fail "the page's header icon is not declared at 48 by 48"

# The vector member is a plain path drawing in the mark's palette. A malformed
# document is refused by the parse itself; the per-element rules refuse anything
# that is not a plain path drawing in the mark's colours.
python3 - "$out/icon.svg" <<'PY' \
  || fail "icon.svg is not a plain palette vector of the mark: $out/icon.svg"
import sys
import xml.etree.ElementTree as ET

svg_ns = "{http://www.w3.org/2000/svg}"
palette = {"#0B1F3F", "#F4F1EA", "#FF9F1C"}
forbidden = {
    "image", "text", "style", "filter", "clipPath", "mask",
    "linearGradient", "radialGradient", "use",
}

root = ET.parse(sys.argv[1]).getroot()
if root.tag != svg_ns + "svg":
    sys.exit("the document's root is not an SVG element")
if root.get("viewBox") != "0 0 1044 1044":
    sys.exit("the document does not carry the mark's 1044-unit viewBox")

for element in root.iter():
    local = element.tag.rsplit("}", 1)[-1]
    if local in forbidden:
        sys.exit(f"the document carries <{local}>")
    if element.get("style"):
        sys.exit("the document carries a style attribute")
    for name, value in element.attrib.items():
        attribute = name.rsplit("}", 1)[-1]
        if attribute == "href" and "//" in value:
            sys.exit(f"the document references another origin: {value}")
        if attribute in {"fill", "stroke", "stop-color"} and value.lower() != "none":
            if value.upper() not in palette:
                sys.exit(f"{attribute}={value!r} is outside the palette")
PY

# No asset may be loaded from another origin; the CSP forbids it and the
# content check refuses the markup that would ask. `<a href>` links are
# navigation and stay allowed: the release page and the source live elsewhere
# by design.
foreign_asset_patterns=(
  '(src|srcset|poster)="(https?:)?//[^"]*"'
  '<link[^>]+href="(https?:)?//[^"]*"'
)
for pattern in "${foreign_asset_patterns[@]}"; do
  foreign="$(grep -Eo "$pattern" "$index" || true)"
  [ -z "$foreign" ] \
    || fail "the page content loads an asset from another origin: $foreign"
done

echo "check-site-content: the page carries the required statements and assets"
