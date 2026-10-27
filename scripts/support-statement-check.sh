#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Refuse a public support statement that disagrees with itself, carries the
# withdrawn export-family wording, promises a plan the evidence does not
# exercise, or is not backed by the one generated fixture.
#
# Usage: scripts/support-statement-check.sh [--root DIR]
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/.." && pwd)"
while [ "$#" -gt 0 ]; do
  case "$1" in
    --root) [ "$#" -ge 2 ] || { echo "usage: support-statement-check.sh [--root DIR]" >&2; exit 2; }; root="$2"; shift 2 ;;
    -h|--help) echo "usage: support-statement-check.sh [--root DIR]"; exit 0 ;;
    *) echo "support-statement: unknown argument: $1" >&2; exit 2 ;;
  esac
done
root="$(cd "$root" && pwd)"
env_file="$root/scripts/support-statement.env"
[ -f "$env_file" ] || { echo "support-statement: no constants at $env_file" >&2; exit 1; }
# shellcheck source=support-statement.env
. "$env_file"

failures=0
fail() { printf 'support-statement: %s\n' "$*" >&2; failures=$((failures + 1)); }

norm() { tr '\n' ' ' | tr -s '[:space:]' ' ' | sed -e 's/^ //' -e 's/ $//'; }
text() { [ -f "$root/$1" ] || { fail "$1: missing carrier"; printf ''; return 0; }; cat "$root/$1"; }
has() { case "$1" in *"$2"*) return 0 ;; *) return 1 ;; esac; }
count() { printf '%s' "$1" | grep -Fo -- "$2" | wc -l | tr -d '[:space:]' || true; }

check_carrier() { # name text
  local name="$1" body="$2" positive shrunk
  positive="$(count "$body" "$SUPPORT_PROJECT")"
  shrunk="$(count "$body" "$SUPPORT_SHRUNK")"
  if [ $((positive + shrunk)) -ne 1 ]; then
    fail "$name: does not carry exactly one support sentence (positive=$positive shrunk=$shrunk)"
  fi
  has "$body" "$BOUNDARY" || fail "$name: does not carry the boundary sentence"
  has "$body" "$NON_AFFILIATION" || fail "$name: does not carry the non-affiliation sentence"
}

scan_forbidden() { # name text
  local name="$1" body="$2" phrase
  for phrase in \
    'supported export' \
    'export family' \
    'Released Roster detail-table' \
    'not a supported duty plan' \
    'This tool converts a crew duty plan into a calendar file.' \
    'The project is not affiliated with any duty-plan provider or airline.'; do
    if has "$body" "$phrase"; then
      fail "$name: carries the withdrawn wording '$phrase'"
    fi
  done
  for phrase in 'will support' 'will convert' 'coming soon' 'future support' 'plans to support' 'roadmap'; do
    if has "$body" "$phrase"; then
      fail "$name: carries the future-support wording '$phrase'"
    fi
  done
}

# --- Carrier 1: the website's statements, parsed from the page source. -------
page_html="$(text web/index.html)"
page_values="$(python3 - "$root/web/index.html" <<'PY'
import sys
from html.parser import HTMLParser

class Statements(HTMLParser):
    def __init__(self):
        super().__init__()
        self.name = None
        self.buffer = []
        self.values = {}
    def handle_starttag(self, tag, attrs):
        if tag != "li":
            return
        attributes = dict(attrs)
        if "data-site-statement" in attributes:
            self.name = attributes["data-site-statement"]
            self.buffer = []
    def handle_data(self, data):
        if self.name is not None:
            self.buffer.append(data)
    def handle_endtag(self, tag):
        if tag == "li" and self.name is not None:
            self.values[self.name] = " ".join("".join(self.buffer).split())
            self.name = None

parser = Statements()
parser.feed(open(sys.argv[1], encoding="utf-8").read())
for name in ("what-it-does", "non-affiliation"):
    print(f"{name}\t{parser.values.get(name, '')}")
PY
)"
what="$(printf '%s\n' "$page_values" | awk -F '\t' '$1 == "what-it-does" {print $2}')"
nonaff="$(printf '%s\n' "$page_values" | awk -F '\t' '$1 == "non-affiliation" {print $2}')"
if [ "$what" != "$SUPPORT_SITE $BOUNDARY" ] && [ "$what" != "$SUPPORT_SITE_SHRUNK $BOUNDARY" ]; then
  fail "web/index.html: what-it-does does not state the support sentence followed by the boundary"
fi
[ "$nonaff" = "$NON_AFFILIATION" ] || fail "web/index.html: non-affiliation does not state the named disclaimer"
has "$(printf '%s' "$page_html" | norm)" "$NON_AFFILIATION" || fail "web/index.html: the footer prose does not carry the non-affiliation sentence"
has "$(printf '%s' "$page_html" | norm)" 'Lufthansa cabin-crew duty plan published as a Jeppesen PDF export' \
  || fail "web/index.html: the lede does not name the supported plan"

