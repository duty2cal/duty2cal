# Tasks: state-site-footer

One page, one rule and two checks. `design` and `design-review` are skipped: no
new screen, no visual state and no token changes; the footer keeps its place and
its links, and the durable primitive that describes it is updated in the same
change.

Use-case context: `publish-public-website` owns the page's copy. Personas:
`visitor--project-reader`, `project-maintainer--release-owner`,
`contributor--web-reviewer`.

## Implementation

- [x] `web/index.html`: the footer no longer carries the non-affiliation and
      as-is sentence; it keeps the Built revision, the local-processing note and
      the two notice links.
- [x] `scripts/check-site-content.sh`: the footer-prose assertion is removed; the
      `non-affiliation` statement element is still required and compared.
- [x] `scripts/support-statement-check.sh`: the same assertion is removed from
      the site carrier check.
- [x] `givn/specs/publish-public-website/usecase.md`: the rule says where each
      sentence lives, and that no sentence is stated twice.
- [x] `docs/design/design-system.md`: the footer primitive names the revision,
      the local-processing note and the links, and says the non-affiliation and
      as-is sentences live in the panel.
- [x] `docs/arc42/07-deployment-view.md` and `11-risks-and-technical-debt.md`:
      the layout row drops "plain prose" and TD-014 is removed as paid.

## Evidence

- [x] `scripts/support-statement-check.sh` clean: five carriers agree.
- [x] `scripts/check-site-content.sh` clean on the assembled output; the page
      carries one non-affiliation sentence (the panel element).
- [x] COMMIT: the revision this change records

## Verification

- [x] `./run-tests.sh` — 94 scenarios, 348 steps, all passed.
- [x] `./run-tests.sh --browser` — 18 scenarios, 127 steps, all passed.
- [x] `bats scripts/tests` — exit 0.
- [x] `givn lint --change state-site-footer` — one advisory (long scenario).
