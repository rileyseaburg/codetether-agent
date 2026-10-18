//! Serializable lease records returned to authenticated mux clients.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Exclusive ownership of one path inside a mux-managed workspace.
/// # Examples
/// ```
/// let lease = codetether_worktree_lease::WorktreeLease {
///     owner: "turn-1".into(), agent: "build".into(), workspace: "/repo".into(),
///     path: "src/lib.rs".into(), expires_at_ms: 90_000,
/// };
/// assert_eq!(lease.owner, "turn-1");
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct WorktreeLease {
    /// Turn or process identity owning the lease.
    pub owner: String,
    /// Display identity of the requesting agent.
    pub agent: String,
    /// Repository or checkout root.
    pub workspace: PathBuf,
    /// Bounded path beneath that root.
    pub path: PathBuf,
    /// Expiry as Unix epoch milliseconds.
    pub expires_at_ms: i64,
}
