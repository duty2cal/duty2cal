@givn.delta @publish-public-website @site-publication
Feature: Publish the public website

  The owning use case is `specs/publish-public-website/usecase.md`. Every
  scenario states one observable outcome of one Site publication: the hosting
  target holds the complete Site output built from a named Built revision and
  the published address serves it, or the publication is refused and the
  failure is named. A refusal before the replacement leaves the previous page in
  place; a refusal of the serving check happens after it, names the address and
  the revision it serves, and never reports a working page. The
  `@site-publication` tag marks scenarios that drive the publication itself, so
  the publication's own evidence gate can exclude them and never re-enter
  itself; they run in the ordinary suite on landing.

  @givn.added @e2e @site-publication @wip
  Scenario: Release maintainer publishes the site and the hosting target holds the built page
    Given a hosting target whose default branch serves an earlier page
    And a source revision three revisions after the release tag "v0.5.0"
    When the release maintainer publishes the site from that revision
    Then the hosting target holds the Site output built from that revision
    And the hosting target holds exactly the page, the capability gate, the script, the stylesheet, the icon, "pkg/duty2cal_wasm.js", "pkg/wasm-inline.js" and "LICENSE.txt"
    And the earlier page has been replaced as a whole
    And the served address and the footer name the Built revision that revision's own history yields, and the footer links that commit
    And the page header shows the project icon at 48 by 48 with the alternative text "duty2cal", left of the page heading and its subheadline, and the page's tab carries the same icon
    And a panel above the converter panel states what the tool does, the browser route, the archive route, the privacy rule, the non-affiliation statement and the as-is statement before the page asks for a file, the archive route saying that downloads for various operating systems are available on request and that the release page carries the published builds
    And a panel below the converter panels states the source and the licence, and the footer repeats the non-affiliation, as-is and privacy terms in prose without repeating any statement the panels make
    And the served page has sent no request to another origin since it loaded
    And the served page shows its empty state before a plan file is offered
    And the served page shows its ready state once a supported plan file is offered
    And the served page meets the visual baseline in the empty state and in the ready state it traverses

  @givn.added @site-publication
  Scenario: A default-branch revision that changes none of the site's inputs starts no Site publication
    Given a hosting target serving the Site output built from "abc1234"
    And the default branch's previous revision "abc1234"
    And a default-branch revision "def5678" that changes only the project's documents
    When the release maintainer publishes the site from that revision, compared with the previous revision
    Then the Site publication reports that the revision changes none of the site's inputs
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication
  Scenario: A release tag republishes the site from the tagged revision and the footer names the tag
    Given a hosting target serving the Site output built from "abc1234"
    And the release tag "v0.5.0" pushed on a revision "abc1234" that changes none of the site's inputs
    And the publication names no earlier revision to compare against
    When the release maintainer publishes the site from the tagged revision
    Then the Site publication starts from the tagged revision even though it changes none of the site's inputs
    And the hosting target holds the Site output built from "abc1234"
    And the footer names the Built revision "v0.5.0" and links the tagged commit
    And the footer names no tag-distance form

  @givn.added @site-publication
  Scenario: A Built revision that cannot be resolved refuses the publication
    Given a hosting target serving the Site output built from "abc1234"
    And a source revision whose history the repository does not hold
    When the release maintainer publishes the site from that revision
    Then the publication is refused and names the Built revision
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication
  Scenario: A source revision whose working tree carries changes it does not hold is refused
    Given a hosting target serving the Site output built from "abc1234"
    And a working tree at the revision "abc1234" carrying edits to files the revision tracks
    When the release maintainer publishes the site from that revision
    Then the publication is refused and names the unclean revision
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication
  Scenario: A revision whose tree cannot be checked out is refused and the worktree is named
    Given a hosting target serving the Site output built from "abc1234"
    And a source revision "def5678" the repository cannot check out
    When the release maintainer publishes the site from that revision
    Then the publication is refused and names the worktree
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication @wip
  Scenario: A browser build that fails refuses the publication and names the build step
    Given a hosting target serving the Site output built from "abc1234"
    And a source revision whose browser build fails
    When the release maintainer publishes the site from that revision
    Then the publication is refused and names the revision's browser build step
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication @wip
  Scenario: A page missing a public statement is refused and the statement is named
    Given a hosting target serving the Site output built from "abc1234"
    And a source revision whose page carries no privacy statement
    When the release maintainer publishes the site from that revision
    Then the publication is refused and names the missing privacy statement
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication @wip
  Scenario: A page whose script requests another origin after it loads is refused and the browser check is named
    Given a hosting target serving the Site output built from "abc1234"
    And a source revision whose page's script attempts a request to another origin after the page loads
    When the release maintainer publishes the site from that revision
    Then the publication is refused and names the browser check
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication @wip
  Scenario: A page that loads a third-party asset is refused and the page-content check is named
    Given a hosting target serving the Site output built from "abc1234"
    And a source revision whose page loads an asset from another origin
    When the release maintainer publishes the site from that revision
    Then the publication is refused and names the page-content check and the foreign asset
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication @wip
  Scenario: A failing conversion check is refused and the conversion suite is named
    Given a hosting target serving the Site output built from "abc1234"
    And a source revision whose conversion of a supported plan file fails its checks
    When the release maintainer publishes the site from that revision
    Then the publication is refused and names the conversion suite
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication @wip
  Scenario: A revision the publication is asked to compare against cannot be resolved
    Given a hosting target serving the Site output built from "abc1234"
    And a revision "def5678" whose site inputs changed
    And an earlier revision "9999999" that the repository does not hold
    When the release maintainer publishes the site from that revision, compared with that earlier revision
    Then the publication is refused and names the earlier revision
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication @wip
  Scenario: A driver package tree installed from a different lockfile is refused and the lockfile is named
    Given a hosting target serving the Site output built from "abc1234"
    And a source revision that pins different driver packages than the working copy installs
    When the release maintainer publishes the site from that revision
    Then the publication is refused and names the lockfile and the revision to prepare
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication @wip
  Scenario: An asset without licence information is refused and the asset is named
    Given a hosting target serving the Site output built from "abc1234"
    And a source revision that adds a site asset with no licence information
    When the release maintainer publishes the site from that revision
    Then the publication is refused and names the licensing check and the asset
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication @wip
  Scenario: Republishing an unchanged revision reports the target as current and writes nothing
    Given a hosting target holding the Site output built from "abc1234"
    When the release maintainer publishes the site from the revision "abc1234" again
    Then the Site publication reports that the hosting target is already current
    And the hosting target's history holds no new commit

  @givn.added @site-publication @wip
  Scenario: An address that serves an earlier revision refuses the publication and is named
    Given a hosting target serving the Site output built from "abc1234"
    And an address that keeps serving the page built from "abc1234" after the hosting target's content is replaced
    And a source revision "def5678" that changes the page
    When the release maintainer publishes the site from that revision
    Then the publication is refused and names the address and the revision it serves
    And the hosting target holds the Site output built from "def5678"

  @givn.added @site-publication @wip
  Scenario: A hand edit in the hosting target is removed by the whole-content replacement
    Given a hosting target serving the Site output built from "abc1234"
    And a file added to the hosting target by hand
    And a source revision "def5678" that changes the page
    When the release maintainer publishes the site from that revision
    Then the hosting target holds the Site output built from "def5678"
    And the hosting target holds no file the Site output does not contain
    And the page built from "abc1234" is in the hosting target's history

  @givn.added @site-publication @wip
  Scenario: A refused publication into the hosting target leaves the previous page
    Given a hosting target serving the Site output built from "abc1234"
    And a hosting target that refuses the incoming content
    When the release maintainer publishes the site from the revision "def5678"
    Then the publication is refused and names the hosting target
    And the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication @wip
  Scenario: Two publications that begin together are serialized and neither is refused
    Given a hosting target serving the Site output built from "abc1234"
    And two publications of the revision "def5678" that begin together
    When both publications run to completion
    Then the hosting target holds the Site output built from "def5678"
    And neither publication was refused as a non-fast-forward push
