//! disk_updates regression.
use super::*;

#[tokio::test]
async fn update_and_run_validation_contract() {
    let dir = tempfile::tempdir().unwrap();
    let repo = OkrRepository::new(dir.path().into());
    assert!(
        repo.create_okr(Okr::new("invalid", "no key results"))
            .await
            .is_err()
    );
    assert!(
        repo.create_run(OkrRun::new(Uuid::new_v4(), " "))
            .await
            .is_err()
    );
    let okr = repo.update_okr(objective()).await.unwrap();
    assert!(repo.get_okr(okr.id).await.unwrap().is_some());
    let mut run = OkrRun::new(okr.id, "first");
    run.name = String::new();
    let run = repo.update_run(run).await.unwrap();
    assert!(repo.get_run(run.id).await.unwrap().unwrap().name.is_empty());
    repo.clear_cache().await;
    assert_eq!(repo.list_runs().await.unwrap().len(), 1);
    assert!(repo.delete_run(run.id).await.unwrap());
    assert!(!repo.delete_run(run.id).await.unwrap());
}
