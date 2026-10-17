//! Release idle clients only when the cache is their sole owner.

use super::{CacheClient, Entries};
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

const IDLE_LIMIT: Duration = Duration::from_secs(120);

pub(super) fn expired(owners: usize, idle: Duration) -> bool {
    owners == 1 && idle >= IDLE_LIMIT
}

pub(super) async fn run<C: CacheClient>(cache: Weak<RwLock<Entries<C>>>) {
    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
        let Some(cache) = cache.upgrade() else {
            return;
        };
        evict(&mut *cache.write().await);
    }
}

pub(super) fn evict<C: CacheClient>(entries: &mut Entries<C>) {
    entries.retain(|name, client| {
        let mut last = client.last_used().lock().unwrap_or_else(|e| e.into_inner());
        let owners = Arc::strong_count(client);
        if owners > 1 {
            *last = Instant::now();
        }
        let remove = expired(owners, last.elapsed());
        if remove {
            tracing::info!(target: "codetether_agent::lsp::client_cache::eviction", server = %name, "Releasing idle language server");
        }
        !remove
    });
}
