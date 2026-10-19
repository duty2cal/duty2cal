# Converter screen design system

Durable design system for the browser converter screen. It is the canonical
home for the screen's tokens, primitives, breakpoints, accessibility rules and
evidence mechanics; `design.md` of the change that introduces or changes the
screen owns the change-specific contract, and this document records the durable
direction.

- Scope: the single converter screen served from the built Site output.
- Interface: web (one page, no separate routes).
- Viewports: mobile 360x780; desktop 1920x1080.
- States: empty, ready, failure, and the separate `cannot-run` startup state
  (WebAssembly/ES-module/module-instantiation failure, before the picker).
  "Converting" is a transient status, not a declared screen state.
- Approval: the operator approved this direction on 2026-10-19 (design review);
  the same decision created the change's `docs/design/design-system.md`.
- Reference: [`reference/converter-reference.html`](reference/converter-reference.html)
  is the approved design-only rendered reference for the empty, ready and
  failure states. It is static because the page source does not exist yet; the
  implementation must use the same markup roles, tokens and copy. The `@e2e`
  screenshots become the execution evidence for the states the scenario
  traverses.

## Tokens

| Group | Values |
|---|---|
| Color | `--color-bg` page white; `--color-surface` light neutral panel; `--color-text` near-black; `--color-muted` secondary text; `--color-border` neutral divider; `--color-accent` blue for links and the primary picker; `--color-accent-contrast` white on accent; `--color-success` dark green for the ready heading; `--color-danger` dark red for the failure heading; `--color-focus` accent, used as a 2 px visible outline. Light theme only; no dark-mode claim. |
| Spacing | 0, 4, 8, 12, 16, 24, 32, 48, 64 px. Page padding 16 px mobile, 24 px at 480 px, 32 px at 720 px. |
| Typography | System stack `system-ui, -apple-system, "Segoe UI", Roboto, sans-serif`; 14 px small, 16 px body, 20 px section heading, 28 px page heading on mobile, 36 px at ≥ 720 px; line-height 1.5 body and 1.2 headings; weights 400 and 600. No third-party font. |
| Radii | 4 px controls and focus shape; 8 px converter and failure panels. |
| Elevation | Flat by default; one optional `0 1px 2px rgb(0 0 0 / 0.08)` panel shadow. No modal or overlay elevation. |
| Motion | One 120 ms opacity/translate transition; `--motion-duration: 0ms` under `prefers-reduced-motion: reduce`; no looping animation. |

Contrast ratios: body text ≥ 7:1; muted text, accent and danger ≥ 4.5:1 on
their surfaces; focus and UI boundaries ≥ 3:1. The values above meet WCAG 2.2
AA; `@axe-core/playwright` checks each rendered state.

## Primitives and states

| Primitive | States and rules |
|---|---|
| Page shell (`header`, `main`, `footer`) | Default only; one `h1`; landmarks; `<html lang="en">`. |
| Drop zone | Default, dragover/hover, focus-within, disabled while converting; accepts one file; visible instruction text. |
| Picker button and labelled file input | Default, hover, focus-visible, disabled/loading while converting; visible label; minimum 44 px touch target below 720 px. |
| Status region | Empty, converting, ready; `role="status"` and `aria-live="polite"`; the converting status is transient. |
| Offer link | Default, hover, focus-visible; text `Save <name>`; no disabled state. |
| Failure panel | Error `h2`, reason, optional diagnostic offer, `Choose another plan`; `role="alert"`; no calendar offer. |
| Reset button | Default, hover, focus-visible. |
| Privacy note | Static text: processing is local, no upload, no storage, no request after load. |

The page never shows an internal identifier or state name (`ok`, `failure`,
`status`, `WASM`, `blob:`) in visible text. Offered filenames are the intended,
user-facing exception.

## Breakpoints

Mobile-first single column.

- `< 480 px`: 16 px padding, full-width controls, 28 px `h1`.
- `480–719 px`: 24 px padding.
- `720–1199 px`: content centered at a 720 px maximum, 32 px padding, 36 px `h1`.
- `≥ 1200 px`: the same 720 px centered column; surrounding whitespace grows.
  There is no multi-column layout.

## Accessibility

WCAG 2.2 AA. Landmarks, one `h1`, a failure `h2`, a visible label on every
control, `role="status"`/`role="alert"`, visible keyboard focus, the token
contrast ratios, `lang="en"`, and reduced motion. The page honors
`prefers-reduced-motion: reduce` and the E2E asserts the computed motion value
is `0ms`. The failure state has a heading and a way back.

## Capture and evidence

The Playwright driver resizes to 360x780 and 1920x1080, emulates
`prefers-reduced-motion: reduce` and a light color scheme, waits for
`document.fonts.ready` and two animation frames after the final assertion, and
calls `page.screenshot({ animations: 'disabled' })`.

- The `@e2e` scenario "Crew member converts a supported duty plan in the browser
  and saves the offered calendar" commits one final-state screenshot per
  viewport: `<viewport>.png` for the ready state and `<viewport>--empty.png`
  for the empty state it traverses.
- Intermediate captures go to `reports/visual/` and are not review evidence.
- Screenshots are execution evidence; the approved reference for the three
  declared states is the committed static reference above. The failure state's
  visual reference is its section in that file; its behavior is asserted by the
  regular failure scenarios.

## What this document does not own

The page's public statements (both routes, source and release links, Built
revision, licence, non-affiliation, as-is) and the published footer belong to
the Site publication. The browser-owned save semantics, the capability gate,
the input bound and the no-network rule are product behavior owned by the
capability specification and the change's `design.md`.
