# Design review: add-vector-brand-assets

Fresh-context grilling of the complete planning layer (proposal, delta specification, design, `arc42.md` assessment, all twelve arc42 chapters, the glossary, the page-content check, the assembler, the publication scripts and the browser driver), followed by a hardening pass. The grilling ran in a separate subagent that had not written the design; every claim below is grounded in a file read or a command the grilling subagent ran, and measurements it reproduced are noted as reproduced.

The change's boundaries were confirmed by the operator before hardening: the README mark remains a source-documentation edit verified by reading (no capability scenario, no check), the change adds no new action icon, and no browser assertion is added for the tab icon.

## Grilling — facts re-tested rather than accepted

| Claim in the plan | Result | Evidence |
|---|---|---|
| Traced master: 4 paths, 9,474 bytes, 1044 viewBox, palette only | **True** | `logo-base-b.svg`; `wc -c` = 9474; paths `#0B1F3F`×2, `#F4F1EA`, `#FF9F1C` |
| Fidelity 1.65% RMSE at 1044 (librsvg 2.62.3 / ImageMagick 7.1.2) | **Reproduced** — 1.655% | `rsvg-convert -w 1044` + `magick compare -metric RMSE` |
| Fidelity 0.32% RMSE at 48 | **Only for a downscaled 1044 render** — a native 48 render measures 3.30% | finding F1 |
| Sample points identical | **True** at both sizes | `magick -format '%[pixel:…]'` |
| New check regexes match the planned markup | **True** — the design's assertions pass against the planned HTML | `/tmp/check.sh` |
| …and reject the intended failures | **Partly false** — flipped order and header-raster are named; a missing/untyped declaration exits silently | finding F2 |
| Assembler defect is real for this delta | **True** — the run emitted `f0002.feature` containing `@givn.delta` while the permanent feature was skipped | `python3 scripts/assemble_features.py --root . --out /tmp/asm-review` |
| Documentation convention is the repo's own | **True** — 8 permanent features skipped with `is @documentation; not executed`; no step binds `publish-public-website` | assembler run, `grep` over `apps/cli/tests` |
| Strict mode | **True** — `apps/cli/tests/features_runner.rs:61` `.fail_on_skipped()` | read |
| Safari fallback rationale | **True** — caniuse `link-icon-svg`: Safari 3.1–18.7 unsupported, 26.0+ supported; Chrome/Edge 80+, Firefox 41+ | caniuse + WebKit Safari 26 post |
| Versions (Playwright 1.64.1, Node 24, wasm-pack 0.15.0) | **True** | `web/e2e/package-lock.json:43`, `.nvmrc`, `web/.wasm-pack-version` |
| Nine members; publication read-back dynamic; `site_inputs` covers `docs/assets/*`; REUSE override covers the new file | **True** | `publish-site.sh:39-45,265-271`; `REUSE.toml:26-29` |
| `givn spec route` — no signal, tie at 14.00 | **Reproduced** | `givn spec route --change add-vector-brand-assets` |
| `givn lint --change` clean | **True** (exit 0, one advisory: >14 steps) | run |

## Grilling — branches walked

