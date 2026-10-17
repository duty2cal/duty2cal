# Live read — 2026-10-17

One authenticated read (`gh`, `repo` + `read:org`) of the public home, captured
before the declaration was finalised. The files are the raw API responses; no
value in `infra/` is a guess.

| File | Command | Values used |
|---|---|---|
| `org.json` | `gh api orgs/duty2cal` | numeric id `343978778`; profile fields empty; default permission `read`; members may create public and private repositories |
| `repo-mono.json` | `gh api repos/duty2cal/duty2cal` | description, feature booleans, merge strategies |
| `repo-mono-topics.json` | `gh api repos/duty2cal/duty2cal/topics` | `cli`, `duty`, `duty-planner`, `icalendar`, `pdf`, `rust`, `roste` |
| `repo-pages.json` | `gh api repos/duty2cal/duty2cal.github.io` | description `duty2cal GitHub Page`; live features |
| `rulesets.json` | `gh api repos/duty2cal/duty2cal/rulesets` | rule set `main protection`, id `24981082` |
| `ruleset-main.json` | `gh api repos/duty2cal/duty2cal/rulesets/24981082` | deletion, non-fast-forward, linear history; no bypass actors |
| `pages.json` | `gh api repos/duty2cal/duty2cal.github.io/pages` | legacy build from `main` at `/` |
| `classic-protection.json` | `gh api repos/duty2cal/duty2cal/branches/main/protection` | the weaker duplicate protection |

Import identifiers derived from this read:

- organization settings: `343978778`
- repository: `duty2cal`
- pages repository: `duty2cal.github.io`
- rule set: `duty2cal:24981082`
- Pages source: `duty2cal.github.io`
