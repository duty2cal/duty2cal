#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Test runner for duty2cal (see design.md "Test runner, strict mode, and
# single-scenario command").
#
# The default run verifies the product: the core/PDF unit and property tests,
# the architecture tests, the golden calendar test, and the
# `export-duty-plan-calendar` Gherkin scenarios. Operational features are
# `@documentation` and are skipped by the assembler; browser scenarios are
# `@browser` and run only in the browser mode.
#
# Usage:
#   ./run-tests.sh                       # product suite, no browser
#   ./run-tests.sh --e2e                 # @e2e CLI scenarios only
#   ./run-tests.sh --browser             # @browser scenarios (builds the Site output)
#   ./run-tests.sh --browser --name "<title>"
#   ./run-tests.sh --name "<title>"      # one scenario
#   ./run-tests.sh --tags "@wip"         # tag filter (never selects @browser)
#
# Runner result contract (givn's archive gate): when GIVN_RESULT_FILE is set,
# this script writes the scope's scenario counts there as
# {"scope": "...", "total": N, "passed": N, "failed": N, "skipped": N}, with the
# scope "regular" for the full run and "e2e" for the @e2e run. The file is
# written whenever the runner produced a summary, so the gate reads counts
# rather than trusting an exit code. A run that selects no scenario is a
# failure, and a named run must select exactly one.
set -euo pipefail
cd "$(dirname "$0")"

# Refuse a compiler that is not the pinned one, or a sysroot without its LLVM
# tools, before any build. See design.md "Coverage guard".
scripts/check-toolchain.sh

transcript="$(mktemp)"
root=""
trap 'if [ -n "${DUTY2CAL_TRANSCRIPT_PATH:-}" ]; then cp "$transcript" "$DUTY2CAL_TRANSCRIPT_PATH" 2>/dev/null || true; fi; rm -rf "$root" "$transcript"' EXIT

phase_line() {
  printf '%s\n' "$*"
  printf '%s\n' "$*" >> "$transcript"
}

filter=()
args=()
has_name=0
has_tags=0
want_e2e=0
want_browser=0
for a in "$@"; do
  case "$a" in
    --e2e)
      want_e2e=1
      ;;
    --browser)
      want_browser=1
      ;;
    --name)
      has_name=1
      args+=("$a")
      ;;
    --name=*)
      has_name=1
      args+=("$a")
      ;;
    --tags)
      has_tags=1
      args+=("$a")
      ;;
    --tags=*)
      has_tags=1
      args+=(--tags "${a#--tags=}")
      ;;
    *)
      args+=("$a")
      ;;
  esac
done

# The browser scenarios drive the rendered page, so the bundle and the Site
# output are built before the runner. `DUTY2CAL_SKIP_WEB_BUILD=1` is for a run
# that only re-executes already built scenarios.
build_web=0
if [ "$want_browser" = 1 ] && [ "${DUTY2CAL_SKIP_WEB_BUILD:-0}" != "1" ]; then
  build_web=1
fi

if [ "$build_web" = 1 ]; then
  phase_start=$SECONDS
  scripts/build-web.sh
  scripts/build-site.sh \
    --repo . \
    --revision "$(git describe --tags --always --match 'v*')" \
    --out dist/site
  export DUTY2CAL_SITE_ROOT="$PWD/dist/site"
  phase_line "phase=site-build seconds=$((SECONDS - phase_start))"
fi

# A named run is already scoped to one scenario; a tag expression is passed
# through with the browser exclusion appended, unless the caller selected the
# browser mode. cucumber-rs rejects --tags together with --name.
if [ "$has_name" = 1 ]; then
  filter=()
elif [ "$want_browser" = 1 ]; then
  filter=(--tags "@browser")
elif [ "$want_e2e" = 1 ]; then
  filter=(--tags "@e2e and not @browser")
elif [ "$has_tags" = 1 ]; then
  filter=()
else
  filter=(--tags "not @browser")
fi

