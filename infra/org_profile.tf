# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
#
# The organization profile repository and its reviewed content (design.md "The
# declaration"). `auto_init` creates the default branch the file resources
# write to; the three feature toggles are stated false because the provider
# sends `false` for an omitted Optional-only boolean.
#
# The source directory is a variable because the declaration is planned from
# three workspace shapes: infra/ itself, the adoption command's staging copy,
# and the test/setup workspace copies. The default is ../org beside the
# declaration; a caller that copies the declaration elsewhere sets the variable
# (or places org/ beside it).

variable "org_source_dir" {
  type        = string
  default     = ""
  description = "Directory holding the reviewed org/ public copy; empty means ../org beside the declaration."
}

locals {
  org_source_dir = var.org_source_dir != "" ? var.org_source_dir : "${path.module}/../org"
}

resource "github_repository" "dot_github" {
  name        = ".github"
  description = "Organization profile and default community health files"
  visibility  = "public"

  has_issues      = false
  has_projects    = false
  has_wiki        = false
  has_discussions = false

  auto_init = true
}

resource "github_repository_file" "org_profile_readme" {
  repository          = github_repository.dot_github.name
  branch              = "main"
  file                = "profile/README.md"
  content             = file("${local.org_source_dir}/profile/README.md")
  commit_message      = "chore: update organization profile"
  overwrite_on_create = true
}

resource "github_repository_file" "org_security" {
  repository          = github_repository.dot_github.name
  branch              = "main"
  file                = "SECURITY.md"
  content             = file("${local.org_source_dir}/SECURITY.md")
  commit_message      = "chore: update security policy"
  overwrite_on_create = true
}
