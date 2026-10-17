#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
#
# The adoption interface (design.md "The adoption command"): bind the selected
# batch of already-existing public-home objects to their declared counterparts
# without creating, modifying or deleting anything live.
#
# Every import runs in a staged copy of the declaration whose state starts as a
# copy of the applied state; only a fully successful batch is moved into the
# declaration. `tofu import` reads the live object and records its identity; no
# `tofu apply` and no GitHub write ever runs here.
#
# Usage:
#   scripts/adopt-public-home.sh [--declaration DIR] [--batch FILE]
#                                [--state FILE] [--api-url URL] [--token TOKEN]
#
# Required environment: TF_VAR_state_passphrase (at least 16 characters), plus
# whatever the declaration's variables require (TF_VAR_billing_email).
#
# Refusals print `refused <object>: <reason>` on stderr and exit 2.
set -euo pipefail

script_dir="$(CDPATH= cd -- "$(dirname "$0")" && pwd)"
repo_root="$(CDPATH= cd -- "$script_dir/.." && pwd)"

declaration="$repo_root/infra"
batch=""
state=""
api_url="https://api.github.com"
token="${GITHUB_TOKEN:-}"

staging=""
tmpdir=""

cleanup() {
  [ -n "$staging" ] && rm -rf "$staging"
  [ -n "$tmpdir" ] && rm -rf "$tmpdir"
}
trap cleanup EXIT INT TERM

refuse() {
  printf 'refused %s: %s\n' "$1" "$2" >&2
  exit 2
}

usage() {
  cat <<'EOF'
Usage: scripts/adopt-public-home.sh [options]

  --declaration DIR  the declaration workspace (default: infra/)
  --batch FILE       the selected batch (default: <declaration>/adoption-batch.txt)
  --state FILE       the applied state, inside the declaration
                     (default: <declaration>/terraform.tfstate)
  --api-url URL      the GitHub API base URL (default: https://api.github.com)
  --token TOKEN      the owner token (default: $GITHUB_TOKEN)

Required environment: TF_VAR_state_passphrase (at least 16 characters) and
whatever the declaration's variables require (TF_VAR_billing_email).
EOF
}

need_value() {
  [ "$2" -ge 2 ] || refuse command "$1 requires a value"
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --declaration)
      need_value "$1" "$#"
      declaration="$2"
      shift 2
      ;;
    --batch)
      need_value "$1" "$#"
      batch="$2"
      shift 2
      ;;
    --state)
      need_value "$1" "$#"
      state="$2"
      shift 2
      ;;
    --api-url)
      need_value "$1" "$#"
      api_url="$2"
      shift 2
      ;;
    --token)
      need_value "$1" "$#"
      token="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      refuse command "unknown argument: $1"
      ;;
  esac
done

if [ -z "$batch" ]; then
  batch="$declaration/adoption-batch.txt"
fi
if [ -z "$state" ]; then
  state="$declaration/terraform.tfstate"
fi

# ---------------------------------------------------------------------------
# 1. Preflight the inputs, before anything is staged and before any object is
#    read.
# ---------------------------------------------------------------------------

command -v tofu >/dev/null 2>&1 || refuse tofu "not found on PATH"
command -v python3 >/dev/null 2>&1 || refuse python3 "not found on PATH"

passphrase="${TF_VAR_state_passphrase:-}"
if [ "${#passphrase}" -lt 16 ]; then
  refuse passphrase "TF_VAR_state_passphrase must be at least 16 characters"
fi

[ -d "$declaration" ] || refuse declaration "not a directory: $declaration"
declaration_abs="$(CDPATH= cd -- "$declaration" && pwd -P)" ||
  refuse declaration "cannot resolve: $declaration"

[ -f "$batch" ] || refuse batch "not a file: $batch"
batch_dir="$(CDPATH= cd -- "$(dirname "$batch")" && pwd -P)" ||
  refuse batch "cannot resolve: $batch"
batch_abs="$batch_dir/$(basename "$batch")"

state_dir="$(CDPATH= cd -- "$(dirname "$state")" 2>/dev/null && pwd -P)" ||
  refuse state "cannot resolve the directory of: $state"
state_abs="$state_dir/$(basename "$state")"
case "$state_abs" in
  "$declaration_abs"/*) ;;
  *) refuse state "$state_abs is outside the declaration $declaration_abs" ;;
esac

# The batch is reviewable data, so every line is parsed before staging: the
# selected set is known to be well formed before any object is touched.
tmpdir="$(mktemp -d)"
entries="$tmpdir/batch.entries"
: > "$entries"
line_no=0
entry_count=0
while IFS= read -r line || [ -n "$line" ]; do
  line_no=$((line_no + 1))
  line="${line%$'\r'}"
  case "$line" in
    ''|'#'*) continue ;;
  esac
  case "$line" in
    *$'\t'*) ;;
    *) refuse batch "line $line_no has no import identifier: $line" ;;
  esac
  address="${line%%$'\t'*}"
  import_id="${line#*$'\t'}"
  [ -n "$address" ] ||
    refuse batch "line $line_no has an empty address: $line"
  [ -n "$import_id" ] ||
    refuse batch "line $line_no has no import identifier: $line"
  case "$address" in
    *[[:space:]]*) refuse batch "line $line_no has a malformed address: $line" ;;
  esac
  case "$import_id" in
    *$'\t'*) refuse batch "line $line_no has more than one import identifier: $line" ;;
  esac
  printf '%s\t%s\n' "$address" "$import_id" >> "$entries"
  entry_count=$((entry_count + 1))
done < "$batch_abs"
[ "$entry_count" -gt 0 ] || refuse batch "no entries in $batch_abs"

# The provider must never fall back to `gh auth token` or the real endpoint.
export GITHUB_TOKEN="$token"
export GITHUB_BASE_URL="$api_url"
export TF_IN_AUTOMATION=1

log="$tmpdir/tofu.log"

# The first diagnostic line the configured tool printed, stripped of the box
# drawing and the source excerpt that follows it.
diagnostic() {
  file="$1"
  line="$(grep -m1 -E '(^|[^[:alnum:]_])Error:' "$file" 2>/dev/null || true)"
  if [ -n "$line" ]; then
    printf '%s' "$line" | sed -e 's/^[^A-Za-z]*//' -e 's/[[:space:]]*$//'
    return
  fi
  grep -v '^[[:space:]]*$' "$file" 2>/dev/null | tail -1 || true
}

