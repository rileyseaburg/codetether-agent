//! Goal and task lists remain distinct during an active turn.

use super::fixture;
use crate::session::tasks::TaskLog;
use serde_json::json;

#[tokio::test]
async fn live_goal_commands_list_tasks_separately_from_objective() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    let log = TaskLog::for_session(slot.view().id()).unwrap();
    log.append(&serde_json::from_value(json!({
        "kind":"task_added", "at":chrono::Utc::now(), "id":"task", "content":"Independent work item", "parent_id":null
    })).unwrap()).await.unwrap();
    let session = slot.take_for_prompt().unwrap();
    app.state.processing = true;
    fixture::submit(&mut app, &mut slot, &runtime, "/tasks").await;
    let tasks = &app.state.messages.last().unwrap().content;
    assert!(tasks.contains("Independent work item"));
    assert!(!tasks.contains("OBJECTIVE:"));
    fixture::submit(&mut app, &mut slot, &runtime, "/goal show").await;
    let goal = &app.state.messages.last().unwrap().content;
    assert!(goal.contains("OBJECTIVE:"));
    assert!(!goal.contains("Independent work item"));
    slot.restore(session);
    runtime.shutdown().await;
}
