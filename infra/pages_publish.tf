# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
#
# The pages publishing credential and the rule set that admits it
# (design.md "The declaration"). One Ed25519 key pair is generated inside the
# declaration; the public half becomes a write-enabled deploy key on the pages
# repository, and the private half becomes the monorepo's PAGES_DEPLOY_KEY
# secret. The key pair has no remote object: it exists only in the encrypted
# applied state.

resource "tls_private_key" "pages_publish" {
  algorithm = "ED25519"
}

# The provider defaults read_only to true; an omitted attribute would register
# a read-only key and every publish would fail.
resource "github_repository_deploy_key" "pages_publish" {
  repository = github_repository.pages.name
  title      = "duty2cal.github.io publish"
  key        = trimspace(tls_private_key.pages_publish.public_key_openssh)
  read_only  = false
}

# One credential pair: the key is registered before its private half is stored,
# so a partial outcome always names the half that exists. The provider seals
# the value for the repository's public key; the plaintext is never sent.
resource "github_actions_secret" "pages_publish" {
  repository  = github_repository.mono.name
  secret_name = "PAGES_DEPLOY_KEY"
  value       = tls_private_key.pages_publish.private_key_openssh

  depends_on = [github_repository_deploy_key.pages_publish]
}

# The bypass is by actor type: the provider says not to set an actor_id for
# DeployKey and GitHub ignores one, so every write-enabled deploy key on the
# pages repository may push while humans cannot force-push or delete.
resource "github_repository_ruleset" "pages_publish" {
  name        = "pages protection"
  repository  = github_repository.pages.name
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

  bypass_actors {
    actor_type  = "DeployKey"
    bypass_mode = "always"
  }
}
