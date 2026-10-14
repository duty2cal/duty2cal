@givn.delta @publish-public-release

Feature: Publish a public release

  Scenario: A version's notes list only the changes a user can receive
    Given a history whose released revisions changed the converter, the release machinery, and both
    When the maintainer generates the change log for version 0.2.0
    Then the section for version 0.2.0 names the change to the converter
    And the section for version 0.2.0 names the change that touched both
    And the section for version 0.2.0 names no change to the release machinery

  @givn.added @wip
  Scenario: A version's notes name a change to the packaging a user receives
    Given a history whose released revisions changed the packaging
    When the maintainer generates the change log for version 0.2.0
    Then the section for version 0.2.0 names the change to the packaging

  Scenario: A configured path a user receives that the repository does not hold is refused
    Given a configured user-receiving path the repository does not hold
    When the maintainer generates the change log for version 0.2.0
    Then the generation fails and names the path missing from the repository
