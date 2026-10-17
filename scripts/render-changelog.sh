#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Render one version's section of the change log (design.md "The renderer").
#
# Usage:
#   scripts/render-changelog.sh --version <version> --repo <dir> --out <path>
#                               [--release-text <path>] [--check]
#
# `<version>` is the bare version being released (a leading `v` is refused, as
# the preparation command already refuses one). `<repo>` is the checkout whose
# history supplies the section; `--out` is the change log it replaces the
# version's section in, or the one it prepends the section to. `--release-text`
# additionally writes the newest section alone, which is what the preparation
# stages as the release text. `--check` renders in memory and compares the
# rendered whole file against the committed change log, writing nothing.
#
# The render is deterministic: the range ends at the released revision, the
# order is history order, the section's date is the released revision's own
# committer date (never the clock), the comparison link is a literal URL built
# from the two versions, and no host path or user name enters the output.
#
# The section's date is the released revision's own committer date — the
# `v<version>` tag when that tag exists, and otherwise `HEAD`, the revision the
# tag will point at. Never the clock, and never a refusal for an untagged
# version: a release renders the section, commits the change log and only then
# creates the tag, so the section has to be renderable before the tag exists,
# and it has to state the date the tagged revision will carry or the drift check
# could never agree with the file.
#
# It refuses, leaving its output untouched: a version with a leading `v`, or a
# repository with no `HEAD` to date the section from; a `git-cliff` whose
# version differs from the one recorded beside `cliff.toml`; a capability
# allow-list differing from the use-case ids under `givn/specs/`; a render
# naming no change a user can see; and, under `--check`, a committed change log
# with no section for the version being released, or one that differs from a
# fresh render.
#
# Linux only, like the rest of the release tooling. The command touches only the
# filesystem, the pinned generator and the local git history — no network, no
# credentials, no GitHub.
set -euo pipefail

here=$(cd "$(dirname "$0")/.." && pwd)
config="$here/cliff.toml"
marker='<!-- capability-allow-list:'
machinery_marker='<!-- machinery-skip-list:'
receiving_marker='<!-- user-receiving-paths:'
remote='https://github.com/duty2cal/duty2cal'

usage() {
  cat >&2 <<'EOF'
usage: render-changelog.sh --version <version> --repo <dir> --out <path> [--release-text <path>] [--check]
EOF
}

refuse() {
  printf 'render-changelog: %s\n' "$1" >&2
  exit 1
}

# Normalize a rendered section: paragraphs separated by one blank line, no
# trailing blank line, exactly one final newline.
normalize() {
  awk '
    /^[[:space:]]*$/ { blank = 1; next }
    { if (started && blank) print ""; print; blank = 0; started = 1 }
  '
}

version=
repo=
out=
release_text=
check=no
while [ "$#" -gt 0 ]; do
  case "$1" in
    --version|--repo|--out|--release-text)
      flag=$1
      if [ "$#" -lt 2 ]; then
        usage
        refuse "missing value for $flag"
      fi
      value=$2
      case "$flag" in
        --version) version=$value ;;
        --repo) repo=$value ;;
        --out) out=$value ;;
        --release-text) release_text=$value ;;
      esac
      shift 2
      ;;
    --check)
      check=yes
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage
      refuse "unknown argument: $1"
      ;;
  esac
done

if [ -z "$version" ]; then usage; refuse "a version is required (--version)"; fi
if [ -z "$repo" ]; then usage; refuse "a repository is required (--repo)"; fi
if [ -z "$out" ]; then usage; refuse "an output path is required (--out)"; fi

case "$version" in
  v*|V*) refuse "the version must be bare, without a leading v: $version" ;;
esac
case "$version" in
  *[!0-9A-Za-z.+-]*) refuse "the version is not a bare version: $version" ;;
  [0-9]*.*) ;;
  *) refuse "the version is not a bare version: $version" ;;
esac

