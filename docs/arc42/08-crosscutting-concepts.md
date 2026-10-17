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

## Printed identification of a travel leg

A plan prints a travel row's crew position (`Pos`) and the flight's activity (`Activity`) in two separate columns. The row keeps both, and one pure rule reads them together: `printed_identification()` in the domain is used for the travel title and for the failure message that names an unmatched travel fragment, so the text a crew member reads and the text a failure reports cannot drift apart. A departure row and an arrival row belong to the same leg when both printed fields are equal, so a fragment printing a different position — or none while its partner prints one — has no compatible partner and the conversion refuses it instead of guessing. The `DH` marker needs no second mechanism: it reaches the title exactly as the plan prints it.

## Release determinism

Preparing the same Windows executable and version twice produces a byte-identical archive, so a third party can reproduce a release and its checksum is meaningful. The three nondeterministic inputs of `tar` are fixed — ordering (`--sort=name`), time (`--mtime=@0`) and ownership (`--owner=0 --group=0 --numeric-owner`) — and `gzip` is invoked with `-n`, so no name or timestamp is embedded.

## Change-log generation determinism

The generator renders the same history to the same bytes on any machine at any time, or the byte-comparing drift check would have no meaning. Three inputs are fixed: ordering (`--sort oldest`, so commits are ordered by history rather than by the order a checkout happened to expose), time (the section's date is the released revision's own committer date — the `v<version>` tag when it exists, otherwise `HEAD`, the revision the tag will point at — never the clock, which is why the renderer never uses the generator's own timestamp and why a section can be rendered before its tag exists), and host paths (no working-directory path, user name or runner name enters the text; the comparison link is a literal URL the renderer appends, built from the previous `v*` tag and the version, with no remote integration enabled).

## Change-log drift checking

The committed `CHANGELOG.md` is checked against a fresh render of the released revision, the same shape the third-party notices drift check already uses: the `changelog` CI job installs the pinned generator and runs `scripts/render-changelog.sh --version <newest v* tag> --check`, and a committed file that differs is refused by name of the version whose section differs. `--check` renders in memory and writes nothing. The release preparation runs the same check on the version being released before it stages anything, so a stale file cannot be published even when the CI job is still running as the tag is pushed. The pinned generator version, the capability allow-list and the marked machinery skip list are checked alongside it, so a stale file, a generator that is not the pinned version, an allow-list that has drifted from the corpus, and a skip list the configuration does not realize all fail loudly rather than silently changing what a reader sees.

The publication gate is the release-time counterpart of the landing gate. The `publish` job resolves every release-workflow check suite on the tagged revision, then waits (bounded) for the remaining check runs to conclude. It refuses a concluded non-success, a set with no successful check, no check evidence outside the release-workflow suites, an unreadable check-run source, or a check still running at the bound. Skipped and neutral conclusions are allowed; an unknown conclusion fails closed. The gate runs before any SBOM, attestation, draft or asset exists, so a refusal publishes nothing.

## What a reader sees

A released change is either a revision whose conventional-commit scope names a capability — the use-case ids present as directories under `givn/specs/` — or a dependency update (`chore(deps): …`), whose subject's words do not name the project's own tooling, and whose change touched a path a user receives. Every other revision is never listed, whatever its type: housekeeping other than a dependency update (`chore`, `docs`, `style`), `test`, `build`, `ci`, any scope outside the allow-list, including the machinery of building, packaging and publishing, a revision whose words name the project's own tooling, which the marked machinery skip list removes (as distinctive tooling phrases, so a subject that merely borrows the word for a crew member's own work stays) so a `fix(<capability>):` that only reports on a coverage report or a CI or release workflow never reaches a reader, and — whatever its words say — a revision that changed none of the marked paths a user receives, which the renderer resolves from the revision's own diff. Each listed entry states the release-note words the revision carries, never a rewritten summary, so the notes and the history agree, and the groups appear with the group a reader looks for first at the top (breaking changes, new capabilities, fixes, performance, reversals, dependency updates); a group whose entries were all filtered out does not appear. Identical subjects collapse to one entry, because one released change carried by many revisions is one entry. A version whose render would name no change a user receives is refused rather than published.

## Completeness refusal in the preparation command

The preparation command is all-or-nothing. It refuses, leaving its output untouched, when the Windows x86_64 executable is missing, when an executable is presented for a platform outside the supported set (including the previously offered Linux and macOS targets), when an executable cannot be read, when that version's set is already staged, or when the committed change log is not current for the version being released. It writes nothing until the whole set is valid, so an incomplete preparation publishes nothing and never replaces an existing prepared set.

## Supply-chain provenance and immutability

Every archive is published with build provenance and a bill of materials (`actions/attest@v4` over the archive, plus an `anchore/sbom-action@v0` SBOM), and the release itself is immutable: once public, its assets cannot be replaced or deleted and its version tag cannot be moved. The verify job reads the published release back and runs `gh attestation verify` on every archive, so the file a downloader holds is tied to the tagged revision and this project's automation. A correction is the next version, never a repair in place.

## Secret handling on a public repository

A public repository adds a credential surface the local private-data guard does not cover. Push protection blocks a leaked credential before it enters the history, and secret scanning raises an alert if one still lands; the existing private-data guard continues to block duty-plan data. Private duty plans and real identifying values never enter the public history or a public asset.

## Version, tag and archive-name consistency

The version is stated once and nowhere disagrees: the archive names, the release text and the version tag all carry the same bare version. The preparation command refuses a version with a leading `v`, so the archive name, the release text and the tag cannot diverge about what was released.

## Declared configuration and the applied state

The public home's intended configuration is the declaration under `infra/`; the
live configuration is what GitHub holds; the applied state is the record that
binds each declared object to its live identity. The state is not a copy of the
intended values and adoption never reconciles the two: a declared value that
differs from live is left for the first preview, so adoption cannot become an
unreviewed apply. The declaration is text and is reviewed like code; the state
is ciphertext and is reviewed for the fact that it is ciphertext, not for its
contents.

## All-or-none adoption

An adopted batch is bound or none is. The command imports into a staged copy of
the state and moves it into place only after every import and a refresh-only
verification succeed; a failure discards the staged state, so a partial batch
cannot be left behind and a crash cannot bind half a batch. This is structural,
not a rollback step, and it is why `tofu import` — which is per-object — is not
exposed directly to the maintainer. Every refusal names the object and the
reason — an unreadable object, one that does not exist live, an already-bound
address, an undeclared batch entry, a live object that changed, a state that
cannot be decrypted or written, a malformed batch line, or an invalid
declaration — and states that nothing was bound.

## State encryption at rest

OpenTofu's `encryption` block derives an AES-GCM key from the passphrase
(PBKDF2) and enforces encryption for both state and plan. The passphrase comes
from `TF_VAR_state_passphrase`, has no default and is never a repository secret;
the state file and its backup are committed as ciphertext. Two consequences are
accepted and recorded: losing the passphrase loses the state (re-adoption
rebuilds it), and no automated service can read the state — which is exactly
why the configuration drift check runs locally and the future publishing key is
not exposed to a workflow.

## Adoption is read-only against the live organization

`tofu import` and `tofu plan -refresh-only` issue reads; the adoption command
never runs `apply`. The suite asserts this at the HTTP boundary through the fake
GitHub API's request journal: the fake refuses every write-shaped request and
every GraphQL mutation, so the assertion is "no refused write and no mutation",
not "no POST". A
refusal that would need a write — creating a missing object, changing a value,
removing a legacy object — is reported as belonging to a configuration change
instead.
