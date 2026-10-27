# Proposal: add-vector-brand-assets

## Release note

The public website and the project's README show the duty2cal mark as a scalable vector icon, and the site's tab declares that vector icon with the existing PNG as its fallback for browsers that cannot use it.

## Use-Case Context

- Use-case ID: `publish-public-website`
- Ideation topic: none — this change has no ideation change seed; it extends the permanent use case the `public-website` topic produced (`givn/specs/publish-public-website/usecase.md`).
- Seed path: none
- Confirmed Personas: `project-maintainer--release-owner` (primary lens: the mark the published page carries and the built output that carries it), `contributor--web-reviewer` (judges the asset's provenance, the page's asset closure under its content security policy, and the enlarged Site output)

## Problem / Opportunity

The project's brand mark exists only as one raster file, `docs/assets/duty2cal.png`: 1044×1044 pixels, 42,802 bytes, RGBA, drawn in three flat colours (navy `#0B1F3F`, cream `#F4F1EA`, orange `#FF9F1C`) with transparent rounded corners. Every surface that shows the mark resamples that one bitmap:

- The published page's header renders it at 48×48, so the browser downsamples 1,089,936 pixels into 2,304 and the mark is softer than the artwork allows.
- The browser tab renders it at whatever the tab strip asks for, including high-density displays, where one raster cannot be sharp at every density.
- The repository README renders it at 160 pixels wide, scaling the same oversized bitmap.

There is no vector master. A size change, a density change or a small palette correction is a re-trace of the bitmap, and the tab icon a high-density visitor gets is bounded by the single raster. The opportunity is to keep one mark crisp at every size from a text asset of roughly a fifth of the bitmap's size, on a page that already serves its images from its own origin and already forbids every other origin.

## Proposed Solution

- The project publishes a vector version of the existing mark as `docs/assets/duty2cal.svg`, at the mark's own 1044×1044 scale, drawn only in the three palette colours and in no other colour. The mark is not redesigned.
- A visitor to the public website sees the header mark rendered from `./icon.svg` at 48×48 with the alternative text "duty2cal", left of the page heading and its subheadline, exactly where the raster mark sits today.
- The page's tab declares the vector mark as its icon and the existing raster as the alternate, so a browser that cannot use an SVG tab icon still shows the project's mark and no other.
- The Site output carries both the vector mark as `icon.svg` and the raster as `icon.png`: nine members instead of eight. Both are served from the page's own origin, and the page still requests nothing from any other origin.
- The page-content check refuses a page whose header mark is not the project's vector, a page whose tab declares no raster alternate, an output missing either icon member, and an `icon.svg` that is not a plain vector of the mark (an embedded raster, or a reference off its own bytes).
- A reader of the repository's README sees the same mark from the vector file at 160 pixels wide.
- The publication's start decision is unchanged: the asset directory `docs/assets/**` is already one of the site's inputs, so a revision that changes the vector starts a Site publication exactly as one that changes the raster does.

The raster is kept byte-for-byte: it remains the required fallback for a browser that cannot use an SVG tab icon, and it remains the project's existing non-code asset under CC0-1.0. Nothing about the licence statement, the page's statements, its states or its layout changes.

## Capability Routing

`givn spec route --change add-vector-brand-assets` reported **no signal** — the top candidates tied, at 13.00 before the delta was written and at 14.00 when re-run afterwards (the authored delta moved the scores, not the outcome) — so no recommendation column is recorded; the decision below is the recorded routing.

| Proposed capability | Decision | Rationale |
|---|---|---|
| publish-public-website | EXTEND publish-public-website | The owning use case already states both requirements: "The page shows the project icon at 48×48 to the left of its heading and as its tab icon" and "The site output carries the icon, copied from the project's own non-code asset under CC0-1.0". The page header mark, its tab icon and the Site output's icon members belong to this capability; no other capability owns them. |

