//! Application cache adapter preserving the swarm API and data-directory policy.
//!
//! [SwarmCache] delegates storage to the shared crate; [CacheConfig], [CacheEntry]
//! and [CacheStats] retain their public application paths.
use super::{SubTask, SubTaskResult};
use anyhow::Result;
pub use codetether_swarm_core::CacheStats;
pub use codetether_swarm_core::cache::{CacheConfig, CacheEntry};
use std::path::PathBuf;
mod access;
mod construct;
mod operations;
/// Swarm result cache using the application's default data directory.
///
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let cache = codetether_agent::swarm::cache::SwarmCache::new(Default::default()).await?;
/// assert!(cache.stats().hit_rate() >= 0.0);
/// # Ok(())
/// # }
/// ```
pub struct SwarmCache {
    inner: codetether_swarm_core::cache::SwarmCache,
}
