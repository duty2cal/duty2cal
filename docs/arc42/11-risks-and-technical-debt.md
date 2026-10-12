# 11. Risks and Technical Debt

## Risks

| ID | Risk | Probability | Impact | Mitigation |
|---|---|---|---|---|
| R-001 | The pinned extraction crate (`pdfplumber = 0.4.1`) exposes different positioned-word/table APIs than expected, so extraction code must be adapted. | Medium | Medium | Build re-proved on rustc 1.98.1; the extraction task adapts to the documented 0.4.1 API before conversion logic is written. |
| R-002 | The Windows GNU cross-build and the executable's absence of companion runtime imports are unproven. | Medium | High | Release-evidence task builds the Windows target and inspects the executable's runtime imports; no release claim without it. |
| R-003 | `./measure-coverage.sh` and `./merge-coverages.sh` are referenced by `givn/commands.yaml` but absent (the coverage addon gates archive). | High | Medium | Setup task creates the coverage scripts around `cargo llvm-cov -p duty2cal` and proves non-zero production coverage. |
| R-004 | The expert-provided SB/SBX/RE semantics and the qualifier boundary rule are not yet validated against a real dated duty-plan row. | Medium | Medium | Generated fixtures for tests; the local real-source validation obligation is carried; generated layouts are not presented as producer evidence. |
| R-005 | Private data leaks into a public artifact. | Low | High | Generated/anonymised fixtures only; review gate; a calendar-syntax-aware guard replaces the first draft's false-positive PII regex. |
| R-006 | The parked ideation questions reopen: source-derived publication rights (Q5), formal FZ expansion/provenance (Q13). | Low | Low | Both out of scope for this change (generated fixtures; no formal FZ claim). |
| R-007 | The GPL-3.0-or-later CLI cannot ship in the iOS App Store without a separate licence grant by the copyright holder (ADR-0001 consequence). | Medium | Medium | Phase 3 decision: ship the iOS UI under a separate grant, or place the iOS UI under MPL-2.0; no Phase-1/2 impact. |
| R-008 | `archunit` 0.0.2 is pre-1.0 and syntax-based (file granularity, no macro expansion), so some intra-crate layer violations are undetected (ADR-0002 consequence). | Medium | Low | The clippy `disallowed-types`/`disallowed-methods` policy covers I/O/clock/environment; review conventions cover the rest; revisit when archunit stabilises. |

## Technical debt

| ID | Item | Impact | Plan to address |
|---|---|---|---|
| TD-001 | Fixtures and golden calendars are generated, so the support claim rests on generated layouts rather than observed producer variation. | A producer layout variation could fail in the field. | Local (uncommitted) validation against real plans; expand generated variations when evidence appears; a real second/third corpus is gated on documented permission (Q5). |
| TD-002 | The user's first draft oracle had five overlong lines, an incorrect future weekday, and untraceable values. | Copying it verbatim would encode contradictory evidence. | The golden calendars are regenerated in this change. |
| TD-003 | Private-source validation cannot be committed. | Reproducibility of real-source checks is manual. | Keep real plans local/ignored; rely on generated fixtures for committed evidence. |
| TD-004 | `archunit` and `pdfplumber` are pre-1.0 pinned dependencies. | Upstream minor releases may break the pinned API. | Pin exactly; Dependabot ignores their minor/major updates (deliberate bumps). |
