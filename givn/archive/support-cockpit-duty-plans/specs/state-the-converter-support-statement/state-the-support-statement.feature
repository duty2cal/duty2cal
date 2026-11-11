@givn.delta @state-the-support-statement @documentation

Feature: State the support statement

  @givn.modified @e2e
  Scenario: Release owner runs the support-statement check and every carrier carries the statement
    Given  the website, the README, the organization profile, chapter 3 and both issue forms carry the support statement
    And  the statement names a Lufthansa crew duty plan published as a Jeppesen PDF export, its boundary and the non-affiliation sentence
    And  the generated fixtures for the cabin-crew and cockpit-crew plans convert through the unchanged conversion
    When  the release owner runs the support-statement check
    Then  the check exits zero and prints "support-statement: clean — 5 carriers agree; the evidence record names apps/cli/tests/steps/support.rs and Crew member converts a supported duty plan and receives a calendar file"
    And  the check names no carrier as differing, as carrying the withdrawn wording or as promising future support

  @givn.added
  Scenario: The evidence record names both crew-role fixtures that exercise the claim
    Given  the supported crew duty plan's evidence record in the repository
    When   a reader reads the evidence record
    Then   it names the cabin-crew fixture and the cockpit-crew fixture
    And    it names the scenario that converts the cockpit-crew fixture
    And    it states that no real plan and no identifying value is committed
