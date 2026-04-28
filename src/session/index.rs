//! Hierarchical summary index — Phase B step 14 scaffolding.
//!
//! The Liu et al. paper (arXiv:2512.22087) calls for an incremental
//! per-(range → summary) cache so the prompt loop can re-derive context
//! in `O(log H)` rather than `O(H)` each turn. This module is the data
//! structure plus the append-time invalidation hook; the producer side
//! (hierarchical RLM summarisation, LRU eviction) lands in step 18.
//!
//! ## Scope in step 14
//!
//! * [`SummaryIndex`] data structure with append-only generation
//!   counter and intersect-invalidation.
//! * Sync [`SummaryIndex::get`] cache lookup; no provider call.
//! * Wired into [`Session::add_message`](crate::session::Session::add_message)
//!   on the hot path, so future commits can populate without changing
//!   call sites.
//! * Persisted inline on the [`Session`] JSON behind
//!   `#[serde(default)]` so legacy sessions deserialise unchanged.
//!
//! Behaviour today is identical to before this commit — no derivation
//! policy reads from the index yet.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Half-open `[start, end)` range over [`Session::messages`] indices.
///
/// Persisted as part of the index so a reload preserves the cache
/// without re-running the producer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SummaryRange {
    /// Inclusive lower bound (message index in the session transcript).
    pub start: usize,
    /// Exclusive upper bound.
    pub end: usize,
}

impl SummaryRange {
    /// Construct a range. Returns `None` if `start >= end`.
    pub fn new(start: usize, end: usize) -> Option<Self> {
        (start < end).then_some(Self { start, end })
    }

    /// Whether this range covers message index `idx`.
    pub fn contains(&self, idx: usize) -> bool {
        self.start <= idx && idx < self.end
    }
}

/// Granularity at which a [`SummaryNode`] was produced.
///
/// Phase B step 18 will use this to choose between turn-level RLM
/// summaries and coarser phase-/session-level rollups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Granularity {
    Turn,
    Phase,
    Session,
}

/// One cached summary entry — produced once, consumed many times until
/// the underlying range is invalidated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryNode {
    /// The summary text.
    pub content: String,
    /// Target token budget the producer was asked to fit.
    pub target_tokens: usize,
    /// Granularity tag — see [`Granularity`].
    pub granularity: Granularity,
    /// Generation at which this node was produced. Compare against the
    /// index's current `generation` to detect staleness across restarts.
    pub generation: u64,
}

/// Upper bound on cached summaries before LRU eviction kicks in.
/// Chosen so the index sidecar stays under ~1 MB for typical sessions.
pub const MAX_CACHED_SUMMARIES: usize = 128;

/// Hierarchical summary cache over the chat transcript.
///
/// [`Self::get`] returns cached summaries for exact range matches.
/// [`Self::summary_for`] is async and produces new summaries via
/// a caller-supplied producer when the cache misses, then stores
/// the result with LRU eviction.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SummaryIndex {
    #[serde(default)]
    tree: BTreeMap<SummaryRange, SummaryNode>,
    /// Monotonic counter bumped whenever the index is invalidated.
    #[serde(default)]
    generation: u64,
    /// LRU ordering: oldest entry at position 0, newest at the end.
    #[serde(default)]
    lru_order: Vec<SummaryRange>,
}

