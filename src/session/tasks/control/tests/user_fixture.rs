//! Isolated active-review fixture with an independent session work item.

use crate::session::tasks::{AnswerReviewAction, AnswerReviewUpdate, TaskEvent, TaskLog};
use chrono::Utc;
use serde_json::json;

pub(super) async fn held() -> (tempfile::TempDir, TaskLog) {
    let directory = tempfile::tempdir().unwrap();
    let log = TaskLog::at(directory.path().join("goal.tasks.jsonl"));
    for event in super::fixture::events() {
        log.append(&event).await.unwrap();
    }
    log.append(
        &serde_json::from_value(json!({
            "kind":"task_added", "at":Utc::now(), "id":"work", "content":"Keep task",
            "parent_id":null
        }))
        .unwrap(),
    )
    .await
    .unwrap();
    log.append(&TaskEvent::AnswerReview(AnswerReviewUpdate {
        at: Utc::now(),
        goal_id: "goal".into(),
        review_id: "review".into(),
        decision: AnswerReviewAction::Begin {
            question: "Why?".into(),
        },
    }))
    .await
    .unwrap();
    (directory, log)
}

pub(super) async fn state(log: &TaskLog) -> crate::session::tasks::TaskState {
    crate::session::tasks::TaskState::from_log(&log.read_all().await.unwrap())
}
