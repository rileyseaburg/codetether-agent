//! Async tests for [`SummaryIndex::summary_for`].

use super::cache::SummaryIndex;
use super::types::{Granularity, SummaryNode, SummaryRange};

fn node(text: &str) -> SummaryNode {
    SummaryNode {
        content: text.to_string(),
        target_tokens: 512,
        granularity: Granularity::Turn,
        generation: 0,
    }
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
    assert_eq!(idx.len(), 1);
}
