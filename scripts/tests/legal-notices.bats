#!/usr/bin/env bats
# SPDX-License-Identifier: MPL-2.0
# The executable evidence for the two legal-notice checks: one clean run over an
# assembled output and one mutated page set per refusal family. The fixture is
# assembled from the repository's own members, never rebuilt, so a refusal is
# attributable to the mutation alone.
#
# `scripts/check-site-content.sh` reads the assembled output;
# `scripts/legal-notices-check.sh` reads the source tree. Both are exercised
# here: the output gate through a fixture output directory, the source check
# through a fixture tree whose profile and README are the repository's own.

setup() {
  REPO="$(cd "$BATS_TEST_DIRNAME/../.." && pwd)"
  TMP="$(mktemp -d)"
  OUT="$TMP/out"
  SRC="$TMP/src"
  mkdir -p "$OUT/pkg" "$SRC/web" "$SRC/org/profile"
  for member in index.html impressum.html privacy.html app.js capability.js styles.css; do
    cp "$REPO/web/$member" "$OUT/$member"
  done
  cp "$REPO/docs/assets/duty2cal.svg" "$OUT/icon.svg"
  cp "$REPO/docs/assets/duty2cal.png" "$OUT/icon.png"
  cp "$REPO/LICENSES/MPL-2.0.txt" "$OUT/LICENSE.txt"
  cp "$REPO/web/pkg/duty2cal_wasm.js" "$OUT/pkg/"
  cp "$REPO/web/pkg/wasm-inline.js" "$OUT/pkg/"
  # The assembler stamps these two anchors; the fixture stamps them the same way
  # so the gate sees the same shape it sees in a real publication.
  sed -i 's|__BUILT_REVISION__|v9.9.9-1-gabcdef1|g; s|__BUILT_COMMIT__|abcdef1234567890abcdef1234567890abcdef12|g' \
    "$OUT/index.html"

  # The source tree the cross-carrier check reads.
  cp "$REPO/web/index.html" "$REPO/web/impressum.html" "$REPO/web/privacy.html" "$SRC/web/"
  cp "$REPO/org/profile/README.md" "$SRC/org/profile/"
  cp "$REPO/README.md" "$SRC/"
}

teardown() {
  rm -rf "$TMP"
}

check_output() {
  run bash "$REPO/scripts/check-site-content.sh" "$OUT"
}

check_source() {
  run bash "$REPO/scripts/legal-notices-check.sh" --root "$SRC"
}

@test "the assembled page set passes both checks" {
  check_output
  [ "$status" -eq 0 ]
  [[ "$output" == *"the page set carries the required statements, notices and assets"* ]]
  check_source
  [ "$status" -eq 0 ]
}

@test "an output that lost the Datenschutzerklärung is refused and the page is named" {
  rm "$OUT/privacy.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"privacy.html"* ]]
}

@test "an output with an extra member is refused" {
  printf 'stray\n' > "$OUT/extra.txt"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"extra.txt"* ]]
}

@test "a page set whose converter footer lost the Impressum link is refused" {
  sed -i 's|<a href="./impressum.html">Impressum</a> · ||' "$OUT/index.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"Impressum"* ]]
}

@test "a notice that does not link the other notice is refused" {
  sed -i 's|<a href="./impressum.html">Impressum</a>||' "$OUT/privacy.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"Datenschutzerklärung links no Impressum"* ]]
}

@test "an Impressum that still carries a placeholder is refused and the marker is named" {
  sed -i 's|Daniel Kressner|TODO|' "$OUT/impressum.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"TODO"* ]]
}

@test "an Impressum without a postal address is refused" {
  sed -i 's|10969 Berlin||' "$OUT/impressum.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"postal address"* ]]
}

@test "an Impressum that offers no mail link is refused" {
  sed -i 's|<a href="mailto:duty2cal662.abridge974@aleeas.com">duty2cal662.abridge974@<wbr>aleeas.com</a>|duty2cal662.abridge974@aleeas.com|' "$OUT/impressum.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"electronic mail address"* ]]
}

@test "a Datenschutzerklärung without a standing date is refused" {
  sed -i 's|Stand: 27.10.2026||' "$OUT/privacy.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"standing date"* ]]
}

@test "a Datenschutzerklärung without the English summary is refused" {
  sed -i 's|The German version is the legally binding one.||' "$OUT/privacy.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"binding-version"* ]]
}

@test "a notice that carries the discontinued dispute-resolution reference is refused" {
  sed -i 's|https://github.com/duty2cal/duty2cal|https://ec.europa.eu/consumers/odr|' "$OUT/impressum.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"ec.europa.eu/consumers/odr"* ]]
}

@test "a notice that loads an asset from another origin is refused and the asset is named" {
  sed -i 's|<link rel="stylesheet" href="./styles.css">|<link rel="stylesheet" href="https://example.invalid/x.css">|' "$OUT/privacy.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"example.invalid"* ]]
}

@test "a notice with an inline event handler is refused" {
  sed -i 's|<body class="notice-page">|<body class="notice-page" onload="alert(1)">|' "$OUT/impressum.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"inline event handler"* ]]
}

@test "a notice that lost its content security policy is refused" {
  sed -i 's|<meta http-equiv="Content-Security-Policy" content="[^"]*">||' "$OUT/privacy.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"content security policy"* ]]
}

@test "a relative link to a page the output does not hold is refused" {
  sed -i 's|</main>|<p><a href="./datenschutz.html">Datenschutz</a></p></main>|' "$OUT/privacy.html"
  check_output
  [ "$status" -eq 1 ]
  [[ "$output" == *"datenschutz.html"* ]]
}

@test "a source page set that lost the Datenschutzerklärung is refused and the page is named" {
  rm "$SRC/web/privacy.html"
  check_source
  [ "$status" -eq 1 ]
  [[ "$output" == *"privacy.html"* ]]
}

@test "a profile that copies the postal address is refused and the carrier is named" {
  printf '\nAnschrift: Prinzenstraße 34, 10969 Berlin\n' >> "$SRC/org/profile/README.md"
  check_source
  [ "$status" -eq 1 ]
  [[ "$output" == *"org/profile/README.md"* ]]
}

@test "a profile that copies the mail address is refused and the carrier is named" {
  printf '\nKontakt: <a href="mailto:duty2cal662.abridge974@aleeas.com">Mail</a>\n' >> "$SRC/org/profile/README.md"
  check_source
  [ "$status" -eq 1 ]
  [[ "$output" == *"copies a mail address"* ]]
}

@test "a README without the labelled Datenschutz link is refused" {
  sed -i 's|\[Datenschutz\](https://duty2cal.github.io/privacy)||' "$SRC/README.md"
  check_source
  [ "$status" -eq 1 ]
  [[ "$output" == *"Datenschutz"* ]]
}
