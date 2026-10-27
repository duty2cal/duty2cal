# Tasks: add-vector-brand-assets

Release note (commit subject on every scenario revision):

> feat(publish-public-website): The public website and the project's README show the duty2cal mark as a scalable vector icon, and the site's tab declares that vector icon with the existing PNG as its fallback for browsers that cannot use it.

Use-case contract: `givn/specs/publish-public-website/usecase.md` — "Make the
Public website at `https://duty2cal.github.io/` serve the Site output built from
a named Built revision." The capability `publish-public-website` owns the page
header, the tab icon and the Site output's member set. Guarantees: the Site
output carries the page, the script, the WASM bundle, the licence file and the
icon; the page shows the project icon at 48×48 left of the heading and as its tab
icon; a failed evidence check leaves the previous page and names the failure.
Personas: `project-maintainer--release-owner` (release and built-output lens),
`contributor--web-reviewer` (asset provenance, CSP, built output). Their
biographies are not Gherkin actors.

## Bundle context — applies to every scenario

- **Runner:** `./run-tests.sh` and `./run-tests.sh --e2e` (a strict subset).
  Strict mode is `.fail_on_skipped()` in `apps/cli/tests/features_runner.rs`.
  The delta feature is `@documentation` and is not assembled, so no Cucumber
  step binds these scenarios; the executable gate is
  `scripts/check-site-content.sh`, run by `scripts/site-evidence.sh`.
- **Single-scenario command:** none for the `@documentation` scenarios; the
  assembler prints `... is @documentation; not executed`. Browser evidence is
  `./run-tests.sh --browser --name "Crew member converts a supported duty plan in the browser and saves the offered calendar"`.
- **Asset:** `docs/assets/duty2cal.svg`, 4 paths, 9,474 bytes,
  `viewBox="0 0 1044 1044"`, fills only `#0B1F3F`, `#F4F1EA`, `#FF9F1C`.
  The raster `docs/assets/duty2cal.png` stays byte-for-byte.
- **Site output:** nine members. `icon.svg` is the header mark and the tab's
  primary icon; `icon.png` is the raster alternate. Both are same-origin.
- **Page markup:** `<link rel="icon" type="image/svg+xml" href="./icon.svg">`
  first, `<link rel="icon" type="image/png" href="./icon.png">` second;
  `<img class="app-icon" src="./icon.svg" alt="duty2cal" width="48" height="48">`.
- **Content check:** fail-by-name; `set -euo pipefail`; `svg_line`/`png_line`
  assignments end in `|| true`; plainness/palette is a Python
  `xml.etree.ElementTree` parse of the assembled `icon.svg`.
- **Licensing:** `docs/assets/**` is already CC0-1.0 through `REUSE.toml`.
- **Evidence:** `givn/changes/add-vector-brand-assets/evidence/visual/crew-member-converts-a-supported-duty-plan-in-the-browser-and-saves-the-offered-calendar/`
  and `evidence/fidelity.md`.

## 0. Setup — assembler rule and evidence capture

- [x] **D8 — the assembler skips a `@documentation` delta.**
      `scripts/assemble_features.py` now calls `is_documentation(lines)` in the
      delta-emission loop and prints the same note as for a permanent feature.

      ```text
      $ scripts/assemble-features.sh 2>/tmp/asm.log
      no delta feature in the corpus
      $ grep add-vector-brand-assets /tmp/asm.log
      assemble_features: /home/dan/dev/PRIVATE/duty2cal/givn/changes/add-vector-brand-assets/specs/publish-public-website/publish-public-website.feature is @documentation; not executed
      ```
- [x] **D7 — the capture names a change id.**
      `evidence_dir()` reads `DUTY2CAL_VISUAL_EVIDENCE_DIR` (default
      `build-browser-converter`), resolves `givn/changes/<id>` vs
      `givn/archive/<id>` by `proposal.md` presence, and appends the scenario
      path. Exercised by the browser run: four screenshots landed under this
      change's evidence directory and the archived record was untouched. Rides
      `eef8497`.
- [x] **Baseline.** `./run-tests.sh` exits 0 before the asset work.

      ```text
      [Summary]
      1 feature
      84 scenarios (84 passed)
      303 steps (303 passed)
      ```

## 1. The publication scenario — the vector mark reaches the published page

Design constraints: the vector master is the header mark and the tab's first
icon; the raster is the tab alternate; the Site output holds both; the check
requires the header at `./icon.svg` with `alt="duty2cal"` and 48×48; the README
uses the vector.

- [x] RED — the strengthened check against the unmodified page refuses it.

      ```text
      $ scripts/check-site-content.sh /tmp/site-red1
      check-site-content: page content check failed: the page's tab declares no vector icon at ./icon.svg with type="image/svg+xml"
      exit=1
      ```
- [x] GREEN — add `docs/assets/duty2cal.svg`; change `web/index.html`,
      `scripts/build-site.sh`, `scripts/check-site-content.sh`, `README.md`.

      ```text
      $ scripts/build-site.sh --repo . --revision HEAD --out /tmp/site-green1
      build-site: assembled the Site output for v0.4.0-147-g9a1dcda (...) into /tmp/site-green1
      $ scripts/check-site-content.sh /tmp/site-green1
      check-site-content: the page carries the required statements and assets
      check=0
      $ find /tmp/site-green1 -type f | wc -l
      9
      ```