if [ ! -d "$repo" ]; then
  refuse "the repository is not a directory: $repo"
fi
if [ ! -f "$config" ]; then
  refuse "the configuration is missing: $config"
fi

# The generator's version is part of the contract: the gate compares bytes, and
# a tool whose output shape moved would make the comparison meaningless. The
# version is recorded beside the configuration and checked here.
pinned=$(sed -n 's/^# git-cliff version: //p' "$config" | head -1)
if [ -z "$pinned" ]; then
  refuse "the pinned generator version is not recorded in $config"
fi
if ! command -v git-cliff >/dev/null 2>&1; then
  refuse "git-cliff is not on PATH; the project pins git-cliff $pinned"
fi
installed=$(git-cliff --version 2>/dev/null | awk '{print $NF}')
if [ "$installed" != "$pinned" ]; then
  refuse "git-cliff $installed is not the pinned version: this repository pins git-cliff $pinned"
fi

# The capability allow-list is the use-case ids present under givn/specs/; a
# capability the repository holds and the configuration leaves out would be
# dropped from every release, so the two are compared and a difference is named.
allow_line=$(grep -F -- "$marker" "$config" | head -1 || true)
if [ -z "$allow_line" ]; then
  refuse "the capability allow-list is not recorded in $config"
fi
allow=$(printf '%s\n' "$allow_line" | sed -e "s|.*$marker||" -e 's|-->||g')
capabilities=
for entry in "$repo"/givn/specs/*/; do
  [ -d "$entry" ] || continue
  capabilities="$capabilities ${entry%/}"
done
capabilities=$(printf '%s\n' ${capabilities# } | sed 's|.*/||')
for allow_name in $allow; do
  matched=no
  for held in $capabilities; do
    if [ "$held" = "$allow_name" ]; then matched=yes; fi
  done
  if [ "$matched" = no ]; then
    refuse "the capability allow-list names a capability the repository does not hold: $allow_name"
  fi
done
for held in $capabilities; do
  matched=no
  for allow_name in $allow; do
    if [ "$held" = "$allow_name" ]; then matched=yes; fi
  done
  if [ "$matched" = no ]; then
    refuse "a capability is missing from the configured allow-list: $held (in $config)"
  fi
done

# The allow-list decides which scopes are released; the machinery skip list
# decides which words are, whatever a revision's scope says — a
# `fix(<capability>):` revision whose words describe the project's own tooling
# is still not a change a reader can act on. Each term is a distinctive tooling
# phrase (comma-separated in the marked line, so a term may contain spaces) and
# is realized as a `skip` parser in the configuration; the two are compared here
# so the marked list cannot drift from what the generator applies.
machinery_line=$(grep -F -- "$machinery_marker" "$config" | head -1 || true)
if [ -z "$machinery_line" ]; then
  refuse "the machinery skip list is not recorded in $config"
fi
machinery=$(printf '%s\n' "$machinery_line" | sed -e "s|.*$machinery_marker||" -e 's|-->||g')
parsers=$(grep -v -F -- "$machinery_marker" "$config")
printf '%s\n' "$machinery" | tr ',' '\n' | while IFS= read -r term; do
  term=$(printf '%s' "$term" | sed -e 's|^[[:space:]]*||' -e 's|[[:space:]]*$||')
  [ -n "$term" ] || continue
  case "$parsers" in
    *"$term"*) ;;
    *) refuse "the machinery skip list names a term the configuration does not skip: $term (in $config)" ;;
  esac
done

# The paths a user receives decide what a released revision must have changed
# to be announced. The marked line is read here and every path it names must be
# in the repository under test: a path that no longer exists would silently
# narrow what can ever be announced, so it stops the render instead. The
# comment above the marked line in the configuration states the line between a
# path a user receives and one that only decides how the project works on what
# it ships.
receiving_line=$(grep -F -- "$receiving_marker" "$config" | head -1 || true)
if [ -z "$receiving_line" ]; then
  refuse "the user-receiving path list is not recorded in $config"
