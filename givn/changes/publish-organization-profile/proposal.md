# Proposal: publish-organization-profile

## Release note

The organization's public profile and its default security policy are published
from the reviewed declaration, so a visitor reads an accurate description of the
local Windows converter and a contributor finds the confidential reporting
route.

## Use-Case Context

- Use-case ID: `apply-configuration-change` — "Apply a configuration change".
- Ideation topic: `public-home-infrastructure`.
- Seed path: none — the request is the operator's instruction of 2026-10-18 and
  no ideation change seed exists for this capability. The owning use case's
  goal, guarantees, rules, Personas and interactions are preserved in
  `givn/specs/apply-configuration-change/usecase.md`.
- Confirmed Personas:
  - `project-maintainer--organization-owner` — records the revision, previews
    and applies it, and answers for the published text.
  - `contributor--infrastructure-reviewer` — reads the declaration and the
    reviewed profile and security-policy text without holding the token or the
    passphrase.
  - `crew-member--personal-calendar-owner` — reads the public profile as a
    prospective user of the converter.
  No new Persona is introduced; the three are the owning use case's existing
  lenses, recorded here for review only. The Gherkin Actor remains the
  organization owner.

## Problem / Opportunity

The organization `duty2cal` has no public profile: the organization's profile
fields are empty and the repository GitHub reads for the profile page,
`duty2cal/.github`, does not exist. A visitor who reaches the organization sees
no statement of what the project does, where the source lives or how to get the
tool. The organization's default security policy is likewise absent: the
monorepo carries its own policy, but a repository without one — including
`duty2cal/duty2cal.github.io` — has no organization-wide confidential reporting
route.

The supplied draft would fill that gap with text the repository does not
support today. It says the converter runs in the browser and that releases are
available for Windows, macOS and Linux. The monorepo has no web UI and no WASM
code; `apps/cli` is the only application; the current release support is
Windows x86_64 only; and the live site still serves the stock Jekyll
placeholder rather than a converter. The draft also names a third-party
duty-plan provider, which the owner excludes for naming-rights reasons. A public
profile that promises a browser converter or three platforms would be false the
day it is published, so the opportunity is to publish the truthful version:
a profile that describes the local, Windows-only tool and the reviewed
declaration that produces it.

The second opportunity is structural. The project already declares its public
home under `infra/` and reviews changes to it through the same preview-and-apply
routine as any other configuration change. The profile page and the default
security policy belong in that declaration: their text should be reviewed as
repository content, not typed into a web form where no diff exists.

Pinned repositories are the exception. GitHub lets an organization owner pin up
to six repositories only through the organization's web settings, and the
pinned provider has no resource for them. That remains a one-time manual
operator step and is recorded as such; this change does not invent a mechanism
for it.

## Proposed Solution

- **A third public repository presents the organization.** The declaration
  names the organization profile repository (`duty2cal/.github`), public, with
  its issue tracker, projects and wiki disabled. It does not exist live, so the
  apply creates it; no adoption batch entry is added.
- **The profile page states what the project actually does.** The profile
  content describes the converter as a local tool that reads a crew duty plan
  PDF and writes an iCalendar file on the reader's own machine. It names
  Windows x86_64 as the only distributed platform, points the download at the
  stable release address, links the source and the site, and states that the
  project is not affiliated with any duty-plan provider or airline. It contains
  no browser claim, no three-platform claim and no third-party provider name.
- **The profile page is self-contained.** Every link is absolute. There are no
  images, badges, statistics widgets or other third-party resources, so opening
  the page does not contact another service. This keeps the profile consistent
  with the converter's own no-network promise.
- **The profile page completes the licence statement.** It states that the
  project is MPL-2.0 and that the page's own prose is CC0-1.0, and it attributes
  the third-party Unicode data to Unicode-3.0 with an absolute link to the
  repository's licence text.
- **The same repository supplies the organization's default security policy.**
  A `SECURITY.md` at the profile repository's root is the file GitHub uses for
  a repository without its own policy. It names the private vulnerability
  reporting route, forbids attaching a real duty plan or a converted calendar,
  and states that only the latest published release is supported.
- **The private-reporting route is enabled before publication.** The reviewed
  policy names the private vulnerability reporting route; before the profile is
  applied, the owner enables private vulnerability reporting on
  `duty2cal/duty2cal` by hand, because the pinned provider has no resource for
  it. The setting remains outside the declaration, the applied state and the
  drift check, and neither the fake nor the suite can prove the live setting.
