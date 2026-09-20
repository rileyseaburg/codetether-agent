//! Lazily start idle eviction for each manager-owned language-server cache.

use super::client::LspClient;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[path = "client_cache/eviction.rs"]
mod eviction;
#[cfg(test)]
#[path = "client_cache/tests.rs"]
mod tests;

type Entries = HashMap<String, Arc<LspClient>>;

/// Manager-local cache; concurrent creation is serialized by its write guard.
#[derive(Default)]
pub(super) struct ClientCache {
    entries: Arc<RwLock<Entries>>,
    started: OnceLock<()>,
}

impl ClientCache {
    fn start(&self) {
        self.started.get_or_init(|| {
            tokio::spawn(eviction::run(Arc::downgrade(&self.entries)));
        });
    }

    pub(super) async fn read(&self) -> RwLockReadGuard<'_, Entries> {
        self.start();
        self.entries.read().await
    }

    pub(super) async fn write(&self) -> RwLockWriteGuard<'_, Entries> {
        self.start();
        self.entries.write().await
    }
}

impl LspClient {
    pub(super) fn touch(self: &Arc<Self>) -> Arc<Self> {
        *self.last_used.lock().unwrap_or_else(|e| e.into_inner()) = std::time::Instant::now();
        Arc::clone(self)
    }
}
