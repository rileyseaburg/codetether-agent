//! Continuation never creates a goal or turns a resume request into a question.
use crate::session::tasks::runtime::answer_review;
#[path = "commands/tests/fixture.rs"]
mod fixture;

#[tokio::test]
async fn answer_review_continuation_without_goal_does_not_intercept() {
    let session = crate::session::Session::new().await.unwrap();
    assert!(!super::release(&session.id).await.unwrap());
}

#[tokio::test]
async fn answer_review_continuation_active_goal_does_not_create_question() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    fixture::submit(&mut app, &mut slot, &runtime, "continue").await;
    assert!(!answer_review::held(slot.view().id()));
    assert_eq!(app.state.status, "Goal continuation requested");
    runtime.shutdown().await;
}
