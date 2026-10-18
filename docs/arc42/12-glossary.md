# 12. Glossary

Confirmed vocabulary promoted by the user's Handoff on **2026-10-12**, plus
five terms added by the change to export released roster calendars
(export-released-roster-calendar) and sixteen release-channel terms added by the
change to publish a public release (publish-public-release), plus four
change-log terms added by the change to generate release notes
(generate-release-notes), plus two printed-field terms added by the change to
separate the printed crew position from the flight number
(separate-position-marker-and-flight-number), plus one dependency-update term
added by the change to announce dependency updates
(announce-dependency-updates), plus six infrastructure terms added by the
public-home infrastructure ideation Handoff (public-home-infrastructure), plus
one security term added by the change to enable repository security
(enable-repository-security), plus one report-time term added by the change to
include the report time in the travel title
(include-report-time-in-travel-title).
Source: [Duty-plan calendar conversion](../ideation/duty-plan-calendar-conversion.md),
[introduction/collision records](../../givn/ideation/duty-plan-calendar-conversion/domain-terms.md),
[approved contract](../../givn/ideation/duty-plan-calendar-conversion/use-cases/export-duty-plan-calendar.md),
and [public-home infrastructure](../ideation/public-home-infrastructure.md) with its
[collision records](../../givn/ideation/public-home-infrastructure/domain-terms.md).
This is vocabulary/required behavior, not an implemented enforcement claim.

## Ubiquitous language