The repository README's logo is source content owned by no capability. No capability or use case owns the README, this change does not invent one, and no scenario is written for it: the README line changes as repository documentation in the same implementation pass, and its requirement — the same mark, at 160 pixels wide, now from the vector — is stated here and verified by reading the file.

## Out of Scope

- New user-interface action icons. The design system defines no action icon and adding one would be new design work, not a vector form of an existing asset.
- Organization-profile artwork: the profile content is applied by a separate configuration change, not by this one.
- Icons for the release archive, social-preview images, or any surface that shows no mark today.
- Dropping the raster, regenerating it from the vector, or changing its bytes.
- Redesigning the mark, changing its palette, proportions or transparent corners.
- Changing the design tokens, the layout, the copy, the page states, or the content security policy.
- Any licensing change: `docs/assets/**` is already CC0-1.0 through `REUSE.toml`, so the new vector file inherits the same licence with no edit there.
- Any change to the publication's trigger path list, its evidence order, the hosting target, the workflows or the release coupling.
- A new capability, a new use case, or an executable scenario for the README mark.

## Open Questions

### D1 — Does the mark get a vector master, and what happens to the raster?

- Question: should the project adopt a vector master for its existing mark, keep the raster alone, or replace the raster with the vector outright?
- Options:
  1. **Vector master beside the raster** — add `docs/assets/duty2cal.svg` as the mark used wherever the mark is displayed at user-interface sizes, and keep `docs/assets/duty2cal.png` as the tab-icon fallback and the existing raster asset; gain: one mark crisp at every display size, a text asset that can be corrected without a re-trace, and no visitor loses the mark; cost: two files must stay one mark, so a future mark change touches both; in practice: the header renders `./icon.svg` while the tab declares `./icon.svg` first and `./icon.png` second.
  2. **Raster only** — leave the mark as it is; gain: nothing new to maintain; cost: the 48×48 header and high-density tab icons stay resampled bitmaps, and a size change stays a re-trace; in practice: the tab on a high-density display shows the same soft mark.
  3. **Vector only** — delete the raster and serve the SVG everywhere; gain: one file; cost: a browser that cannot use an SVG tab icon shows no icon at all, and the existing CC0-1.0 raster asset disappears from the repository; in practice: an older browser's tab is blank where it showed the mark.
- Disposition: answered — option 1. The owning use case already requires the project icon in the header and as the tab icon, the design system names the raster as that icon, and the tab's fallback requirement rules out option 3.

### D2 — How does the tab declare its icons?

- Question: should the page's tab icon be the vector alone, the vector with the raster as an alternate, or the raster with the vector as an alternate?
- Options:
  1. **Vector first, raster alternate** — two `<link rel="icon">` declarations, the SVG typed as `image/svg+xml` first and the PNG typed as `image/png` second; gain: every browser that understands SVG takes the sharp vector, a browser that does not falls back to the mark it already had, and the existing browser evidence that reads the first tab icon needs no change of meaning; cost: two declarations to keep correct, which the page-content check enforces; in practice: a current Chrome tab is sharp at any density, while Safari up to 18.7 — which cannot use an SVG tab icon — keeps the raster mark it shows today.
  2. **Vector only** — one declaration; gain: the simplest markup; cost: a browser without SVG tab-icon support shows no icon, which is a visible regression from today; in practice: that Safari tab is blank where the project mark was.
  3. **Raster first, vector alternate** — gain: no new markup risk; cost: browsers commonly take the first declaration, so the sharpness goal is lost on the surfaces that matter; in practice: the tab stays a resampled bitmap.
- Disposition: answered — option 1. It is the only option that keeps today's fallback while giving a current browser the vector, and the content check can require both declarations.

### D3 — How close must the vector be to the raster?

