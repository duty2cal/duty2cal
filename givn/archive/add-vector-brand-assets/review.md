# Review: add-vector-brand-assets

## Scope

The change gives the project's existing mark a vector master
(`docs/assets/duty2cal.svg`) and uses it for the Site header mark, the Site
tab's primary icon and the README logo, keeping `docs/assets/duty2cal.png`
byte-for-byte as the tab alternate. The Site output grows from eight to nine
members. The page-content check gains the mark, fallback, order/type, member and
plainness/palette refusals. Two enabling fixes are included: the assembler skips
a `@documentation` delta, and the browser evidence directory names a change id.

## Fabrication audit

### @e2e tag integrity

The delta retains `@e2e` on the modified publication scenario. No scenario's
`@e2e` tag was removed.

```text
1:@givn.delta @publish-public-website @documentation @nested-build
4:  @givn.modified @e2e @site-publication
22,30,38,46:  @givn.added @site-publication
```

No `@wip` remains in the delta.

### Empty step bodies

No step definition is added or changed for the delta: it is a `@documentation`
feature and the assembler skips it. The only changed test helper is
`evidence_dir()` in
`apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs`, which resolves the
capture directory; it has a real body and is exercised by the browser run.
Scan of the changed file for empty/bare bodies: none.

### Task and commit check

| Task | Commit | Production files touched |
|---|---|---|
| Publication scenario (vector asset, page, build, check, README) | `eef8497ea7409349aad6ff7c2e8094794b02032f` | `docs/assets/duty2cal.svg`, `web/index.html`, `scripts/build-site.sh`, `scripts/check-site-content.sh`, `README.md`, `scripts/assemble_features.py`, `apps/cli/tests/steps/e2e/export_duty_plan_calendar.rs` |
| Header raster refusal | `ee27947f0fb0ae508a571f54e22c901b6697155e` | `scripts/check-site-content.sh` |
| Raster alternate refusal | `be598de05334df5e5cc93ab44b2864c1ff275376` | `scripts/check-site-content.sh` |
| Missing icon member refusal | `480864498f6cf746bdcb0834692e9053d26b0410` | `scripts/check-site-content.sh` |
| Non-plain vector refusal | `43b79a8dd2bb81517d241fe327d097c4a820bf93` | `scripts/check-site-content.sh` |
| Evidence and browser twin | `a586841d63a1c5ba823228de008bb26f6a6ac106` | `web/e2e/driver.mjs`, evidence |

Every checked task has a commit that touches production or the executable gate.
The bookkeeping commit `11cafdba` only records `tasks.md`.

### Promised components

| Design promise | Present |
|---|---|
| `docs/assets/duty2cal.svg`, 4 paths, 9,474 bytes, 1044 viewBox | yes |
| Page: vector tab first, raster alternate, header `./icon.svg` 48×48 | yes |
| `scripts/build-site.sh` copies both, nine members | yes |
| `scripts/check-site-content.sh`: members, tabs, header, plainness/palette | yes |
| `scripts/assemble_features.py` skips `@documentation` deltas | yes |
| `evidence_dir()` change-id override | yes |
| README vector mark | yes |

### Strict mode and execution evidence

`.fail_on_skipped()` remains in `apps/cli/tests/features_runner.rs`. The tasks
verification receipt records 84 regular scenarios, 0 failed, 0 skipped. The e2e
scope records 1 scenario, 0 failed, 0 skipped.

### @e2e assertions and driver

The delta's `@e2e` publication scenario is an operational `@documentation`
record; the repository's eight operational features follow the same convention
and are skipped by the assembler. The executable browser evidence is the
conversion scenario "Crew member converts a supported duty plan in the browser
and saves the offered calendar", which drives Playwright through
`web/e2e/driver.mjs` and asserts the rendered page (including the visual
baseline), not repository state. It was run for this change and captured the
four screenshots below.

### E2E scope

`verify.command` is `./run-tests.sh`; `verify.e2e_command` is
`./run-tests.sh --e2e` (a tag filter, not the same string). The recorded counts
are 84 regular scenarios and 1 e2e scenario; the e2e scope is strictly smaller.

### Interaction coverage

| Inventory entry (`usecase.md`) | Design matrix row | @e2e scenario | Driving mechanism |
|---|---|---|---|
| publish the site from a source revision | present | Release maintainer publishes the site and the hosting target holds the built page | content check over the assembled output + Playwright for the rendered page |

The added refusal scenarios are error cases of the same action and add no
inventory entry.

### Design conformance

All named commands, file paths and the Playwright framework match `design.md`.
One implementation detail is not named in `design.md`: the browser twin's
`CONTENT_TYPES` map gained `.svg: image/svg+xml`. Without it the twin served the
assembled `icon.svg` as `application/octet-stream` and the page rendered a broken
image — the first evidence run showed exactly that. The twin's stated job is to
serve the assembled output as the host does; the MIME map is part of that job,
not a changed design decision. The evidence was recaptured after the fix.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Output constraint | `docs/arc42/02` | affected | nine members, both marks | scenario 1 | `build-site.sh`, 9 files in `dist/site` | yes |
| Published-site strategy | `docs/arc42/04` | affected | vector mark with raster fallback | scenario 1 | page and build | yes |
| Building blocks | `docs/arc42/05` | affected | page, build, check, new asset | scenarios 1-5 | files present, checks exercised | yes |
| Deployment artefact | `docs/arc42/07` | affected | nine members | scenario 1 | build output | yes |
| Local image rule | `docs/arc42/08` | affected | same-origin SVG + PNG alternate | scenarios 1-4 | content check | yes |
| ADR register | `docs/arc42/09` | affected | no ADR, one canonical destination | — | `arc42.md` verdict | yes |
| Risk TD-026 | `docs/arc42/11` | affected | two files, one mark | — | `11` row 117 | yes |
| Brand mark term | `docs/arc42/12` | affected | umbrella of vector master and raster alternate | — | glossary | yes |
| Chapters 1, 3, 6, 10 | unchanged | No | no goal/boundary/flow/quality change | — | diff | yes |

