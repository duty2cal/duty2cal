# arc42 assessment — add-nightly-ci

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and goals | No | Goals and stakeholders are unchanged. |
| 2 | Architecture constraints | Yes | The nightly workflow's read-only boundary and no-secret rule are recorded. |
| 3 | Context and scope | No | No external system or product surface changes. |
| 4 | Solution strategy | No | The product, site and release strategies are unchanged. |
| 5 | Building block view | No | No product building block changes; the nightly reuses the existing runner and browser driver. |
| 6 | Runtime view | No | No product runtime flow changes. |
| 7 | Deployment view | Yes | The runner description now includes the nightly workflow. |
| 8 | Cross-cutting concepts | Yes | The test-scope concept now names the per-push smoke, the nightly all-variant run and the 60-day schedule limit. |
| 9 | Architecture decisions | No | No new ADR; ADR-0006 still excludes the passphrase and administer token from CI. |
| 10 | Quality requirements | Yes | New QS-042 for nightly verification. |
| 11 | Risks and technical debt | Yes | R-070's mitigation now names the nightly all-variant run. |
| 12 | Glossary | No | No new domain term. |

## Status

STATUS: DONE