| Branch | Verdict | Evidence |
|---|---|---|
| **Scope** — spec matches proposal, nothing extra, "done" unambiguous | ⚠️ **Gap** | The proposal promises four refusals; the delta documented one. Nothing extra: the two tooling edits are justified. |
| **Tech choices** | ⚠️ **Weakest link** | Plain SVG 1.1, no runtime dependency, frozen raster — sound. But the asset contract was enforced by `grep` where `python3` is already a publication dependency; findings F4 and F5. |
| **Missing scenarios** | ❌ **Finding F3** | tab-without-raster, missing member and non-plain `icon.svg` had no scenario. |
| **Testability** | ❌ **Findings F1, F2** | Strict mode named, single-scenario command named, no stub needed (no step). The acceptance bar was unreachable by the required browser measurement and one refusal was silent. |
| **E2E fidelity** (`givn instructions specs`) | ✅ with one correction | One inventory entry → one `@e2e`; no redefinition; real driver Playwright; `@e2e` preserved. Discrepancy: the design claimed an assertion on the first tab link that no step performs (F7). |
| **Interaction Coverage** | ✅ | `usecase.md` §Interactions has exactly one row; the matrix has exactly that row; the refusal is an error case of the same action. |
| **Risk** | ⚠️ | Most likely implementation failure: the content-check shell code (F2) and the fidelity bar (F1). TD-026 records the two-file drift risk correctly. |
| **Use-case & Persona context** | ✅ | Single-outcome goal, primary actor named, capability placement goal-shaped, every Extension references a main-flow step, preconditions consistent; contract unchanged; no rejection criterion applies. |
| **Decision ledger** | ✅ | D1–D6 each resolved in `## Decisions`; D7/D8 recorded as design-local; no seed, no carried question. |
| **Visual design contract** | ✅ | `Interface: web`, both viewports, design system, states, tokens, capture mechanism; the approved direction exists in `docs/design/design-system.md` (amended 2026-10-27), so no new options were required. |
| **Deferrals** | ⚠️ operator question Q3 | The README requirement is excluded from specs and checks by recorded decision; confirmed by the operator in hardening. |
| **Persona disposition** | ✅ | Both confirmed personas carry relevance + evidence; a web contract with ≥1 confirmed persona is satisfied. |
| **Interface classification** | ✅ | Opposite tested: the page renders HTML/CSS/JS, so Web UI; the design declares `web` + Playwright. The CLI interface belongs to the publication action, matching the use-case inventory. |
| **ADR qualification** (`givn instructions arc42-docs`) | ⚠️ **finding F9** | Verdict complete; existing-ADR check verified against ADR-0001…0007; none covers a brand asset. The routing named two canonical artifacts. |
| **arc42** | ✅ rows agree; ⚠️ two wording defects | Independent derivation below; all Yes chapters substantive; no ASCII art; findings F9, F10. |
| **Ubiquitous language** | ❌ **Finding F8** | `Brand mark` was coined in the glossary and used nowhere else. |

## Grilling — findings and dispositions

### F1 — The 48-pixel fidelity claim was not the fidelity of the delivered icon (blocking)

*Priority 1; `design.md` fidelity section and D3.*

The recorded 0.32% at 48×48 is not the fidelity of the delivered 48-pixel icon. The grilling reproduced 1.655% at 1044, and 0.324% only when the **1044 render is downscaled** and compared with the PNG downscaled the same way. Rendering the vector at its actual 48-pixel box — what the page does and what the required Chromium re-measure does — measures **3.30%** against the same 48-pixel raster (3.12% with a Lanczos pre-resize), three times the 1.0% bar. The design also said the bar "applies to both renderers", so an honest browser measurement either failed D3 or silently picked the favourable pipeline. Because `magick`'s SVG delegate is librsvg (`RSVG 2.62.3`), the two-line recipe would also have silently re-measured librsvg, and "at 48 pixels the downscale hides it" was wrong for a native render.

**Disposition — applied.** The comparison pipeline is now fixed and each bar belongs to one pipeline:

- RGBA RMSE, not RGB; with alpha discarded the transparent corners make an RGB-only comparison meaningless (26.4% at 1044).
- Rasterise at 1044 with `rsvg-convert -w 1044 -h 1044`, compare with `docs/assets/duty2cal.png`: **1.65%, bar ≤ 2.0%**.
- Downscale both the 1044 render and the PNG with `magick -resize 48x48`, compare: **0.32%, bar ≤ 1.0%**.
- Browser-native 48-pixel render (Chromium, DPR 1) against the 48-pixel raster: **3.30% measured (3.12% with a Lanczos pre-resize), separate bar ≤ 4.0%**; a native render is never compared against the 1.0% bar.
- The exact commands are recorded with `rsvg-convert`/`magick`, and the implementation re-measures the 1044, downscaled-48 and browser-native-48 numbers in Chromium and records all three.

`design.md` ("The vector master", D3) and `proposal.md` D3 now carry this pipeline and these bars.

### F2 — The new line-number assignments exited silently under `set -euo pipefail` (blocking)

*Priority 1; `design.md` check snippet.*

`check-site-content.sh` runs under `set -euo pipefail` (`scripts/check-site-content.sh:15`). `svg_line="$(grep … | head -n 1 | cut -d: -f1)"` is a failing simple command when `grep` matches nothing, so the shell exited **before** the following `[ -n "$svg_line" ] || fail "…"` could run. Reproduced: the design's snippet against a page whose vector declaration was missing or untyped exited 1 with **no output**, so the design's claim that such a page "is refused by name" was false. The page was still refused, so the gate was not weakened — the failure was un-named, which breaks the scenario's "names the page-content check and …" contract. The file already guards every equivalent pipeline the same way (`statement_count`, `foreign_asset_patterns`).

