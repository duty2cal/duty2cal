# duty2cal

<!-- givn:begin:coverage-badge -->
[![Line Coverage: 98%](https://img.shields.io/badge/line%20coverage-98%25-brightgreen)](coverage/cobertura-coverage.xml)
[![Branch Coverage: n/a](https://img.shields.io/badge/branch%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
<!-- givn:end:coverage-badge -->

[![CI](https://github.com/kressnerd/duty2cal/actions/workflows/ci.yml/badge.svg)](https://github.com/kressnerd/duty2cal/actions/workflows/ci.yml)
[![Release](https://github.com/kressnerd/duty2cal/actions/workflows/release.yml/badge.svg)](https://github.com/kressnerd/duty2cal/releases/latest)
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
git clone https://github.com/kressnerd/duty2cal.git duty2cal
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
<https://github.com/kressnerd/duty2cal/releases/latest>, and every release holds
exactly one archive per supported platform, each with its checksum file beside
it. Pick the archive named for your machine:

| Your machine | Archive |
|---|---|
| Windows x86_64 | `duty2cal-<version>-windows-x86_64.tar.gz` |
| Linux x86_64 | `duty2cal-<version>-linux-x86_64.tar.gz` |
| Linux arm64 | `duty2cal-<version>-linux-arm64.tar.gz` |
| macOS arm64 (Apple Silicon) | `duty2cal-<version>-macos-arm64.tar.gz` |
| macOS x86_64 (Intel) | `duty2cal-<version>-macos-x86_64.tar.gz` |

Replace `<version>` with the released version, for example `0.1.0`. Every
archive is a `.tar.gz` holding only that platform's executable — `duty2cal`, or
`duty2cal.exe` on Windows — so one extraction route serves all five platforms:
`tar` reads the archive on Windows 10 and later, on Linux and on macOS, and
nothing has to be installed first.

On Linux and macOS, if the extracted file is not executable, run
`chmod +x duty2cal` once: the `0.1.0` archives store the executable without that
bit, and later releases do not.

```sh
tar -xzf duty2cal-0.1.0-linux-x86_64.tar.gz
./duty2cal ./2026-06.pdf
```

### Verify what you received

Verify the archive's checksum before extracting it, and verify the build
provenance and the bill of materials of the file in hand with the
[GitHub CLI](https://cli.github.com/):

```sh
sha256sum --check duty2cal-0.1.0-linux-x86_64.tar.gz.sha256
gh attestation verify duty2cal-0.1.0-linux-x86_64.tar.gz --repo kressnerd/duty2cal
```

The checksum file covers the archive it names; on macOS, which ships `shasum`
rather than `sha256sum`, run `shasum -a 256 -c duty2cal-0.1.0-macos-arm64.tar.gz.sha256`
instead, and on Windows, which has neither, run
`Get-FileHash duty2cal-0.1.0-windows-x86_64.tar.gz -Algorithm SHA256` in
PowerShell and compare the digest it prints with the one in the `.sha256` file.
`gh attestation verify` proves that the archive was built by this
repository's release automation from the tagged revision, and shows the bill of
materials the build was made from.

### macOS: the one-time approval

The macOS archives are published unsigned, so Gatekeeper quarantines the
executable when the archive arrives through a browser. The dialog means the
archive carries no Apple signature, not that it was tampered with. Approve it
once: right-click the extracted `duty2cal` in Finder and choose **Open**, or use
System Settings → Privacy & Security → **Open Anyway** after the first launch is
blocked. Downloading with `curl` or `gh` and extracting with `tar` clears the
quarantine attribute and needs no approval. Signing and notarising the archives
is out of scope for now (see [ADR-0003](docs/arc42/adr/ADR-0003-release-channel-and-platforms.md)).

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

Line coverage: 98% (871/886)
Branch coverage: n/a (0/0)

Reproduce the result:

```sh
./measure-coverage.sh
./merge-coverages.sh
```
<!-- givn:end:coverage -->
