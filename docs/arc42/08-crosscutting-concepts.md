# 8. Cross-cutting Concepts

## Complete-or-explicit-failure

Every conversion either produces a complete calendar or fails explicitly. The `Roster` aggregate gates publication: no side effect occurs until every required activity and field resolves, and a required-value contradiction or an unknown required code rejects the conversion. A cron-like "best effort" partial calendar is never produced, because a calendar that looks complete but omits a leg is worse than a visible failure.

## No-clobber publication

New calendars and diagnostics are written with `create_new`, under the plain name first and then an enumerated suffix when that name is occupied; the create is the write, so an existing calendar or report is never replaced, and the user's earlier outputs survive re-runs and a competing process alike. A name another process claims between the check and the create is enumerated past, so the run still publishes without replacing anything; a name that only looked free and cannot be written fails the publication, and that failure is reported instead of being hidden behind a later name. Enforced in the filesystem boundary of `apps/cli`, not in the domain, which stays pure.

## Explicit failure reporting

A failure carries a primary reason and is reported independently of diagnostic persistence. The diagnostic path is reported only after a successful write; when no new report can be saved, the message states that and never presents an older report as current. Exit status is zero only for a real publication. The browser realization keeps the same shape: the reason is visible, the calendar is not offered on failure, a diagnostic offer is attempted, and a failed offer never replaces the primary failure.

## Source-snapshot-scoped identity and determinism

Component identity is derived from the source snapshot and the represented activity, so converting the identical input twice is deterministic (same identifiers) while a revised input yields separate identities. Output filename counters and human titles are never identity. DTSTAMP prefers the source's revision footer and falls back to the actual construction timestamp; revision metadata never supplies an activity date.

## Privacy

Real duty plans are read for analysis but never committed. Fixtures, golden calendars, and public examples are generated or anonymized, and privacy checks consider identifying combinations (dates, itineraries, trip identifiers) rather than names alone. No real plan or identifying output is sent to an external service.

## Browser-local conversion and the no-network boundary

The browser page is a second driving adapter, not a second conversion. It
passes the selected file's bytes and name to the unchanged `ConvertRoster`
through `ffi/wasm`; the facade performs no filesystem, network or environment
I/O and uses a browser clock only for the existing fallback. The page makes no
request to another origin after it loads and loads no third-party asset. The
meta CSP is `default-src 'self'; connect-src 'none'; script-src 'self'
'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; object-src 'none';
base-uri 'none'; form-action 'none'`. The WASM bytes are inlined as base64 so
the generated loader never fetches the `.wasm` file; the CSP is not weakened
for it. The driver records every request and the `@e2e` scenario fails on any
other-origin request after load.

## Browser offer and browser-owned save

The page never writes a file and never uses the File System Access API. It
creates an object URL over a `Blob` and an `<a download="<name>">`; the browser
performs the save and owns the location and any collision suffix. The page
never claims a save it did not observe and never promises an unused numbered
name. Every `acceptFile` clears the prior offer and status first, so a failed
attempt withdraws an earlier calendar offer. A failed calendar offer is a
failure, never a ready state; a failed diagnostic offer leaves the primary
failure visible and says no diagnostic was offered. Object URLs are revoked when
replaced and on `pagehide`. No plan or derived data enters localStorage,
sessionStorage, IndexedDB, cookies, a history entry or a navigable URL.

## Capability gate, startup failure and the input bound

A classic deferred check tests `WebAssembly`, `WebAssembly.instantiate` and
ES-module support before the module is used; the picker and drop zone stay
hidden until the gate passes and the module initializes. A missing capability
or a module-load/instantiation failure renders the cannot-run treatment, offers
no picker and no calendar, and never falls back to a server or an upload. The
page refuses a plan larger than 20 MiB by name before reading it; the converter
still checks the family and content.

## Browser toolchain pinning

The browser test surface is pinned: Playwright and `@axe-core/playwright` in
`web/e2e/package-lock.json`, Node in `web/e2e/.nvmrc` after one LTS resolution,
Chromium through the Playwright package, `wasm-pack` in
`web/.wasm-pack-version`. These are test/build dependencies, not runtime
assets. The Rust resolution is in `Cargo.lock`, and `wasm-bindgen`, `js-sys`
and `getrandom` are target-scoped so native builds stay clean. A setup without
the pinned tools fails loudly rather than skipping the browser scenarios.

## Classification and timing as pure rules

Source-code classification (all-day vs timed vs travel vs layover vs unknown) and UTC endpoint date resolution are pure functions over source-derived data. Keeping them free of I/O lets return-driven edge cases (adjacent-day qualifiers, the narrow overnight rollover, zero-length intervals) be specified and tested without a real PDF, and prevents the extraction dependency from leaking into the domain.

## Printed identification of a travel leg

A plan prints a travel row's crew position (`Pos`) and the flight's activity (`Activity`) in two separate columns. The row keeps both, and one pure rule reads them together: `printed_identification()` in the domain is used for the travel title and for the failure message that names an unmatched travel fragment, so the text a crew member reads and the text a failure reports cannot drift apart. A departure row and an arrival row belong to the same leg when both printed fields are equal, so a fragment printing a different position — or none while its partner prints one — has no compatible partner and the conversion refuses it instead of guessing. The `DH` marker needs no second mechanism: it reaches the title exactly as the plan prints it.

