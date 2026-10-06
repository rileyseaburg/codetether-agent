//! Delayed completion of the old objective must not undo a human override.

use super::{fixture, user_fixture};
use crate::session::tasks::{GoalStatus, GoalStatusChecked, TaskEvent};
use chrono::Utc;

#[tokio::test]
async fn session_goal_user_override_rejects_old_verifier_completion() {
    let (_dir, log) = user_fixture::held().await;
    let before = user_fixture::state(&log).await;
    let expected = before.goal.as_ref().unwrap();
    let mut request = fixture::request(&before, "override");
    request.objective = Some("User changed the objective".into());
    request.token_budget = Some(None);
    super::super::edit::apply_user(&log, request).await.unwrap();
    log.append(&TaskEvent::GoalStatusChecked(GoalStatusChecked {
        at: Utc::now(),
        goal_id: expected.id.clone(),
        expected_updated_at: expected.last_updated_at,
        status: GoalStatus::Complete,
    }))
    .await
    .unwrap();
    let after = user_fixture::state(&log).await;
    let goal = after.goal.unwrap();
    assert_eq!(goal.status, GoalStatus::Active);
    assert_eq!(goal.objective, "User changed the objective");
    assert!(after.tasks.contains_key("work"));
}
