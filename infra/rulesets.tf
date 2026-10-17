# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
#
# The existing `main protection` rule set (design.md "Declared objects and
# values", proposal D3). The live name is declared so the binding is the same
# object. No bypass actor is declared, and no legacy
# `github_branch_protection` resource exists; retiring the duplicate legacy
# protection belongs to the first configuration change.

resource "github_repository_ruleset" "main" {
  name        = "main protection"
  repository  = github_repository.mono.name
  target      = "branch"
  enforcement = "active"

  conditions {
    ref_name {
      include = ["~DEFAULT_BRANCH"]
      exclude = []
    }
  }

  rules {
    deletion                = true
    non_fast_forward        = true
    required_linear_history = true
  }
}
