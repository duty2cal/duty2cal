Feature: Publish a public release

  Scenario: A prepared release offers one archive for every supported platform
    Given  an executable built for every supported platform
    When  the maintainer prepares the release for version 0.1.0
    Then  the prepared release holds exactly these archives, each with its checksum file
      | archive | executable |
      | duty2cal-0.1.0-windows-x86_64.tar.gz | duty2cal.exe |
      | duty2cal-0.1.0-linux-x86_64.tar.gz | duty2cal |
      | duty2cal-0.1.0-linux-arm64.tar.gz | duty2cal |
      | duty2cal-0.1.0-macos-arm64.tar.gz | duty2cal |
      | duty2cal-0.1.0-macos-x86_64.tar.gz | duty2cal |
    And  every archive holds only its own executable

  Scenario: A platform whose executable was not built stops the release
    Given  an executable built for every supported platform except "linux-arm64"
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation fails and names "linux-arm64"
    And  the prepared release holds no archive

  Scenario: An executable for an unrecognised platform is refused
    Given  an executable built for every supported platform
    And  an executable presented as the unrecognised platform "freebsd-x86_64"
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation fails and names "freebsd-x86_64"
    And  the prepared release holds no archive

  Scenario: Each archive's checksum covers that archive
    Given  a prepared release for version 0.1.0
    When  anyone verifies the checksum against the archive for "linux-x86_64"
    Then  the checksum matches the archive it names

  Scenario: Preparing the same release twice produces the same archives
    Given  a prepared release for version 0.1.0
    When  the maintainer prepares version 0.1.0 again from the same executables
    Then  the archives are identical to the first preparation, byte for byte

  Scenario: An already prepared version is not replaced
    Given  a prepared release for version 0.1.0
    When  the maintainer prepares version 0.1.0 in the same place again
    Then  the preparation fails and names version 0.1.0
    And  the staged archives keep their earlier modification times

  Scenario: An unreadable executable stops the preparation
    Given  an executable built for the platform "linux-x86_64" that cannot be read
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation fails and names "linux-x86_64"
    And  the prepared release holds no archive

  Scenario: The release text states the released revisions' release notes
    Given  a history whose released revisions name a new capability, a fix, and work no user can see
    And  one released change is named by several revisions
    When  the maintainer prepares the release for version 0.2.0
    Then  the release text names version 0.2.0
    And  the release text names the new capability before the fix
    And  the release text names the released change once
    And  the release text omits the work no user can see

  @e2e
  Scenario: Maintainer prepares a release and runs the archived executable
    Given  the executable built for this machine
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation reports the archive and its checksum for this machine's platform
    And  the release text names version 0.1.0
    When  the maintainer extracts the archive for this machine's platform
    And  runs the executable taken from that archive
    Then  the executable prints "usage: duty2cal <duty-plan.pdf>" and reports failure through its process exit status
  Scenario: A version's notes group the changes a reader can see
    Given  a history whose released revisions name a new capability, a fix, and work no user can see
    And  a revision typed as a fix of that capability whose words name the project's own tooling
    And  a revision whose subject borrows a tooling word for a crew member's own work
    When  the maintainer generates the change log for version 0.2.0
    Then  the notes for version 0.2.0 name the new capability and the fix
    And  the notes name the new capability before the fix
    And  the notes name no revision whose work no user can see
    And  the notes name no revision whose words name the project's own tooling
    And  the notes name the revision whose subject borrows a tooling word for a crew member's own work

  Scenario: A version that changes nothing a user can see is refused
    Given  a history whose released revisions name only work no user can see, including dependency updates that changed no path a user receives
    When  the maintainer generates the change log for version 0.2.0
    Then  the generation fails and states that the version changes nothing a user can see

  Scenario: The change log keeps the earlier versions beside the new one
    Given  a change log whose newest section is version 0.1.0
    When  the maintainer generates the change log for version 0.2.0
    Then  version 0.2.0 is the newest section of the change log
    And  the section for version 0.1.0 is unchanged

  Scenario: A change log that does not match the released revision is refused
    Given  a committed change log whose newest section differs from what the history states
    When  the change log is checked against the released revision
    Then  the check fails and names the version whose section differs

  Scenario: Each version's section is dated and compared with the one before it
    Given  a change log holding version 0.1.0 and a history releasing version 0.2.0
    When  the maintainer generates the change log for version 0.2.0
    Then  the section for version 0.2.0 states its own date
    And  the section for version 0.2.0 states the comparison against version 0.1.0

  Scenario: The first released version is recorded from the history
    Given  a history whose released revisions name a new capability and work no user can see
    And  no change log exists yet
    When  the maintainer generates the change log for version 0.1.0
    Then  the change log holds one section, for version 0.1.0
    And  that section names the new capability
    And  that section states no comparison against an earlier version

  Scenario: A version's section can be rendered before its tag exists
    Given  a history whose newest revision names a new capability and carries its own date
    And  the version is not tagged yet
    When  the maintainer generates the change log for version 0.1.0
    Then  the change log holds one section, for version 0.1.0
    And  that section states the date of the revision it will be tagged at

  @e2e
  Scenario: Maintainer generates the change log for a version and reads its newest section
    Given  a repository whose history releases version 0.1.0
    When  the maintainer generates the change log for version 0.1.0
    Then  the change log file holds a section for version 0.1.0
    And  that section names the released change a user can see

  Scenario: The prepared release carries the change log its section came from
    Given  a history whose released revisions name a new capability and work no user can see
    When  the maintainer prepares the release for version 0.2.0
    Then  the prepared release stages the change log for version 0.2.0
    And  the staged change log holds the section the release text states

  Scenario: A capability missing from the configured allow-list is refused
    Given  a capability the repository holds that the configuration does not name
    When  the maintainer generates the change log for version 0.2.0
    Then  the generation fails and names the capability missing from the configuration

  Scenario: A committed change log missing the released version's section is refused
    Given  a committed change log that holds no section for version 0.2.0
    When  the change log is checked against the released revision
    Then  the check fails and names version 0.2.0
  Scenario: A version's notes list only the changes a user can receive
    Given  a history whose released revisions changed the converter, the release machinery, and both
    When  the maintainer generates the change log for version 0.2.0
    Then  the section for version 0.2.0 names the change to the converter
    And  the section for version 0.2.0 names the change that touched both
    And  the section for version 0.2.0 names no change to the release machinery

  Scenario: A version's notes name a change to the packaging a user receives
    Given  a history whose released revisions changed the packaging
    When  the maintainer generates the change log for version 0.2.0
    Then  the section for version 0.2.0 names the change to the packaging

  Scenario: A configured path a user receives that the repository does not hold is refused
    Given  a configured user-receiving path the repository does not hold
    When  the maintainer generates the change log for version 0.2.0
    Then  the generation fails and names the path missing from the repository
  Scenario: A version whose only changes are dependency updates is announced
    Given  a history whose released revisions carry only dependency updates
    When  the maintainer generates the change log for version 0.2.0
    Then  the notes for version 0.2.0 name the dependency updates
    And  the notes name no housekeeping revision

  Scenario: Identical dependency update subjects collapse to one entry
    Given  a history whose dependency updates repeat one subject
    When  the maintainer generates the change log for version 0.2.0
    Then  the notes name that dependency update once

  Scenario: A dependency-only version can be prepared
    Given  a history whose released revisions carry only dependency updates
    And  an executable built for every supported platform
    When  the maintainer prepares the release for version 0.2.0
    Then  the prepared release holds an archive for every supported platform
    And  the release text names version 0.2.0

  Scenario: The Dependencies group follows Reversals
    Given  a history whose released revisions include a fix, a reversal, and dependency updates
    When  the maintainer generates the change log for version 0.2.0
    Then  the notes name the fix before the dependency update
    And  the notes name the reversal before the dependency update
    And  the notes name the dependency updates in history order

  Scenario: A dependency update that changes only the project's checks is not listed
    Given  a history whose dependency updates changed only the project's own checks
    When  the maintainer generates the change log for version 0.2.0
    Then  the notes name the converter fix
    And  the notes name no dependency update
    And  the notes name no Dependencies group

  Scenario: A dependency update that changes the release workflow is listed
    Given  a history whose dependency update changed the release workflow
    When  the maintainer generates the change log for version 0.2.0
    Then  the notes name the dependency update

  Scenario: A dependency update typed with a capability is announced once
    Given  a history whose dependency update carries a capability scope
    When  the maintainer generates the change log for version 0.2.0
    Then  the notes name the capability-scoped update once
    And  the Dependencies group names only the update that carries no capability scope

  Scenario: The release text and the change log state the same dependency updates
    Given  a history whose released revisions carry only dependency updates
    And  an executable built for every supported platform
    When  the maintainer prepares the release for version 0.2.0
    Then  the release text names the dependency updates
    And  the staged change log holds the section the release text states
