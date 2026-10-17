//! Shared forage regression coverage.
use super::*;

#[tokio::test]
async fn seed_default_okr_is_noop_when_repo_not_empty() {
    let dir = tempdir().expect("create tempdir");
    let repo = codetether_okr::OkrRepository::new(dir.path().to_path_buf());

    let mut existing = Okr::new("Existing Objective", "Do not overwrite");
    existing.status = OkrStatus::Active;
    existing.add_key_result(KeyResult::new(existing.id, "KR1", 100.0, "%"));
    let _ = repo.create_okr(existing).await.expect("create existing");

    seed_default_okr_if_empty(&repo)
        .await
        .expect("seed should succeed");

    let okrs = repo.list_okrs().await.expect("list okrs");
    assert_eq!(okrs.len(), 1);
    assert_eq!(okrs[0].title, "Existing Objective");
}
