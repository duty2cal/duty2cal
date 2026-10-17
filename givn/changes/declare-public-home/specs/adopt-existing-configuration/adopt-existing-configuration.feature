@givn.delta @adopt-existing-configuration

Feature: Adopt the existing public-home configuration

  @givn.added @e2e @wip
  Scenario: Maintainer adopts the existing public home and every object is bound without changing it
    Given the public home already exists with its hand-made configuration
    And the declared configuration names the organization, both repositories, the default-branch rule and the site source
    And the selected batch names those objects
    When the organization owner adopts the batch
    Then the adoption reports the organization, both repositories, the default-branch rule and the site source as bound
    And the live configuration is unchanged

  @givn.added
  Scenario: An invalid declaration refuses before any object is read
    Given the declared configuration fails validation
    When the organization owner adopts the batch
    Then the adoption refuses the batch and names the declaration error
    And no object is read from the live organization
    And no object in the batch is bound

  @givn.added
  Scenario: A malformed batch entry refuses before any import
    Given the selected batch has a line without an import identifier
    When the organization owner adopts the batch
    Then the adoption refuses the batch and names the malformed batch line
    And no object is imported
    And no object in the batch is bound

  @givn.added
  Scenario: A declared object that does not exist live refuses the whole batch
    Given the declared configuration names an additional repository "duty2cal/example"
    And the selected batch names that repository
    And it does not exist live
    When the organization owner adopts the batch
    Then the adoption refuses the batch and names "duty2cal/example"
    And the refusal states that the repository does not exist live
    And no object in the batch is bound
    And nothing is created in the live organization

  @givn.added @wip
  Scenario: An object already bound is not bound again
    Given the monorepo is already bound in the applied state
    When the organization owner adopts the batch again
    Then the adoption refuses the batch and names "duty2cal/duty2cal"
    And the applied state binds the monorepo once

  @givn.added @wip
  Scenario: An unreadable object refuses the whole batch
    Given the organization settings cannot be read with the owner's authority
    When the organization owner adopts the batch
    Then the adoption refuses the batch and names the organization settings
    And the refusal states that the organization settings cannot be read
    And no object in the batch is bound
    And the live configuration is unchanged

  @givn.added @wip
  Scenario: A batch entry the declaration does not name refuses the whole batch
    Given the selected batch names the archived repository, which the declared configuration does not name
    When the organization owner adopts the batch
    Then the adoption refuses the batch and names the archived repository
    And no object in the batch is bound

  @givn.added @wip
  Scenario: A live object that changes after it was read refuses the batch
    Given the monorepo's live settings change after the adoption reads them
    When the organization owner adopts the batch
    Then the adoption refuses the batch and names "duty2cal/duty2cal"
    And no object in the batch is bound

  @givn.added @wip
  Scenario: An applied state encrypted with another passphrase refuses before any binding
    Given the applied state was encrypted with a different passphrase
    When the organization owner adopts the batch
    Then the adoption refuses the batch and names the applied state
    And no object in the batch is bound
    And the live configuration is unchanged

  @givn.added @wip
  Scenario: A failure to record the applied state binds nothing and changes nothing
    Given the applied state cannot be written
    When the organization owner adopts the batch
    Then the adoption refuses the batch and names the applied state
    And no object in the batch is bound
    And the live configuration is unchanged

  @givn.added @wip
  Scenario: Only the selected batch is bound
    Given the declared configuration names both repositories
    And the selected batch names only the organization and the monorepo
    When the organization owner adopts the batch
    Then the adoption reports the organization and the monorepo as bound
    And the site repository is not bound
