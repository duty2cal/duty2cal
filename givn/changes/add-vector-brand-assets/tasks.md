# Tasks: add-vector-brand-assets

Release note (commit subject on every scenario revision):

> feat(publish-public-website): The public website and the project's README show the duty2cal mark as a scalable vector icon, and the site's tab declares that vector icon with the existing PNG as its fallback for browsers that cannot use it.

Use-case contract: `givn/specs/publish-public-website/usecase.md` — "Make the
Public website at `https://duty2cal.github.io/` serve the Site output built from
a named Built revision." The capability `publish-public-website` owns the page
header, the tab icon and the Site output's member set. Guarantees: the Site
output is the page, the script and the WASM bundle plus the licence file and the
icon; the page shows the project icon at 48×48 left of the heading and as its tab
icon; a failed evidence check leaves the previous page and names the failure.
Personas: `project-maintainer--release-owner` (release and built-output lens),
`contributor--web-reviewer` (asset provenance, CSP, built output). Their
biographies are not Gherkin actors.

## Bundle context — applies to every scenario

- **Runner:** `./run-tests.sh` (verify.command) and
  `./run-tests.sh --e2e` (verify.e2e_command, a strict subset). Strict mode is
  `.fail_on_skipped()` in `apps/cli/tests/features_runner.rs`. The delta feature
  is `@documentation` and is not assembled once D8 lands, so no Cucumber step
  binds these scenarios; the executable gate for this capability is
  `scripts/check-site-content.sh`, run by `scripts/site-evidence.sh`.
- **Single-scenario command:** none for the `@documentation` scenarios; the
  assembler prints `... is @documentation; not executed`. The browser evidence is
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
  assignments end in `|| true`; the plainness/palette check is a Python
  `xml.etree.ElementTree` parse of the assembled `icon.svg`.
- **Licensing:** `docs/assets/**` is already CC0-1.0 through `REUSE.toml`; no
  edit.
- **Evidence:** the `@browser` scenario writes
  `givn/changes/add-vector-brand-assets/evidence/visual/crew-member-converts-a-supported-duty-plan-in-the-browser-and-saves-the-offered-calendar/{mobile,desktop,mobile--empty,desktop--empty}.png`
  through `DUTY2CAL_VISUAL_EVIDENCE_DIR=add-vector-brand-assets` and
  `DUTY2CAL_UPDATE_VISUAL_EVIDENCE=1`.

## 0. Setup — assembler rule and evidence capture

- [ ] **D8 — the assembler skips a `@documentation` delta.** In
      `scripts/assemble_features.py`'s delta-emission loop, call the existing
      `is_documentation(lines)` and `continue` with the same
      `assemble_features: <path> is @documentation; not executed` note used for
      permanent features. Proof: `scripts/assemble-features.sh` no longer emits
      `@givn.delta` for this change, and its log names the delta as not executed.
      No commit alone; it rides the first scenario revision.
- [ ] **D7 — the capture names a change id.** In
      `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`, `evidence_dir()`
      reads `DUTY2CAL_VISUAL_EVIDENCE_DIR` (default `build-browser-converter`),
      resolves `givn/changes/<id>` vs `givn/archive/<id>` by `proposal.md`
      presence, and appends the scenario's evidence path. No commit alone.
- [ ] **Baseline.** `./run-tests.sh` exits 0 after D8 and before the asset work;
      paste the summary.

## 1. The publication scenario — the vector mark reaches the published page

Design constraints: the vector master is the header mark and the tab's first
icon; the raster is the tab alternate; the Site output holds both; the content
check requires the header at `./icon.svg` with `alt="duty2cal"` and 48×48, and
the two typed tab declarations in that order; the README uses the vector.

- [ ] RED — build the Site output from the unmodified page and run
      `scripts/check-site-content.sh`: it passes while the header and tab still
      name `./icon.png`, so the new assertions are absent. Paste the exit status.
- [ ] GREEN — add `docs/assets/duty2cal.svg`; change `web/index.html`,
      `scripts/build-site.sh`, `scripts/check-site-content.sh`, and `README.md`.
      Rebuild the output; `scripts/check-site-content.sh` exits 0 and the output
      holds nine members. Paste the command output and the member list.
- [ ] REFACTOR — remove duplicated icon source variables or dead comments in
      `build-site.sh`; re-run the check. Paste the exit status.
- [ ] COMMIT: `<hash>` — `feat(publish-public-website): <release note>` with the
      scenario title in the body.

## 2. A page whose header mark is not the vector is refused

- [ ] RED — doctor the assembled `index.html` to `src="./icon.png"`; the check
      from task 1 still exits 0 (the refusal is absent). Paste the exit status.
- [ ] GREEN — add the header-raster refusal to `check-site-content.sh`; the same
      doctored output exits non-zero and names the header mark. Paste the named
      failure.
- [ ] REFACTOR — keep the two header greps adjacent and the message in the
      existing fail-by-name idiom; re-run. Paste the exit status.
- [ ] COMMIT: `<hash>`.

## 3. A page whose tab declares no raster alternate is refused

- [ ] RED — remove the raster tab declaration from the assembled page; the check
      still exits 0 (the tab assertions are absent). Paste the exit status.
- [ ] GREEN — add the `svg_line`/`png_line` typed-order assertions with `|| true`;
      the doctored page exits non-zero and names the missing raster alternate.
      Paste the named failure.
- [ ] REFACTOR — re-run the valid output and the doctored output. Paste both.
- [ ] COMMIT: `<hash>`.

## 4. An output missing an icon member is refused

- [ ] RED — remove `icon.svg` from the assembled output; the check still exits 0
      (the member assertion is absent). Paste the exit status.
- [ ] GREEN — add `[ -s "$out/icon.svg" ]`; the same output exits non-zero and
      names the member. Paste the named failure.
- [ ] REFACTOR — re-run the valid output. Paste the exit status.
- [ ] COMMIT: `<hash>`.

## 5. A vector that is not the plain palette mark is refused

- [ ] RED — replace the assembled `icon.svg` with a well-formed document that
      carries `<image>`; the check still exits 0 (the parse is absent). Paste the
      exit status.
- [ ] GREEN — add the stdlib XML plainness/palette parse; the doctored file exits
      non-zero and names the vector. Paste the named failure.
- [ ] REFACTOR — re-run the valid output and a second doctored file (a colour
      outside the palette). Paste both.
- [ ] COMMIT: `<hash>`.

## 6. Closeout — evidence and full verification

- [ ] `./run-tests.sh` exits 0; paste the summary.
- [ ] `./run-tests.sh --e2e` exits 0; paste the summary and the selected count.
- [ ] Browser evidence captured at both viewports (or the local renderer limit
      stated with the content check as the executed proof).
- [ ] `reuse lint` exits 0 with the new asset.
- [ ] `givn lint --change add-vector-brand-assets` exits 0 (advisories allowed).
- [ ] Every scenario above records exactly one COMMIT hash naming its scenario.
