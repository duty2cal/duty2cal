@givn.delta @export-duty-plan-calendar

Feature: Export a duty-plan calendar

  Scenario: A crew position and its flight number are printed as separate words
    Given a supported duty plan whose travel leg prints crew position FB and flight number 123 from ABC to DEF with resolved UTC start 08:00 and end 09:00
    When the crew member converts it
    Then that activity's title is "FB 123 ABC-DEF"

  @givn.added @wip
  Scenario: A travel leg that prints no crew position keeps its flight number alone
    Given a supported duty plan whose travel leg prints flight number 123 from ABC to DEF with resolved UTC start 08:00 and end 09:00 and no crew position
    When the crew member converts it
    Then that activity's title is "123 ABC-DEF"

  @givn.added @wip
  Scenario: A crew position printed beside a non-flight activity stays out of its title
    Given a supported duty plan whose dated row prints crew position AK beside activity code EM with resolved UTC start 06:00 and end 10:00
    When the crew member converts it
    Then that activity's title is "EM"

  @givn.added @wip
  Scenario: An unmatched travel fragment is named with its position and flight number separate
    Given a supported duty plan whose unmatched travel fragment prints crew position FB and flight number 123
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "FB 123"

  Scenario: A positioning departure fragment has no partner when its arrival fragment prints no crew position
    Given a supported duty plan whose departure fragment prints crew position DH and flight number 124 and whose arrival fragment prints flight number 124 without a crew position
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "no unique compatible partner"
