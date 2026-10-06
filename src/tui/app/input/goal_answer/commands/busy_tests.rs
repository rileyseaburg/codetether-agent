//! Busy-session goal edits are not queued as chat or rejected for missing ownership.

use super::fixture;
use crate::session::tasks::runtime::answer_review;

#[tokio::test]
async fn live_goal_commands_edit_a_checked_out_session_without_tool_approval() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    let id = slot.view().id().to_string();
    let session = slot.take_for_prompt().unwrap();
    app.state.processing = true;
    app.state.approval_waiting = true;
    fixture::submit(&mut app, &mut slot, &runtime, "/goal edit revised goal").await;
    assert!(slot.borrow().is_none());
    assert_eq!(
        answer_review::read(&id).unwrap().goal.unwrap().objective,
        "revised goal"
    );
    assert!(app.state.approval_waiting);
    assert!(super::super::pending::take(&id));
    fixture::submit(&mut app, &mut slot, &runtime, "/goal budget none").await;
    assert_eq!(
        answer_review::read(&id).unwrap().goal.unwrap().token_budget,
        None
    );
    assert!(super::super::pending::take(&id));
    fixture::submit(&mut app, &mut slot, &runtime, "/goal budget -1").await;
    assert!(!super::super::pending::take(&id));
    assert!(app.state.status.contains("budget must be positive"));
    slot.restore(session);
    runtime.shutdown().await;
}
