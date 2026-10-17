# Change seed: publish-site-plumbing

Status: ready

- Use-case IDs: `apply-configuration-change`
- Topic: `public-home-infrastructure`
- Personas: `project-maintainer--organization-owner`, `contributor--infrastructure-reviewer`, `crew-member--personal-calendar-owner`
- Suggested start: 3; depends on `declare-public-home`.
- Handoff: confirmed by the user on 2026-10-17 (Q12).

## Accepted decisions

### Create the deploy key and the secret; never adopt them

- Context: the pages repository exists and serves the stock Jekyll site; no
  deploy key and no repository secret exist. The Frame includes the publishing
  plumbing by the user's Q1 choice.
- Options:
  1. **Create both through a configuration change** — the key pair is generated
     once, the public half becomes a write-enabled deploy key on the pages
     repository, the private half becomes `PAGES_DEPLOY_KEY` on the monorepo.
     In practice: the preview names two creations; the apply creates both; the
     site is unchanged.
     Gain: the plumbing is declared and drift-checked; the website topic starts
     from applied state.
     Cost: a credential exists before its consumer; the private half sits in
     the applied state.
  2. **Adopt a key created by hand** — register the key manually, then import.
     Gain: the key material never enters a plan.
     Cost: an unrecorded creation step; the import still stores the reference.
- Chosen: **Create both through the configuration change** because the user
  included the plumbing in Q1 and the adoption contract forbids creating
  objects. Rejected: **Adopt a hand-made key** because it reintroduces the
  click-through the topic exists to remove.

### The stock site stays untouched

- Context: the pages repository serves the default Jekyll page; the site's
  content and build belong to a separate topic.
- Options:
  1. **Leave the site as it is** — this seed only adds the credentials and
     keeps the existing Pages source.
     In practice: `https://duty2cal.github.io/` still serves the stock page.
     Gain: no website work in an infrastructure seed.
     Cost: the credential is dormant until the website topic.
  2. **Replace the site with a placeholder** — publish a "coming soon" page.
     Gain: the plumbing is exercised end to end.
     Cost: the seed takes on website content and a publish workflow it does
     not own.
- Chosen: **Leave the site as it is** because the website's content is outside
  the confirmed Frame. Rejected: **Replace the site** because it exceeds the
  topic's scope.

## Open questions

### May the pages ruleset bypass its publish deploy key? (Q2)

The only writer to the pages repository is the deploy key. A ruleset that
requires pull requests or passing checks without a bypass blocks publishing.
Bypassing the key protects the repository from human mistakes; restricting the
ruleset to deletion and non-fast-forward avoids a bypass actor; omitting it
leaves the repository unprotected.

### Does review gate the apply? (Q10)

The same gating decision as the other apply seeds.

### What may an apply delete, and how are credentials rotated? (Q11)

A future key rotation removes and creates a credential. Whether removal deletes
live configuration or only stops managing it decides whether rotation is one
change or two.
