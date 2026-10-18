//! Bind application-directory cache storage to executor management.
use crate::{CacheStats, SwarmCache};
impl codetether_swarm_core::cache::CacheAccess for SwarmCache {
    fn stats(&self) -> &CacheStats {
        SwarmCache::stats(self)
    }
    async fn clear(&mut self) -> anyhow::Result<()> {
        SwarmCache::clear(self).await
    }
}
