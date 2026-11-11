@givn.delta @export-duty-plan-calendar

Feature: Export a duty-plan calendar

  @givn.added
  Scenario: A detail date printed in the previous month is resolved by its weekday
    Given a supported duty plan whose heading names August 2099 and whose detail rows begin with 30 Thu and 31 Fri
    When the crew member converts it
    Then the calendar contains a travel activity on 2099-07-30 titled "CP 1916 ABC-DEF"
    And the calendar contains a travel activity on 2099-07-31 titled "CP 1917 DEF-ABC"
    And the calendar contains an all-day activity on 2099-08-01

  @givn.added
  Scenario: A detail date printed in the next month is resolved by its weekday
    Given a supported duty plan whose heading names August 2099 and whose detail rows end with 31 Mon and 01 Tue
    When the crew member converts it
    Then the calendar contains a travel activity on 2099-08-31 titled "CP 1918 ABC-DEF"
    And the calendar contains a travel activity on 2099-09-01 titled "CP 1919 DEF-ABC"

  @givn.added
  Scenario: A Created footer printed directly below the last detail row is not a detail row
    Given a supported duty plan whose Created footer is printed directly below its last dated row
    When the crew member converts it
    Then the invocation reports success through its process exit status
    And the all-day activity for 2099-06-07 ends on 2099-06-08
    And the published calendar's revision timestamp is 2099-05-21 15:52 UTC
