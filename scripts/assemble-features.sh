#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Assemble the permanent corpus (givn/specs/**) and every active change's
# specs (givn/changes/*/specs/**) into a temporary root and print that root on
# stdout. The Gherkin-aware merge — dropping the permanent scenarios a delta
# replaces and the deltas' retired placeholders — lives in
# assemble_features.py.
#
# cucumber-rs takes a single feature root, so ./run-tests.sh and
# ./measure-coverage.sh both use this assembly. The caller owns cleanup of the
# printed directory.
set -euo pipefail
cd "$(dirname "$0")/.."

root="$(mktemp -d)"
python3 scripts/assemble_features.py --root "$PWD" --out "$root"

printf '%s\n' "$root"
