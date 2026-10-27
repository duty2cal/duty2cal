#!/usr/bin/env bats
# SPDX-License-Identifier: MPL-2.0
# Early refusals of scripts/publish-site.sh: every case must fail before the
# script creates a worktree, runs a build, or touches a target. The CI `scripts`
# job runs these with no cargo build and no network.

setup() {
  repo_root="$(cd "$BATS_TEST_DIRNAME/../.." && pwd)"
  script="$repo_root/scripts/publish-site.sh"
  tmp="$(mktemp -d)"
}

teardown() {
  rm -rf "$tmp"
}

@test "no arguments prints usage and is refused" {
  run "$script"
  [ "$status" -eq 1 ]
  [[ "$output" == *"usage: scripts/publish-site.sh"* ]]
}

@test "an unknown argument is refused before any build" {
  run "$script" --nonsense
  [ "$status" -eq 1 ]
  [[ "$output" == *"unknown argument: --nonsense"* ]]
}

@test "a missing repository is named" {
  run "$script" \
    --repo "$tmp/missing" \
    --target "$tmp/target.git" \
    --out "$tmp/out" \
    --verify-url https://example.invalid/
  [ "$status" -eq 1 ]
  [[ "$output" == *"the repository does not exist"* ]]
}

@test "a missing target is refused" {
  run "$script" --repo "$tmp" --out "$tmp/out" --verify-url https://example.invalid/
  [ "$status" -eq 1 ]
  [[ "$output" == *"--target is required"* ]]
}

@test "a missing output directory is refused" {
  run "$script" --repo "$tmp" --target "$tmp/target.git" --verify-url https://example.invalid/
  [ "$status" -eq 1 ]
  [[ "$output" == *"--out is required"* ]]
}

@test "a missing verification address is refused" {
  run "$script" --repo "$tmp" --target "$tmp/target.git" --out "$tmp/out"
  [ "$status" -eq 1 ]
  [[ "$output" == *"--verify-url is required"* ]]
}

@test "an unclean tracked tree is refused before a worktree or build" {
  git -C "$tmp" init -q
  git -C "$tmp" config user.email test@example.invalid
  git -C "$tmp" config user.name test
  printf 'one\n' > "$tmp/tracked.txt"
  git -C "$tmp" add tracked.txt
  git -C "$tmp" commit -q -m initial
  printf 'two\n' >> "$tmp/tracked.txt"

  run "$script" \
    --repo "$tmp" \
    --target "$tmp/target.git" \
    --out "$tmp/out" \
    --verify-url https://example.invalid/
  [ "$status" -eq 1 ]
  [[ "$output" == *"is unclean"* ]]
}

@test "an unknown comparison base is refused before a worktree or build" {
  git -C "$tmp" init -q
  git -C "$tmp" config user.email test@example.invalid
  git -C "$tmp" config user.name test
  printf 'one\n' > "$tmp/tracked.txt"
  git -C "$tmp" add tracked.txt
  git -C "$tmp" commit -q -m initial

  run "$script" \
    --repo "$tmp" \
    --target "$tmp/target.git" \
    --out "$tmp/out" \
    --verify-url https://example.invalid/ \
    --since 0000000000000000000000000000000000000000
  [ "$status" -eq 1 ]
  [[ "$output" == *"cannot resolve the comparison base"* ]]
}

@test "a documentation-only revision reports no site input changed and writes nothing" {
  git -C "$tmp" init -q
  git -C "$tmp" config user.email test@example.invalid
  git -C "$tmp" config user.name test
  printf 'one\n' > "$tmp/README.md"
  git -C "$tmp" add README.md
  git -C "$tmp" commit -q -m initial
  printf 'two\n' >> "$tmp/README.md"
  git -C "$tmp" commit -q -am documents

  run "$script" \
    --repo "$tmp" \
    --target "$tmp/target.git" \
    --out "$tmp/out" \
    --verify-url https://example.invalid/ \
    --since HEAD~1 \
    --revision HEAD
  [ "$status" -eq 0 ]
  [[ "$output" == *"no site input changed"* ]]
  [ ! -e "$tmp/out" ]
  [ ! -e "$tmp/target.git" ]
}

@test "an unresolvable Built revision is refused before a worktree or build" {
  git -C "$tmp" init -q
  git -C "$tmp" config user.email test@example.invalid
  git -C "$tmp" config user.name test
  printf 'one\n' > "$tmp/tracked.txt"
  git -C "$tmp" add tracked.txt
  git -C "$tmp" commit -q -m initial

  run "$script" \
    --repo "$tmp" \
    --target "$tmp/target.git" \
    --out "$tmp/out" \
    --verify-url https://example.invalid/ \
    --revision 0000000000000000000000000000000000000000
  [ "$status" -eq 1 ]
  [[ "$output" == *"cannot resolve the Built revision"* ]]
}
