# Proposal: publish-legal-notices

## Release note

> Required. One complete, user-facing sentence naming what this change does.
> Every revision this change records carries the release note as its message subject, with the scenario title in the message body.

The public website carries an Impressum and a Datenschutzerklärung reachable from every page's footer, and the publication refuses a page set that lacks them.

## Use-Case Context

- Use-case ID: `state-the-public-legal-notices` (new), `publish-public-website` (extended: the two pages, the footer links and the page-content check), `publish-organization-profile` (extended: the profile's link), `publish-public-release` (extended, document-only: the README's legal section and the public-text rule it owns)
- Ideation topic: none — this change has no ideation seed. Its source is the operator's legal review of 2026-10-27, which is not a repository document; every finding the proposal rests on is restated here so a reader does not need that review.
- Seed path: none. No `givn/ideation/<topic>/changes/` seed exists for legal notices, so the seed-stamping and question-disposition obligations do not apply.
- Confirmed Personas: `visitor--project-reader`, `project-maintainer--release-owner`, `contributor--web-reviewer`, `project-maintainer--organization-owner` — all four are confirmed in `givn/personas/` and already named by `publish-public-website`'s Personas; no Persona is invented or promoted here. The visitor is the reader the notices exist for; the release owner answers for them; the web reviewer judges the same-origin and no-third-party claims; the organization owner answers for the profile carrier.

## Problem / Opportunity

The Public website is live and states no operator and no data processing.

- `https://duty2cal.github.io/` is published from the default branch on every site-input revision, and the page carries the conversion, the six public statements, the source and licence panel and the footer. None of them names who publishes the site, where that person can be served, or what happens to the reader's data.
- Under § 18 Abs. 1 MStV, an operator of telemedia that do not serve exclusively personal or family purposes must keep its name and postal address permanently available. The site addresses unknown crew members and is advertised in a public README, so it is not exclusively personal or family use; the commerciality threshold of § 5 DDG is a separate question the notice does not have to settle, because the MStV duty has none.
- Under Art. 13 DSGVO, the operator is controller for the page request and must inform. GitHub, Inc. logs the visitor's IP address when any GitHub Pages page is served and stores it for security purposes; the request also carries referrer, page, and browser and device information. The site itself neither collects nor sets anything — it sets no cookie and loads no third-party asset, which its content security policy and its browser check already enforce — but "no request after load" is not "no processing", and a reader today cannot learn the difference.
- The organization profile and the repository README are separate telemedia of the same operator. Neither carries a notice, and neither links one.
- Consequence: a reader cannot find out who is responsible or what happens to their address; the project's own claim that processing is local is read as a claim that nothing is processed at all; and the operator publishes without the notice the law requires, which carries a fine and a warning risk even for a non-commercial project.

## Proposed Solution

The operator states, once and for every carrier, who publishes the site and how a page request is processed:

- The site serves two more pages: an Impressum and a Datenschutzerklärung. Both are static pages of the same Site output, both on the site's own origin, both under the same content security policy.
- The Impressum states the operator's name, a postal address at which service is possible, and an electronic mail address; that the employer address is a service address for this private project and not the employer's offering; the non-affiliation and trademark sentence; the source repository and the licence; and the as-is sentence. It states no placeholder, no unresolved marker and no reference to the discontinued EU online dispute-resolution platform.
- The Datenschutzerklärung states the controller and how to reach it; that the site is hosted by GitHub, Inc. in the United States, that GitHub logs the visitor's IP address and stores it for security purposes, that GitHub also processes the time of the request, the page, the referrer, and the browser and device information, that a fixed retention period is not published, and that the operator has no access to those logs; the legal basis for that processing; the third-country transfer and its basis; that GitHub uses sub-processors to deliver the pages; that the mail address is a forwarding alias, naming the forwarding provider or stating that there is none, its role, its location and the legal basis; that the conversion runs in the browser with no upload, no storage and no request after the page loads; that the site sets no cookie and loads no third-party content; the reader's rights and the right to complain to a supervisory authority; and a standing date.
- Both pages are written in German, because the addressees of the duty are German authorities and courts, with a short English summary below the German text and the sentence that the German version is the binding one.
- The converter page's footer links both notices, labelled "Impressum" and "Datenschutz"; each notice's footer links the other notice and the converter page, so either is reachable in one step from the converter page, from the other notice and from the site root.
- The publication refuses a page set that lacks either notice, a page that does not link them, a notice that still carries a placeholder or an unresolved marker, a notice that carries the discontinued dispute-resolution reference, and a notice that loads an asset from another origin or carries a script. A refusal names the page and the missing or forbidden element, so a site with half a notice never goes live and the previously published site stays.
- The organization profile and the repository README each carry two labelled links — "Impressum" to `https://duty2cal.github.io/impressum` and "Datenschutz" to `https://duty2cal.github.io/privacy`; the README carries them in a short Legal section that `scripts/public-text-check.sh` refuses to lose. Neither copies the postal address: one source, everywhere else a link, so a changed address is one edit.
- No consent banner appears anywhere: the site set no cookie before this change and sets none after it, and a banner would ask for a consent that is not needed and would itself be wrong.
- The site's own local-processing claim keeps its scope: it is about the conversion, which uploads nothing, and the hosting disclosure lives on the Datenschutzerklärung page rather than in a sentence that would have to grow on every page.

