# 1. Introduction and Goals

## Requirements overview

`duty2cal` (the deliverable of the change to export released roster calendars) is a local, offline command-line converter. A crew member supplies one published Released Roster duty-plan PDF; the converter writes one complete RFC 5545 calendar beside it under an unused name, or fails explicitly and attempts a numbered diagnostic. It never overwrites an existing calendar or report.

Top requirements, from the stakeholder perspective:

1. **Faithful conversion** — every required activity in a supported plan is represented, with correct all-day/timed distinction and correct UTC endpoints; nothing is silently omitted.
2. **Complete-or-explicit-failure** — a plan that cannot be represented completely is rejected with a visible failure and a diagnostic, never a partial calendar.
3. **No destruction** — an existing calendar or report is never replaced; new outputs use an unused enumerated name.
4. **Privacy** — private duty plans and identifying values never reach the public repository; fixtures and golden calendars are generated or anonymized.
5. **Reproducible public home** — the organization's settings, both repositories, the protection rule for the default branch and the published site's source are declared in the repository, and the objects that already exist on GitHub are bound to their declared counterparts without being created, modified or deleted, so the public home can be reviewed, diffed and rebuilt instead of remembered.
6. **Distributable release** — one self-contained Windows x86_64 executable, published from one public repository as a verifiable, immutable release whose notes state the changes a user receives, including dependency updates, drawn from the change log the repository keeps, licensed MPL-2.0 project-wide (weak file-level copyleft; App-Store distributable), with claims backed by evidence. Windows x86_64 is the only supported platform; other platforms are not supported, tested or distributed, and their source can be built with a Rust toolchain.

The authoritative behaviour specification lives in the merged corpus `givn/specs/export-duty-plan-calendar/`; the change to export released roster calendars holds the historical home of that delta at `givn/archive/export-released-roster-calendar/specs/`.

## Quality goals

| Priority | Quality attribute | Motivation |
|---|---|---|
| 1 | Correctness / completeness | A calendar that looks complete but omits or invents activities is worse than an explicit failure; the whole-plan completeness boundary enforces this. |
| 2 | Reliability (no data loss) | Existing calendars and reports must survive any conversion attempt, including concurrent ones. |
| 3 | Interoperability | The output must be a valid RFC 5545 file (CRLF, escaping, octet-aware folding) usable by real calendar clients. |
| 4 | Privacy | Duty plans carry identifying data; public artifacts must contain only generated or anonymized material. |
| 5 | Maintainability / portability | Clean Architecture layering plus one self-contained Windows x86_64 executable keep the converter verifiable and distributable. |
| 6 | Reproducibility of the public home | The public home's configuration must be reviewable, diffable and rebuildable from the repository, and the one-time adoption of objects that already exist must change nothing live; a record that can only be remembered in the web interface is the failure this goal prevents. |
| 7 | Release integrity / verifiability | A released version offers exactly one Windows x86_64 archive, byte-reproducible, checksummed, carrying build provenance and a bill of materials, and published immutably, so a downloader can trace the file in hand to the released revision and no published promise outruns its evidence. The release also states what changed: its notes are the newest section of the repository's change log, grouping the changes a user receives — capability changes and dependency updates — and refusing a version that would name none. |

## Stakeholders

| Role | Expectation |
|---|---|
| Crew member exporting their own duty plan (`crew-member--personal-calendar-owner`) | A usable personal calendar that faithfully represents their plan, with clear failures and no invented dates — and, on Windows x86_64, a downloadable executable that runs without installation, together with release notes that state what changed since the version they have. A crew member on any other platform reads that it is not supported or distributed and that the source can be built with a Rust toolchain. |
| Person identified in a duty plan (including its owner) | Their identifying information never enters the public repository or public test material. |
| Maintainer responsible for a public release (`project-maintainer--release-owner`) | A maintainable, verifiable, legally distributable converter whose public support claims match its evidence, and a public home whose release channel serves one verifiable, immutable Windows x86_64 archive per version and states, in the repository's change log, what each version changed. |
