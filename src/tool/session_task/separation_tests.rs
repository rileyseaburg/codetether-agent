//! Task discovery/listing cannot conflate the objective with work items.

use crate::session::tasks::{TaskLog, TaskState, goal_block, task_block};
use serde_json::json;

#[test]
fn session_tasks_schema_advertises_only_work_item_operations() {
    let schema = super::value();
    assert_eq!(
        schema["properties"]["action"]["enum"],
        json!(["task_add", "task_status", "list"])
    );
    assert!(schema["properties"].get("objective").is_none());
}

#[tokio::test]
async fn session_tasks_completion_and_listing_do_not_complete_goal() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("tasks.jsonl"));
    for event in [
        json!({"kind":"goal_set","at":chrono::Utc::now(),"goal_id":"g","objective":"Overall outcome","success_criteria":[],"forbidden":[]}),
        json!({"kind":"task_added","at":chrono::Utc::now(),"id":"t","content":"Individual work","parent_id":null}),
        json!({"kind":"task_status","at":chrono::Utc::now(),"id":"t","status":"done","note":null}),
    ] {
        log.append(&serde_json::from_value(event).unwrap())
            .await
            .unwrap();
    }
    let state = TaskState::from_log(&log.read_all().await.unwrap());
    assert!(state.goal.as_ref().unwrap().status.is_active());
    assert!(task_block(&state).contains("[done]"));
    assert!(!task_block(&state).contains("Overall outcome"));
    assert!(!goal_block(&state).unwrap().contains("Individual work"));
    let result = super::super::handlers::list(&log).await.unwrap();
    assert!(result.output.contains("Individual work"));
    assert!(!result.output.contains("Overall outcome"));
}
