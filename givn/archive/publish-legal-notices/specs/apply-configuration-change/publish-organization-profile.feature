@givn.delta @publish-organization-profile @documentation

Feature: Publish the organization profile

  @givn.modified
  Scenario: The published profile content states only supported claims and requests no third-party resource
    Given  the organization profile repository is published
    When  a reader reads the published profile content
    Then  the profile content describes the converter as a local tool that reads a crew duty plan and writes a calendar
    And  the profile content states that duty2cal converts a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export into one calendar file
    And  the profile content states the boundary: one roster month of cabin crew; not cockpit duties, other Lufthansa duty plans, other Jeppesen exports or another operator
    And  the profile content states that the project is not affiliated with Lufthansa, Jeppesen, or any duty-plan provider or airline
    And  the profile content names Windows x86_64 as the only published archive platform
    And  the profile content states that builds for other operating systems are available on request
    And  the profile content states that a current browser converts a plan on the public website at "https://duty2cal.github.io/"
    And  the profile content links the public website at "https://duty2cal.github.io/"
    And  the profile content links the Impressum at "https://duty2cal.github.io/impressum" under the label "Impressum"
    And  the profile content links the Datenschutzerklärung at "https://duty2cal.github.io/privacy" under the label "Datenschutz"
    And  the profile content copies no postal address and no electronic mail address
    And  the profile content links the stable latest-release address "https://github.com/duty2cal/duty2cal/releases/latest"
    And  the profile content links the source repository "https://github.com/duty2cal/duty2cal" and the site repository "https://github.com/duty2cal/duty2cal.github.io"
    And  the profile content attributes the third-party Unicode data to Unicode-3.0
    And  every link in the profile content is absolute
    And  the profile content embeds no image, badge or other third-party resource
