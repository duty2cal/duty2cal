# Published page design system

Durable design system for the pages served from the Site output. It is the
canonical home for the page set's tokens, primitives, breakpoints, accessibility
rules and evidence mechanics; `design.md` of the change that introduces or
changes a screen owns the change-specific contract, and this document records the
durable direction.

- Scope: the page set served from the Site output — the documents assembled to
  `dist/site/` and published at `https://duty2cal.github.io/`: the converter
  screen and the two legal notice screens, the Impressum and the
  Datenschutzerklärung.
- Interface: web (three documents, no client-side routing).
- Viewports: mobile 360x780; desktop 1920x1080.
- States: for the converter screen, empty, ready, ready-with-warnings, failure, and
  the separate `cannot-run` startup state (WebAssembly/ES-module/module-instantiation
  failure, before the picker); "Converting" is a transient status, not a declared
  screen state. `ready-with-warnings` is the ready state with one or more rows
  that were represented by a documented fallback; it shows the same calendar
  offer, the unknown duties under the heading `Unknown duties`, and the
  warning-report offer labelled `Save unknown duty warning for publisher`. The
  failure state carries the error and warning lists. The notice screens declare
  the default state only.
- Approval: the operator approved this direction on 2026-10-19 (design review);
  the same decision created the change's `docs/design/design-system.md`. The
  operator approved an amendment on 2026-10-25 for the site publication: the
  statements panel sits **above** the converter panel, a separate "Source and
  licence" panel sits **below** the converter panels and above the footer, the
  footer stays full, and the page header gains the project icon as a 48×48 mark
  left of the heading. An amendment on 2026-10-27 changes the rendered form of
  that mark, not the direction: the header mark and the tab's primary icon come
  from the project's vector mark (`docs/assets/duty2cal.svg`, copied into the
  Site output as `icon.svg`), the tab keeps `icon.png` as the raster alternate
  for a browser without SVG tab icons, and the box stays 48×48 with the same
  palette and the same placement.
- Amendment 2026-10-27 (`report-row-diagnostics`): the converter screen declares
  the `ready-with-warnings` state and the `Diagnostic list` primitive. The
  operator's brief for that change requires every row's errors and warnings to
  be visible on the page and requires the matching report to be offered, so the
  ready state shows its warnings beside the calendar and the failure state shows
  both severity lists. No token changes: the lists use the existing border, ink,
  muted, danger, spacing and type tokens. **The operator approved this amendment
  as drawn on 2026-10-27**, choosing the rendered reference literally over a
  reconciled variant: the ready state keeps the drawn `Choose another duty plan`
  button and the copy `Your complete calendar is ready. The browser chooses
  where the file is saved.` with the status `Calendar ready for <name>.`, the
  ready-with-warnings state uses the plural copy and
  `Your calendar is ready to save. <n> rows were converted with warnings.`, and
  the failure state keeps `Conversion failed`, a count-based reason sentence,
  the two lists, the report offer, the note and `Choose another plan`. The
  approval covers only these page states and controls; it authorizes no token,
  layout, breakpoint or script change.
- Amendment 2026-10-28 (`label-unknown-duties`): the converter screen's two ready
  states are drawn with the settled copy, and this amendment supersedes the
  ready-state copy and labels recorded in the 2026-10-27 amendment above. Both
  states are headed `Calendar ICS ready` and carry the offer copy `You can
  choose where to save the file.` and the way back `Open new/next duty plan
  PDF`. The plain ready status stays `Calendar ready for <name>.`; the
  ready-with-warnings status becomes `Your calendar is ready to save. <n> rows
  were converted with unknown duties.`, with
  `Your calendar is ready to save. 1 row was converted with an unknown duty.`
  for one row. Its list is headed `Unknown duties` and its entries name the
  date, the optional UTC start and the code — `DD-MM-YYYY HH:MM UTC unknown
  duty: <code>` for a timed duty and `DD-MM-YYYY unknown duty: <code>` for an
  all-day one — one entry per unknown duty in plan order. The calendar offer
  keeps `Save <stem>.ics`; the warning-report offer becomes `Save unknown duty
  warning for publisher` while its file stays `<stem>.parse-warnings.txt`. The
  same title rule applies to the calendar: an activity represented from an
  unrecognized code is titled `unknown duty: <printed code>`. The failure
  state, the report text and filename, and every token are unchanged. The
  amendment changes the child order inside the offer region only: heading,
  status, list, save-choice copy, offers, note, way back; tokens, breakpoints
  and the surrounding panels are unchanged, so it authorizes no token, layout,
  breakpoint or script change beyond that ordering.
