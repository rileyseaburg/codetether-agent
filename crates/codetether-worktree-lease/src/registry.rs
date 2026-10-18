//! Concurrent lease table owned exclusively by a mux server.

use super::{WorktreeLease, key::LeaseKey};
use std::collections::HashMap;
use std::sync::Mutex;

/// Thread-safe, process-local authority for exclusive worktree path ownership.
/// # Examples
/// ```
/// let registry = codetether_worktree_lease::LeaseRegistry::new();
/// assert!(matches!(registry.snapshot(), codetether_worktree_lease::CoordinationReply::Snapshot { .. }));
/// ```
pub struct LeaseRegistry {
    pub(super) entries: Mutex<HashMap<LeaseKey, WorktreeLease>>,
    pub(super) changes: tokio::sync::watch::Sender<u64>,
}

impl LeaseRegistry {
    /// Create an empty lease table with its change-notification channel.
    pub fn new() -> Self {
        let (changes, _) = tokio::sync::watch::channel(0);
        Self {
            entries: Mutex::new(HashMap::new()),
            changes,
        }
    }
}