The same title also carries the plan's printed report time when the leg has one single value: `DutyEntry::summary()` appends ` Report <value> LT` after the route, giving `LU 742 Tychon-Mareum Report 07:30 LT`. Exactly one space precedes `Report`, one space separates it from the value, and `LT` is literal text stating that the printed value is local time; it never becomes a UTC instant or an activity date, and no timing code reads it. The suffix is omitted when the plan supplies no report time, so the title stays route-only, and it is also omitted when the departure and arrival fragments print different report times, because the leg then has no single briefing time; the description keeps both fragment-labelled `Report (LT)` values in that case. Equal values from both fragments collapse to one value before composition, so they produce exactly one suffix. The suffix belongs to the one title rule in the domain, not to the ICS adapter, so the text `SUMMARY` publishes and the text the golden scenario compares cannot drift apart. Other activity kinds — all-day absences, availability windows, non-flight duties and layovers — keep their current titles.

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
unreviewed apply. An object the declaration names and live lacks is different:
there is nothing to adopt, so the preview names a creation, the apply creates
it, and the state records the new binding — the organization profile repository
and its two files enter this way. The declaration resolves the reviewed profile
source through `variable "org_source_dir"` (default `../org` beside `infra/`),
so it can be planned from `infra/`, from an adoption staging copy or from a
test workspace. The adoption command stages the declaration without
`org_profile.tf`, and the setup proof and security base workspaces omit it,
because the profile repository is created, never adopted. The declaration is
text and is reviewed like code; the state is ciphertext and is reviewed for the
fact that it is ciphertext, not for its contents.

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

## Declared security settings on the public repository

A public repository adds a credential surface, so the settings that guard it are
declared values, not remembered ones. The monorepo's `security_and_analysis`
supplies secret scanning and push protection, and the two Dependabot settings
use their own resources because the repository's `vulnerability_alerts` field is
deprecated in the pinned provider. The settings that are already on are bound to
the live configuration before the change is applied, so the preview reports only
real differences and a later manual change to either is a difference the local
check can name. Settings the provider cannot supply for a public repository —
advanced security — are never declared, and settings the provider cannot express
at all, such as private vulnerability reporting, stay manual and are recorded as
a gap.

## Public organization copy

The organization profile and the organization's default security policy are
public copy, not product output. They live in `org/` in the monorepo, are
reviewed like code, are licensed CC0-1.0, and are published into the public
profile repository by the declaration; the profile repository is output and is
never edited by hand. The copy states only what the repository keeps: the
converter and its crew duty plan input, Windows x86_64 as the only downloadable
archive platform, the browser conversion route only once the Site publication
makes it reachable (that copy belongs to `publish-public-website`), no hosted
converter or upload, no third-party provider name. The profile must not describe
the built-but-unpublished page as live. It links the stable
latest-release address, the source and the site repositories, attributes the
third-party Unicode data to Unicode-3.0 with an absolute licence link, and
states explicitly that the project is not affiliated with any duty-plan
provider or airline. The two published terms are the **profile content**
(`profile/README.md`) and the **default security policy** (`SECURITY.md`); the
reviewed pair is the **reviewed public copy**.

The page is self-contained: every link is absolute and there are no images,
badges, statistics widgets or other third-party resources. A reader opening the
profile therefore issues no request beyond GitHub's own, which is the same
promise the converter makes at runtime. The content-invariant scenario compares
the published bytes with the reviewed `org/` source and asserts the positive
claims; the profile is not trusted because a reviewer once looked at it, and the
absence of the excluded provider name is a review control rather than a stored
deny-list.

## Default community health files

GitHub uses `SECURITY.md` at the root of a public `.github` repository as the
organization's default security policy for an organization repository that has
none of its own. The profile repository publishes that file, so a repository
without a policy — including the Pages repository — inherits a confidential
reporting route. The file is a short organization-wide policy, not a copy of the
monorepo's longer policy: it names the private advisory route, forbids attaching
a real plan or a converted calendar, states that only the latest published
release is supported, and describes how a report is handled.

The route exists only when the owner enables private vulnerability reporting on
`duty2cal/duty2cal` by hand before publication, because the pinned provider has
no resource for the setting; it remains outside the declaration, the applied
state and the drift check (chapter 7, R-030). The suite asserts that the policy
names the route; it cannot prove the live setting.

The fallback itself is GitHub's behaviour, not the project's code. The
capability proves that the file exists at the required path and carries the
route; it does not simulate GitHub's renderer. `CONTRIBUTING.md`,
`CODE_OF_CONDUCT.md` and the issue and pull-request templates remain in the
monorepo and are not published into the profile repository by this change.

## Declared objects and manual operator steps