fi
receiving=$(printf '%s\n' "$receiving_line" | sed -e "s|.*$receiving_marker||" -e 's|-->||g')
for path in $receiving; do
  if [ ! -e "$repo/$path" ]; then
    refuse "a configured user-receiving path is not in the repository: $path (in $config)"
  fi
done

# The released revision: the `v<version>` tag when it exists, and otherwise
# `HEAD`, the revision the tag will point at. The section's date is that
# revision's own committer date — never the clock — so a version is renderable
# before its tag exists, which is what the release order needs (render the
# section, commit the change log, then create the tag), and so the tagged
# revision will carry exactly the date the section states.
tag_args=
if released=$(git -C "$repo" rev-parse -q --verify "refs/tags/v$version^{commit}" 2>/dev/null); then
  :
else
  tag_args="--tag v$version"
  if ! released=$(git -C "$repo" rev-parse -q --verify "HEAD^{commit}" 2>/dev/null); then
    refuse "the repository has no HEAD to date version $version from: $repo"
  fi
fi
section_date=$(git -C "$repo" log -1 --format=%cd --date=short "$released")

# The previous release tag: the newest `v*` tag reachable from the released
# revision's parent. With none — the first release — the range is every revision
# reachable from the released revision, so a first release announces everything
# it contains.
previous=$(
  git -C "$repo" describe --tags --abbrev=0 --match 'v*' "$released^" 2>/dev/null || true
)

