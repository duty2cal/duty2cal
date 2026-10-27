@givn.delta @publish-public-website @documentation @nested-build
Feature: Publish the public website

  @givn.modified @e2e @site-publication
  Scenario: Release maintainer publishes the site and the hosting target holds the built page
    Given  a hosting target whose default branch serves an earlier page
    And  a source revision three revisions after the release tag "v0.5.0"
    When  the release maintainer publishes the site from that revision
    Then  the hosting target holds the Site output built from that revision
    And  the hosting target holds exactly the page, the capability gate, the script, the stylesheet, the project's vector mark and raster mark, "pkg/duty2cal_wasm.js", "pkg/wasm-inline.js" and "LICENSE.txt"
    And  the earlier page has been replaced as a whole
    And  the served address and the footer name the Built revision that revision's own history yields, and the footer links that commit
    And  the page header shows the project's vector mark at 48 by 48 with the alternative text "duty2cal", left of the page heading and its subheadline
    And  the page's tab declares the project's vector mark first as its icon and the raster mark after it as the alternate for a browser that cannot use the vector, each declaration naming its own type
    And  a panel above the converter panel states what the tool does, the browser route, the archive route, the privacy rule, the non-affiliation statement and the as-is statement before the page asks for a file, the archive route saying that downloads for various operating systems are available on request and that the release page carries the published builds
    And  a panel below the converter panels states the source and the licence, and the footer repeats the non-affiliation, as-is and privacy terms in prose without repeating any statement the panels make
    And  the served page has sent no request to another origin since it loaded
    And  the served page shows its empty state before a plan file is offered
    And  the served page shows its ready state once a supported plan file is offered
    And  the served page meets the visual baseline in the empty state and in the ready state it traverses

  @givn.added @site-publication
  Scenario: A page whose header mark is not the project's vector is refused and the page-content check names the mark
    Given  a hosting target serving the Site output built from "abc1234"
    And  a source revision whose page shows the raster mark in its header in place of the project's vector mark
    When  the release maintainer publishes the site from that revision
    Then  the publication is refused and names the page-content check and the header mark
    And  the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication
  Scenario: A page whose tab declares no raster alternate is refused and the page-content check names the tab declaration
    Given  a hosting target serving the Site output built from "abc1234"
    And  a source revision whose page declares only the vector tab icon
    When  the release maintainer publishes the site from that revision
    Then  the publication is refused and names the page-content check and the missing raster tab declaration
    And  the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication
  Scenario: An output missing an icon member is refused and the page-content check names the member
    Given  a hosting target serving the Site output built from "abc1234"
    And  a source revision whose Site output carries the vector mark and no raster mark
    When  the release maintainer publishes the site from that revision
    Then  the publication is refused and names the page-content check and the missing raster mark
    And  the hosting target still serves the Site output built from "abc1234"

  @givn.added @site-publication
  Scenario: A page carrying a vector mark that is not the project's plain palette vector is refused and the file is named
    Given  a hosting target serving the Site output built from "abc1234"
    And  a source revision whose vector mark embeds a raster image
    When  the release maintainer publishes the site from that revision
    Then  the publication is refused and names the page-content check and the vector mark
    And  the hosting target still serves the Site output built from "abc1234"
