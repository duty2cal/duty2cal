# duty2cal

<!-- givn:begin:coverage-badge -->
[![Line Coverage: n/a](https://img.shields.io/badge/line%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
[![Branch Coverage: n/a](https://img.shields.io/badge/branch%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
<!-- givn:end:coverage-badge -->

<!-- givn:todo: One-paragraph description of what this project does and who it is for. -->

## Quick start

<!-- givn:todo: Exact commands to clone and build the project. -->

<!-- givn:begin:run -->
<!-- givn:todo: set run.command and run.product_type in givn/commands.yaml -->
<!-- givn:end:run -->


## Prerequisites

<!-- givn:todo: Required tools, languages, or runtimes. -->

## Usage

<!-- givn:todo: A brief example or --help output showing how to use the project. -->

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


## Contributing

<!-- givn:todo: PR guidelines, commit conventions, code style, review process. -->

## License

MPL-2.0, project-wide (see [ADR-0001](docs/arc42/adr/ADR-0001-project-license.md)).

Per-file weak copyleft: a redistributor of a modified product must publish the
modified MPL files. This keeps the future iOS UI App-Store distributable.
Contributions use DCO sign-off (`git commit -s`); there is no CLA.