- Reference: [`reference/converter-reference.html`](reference/converter-reference.html)
  is the approved design-only rendered reference for the empty, ready and
  failure states. It is static because the page source does not exist yet; the
  implementation must use the same markup roles, tokens and copy. The 2026-10-25
  approval extends it with the statements panel above the converter, the "Source
  and licence" panel below it, the full footer and the header icon, and the
  2026-10-27 amendment draws that icon from the vector mark, so the approved
  direction shows the same placement, tokens, copy and mark the page must use.
  The `@e2e` screenshots become the execution evidence for the states the
  scenario traverses. The 2026-10-27 `report-row-diagnostics` amendment added
  the `ready-with-warnings` view, the ready states' way back and the failure
  view's `Errors`/`Warnings` lists; the 2026-10-28 `label-unknown-duties`
  amendment replaced the ready views' heading, offer copy, list heading,
  entries, offer labels and way-back label, so the ready views name none of
  the superseded copy; those two states are
  exercised by regular `@browser` scenarios, whose screenshots are the
  change-local execution evidence, not new `@e2e` evidence, and which run in the
  nightly `--browser` job (R-070/R-071) rather than the per-push gate. The
  reference's ready, ready-with-warnings and failure views are authoritative for
  copy and controls, as the operator approved on 2026-10-27 and amended on
  2026-10-28 for `label-unknown-duties`.
