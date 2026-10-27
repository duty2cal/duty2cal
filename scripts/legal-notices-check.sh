#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Refuse a page set whose legal notices are missing, unlinked, incomplete, or
# copied into a carrier instead of linked (design.md "The checks", use case
# `state-the-public-notices`).
#
# This check reads the source tree: the three pages under `web/`, the
# organization profile and the README. The assembled output is checked by
# scripts/check-site-content.sh instead, because a page that is complete here
# can still be lost while it is assembled.
#
# Usage: scripts/legal-notices-check.sh [--root DIR]
set -euo pipefail

root="."
while [ "$#" -gt 0 ]; do
  case "$1" in
    --root) [ "$#" -ge 2 ] || { echo "usage: scripts/legal-notices-check.sh [--root DIR]" >&2; exit 2; }; root="$2"; shift 2 ;;
    -h|--help) echo "usage: scripts/legal-notices-check.sh [--root DIR]"; exit 0 ;;
    *) echo "usage: scripts/legal-notices-check.sh [--root DIR]" >&2; exit 2 ;;
  esac
done

here="$(cd "$(dirname "$0")" && pwd)"
# shellcheck source=legal-notices.env
. "$here/legal-notices.env"

fail() {
  echo "legal-notices-check: $*" >&2
  exit 1
}

[ -d "$root" ] || fail "the tree does not exist: $root"

impressum="$root/web/impressum.html"
privacy="$root/web/privacy.html"
index="$root/web/index.html"
for page in "$impressum" "$privacy" "$index"; do
  [ -s "$page" ] || fail "the page set holds no $(basename "$page")"
done

# Both notices are pages of the set; each links the other and the converter
# page, and the converter page's footer links both.
for link in './impressum.html' './privacy.html'; do
  grep -Fq "href=\"$link\"" "$index" \
    || fail "index.html does not link $link"
done
grep -Fq 'href="./privacy.html"' "$impressum" \
  || fail "impressum.html does not link the Datenschutzerklärung"
grep -Fq 'href="./impressum.html"' "$privacy" \
  || fail "privacy.html does not link the Impressum"
for page in "$impressum" "$privacy"; do
  grep -Fq 'href="./index.html"' "$page" \
    || fail "$(basename "$page") does not link the converter page"
done

require() { # page, literal, name
  grep -Fq -- "$2" "$1" || fail "$(basename "$1") carries no $3"
}

for page in "$impressum" "$privacy"; do
  require "$page" "$NOTICE_DOCUMENT_LANG" "German document language"
  require "$page" "$NOTICE_SUMMARY_LANG" "English summary language"
  require "$page" "$NOTICE_SUMMARY_BINDING" "English summary with the binding sentence"
  for marker in "${NOTICE_FORBIDDEN[@]}"; do
    if grep -Fq -- "$marker" "$page"; then
      fail "$(basename "$page") carries the forbidden marker '$marker'"
    fi
  done
  foreign="$(grep -Eo '(src|srcset|poster)="(https?:)?//[^"]*"|<link[^>]+href="(https?:)?//[^"]*"' "$page" || true)"
  if [ -n "$foreign" ]; then
    fail "$(basename "$page") loads an asset from another origin: $foreign"
  fi
done

require "$impressum" "$IMP_NAME" "operator name"
require "$impressum" "$IMP_SERVICE_ADDRESS" "service-address sentence"
require "$impressum" "$IMP_NON_AFFILIATION" "non-affiliation sentence"
require "$impressum" "$IMP_TRADEMARK" "trademark sentence"
require "$impressum" "$IMP_LICENCE" "licence sentence"
require "$impressum" "$IMP_AS_IS" "as-is sentence"
require "$impressum" "$IMP_SOURCE" "source repository"

# The Impressum's postal address carries a postal code and a city inside an
# <address> element, and an electronic mail link exists.
if ! tr '\n' ' ' < "$impressum" | sed -n 's:.*<address>\(.*\)</address>.*:\1:p' \
  | grep -Eq '[0-9]{5}[[:space:]]+[A-ZÄÖÜ][[:alpha:]äöüß]'; then
  fail "impressum.html carries no postal address with a postal code and a city"
fi
grep -Eq 'href="mailto:[^"]+"' "$impressum" \
  || fail "the Impressum offers no electronic mail address"

require "$privacy" "$PRIV_CONTROLLER" "controller section"
require "$privacy" "$PRIV_HOST" "hosting provider"
require "$privacy" "$PRIV_IP" "IP-address disclosure"
require "$privacy" "$PRIV_PURPOSE" "purpose of the logging"
require "$privacy" "$PRIV_FURTHER_DATA" "further processed data"
require "$privacy" "$PRIV_BASIS" "legal basis"
require "$privacy" "$PRIV_TRANSFER" "third-country transfer"
require "$privacy" "$PRIV_SCC" "standard contractual clauses"
require "$privacy" "$PRIV_SUBPROCESSORS" "sub-processors"
require "$privacy" "$PRIV_RETENTION" "retention statement"
require "$privacy" "$PRIV_LOCAL_PROCESSING" "browser-local conversion"
require "$privacy" "$PRIV_COOKIES" "no-cookie statement"
require "$privacy" "$PRIV_RIGHTS" "rights and supervisory authority"
require "$privacy" "$PRIV_MAIL_PROVIDER" "mail handling"
grep -Eq "Stand: [0-9]{2}\.[0-9]{2}\.[0-9]{4}" "$privacy" \
  || fail "privacy.html carries no standing date"

# The two carriers outside the Site output link the notices and copy neither the
# postal address nor the mail address: one source, everywhere else a link.
for carrier in "$CARRIER_PROFILE" "$CARRIER_README"; do
  path="$root/$carrier"
  [ -s "$path" ] || fail "the carrier $carrier does not exist"
  require "$path" "$NOTICE_LINK_IMPRESSUM" "labelled Impressum link"
  require "$path" "$NOTICE_LINK_PRIVACY" "labelled Datenschutz link"
  if grep -Eq 'mailto:' "$path"; then
    fail "$carrier copies a mail address instead of linking the notices"
  fi
  if grep -Eq '[0-9]{5}[[:space:]]+[A-ZÄÖÜ][[:alpha:]äöüß]' "$path"; then
    fail "$carrier copies a postal address instead of linking the notices"
  fi
done

echo "legal-notices-check: both notices are complete, linked from every page, and their carriers link them"
