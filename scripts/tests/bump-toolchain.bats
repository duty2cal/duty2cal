#!/usr/bin/env bats
# SPDX-License-Identifier: MPL-2.0
# Argument and failure behaviour of scripts/bump-toolchain.sh with a stub nix;
# no network, no real toolchain.

setup() {
  repo_root="$(cd "$BATS_TEST_DIRNAME/../.." && pwd)"
  tmp="$(mktemp -d)"
  mkdir -p "$tmp/scripts" "$tmp/bin"
  cp "$repo_root/scripts/bump-toolchain.sh" "$tmp/scripts/"
  chmod +x "$tmp/scripts/bump-toolchain.sh"
  printf '%s\n' \
    '# SPDX-License-Identifier: MPL-2.0' \
    '[toolchain]' \
    'channel = "1.99.1"' \
    'profile = "minimal"' >"$tmp/rust-toolchain.toml"
  printf '%s\n' '{}' >"$tmp/flake.lock"
  cat >"$tmp/bin/nix" <<'EOS'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"${NIX_LOG:?}"
exit 0
EOS
  chmod +x "$tmp/bin/nix"
  export NIX_LOG="$tmp/nix.log"

  mkdir -p "$tmp/tools"
  for tool in bash cat cp dirname head mktemp mv rm sed; do
    resolved="$(command -v "$tool" 2>/dev/null)" || continue
    ln -s "$resolved" "$tmp/tools/$tool"
  done
  tool_path="$tmp/tools"
}

teardown() {
  rm -rf "$tmp"
}

@test "no argument prints usage and exits 2" {
  run env PATH="$tool_path:$tmp/bin" NIX_LOG="$NIX_LOG" "$tmp/scripts/bump-toolchain.sh"
  [ "$status" -eq 2 ]
  [[ "$output" == *"usage: scripts/bump-toolchain.sh"* ]]
}

@test "a moving channel is refused before any edit" {
  run env PATH="$tool_path:$tmp/bin" NIX_LOG="$NIX_LOG" "$tmp/scripts/bump-toolchain.sh" stable
  [ "$status" -eq 2 ]
  [[ "$output" == *"not an exact release"* ]]
  grep -q 'channel = "1.99.1"' "$tmp/rust-toolchain.toml"
  [ ! -s "$NIX_LOG" ]
}

@test "a released version updates the pin and the overlay lock" {
  run env PATH="$tool_path:$tmp/bin" NIX_LOG="$NIX_LOG" "$tmp/scripts/bump-toolchain.sh" 1.100.0
  [ "$status" -eq 0 ]
  grep -q 'channel = "1.100.0"' "$tmp/rust-toolchain.toml"
  grep -q 'flake update rust-overlay' "$NIX_LOG"
  grep -q 'check-toolchain.sh' "$NIX_LOG"
}

@test "a missing nix is refused before any edit" {
  run env PATH="$tool_path" NIX_LOG="$NIX_LOG" "$tmp/scripts/bump-toolchain.sh" 1.100.0
  [ "$status" -eq 1 ]
  [[ "$output" == *"nix is required"* ]]
  grep -q 'channel = "1.99.1"' "$tmp/rust-toolchain.toml"
}

@test "the same version is a no-op" {
  run env PATH="$tool_path:$tmp/bin" NIX_LOG="$NIX_LOG" "$tmp/scripts/bump-toolchain.sh" 1.99.1
  [ "$status" -eq 0 ]
  [[ "$output" == *"already pins 1.99.1"* ]]
  [ ! -s "$NIX_LOG" ]
}
