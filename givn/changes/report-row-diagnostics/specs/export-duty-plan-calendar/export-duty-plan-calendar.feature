@givn.delta @export-duty-plan-calendar

Feature: Export a duty-plan calendar

  @givn.removed
  Scenario: An unknown required activity code rejects the conversion
    Given the scenario is retired by the row-diagnostics change

  @givn.removed
  Scenario: An unknown activity code with no route rejects the conversion
    Given the scenario is retired by the row-diagnostics change

  @givn.added
  Scenario: An unrecognized activity code with a printed window becomes a timed activity with a warning
    Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF at station ABC and printed UTC start 08:00 and end 15:30
    When the crew member converts it
    Then the calendar contains one timed activity titled "ZZ_M_CF" from 08:00 to 15:30 UTC on 2099-06-07 at ABC
    And the converter reports one conversion warning naming detail row 1 and the date 2099-06-07
    And the warning names the activity code "ZZ_M_CF"
    And the converter publishes a warning report beside the source
    And the converter reports no conversion error
    And the invocation reports success through its process exit status

  @givn.added
  Scenario: An unrecognized activity code without printed times becomes an all-day activity with a warning
    Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and no printed UTC times and no route
    When the crew member converts it
    Then the calendar contains one all-day activity for 2099-06-07 titled "ZZ_M_CF"
    And that activity carries no timed interval
    And the converter reports one conversion warning naming detail row 1 and the date 2099-06-07
    And the converter publishes a warning report beside the source

  @givn.added
  Scenario: An unrecognized activity code with a single printed endpoint rejects the conversion
    Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and a printed UTC start but no end and no route
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming "ZZ_M_CF"
    And the failure report lists that row under "Errors:"
    And the invocation reports failure through a non-zero process exit status

  @givn.added
  Scenario: Several unrecognized rows are all reported in one conversion
    Given a supported duty plan with three detail rows carrying unrecognized activity codes, each with printed UTC start and end
    When the crew member converts it
    Then the calendar contains one activity for each of the three rows
    And the converter reports three conversion warnings
    And the converter reports no conversion error
    And the warning report lists one entry for each warned row

  @givn.added
  Scenario: Several failing rows are all reported with the warnings in one failure report
    Given a supported duty plan with two detail rows that cannot be represented and one unrecognized row that can
    When the crew member converts it
    Then no calendar is published
    And the converter reports two conversion errors
    And the converter reports one conversion warning
    And the failure report lists both errors and the warning
    And the invocation reports failure through a non-zero process exit status

  @givn.added
  Scenario: Error and warning entries are distinguishable in the failure report
    Given a supported duty plan with one detail row that cannot be represented and one unrecognized row that can
    When the crew member converts it
    Then the failure report carries an "Errors:" section listing one entry
    And the failure report carries a "Warnings:" section listing one entry
    And the failure report identifies the source PDF and the primary reason

  @givn.added
  Scenario: A second conversion with warnings uses the next unused warning-report name
    Given a supported duty plan with an unrecognized activity code beside existing reports roster.parse-warnings.txt and roster.parse-warnings-1.txt
    When the crew member converts it
    Then the warning report is written as roster.parse-warnings-2.txt
    And roster.parse-warnings.txt and roster.parse-warnings-1.txt are unchanged

  @givn.added
  Scenario: The executable prints each warning beside the new calendar path
    Given a supported duty plan with two detail rows carrying unrecognized activity codes
    When the crew member converts it
    Then the transcript names the newly published calendar
    And the transcript carries one warning line for each warned row
    And the transcript names the warning report

  @givn.added
  Scenario: A conversion without warnings publishes no warning report
    Given a supported duty plan whose every dated activity carries a recognized activity code
    When the crew member converts it
    Then a complete calendar is published
    And no warning report is published
    And the transcript reports no conversion warning
    And the invocation reports success through its process exit status

  @givn.added
  Scenario: A warning report that cannot be published keeps the calendar and the warnings visible
    Given a supported duty plan with an unrecognized activity code and no writable destination for a new warning report
    When the crew member converts it
    Then the calendar is published
    And the invocation reports success through its process exit status
    And the transcript carries the warning line
    And the transcript states that no new warning report was saved

  @givn.added @browser @wip
  Scenario: The page shows the warnings and offers both files when a plan converts with warnings
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
    Then  the page shows that the calendar is ready with warnings
    And  the page lists one warning naming the activity code
    And  the page offers the file "2026-06.ics"
    And  the page offers the warning report "2026-06.parse-warnings.txt"
    And  the page has not reported that it saved a file
    And  the page has sent no request to another origin since it loaded
    And  the page meets the visual baseline in the ready-with-warnings state

  @givn.added @browser @wip
  Scenario: The page lists the errors and the warnings when a plan cannot be converted
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" with a row that cannot be represented and a row that was converted with a warning
    Then  the page shows a failure heading and the count-based reason sentence
    And  the page lists the error naming the unrepresentable row
    And  the page lists the warning naming the converted row
    And  the page offers no calendar
    And  the page offers the diagnostic text "2026-06.parse-error.txt"
    And  the page offers a way to choose another plan
    And  the page meets the visual baseline in the failure state

  @givn.added @browser @wip
  Scenario: The page shows no warning list when the plan converts without warnings
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" whose every dated activity is recognized
    Then  the page shows that the calendar is ready
    And  the page shows no warning list
    And  the page offers the file "2026-06.ics"
    And  the page offers no warning report
    And  the page meets the visual baseline in the ready state

  @givn.added
  Scenario: A timed activity code with a route and an incomplete printed interval rejects the conversion
    Given a supported duty plan dated 2099-06-07 with activity code TK, a printed route from ABC to DEF, a printed UTC start 08:00 and no end
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "a route row carries an incomplete printed interval"
    And the failure report lists that row under "Errors:"
    And the invocation reports failure through a non-zero process exit status

  @givn.added
  Scenario: An unrecognized activity code with a route and a single printed endpoint rejects the conversion
    Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF, a printed route from ABC to DEF, a printed UTC start 08:00 and no end
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "a route row carries an incomplete printed interval"
    And the failure report lists that row under "Errors:"
    And the invocation reports failure through a non-zero process exit status

  @givn.added
  Scenario: A known timed activity code that also prints a route stays a timed non-flight activity
    Given a supported duty plan dated 2099-06-07 with activity code TK, a printed route from ABC to DEF and printed UTC times 08:00 and 12:00
    When the crew member converts it
    Then the calendar contains one timed non-flight activity from 08:00 to 12:00 UTC on 2099-06-07 lasting 4 hours
    And the calendar contains no travel activity on 2099-06-07
    And the converter reports no conversion warning
    And no warning report is published

  @givn.added
  Scenario: A travel leg with a flight number produces no warning
    Given a supported duty plan with a travel leg printing flight number LU742 from ABC to DEF with printed UTC times 09:00 and 10:00 and no crew position
    When the crew member converts it
    Then the calendar contains one travel activity from 09:00 to 10:00 UTC on the duty-plan date
    And the converter reports no conversion warning
    And no warning report is published

  @givn.added
  Scenario: A silent layout row produces no warning
    Given a supported duty plan with a detail row that carries only a date and no activity code
    When the crew member converts it
    Then the calendar contains no absence or layover for that date
    And the converter reports no conversion warning
    And no warning report is published

  @givn.added
  Scenario: A silent layout row before a warned row keeps its place in the row numbering
    Given a supported duty plan whose first detail row is a silent layout row carrying only a date and whose second detail row carries the unrecognized activity code ZZ_M_CF with printed UTC times 08:00 and 15:30
    When the crew member converts it
    Then the converter reports one conversion warning naming detail row 2 and the date 2099-06-07
    And the warning names the activity code "ZZ_M_CF"

  @givn.added @wip
  Scenario: An unmatched departure and a later warning are reported in plan order
    Given a supported duty plan with an unmatched departure fragment on an earlier detail row and an unrecognized activity code with printed UTC times on a later detail row
    When the crew member converts it
    Then no calendar is published
    And the failure report lists the departure row's error before the later row's warning
    And the failure report's reason line is the departure row's error
    And the invocation reports failure through a non-zero process exit status

  @givn.added @wip
  Scenario: An unrecognized activity code with an unqualified window across midnight keeps it as the following UTC day
    Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF, one local-date block and unqualified printed UTC clocks 23:10 and 01:05
    When the crew member converts it
    Then the calendar contains one timed activity titled "ZZ_M_CF" from 23:10 on 2099-06-07 to 01:05 on 2099-06-08 UTC
    And the converter reports one conversion warning naming detail row 1 and the date 2099-06-07

  @givn.added @wip
  Scenario: An unrecognized activity code with an equal printed window rejects the conversion
    Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and equal printed UTC clocks 10:00 and 10:00
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "a timed interval must end after it starts"
    And the invocation reports failure through a non-zero process exit status

  @givn.added @wip
  Scenario: A mis-paired travel leg reports both the arrival error and the orphan departure
    Given a supported duty plan whose departure fragment prints crew position DH and flight number 124 and whose arrival fragment prints flight number 124 without a crew position
    When the crew member converts it
    Then no calendar is published
    And the converter reports two conversion errors
    And the failure report lists the departure row's error before the arrival row's error
    And the invocation reports failure through a non-zero process exit status

  @givn.added @wip
  Scenario: An unrecognized activity code's activity carries no description
    Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF at station ABC, printed UTC times 08:00 and 15:30, and a printed Trip ID T42
    When the crew member converts it
    Then the calendar contains one timed activity titled "ZZ_M_CF" from 08:00 to 15:30 UTC on 2099-06-07 at ABC
    And that activity carries no description
    And the converter reports one conversion warning naming detail row 1 and the date 2099-06-07

  @givn.added @wip
  Scenario: The warning report identifies the source and carries a warnings section
    Given a supported duty plan dated 2099-06-07 with activity code ZZ_M_CF and printed UTC times 08:00 and 15:30
    When the crew member converts it
    Then the warning report's first line states the source PDF "roster.pdf"
    And the warning report carries a "Warnings:" section listing one entry
    And that entry names detail row 1 and the date 2099-06-07

  @givn.added @browser @wip
  Scenario: The page states how many rows were converted with warnings
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" containing two rows with unrecognized activity codes and printed windows
    Then  the page shows that the calendar is ready with warnings
    And  the page states "Your calendar is ready to save. 2 rows were converted with warnings."
    And  the page meets the visual baseline in the ready-with-warnings state

  @givn.added @browser @wip
  Scenario: The page offers a warning report that names the source and carries the warnings
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
    Then  the page offers the warning report "2026-06.parse-warnings.txt"
    And  the offered warning report begins with "Source PDF: 2026-06.pdf" and carries a "Warnings:" section
    And  the offered warning report's entry names the detail row and its date
    And  the page meets the visual baseline in the ready-with-warnings state

  @givn.added @browser @wip
  Scenario: A second attempt clears the previous attempt's warning list
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
    And  the crew member supplies the supported duty plan "2026-07.pdf" whose every dated activity is recognized
    Then  the page shows that the calendar is ready
    And  the page shows no warning list
    And  the page offers no warning report
    And  the page offers the file "2026-07.ics"
    And  the page meets the visual baseline in the ready state

  @givn.added @browser @wip
  Scenario: The page keeps the calendar and the warnings when it cannot offer the warning report
    Given  the converter page served from the built Site output that cannot create a download offer for the warning report
    When  the crew member supplies the supported duty plan "2026-06.pdf" containing an unrecognized activity code with a printed window
    Then  the page shows that the calendar is ready with warnings
    And  the page lists one warning naming the activity code
    And  the page still offers the calendar
    And  the page states that no warning report was offered
    And  the page meets the visual baseline in the ready-with-warnings state
