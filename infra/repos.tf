# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
#
# Both repositories with the decided values (design.md "Declared objects and
# values", proposal D2). Every feature boolean is stated explicitly because the
# provider sends `false` for an omitted Optional-only boolean; omission would
# not be neutral. `license_template` is omitted because it only acts at
# creation. `pages` is ignored because the Pages source is declared by
# `github_repository_pages.pages` in pages.tf.

resource "github_repository" "mono" {
  name        = "duty2cal"
  description = "Convert a duty plan to a calendar"
  topics      = ["rust", "icalendar", "roster", "duty-plan", "pdf", "cli"]

  has_issues      = true
  has_projects    = false
  has_wiki        = false
  has_discussions = false

  allow_auto_merge       = false
  allow_forking          = true
  allow_merge_commit     = false
  allow_rebase_merge     = false
  allow_squash_merge     = true
  delete_branch_on_merge = true

  lifecycle {
    ignore_changes = [pages]
  }
}

resource "github_repository" "pages" {
  name        = "duty2cal.github.io"
  description = ""
  visibility  = "public"

  has_issues   = false
  has_wiki     = false
  has_projects = true

  lifecycle {
    ignore_changes = [pages]
  }
}
