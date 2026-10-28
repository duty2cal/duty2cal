#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Check the mechanical dependency split: no third-party PDF extractor in a
# shipped graph, the oracle present in the test graph.
#
# Usage:
#   scripts/check-extraction-dependency.sh --normal --target <target>
#   scripts/check-extraction-dependency.sh --dev --package <package>
#
# Prints one line and exits non-zero with the offending target or package named.
set -euo pipefail
cd "$(dirname "$0")/.."

mode=""
target=""
package=""
while [ "$#" -gt 0 ]; do
  case "$1" in
    --normal)
      mode="normal"
      shift
      ;;
    --dev)
      mode="dev"
      shift
      ;;
    --target)
      target="${2:-}"
      shift 2
      ;;
    --package)
      package="${2:-}"
      shift 2
      ;;
    *)
      echo "check-extraction-dependency: unknown argument $1" >&2
      exit 2
      ;;
  esac
done

case "$mode" in
  normal)
    if [ -z "$target" ]; then
      echo "check-extraction-dependency: --normal requires --target" >&2
      exit 2
    fi
    if cargo tree -e normal --target "$target" -p duty2cal | grep -q 'pdfplumber'; then
      echo "check-extraction-dependency: pdfplumber appears in the normal graph for $target" >&2
      exit 1
    fi
    echo "check-extraction-dependency: no pdfplumber in the normal graph for $target"
    ;;
  dev)
    if [ -z "$package" ]; then
      echo "check-extraction-dependency: --dev requires --package" >&2
      exit 2
    fi
    if ! cargo tree -e dev -p "$package" | grep -q 'pdfplumber'; then
      echo "check-extraction-dependency: the test graph of $package no longer carries the pdfplumber oracle" >&2
      exit 1
    fi
    echo "check-extraction-dependency: pdfplumber is the test oracle of $package"
    ;;
  *)
    echo "check-extraction-dependency: choose --normal or --dev" >&2
    exit 2
    ;;
esac