impl SummaryIndex {
    /// Construct an empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Current generation counter.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Number of cached summaries currently in the tree.
    pub fn len(&self) -> usize {
        self.tree.len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    /// Look up a cached summary by exact range match (immutable).
    pub fn get(&self, range: SummaryRange) -> Option<&SummaryNode> {
        self.tree.get(&range)
    }

    /// Insert a new summary node. Maintains LRU ordering.
    pub fn insert(&mut self, range: SummaryRange, node: SummaryNode) -> Option<SummaryNode> {
        self.touch_lru(range);
        let prev = self.tree.insert(range, node);
        self.evict_if_over_budget();
        prev
    }

    /// Hot-path hook for [`Session::add_message`].
    ///
    /// Drops every cached summary whose range covers `idx`; bumps the
    /// generation counter whenever any drop occurs.
    pub fn append(&mut self, idx: usize) {
        let to_drop: Vec<SummaryRange> = self
            .tree
            .keys()
            .filter(|range| range.contains(idx))
            .copied()
            .collect();
        for range in &to_drop {
            self.tree.remove(range);
        }
        self.lru_order.retain(|r| !to_drop.contains(r));
        if !to_drop.is_empty() {
            self.generation = self.generation.wrapping_add(1);
        }
    }

    /// Drop every range whose `end > idx`. Used when the tail is
    /// rewritten (compaction, reset).
    pub fn invalidate_after(&mut self, idx: usize) {
        let to_drop: Vec<SummaryRange> = self
            .tree
            .keys()
            .filter(|range| range.end > idx)
            .copied()
            .collect();
        for range in &to_drop {
            self.tree.remove(range);
        }
        self.lru_order.retain(|r| !to_drop.contains(r));
        if !to_drop.is_empty() {
            self.generation = self.generation.wrapping_add(1);
        }
    }

    /// Async summary-for — the main read path for derivation policies.
    ///
    /// Cache hit: returns immediately. Cache miss: calls `producer`
    /// which encapsulates the RLM summarisation, then caches the
    /// result under the requested range with LRU eviction.
    pub async fn summary_for<F, Fut>(
        &mut self,
        range: SummaryRange,
        producer: F,
    ) -> anyhow::Result<SummaryNode>
    where
        F: FnOnce(SummaryRange) -> Fut,
        Fut: std::future::Future<Output = anyhow::Result<SummaryNode>>,
    {
        if let Some(node) = self.tree.get(&range).cloned() {
            self.touch_lru(range);
            return Ok(node);
        }
        let node = producer(range).await?;
        self.insert(range, node.clone());
        Ok(node)
    }

    /// Move `range` to the end of the LRU order (most recently used).
    fn touch_lru(&mut self, range: SummaryRange) {
        self.lru_order.retain(|r| *r != range);
        self.lru_order.push(range);
    }

    /// Evict the oldest entries if the cache exceeds the budget.
    fn evict_if_over_budget(&mut self) {
        while self.tree.len() > MAX_CACHED_SUMMARIES {
            if let Some(oldest) = self.lru_order.first().copied() {
                self.tree.remove(&oldest);
                self.lru_order.remove(0);
            } else {
                break;
            }
        }
    }

    /// Iterate over every cached `(range, node)` in range order.
    pub fn entries(&self) -> impl Iterator<Item = (&SummaryRange, &SummaryNode)> {
        self.tree.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(text: &str) -> SummaryNode {
        SummaryNode {
            content: text.to_string(),
            target_tokens: 512,
            granularity: Granularity::Turn,
            generation: 0,
        }
    }

    #[test]
    fn empty_index_returns_none_on_get() {
        let mut idx = SummaryIndex::new();
        assert!(idx.is_empty());
        assert!(idx.get(SummaryRange::new(0, 4).unwrap()).is_none());
    }

    #[test]
    fn append_invalidates_only_intersecting_ranges() {
        let mut idx = SummaryIndex::new();
        idx.insert(SummaryRange::new(0, 4).unwrap(), node("first"));
        idx.insert(SummaryRange::new(4, 8).unwrap(), node("second"));
        idx.insert(SummaryRange::new(8, 12).unwrap(), node("third"));
        assert_eq!(idx.len(), 3);

        idx.append(5);

        assert_eq!(idx.len(), 2);
        assert!(idx.get(SummaryRange::new(4, 8).unwrap()).is_none());
        assert!(idx.get(SummaryRange::new(0, 4).unwrap()).is_some());
        assert!(idx.get(SummaryRange::new(8, 12).unwrap()).is_some());
        assert_eq!(idx.generation(), 1);
    }

    #[test]
    fn append_outside_any_range_is_a_noop() {
        let mut idx = SummaryIndex::new();
        idx.insert(SummaryRange::new(0, 4).unwrap(), node("first"));
        idx.append(99);
        assert_eq!(idx.len(), 1);
        assert_eq!(idx.generation(), 0);
    }

    #[test]
    fn invalidate_after_drops_tail_ranges() {
        let mut idx = SummaryIndex::new();
        idx.insert(SummaryRange::new(0, 4).unwrap(), node("first"));
        idx.insert(SummaryRange::new(4, 8).unwrap(), node("second"));
        idx.insert(SummaryRange::new(8, 12).unwrap(), node("third"));

        idx.invalidate_after(6);

        // Range [4,8) ends at 8 > 6, so it's dropped along with [8,12).
        assert_eq!(idx.len(), 1);
        assert!(idx.get(SummaryRange::new(0, 4).unwrap()).is_some());
        assert_eq!(idx.generation(), 1);
    }

    #[test]
    fn range_contains_is_half_open() {
        let r = SummaryRange::new(2, 5).unwrap();
        assert!(!r.contains(1));
        assert!(r.contains(2));
        assert!(r.contains(4));
        assert!(!r.contains(5));
    }

    #[test]
    fn round_trip_through_serde() {
        let mut idx = SummaryIndex::new();
        idx.insert(SummaryRange::new(0, 4).unwrap(), node("hello"));
        idx.append(2); // bump generation

        let json = serde_json::to_string(&idx).unwrap();
        let back: SummaryIndex = serde_json::from_str(&json).unwrap();
        assert_eq!(back.generation(), 1);
        assert!(back.is_empty()); // append() dropped the entry
    }

    #[test]
    fn empty_range_rejected() {
        assert_eq!(SummaryRange::new(5, 5), None);
        assert_eq!(SummaryRange::new(7, 5), None);
        assert!(SummaryRange::new(5, 6).is_some());
    }

    #[test]
    fn lru_eviction_removes_oldest() {
        let mut idx = SummaryIndex::new();
        // Insert MAX+1 entries; the first should be evicted.
        for i in 0..=MAX_CACHED_SUMMARIES {
            let r = SummaryRange::new(i * 4, i * 4 + 4).unwrap();
            idx.insert(r, node(&format!("entry-{i}")));
        }
        assert_eq!(idx.len(), MAX_CACHED_SUMMARIES);
        // Range [0,4) was inserted first and should be evicted.
        assert!(idx.get(SummaryRange::new(0, 4).unwrap()).is_none());
        // Range [4,8) (second insert) should still be present.
        assert!(idx.get(SummaryRange::new(4, 8).unwrap()).is_some());
    }

    #[tokio::test]
    async fn summary_for_cache_hit_skips_producer() {
        let mut idx = SummaryIndex::new();
        let range = SummaryRange::new(0, 4).unwrap();
        idx.insert(range, node("cached"));

        let result = idx
            .summary_for(range, |_r| async {
                panic!("producer should not be called on cache hit");
            })
            .await
            .unwrap();

        assert_eq!(result.content, "cached");
    }

    #[tokio::test]
    async fn summary_for_cache_miss_calls_producer() {
        let mut idx = SummaryIndex::new();
        let range = SummaryRange::new(0, 4).unwrap();

        let result = idx
            .summary_for(range, |r| async move {
                Ok(SummaryNode {
                    content: format!("produced-for-{}-{}", r.start, r.end),
                    target_tokens: 256,
                    granularity: Granularity::Turn,
                    generation: 0,
                })
            })
            .await
            .unwrap();

        assert_eq!(result.content, "produced-for-0-4");
        // Now cached
        assert_eq!(idx.len(), 1);
    }
}
