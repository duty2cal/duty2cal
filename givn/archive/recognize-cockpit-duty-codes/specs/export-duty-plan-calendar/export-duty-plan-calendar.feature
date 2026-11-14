@givn.delta @export-duty-plan-calendar

Feature: Export a duty-plan calendar

  @givn.modified
  Scenario Outline: All-day codes ignore printed clocks
    Given a supported duty plan dated <date> with activity code <code> and printed UTC times <start> and <end>
    When the crew member converts it
    Then the calendar contains one all-day activity for <date> whose source code is <code>
    And that activity carries no timed interval

    Examples:
      | date       | code  | start | end   |
      | 2099-01-12 | U     |       |       |
      | 2099-01-13 | OFF   | 23:00 | 01:00 |
      | 2099-01-14 | FZ    | 09:00 | 17:00 |
      | 2099-01-15 | FREE  | 00:00 | 00:00 |
      | 2099-01-16 | U1    | 23:00 | 01:00 |
      | 2099-01-17 | OFF_2 | 23:00 | 01:00 |

  @givn.added
  Scenario Outline: Medical and training duties use their printed UTC window
    Given a supported duty plan dated 2099-01-18 with activity code <code> and source station ABC and resolved UTC start 06:00 and end 10:00
    When the crew member converts it
    Then the calendar contains one timed activity titled "<code>" from 06:00 to 10:00 UTC on 2099-01-18
    And the converter reports no conversion warning

    Examples:
      | code   |
      | MED    |
      | WBT_EM |
      | WBT_GR |
      | WBT_XY |
      | EMCRM  |
      | EBTF3A |
      | EBTF3B |
      | EBTF3C |

  @givn.added
  Scenario Outline: A code outside the confirmed training families still warns
    Given a supported duty plan dated 2099-01-19 with activity code <code> and source station ABC and resolved UTC start 06:00 and end 10:00
    When the crew member converts it
    Then the calendar contains one timed activity titled "unknown duty: <code>" from 06:00 to 10:00 UTC on 2099-01-19
    And the converter reports one conversion warning

    Examples:
      | code   |
      | WBTX   |
      | EBTF4A |