The declaration covers what the pinned provider can express. Two public-home
facts do not fit that boundary and are recorded rather than forced into it:
the organization's profile fields stay empty (they are settings, not profile
copy), and the pinned repositories are set once in the web interface because the
provider exposes no resource for them. A manual step has no preview, no state
binding and no drift report; the deployment chapter records the step and the
risk chapter records the consequence. This is preferable to inventing a second
write path with its own credential, which would contradict the one-declaration,
one-writer rule.

## Apply semantics: the saved plan, the preflight and partial reporting

A configuration change is a saved plan applied unchanged (the saved plan is targeted at the changed objects — a repository, or the profile repository and its two files; a full-declaration plan feeds the unadopted-setting guard where one is needed): `tofu plan -out`
produces the artifact the operator reads, and `tofu apply` of that file performs
exactly those actions. OpenTofu refuses a plan whose state has moved on, so the
applied revision is the previewed one, and the plan file is encrypted and
git-ignored so no transient artifact enters the repository. A declaration edit
made after the preview is not carried; it takes effect only after a new preview.

A saved plan does not re-read the live configuration at apply time, so the
routine runs a read-only `tofu plan -refresh-only -detailed-exitcode -json`
immediately before the apply and refuses a non-no-op change by address. The
JSON report is authoritative, not the exit status — OpenTofu 1.13.1 can return
exit 2 for a no-op — and an incomplete report is refused. The preview itself
refuses a plan that would create a setting which already exists live and is
unbound, naming it. A live edit made during the preflight is the residual
window (R-031).

A failure is reported the way the declaration's other operations report one. A
refusal before the first write — an unreadable setting, a wrong passphrase, an
authority that cannot perform the change — leaves the live configuration and the
state unchanged and names the object. A failure after an earlier resource was
applied stops the run, names the applied resources and the failed address, and
leaves the state recording only what succeeded; the routine writes
`terraform.tfstate.stale` beside that state and does not commit it. While the
marker exists the preview and the apply refuse and name the stale state, and a
green apply removes it, so the revision is recorded only when every change
succeeded.

The profile scenario's test tree is the inherited pre-security base tree:
priming adopts both existing batches and runs the base declaration's real
apply, which performs the security change's scanning write. The scenario takes
its journal offset after that apply, so only the three profile creations are
asserted, and the base workspaces omit `org_profile.tf`.

## Adoption is read-only against the live organization

`tofu import` and `tofu plan -refresh-only` issue reads; the adoption command
never runs `apply`. The suite asserts this at the HTTP boundary through the fake
GitHub API's request journal: the fake refuses every write-shaped request and
every GraphQL mutation, so the assertion is "no refused write and no mutation",
not "no POST". A
refusal that would need a write — creating a missing object, changing a value,
removing a legacy object — is reported as belonging to a configuration change
instead.

## Panic behavior and the conversion diagnostic

The release profile builds with `panic = "abort"`, so a bug-induced panic
prints the standard Rust panic message on stderr and ends the process without
unwinding or a backtrace. This does not change the promise for expected
failures: a conversion that cannot be represented still returns the typed
failure and attempts a numbered diagnostic naming the source PDF and the
reason, and when no report can be saved the primary failure remains visible.
The product uses no `unwrap()`/`expect()` outside test code. The host lever
build is run on an unsupported source by the reduction scenario; the Windows
executable is started by the release workflow's Windows smoke job.

## Measured size attribution

The size attribution is evidence, not a promise, and it is built to make
substitution impossible. A measurement resolves a named revision into a
temporary worktree and builds it twice: the shipped artifact with the committed
build configuration, and a second build of the same revision with `strip`
disabled for that build only, in a separate target directory. The first gives
the executable's file size, digest and section sizes; the second gives the
crate and function split and the `.text` total of that build. The two
numbers are separate fields filled by separate reads, and the human report
labels them separately, because the attribution describes a rebuild with names
while the distributed executable is stripped.

The report's status is `complete`, `incomplete` or `unreadable`. A failed build
or a build that keeps no names produces no attribution and states what could
not be read; a stripped executable whose file size cannot be read leaves the
report incomplete, states that the size is unavailable, and never presents the
split's total as the file size. No number is inferred from the release archive,
and a build for the host machine is not accepted as evidence about the
supported platform: the supplied artifact is checked for a PE header and the
x86-64 machine, and a host build or an archive is refused by name.

The measurement is evidence, not a gate. The CI step that produces it always
publishes the report (or a note that none was written) and exits zero, so an
unreadable report never refuses a revision, the Windows smoke or a release.
The report is what a reviewer reads; the automation does not decide on size.

The generator's numbers are estimates of the `.text` section, not a byte-exact
file size: `cargo-bloat`'s own documentation calls them guesswork. The report
repeats that caveat, and the attribution total stays a separate field from the
stripped executable's byte count.

The report names the requested revision, the resolved revision, the platform,
the toolchain, the pinned generator's version and the digests of the build
configuration, so a reader can judge whether it is reproducible; the generator
is pinned for the same reason the change-log and notices generators are. A
measurement is produced for every pushed revision by the Windows cross-build
job and published beside that run, and the same command is available to the
maintainer for any named revision.