**Disposition — applied.** Both assignments now end in `|| true`, so a no-match pipeline cannot exit before the named `fail`:

```sh
svg_line="$(grep -nE '<link[^>]+rel="icon"[^>]+type="image/svg[+]xml"[^>]+href="\./icon\.svg"' "$index" | head -n 1 | cut -d: -f1 || true)"
[ -n "$svg_line" ] \
  || fail "the page's tab declares no vector icon at ./icon.svg with type=\"image/svg+xml\""
png_line="$(grep -nE '<link[^>]+rel="icon"[^>]+type="image/png"[^>]+href="\./icon\.png"' "$index" | head -n 1 | cut -d: -f1 || true)"
[ -n "$png_line" ] \
  || fail "the page's tab declares no raster alternate at ./icon.png with type=\"image/png\""
```

The design's claim is corrected rather than dropped: a page whose declaration is absent or untyped now reaches the named refusal, and the text states that the guard is part of the assertion (without it the page is refused but un-named). Verified in a `set -euo pipefail` harness: the guarded miss prints the named message and exits 1.

### F3 — Three of the four promised refusals had no scenario (high)

*Priority 2; delta feature.*

The proposal promises the check refuses "a page whose tab declares no raster alternate, an output missing either icon member, and an `icon.svg` that is not a plain vector of the mark, or a reference off its own bytes"; the delta documented only the header-mark refusal. The permanent feature already carries one refusal scenario per evidence-check failure class, so this is the repo's own convention. The three missing refusals are cheap `@givn.added` documentation scenarios with no binding.

**Disposition — applied.** Added to `specs/publish-public-website/publish-public-website.feature` with the same `@givn.added @site-publication` tags:

- "A page whose tab declares no raster alternate is refused and the page-content check names the tab declaration"
- "An output missing an icon member is refused and the page-content check names the member"
- "A page carrying a vector mark that is not the project's plain palette vector is refused and the file is named"

### F4 — The palette and plainness assertions did not enforce the contract they claimed (high)

*Priority 2; `design.md` plainness block.*

The forbidden list omitted `<clipPath` and `<mask>`, and the palette scan matched only `fill="#…"`-shaped literals, so `fill="navy"`, `fill="rgb(1,2,3)"`, `style="fill:#123456"` and `fill = "#123456"` all passed (verified by running the block). The headline claims "Every colour literal is a palette value" and "a doctored output is refused as well as a doctored source" therefore overstated the gate. `python3` is already a dependency of the publication (`publish-site.sh:283`), so a parser-based check costs nothing new.

