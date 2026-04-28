//! [`SummaryIndex`] — hierarchical summary cache with LRU eviction.
//!
//! Stores `(SummaryRange → SummaryNode)` pairs in a BTreeMap, with
//! append-time invalidation and an LRU eviction policy.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::types::{SummaryNode, SummaryRange, MAX_CACHED_SUMMARIES};

/// Hierarchical summary cache over the chat transcript.
///
/// Cache hit: returns immediately via [`Self::get`].
/// Cache miss: [`Self::summary_for`] calls a producer, then caches.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SummaryIndex {
    #[serde(default)]
    pub(super) tree: BTreeMap<SummaryRange, SummaryNode>,
    #[serde(default)]
    pub(super) generation: u64,
    #[serde(default)]
    pub(super) lru_order: Vec<SummaryRange>,
}

// Construction, accessors, and iteration.
impl SummaryIndex {
    /// Empty index.
    pub fn new() -> Self { Self::default() }

    /// Current generation counter.
    pub fn generation(&self) -> u64 { self.generation }

    /// Number of cached summaries.
    pub fn len(&self) -> usize { self.tree.len() }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool { self.tree.is_empty() }

    /// Exact range lookup.
    pub fn get(&self, range: SummaryRange) -> Option<&SummaryNode> {
        self.tree.get(&range)
    }

    /// Iterate over `(range, node)` pairs in range order.
    pub fn entries(&self) -> impl Iterator<Item = (&SummaryRange, &SummaryNode)> {
        self.tree.iter()
    }
}
