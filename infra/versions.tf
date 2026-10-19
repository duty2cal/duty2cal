# SPDX-License-Identifier: MPL-2.0
# SPDX-FileCopyrightText: 2026 Daniel Kressner
#
# The declaration's tool contract and the encrypted applied state (design.md
# "The encryption block and variables", ADR-0006). `enforced = true` makes
# OpenTofu refuse to write unencrypted state or a plan file, which is what
# makes committing the state acceptable.

terraform {
  required_version = ">= 1.7"

  required_providers {
    github = {
      source  = "integrations/github"
      version = "~> 6.0"
    }
    tls = {
      source  = "hashicorp/tls"
      version = "~> 4.0"
    }
  }

  encryption {
    key_provider "pbkdf2" "passphrase" {
      passphrase = var.state_passphrase
    }

    method "aes_gcm" "state" {
      keys = key_provider.pbkdf2.passphrase
    }

    state {
      method   = method.aes_gcm.state
      enforced = true
    }

    plan {
      method   = method.aes_gcm.state
      enforced = true
    }
  }
}

variable "state_passphrase" {
  type        = string
  description = "Passphrase for the applied state; supplied through TF_VAR_state_passphrase from the maintainer's password manager."
  sensitive   = true
}

variable "billing_email" {
  type        = string
  description = "The organization's billing address; supplied through TF_VAR_billing_email."
  sensitive   = true
}
