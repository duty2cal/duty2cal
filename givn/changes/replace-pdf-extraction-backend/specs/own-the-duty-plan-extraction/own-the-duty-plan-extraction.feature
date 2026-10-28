@givn.delta @own-the-duty-plan-extraction

Feature: Own the duty-plan extraction

  @givn.added @e2e @wip
  Scenario: Release maintainer converts the supported plan through the executable and receives the accepted calendar
    Given a supported duty plan whose accepted calendar is recorded
    When the release maintainer converts it through the executable
    Then the executable publishes the complete calendar at an unused path
    And the published calendar is identical to the accepted calendar after the agreed normalisation
    And the executable reports success through its process exit status

  @givn.added
  Scenario: The same plan encoded with object streams publishes the same calendar content
    Given the supported duty plan encoded with a classic xref table
    And the same plan encoded with an xref stream and object streams
    When the release maintainer converts both plans through the executable
    Then both published calendars are identical apart from their source-snapshot identities
    And the executable reports success for both conversions

  @givn.added
  Scenario Outline: A document outside the frozen subset is refused by name
    Given a duty plan that exercises the construct "<construct>"
    When the release maintainer converts it through the executable
    Then no calendar is published
    And the numbered failure report names the source PDF, the construct "<construct>" and the page it occurs on
    And the executable reports failure through a non-zero process exit status

    Examples:
      | construct |
      | Type3 font |
      | text rotation |
      | page rotation |
      | horizontal text scaling |
      | inline image |
      | unsupported stream filter |
      | font encoding other than Identity-H |
      | missing ToUnicode CMap |
      | unmapped glyph code |
      | page box other than the media box |
      | form XObject |
      | font descriptor without a descent |
      | text operator TJ |
      | unknown content-stream operator |

  @givn.added
  Scenario: An encrypted document is refused by name
    Given a duty plan that is encrypted
    When the release maintainer converts it through the executable
    Then no calendar is published
    And the numbered failure report names the source PDF and the encrypted document
    And the executable reports failure through a non-zero process exit status

  @givn.added
  Scenario: A readable PDF outside the supported family is refused without an empty calendar
    Given a readable PDF that prints no detail-table header
    When the release maintainer converts it through the executable
    Then no calendar is published
    And the numbered failure report names the source PDF and the unsupported family
    And the executable reports failure through a non-zero process exit status

  @givn.added
  Scenario: A truncated document is refused and the process does not abort
    Given a truncated PDF
    When the release maintainer converts it through the executable
    Then no calendar is published
    And the numbered failure report names the source PDF and that the source is not a readable PDF
    And the executable reports failure through a non-zero process exit status

  @givn.added
  Scenario: A refusal names the construct and no printed plan content
    Given a duty plan that uses a Type3 font and prints an identifying name
    When the release maintainer converts it through the executable
    Then the numbered failure report names the source PDF and the Type3 font
    And the failure report contains no printed text from the plan

  @givn.added
  Scenario: A supported plan whose table continues on a second page keeps every detail row
    Given a supported duty plan whose detail table continues on a second page with a repeated header and a TK row on each page
    When the release maintainer converts it through the executable
    Then the published calendar contains exactly two TK events, one dated 12 June 2026 and one dated 13 June 2026
    And no detail row is represented twice

  @givn.added
  Scenario: An unrecognized activity code still publishes the calendar with its warning
    Given a supported duty plan whose one row uses an unrecognized activity code with both printed endpoints
    When the release maintainer converts it through the executable
    Then the complete calendar is published
    And the warning report names that detail row and the fallback applied
