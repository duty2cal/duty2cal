#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Publish one prepared release, but only when the hosting target holds the Site
# output built from the tagged revision (design.md "The release coupling").
#
# Usage:
#   scripts/publish-release.sh --version V --tag TAG --staged DIR
#                              --repo OWNER/NAME --site-repo DIR --target URL
#                              [--key FILE]
#
# `--site-repo` is a git repository (a checkout or clone) used to read the
# hosting target; `--target` is the target's URL. The target's default branch is
# read with `git ls-remote` and the tip is fetched before any `gh` call. A tip
# whose page does not carry the tagged Built revision refuses and names the
# revision it holds; no `gh` command runs in that case, so nothing becomes
# public and the staged set is left untouched.
#
# When the target holds the tagged revision, the command creates the draft
# release with every staged asset, compares the draft's asset set with the
# staged set (`release-notes.md` excluded, because it is the release body and
# never an asset), and publishes the draft only when the two agree.
set -euo pipefail

version=""
tag=""
staged=""
repo=""
site_repo=""
target=""
key=""
branch="main"

usage() {
  cat >&2 <<'EOF'
usage: scripts/publish-release.sh --version V --tag TAG --staged DIR
                                  --repo OWNER/NAME --site-repo DIR --target URL
                                  [--key FILE]
EOF
}

refuse() {
  echo "publish-release: $*" >&2
  exit 1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --version) [ "$#" -ge 2 ] || { usage; exit 2; }; version="$2"; shift 2 ;;
    --tag) [ "$#" -ge 2 ] || { usage; exit 2; }; tag="$2"; shift 2 ;;
    --staged) [ "$#" -ge 2 ] || { usage; exit 2; }; staged="$2"; shift 2 ;;
    --repo) [ "$#" -ge 2 ] || { usage; exit 2; }; repo="$2"; shift 2 ;;
    --site-repo) [ "$#" -ge 2 ] || { usage; exit 2; }; site_repo="$2"; shift 2 ;;
    --target) [ "$#" -ge 2 ] || { usage; exit 2; }; target="$2"; shift 2 ;;
    --key) [ "$#" -ge 2 ] || { usage; exit 2; }; key="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) usage; refuse "unknown argument: $1" ;;
  esac
done

[ -n "$version" ] || { usage; refuse "--version is required"; }
[ -n "$tag" ] || { usage; refuse "--tag is required"; }
[ -n "$staged" ] || { usage; refuse "--staged is required"; }
[ -n "$repo" ] || { usage; refuse "--repo is required"; }
[ -n "$site_repo" ] || { usage; refuse "--site-repo is required"; }
[ -n "$target" ] || { usage; refuse "--target is required"; }

case "$version" in
  v*|V*) refuse "the version must be bare, without a leading v: $version" ;;
esac
[ "$tag" = "v$version" ] || refuse "the tag '$tag' does not name version '$version'"
[ -d "$staged" ] || refuse "the staged directory does not exist: $staged"
[ -d "$site_repo" ] || refuse "the site repository does not exist: $site_repo"

notes="$staged/release-notes.md"
changelog="$staged/CHANGELOG.md"
[ -f "$notes" ] || refuse "the staged set carries no release text: $notes"
[ -f "$changelog" ] || refuse "the staged set carries no change log: $changelog"

archives=("$staged"/*.tar.gz)
checksums=("$staged"/*.tar.gz.sha256)
[ -e "${archives[0]}" ] || refuse "the staged set carries no archive: $staged/*.tar.gz"
for archive in "${archives[@]}"; do
  [ -f "$archive" ] || refuse "the staged set carries no archive: $archive"
  [ -f "$archive.sha256" ] || refuse "no checksum beside the staged archive: $archive"
done
for checksum in "${checksums[@]}"; do
  [ -f "$checksum" ] || refuse "the staged set carries no checksum: $checksum"
done

if [ -n "$key" ]; then
  [ -f "$key" ] || refuse "the deploy key does not exist: $key"
  export GIT_SSH_COMMAND="ssh -i '$key' -o IdentitiesOnly=yes"
fi

# Step 1: verify the hosting target holds the tagged Built revision, before any
# `gh` call. The tip is read from the target itself and fetched into the local
# repository; the page it carries is then read from the fetched object.
refs="$(git -C "$site_repo" ls-remote --heads "$target" 2>/dev/null)" \
  || refuse "cannot read the hosting target '$target'"
tip="$(printf '%s\n' "$refs" | awk -v ref="refs/heads/$branch" '$2 == ref { print $1 }')"
[ -n "$tip" ] || refuse "the hosting target '$target' holds no '$branch' branch to verify"
git -C "$site_repo" fetch --quiet --no-tags "$target" \
  "+refs/heads/$branch:refs/duty2cal/site-$branch" 2>/dev/null \
  || refuse "cannot fetch the hosting target's '$branch' branch from '$target'"
tip="$(git -C "$site_repo" rev-parse "refs/duty2cal/site-$branch")" \
  || refuse "cannot read the fetched hosting target's '$branch' branch"
page="$(git -C "$site_repo" show "$tip:index.html" 2>/dev/null)" \
  || refuse "the hosting target's revision '$tip' carries no page"
held="$(printf '%s\n' "$page" \
  | sed -n 's/.*data-site-statement="built-revision">\([^<]*\)<.*/\1/p' \
  | head -1)"
if [ "$held" != "$tag" ]; then
  refuse "the hosting target's branch tip holds the Built revision '${held:-none}' instead of '$tag'; no release is published"
fi

# Step 2: create the draft release with every staged asset.
gh release create "$tag" \
  --repo "$repo" \
  --verify-tag \
  --draft \
  --notes-file "$notes" \
  "${archives[@]}" "${checksums[@]}" "$changelog"

# Step 3: the draft's asset set must be the staged set. The release text is the
# body, never an asset, so it is excluded on the staged side.
staged_names="$(mktemp)"
draft_names="$(mktemp)"
trap 'rm -f "$staged_names" "$draft_names"' EXIT
(
  cd "$staged"
  find . -maxdepth 1 -type f -printf '%f\n'
) | grep -v '^release-notes\.md$' | sort > "$staged_names"
gh release view "$tag" --repo "$repo" --json assets --jq '.assets[].name' \
  | sort > "$draft_names"
if ! diff -u "$staged_names" "$draft_names" >&2; then
  refuse "the draft release's asset set differs from the staged set; the release stays a draft"
fi

# Step 4: publish the inspected draft.
gh release edit "$tag" --repo "$repo" --draft=false
