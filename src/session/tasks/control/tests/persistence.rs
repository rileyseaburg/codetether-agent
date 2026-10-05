//! Edits survive native journal reload without replacing the session goal.

use super::fixture;
use crate::session::tasks::{TaskLog, TaskState};

/// Successful edits append one event and preserve all earlier journal bytes.
#[tokio::test]
async fn session_goal_controls_persist_edits_atomically() {
    let directory = tempfile::tempdir().unwrap();
    let log = TaskLog::at(directory.path().join("session.tasks.jsonl"));
    for event in fixture::events() {
        log.append(&event).await.unwrap();
    }
    let original = tokio::fs::read(log.path()).await.unwrap();
    let state = TaskState::from_log(&log.read_all().await.unwrap());
    let mut request = fixture::request(&state, "edit");
    request.objective = Some("Updated objective".into());
    request.success_criteria = Some(vec!["New criterion".into()]);
    request.token_budget = Some(None);
    let after = super::super::edit::apply(&log, request.clone())
        .await
        .unwrap();
    assert_eq!(after.goal.as_ref().unwrap().objective, "Updated objective");
    assert_eq!(after.goal.as_ref().unwrap().tokens_used, 37);
    assert!(super::super::edit::apply(&log, request).await.is_err());
    let reloaded = TaskState::from_log(&log.read_all().await.unwrap());
    assert_eq!(reloaded.goal.unwrap().success_criteria, ["New criterion"]);
    assert!(
        tokio::fs::read(log.path())
            .await
            .unwrap()
            .starts_with(&original)
    );
    assert_eq!(log.read_all().await.unwrap().len(), 3);
    let clear = fixture::request(&after, "clear");
    assert!(
        super::super::edit::apply(&log, clear)
            .await
            .unwrap()
            .goal
            .is_none()
    );
}
