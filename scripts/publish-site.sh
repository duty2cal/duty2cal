#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Publish the Site output built from one Built revision into a git hosting
# target (design.md "scripts/publish-site.sh").
#
# Usage:
#   scripts/publish-site.sh --repo DIR [--revision REV] [--since COMMIT]
#                           --target URL --out DIR --verify-url URL
#                           [--key FILE] [--branch main] [--lock PATH]
#
# Steps, in order: when --since is supplied, decide whether the revision
# changes a site input and exit 0 with `no site input changed` when it does
# not; resolve the Built revision; build the revision's own tree in a detached
# worktree; run the evidence gate in that worktree; publish under flock as one
# fast-forward commit whose tree is exactly the Site output; read the pushed
# tree back; and read the published address until it serves the Built
# revision.
#
# A failure before the replacement leaves the target's previous page in place.
# A failed serving check happens after the replacement, so it names the address
# and the revision the address serves and never reports a working page.
set -euo pipefail

repo=""
revision="HEAD"
since=""
target=""
out=""
verify_url=""
key=""
branch="main"
lock=""

# The site-input paths this command owns. It is deliberately not the change
# log's user-receiving list: that answers what a released archive contains.
site_inputs="crates/* ffi/wasm/* web/* docs/assets/*"
site_inputs="$site_inputs Cargo.toml Cargo.lock rust-toolchain.toml LICENSES/MPL-2.0.txt"
site_inputs="$site_inputs .github/workflows/web.yml .github/workflows/site.yml"

site_input() {
  local path="$1" pattern
  for pattern in $site_inputs; do
    # shellcheck disable=SC2254 # the pattern is the glob being matched
    case "$path" in $pattern) return 0 ;; esac
  done
  return 1
}

usage() {
  cat >&2 <<'EOF'
usage: scripts/publish-site.sh --repo DIR [--revision REV] [--since COMMIT]
                               --target URL --out DIR --verify-url URL
                               [--key FILE] [--branch main] [--lock PATH]
EOF
}

refuse() {
  echo "publish-site: $*" >&2
  exit 1
}

# A failed build or gate step prints its own log before the refusal.
fail_with_log() {
  cat "$1" >&2
  exit 1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --repo) [ "$#" -ge 2 ] || { usage; exit 2; }; repo="$2"; shift 2 ;;
    --revision) [ "$#" -ge 2 ] || { usage; exit 2; }; revision="$2"; shift 2 ;;
    --since) [ "$#" -ge 2 ] || { usage; exit 2; }; since="$2"; shift 2 ;;
    --target) [ "$#" -ge 2 ] || { usage; exit 2; }; target="$2"; shift 2 ;;
    --out) [ "$#" -ge 2 ] || { usage; exit 2; }; out="$2"; shift 2 ;;
    --verify-url) [ "$#" -ge 2 ] || { usage; exit 2; }; verify_url="$2"; shift 2 ;;
    --key) [ "$#" -ge 2 ] || { usage; exit 2; }; key="$2"; shift 2 ;;
    --branch) [ "$#" -ge 2 ] || { usage; exit 2; }; branch="$2"; shift 2 ;;
    --lock) [ "$#" -ge 2 ] || { usage; exit 2; }; lock="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) usage; refuse "unknown argument: $1" ;;
  esac
done

[ -n "$repo" ] || { usage; refuse "--repo is required"; }
[ -n "$target" ] || { usage; refuse "--target is required"; }
[ -n "$out" ] || { usage; refuse "--out is required"; }
[ -n "$verify_url" ] || { usage; refuse "--verify-url is required"; }
[ -d "$repo" ] || refuse "the repository does not exist: $repo"

# Refuse an unclean caller tree. Untracked and ignored files cannot reach the
# Site output, so they are not reported; a modified or deleted tracked file
# would be missed if it were not.
if ! status="$(git -C "$repo" status --porcelain --untracked-files=no 2>/dev/null)"; then
  refuse "the repository cannot be read: $repo"
