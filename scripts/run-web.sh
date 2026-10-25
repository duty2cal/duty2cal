#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Build the Site output and serve it on loopback for manual use (design.md
# "Local runnability"). ES modules and WASM are not reliably loadable from
# file://, and the page behaves as it will in production, stamped footer
# included, when it is served from the assembled output.
set -euo pipefail
cd "$(dirname "$0")/.."

if [ ! -f dist/site/index.html ]; then
  scripts/build-web.sh
  scripts/build-site.sh --repo . --revision HEAD --out dist/site
fi

port="${PORT:-8000}"
echo "serving http://127.0.0.1:$port/"
exec python3 -m http.server "$port" --bind 127.0.0.1 --directory dist/site
