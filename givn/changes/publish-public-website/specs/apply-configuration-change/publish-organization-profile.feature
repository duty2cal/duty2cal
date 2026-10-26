@givn.delta @publish-organization-profile
Feature: Publish the organization profile

  The owning use case is `specs/apply-configuration-change/usecase.md`, whose
  copy in this change is unchanged: the goal, the flow and the guarantees do
  not change, and the capability's interactions are the two it already had.
  This delta replaces one scenario so the published copy stops denying the
  browser route the Public website now serves.

  @givn.modified
  Scenario: The published profile content states only supported claims and requests no third-party resource
    Given the organization profile repository is published
    When a reader reads the published profile content
    Then the profile content describes the converter as a local tool that reads a crew duty plan and writes a calendar
    And the profile content names Windows x86_64 as the only published archive platform
    And the profile content states that builds for other operating systems are available on request
    And the profile content states that a current browser converts a plan on the public website at "https://duty2cal.github.io/"
    And the profile content links the public website at "https://duty2cal.github.io/"
    And the profile content links the stable latest-release address "https://github.com/duty2cal/duty2cal/releases/latest"
    And the profile content links the source repository "https://github.com/duty2cal/duty2cal" and the site repository "https://github.com/duty2cal/duty2cal.github.io"
    And the profile content states that the project is not affiliated with any duty-plan provider or airline
    And the profile content attributes the third-party Unicode data to Unicode-3.0
    And every link in the profile content is absolute
    And the profile content embeds no image, badge or other third-party resource
