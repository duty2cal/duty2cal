#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Move the pinned compiler to a released stable version and update the
# rust-overlay revision the Nix shell resolves. Restores both files when the
# release cannot be resolved or verified.
set -euo pipefail
cd "$(dirname "$0")/.."

usage() {
  printf 'usage: scripts/bump-toolchain.sh <major.minor.patch>\n' >&2
}

new="${1:-}"
if [ -z "$new" ]; then
  usage
  exit 2
fi
if ! [[ "$new" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  printf 'bump-toolchain: %s is not an exact release (major.minor.patch); a moving channel cannot pair the compiler with its LLVM tools\n' "$new" >&2
  exit 2
fi
if ! command -v nix >/dev/null 2>&1; then
  printf 'bump-toolchain: nix is required to update flake.lock and verify the shell\n' >&2
  exit 1
fi

current="$(sed -n 's/^channel = "\(.*\)"/\1/p' rust-toolchain.toml | head -n 1)"
if [ "$current" = "$new" ]; then
  printf 'bump-toolchain: rust-toolchain.toml already pins %s\n' "$new"
  exit 0
fi

backup="$(mktemp -d)"
restore() {
  cp "$backup/rust-toolchain.toml" rust-toolchain.toml
  cp "$backup/flake.lock" flake.lock
}
trap 'restore' ERR
cp rust-toolchain.toml "$backup/rust-toolchain.toml"
cp flake.lock "$backup/flake.lock"

new_file="$(mktemp)"
sed -E "s/^channel = \"[0-9]+\.[0-9]+\.[0-9]+\"$/channel = \"$new\"/" rust-toolchain.toml >"$new_file"
mv "$new_file" rust-toolchain.toml
printf 'bump-toolchain: rust-toolchain.toml now pins %s\n' "$new"

nix flake update rust-overlay
nix develop -c bash -c '
  set -euo pipefail
  scripts/check-toolchain.sh
  rustc -V
  cargo fmt --version
  cargo clippy --version
'

trap - ERR
rm -rf "$backup"
printf 'bump-toolchain: done; review `git diff` and commit\n'