# --- Carriers 2-5: README, profile, chapter 3 subsection, both issue forms. --
readme="$(text README.md | norm)"
check_carrier README.md "$readme"

profile="$(text org/profile/README.md | norm)"
check_carrier 'org/profile/README.md' "$profile"

ch3_subsection="$(awk '
  /^### The supported duty plan and the support statement/ { inside = 1 }
  inside && /^### / && !/^### The supported duty plan and the support statement/ { exit }
  inside { print }
' "$root/docs/arc42/03-context-and-scope.md" | norm)"
[ -n "$ch3_subsection" ] || fail "docs/arc42/03-context-and-scope.md: the support-statement subsection is missing"
check_carrier 'docs/arc42/03-context-and-scope.md' "$ch3_subsection"

for form in bug.yml proposal.yml; do
  body="$(text ".github/ISSUE_TEMPLATE/$form" | norm)"
  check_carrier ".github/ISSUE_TEMPLATE/$form" "$body"
  has "$body" 'id: converted' || fail ".github/ISSUE_TEMPLATE/$form: asks no conversion outcome"
  has "$body" 'id: refusal' || fail ".github/ISSUE_TEMPLATE/$form: asks no refusal outcome"
  if has "$body" 'Released Roster'; then fail ".github/ISSUE_TEMPLATE/$form: carries the withdrawn family name"; fi
  if has "$body" '(supported)'; then fail ".github/ISSUE_TEMPLATE/$form: carries the withdrawn supported marker"; fi
  if has "$body" 'export-family'; then fail ".github/ISSUE_TEMPLATE/$form: carries the withdrawn export-family field"; fi
  if has "$body" 'export family'; then fail ".github/ISSUE_TEMPLATE/$form: carries the withdrawn export-family wording"; fi
done

# --- The withdrawn / future-support scan on the public carriers. -------------
for carrier in README.md org/profile/README.md .github/pull_request_template.md CONTRIBUTING.md; do
  scan_forbidden "$carrier" "$(text "$carrier" | norm)"
done
scan_forbidden 'docs/arc42/03-context-and-scope.md' "$ch3_subsection"
scan_forbidden 'web/index.html' "$(printf '%s' "$page_html" | norm)"

# --- The evidence record. ----------------------------------------------------
record="$(text docs/evidence/supported-duty-plan.md | norm)"
if [ -z "$record" ]; then
  fail 'docs/evidence/supported-duty-plan.md: the evidence record is missing'
else
  has "$record" "$SUPPORT_PROJECT" || fail 'docs/evidence/supported-duty-plan.md: does not carry the literal support sentence'
  has "$record" 'apps/cli/tests/steps/support.rs' || fail 'docs/evidence/supported-duty-plan.md: does not name the fixture builder'
  has "$record" 'Crew member converts a supported duty plan and receives a calendar file' \
    || fail 'docs/evidence/supported-duty-plan.md: does not name the conversion scenario'
  has "$record" '2099-06' || fail 'docs/evidence/supported-duty-plan.md: does not name the fictional roster month 2099-06'
  for label in 'Date (LT)' 'Trip' 'Report (LT)' 'Pos' 'Activity' 'From' 'To' 'Start (UTC)' 'End (UTC)' 'A/C' 'Blocktime' 'Timezone Difference' 'Layover' 'Tags'; do
    has "$record" "$label" || fail "docs/evidence/supported-duty-plan.md: does not name the printed column '$label'"
  done
  for code in FREE OFF U FZ SB SBX RE TK DTUSA EM; do
    has "$record" "$code" || fail "docs/evidence/supported-duty-plan.md: does not name the activity code '$code'"
  done
  has "$record" 'LU742' || fail 'docs/evidence/supported-duty-plan.md: does not name the travel leg the fixture prints'
  has "$record" 'no real plan' || fail 'docs/evidence/supported-duty-plan.md: does not state the privacy rule'
  if has "$record" 'DTSTART'; then fail 'docs/evidence/supported-duty-plan.md: carries the DTSTART property'; fi
  if has "$record" 'CREATED'; then fail 'docs/evidence/supported-duty-plan.md: carries the CREATED property'; fi
  if printf '%s' "$record" | grep -Eq '\b(19|20)[0-9]{2}-[0-9]{2}-[0-9]{2}\b'; then
    fail 'docs/evidence/supported-duty-plan.md: carries a complete calendar date'
  fi
fi

if [ "$failures" -ne 0 ]; then
  printf 'support-statement: FAILED — %d finding(s)\n' "$failures" >&2
  exit 1
fi
printf '%s\n' 'support-statement: clean — 5 carriers agree; evidence record names apps/cli/tests/steps/support.rs and Crew member converts a supported duty plan and receives a calendar file'
