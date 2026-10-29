#!/usr/bin/env bats
# SPDX-License-Identifier: MPL-2.0
# The npm Playwright client and the flake's browser build must agree.

setup() {
  repo_root="$(cd "$BATS_TEST_DIRNAME/../.." && pwd)"
  tmp="$(mktemp -d)"
  mkdir -p "$tmp/scripts" "$tmp/web/e2e"
  cp "$repo_root/scripts/check-playwright-version.sh" "$tmp/scripts/"
  chmod +x "$tmp/scripts/check-playwright-version.sh"
  cat >"$tmp/web/e2e/package-lock.json" <<'EOS'
{"packages": {"node_modules/playwright": {"version": "1.64.1"}}}
EOS
}

teardown() {
  rm -rf "$tmp"
}

@test "matching client and browser build pass" {
  run env PLAYWRIGHT_DRIVER_VERSION=1.64.1 "$tmp/scripts/check-playwright-version.sh"
  [ "$status" -eq 0 ]
  [[ "$output" == *"npm 1.64.1 == nix 1.64.1"* ]]
}

@test "a client bump without the flake lock is refused" {
  run env PLAYWRIGHT_DRIVER_VERSION=1.65.0 "$tmp/scripts/check-playwright-version.sh"
  [ "$status" -eq 1 ]
  [[ "$output" == *"pins Playwright 1.64.1"* ]]
  [[ "$output" == *"browser build is 1.65.0"* ]]
  [[ "$output" == *"nix flake update nixpkgs"* ]]
}

@test "the check requires the Nix shell variable" {
  run env -u PLAYWRIGHT_DRIVER_VERSION "$tmp/scripts/check-playwright-version.sh"
  [ "$status" -eq 1 ]
  [[ "$output" == *"PLAYWRIGHT_DRIVER_VERSION is not set"* ]]
}