Observable difference for the reader: from any page, two labelled links lead to who publishes the site, to an address that can be served, to an e-mail address, and to a plain statement that the host logs the IP address and what the reader can do about it. Observable difference for the release owner: a revision that drops a notice, its link, or pastes a placeholder into it is refused before the page is replaced. Observable difference for the organization owner: the profile carries the same two links, not a second copy of the address.

## Capability Routing

`givn spec route --change publish-legal-notices` reported no signal ("the top candidates tie at 13.00") and printed no ranked list, so no recommendation column is recorded.

| Proposed capability | Decision | Rationale |
|---|---|---|
| `state-the-legal-notices` | `NEW in state-the-public-legal-notices` | The notices are a standing claim with their own actor goal — the operator states who publishes and what a page request does — their own evidence (the two pages exist, carry the fixed elements, and every carrier links them) and their own refusal. They advance that goal, not the site's publication or the profile apply, so they are a new goal-shaped capability, exactly as the support statement became one beside the carriers that carry it. |
| `publish-public-website` | `EXTEND publish-public-website` | The two pages are members of the Site output, the footer links are page content, and the page-content check already refuses a page set that lacks a required element. This change edits those through that capability; the publication mechanism, the Built revision and the hosting target are untouched. |
| `publish-organization-profile` | `EXTEND publish-organization-profile` | The profile is its own telemedia of the same operator and needs its own notice or link. Its carrier is the reviewed file the configuration apply publishes; the change adds two labelled links to that file and the apply is the operator's step. |
| `publish-public-release` | `EXTEND publish-public-release` | The README is the repository's public text and its promises are owned by that capability's public-text rule. The change adds one Legal section and the rule that the two links survive a README rewrite; the release archive, the platform promise and the release text are unchanged. |

## Out of Scope

- Legal advice and any statement that the notices are legally sufficient: the texts carry the disclosures the operator's review identified, and the operator answers for the wording. A later legal question is its own change.
- Changing the host, the hosting target, the publication mechanism, the Built revision rule or the site's inputs.
- Any analytics, consent management, cookie, third-party asset, custom domain or server-side processing: the site stays a static, same-origin page set.
- The conversion, its statements' wording, its refusals and the calendar contract.
- The employer's written consent for the c/o address, and the concrete postal address and mail address themselves: they are operator-supplied facts, not repository behaviour.
- An English-language Impressum and Datenschutzerklärung as separate binding documents; the English text is a summary under the German one.
- A cookie or consent banner, a dispute-resolution reference, a voluntary code of conduct and a data-protection officer: none of them is owed, and the first is refused by the check.
- The release text beyond announcing the change; the organization profile apply itself, which stays the operator's step.

## Open Questions

### D1 — Which contact block the Impressum carries

- Question: which postal address does the Impressum publish, given that it must be one at which service is possible, and what does publishing it commit the operator to?
- Options:
  1. **The operator's private address** — name and home address in plain text; gain: no third party's consent, no cost, valid across a job change; cost: the home address is public and crawled, and a move is a revision plus a publication; in practice: the page shows "Daniel Kressner, Musterstraße 1, 12345 Musterstadt".
  2. **A c/o address at the employer** — "Name, c/o Employer GmbH, Straße, PLZ Ort"; gain: the home address stays private and the operator is genuinely reachable there; cost: the employer must consent in writing as an organization to accept service and authority mail, the address dies with the job, and the employer's name in a notice for a project it does not run reads as the employer's own offering; in practice: the page shows the employer's name, and a colleague asks the employer about the tool.
  3. **A rented service address** — an impressum service or co-working address that accepts post; gain: neither home nor employer is public; cost: a recurring fee against the no-cost constraint, and a pure mailbox with no activity on site is not accepted as a serviceable address; in practice: 20–40 € per month for an address at which nobody runs the project.
  4. **Publish nothing until the block is decided** — the site goes offline; gain: no breach while the question is open; cost: the browser route and the README's conversion sentence stop working; in practice: `duty2cal.github.io` returns 404.
