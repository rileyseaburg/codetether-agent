//! Explicit user completion and pause are independent of task completion.

use super::{fixture, user_fixture};
use crate::session::tasks::GoalStatus;

#[tokio::test]
async fn session_goal_user_pause_survives_review_and_force_complete_is_privileged() {
    let (_dir, log) = user_fixture::held().await;
    let request = fixture::request(&user_fixture::state(&log).await, "pause");
    let paused = super::super::edit::apply_user(&log, request).await.unwrap();
    assert_eq!(
        paused.answer_review.unwrap().resume_status,
        GoalStatus::Paused
    );
    let request = fixture::request(&user_fixture::state(&log).await, "force_complete");
    assert!(
        super::super::edit::apply(&log, request.clone())
            .await
            .is_err()
    );
    let done = super::super::edit::apply_user(&log, request).await.unwrap();
    assert!(done.answer_review.is_none());
    assert_eq!(done.goal.unwrap().status, GoalStatus::Complete);
    assert!(done.tasks.contains_key("work"));
    let mut request = fixture::request(&user_fixture::state(&log).await, "override");
    request.token_budget = Some(None);
    let active = super::super::edit::apply_user(&log, request).await.unwrap();
    assert_eq!(active.goal.unwrap().status, GoalStatus::Active);
    let request = fixture::request(&user_fixture::state(&log).await, "clear");
    let cleared = super::super::edit::apply_user(&log, request).await.unwrap();
    assert!(cleared.goal.is_none());
    assert!(cleared.tasks.contains_key("work"));
}
