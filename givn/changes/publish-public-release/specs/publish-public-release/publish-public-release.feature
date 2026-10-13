@givn.delta @publish-public-release

Feature: Publish a public release

  The maintainer prepares one archive and one checksum for every supported
  platform, inspects the set, and publishes it as a release that a crew member
  can obtain for their own machine and trace to the revision it was built from.

  @givn.added @wip
  Scenario: A prepared release offers one archive for every supported platform
    Given an executable built for every supported platform
    When the maintainer prepares the release for version 0.1.0
    Then the prepared release holds exactly these archives
      | archive                              | executable   |
      | duty2cal-0.1.0-windows-x86_64.tar.gz | duty2cal.exe |
      | duty2cal-0.1.0-linux-x86_64.tar.gz   | duty2cal     |
      | duty2cal-0.1.0-linux-arm64.tar.gz    | duty2cal     |
      | duty2cal-0.1.0-macos-arm64.tar.gz    | duty2cal     |
      | duty2cal-0.1.0-macos-x86_64.tar.gz   | duty2cal     |
    And every archive holds only its own executable

  @givn.added @wip
  Scenario: A platform whose executable was not built stops the release
    Given an executable built for every supported platform except "linux-arm64"
    When the maintainer prepares the release for version 0.1.0
    Then the preparation fails and names "linux-arm64"
    And the prepared release holds no archive

  @givn.added @wip
  Scenario: An executable for an unrecognised platform is refused
    Given an executable built for every supported platform
    And an executable presented as the unrecognised platform "freebsd-x86_64"
    When the maintainer prepares the release for version 0.1.0
    Then the preparation fails and names "freebsd-x86_64"
    And the prepared release holds no archive

  @givn.added @wip
  Scenario: Each archive's checksum covers that archive
    Given a prepared release for version 0.1.0
    When anyone verifies the checksum against the archive for "linux-x86_64"
    Then the checksum matches the archive it names

  @givn.added @wip
  Scenario: Preparing the same release twice produces the same archives
    Given a prepared release for version 0.1.0
    When the maintainer prepares version 0.1.0 again from the same executables
    Then the archives are identical to the first preparation, byte for byte

  @givn.added @wip
  Scenario: An already prepared version is not replaced
    Given a prepared release for version 0.1.0
    When the maintainer prepares version 0.1.0 in the same place again
    Then the preparation fails and names version 0.1.0
    And the prepared release still holds exactly its five archives

  @givn.added @wip
  Scenario: An unreadable executable stops the preparation
    Given an executable built for the platform "linux-x86_64" that cannot be read
    When the maintainer prepares the release for version 0.1.0
    Then the preparation fails and names "linux-x86_64"
    And the prepared release holds no archive

  @givn.added @wip
  Scenario: The release text states the released revisions' release notes
    Given a history whose released revisions carry release-note subjects and whose housekeeping revisions do not
    When the maintainer prepares the release for version 0.2.0
    Then the release text names version 0.2.0
    And the release text lists these release notes in release order
      | release note                               |
      | Offer an archive for every supported platform |
      | Publish only an archive the evidence supports |
    And the release text omits the housekeeping revisions

  @givn.added @e2e @wip
  Scenario: Maintainer prepares a release and runs the archived executable
    Given the executable built for this machine
    And a prepared release for version 0.1.0 holding this machine's archive
    When the maintainer extracts the archive for this machine's platform
    And runs the executable taken from that archive
    Then the executable states its usage and names the duty plan it expects
