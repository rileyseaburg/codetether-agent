//! Repository test_list_and_stats regression.

use super::*;

#[tokio::test]
async fn test_list_and_stats() {
    let repo = temp_repo();

    // Create OKRs
    let mut okr = Okr::new("Test", "Desc");
    let kr = KeyResult::new(okr.id, "KR", 100.0, "%");
    okr.add_key_result(kr);
    repo.create_okr(okr).await.unwrap();

    // List
    let all_okrs = repo.list_okrs().await.unwrap();
    assert_eq!(all_okrs.len(), 1);

    let all_runs = repo.list_runs().await.unwrap();
    assert_eq!(all_runs.len(), 0);

    // Stats
    let stats = repo.stats().await.unwrap();
    assert_eq!(stats.total_okrs, 1);
}
