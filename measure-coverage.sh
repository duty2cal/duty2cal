#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Produce the two Cobertura reports from DISTINCT instrumentation runs
# (design.md "Coverage process boundaries"; givn/commands.yaml `coverage:`).
#
#   non-e2e: the scenario suite without @e2e
#   e2e:     only the @e2e scenarios, which spawn the built CLI
#
# `--ignore-run-fail` keeps the reports coming while a scenario is still RED;
# the green gate is `verify.command`, and ./merge-coverages.sh combines these.
set -euo pipefail
cd "$(dirname "$0")"

non_e2e="${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}"
e2e="${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}"
mkdir -p "$(dirname "$non_e2e")" "$(dirname "$e2e")"

root="$(scripts/assemble-features.sh)"
trap 'rm -rf "$root"' EXIT
export GIVN_FEATURES="$root"

echo "== non-e2e profile =="
cargo llvm-cov -p duty2cal --test features_runner \
  --cobertura --output-path "$non_e2e" --ignore-run-fail -- --tags "not @e2e"

echo "== e2e profile =="
cargo llvm-cov -p duty2cal --test features_runner \
  --cobertura --output-path "$e2e" --ignore-run-fail -- --tags "@e2e"

echo "wrote $non_e2e and $e2e"
