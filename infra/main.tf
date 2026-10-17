# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
#
# The provider and the organization's existing settings (design.md "Declared
# objects and values"). The token and the API base URL come from GITHUB_TOKEN
# and GITHUB_BASE_URL in the environment, so the tests can point the provider
# at the local fake without a declaration change (design.md "The adoption
# command", "Test seam: the fake GitHub API").

provider "github" {
  owner = "duty2cal"
}

resource "github_organization_settings" "org" {
  name          = "duty2cal"
  billing_email = var.billing_email

  # Today's live values: the profile fields are empty and the member defaults
  # are GitHub's read-only defaults. No draft profile text is adopted.
  description      = ""
  location         = ""
  company          = ""
  blog             = ""
  twitter_username = ""

  default_repository_permission            = "read"
  members_can_create_repositories          = true
  members_can_create_public_repositories   = true
  members_can_create_private_repositories  = true
  members_can_create_internal_repositories = false
}
