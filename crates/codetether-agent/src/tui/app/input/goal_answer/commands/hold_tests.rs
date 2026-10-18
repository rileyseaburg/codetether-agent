//! A human can revise or explicitly override a held goal without a model turn.

use super::fixture;
use crate::session::tasks::{GoalStatus, runtime::answer_review};

#[tokio::test]
async fn live_goal_commands_override_and_complete_a_review_hold() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    let id = slot.view().id().to_string();
    answer_review::begin(&id, "Why?").await.unwrap();
    fixture::submit(&mut app, &mut slot, &runtime, "/goal edit better objective").await;
    assert!(answer_review::held(&id));
    fixture::submit(
        &mut app,
        &mut slot,
        &runtime,
        "/goal override authoritative objective",
    )
    .await;
    let state = answer_review::read(&id).unwrap();
    assert!(state.answer_review.is_none());
    assert_eq!(state.goal.unwrap().objective, "authoritative objective");
    answer_review::begin(&id, "Another question").await.unwrap();
    fixture::submit(&mut app, &mut slot, &runtime, "/goal done").await;
    assert!(!answer_review::held(&id));
    assert_eq!(
        answer_review::read(&id).unwrap().goal.unwrap().status,
        GoalStatus::Complete
    );
    assert!(super::super::pending::take(&id));
    runtime.shutdown().await;
}
