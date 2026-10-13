# duty2cal

<!-- givn:begin:coverage-badge -->
[![Line Coverage: n/a](https://img.shields.io/badge/line%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
[![Branch Coverage: n/a](https://img.shields.io/badge/branch%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
<!-- givn:end:coverage-badge -->

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
git clone <repository-url> duty2cal
cd duty2cal
cargo build --release -p duty2cal
target/release/duty2cal ./2026-06.pdf
```

The calendar appears beside the source as `./2026-06.ics`.

<!-- givn:begin:run -->
Start the built product on one plan:

```sh
cargo run --release -p duty2cal -- ./2026-06.pdf
```
<!-- givn:end:run -->


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

Quality gates, clean before a change is proposed:

```sh
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --all-targets
```

<!-- givn:end:development -->


## Contributing

- Behaviour is specified in Gherkin under `givn/changes/<change>/specs/` before it
  is implemented, and a scenario is RED before it is GREEN.
- Commits use conventional subjects; one release note is the commit subject and
  the scenario title goes in the body (`givn commit` stamps the revision).
- `./run-tests.sh` and `./run-tests.sh --e2e` must both pass, together with the
  quality gates above.
- Documentation follows arc42 under `docs/arc42/`; architecture decisions are
  recorded as ADRs there.
- Proposed changes name the destination station codes and dates they exercise;
  real duty plans and personal itineraries never enter the repository.
- Sign off your commits with the DCO (`git commit -s`); there is no CLA.

## License

MPL-2.0, project-wide (see [ADR-0001](docs/arc42/adr/ADR-0001-project-license.md)).

Per-file weak copyleft: a redistributor of a modified product must publish the
modified MPL files. This keeps the future iOS UI App-Store distributable.
Contributions use DCO sign-off (`git commit -s`); there is no CLA.
