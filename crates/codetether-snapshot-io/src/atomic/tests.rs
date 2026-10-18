//! Snapshot replacement remains whole under competing writers and errors.

use super::atomic_write;

#[tokio::test]
async fn concurrent_snapshots_never_share_a_temporary_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("session.json");
    let mut writes = tokio::task::JoinSet::new();
    for value in 0..16u8 {
        let path = path.clone();
        writes.spawn(async move { atomic_write(&path, vec![value; 65536]).await });
    }
    while let Some(result) = writes.join_next().await {
        result.unwrap().unwrap();
    }
    let saved = std::fs::read(&path).unwrap();
    assert_eq!(saved.len(), 65536);
    assert!(saved.iter().all(|byte| *byte == saved[0]));
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}

#[tokio::test]
async fn failed_replace_preserves_existing_destination() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("session.json");
    std::fs::create_dir(&path).unwrap();
    let marker = path.join("original");
    std::fs::write(&marker, b"keep").unwrap();
    assert!(atomic_write(&path, b"replacement".to_vec()).await.is_err());
    assert_eq!(std::fs::read(&marker).unwrap(), b"keep");
}

#[tokio::test]
async fn replacement_updates_existing_snapshot() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("session.json");
    atomic_write(&path, b"first".to_vec()).await.unwrap();
    atomic_write(&path, b"second".to_vec()).await.unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), b"second");
}
