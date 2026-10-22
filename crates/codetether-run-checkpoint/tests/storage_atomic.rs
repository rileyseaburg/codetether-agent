mod storage_support;
use codetether_run_checkpoint::{
    RunCheckpoint,
    storage::{Storage, load, save},
};
use storage_support::Session;
fn checkpoint(id: &str, objective: &str) -> RunCheckpoint {
    RunCheckpoint::from_session_messages(objective, 1, id, None, 0, &[])
}
#[tokio::test]
async fn failed_canonical_save_keeps_previous_metadata_and_sidecar() {
    let mut session = Session::new("save-failure");
    let old = checkpoint(session.id(), "original");
    let path = save(&mut session, old.clone()).await.unwrap();
    session.fail = true;
    let new = checkpoint(session.id(), "changed");
    assert!(save(&mut session, new).await.is_err());
    assert_eq!(session.checkpoint, Some(old.clone()));
    session.checkpoint = None;
    assert_eq!(load(&session).await.unwrap(), Some(old));
    assert!(path.is_file());
}
#[tokio::test]
async fn sidecar_failure_keeps_committed_canonical_checkpoint() {
    let mut session = Session::new("sidecar-failure");
    let path = Session::sessions_dir()
        .unwrap()
        .join("sidecar-failure.checkpoint.json");
    tokio::fs::create_dir(&path).await.unwrap();
    let new = checkpoint(session.id(), "committed");
    let error = save(&mut session, new.clone()).await.unwrap_err();
    assert!(error.to_string().contains("canonical checkpoint committed"));
    assert_eq!(session.saved.lock().unwrap().as_ref(), Some(&new));
    assert_eq!(load(&session).await.unwrap(), Some(new));
}
#[tokio::test]
async fn atomic_replacement_preserves_readable_fallback() {
    let mut session = Session::new("replacement");
    let initial = checkpoint(session.id(), "initial");
    let path = save(&mut session, initial).await.unwrap();
    let reader = tokio::spawn(async move {
        for _ in 0..100 {
            let bytes = tokio::fs::read(&path).await.unwrap();
            let cp: RunCheckpoint = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(cp.session_id, "replacement");
            tokio::task::yield_now().await;
        }
    });
    for index in 0..12 {
        let cp = checkpoint(session.id(), &format!("{index}: {}", "🦀".repeat(8_000)));
        let path = save(&mut session, cp.clone()).await.unwrap();
        let persisted: RunCheckpoint =
            serde_json::from_slice(&tokio::fs::read(path).await.unwrap()).unwrap();
        assert_eq!(persisted, cp);
    }
    reader.await.unwrap();
}
