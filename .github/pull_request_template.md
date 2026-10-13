## What this changes

<!-- Behaviour before and after, in one paragraph. -->

## The scenario that carries it

<!-- The Gherkin scenario's title, and the change it belongs to
     (givn/changes/<change>/specs/<capability>/). -->

## The export family and what it exercises

<!-- Which export family, which destination station codes and which dates.
     A real duty plan is never attached — see CONTRIBUTING.md and SECURITY.md. -->

## Checks

- [ ] `./run-tests.sh` passes, and `./run-tests.sh --e2e` passes
- [ ] `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass
- [ ] every commit carries a `Signed-off-by:` trailer (`git commit -s`; there is no CLA)
- [ ] `bash scripts/license-headers.sh`, `bash scripts/pii-guard.sh` and `bash scripts/public-text-check.sh` are clean

## Privacy

- [ ] No real duty plan, no calendar converted from one, and no personal itinerary is attached to this pull request or committed in it.
- [ ] Any fixture, example or golden calendar is generated or stripped, and follows the project's fictional date convention.
