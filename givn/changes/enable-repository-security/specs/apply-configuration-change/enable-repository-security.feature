@givn.delta @enable-repository-security

Feature: Enable repository security

  @givn.added @e2e
  Scenario: Organization owner previews the security settings and only the declared differences are named
    Given the applied state binds the organization, both repositories, the default-branch rule, the site source and the two Dependabot settings
    And that applied state was produced by the base declaration's own apply
    And the declaration enables secret scanning and push protection on the monorepo
    And the monorepo reports both settings disabled
    When the organization owner previews the declaration
    Then the preview names secret scanning and push protection as the only live changes
    And the plan's only changed resource is "github_repository.mono"
    And the plan's only changed live attributes are "secret_scanning" and "secret_scanning_push_protection"
    And the preview reports no object to create or delete
    And no write request reaches GitHub

  @givn.added @e2e @wip
  Scenario: Organization owner applies the previewed revision and the security settings are enabled
    Given the applied state binds every declared object
    And a previewed revision that enables secret scanning and push protection
    When the organization owner applies the previewed revision
    Then the apply reports the monorepo as changed and exits successfully
    And the monorepo reports secret scanning and push protection enabled
    And the applied state records both settings as enabled
    And the journal's repository request changed only "security_and_analysis" with both statuses enabled
    And no other declared field of the monorepo changed
    And a later preview reports no differences

  @givn.added
  Scenario: An unreadable repository setting refuses the preview and names the repository
    Given the owner's authority cannot read the monorepo's security settings
    When the organization owner previews the declaration
    Then the preview fails and names "duty2cal/duty2cal"
    And no write request reaches GitHub
    And the applied state is unchanged

  @givn.added
  Scenario: An apply whose first change is refused changes nothing
    Given the applied state binds every declared object
    And a previewed revision that enables secret scanning and push protection
    And the owner's authority cannot change the monorepo
    When the organization owner applies the previewed revision
    Then the apply fails and names "duty2cal/duty2cal"
    And no setting changes live
    And the applied state is unchanged
    And the applied state is not marked stale
    And a later preview names the same change again

  @givn.added
  Scenario: A live edit between the preview and the apply is refused and named
    Given the applied state binds every declared object
    And a previewed revision that enables secret scanning and push protection
    And the monorepo's projects are switched on by hand after the preview
    When the organization owner applies the previewed revision
    Then the apply is refused before any write and names "duty2cal/duty2cal"
    And the only live change is the manual edit
    And the saved plan is not applied
    And a new preview names the projects change

  @givn.added
  Scenario: The preview refuses to add a Dependabot setting that already exists live and is unbound
    Given the applied state binds the organization, both repositories, the default-branch rule and the site source
    And the applied state does not bind either Dependabot setting
    And the monorepo reports Dependabot alerts and Dependabot security updates enabled
    And the declaration enables Dependabot alerts and Dependabot security updates
    When the organization owner previews the declaration
    Then the preview is refused and names "github_repository_vulnerability_alerts.mono"
    And the preview names "github_repository_dependabot_security_updates.mono"
    And no write request reaches GitHub
    And the applied state is unchanged

  @givn.added
  Scenario: A failed read-back after the PATCH is reported and the state is marked stale
    Given the applied state binds every declared object
    And a previewed revision that enables secret scanning and push protection
    And the read of the monorepo fails once after the PATCH is applied
    When the organization owner applies the previewed revision
    Then the apply fails and names "duty2cal/duty2cal"
    And the monorepo reports secret scanning and push protection enabled
    And the applied state is marked stale
    And a new preview refuses and names the stale applied state

  @givn.added
  Scenario: A partial failure reports the applied settings and names the failed one
    Given the second adoption batch was skipped, so the applied state does not bind the Dependabot settings
    And the owner's authority cannot change the Dependabot security updates
    When the organization owner applies the previewed revision outside the routine
    Then the apply reports secret scanning and push protection enabled
    And the apply names the Dependabot security updates as failed
    And the applied state records the monorepo and the vulnerability alerts but not the security updates
    And the applied state is marked stale
    And a preview made after the failure refuses and names the stale applied state
