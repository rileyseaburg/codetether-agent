//! Question interruption is independent of slash commands and tool feedback.

use crate::session::tasks::runtime::answer_review;
use crate::tui::app::{
    session_runtime::{SessionSlot, spawn},
    state::App,
};

#[tokio::test]
async fn answer_review_intercepts_plain_and_ask_questions() {
    for question in ["Why?", "/ask Why?"] {
        let session = crate::session::tasks::answer_review_test_support::session().await;
        let slot = SessionSlot::new(session);
        let mut app = App::default();
        app.state.processing = true;
        let (event_tx, _event_rx) = tokio::sync::mpsc::channel(4);
        let (notice_tx, _notice_rx) = tokio::sync::mpsc::channel(4);
        let runtime = spawn(event_tx, notice_tx);
        app.state.input = "/continue".into();
        assert!(super::intercept(&mut app, &slot, &runtime).await);
        assert!(!answer_review::held(slot.view().id()));
        app.state.input = question.into();
        assert!(super::intercept(&mut app, &slot, &runtime).await);
        assert!(app.state.input.is_empty());
        assert!(!app.state.answer_review_yes);
        let state = answer_review::read(slot.view().id()).unwrap();
        assert_eq!(
            state.answer_review.unwrap().question.as_deref(),
            Some("Why?")
        );
        runtime.shutdown().await;
    }
}
