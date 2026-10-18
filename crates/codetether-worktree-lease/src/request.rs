//! Authenticated requests accepted by the mux lease authority.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// One shared-worktree coordination operation.
/// # Examples
/// ```
/// let request = codetether_worktree_lease::CoordinationRequest::Snapshot;
/// assert!(matches!(request, codetether_worktree_lease::CoordinationRequest::Snapshot));
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum CoordinationRequest {
    /// Atomically acquire bounded paths, optionally waiting for conflicts.
    Acquire {
        /// Owner identity.
        owner: String,
        /// Agent display identity.
        agent: String,
        /// Checkout root.
        workspace: PathBuf,
        /// Requested bounded paths.
        paths: Vec<PathBuf>,
        /// Maximum requested wait, capped by the authority.
        #[serde(default)]
        wait_ms: u64,
    },
    /// Extend every lease held by one owner.
    Renew {
        /// Owner identity.
        owner: String,
    },
    /// Release every lease held by one owner.
    Release {
        /// Owner identity.
        owner: String,
    },
    /// Inspect all unexpired leases.
    Snapshot,
}
