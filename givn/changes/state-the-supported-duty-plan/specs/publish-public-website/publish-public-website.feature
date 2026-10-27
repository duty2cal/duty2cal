@documentation @givn.delta @publish-public-website

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
    And  the what-it-does statement names a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export, one roster month of cabin crew, and the limits that cockpit duties, other Lufthansa duty plans, other Jeppesen exports and another operator are not covered
    And  the non-affiliation statement names Lufthansa, Jeppesen and any duty-plan provider or airline
    And  a panel below the converter panels states the source and the licence, and the footer repeats the non-affiliation, as-is and privacy terms in prose without repeating any statement the panels make
    And  the served page has sent no request to another origin since it loaded
    And  the served page shows its empty state before a plan file is offered
    And  the served page shows its ready state once a supported plan file is offered
    And  the served page meets the visual baseline in the empty state and in the ready state it traverses

  @givn.added @site-publication
  Scenario: A page whose what-it-does statement does not name the supported plan is refused and the statement is named
    Given  a hosting target serving the Site output built from "abc1234"
    And  a source revision whose page states that the tool converts a crew duty plan without naming the operator or the export
    When  the release maintainer publishes the site from that revision
    Then  the publication is refused and names the page-content check and the what-it-does statement
    And  the hosting target still serves the Site output built from "abc1234"
