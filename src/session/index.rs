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

/// Hierarchical summary cache over the chat transcript.
///
/// Step 14 is **scaffolding only** — no producer wired yet, no eviction.
/// [`Self::get`] always returns `None` until step 18 lands.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SummaryIndex {
    #[serde(default)]
    tree: BTreeMap<SummaryRange, SummaryNode>,
    /// Monotonic counter bumped whenever the index is invalidated.
    /// Producer compares against `SummaryNode::generation` to detect
    /// stale entries.
    #[serde(default)]
    generation: u64,
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

    /// Look up a cached summary by exact range match.
    ///
    /// Phase B step 18 will replace this with a fuzzier lookup that
    /// can re-cover overlapping ranges from existing nodes.
    pub fn get(&self, range: SummaryRange) -> Option<&SummaryNode> {
        self.tree.get(&range)
    }

    /// Insert a new summary node. Caller is responsible for bumping
    /// generation if the new node supersedes an invalidated one.
    ///
    /// Returns the previous node at the same range, if any.
    pub fn insert(&mut self, range: SummaryRange, node: SummaryNode) -> Option<SummaryNode> {
        self.tree.insert(range, node)
    }

    /// Hot-path hook for [`Session::add_message`].
    ///
    /// Drops every cached summary whose range covers `idx`; bumps the
    /// generation counter whenever any drop occurs so consumers can
    /// detect cache turnover. Untouched ranges keep their nodes — that
    /// is the whole point of this structure.
    pub fn append(&mut self, idx: usize) {
        let before = self.tree.len();
        self.tree.retain(|range, _| !range.contains(idx));
        if self.tree.len() != before {
            self.generation = self.generation.wrapping_add(1);
        }
    }

    /// Drop every range whose `end > idx`. Used when the tail of the
    /// transcript is rewritten (compaction, reset).
    pub fn invalidate_after(&mut self, idx: usize) {
        let before = self.tree.len();
        self.tree.retain(|range, _| range.end <= idx);
        if self.tree.len() != before {
            self.generation = self.generation.wrapping_add(1);
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
        let idx = SummaryIndex::new();
        assert!(idx.is_empty());
        assert_eq!(idx.get(SummaryRange::new(0, 4).unwrap()), None);
    }

    #[test]
    fn append_invalidates_only_intersecting_ranges() {
        let mut idx = SummaryIndex::new();
        idx.insert(SummaryRange::new(0, 4).unwrap(), node("first"));
        idx.insert(SummaryRange::new(4, 8).unwrap(), node("second"));
        idx.insert(SummaryRange::new(8, 12).unwrap(), node("third"));
        assert_eq!(idx.len(), 3);

        idx.append(5);

        // The middle range is dropped; the disjoint ones survive.
        assert_eq!(idx.len(), 2);
        assert!(idx.get(SummaryRange::new(0, 4).unwrap()).is_some());
        assert!(idx.get(SummaryRange::new(4, 8).unwrap()).is_none());
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
}
