# 11. Risks and Technical Debt

## Risks

| ID | Risk | Probability | Impact | Mitigation |
|---|---|---|---|---|
| R-001 | The exact-pinned extraction crate (`pdfplumber = 0.1.0`) does not build or expose the needed positioned-word/table APIs on the resolved toolchain; its ideation read found that the draft's snippets do not match its 0.1.0 API, and it declares edition 2024 / Rust 1.85. | Medium | High | Setup task compiles and exercises extraction against a generated fixture before any conversion logic is written; a feasibility conflict is reported to the user rather than substituted (the pin is a supplied constraint). |
| R-002 | The Windows GNU cross-build and the executable's absence of companion runtime imports are unproven. | Medium | High | Release-evidence task builds the Windows target and inspects the executable's runtime imports; no release claim without it. |
| R-003 | `./measure-coverage.sh` and `./merge-coverages.sh` are referenced by `givn/commands.yaml` but absent (the coverage addon gates archive); `./run-tests.sh` now exists and `verify.command` runs it. | High | Medium | Setup task creates the coverage scripts around `cargo llvm-cov` and proves non-zero production coverage. |
| R-004 | Expert-provided SB/SBX/RE semantics and the qualifier boundary rule are not yet validated against a real dated duty-plan row. | Medium | Medium | Keep generated fixtures for tests and carry the local real-source validation obligation; do not present generated layouts as producer evidence. |
| R-005 | Private data leaks into a public artifact (names, itineraries, or identifying combinations). | Low | High | Generated/anonymized fixtures only; review gate; the first draft's PII regex false-positive on `DTSTAMP` must be replaced by a calendar-syntax-aware guard. |
| R-006 | The two parked ideation questions reopen and block release evidence: source-derived publication rights (Q5) and formal FZ expansion/provenance (Q13). | Low | Low | Both are out of scope for this change; only a future source-derived artifact or a formal-claim need would reopen them. |

## Technical debt

| ID | Item | Impact | Plan to address |
|---|---|---|---|
| TD-001 | Fixtures and golden calendars are generated, so the support claim rests on generated layouts rather than observed producer variation. | A producer layout variation could fail in the field. | Local (uncommitted) validation against real plans; expand generated variations when evidence appears. |
| TD-002 | The user's first draft oracle had five overlong lines, an incorrect future weekday, and untraceable values. | Copying it verbatim would encode contradictory evidence. | Do not reuse the draft oracle; the golden calendars are regenerated in this change. |
| TD-003 | Private-source validation cannot be committed. | Reproducibility of real-source checks is manual. | Keep real plans local/ignored; rely on generated fixtures for the committed evidence. |
