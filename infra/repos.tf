# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
#
# The monorepo and the Pages repository with the decided values (design.md
# "Declared objects and values", proposal D2). The organization profile
# repository is declared separately in org_profile.tf. Every feature boolean
# is stated explicitly because the provider sends `false` for an omitted
# Optional-only boolean; omission would not be neutral. `license_template` is
# omitted because it only acts at creation. `pages` is ignored because the
# Pages source is declared by `github_repository_pages.pages` in pages.tf.

resource "github_repository" "mono" {
  name        = "duty2cal"
  description = "Convert a duty plan to a calendar"
  topics      = ["cli", "duty", "duty-planner", "icalendar", "pdf", "rust", "roste"]

  has_issues      = true
  has_projects    = false
  has_wiki        = false
  has_discussions = false

  allow_auto_merge       = false
  allow_forking          = true
  allow_merge_commit     = false
  allow_rebase_merge     = true
  allow_squash_merge     = false
  delete_branch_on_merge = true

  security_and_analysis {
    secret_scanning {
      status = "enabled"
    }
    secret_scanning_push_protection {
      status = "enabled"
    }
  }

  lifecycle {
    ignore_changes = [pages]
  }
}

resource "github_repository" "pages" {
  name        = "duty2cal.github.io"
  description = "duty2cal GitHub Page"
  visibility  = "public"

  has_issues   = false
  has_wiki     = false
  has_projects = false

  lifecycle {
    ignore_changes = [pages]
  }
}
