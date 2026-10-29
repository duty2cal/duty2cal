#!/usr/bin/env bats
# SPDX-License-Identifier: MPL-2.0
# `scripts/setup-dev.sh --check` reports what a machine lacks without
# installing anything and without root. The CI `scripts` job runs these.

setup() {
  repo_root="$(cd "$BATS_TEST_DIRNAME/../.." && pwd)"
  tmp="$(mktemp -d)"
  mkdir -p "$tmp/scripts" "$tmp/bin" "$tmp/web/e2e" "$tmp/home" "$tmp/cargo"
  cp "$repo_root/scripts/setup-dev.sh" "$tmp/scripts/"
  cp "$repo_root/scripts/dev-tool-versions.env" "$tmp/scripts/"
  cp "$repo_root/web/e2e/.nvmrc" "$tmp/web/e2e/"
  chmod +x "$tmp/scripts/setup-dev.sh"

  # A PATH with the core utilities the script uses but without cargo, rustup or
  # the developer tools it is meant to report.
  tool_path=""
  for tool in cat dirname find head ldd sed sort tr uname awk; do
    if resolved="$(command -v "$tool" 2>/dev/null)"; then
      tool_path="$tool_path:$(dirname "$resolved")"
    fi
  done
  tool_path="${tool_path#:}"
  export HOME="$tmp/home"
  export CARGO_HOME="$tmp/cargo"
  unset NVM_DIR || true
}

teardown() {
  rm -rf "$tmp"
}

@test "a missing rustup is reported without installing" {
  run env PATH="$tmp/bin:$tool_path" "$tmp/scripts/setup-dev.sh" --check
  [ "$status" -eq 1 ]
  [[ "$output" == *"rustup is not installed"* ]]
  [ ! -e "$tmp/cargo/bin/rustup" ]
}

@test "a missing cargo is named after rustup is present" {
  cat >"$tmp/bin/rustup" <<'EOS'
#!/usr/bin/env bash
exit 0
EOS
  cat >"$tmp/bin/rustc" <<'EOS'
#!/usr/bin/env bash
printf 'rustc 1.99.1 (deadbeef 2026-10-13)\n'
EOS
  chmod +x "$tmp/bin/rustup" "$tmp/bin/rustc"
  run env PATH="$tmp/bin:$tool_path" "$tmp/scripts/setup-dev.sh" --check
  [ "$status" -eq 1 ]
  [[ "$output" == *"cargo is not installed"* ]]
  [[ "$output" != *"installing"* ]]
}

@test "a different Node major is refused" {
  cat >"$tmp/bin/node" <<'EOS'
#!/usr/bin/env bash
printf 'v22.1.0\n'
EOS
  chmod +x "$tmp/bin/node"
  run env PATH="$tmp/bin:$tool_path" "$tmp/scripts/setup-dev.sh" --check
  [ "$status" -eq 1 ]
  [[ "$output" == *"Node 24 is required"* ]]
}