- [x] REFACTOR — no further change; the check still exits 0.
- [x] COMMIT: `eef8497ea7409349aad6ff7c2e8094794b02032f` — release-note subject,
      scenario title in the body.

## 2. A page whose header mark is not the vector is refused

- [x] RED — without the explicit refusal, a doctored raster header is refused
      only by the generic positive assertion; the named raster refusal is
      absent.

      ```text
      $ /tmp/check-pre2.sh /tmp/site-red2
      check-site-content: page content check failed: the page's header mark does not point at ./icon.svg
      exit=1
      ```
- [x] GREEN — the explicit refusal runs first and names the substitution.

      ```text
      $ scripts/check-site-content.sh /tmp/site-red2
      check-site-content: page content check failed: the page's header mark is the raster; the vector mark is required
      exit=1
      $ scripts/check-site-content.sh /tmp/site-green1
      check-site-content: the page carries the required statements and assets
      ```
- [x] REFACTOR — the two header greps stay adjacent; valid output still exits 0.
- [x] COMMIT: `ee27947f0fb0ae508a571f54e22c901b6697155e`.

## 3. A page whose tab declares no raster alternate is refused

- [x] RED — without the alternate assertions, a page with only the vector
      declaration passes.

      ```text
      $ /tmp/check-pre3.sh /tmp/site-red3
      check-site-content: the page carries the required statements and assets
      exit=0
      ```
- [x] GREEN — the typed order assertions refuse the missing alternate and the
      reversed order.

      ```text
      $ scripts/check-site-content.sh /tmp/site-red3
      check-site-content: page content check failed: the page's tab declares no raster alternate at ./icon.png with type="image/png"
      $ scripts/check-site-content.sh /tmp/site-rev3
      check-site-content: page content check failed: the page declares the raster tab icon (line 10) before the vector (line 11); the vector must be the tab's first declaration
      ```
- [x] REFACTOR — valid output still exits 0.
- [x] COMMIT: `be598de05334df5e5cc93ab44b2864c1ff275376`.

## 4. An output missing an icon member is refused

- [x] RED — removing `icon.svg` from the output passed before the assertion.

      ```text
      $ scripts/check-site-content.sh /tmp/site-red4
      check-site-content: the page carries the required statements and assets
      exit=0
      ```
- [x] GREEN — the member assertion refuses it.

      ```text
      $ scripts/check-site-content.sh /tmp/site-red4
      check-site-content: page content check failed: the output holds no icon.svg
      exit=1
      ```
- [x] REFACTOR — valid output still exits 0.
- [x] COMMIT: `480864498f6cf746bdcb0834692e9053d26b0410`.

## 5. A vector that is not the plain palette mark is refused

- [x] RED — a well-formed SVG carrying `<image>` passed before the parse.

      ```text
      $ scripts/check-site-content.sh /tmp/site-red5
      check-site-content: the page carries the required statements and assets
      exit=0
      ```
- [x] GREEN — the XML parse refuses the embedded raster, a stray colour, and a
      truncated document; the valid member passes.

      ```text
      $ scripts/check-site-content.sh /tmp/site-red5
      the document carries <image>
      check-site-content: page content check failed: icon.svg is not a plain palette vector of the mark: ...
      $ scripts/check-site-content.sh /tmp/site-red5b
      fill='#123456' is outside the palette
      $ scripts/check-site-content.sh /tmp/site-red5c
      xml.etree.ElementTree.ParseError: unclosed token: line 3, column 0
      $ scripts/check-site-content.sh /tmp/site-green1
      check-site-content: the page carries the required statements and assets
      ```
- [x] REFACTOR — the parse stays one stdlib block; valid output still exits 0.
- [x] COMMIT: `43b79a8dd2bb81517d241fe327d097c4a820bf93`.

## 6. Closeout — evidence and full verification

- [x] `./run-tests.sh` exits 0:

      ```text
      1 feature
      84 scenarios (84 passed)
      303 steps (303 passed)
      ```
- [x] `./run-tests.sh --e2e` exits 0:

      ```text
      1 feature
      1 scenario (1 passed)
      5 steps (5 passed)
      ```
- [x] Browser evidence captured at both viewports (360×780, 1920×1080) under
      this change's evidence directory; the mobile capture shows the vector mark
      in the header after the browser twin was taught `image/svg+xml`. The
      evidence commit is `a586841d63a1c5ba823228de008bb26f6a6ac106`.
- [x] `reuse lint` exits 0: 539/539 files with copyright and licence
      information, REUSE 3.3 compliant.
- [x] `givn lint --change add-vector-brand-assets` exits 0, one `LONG` advisory
      on the modified publication scenario (the permanent scenario has the same
      shape).
- [x] Fidelity re-measured in Chromium 156.0.8078.47: 1.68% at 1044 (bar ≤2.0%),
      0.33% for the downscaled 48 comparison (bar ≤1.0%), 3.85% for the native
      48-pixel render (bar ≤4.0%). Recorded in `evidence/fidelity.md`.
