#!/usr/bin/env bats
# SPDX-License-Identifier: MPL-2.0
# The executable evidence for the support-statement check: the clean run and
# one mutated tree per refusal family (a differing carrier, the withdrawn
# wording, the future-support wording, a missing fixture, a missing outcome
# field, and the shrunk statement).

setup() {
  REPO="$(cd "$BATS_TEST_DIRNAME/../.." && pwd)"
  TMP="$(mktemp -d)"
  mkdir -p "$TMP/scripts" "$TMP/web" "$TMP/org/profile" "$TMP/docs/arc42" \
    "$TMP/docs/evidence" "$TMP/.github/ISSUE_TEMPLATE"
  cp "$REPO/scripts/support-statement.env" "$TMP/scripts/"
  cp "$REPO/web/index.html" "$TMP/web/"
  cp "$REPO/README.md" "$TMP/"
  cp "$REPO/org/profile/README.md" "$TMP/org/profile/"
  cp "$REPO/docs/arc42/03-context-and-scope.md" "$TMP/docs/arc42/"
  cp "$REPO/docs/evidence/supported-duty-plan.md" "$TMP/docs/evidence/"
  cp "$REPO/.github/ISSUE_TEMPLATE/bug.yml" "$TMP/.github/ISSUE_TEMPLATE/"
  cp "$REPO/.github/ISSUE_TEMPLATE/proposal.yml" "$TMP/.github/ISSUE_TEMPLATE/"
  cp "$REPO/.github/pull_request_template.md" "$TMP/.github/"
  cp "$REPO/CONTRIBUTING.md" "$TMP/"
  # shellcheck source=../support-statement.env
  . "$REPO/scripts/support-statement.env"
}

teardown() {
  rm -rf "$TMP"
}

run_check() {
  run bash "$REPO/scripts/support-statement-check.sh" --root "$TMP"
}

replace_sentence() { # file old new
  python3 - "$1" "$2" "$3" <<'PY'
import re
import sys

path, old, new = sys.argv[1], sys.argv[2], sys.argv[3]
text = open(path, encoding="utf-8").read()
pattern = re.escape(old).replace(r"\ ", r"\s+")
replaced, count = re.subn(pattern, new, text, count=1)
if count != 1:
    raise SystemExit(f"the expected sentence is missing from {path}")
open(path, "w", encoding="utf-8").write(replaced)
PY
}

@test "every carrier carries the statement and the evidence record names the fixture" {
  run_check
  [ "$status" -eq 0 ]
  [[ "$output" == *"support-statement: clean"* ]]
  [[ "$output" == *"5 carriers agree"* ]]
  [[ "$output" == *"apps/cli/tests/steps/support.rs"* ]]
}

@test "a carrier that disagrees is refused and named" {
  python3 - "$TMP/README.md" <<'PY'
import sys
path = sys.argv[1]
text = open(path, encoding="utf-8").read()
text = text.replace(
    "duty2cal converts a Lufthansa cabin-crew duty plan",
    "duty2cal converts a crew duty plan",
    1,
)
open(path, "w", encoding="utf-8").write(text)
PY
  run_check
  [ "$status" -ne 0 ]
  [[ "$output" == *"README.md: does not carry exactly one support sentence"* ]]
}

@test "the withdrawn export-family wording is refused and named" {
  printf '\nThe supported export is not named here.\n' >> "$TMP/README.md"
  run_check
  [ "$status" -ne 0 ]
  [[ "$output" == *"README.md: carries the withdrawn wording 'supported export'"* ]]
}

@test "future-support wording is refused and named" {
  printf '\nIt will support other plans.\n' >> "$TMP/org/profile/README.md"
  run_check
  [ "$status" -ne 0 ]
  [[ "$output" == *"org/profile/README.md: carries the future-support wording 'will support'"* ]]
}

@test "an evidence record that names no fixture is refused" {
  sed -i '/apps\/cli\/tests\/steps\/support.rs/d' "$TMP/docs/evidence/supported-duty-plan.md"
  run_check
  [ "$status" -ne 0 ]
  [[ "$output" == *"docs/evidence/supported-duty-plan.md: does not name the fixture builder"* ]]
}

@test "a form without the outcome fields is refused" {
  sed -i '/id: refusal/d' "$TMP/.github/ISSUE_TEMPLATE/proposal.yml"
  run_check
  [ "$status" -ne 0 ]
  [[ "$output" == *"proposal.yml: asks no refusal outcome"* ]]
}

@test "a shrunk statement is accepted when it keeps the plan and the boundary" {
  replace_sentence "$TMP/README.md" "$SUPPORT_PROJECT" "$SUPPORT_SHRUNK"
  python3 - "$TMP/web/index.html" "$SUPPORT_SITE $BOUNDARY" "$SUPPORT_SITE_SHRUNK $BOUNDARY" <<'PY'
import sys
path, old, new = sys.argv[1], sys.argv[2], sys.argv[3]
text = open(path, encoding="utf-8").read()
if old not in text:
    raise SystemExit("the site support statement is missing")
open(path, "w", encoding="utf-8").write(text.replace(old, new, 1))
PY
  run_check
  [ "$status" -eq 0 ]
  [[ "$output" == *"support-statement: clean"* ]]
}