# ---------------------------------------------------------------------------
# 2. Stage and validate. `tofu init -backend=false` comes before `validate`, so
#    a fresh checkout with no `infra/.terraform/` validates like any other.
# ---------------------------------------------------------------------------

staging="$declaration_abs/.adoption-staging"
rm -rf "$staging"
mkdir -p "$staging" ||
  refuse state "cannot create the staging workspace $staging"

for entry in "$declaration_abs"/* "$declaration_abs"/.[!.]* "$declaration_abs"/..?*; do
  [ -e "$entry" ] || continue
  name="${entry##*/}"
  case "$name" in
    .adoption-staging|.terraform|terraform.tfstate|terraform.tfstate.backup) continue ;;
  esac
  cp -R "$entry" "$staging/" ||
    refuse declaration "cannot copy $name into the staging workspace"
done

if [ -e "$state_abs" ]; then
  cp "$state_abs" "$staging/terraform.tfstate" ||
    refuse state "cannot copy $state_abs into the staging workspace"
fi

if ! (cd "$staging" && tofu init -backend=false -input=false -no-color >"$log" 2>&1); then
  reason="$(diagnostic "$log")"
  refuse declaration "${reason:-tofu init failed}"
fi
if ! (cd "$staging" && tofu validate -no-color >"$log" 2>&1); then
  reason="$(diagnostic "$log")"
  refuse declaration "${reason:-tofu validate failed}"
fi

# ---------------------------------------------------------------------------
# 3. Decrypt the applied state before any import. A state encrypted with a
#    different passphrase refuses here, naming the applied state.
# ---------------------------------------------------------------------------

if [ -f "$staging/terraform.tfstate" ]; then
  if ! (cd "$staging" && tofu state list >"$log" 2>&1); then
    reason="$(diagnostic "$log")"
    refuse state "$state_abs cannot be read with the supplied passphrase${reason:+: $reason}"
  fi
fi

# ---------------------------------------------------------------------------
# 4. Import in batch order. The staged state is discarded on any failure, so
#    the real state and the live configuration are untouched.
# ---------------------------------------------------------------------------

while IFS=$'\t' read -r address import_id; do
  if ! (cd "$staging" && tofu import -input=false -no-color "$address" "$import_id" >"$log" 2>&1); then
    reason="$(diagnostic "$log")"
    refuse "$address" "import $import_id failed${reason:+: $reason}"
  fi
done < "$entries"

# ---------------------------------------------------------------------------
# 5. Verify the batch. `-refresh-only` compares state to live, so an unchanged
#    live object is a no-op; a live object that changed after it was read is a
#    non-no-op change and refuses the batch by address.
# ---------------------------------------------------------------------------

plan_log="$tmpdir/plan.json"
plan_status=0
(cd "$staging" && tofu plan -refresh-only -detailed-exitcode -json -input=false \
  >"$plan_log" 2>&1) || plan_status=$?

if [ "$plan_status" -eq 1 ]; then
  reason="$(diagnostic "$plan_log")"
  refuse plan "the verification plan failed${reason:+: $reason}"
fi

if [ "$plan_status" -eq 2 ]; then
  changed="$(python3 - "$plan_log" <<'PY'
import json
import sys

addresses = []
with open(sys.argv[1], encoding="utf-8") as handle:
    for line in handle:
        line = line.strip()
        if not line:
            continue
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        if message.get("type") not in ("planned_change", "resource_drift"):
            continue
        change = message.get("change") or {}
        actions = change.get("actions") or []
        if actions and actions != ["no-op"]:
            address = ((change.get("resource") or {}).get("addr") or "").strip()
            if address:
                addresses.append(address)

for address in dict.fromkeys(addresses):
    print(address)
PY
)"
  if [ -n "$changed" ]; then
    while IFS= read -r address; do
      [ -n "$address" ] ||
        continue
      printf 'refused %s: the live object changed after it was read\n' "$address" >&2
    done <<EOF
$changed
EOF
    exit 2
  fi
  refuse plan "the verification plan reported an unclassified change"
fi

# ---------------------------------------------------------------------------
# 6. Bind atomically. Staging sits beside the state, so the move is a rename;
#    on the first adoption the backup is a copy of the new state, on later runs
#    it keeps the previous state.
# ---------------------------------------------------------------------------

new_state="$staging/terraform.tfstate"
[ -f "$new_state" ] || refuse state "the import produced no state to record"

if [ -e "$state_abs" ]; then
  cp "$state_abs" "$state_abs.backup" ||
    refuse state "cannot record the previous state as $state_abs.backup"
  mv "$new_state" "$state_abs" ||
    refuse state "cannot record the applied state at $state_abs"
else
  mv "$new_state" "$state_abs" ||
    refuse state "cannot record the applied state at $state_abs"
  cp "$state_abs" "$state_abs.backup" ||
    refuse state "cannot record the applied state backup at $state_abs.backup"
fi

while IFS=$'\t' read -r address import_id; do
  printf 'adopted %s %s\n' "$address" "$import_id"
done < "$entries"
