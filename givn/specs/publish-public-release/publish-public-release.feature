@documentation
Feature: Publish a public release

  Scenario: A prepared release offers one archive for every supported platform
    Given  an executable built for every supported platform
    When  the maintainer prepares the release for version 0.1.0
    Then  the prepared release holds exactly these archives, each with its checksum file
      | archive | executable |
      | duty2cal-0.1.0-windows-x86_64.tar.gz | duty2cal.exe |
    And  every archive holds only its own executable

  Scenario: A platform whose executable was not built stops the release
    Given  no executable was built for the supported platform
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation fails and names "windows-x86_64"
    And  the prepared release holds no archive

  Scenario: An executable for an unrecognised platform is refused
    Given  an executable built for every supported platform
    And  an executable presented as the unrecognised platform "freebsd-x86_64"
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation fails and names "freebsd-x86_64"
    And  the prepared release holds no archive

  Scenario: Each archive's checksum covers that archive
    Given  a prepared release for version 0.1.0
    When  anyone verifies the checksum against the archive for "windows-x86_64"
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
    Given  an executable built for the platform "windows-x86_64" that cannot be read
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation fails and names "windows-x86_64"
    And  the prepared release holds no archive

  Scenario: The release text states the released revisions' release notes
    Given  a history whose released revisions name a new capability, a fix, and work no user can see
    And  one released change is named by several revisions
    When  the maintainer prepares the release for version 0.2.0
    Then  the release text names version 0.2.0
    And  the release text names the new capability before the fix
    And  the release text names the released change once
    And  the release text omits the work no user can see

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

  Scenario: An emptied group before Dependencies is not listed
    Given  a history whose capability revision changed only a path no user receives, beside a dependency update that changed the dependency graph
    When  the maintainer generates the change log for version 0.2.0
    Then  the notes name the dependency update in Dependencies
    And  the notes name no New capabilities group

  Scenario: The release text and the change log state the same dependency updates
    Given  a history whose released revisions carry only dependency updates
    And  an executable built for every supported platform
    When  the maintainer prepares the release for version 0.2.0
    Then  the release text names the dependency updates
    And  the staged change log holds the section the release text states
  Scenario: A tagged revision whose checks are still running is not published until they conclude
    Given  a tagged revision whose checks are still running
    When  the publication gate examines the tagged revision
    Then  the gate reports the still-running checks
    And  the gate reports the tagged revision ready only after every check has concluded successfully

  Scenario: A completed check that is not successful refuses publication
    Given  a tagged revision whose checks include a completed failure and a check still running
    When  the publication gate examines the tagged revision
    Then  the gate refuses publication, naming the failed check
    And  the gate does not wait for the check still running

  Scenario: A check that does not conclude within the bounded wait refuses publication
    Given  a tagged revision whose checks never conclude
    When  the publication gate examines the tagged revision until the elapsed limit
    Then  the gate refuses publication, naming the still-running checks

  Scenario: A skipped or neutral check does not refuse publication
    Given  a tagged revision whose completed checks are one success, one skipped, and one neutral
    When  the publication gate examines the tagged revision
    Then  the gate reports the tagged revision ready to publish

  Scenario: A tagged revision whose checks all concluded without success is refused
    Given  a tagged revision whose completed checks are one skipped and one neutral
    When  the publication gate examines the tagged revision
    Then  the gate refuses publication, stating that no check concluded successfully

  Scenario: The check-run source cannot be read
    Given  a tagged revision whose check runs cannot be read
    When  the publication gate examines the tagged revision
    Then  the gate refuses publication, stating that the check runs could not be read

  Scenario: A tagged revision with no check evidence is refused
    Given  a tagged revision whose only check runs belong to the release workflow
    When  the publication gate examines the tagged revision
    Then  the gate refuses publication, stating that the revision carries no check evidence

  Scenario: A branch and its tag pushed together publish only after the checks in both suites conclude
    Given  the default branch and a version tag pushed together, each starting a check suite on the same revision
    When  the publication gate examines the tagged revision
    Then  the gate waits for both suites' checks instead of refusing them as not yet concluded
    And  the gate reports the revision ready to publish only once every check in both suites has concluded successfully
  @e2e
  Scenario: Maintainer prepares a Windows release and inspects the archived executable
    Given  a stand-in executable built for "windows-x86_64"
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation reports the archive and its checksum for "windows-x86_64"
    And  the release text names version 0.1.0
    When  the maintainer extracts the archive for "windows-x86_64"
    Then  the extracted archive holds exactly the executable "duty2cal.exe"
    And  the extracted executable is the executable the preparation staged

  Scenario: An executable for the previously offered Linux platform is refused
    Given  an executable built for every supported platform
    And  an executable presented as the previously offered platform "linux-x86_64"
    When  the maintainer prepares the release for version 0.1.0
    Then  the preparation fails and names "linux-x86_64"
    And  the prepared release holds no archive
  Scenario: A revision that changes the public website is announced in the release notes
    Given  a released revision of the capability "publish-public-website" that changed "web/"
    And  a released revision that changed only the project's documents
    When  the maintainer generates the change log for version 0.2.0
    Then  the notes for version 0.2.0 name the change to the public website
    And  the notes for version 0.2.0 name no revision that changed only the project's documents

  Scenario: The public text states the browser route beside the downloadable archive platform
    Given  the repository's public text
    When  a reader reads the public text
    Then  the public text states that a current browser converts a plan on the public website at "https://duty2cal.github.io/"
    And  the public text names Windows x86_64 as the only published archive platform
    And  the public text states that builds for other operating systems are available on request and that their source can be built with a Rust toolchain
    And  the public text names no archive for a platform other than Windows x86_64

  @site-publication
  Scenario: A release whose hosting target does not hold the tagged site revision is refused and publishes no release
    Given  a prepared release for the version "v0.5.0"
    And  a hosting target whose default branch does not hold the Site output built from the tagged revision "v0.5.0"
    When  the maintainer publishes the release for "v0.5.0"
    Then  the release is refused and names the site revision the hosting target holds
    And  no release is published for "v0.5.0"
    And  the prepared release's staged archive is unchanged
