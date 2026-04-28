//! Mutation methods for [`SummaryIndex`].
//!
//! Separated from the struct definition for SRP — these methods
//! modify the cache tree and maintain LRU ordering.

use super::cache::SummaryIndex;
use super::types::{MAX_CACHED_SUMMARIES, SummaryNode, SummaryRange};

impl SummaryIndex {
    /// Insert a node, maintaining LRU ordering and evicting if over budget.
    pub fn insert(&mut self, range: SummaryRange, node: SummaryNode) -> Option<SummaryNode> {
        self.touch_lru(range);
        let prev = self.tree.insert(range, node);
        self.evict_if_over_budget();
        prev
    }

    /// Hot-path hook for `Session::add_message`.
    ///
    /// Drops every cached summary whose range covers `idx`.
    pub fn append(&mut self, idx: usize) {
        let to_drop: Vec<SummaryRange> = self
            .tree
            .keys()
            .filter(|r| r.contains(idx))
            .copied()
            .collect();
        if !to_drop.is_empty() {
            for r in &to_drop {
                self.tree.remove(r);
            }
            self.lru_order.retain(|r| !to_drop.contains(r));
            self.generation = self.generation.wrapping_add(1);
        }
    }

    /// Drop ranges whose `end > idx` (compaction/reset).
    pub fn invalidate_after(&mut self, idx: usize) {
        let to_drop: Vec<SummaryRange> =
            self.tree.keys().filter(|r| r.end > idx).copied().collect();
        if !to_drop.is_empty() {
            for r in &to_drop {
                self.tree.remove(r);
            }
            self.lru_order.retain(|r| !to_drop.contains(r));
            self.generation = self.generation.wrapping_add(1);
        }
    }

    /// Move `range` to most-recently-used position.
    pub(super) fn touch_lru(&mut self, range: SummaryRange) {
        self.lru_order.retain(|r| *r != range);
        self.lru_order.push(range);
    }

    /// Evict oldest entries exceeding [`MAX_CACHED_SUMMARIES`].
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
}