- Question: what acceptance proves the vector reproduces the existing mark without redesigning it — an exact palette with a measured closeness bound, a pixel-exact trace, or visual review alone?
- Options:
  1. **Exact palette plus a measured bound** — the vector's fills are exactly the three palette colours, four named sample points (navy field, cream disc, orange accent, transparent corner) match the raster, and the rendered difference against the raster is at most 2.0% RGBA RMSE at 1044 pixels, at most 1.0% for the 1044 renders downscaled to 48, and at most 4.0% for a browser-native 48-pixel render measured separately; gain: a number that refuses a wrong trace and a measured margin; cost: one manual render-and-compare run per mark change; in practice: the traced master measures 1.65% at 1044 and 0.32% for the 1044 renders downscaled to 48 when rendered with librsvg 2.62.3 through ImageMagick 7.1.2 and compared with `magick compare -metric RMSE`, a browser-native 48-pixel render measures 3.30%, the residual being the raster's antialiased edge fringe, and the implementation re-measures all three values in the pinned Chromium and records them.
  2. **Pixel-exact trace** — reproduce every antialiased edge pixel; gain: nothing visible; cost: a path per fringe value, orders of magnitude larger and unmaintainable; in practice: a 40 KB SVG that renders the same at 48 pixels.
  3. **Visual review only** — gain: no measurement to maintain; cost: no bound to refuse an accidentally shifted layer or a wrong palette entry; in practice: a mark with the wrong navy passes review.
- Disposition: answered — option 1. The measured values are recorded in `design.md`, which also names the rendering procedure and the structural checks the page-content check enforces on the shipped file.

### D4 — Which surfaces adopt the vector?

- Question: does the vector reach only the published page, the page and the README, or every surface that could show a mark?
- Options:
  1. **The published page and the README** — the header mark, the tab icon and the README logo; gain: every place the mark is displayed at a user-interface size becomes sharp, and the change stays inside one capability plus one documentation file; cost: the raster stays for the fallback; in practice: the page header, the tab and the README are vector, and the organization profile keeps its own apply path.
  2. **The published page only** — gain: the smallest change; cost: the README keeps a scaled 1044-pixel bitmap beside a page that no longer needs one; in practice: the README logo stays soft.
  3. **Every surface** — the page, the README, the organization profile and the release archive; gain: uniform artwork; cost: the profile is applied by a different change and the archive carries no icon, so this invents scope this change cannot land; in practice: an unlandable dependency.
- Disposition: answered — option 1.

### D5 — Do new user-interface action icons join this change?

- Question: should this change also add vector action icons (buttons, panels, states) now that a vector pipeline exists?
- Options:
  1. **No action icons** — the design system defines none today, and adding one is new design work with its own review; gain: the change stays a vector form of an existing asset; cost: none of the page's controls gain an icon; in practice: the page's controls keep their visible text labels.
  2. **Add action icons** — gain: a vector icon set; cost: new visual design nobody approved, and the design system, the reference and the baseline would all change; in practice: an unreviewed icon appears beside "Choose a duty plan".
- Disposition: out of scope — the change adds no icon the design system does not already define.

### D6 — What does the Site output hold, and does the licence change?

- Question: does the Site output gain a ninth member, and does the vector need new licence information?
- Options:
  1. **Nine members, licence unchanged** — `icon.svg` and `icon.png` travel together, and the vector inherits the existing CC0-1.0 annotation for `docs/assets/**`; gain: the page can declare both tab icons from its own origin, and no licensing edit is needed; cost: one more member in the published output the read-back compares; in practice: the output grows by about 9.5 KB and `reuse lint` passes unchanged.
  2. **Replace the raster in the output** — publish the vector only; gain: the output stays eight members; cost: the tab has no raster fallback, contradicting the use case's icon requirement for browsers without SVG support; in practice: option 3 of D2.
  3. **Copy the vector under a new licence file** — gain: an explicit licence for the new file; cost: a second licence statement for one mark, contradicting the page's own licence sentence; in practice: two CC0-1.0 assets with different paperwork.
- Disposition: answered — option 1. `REUSE.toml` already overrides `docs/assets/**` to CC0-1.0 and the publication's read-back compares the whole output, so the ninth member is enforced by the existing machinery.
