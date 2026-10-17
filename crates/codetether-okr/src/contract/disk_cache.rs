//! disk_cache regression.
use super::*;

#[tokio::test]
async fn disk_cache_and_corrupt_entry_contract() {
    let dir = tempfile::tempdir().unwrap();
    let repo = OkrRepository::new(dir.path().into());
    let okr = repo.create_okr(objective()).await.unwrap();
    let path = dir
        .path()
        .join("objectives")
        .join(format!("{}.json", okr.id));
    assert!(path.is_file());
    assert!(!path.with_extension("json.tmp").exists());
    let mut disk: Okr = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    disk.title = "Changed on disk".into();
    std::fs::write(&path, serde_json::to_vec(&disk).unwrap()).unwrap();
    assert_eq!(
        repo.get_okr(okr.id).await.unwrap().unwrap().title,
        "Objective"
    );
    repo.clear_cache().await;
    assert_eq!(
        repo.get_okr(okr.id).await.unwrap().unwrap().title,
        "Changed on disk"
    );
    repo.clear_cache().await;
    let broken = Uuid::new_v4();
    std::fs::write(
        dir.path().join("objectives").join(format!("{broken}.json")),
        "broken",
    )
    .unwrap();
    assert_eq!(repo.list_okrs().await.unwrap().len(), 1);
    assert!(repo.get_okr(broken).await.is_err());
    assert!(repo.delete_okr(okr.id).await.unwrap());
    assert!(!repo.delete_okr(okr.id).await.unwrap());
}