if [ "$has_tags" = 1 ] && [ "$has_name" = 0 ] && [ "$want_browser" = 0 ]; then
  # Keep a caller's tag expression, but never pull browser scenarios into a
  # run that did not build the page.
  rewritten=()
  for a in "${args[@]}"; do
    case "$a" in
      --tags)
        rewritten+=("$a")
        expect_tags=1
        ;;
      *)
        if [ "${expect_tags:-0}" = 1 ]; then
          rewritten+=("($a) and not @browser")
          expect_tags=0
        else
          rewritten+=("$a")
        fi
        ;;
    esac
  done
  args=("${rewritten[@]}")
fi

phase_start=$SECONDS
root="$(scripts/assemble-features.sh)"
phase_line "phase=assemble seconds=$((SECONDS - phase_start))"
export GIVN_FEATURES="$root"

status=0

# The full default run also executes the Rust unit/property/architecture tests
# and the golden test. A scoped run (--name, --tags, --e2e, --browser) has
# already been narrowed by the caller and runs only the scenarios. All classes
# run even when one fails, so a red unit test does not hide a red scenario.
if [ "$has_name" = 0 ] && [ "$has_tags" = 0 ] && [ "$want_e2e" = 0 ] && [ "$want_browser" = 0 ]; then
  phase_start=$SECONDS
  cargo test -p duty2cal-core -p duty2cal-pdf --no-fail-fast \
    2>&1 | tee -a "$transcript" || status=1
  phase_line "phase=rust-tests seconds=$((SECONDS - phase_start))"

  phase_start=$SECONDS
  cargo test -p duty2cal --test golden --no-fail-fast \
    2>&1 | tee -a "$transcript" || status=1
  phase_line "phase=golden seconds=$((SECONDS - phase_start))"
fi

concurrency="${DUTY2CAL_CUCUMBER_CONCURRENCY:-$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 4)}"
cucumber_ran=0
phase_start=$SECONDS
cucumber_ran=1
cargo test -p duty2cal --test features_runner --no-fail-fast -- \
  ${filter[@]+"${filter[@]}"} \
  --concurrency "$concurrency" \
  ${args[@]+"${args[@]}"} \
  2>&1 | tee -a "$transcript" || status=1
phase_line "phase=cucumber seconds=$((SECONDS - phase_start)) concurrency=$concurrency"

summary="$(grep -E '^[0-9]+ scenarios?' "$transcript" | tail -1 || true)"
total="${summary%% *}"
counts="${summary#*\(}"
counts="${counts%\)*}"
passed=0
failed=0
skipped=0
if [ -n "$summary" ]; then
  IFS=',' read -r -a parts <<< "$counts"
  for part in "${parts[@]}"; do
    value="${part%% *}"
    kind="${part##* }"
    case "$kind" in
      passed) passed="$value" ;;
      failed) failed="$value" ;;
      skipped) skipped="$value" ;;
    esac
  done
fi

# A filter that selects nothing proves nothing. A named run must select
# exactly one scenario; every other run must select at least one.
if [ "$cucumber_ran" = 1 ]; then
  if [ -z "$summary" ] || [ "${total:-0}" -lt 1 ]; then
    echo "run-tests: no scenario was selected; a run that selects nothing is a refusal" >&2
    status=1
  fi
  if [ "$has_name" = 1 ] && [ "${total:-0}" -ne 1 ]; then
    echo "run-tests: the name selected ${total:-0} scenarios; exactly one is required" >&2
    status=1
  fi
fi

if [ "$status" -ne 0 ] && [ "$failed" -eq 0 ]; then
  failed=1
fi

if [ -n "${GIVN_RESULT_FILE:-}" ]; then
  # The scope the gate asked for: "e2e" when the run is the @e2e filter.
  scope="regular"
  [ "$want_e2e" = 1 ] && scope="e2e"

  mkdir -p "$(dirname "$GIVN_RESULT_FILE")"
  printf '{"scope":"%s","total":%s,"passed":%s,"failed":%s,"skipped":%s}\n' \
    "$scope" "${total:-0}" "$passed" "$failed" "$skipped" > "$GIVN_RESULT_FILE"
fi

exit "$status"
