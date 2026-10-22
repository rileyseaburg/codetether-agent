mod storage_support;
use codetether_run_checkpoint::{
    RunCheckpoint,
    storage::{Storage, clear, load, save},
};
use std::sync::atomic::Ordering;
use storage_support::Session;
fn cp(id: &str) -> RunCheckpoint {
    RunCheckpoint::from_session_messages("task", 2, id, None, 0, &[])
}
#[tokio::test]
async fn interrupted_clear_cannot_load_an_old_or_malformed_sidecar() {
    let mut session = Session::new("clear-leftover");
    let old = cp(session.id());
    let path = save(&mut session, old.clone()).await.unwrap();
    clear(&mut session).await.unwrap();
    assert!(session.saved_cleared.load(Ordering::SeqCst));
    tokio::fs::write(&path, serde_json::to_vec(&old).unwrap())
        .await
        .unwrap();
    let mut cold = Session::new("clear-leftover");
    cold.cleared = session.saved_cleared.load(Ordering::SeqCst);
    assert!(load(&cold).await.unwrap().is_none());
    tokio::fs::write(&path, b"malformed old sidecar")
        .await
        .unwrap();
    assert!(load(&cold).await.unwrap().is_none());
}
#[tokio::test]
async fn failed_clear_does_not_change_metadata_or_sidecar() {
    let mut session = Session::new("clear-failed");
    let old = cp(session.id());
    let path = save(&mut session, old.clone()).await.unwrap();
    session.fail = true;
    assert!(clear(&mut session).await.is_err());
    assert_eq!(session.checkpoint, Some(old.clone()));
    assert!(!session.cleared);
    assert_eq!(
        serde_json::from_slice::<RunCheckpoint>(&tokio::fs::read(path).await.unwrap()).unwrap(),
        old
    );
}
#[tokio::test]
async fn failed_cleanup_keeps_clear_marker_and_new_run_resets_it() {
    let mut session = Session::new("clear-cleanup-failed");
    let path = save(&mut session, cp("clear-cleanup-failed"))
        .await
        .unwrap();
    tokio::fs::remove_file(&path).await.unwrap();
    tokio::fs::create_dir(&path).await.unwrap();
    let error = clear(&mut session).await.unwrap_err();
    assert!(error.to_string().contains("clear committed"));
    assert!(load(&session).await.unwrap().is_none());
    tokio::fs::remove_dir(&path).await.unwrap();
    save(&mut session, cp("clear-cleanup-failed"))
        .await
        .unwrap();
    assert!(!session.cleared);
    assert!(load(&session).await.unwrap().is_some());
}
#[tokio::test]
async fn failed_new_run_restores_existing_tombstone() {
    let mut session = Session::new("cleared-then-failed");
    session.cleared = true;
    session.fail = true;
    assert!(save(&mut session, cp("cleared-then-failed")).await.is_err());
    assert!(session.cleared);
    assert!(session.checkpoint.is_none());
}
