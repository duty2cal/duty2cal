# Design review: publish-legal-notices

- Inputs: `givn/changes/publish-legal-notices/{proposal.md,specs/**,design.md,arc42.md}`, `docs/arc42/**`, `docs/design/design-system.md`, the permanent specs under `givn/specs/`, and the machinery the design names (`scripts/build-site.sh`, `scripts/check-site-content.sh`, `scripts/public-text-check.sh`, `scripts/publish-site.sh`, `scripts/site-evidence.sh`, `run-tests.sh`, `web/e2e/driver.mjs`, `apps/cli/tests/steps/`).
- Method: a fresh-context grilling subagent wrote the plan, produced 22 findings ranked by blast radius and walked every required branch; the orchestrator decided the findings the codebase answers, put the one operator decision to the operator, and a separate hardening pass applied the decisions.
- Verification at sign-off: `givn lint --change publish-legal-notices` exits 2 with advisories only (12 `@wip`, four persona-path advisories, three long-scenario advisories); `bash -n scripts/legal-notices.env` passes; `REUSE.toml` parses.

## Grilling findings and dispositions

| # | Finding | Disposition |
|---|---|---|
| 1 | Both operational deltas lose the documentation tag they inherit from their permanent features, so the assembler would emit them into the runnable corpus and the first undefined operational step would fail the suite. | Applied: `@documentation` / `@documentation @nested-build` restored on both deltas. |
| 2 | The refusal scenarios exercise the assembled output, but no runner mode both builds the output and selects them; the only GREEN path fabricated the output. | Applied: the Gherkin refusals now exercise the source-level check on a page set, the assembled-output gate is evidenced by `scripts/tests/legal-notices.bats` (fixture output, no build), and the browser scenarios prove the built output serves both pages. |
| 3 | The checks asserted fewer elements than the design's required-element table claimed, so a published notice could omit the retention statement, the rights or the English summary. | Applied: one named list in `scripts/legal-notices.env` drives both checks; a refusal scenario for a missing English summary was added. |
| 4 | D1's service-address sentence and D2's forwarding-provider disclosure never reached the page content, and the host's further processed data was reduced to "further automatically processed data". | Applied: the sentence, the mail-handling section with provider, role, location and basis, and the named data (time, page, referrer, browser and device) are in the use case, the design, the env list and the two e2e scenarios. |
| 5 | The proposal routed the README carrier to `publish-public-release`, but the change had no delta for it. | Applied: `specs/publish-public-release/publish-public-release.feature` added with one documentation scenario; the routing row and the use case's public-text rule were aligned. |
| 6 | The foreign-asset scan read only `index.html`, so a notice could load a third-party image. | Applied: the scan covers every `*.html` member, the notice CSP is asserted, and a refusal scenario was added; chapters 05 and 06 were corrected. |
| 7 | `navigate` + `read_text` cannot activate a link, so the scenarios would assert a page fetched by path rather than the link a reader clicks; the loopback server also 404s the extensionless public address. | Applied: the driver gains `click`; the loopback resolves `<path>` to `<path>.html`; the steps activate the footer link. |
| 8 | The visual-baseline helper hard-codes `lang="en"` and lives in the conversion capability's step file. | Applied: the helper moves to a shared module with a language parameter; links still count as labelled controls. |
| 9 | The reachability rule contradicted itself: "every page links both" versus "each notice links the other". | Applied: one rule everywhere — the converter footer links both, each notice links the other and the converter page; use case, arc42 01/02/03/08/10/12 and the change's arc42 note were aligned. |
| 10 | The design claimed both notices carry a standing date; the Impressum content had none. | Applied: only the Datenschutzerklärung carries it; design and every arc42/glossary place corrected. |
| 11 | "No script, no cookie, no banner" was claimed without an assertion. | Applied: the gate asserts the CSP meta, the absence of `<script` and of `on*=` handlers; both browser scenarios assert empty cookie and storage. |
| 12 | The browser evidence is produced neither by `verify.e2e_command` (which excludes `@browser`) nor by the per-push CI job, which ran one named conversion scenario. | Applied: the `browser` job gains two named steps; the design states which command produces the evidence. |
| 13 | The design-system extension was named but unspecified, and its approval covered only the converter screen. | Applied: the operator approved the inherited-shell direction; the design system records the amendment, the notice primitive, the per-document language and the captures. |
| 14 | D3's CC0 realization was false: `REUSE.toml` had no `web/**` override, so the two pages would have fallen under the aggregate MPL-2.0 while the page states CC0-1.0 for non-code assets. | Applied: an override for exactly the two notice pages; `index.html` stays under the aggregate, and D3's realization text says so. |
| 15 | `arc42.md` lacked the completion marker the arc42 contract requires. | Applied: `## Status` / `STATUS: DONE`. |
| 16 | **Carrier** was reused with a second meaning. | Applied: **Legal-notice carrier** added; the support statement's **Carrier** is unchanged. |
| 17 | The profile label "Impressum & Datenschutz" pointed at one page, the README carried a different set, and the proposal, design, scenario and chapter disagreed. | Applied: each carrier carries two labelled links, "Impressum" and "Datenschutz"; `/privacy` stays the Datenschutz page's address; all four artifacts aligned. |
| 18 | Nothing resolved a relative link against the output, so a notice could link a file the publication does not carry; "exactly eleven members" was asserted by no check. | Applied: the gate resolves every relative `href` in every `*.html` member and asserts the eleven named members. |
| 19 | The coverage table claimed an e2e/browser profile that `measure-coverage.sh` does not produce. | Applied: the row is deleted and the accepted gap is stated. |
| 20 | The notice scenarios asserted static text only; the rendered-state rule and the notice-to-notice links were unaccounted for. | Applied: each scenario asserts the site root's `h1` before activation and the notice's `h1` after; the remaining links are recorded as variants of the same two inventory actions. |
| 21 | "A second occurrence in any carrier is refused" overclaimed the check's scan set. | Applied: reworded to the profile and the README, the carriers the check reads. |
| 22 | Four persona-path lint advisories remain. | Recorded, not fixed: the links are correct for the document's durable home `givn/specs/<usecase-id>/`; the advisory is an artifact of the change directory's depth and disappears on archive. |

## Operator decision put at review

**Which visual direction the notice screens take.** Two rendered references were built and shown: the inherited site shell (header with the vector mark, surface panel, 720 px column, full footer) and a standalone document without the mark. Chosen on 2026-10-27: the inherited shell. Rejected: the standalone document, because it would add a second header and measure to the design system and break the visual continuity from the converter page for no gain the operator wanted. The standalone reference file was deleted with the decision.

**Not a review question, but a precondition of implementation.** The Impressum's literal contact block (name, employer, street, postal code, city), the employer's written consent to accept service mail, and the mail alias are operator-supplied. No placeholder is committed: the pages and the checks that refuse them land together once the block exists.

## Risk

The most likely implementation failure is the browser evidence: the driver gains a third action and the shared baseline helper changes a file the conversion scenarios already depend on, so a mistake there turns the conversion evidence red as well. Mitigation: the driver actions are added before the notice steps, the baseline helper's move is its own task with the conversion scenarios run unchanged, and the two notice scenarios are proven RED first.

## Sign-off

Every grilling question is answered or recorded as an accepted, named gap (#22). The hardening edits are applied. The residual gap is the operator's contact block, which blocks implementation by design rather than by an open question.

DESIGN-REVIEW: PASS
