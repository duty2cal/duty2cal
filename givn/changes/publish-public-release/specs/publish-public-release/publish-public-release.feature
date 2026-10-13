@givn.delta @publish-public-release

Feature: Publish a public release

  The maintainer prepares one archive and one checksum for every supported
  platform, inspects the set, and publishes it as a release that a crew member
  can obtain for their own machine and trace to the revision it was built from.

  @givn.added
  Scenario: A prepared release offers one archive for every supported platform
    Given an executable built for every supported platform
    When the maintainer prepares the release for version 0.1.0
    Then the prepared release holds exactly these archives, each with its checksum file
      | archive                              | executable   |
      | duty2cal-0.1.0-windows-x86_64.tar.gz | duty2cal.exe |
      | duty2cal-0.1.0-linux-x86_64.tar.gz   | duty2cal     |
      | duty2cal-0.1.0-linux-arm64.tar.gz    | duty2cal     |
      | duty2cal-0.1.0-macos-arm64.tar.gz    | duty2cal     |
      | duty2cal-0.1.0-macos-x86_64.tar.gz   | duty2cal     |
    And every archive holds only its own executable

  @givn.added
  Scenario: A platform whose executable was not built stops the release
    Given an executable built for every supported platform except "linux-arm64"
    When the maintainer prepares the release for version 0.1.0
    Then the preparation fails and names "linux-arm64"
    And the prepared release holds no archive

  @givn.added
  Scenario: An executable for an unrecognised platform is refused
    Given an executable built for every supported platform
    And an executable presented as the unrecognised platform "freebsd-x86_64"
    When the maintainer prepares the release for version 0.1.0
    Then the preparation fails and names "freebsd-x86_64"
    And the prepared release holds no archive

  @givn.added
  Scenario: Each archive's checksum covers that archive
    Given a prepared release for version 0.1.0
    When anyone verifies the checksum against the archive for "linux-x86_64"
    Then the checksum matches the archive it names

  @givn.added
  Scenario: Preparing the same release twice produces the same archives
    Given a prepared release for version 0.1.0
    When the maintainer prepares version 0.1.0 again from the same executables
    Then the archives are identical to the first preparation, byte for byte

  @givn.added
  Scenario: An already prepared version is not replaced
    Given a prepared release for version 0.1.0
    When the maintainer prepares version 0.1.0 in the same place again
    Then the preparation fails and names version 0.1.0
    And the staged archives keep their earlier modification times

  @givn.added
  Scenario: An unreadable executable stops the preparation
    Given an executable built for the platform "linux-x86_64" that cannot be read
    When the maintainer prepares the release for version 0.1.0
    Then the preparation fails and names "linux-x86_64"
    And the prepared release holds no archive

  @givn.added @wip
  Scenario: The release text states the released revisions' release notes
    Given a history whose released revisions carry release-note subjects, including one subject repeated by several revisions and one subject that carries no release-note type, and whose housekeeping revisions do not
    When the maintainer prepares the release for version 0.2.0
    Then the release text names version 0.2.0
    And the release text lists these release notes in release order
      | release note                               |
      | Offer an archive for every supported platform |
      | Publish only an archive the evidence supports |
    And the release text omits the housekeeping revisions and the subject that carries no release-note type

  @givn.added @e2e @wip
  Scenario: Maintainer prepares a release and runs the archived executable
    Given the executable built for this machine
    When the maintainer prepares the release for version 0.1.0
    Then the preparation reports the archive and its checksum for this machine's platform
    And the release text names version 0.1.0
    When the maintainer extracts the archive for this machine's platform
    And runs the executable taken from that archive
    Then the executable prints "usage: duty2cal <duty-plan.pdf>" and reports failure through its process exit status
