# 11. Risks and Technical Debt

## Risks

| ID | Risk | Probability | Impact | Mitigation |
|---|---|---|---|---|
| R-001 | The pinned extraction crate (`pdfplumber = 0.4.1`) exposes different positioned-word/table APIs than expected, so extraction code must be adapted. | Medium | Medium | Build re-proved on rustc 1.98.1; the extraction task adapts to the documented 0.4.1 API before conversion logic is written. |
| R-002 | The Windows GNU cross-build and the executable's absence of companion runtime imports are unproven. | Medium | High | Release-evidence task builds the Windows target and inspects the executable's runtime imports; no release claim without it. |
| R-003 | **Closed**: `./measure-coverage.sh` and `./merge-coverages.sh` exist at the repository root, are wired into `givn/commands.yaml` (`measure_command`, `merge_command`), and measure the whole workspace with `cargo llvm-cov --workspace`, as design.md records. | Low | Low | None open: the scripts run and the merged report feeds the README badges. |
| R-004 | The expert-provided SB/SBX/RE semantics and the qualifier boundary rule are not yet validated against a real dated duty-plan row. | Medium | Medium | Generated fixtures for tests; the local real-source validation obligation is carried; generated layouts are not presented as producer evidence. |
| R-005 | Private data leaks into a public artifact. | Low | High | Generated/anonymised fixtures only; review gate; a calendar-syntax-aware guard replaces the first draft's false-positive PII regex. |
| R-006 | The parked ideation questions reopen: source-derived publication rights (Q5), formal FZ expansion/provenance (Q13). | Low | Low | Both out of scope for this change (generated fixtures; no formal FZ claim). |
| R-007 | MPL-2.0 is file-level copyleft: a redistributor of a modified product must publish the modified MPL files, but nothing protects the rest of a larger proprietary or store-distributed work (ADR-0001 consequence). | Low | Low | Accepted — this is the price of App-Store distribution for the future iOS UI. Revisit only if strong copyleft becomes a requirement, which would need a CLA. |
| R-008 | `archunit` 0.0.2 is pre-1.0 and syntax-based (file granularity, no macro expansion), so some intra-crate layer violations are undetected (ADR-0002 consequence). | Medium | Low | The clippy `disallowed-types`/`disallowed-methods` policy covers I/O/clock/environment; review conventions cover the rest; revisit when archunit stabilises. |
| R-009 | The release workflow's correctness is provable only on the host. | Medium | High | `rhysd/actionlint:1.7.12` covers schema, expressions and known action inputs before the push; the rest is proven by the first tagged run, recorded as evidence. A failed `build` job leaves no release at all. |
| R-010 | The macOS archives are unsigned, so a browser download is quarantined and needs a one-time approval. | High | Low | The README documents the approval step before the downloader meets the dialog; removing it needs a paid Apple Developer account (out of scope, D4). |
| R-011 | The release path's actions are pinned by mutable major tags (`actions/checkout@v7`, `actions/upload-artifact@v7`, `actions/download-artifact@v8`, `actions/attest@v4`, `anchore/sbom-action@v0`), so a moved tag could inject code. | Low | High | The release is immutable and attested, so a substitution is detectable after the fact; only `anchore/sbom-action` and the attest action are third-party in the path. Follow-up: adopt commit-SHA pins repository-wide in one change, with Dependabot updating them. |
| R-012 | A published release is immutable, so a botched tag or a broken archive cannot be repaired in place and a used tag is never reusable. | Medium | Medium | Accepted deliberately as the price of a verifiable release (D3): a correction is the next version, never a repair in place. |
| R-013 | The pinned `pdfplumber` was proven only on Linux/glibc before this change. | Medium | Medium | The build matrix now proves musl, Linux arm64 and both macOS targets, and the preparation refuses an incomplete set, so an unbuildable platform blocks the release rather than shipping quietly. |

## Technical debt

| ID | Item | Impact | Plan to address |
|---|---|---|---|
| TD-001 | Fixtures and golden calendars are generated, so the support claim rests on generated layouts rather than observed producer variation. | A producer layout variation could fail in the field. | Local (uncommitted) validation against real plans; expand generated variations when evidence appears; a real second/third corpus is gated on documented permission (Q5). |
| TD-002 | The user's first draft oracle had five overlong lines, an incorrect future weekday, and untraceable values. | Copying it verbatim would encode contradictory evidence. | The golden calendars are regenerated in this change. |
| TD-003 | Private-source validation cannot be committed. | Reproducibility of real-source checks is manual. | Keep real plans local/ignored; rely on generated fixtures for committed evidence. |
| TD-004 | `archunit` and `pdfplumber` are pre-1.0 pinned dependencies. | Upstream minor releases may break the pinned API. | Pin exactly; Dependabot ignores their minor/major updates (deliberate bumps). |
| TD-005 | The release path's actions are pinned by mutable major tags rather than commit SHAs. | A moved tag could inject code into the release path; the stronger reference is not yet used. | Adopt commit-SHA pins repository-wide in one change, with Dependabot updating them (R-011). |
| TD-006 | The macOS archives are unsigned; the one-time download approval is documented instead of signing and notarising. | A Mac download from a browser is quarantined, and a reader may mistake the warning for a defect. | Buy an Apple Developer account and notarise; a separate decision with its own cost (D4). |
