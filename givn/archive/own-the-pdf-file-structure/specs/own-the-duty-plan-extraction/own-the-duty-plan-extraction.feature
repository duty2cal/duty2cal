@givn.delta @own-the-duty-plan-extraction

Feature: Own the duty-plan extraction

  The supported plan's file structure is read by the project itself. These
  scenarios are the characterisation harness for that replacement: every
  observable outcome here holds for the previously accepted reading and must
  still hold once the reading is the project's own.

  @givn.added
  Scenario Outline: A supported plan whose file structure uses an accepted spelling publishes the accepted calendar content
    Given a supported duty plan whose file structure uses "<spelling>"
    When the release maintainer converts it through the executable
    Then the complete calendar is published
    And the published calendar matches the accepted calendar apart from its source-snapshot identity
    And the executable reports success for the conversion
    Examples:
      | spelling |
      | an xref stream with the PNG predictor |
      | an incremental update |
      | an indirect stream length |
      | resources inherited from the page tree |

  @givn.added
  Scenario: A plan whose cross-reference pointer lies outside the file is refused as unreadable
    Given a supported duty plan whose cross-reference pointer lies outside the file
    When the release maintainer converts it through the executable
    Then no calendar is published
    And the numbered failure report names the source PDF and that the source is not a readable PDF
    And the executable reports failure through a non-zero process exit status
