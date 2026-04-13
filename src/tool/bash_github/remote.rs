//! Remote URL parsing helpers for GitHub auth loading.
//!
//! This module extracts the host and repository path from HTTPS remotes so the
//! credential service can request scoped tokens.
//!
//! # Examples
//!
//! ```ignore
//! let parsed = parse_https_remote("https://github.com/owner/repo");
//! assert_eq!(parsed.unwrap().0, "github.com");
//! ```

/// Parses an HTTPS remote into a host and repository path.
///
/// SSH remotes are ignored because the Git credential endpoint currently issues
/// HTTPS credentials for GitHub App operations.
///
/// # Examples
///
/// ```ignore
/// let parsed = parse_https_remote("https://github.com/owner/repo.git");
/// assert_eq!(parsed.unwrap().0, "github.com");
/// ```
pub(super) fn parse_https_remote(remote_url: &str) -> Option<(String, String)> {
    let trimmed = remote_url.trim().strip_prefix("https://")?;
    let (host, path) = trimmed.split_once('/')?;
    Some((host.trim().to_ascii_lowercase(), path.trim().to_string()))
}
