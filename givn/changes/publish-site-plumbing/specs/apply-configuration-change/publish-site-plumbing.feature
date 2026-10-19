@givn.delta @publish-site-plumbing

Feature: Publish site plumbing

  @givn.added @e2e @wip
  Scenario: Organization owner previews the pages plumbing and only the three live creations are named
    Given the applied state binds every object the earlier configuration changes declared
    And the pages repository has no deploy key and no rule set
    And the monorepo has no "PAGES_DEPLOY_KEY" secret
    And the declaration adds the pages publishing key, its secret and the pages rule set
    When the organization owner previews the declaration
    Then the preview names the pages deploy key as an object to create
    And the preview names the "PAGES_DEPLOY_KEY" secret as an object to create
    And the preview names the pages rule set as an object to create
    And the preview names the generated pages publishing key material as an object to create
    And the preview names no other live object to create or change
    And the live configuration is unchanged
    And the applied state is unchanged

  @givn.added @e2e @wip
  Scenario: Organization owner applies the previewed revision and the pages publishing credential is created
    Given the applied state binds every object the earlier configuration changes declared
    And a previewed revision that adds the pages publishing key, its secret and the pages rule set
    When the organization owner applies the previewed revision
    Then the apply reports the deploy key, the secret and the rule set as created
    And the pages repository carries a write-enabled deploy key titled "duty2cal.github.io publish"
    And the secret write carries a sealed ciphertext for the fetched repository public key and no plaintext key
    And the pages rule set is active and grants write-enabled deploy keys an always bypass
    And the pages rule set forbids deletion, force pushes and merge commits
    And the applied state records the deploy key, the secret and the rule set
    And the applied state records the generated pages publishing key material
    And a later targeted preview of the pages plumbing reports no differences

  @givn.added
  Scenario: The preview refuses each live object the revision would create and names it
    Given the applied state binds every object the earlier configuration changes declared
    And the pages repository already has a deploy key titled "duty2cal.github.io publish"
    And the monorepo already stores "PAGES_DEPLOY_KEY"
    And the pages repository already has a rule set named "pages protection"
    And the declaration adds the pages publishing key, its secret and the pages rule set
    When the organization owner previews the declaration
    Then the preview is refused and names the pages deploy key
    And the preview names the "PAGES_DEPLOY_KEY" secret
    And the preview names the pages rule set
    And the live configuration is unchanged
    And the applied state is unchanged

  @givn.added
  Scenario: An unreadable pages repository refuses the preview and names it
    Given the applied state binds every object the earlier configuration changes declared
    And the owner's authority cannot read the pages repository
    And the declaration adds the pages publishing key, its secret and the pages rule set
    When the organization owner previews the declaration
    Then the preview fails and names "duty2cal/duty2cal.github.io"
    And the live configuration is unchanged
    And the applied state is unchanged

  @givn.added
  Scenario: A live object created after the preview refuses the apply and names it
    Given the applied state binds every object the earlier configuration changes declared
    And a previewed revision that adds the pages publishing key, its secret and the pages rule set
    And the pages repository gains a deploy key titled "duty2cal.github.io publish" after the preview
    When the organization owner applies the previewed revision
    Then the apply is refused and names the pages deploy key
    And no live object the revision declares is created
    And the applied state is unchanged
    And the applied state is not marked stale

  @givn.added
  Scenario: A partial failure after the deploy key is created names the applied and failed objects and marks the state stale
    Given the applied state binds every object the earlier configuration changes declared
    And a previewed revision that adds the pages publishing key, its secret and the pages rule set
    And the owner's authority cannot store the monorepo's "PAGES_DEPLOY_KEY" secret
    And the owner's authority cannot create the pages rule set
    When the organization owner applies the previewed revision
    Then the apply fails and names the "PAGES_DEPLOY_KEY" secret
    And the failure report names the deploy key as created
    And the failure report names the "PAGES_DEPLOY_KEY" secret and the pages rule set as not created
    And the failure report states a 403 status
    And the pages repository carries the write-enabled deploy key
    And the monorepo does not store "PAGES_DEPLOY_KEY"
    And the pages repository does not carry the pages rule set
    And the applied state records the deploy key and not the secret
    And the applied state does not record the pages rule set
    And the stale marker names the deploy key and no other object
    And the applied state is marked stale
    And a new preview refuses and names the stale applied state

  @givn.added
  Scenario: A failed read-back after the publishing secret is stored is reported and the state is marked stale
    Given the applied state binds every object the earlier configuration changes declared
    And a previewed revision that adds the pages publishing key, its secret and the pages rule set
    And the routine's post-apply read of "PAGES_DEPLOY_KEY" fails once after the secret is stored
    When the organization owner applies the previewed revision
    Then the apply fails and names the "PAGES_DEPLOY_KEY" secret
    And the monorepo stores "PAGES_DEPLOY_KEY"
    And the failure report names the deploy key, the secret and the rule set as applied
    And the failure report states a 403 status
    And the applied state records the deploy key, the secret and the rule set
    And the applied state is marked stale
    And a new preview refuses and names the stale applied state

  @givn.added
  Scenario: An apply refused before the first live write leaves the state unchanged and writes no stale marker
    Given the applied state binds every object the earlier configuration changes declared
    And a previewed revision that adds the pages publishing key, its secret and the pages rule set
    And the owner's authority cannot create any live object the revision declares
    And the failed apply records the planned deploy key in its state without creating it live
    When the organization owner applies the previewed revision
    Then the apply fails and names the pages rule set
    And the failure report names the pages deploy key and the pages rule set as not created
    And no live object is created
    And the applied state is unchanged
    And the applied state is not marked stale

  @givn.added
  Scenario: A green apply after a partial failure creates only the objects the state does not bind and completes the revision
    Given the applied state binds the pages deploy key and not the publishing secret
    And the applied state does not record the pages rule set
    And the applied state is marked stale
    And the owner clears the stale marker after reading the partial report
    When the organization owner applies the previewed revision
    Then the preview names every declared object the applied state does not bind
    And the preview does not name the pages deploy key as an object to create
    And the apply creates the missing objects and leaves the bound deploy key unchanged
    And the applied state records the deploy key, the secret and the rule set
    And the applied state is no longer marked stale
    And a later targeted preview of the pages plumbing reports no differences

  @givn.added
  Scenario: Applying the pages plumbing leaves the published site and its source untouched
    Given the applied state binds every object the earlier configuration changes declared
    And the pages repository serves the stock site from "main" at "/"
    And a previewed revision that adds the pages publishing key, its secret and the pages rule set
    When the organization owner applies the previewed revision
    Then the pages repository still serves the stock site from "main" at "/"
    And the apply wrote no site file, no Pages configuration and no repository setting
    And the journal records no applied write other than the deploy key, the secret and the rule set
