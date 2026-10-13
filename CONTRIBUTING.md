# Contributing to duty2cal

Thanks for considering a contribution. This page holds what a change needs to
land; the [README](README.md) points here rather than repeating it.

## The route a change takes

1. **Behaviour first.** Every behaviour is specified in Gherkin before it is
   implemented. A change adds or amends a `.feature` under
   `givn/changes/<change>/specs/<capability>/`, and a scenario is RED before it
   is GREEN: write the step definitions, watch the scenario fail, then make it
   pass. The feature files are the executable specification, not a later
   translation of the code.
2. **One change at a time.** Work is organised as a change under
   `givn/changes/<change>/` (`givn instructions`, `givn status --change <change>`).
   One capability's scenario is one revision.
3. **Commit shape.** Commits use conventional subjects. The release note is the
   commit subject and the scenario title goes in the body, so the release text
   can quote the project's own history. `givn commit -m "<subject>"` stamps the
   revision with the project's attribution.
4. **Sign-off, no CLA.** Every commit carries the Developer Certificate of
   Origin: `git commit -s` adds the `Signed-off-by:` trailer, which states that
   you wrote the change or have the right to submit it. There is no CLA and no
   copyright assignment. A pull request whose commits carry no sign-off fails
   the `dco` check.
5. **Documentation.** Architecture decisions are recorded as ADRs under
   `docs/arc42/adr/`; the durable architecture chapters live under `docs/arc42/`.

## Checks to run before you propose a change

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
./run-tests.sh
./run-tests.sh --e2e
bash scripts/license-headers.sh
bash scripts/pii-guard.sh
bash scripts/public-text-check.sh
nix-shell -p actionlint --run 'actionlint .github/workflows/*.yml'
```

Actionlint is also available as the image the CI job uses:
`docker run --rm -v "$PWD:/repo" -w /repo rhysd/actionlint:1.7.12 .github/workflows/*.yml`.

The full CI set additionally runs `cargo deny check`, `reuse lint`, the
third-party notices drift check (`cargo about generate about.hbs
--output-file THIRD-PARTY-NOTICES.md` committed with no diff) and the coverage
scripts (`./measure-coverage.sh`, `./merge-coverages.sh`). A change does not land
while any of them fails.

Every source file carries an SPDX header on line 1 (or on line 2 after a
shebang): `SPDX-License-Identifier: MPL-2.0`.

## Privacy: never attach a real duty plan

`duty2cal` converts a crew member's own plan, and a plan is personal data.
Therefore:

- never commit, upload or attach a real duty plan, a calendar converted from
  one, or a personal itinerary — not in an issue, not in a pull request, not as a
  test fixture;
- the issue forms and the pull-request template ask which **export family** a
  sample comes from and for a **stripped sample**: the page structure with every
  real name, employee number, date, station and route removed or replaced;
- fixtures, examples and golden calendars in the repository are generated or
  anonymised and follow the project's fictional 2098/2099 date convention;
  `scripts/pii-guard.sh` checks both the tree and the published history for
  plan-shaped material;
- a proposed change names the destination station codes and the dates it
  exercises, so the behaviour is reviewable without anyone's real plan.

A defect that could expose private plan data does not belong in a public issue:
use the confidential route in [SECURITY.md](SECURITY.md).

## Issues and pull requests

- Bug reports and proposals use the forms under
  [`.github/ISSUE_TEMPLATE/`](.github/ISSUE_TEMPLATE/).
- A pull request states what changed, which scenario carries it, the export
  family and the dates it exercises, and confirms that no real plan is attached.

## Code of conduct

Participation is governed by the
[Contributor Covenant 2.1 code of conduct](CODE_OF_CONDUCT.md). Conduct concerns
go to the maintainer by direct message, as the code of conduct states.

## License

Contributions are licensed under MPL-2.0, the project's licence
([LICENSES/MPL-2.0.txt](LICENSES/MPL-2.0.txt)).
