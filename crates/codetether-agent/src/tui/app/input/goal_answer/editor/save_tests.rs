//! Save text changes while retaining usage, criteria, limits, and work items.
use super::{constraints, fixture, keys};
use crate::session::tasks::{TaskLog, runtime::answer_review};
use serde_json::json;

#[tokio::test]
async fn goal_editor_saves_multiline_edits_after_live_accounting() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    let id = slot.view().id().to_string();
    constraints::set(&id).await;
    let log = TaskLog::for_session(&id).unwrap();
    let original = answer_review::read(&id).unwrap().goal.unwrap();
    fixture::submit(&mut app, &mut slot, &runtime, "/goal edit").await;
    keys::end(&mut app, &mut slot, &runtime).await;
    super::super::paste(&mut app, "\r\nKeep the API compatible — café");
    for event in [
        json!({"kind":"goal_runtime","at":chrono::Utc::now(),"goal_id":original.id,"token_delta":13,"elapsed_seconds":2,"continuation_delta":0}),
        json!({"kind":"task_added","at":chrono::Utc::now(),"id":"work","content":"Do work","parent_id":null}),
    ] {
        log.append(&serde_json::from_value(event).unwrap())
            .await
            .unwrap();
    }
    keys::save(&mut app, &mut slot, &runtime).await;
    assert!(app.state.goal_editor.is_none());
    let state = answer_review::read(&id).unwrap();
    let goal = state.goal.unwrap();
    assert_eq!(
        goal.objective,
        "Finish the goal\nKeep the API compatible — café"
    );
    assert_eq!(goal.id, original.id);
    assert_eq!(goal.tokens_used, 13);
    assert_eq!(goal.status, original.status);
    assert_eq!(goal.success_criteria, original.success_criteria);
    assert_eq!(goal.forbidden, original.forbidden);
    assert_eq!(goal.token_budget, original.token_budget);
    assert_eq!(state.tasks["work"].content, "Do work");
    runtime.shutdown().await;
}
