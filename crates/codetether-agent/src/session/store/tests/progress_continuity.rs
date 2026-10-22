//! Durable result evidence survives working-window eviction and cold reload.
use super::super::{load, save};
use crate::session::Session;
#[path = "progress_fixture.rs"]
mod fixture;
#[tokio::test]
async fn failed_edits_survive_reload_and_automatic_continuations() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("progress.json");
    let mut session = Session::new().await.unwrap();
    for id in 0..3 {
        session.add_message(fixture::call(id));
        session.add_message(fixture::result(id));
        save(&session, &path).await.unwrap();
        session = load(&path, 1).await.unwrap().session;
    }
    assert!(fixture::blocked(&session));
    session.add_human_message(super::message(
        "Continue this live CodeTether run from its structured checkpoint.\n...",
    ));
    assert!(fixture::blocked(&session));
    session.add_human_message(super::message("I repaired the target. Please retry."));
    assert!(!fixture::blocked(&session));
    assert_eq!(session.metadata.progress.observations, 3);
}
#[tokio::test]
async fn clear_marker_survives_cold_load_without_sidecar_access() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("cleared.json");
    let mut session = Session::new().await.unwrap();
    session.metadata.run_checkpoint = Some(
        codetether_run_checkpoint::RunCheckpoint::from_session_messages(
            "task",
            1,
            &session.id,
            None,
            0,
            &[],
        ),
    );
    save(&session, &path).await.unwrap();
    session.clear_run_checkpoint().await.unwrap();
    let cold = load(&path, 0).await.unwrap().session;
    assert!(cold.metadata.run_checkpoint_cleared);
    assert!(cold.load_run_checkpoint().await.unwrap().is_none());
}
