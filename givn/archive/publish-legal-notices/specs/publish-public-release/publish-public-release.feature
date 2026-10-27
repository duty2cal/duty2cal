@givn.delta @publish-public-release @documentation

Feature: Publish the public release

  @givn.added @documentation
  Scenario: The README carries a Legal section that links both notices
    Given  the repository README is published
    When   a reader reads the README's Legal section
    Then   the section links the Impressum at "https://duty2cal.github.io/impressum" under the label "Impressum"
    And    the section links the Datenschutzerklärung at "https://duty2cal.github.io/privacy" under the label "Datenschutz"
    And    the README copies no postal address and no electronic mail address
    And    a README that loses its Legal section is refused by "scripts/public-text-check.sh"
