//! Concurrent lease table owned exclusively by a mux server.

use super::{WorktreeLease, key::LeaseKey};
use std::collections::HashMap;
use std::sync::Mutex;

pub(in crate::mux) struct LeaseRegistry {
    pub(super) entries: Mutex<HashMap<LeaseKey, WorktreeLease>>,
    pub(super) changes: tokio::sync::watch::Sender<u64>,
}

impl LeaseRegistry {
    pub(in crate::mux) fn new() -> Self {
        let (changes, _) = tokio::sync::watch::channel(0);
        Self {
            entries: Mutex::new(HashMap::new()),
            changes,
        }
    }
}
