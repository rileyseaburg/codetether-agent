//! Query types for Git credential helper requests.
//!
//! Git invokes the helper with protocol, host, and path fields. This module
//! keeps the decoded request shape small and reusable.
//!
//! # Examples
//!
//! ```ignore
//! let query = GitCredentialQuery::default();
//! assert!(query.host.is_none());
//! ```

/// Parsed `git credential` request fields from stdin.
///
/// Missing fields are represented as `None` so callers can layer in defaults
/// from credential material returned by the control plane.
///
/// # Examples
///
/// ```ignore
/// let query = GitCredentialQuery::default();
/// assert!(query.protocol.is_none());
/// ```
#[derive(Debug, Default)]
pub struct GitCredentialQuery {
    pub protocol: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
}
