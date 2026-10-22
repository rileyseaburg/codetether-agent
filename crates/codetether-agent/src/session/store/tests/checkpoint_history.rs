//! Canonical checkpoints survive a saturated working window and cold reload.
use super::super::{WINDOW, load, save};
use crate::session::Session;
#[path = "checkpoint_history_fixture.rs"]
mod fixture;
#[tokio::test]
async fn canonical_checkpoint_recovers_evicted_constraints_and_tool_results() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("checkpoint.json");
    let mut session = Session::new().await.unwrap();
    session.messages.extend(fixture::history(WINDOW * 2));
    save(&session, &path).await.unwrap();
    let mut tail = load(&path, WINDOW).await.unwrap().session;
    let before = tail.message_count();
    tail.add_message(super::message("Latest correction: preserve public API"));
    assert_eq!(tail.messages.len(), WINDOW);
    assert!(codetether_run_resume::should_checkpoint(before, &tail, 2));
    let cp = tail
        .build_run_checkpoint("repair", 2, root.path())
        .await
        .unwrap();
    assert_eq!(cp.message_count, before + 1);
    assert!(
        cp.user_context
            .iter()
            .any(|s| s.source_turn == 0 && s.text == "Never deploy")
    );
    assert_eq!(
        cp.actions[0].status,
        codetether_run_checkpoint::ActionStatus::Failed
    );
    assert_eq!(cp.actions[0].source_turn, 20);
    assert_eq!(cp.actions[0].result_turn, Some(1_100));
    tail.metadata.run_checkpoint = Some(cp.clone());
    save(&tail, &path).await.unwrap();
    let cold = load(&path, 16).await.unwrap().session;
    let rebuilt = cold
        .build_run_checkpoint("repair", 2, root.path())
        .await
        .unwrap();
    assert_eq!(rebuilt.user_context, cp.user_context);
    assert_eq!(rebuilt.actions, cp.actions);
    assert_eq!(rebuilt.message_count, cp.message_count);
    assert_eq!(
        cold.metadata.run_checkpoint.unwrap().user_context,
        cp.user_context
    );
}
