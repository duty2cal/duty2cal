<p align="center">
  <img src="docs/assets/duty2cal.svg" alt="duty2cal app icon" width="160">
</p>

# duty2cal

<!-- givn:begin:coverage-badge -->
[![Line Coverage: 98%](https://img.shields.io/badge/line%20coverage-98%25-brightgreen)](coverage/cobertura-coverage.xml)
[![Branch Coverage: n/a](https://img.shields.io/badge/branch%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
<!-- givn:end:coverage-badge -->

[![CI](https://github.com/duty2cal/duty2cal/actions/workflows/ci.yml/badge.svg)](https://github.com/duty2cal/duty2cal/actions/workflows/ci.yml)
[![Release](https://github.com/duty2cal/duty2cal/actions/workflows/release.yml/badge.svg)](https://github.com/duty2cal/duty2cal/releases/latest)
[![License: MPL-2.0](https://img.shields.io/badge/license-MPL--2.0-blue)](LICENSES/MPL-2.0.txt)

`duty2cal` turns a released duty plan into a calendar. It reads the PDF a crew
scheduling system publishes for one roster month and writes one RFC 5545 calendar
file beside it, so that month's flights, standbys, non-flight duties, days off and
layovers appear in the calendar the crew member already uses. It is built for the
crew member who holds the plan: no account, no network, no service, one file in
and one file out.

A plan it cannot read as a supported export is never half-converted. It publishes
no calendar, writes a numbered diagnostic naming the source and the reason, and
reports the failure through its exit status, so an unattended run can tell a failed
conversion from a converted month.

## Quick start

```sh
git clone https://github.com/duty2cal/duty2cal.git duty2cal
cd duty2cal
cargo build --release -p duty2cal
target/release/duty2cal ./2026-06.pdf
```

The calendar appears beside the source as `./2026-06.ics`.

<!-- givn:begin:run -->
## Run

Run the tool:

```sh
cargo run --release -p duty2cal -- <duty-plan.pdf>
```

<!-- givn:end:run -->





















## Download a release

Every version tag publishes one release at the stable address
<https://github.com/duty2cal/duty2cal/releases/latest>, and every release
holds exactly one archive with its checksum file beside it.
Windows x86_64 is the only published archive platform.
The table names the one archive:

| Your machine | Archive |
|---|---|
| Windows x86_64 | `duty2cal-<version>-windows-x86_64.tar.gz` |

Replace `<version>` with the released version, for example `0.1.0`. The
archive is a `.tar.gz` holding only `duty2cal.exe`, so one extraction route
serves the one published archive platform:

```powershell
tar -xzf duty2cal-0.1.0-windows-x86_64.tar.gz
.\duty2cal.exe .\2026-06.pdf
```

A current browser converts a plan on the [Public website](https://duty2cal.github.io/), so no download is needed for that route.
Builds for other operating systems are available on request; their source can be built with a Rust toolchain.
No archive is published for any other platform.
A platform may be added back when users request it.

### Verify what you received

Verify the archive's checksum before extracting it, and verify the build
provenance and the bill of materials of the file in hand with the
[GitHub CLI](https://cli.github.com/):

```powershell
Get-FileHash duty2cal-0.1.0-windows-x86_64.tar.gz -Algorithm SHA256
```

The digest must match the one in
`duty2cal-0.1.0-windows-x86_64.tar.gz.sha256`. In a POSIX shell the checksum
file verifies itself:

```sh
sha256sum --check duty2cal-0.1.0-windows-x86_64.tar.gz.sha256
gh attestation verify duty2cal-<version>-windows-x86_64.tar.gz --repo duty2cal/duty2cal
```

`gh attestation verify` proves that the archive was built by this
repository's release automation from the tagged revision, and shows the bill
of materials the build was made from. Every release published after the
repository moved to the `duty2cal` organization carries those attestations;
the releases published before 2026-10-17 no longer have retrievable
attestations, so check those against their published `.sha256` only.

Already-published releases keep their original assets and links; only future releases are Windows-only.

## Prerequisites

- A stable Rust toolchain with Cargo; the workspace uses the 2021 edition.
- Nothing else at run time: the executable is self-contained, with no service,
  database or network access, and it reads only the file it is given.
- Coverage runs additionally need `cargo-llvm-cov` and the LLVM tools
  (`llvm-cov` and `llvm-profdata`).

## Usage

```
usage: duty2cal <duty-plan.pdf>
```

One argument, the supplied plan. On success the calendar is written beside the
source, under the source's name with the `.ics` extension, and its path is
printed:

```sh
$ target/release/duty2cal ./2026-06.pdf
Written: ./2026-06.ics
```

On failure no calendar is published, a diagnostic report is written beside the
source under `<source>.parse-error.txt` (numbered when that name is taken), and
the process exits non-zero:

```sh
$ target/release/duty2cal ./unknown.pdf
conversion failed for ./unknown.pdf: a required activity could not be resolved: the detail table carries no From column
Diagnostic: ./unknown.parse-error.txt
```

Neither an existing calendar nor an existing report is ever overwritten.

<!-- givn:begin:development -->
## Development

Run the project-owned scripts configured in `givn/commands.yaml`:

```sh
./run-tests.sh
./run-tests.sh --e2e
```

`./measure-coverage.sh` writes the non-E2E and E2E Cobertura reports.
`./merge-coverages.sh` combines them into the merged report linked by the
badges above.

Add lint, build, and other project commands alongside these scripts.

<!-- givn:end:development -->



















### Releasing a version

A version's notes are the release-note words of its released revisions, filtered
to the changes a user receives: a revision that changed none of the paths listed
as user-receiving in `cliff.toml` is left out whatever its subject says. Write a
released subject so a crew member can act on it — what the tool does differently
for them — and commit engineering work (the pipeline, the checks, the fixtures,
the documents) with a housekeeping type (`chore`, `docs`, `test`, `build`, `ci`)
so it is never released at all.

A dependency update is the one housekeeping-type exception: a `chore(deps): …`
revision is released when it changed a user-receiving path and its words do not
name the project's own tooling, and a version whose only announced changes are
such updates is releasable. Its updates appear under
**Dependencies**, after Reversals. A dependency bump that changed only the
project's own checks stays unannounced.

Releasing a version gains one step before the tag. The version's section has to
be in the change log already, because the drift check compares the committed
`CHANGELOG.md` against a fresh render of the tagged revision, and the release
workflow draws the release text from that same section.

The generator is pinned to `git-cliff` 2.14.2 (the version recorded in
`cliff.toml`); the renderer refuses any other version, so run it through that
pin:

```sh
released=$(git rev-parse HEAD)
nix-shell -p git-cliff --run \
  'scripts/render-changelog.sh --version <version> --repo . --out CHANGELOG.md'
git add CHANGELOG.md
GIT_COMMITTER_DATE="$(git log -1 --date=iso-strict --format=%cd "$released")" \
  git commit -m "docs(changelog): record <version>"
git tag v<version>
git push --atomic origin <branch> v<version>
```

The renderer has no tag to read yet, so the section's date comes from `released`
— the revision the tag will point at — and the commit that records the section
must carry that same committer date: the tagged revision has to state the date
the section states, or the drift check could never agree with the file. The
branch and the tag are pushed together with `--atomic`, so a rejected branch
push cannot leave the tag published against a revision that never reached the
default branch. Pushing the tag starts the release workflow, whose preparation
checks the committed change log for drift before it stages anything, then stages
the section as the release text and the change log as an asset of the draft
release, which is inspected before it becomes public.


## Contributing

The contribution route is [CONTRIBUTING.md](CONTRIBUTING.md): behaviour is
specified in Gherkin before it is implemented, the checks to run locally, and
the sign-off (`git commit -s`, no CLA). Participation is governed by the
[code of conduct](CODE_OF_CONDUCT.md), and a defect that could expose private
plan data goes through the confidential route in [SECURITY.md](SECURITY.md)
rather than a public issue.

Issue forms and the pull-request template ask for the export family and a
stripped sample. A real duty plan or a personal itinerary never enters the
repository.

## License

`duty2cal` is MPL-2.0, project-wide; the licence text is
[LICENSES/MPL-2.0.txt](LICENSES/MPL-2.0.txt) and the reasoning is
[ADR-0001](docs/arc42/adr/ADR-0001-project-license.md). It is per-file weak
copyleft: a redistributor of a modified product must publish the modified MPL
files, which keeps the future iOS UI App-Store distributable.

The Unicode character data carried by the third-party crates `duty2cal` depends
on is licensed under [Unicode-3.0](LICENSES/Unicode-3.0.txt). That licence
covers the third-party data file only; it is not a second licence for this
project. The complete third-party list is in
[THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md).

Contributions are signed off under the DCO (`git commit -s`); there is no CLA
(see [CONTRIBUTING.md](CONTRIBUTING.md)).

<!-- givn:begin:coverage -->
## Coverage

Merged report: [coverage/cobertura-coverage.xml](coverage/cobertura-coverage.xml)

Line coverage: 98% (880/894)
Branch coverage: n/a (0/0)

Reproduce the result:

```sh
./measure-coverage.sh
./merge-coverages.sh
```
<!-- givn:end:coverage -->
