//! Request rendering for `gh auth git-credential`.
//!
//! GitHub CLI expects a `git credential`-style payload on stdin. This module
//! keeps that formatting isolated from process spawning.
//!
//! # Examples
//!
//! ```ignore
//! let request = render_gh_credential_query(&query, &creds);
//! assert!(request.contains("protocol=https"));
//! ```

use super::{GitCredentialMaterial, GitCredentialQuery};

/// Renders the stdin payload expected by `gh auth git-credential get`.
///
/// Host and path fall back to credential material when the original query did
/// not provide them.
///
/// # Examples
///
/// ```ignore
/// let request = render_gh_credential_query(&query, &creds);
/// assert!(request.ends_with("\\n\\n"));
/// ```
pub(super) fn render_gh_credential_query(
    query: &GitCredentialQuery,
    credentials: &GitCredentialMaterial,
) -> String {
    let mut request = format!(
        "protocol={}\n",
        query.protocol.as_deref().unwrap_or("https")
    );
    if let Some(host) = query.host.as_deref().or(credentials.host.as_deref()) {
        request.push_str(&format!("host={}\n", host.trim()));
    }
    if let Some(path) = query.path.as_deref().or(credentials.path.as_deref()) {
        request.push_str(&format!("path={}\n", path.trim()));
    }
    request.push('\n');
    request
}