| Term | Definition | Contract use | Code symbol | Contrast / anti-terms |
|---|---|---|---|---|
| Additional vacation day | FZ roster activity denoting an additional vacation-related day off. | All-day; source times ignored. | `DutyEntry::AllDay`, `ALL_DAY_CODES` (`crates/duty2cal-core/src/application/mod.rs`) | Distinct from U; no verified entitlement/quota or acronym expansion. |
| Adoption | The one-time act of placing an already-existing GitHub object under the declared configuration instead of creating it. | The first-time path an apply depends on; a batch is adopted or none is. | `scripts/adopt-public-home.sh` and `tofu import`; `infra/adoption-batch.txt`; the encrypted `infra/terraform.tfstate` | Not an apply; not creation; not the prior topic's contract adoption. |
| Availability duty | Bounded standby or reserve window the crew member must be reachable in (SB, SBX, RE). | One timed availability activity with its own UTC window and station. | `DutyEntry::Availability`, `AVAILABILITY_CODES` (`crates/duty2cal-core/src/application/mod.rs`) | Not an all-day day off, not a flight, not a non-flight duty. |
| Build provenance | A signed attestation tying a published archive to the tagged revision and the automation that built it. | Published with every archive; a downloader checks it with `gh attestation verify`. | `actions/attest@v4` (`.github/workflows/release.yml`) | Not the checksum, which covers bytes only; not the bill of materials. |
| Capability allow-list | The set of released scopes — the use-case ids present as directories under `givn/specs/` — that a revision's scope must name to be a released change. | Materialised in `cliff.toml` and checked against the corpus by the renderer, which refuses any difference. | `givn/specs/`; `cliff.toml` (scope pattern) | Not every kebab-case scope: `fix(ci)` is shaped like `fix(publish-public-release)` but names no capability. |
| Change log | The repository file (`CHANGELOG.md`) whose newest section is the version being released and whose earlier sections stay as they were, generated from the repository's history. | Its newest section is the release text, and the committed file is drift-checked against a fresh render of the released revision. | `CHANGELOG.md`; `scripts/render-changelog.sh` | Not the release page alone; not a hand-maintained file. |
| Check run | One check that reports on a commit, with a status (`queued`, `in_progress`, `completed`) and a conclusion when complete (`success`, `skipped`, `neutral`, `failure`, …). | The publication gate classifies every check run on the tagged revision outside the release-workflow suites. | the check-runs endpoint consumed by `scripts/await-green-checks.sh` | Not a check suite; not a workflow run. |
| Check suite | The GitHub grouping that owns a set of check runs, usually one per workflow run. | The gate excludes every suite the release workflow created for the tagged revision, so the release's own checks cannot gate themselves. | `check_suite.id` in the check-runs response; `.github/workflows/release.yml` runs | Not a check run; not a workflow name. |
| Checksum | A SHA-256 digest covering one archive, published beside it and naming the file it covers. | A downloader verifies what they received before extracting it. | `sha256sum`, `duty2cal-<version>-<platform>.tar.gz.sha256` | Not the archive's size; not the release attestation. |
| Configuration change | A reviewed revision of the declared configuration that reaches GitHub through an apply. | Previewed as a saved plan and applied unchanged behind a read-only refresh-only preflight; a refusal before the first write leaves live and the state unchanged; a partial failure names the applied settings and the failed object and is marked stale beside the state, not recorded as applied; a stale marker refuses the preview and the apply until a green apply removes it. | `infra/` revision; `tofu plan -out` and `tofu apply <plan>`; the encrypted `infra/terraform.tfstate`; `infra/terraform.tfstate.stale` | Not a product change; not a release. |
| Configuration drift | A difference between the declared configuration and what the organization currently holds. | A report names every difference it can compare and marks unreadable fields unchecked. | configuration drift check (`tofu plan -detailed-exitcode`, planned) | Not the **Drift check**, which compares a committed change log with a fresh render. |
| Consulate visa duty | Non-flight DTUSA duty for a consulate visit to obtain a USA visa. | Own printed UTC interval. | `DutyEntry::NonFlightDuty`, `NON_FLIGHT_CODES` (`application/mod.rs`) | Not a flight, converter-performed visa application, or generic DT-suffix rule. |
| Crew position | The letters a plan prints in its `Pos` column on a travel row: the assignment the crew member flies that leg under (FB, Flugbegleiter — cabin crew — or DH for positioning travel). | Printed verbatim in a travel title, one space from the flight number; a position printed beside a non-flight activity stays out of that activity's title. | the `Pos` column cell held by `DutyEntry::Travel.position` (`crates/duty2cal-core/src/domain/mod.rs`) | Not a flight-designator prefix, not a second activity for the leg, not duration or pay evidence. |
| Deadhead duty | Pos DH positioning to a departure destination in uniform without usual operating work, still duty time. | One travel interval whose title carries the printed position, for example `DH 007 ABC-DEF`. | `DutyEntry::Travel.position`, `DutyEntry::summary` (`crates/duty2cal-core/src/domain/mod.rs`) | Not time off, another event for the same leg, or duration derived from Blocktime. |
| Declared configuration | The organization's and its repositories' configuration as the repository's `infra/` states it should be. | Reviewed in the repository, applied to the live organization, compared by the configuration drift report. | `infra/*.tf`; `infra/adoption-batch.txt` | Not the live configuration; not the product's build configuration (`Cargo.toml`, `cliff.toml`). |
| Dependency update | A revision whose subject names a dependency bump, such as `chore(deps): …`. | Announced under **Dependencies** when it changed a path a user receives and its words do not name the project's own tooling; it is the one housekeeping-typed revision that can be announced. A dependency update whose subject also names a capability stays in that capability's group. | `cliff.toml` (the `chore(deps):` parser and the Dependencies group); the marked user-receiving path list, read by `scripts/render-changelog.sh` | Not every dependency change: a bump that changed only the project's checks or automation is not announced; not a capability-scoped revision. |
| Default branch | The branch (`main`) that holds the project's accepted history; a change reaches it only after the gate passes. | The gate on landing is enforced on it; force pushes and deletion are blocked. | repository ruleset (`Settings → Rules → Rulesets`) | Not a release tag; not a pull-request branch. |
| Deploy key | The write-enabled public key on the pages repository whose private half lets automation publish the site. | Registered on `duty2cal/duty2cal.github.io`; the private half is the `PAGES_DEPLOY_KEY` secret on the monorepo. | `github_repository_deploy_key` (planned) | Not a personal SSH key; not the workflow's `GITHUB_TOKEN`. |
| Drift check | The check that refuses a committed change log differing from a fresh render of the released revision. | The `changelog` CI job runs `scripts/render-changelog.sh --check`, and the release preparation runs the same check before it stages anything; both refuse by name of the version whose section differs. | `scripts/render-changelog.sh --check`; `.github/workflows/ci.yml` (`changelog` job); `scripts/prepare-release.sh` | Not a reformatting pass: the released version's section is compared byte for byte with a fresh render of it, and every earlier section is kept verbatim. |
| DT prefix | Activity-code prefix expanding to Duty Time in observed DTUSA. | Preserve known meaning without interpreting unknown suffixes. | `NON_FLIGHT_CODES` (`application/mod.rs`) | Code text, not a numeric duty-duration metric. |
| Duty-plan calendar | Complete activity composition from one supplied duty plan governing readiness to export. | Success only after required activities/dates/fields resolve. | `Roster` (`domain/mod.rs`) | Aggregate, not PDF, output file, request record, service, or snapshot. |
| EM duty | Non-flight duty represented from its printed UTC interval and source station; the code's expansion is not asserted. | Timed activity using its own printed UTC window. | `DutyEntry::NonFlightDuty`, `NON_FLIGHT_CODES` (`application/mod.rs`) | Not a flight, vacation, or an asserted code expansion. |
| Flight number | The digits a plan prints in its `Activity` column on a travel row, identifying the flight. | Printed verbatim beside the crew position in a travel title, leading zeroes kept; the two are never spliced into one token. | the `Activity` column cell held by `DutyEntry::Travel.activity`; `printed_identification()` is the one rule that reads the two fields together (`crates/duty2cal-core/src/domain/mod.rs`) | Not a rotation number to reformat or convert into a quantity; a letters-and-digits cell the plan prints as one field (`LU900`) is not a position beside a flight number; not component identity. |
| Gate (on landing) | The requirement that a change pass all the project's checks before it can land on the default branch, enforced by the repository. | Formatting, lint, tests, licences, SPDX headers, third-party notices drift, change-log drift, private-data guard, coverage, Windows cross-build, public text and actionlint; DCO on pull requests. Its release-time counterpart is the **Publication gate**. | `.github/workflows/ci.yml`, the required ruleset on `main` | Not a convention or a habit; not the release pipeline. |
| Protection ruleset | The rules that constrain changes to a protected branch — deletion, force pushes, required checks, optionally pull requests — with named bypass actors. | `main` carries deletion, non-fast-forward and linear-history rules. The pages repository's rule set, when it exists, must not block the publish deploy key; whether it bypasses that key is the carried open question decided by `publish-site-plumbing`, not by adoption. | `github_repository_ruleset` (`infra/rulesets.tf`) | Not the legacy branch-protection setting as a separate object; not the gate itself. |
| Publication gate | The bounded wait that allows a version to become public only when every check run on the tagged revision, outside the release-workflow check suites, has concluded and at least one succeeded. | The first step of the `publish` job; refuses a concluded non-success, no success, no check evidence, an unreadable check-run source, or a check still running at the bound, naming the checks. Skipped and neutral are allowed; an unknown conclusion fails closed. | `scripts/await-green-checks.sh`; `.github/workflows/release.yml` (`publish`) | Not the **Gate (on landing)**, which admits a change to the default branch; not a fixed list of required check names; not a refusal that waits “until” a check passes. |
| Housekeeping revision | A released revision whose conventional-commit type is not a release note (`chore`, `docs`, `test`, `build`, `ci`, `style`). | Its subject is omitted from a version's notes, though the revision is still part of the released history. With the generated change log, a revision whose scope names no capability is omitted the same way, whatever its type — and so is one whose subject's words name the project's own tooling, even when its scope is a capability. A dependency update (`chore(deps): …`) is the one exception: it is announced when it changed a path a user receives and its words do not name the project's own tooling. | `cliff.toml` (the marked allow-list and skip rules); `scripts/render-changelog.sh` (which reads the marks and renders a version's notes) | Not a release note; not removed from the history, only from a version's notes. |
| Immutable release | A published release whose assets cannot be replaced or deleted and whose version tag cannot be moved. | Once public, a correction is the next version, never a repair in place. | `gh release` draft-then-publish flow; the release-immutability setting; the verify job's `immutable: true` assertion | Not a mutable release whose assets can be re-uploaded. |
| Layover | Explicit dated station-bearing source row representing a rest stop. | All-day activity on its date; a travel row's printed duration stays descriptive. | `DutyEntry::Layover`, `layover_station` (`application/mod.rs`) | Not a travel leg, all-day absence, or an invented interval. |
| Office or seminar duty | TK non-flight duty requiring office/development/team-information attendance. | Printed UTC Start/End; no flight route required. | `DutyEntry::NonFlightDuty`, `NON_FLIGHT_CODES` (`application/mod.rs`) | Not vacation/flight; no unprovided acronym expansion. |
| Public home | The public repository that holds the project's history and serves its releases, at `duty2cal/duty2cal`. | `git clone` works from it; the gate, alerts, scanning and attestations live there. | `https://github.com/duty2cal/duty2cal` | Not the local working copy (`duty2ical`); not a package registry. |
| Release archive | One `.tar.gz` holding the supported platform's executable and nothing else, named for the tool, the version and the platform. | Exactly one per version — the Windows x86_64 archive — in the same format. | `duty2cal-<version>-<platform>.tar.gz` | Not a container image; not a registry package; not the bare executable. |
| Release attestation | The attestation GitHub issues for an immutable release as a whole. | The published release carries it; the verify job asserts the release is immutable. | GitHub Releases; the verify job in `.github/workflows/release.yml` | Not a per-archive build-provenance attestation. |
| Release preparation | The step that turns the built executables into the staged archive set, the release text and the change log, before anything is public. | Run by the maintainer and by the packaging job; refuses an incomplete or unrecognised set, a committed change log that is not current for the version being released, and a version whose section names nothing at all — no user-receiving change and no dependency update. | `scripts/prepare-release.sh` | Not publication itself; not a hand-attached asset. |
| Release text | The newest section of the change log: the version's released changes, grouped by kind and filtered to the changes a user receives, including dependency updates under **Dependencies**, each stating the release-note words its revision carries. | Becomes the release page's text; states the version, the released revision's own date and the comparison against the previous version. | `scripts/render-changelog.sh --release-text`; `release-notes.md` | Not a flat list of revision subjects; not a changelog of every revision. |
| Release-note subject | The commit subject that states what a released revision changed, taken after `<type>(<scope>): ` from a conventional-commit subject. | Quoted in the release text when the configuration releases its scope and type; identical subjects from several revisions collapse to one occurrence. | `scripts/render-changelog.sh` (`cliff.toml`'s group and skip rules decide which subjects are released) | Not the raw conventional-commit subject; not a housekeeping revision's subject. |
| Released change | A released revision whose conventional-commit scope names a capability — a use-case id present as a directory under `givn/specs/` — or a dependency update (`chore(deps): …`), whose words do not name the project's own tooling, and whose change touched a path a user receives. | It is listed in a version's notes; a revision naming a tool, a workflow, a check, a test, a document or build/packaging/publishing machinery is not, whatever its scope says, and neither is one that changed only paths no user receives. A dependency update whose subject also names a capability is listed once, in that capability's group. | `cliff.toml` (the scope pattern, the `chore(deps):` parser and Dependencies group, the marked machinery skip list and the marked user-receiving path list, the first checked against `givn/specs/` and the last against the tree by the renderer) | Not every revision a version contains; not a housekeeping revision other than a dependency update. |
| Released Roster detail-table family | The supported source layout: the monthly crew duty-plan detail table, with repeated headers and page continuations. | The first release's supported source; an unrecognized family fails explicitly. | `PdfRosterSource`, `SourceError::UnsupportedFamily` (`crates/duty2cal-pdf/src/lib.rs`) | Not every PDF producer layout; not the overview, statistics, or footer tables. |
| Report time | The local briefing time a supported duty plan prints in its `Report (LT)` column for a travel row. | Descriptive only: rendered in the event details as the labelled `Report (LT)` value (both labelled values when the fragments differ) and appended to the travel title as ` Report <value> LT` when one value is supplied. | `DutyEntry::Travel.report_lt`, `FieldValue`, `DutyEntry::summary()`, `description_fields()` (`crates/duty2cal-core/src/domain/mod.rs`) | Not an activity instant, UTC offset, or accounting duration; not an activity date; not a title suffix when no single value exists. |
| Reserve duty | Bounded RE availability differing from SB in activation-to-report notice time. | Own UTC window; no notice calculator. | `DutyEntry::Availability`, `AVAILABILITY_CODES` (`application/mod.rs`) | Not an activation workflow or all-day default. |
| Roster trip | Source grouping of assigned travel legs associated with one Trip ID. | Carry known ID within one unambiguous travel trip. | the Trip ID column cell and `Row::carried_trip` (`application/mod.rs`) | Not one leg, leisure trip, or independent aggregate. |
| Security settings | The repository features that guard the public history: secret scanning, push protection, Dependabot alerts and Dependabot security updates. | Declared as the monorepo's repository settings and previewed and applied like any other declared value; advanced security and private vulnerability reporting are not expressible and stay manual. | `security_and_analysis` on `github_repository.mono`; `github_repository_vulnerability_alerts.mono`; `github_repository_dependabot_security_updates.mono` (`infra/`) | Not the organization's member defaults; not a protection rule set; not the release pipeline's attestations. |
| Single-day standby | Isolated/replacement SBX standby with its own UTC window. | Current SBX interval, not a former flight's times. | `DutyEntry::Availability`, `row_interval` (`application/mod.rs`) | Single-day is not all-day; no extra cancelled-flight/import-cancellation logic. |
| Software bill of materials | The machine-readable list of what went into an archive's build. | Published with every archive alongside its build provenance. | `anchore/sbom-action@v0` (syft reads `Cargo.lock`) | Not the checksum; not the provenance attestation. |
| Source revision timestamp | UTC snapshot revision metadata using the user-accepted Created footer convention for this family. | Preferred DTSTAMP; actual construction-time fallback. | `SourceDocument::revision`, `created_footer` (`crates/duty2cal-pdf/src/lib.rs`) | Not activity start, month-first constant, or universal Created/Last-Modified equivalence. |
| Source snapshot | Fixed input PDF version scoping its component identities. | Identical snapshot is deterministic; revised snapshot has separate IDs. | `SnapshotId`, `fingerprint` (`application/mod.rs`) | Not numbered output copy, another aggregate, or calendar-data anonymization. |
| Stable latest-release address | The fixed URL `https://github.com/duty2cal/duty2cal/releases/latest` that keeps resolving to the latest published release as later versions are published. | A README link or a colleague's link does not rot as versions change. | `https://github.com/duty2cal/duty2cal/releases/latest` (chapter 7) | Not a version-pinned release URL; not a package registry. |
| Standby duty | Bounded SB/Stby availability with start/end times for possible activation. | Separate dated UTC windows with FREE interruptions preserved. | `DutyEntry::Availability`, `AVAILABILITY_CODES` (`application/mod.rs`) | Not time off, continuous multi-day block, or annual-statistic event. |
| Supported platform | Windows x86_64 — the one platform/architecture combination the project supports, tests and offers for download. | A version offers exactly one archive, for it; nothing else is supported, tested or distributed. Linux remains the project's CI, coverage and local test host, not a supported download platform. | `PLATFORMS` in `scripts/prepare-release.sh` | Not every platform the tool could be built for; Linux, macOS, Windows on ARM and 32-bit Windows are unsupported, and their source can be built with a Rust toolchain. |
| Travel leg | One flight from a departure station to an arrival station, printed either as a whole row or as a departure row and an arrival row that the converter pairs by their two printed fields. | One timed travel activity titled by the printed crew position and activity, then the route, and ending with ` Report <value> LT` when the source supplies one report time (a whole-row value, a single fragment, or equal values on both fragments); no report time, or differing fragment values, leave the title route-only. | `DutyEntry::Travel`, `DutyEntry::summary()` (`crates/duty2cal-core/src/domain/mod.rs`) | Not a printed row, not two activities when a leg is split across rows, not an all-day absence. |
| Vacation day | U/Urlaub activity denoting vacation on its duty-plan date. | All-day; source times ignored. | `DutyEntry::AllDay`, `ALL_DAY_CODES` (`application/mod.rs`) | Distinct from FREE/OFF/FZ; no entitlement inference. |
| User-receiving path | A repository path whose contents reach a downloader: the converter's crates and command-line application, the build inputs that decide what the executable contains, and the packaging and release workflow that decide what an archive holds. | Listed in `cliff.toml`; a released revision that changed none of them is left out of a version's notes, and a listed path the tree does not hold stops the render. | the marked `user-receiving-paths` line in `cliff.toml`, read by `scripts/render-changelog.sh` | Not every path a revision changes: `.github/workflows/ci.yml`, `scripts/render-changelog.sh`, `cliff.toml`, `docs/`, `givn/` and test code decide how the project works on what it ships, not what a downloader receives. |
| Version tag | The `v<version>` Git tag that names the revision a release publishes; pushing it triggers the release pipeline. | Stated identically to the version in the archive names and the release text; a tag used by a published immutable release is never reusable. | `release.yml` (`push` of `v*`); `duty2cal-<version>-<platform>.tar.gz` | Not the bare version string (a leading `v` is refused in the preparation command); not a branch. |

## Aggregates / boundaries

| Root | Extent and invariant | Commands | Events |
|---|---|---|---|
| Duty-plan calendar | Required representations from one supplied plan, including fragments/date evidence; success only when every required activity/field resolves, never an apparently complete partial calendar. | Export duty plan; Complete duty-plan calendar; Publish calendar export; Record conversion failure. Last three are internal responsibilities for the Crew member. | Duty plan supplied; Duty-plan calendar completed; Duty-plan conversion rejected; Calendar export published; Calendar export publication failed; Conversion diagnostic written; Conversion diagnostic unavailable. |

PDFs, output files, diagnostics, source snapshots, and trip groupings are
records/projections or source context, **not additional aggregates**. The boundary
requires no database, persistent workflow, or particular code type.

## Invariants and required enforcement sites

| Invariant | Required future enforcement — not implemented yet |
|---|---|
| Resolve required activities/fields before successful publication; do not silently omit unknown required data. | Domain/application validation and scenario coverage. |
| Timed End follows Start; explicit day qualifiers use unique adjacent Gregorian dates and remain authoritative. | Activity/date construction and source interpretation tests. |
| FREE/OFF/U/FZ stay all-day regardless of clocks; SB/SBX/RE/TK/DTUSA use own UTC windows. | Classification/representation and expert-source validation. |
| Reporting, offsets, accounting values, and revision metadata do not calculate activity instants. | Date assembly and independent-field regressions. |
| Source/prior calendar/report contents remain unchanged; occupied candidates are not clobbered under races. | Publication/diagnostic ports and filesystem fault/concurrency evidence. |
| Every failure attempts a new report; failed saving preserves the primary error and never calls an older report current. | Application failure flow/read model and failure scenarios. |
| IDs are globally scoped/distinct and deterministic for the identical snapshot, independent of counter/title. | Snapshot identity/encoding and repeated/distinct-source evidence. |
| Private/identifying source values are not committed; public data has generated/anonymized provenance. | Repository/source-provenance gates and human review. |

Differing optional descriptions remain an unobserved parked policy; these
invariants do not silently choose their retention/merge handling.

## Lifecycle events

| Event | Observation |
|---|---|
| Duty plan supplied | A source is available for the Crew member's conversion attempt. |
| Duty-plan calendar completed | Required data resolved; publication has not yet succeeded. |
| Duty-plan conversion rejected | Interpretation cannot establish a complete representation. |
| Calendar export published | A complete new calendar was actually published at an unused path. |
| Calendar export publication failed | Completion did not produce a successful new output; failure is explicit. |
| Conversion diagnostic written | A new current-run report was actually persisted at an unused numbered path. |
| Conversion diagnostic unavailable | The current failure could not save a new report; old files prove no new write. |

## Promotion boundary and unverified terms

Eighteen confirmed introductions are promoted: the thirteen Handoff terms plus
**EM duty**, **Layover**, **Released Roster detail-table family**, **Travel leg**
and **Availability duty**, added by
the change to export released roster calendars (export-released-roster-calendar)
through its design-review glossary-scope decision. The change to publish a
public release adds sixteen release-channel terms — **Build provenance**,
**Checksum**, **Default branch**, **Gate (on landing)**, **Housekeeping
revision**, **Immutable release**, **Public home**, **Release archive**,
**Release attestation**, **Release preparation**, **Release text**,
**Release-note subject**, **Software bill of materials**, **Stable
latest-release address**, **Supported platform** and **Version tag** — as
technical vocabulary of the distribution channel, not domain rules of the
conversion. The change to generate release notes (generate-release-notes) adds
four further terms — **Capability allow-list**, **Change log**, **Drift check**
and **Released change** — and refines **Release text**, which is now the newest
section of the change log rather than a flat list of revision subjects. The
change to separate the printed crew position from the flight number
(separate-position-marker-and-flight-number) adds two printed-field terms —
**Crew position** and **Flight number** — and refines **Travel leg**'s contract
use and **Deadhead duty**'s code symbol; no artifact names the joined pair a
designator any more. The public-home infrastructure ideation Handoff
(public-home-infrastructure) adds six technical terms — **Adoption**,
**Configuration change**, **Configuration drift**, **Declared configuration**,
**Deploy key** and **Protection ruleset** — as vocabulary of the project's
GitHub home; no new aggregate or invariant is introduced (the declared
configuration is a specification and the applied state is a record). The change
to announce dependency updates (announce-dependency-updates) adds
**Dependency update** and refines
**Housekeeping revision**, **Released change**, **Release preparation** and
**Release text**. The change to wait for green checks before publication
(wait-for-green-checks-before-publish) adds **Check run**, **Check suite** and
**Publication gate**, and refines **Gate (on landing)** to name its release-time
counterpart. The change to list only the changes a user receives
(list-only-changes-a-user-receives) adds **User-receiving path** and refines
**Released change**, which now also requires a change to a path a user
receives. The change to declare the public home (declare-public-home) adds no
new term: it realizes **Adoption** and **Declared configuration** in `infra/`
and records their code symbols; **Configuration change**, **Configuration
drift** and **Deploy key** remain planned for the apply and publishing seeds. The
change to enable repository security (enable-repository-security) adds
**Security settings** and refines **Configuration change**, which now names the
saved plan, the read-only refresh-only preflight, the apply, the state it
records and the stale marker that refuses a partial state; **Configuration
drift** and **Deploy key** remain planned for the drift-report and publishing
changes. The change to include the report time in the travel title
(include-report-time-in-travel-title) adds **Report time** and refines
**Travel leg**'s contract use, which now names the conditional
` Report <value> LT` title suffix; the source column's printed `Report (LT)`
label names that term and is no longer an unconfirmed raw label. Raw
source-column labels
(A/C, Blocktime, Timezone Difference, Tags) and the FREE/OFF day
codes are used verbatim in the specification and are not confirmed domain terms.

Candidate **Duty plan** and **Calendar export** remain topic
candidates, not silently confirmed here. No rejected entry exists to remove.
Suggested FZ expansion and agreement/age/quota provenance remain background, not
leave/eligibility rules.
