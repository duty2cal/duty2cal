@givn.delta @export-duty-plan-calendar

Feature: Export a duty-plan calendar

  @givn.added
  Scenario: A travel activity's title ends with its report time in local time
    Given a supported duty plan travel leg supplying Trip ID T42, Report (LT) 07:30, A/C ABC, Blocktime 02:00, Timezone Difference +01, Layover duration 03:00, and Tags 5_MFL
    When the crew member converts it
    Then that activity's title is "LU742 ABC-DEF Report 07:30 LT"

  @givn.added
  Scenario: A travel activity without a report time keeps its title unchanged
    Given a supported duty plan travel leg without a report time
    When the crew member converts it
    Then that activity's title is "LU742 ABC-DEF"

  @givn.added @wip
  Scenario: Differing report times leave the title without a report suffix
    Given a compatible departure and arrival fragment of one travel leg supplying different report times 07:30 and 08:00
    When the crew member converts it
    Then that activity's title is "LU742 ABC-DEF"
    And the travel activity's description states "Report (LT) 07:30 (departure fragment)" and "Report (LT) 08:00 (arrival fragment)"

  @givn.added @wip
  Scenario: Equal report times on both fragments produce one title suffix
    Given a compatible departure and arrival fragment of one travel leg supplying the same report time 07:30
    When the crew member converts it
    Then that activity's title is "LU742 ABC-DEF Report 07:30 LT"
