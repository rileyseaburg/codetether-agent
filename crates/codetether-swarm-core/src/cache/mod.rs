//! Disk-backed task-result caching with host-provided default directories.
//!
//! Construct [SwarmCache] from [CacheConfig], then use get/put to reuse successful
//! [SubTaskResult] values. [CacheEntry] stores expiry and content metadata.
use crate::{CacheStats, SubTask, SubTaskResult};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    time::{Duration, SystemTime},
};
use tokio::fs;
mod entry;
pub use entry::CacheEntry;
mod config;
pub use config::CacheConfig;
mod access;
mod contract;
pub use contract::CacheAccess;
mod clear;
mod construct;
mod disk;
mod eviction;
mod insert;
mod key;
mod load;
mod lookup;
#[cfg(test)]
mod tests;

/// Swarm result cache using content-based hashing
///
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let cache = codetether_swarm_core::cache::SwarmCache::new(
///     Default::default(), || std::path::PathBuf::from("cache"),
/// ).await?;
/// assert_eq!(cache.cache_dir(), std::path::Path::new("cache"));
/// # Ok(())
/// # }
/// ```
pub struct SwarmCache {
    config: CacheConfig,
    cache_dir: PathBuf,
    stats: CacheStats,
    /// In-memory index of cache entries
    index: HashMap<String, CacheEntry>,
}

mod tasks;
pub use tasks::{read_only_tasks, retarget};
