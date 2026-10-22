use super::{load, tests::goal};
use chrono::Utc;
use codetether_goal_core::{
    AnswerReviewAction, AnswerReviewUpdate, GoalRuntimeUpdate, GoalStatus, TaskEvent, TaskLog,
    TaskState,
};

#[test]
fn incremental_replay_preserves_budget_holds_and_exactly_once_accounting() {
    let dir = tempfile::tempdir().unwrap();
    let log = TaskLog::at(dir.path().join("journal"));
    log.append_blocking(&goal("goal")).unwrap();
    load(&log).unwrap();
    log.append_blocking(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: "goal".into(),
        objective: None,
        status: None,
        token_budget: Some(10),
        token_delta: 7,
        elapsed_seconds: 2,
        continuation_delta: 1,
    }))
    .unwrap();
    for _ in 0..3 {
        let goal = load(&log).unwrap().goal.unwrap();
        assert_eq!(
            (goal.tokens_used, goal.time_used_seconds, goal.turns_used),
            (7, 2, 2)
        );
    }
    log.append_blocking(&TaskEvent::AnswerReview(AnswerReviewUpdate {
        at: Utc::now(),
        goal_id: "goal".into(),
        review_id: "review".into(),
        decision: AnswerReviewAction::Begin {
            question: "Explain this".into(),
        },
    }))
    .unwrap();
    let held = load(&log).unwrap();
    assert!(held.answer_review.is_some());
    log.append_blocking(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: "goal".into(),
        objective: None,
        status: None,
        token_budget: None,
        token_delta: 5,
        elapsed_seconds: 1,
        continuation_delta: 0,
    }))
    .unwrap();
    let warm = load(&log).unwrap();
    let cold = TaskState::from_log(&log.read_all_blocking().unwrap());
    let warm_goal = warm.goal.unwrap();
    let cold_goal = cold.goal.unwrap();
    assert_eq!(warm_goal.status, cold_goal.status);
    assert_eq!(warm_goal.tokens_used, 12);
    assert_eq!(warm_goal.status, GoalStatus::Paused);
    assert_eq!(warm_goal.token_budget, Some(10));
    assert!(warm.answer_review.is_some());
}
