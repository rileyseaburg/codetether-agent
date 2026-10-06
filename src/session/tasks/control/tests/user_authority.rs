//! Human overrides preserve task state and usage and cannot be invoked by a model.

use super::{fixture, user_fixture};
use crate::session::tasks::GoalStatus;

#[tokio::test]
async fn session_goal_user_override_preserves_tasks_and_accounting() {
    let (_dir, log) = user_fixture::held().await;
    let before = user_fixture::state(&log).await;
    let mut request = fixture::request(&before, "override");
    request.objective = Some("New objective".into());
    request.token_budget = Some(None);
    assert!(
        super::super::edit::apply(&log, request.clone())
            .await
            .is_err()
    );
    let after = super::super::edit::apply_user(&log, request).await.unwrap();
    assert!(after.answer_review.is_none());
    assert_eq!(after.tasks["work"].content, "Keep task");
    let goal = after.goal.unwrap();
    assert_eq!(goal.id, "goal");
    assert_eq!(goal.objective, "New objective");
    assert_eq!(goal.tokens_used, 37);
    assert_eq!(goal.time_used_seconds, 4);
    assert_eq!(goal.turns_used, 3);
    assert_eq!(goal.status, GoalStatus::Active);
}

#[tokio::test]
async fn session_goal_user_override_does_not_silently_remove_budget() {
    let (_dir, log) = user_fixture::held().await;
    let request = fixture::request(&user_fixture::state(&log).await, "override");
    let after = super::super::edit::apply_user(&log, request).await.unwrap();
    assert!(after.answer_review.is_none());
    assert_eq!(after.goal.unwrap().status, GoalStatus::BudgetLimited);
    assert!(after.tasks.contains_key("work"));
}
