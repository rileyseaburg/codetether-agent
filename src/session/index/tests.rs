//! Tests for the summary index.

use super::cache::SummaryIndex;
use super::types::{Granularity, SummaryNode, SummaryRange, MAX_CACHED_SUMMARIES};

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

    assert_eq!(idx.len(), 1);
    assert!(idx.get(SummaryRange::new(0, 4).unwrap()).is_some());
    assert_eq!(idx.generation(), 1);
}
