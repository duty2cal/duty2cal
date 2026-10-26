@givn.delta @publish-public-release
Feature: Publish a public release

  The owning use case is `specs/publish-public-release/usecase.md`, whose copy
  in this change adjusts the supported-platform wording so the browser route is
  not read as an archive platform, and names the site publication the release
  waits for. These scenarios add the three observable outcomes that follow: a
  site revision is announced, the public text distinguishes the routes, and a
  release whose hosting target does not hold the tagged site revision is
  refused.

  @givn.added
  Scenario: A revision that changes the public website is announced in the release notes
    Given a released revision of the capability "publish-public-website" that changed "web/"
    And a released revision that changed only the project's documents
    When the maintainer generates the change log for version 0.2.0
    Then the notes for version 0.2.0 name the change to the public website
    And the notes for version 0.2.0 name no revision that changed only the project's documents

  @givn.added
  Scenario: The public text states the browser route beside the downloadable archive platform
    Given the repository's public text
    When a reader reads the public text
    Then the public text states that a current browser converts a plan on the public website at "https://duty2cal.github.io/"
    And the public text names Windows x86_64 as the only published archive platform
    And the public text states that builds for other operating systems are available on request and that their source can be built with a Rust toolchain
    And the public text names no archive for a platform other than Windows x86_64

  @givn.added @site-publication
  Scenario: A release whose hosting target does not hold the tagged site revision is refused and publishes no release
    Given a prepared release for the version "v0.5.0"
    And a hosting target whose default branch does not hold the Site output built from the tagged revision "v0.5.0"
    When the maintainer publishes the release for "v0.5.0"
    Then the release is refused and names the site revision the hosting target holds
    And no release is published for "v0.5.0"
    And the prepared release's staged archive is unchanged
