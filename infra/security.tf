# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
#
# The monorepo's Dependabot settings (design.md "The declaration, The two
# settings toggles"). Both are already enabled live and are bound by the second
# adoption batch before the apply, so the apply does not create them. The
# dependency is GitHub's: vulnerability alerts must be enabled before
# Dependabot security updates can be.

resource "github_repository_vulnerability_alerts" "mono" {
  repository = github_repository.mono.name
  enabled    = true
}

resource "github_repository_dependabot_security_updates" "mono" {
  repository = github_repository.mono.name
  enabled    = true

  depends_on = [github_repository_vulnerability_alerts.mono]
}
