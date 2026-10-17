//! Shared forage regression coverage.
use super::*;

#[tokio::test]
async fn seed_default_okr_populates_empty_repo() {
    let dir = tempdir().expect("create tempdir");
    let repo = codetether_okr::OkrRepository::new(dir.path().to_path_buf());

    seed_default_okr_if_empty(&repo)
        .await
        .expect("seed should succeed");

    let okrs = repo.list_okrs().await.expect("list okrs");
    assert_eq!(okrs.len(), 1);
    assert_eq!(
        okrs[0].title,
        "Mission: Autonomous Business-Aligned Execution"
    );
    assert_eq!(okrs[0].status, OkrStatus::Active);
    assert_eq!(okrs[0].key_results.len(), 3);
}