- Amendment 2026-10-27 (legal notices): the two notice screens inherit the
  approved shell — the header with the 48x48 vector mark and the page title, the
  surface panel, the 720 px column and the footer — and add no token, no image
  and no script. The operator chose this direction on 2026-10-27 over a
  standalone document without the mark. Their rendered reference is
  [`reference/notice-reference.html`](reference/notice-reference.html).

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
| Status region | Empty, converting, ready, ready-with-warnings; `role="status"` and `aria-live="polite"`; the converting status is transient. The ready status reads `Calendar ready for <name>.`; the ready-with-warnings status reads `Your calendar is ready to save. <n> rows were converted with unknown duties.` (`Your calendar is ready to save. 1 row was converted with an unknown duty.`), so it states how many rows were converted with an unknown duty without naming an internal state. |
| Offer link | Default, hover, focus-visible; text `Save <name>` in general; no disabled state. The calendar offer reads `Save <stem>.ics`, and in the ready-with-warnings state the warning report is offered as `Save unknown duty warning for publisher` for the file `<stem>.parse-warnings.txt`, so the two offers beside each other name the calendar and the publisher's note distinctly. |
| Diagnostic list | Default (visible with entries) and empty (hidden). A block with a 3 px `--color-border` left rule, a 16 px heading, and a real `<ul>` whose items wrap with `overflow-wrap: anywhere` so a long activity code cannot clip at 360 px. In the ready-with-warnings state the block is headed `Unknown duties` and its items are the unknown-duty lines — `DD-MM-YYYY HH:MM UTC unknown duty: <code>` or `DD-MM-YYYY unknown duty: <code>` — one per converted row in plan order, using the panel's text colors; in the failure state two blocks are headed `Errors` and `Warnings` and carry the row diagnostics, and their list markers use `--color-danger`. Not interactive: no hover, focus or disabled state. The visible words carry the meaning; the marker color never does. |
| Failure panel | Error `h2`, a count-based reason sentence naming the source and how many rows cannot be represented and how many were converted with warnings, optional `Errors` and `Warnings` diagnostic lists, optional report offer, `Choose another plan`; `role="alert"`; no calendar offer. The first error sentence appears only as the first `Errors:` entry, never also as the reason. |
| Reset button (`#reset`) | Default, hover, focus-visible; labelled `Choose another plan`; the failure state's way back. |
| Ready reset button (`#reset-ready`) | Default, hover, focus-visible; labelled `Open new/next duty plan PDF`; shown in the ready and ready-with-warnings states as the reference draws them; returns the screen to `empty` and clears the attempt. No disabled state. |
| Privacy note | Static text: processing is local, no upload, no storage, no request after load. |
| Public statements panel | Default only. Its own surface panel **above** the converter panel, headed "Before you choose a plan": a single-column list stating what the tool does (`what-it-does`) — the one supported plan, a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export, with its boundary — both conversion routes — this page in a current browser (`browser-route`) and downloads for various operating systems available on request, with the release page `https://github.com/duty2cal/duty2cal/releases` carrying the published builds (`archive-route`) — the privacy rule (`privacy`), the non-affiliation sentence naming Lufthansa and Jeppesen (`non-affiliation`) and the as-is sentence (`as-is`). Each statement carries `data-site-statement="<name>"` with non-empty text; every link is keyboard reachable with visible focus; no disabled, loading or error state. Uses the existing panel surface, 8 px radius and body/link typography. |
| Source and licence panel | Default only. A separate surface panel **below** the converter panels and above the footer: the source repository link (`source`) and the licence statement with its licence-text link (`license`), naming MPL-2.0 for the code, CC0-1.0 for the non-code assets and the Unicode-3.0 attribution. Each statement carries `data-site-statement="<name>"` with non-empty text; links are keyboard reachable with visible focus; no state varies. |
| Header icon and favicon | Default only. The project's vector mark (`docs/assets/duty2cal.svg`, CC0-1.0, the same silhouette and palette as `docs/assets/duty2cal.png`) rendered as a 48×48 mark **left of the `h1`** in a flex row with the heading and its subheadline, with the alternative text `duty2cal`, and copied into the Site output as `icon.svg`. The tab declares the mark twice: `<link rel="icon" type="image/svg+xml" href="./icon.svg">` first, so the vector is the primary tab icon, and `<link rel="icon" type="image/png" href="./icon.png">` second, the raster alternate (`docs/assets/duty2cal.png`, CC0-1.0) for a browser that cannot use an SVG tab icon. Both paths are relative and same-origin; the inline `data:, ` placeholder is gone. The mark's alt text names the project and no meaning is carried by the image alone. No state varies. |
| Legal notice screen | Default only. The same header as the converter screen carries the notice's `h1` and its lede (the standing date on the Datenschutzerklärung); below it each section is its own surface panel — the contact data in an `<address>` block, section `h2`s, and the English summary as a panel under `lang="en"` that states the German version is binding. The footer links the converter page and the other notice. Links are coloured with the accent token and are the only interactive element: default, hover and focus-visible. No image beyond the header mark, no script, and the standing date only on the Datenschutzerklärung. |
| Site footer | Default only, and kept full on the converter screen. On the notice screens the footer is the reduced form: the link to the converter page and to the other notice and no prose; the Built revision, the local-processing note and the non-affiliation and as-is sentences are converter-screen statements, and the Impressum states the non-affiliation and licence sentences in its own panel. The Built revision ("Built from `<describe>`" linking the commit), the page's local-processing note and the two notice links; the non-affiliation and as-is sentences live in the statements panel, so the footer repeats none of them. The footer repeats no `data-site-statement` attribute from either panel — those live in the two panels — while the revision keeps the `built-revision` attribute the content check anchors on; the revision text is the only build-derived visible string. Rendered above the existing top border in the existing muted small-text treatment; no state varies. |

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

The header is a flex row: the 48×48 icon, then the `h1` with its subheadline.
Below it the statements panel, the converter panel, the "Source and licence"
panel and the full footer stack in that order within the same column, and the
statements panel's list is always single-column. The header mark stays 48×48 px
at every breakpoint. On the notice screens the same header is followed by the
notice panel and the footer, with no converter panel and no statements panel;
the notice text is a single column with the same 720 px maximum measure.

## Accessibility

WCAG 2.2 AA. Landmarks, one `h1`, a failure `h2`, a visible label on every
control, `role="status"`/`role="alert"`, visible keyboard focus, the token
contrast ratios, `lang="en"`, and reduced motion. The diagnostic lists are real
lists with a heading: an `h3` under the state's `h2` in both the ready and the
failure state (`Unknown duties` in the ready-with-warnings state, `Errors` and
`Warnings` in the failure state), so the heading order stays valid, and no
meaning is carried by the marker colour alone. An unknown-duty entry names its
date, its optional UTC start and the activity code as visible words, so its
meaning is not left to a marker or an abbreviation. Each document declares its own
language: the converter screen `lang="en"`, the notice screens `lang="de"` with
`lang="en"` on their English summary, and their contact data sits in an
`<address>` element. The page honors
`prefers-reduced-motion: reduce` and the E2E asserts the computed motion value
is `0ms`. The failure state has a heading and a way back.

