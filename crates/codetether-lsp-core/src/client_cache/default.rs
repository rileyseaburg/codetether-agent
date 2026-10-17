//! Construct an empty cache without starting an eviction task.
use super::*;
impl<C: CacheClient> Default for ClientCache<C> {
    fn default() -> Self {
        Self {
            entries: Arc::default(),
            started: OnceLock::default(),
        }
    }
}