- **The reviewed text is the source; the profile repository is output.** The
  text lives in `org/` in the monorepo and is published by the declaration, so
  it is reviewed, diffed and licensed beside the code. The profile repository is
  never edited by hand.
- **The owner previews and applies as usual.** A preview names the profile
  repository and its two files as the only objects to create and makes no live
  change. The apply creates them and records them in the applied state, and a
  later preview reports no differences. A later revision that changes only the
  profile text updates only that file.
- **The published text is asserted, not trusted.** The capability's scenarios
  compare the published profile content and default security policy with the
  reviewed `org/` source and check the claims: local conversion, Windows x86_64
  only, no browser claim, the input described only as a crew duty plan, the
  explicit non-affiliation sentence, the stable release and repository links,
  the Unicode-3.0 attribution, absolute links only, and no embedded third-party
  resource. The absence of the excluded provider name is a review control, not
  a mechanical deny-list.
- **Pinned repositories stay a manual step.** The owner pins the source
  repository and the site repository in the organization's settings once. The
  step is recorded in the deployment documentation; there is no declared object
  for it and the preview cannot report drift in it.

## Capability Routing

`givn spec route --change publish-organization-profile` was run again after this
proposal existed. It reported `EXTEND enable-repository-security` (score 15.00,
confidence: signal). That recommendation is not followed: its signal is lexical
— the proposal names `enable-repository-security` because it is the sibling
capability under the same use case — and that capability owns the repository's
scanning and Dependabot settings, not the organization's public profile. The
capability proposed here publishes the profile content and the default security
policy through the configuration-apply goal and shares no object, rule or
outcome with the security-settings capability.

| Proposed capability | Route's recommendation | Decision | Rationale (if we deviated from route) |
|---|---|---|---|
| `publish-organization-profile` | `EXTEND enable-repository-security` | `NEW in apply-configuration-change` | The owner's goal — declare the public profile and its default security policy, preview the revision, apply it and read the outcome — belongs to the administration use case, and neither existing capability owns profile publication. The route's signal is lexical: it matches the sibling capability's name and the shared use-case words, not an object or outcome. The owning use case already holds the preview-and-apply contract; this is a new capability under it. |

## Out of Scope

- **The member-only variant.** A private `.github-private` repository with a
  separate member profile is not created; it has no benefit for a public
  open-source project.
- **The other default community health files.** `CONTRIBUTING.md`,
  `CODE_OF_CONDUCT.md` and the issue and pull-request templates stay in the
  monorepo. The profile repository carries the profile content and the security
  policy only; copying the remaining files is a separate change with its own
  review.
- **The organization's profile fields.** The organization's display name,
  description, location, company, blog and social handle stay empty. This
  change adds the profile page content; it does not populate the organization
  settings.
- **Pinned-repository automation.** No provider resource exists, and no API
  script or second write path is introduced. The one-time manual step is
  recorded, not automated.
- **The browser/WASM converter and the website's content.** The site is not
  published or changed by this change. Its repository is linked truthfully as
  generated output that does not host the converter today.
- **A shared disclaimer snippet across repositories.** The site's footer is
  generated in another repository; no cross-repository build step is added.
  The disclaimer is duplicated and reviewed in both places.
- **Security settings on the profile repository.** Its scanning state is
  GitHub's public-repository default and is not declared as a value; the change
  adds no new security-setting declaration.
- **Any product behaviour, release workflow or conversion behaviour.** The
  converter still never contacts GitHub.
- **Other platforms.** No Linux or macOS archive is promised; other platforms
  remain unsupported, untested and undistributed.
- **The third-party provider name.** The new public profile content does not
  contain it; existing repository documents that already mention it are not
  edited by this change.

## Open Questions

### D1 — What may the profile say about how the converter runs?

- Question: the draft says the converter runs entirely in the browser. The
  repository has no web UI and no WASM code, `apps/cli` is the only application,
  and the live site is a stock placeholder. What should the public profile say
  about how a reader uses the tool?
- Options:
  1. **Keep the draft's browser claim** — describe the site as a converter that
     runs in the browser; gain: it advertises the planned browser experience;
     cost: it is false today and the linked site contains no converter; in
     practice: a visitor follows the link and finds a placeholder.
  2. **State the current local converter and keep the privacy promise** —
     describe a tool that reads a plan and writes a calendar on the reader's own
     machine, with no account, no network and no service; gain: true today and
     it preserves the strongest reason to use the tool; cost: it does not
     advertise the future browser application; in practice: the profile says
     "download it" and the download is the Windows executable.
  3. **Omit how it runs** — state only that it converts plans; gain: no false
     claim; cost: loses the privacy statement and leaves the download route
     unexplained.
