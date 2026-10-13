# Design review: separate-position-marker-and-flight-number

Run after `design.md`, before `tasks.md`. The grilling phase ran in a fresh context: a subagent that wrote none of these artifacts read the proposal, the delta, the design, the arc42 assessment, the permanent use case and feature, all twelve arc42 chapters, and the implementing code (`crates/duty2cal-core/src/application/mod.rs`, `crates/duty2cal-core/src/domain/mod.rs`, the step definitions, the fixture builder, the expected calendar), and ran the specs, design-review and arc42 instructions plus `givn lint`. Two of its questions went to the operator; the rest are recorded with the answer this review settled on.

## Questions and dispositions

| # | Question | Disposition |
|---|---|---|
| R1 (grilling Q1) | How far does the vocabulary correction reach — the use-case rule only, also the permanent deadhead scenario's step text, or also the code symbols? | **Answered by the operator**: the use-case rule is reworded *and* the code symbols are renamed. The permanent deadhead scenario's Gherkin step text is deliberately left alone: a step-text reword needs a `@givn.modified` scenario whose old step stays registered until archive (both texts must pass side by side), and leaves an obsolete registration to delete afterwards — poor value for a noun that names a value inside a step whose wording this change does not otherwise touch. The word therefore survives in that one step and its matched parameter, and nowhere else; recorded here so the operator can reverse it cheaply. |
| R2 (grilling Q2) | Should the delta pin "whatever the position column prints is kept verbatim" and leading zeroes on an operating leg with dedicated scenarios? | **No** — input variation of the same rule. The join has no marker whitelist and no digit reformatting, and the permanent deadhead scenario (`DH 007 ABC-DEF`) pins leading zeroes. |
| R3 (grilling Q3) | How should `tasks.md` frame the delta scenarios that already pass before the production change? | **Regression guards.** Only the title scenario, the failure-message scenario and the pairing scenario can fail RED against the production change; the "no position" and "non-flight activity" scenarios already pass and exist to keep the rule from over-reaching. The task list states exactly which is which and never claims three behaviour changes. |
| R4 (grilling Q4) | Is a new terminal transcript owed for this change? | **No.** The inventory action is unchanged, so no `@e2e` scenario is added and the existing transcript (which names the published path and the exit status) stays valid; the rendering change is visible only inside the `.ics`, which the golden-oracle scenario pins. |
| R5 | Should the model keep one pre-joined string with a new name, or keep the plan's two printed fields apart? | **Answered by the operator**: keep the two printed fields apart and let the title join them. The consequence the option named — pairing now compares both printed fields — is accepted and pinned by a new scenario, and recorded as TD-010. |

## Findings

| # | Branch | Finding | Where it landed |
|---|---|---|---|
| F1 | Scope | No issue. Each proposal claim maps to a delta scenario, and the claims covered by permanent scenarios (positioning travel unchanged, `LU900` not split) need no delta. | — |
| F2 | Tech choices | The design's evidence claims were true of the code, but the use-case traceability table cited the failure-message scenario as evidence for the deadhead rule. | Fixed while rewriting `design.md`; the traceability table now cites the title scenario for the deadhead rule and the failure-message scenario for the message. |
| F3 | Tech choices | The claim that every split leg in the operator's plans prints its position on both rows is a read of uncommitted local plans. | Recorded in `design.md` as a local-corpus observation with a note that those plans are never committed, and re-checked by `scripts/validate-local-corpus.sh` during implementation. |
| F4 | Missing scenarios | No delta scenario pinned the pairing consequence of keeping the two fields apart. | Added: "A crew position printed on one travel fragment only leaves it without a partner", with TD-010 and the chapter-08 concept recording why refusing beats guessing. |
| F5 | Testability | Three of the five delta scenarios are RED-capable against the production change; two already pass. | Recorded in R3 and carried into `tasks.md` as regression guards. |
| F6 | E2E fidelity | No issue. The inventory row matches the design's Interaction Coverage Matrix and the permanent `@e2e` scenario; `verify.e2e_command` is configured. | — |
| F7 | Persona disposition | No issue. Both Personas are confirmed; the proposal and the design record a lens and relevance for each, and Gherkin uses the domain actor. | — |
| F8 | Decision ledger | No issue. The proposal records no `D<n>`, and `design.md` says so. The two operator decisions this review produced are recorded at the top of `design.md`. | — |
| F9 | Visual contract / interface classification | No issue. `design.md` carries the contract with `- Interface: terminal`; the real interface is the built binary driven as a subprocess, `givn/commands.yaml` declares a CLI product, and no HTML/CSS/JS is rendered, so the Web-UI fail-closed rule cannot apply. No mandatory check is deferred. | — |
| F10 | ADR qualification | No issue. The verdict is NOT_QUALIFIED with the existing-ADR check performed and exactly one canonical destination (`design.md`). | The verdict was rewritten onto the two-field decision in `arc42.md`. |
| F11 | arc42 documentation | No issue. An independent 12-row derivation gives Yes on rows 5, 8, 10, 11 and 12 — the review's own derivation found row 8 and row 11, which the earlier assessment had as No because the model decision did not exist yet. Every chapter file was opened; chapter 09 correctly holds no MADR. | `arc42.md` rows 5, 8, 10, 11 and 12 rewritten; chapters 05, 08, 10, 11 and 12 updated. |
| F12 | Ubiquitous language | The glossary had dropped "designator" while the use-case rule still depended on it, and the code symbols named a helper that no longer exists after R5. | The rule is reworded (see the change's edits to `givn/specs/export-duty-plan-calendar/usecase.md`); the glossary maps **Crew position** and **Flight number** to the fields that hold them and corrects **Deadhead duty**'s symbol. |
| F13 | Risk | The expected calendar is regenerated by running the converter, so the oracle could bake in whatever the new code emits and could never be RED. | `design.md` requires a surgical regeneration: only the travel `SUMMARY` lines change, each gaining exactly one space, with every UID, instant, location and non-travel title byte-identical; the delta's own scenarios are the independent RED proof. Carried into `tasks.md` as an explicit acceptance check. |

## Hardening applied

- `design.md` rewritten around the two-field model: the single join rule, classification reading the activity cell, pairing on the printed pair with its accepted consequence, the deleted deadhead special case, the corrected traceability table, and the surgical golden-regeneration requirement.
- Delta spec: the pairing scenario added; the five scenarios re-read against the proposal's claims.
- `arc42.md` rows 5, 8, 10, 11 and 12 rewritten, and chapters 05, 08, 10, 11 and 12 updated.
- `givn lint --change separate-position-marker-and-flight-number` → the delta is well formed; the reported findings are the permanent corpus's pre-existing advisories.

No `@e2e` tag was removed or added. No mandatory check was deferred.

DESIGN-REVIEW: PASS