if [ -n "$previous" ]; then
  previous_version=${previous#v}
else
  previous_version=
fi

# The whole history is rendered and the version's section is taken from it: the
# range that ends at the released revision is expressed by the extraction, and
# the release date is stamped from the released revision. `git-cliff` 2.14.2
# associates a revision with a release tag using the checkout's own tags, and a
# render carried by `--tag` groups a whole history under the newest tag when a
# later tag exists (reproduced on both the fixtures and the real tree), so the
# renderer never passes `--tag` for a version that is already tagged. It does
# pass `--tag v<version>` for one that is not, because the tool needs a name for
# the newest release and that version is the newest by construction: `HEAD`.
rendered=$(git-cliff --config "$config" --repository "$repo" --sort oldest $tag_args 2>/dev/null) || {
  refuse "the generator could not render version $version from $repo"
}

# The generator does not collapse identical release-note subjects, and one
# released change carried by many revisions is one entry. The group markers it
# uses for ordering are stripped here too. The version's section is taken
# without collapsing, because each entry's revision marker must be read before
# anything keys on the line.
section=$(
  printf '%s\n' "$rendered" |
    sed -e 's|<!-- [0-9]* -->||g' -e "s|@@DATE@@|$section_date|" |
    awk -v version="$version" '
      $0 == "## [" version "]" || index($0, "## [" version "] ") == 1 { inside = 1 }
      inside && /^## \[/ && index($0, "## [" version "]") != 1 { inside = 0 }
      inside { print }'
)

# A released revision is announced only when it changed a path a user receives:
# each entry is resolved to its revision, the marker is stripped, and an entry
# whose revision changed none of the configured paths is left out. Rename
# detection is pinned off because it is a checkout configuration — with it on,
# a file moved out of a listed path is listed only under its new one and the
# revision would vanish. A revision whose paths cannot be read refuses, rather
# than being dropped as if it had changed nothing.
section_file=$(mktemp)
kept_file=$(mktemp)
printf '%s\n' "$section" > "$section_file"
while IFS= read -r line; do
  case "$line" in
    '- '*)
      revision=$(printf '%s' "$line" | sed -n 's|.*<!--rev:\([0-9a-f][0-9a-f]*\)-->.*|\1|p')
      text=$(printf '%s' "$line" | sed -e 's|<!--rev:[0-9a-f][0-9a-f]*-->||')
      if [ -z "$revision" ]; then
        printf '%s\n' "$text" >> "$kept_file"
        continue
      fi
      if ! changed=$(git -c diff.renames=false -C "$repo" show --name-only --format= "$revision" 2>/dev/null); then
        rm -f "$section_file" "$kept_file"
        refuse "the changed paths of revision $revision cannot be read: $repo"
      fi
      # This repository's paths carry no whitespace, so word splitting is the
      # list of changed paths. A listed directory names itself and everything
      # under it; a listed file names only itself.
      keep=no
      for path in $changed; do
        for prefix in $receiving; do
          case "$path" in
            "${prefix%/}" | "${prefix%/}"/*) keep=yes ;;
          esac
        done
      done
      if [ "$keep" = yes ]; then
        printf '%s\n' "$text" >> "$kept_file"
      fi
      ;;
    *) printf '%s\n' "$line" >> "$kept_file" ;;
  esac
done < "$section_file"

# Identical subjects now collapse, as before: one entry per released change.
section=$(awk '/^- / { if (seen[$0]++) next } { print }' "$kept_file")
rm -f "$section_file" "$kept_file"

# A group whose only entries the path filter removed leaves a heading with no
# bullets. Drop a `### ` heading whose next non-blank line is another heading or
# the section end, so the change log and the release text never carry an empty
# group. This runs before the visible-count refusal: a section with no bullet
# left still refuses as before.
section=$(printf '%s\n' "$section" | awk '
  { line[NR] = $0 }
  END {
    for (i = 1; i <= NR; i++) {
      if (line[i] ~ /^### /) {
        j = i + 1
        while (j <= NR && line[j] ~ /^[[:space:]]*$/) j++
        if (j > NR || line[j] ~ /^##/) { i = j - 1; continue }
      }
      print line[i]
    }
  }')

visible=$(printf '%s\n' "$section" | grep -c '^- ' || true)
if [ "$visible" -eq 0 ]; then
  refuse "version $version changes nothing a user can see: nothing was written"
fi

if [ -n "$previous" ]; then
  comparison="[$previous_version...$version]($remote/compare/$previous...v$version)"
  section=$(printf '%s\n%s\n' "$section" "$comparison" | normalize)
else
  section=$(printf '%s\n' "$section" | normalize)
fi

# A render replaces the section that already carries the target version and
# prepends a version not yet present, keeping every earlier section verbatim.
if [ -e "$out" ]; then
  heading_present=no
  if grep -qE "^## \[$version\]" "$out"; then
    heading_present=yes
  fi
  if [ "$heading_present" = no ] && [ "$check" = yes ]; then
    refuse "the committed change log holds no section for version $version"
  fi
else
  heading_present=no
fi

rendered_file=$(mktemp)
trap 'rm -f "$rendered_file"' EXIT
if [ "$heading_present" = yes ]; then
  awk -v version="$version" -v replacement="$section" '
    BEGIN { skipping = 0 }
    !skipping && $0 ~ "^## \\[" version "\\]" { print replacement; skipping = 1; next }
    skipping && $0 ~ "^## \\[" { skipping = 0 }
    !skipping { print }
  ' "$out" > "$rendered_file"
else
  {
    printf '%s\n' "$section"
    if [ -e "$out" ]; then cat "$out"; fi
  } > "$rendered_file"
fi

# The diff check compares the rendered whole file against the committed change
# log and writes nothing.
if [ "$check" = yes ]; then
  if ! diff -u "$out" "$rendered_file"; then
    refuse "the committed change log differs from a fresh render of version $version"
  fi
  printf 'change log is current for version %s\n' "$version"
  exit 0
fi

if [ -n "$release_text" ]; then
  printf '%s\n' "$section" > "$release_text"
fi

mkdir -p "$(dirname "$out")"
mv "$rendered_file" "$out"
printf 'rendered version %s into %s\n' "$version" "$out"
if [ -n "$release_text" ]; then
  printf 'release text %s states the newest section for version %s\n' "$release_text" "$version"
fi
