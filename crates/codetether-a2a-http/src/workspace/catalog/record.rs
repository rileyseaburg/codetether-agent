//! Workspace identity and local checkout path.

/// A resolved workspace linking a server-side ID to a local path.
///
/// # Examples
/// ```rust
/// use codetether_a2a_http::workspace::ResolvedWorkspace;
/// let workspace = ResolvedWorkspace { id: "one".into(), path: "/workspace/one".into() };
/// assert_eq!(workspace.id, "one");
/// ```
#[derive(Debug, Clone)]
pub struct ResolvedWorkspace {
    /// Server-side workspace ID.
    pub id: String,
    /// Local filesystem path.
    pub path: String,
}
