//! Legacy import keeps exact evidence and refuses stale legacy writers.
use super::super::{batch, load, migration, save, transaction};
use super::message;
use crate::session::Session;
#[tokio::test]
async fn import_retains_original_and_tail_identity() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("legacy.json");
    let mut session = Session::new().await.unwrap();
    session.messages.extend((0..20).map(|_| message("legacy")));
    let original = serde_json::to_vec(&session).unwrap();
    std::fs::write(&path, &original).unwrap();
    let loaded = load(&path, 3).await.unwrap();
    assert_eq!(loaded.session.id, session.id);
    assert_eq!(loaded.dropped, 17);
    assert_eq!(
        std::fs::read(path.with_extension("legacy.json")).unwrap(),
        original
    );
    std::fs::write(&path, &original).unwrap();
    migration::ensure(&path).unwrap(); // commit survived, locator did not
    let mut live = load(&path, 3).await.unwrap().session;
    live.messages.push(message("new"));
    save(&live, &path).await.unwrap();
    std::fs::write(&path, &original).unwrap();
    assert!(load(&path, 3).await.is_err());
}
#[tokio::test]
async fn replay_after_commit_before_locator_is_idempotent() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("recover.json");
    let mut session = Session::new().await.unwrap();
    session.messages.push(message("committed"));
    let original = serde_json::to_vec(&session).unwrap();
    std::fs::write(&path, &original).unwrap();
    std::fs::write(path.with_extension("legacy.json"), &original).unwrap();
    let pending = batch::prepare(&session, &Default::default()).unwrap();
    transaction::commit(&path, &pending).unwrap();
    let loaded = load(&path, 10).await.unwrap();
    assert_eq!(loaded.session.messages.len(), 1);
    assert_eq!(loaded.session.storage.0.lock().unwrap().revision, 1);
}
