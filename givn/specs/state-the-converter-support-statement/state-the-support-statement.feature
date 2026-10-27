@documentation
Feature: State the support statement

  @e2e
  Scenario: Release owner runs the support-statement check and every carrier carries the statement
    Given  the website, the README, the organization profile, chapter 3 and both issue forms carry the support statement
    And  the statement names a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export, its boundary and the non-affiliation sentence
    And  the generated fixture for that plan converts through the unchanged conversion
    When  the release owner runs the support-statement check
    Then  the check exits zero and prints "support-statement: clean — 5 carriers agree; the evidence record names apps/cli/tests/steps/support.rs and Crew member converts a supported duty plan and receives a calendar file"
    And  the check names no carrier as differing, as carrying the withdrawn wording or as promising future support

  Scenario: A carrier whose statement differs is refused and the carrier is named
    Given  four carriers carry the support statement
    And  the README still carries an earlier statement that names a crew duty plan without its operator or export
    When  the release owner runs the support-statement check
    Then  the check is refused and names the README
    And  the check reports the other four carriers as carrying the statement

  Scenario: A carrier that still carries the export-family wording is refused and the wording is named
    Given  every carrier carries the support statement
    And  a proposal issue form still offers an export family and the withdrawn Released Roster detail-table wording
    When  the release owner runs the support-statement check
    Then  the check is refused and names the proposal issue form and the withdrawn wording
    And  no statement is published as agreed across the carriers

  Scenario: A statement whose evidence record names no fixture is refused
    Given  every carrier carries the support statement
    And  the evidence record names neither the generated fixture nor its conversion scenario
    When  the release owner runs the support-statement check
    Then  the check is refused and names the evidence record and the missing fixture
    And  the previously true statement stays in place

  Scenario: A fixture failure shrinks the statement and the refusal names what could not be read
    Given  the generated fixture for the named plan prints an activity code the conversion does not know
    When  the conversion runs and the release owner revises the statement
    Then  the conversion is refused and names the unknown activity code
    And  every carrier states that the plan is currently refused instead of that it converts
    And  the boundary and the non-affiliation sentence are unchanged

  Scenario: A statement that promises a plan's future support is refused
    Given  every carrier carries the support statement
    And  the README states that the converter "will support" a cockpit duty plan
    When  the release owner runs the support-statement check
    Then  the check is refused and names the README and the future-support wording

  Scenario: The evidence record states what the sample showed and supplies no sample
    Given  the named plan's evidence record in the repository
    When  a reader reads the evidence record
    Then  it names the generated fixture and the conversion scenario that exercises it
    And  it states the fictional roster month 2099-06, the printed column labels, the activity codes and the travel leg the fixture prints
    And  it states that no real plan and no identifying value is committed
    And  it carries no real plan, no personal identifying value and no committed sample
    And  it carries no complete calendar date, no DTSTART property and no CREATED property

  Scenario: Reporter opens an issue form and reports the outcome
    Given  a crew member's plan did not convert
    When  the crew member opens the bug-report issue form
    Then  the form's introduction states the support statement, its boundary and the non-affiliation sentence
    And  the form asks whether the plan converted
    And  the form asks what the refusal said
    And  the form offers a stripped sample only when one is needed
    And  the form asks no export-family question
