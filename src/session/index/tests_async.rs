//! Additional summary index tests — serde, range, LRU, and async.

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
    idx.append(2);
    let json = serde_json::to_string(&idx).unwrap();
    let back: SummaryIndex = serde_json::from_str(&json).unwrap();
    assert_eq!(back.generation(), 1);
    assert!(back.is_empty());
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
    for i in 0..=MAX_CACHED_SUMMARIES {
        let r = SummaryRange::new(i * 4, i * 4 + 4).unwrap();
        idx.insert(r, node(&format!("entry-{i}")));
    }
    assert_eq!(idx.len(), MAX_CACHED_SUMMARIES);
    assert!(idx.get(SummaryRange::new(0, 4).unwrap()).is_none());
    assert!(idx.get(SummaryRange::new(4, 8).unwrap()).is_some());
}
