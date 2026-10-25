#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Assemble the Site output from one revision of the repository (design.md
# "The assembled Site output"). The output holds exactly eight members:
#
#   index.html app.js capability.js styles.css
#   icon.png                                (docs/assets/duty2cal.png)
#   pkg/duty2cal_wasm.js pkg/wasm-inline.js (from scripts/build-web.sh)
#   LICENSE.txt                             (LICENSES/MPL-2.0.txt)
#
# The page's footer anchor is stamped with the Built revision text and the
# commit link, both derived from `--revision`, so the string the page carries
# and the tree it describes are the same commit.
#
# Usage:
#   scripts/build-site.sh --repo DIR --revision REV --out DIR
set -euo pipefail

repo="."
revision=""
out=""

usage() {
  echo "usage: scripts/build-site.sh --repo DIR --revision REV --out DIR" >&2
}

die() {
  echo "build-site: $*" >&2
  exit 1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --repo) [ "$#" -ge 2 ] || { usage; exit 2; }; repo="$2"; shift 2 ;;
    --revision) [ "$#" -ge 2 ] || { usage; exit 2; }; revision="$2"; shift 2 ;;
    --out) [ "$#" -ge 2 ] || { usage; exit 2; }; out="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) usage; die "unknown argument: $1" ;;
  esac
done

[ -n "$revision" ] || { usage; die "--revision is required"; }
[ -n "$out" ] || { usage; die "--out is required"; }
[ -d "$repo" ] || die "the checkout does not exist: $repo"

web="$repo/web"
for member in index.html app.js capability.js styles.css; do
  [ -f "$web/$member" ] || die "the checkout holds no web/$member: $repo"
done

# A page without its bundle would hide the converter behind the startup
# failure on the public site, so the missing build step is named instead.
if [ ! -f "$web/pkg/duty2cal_wasm.js" ] || [ ! -f "$web/pkg/wasm-inline.js" ]; then
  die "the checkout holds no browser bundle under $web/pkg; run scripts/build-web.sh first"
fi

icon_source="$repo/docs/assets/duty2cal.png"
[ -f "$icon_source" ] || die "the project icon is missing: $icon_source"
licence_source="$repo/LICENSES/MPL-2.0.txt"
[ -f "$licence_source" ] || die "the licence text is missing: $licence_source"

describe="$(git -C "$repo" describe --tags --always --match 'v*' "$revision" 2>/dev/null)" \
  || die "cannot derive the Built revision text from '$revision'"
commit="$(git -C "$repo" rev-parse --verify "$revision^{commit}" 2>/dev/null)" \
  || die "cannot resolve the Built revision '$revision'"

grep -q '__BUILT_REVISION__' "$web/index.html" \
  || die "web/index.html carries no footer revision anchor to stamp"
grep -q '__BUILT_COMMIT__' "$web/index.html" \
  || die "web/index.html carries no footer commit anchor to stamp"

rm -rf "$out"
mkdir -p "$out/pkg"

cp "$web/app.js" "$out/app.js"
cp "$web/capability.js" "$out/capability.js"
cp "$web/styles.css" "$out/styles.css"
cp "$web/pkg/duty2cal_wasm.js" "$out/pkg/duty2cal_wasm.js"
cp "$web/pkg/wasm-inline.js" "$out/pkg/wasm-inline.js"
cp "$icon_source" "$out/icon.png"
cp "$licence_source" "$out/LICENSE.txt"

# The revision text and the commit link are the only build-derived visible
# strings. A slash in the replacement would end sed's substitution, so the
# describe text is passed through a delimiter the revision cannot carry.
escaped_describe="$(printf '%s' "$describe" | sed 's/[&|\\]/\\&/g')"
sed "s|__BUILT_REVISION__|$escaped_describe|g; s|__BUILT_COMMIT__|$commit|g" \
  "$web/index.html" > "$out/index.html"

printf 'build-site: assembled the Site output for %s (%s) into %s\n' \
  "$describe" "$commit" "$out"
