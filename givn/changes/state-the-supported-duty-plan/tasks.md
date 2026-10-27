# Tasks: state-the-supported-duty-plan

Release note (commit subject on every scenario revision):

> feat(publish-public-website): Every public statement now names the duty plan duty2cal converts: a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export.

Use-case contract: `givn/specs/state-the-converter-support-statement/usecase.md` — "Keep the public support statement true to the exercised evidence." The capability `state-the-support-statement` is new; the site carrier extends `publish-public-website`, the profile carrier extends `apply-configuration-change`, and the release use case changes only its contributor-route description. Guarantees: every carrier carries the same statement; the statement names only the plan whose generated fixture converts; a refused publication leaves the previously true statement. Personas: release owner, crew member, web reviewer, publication reader.

## Bundle context

- **Runner:** `./run-tests.sh` and `./run-tests.sh --e2e`; the delta features are `@documentation` and are skipped by the assembler. The executable evidence is `bats scripts/tests/support-statement.bats` and the browser smoke.
- **Check:** `scripts/support-statement-check.sh` (sourced constants in `scripts/support-statement.env`) runs from `scripts/public-text-check.sh` and as the first step of `scripts/site-evidence.sh`.
- **Site page:** the `what-it-does` and `non-affiliation` statements are exact; `scripts/check-site-content.sh` sources the same constants.
- **Evidence:** `evidence/visual/release-owner-runs-the-support-statement-check…/transcript.txt`, `evidence/visual/release-maintainer-publishes-the-site…/{mobile,desktop}.png`, `evidence/visual/a-revised-profile-content-updates-only-the-profile-file/transcript.txt`; the browser smoke's four frames are supplementary.
- **Profile:** the live apply was the operator's step; the state and the read-back are committed, and the live profile serves the statement.

## 1. The support statement, every carrier, and the check

Design constraints: the support/website/boundary/non-affiliation constants; the five carriers; the evidence record; the forms' outcome fields; the exact site statements; the check's named refusals; the site gate runs the check first.

- [x] RED — the check refuses a carrier that disagrees, the withdrawn wording, the future-support wording, a record without a fixture, and a form without the outcome fields; the bats suite exercises each against a mutated temporary tree.
- [x] GREEN — `scripts/support-statement-check.sh` and `scripts/support-statement.env`; the five carriers; the evidence record; the issue-form outcome fields; `scripts/check-site-content.sh`; `scripts/public-text-check.sh`; `scripts/site-evidence.sh`; the browser smoke passed and its four frames are committed.
- [x] REFACTOR — the constants live in one file, sourced by both checks; the site and public-text gates reconstruct the check's result.
- [x] COMMIT: `327b6f84e8bf89405caa662563dfcb725edaf445` — the check, carriers, forms, evidence record, site page and the browser frames; the refusal families are the bats tests in the same revision.

## 2. The site publication evidence

Design constraints: the page's statement and the site publication scenario's evidence frames under its slug; the page-content check exact; the site publication gate runs the support check first.

- [x] GREEN — the browser smoke captured the changed page at 360×780 and 1920×1080; the ready-state frames are committed under the publication scenario's slug.
- [x] REFACTOR — the four browser-smoke frames remain supplementary; the publication scenario's two frames are the same built output.
- [x] COMMIT: `327b6f84e8bf89405caa662563dfcb725edaf445` — the page copy and the evidence.

## 3. The profile carrier applied

Design constraints: the profile source is the reviewed `org/profile/README.md`; the apply was the operator's step; the state is committed after the green apply; the served content carries the three sentences.

- [x] GREEN — the operator ran the targeted plan/apply; the state changed; the live profile serves the support, boundary and non-affiliation sentences.
- [x] REFACTOR — the transcript records the state change and the live read-back; no plan file is committed.
- [x] COMMIT: `38fd9ecff775927b752c85ec5ec6cb7574cdaa2c` — the applied state and the profile transcript.

## Closeout

- [x] `./run-tests.sh` 84 scenarios / 303 steps pass; `./run-tests.sh --e2e` 1 scenario / 5 steps pass.
- [x] `bats scripts/tests/support-statement.bats` 7 tests pass; the support check is clean; `public-text-check.sh` clean (18 promises); `pii-guard.sh` clean.
- [x] `scripts/build-site.sh` and `scripts/check-site-content.sh` pass on the nine-member output; the browser smoke 12/12 steps passes.
- [x] `reuse lint` and `givn lint --change state-the-supported-duty-plan` pass (two inherited long-scenario advisories).
- [x] The live site footer is `v1.0.0-3-g327b6f8` and the live organization profile serves the statement.
