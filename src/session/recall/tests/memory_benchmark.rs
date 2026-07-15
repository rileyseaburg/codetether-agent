//! Manual real-workspace benchmark for retained recall-query memory.

use std::time::Instant;

use crate::telemetry::memory::MemorySnapshot;

#[tokio::test]
#[ignore = "manual benchmark requiring CODETETHER_DATA_DIR and CODETETHER_BENCH_WORKSPACE"]
async fn workspace_recall_memory_benchmark() {
    let workspace = std::env::var("CODETETHER_BENCH_WORKSPACE")
        .expect("set CODETETHER_BENCH_WORKSPACE to an indexed workspace");
    let before = MemorySnapshot::capture().rss_kb.unwrap_or_default();
    let started = Instant::now();
    let hits = super::super::search::run(
        std::path::Path::new(&workspace),
        "deployment policy",
        super::super::search::SearchOptions {
            session_id: None,
            excluded_session: None,
            limit: 10,
            minimum_score: 0.1,
        },
    )
    .await
    .unwrap();
    let after = MemorySnapshot::capture().rss_kb.unwrap_or_default();
    println!(
        "BENCH recall_workspace hits={} elapsed_ms={} retained_rss_delta_kib={}",
        hits.len(),
        started.elapsed().as_millis(),
        after.saturating_sub(before)
    );
    assert!(
        !hits.is_empty(),
        "benchmark workspace produced no recall hits"
    );
    assert!(hits.len() <= 10);
}
