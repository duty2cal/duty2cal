# Proposal: state-organization-fields

## Release note

> Required. One complete, user-facing sentence naming what this change does.
> Every revision this change records carries the release note as its message subject, with the scenario title in the message body.

The organization's public profile names the project's description, its website, its public mail address and its location, and declares no social account.

## Use-Case Context

- Use-case ID: `apply-configuration-change` (the declared organization settings are applied by that capability; no new capability)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `project-maintainer--organization-owner` — the operator who owns the declaration and runs the apply; `visitor--project-reader` — the reader who meets the organization page before the site. No Persona is invented or promoted.

## Problem / Opportunity

The organization page at `https://github.com/duty2cal` is a public carrier and currently says nothing: the declared profile fields are empty, so a reader who arrives there sees a name and two repositories and no description, no website, no address and no place. The site's Impressum and the profile repository's README both exist, but the organization page — the first thing GitHub shows for the project — does not point at either.

The four fields are one public statement of the same facts the notices already carry. Leaving them empty is not neutral: it hides the mail alias the Impressum publishes and the site the project exists to serve.

## Proposed Solution

The declaration in `infra/main.tf` sets four profile fields, and the operator's ordinary preview-and-apply publishes them:

| Field | Value |
|---|---|
| Description | "Converts a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export into one calendar file." |
| URL (`blog`) | `https://duty2cal.github.io/` |
| Email (public) | `duty2cal662.abridge974@aleeas.com` — the alias the Impressum publishes |
| Location | `Berlin, Germany` |

The description is the one support sentence without the project's name, because GitHub renders the name beside it. The mail address is the Impressum's alias, so a reader who writes from the organization page reaches the same mailbox the notice names; it is not copied into the profile README, which links the notices instead.

`company` stays empty and `twitter_username` stays empty, and no `github_organization_social_account` resource is declared: the project has no social account and no Mastodon account. A social profile would be a second public carrier with its own notice obligations, so it is not added here or by a later value change.

Observable difference for the reader: the organization page states what the project does, opens the website, offers the mail address, and names Berlin. Observable difference for the operator: one preview names exactly the four field changes, and the apply reports them.

## Out of Scope

- A social account, a Mastodon account, a custom social link, or a verification claim: none is declared, and adding one needs its own notice decision.
- The profile README's text and its links, the site's publication, the pinned repositories and the security settings: unchanged.
- The live apply itself: it is the operator's step (token and passphrase live only in that environment), and the state is committed only after a green apply.
- Any other declared object: the change touches the organization settings' profile fields and nothing else.
