@givn.delta @measure-distributed-executable-size

Feature: Measure the distributed executable's size

  @givn.added @e2e @wip
  Scenario: Maintainer requests an attribution for a revision and reads the report
    Given a repository whose revision builds for the supported platform
    And that revision is tagged "v0.3.0"
    When the maintainer requests the size attribution for "v0.3.0"
    Then the report names the requested revision "v0.3.0" and a resolved commit
    And the attribution names the platform "x86_64-pc-windows-gnu"
    And the build identity records the rustc version, "cargo-bloat 0.12.1", and a digest of "Cargo.toml" and ".cargo/config.toml"
    And the attribution divides the size by crate, naming "fixture_support"
    And the attribution divides the size by function, naming "fixture_support::fixture_alpha"
    And the stripped executable's file size equals the fixture executable's byte count
    And the attribution's total is less than the executable's file size
    And the attribution build's file size is greater than the executable's file size

  @givn.added
  Scenario: An attribution requested for an earlier revision describes that revision
    Given a repository whose earlier revision "v0.3.0" carries the function "fixture_alpha" and whose newest revision carries "fixture_beta" instead
    When the maintainer requests the size attribution for "v0.3.0"
    Then the report names the requested revision "v0.3.0"
    And the attribution names the resolved commit as the revision it describes
    And the attribution names the function "fixture_alpha"
    And the attribution names no function "fixture_beta"

  @givn.added
  Scenario: A build that fails produces no attribution
    Given a revision that does not build
    When the maintainer requests the size attribution for that revision
    Then the report names the build as unreadable
    And the report's reason names the build's "fixture_compile_error" failure
    And the report states no attribution

  @givn.added
  Scenario: A build that keeps no names produces no attribution
    Given a revision whose build keeps no symbol names
    When the maintainer requests the size attribution for that revision
    Then the report names the missing capability
    And the report states no attribution

  @givn.added
  Scenario Outline: An artifact that is not the supported platform's executable is refused as evidence
    Given <artifact>
    When the maintainer requests the size attribution for the revision, describing that artifact
    Then the measurement is refused, naming "x86_64-pc-windows-gnu"
    And the report exists and states that no attribution was produced

    Examples:
      | artifact |
      | an executable built for the host machine |
      | a release archive presented as the executable |

  @givn.added
  Scenario: An executable whose file size cannot be read leaves the attribution incomplete
    Given the supported platform's executable at the named path cannot be read
    When the maintainer requests the size attribution for the revision, describing that executable
    Then the attribution states that the executable's file size is unavailable
    And the attribution's total is not presented as the executable's file size

  @givn.added
  Scenario: A revision name that resolves to nothing is refused by name
    Given a repository whose history does not contain the revision "v9.9.9"
    When the maintainer requests the size attribution for "v9.9.9"
    Then the measurement is refused, naming "v9.9.9"
    And no report is written

  @givn.added @wip
  Scenario: The attribution lists the stripped executable's sections
    Given a repository whose revision builds for the supported platform
    When the maintainer requests the size attribution for that revision
    Then the report lists the stripped executable's sections
    And the section table names the ".text" section

  @givn.added @wip
  Scenario: An absent objdump leaves the section table unread
    Given a repository whose revision builds for the supported platform
    And "objdump" is not available on the host
    When the maintainer requests the size attribution for that revision
    Then the report states that the section sizes could not be read
    And the attribution still states the stripped executable's file size
