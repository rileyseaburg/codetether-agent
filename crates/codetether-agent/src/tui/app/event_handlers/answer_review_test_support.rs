//! Ready-review fixtures for keyboard and rendering regressions.

use crate::session::tasks::{AnswerReviewAction, runtime::answer_review};
use crate::tui::app::{session_runtime::SessionSlot, state::App};

pub(crate) async fn ready() -> (App, SessionSlot) {
    let session = crate::session::tasks::answer_review_test_support::session().await;
    answer_review::begin(&session.id, "Why?").await.unwrap();
    let review = answer_review::read(&session.id)
        .unwrap()
        .answer_review
        .unwrap();
    answer_review::record(
        &session.id,
        &review.goal_id,
        &review.id,
        AnswerReviewAction::Answered,
    )
    .await
    .unwrap();
    let mut app = App::default();
    app.state.session_id = Some(session.id.clone());
    (app, SessionSlot::new(session))
}
