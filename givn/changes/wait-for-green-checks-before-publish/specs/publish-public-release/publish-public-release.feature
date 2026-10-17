@givn.delta @publish-public-release

Feature: Publish a public release

  @givn.added
  Scenario: A tagged revision whose checks are still running is not published until they conclude
    Given a tagged revision whose checks are still running
    When the publication gate examines the tagged revision
    Then the gate reports the still-running checks
    And the gate reports the tagged revision ready only after every check has concluded successfully

  @givn.added
  Scenario: A completed check that is not successful refuses publication
    Given a tagged revision whose checks include a completed failure and a check still running
    When the publication gate examines the tagged revision
    Then the gate refuses publication, naming the failed check
    And the gate does not wait for the check still running

  @givn.added
  Scenario: A check that does not conclude within the bounded wait refuses publication
    Given a tagged revision whose checks never conclude
    When the publication gate examines the tagged revision until the elapsed limit
    Then the gate refuses publication, naming the still-running checks

  @givn.added
  Scenario: A skipped or neutral check does not refuse publication
    Given a tagged revision whose completed checks are one success, one skipped, and one neutral
    When the publication gate examines the tagged revision
    Then the gate reports the tagged revision ready to publish

  @givn.added
  Scenario: A tagged revision whose checks all concluded without success is refused
    Given a tagged revision whose completed checks are one skipped and one neutral
    When the publication gate examines the tagged revision
    Then the gate refuses publication, stating that no check concluded successfully

  @givn.added
  Scenario: The check-run source cannot be read
    Given a tagged revision whose check runs cannot be read
    When the publication gate examines the tagged revision
    Then the gate refuses publication, stating that the check runs could not be read

  @givn.added
  Scenario: A tagged revision with no check evidence is refused
    Given a tagged revision whose only check runs belong to the release workflow
    When the publication gate examines the tagged revision
    Then the gate refuses publication, stating that the revision carries no check evidence

  @givn.added
  Scenario: A branch and its tag pushed together publish only after the checks in both suites conclude
    Given the default branch and a version tag pushed together, each starting a check suite on the same revision
    When the publication gate examines the tagged revision
    Then the gate waits for both suites' checks instead of refusing them as not yet concluded
    And the gate reports the revision ready to publish only once every check in both suites has concluded successfully