fi
if [ -n "$status" ]; then
  printf '%s\n' "$status" >&2
  refuse "the working tree at revision '$revision' is unclean; commit or discard the tracked changes first"
fi

# Decide whether the revision changes a site input. Only --since asks the
# question; a tag publication omits it and always builds.
if [ -n "$since" ]; then
  base="$(git -C "$repo" rev-parse --verify "$since^{commit}" 2>/dev/null)"
  changed="$(git -C "$repo" diff --name-only "$base..$revision" 2>/dev/null)"
  changed_site=0
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    if site_input "$path"; then
      changed_site=1
      break
    fi
  done <<< "$changed"
  if [ "$changed_site" -eq 0 ]; then
    echo "no site input changed"
    exit 0
  fi
fi

# Resolve the Built revision: the commit it names and the text the footer
# carries. A page cannot name a revision that did not build it, so an
# unresolvable revision refuses before any build.
commit="$(git -C "$repo" rev-parse --verify "$revision^{commit}" 2>/dev/null)" \
  || refuse "cannot resolve the Built revision '$revision'"
describe="$(git -C "$repo" describe --tags --always --match 'v*' "$commit" 2>/dev/null)" \
  || refuse "cannot derive the Built revision text for '$revision'"

# Build the revision's own tree in a detached worktree. The trap removes it on
# every exit path.
scratch_parent="${TMPDIR:-/tmp}"
scratch="$(mktemp -d "$scratch_parent/duty2cal-site.XXXXXX")"
tree="$scratch/tree"
cleanup() {
  if [ -d "$tree" ]; then
    git -C "$repo" worktree remove --force "$tree" >/dev/null 2>&1 || rm -rf "$tree"
  fi
  rm -rf "$scratch"
}
trap cleanup EXIT

git -C "$repo" worktree add --detach "$tree" "$commit" \
  >/dev/null 2>"$scratch/worktree.err"

# The revision's driver tree is its own pinned set, never the caller's unless
# the lockfiles agree.
caller_lock="$repo/web/e2e/package-lock.json"
revision_lock="$tree/web/e2e/package-lock.json"
if [ -f "$caller_lock" ] && [ -f "$revision_lock" ] && ! cmp -s "$caller_lock" "$revision_lock"; then
  refuse "the driver lockfile '$revision_lock' differs from the caller's; run 'npm ci --prefix web/e2e' at revision '$revision' first"
fi
if [ -d "$repo/web/e2e/node_modules" ]; then
  ln -s "$repo/web/e2e/node_modules" "$tree/web/e2e/node_modules"
else
  refuse "the caller holds no installed driver tree at '$repo/web/e2e/node_modules'; run 'npm ci --prefix web/e2e' first"
fi

# The revision's own browser build, then the assembler.
if ! ( cd "$tree" && ./scripts/build-web.sh ) >"$scratch/build-web.log" 2>&1; then
  fail_with_log "$scratch/build-web.log"
fi
if ! "$tree/scripts/build-site.sh" --repo "$tree" --revision "$commit" --out "$out" \
    >"$scratch/build-site.log" 2>&1; then
  fail_with_log "$scratch/build-site.log"
fi

# Gate in the worktree, with the assembled output handed in.
if ! "$tree/scripts/site-evidence.sh" --output "$out" --repo "$tree" \
    >"$scratch/site-evidence.log" 2>&1; then
  fail_with_log "$scratch/site-evidence.log"
fi

# Publish under an exclusive lock. One fast-forward commit whose tree is
# exactly the Site output; no force push and no merge commit.
git_dir="$(git -C "$repo" rev-parse --absolute-git-dir)"
lock="${lock:-$git_dir/publish-site.lock}"
mkdir -p "$(dirname "$lock")"
exec 9>"$lock"
if ! flock -w 300 9; then
  refuse "cannot acquire the publication lock '$lock'"
