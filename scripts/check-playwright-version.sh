#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# The npm Playwright client and the Nix shell's browser build must be the same
# release: a lockfile bump alone makes `nix develop` unusable for the browser
# suite, and a flake.lock bump alone breaks the npm client. Run inside
# `nix develop`, where PLAYWRIGHT_DRIVER_VERSION comes from the flake.
set -euo pipefail
cd "$(dirname "$0")/.."

driver="${PLAYWRIGHT_DRIVER_VERSION:-}"
if [ -z "$driver" ]; then
  printf 'check-playwright-version: PLAYWRIGHT_DRIVER_VERSION is not set; run inside nix develop\n' >&2
  exit 1
fi

npm_version="$(
  python3 - <<'PY'
import json
print(json.load(open("web/e2e/package-lock.json"))["packages"]["node_modules/playwright"]["version"])
PY
)"

if [ "$npm_version" != "$driver" ]; then
  printf 'check-playwright-version: web/e2e pins Playwright %s but the Nix browser build is %s; update web/e2e and flake.lock together (nix flake update nixpkgs)\n' \
    "$npm_version" "$driver" >&2
  exit 1
fi

printf 'playwright: npm %s == nix %s\n' "$npm_version" "$driver"
