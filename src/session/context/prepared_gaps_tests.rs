//! Tests for prepared-only incremental summary selection.

use super::select;
use crate::session::index::{Granularity, SummaryIndex, SummaryNode, SummaryRange};

#[test]
fn cache_miss_does_not_generate_request_time_summary() {
    assert!(select(&SummaryIndex::new(), &[(0, 16)]).is_empty());
}

#[test]
fn selects_earliest_largest_prepared_blocks() {
    let mut index = SummaryIndex::new();
    insert(&mut index, 0, 8, "first");
    insert(&mut index, 8, 16, "second");
    let gaps = select(&index, &[(0, 16)]);
    assert_eq!(gaps.len(), 2);
    assert_eq!(gaps[0].content, "first");
    assert_eq!(gaps[1].content, "second");
}

fn insert(index: &mut SummaryIndex, start: usize, end: usize, content: &str) {
    let range = SummaryRange::new(start, end).unwrap();
    index.insert(
        range,
        SummaryNode {
            content: content.into(),
            target_tokens: 512,
            granularity: Granularity::Phase,
            generation: 1,
        },
    );
}
