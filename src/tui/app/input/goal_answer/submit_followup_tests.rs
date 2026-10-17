//! Existing answer-review dialogue still accepts context-only follow-ups.

use crate::session::runtime::goals::answer_review;
use crate::tui::app::{
    session_runtime::{SessionSlot, spawn},
    state::App,
};

#[tokio::test]
async fn held_goal_plain_followup_keeps_review_and_does_not_resume_execution() {
    let session = crate::session::tasks::answer_review_test_support::session().await;
    let id = session.id.clone();
    answer_review::begin(&id, "First question").await.unwrap();
    let slot = SessionSlot::new(session);
    let mut app = App::default();
    app.state.input = "Explain that part again".into();
    let (events, _) = tokio::sync::mpsc::channel(4);
    let (notices, _) = tokio::sync::mpsc::channel(4);
    let runtime = spawn(events, notices);
    assert!(super::intercept(&mut app, &slot, &runtime).await);
    let state = answer_review::read(&id).unwrap();
    assert_eq!(
        state.answer_review.unwrap().question.as_deref(),
        Some("Explain that part again")
    );
    assert!(!state.goal.unwrap().status.is_active());
    assert!(app.state.input.is_empty());
    runtime.shutdown().await;
}