- Disposition: answered — option 2, chosen by the operator on 2026-10-27. The operator's employer is an IT consultancy, not an airline, so the address does not read as the operator's own duty-plan provider; the non-affiliation sentence keeps its subject, the project, and the notice states explicitly that the employer address is a service address for this private project. Rejected: option 1 publishes the home address, which the operator declined; option 3 spends money against the no-cost constraint; option 4 withdraws the live browser route. The literal block (name, employer, street, postal code, city), the employer's written consent to accept service, and the mail alias are operator-supplied facts and preconditions of the first publication; the operator supplies them by hand, and the page never carries a placeholder in their absence.

### D2 — Which electronic mail address the Impressum publishes

- Question: which mail address satisfies the requirement that the notice allow direct electronic contact, without publishing the operator's private mailbox?
- Options:
  1. **An alias that forwards to the existing mailbox** — a free alias service in front of the operator's mailbox; gain: the private address stays unpublished and the alias is rotated by editing one line; cost: replies run through the alias service and its provider joins the processing described in the Datenschutzerklärung; in practice: the page shows `duty2cal.<tag>@<alias-domain>`, and a spam wave costs a new alias and one revision.
  2. **A separate project mailbox** — a second free mailbox; gain: complete separation, replies come from the project address; cost: a second login that must be read regularly, and an unread legal notice is a breach; in practice: a Proton or second Gmail account checked weekly.
  3. **A plus-address on the private mailbox** — `name+duty2cal@…`; gain: no new service; cost: the tag is trivially stripped, so the private address is effectively published, and replies leave from the base address; in practice: the page shows the plus form and anyone can derive the base address.
  4. **A project domain with mail routing** — `kontakt@duty2cal.<tld>`; gain: the address survives any provider change and pairs with a future domain; cost: a yearly domain fee and, for sending, a paid plan or a foreign SMTP; in practice: an address that outlives the alias service.
- Disposition: answered — option 1, on the operator's review of 2026-10-27, with option 4 named as the later migration when the project takes a domain. Rejected: option 3 publishes the private address; option 2 adds a mailbox the operator must remember to read; option 4 costs money now. The concrete alias is operator-supplied and is the same one edit as D1.

### D3 — Where the German Datenschutzerklärung text comes from

- Question: is the privacy text taken from a generator, or written for this site?
- Options:
  1. **Generator text** — a free German privacy generator with the hosting and contact modules, its attribution line kept; gain: a reviewed template and a maintainer's starting point; cost: the attribution line is a condition of the free licence, so the page cannot be CC0-1.0 like the rest of the project's prose, and a later revision needs the generator run again; in practice: the page ends with "Erstellt mit …" and the site's licence statement has one text it does not cover.
  2. **Text written for this site** — the disclosures the operator's review identified, written once against this page set; gain: the page stays CC0-1.0 like every other page text, one source owns the wording, and a revision edits the page directly; cost: the operator carries the wording, and a new processing activity needs a deliberate edit; in practice: the page lists the host, the IP logging, the legal basis, the transfer, the browser-local conversion, the rights and the standing date, with no third-party attribution.
- Disposition: answered — option 2, decided by this proposal. Rejected: option 1's attribution would carve the privacy page out of the project's licence statement and leave the wording owned by a tool that is not in the repository; the disclosures it would insert are the ones the operator's review already enumerates, so the generator adds a dependency, not information.

### D4 — How a reader reaches the notices

- Question: what reachability does the change guarantee for the notices, beyond the site's own pages?
- Options:
  1. **Footer links on the site, plus two labelled links in the profile and the README** — gain: one step from any page, and the two other telemedia of the same operator carry their own links; cost: three carriers to keep in step, enforced by the content check and the public-text check; in practice: the profile and the README each link "Impressum" and "Datenschutz".
  2. **Footer link on the site only** — gain: one carrier; cost: a reader who meets the project through the profile or the README cannot reach the operator from where they are, and a profile is its own telemedium; in practice: the README links the site, and the reader hunts the footer.
  3. **Copy the address into the profile and the README** — gain: the address is visible without a click; cost: two more copies drift on every change, and a stale address is a fresh breach; in practice: three places to edit on a move, and the check has to compare them.
- Disposition: answered — option 1, on the operator's review of 2026-10-27. Rejected: option 2 leaves two public carriers without a notice; option 3 duplicates the address, which the review explicitly warns against.
