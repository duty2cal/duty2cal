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

  # The public profile fields, established by the `state-organization-fields`
  # change. The description is the one support sentence without the project's
  # name (GitHub renders the name beside it); the URL is the Public website's
  # canonical address; the mail address is the Impressum's alias, so a reader
  # who writes from the organization page reaches the same mailbox.
  description = "Converts a Lufthansa cabin-crew duty plan published as a Jeppesen PDF export into one calendar file."
  location    = "Berlin, Germany"
  company     = ""
  blog        = "https://duty2cal.github.io/"
  email       = "duty2cal662.abridge974@aleeas.com"

  # No social account is declared: no `github_organization_social_account`
  # resource exists, the Twitter/X field stays empty, and the project has no
  # Mastodon account. A social profile would be a second telemedium with its
  # own notice obligations; the project keeps the site, the profile and the
  # README as its public carriers.
  twitter_username = ""

  default_repository_permission            = "read"
  members_can_create_repositories          = true
  members_can_create_public_repositories   = true
  members_can_create_private_repositories  = true
  members_can_create_internal_repositories = false
}
