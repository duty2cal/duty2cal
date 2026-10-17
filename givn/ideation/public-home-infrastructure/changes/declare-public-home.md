# Change seed: declare-public-home

Status: ready

- Use-case IDs: `adopt-existing-configuration`
- Topic: `public-home-infrastructure`
- Personas: `project-maintainer--organization-owner`, `contributor--infrastructure-reviewer`, `crew-member--personal-calendar-owner`
- Suggested start: 1; no seed dependency.
- Handoff: confirmed by the user on 2026-10-17 (Q12).

## Accepted decisions

### Include the site-publishing plumbing in the topic

- Context: the pages repository exists with a stock Jekyll site, but no site
  source or build workflow exists in the monorepo. The draft nevertheless
  supplied a deploy key and a secret.
- Options:
  1. **Include the plumbing now** — the deploy key, the secret and the Pages
     source are declared and applied with the rest of the public home.
     In practice: the first apply registers a deploy key and stores
     `PAGES_DEPLOY_KEY` while the site remains stock.
     Gain: one bootstrap for the whole public home.
     Cost: a credential exists before its consumer.
  2. **Exclude it until the website exists** — the site topic adds the key and
     workflow together.
     Gain: every credential arrives with its consumer.
     Cost: a second topic revisits the Pages configuration.
- Chosen: **Include the plumbing now** because the user chose option 1 in Q1.
  Rejected: **Exclude it** because the user did not choose the narrower
  boundary.

### Adoption is an all-or-none batch that changes no live object

- Context: the organization, both repositories, the Pages source and the two
  protections already exist. A declaration must bind them without recreating
  or reconfiguring them.
- Options:
  1. **Batch adoption, identity only** — every selected object is bound or none
     is; declared values may differ from live and the difference waits for the
     first preview.
     In practice: adopting the monorepo with projects declared off leaves live
     projects on.
     Gain: no accidental live change during the import; the first preview is
     the only place differences appear.
     Cost: the state and the live values differ until the first apply.
  2. **Adopt and immediately reconcile** — adoption also writes the declared
     values.
     Gain: state and live agree at once.
     Cost: a one-time import silently changes live settings.
- Chosen: **Batch adoption, identity only** because the adoption contract's
  guarantees forbid any live change. Rejected: **Adopt and reconcile** because
  it turns adoption into an unreviewed apply.

## Open questions

### Which setting values are intended? (Q5)

The declaration must state values for projects, merge methods,
delete-branch-on-merge, topics and the organization profile. The draft proposes
projects off and squash-only; live has projects on and all merges allowed.
Preserving live ratifies settings nobody chose; adopting the draft changes them
in the first apply. The proposal must decide setting by setting or choose one
policy.

### Where does the applied state live? (Q4)

The binding must be recorded somewhere the local apply (and later the drift
check) can read. Local-only state supports the apply but not the CI report; an
encrypted remote backend supports both at the cost of a service and a
credential.

### Do the classic protection and the ruleset both become declarations? (Q9)

`main` currently carries both. Adopting both preserves duplication;
consolidating into the ruleset retires the classic protection and changes live
protection; declaring only the ruleset leaves an unmanaged path.
