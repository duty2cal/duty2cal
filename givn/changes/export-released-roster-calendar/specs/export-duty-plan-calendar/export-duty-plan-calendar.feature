@givn.delta @export-duty-plan-calendar

Feature: Export a duty-plan calendar

  @givn.added @e2e @wip
  Scenario: Crew member converts a supported duty plan and receives a calendar file
    Given a supported Released Roster duty plan supplied for conversion
    When the crew member converts it
    Then a complete calendar file is published beside the source under an unused name
    And the converter reports the path of the newly published calendar
    And the invocation reports success through its process exit status

  @givn.added
  Scenario: A source outside the supported family is rejected
    Given a duty plan that is not a supported Released Roster detail-table export
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming the source
    And the invocation reports failure through a non-zero process exit status
    And a new unused numbered diagnostic report identifies the source and the reason

  @givn.added
  Scenario: Overview and statistics material supplies no dated activities
    Given a supported duty plan whose overview calendar and statistics name activity codes that never appear in its dated detail rows
    When the crew member converts it
    Then the published calendar contains no activity taken from the overview or statistics

  @givn.added
  Scenario: Footer records supply no dated activities
    Given a supported duty plan whose footer names activity codes that never appear in its dated detail rows
    When the crew member converts it
    Then the published calendar contains no activity taken from the footer

  @givn.added @wip
  Scenario: A date-only row is not turned into an activity
    Given a supported duty plan with a detail row that carries only a date and no activity code
    When the crew member converts it
    Then the published calendar contains no absence or layover for that date

  @givn.added @wip
  Scenario: A mixed plan represents every required activity
    Given a supported duty plan with one timed travel leg, one all-day U row, and one timed SB row
    When the crew member converts it
    Then the calendar contains exactly three activities: one timed travel activity, one all-day U activity, and one timed SB activity

  @givn.added @wip
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

  @givn.added @wip
  Scenario: Office or seminar duty uses its printed UTC window
    Given a supported duty plan dated 2099-01-16 with activity code TK and resolved UTC start 08:00 and end 12:00
    When the crew member converts it
    Then the calendar contains one timed non-flight activity from 08:00 to 12:00 UTC on 2099-01-16 lasting 4 hours

  @givn.added @wip
  Scenario: Consulate visa duty uses its printed UTC window
    Given a supported duty plan dated 2099-01-17 with activity code DTUSA and resolved UTC start 09:00 and end 15:00
    When the crew member converts it
    Then the calendar contains one timed non-flight activity from 09:00 to 15:00 UTC on 2099-01-17 lasting 6 hours

  @givn.added @wip
  Scenario: Extended non-flight duty uses its printed UTC window
    Given a supported duty plan dated 2099-01-18 with activity code EM and source station ABC and resolved UTC start 06:00 and end 10:00
    When the crew member converts it
    Then the calendar contains one timed non-flight activity from 06:00 to 10:00 UTC on 2099-01-18 lasting 4 hours

  @givn.added @wip
  Scenario: Standby duty uses its own UTC window
    Given a supported duty plan dated 2099-01-18 with activity code SB and resolved UTC start 08:00 and end 16:00
    When the crew member converts it
    Then the calendar contains one timed standby activity from 08:00 to 16:00 UTC lasting 8 hours

  @givn.added @wip
  Scenario: Reserve duty uses its own UTC window
    Given a supported duty plan dated 2099-01-18 with activity code RE and resolved UTC start 09:00 and end 17:00
    When the crew member converts it
    Then the calendar contains one timed reserve activity from 09:00 to 17:00 UTC lasting 8 hours

  @givn.added @wip
  Scenario: Consecutive standby days interrupted by a day off stay separate
    Given a supported duty plan with SB window 08:00 to 12:00 on 2099-01-19, a FREE row on 2099-01-20, and SB window 08:00 to 12:00 on 2099-01-21
    When the crew member converts it
    Then the calendar contains two timed standby activities on 2099-01-19 and 2099-01-21
    And the calendar contains one all-day day-off activity on 2099-01-20
    And no standby activity spans 2099-01-20

  @givn.added @wip
  Scenario: Single-day standby uses its own window rather than a cancelled flight
    Given a supported duty plan dated 2099-01-22 with activity code SBX and its own resolved UTC start 08:00 and end 16:00
    When the crew member converts it
    Then the calendar contains one timed SBX activity from 08:00 to 16:00 UTC
    And the calendar contains no activity for any cancelled flight

  @givn.added @wip
  Scenario: Deadhead positioning travel keeps one event and marks the assignment in its title
    Given a supported duty plan with designator 007 from ABC to DEF, position DH, resolved UTC start 06:00 and end 07:00, no aircraft, and accounting Blocktime 00:00
    When the crew member converts it
    Then the calendar contains one travel activity from 06:00 to 07:00 UTC on the duty-plan date
    And that activity's title is "DH 007 ABC-DEF"
    And the calendar contains no second activity for that leg

  @givn.added @wip
  Scenario: A dated station-bearing layover row becomes an all-day activity
    Given a supported duty plan with a dated Layover row for station ABC
    When the crew member converts it
    Then the calendar contains one all-day layover activity for that date

  @givn.added @wip
  Scenario: An unknown required activity code rejects the conversion
    Given a supported duty plan containing a required activity code the converter does not recognize
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming the source
    And a new unused numbered diagnostic report identifies the source and the reason

  @givn.added @wip
  Scenario: A required travel leg without a compatible partner rejects the conversion
    Given a supported duty plan whose travel fragment has no unique compatible partner
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming the source
    And a new unused numbered diagnostic report identifies the source and the reason

  @givn.added @wip
  Scenario: An explicit next-day qualifier moves the endpoint to the following UTC day
    Given a supported duty plan whose local block for 2099-01-12 carries a departure start "(13) 00:20" and whose arrival block for 2099-01-13 ends at 08:10 UTC
    When the crew member converts it
    Then the calendar contains one travel activity on UTC day 2099-01-13 lasting 7 hours 50 minutes

  @givn.added @wip
  Scenario: A split across local dates can remain on the same UTC day
    Given a supported duty plan whose local block for 2099-01-12 starts at 19:15 UTC and whose next block ends "(12) 21:05" UTC
    When the crew member converts it
    Then the calendar contains one travel activity on UTC day 2099-01-12 lasting 1 hour 50 minutes

  @givn.added @wip
  Scenario: Unqualified endpoints that appear to cross midnight roll to the next UTC day
    Given a supported duty plan with one local block for 2099-01-12 whose travel start is 23:10 and end is 01:05, with no day qualifiers and no separate arrival block
    When the crew member converts it
    Then the calendar contains one travel activity from 23:10 on 2099-01-12 to 01:05 on 2099-01-13 UTC lasting 1 hour 55 minutes

  @givn.added @wip
  Scenario: A zero-length travel interval is not repaired
    Given a supported duty plan with one local block whose travel start and end clocks are equal and unqualified
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming the source

  @givn.added @wip
  Scenario: A previous-year UTC qualifier resolves across the year boundary
    Given a supported duty plan with local date 2099-01-01 and a UTC cell "(31) 23:10"
    When the crew member converts it
    Then that endpoint is 2098-12-31 23:10 UTC

  @givn.added @wip
  Scenario: A next-month UTC qualifier resolves across the month boundary
    Given a supported duty plan with local date 2099-01-31 and a UTC cell "(1) 00:20"
    When the crew member converts it
    Then that endpoint is 2099-02-01 00:20 UTC

  @givn.added @wip
  Scenario: An explicit qualifier with no unique adjacent date rejects the conversion
    Given a supported duty plan whose explicit UTC day number matches none of the previous, same, or next dates around its local-date block
    When the crew member converts it
    Then no calendar is published
    And the converter reports a conversion failure naming the source

  @givn.added @wip
  Scenario: A known trip identifier is carried across the legs of one roster trip
    Given a supported duty plan whose roster trip T42 has legs 007 and 008 where only leg 007 prints the Trip ID, followed by roster trip T43
    When the crew member converts it
    Then the descriptions of leg 007 and leg 008 both contain "Trip T42"
    And the description of the leg in trip T43 does not contain "Trip T42"

  @givn.added @wip
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

  @givn.added @wip
  Scenario: Complementary optional values from compatible fragments combine
    Given a compatible departure and arrival fragment of one travel leg where the departure fragment supplies A/C ABC and the arrival fragment supplies Blocktime 02:00
    When the crew member converts it
    Then the travel activity's description contains A/C ABC and Blocktime 02:00

  @givn.added @wip
  Scenario: Description fields follow the approved order
    Given a supported duty plan travel leg supplying Trip ID T42, Report (LT) 07:30, A/C ABC, Blocktime 02:00, Timezone Difference +01, Layover duration 03:00, and Tags 5_MFL
    When the crew member converts it
    Then the travel activity's description lists them in the order Trip ID, Report (LT), A/C, Blocktime, Timezone Difference, Layover duration, Tags

  @givn.added @wip
  Scenario: Differing optional values are both retained with their fragment roles
    Given a compatible departure and arrival fragment of one travel leg supplying Report (LT) 07:30 and Report (LT) 08:00 respectively
    When the crew member converts it
    Then the travel activity's description states "Report (LT) 07:30 (departure fragment)" and "Report (LT) 08:00 (arrival fragment)"
    And neither printed value changes the activity's UTC interval

  @givn.added @wip
  Scenario: All-day activities use dates with a non-inclusive following-day end
    Given a supported duty plan dated 2099-01-12 with activity code U
    When the crew member converts it
    Then the all-day activity for 2099-01-12 ends on 2099-01-13

  @givn.added @wip
  Scenario: The published calendar is a valid RFC 5545 file
    Given a supported duty plan with timed and all-day activities
    When the crew member converts it
    Then the published calendar uses CRLF line endings and UTF-8 text
    And its text values are escaped as RFC 5545 requires
    And its content lines are folded within 75 octets without splitting a character
    And every timed activity ends after it starts

  @givn.added @wip
  Scenario: An identical source snapshot keeps its component identifiers
    Given a supported duty plan converted once and then converted again under a different output name
    When the crew member compares the two calendars
    Then the same activity carries the same component identifier in both calendars

  @givn.added @wip
  Scenario: A revised source snapshot has separate component identifiers
    Given a supported duty plan whose later revision changes one flight's UTC time while its other activities are unchanged
    When the crew member converts both revisions
    Then every activity of the revised snapshot carries a component identifier distinct from the earlier snapshot

  @givn.added @wip
  Scenario: The revision timestamp comes from the source's Created footer
    Given a supported duty plan whose footer states Created 2099-01-10 12:34 UTC
    When the crew member converts it
    Then the published calendar's revision timestamp is 2099-01-10 12:34 UTC

  @givn.added @wip
  Scenario: Without usable source metadata the construction time is the revision timestamp
    Given a supported duty plan with no usable source revision metadata and a controlled construction clock of 2099-01-11 08:00 UTC
    When the crew member converts it
    Then the published calendar's revision timestamp is 2099-01-11 08:00 UTC
    And its activity dates are unaffected

  @givn.added @wip
  Scenario: An occupied calendar name yields a new unused numbered name
    Given the source roster.pdf beside it existing calendars roster.ics and roster-1.ics
    When the crew member converts it
    Then a complete calendar is published as roster-2.ics
    And roster.ics and roster-1.ics are unchanged

  @givn.added @wip
  Scenario: An interpretation failure preserves an older diagnostic
    Given a supported duty plan with an unresolvable required activity and an existing roster.parse-error.txt beside it
    When the crew member converts it
    Then no calendar is published
    And the converter reports the failure
    And roster.parse-error-1.txt names the source PDF and the reason
    And roster.parse-error.txt is unchanged

  @givn.added @wip
  Scenario: A publication failure is exposed and a diagnostic is attempted
    Given a supported duty plan whose activities resolve but whose calendar cannot be published
    When the crew member converts it
    Then the converter reports the failure
    And a new unused numbered diagnostic report is attempted
    And existing calendars are unchanged

  @givn.added @wip
  Scenario: An unsavable diagnostic keeps the original failure visible
    Given a conversion failure and no writable destination for a new numbered report
    When the crew member converts it
    Then the original failure remains visible
    And the converter does not present an older report as current

  @givn.added @wip
  Scenario: An occupied diagnostic name yields a new unused numbered report
    Given a conversion failure beside existing reports roster.parse-error.txt and roster.parse-error-1.txt
    When the crew member converts it
    Then the new diagnostic is written as roster.parse-error-2.txt
    And roster.parse-error.txt and roster.parse-error-1.txt are unchanged

  @givn.added @wip
  Scenario: The reference roster converts to the expected calendar
    Given the generated reference roster PDF and its expected calendar fixture
    When the crew member converts the reference roster
    Then the published calendar equals the expected calendar after CRLF-to-LF and folding normalisation
