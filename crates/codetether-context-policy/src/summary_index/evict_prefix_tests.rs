//! Regression coverage for rolling summary-cache coordinates and LRU order.
use super::{Granularity, SummaryIndex, SummaryNode, SummaryRange};

fn insert(index: &mut SummaryIndex, start: usize, end: usize) {
    index.insert(
        SummaryRange::new(start, end).unwrap(),
        SummaryNode {
            content: format!("source {start}..{end}"),
            target_tokens: 64,
            granularity: Granularity::Turn,
            generation: 0,
            provenance: None,
        },
    );
}

#[test]
fn eviction_drops_overlaps_and_rebases_retained_summaries() {
    let mut index = SummaryIndex::new();
    insert(&mut index, 0, 2);
    insert(&mut index, 1, 4);
    insert(&mut index, 2, 5);
    insert(&mut index, 8, 10);
    index.evict_prefix(2);
    assert_eq!(index.len(), 2);
    assert_eq!(
        index.get(SummaryRange::new(0, 3).unwrap()).unwrap().content,
        "source 2..5"
    );
    assert_eq!(
        index.get(SummaryRange::new(6, 8).unwrap()).unwrap().content,
        "source 8..10"
    );
    assert_eq!(index.generation(), 1);
    index.evict_prefix(0);
    assert_eq!(index.generation(), 1);
    index.evict_prefix(1);
    assert_eq!(index.len(), 1);
    assert!(index.get(SummaryRange::new(5, 7).unwrap()).is_some());
    index.evict_prefix(usize::MAX);
    assert!(index.is_empty());
}

#[test]
fn eviction_preserves_lru_order_after_serialization() {
    let mut index = SummaryIndex::new();
    for start in 0..128 {
        insert(&mut index, start, start + 1);
    }
    // Refresh the oldest surviving entry before shifting the window.
    insert(&mut index, 1, 2);
    index.evict_prefix(1);
    let mut index: SummaryIndex =
        serde_json::from_str(&serde_json::to_string(&index).unwrap()).unwrap();
    insert(&mut index, 127, 128);
    insert(&mut index, 128, 129);
    assert!(index.get(SummaryRange::new(0, 1).unwrap()).is_some());
    assert!(index.get(SummaryRange::new(1, 2).unwrap()).is_none());
    assert_eq!(index.len(), 128);
}
