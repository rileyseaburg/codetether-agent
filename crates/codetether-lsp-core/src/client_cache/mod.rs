//! Lazily start idle eviction for each manager-owned language-server cache.

pub use contract::CacheClient;
mod contract;
mod default;
mod touch;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use touch::touch;

mod eviction;
#[cfg(test)]
mod tests;

type Entries<C> = HashMap<String, Arc<C>>;

/// Manager-local cache; concurrent creation is serialized by its write guard.
///
/// Reads and writes lazily start a single idle-eviction task. The cache retains
/// the caller's client type and its Arc ownership without adding another wrapper.
///
/// # Examples
/// ```
/// let cache = codetether_lsp_core::client_cache::ClientCache::<codetether_lsp_core::LspClient>::default();
/// ```
pub struct ClientCache<C: CacheClient> {
    entries: Arc<RwLock<Entries<C>>>,
    started: OnceLock<()>,
}

impl<C: CacheClient> ClientCache<C> {
    fn start(&self) {
        self.started.get_or_init(|| {
            tokio::spawn(eviction::run(Arc::downgrade(&self.entries)));
        });
    }

    /// Start eviction if needed and borrow cached clients for reading.
    pub async fn read(&self) -> RwLockReadGuard<'_, Entries<C>> {
        self.start();
        self.entries.read().await
    }

    /// Start eviction if needed and exclusively borrow cached clients.
    pub async fn write(&self) -> RwLockWriteGuard<'_, Entries<C>> {
        self.start();
        self.entries.write().await
    }
}
