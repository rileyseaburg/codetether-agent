//! Atomic set of owner-scoped names currently being spawned.

use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Clone, Eq, Hash, PartialEq)]
pub(super) struct Key {
    owner: Option<String>,
    name: String,
}

/// Pending identity claims for one process.
#[derive(Default)]
pub(super) struct Registry {
    pending: Mutex<HashSet<Key>>,
}

impl Registry {
    /// Atomically claim an owner/name pair when it is not already pending.
    pub(super) fn claim(&self, name: &str, owner: Option<&str>) -> Option<Key> {
        let key = Key {
            owner: owner.map(str::to_string),
            name: name.into(),
        };
        self.lock().insert(key.clone()).then_some(key)
    }

    /// Release a pending claim after rollback or durable handoff.
    pub(super) fn release(&self, key: &Key) {
        self.lock().remove(key);
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashSet<Key>> {
        self.pending
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}