**Disposition — applied.** The grep-only block is replaced by one stdlib `xml.etree.ElementTree` check over the assembled `icon.svg` (clean Python, not the report's `__next__` expression). It:

- parses the document, so a malformed file fails there;
- requires the SVG root and `viewBox="0 0 1044 1044"`;
- rejects `image`, `text`, `style`, `filter`, `clipPath`, `mask`, `linearGradient`, `radialGradient` and `use`;
- rejects any `style` attribute;
- rejects any `fill`, `stroke` or `stop-color` outside the palette, case-insensitively, allowing `none`;
- rejects an external `href` (plain or xlink-namespaced) containing `//`.

The shell keeps its `fail` on non-zero and names the file. Verified against seven fixtures: a palette-only document passed; a named colour, a `style` attribute, a `clipPath`, an external `image` href, a truncated path and a wrong viewBox were each refused.

### F5 — Nothing proved the shipped `icon.svg` parses or renders (medium)

*Priority 2; `design.md` structural contract.*

A vector whose path data was malformed (or truncated mid-path, exactly what a botched re-trace produces) satisfied `grep '<svg'`, the viewBox grep, the forbidden-substring loop and the palette scan, then shipped as `icon.svg`; the browser would show alt text in the header and no tab mark. No test asserted the icon loaded: `driver.mjs` emits `iconSrc` and `tabIcon` as observations but no Rust step consumes either, and the reference renderer's `naturalWidth === 1044` covers only the design reference.

**Disposition — applied with a narrower closure.** The XML parse in F4 closes the parse/malformed case. The remaining "well-formed but does not render" case is stated explicitly in `design.md` as covered by the committed browser screenshots at both viewports, reviewed as visual evidence — no new browser assertion and no second renderer gate is added (the operator confirmed this boundary). The design's mitigation is stated where it exists instead of over-claiming.

### F6 — The evidence override bypassed the helper's active→archive resolution (medium)

*Priority 2; `design.md` evidence section and D7.*

`evidence_dir()` resolves the change directory by presence of `proposal.md`, with the explicit reason that the archive gate moves the change before it runs the suite. The proposed `DUTY2CAL_VISUAL_EVIDENCE_DIR` returned a hardcoded path and skipped that resolution: if the variable were still exported when this change archives, `create_dir_all` would recreate `givn/changes/add-vector-brand-assets/evidence/…` outside the archive and write the frozen record past the candidate.

**Disposition — applied.** The override now names a **change id**, defaulting to `build-browser-converter`; the helper resolves `givn/changes/<id>` versus `givn/archive/<id>` by `proposal.md` presence exactly as it does today, then appends the scenario evidence path. `design.md`'s files table and D7 state the change-id contract and that the resolution is unchanged.

### F7 — The design claimed a browser assertion on the tab icon that no step performs (low)

*Priority 3; `design.md` anticipated obstacles.*

The obstacle claimed an assertion that the first `link[rel="icon"]` is the vector; the ordering is asserted by the content check over the markup, but nothing consumes the driver's `tabIcon`.

**Disposition — applied.** The obstacle now says the content check asserts the markup order, and the browser driver records the served tab declaration's `href` as an observation, not an assertion. No new step is added.

### F8 — The glossary coined a term nothing else used (low)

*Priority 3; `docs/arc42/12-glossary.md`.*

`Brand mark` appeared in the glossary and `arc42.md` only, nowhere in the proposal, design, delta spec or design system, while "vector mark"/"raster mark"/"raster alternate" were used freely.

**Disposition — applied.** The glossary term is kept as the umbrella whose two forms are the **vector master** and the **raster alternate**. "Brand mark" is now used once in the design scope and once in the proposal, so the term is not glossary-only. No other term is renamed.

### F9 — The non-qualifying candidate routed to two artifacts, and chapter 9 repeated the argument (low)

*Priority 3; `arc42.md`; chapter 9.*

`CANONICAL_ARTIFACT` requires exactly one destination, but the prose added "together with the durable `docs/design/design-system.md`". Chapter 9 restated the three options and the per-dimension reasons that `design.md` D1–D3 already own, and conflated a dimension with a kind-classification falsification test.

**Disposition — applied.** `arc42.md` routes the candidate to exactly one canonical artifact, `design.md`; the design system is named only as supporting context. Chapter 9 is trimmed to the register-level note: the member count (nine, including the vector mark and its raster tab alternate) plus the one-line `NOT_QUALIFIED` routing. The per-dimension argument lives in `design.md` and `arc42.md`.

### F10 — The chapter-2 documentation rule covered only permanent features (low)

*Priority 3; `docs/arc42/02-architecture-constraints.md`.*

The change introduces the first `@documentation` **delta** (the assembler fix exists for it), while chapter 2 recorded the convention only for feature files under `givn/specs`.

**Disposition — applied.** The rule now says operational feature files stay in `givn/specs` under a feature-level `@documentation` tag, a change's delta for such a capability carries the same tag, the assembler skips both the same way, and their step definitions are deleted.

## Operator questions and dispositions

**Q1 — Which pipeline defines the 48-pixel fidelity number?** Answered 2026-10-27 (operator): pin the downscale pipeline and keep 1.0% for it, and record the browser-native number against its own 4.0% bar. Applied as F1. Rejected alternatives: keep 1.0% for both and accept a failing re-measure; drop the 48-pixel row.

**Q2 — Should the three uncovered refusals become delta scenarios?** Answered 2026-10-27 (operator): yes, as three `@givn.added` documentation scenarios with the exact Given/When/Then text from the grilling, zero bindings, one merge per refusal. Applied as F3. Rejected: one combined "the check refuses a wrong page" scenario, which loses the named-refusal assertion.

**Q3 — Confirm the README deferral.** Answered 2026-10-27 (operator): confirmed. The README mark remains a source-documentation edit stated in the proposal (D4) and verified by reading the file; no scenario is written for it and no check reads `README.md`. The alternative — a check that greps `README.md` line 2 — was rejected as a failure surface for a documentation file. The same decision keeps D5's "no new action icons" unchanged.

## Independent arc42 re-derivation

The grilling derived each row from `proposal.md`, the delta spec and `design.md` before opening `arc42.md`, reasoning: no new goal/stakeholder (1 No); a durable, enforced asset-format and fallback constraint (2 Yes); no new external system or surface (3 No); the icon strategy is recorded in the published-site technology and the gated-command decision (4 Yes); the member set and the build/check blocks change (5 Yes); the publication flow, order and participants are unchanged (6 No); the deployed artefact content changes (7 Yes); the local-image rule and its refusals are cross-cutting (8 Yes); the register's member count and the candidate's routing change (9 Yes); no quality scenario is added and the fidelity bound is a design-level measurement (10 No); TD-026 (11 Yes); the mark's vocabulary is new (12 Yes).

| # | Chapter | Mine | Marker | Verdict |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No | agree |
| 2 | 02 architecture-constraints | Yes | Yes | agree (F10 wording) |
| 3 | 03 context-and-scope | No | No | agree |
| 4 | 04 solution-strategy | Yes | Yes | agree |
| 5 | 05 building-block-view | Yes | Yes | agree |
| 6 | 06 runtime-view | No | No | agree |
| 7 | 07 deployment-view | Yes | Yes | agree |
| 8 | 08 crosscutting-concepts | Yes | Yes | agree |
| 9 | 09 architecture-decisions | Yes | Yes | agree (F9) |
| 10 | 10 quality-requirements | No | No | agree |
| 11 | 11 risks-and-technical-debt | Yes | Yes | agree (TD-026 present, numbered after TD-025) |
| 12 | 12 glossary | Yes | Yes | agree (F8) |

No row where the grilling said Yes and the marker said No; no omitted row; every row carries a reason.

**Chapter content check (all 12 files):** all twelve exist with substantive, change-specific text — no scaffold placeholders (`grep TODO/TBD/Lorem` empty; only legitimate uses of "placeholder" about GitHub's stock profile page). Diagrams are Mermaid (05 and 07 flowcharts); no box-drawing characters in any file and no pseudo-diagram blocks. The literal `|`+`---` scan matches markdown table separator rows in twelve files — the scan's predictable false positive, not ASCII art. Edited chapters (02, 04, 05, 07, 08, 09, 11, 12) match `design.md`: nine members, `./icon.svg` header, two typed declarations, both members, TD-026, and the glossary's two updated rows.

## Hardening applied

| Finding | Files changed | What changed |
|---|---|---|
| F1 | `design.md` ("The vector master", D3), `proposal.md` D3 | Fixed RGBA pipeline, three commands, bars 2.0% / 1.0% / 4.0%, all three re-measured in Chromium and recorded |
| F2 | `design.md` check snippet and its prose | `|| true` on both line-number assignments; claim corrected and the guard's role stated |
| F3 | `specs/publish-public-website/publish-public-website.feature` | Three `@givn.added @site-publication` documentation scenarios |
| F4 | `design.md` plainness block, vector-format decision, check prose | Stdlib XML parse replacing the grep-only block |
| F5 | `design.md` check prose | Malformed path caught by the parse; non-rendering mark covered by reviewed committed screenshots; no new assertion or renderer gate |
| F6 | `design.md` evidence section, files table, D7 | Override names a change id; active/archived resolution preserved |
| F7 | `design.md` anticipated obstacles | Content check asserts markup order; driver observes the tab href |
| F8 | `docs/arc42/12-glossary.md`, `design.md` scope, `proposal.md` | Umbrella definition; one use in the design and one in the proposal |
| F9 | `givn/changes/add-vector-brand-assets/arc42.md`, `docs/arc42/09-architecture-decisions.md` | One canonical artifact; chapter 9 trimmed to the register note |
| F10 | `docs/arc42/02-architecture-constraints.md` | Delta half of the `@documentation` rule |

Verification of the hardened artifacts, run in this session:

- The Python XML check was executed against seven fixtures — palette-only PASS; `navy`, `style`, `clipPath`, external `href`, truncated path and wrong viewBox each REFUSED.
- The `|| true` guard was executed in a `set -euo pipefail` harness against a page missing the raster declaration — the named refusal printed and the shell exited 1.
- `givn lint --change add-vector-brand-assets` — exit 0, one advisory (the modified publication scenario has more than 14 steps, as before this review).
- `givn status --change add-vector-brand-assets` — stage `design-review-done`; proposal, specs, design, arc42-docs and design-review artifacts present; `tasks` next.

## Verdict

All ten findings are resolved, Q1–Q3 are answered and their decisions applied, every required grilling branch was walked, and the arc42 chapter set agrees with an independent twelve-row derivation. No question remains open, no deferral is unrecorded, and no hardening edit is outstanding.

DESIGN-REVIEW: PASS
