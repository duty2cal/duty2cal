# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
#
# The site's existing Pages source (design.md "Declared objects and values"):
# the legacy build from `main` at `/`, imported rather than enabled anew.

resource "github_repository_pages" "pages" {
  repository = github_repository.pages.name
  build_type = "legacy"

  source {
    branch = "main"
    path   = "/"
  }
}
