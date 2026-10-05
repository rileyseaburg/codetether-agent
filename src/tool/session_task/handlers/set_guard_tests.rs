//! Tool-level replacement must not bypass the explicit satisfaction decision.

use crate::session::tasks::answer_review_test_support::{decision, goal};
use crate::session::tasks::{AnswerReviewAction as Action, TaskLog, TaskState};
use serde_json::json;

#[tokio::test]
async fn answer_review_model_cannot_replace_goal_until_accepted() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(&dir.path().join("review.tasks.jsonl"));
    for event in [
        goal("goal"),
        decision(
            "review",
            Action::Begin {
                question: "Why?".into(),
            },
        ),
    ] {
        log.append(&event).await.unwrap();
    }
    let call = json!({"action": "set_goal", "objective": "Replacement"});
    let result = super::super::dispatch::run(&log, serde_json::from_value(call.clone()).unwrap())
        .await
        .unwrap();
    assert!(!result.success);
    assert!(result.output.contains("answer review"));
    assert_eq!(log.read_all().await.unwrap().len(), 2);
    let state = TaskState::from_log(&log.read_all().await.unwrap());
    assert_eq!(state.goal.unwrap().id, "goal");
    assert!(state.answer_review.is_some());
    for action in [Action::Answered, Action::Satisfied] {
        log.append(&decision("review", action)).await.unwrap();
    }
    let result = super::super::dispatch::run(&log, serde_json::from_value(call).unwrap())
        .await
        .unwrap();
    assert!(result.success);
    let state = TaskState::from_log(&log.read_all().await.unwrap());
    assert_eq!(state.goal.unwrap().objective, "Replacement");
    assert!(state.answer_review.is_none());
}
