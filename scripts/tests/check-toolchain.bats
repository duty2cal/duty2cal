#!/usr/bin/env bats
# SPDX-License-Identifier: MPL-2.0
# Refusals of scripts/check-toolchain.sh with a stubbed rustc and sysroot; no
# real toolchain and no network. The CI `scripts` job runs these.

setup() {
  repo_root="$(cd "$BATS_TEST_DIRNAME/../.." && pwd)"
  tmp="$(mktemp -d)"
  mkdir -p "$tmp/scripts" "$tmp/bin" \
    "$tmp/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib" \
    "$tmp/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/bin"
  cp "$repo_root/scripts/check-toolchain.sh" "$tmp/scripts/"
  chmod +x "$tmp/scripts/check-toolchain.sh"
  printf '%s\n' '[toolchain]' 'channel = "1.99.1"' >"$tmp/rust-toolchain.toml"

  bindir="$tmp/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/bin"
  touch "$bindir/llvm-cov" "$bindir/llvm-profdata"
  chmod +x "$bindir/llvm-cov" "$bindir/llvm-profdata"

  cat >"$tmp/bin/rustc" <<'EOS'
#!/usr/bin/env bash
case "${1:-}" in
  -V) printf 'rustc %s (deadbeef 2026-10-13)\n' "${STUB_RUSTC_VERSION:-1.99.1}" ;;
  --print) printf '%s\n' "${STUB_TARGET_LIBDIR}" ;;
  *) exit 2 ;;
esac
EOS
  chmod +x "$tmp/bin/rustc"

  export PATH="$tmp/bin:$PATH"
  export STUB_TARGET_LIBDIR="$tmp/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib"
}

teardown() {
  rm -rf "$tmp"
}

@test "a matching compiler and its LLVM tools pass" {
  run "$tmp/scripts/check-toolchain.sh"
  [ "$status" -eq 0 ]
  [[ "$output" == *"toolchain: rustc 1.99.1"* ]]
}

@test "a different compiler is refused by name" {
  export STUB_RUSTC_VERSION=1.98.0
  run "$tmp/scripts/check-toolchain.sh"
  [ "$status" -eq 1 ]
  [[ "$output" == *"does not match"* ]]
}

@test "a missing llvm-cov is refused with the component remedy" {
  rm "$tmp/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov"
  run "$tmp/scripts/check-toolchain.sh"
  [ "$status" -eq 1 ]
  [[ "$output" == *"llvm-cov is missing"* ]]
  [[ "$output" == *"llvm-tools-preview"* ]]
}

@test "a set LLVM_PROFDATA is refused" {
  export LLVM_PROFDATA=/somewhere/llvm-profdata
  run "$tmp/scripts/check-toolchain.sh"
  [ "$status" -eq 1 ]
  [[ "$output" == *"LLVM_PROFDATA is set"* ]]
}

@test "a moving channel is refused" {
  printf '%s\n' '[toolchain]' 'channel = "stable"' >"$tmp/rust-toolchain.toml"
  run "$tmp/scripts/check-toolchain.sh"
  [ "$status" -eq 1 ]
  [[ "$output" == *"must pin an exact release"* ]]
}
