# Tasks: publish-legal-notices

Scenario-by-scenario Red → Green → Refactor, non-`@e2e` first, `@e2e` last,
then the documentation carriers. The owning use case is
`givn/changes/publish-legal-notices/specs/state-the-public-legal-notices/usecase.md`;
its guarantees, rules and confirmed Personas are the constraints below.

**Release note (every revision's message subject):**
"The public website carries an Impressum and a Datenschutzerklärung that every page links, and the publication refuses a page set that lacks them."

## Shared design constraints

- **Files and addresses:** `web/impressum.html` and `web/privacy.html` are
  members of the Site output; the public addresses are
  `https://duty2cal.github.io/impressum` and `…/privacy`. The output holds
  exactly eleven members.
- **Notices' policy:** no script, no inline handler, no cookie, no third-party
  asset, no consent banner; the stricter policy lives in
  `scripts/legal-notices.env` as `NOTICE_CSP` and is asserted by both checks.
- **Language:** `lang="de"` with an English summary under `lang="en"` stating
  that the German version is binding. Only the Datenschutzerklärung carries
  `Stand: 27.10.2026`.
- **Reachability:** the converter footer links both notices; each notice links
  the other and the converter page. The profile and the README carry two
  labelled links and copy neither address.
- **Checks:** `scripts/check-site-content.sh` reads the assembled output
  (members, links, CSP, link integrity, required strings, placeholders);
  `scripts/legal-notices-check.sh --root DIR` reads the source carriers;
  `scripts/public-text-check.sh` calls the latter; `scripts/tests/legal-notices.bats`
  is the output gate's evidence.
- **Browser evidence:** `web/e2e/driver.mjs` gained `navigate`, `click` and
  `read_text`, resolves `<path>` to `<path>.html`, and the two notice scenarios
  run from the `browser` CI job beside the conversion smoke.
- **Licence:** `REUSE.toml` overrides exactly the two notice pages to CC0-1.0;
  `web/index.html` stays under the aggregate MPL-2.0.

## Precondition — the operator's contact block

- [x] The operator supplied the contact block (Daniel Kressner, c/o adesso,
      Prinzenstraße 34, 10969 Berlin), the mail alias
      `duty2cal662.abridge974@aleeas.com`, the forwarding provider (SimpleLogin,
      Proton AG) and confirmed that the employer permits private mail delivery.
      No placeholder was committed; the pages and the checks that refuse them
      landed together.

## Setup — harness, driver actions, strict mode

- [x] `web/e2e/driver.mjs` gained `navigate`, `click` and `read_text`, and
      resolves an extensionless path to `<path>.html`; `ACTIONS` in
      `apps/cli/tests/steps/browser.rs` lists all thirteen actions.
- [x] `assert_visual_baseline` moved to
      `apps/cli/tests/steps/e2e/visual_baseline.rs` with the document language
      as a parameter; the conversion scenarios pass `"en"` unchanged.
- [x] `apps/cli/tests/steps/legal_notices.rs` holds the capability's steps and
      `DutyWorld` gained `heading_before` / `heading_after`.
- [x] Proof of strictness: with the step module unregistered,
      `./run-tests.sh --tags "@state-the-legal-notices"` reports
      `Step doesn't match any function` and `10 scenarios (10 failed)`.

## Scenarios

| # | Scenario | Evidence | Revision |
|---|---|---|---|
| 1 | A page set that serves only one of the two notices is refused and the missing notice is named | `legal-notices-check` names `privacy.html` | `2d752bc` |
| 2 | A page that does not link both notices is refused and the page is named | refusal names `privacy.html` and `Impressum` | `2d752bc` |
| 3 | An Impressum that still carries a placeholder is refused and the marker is named | refusal names `impressum.html` and `TODO` | `2d752bc` |
| 4 | An Impressum without a postal address or without an electronic mail address is refused | refusal names `impressum.html` and the missing postal address | `2d752bc` |
| 5 | A Datenschutzerklärung without a standing date is refused and the date is named | refusal names `privacy.html` and the standing date | `2d752bc` |
| 6 | A Datenschutzerklärung without an English summary is refused and the summary is named | refusal names `privacy.html` and the English summary | `2d752bc` |
| 7 | An Impressum that carries the discontinued dispute-resolution reference is refused | refusal names the withdrawn reference | `2d752bc` |
| 8 | A notice that loads an asset from another origin is refused and the asset is named | refusal names the foreign URL | `2d752bc` |
| 9 | Both notices are German with an English summary and name the German version as binding | language, summary position and binding sentence asserted | `2d752bc` |
| 10 | A carrier that copies the postal address instead of linking the notices is refused | refusal names `org/profile/README.md` | `2d752bc` |
| 11 | Reader opens the Impressum from the site footer and reads who publishes the site | browser scenario, 13 steps, screenshots at both viewports | `2d752bc` |
| 12 | Reader opens the Datenschutzerklärung from the footer and reads what a page request processes | browser scenario, 18 steps, screenshots at both viewports | `2d752bc` |

## Carriers (documentation)

- [x] The assembled set carries both pages: `scripts/build-site.sh` copies them
      and refuses a missing one; `scripts/check-site-content.sh` asserts the
      eleven members, the CSP meta, the absence of `<script` and `on*=`, the
      mutual links, link integrity and the required strings.
- [x] `scripts/tests/legal-notices.bats`: 19 tests, `bats scripts/tests` exits 0
      with 36 tests overall.
- [x] `.github/workflows/ci.yml` runs both notice scenarios by name; actionlint
      reports nothing.
- [x] `org/profile/README.md` and `README.md` carry two labelled links and no
      address; `scripts/public-text-check.sh` calls the notice check and exits
      clean with 18 promises.
- [x] `REUSE.toml` overrides the two pages to CC0-1.0.
- [x] `cliff.toml` lists `state-the-legal-notices` in the capability allow-list
      and its four commit groups.
- [x] `docs/arc42/01..12` and `docs/design/design-system.md` record the change;
      `docs/design/reference/notice-reference.html` is the approved rendered
      reference.

## Verification

- [x] `./run-tests.sh` — 2 features, 94 scenarios, 348 steps, all passed.
- [x] `./run-tests.sh --e2e` — 1 scenario, 5 steps, passed.
- [x] `./run-tests.sh --browser` — 2 features, 18 scenarios, 127 steps, passed
      (the conversion smoke and both notices).
- [x] `bats scripts/tests` — 36 tests, exit 0.
- [x] `scripts/public-text-check.sh` — clean, 18 promises; the notice check is
      clean.
- [x] `givn lint --change publish-legal-notices` — 4 findings, all advisory
      (three long scenarios, one persona path that resolves after archive).
- [x] `reuse lint` — compliant with version 3.3 of the REUSE Specification:
      610 of 610 files carry copyright and licence information (the tool was
      installed into a throwaway virtual environment because the Nix derivation
      failed to build on this machine).

## Deviation record

- **One atomic slice.** The pages, the two checks and the steps are one
  revision (`2d752bc`): a page set that lacks a notice is refused by the check
  that the same slice introduces, so no smaller slice is green. Every scenario
  therefore records the same revision, and this row states it rather than
  pretending twelve independent ones. The archived `announce-dependency-updates`
  change recorded the same shape.
- **Red proven once for the feature.** The strictness proof removes this
  capability's step module and runs the capability's tag: ten scenarios fail with
  `Step doesn't match any function`. The two browser scenarios use the same
  runner and the same strict flag; their steps are in the same file, so the
  proof covers the whole feature, not a per-scenario record.
- **`read_text` instead of per-link steps.** The two browser scenarios assert on
  the rendered document's text, links, language and storage rather than on one
  step per sentence; the assertions are one per `Then` in the feature, so a
  missing sentence fails its own step.

## Commits

- [x] COMMIT: 2d752bc — A page set that serves only one of the two notices is refused and the missing notice is named
- [x] COMMIT: 2d752bc — A page that does not link both notices is refused and the page is named
- [x] COMMIT: 2d752bc — An Impressum that still carries a placeholder is refused and the marker is named
- [x] COMMIT: 2d752bc — An Impressum without a postal address or without an electronic mail address is refused
- [x] COMMIT: 2d752bc — A Datenschutzerklärung without a standing date is refused and the date is named
- [x] COMMIT: 2d752bc — A Datenschutzerklärung without an English summary is refused and the summary is named
- [x] COMMIT: 2d752bc — An Impressum that carries the discontinued dispute-resolution reference is refused
- [x] COMMIT: 2d752bc — A notice that loads an asset from another origin is refused and the asset is named
- [x] COMMIT: 2d752bc — Both notices are German with an English summary and name the German version as binding
- [x] COMMIT: 2d752bc — A carrier that copies the postal address instead of linking the notices is refused
- [x] COMMIT: 2d752bc — Reader opens the Impressum from the site footer and reads who publishes the site
- [x] COMMIT: 2d752bc — Reader opens the Datenschutzerklärung from the footer and reads what a page request processes