fi

if [ -n "$key" ]; then
  export GIT_SSH_COMMAND="ssh -i $(printf '%q' "$key") -o IdentitiesOnly=yes"
fi
export GIT_TERMINAL_PROMPT=0

stage="$scratch/target"
git init -q "$stage"
git -C "$stage" remote add origin "$target"

tip=""
if git -C "$stage" fetch -q origin "$branch" 2>/dev/null; then
  git -C "$stage" checkout -q -B "$branch" FETCH_HEAD
  tip="$(git -C "$stage" rev-parse HEAD)"
fi

# Report an unchanged target as current: compare the fetched tip's tree with
# the assembled output member by member.
if [ -n "$tip" ]; then
  mkdir -p "$scratch/tip"
  if git -C "$stage" archive "$tip" | tar -x -C "$scratch/tip" 2>/dev/null \
      && diff -r "$out" "$scratch/tip" >/dev/null 2>&1; then
    echo "already current"
    exit 0
  fi
fi

if [ -n "$tip" ]; then
  git -C "$stage" checkout -q -B "$branch" "$tip"
else
  git -C "$stage" checkout -q --orphan "$branch"
fi
find "$stage" -mindepth 1 -maxdepth 1 ! -name .git -exec rm -rf {} +
cp -R "$out"/. "$stage"/
git -C "$stage" add -A
git -C "$stage" \
  -c user.name="duty2cal site publication" \
  -c user.email="site-publication@duty2cal.invalid" \
  commit -q -m "publish-site: publish the Site output built from $describe ($commit)"

if ! git -C "$stage" push -q origin "HEAD:refs/heads/$branch" 2>"$scratch/push.err"; then
  cat "$scratch/push.err" >&2
  refuse "the hosting target '$target' refused the publication"
fi

# Read the pushed tree back and compare the member list with the output.
pushed="$(git -C "$stage" rev-parse HEAD)"
git -C "$stage" fetch -q origin "$branch"
if [ "$(git -C "$stage" rev-parse FETCH_HEAD)" != "$pushed" ]; then
  refuse "the hosting target '$target' does not hold the revision that was pushed"
fi
expected="$(cd "$out" && find . -type f | sed 's|^\./||' | LC_ALL=C sort)"
actual="$(git -C "$stage" ls-tree -r --name-only "$pushed" | LC_ALL=C sort)"
if [ "$expected" != "$actual" ]; then
  printf 'publish-site: the Site output holds:\n%s\n' "$expected" >&2
  printf 'publish-site: the hosting target holds:\n%s\n' "$actual" >&2
  refuse "the hosting target '$target' holds a different member list than the Site output"
fi

# Verify the served page. Bounded reads: a fixed attempt count with a short
# wait, never an unbounded loop.
fetch_page() {
  python3 - "$1" <<'PY' 2>/dev/null
import sys
import urllib.request

try:
    with urllib.request.urlopen(sys.argv[1], timeout=5) as response:
        sys.stdout.write(response.read().decode("utf-8", "replace"))
except Exception:
    raise SystemExit(1)
PY
}

served=""
attempt=0
while [ "$attempt" -lt 20 ]; do
  attempt=$((attempt + 1))
  body="$(fetch_page "$verify_url" || true)"
  if [ -n "$body" ]; then
    served="$(printf '%s\n' "$body" \
      | sed -n 's/.*data-site-statement="built-revision">\([^<]*\)<.*/\1/p' \
      | head -n 1)"
    if [ "$served" = "$describe" ]; then
      break
    fi
  fi
  sleep 0.25
done

if [ "$served" != "$describe" ]; then
  refuse "the address '$verify_url' serves the Built revision '${served:-nothing}' instead of '$describe'; the target holds the complete output built from $describe"
fi

echo "publish-site: published $describe ($commit) to $target; $verify_url serves it"
