@givn.delta @export-duty-plan-calendar

Feature: Export a duty-plan calendar

  @givn.added
  Scenario: Standby shift codes keep their own printed windows
    Given a supported duty plan with SB_F_CF 03:30 to 13:00 on 2099-11-15, SB_M_CF 08:00 to 15:30 on 2099-11-16, and SB_S_CF 14:00 to 22:00 on 2099-11-17 at station FRA
    When the crew member converts it
    Then the calendar contains three timed activities titled SB_F_CF, SB_M_CF and SB_S_CF on 2099-11-15, 2099-11-16 and 2099-11-17 from 03:30 to 13:00, 08:00 to 15:30 and 14:00 to 22:00 UTC at FRA
    And the converter reports no conversion warning
    And no warning report is published

  @givn.added
  Scenario: A standby-like code outside the three shift codes still warns
    Given a supported duty plan dated 2099-11-18 with activity code SB_X_CF and source station FRA and resolved UTC start 08:00 and end 15:30
    When the crew member converts it
    Then the calendar contains one timed activity titled "SB_X_CF" from 08:00 to 15:30 UTC on 2099-11-18 at FRA
    And the converter reports one conversion warning naming detail row 1 and the date 2099-11-18
    And the warning names the activity code "SB_X_CF"
    And the warning states that the row was converted as a timed activity using its printed UTC window 08:00-15:30

  @givn.added
  Scenario: Consecutive standby shift days interrupted by a day off stay separate
    Given a supported duty plan with SB_F_CF windows 03:30 to 13:00 on 2099-11-15 and 2099-11-17 and a FREE row on 2099-11-16
    When the crew member converts it
    Then the calendar contains two timed SB_F_CF activities from 03:30 to 13:00 UTC on 2099-11-15 and 2099-11-17 at FRA
    And the calendar contains one all-day day-off activity on 2099-11-16
    And the converter reports no conversion warning
    And no warning report is published

  @givn.added @wip
  Scenario: A standby shift row without printed times rejects the conversion
    Given a supported duty plan dated 2099-11-19 with activity code SB_F_CF at station FRA and no printed UTC times
    When the crew member converts it
    Then no calendar is published
    And the converter reports no conversion warning
    And the converter reports a conversion failure naming "a required timing could not be resolved"
    And the failure report lists that row under "Errors:"
    And the invocation reports failure through a non-zero process exit status
