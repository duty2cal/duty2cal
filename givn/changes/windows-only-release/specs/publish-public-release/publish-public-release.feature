@givn.delta @publish-public-release

Feature: Publish a public release

  A version offers one downloadable archive for Windows x86_64, the supported
  platform. Other platforms are not supported or distributed; the source can be
  built with a Rust toolchain.

  @givn.modified
  Scenario: A prepared release offers one archive for every supported platform
    Given  an executable built for every supported platform
    When  the maintainer prepares the release for version 0.1.0
    Then  the prepared release holds exactly these archives, each with its checksum file
      | archive | executable |
      | duty2cal-0.1.0-windows-x86_64.tar.gz | duty2cal.exe |
    And  every archive holds only its own executable

  @givn.modified
  Scenario: A platform whose executable was not built stops the release
    Given  no executable was built for the supported platform
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation fails and names "windows-x86_64"
    And  the prepared release holds no archive

  @givn.modified @wip
  Scenario: Each archive's checksum covers that archive
    Given  a prepared release for version 0.1.0
    When  anyone verifies the checksum against the archive for "windows-x86_64"
    Then  the checksum matches the archive it names

  @givn.modified @wip
  Scenario: An unreadable executable stops the preparation
    Given  an executable built for the platform "windows-x86_64" that cannot be read
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation fails and names "windows-x86_64"
    And  the prepared release holds no archive

  @givn.removed
  Scenario: Maintainer prepares a release and runs the archived executable
    Given this scenario is retired

  @givn.added @e2e @wip
  Scenario: Maintainer prepares a Windows release and inspects the archived executable
    Given  a stand-in executable built for "windows-x86_64"
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation reports the archive and its checksum for "windows-x86_64"
    And  the release text names version 0.1.0
    When  the maintainer extracts the archive for "windows-x86_64"
    Then  the extracted archive holds exactly the executable "duty2cal.exe"
    And  the extracted executable is the executable the preparation staged

  @givn.added @wip
  Scenario: An executable for the previously offered Linux platform is refused
    Given  an executable built for every supported platform
    And  an executable presented as the previously offered platform "linux-x86_64"
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation fails and names "linux-x86_64"
    And  the prepared release holds no archive
