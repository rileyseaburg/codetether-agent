//! Per-child activation locks with weak-entry cleanup.

use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex, Weak};
use tokio::sync::{Mutex as AsyncMutex, OwnedMutexGuard};

static LOADS: LazyLock<LoadLocks> = LazyLock::new(LoadLocks::default);

#[derive(Default)]
pub(super) struct LoadLocks {
    entries: Mutex<HashMap<String, Weak<AsyncMutex<()>>>>,
}

impl LoadLocks {
    pub(super) async fn acquire(&self, agent_id: &str) -> OwnedMutexGuard<()> {
        self.lock_for(agent_id).lock_owned().await
    }

    fn lock_for(&self, agent_id: &str) -> Arc<AsyncMutex<()>> {
        let mut entries = self
            .entries
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        entries.retain(|_, lock| lock.strong_count() > 0);
        if let Some(lock) = entries.get(agent_id).and_then(Weak::upgrade) {
            return lock;
        }
        let lock = Arc::new(AsyncMutex::new(()));
        entries.insert(agent_id.into(), Arc::downgrade(&lock));
        lock
    }
}

/// Acquire the lifecycle transition guard for one durable child ID.
pub(in crate::tool::agent) async fn acquire(agent_id: &str) -> OwnedMutexGuard<()> {
    LOADS.acquire(agent_id).await
}
