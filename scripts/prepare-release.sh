#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Prepare one release (design.md "The preparation command"): one deterministic
# archive and one checksum per supported platform, plus the release text.
#
# Linux only: the archives are built with GNU tar's determinism flags. The
# command touches only the filesystem and the local git history — no network,
# no credentials, no GitHub.
#
# Usage:
#   scripts/prepare-release.sh --version <version> --binaries <dir> --out <dir> [--repo <dir>]
#
# `<binaries>/<platform>/duty2cal` (or `duty2cal.exe` for Windows) is the input
# for each supported platform. The command writes, into `<out>`,
# `duty2cal-<version>-<platform>.tar.gz` and its `.sha256` for every supported
# platform, plus `release-notes.md`, and prints one line per staged archive and
# one line for the release text. It refuses, leaving `<out>` untouched, when a
# supported platform's executable is missing, when the binaries directory holds
# a platform outside the supported set, when an executable cannot be read, when
# `<out>` already holds a prepared archive for that version, or when the version
# is not a bare version.
set -euo pipefail

# The canonical supported platform set, in release order. The release workflow's
# matrix is written by hand; a disagreement fails loudly — a platform missing
# from the matrix trips the completeness refusal, one outside the set trips the
# whitelist refusal.
PLATFORMS='windows-x86_64 linux-x86_64 linux-arm64 macos-arm64 macos-x86_64'
# Conventional-commit types. Release-note types are quoted in the release text;
# housekeeping types are omitted; a type in neither set is omitted as well, and
# no commit message ever fails a preparation.
RELEASE_NOTE_TYPES='feat fix perf refactor revert'
HOUSEKEEPING_TYPES='chore docs test build ci style'

usage() {
  cat >&2 <<'EOF'
usage: prepare-release.sh --version <version> --binaries <dir> --out <dir> [--repo <dir>]
EOF
}

refuse() {
  printf 'prepare-release: %s\n' "$1" >&2
  exit 1
}

# The executable name a platform uses.
executable_for() {
  case "$1" in
    windows-x86_64) printf 'duty2cal.exe' ;;
    *) printf 'duty2cal' ;;
  esac
}

is_supported() {
  case " $PLATFORMS " in
    *" $1 "*) return 0 ;;
    *) return 1 ;;
  esac
}

# The release-note subjects of the revisions being released, in release order,
# written one per line to the file named by $1.
collect_release_notes() {
  notes=$1
  : > "$notes"
  # The released revision is HEAD. The previous release tag is the newest `v*`
  # tag reachable from HEAD's parent; with none — the first release — the range
  # is every revision reachable from HEAD, so nothing is released unannounced.
  range=HEAD
  if previous=$(git -C "$repo" describe --tags --abbrev=0 --match 'v*' HEAD^ 2>/dev/null); then
    range="$previous..HEAD"
  fi
  git -C "$repo" log --reverse --format='%s' "$range" | while IFS= read -r subject; do
    # A conventional subject is `<type>(<scope>): <note>`; without a colon it
    # carries no type at all and is omitted.
    type_part=${subject%%:*}
    if [ "$type_part" = "$subject" ]; then
      continue
    fi
    note=${subject#*:}
    # The type is what precedes the scope and any breaking-change `!`.
    type=${type_part%%(*}
    type=${type%!}
    case " $HOUSEKEEPING_TYPES " in
      *" $type "*) continue ;;
    esac
    case " $RELEASE_NOTE_TYPES " in
      *" $type "*) ;;
      *) continue ;;
    esac
    note=${note# }
    if [ -z "$note" ]; then
      continue
    fi
    # Identical release notes collapse to one occurrence.
    if grep -Fxq -- "$note" "$notes"; then
      continue
    fi
    printf '%s\n' "$note" >> "$notes"
  done
}

version=
binaries=
out=
repo=
while [ "$#" -gt 0 ]; do
  case "$1" in
    --version|--binaries|--out|--repo)
      flag=$1
      if [ "$#" -lt 2 ]; then
        usage
        refuse "missing value for $flag"
      fi
      value=$2
      case "$flag" in
        --version) version=$value ;;
        --binaries) binaries=$value ;;
        --out) out=$value ;;
        --repo) repo=$value ;;
      esac
      shift 2
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

