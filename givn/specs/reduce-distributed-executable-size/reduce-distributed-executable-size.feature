@nested-build
Feature: Reduce the distributed executable's size

  @e2e
  Scenario: Maintainer measures the panic-abort lever and the landed executable is smaller
    Given  a repository at the revision carrying the landed panic-abort lever
    And  a published binary size of 2097152 B
    When  the maintainer measures the "panic=abort" lever
    Then  the lever is applied and the landed executable is smaller
    And  the published binary's size is reported separately
    And  the saved bytes are measured against the baseline, not the published binary's size
    And  the lever executable still writes a numbered diagnostic for an unsupported source

  Scenario: A lever that measures larger is rejected
    Given  a repository at the revision carrying the landed panic-abort lever
    When  the maintainer measures the "opt-level=s" lever
    Then  the lever is rejected with its measured comparison
    And  the build configuration is unchanged

  Scenario: A second lever is measured against the configuration carrying the first
    Given  a repository at the revision carrying the landed panic-abort lever
    When  the maintainer measures the "panic=abort,opt-level=s" levers
    Then  the first lever is applied
    And  the second lever is measured against the first lever's build
    And  the second lever is rejected

  Scenario: A nightly-only proposal is refused before a build
    Given  a repository at the revision carrying the landed panic-abort lever
    When  the maintainer proposes the "nightly" lever
    Then  the proposal is refused, naming the stable-toolchain constraint
    And  no baseline is built

  Scenario Outline: A proposal outside the supported approach is refused before a build
    Given  a repository at the revision carrying the landed panic-abort lever
    When  the maintainer proposes the "<lever>" lever
    Then  the proposal is refused, naming the <constraint> constraint
    And  no baseline is built
    Examples:
      | lever | constraint |
      | upx | single-file |
      | dynamic-linking | static-linking |

  Scenario: An unknown lever is refused before a build
    Given  a repository at the revision carrying the landed panic-abort lever
    When  the maintainer proposes the "turbo" lever
    Then  the proposal is refused, naming the unsupported lever
    And  no baseline is built

  Scenario: A build that fails records no delta
    Given  a fixture revision that does not build
    When  the maintainer measures the "panic=abort" lever
    Then  the build failure is recorded with no delta