The header icon is a local same-origin vector image and the only image on the
page: its `alt` text is `duty2cal`, its rendered box is 48×48 px at every
breakpoint, it sits left of the heading in a flex row, and the content security
policy adds `img-src 'self'` beside the existing directives so the favicon and
the mark cannot load from another origin. The raster tab alternate is browser
chrome, not page content, and carries the same mark. The icon is not the sole
carrier of any information. The statements panel is a list in document order
before the converter panel; the "Source and licence" panel follows the converter
panels; the full footer stays inside the page footer landmark and its
built-revision link is reachable with visible focus, with its prose read after
the panels.

## Capture and evidence

The Playwright driver resizes to 360x780 and 1920x1080, emulates
`prefers-reduced-motion: reduce` and a light color scheme, waits for
`document.fonts.ready` and two animation frames after the final assertion, and
calls `page.screenshot({ animations: 'disabled' })`.

- The `@e2e` scenario "Crew member converts a supported duty plan in the browser
  and saves the offered calendar" commits one final-state screenshot per
  viewport: `<viewport>.png` for the ready state and `<viewport>--empty.png`
  for the empty state it traverses. Evidence filenames use the contract
  viewport labels `mobile` and `desktop`.
- The regular `@browser` scenarios "The page shows the warnings and offers both
  files when a plan converts with warnings" and "The page lists the errors and
  the warnings when a plan cannot be converted" commit
  `<viewport>--ready-with-warnings.png` and `<viewport>--failure.png`
  respectively, under
  `givn/changes/report-row-diagnostics/evidence/visual/<scenario-slug>/`, because
  the change declares those two states and adds no `@e2e` scenario.
- Intermediate captures go to `reports/visual/` and are not review evidence.
- The `@e2e` scenario "Release maintainer publishes the site and the hosting
  target holds the built page" commits the final state of the published page —
  as a visitor first sees it, with the header icon, the statements panel above
  the converter, the "Source and licence" panel below it and the full footer —
  per viewport, because it renders the target's checked-out tree after the
  publication.
- The `@e2e` scenarios "Reader opens the Impressum from the site footer and
  reads who publishes the site" and "Reader opens the Datenschutzerklärung from
  the footer and reads what a page request processes" commit one final-state
  screenshot per viewport of the rendered notice screen, in the same evidence
  layout, under the scenario title slugified by the archive gate.
- Screenshots are execution evidence; the approved reference for the three
  declared states is the committed static reference above. The failure state's
  visual reference is its section in that file; its behavior is asserted by the
  regular failure scenarios.

## What this document does not own

The statements panel, the "Source and licence" panel, the header icon and the
footer are placed and styled under this document's tokens, primitives and
breakpoints; their *content requirement* — which statements must be present
before the page offers a file picker, which belong to the panel below it, that
the header names the project with an icon, and that the footer names and links
the Built revision — belongs to the Site publication's use case and capability
specification, and their wording is reviewed copy. The archive route's copy
names downloads for various operating systems available on request and links
the project's release page; the page does not claim a Windows-only download
instruction, and the one-archive platform rule stays where it already is — the
release capability and chapter 7. The icons' licence is a project fact:
`docs/assets/duty2cal.svg`, `docs/assets/duty2cal.png` and the rest of
`docs/assets/**` are CC0-1.0 through a `REUSE.toml` override, the vector being
traced from the raster and therefore derived from the same CC0-1.0 asset, while
the published page states MPL-2.0 for the code, CC0-1.0 for the non-code assets
and the Unicode-3.0 attribution. The site publication refuses a page whose
header mark is not the vector or whose tab declares no raster alternate, so the
two files travel together in the Site output as `icon.svg` and `icon.png`. The
browser-owned save semantics, the capability gate, the input bound and the
no-network rule are product behavior owned by the conversion capability
specification and the archived browser change's `design.md`.
