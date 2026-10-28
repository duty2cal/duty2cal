@givn.delta @export-duty-plan-calendar

Feature: Export a duty-plan calendar

  @givn.modified
  Scenario: An unrecognized activity code with a printed window becomes a timed activity with a warning
    Given  a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF at station ABC and printed UTC start 08:00 and end 15:30
    When  the crew member converts it
    Then  the calendar contains one timed activity titled "unknown duty: ZZ_M_CF" from 08:00 to 15:30 UTC on 2099-06-07 at ABC
    And  the converter reports one conversion warning naming detail row 1 and the date 2099-06-07
    And  the warning names the activity code "ZZ_M_CF"
    And  the converter publishes a warning report beside the source
    And  the converter reports no conversion error
    And  the invocation reports success through its process exit status

  @givn.modified
  Scenario: An unrecognized activity code without printed times becomes an all-day activity with a warning
    Given  a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and no printed UTC times and no route
    When  the crew member converts it
    Then  the calendar contains one all-day activity for 2099-06-07 titled "unknown duty: ZZ_M_CF"
    And  that activity carries no timed interval
    And  the converter reports one conversion warning naming detail row 1 and the date 2099-06-07
    And  the converter publishes a warning report beside the source

  @givn.modified
  Scenario: An unrecognized activity code with an unqualified window across midnight keeps it as the following UTC day
    Given  a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF, one local-date block and unqualified printed UTC clocks 23:10 and 01:05
    When  the crew member converts it
    Then  the calendar contains one timed activity titled "unknown duty: ZZ_M_CF" from 23:10 on 2099-06-07 to 01:05 on 2099-06-08 UTC
    And  the converter reports one conversion warning naming detail row 1 and the date 2099-06-07

  @givn.modified
  Scenario: An unrecognized activity code's activity carries no description
    Given  a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF at station ABC, printed UTC times 08:00 and 15:30, and a printed Trip ID T42
    When  the crew member converts it
    Then  the calendar contains one timed activity titled "unknown duty: ZZ_M_CF" from 08:00 to 15:30 UTC on 2099-06-07 at ABC
    And  that activity carries no description
    And  the converter reports one conversion warning naming detail row 1 and the date 2099-06-07

  @givn.modified
  Scenario: A standby-like code outside the three shift codes still warns
    Given  a supported duty plan dated 2099-11-18 with activity code SB_X_CF and source station FRA and resolved UTC start 08:00 and end 15:30
    When  the crew member converts it
    Then  the calendar contains one timed activity titled "unknown duty: SB_X_CF" from 08:00 to 15:30 UTC on 2099-11-18 at FRA
    And  the converter reports one conversion warning naming detail row 1 and the date 2099-11-18
    And  the warning names the activity code "SB_X_CF"
    And  the warning states that the row was converted as a timed activity using its printed UTC window 08:00-15:30

  @givn.modified @browser
  Scenario: The page shows the warnings and offers both files when a plan converts with warnings
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
    Then  the page shows that the calendar is ready with warnings
    And  the page shows the text "Calendar ICS ready"
    And  the page states "Your calendar is ready to save. 1 row was converted with an unknown duty."
    And  the page shows the text "Unknown duties"
    And  the page lists the unknown duty "07-06-2099 08:00 UTC unknown duty: ZZ_M_CF"
    And  the page offers the calendar labelled "Save 2026-06.ics"
    And  the page offers the warning report "2026-06.parse-warnings.txt" labelled "Save unknown duty warning for publisher"
    And  the page shows the text "You can choose where to save the file."
    And  the page offers the way back "Open new/next duty plan PDF"
    And  the page has not reported that it saved a file
    And  the page has sent no request to another origin since it loaded
    And  the page meets the visual baseline in the ready-with-warnings state

  @givn.added @browser
  Scenario: The page lists an all-day unknown duty with its date and code
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code without printed times
    Then  the page shows that the calendar is ready with warnings
    And  the page lists the unknown duty "07-06-2099 unknown duty: ZZ_M_CF"
    And  the page meets the visual baseline in the ready-with-warnings state

  @givn.modified @browser
  Scenario: The page shows no warning list when the plan converts without warnings
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" whose every dated activity is recognized
    Then  the page shows that the calendar is ready
    And  the page shows the text "Calendar ICS ready"
    And  the page shows the text "You can choose where to save the file."
    And  the page offers the way back "Open new/next duty plan PDF"
    And  the page shows no warning list
    And  the page offers the file "2026-06.ics"
    And  the page offers no warning report
    And  the page meets the visual baseline in the ready state

  @givn.modified @browser
  Scenario: The page states how many rows were converted with warnings
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" containing two rows with unrecognized activity codes and printed windows
    Then  the page shows that the calendar is ready with warnings
    And  the page states "Your calendar is ready to save. 2 rows were converted with unknown duties."
    And  the page lists the unknown duty "07-06-2099 08:00 UTC unknown duty: ZZ_M_CF"
    And  the page lists the unknown duty "08-06-2099 09:00 UTC unknown duty: ZZ_TST"
    And  the page meets the visual baseline in the ready-with-warnings state

  @givn.modified @browser
  Scenario: The page keeps the calendar and the warnings when it cannot offer the warning report
    Given  the converter page served from the built Site output that cannot create a download offer for the warning report
    When  the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
    Then  the page shows that the calendar is ready with warnings
    And  the page lists the unknown duty "07-06-2099 08:00 UTC unknown duty: ZZ_M_CF"
    And  the page still offers the calendar
    And  the page states that no warning report was offered
    And  the page meets the visual baseline in the ready-with-warnings state
