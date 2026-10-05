//! Task-log replay retains satisfaction holds across restarts.

use crate::session::tasks::answer_review_test_support::{decision, goal};
use crate::session::tasks::{AnswerReviewAction as Action, GoalStatus, TaskLog, TaskState};

#[tokio::test]
async fn answer_review_restart_requires_yes_again_until_accepted() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("review.tasks.jsonl");
    let log = TaskLog::at(&path);
    log.append(&goal("goal")).await.unwrap();
    log.append(&decision(
        "review",
        Action::Begin {
            question: "Why?".into(),
        },
    ))
    .await
    .unwrap();
    log.append(&decision("review", Action::Answered))
        .await
        .unwrap();
    let replay = || TaskState::from_log(&TaskLog::at(&path).read_all_blocking().unwrap());
    assert!(replay().answer_review.unwrap().ready);
    assert_eq!(replay().goal.unwrap().status, GoalStatus::Paused);
    log.append(&decision("review", Action::Unsatisfied))
        .await
        .unwrap();
    assert!(!replay().answer_review.unwrap().ready);
    assert_eq!(replay().goal.unwrap().status, GoalStatus::Paused);
}

#[test]
fn answer_review_absent_in_legacy_log_does_not_create_hold() {
    let state = TaskState::from_log(&[goal("goal")]);
    assert!(state.answer_review.is_none());
    assert_eq!(state.goal.unwrap().status, GoalStatus::Active);
}
