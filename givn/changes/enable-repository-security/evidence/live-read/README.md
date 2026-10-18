# Live read — 2026-10-18 (post-apply)

One authenticated, read-only read of the public home, captured after the earlier
configuration change (`c87a34d`, "record the applied settings and drop the
transient plan file") and before this change. It is the source of the
`security-applied` base tree and the evidence for the proposal's
"the security change after the earlier apply" wording. No write request was
issued; every command below is a `GET`.

| File | Command | Values used |
|---|---|---|
| `org.json` | `gh api orgs/duty2cal` | display name `duty2cal`; default repository permission `read`; members may create public and private repositories |
| `repo-mono.json` | `gh api repos/duty2cal/duty2cal` | description `Convert a duty plan to a calendar`; projects off, wiki off, downloads off; merge and squash merges off, rebase on, delete-branch-on-merge on; topics `cli, duty, duty-planner, icalendar, pdf, rust, roste`; no `advanced_security` status; `permissions.admin = true` |
| `repo-mono-topics.json` | `gh api repos/duty2cal/duty2cal/topics` | the same topic list |
| `repo-pages.json` | `gh api repos/duty2cal/duty2cal.github.io` | issues, wiki and projects off; Pages enabled; scanning statuses disabled |
| `pages.json` | `gh api repos/duty2cal/duty2cal.github.io/pages` | legacy build from `main` at `/`, HTTPS enforced |
| `rulesets.json` | `gh api repos/duty2cal/duty2cal/rulesets` | one rule set `main protection`, id `24981082` |
| `ruleset-main.json` | `gh api repos/duty2cal/duty2cal/rulesets/24981082` | deletion, non-fast-forward and linear history; no bypass actors |
| `repo-mono-vulnerability-alerts.http` | `gh api --include repos/duty2cal/duty2cal/vulnerability-alerts` | `HTTP/2.0 204 No Content` — Dependabot alerts on |
| `repo-pages-vulnerability-alerts.http` | `gh api --include repos/duty2cal/duty2cal.github.io/vulnerability-alerts` | `HTTP/2.0 204 No Content` |
| `repo-mono-automated-security-fixes.json` | `gh api repos/duty2cal/duty2cal/automated-security-fixes` | `{"enabled":true,"paused":false}` — Dependabot security updates on |
| `repo-pages-automated-security-fixes.json` | `gh api repos/duty2cal/duty2cal.github.io/automated-security-fixes` | `{"enabled":false,"paused":false}` |
| `classic-protection.json` | `gh api repos/duty2cal/duty2cal/branches/main/protection` | `404 Branch not protected` — the classic protection was removed by hand on 2026-10-18 |

## Post-apply read — 2026-10-18

One authenticated, read-only read of the same objects after the operator's
targeted security apply. No write request was issued; every command below is a
`GET`.

| File | Command | Values used |
|---|---|---|
| `post-apply-security.json` | `gh api repos/duty2cal/duty2cal --jq .security_and_analysis` | secret scanning and push protection `enabled`; Dependabot security updates `enabled`; non-provider patterns and validity checks `disabled` |
| `post-apply-repo-settings.json` | `gh api repos/duty2cal/duty2cal --jq '{description,has_projects,has_wiki,has_downloads,allow_merge_commit,allow_squash_merge,allow_rebase_merge,delete_branch_on_merge,topics}'` | the declared repository settings are unchanged: description, projects/wiki/downloads off, merge and squash off, rebase on, delete-branch-on-merge on, the same topics |
| `post-apply-org-settings.json` | `gh api orgs/duty2cal --jq '{default_repository_permission,members_can_create_public_repositories,members_can_create_private_repositories}'` | default repository permission `read`; members may create public and private repositories |

## `security_and_analysis` (monorepo)

```json
{
  "secret_scanning": {"status": "disabled"},
  "secret_scanning_push_protection": {"status": "disabled"},
  "dependabot_security_updates": {"status": "enabled"},
  "secret_scanning_non_provider_patterns": {"status": "disabled"},
  "secret_scanning_validity_checks": {"status": "disabled"}
}
```

No `advanced_security` member is returned. The pages repository reports the
same statuses with `dependabot_security_updates` `disabled`.

## How the fixtures use this read

- `scripts/fake-github/fixtures/security-applied/repos/**` is derived from
  `repo-mono.json`, `repo-pages.json`, `org.json` and the ruleset/pages reads:
  the monorepo and pages field values above, plus the security block on both
  repository bodies. The two settings resources are served from the
  vulnerability-alerts 204 responses and the two automated-security-fixes
  bodies, the same shapes the provider reads.
- `fixtures/adopt-all/` keeps the 2026-10-17 pre-apply values for the
  predecessor's adoption scenarios; it gains the security block from that read,
  not from this one. The contrast between the two trees is what the adoption
  scenario asserts.
- If a later live read differs from `security-applied`, the fixture is stale:
  re-derive it from a fresh read under a new dated directory rather than
  editing the invocation.
