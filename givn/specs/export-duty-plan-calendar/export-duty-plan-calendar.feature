@export-duty-plan-calendar

Feature: Export a duty-plan calendar

  @e2e
  Scenario: Crew member converts a supported duty plan and receives a calendar file
    Given a supported Released Roster duty plan supplied for conversion
    When the crew member converts it
    Then a complete calendar file is published beside the source under an unused name
    And the converter reports the path of the newly published calendar
    And the invocation reports success through its process exit status

  Scenario: A source outside the supported family is rejected
    Given a duty plan that is not a supported Released Roster detail-table export
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming the source
    And the invocation reports failure through a non-zero process exit status
    And a new unused numbered diagnostic report identifies the source and the reason

  Scenario: Overview and statistics material supplies no dated activities
    Given a supported duty plan whose overview calendar and statistics name activity codes that never appear in its dated detail rows
    When the crew member converts it
    Then the published calendar contains no activity taken from the overview or statistics

  Scenario: Footer records supply no dated activities
    Given a supported duty plan whose footer names activity codes that never appear in its dated detail rows
    When the crew member converts it
    Then the published calendar contains no activity taken from the footer

  Scenario: A date-only row is not turned into an activity
    Given a supported duty plan with a detail row that carries only a date and no activity code
    When the crew member converts it
    Then the published calendar contains no absence or layover for that date

  Scenario: A mixed plan represents every required activity
    Given a supported duty plan with one timed travel leg, one all-day U row, and one timed SB row
    When the crew member converts it
    Then the calendar contains exactly three activities: one timed travel activity, one all-day U activity, and one timed SB activity

  Scenario Outline: All-day codes ignore printed clocks
    Given a supported duty plan dated <date> with activity code <code> and printed UTC times <start> and <end>
    When the crew member converts it
    Then the calendar contains one all-day activity for <date> whose source code is <code>
    And that activity carries no timed interval

    Examples:
      | date       | code | start | end   |
      | 2099-01-12 | U    |       |       |
      | 2099-01-13 | OFF  | 23:00 | 01:00 |
      | 2099-01-14 | FZ   | 09:00 | 17:00 |
      | 2099-01-15 | FREE | 00:00 | 00:00 |

  Scenario: Office or seminar duty uses its printed UTC window
    Given a supported duty plan dated 2099-01-16 with activity code TK and resolved UTC start 08:00 and end 12:00
    When the crew member converts it
    Then the calendar contains one timed non-flight activity from 08:00 to 12:00 UTC on 2099-01-16 lasting 4 hours

  Scenario: Consulate visa duty uses its printed UTC window
    Given a supported duty plan dated 2099-01-17 with activity code DTUSA and resolved UTC start 09:00 and end 15:00
    When the crew member converts it
    Then the calendar contains one timed non-flight activity from 09:00 to 15:00 UTC on 2099-01-17 lasting 6 hours

  Scenario: EM duty uses its printed UTC window
    Given a supported duty plan dated 2099-01-18 with activity code EM and source station ABC and resolved UTC start 06:00 and end 10:00
    When the crew member converts it
    Then the calendar contains one timed non-flight activity from 06:00 to 10:00 UTC on 2099-01-18 lasting 4 hours

  Scenario: Standby duty uses its own UTC window
    Given a supported duty plan dated 2099-01-18 with activity code SB and resolved UTC start 08:00 and end 16:00
    When the crew member converts it
    Then the calendar contains one timed standby activity from 08:00 to 16:00 UTC lasting 8 hours

  Scenario: Reserve duty uses its own UTC window
    Given a supported duty plan dated 2099-01-18 with activity code RE and resolved UTC start 09:00 and end 17:00
    When the crew member converts it
    Then the calendar contains one timed reserve activity from 09:00 to 17:00 UTC lasting 8 hours

  Scenario: Consecutive standby days interrupted by a day off stay separate
    Given a supported duty plan with SB window 08:00 to 12:00 on 2099-01-19, a FREE row on 2099-01-20, and SB window 08:00 to 12:00 on 2099-01-21
    When the crew member converts it
    Then the calendar contains two timed standby activities on 2099-01-19 and 2099-01-21
    And the calendar contains one all-day day-off activity on 2099-01-20
    And no standby activity spans 2099-01-20

  Scenario: Single-day standby uses its own window rather than a cancelled flight
    Given a supported duty plan dated 2099-01-22 with activity code SBX and its own resolved UTC start 08:00 and end 16:00
    When the crew member converts it
    Then the calendar contains one timed SBX activity from 08:00 to 16:00 UTC
    And the calendar contains no activity for any cancelled flight

  Scenario: Deadhead positioning travel keeps one event and marks the assignment in its title
    Given a supported duty plan with designator 007 from ABC to DEF, position DH, resolved UTC start 06:00 and end 07:00, no aircraft, and accounting Blocktime 00:00
    When the crew member converts it
    Then the calendar contains one travel activity from 06:00 to 07:00 UTC on the duty-plan date
    And that activity's title is "DH 007 ABC-DEF"
    And the calendar contains no second activity for that leg

  Scenario: A dated station-bearing layover row becomes an all-day activity
    Given a supported duty plan with a dated Layover row for station ABC
    When the crew member converts it
    Then the calendar contains one all-day layover activity for that date

  Scenario: An unknown required activity code rejects the conversion
    Given a supported duty plan containing a required activity code the converter does not recognize
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming the source
    And a new unused numbered diagnostic report identifies the source and the reason

  Scenario: A required travel leg without a compatible partner rejects the conversion
    Given a supported duty plan whose travel fragment has no unique compatible partner
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming the source
    And a new unused numbered diagnostic report identifies the source and the reason

  Scenario: An explicit next-day qualifier moves the endpoint to the following UTC day
    Given a supported duty plan whose local block for 2099-01-12 carries a departure start "(13) 00:20" and whose arrival block for 2099-01-13 ends at 08:10 UTC
    When the crew member converts it
    Then the calendar contains one travel activity on UTC day 2099-01-13 lasting 7 hours 50 minutes

  Scenario: A split across local dates can remain on the same UTC day
    Given a supported duty plan whose local block for 2099-01-12 starts at 19:15 UTC and whose next block ends "(12) 21:05" UTC
    When the crew member converts it
    Then the calendar contains one travel activity on UTC day 2099-01-12 lasting 1 hour 50 minutes

  Scenario: Unqualified endpoints that appear to cross midnight roll to the next UTC day
    Given a supported duty plan with one local block for 2099-01-12 whose travel start is 23:10 and end is 01:05, with no day qualifiers and no separate arrival block
    When the crew member converts it
    Then the calendar contains one travel activity from 23:10 on 2099-01-12 to 01:05 on 2099-01-13 UTC lasting 1 hour 55 minutes

  Scenario: A zero-length travel interval is not repaired
    Given a supported duty plan with one local block whose travel start and end clocks are equal and unqualified
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming the source

  Scenario: A previous-year UTC qualifier resolves across the year boundary
    Given a supported duty plan with local date 2099-01-01 and a UTC cell "(31) 23:10"
    When the crew member converts it
    Then that endpoint is 2098-12-31 23:10 UTC

  Scenario: A next-month UTC qualifier resolves across the month boundary
    Given a supported duty plan with local date 2099-01-31 and a UTC cell "(1) 00:20"
    When the crew member converts it
    Then that endpoint is 2099-02-01 00:20 UTC

  Scenario: An explicit qualifier with no unique adjacent date rejects the conversion
    Given a supported duty plan whose explicit UTC day number matches none of the previous, same, or next dates around its local-date block
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming the source

  Scenario: A known trip identifier is carried across the legs of one roster trip
    Given a supported duty plan whose roster trip T42 has legs 007 and 008 where only leg 007 prints the Trip ID, followed by roster trip T43
    When the crew member converts it
    Then the descriptions of leg 007 and leg 008 both contain "Trip T42"
    And the description of the leg in trip T43 does not contain "Trip T42"

  Scenario Outline: An unsupplied optional description field is omitted
    Given a supported duty plan travel leg whose <field> is not supplied
    When the crew member converts it
    Then the travel activity's description carries no <field>

    Examples:
      | field               |
      | Trip ID             |
      | Report (LT)         |
      | A/C                 |
      | Blocktime           |
      | Timezone Difference |
      | Layover duration    |
      | Tags                |

  Scenario: Complementary optional values from compatible fragments combine
    Given a compatible departure and arrival fragment of one travel leg where the departure fragment supplies A/C ABC and the arrival fragment supplies Blocktime 02:00
    When the crew member converts it
    Then the travel activity's description contains A/C ABC and Blocktime 02:00

  Scenario: Description fields follow the approved order
    Given a supported duty plan travel leg supplying Trip ID T42, Report (LT) 07:30, A/C ABC, Blocktime 02:00, Timezone Difference +01, Layover duration 03:00, and Tags 5_MFL
    When the crew member converts it
    Then the travel activity's description lists them in the order Trip ID, Report (LT), A/C, Blocktime, Timezone Difference, Layover duration, Tags

  Scenario: Differing optional values are both retained with their fragment roles
    Given a compatible departure and arrival fragment of one travel leg supplying Report (LT) 07:30 and Report (LT) 08:00 respectively
    When the crew member converts it
    Then the travel activity's description states "Report (LT) 07:30 (departure fragment)" and "Report (LT) 08:00 (arrival fragment)"
    And neither printed value changes the activity's UTC interval

  Scenario: All-day activities use dates with a non-inclusive following-day end
    Given a supported duty plan dated 2099-01-12 with activity code U
    When the crew member converts it
    Then the all-day activity for 2099-01-12 ends on 2099-01-13

  Scenario: The published calendar is a valid RFC 5545 file
    Given a supported duty plan with timed and all-day activities
    When the crew member converts it
    Then the published calendar uses CRLF line endings and UTF-8 text
    And its text values are escaped as RFC 5545 requires
    And its content lines are folded within 75 octets without splitting a character
    And every timed activity ends after it starts

  Scenario: An identical source snapshot keeps its component identifiers
    Given a supported duty plan converted once and then converted again under a different output name
    When the crew member compares the two calendars
    Then the same activity carries the same component identifier in both calendars

  Scenario: A revised source snapshot has separate component identifiers
    Given a supported duty plan whose later revision changes one flight's UTC time while its other activities are unchanged
    When the crew member converts both revisions
    Then every activity of the revised snapshot carries a component identifier distinct from the earlier snapshot

  Scenario: The revision timestamp comes from the source's Created footer
    Given a supported duty plan whose footer states Created 2099-01-10 12:34 UTC
    When the crew member converts it
    Then the published calendar's revision timestamp is 2099-01-10 12:34 UTC

  Scenario: Without usable source metadata the construction time is the revision timestamp
    Given a supported duty plan with no usable source revision metadata and a controlled construction clock of 2099-01-11 08:00 UTC
    When the crew member converts it
    Then the published calendar's revision timestamp is 2099-01-11 08:00 UTC
    And its activity dates are unaffected

  Scenario: An occupied calendar name yields a new unused numbered name
    Given the source roster.pdf beside it existing calendars roster.ics and roster-1.ics
    When the crew member converts it
    Then a complete calendar is published as roster-2.ics
    And roster.ics and roster-1.ics are unchanged

  Scenario: An interpretation failure preserves an older diagnostic
    Given a supported duty plan with an unresolvable required activity and an existing roster.parse-error.txt beside it
    When the crew member converts it
    Then no calendar is published
    And the converter reports the failure
    And roster.parse-error-1.txt names the source PDF and the reason
    And roster.parse-error.txt is unchanged

  Scenario: A publication failure is exposed and a diagnostic is attempted
    Given a supported duty plan whose activities resolve but whose calendar cannot be published
    When the crew member converts it
    Then the converter reports the failure
    And a new unused numbered diagnostic report is attempted
    And existing calendars are unchanged

  Scenario: An unsavable diagnostic keeps the original failure visible
    Given a conversion failure and no writable destination for a new numbered report
    When the crew member converts it
    Then the original failure remains visible
    And the converter does not present an older report as current

  Scenario: An occupied diagnostic name yields a new unused numbered report
    Given a conversion failure beside existing reports roster.parse-error.txt and roster.parse-error-1.txt
    When the crew member converts it
    Then the new diagnostic is written as roster.parse-error-2.txt
    And roster.parse-error.txt and roster.parse-error-1.txt are unchanged

  Scenario: The reference roster converts to the expected calendar
    Given the generated reference roster PDF and its expected calendar fixture
    When the crew member converts the reference roster
    Then the published calendar equals the expected calendar after CRLF-to-LF and folding normalisation

  Scenario: A detail row printed before any dated row rejects the conversion
    Given a supported duty plan whose first detail row prints no date
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "no earlier row supplies one"

  Scenario: A layover station printed in the next column is still read
    Given a supported duty plan with a layover marker whose station sits in the following column
    When the crew member converts it
    Then the calendar contains a layover activity on 2099-06-08 at ABC

  Scenario: A date cell whose weekday contradicts the day is not accepted as activity timing
    Given a supported duty plan whose dated row prints a weekday that contradicts the day
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "unresolved date"

  Scenario: A dated row in a plan with no roster month heading rejects the conversion
    Given a supported duty plan whose dated rows print no roster month heading
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "unresolved date"

  Scenario: A detail table without a required column rejects the conversion
    Given a supported duty plan whose detail-table header omits the From label
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "carries no From column"

  Scenario: An arrival row with no departure partner rejects the conversion
    Given a supported duty plan whose arrival row has no departure partner
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "no unique compatible partner"

  Scenario: An unknown activity code with no route rejects the conversion
    Given a supported duty plan with an unknown activity code and no route
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "LU900"

  Scenario: A split leg whose departure alone prints a field keeps that value
    Given a supported duty plan whose split leg prints its aircraft on the departure only
    When the crew member converts it
    Then the travel activity's description carries "A/C LU7"

  Scenario: A tag carrying a backslash is escaped in the calendar text
    Given a supported duty plan whose travel leg carries the tag "3_OFF\"
    When the crew member converts it
    Then the calendar text carries "3_OFF\\"

  Scenario: A date cell with a single token is not accepted as activity timing
    Given a supported duty plan whose dated row prints only a day number
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "unresolved date"

  Scenario: A date cell whose day cannot be read is not accepted as activity timing
    Given a supported duty plan whose dated row prints an unreadable day
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "unresolved date"

  Scenario: A date cell whose weekday is unknown is not accepted as activity timing
    Given a supported duty plan whose dated row prints an unknown weekday
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "unresolved date"

  Scenario: A date cell naming a day the month does not have is not accepted as activity timing
    Given a supported duty plan whose dated row names a day the month does not have
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "unresolved date"

  Scenario: A roster heading whose year touches its month still supplies the month
    Given a supported duty plan whose roster heading prints the year touching the month
    When the crew member converts it
    Then the calendar contains a travel activity on 2099-06-07

  Scenario: A revision footer naming no UTC zone supplies no revision
    Given a supported duty plan whose revision footer names no UTC zone
    When the crew member converts it
    Then the calendar's revision is the construction clock

  Scenario: A supplied document that is not a roster export is rejected
    Given a supplied document that carries no detail-table labels
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming the source

  Scenario: An invocation without a plan prints its usage and reports failure
    When the invocation supplies no duty plan
    Then the invocation reports failure through a non-zero process exit status
    And the transcript states "usage: duty2cal <duty-plan.pdf>"

  Scenario: A supplied plan that cannot be read fails naming the source
    Given a supplied duty plan path that does not exist
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "cannot read the source"

  Scenario: Without the construction clock the revision is the current time
    Given a supported duty plan with no construction clock supplied
    When the crew member converts it
    Then the calendar's revision is a current UTC stamp

  Scenario: A row with a station but neither a code nor a route rejects the conversion
    Given a supported duty plan whose row prints only a station
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "neither an activity code nor a route"

  Scenario: A split leg whose arrival alone prints a field keeps that value
    Given a supported duty plan whose split leg prints its aircraft on the arrival only
    When the crew member converts it
    Then the travel activity's description carries "A/C LU7"

  Scenario: A source name that yields no usable path fails without writing anything
    Given a supplied duty plan whose name is not valid text
    When the crew member converts it
    Then no calendar is published
    And the conversion failure names "no unused calendar name is available"

  Scenario: A long description that folds across a character boundary still publishes
    Given a supported duty plan whose travel leg carries a long tag with accented letters
    When the crew member converts it
    Then the calendar text carries "DTSTART:20990607T090000Z"

  Scenario: A revision footer with an unusable stamp supplies no revision
    Given a supported duty plan whose revision footer prints an unusable stamp
    When the crew member converts it
    Then the calendar's revision is the construction clock
  Scenario: A crew position and its flight number are printed as separate words
    Given  a supported duty plan whose travel leg prints crew position FB and flight number 123 from ABC to DEF with resolved UTC start 08:00 and end 09:00
    When  the crew member converts it
    Then  that activity's title is "FB 123 ABC-DEF"

  Scenario: A travel leg that prints no crew position keeps its flight number alone
    Given  a supported duty plan whose travel leg prints flight number 123 from ABC to DEF with resolved UTC start 08:00 and end 09:00 and no crew position
    When  the crew member converts it
    Then  that activity's title is "123 ABC-DEF"

  Scenario: A crew position printed beside a non-flight activity stays out of its title
    Given  a supported duty plan whose dated row prints crew position AK beside activity code EM with resolved UTC start 06:00 and end 10:00
    When  the crew member converts it
    Then  that activity's title is "EM"

  Scenario: An unmatched travel fragment is named with its position and flight number separate
    Given  a supported duty plan whose unmatched travel fragment prints crew position FB and flight number 123
    When  the crew member converts it
    Then  no calendar is published
    And  the conversion failure names "FB 123"

  Scenario: A positioning departure fragment has no partner when its arrival fragment prints no crew position
    Given  a supported duty plan whose departure fragment prints crew position DH and flight number 124 and whose arrival fragment prints flight number 124 without a crew position
    When  the crew member converts it
    Then  no calendar is published
    And  the conversion failure names "no unique compatible partner"
  Scenario: A travel activity's title ends with its report time in local time
    Given  a supported duty plan travel leg supplying Trip ID T42, Report (LT) 07:30, A/C ABC, Blocktime 02:00, Timezone Difference +01, Layover duration 03:00, and Tags 5_MFL
    When  the crew member converts it
    Then  that activity's title is "LU742 ABC-DEF Report 07:30 LT"

  Scenario: A travel activity without a report time keeps its title unchanged
    Given  a supported duty plan travel leg without a report time
    When  the crew member converts it
    Then  that activity's title is "LU742 ABC-DEF"

  Scenario: Differing report times leave the title without a report suffix
    Given  a compatible departure and arrival fragment of one travel leg supplying different report times 07:30 and 08:00
    When  the crew member converts it
    Then  that activity's title is "LU742 ABC-DEF"
    And  the travel activity's description states "Report (LT) 07:30 (departure fragment)" and "Report (LT) 08:00 (arrival fragment)"

  Scenario: Equal report times on both fragments produce one title suffix
    Given  a compatible departure and arrival fragment of one travel leg supplying the same report time 07:30
    When  the crew member converts it
    Then  that activity's title is "LU742 ABC-DEF Report 07:30 LT"
  @e2e
  Scenario: Crew member converts a supported duty plan in the browser and saves the offered calendar
    Given  the converter page served from the built Site output
    Then  the page shows the empty state and offers no calendar
    When  the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
    Then  the page shows that the calendar is ready
    And  the page offers the file "2026-06.ics"
    When  the crew member saves the offered calendar
    Then  the browser has saved a complete RFC 5545 calendar
    And  the saved calendar matches the reference calendar for "2026-06.pdf"
    And  the page has not reported that it saved a file
    And  the page has sent no request to another origin since it loaded
    And  the page asks for no account, email, or upload
    And  the page meets the visual baseline at every displayed state

  Scenario: The browser offers the same calendar the executable produces for the same plan
    Given  the supported duty plan "2026-06.pdf" whose Created footer supplies its revision timestamp
    And  the converter page served from the built Site output
    And  the executable has published the calendar for that plan
    When  the crew member supplies that plan through the file picker
    Then  the offered calendar is byte-identical to the executable's calendar

  Scenario: A plan at the supported size bound is accepted
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" of exactly 20 MiB
    Then  the page shows that the calendar is ready
    And  the page offers the file "2026-06.ics"

  Scenario: A plan larger than the supported size bound is refused by name
    Given  the converter page served from the built Site output
    When  the crew member supplies the plan "2026-06.pdf" of 25 MiB
    Then  the page shows a failure heading and a failure naming the 20 MiB bound
    And  the page offers no calendar
    And  the page offers the diagnostic text "2026-06.parse-error.txt"
    And  the diagnostic text identifies "2026-06.pdf" and the size failure
    And  the page offers a way to choose another plan

  Scenario: A source outside the supported family is rejected with a diagnostic offer
    Given  the converter page served from the built Site output
    When  the crew member supplies the file "notes.pdf" that is not a supported duty plan
    Then  the page shows a failure heading and a failure naming "notes.pdf"
    And  the page offers no calendar
    And  the page offers the diagnostic text "notes.parse-error.txt"
    And  the diagnostic text identifies "notes.pdf" and the unsupported-family reason
    And  the page offers a way to choose another plan

  Scenario: An unresolved required activity is rejected with a diagnostic offer
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" containing a required activity the converter cannot resolve
    Then  the page shows a failure heading and the reason
    And  the page offers no calendar
    And  the page offers the diagnostic text "2026-06.parse-error.txt"
    And  the diagnostic text identifies "2026-06.pdf" and the reason
    And  the page offers a way to choose another plan

  Scenario: A failed attempt withdraws a calendar offered by an earlier attempt
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
    Then  the page shows that the calendar is ready
    And  the page offers the file "2026-06.ics"
    When  the crew member supplies the file "notes.pdf" that is not a supported duty plan
    Then  the page shows a failure heading and a failure naming "notes.pdf"
    And  the page offers no calendar and no longer offers "2026-06.ics"
    And  the page offers the diagnostic text "notes.parse-error.txt"

  Scenario: A calendar that cannot be offered is not reported as ready
    Given  the converter page served from the built Site output that cannot create a download offer for the calendar
    When  the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
    Then  the page shows a failure heading and states that the calendar could not be offered
    And  the page offers no calendar
    And  the page offers the diagnostic text "2026-06.parse-error.txt"
    And  the page offers a way to choose another plan

  Scenario: A diagnostic that cannot be offered does not hide the primary failure
    Given  the converter page served from the built Site output that cannot create a download offer for the diagnostic
    When  the crew member supplies the file "notes.pdf" that is not a supported duty plan
    Then  the page shows a failure heading and the primary failure naming "notes.pdf"
    And  the page states that no diagnostic was offered
    And  the page offers no calendar
    And  the page offers a way to choose another plan

  Scenario: A browser without WebAssembly shows the cannot-run message
    Given  the converter page served from the built Site output in a browser without WebAssembly
    Then  the page shows a cannot-run heading and states that WebAssembly is required
    And  the page offers no file picker
    And  the page offers no calendar

  Scenario: A browser without ES-module support shows the cannot-run message
    Given  the converter page served from the built Site output in a browser without ES-module support
    Then  the page shows a cannot-run heading and states that ES modules are required
    And  the page offers no file picker
    And  the page offers no calendar

  Scenario: A converter module that cannot be instantiated shows the startup failure
    Given  the converter page served from the built Site output whose converter module cannot be instantiated
    Then  the page shows a cannot-run heading and states that the converter could not start
    And  the page offers no file picker
    And  the page offers no calendar
    And  the page has sent no request to another origin since it loaded

  Scenario: A supported plan dropped onto the drop zone is converted
    Given  the converter page served from the built Site output
    When  the crew member drops the supported duty plan "2026-06.pdf" onto the drop zone
    Then  the page shows that the drop was accepted and the calendar is ready
    And  the page offers the file "2026-06.ics"

  Scenario: A cancelled save is not reported as a save
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
    And  the crew member dismisses the save of the offered calendar
    Then  the page still offers the calendar
    And  the page has not reported that it saved a file

  Scenario: The offered names use the source file's stem
    Given  the converter page served from the built Site output
    When  the crew member supplies the file "roster.v2.pdf" that is not a supported duty plan
    Then  the page offers the diagnostic text "roster.v2.parse-error.txt"
    And  the page offers no calendar

  Scenario: A browser conversion leaves no plan in persistent storage or a navigable address
    Given  the converter page served from the built Site output
    When  the crew member supplies the supported duty plan "2026-06.pdf" through the file picker
    Then  the page shows that the calendar is ready
    And  the page offers the file "2026-06.ics"
    And  the page's address is unchanged and the browser history has no new entry
    And  the page has stored nothing in persistent browser storage
    When  the crew member reloads the page
    Then  the page shows the empty state and offers no calendar
