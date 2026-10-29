#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Refuse a toolchain that cannot pair the compiler with its own LLVM tools.
# Runs before the suite and before coverage; see design.md "Coverage guard".
set -euo pipefail
cd "$(dirname "$0")/.."

refuse() {
  printf 'check-toolchain: %s\n' "$*" >&2
  exit 1
}

channel="$(sed -n 's/^channel = "\(.*\)"/\1/p' rust-toolchain.toml | head -n 1)"
[ -n "$channel" ] || refuse "rust-toolchain.toml names no channel"
if ! [[ "$channel" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  refuse "rust-toolchain.toml must pin an exact release (found '$channel'); a moving channel cannot pair the compiler with its LLVM tools"
fi

if [ -n "${LLVM_COV:-}" ] || [ -n "${LLVM_PROFDATA:-}" ]; then
  refuse "LLVM_COV or LLVM_PROFDATA is set; unset both so coverage uses the toolchain's own LLVM tools"
fi

rustc_bin="${RUSTC:-rustc}"
command -v "$rustc_bin" >/dev/null 2>&1 \
  || refuse "rustc is not on PATH; run 'rustup show' in the repository root or enter 'nix develop'"
active="$("$rustc_bin" -V | awk '{print $2}')"
[ "$active" = "$channel" ] \
  || refuse "active rustc $active does not match rust-toolchain.toml channel $channel; run 'rustup show' in the repository root or enter 'nix develop'"

libdir="$("$rustc_bin" --print target-libdir)"
# The Windows runner's rustc prints a backslashed path; Git Bash's dirname
# does not parse it.
libdir="${libdir//\\//}"
bindir="$(dirname "$libdir")/bin"
for tool in llvm-cov llvm-profdata; do
  if [ -x "$bindir/$tool" ] || [ -x "$bindir/$tool.exe" ]; then
    continue
  fi
  refuse "$tool is missing from $bindir; run 'rustup component add llvm-tools-preview --toolchain $channel' or enter 'nix develop'"
done

printf 'toolchain: rustc %s; LLVM tools in %s\n' "$active" "$bindir"
