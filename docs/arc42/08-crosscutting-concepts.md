# 8. Cross-cutting Concepts

## Complete-or-explicit-failure

Every conversion either produces a complete calendar or fails explicitly. The `Roster` aggregate gates publication: no side effect occurs until every required activity and field resolves, and a required-value contradiction or an unknown required code rejects the conversion. A cron-like "best effort" partial calendar is never produced, because a calendar that looks complete but omits a leg is worse than a visible failure.

## No-clobber publication

New calendars and diagnostics are written with `create_new`, under the plain name first and then an enumerated suffix when that name is occupied; the create is the write, so an existing calendar or report is never replaced, and the user's earlier outputs survive re-runs and a competing process alike. A name another process claims between the check and the create is enumerated past, so the run still publishes without replacing anything; a name that only looked free and cannot be written fails the publication, and that failure is reported instead of being hidden behind a later name. Enforced in the filesystem boundary of `apps/cli`, not in the domain, which stays pure.

## Explicit failure reporting

A failure carries a primary reason and is reported independently of diagnostic persistence. The diagnostic path is reported only after a successful write; when no new report can be saved, the message states that and never presents an older report as current. Exit status is zero only for a real publication.

## Source-snapshot-scoped identity and determinism

Component identity is derived from the source snapshot and the represented activity, so converting the identical input twice is deterministic (same identifiers) while a revised input yields separate identities. Output filename counters and human titles are never identity. DTSTAMP prefers the source's revision footer and falls back to the actual construction timestamp; revision metadata never supplies an activity date.

## Privacy

Real duty plans are read for analysis but never committed. Fixtures, golden calendars, and public examples are generated or anonymized, and privacy checks consider identifying combinations (dates, itineraries, trip identifiers) rather than names alone. No real plan or identifying output is sent to an external service.

## Classification and timing as pure rules

Source-code classification (all-day vs timed vs travel vs layover vs unknown) and UTC endpoint date resolution are pure functions over source-derived data. Keeping them free of I/O lets return-driven edge cases (adjacent-day qualifiers, the narrow overnight rollover, zero-length intervals) be specified and tested without a real PDF, and prevents the extraction dependency from leaking into the domain.

## Release determinism

Preparing the same five executables and version twice produces byte-identical archives, so a third party can reproduce a release and its checksum is meaningful. The three nondeterministic inputs of `tar` are fixed — ordering (`--sort=name`), time (`--mtime=@0`) and ownership (`--owner=0 --group=0 --numeric-owner`) — and `gzip` is invoked with `-n`, so no name or timestamp is embedded.

## Completeness refusal in the preparation command

The preparation command is all-or-nothing. It refuses, leaving its output untouched, when a supported platform's executable is missing, when an executable is presented for a platform outside the supported set, when an executable cannot be read, or when that version's set is already staged. It writes nothing until the whole set is valid, so an incomplete preparation publishes nothing and never replaces an existing prepared set.

## Supply-chain provenance and immutability

Every archive is published with build provenance and a bill of materials (`actions/attest@v4` over the archive, plus an `anchore/sbom-action@v0` SBOM), and the release itself is immutable: once public, its assets cannot be replaced or deleted and its version tag cannot be moved. The verify job reads the published release back and runs `gh attestation verify` on every archive, so the file a downloader holds is tied to the tagged revision and this project's automation. A correction is the next version, never a repair in place.

## Secret handling on a public repository

A public repository adds a credential surface the local private-data guard does not cover. Push protection blocks a leaked credential before it enters the history, and secret scanning raises an alert if one still lands; the existing private-data guard continues to block duty-plan data. Private duty plans and real identifying values never enter the public history or a public asset.

## Version, tag and archive-name consistency

The version is stated once and nowhere disagrees: the archive names, the release text and the version tag all carry the same bare version. The preparation command refuses a version with a leading `v`, so the archive name, the release text and the tag cannot diverge about what was released.
