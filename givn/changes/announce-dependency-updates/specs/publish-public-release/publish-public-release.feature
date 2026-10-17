@givn.delta @publish-public-release

Feature: Publish a public release

  Dependency updates a user receives are announced beside the changes a user
  can see, and a version whose only changes are dependency updates is
  releasable.

  @givn.added
  Scenario: A version whose only changes are dependency updates is announced
    Given a history whose released revisions carry only dependency updates
    When the maintainer generates the change log for version 0.2.0
    Then the notes for version 0.2.0 name the dependency updates
    And the notes name no housekeeping revision

  @givn.added
  Scenario: Identical dependency update subjects collapse to one entry
    Given a history whose dependency updates repeat one subject
    When the maintainer generates the change log for version 0.2.0
    Then the notes name that dependency update once

  @givn.added
  Scenario: A dependency-only version can be prepared
    Given a history whose released revisions carry only dependency updates
    And an executable built for every supported platform
    When the maintainer prepares the release for version 0.2.0
    Then the prepared release holds an archive for every supported platform
    And the release text names version 0.2.0

  @givn.added
  Scenario: The Dependencies group follows Reversals
    Given a history whose released revisions include a fix, a reversal, and dependency updates
    When the maintainer generates the change log for version 0.2.0
    Then the notes name the fix before the dependency update
    And the notes name the reversal before the dependency update
    And the notes name the dependency updates in history order

  @givn.added
  Scenario: A dependency update that changes only the project's checks is not listed
    Given a history whose dependency updates changed only the project's own checks
    When the maintainer generates the change log for version 0.2.0
    Then the notes name the converter fix
    And the notes name no dependency update
    And the notes name no Dependencies group

  @givn.added
  Scenario: A dependency update that changes the release workflow is listed
    Given a history whose dependency update changed the release workflow
    When the maintainer generates the change log for version 0.2.0
    Then the notes name the dependency update

  @givn.modified
  Scenario: A version that changes nothing a user can see is refused
    Given a history whose released revisions name only work no user can see, including dependency updates that changed no path a user receives
    When the maintainer generates the change log for version 0.2.0
    Then the generation fails and states that the version changes nothing a user can see

  @givn.added
  Scenario: A dependency update typed with a capability is announced once
    Given a history whose dependency update carries a capability scope
    When the maintainer generates the change log for version 0.2.0
    Then the notes name the capability-scoped update once
    And the Dependencies group names only the update that carries no capability scope

  @givn.added
  Scenario: The release text and the change log state the same dependency updates
    Given a history whose released revisions carry only dependency updates
    And an executable built for every supported platform
    When the maintainer prepares the release for version 0.2.0
    Then the release text names the dependency updates
    And the staged change log holds the section the release text states
