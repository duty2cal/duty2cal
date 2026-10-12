#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Combine the two Cobertura reports into the badge/summary report
# (givn/commands.yaml `coverage.output_path`).
set -euo pipefail
cd "$(dirname "$0")"

non_e2e="${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}"
e2e="${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}"
output="${GIVN_COVERAGE_OUTPUT_PATH:-coverage/cobertura-coverage.xml}"

if [ ! -f "$non_e2e" ] || [ ! -f "$e2e" ]; then
  echo "merge-coverages: run ./measure-coverage.sh first" >&2
  exit 1
fi

mkdir -p "$(dirname "$output")"
python3 scripts/merge-cobertura.py "$non_e2e" "$e2e" "$output"
