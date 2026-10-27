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
(include-report-time-in-travel-title), plus seven public-profile terms added by
the change to publish the organization profile
(publish-organization-profile), plus four size terms added by the release
executable size ideation Handoff (release-executable-size), plus five
public-website terms added by the public website ideation Handoff
(public-website), plus one browser-offer term and the generalized browser
lifecycle observations added by the change to build the browser converter
(build-browser-converter), plus one measurement-report term added by the change
to measure the executable's size (measure-executable-size), plus three
reduction terms added by the change to reduce the executable's size
(reduce-executable-size), plus the term **Pages publishing plumbing**, the
realization of **Deploy key** and the refinement of **Protection ruleset** by
the change to create the pages publishing plumbing (publish-site-plumbing),
plus the realization of the five public-website terms — **Public website**,
**Site publication**, **Site output** and **Built revision** among them — and
the site's lifecycle observations by the change to publish the public website
(publish-public-website); that change introduces no new term, plus three
input-generalization terms added by the generalized duty-plan input ideation
Handoff (generalized-duty-plan-input) — **Duty plan**, **Evidence for a
publication** and **Support statement**.
Source: [Duty-plan calendar conversion](../ideation/duty-plan-calendar-conversion.md),
[introduction/collision records](../../givn/ideation/duty-plan-calendar-conversion/domain-terms.md),
[approved contract](../../givn/ideation/duty-plan-calendar-conversion/use-cases/export-duty-plan-calendar.md),
and [public-home infrastructure](../ideation/public-home-infrastructure.md) with its
[collision records](../../givn/ideation/public-home-infrastructure/domain-terms.md),
and [generalized duty-plan input](../ideation/generalized-duty-plan-input.md) with its
[collision records](../../givn/ideation/generalized-duty-plan-input/domain-terms.md).
This is vocabulary/required behavior, not an implemented enforcement claim.

## Ubiquitous language

