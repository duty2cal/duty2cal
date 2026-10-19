# Public website — use-case definition

Status: revised 2026-10-19 after the Use Case Definition boundary review.
Identifiers below must survive elaboration, change seeds and the permanent leaf
documents.

## Actor-goal cut

| Name and stable use-case ID | Level | Primary Actor | Goal | Boundary | Mutation |
|---|---|---|---|---|---|
| Export a duty-plan calendar (`export-duty-plan-calendar`) — existing permanent leaf, affected by the browser capability | ! | Crew member | Export one published duty plan as an importable personal calendar file. | Duty-plan calendar. The browser is a driving channel; the conversion rules are unchanged. | Changing: offers a new calendar file; the page itself writes nothing. |
| Publish the public website (`publish-public-website`) — new leaf | ! | Release maintainer | Make the Public website at its canonical address serve the Site output built from a named Built revision. | Public website publication: the Site output, the Built revision and the evidence that gates publication. | Changing: publishes a new Public website; changes neither the release archive nor the declared configuration. |

## Affected existing permanent leaves

These are amendments to existing goals, not new use cases.

| Existing leaf | Change | Why it stays one goal |
|---|---|---|
| Export a duty-plan calendar (`export-duty-plan-calendar`) | Adds the browser capability: the page offers the calendar and the diagnostic; the conversion rules are inherited unchanged (Q3, Q4). | The goal is the calendar the Crew member obtains; the browser is a channel under it. |
| Publish a public release (`publish-public-release`) | Adjusts the platform wording so the browser route is not confused with an archive platform; the Windows x86_64 archive contract is unchanged. | The goal is still the release archive; the wording distinguishes a second route. |
| Apply a configuration change (`apply-configuration-change`) | Publishes the corrected organization profile copy from `org/` through the declared configuration (Q7). | The goal is still the live configuration; the copy is declared content. |

The root repository `README.md` is a source file, not declared configuration;
its correction is an ordinary source revision in the site's change.

## Why the browser conversion is not a new use case

The conversion rules are channel-independent: they describe the calendar the
Crew member obtains, not the program that produces it. The publication and
diagnostic realization is channel-specific and is amended by the browser
delta. A browser variant of the action lives under the same goal (sizing rule
2), so this topic adds a capability and a driving adapter under
`export-duty-plan-calendar`; it does not mint a second conversion goal.

## Why "read the website" is not a use case

The Visitor's reading is a read model, not an actor goal with a system
guarantee. The page content is produced by the Site publication (K4) and
served; the `visitor--project-reader` Persona reviews that content. A separate
observing use case would name the channel, not an outcome, and would duplicate
the publish use case's content guarantee.

## Bounded outcomes

### Export a duty-plan calendar — browser capability

- **Trigger:** the Crew member supplies one duty plan through the Public
  website.
- **Success guarantee:** the Crew member obtains a complete RFC 5545 calendar
  file representing every required resolved activity, through the file the
  page offers for saving. The conversion rules, identities, descriptions and
  source family are the permanent contract's; only the offer-and-save channel
  is new.
- **Minimal guarantee:** the page writes nothing and never reports a save it
  did not observe; no existing file is touched by the page; a rejected
  conversion is never reported as success; on failure the page shows the
  reason and attempts to offer a diagnostic text, and a failed diagnostic
  offer does not hide the primary failure.
- **Browser bounds:** a current browser with WebAssembly and ES modules; plans
  up to 20 MiB. A larger plan or a missing capability is refused by name.
  (Q3)
- **Closure test:** after saving the offered file the Crew member can stop; no
  existing file was replaced by the page.
- **Unchanged:** classification, timing, identity, descriptions, RFC 5545
  output and the supported source family.

### Publish the public website

- **Trigger:** a source revision that changes the site's inputs reaches the
  default branch, or a release tag is pushed. The Release maintainer owns the
  site's value and answers for the published page; the path-filtered push or
  the release tag is the observable start (Q8, amended 2026-10-19).
