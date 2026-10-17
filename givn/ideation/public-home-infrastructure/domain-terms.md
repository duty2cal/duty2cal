# Public-home infrastructure — domain terms

Confirmed by the user on 2026-10-17 through the Event Storming term check
(Q8). Tool vocabulary — `plan`, `apply`, `state`, `provider`, `workspace` —
remains mechanism language for design, not a domain term. Tool vocabulary — `plan`, `apply`, `state`, `provider`,
`workspace` — is mechanism language for design, not a domain term.

## Reused confirmed terms (no new entry)

| Term | Source | Note |
|---|---|---|
| Public home | `docs/arc42/12-glossary.md` line 46 | The public repository and release channel; this topic extends the notion to the organization and the pages repository around it, without redefining the term. |
| Default branch | `docs/arc42/12-glossary.md` line 34 | `main`; the ruleset protects it. |
| Gate (on landing) | `docs/arc42/12-glossary.md` line 40 | The checks a change must pass; the ruleset makes them required rather than conventional. |
| Immutable release | `docs/arc42/12-glossary.md` line 49 | Not configurable from code; recorded as a provider gap, not redefined. |

## Candidates

| Term | Status | Definition | Contrast | First use | Collision check |
|---|---|---|---|---|---|
| Declared configuration | confirmed | The organization's and its repositories' configuration as the repository's `infra/` states it should be. | Not the live configuration; not the product's build configuration (`Cargo.toml`, `cliff.toml`). | `frame.md` (Vision) | `grep -ric 'declared configuration' docs/arc42/12-glossary.md` → 0; `grep -ril 'declared configuration' .` (excluding `.git`, `target`, `private`, `archive`) → 0 files before this topic. |
| Configuration drift | confirmed | A difference between the declared configuration and what the organization currently holds. | Not the glossary's **Drift check**, which compares a committed change log with a fresh render. | `frame.md` (Vision) | `grep -ric 'drift' docs/arc42/12-glossary.md` → 4 lines, all the change-log sense; `grep -ril 'configuration drift' .` → 0 files. The new term must always carry the qualifier "configuration". |
| Configuration change | confirmed | A reviewed revision to the declared configuration that reaches GitHub through an apply. | Not a product change; not a release. | `frame.md` (Vision) | `grep -ric 'configuration change' docs/arc42/12-glossary.md` → 0; `grep -ril 'configuration change' .` → 3 files (chapter 11, this topic), none defining it. |
| Deploy key | confirmed | The write-enabled public key registered on the pages repository whose private half lets the monorepo's automation publish the site. | Not a personal SSH key; not the workflow's `GITHUB_TOKEN`. | `frame.md` (Findings) | `grep -ril 'deploy key' .` → 0 files before this topic; glossary → 0. |
| Adoption | confirmed | The one-time act of placing an already-existing GitHub object under the declaration instead of creating it. | Not the everyday apply; not creation; not the prior topic's "contract adoption", which accepted a use-case contract. | `frame.md` (Proposed scope) | Glossary → 0; `grep -ril 'adoption' .` → 8 files, all the prior topic's contract-adoption sense or this topic. The qualifier "object" may be needed in prose. |
| Protection ruleset | confirmed | The repository rule set that constrains changes to a protected branch — deletion, force pushes, required checks, and optionally pull requests — and may grant named actors a bypass. | Not the legacy branch-protection setting as a separate object; not the gate itself, which the ruleset enforces; a required pull-request rule is not implied, because the pages repository's publish deploy key must be able to push. | `frame.md` (Proposed scope) | `grep -ric 'ruleset' docs/arc42/12-glossary.md` → 2 lines, code-symbol references only; `grep -ril 'ruleset' .` → 2 durable documents (chapters 6 and 12), neither defining it. |