- Disposition: **answered** — option 2, decided by this proposal on 2026-10-18
  from the repository facts and the owner's instruction to publish current
  truth. Rejected: option 1, because the repository has no browser application
  and the site is a placeholder; option 3, because the local, no-network
  behaviour is the tool's strongest true claim. Realized in the design's
  profile-copy section and asserted by the content-invariant scenario.

### D2 — Which platforms does the profile name?

- Question: the draft says releases exist for Windows, macOS and Linux. The
  current release support is Windows x86_64 only, and older Linux and macOS
  assets are historical. What should the profile state?
- Options:
  1. **Name all three platforms** — match the draft; gain: a broader-looking
     download offer; cost: false for current releases and it invites downloads
     that do not exist; in practice: a macOS reader looks for an archive that is
     not published.
  2. **Name Windows x86_64 only** — state the single supported platform and
     that the source builds elsewhere with a Rust toolchain; gain: true and
     consistent with the release policy and the README; cost: the profile looks
     narrower; in practice: the download line names `Windows x86_64` and the
     archive name.
  3. **Name no platform** — gain: no platform claim; cost: a reader cannot tell
     whether they can run it.
- Disposition: **answered** — option 2, because ADR-0003 as amended and the
  README state Windows x86_64 as the only supported and distributed platform.
  Rejected: option 1 as false today; option 3 as unhelpful. Realized in the
  design and asserted by the content-invariant scenario.

### D3 — How is the third-party duty-plan provider described?

- Question: the draft names the provider whose plans the converter reads. The
  owner has excluded that name from new public content because of possible
  naming rights. How should the profile describe the input?
- Options:
  1. **Keep the provider name** — gain: precise about the input; cost: the
     owner's naming-rights concern and an implied affiliation; in practice: the
     profile names a company the project is not affiliated with.
  2. **Describe the input generically and state non-affiliation** — say "crew
     duty plans (PDF)" and "Not affiliated with any duty-plan provider or
     airline"; gain: accurate, neutral and explicit about the absence of a
     relationship; cost: less specific about the export family; in practice: a
     reader knows the input is a crew duty plan without a company name.
  3. **Omit the input and the disclaimer** — gain: no company name; cost: the
     profile does not say what the tool reads and leaves affiliation unstated.
- Disposition: **answered** — option 2, by the owner's instruction of
  2026-10-18. Rejected: option 1 (naming-rights risk) and option 3 (unclear and
  silent on affiliation). Realized in the profile copy; the content-invariant
  scenario compares the published bytes with the reviewed `org/` source and
  asserts the generic description and the disclaimer. The absence of the
  excluded name is a review control, not a mechanical deny-list.

### D4 — Does the profile link the public site, and how is it described?

- Question: `duty2cal/duty2cal.github.io` exists and is public, but its Pages
  site currently serves the stock Jekyll placeholder, not the converter. Should
  the profile link it, and with what description?
- Options:
  1. **Link it as the usable converter** — gain: a second route to the tool;
     cost: false today; in practice: the link opens a placeholder.
  2. **Link it as generated output that does not host the converter today** —
     gain: true, and it explains the repository's purpose; cost: it advertises a
     placeholder; in practice: the row says "Project website — generated
     output; it does not host the converter today".
  3. **Omit the site** — gain: no placeholder exposure; cost: the repository
     table loses one of the organization's three public repositories and a
     reader cannot find the site.
- Disposition: **answered** — option 2, because it is current truth without a
  promise. Rejected: option 1 as false; option 3 as incomplete for an
  organization profile. Realized in the design's profile copy and asserted by
  the content-invariant scenario.

### D5 — Where is the profile content maintained?

- Question: should the profile text and the default security policy be typed
  directly into the `.github` repository, sourced from the monorepo and
  published by the declaration, or generated by a separate publishing workflow?
- Options:
  1. **Hand-edit the `.github` repository** — gain: no declaration change; cost:
     no reviewable diff beside the code, no REUSE coverage, and a second place
     where public claims live; in practice: a typo fix is a web-form commit
     nobody reviews.
  2. **Source from `org/` and publish through the declaration** — gain: the text
     is reviewed, diffed and licensed in the monorepo, and the profile
     repository is output; cost: the declaration grows two file resources and
     the apply writes them; in practice: a text change is a reviewed revision,
     previewed and applied like any other configuration change.
  3. **A separate publishing workflow** — gain: the monorepo stays the source;
     cost: a second write path with its own credential outside the declaration
     and its preview; in practice: two mechanisms now write public content.