- **Preconditions:** the hosting target exists with the deploy key and secret
  from the prior topic's `publish-site-plumbing` seed, and its Pages source is
  `main` at `/`. That seed is now the `publish-site-plumbing` change; its
  proposal D1 closes the ruleset bypass, and applying the credential is the
  remaining external prerequisite, not a reopened question here.
- **Success guarantee:** the Public website at `https://duty2cal.github.io/`
  serves the Site output built from the named Built revision; the footer names
  and links that revision; the evidence gate passed; and, before it asks for a
  file, the page states what it does, both conversion routes, the source, the
  release, the licence, the privacy rule, the non-affiliation statement and
  the as-is statement (K4).
- **Minimal guarantee:** no source revision is published without passing the
  evidence; a failed build, gate or publication leaves the previous Public
  website in place and names the failure; the published page's source is never
  edited by hand; no release artifact or host setting changes.
- **Closure test:** after the Site publication the maintainer can stop; the
  served page is the built one and the previous page is either replaced as a
  whole or untouched.
- **Open:** no open question blocks this definition. The prior change's
  proposal D1 closes the pages-ruleset bypass; applying the credential is the
  remaining external prerequisite.

## Stakeholder tags

| Use case | Confirmed Persona lenses |
|---|---|
| Export a duty-plan calendar — browser capability | `crew-member--personal-calendar-owner` (primary); `project-maintainer--release-owner`; `contributor--web-reviewer`. |
| Publish the public website | `project-maintainer--release-owner` (primary); `contributor--web-reviewer`; `visitor--project-reader`; `crew-member--personal-calendar-owner`. The organization owner reviews the profile-copy apply in the same change seed, not this use case. |

## Inside the goals, not separate use cases

- Building the Site output, evaluating the evidence and writing the hosting
  target: steps of one Site publication.
- The footer's Built revision, the CSP, the page copy and the licence files:
  content of the Site output, not separate goals.
- The organization profile copy correction: the existing
  `apply-configuration-change` use case, applied in the same change seed after
  the site is live (Q7). The root README correction is an ordinary source
  revision in that change.
- The browser capability's file selection, save offer and diagnostic offer:
  steps of the conversion goal.

## Excluded actor goals

- Reading the Public website: a read model, reviewed by the Visitor Persona.
- Applying the profile copy: `apply-configuration-change`.
- Publishing the Windows release: `publish-public-release`.
- Changing the conversion rules or the supported PDF family.
- A hosted conversion, upload, account or analytics service.
- A custom domain and the project-site redirect (Q6).
- Rotating or adopting the deploy key/secret: the prior topic's seed.
- Calendar-client import and reconciliation.

## Sizing checks

| Rule | Result |
|---|---|
| One goal, one primary actor | The conversion goal is unchanged; the new goal is one phrase for the Release maintainer. The profile copy is a separate use case, not folded in. |
| Goal-shaped, never component-shaped | "Publish the public website" names the public artifact the maintainer makes available, not a framework or a deployment tool. The browser is a channel under the conversion goal, not a goal. |
| Level discipline | Both are `!` leaves. The site publication has one outcome; no `+` summary is needed. |
| Mutation containment | Conversion is changing; site publication is changing. Neither is an observing case. The site publication changes the served page only; the declared configuration is changed by its own use case. |
| Extension discipline | Build, gate, publication and credential failures protect the minimal guarantee; none introduces a new actor goal. |
| Preconditions and flow | The site publication requires the hosting plumbing and a source revision; the conversion requires a browser capability and a plan. The triggers are an actor action or an event, never a desire. |
| Implementation independence | Goal, Trigger, Main flow and Guarantees name the Public website, the Site output and the Built revision, not Pages, the deploy key or the WASM toolchain. The evidence list and host mechanisms stay in Preconditions and Rules. |
| Relationship semantics | No Includes/Extends. The site publication is not a child of the release or configuration use cases; the same change seed may cover it and the profile copy. |

## Boundary

The conversion boundary is the existing **Duty-plan calendar**. The new
boundary is **Public website publication**: the Site output, the Built
revision and the evidence. `Site publication` is not the glossary's
**Publication gate** (a release-time wait) and not **Release preparation** (the
Windows archive); `Public website` is not **Public home** (the source
repository) and not the **Organization profile**.
