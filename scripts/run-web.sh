#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Serve the built converter page on loopback for manual use (design.md "Local
# runnability"). ES modules and WASM are not reliably loadable from file://.
set -euo pipefail
cd "$(dirname "$0")/.."

[ -f web/pkg/duty2cal_wasm.js ] || scripts/build-web.sh

port="${PORT:-8000}"
echo "serving http://127.0.0.1:$port/"
exec python3 -m http.server "$port" --bind 127.0.0.1 --directory web