- Disposition: **answered** — option 2, because it fits the existing
  declaration and ADR-0005's one-writer boundary. Rejected: option 1
  (unreviewed) and option 3 (second write path). Realized in the design's
  declaration and test seam.

### D6 — Which license covers the `org/` content?

- Question: the draft assigns CC0-1.0 to the profile and security-policy text,
  but `LICENSES/` contains no CC0-1.0 text today. Which license should apply,
  and how is it recorded?
- Options:
  1. **CC0-1.0 for `org/**`** — gain: public profile text is freely reusable
     and the draft's choice is kept; cost: a new license text and a REUSE
     override must be added; in practice: `LICENSES/CC0-1.0.txt` is added and
     `org/**` is annotated CC0-1.0.
  2. **MPL-2.0 for `org/**`** — gain: one license project-wide and no new
     license text; cost: the public profile text becomes file-level copyleft,
     which is a poor fit for prose meant to be quoted; in practice: a reader who
     reuses the profile text inherits MPL obligations.
  3. **No license annotation** — gain: no new license file; cost: REUSE lint
     fails and the text's reuse terms are unstated.
- Disposition: **answered** — option 1, following the supplied draft, on
  2026-10-18. Rejected: option 2 (wrong fit for prose) and option 3 (unlicensed
  and lint-failing). Realized in the design's licensing section; the
  implementation adds the canonical CC0-1.0 text.

### D7 — How are pinned repositories handled?

- Question: GitHub shows up to six pinned repositories on the organization
  profile, and the organization owner sets them in the web interface. The
  pinned provider has no resource for them. Should this change declare them,
  keep a recorded manual step, or omit pinning?
- Options:
  1. **Declare them** — gain: one reviewed mechanism for everything on the
     profile; cost: no provider resource exists, so this would mean inventing a
     script or an unsupported resource; in practice: a second write path with no
     preview and no drift detection.
  2. **Record a one-time manual operator step** — gain: no invented mechanism,
     and the step is written down in the deployment documentation; cost: the
     owner must do it by hand and a later change is invisible to the preview;
     in practice: the owner pins `duty2cal` and `duty2cal.github.io` once after
     the apply.
  3. **Omit pinning** — gain: no manual step; cost: the profile's overview does
     not present the source or the site, which is most of the page's value.
- Disposition: **answered** — option 2, because no provider resource exists and
  a second write path is rejected. Rejected: option 1 (invented mechanism) and
  option 3 (loses the curated entry points). Realized in
  `docs/arc42/07-deployment-view.md` and recorded as a risk.

### D8 — Which default community health files does this change publish?

- Question: a `.github` repository can supply default community health files
  for every organization repository that lacks its own. Which files does this
  change publish?
- Options:
  1. **The profile content only** — gain: the smallest change; cost: the
     organization still has no default security policy; in practice: a
     repository with no policy has no confidential route.
  2. **The profile content and the security policy** — gain: the two files the
     request names as the profile's purpose and the contributor route; cost:
     two file resources; in practice: the overview has content and a repository
     without a policy inherits the organization's.
  3. **All health files** — copy `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md` and the
     issue and pull-request templates as well; gain: one place for every
     contributor route; cost: scope beyond the request, and the monorepo's own
     files already serve it; in practice: three more files to review and keep in
     sync.
- Disposition: **answered** — option 2, following the supplied draft. Rejected:
  option 1 (leaves the security gap) and option 3 (scope creep). Realized in
  the declaration and the default-security scenario.

### D9 — Is the disclaimer shared with the site footer?

- Question: the draft wants the "not affiliated" disclaimer to have one source
  for the profile and the site footer. The site is generated in another
  repository. Should this change introduce a shared snippet, duplicate the
  sentence and review both, or omit it?
- Options:
  1. **Shared snippet and build step** — gain: one source; cost: a new
     cross-repository build dependency for one sentence; in practice: the site
     build must consume a file from this repository, and a failure leaves the
     footer empty.
  2. **Duplicate the sentence and review both** — gain: no new mechanism and
     each repository remains self-contained; cost: the two copies can diverge;
     in practice: the profile and the site footer each state the disclaimer and
     a review checks both.
  3. **Omit the disclaimer** — gain: nothing to keep in sync; cost: affiliation
     is left unstated, which is the risk the sentence exists to remove.
- Disposition: **answered** — option 2, because the site's generation is
  outside this repository and a cross-repository build step is a new mechanism.
  Rejected: option 1 (new dependency) and option 3 (leaves the risk). Realized
  in the design's content section and recorded as technical debt.
