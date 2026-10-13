@givn.delta @publish-public-release

Feature: Publish a public release

  A released version states the changes a user can see, grouped by kind, drawn
  from the newest section of the change log the repository keeps.

  @givn.added @wip
  Scenario: A version's notes group the changes a reader can see
    Given a history whose released revisions name a new capability, a fix, and work no user can see
    When the maintainer generates the change log for version 0.2.0
    Then the notes for version 0.2.0 name the new capability and the fix
    And the notes name the new capability before the fix
    And the notes name no revision whose work no user can see

  @givn.added @wip
  Scenario: A version that changes nothing a user can see is refused
    Given a history whose released revisions name only work no user can see
    When the maintainer generates the change log for version 0.2.0
    Then the generation fails and states that the version changes nothing a user can see

  @givn.added @wip
  Scenario: The change log keeps the earlier versions beside the new one
    Given a change log whose newest section is version 0.1.0
    When the maintainer generates the change log for version 0.2.0
    Then version 0.2.0 is the newest section of the change log
    And the section for version 0.1.0 is unchanged

  @givn.added @wip
  Scenario: A change log that does not match the released revision is refused
    Given a committed change log whose newest section differs from what the history states
    When the change log is checked against the released revision
    Then the check fails and names the version whose section differs

  @givn.added @wip
  Scenario: Each version's section is dated and compared with the one before it
    Given a change log holding version 0.1.0 and a history releasing version 0.2.0
    When the maintainer generates the change log for version 0.2.0
    Then the section for version 0.2.0 states its own date
    And the section for version 0.2.0 states the comparison against version 0.1.0

  @givn.added @wip
  Scenario: The first released version is recorded from the history
    Given a history whose released revisions name a new capability and work no user can see
    And no change log exists yet
    When the maintainer generates the change log for version 0.1.0
    Then the change log holds one section, for version 0.1.0
    And that section names the new capability
    And that section states no comparison against an earlier version

  @givn.added @e2e @wip
  Scenario: Maintainer generates the change log for a version and reads its newest section
    Given a repository whose history releases version 0.1.0
    When the maintainer generates the change log for version 0.1.0
    Then the change log file holds a section for version 0.1.0
    And that section names the released change a user can see