// SPDX-License-Identifier: MPL-2.0
//! Step definitions for the capability `publish-site-plumbing`.
//!
//! The non-e2e scenarios drive the real `tofu` binary against the local fake
//! GitHub API. The `@e2e` scenarios live in `steps/e2e/` because they commit
//! the scrubbed transcript.
