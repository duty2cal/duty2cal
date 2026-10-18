@givn.delta @publish-organization-profile

Feature: Publish the organization profile

  @givn.added @e2e @wip
  Scenario: Organization owner applies the previewed revision and the organization profile is published
    Given the applied state binds every declared object except the organization profile repository and its two files
    And a previewed revision that declares the organization profile repository, its profile content and its default security policy
    When the organization owner applies the previewed revision
    Then the apply reports the organization profile repository and both files as created
    And the organization profile repository is public and offers no issue tracker, projects or wiki
    And the organization profile repository serves the reviewed profile content as "profile/README.md"
    And the organization profile repository serves the default security policy as "SECURITY.md"
    And the applied state records the organization profile repository and both files
    And a later targeted preview of the organization profile reports no differences

  @givn.added @e2e @wip
  Scenario: Organization owner previews the profile declaration and only the three creations are named
    Given the applied state binds every declared object
    And the live organization has no organization profile repository
    And the declaration adds the organization profile repository, its profile content and its default security policy
    When the organization owner previews the declaration
    Then the preview names the organization profile repository as the only object to create
    And the preview names the profile content and the default security policy as the only files to create
    And the preview names no change other than the three creations
    And the live configuration is unchanged

  @givn.added
  Scenario: A revised profile content updates only the profile file
    Given the applied state binds the organization profile repository and both files
    And a previewed revision whose profile content differs from the published profile content
    When the organization owner applies the previewed revision
    Then the apply reports the profile content as the only changed object
    And the organization profile repository serves the revised profile content
    And the default security policy served by the organization profile repository is unchanged
    And the organization profile repository itself is unchanged

  @givn.added
  Scenario: The published profile content states only supported claims and requests no third-party resource
    Given the organization profile repository is published
    When a reader reads the published profile content
    Then the profile content describes the converter as a local tool that reads a crew duty plan and writes a calendar
    And the profile content names Windows x86_64 as the only distributed platform
    And the profile content makes no claim that the converter runs in a browser
    And the profile content links the stable latest-release address "https://github.com/duty2cal/duty2cal/releases/latest"
    And the profile content links the source repository "https://github.com/duty2cal/duty2cal" and the site repository "https://github.com/duty2cal/duty2cal.github.io"
    And the profile content states that the project is not affiliated with any duty-plan provider or airline
    And the profile content attributes the third-party Unicode data to Unicode-3.0
    And every link in the profile content is absolute
    And the profile content embeds no image, badge or other third-party resource

  @givn.added
  Scenario: The profile repository carries the organization's default security policy
    Given the organization profile repository is published
    And the pages repository has no security policy of its own
    When a contributor reads the organization's default security policy
    Then the organization profile repository carries the default security policy as "SECURITY.md"
    And the default security policy names the private vulnerability reporting route
    And the default security policy forbids attaching a real duty plan or a converted calendar
    And the default security policy states that only the latest published release is supported
    And the organization profile repository offers no public issue tracker for security reports
    And the pages repository serves no security policy of its own

  @givn.added
  Scenario: A profile repository that already exists live and is unbound refuses the preview
    Given the applied state binds every declared object except the organization profile repository and its two files
    And the live organization already has the organization profile repository
    And the declaration adds the organization profile repository, its profile content and its default security policy
    When the organization owner previews the declaration
    Then the preview is refused and names the organization profile repository
    And no object in the preview is bound
    And the live configuration is unchanged