| Term | Definition | Contract use | Code symbol | Contrast / anti-terms |
|---|---|---|---|---|
| Additional vacation day | FZ roster activity denoting an additional vacation-related day off. | All-day; source times ignored. | `DutyEntry::AllDay`, `ALL_DAY_CODES` (`crates/duty2cal-core/src/application/mod.rs`) | Distinct from U; no verified entitlement/quota or acronym expansion. |
| Adoption | The one-time act of placing an already-existing GitHub object under the declared configuration instead of creating it. | The first-time path an apply depends on; a batch is adopted or none is. | `scripts/adopt-public-home.sh` and `tofu import`; `infra/adoption-batch.txt`; the encrypted `infra/terraform.tfstate` | Not an apply; not creation; not the prior topic's contract adoption. |
| Availability duty | Bounded standby or reserve window the crew member must be reachable in (SB, SBX, RE). | One timed availability activity with its own UTC window and station. | `DutyEntry::Availability`, `AVAILABILITY_CODES` (`crates/duty2cal-core/src/application/mod.rs`) | Not an all-day day off, not a flight, not a non-flight duty. |
| Browser conversion | A conversion of one duty plan performed inside the visitor's browser tab, with the plan never sent to another host. | The browser capability under the use case to export a duty-plan calendar; the page offers the calendar and the diagnostic and writes nothing. Built from the same application boundary as the executable. | `ffi/wasm` (package `duty2cal-wasm`); `web/` page; `web/e2e/driver.mjs` | Not executable conversion, which writes beside the source; not a hosted conversion, which would receive the plan. |
| Build configuration | The committed declaration that decides what the distributed executable contains: Cargo profiles, feature selection and target link flags. | Measured before a size lever is applied; a lever that changes it is accepted only with a cumulative measured delta and its failure-outcome evidence. | `Cargo.toml` `[profile.release]`; `.cargo/config.toml` | Not the **Declared configuration** under `infra/`; not the release workflow, which decides what an archive holds. |
| Build provenance | A signed attestation tying a published archive to the tagged revision and the automation that built it. | Published with every archive; a downloader checks it with `gh attestation verify`. | `actions/attest@v4` (`.github/workflows/release.yml`) | Not the checksum, which covers bytes only; not the bill of materials. |
| Built revision | The repository revision the **Public website** was built from, named in its footer. | Rendered as `git describe` output and linked to the commit; the publication refuses an unresolvable revision or one whose working tree is not clean, so the page cannot name a revision that did not build it. | `scripts/build-site.sh` (`git describe --tags --always --match 'v*'` plus `rev-parse`); the footer's `data-site-statement="built-revision"` element | Not the **Source revision timestamp**, the duty plan's PDF metadata; not necessarily a **Version tag**. |
| Capability allow-list | The set of released scopes — the use-case ids present as directories under `givn/specs/` — that a revision's scope must name to be a released change. | Materialised in `cliff.toml` and checked against the corpus by the renderer, which refuses any difference. | `givn/specs/`; `cliff.toml` (scope pattern) | Not every kebab-case scope: `fix(ci)` is shaped like `fix(publish-public-release)` but names no capability. |
| Change log | The repository file (`CHANGELOG.md`) whose newest section is the version being released and whose earlier sections stay as they were, generated from the repository's history. | Its newest section is the release text, and the committed file is drift-checked against a fresh render of the released revision. | `CHANGELOG.md`; `scripts/render-changelog.sh` | Not the release page alone; not a hand-maintained file. |
| Check run | One check that reports on a commit, with a status (`queued`, `in_progress`, `completed`) and a conclusion when complete (`success`, `skipped`, `neutral`, `failure`, …). | The publication gate classifies every check run on the tagged revision outside the release-workflow suites. | the check-runs endpoint consumed by `scripts/await-green-checks.sh` | Not a check suite; not a workflow run. |
| Check suite | The GitHub grouping that owns a set of check runs, usually one per workflow run. | The gate excludes every suite the release workflow created for the tagged revision, so the release's own checks cannot gate themselves. | `check_suite.id` in the check-runs response; `.github/workflows/release.yml` runs | Not a check run; not a workflow name. |
| Checksum | A SHA-256 digest covering one archive, published beside it and naming the file it covers. | A downloader verifies what they received before extracting it. | `sha256sum`, `duty2cal-<version>-<platform>.tar.gz.sha256` | Not the archive's size; not the release attestation. |
| Configuration change | A reviewed revision of the declared configuration that reaches GitHub through an apply. | Previewed as a saved plan and applied unchanged behind a read-only refresh-only preflight; a refusal before the first write leaves live and the state unchanged; a partial failure names the applied objects, the failed object or read and the reason and is marked stale beside the state, not recorded as applied; the preview and the apply refuse while the stale marker exists; the operator removes it after reading the partial report, and the next green apply completes the revision. | `infra/` revision; `tofu plan -out` and `tofu apply <plan>`; the encrypted `infra/terraform.tfstate`; `infra/terraform.tfstate.stale` | Not a product change; not a release. |
| Configuration drift | A difference between the declared configuration and what the organization currently holds. | A report names every difference it can compare and marks unreadable fields unchecked. | configuration drift check (`tofu plan -detailed-exitcode`, planned) | Not the **Drift check**, which compares a committed change log with a fresh render. |
| Consulate visa duty | Non-flight DTUSA duty for a consulate visit to obtain a USA visa. | Own printed UTC interval. | `DutyEntry::NonFlightDuty`, `NON_FLIGHT_CODES` (`application/mod.rs`) | Not a flight, converter-performed visa application, or generic DT-suffix rule. |
| Crew position | The letters a plan prints in its `Pos` column on a travel row: the assignment the crew member flies that leg under (FB, Flugbegleiter — cabin crew — or DH for positioning travel). | Printed verbatim in a travel title, one space from the flight number; a position printed beside a non-flight activity stays out of that activity's title. | the `Pos` column cell held by `DutyEntry::Travel.position` (`crates/duty2cal-core/src/domain/mod.rs`) | Not a flight-designator prefix, not a second activity for the leg, not duration or pay evidence. |
| Deadhead duty | Pos DH positioning to a departure destination in uniform without usual operating work, still duty time. | One travel interval whose title carries the printed position, for example `DH 007 ABC-DEF`. | `DutyEntry::Travel.position`, `DutyEntry::summary` (`crates/duty2cal-core/src/domain/mod.rs`) | Not time off, another event for the same leg, or duration derived from Blocktime. |
| Declared configuration | The organization's and its repositories' configuration as the repository's `infra/` states it should be. | Reviewed in the repository, applied to the live organization, compared by the configuration drift report. | `infra/*.tf`; `infra/adoption-batch.txt` | Not the live configuration; not the product's build configuration (`Cargo.toml`, `cliff.toml`). |
| Dependency update | A revision whose subject names a dependency bump, such as `chore(deps): …`. | Announced under **Dependencies** when it changed a path a user receives and its words do not name the project's own tooling; it is the one housekeeping-typed revision that can be announced. A dependency update whose subject also names a capability stays in that capability's group. | `cliff.toml` (the `chore(deps):` parser and the Dependencies group); the marked user-receiving path list, read by `scripts/render-changelog.sh` | Not every dependency change: a bump that changed only the project's checks or automation is not announced; not a capability-scoped revision. |
| Default branch | The branch (`main`) that holds the project's accepted history; a change reaches it only after the gate passes. | The gate on landing is enforced on it; force pushes and deletion are blocked. | repository ruleset (`Settings → Rules → Rulesets`) | Not a release tag; not a pull-request branch. |
| Default community health files | Files at the root of the organization's public `.github` repository that GitHub uses for an organization repository that has no such file of its own. | The archived profile change published `SECURITY.md`; a repository without its own policy inherits the organization's private reporting route. `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md` and the templates remain in the monorepo. | `org/SECURITY.md`; `github_repository_file.org_security` (`infra/org_profile.tf`) | Not the monorepo's own policy file; not a per-repository copy; not a setting the declaration can preview for every repository. |
| Default security policy | The reviewed `org/SECURITY.md`, published at the profile repository's root; GitHub uses it for an organization repository that has no policy of its own. | Names the private vulnerability reporting route, forbids attaching a real duty plan or a converted calendar, and states that only the latest published release is supported; the route was enabled manually before that publication (chapter 7, R-030). | `org/SECURITY.md`; `github_repository_file.org_security` (`infra/org_profile.tf`) | Not the monorepo's own longer policy; not a per-repository copy; not the profile content. |
| Deploy key | The write-enabled public key on the pages repository whose private half lets automation publish the site. | Registered as `duty2cal.github.io publish` on `duty2cal/duty2cal.github.io`; the private half is the `PAGES_DEPLOY_KEY` secret on the monorepo and exists only in the encrypted applied state, which is its only recoverable copy. | `tls_private_key.pages_publish`; `github_repository_deploy_key.pages_publish`; `github_actions_secret.pages_publish` (`infra/pages_publish.tf`) | Not a personal SSH key; not the workflow's `GITHUB_TOKEN`; the public half is not the credential. |
| Distributed executable | The artifact built for `x86_64-pc-windows-gnu` and packaged in the release archive; the thing a downloader runs. | Its stripped file size is reported beside a size attribution and is never substituted for the attribution's total. | `target/x86_64-pc-windows-gnu/release/duty2cal.exe`; `duty2cal-<version>-windows-x86_64.tar.gz` | Not the Linux host build; not the compressed archive; not the symbol-bearing build used for attribution. |
| Drift check | The check that refuses a committed change log differing from a fresh render of the released revision. | The `changelog` CI job runs `scripts/render-changelog.sh --check`, and the release preparation runs the same check before it stages anything; both refuse by name of the version whose section differs. | `scripts/render-changelog.sh --check`; `.github/workflows/ci.yml` (`changelog` job); `scripts/prepare-release.sh` | Not a reformatting pass: the released version's section is compared byte for byte with a fresh render of it, and every earlier section is kept verbatim. |
| DT prefix | Activity-code prefix expanding to Duty Time in observed DTUSA. | Preserve known meaning without interpreting unknown suffixes. | `NON_FLIGHT_CODES` (`application/mod.rs`) | Code text, not a numeric duty-duration metric. |
| Duty plan | The schedule a crew member's crew scheduling system publishes for one period, assigning dated activities to that crew member together with the facts each activity states. | The subject of every conversion: read as it states itself, then interpreted under the confirmed rules; never classified into a format first. | `SourceDocument` (`crates/duty2cal-core/src/application/mod.rs`) — the read record of the supplied document | Not the personal calendar derived from it; not the crew scheduling system; not a file format or a publication layout. |
| Duty-plan calendar | Complete activity composition from one supplied duty plan governing readiness to export. | Success only after required activities/dates/fields resolve. | `Roster` (`domain/mod.rs`) | Aggregate, not PDF, output file, request record, service, or snapshot. |
| EM duty | Non-flight duty represented from its printed UTC interval and source station; the code's expansion is not asserted. | Timed activity using its own printed UTC window. | `DutyEntry::NonFlightDuty`, `NON_FLIGHT_CODES` (`application/mod.rs`) | Not a flight, vacation, or an asserted code expansion. |
| Evidence for a publication | The set of artifacts on which a claim about one publication rests: a stripped sample of it, the facts that sample states, and a fixture built from it that converts. | A publication may be named in the **Support statement** only when its evidence is exercised by the checks; the landed reading on its own is not evidence. | planned: the sample and fixture of the change that lands a reading; `scripts/validate-local-corpus.sh` is the nearest existing local-evidence check | Not the landed reading; not the fixture alone, because the sample decides whether it proves anything; not release evidence, which backs a version's build. |
| Flight number | The digits a plan prints in its `Activity` column on a travel row, identifying the flight. | Printed verbatim beside the crew position in a travel title, leading zeroes kept; the two are never spliced into one token. | the `Activity` column cell held by `DutyEntry::Travel.activity`; `printed_identification()` is the one rule that reads the two fields together (`crates/duty2cal-core/src/domain/mod.rs`) | Not a rotation number to reformat or convert into a quantity; a letters-and-digits cell the plan prints as one field (`LU900`) is not a position beside a flight number; not component identity. |
| Gate (on landing) | The requirement that a change pass all the project's checks before it can land on the default branch, enforced by the repository. | Formatting, lint, tests, licences, SPDX headers, third-party notices drift, change-log drift, private-data guard, coverage, Windows cross-build, public text and actionlint; DCO on pull requests. Its release-time counterpart is the **Publication gate**. | `.github/workflows/ci.yml`, the required ruleset on `main` | Not a convention or a habit; not the release pipeline. |
| Pinned repositories | Up to six repositories the organization owner selects in the organization's web settings to appear on the profile overview. | Set once after the profile repository is created; they have no declared object, no state binding and no drift report. | the organization's profile settings in the web interface (chapter 7) | Not a declared object; not a repository feature; not a previewable change. |
| Profile content | The reviewed `org/profile/README.md`, published as `profile/README.md`; GitHub renders it on the organization overview. | States the local conversion and its crew duty plan input, Windows x86_64 as the only published archive platform with builds for other operating systems available on request, the browser conversion route now reachable on the Public website at `https://duty2cal.github.io/`, the stable release and repository links, the non-affiliation sentence and the Unicode-3.0 attribution; it embeds no third-party resource. | `org/profile/README.md`; `github_repository_file.org_profile_readme` (`infra/org_profile.tf`) | Not the organization's profile settings fields (name, description, blog, social handle), which stay empty; not a rendered screen owned by the project; not the default security policy. |
| Protection ruleset | The rules that constrain changes to a protected branch — deletion, force pushes, required checks, optionally pull requests — with named bypass actors. | `main` carries deletion, non-fast-forward and linear-history rules. The pages repository carries `pages protection` with the same three rules and a `DeployKey` bypass actor with `always` and no id, so every write-enabled deploy key on that repository may push while humans cannot force-push or delete; the bypass is by actor type, not by key identity. | `github_repository_ruleset.main` (`infra/rulesets.tf`); `github_repository_ruleset.pages_publish` (`infra/pages_publish.tf`) | Not the legacy branch-protection setting as a separate object; not the gate itself; not a required-pull-request rule, which the pages rule set deliberately omits. |
| Pages publishing plumbing | The deploy key, its `PAGES_DEPLOY_KEY` secret and the `pages protection` rule set as one declared unit. | Created by one configuration change; the preview names the three live creations and the generated key; the apply creates and records them; a later targeted preview reports no differences; it is not the publication of the site. | `infra/pages_publish.tf`; `tls_private_key.pages_publish`, `github_repository_deploy_key.pages_publish`, `github_actions_secret.pages_publish`, `github_repository_ruleset.pages_publish` | Not the **Site publication**, which builds and publishes the page and is the credential's only consumer; not the **Public website**; not the rule set alone, which is one of its three objects. |
| Publication gate | The bounded wait that allows a version to become public only when every check run on the tagged revision, outside the release-workflow check suites, has concluded and at least one succeeded. | The first step of the `publish` job; refuses a concluded non-success, no success, no check evidence, an unreadable check-run source, or a check still running at the bound, naming the checks. Skipped and neutral are allowed; an unknown conclusion fails closed. | `scripts/await-green-checks.sh`; `.github/workflows/release.yml` (`publish`) | Not the **Gate (on landing)**, which admits a change to the default branch; not a fixed list of required check names; not a refusal that waits “until” a check passes. |
| Housekeeping revision | A released revision whose conventional-commit type is not a release note (`chore`, `docs`, `test`, `build`, `ci`, `style`). | Its subject is omitted from a version's notes, though the revision is still part of the released history. With the generated change log, a revision whose scope names no capability is omitted the same way, whatever its type — and so is one whose subject's words name the project's own tooling, even when its scope is a capability. A dependency update (`chore(deps): …`) is the one exception: it is announced when it changed a path a user receives and its words do not name the project's own tooling. | `cliff.toml` (the marked allow-list and skip rules); `scripts/render-changelog.sh` (which reads the marks and renders a version's notes) | Not a release note; not removed from the history, only from a version's notes. |
| Immutable release | A published release whose assets cannot be replaced or deleted and whose version tag cannot be moved. | Once public, a correction is the next version, never a repair in place. | `gh release` draft-then-publish flow; the release-immutability setting; the verify job's `immutable: true` assertion | Not a mutable release whose assets can be re-uploaded. |
| Layover | Explicit dated station-bearing source row representing a rest stop. | All-day activity on its date; a travel row's printed duration stays descriptive. | `DutyEntry::Layover`, `layover_station` (`application/mod.rs`) | Not a travel leg, all-day absence, or an invented interval. |
| Office or seminar duty | TK non-flight duty requiring office/development/team-information attendance. | Printed UTC Start/End; no flight route required. | `DutyEntry::NonFlightDuty`, `NON_FLIGHT_CODES` (`application/mod.rs`) | Not vacation/flight; no unprovided acronym expansion. |
| Offer | The browser realization's act of making a calendar or diagnostic available as a download named from the source stem, leaving the save location and any name collision to the browser. | The page writes nothing and never claims a save it did not observe; a failed attempt withdraws the previous offer, and a failed diagnostic offer leaves the primary failure visible. | `web/app.js` (`offerCalendar`, `acceptFile`); `web/e2e/driver.mjs` (`save_offer`) | Not publication beside the source; not the browser's save; not a guaranteed unused numbered name. |
| Organization profile | The public overview page GitHub renders for the `duty2cal` organization from `profile/README.md` in the public `.github` repository. | States what the project does, the only published archive platform with builds for other operating systems available on request, the download and source links, the browser route on the Public website at `https://duty2cal.github.io/`, and the non-affiliation sentence; it is reviewed copy, not a system-rendered UI. | `org/profile/README.md`; `github_repository_file.org_profile_readme` (`infra/org_profile.tf`) | Not the organization's profile settings fields (name, description, blog, social handle), which stay empty; not the Pages site. |
| Organization profile repository | The public `duty2cal/.github` repository that holds the organization profile and its default community health files. | Output of the declaration; created by an apply when absent live, never edited by hand. | `github_repository.dot_github` (`infra/org_profile.tf`) | Not the member-only `.github-private`; not a source repository. |
| Public home | The public repository that holds the project's history and serves its releases, at `duty2cal/duty2cal`. | `git clone` works from it; the gate, alerts, scanning and attestations live there. | `https://github.com/duty2cal/duty2cal` | Not the local working copy (`duty2ical`); not a package registry. |
| Public website | The published static page at `duty2cal.github.io` that offers browser conversion and the project's public statements. | Served from the Pages repository; its content is the Site output the Site publication gated; the page states what it does and both routes before it offers a file picker, carries the source and licence below the converter, carries the project icon, and its footer names the Built revision. | `web/index.html`; `docs/assets/duty2cal.png`; `scripts/publish-site.sh`; `.github/workflows/site.yml`; `duty2cal.github.io` | Not the **Public home**, the source repository; not the **Organization profile**, GitHub's rendering. |
| Release archive | One `.tar.gz` holding the supported platform's executable and nothing else, named for the tool, the version and the platform. | Exactly one per version — the Windows x86_64 archive — in the same format. | `duty2cal-<version>-<platform>.tar.gz` | Not a container image; not a registry package; not the bare executable. |
| Release attestation | The attestation GitHub issues for an immutable release as a whole. | The published release carries it; the verify job asserts the release is immutable. | GitHub Releases; the verify job in `.github/workflows/release.yml` | Not a per-archive build-provenance attestation. |
| Release preparation | The step that turns the built executables into the staged archive set, the release text and the change log, before anything is public. | Run by the maintainer and by the packaging job; refuses an incomplete or unrecognised set, a committed change log that is not current for the version being released, and a version whose section names nothing at all — no user-receiving change and no dependency update. | `scripts/prepare-release.sh` | Not publication itself; not a hand-attached asset. |
| Release text | The newest section of the change log: the version's released changes, grouped by kind and filtered to the changes a user receives, including dependency updates under **Dependencies**, each stating the release-note words its revision carries. | Becomes the release page's text; states the version, the released revision's own date and the comparison against the previous version. | `scripts/render-changelog.sh --release-text`; `release-notes.md` | Not a flat list of revision subjects; not a changelog of every revision. |
| Release-note subject | The commit subject that states what a released revision changed, taken after `<type>(<scope>): ` from a conventional-commit subject. | Quoted in the release text when the configuration releases its scope and type; identical subjects from several revisions collapse to one occurrence. | `scripts/render-changelog.sh` (`cliff.toml`'s group and skip rules decide which subjects are released) | Not the raw conventional-commit subject; not a housekeeping revision's subject. |
| Released change | A released revision whose conventional-commit scope names a capability — a use-case id present as a directory under `givn/specs/` — or a dependency update (`chore(deps): …`), whose words do not name the project's own tooling, and whose change touched a path a user receives. | It is listed in a version's notes; a revision naming a tool, a workflow, a check, a test, a document or build/packaging/publishing machinery is not, whatever its scope says, and neither is one that changed only paths no user receives. A dependency update whose subject also names a capability is listed once, in that capability's group. | `cliff.toml` (the scope pattern, the `chore(deps):` parser and Dependencies group, the marked machinery skip list and the marked user-receiving path list, the first checked against `givn/specs/` and the last against the tree by the renderer) | Not every revision a version contains; not a housekeeping revision other than a dependency update. |
| Released Roster detail-table family | The supported source layout: the monthly crew duty-plan detail table, with repeated headers and page continuations. | The first release's supported source; an unrecognized family fails explicitly. | `PdfRosterSource`, `SourceError::UnsupportedFamily` (`crates/duty2cal-pdf/src/lib.rs`) | Not every PDF producer layout; not the overview, statistics, or footer tables. |
| Report time | The local briefing time a supported duty plan prints in its `Report (LT)` column for a travel row. | Descriptive only: rendered in the event details as the labelled `Report (LT)` value (both labelled values when the fragments differ) and appended to the travel title as ` Report <value> LT` when one value is supplied. | `DutyEntry::Travel.report_lt`, `FieldValue`, `DutyEntry::summary()`, `description_fields()` (`crates/duty2cal-core/src/domain/mod.rs`) | Not an activity instant, UTC offset, or accounting duration; not an activity date; not a title suffix when no single value exists. |
| Reserve duty | Bounded RE availability differing from SB in activation-to-report notice time. | Own UTC window; no notice calculator. | `DutyEntry::Availability`, `AVAILABILITY_CODES` (`application/mod.rs`) | Not an activation workflow or all-day default. |
| Reviewed public copy | The umbrella term for the **profile content** and the **default security policy** under `org/`, reviewed and licensed before an apply publishes them. | States only claims the repository keeps; the published bytes are compared with this source by the content-invariant scenario. | `org/profile/README.md`; `org/SECURITY.md`; the `org/**` REUSE annotation | Not a third file or a synonym for either term; not text typed into the profile repository's web editor; not a generated site page. |
| Roster trip | Source grouping of assigned travel legs associated with one Trip ID. | Carry known ID within one unambiguous travel trip. | the Trip ID column cell and `Row::carried_trip` (`application/mod.rs`) | Not one leg, leisure trip, or independent aggregate. |
| Security settings | The repository features that guard the public history: secret scanning, push protection, Dependabot alerts and Dependabot security updates. | Declared as the monorepo's repository settings and previewed and applied like any other declared value; advanced security and private vulnerability reporting are not expressible and stay manual. | `security_and_analysis` on `github_repository.mono`; `github_repository_vulnerability_alerts.mono`; `github_repository_dependabot_security_updates.mono` (`infra/`) | Not the organization's member defaults; not a protection rule set; not the release pipeline's attestations. |
| Site output | The built page, icon, script and WASM bundle that the publish command writes into the Pages repository. | Produced by the Site publication as one assembled directory of exactly eight members, enumerated in chapter 7; the licence file is one of them; the artifact the gate inspected is the artifact the commit carries. | `dist/site/` assembled by `scripts/build-site.sh`; `web/`, `web/pkg/` and `docs/assets/duty2cal.png` are its sources | Not the **Release archive**, which holds the Windows executable; not the source. |
| Site publication | The build-and-publish process that turns a source revision into the **Public website**. | Starts on a default-branch revision the command's `--since` comparison judges a site input, or on a release tag (which always builds); gated by page-content, licensing, browser and conversion evidence; builds the revision's tree in a detached worktree at the **Built revision**; replaces the target's content as a whole in one fast-forward commit under an exclusive lock, or reports the target as current when it already holds the output; verifies the served page names the Built revision; the release publishes only after it succeeded and the target carries the tag. | `scripts/publish-site.sh`; `scripts/site-evidence.sh`; `scripts/build-site.sh`; `.github/workflows/site.yml` | Not the **Publication gate**, a release-time wait; not **Release preparation**, the Windows archive; not a **Configuration change**. |
| Single-day standby | Isolated/replacement SBX standby with its own UTC window. | Current SBX interval, not a former flight's times. | `DutyEntry::Availability`, `row_interval` (`application/mod.rs`) | Single-day is not all-day; no extra cancelled-flight/import-cancellation logic. |
| Size attribution | The measurement that breaks down the code the distributed executable contains, by crate and by function, from a build that retains the names the measurement needs. | Produced for a named revision and the supported platform; the stripped artifact's file size is reported beside it, and the two are never substituted. | `cargo bloat --release --target x86_64-pc-windows-gnu --crates` (planned) | Not the archive's compressed size; not the stripped file's byte count; not an estimate. |
| Size lever | The build-configuration change whose effect on the distributed executable can be measured and accepted or rejected. | Accepted only with a cumulative measured delta against the configuration carrying the previously accepted levers, and with its failure-outcome evidence recorded before it lands. | planned change `reduce-executable-size`; `Cargo.toml` | Not a nightly-only compiler flag; not UPX or dynamic linking, refused before a build. |
| Software bill of materials | The machine-readable list of what went into an archive's build. | Published with every archive alongside its build provenance. | `anchore/sbom-action@v0` (syft reads `Cargo.lock`) | Not the checksum; not the provenance attestation. |
| Source revision timestamp | UTC snapshot revision metadata using the user-accepted Created footer convention for this family. | Preferred DTSTAMP; actual construction-time fallback. | `SourceDocument::revision`, `created_footer` (`crates/duty2cal-pdf/src/lib.rs`) | Not activity start, month-first constant, or universal Created/Last-Modified equivalence. |
| Source snapshot | Fixed input PDF version scoping its component identities. | Identical snapshot is deterministic; revised snapshot has separate IDs. | `SnapshotId`, `fingerprint` (`application/mod.rs`) | Not numbered output copy, another aggregate, or calendar-data anonymization. |
| Stable latest-release address | The fixed URL `https://github.com/duty2cal/duty2cal/releases/latest` that keeps resolving to the latest published release as later versions are published. | A README link or a colleague's link does not rot as versions change. | `https://github.com/duty2cal/duty2cal/releases/latest` (chapter 7) | Not a version-pinned release URL; not a package registry. |
| Standby duty | Bounded SB/Stby availability with start/end times for possible activation. | Separate dated UTC windows with FREE interruptions preserved. | `DutyEntry::Availability`, `AVAILABILITY_CODES` (`application/mod.rs`) | Not time off, continuous multi-day block, or annual-statistic event. |
| Support statement | The public statement of what a duty plan must state for conversion and which publications the exercised evidence covers. | Carried identically by the repository README, the organization profile, chapter 3 and the issue form; it reports evidence, never intent, and shrinks when a publication's evidence stops holding. | `README.md`; `org/profile/README.md`; `docs/arc42/03-context-and-scope.md`; `.github/ISSUE_TEMPLATE/bug.yml` (planned as one statement in all four) | Not the **Release text**, which reports one version's changes; not the **Profile content**, which renders the statement among other statements; not a roadmap or a promise of future support. |
| Supported platform | Windows x86_64 — the one platform/architecture combination the project publishes as an archive, and the one it tests and distributes in that form. | A version publishes exactly one archive, for it. **Supported platform** is about published archive platforms, not about conversion routes: the browser is a supported conversion route and offers no archive. Builds for other operating systems are available on request. Linux remains the project's CI, coverage and local test host, not a supported download platform. | `PLATFORMS` in `scripts/prepare-release.sh`; the browser route in `web/` and `ffi/wasm` | Not every platform the tool could be built for; Linux, macOS, Windows on ARM and 32-bit Windows have no published archive, obtain a build on request, and their source can be built with a Rust toolchain. Not the browser route, which has no archive. |
| Travel leg | One flight from a departure station to an arrival station, printed either as a whole row or as a departure row and an arrival row that the converter pairs by their two printed fields. | One timed travel activity titled by the printed crew position and activity, then the route, and ending with ` Report <value> LT` when the source supplies one report time (a whole-row value, a single fragment, or equal values on both fragments); no report time, or differing fragment values, leave the title route-only. | `DutyEntry::Travel`, `DutyEntry::summary()` (`crates/duty2cal-core/src/domain/mod.rs`) | Not a printed row, not two activities when a leg is split across rows, not an all-day absence. |
| Vacation day | U/Urlaub activity denoting vacation on its duty-plan date. | All-day; source times ignored. | `DutyEntry::AllDay`, `ALL_DAY_CODES` (`application/mod.rs`) | Distinct from FREE/OFF/FZ; no entitlement inference. |
| User-receiving path | A repository path whose contents reach a downloader: the converter's crates and command-line application, the build inputs that decide what the executable contains, and the packaging and release workflow that decide what an archive holds. | Listed in `cliff.toml`; a released revision that changed none of them is left out of a version's notes, and a listed path the tree does not hold stops the render. | the marked `user-receiving-paths` line in `cliff.toml`, read by `scripts/render-changelog.sh` | Not every path a revision changes: `.github/workflows/ci.yml`, `scripts/render-changelog.sh`, `cliff.toml`, `docs/`, `givn/` and test code decide how the project works on what it ships, not what a downloader receives. |
| Version tag | The `v<version>` Git tag that names the revision a release publishes; pushing it triggers the release pipeline. | Stated identically to the version in the archive names and the release text; a tag used by a published immutable release is never reusable. | `release.yml` (`push` of `v*`); `duty2cal-<version>-<platform>.tar.gz` | Not the bare version string (a leading `v` is refused in the preparation command); not a branch. |
| Distributed executable | The artifact built for `x86_64-pc-windows-gnu` and packaged in the release archive; the thing a downloader runs. | Its stripped file size is reported beside a size attribution and is never substituted for the attribution's total. | `target/x86_64-pc-windows-gnu/release/duty2cal.exe`; `duty2cal-<version>-windows-x86_64.tar.gz` | Not the Linux host build; not the compressed archive; not the build that retains the names used for attribution. |
| Size attribution | The measurement that breaks down the code the distributed executable contains, by crate and by function, from a build that retains the names the measurement needs. | Produced for every pushed revision and on request for any named revision; names the requested revision, the resolved revision, the platform and the build's identity; the stripped artifact's file size is reported beside it, and the two are never substituted. | `scripts/measure_executable_size.py`; `cargo bloat --release --locked --target x86_64-pc-windows-gnu --crates` and the functions run (pinned `cargo-bloat` 0.12.1) | Not the archive's compressed size; not the stripped file's byte count; not a byte-exact file size (the generator's numbers are estimates of `.text`). |
| Size attribution report | The pair of files a measurement writes — a machine-readable report and a human rendering of the same data — naming the requested revision, the resolved revision, the platform and the build's identity, with the crate and function split and the stripped executable's file size as separate fields. | Its status is `complete`, `incomplete` or `unreadable`: an incomplete report states that the file size is unavailable, an unreadable one states what could not be read, and neither infers a number. Produced for every pushed revision and requestable for any named revision. | `attribution.json`, `attribution.md`; the run's `size-attribution` artifact | Not the attribution itself; not the release archive; not the change log. |
| Size lever | The build-configuration change whose effect on the distributed executable can be measured and accepted or rejected. | Applied `panic = "abort"`: 2,014,208 B against a 2,098,176 B rebuild of the unchanged configuration (saves 83,968 B on the release branch); `opt-level = "s"` was measured larger and rejected; the panic and conversion outcomes are recorded before landing. | `Cargo.toml` `[profile.release]` `panic = "abort"`; `scripts/measure_size_lever.py`; `size-lever.json`/`size-lever.md` | Not a nightly-only compiler flag; not UPX or dynamic linking, refused before a build; not a lever whose saving is not cumulative. |
| Baseline | The same revision rebuilt with the unchanged build configuration (`panic = "unwind"`, `opt-level = "z"`) by `scripts/measure_size_lever.py`. | The saving is measured against it; the published binary's size is reported separately and never used as the baseline. | `--repo`/`--revision`; the baseline build in `size-lever.json`; the `- Baseline (rebuild, unchanged):` line | Not the published binary; not the previous release; not the lever build. |
| Published binary | The executable inside the published release archive, built from the tagged revision and the shipped profile. | Its file size is reported separately from the rebuilt baseline; it is not the baseline and not the lever build. | the release archive's `duty2cal.exe`; `--published-size` in `scripts/measure_size_lever.py` | Not the rebuilt baseline; not the lever build; not the compressed archive. |
| Saved bytes | The baseline file size minus the lever build's file size; a positive value means the lever build is smaller. | The cumulative table records it per lever; only a positive value is applied. | `saved_bytes` in `size-lever.json`; the cumulative table in `size-lever.md` | Not the file size; not the archive's compressed size; not a signed size change. |

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
| Calendar export published | A complete new calendar was actually published at an unused path. The browser realization is **Calendar export offered**: the page made the complete calendar available for saving and wrote nothing. |
| Calendar export offered | The browser realization of Calendar export published: the page made the complete calendar available as a download named from the source stem; the browser owns the save and the page claims no save. |
| Calendar export publication failed | Completion did not produce a successful new output; failure is explicit. In the browser realization, the page could not create the calendar offer. |
| Conversion diagnostic written | A new current-run report was actually persisted at an unused numbered path. The browser realization is **Conversion diagnostic offered**: the page made the diagnostic text available and wrote nothing. |
| Conversion diagnostic offered | The browser realization of Conversion diagnostic written: the page made `<source-stem>.parse-error.txt` available as a download; the browser owns the save. |
| Conversion diagnostic unavailable | The current failure could not save a new report, or the browser page could not offer one; old files prove no new write, and the primary failure stays visible. |
| Source revision received | A revision that changes the site's inputs reached the default branch, or a release tag was pushed; a Site publication may start, and a revision that changes none of them starts none. |
| Site build started | The Site publication resolved the Built revision and began assembling the Site output. |
| Site built | The Site output and its licence file exist, with the footer naming the Built revision; nothing is public yet. |
| Site evidence refused | A required check failed and is named; the previous Public website stays and no content is written. |
| Site published | The hosting target's content was replaced as a whole by one commit; the target holds the complete Site output from the Built revision. |
| Site publication failed | A build, evidence or publish step failed before the replacement; the previous Public website stays and the step is named. |
| Site serving refused | After the replacement, the verified address did not serve the Built revision within the retry bound; the publication is refused, the address and the served revision are named, and the target keeps the new page. |
| Site served | The hosting service serves the published branch, and the publication observed the served page carrying the Built revision. The project's evidence is the page it read, not the service's timing. |

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
drift** remains planned for the drift-report change, and **Deploy key** is
realized by the pages publishing plumbing change (publish-site-plumbing), which
also refines **Protection ruleset** with the pages rule set and its actor-type
bypass. The change to include the report time in the travel title
(include-report-time-in-travel-title) adds **Report time** and refines
**Travel leg**'s contract use, which now names the conditional
` Report <value> LT` title suffix; the source column's printed `Report (LT)`
label names that term and is no longer an unconfirmed raw label. The change to
publish the organization profile (publish-organization-profile) adds seven
technical terms — **Default security policy**, **Default community health
files**, **Organization profile**, **Organization profile repository**,
**Pinned repositories**, **Profile content** and **Reviewed public copy** — as
vocabulary of the project's public presentation; it introduces no new aggregate
or domain rule. The release executable size ideation Handoff
(release-executable-size) adds four technical terms — **Build configuration**,
**Distributed executable**, **Size attribution** and **Size lever** — as
vocabulary of the build and release process; it introduces no aggregate, since
the write boundary is the build configuration, the size-lever decision record
and the built executable. The public website ideation Handoff (public-website)
adds five technical terms — **Browser conversion**, **Public website**, **Site
publication**, **Site output** and **Built revision** — as vocabulary of the
browser delivery channel; it introduces no aggregate, since the conversion
boundary stays the Duty-plan calendar and the site is a published record. The
change seed generalizes the lifecycle events **Calendar export published** and
**Conversion diagnostic written** to name the browser realization without
replacing the executable one; the change to build the browser converter
(build-browser-converter) records the browser observations **Calendar export
offered** and **Conversion diagnostic offered**, adds **Offer**, and clarifies
that **Supported platform** names published archive platforms while the
browser is a supported conversion route with no archive. The change to measure
the executable's size (measure-executable-size) refines **Size attribution**
with the per-revision cadence, the build identity and the command, and adds
**Size attribution report** for the two files a measurement writes and their
three statuses; it introduces no aggregate either, because the measurement
observes a build and commits nothing. The change to reduce the executable's
size (reduce-executable-size) applies **Size lever** and adds **Baseline**,
**Published binary** and **Saved bytes**; it introduces no aggregate either,
because the reduction is a build-configuration decision observed by the
measurement. The change that publishes the Public website
(publish-public-website) adds **no new term**: it realizes **Public website**,
**Site publication**, **Site output** and **Built revision** with their code
symbols, records the site's lifecycle observations (source revision received,
site build started, site built, site evidence refused, site published, site
publication failed, site serving refused, site served), and leaves the
conversion aggregate and its invariants unchanged — the site is a published
record, not a new write boundary. Its serving check is a step of the
**Site publication**, not a term of its own. The generalized duty-plan input
ideation Handoff (generalized-duty-plan-input) adds three terms — **Duty
plan**, **Evidence for a publication** and **Support statement** — and
introduces no aggregate: the intake record remains inside the Duty-plan
calendar boundary, and the statement is a published record. Raw
source-column labels
(A/C, Blocktime, Timezone Difference, Tags) and the FREE/OFF day
codes are used verbatim in the specification and are not confirmed domain terms.

Candidate **Calendar export** remains a topic candidate, not silently confirmed
here. No rejected entry exists to remove.
Suggested FZ expansion and agreement/age/quota provenance remain background, not
leave/eligibility rules.
