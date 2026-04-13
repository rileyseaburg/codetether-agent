//! Extra build-agent guidance fragments.
//!
//! This module keeps small, concern-specific prompt fragments out of the main
//! built-in prompt definitions so new guidance does not accumulate inline.
//!
//! # Examples
//!
//! ```ignore
//! assert!(BUILD_GITHUB_AUTH_GUIDANCE.contains("GitHub authentication"));
//! ```

pub(super) const BUILD_GITHUB_AUTH_GUIDANCE: &str = "\n- In repositories prepared by CodeTether, assume GitHub authentication is already provisioned for `git` and `gh`. Do not search the filesystem for tokens or run interactive `gh auth login`.";
