//! Repository status totals.
use super::*;
#[tokio::test]
async fn status_totals() {
    let (_dir, repo, _first, _run) = super::query_fixture::repository().await;
    let stats = repo.stats().await.unwrap();
    assert_eq!(stats.total_okrs, 2);
    assert_eq!(stats.total_runs, 1);
    assert_eq!(stats.okr_status_counts[&OkrStatus::Active], 1);
    assert_eq!(stats.run_status_counts[&OkrRunStatus::Draft], 1);
}