`ARC42 CONFORMANCE: CLEAN`

## Ubiquitous language conformance

**Brand mark** is recorded in `docs/arc42/12-glossary.md` as the umbrella whose
two forms are the vector master and the raster alternate. The proposal and the
design scope use the term; the spec and design use "vector mark" and "raster
mark" for those forms. No invented or inconsistently reused term remains.

## Coverage

No instrumented run exists for this change yet; the archive receipt will carry
the merged result. Inspection classifies the changed surfaces:

- **Dead code:** none added; no unused path.
- **Missing test coverage:** the new gate is exercised by the five scenario
  evidence runs in `tasks.md` (valid + doctored outputs) and by the browser
  scenario; no uncovered production path remains.
- **Legitimately hard to test:** the per-mark fidelity measurement is a manual
  one-off (`evidence/fidelity.md`) because a renderer in the publication gate
  would be a new permanent dependency for an asset that changes only with a
  design decision. The screenshots cover the rendered result.

Redundant unit tests: none added.

## E2e coverage

The interface is a Web UI; the design declares `Interface: web` and Playwright.
The inventory's one action maps to one `@e2e` scenario. The browser step drives
the real browser (navigate, set file, read rendered state, screenshot), never an
HTTP client or `fetch`. No over-produced e2e coverage.

## Use-case and Persona conformance

PERSONAS: project-maintainer--release-owner
PERSONAS: contributor--web-reviewer

The use case keeps one goal, one primary actor (Release maintainer), the same
capability placement, extension-to-step references and preconditions. No
rejection criterion applies: the goal has one verb phrase, no mechanism name, no
component/channel-shaped capability, and the leaf does not name three actor
goals.

Persona dispositions:

- `project-maintainer--release-owner` — relevant; judges the published mark, the
  nine-member output and the release coupling. Evidence: the screenshots, the
  content check, the build.
- `contributor--web-reviewer` — relevant; judges the asset provenance,
  same-origin closure and the built output. Evidence: `reuse lint`, the content
  check's foreign-asset patterns, the browser no-foreign-request assertion.
- `visitor--project-reader` and `crew-member--personal-calendar-owner` —
  unchanged lenses; the page copy, states and conversion route are untouched.

## Visual review

Two fresh-context vision reviews ran, one per confirmed Persona. Both saw the
four committed captures:

- `desktop--empty.png` — empty state, page from the top, header mark in frame.
- `mobile--empty.png` — empty state, page from the top, header mark in frame.
- `desktop.png` — ready state, viewport scrolled to the statements/result panel.
- `mobile.png` — ready state, viewport scrolled to the statements/result panel.

The tab icon is browser chrome and cannot appear in a page screenshot; its
declarations and order are asserted by the content check.

The publication scenario's two frames
(`evidence/visual/release-maintainer-publishes-the-site-and-the-hosting-target-holds-the-built-page/{mobile,desktop}.png`)
are the same assembled page at the ready state, reused from the browser run
because the publication feature is `@documentation` and has no runner. They show
the page the publication ships; the header mark is visible in the empty-state
captures of the same output.

| Persona | Goal reachable | Findings | Verdict |
|---|---|---|---|
| `project-maintainer--release-owner` | yes — the header carries the mark at 48×48 left of the h1 and lede, the statements/panels/footer are unchanged, and both states carry the Built revision of the same assembled output | none; the ready-state framing and the tab-icon limit are unchanged driver behaviour and contract scope, not defects | PASS |
| `contributor--web-reviewer` | yes — the mark is the same project mark rendered sharply at 48×48, one image, no broken or foreign asset, nine-member output | none; the tab icon is evidenced in markup and the assembled output, and the ready-state framing is the unchanged capture flow | PASS |

Both reviewers diffed the empty-state captures against the published-site
baseline: the only changes are the mark box and the build-derived revision text.

VISUAL-REVIEW: PASS

## Overlap dispositions

<!-- filled from givn check review -->

## Split-or-keep

| Long scenario | Decision | Reason |
|---|---|---|
| Release maintainer publishes the site and the hosting target holds the built page | keep | It is one publication action; splitting it would duplicate the action or create fragments that cannot run, because the feature is `@documentation`. |

## README impact

README-IMPACT: updated - Description

The Description's top mark now points at `docs/assets/duty2cal.svg`; the prose
is unchanged.

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching production code.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope (84 regular, 1 e2e).
- [x] Coverage classification uses the available evidence; the archive receipt
      will carry the merged result.
- [x] Every coverage gap classified and resolved.
- [x] No dead code; missing tests added; the hard-to-test measurement justified.
- [x] No redundant unit tests.
- [x] No `@wip` tags; no implementation-layer detail in the spec.
- [x] Canonical E2E policy applied: one inventory action, one `@e2e`, real
      driver.
- [x] Every E2E scenario has a real-interface primary assertion.
- [x] Local run command starts the stack (`scripts/run-web.sh`; the browser
      scenario starts its loopback twin).
- [x] The exact e2e file was identified; no parallel implementation exists.
- [x] `verify.e2e_command` differs from `verify.command`; counts prove the
      subset.
- [x] Implementation matches `design.md`; the twin MIME map is an implementing
      detail, named above.
- [x] Interaction coverage verified end to end.
- [x] No finding excused outside the three buckets.

REVIEW: PASS
