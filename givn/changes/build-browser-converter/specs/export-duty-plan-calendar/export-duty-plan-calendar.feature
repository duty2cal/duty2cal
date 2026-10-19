@givn.delta @export-duty-plan-calendar
Feature: Export a duty-plan calendar

  @givn.added @e2e @wip
  Scenario: Crew member converts a supported duty plan in the browser and saves the offered calendar
    Given the converter page served from the built Site output
    Then the page shows the empty state and offers no calendar
    When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
    Then the page shows that the calendar is ready
    And the page offers the file "2026-06.ics"
    When the crew member saves the offered calendar
    Then the browser has saved a complete RFC 5545 calendar
    And the saved calendar matches the reference calendar for "2026-06.pdf"
    And the page has not reported that it saved a file
    And the page has sent no request to another origin since it loaded
    And the page asks for no account, email, or upload
    And the page meets the visual baseline at every displayed state

  @givn.added
  Scenario: The browser offers the same calendar the executable produces for the same plan
    Given the supported duty plan "2026-06.pdf" whose Created footer supplies its revision timestamp
    And the converter page served from the built Site output
    And the executable has published the calendar for that plan
    When the crew member supplies that plan through the file picker
    Then the offered calendar is byte-identical to the executable's calendar

  @givn.added
  Scenario: A plan at the supported size bound is accepted
    Given the converter page served from the built Site output
    When the crew member supplies the supported duty plan "2026-06.pdf" of exactly 20 MiB
    Then the page shows that the calendar is ready
    And the page offers the file "2026-06.ics"

  @givn.added
  Scenario: A plan larger than the supported size bound is refused by name
    Given the converter page served from the built Site output
    When the crew member supplies the plan "2026-06.pdf" of 25 MiB
    Then the page shows a failure heading and a failure naming the 20 MiB bound
    And the page offers no calendar
    And the page offers the diagnostic text "2026-06.parse-error.txt"
    And the diagnostic text identifies "2026-06.pdf" and the size failure
    And the page offers a way to choose another plan

  @givn.added
  Scenario: A source outside the supported family is rejected with a diagnostic offer
    Given the converter page served from the built Site output
    When the crew member supplies the file "notes.pdf" that is not a supported duty plan
    Then the page shows a failure heading and a failure naming "notes.pdf"
    And the page offers no calendar
    And the page offers the diagnostic text "notes.parse-error.txt"
    And the diagnostic text identifies "notes.pdf" and the unsupported-family reason
    And the page offers a way to choose another plan

  @givn.added @wip
  Scenario: An unresolved required activity is rejected with a diagnostic offer
    Given the converter page served from the built Site output
    When the crew member supplies the supported duty plan "2026-06.pdf" containing a required activity the converter cannot resolve
    Then the page shows a failure heading and the reason
    And the page offers no calendar
    And the page offers the diagnostic text "2026-06.parse-error.txt"
    And the diagnostic text identifies "2026-06.pdf" and the reason
    And the page offers a way to choose another plan

  @givn.added @wip
  Scenario: A failed attempt withdraws a calendar offered by an earlier attempt
    Given the converter page served from the built Site output
    When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
    Then the page shows that the calendar is ready
    And the page offers the file "2026-06.ics"
    When the crew member supplies the file "notes.pdf" that is not a supported duty plan
    Then the page shows a failure heading and a failure naming "notes.pdf"
    And the page offers no calendar and no longer offers "2026-06.ics"
    And the page offers the diagnostic text "notes.parse-error.txt"

  @givn.added @wip
  Scenario: A calendar that cannot be offered is not reported as ready
    Given the converter page served from the built Site output that cannot create a download offer for the calendar
    When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
    Then the page shows a failure heading and states that the calendar could not be offered
    And the page offers no calendar
    And the page offers the diagnostic text "2026-06.parse-error.txt"
    And the page offers a way to choose another plan

  @givn.added @wip
  Scenario: A diagnostic that cannot be offered does not hide the primary failure
    Given the converter page served from the built Site output that cannot create a download offer for the diagnostic
    When the crew member supplies the file "notes.pdf" that is not a supported duty plan
    Then the page shows a failure heading and the primary failure naming "notes.pdf"
    And the page states that no diagnostic was offered
    And the page offers no calendar
    And the page offers a way to choose another plan

  @givn.added @wip
  Scenario: A browser without WebAssembly shows the cannot-run message
    Given the converter page served from the built Site output in a browser without WebAssembly
    Then the page shows a cannot-run heading and states that WebAssembly is required
    And the page offers no file picker
    And the page offers no calendar

  @givn.added @wip
  Scenario: A browser without ES-module support shows the cannot-run message
    Given the converter page served from the built Site output in a browser without ES-module support
    Then the page shows a cannot-run heading and states that ES modules are required
    And the page offers no file picker
    And the page offers no calendar

  @givn.added @wip
  Scenario: A converter module that cannot be instantiated shows the startup failure
    Given the converter page served from the built Site output whose converter module cannot be instantiated
    Then the page shows a cannot-run heading and states that the converter could not start
    And the page offers no file picker
    And the page offers no calendar
    And the page has sent no request to another origin since it loaded

  @givn.added @wip
  Scenario: A supported plan dropped onto the drop zone is converted
    Given the converter page served from the built Site output
    When the crew member drops the supported duty plan "2026-06.pdf" onto the drop zone
    Then the page shows that the drop was accepted and the calendar is ready
    And the page offers the file "2026-06.ics"

  @givn.added @wip
  Scenario: A cancelled save is not reported as a save
    Given the converter page served from the built Site output
    When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
    And the crew member dismisses the save of the offered calendar
    Then the page still offers the calendar
    And the page has not reported that it saved a file

  @givn.added @wip
  Scenario: The offered names use the source file's stem
    Given the converter page served from the built Site output
    When the crew member supplies the file "roster.v2.pdf" that is not a supported duty plan
    Then the page offers the diagnostic text "roster.v2.parse-error.txt"
    And the page offers no calendar

  @givn.added @wip
  Scenario: A browser conversion leaves no plan in persistent storage or a navigable address
    Given the converter page served from the built Site output
    When the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
    Then the page shows that the calendar is ready
    And the page offers the file "2026-06.ics"
    And the page's address is unchanged and the browser history has no new entry
    And the page has stored nothing in persistent browser storage
    When the crew member reloads the page
    Then the page shows the empty state and offers no calendar