if [ -z "$version" ]; then
  usage
  refuse "a version is required (--version)"
fi
if [ -z "$binaries" ]; then
  usage
  refuse "a binaries directory is required (--binaries)"
fi
if [ -z "$out" ]; then
  usage
  refuse "an output directory is required (--out)"
fi
if [ -z "$repo" ]; then
  # Default: the checkout containing this script.
  repo=$(cd "$(dirname "$0")/.." && pwd)
fi

# A bare version, so the archive name, the release text and the tag cannot
# disagree: no leading `v`, and nothing a tag could not carry.
case "$version" in
  v*|V*) refuse "the version must be bare, without a leading v: $version" ;;
esac
case "$version" in
  *[!0-9A-Za-z.+-]*) refuse "the version is not a bare version: $version" ;;
  [0-9]*.*) ;;
  *) refuse "the version is not a bare version: $version" ;;
esac

if [ ! -d "$binaries" ]; then
  refuse "the binaries directory is not a directory: $binaries"
fi

# No invented platform is ever published.
for entry in "$binaries"/*; do
  if [ ! -d "$entry" ]; then
    continue
  fi
  name=${entry##*/}
  if ! is_supported "$name"; then
    refuse "unsupported platform: $name (supported: $PLATFORMS)"
  fi
done

# An archive set missing a platform would promise a download that does not
# exist, and an unreadable executable stages nothing.
for platform in $PLATFORMS; do
  exe=$(executable_for "$platform")
  path="$binaries/$platform/$exe"
  if [ ! -f "$path" ]; then
    refuse "no executable was built for platform $platform: $path"
  fi
  if [ ! -r "$path" ]; then
    refuse "the executable for platform $platform cannot be read: $path"
  fi
done

# A published release is immutable, so a prepared version is never replaced.
for platform in $PLATFORMS; do
  archive="duty2cal-$version-$platform.tar.gz"
  if [ -e "$out/$archive" ]; then
    refuse "the release for version $version is already prepared in $out: $archive exists"
  fi
done

# Every refusal has fired by now, so the staged set is built apart from `<out>`
# and moved in only when it is complete.
mkdir -p "$out"
staging=$(mktemp -d "$out/.prepare-release.XXXXXX")
trap 'rm -rf "$staging"' EXIT

for platform in $PLATFORMS; do
  exe=$(executable_for "$platform")
  archive="duty2cal-$version-$platform.tar.gz"
  # Ownership, ordering and time are tar's three nondeterministic inputs; gzip
  # embeds no name or timestamp, so the same inputs reproduce the same bytes.
  # The mode is forced because CI's artifact transport does not carry file
  # permissions: without it a binary that arrives without the execute bit is
  # published unexecutable, which is what the v0.1.0 archives did.
  (
    cd "$binaries/$platform"
    tar --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner --mode=755 -cf - "$exe" | gzip -n
  ) > "$staging/$archive"
  (
    cd "$staging"
    sha256sum "$archive"
  ) > "$staging/$archive.sha256"
done

collect_release_notes "$staging/.release-notes.txt"
{
  printf '# duty2cal %s\n\n' "$version"
  if [ -s "$staging/.release-notes.txt" ]; then
    printf 'Release notes for version %s.\n\n' "$version"
    while IFS= read -r note; do
      printf '%s\n' "- $note"
    done < "$staging/.release-notes.txt"
  else
    printf 'Release notes for version %s: no release-note subjects.\n' "$version"
  fi
} > "$staging/release-notes.md"

for platform in $PLATFORMS; do
  archive="duty2cal-$version-$platform.tar.gz"
  mv "$staging/$archive" "$out/$archive"
  mv "$staging/$archive.sha256" "$out/$archive.sha256"
  digest=$(cut -d' ' -f1 < "$out/$archive.sha256")
  printf 'staged %s sha256:%s\n' "$archive" "$digest"
done
mv "$staging/release-notes.md" "$out/release-notes.md"
printf 'release text release-notes.md names version %s\n' "$version"
