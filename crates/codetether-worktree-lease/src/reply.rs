//! Results returned by the mux lease authority.

use super::WorktreeLease;
use serde::{Deserialize, Serialize};

/// Outcome of one coordination request.
/// # Examples
/// ```
/// let reply = codetether_worktree_lease::CoordinationReply::Released { count: 0 };
/// assert!(matches!(reply, codetether_worktree_lease::CoordinationReply::Released { count: 0 }));
/// ```
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum CoordinationReply {
    /// Workspace ownership is forbidden; no paths from this request were leased.
    WorkspaceScopeForbidden,
    /// All requested paths were acquired atomically.
    Acquired {
        /// Granted leases.
        leases: Vec<WorktreeLease>,
        /// Total time spent waiting.
        waited_ms: u64,
    },
    /// Another owner holds overlapping paths.
    Blocked {
        /// Conflicting leases.
        conflicts: Vec<WorktreeLease>,
        /// Total time spent waiting.
        waited_ms: u64,
        /// Earliest remaining conflict expiry delay.
        retry_after_ms: u64,
    },
    /// Matching leases had their expiry extended.
    Renewed {
        /// Number renewed.
        count: usize,
    },
    /// Matching leases were removed.
    Released {
        /// Number released.
        count: usize,
    },
    /// Current unexpired lease table.
    Snapshot {
        /// Active leases.
        leases: Vec<WorktreeLease>,
    },
}
