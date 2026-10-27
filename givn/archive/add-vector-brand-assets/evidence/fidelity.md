# Fidelity measurement — duty2cal.svg

Date: 2026-10-27. Master: `docs/assets/duty2cal.svg` (9,474 bytes, 4 paths,
`viewBox="0 0 1044 1044"`), derived from `docs/assets/duty2cal.png`
(1044×1044, 42,802 bytes). Comparisons are RGBA RMSE with
`magick compare -metric RMSE`; the transparent rounded corners make an RGB-only
comparison meaningless.

## Recorded at design time — librsvg 2.62.3 / ImageMagick 7.1.2

| Comparison | Value | Bar |
|---|---|---|
| 1044×1044 | 1.65% | ≤ 2.0% |
| 1044 renders downscaled to 48×48 | 0.32% | ≤ 1.0% |
| Browser-native 48×48 (librsvg) | 3.30% | ≤ 4.0% |

## Re-measured at implementation — Chromium 156.0.8078.47 via Playwright

Chromium was supplied through `CHROMIUM_PATH` (the repository's supported
override); DPR 1, transparent screenshot, viewport matching the clip.

| Comparison | Value | Bar | Result |
|---|---|---|---|
| 1044×1044 | 1.68% | ≤ 2.0% | pass |
| 1044 render downscaled to 48×48 | 0.33% | ≤ 1.0% | pass |
| Native 48×48 | 3.85% | ≤ 4.0% | pass |

The native 48-pixel result is the honest cost of rasterising the vector at the
tab's size; it is never compared against the 1.0% bar, which belongs to the
downscaled 1044 comparison.

## Commands

```sh
magick compare -metric RMSE docs/assets/duty2cal.png render-1044.png null:
magick docs/assets/duty2cal.png -resize 48x48 raster-48.png
magick render-1044.png         -resize 48x48 render-48.png
magick compare -metric RMSE raster-48.png render-48.png null:
```

The 48-pixel native capture is a 48×48 screenshot of the SVG rendered at 48×48
with `deviceScaleFactor: 1`, compared against the raster resized to 48×48.
